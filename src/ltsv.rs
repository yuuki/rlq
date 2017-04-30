use std::io::{self, BufRead, BufReader, Stdin, stdin};
use std::fs::File;
use std::path::Path;
use std::collections::HashMap;

use error::*;

pub type Record = HashMap<String, String>;

pub enum LineReader {
    Stdin(Stdin),
    FileIn(BufReader<File>),
}

impl LineReader {
    fn read_line(&mut self, buf: &mut String) -> io::Result<usize> {
        match *self {
            LineReader::Stdin(ref mut r) => r.read_line(buf),
            LineReader::FileIn(ref mut r) => r.read_line(buf),
        }
    }
}

pub fn open_file(name: &str) -> Result<LineReader, Error> {
    match name {
        "-" => Ok(LineReader::Stdin(stdin())),
        _ => {
            let f = try!(File::open(&Path::new(name)));
            Ok(LineReader::FileIn(BufReader::new(f)))
        }
    }
}

pub fn parse_head(input: &mut LineReader) -> Result<Record, Error> {
    let found: String;
    loop {
        let mut line = String::new();
        match input.read_line(&mut line).map_err(Error::Io) {
            Ok(0) | Ok(1) => continue,
            Ok(_) => {
                found = line;
                break;
            }
            Err(err) => return Err(err),
        }
    }

    let mut record = Record::new();
    for item in found.split('\t').collect::<Vec<&str>>().into_iter() {
        let v = item.splitn(2, ':').collect::<Vec<&str>>();
        match v.len() {
            0 | 1 => {
                return Err(ParseError { msg: format!("invalid ltsv item: {}", item) })
                    .map_err(Error::Parse);
            }
            2 => record.insert(v[0].to_string(), v[1].to_string()),
            _ => {
                return Err(ParseError { msg: format!("unreachable error: {}", item) })
                    .map_err(Error::Parse);
            }
        };
    }

    Ok(record)
}

#[cfg(test)]
mod test {
    #[test]
    fn test_parse_head() {}
}
