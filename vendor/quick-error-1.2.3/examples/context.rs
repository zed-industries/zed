#[macro_use(quick_error)] extern crate quick_error;

use std::io::{self, stderr, Read, Write};
use std::fs::File;
use std::env;
use std::num::ParseIntError;
use std::path::{Path, PathBuf};

use quick_error::ResultExt;

quick_error! {
    #[derive(Debug)]
    pub enum Error {
        NoFileName {
            description("no file name specified")
        }
        Io(err: io::Error, path: PathBuf) {
            display("could not read file {:?}: {}", path, err)
            context(path: &'a Path, err: io::Error)
                -> (err, path.to_path_buf())
        }
        Parse(err: ParseIntError, path: PathBuf) {
            display("could not parse file {:?}: {}", path, err)
            context(path: &'a Path, err: ParseIntError)
                -> (err, path.to_path_buf())
        }
    }
}

fn parse_file() -> Result<u64, Error> {
    let fname = try!(env::args().skip(1).next().ok_or(Error::NoFileName));
    let fname = Path::new(&fname);
    let mut file = try!(File::open(fname).context(fname));
    let mut buf = String::new();
    try!(file.read_to_string(&mut buf).context(fname));
    Ok(try!(buf.parse().context(fname)))
}

fn main() {
    match parse_file() {
        Ok(val) => {
            println!("Read: {}", val);
        }
        Err(e) => {
            writeln!(&mut stderr(), "Error: {}", e).ok();
        }
    }
}
