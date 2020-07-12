#[cfg(test)]
mod mock;
#[cfg(test)]
mod tests;

use regex::Regex;
use rusoto_core::RusotoError;
use rusoto_logs::{
  CloudWatchLogs, CreateLogGroupError, CreateLogGroupRequest, CreateLogStreamError,
  CreateLogStreamRequest, DescribeLogGroupsError, DescribeLogGroupsRequest,
  DescribeLogStreamsError, DescribeLogStreamsRequest, InputLogEvent, PutLogEventsError,
  PutLogEventsRequest,
};
use std::error;
use std::io;

async fn log_group_exist<C>(
  client: &C,
  log_group_name: &str,
) -> Result<bool, RusotoError<DescribeLogGroupsError>>
where
  C: CloudWatchLogs,
{
  let mut next_token = None;

  loop {
    let req = DescribeLogGroupsRequest {
      log_group_name_prefix: Some(log_group_name.to_string()),
      limit: None,
      next_token: next_token,
    };

    let res = client.describe_log_groups(req).await?;

    if res.log_groups.is_none() {
      break;
    }

    for log_group in res.log_groups.unwrap() {
      if log_group.log_group_name.unwrap() == log_group_name {
        return Ok(true);
      }
    }

    next_token = res.next_token;

    if next_token.is_none() {
      break;
    }
  }

  Ok(false)
}

async fn create_log_group<C>(
  client: &C,
  log_group_name: &str,
) -> Result<(), RusotoError<CreateLogGroupError>>
where
  C: CloudWatchLogs,
{
  let req = CreateLogGroupRequest {
    log_group_name: log_group_name.to_string(),
    kms_key_id: None,
    tags: None,
  };

  client.create_log_group(req).await?;

  Ok(())
}

async fn log_stream_exist<C>(
  client: &C,
  log_group_name: &str,
  log_stream_name: &str,
) -> Result<bool, RusotoError<DescribeLogStreamsError>>
where
  C: CloudWatchLogs,
{
  let req = DescribeLogStreamsRequest {
    log_group_name: log_group_name.to_string(),
    log_stream_name_prefix: Some(log_stream_name.to_string()),
    descending: Some(false),
    order_by: Some("LogStreamName".to_string()),
    limit: Some(1),
    next_token: None,
  };

  let res = client.describe_log_streams(req).await?;

  if res.log_streams.is_none() {
    return Ok(false);
  }

  for log_stream in res.log_streams.unwrap() {
    if log_stream.log_stream_name.unwrap() == log_stream_name {
      return Ok(true);
    }
  }

  Ok(false)
}

async fn create_log_stream<C>(
  client: &C,
  log_group_name: &str,
  log_stream_name: &str,
) -> Result<(), RusotoError<CreateLogStreamError>>
where
  C: CloudWatchLogs,
{
  let req = CreateLogStreamRequest {
    log_group_name: log_group_name.to_string(),
    log_stream_name: log_stream_name.to_string(),
  };

  client.create_log_stream(req).await?;

  Ok(())
}

async fn put_log_event<C>(
  client: &C,
  log_group_name: &str,
  log_stream_name: &str,
  message: &str,
  sequence_token: Option<String>,
) -> Result<Option<String>, RusotoError<PutLogEventsError>>
where
  C: CloudWatchLogs,
{
  let log_event = InputLogEvent {
    message: message.to_string(),
    timestamp: chrono::Utc::now().timestamp_millis(),
  };

  let req = PutLogEventsRequest {
    log_group_name: log_group_name.to_string(),
    log_stream_name: log_stream_name.to_string(),
    log_events: vec![log_event],
    sequence_token: sequence_token,
  };

  match client.put_log_events(req).await {
    Ok(res) => Ok(res.next_sequence_token),
    Err(e) => Err(e),
  }
}

pub async fn tee<C, R, W>(
  client: &C,
  log_group_name: &str,
  log_stream_name: &str,
  mut reader: R,
  out: &mut W,
) -> Result<(), Box<dyn error::Error>>
where
  C: CloudWatchLogs,
  R: io::prelude::BufRead,
  W: io::Write,
{
  if !log_group_exist(client, log_group_name).await? {
    create_log_group(client, log_group_name).await?;
  }

  if !log_stream_exist(client, log_group_name, log_stream_name).await? {
    create_log_stream(client, log_group_name, log_stream_name).await?;
  }

  let mut buf = String::new();
  let mut sequence_token = None;
  let sequence_token_re = Regex::new(r#"\bsequenceToken(?: is)?: (\S+)\b"#).unwrap();

  while reader.read_line(&mut buf)? > 0 {
    let buf_no_nl = buf.trim_end();

    loop {
      let res = put_log_event(
        client,
        log_group_name,
        log_stream_name,
        &buf_no_nl,
        sequence_token.clone(),
      )
      .await;

      match res {
        Ok(st) => sequence_token = st,
        Err(RusotoError::Service(PutLogEventsError::DataAlreadyAccepted(msg)))
        | Err(RusotoError::Service(PutLogEventsError::InvalidSequenceToken(msg))) => {
          let m = sequence_token_re.captures(&msg).unwrap();
          sequence_token = Some(m[1].to_string());
          continue;
        }
        Err(e) => return Err(Box::new(e)),
      }

      break;
    }

    writeln!(out, "{}", buf_no_nl)?;
    buf.clear();
  }

  Ok(())
}
