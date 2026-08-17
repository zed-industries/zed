use compression_codecs::brotli::params::EncoderParams;

#[macro_use]
mod utils;

test_cases!(brotli);

#[test]
pub fn brotli_params() {
    let _ = EncoderParams::default();
}
