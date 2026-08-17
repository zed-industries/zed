use zstd;

use std::env;
use std::fs;
use std::io;

const SUFFIX: &'static str = ".zst";

fn main() {
    for arg in env::args().skip(1) {
        if arg.ends_with(SUFFIX) {
            match decompress(&arg) {
                Ok(()) => println!("Decompressed {}", arg),
                Err(e) => println!("Error decompressing {}: {}", arg, e),
            }
        } else {
            match compress(&arg) {
                Ok(()) => println!("Compressed {}", arg),
                Err(e) => println!("Error compressing {}: {}", arg, e),
            }
        }
    }
}

fn compress(source: &str) -> io::Result<()> {
    let mut file = fs::File::open(source)?;
    let mut encoder = {
        let target = fs::File::create(source.to_string() + SUFFIX)?;
        zstd::Encoder::new(target, 1)?
    };

    io::copy(&mut file, &mut encoder)?;
    encoder.finish()?;

    Ok(())
}

fn decompress(source: &str) -> io::Result<()> {
    let mut decoder = {
        let file = fs::File::open(source)?;
        zstd::Decoder::new(file)?
    };

    let mut target = fs::File::create(source.trim_end_matches(SUFFIX))?;

    io::copy(&mut decoder, &mut target)?;

    Ok(())
}
