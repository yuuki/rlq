#![crate_name = "rlq"]

extern crate getopts;
extern crate log;

use std::env;
use std::process::exit;
use std::io::{self, Write};
use getopts::Options;

extern crate rlq;
use rlq::config::*;
use rlq::ltsv;
use rlq::error::CliError;

#[macro_use]
pub mod error;

fn main() {
    exit(run(env::args().collect()));
}

fn run(args: Vec<String>) -> i32 {
    let (args, config) = parse_config(args);
    let ret = if config.query_list {
        do_list(args)
    } else if config.query_select.len() > 0 {
        do_select(args, config.query_select)
    } else if config.query_groupby != "" {
        do_groupby(args, config.query_groupby)
    } else if config.query_orderby != "" {
        do_orderby(args, config.query_orderby)
    } else {
        None
    };
    match ret {
        Some(CliError::Other) => 3,
        Some(err) => {
            stderr!("{}", err);
            2
        }
        None => 0,
    }
}

fn print_usage(opts: &Options) {
    let message = format!("Usage: rlq [ options ... ] [URL]\n\twhere options include");
    println!("{}", opts.usage(&message));
}

fn args_fail(msg: &str) -> ! {
    stderr!("{}", msg);
    exit(1)
}

fn parse_config(args: Vec<String>) -> (Vec<String>, Config) {
    let mut opts = Options::new();
    opts.optflag("l", "list", "list LTSV labels");
    opts.optmulti("s", "select", "select fields by specified labels", "LABEL");
    opts.optopt("g",
                "groupby",
                "group element by specified labels (the default aggregation method: 'count')",
                "LABEL");
    opts.optopt("o",
                "orderby",
                "prder record by specified labels (the default order: 'asc')",
                "LABEL");
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

    (opt_match.free.clone(),
     Config {
         query_list: opt_match.opt_present("list"),
         query_select: opt_match.opt_strs("select"),
         query_groupby: opt_match.opt_str("groupby").unwrap_or_default(),
         query_orderby: opt_match.opt_str("orderby").unwrap_or_default(),
     })
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
                    stderr!("failed to parse head: {}", err);
                    return Some(CliError::Other);
                }
                Ok(record) => {
                    for (label, _) in &record {
                        println!("{}", label)
                    }
                    return None;
                }
            }
        }
    }
}

fn do_select(args: Vec<String>, arg_labels: Vec<String>) -> Option<CliError> {
    if args.len() == 0 {
        return Some(CliError::NotEnoughArgs);
    }
    if args.len() == 2 {
        return Some(CliError::TooManyArgs);
    }
    match ltsv::open_file(args[0].as_ref()) {
        Err(err) => {
            stderr!("failed to open file: {}", err);
            Some(CliError::Other)
        }
        Ok(mut f) => {
            // Validate that each label exists on the target data.
            match ltsv::parse_head(&mut f) {
                Err(err) => {
                    stderr!("failed to parse head: {}", err);
                    return Some(CliError::Other);
                }
                Ok(fields) => {
                    let labels = fields.keys().collect::<Vec<&String>>();
                    for arg_label in &arg_labels {
                        if !labels.contains(&arg_label) {
                            stderr!("no such label: {}", arg_label);
                            return Some(CliError::Other);
                        }
                    }
                }
            }

            let printer = |record: &ltsv::Record| {
                let line = arg_labels.iter()
                    .map(|label| {
                        match record.get(label) {
                            Some(r) => format!("{}:{}", label, r),
                            None => "".to_string(), // TODO
                        }
                    })
                    .collect::<Vec<String>>()
                    .join("\t");
                println!("{}", line);
            };
            match ltsv::each_record(&mut f, printer) {
                Err(err) => {
                    stderr!("failed to print each record: {}", err);
                    Some(CliError::Other)
                }
                Ok(_) => None,
            }
        }
    }
}

fn do_groupby(args: Vec<String>, arg_label: String) -> Option<CliError> {
    if args.len() == 0 {
        return Some(CliError::NotEnoughArgs);
    }
    if args.len() == 2 {
        return Some(CliError::TooManyArgs);
    }
    match ltsv::open_file(args[0].as_ref()) {
        Err(err) => {
            stderr!("failed to open file: {}", err);
            Some(CliError::Other)
        }
        Ok(mut f) => {
            match ltsv::group_by(&mut f, &arg_label) {
                Err(err) => {
                    stderr!("failed to group by {}: {}", arg_label, err);
                    Some(CliError::Other)
                }
                Ok(group) => {
                    for (label_value, count) in group {
                        println!("{}:{}\tcount:{}", arg_label, label_value, count);
                    }
                    None
                }
            }
        }
    }
}

fn do_orderby(args: Vec<String>, arg_label: String) -> Option<CliError> {
    if args.len() == 0 {
        return Some(CliError::NotEnoughArgs);
    }
    if args.len() == 2 {
        return Some(CliError::TooManyArgs);
    }
    match ltsv::open_file(args[0].as_ref()) {
        Err(err) => {
            stderr!("failed to open file: {}", err);
            Some(CliError::Other)
        }
        Ok(mut f) => {
            match ltsv::order_by(&mut f, &arg_label) {
                Err(err) => {
                    stderr!("failed to order by {}: {}", arg_label, err);
                    Some(CliError::Other)
                }
                Ok(lines) => {
                    for line in lines {
                        print!("{}", line);
                    }
                    None
                }
            }
        }
    }
}
