use clap::{App, Arg};
use std::fs;
use std::io;

fn main() {
    // This will be a simple application:
    // takes a single (repeatable and optional) argument.
    let matches = App::new("zstdcat")
        .version("0.1")
        .author("Yann Collet (zstd), Alexandre Bury (zstd-rs)")
        .about("Decompress FILEs to standard output.")
        .arg(Arg::new("FILE").index(1).multiple_occurrences(true).help(
            "Files to decompress. With no file, or when given -, \
             read standard input.",
        ))
        .get_matches();

    // If nothign was given, act as if `-` was there.
    match matches.values_of("FILE") {
        None => decompress_file("-").unwrap(),
        Some(files) => {
            // Decompress each file sequentially.
            for file in files {
                decompress_file(file).unwrap();
            }
        }
    }
}

// Dispatch the source reader depending on the filename
fn decompress_file(file: &str) -> io::Result<()> {
    match file {
        "-" => decompress_from(io::stdin()),
        other => decompress_from(io::BufReader::new(fs::File::open(other)?)),
    }
}

// Decompress from a `Reader` into stdout
fn decompress_from<R: io::Read>(r: R) -> io::Result<()> {
    let mut decoder = zstd::Decoder::new(r)?;
    io::copy(&mut decoder, &mut io::stdout())?;
    Ok(())
}
