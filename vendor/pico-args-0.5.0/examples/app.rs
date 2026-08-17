#![allow(dead_code)]

const HELP: &str = "\
App

USAGE:
  app [OPTIONS] --number NUMBER [INPUT]

FLAGS:
  -h, --help            Prints help information

OPTIONS:
  --number NUMBER       Sets a number
  --opt-number NUMBER   Sets an optional number
  --width WIDTH         Sets width [default: 10]
  --output PATH         Sets an output path

ARGS:
  <INPUT>
";

#[derive(Debug)]
struct AppArgs {
    number: u32,
    opt_number: Option<u32>,
    width: u32,
    input: std::path::PathBuf,
    output: Option<std::path::PathBuf>,
}

fn main() {
    let args = match parse_args() {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Error: {}.", e);
            std::process::exit(1);
        }
    };

    println!("{:#?}", args);
}

fn parse_args() -> Result<AppArgs, pico_args::Error> {
    let mut pargs = pico_args::Arguments::from_env();

    // Help has a higher priority and should be handled separately.
    if pargs.contains(["-h", "--help"]) {
        print!("{}", HELP);
        std::process::exit(0);
    }

    let args = AppArgs {
        // Parses a required value that implements `FromStr`.
        // Returns an error if not present.
        number: pargs.value_from_str("--number")?,
        // Parses an optional value that implements `FromStr`.
        opt_number: pargs.opt_value_from_str("--opt-number")?,
        // Parses an optional value from `&str` using a specified function.
        width: pargs.opt_value_from_fn("--width", parse_width)?.unwrap_or(10),
        // Parses an optional value from `&OsStr` using a specified function.
        output: pargs.opt_value_from_os_str("--input", parse_path)?,
        // Parses a required free-standing/positional argument.
        input: pargs.free_from_str()?,
    };

    // It's up to the caller what to do with the remaining arguments.
    let remaining = pargs.finish();
    if !remaining.is_empty() {
        eprintln!("Warning: unused arguments left: {:?}.", remaining);
    }

    Ok(args)
}

fn parse_width(s: &str) -> Result<u32, &'static str> {
    s.parse().map_err(|_| "not a number")
}

fn parse_path(s: &std::ffi::OsStr) -> Result<std::path::PathBuf, &'static str> {
    Ok(s.into())
}
