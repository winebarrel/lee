use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct LogGroup {
  #[serde(rename = "logGroupName")]
  pub log_group_name: String,
}

#[derive(Debug, Serialize)]
pub struct DescribeLogGroupsResponse {
  #[serde(rename = "logGroups")]
  pub log_groups: Vec<LogGroup>,
  #[serde(rename = "nextToken", skip_serializing_if = "Option::is_none")]
  pub next_token: Option<String>,
}

impl DescribeLogGroupsResponse {
  pub fn new(log_groups: Vec<&str>, next_token: Option<String>) -> DescribeLogGroupsResponse {
    DescribeLogGroupsResponse {
      log_groups: log_groups
        .iter()
        .map(|g| LogGroup {
          log_group_name: g.to_string(),
        })
        .collect(),
      next_token: next_token,
    }
  }
}

#[derive(Debug, Serialize)]
pub struct LogStream {
  #[serde(rename = "logStreamName")]
  pub log_stream_name: String,
}

#[derive(Debug, Serialize)]
pub struct DescribeLogStreamsResponse {
  #[serde(rename = "logStreams")]
  pub log_streams: Vec<LogStream>,
  #[serde(rename = "nextToken", skip_serializing_if = "Option::is_none")]
  pub next_token: Option<String>,
}

impl DescribeLogStreamsResponse {
  pub fn new(log_streams: Vec<&str>, next_token: Option<String>) -> DescribeLogStreamsResponse {
    DescribeLogStreamsResponse {
      log_streams: log_streams
        .iter()
        .map(|s| LogStream {
          log_stream_name: s.to_string(),
        })
        .collect(),
      next_token: next_token,
    }
  }
}

#[derive(Debug, Serialize)]
pub struct PutLogEventsResponse {
  #[serde(rename = "nextSequenceToken")]
  pub next_sequence_token: String,
}

impl PutLogEventsResponse {
  pub fn new(next_sequence_token: &str) -> PutLogEventsResponse {
    PutLogEventsResponse {
      next_sequence_token: next_sequence_token.to_string(),
    }
  }
}
