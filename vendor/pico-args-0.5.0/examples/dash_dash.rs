#![allow(dead_code)]

use std::ffi::OsString;

#[derive(Debug)]
struct Args {
    forwarded_args: Vec<OsString>,
    help: bool,
}

fn parse_args() -> Result<Args, pico_args::Error> {
    // `from_vec` takes `OsString`, not `String`.
    let mut args: Vec<_> = std::env::args_os().collect();
    args.remove(0); // remove the executable path.

    // Find and process `--`.
    let forwarded_args = if let Some(dash_dash) = args.iter().position(|arg| arg == "--") {
        // Store all arguments following ...
        let later_args = args.drain(dash_dash+1..).collect();
        // .. then remove the `--`
        args.pop();
        later_args
    } else {
        Vec::new()
    };

    // Now pass the remaining arguments through to `pico_args`.
    let mut args = pico_args::Arguments::from_vec(args);
    let res = Args {
        forwarded_args,
        help: args.contains(["-h", "--help"]),
    };

    // It's up to the caller what to do with the remaining arguments.
    let remaining = args.finish();
    if !remaining.is_empty() {
        eprintln!("Warning: unused arguments left: {:?}", remaining);
    }

    Ok(res)
}

fn main() {
    match parse_args() {
        Ok(args) => println!("{:#?}", args),
        Err(err) => eprintln!("{}", err),
    }
}
