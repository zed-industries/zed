use std::io::{stdin, IsTerminal, Read};

use anyhow::Context;
use human_bytes::human_bytes;
use lexopt::prelude::*;

const fn usage() -> &'static str {
    "Usage: hb [options] <bytes>
    -h, --help       Print this help message
    -V, --version    Print version info"
}

fn main() -> anyhow::Result<()> {
    let mut bytes: Option<u64> = None;

    if stdin().is_terminal() {
        let mut cli = lexopt::Parser::from_env();
        while let Some(arg) = cli.next()? {
            match arg {
                Value(val) if bytes.is_none() => {
                    bytes = Some(val.to_string_lossy().parse()?);
                }
                Short('h') | Long("help") => {
                    println!("{}", usage());
                    std::process::exit(0);
                }
                Short('V') | Long("version") => {
                    println!("{} v{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
                    std::process::exit(0);
                }
                _ => return Err(arg.unexpected())?,
            }
        }
    } else {
        let mut tmp = String::new();
        std::io::stdin().read_to_string(&mut tmp)?;
        bytes = Some(tmp.trim().parse()?);
    }

    let bytes = bytes.context("missing <bytes> (see -h)")?;
    println!("{}", human_bytes(bytes as f64));
    Ok(())
}
