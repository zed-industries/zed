use humansize::{file_size_opts, FileSize};
use std::io::Read;

fn main() {
    let matches = clap::App::new("benchmark")
        .author("Alexandre Bury <alexandre.bury@gmail.com>")
        .about("Benchmark zstd-rs")
        .arg(
            clap::Arg::new("DIR")
                .help(
                    "Directory containing the data to compress.

To use the silesia corpus, run the following commands:

wget http://sun.aei.polsl.pl/~sdeor/corpus/silesia.zip
unzip silesia.zip -d silesia/
cargo run --example benchmark -- silesia/",
                )
                .required(true),
        )
        .arg(
            clap::Arg::new("begin")
                .short('b')
                .long("begin")
                .takes_value(true),
        )
        .arg(
            clap::Arg::new("end")
                .short('e')
                .long("end")
                .takes_value(true),
        )
        .get_matches();

    let dir = matches.value_of("DIR").unwrap();
    let begin: i32 = matches.value_of_t("begin").unwrap_or(1);
    let end: i32 = matches.value_of_t("end").unwrap_or(10);

    // Step 1: load data in memory
    let files: Vec<Vec<u8>> = std::fs::read_dir(dir)
        .unwrap()
        .map(|file| {
            let file = file.unwrap();

            let mut content = Vec::new();
            std::fs::File::open(file.path())
                .unwrap()
                .read_to_end(&mut content)
                .unwrap();
            content
        })
        .collect();

    let total_size: usize = files.iter().map(|data| data.len()).sum();

    // Step 3: compress data

    // Print tsv headers
    println!(
        "{}\t{}\t{}\t{}",
        "Compression level",
        "Compression ratio",
        "Compression speed",
        "Decompression speed"
    );

    for level in begin..end {
        // Compress each sample sequentially.
        let start = std::time::Instant::now();

        let compressed: Vec<Vec<u8>> = files
            .iter()
            .map(|data| zstd::encode_all(&data[..], level).unwrap())
            .collect();
        let mid = std::time::Instant::now();

        let uncompressed: Vec<Vec<u8>> = compressed
            .iter()
            .map(|data| zstd::decode_all(&data[..]).unwrap())
            .collect();
        let end = std::time::Instant::now();

        for (original, processed) in files.iter().zip(uncompressed.iter()) {
            assert_eq!(&original[..], &processed[..]);
        }

        let compress_time = mid - start;
        let decompress_time = end - mid;

        let compress_seconds = compress_time.as_secs() as f64
            + compress_time.subsec_nanos() as f64 * 1e-9;

        let decompress_seconds = decompress_time.as_secs() as f64
            + decompress_time.subsec_nanos() as f64 * 1e-9;

        let compressed_size: usize = compressed.iter().map(Vec::len).sum();

        let speed = (total_size as f64 / compress_seconds) as usize;
        let speed = speed.file_size(file_size_opts::DECIMAL).unwrap();

        let d_speed = (total_size as f64 / decompress_seconds) as usize;
        let d_speed = d_speed.file_size(file_size_opts::DECIMAL).unwrap();

        let ratio = compressed_size as f64 / total_size as f64;
        println!("{}\t{:.3}\t{}/s\t{}/s", level, 1.0 / ratio, speed, d_speed);
    }
}
