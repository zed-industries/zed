use std::convert::TryInto;

use compression_core::Level;

#[derive(Debug, Clone)]
pub struct FlateEncoderParams {
    inner: flate2::Compression,
}

impl From<flate2::Compression> for FlateEncoderParams {
    fn from(inner: flate2::Compression) -> Self {
        Self { inner }
    }
}
impl From<FlateEncoderParams> for flate2::Compression {
    fn from(value: FlateEncoderParams) -> Self {
        value.inner
    }
}

impl From<Level> for FlateEncoderParams {
    fn from(value: Level) -> Self {
        let fastest = flate2::Compression::fast();
        let best = flate2::Compression::best();
        let none = flate2::Compression::none();

        let inner = match value {
            Level::Fastest => fastest,
            Level::Best => best,
            Level::Precise(quality) => flate2::Compression::new(
                quality
                    .try_into()
                    .unwrap_or(0)
                    .clamp(none.level(), best.level()),
            ),
            _ => flate2::Compression::default(),
        };
        Self { inner }
    }
}
