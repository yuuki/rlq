#![crate_name = "rllq"]

extern crate getopts;
#[macro_use]
extern crate log;

use std::env;
use std::process::exit;
use std::io::{self, Write};
use getopts::Options;

extern crate rllq;
use rllq::config::*;
use rllq::ltsv;
use rllq::error::CliError;

#[macro_use]
pub mod error;

fn main() {
    let (args, config) = parse_config(env::args().collect());
    if config.query_list {
        let ret = match do_list(args) {
            Some(CliError::Other) => 3,
            Some(err) => {
                stderr!("{}", err);
                2
            }
            None => 0,
        };
        exit(ret);
    }
    exit(0);
}

fn print_usage(opts: &Options) {
    let message = format!("Usage: rllq [ options ... ] [URL]\n\twhere options include");
    println!("{}", opts.usage(&message));
}

fn args_fail(msg: &str) -> ! {
    stderr!("{}", msg);
    exit(1)
}

fn parse_config(args: Vec<String>) -> (Vec<String>, Config) {
    let mut opts = Options::new();
    opts.optflag("l", "list", "list LTSV labels");
    opts.optflag("h", "help", "show this message");

    let (_, args) = args.split_first().unwrap();
    if args.len() == 0 || args[0] == "-h" || args[0] == "--help" {
        print_usage(&opts);
        exit(0);
    }

    let opt_match = match opts.parse(args) {
        Ok(m) => m,
        Err(f) => args_fail(&f.to_string()),
    };

    if opt_match.opt_present("h") || opt_match.opt_present("help") {
        print_usage(&opts);
        exit(0);
    }

    (opt_match.free.clone(), Config { query_list: opt_match.opt_present("list") })
}

fn do_list(args: Vec<String>) -> Option<CliError> {
    if args.len() == 0 {
        return Some(CliError::NotEnoughArgs);
    }
    if args.len() == 2 {
        return Some(CliError::TooManyArgs);
    }
    match ltsv::open_file(args[0].as_ref()) {
        Err(err) => {
            stderr!("failed to open file: {}", err);
            return Some(CliError::Other);
        }
        Ok(mut f) => {
            match ltsv::parse_head(&mut f) {
                Err(err) => {
                    stderr!("failed to parse head: {:?}", err);
                    return Some(CliError::Other);
                }
                Ok(items) => {
                    for (k, _) in &items {
                        println!("{}", k)
                    }
                    return None;
                }
            }
        }
    }
}
