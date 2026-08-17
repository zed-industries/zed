#[cfg(feature = "aesprite")]
pub mod aesprite;
#[cfg(feature = "bmp")]
pub mod bmp;
#[cfg(feature = "dds")]
pub mod dds;
#[cfg(feature = "exr")]
pub mod exr;
#[cfg(feature = "farbfeld")]
pub mod farbfeld;
#[cfg(feature = "gif")]
pub mod gif;
#[cfg(feature = "hdr")]
pub mod hdr;
#[cfg(feature = "ico")]
pub mod ico;
#[cfg(feature = "ilbm")]
pub mod ilbm;
#[cfg(feature = "jpeg")]
pub mod jpeg;
#[cfg(feature = "jxl")]
pub mod jxl;
#[cfg(feature = "ktx2")]
pub mod ktx2;
#[cfg(feature = "png")]
pub mod png;
#[cfg(feature = "pnm")]
pub mod pnm;
#[cfg(feature = "psd")]
pub mod psd;
#[cfg(feature = "qoi")]
pub mod qoi;
#[cfg(feature = "tga")]
pub mod tga;
#[cfg(feature = "tiff")]
pub mod tiff;
#[cfg(feature = "vtf")]
pub mod vtf;
#[cfg(feature = "webp")]
pub mod webp;

use crate::{container, ImageError, ImageResult, ImageType};
use std::io::{BufRead, Seek};

pub fn image_type<R: BufRead + Seek>(reader: &mut R) -> ImageResult<ImageType> {
    let mut header = [0; 12];
    reader.read_exact(&mut header)?;

    // Currently there are no formats where 1 byte is enough to determine format
    if header.len() < 2 {
        return Err(
            std::io::Error::new(std::io::ErrorKind::UnexpectedEof, "Not enough data").into(),
        );
    }

    // This is vaguely organized in what I assume are the most commonly used formats.
    // I don't know how much this matters for actual execution time.
    #[cfg(feature = "jpeg")]
    if jpeg::matches(&header) {
        return Ok(ImageType::Jpeg);
    }

    #[cfg(feature = "png")]
    if png::matches(&header) {
        return Ok(ImageType::Png);
    }

    #[cfg(feature = "gif")]
    if gif::matches(&header) {
        return Ok(ImageType::Gif);
    }

    #[cfg(feature = "tiff")]
    if tiff::matches(&header) {
        return Ok(ImageType::Tiff);
    }

    #[cfg(feature = "webp")]
    if webp::matches(&header) {
        return Ok(ImageType::Webp);
    }

    #[cfg(feature = "heif")]
    if let Some(c) = container::heif::matches(&header, reader) {
        return Ok(ImageType::Heif(c));
    }

    #[cfg(feature = "jxl")]
    if jxl::matches(&header) {
        return Ok(ImageType::Jxl);
    }

    #[cfg(feature = "bmp")]
    if bmp::matches(&header) {
        return Ok(ImageType::Bmp);
    }

    #[cfg(feature = "psd")]
    if psd::matches(&header) {
        return Ok(ImageType::Psd);
    }

    #[cfg(feature = "ico")]
    if ico::matches(&header) {
        return Ok(ImageType::Ico);
    }

    #[cfg(feature = "aesprite")]
    if aesprite::matches(&header) {
        return Ok(ImageType::Aseprite);
    }

    #[cfg(feature = "exr")]
    if exr::matches(&header) {
        return Ok(ImageType::Exr);
    }

    #[cfg(feature = "hdr")]
    if hdr::matches(&header) {
        return Ok(ImageType::Hdr);
    }

    #[cfg(feature = "dds")]
    if dds::matches(&header) {
        return Ok(ImageType::Dds);
    }

    #[cfg(feature = "ktx2")]
    if ktx2::matches(&header) {
        return Ok(ImageType::Ktx2);
    }

    #[cfg(feature = "qoi")]
    if qoi::matches(&header) {
        return Ok(ImageType::Qoi);
    }

    #[cfg(feature = "farbfeld")]
    if farbfeld::matches(&header) {
        return Ok(ImageType::Farbfeld);
    }

    #[cfg(feature = "pnm")]
    if pnm::matches(&header) {
        return Ok(ImageType::Pnm);
    }

    #[cfg(feature = "vtf")]
    if vtf::matches(&header) {
        return Ok(ImageType::Vtf);
    }

    #[cfg(feature = "ilbm")]
    if ilbm::matches(&header) {
        return Ok(ImageType::Ilbm);
    }

    // Keep TGA last because it has the highest probability of false positives
    #[cfg(feature = "tga")]
    if tga::matches(&header, reader) {
        return Ok(ImageType::Tga);
    }

    Err(ImageError::NotSupported)
}
