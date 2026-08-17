mod decoder;
mod encoder;

#[derive(Debug)]
pub enum Xz2FileFormat {
    Xz,
    Lzma,
}

pub use self::{decoder::Xz2Decoder, encoder::Xz2Encoder};
