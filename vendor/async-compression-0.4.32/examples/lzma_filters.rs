//! Run this example with the following command in a terminal:
//!
//! ```console
//! $ echo -n 'example' | zstd | cargo run --example zstd_gzip --features="tokio,zstd,gzip" | gunzip -c
//! 7example
//! ```
//!
//! Note that the "7" prefix (input length) is printed to stdout but will likely show up as shown
//! above. This is not an encoding error; see the code in `main`.

use std::convert::TryFrom;
use std::io::Result;

use compression_codecs::lzma::params::{
    LzmaDecoderParams, LzmaEncoderParams, LzmaFilter, LzmaFilters, LzmaOptions,
};
use tokio::io::{stdin, stdout, BufReader, Stdin, Stdout};
use tokio::io::{
    AsyncReadExt as _,  // for `read_to_end`
    AsyncWriteExt as _, // for `write_all` and `shutdown`
};

const OPTIONS: &str = r"
Usage: lzma_filter [OPTIONS]

    Read stdin and output result to stdout

    Options:
                -d   Decompress (Default)
                -c   Compress
";

fn usage(msg: &str) -> Result<()> {
    if !msg.is_empty() {
        eprintln!("{msg}...")
    }

    eprintln!("{}", OPTIONS);
    std::process::exit(1);
}

/// Read compressed and output decompressed or
/// Read decompressed and output compressed
/// echo "this is a test" | lzma_filters -- -c |  lzma_filters -- -d | xxd
#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        return usage("too many args");
    }

    let stdin = stdin();
    let stdout = stdout();
    let filters =
        LzmaFilters::default().add_filter(LzmaFilter::Lzma2(LzmaOptions::default().preset(7)));
    let mut stdout = if args[1] == "-d" {
        decompress(stdin, filters, stdout).await?
    } else if args[1] == "-c" {
        compress(stdin, filters, stdout).await?
    } else {
        return usage(&format!("invalid argument: {}", args[1]));
    };

    stdout.flush().await?;
    stdout.shutdown().await?;

    Ok(())
}

async fn decompress(stdin: Stdin, filters: LzmaFilters, mut stdout: Stdout) -> Result<Stdout> {
    let params = LzmaDecoderParams::Raw { filters };
    let codec = compression_codecs::Xz2Decoder::try_from(params).expect("Could not create encoder");
    let codec = compression_codecs::LzmaDecoder::from(codec);
    let mut reader =
        async_compression::tokio::bufread::LzmaDecoder::with_codec(BufReader::new(stdin), codec);

    let mut buf = vec![];
    reader.read_to_end(&mut buf).await?;
    stdout.write_all(&buf).await?;

    Ok(stdout)
}
async fn compress(mut stdin: Stdin, filters: LzmaFilters, stdout: Stdout) -> Result<Stdout> {
    let params = LzmaEncoderParams::Raw { filters };
    let codec = compression_codecs::Xz2Encoder::try_from(params).expect("Could not create encoder");
    let codec = compression_codecs::LzmaEncoder::from(codec);
    let mut writer = async_compression::tokio::write::LzmaEncoder::with_codec(stdout, codec);

    let mut buf = vec![];
    stdin.read_to_end(&mut buf).await?;
    writer.write_all(&buf).await?;
    writer.shutdown().await?;

    Ok(writer.into_inner())
}
