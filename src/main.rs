mod cli;
mod lee;

use cli::parse_opts;
use lee::tee;
use rusoto_core::Region;
use rusoto_logs::CloudWatchLogsClient;
use std::io;

#[tokio::main]
async fn main() {
  let opts = parse_opts();
  let client = CloudWatchLogsClient::new(Region::default());
  let sin = io::stdin();
  let reader = io::BufReader::new(sin.lock());
  let out = io::stdout();
  let mut out = out.lock();

  tee(
    &client,
    &opts.log_group_name,
    &opts.log_stream_name,
    reader,
    &mut out,
  )
  .await
  .unwrap()
}
