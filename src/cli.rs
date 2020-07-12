extern crate getopts;

use std::env;
use std::process;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

#[derive(Debug)]
pub struct Options {
  pub log_group_name: String,
  pub log_stream_name: String,
}

fn print_usage(program: &str, opts: getopts::Options) {
  let brief = format!("Usage: {} [options]", program);
  print!("{}", opts.usage(&brief));
}

pub fn parse_opts() -> Options {
  let args: Vec<String> = env::args().collect();
  let program = &args[0];
  let mut opts = getopts::Options::new();

  opts.optopt("g", "log-group-name", "log group name", "NAME");
  opts.optopt("s", "log-stream-name", "log stream name", "NAME");
  opts.optflag("v", "version", "print version and exit");
  opts.optflag("h", "help", "print usage and exit");

  let matches = opts.parse(&args[1..]).unwrap();

  if matches.opt_present("h") {
    print_usage(&program, opts);
    process::exit(0)
  }

  if matches.opt_present("v") {
    println!("{}", VERSION);
    process::exit(0)
  }

  Options {
    log_group_name: matches.opt_str("g").unwrap(),
    log_stream_name: matches.opt_str("s").unwrap(),
  }
}
