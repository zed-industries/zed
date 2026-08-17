use ammonia::Builder;
use std::env;
use std::fs::File;
use std::io::{self, Read, Write};
use std::process;

fn run() -> io::Result<()> {
    let input = env::args().nth(1).unwrap_or_else(|| String::from("-"));
    let output = env::args().nth(2).unwrap_or_else(|| String::from("-"));

    let mut rdr: Box<dyn Read> = if input == "-" {
        Box::new(io::stdin())
    } else {
        Box::new(File::open(input)?)
    };

    let mut wrt: Box<dyn Write> = if output == "-" {
        Box::new(io::stdout())
    } else {
        Box::new(File::create(output)?)
    };

    Builder::new()
        .clean_from_reader(&mut rdr)?
        .write_to(&mut wrt)?;
    Ok(())
}

fn main() {
    env_logger::init();
    if let Err(ref e) = run() {
        eprintln!("error: {e}");
        process::exit(1);
    }
}
