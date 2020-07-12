use super::mock::{DescribeLogGroupsResponse, DescribeLogStreamsResponse, PutLogEventsResponse};
use crate::tee;
use rusoto_core::Region;
use rusoto_logs::CloudWatchLogsClient;
use rusoto_mock::{MockCredentialsProvider, MockRequestDispatcher, MultipleMockRequestDispatcher};

#[tokio::test]
async fn test_tee() {
  let mut buf = vec![];
  let reader = b"foo\nbar\nzoo" as &[u8];

  let responses = vec![
    MockRequestDispatcher::default()
      .with_json_body(DescribeLogGroupsResponse::new(&["log_group_name"], None)),
    MockRequestDispatcher::default()
      .with_json_body(DescribeLogStreamsResponse::new(&["log_stream_name"], None)),
    MockRequestDispatcher::default()
      .with_json_body(PutLogEventsResponse::new("next_sequence_token")),
    MockRequestDispatcher::default()
      .with_json_body(PutLogEventsResponse::new("next_sequence_token")),
    MockRequestDispatcher::default()
      .with_json_body(PutLogEventsResponse::new("next_sequence_token")),
  ];

  let client = CloudWatchLogsClient::new_with(
    MultipleMockRequestDispatcher::new(responses),
    MockCredentialsProvider,
    Region::UsEast1,
  );

  tee(
    &client,
    "log_group_name",
    "log_stream_name",
    reader,
    &mut buf,
  )
  .await
  .unwrap();

  assert_eq!(String::from_utf8(buf).unwrap(), "foo\nbar\nzoo\n");
}

#[tokio::test]
async fn test_tee_with_create_group() {
  let mut buf = vec![];
  let reader = b"foo\nbar\nzoo" as &[u8];

  let responses = vec![
    MockRequestDispatcher::default().with_json_body(DescribeLogGroupsResponse::new(
      &["log_group_name_not_exist"],
      None,
    )),
    MockRequestDispatcher::default(), // CreateLogGroup response
    MockRequestDispatcher::default()
      .with_json_body(DescribeLogStreamsResponse::new(&["log_stream_name"], None)),
    MockRequestDispatcher::default()
      .with_json_body(PutLogEventsResponse::new("next_sequence_token")),
    MockRequestDispatcher::default()
      .with_json_body(PutLogEventsResponse::new("next_sequence_token")),
    MockRequestDispatcher::default()
      .with_json_body(PutLogEventsResponse::new("next_sequence_token")),
  ];

  let client = CloudWatchLogsClient::new_with(
    MultipleMockRequestDispatcher::new(responses),
    MockCredentialsProvider,
    Region::UsEast1,
  );

  tee(
    &client,
    "log_group_name",
    "log_stream_name",
    reader,
    &mut buf,
  )
  .await
  .unwrap();

  assert_eq!(String::from_utf8(buf).unwrap(), "foo\nbar\nzoo\n");
}

#[tokio::test]
async fn test_tee_with_create_stream() {
  let mut buf = vec![];
  let reader = b"foo\nbar\nzoo" as &[u8];

  let responses = vec![
    MockRequestDispatcher::default()
      .with_json_body(DescribeLogGroupsResponse::new(&["log_group_name"], None)),
    MockRequestDispatcher::default().with_json_body(DescribeLogStreamsResponse::new(
      &["log_stream_name_not_exist"],
      None,
    )),
    MockRequestDispatcher::default(), // CreateLogStream response
    MockRequestDispatcher::default()
      .with_json_body(PutLogEventsResponse::new("next_sequence_token")),
    MockRequestDispatcher::default()
      .with_json_body(PutLogEventsResponse::new("next_sequence_token")),
    MockRequestDispatcher::default()
      .with_json_body(PutLogEventsResponse::new("next_sequence_token")),
  ];

  let client = CloudWatchLogsClient::new_with(
    MultipleMockRequestDispatcher::new(responses),
    MockCredentialsProvider,
    Region::UsEast1,
  );

  tee(
    &client,
    "log_group_name",
    "log_stream_name",
    reader,
    &mut buf,
  )
  .await
  .unwrap();

  assert_eq!(String::from_utf8(buf).unwrap(), "foo\nbar\nzoo\n");
}
