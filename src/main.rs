#![crate_name = "rllq"]

extern crate getopts;
extern crate log;

use std::env;
use std::process;
use std::io::{self, Write};
use getopts::Options;
use config::Config;

pub mod config;

fn main() {
    let config = parse_config(env::args().collect());
}

fn print_usage(opts: &Options) {
    let message = format!("Usage: rllq [ options ... ] [URL]\n\twhere options include");
    println!("{}", opts.usage(&message));
}

fn args_fail(msg: &str) -> ! {
    writeln!(io::stderr(), "{}", msg).unwrap();
    process::exit(1)
}

pub fn parse_config(args: Vec<String>) -> Config {
    let mut opts = Options::new();
    opts.optflag("l", "list", "list LTSV keys");
    opts.optflag("h", "help", "show this message");

    let (_, args) = args.split_first().unwrap();
    if args.len() == 0 || args[0] == "-h" || args[0] == "--help" {
        print_usage(&opts)
    }

    let opt_match = match opts.parse(args) {
        Ok(m) => m,
        Err(f) => args_fail(&f.to_string()),
    };

    if opt_match.opt_present("h") || opt_match.opt_present("help") {
        print_usage(&opts);
        process::exit(0);
    }

    Config { query_list: opt_match.opt_present("list") }
}

pub fn do_list(config: &Config) {}
