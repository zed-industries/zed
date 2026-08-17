use clap::{App, Arg};
use std::io;

// This program trains a dictionary from one or more files,
// to make future compression of similar small files more efficient.
//
// The dictionary will need to be present during decompression,
// but if you need to compress many small files individually,
// it may be worth the trouble.
fn main() {
    let matches = App::new("train")
        .author("Alexandre Bury <alexandre.bury@gmail.com>")
        .about("A zstd dict trainer")
        .arg(
            Arg::new("MAX_SIZE")
                .help("Maximum dictionary size in bytes")
                .short('s')
                .long("max_size")
                .takes_value(true),
        )
        .arg(
            Arg::new("FILE")
                .help("Files to use as input")
                .required(true)
                .multiple_occurrences(true),
        )
        .get_matches();

    let size: usize = matches.value_of_t("MAX_SIZE").unwrap_or(110 * 1024);

    let files: Vec<_> = matches.values_of("FILE").unwrap().collect();

    let dict = zstd::dict::from_files(&files, size).unwrap();

    let mut dict_reader: &[u8] = &dict;
    io::copy(&mut dict_reader, &mut io::stdout()).unwrap();
}
