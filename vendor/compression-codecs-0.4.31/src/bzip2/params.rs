use std::convert::TryInto;

use bzip2::Compression;
use compression_core::Level;

#[derive(Debug)]
pub struct Bzip2EncoderParams {
    inner: Compression,
}

impl From<Bzip2EncoderParams> for Compression {
    fn from(value: Bzip2EncoderParams) -> Self {
        value.inner
    }
}
impl From<Level> for Bzip2EncoderParams {
    fn from(value: Level) -> Self {
        let fastest = bzip2::Compression::fast();
        let best = bzip2::Compression::best();

        let inner = match value {
            Level::Fastest => fastest,
            Level::Best => best,
            Level::Precise(quality) => bzip2::Compression::new(
                quality
                    .try_into()
                    .unwrap_or(0)
                    .clamp(fastest.level(), best.level()),
            ),
            _ => bzip2::Compression::default(),
        };
        Self { inner }
    }
}
