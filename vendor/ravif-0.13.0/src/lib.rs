//! ```rust
//! use ravif::*;
//! # fn doit(pixels: &[RGBA8], width: usize, height: usize) -> Result<(), Error> {
//! let res = Encoder::new()
//!     .with_quality(70.)
//!     .with_speed(4)
//!     .encode_rgba(Img::new(pixels, width, height))?;
//! std::fs::write("hello.avif", res.avif_file);
//! # Ok(()) }

mod av1encoder;

mod error;
pub use av1encoder::ColorModel;
pub use error::Error;

#[doc(hidden)]
#[deprecated = "Renamed to `ColorModel`"]
pub type ColorSpace = ColorModel;

pub use av1encoder::{AlphaColorMode, BitDepth, EncodedImage, Encoder};
#[doc(inline)]
pub use rav1e::prelude::{MatrixCoefficients, PixelRange};

mod dirtyalpha;

#[doc(no_inline)]
pub use imgref::Img;
#[doc(no_inline)]
pub use rgb::{RGB8, RGBA8};

#[cfg(not(feature = "threading"))]
mod rayoff {
    pub fn current_num_threads() -> usize {
        std::thread::available_parallelism().map(|v| v.get()).unwrap_or(1)
    }

    pub fn join<A, B>(a: impl FnOnce() -> A, b: impl FnOnce() -> B) -> (A, B) {
        (a(), b())
    }
}

#[test]
fn encode8_with_alpha() {
    let img = imgref::ImgVec::new((0..200).flat_map(|y| (0..256).map(move |x| {
        RGBA8::new(x as u8, y as u8, 255, (x + y) as u8)
    })).collect(), 256, 200);

    let enc = Encoder::new()
        .with_quality(22.0)
        .with_bit_depth(BitDepth::Eight)
        .with_speed(1)
        .with_alpha_quality(22.0)
        .with_alpha_color_mode(AlphaColorMode::UnassociatedDirty)
        .with_num_threads(Some(2));
    let EncodedImage { avif_file, color_byte_size, alpha_byte_size , .. } = enc.encode_rgba(img.as_ref()).unwrap();
    assert!(color_byte_size > 50 && color_byte_size < 1000);
    assert!(alpha_byte_size > 50 && alpha_byte_size < 1000); // the image must have alpha

    let parsed = avif_parse::read_avif(&mut avif_file.as_slice()).unwrap();
    assert!(parsed.alpha_item.is_some());
    assert!(parsed.primary_item.len() > 100);
    assert!(parsed.primary_item.len() < 1000);

    let md = parsed.primary_item_metadata().unwrap();
    assert_eq!(md.max_frame_width.get(), 256);
    assert_eq!(md.max_frame_height.get(), 200);
    assert_eq!(md.bit_depth, 8);
}

#[test]
fn encode8_opaque() {
    let img = imgref::ImgVec::new((0..101).flat_map(|y| (0..129).map(move |x| {
        RGBA8::new(255, 100 + x as u8, y as u8, 255)
    })).collect(), 129, 101);

    let enc = Encoder::new()
        .with_quality(33.0)
        .with_speed(10)
        .with_alpha_quality(33.0)
        .with_bit_depth(BitDepth::Auto)
        .with_alpha_color_mode(AlphaColorMode::UnassociatedDirty)
        .with_num_threads(Some(1));
    let EncodedImage { avif_file, color_byte_size, alpha_byte_size , .. } = enc.encode_rgba(img.as_ref()).unwrap();
    assert_eq!(0, alpha_byte_size); // the image must not have alpha
    let tmp_path = format!("/tmp/ravif-encode-test-failure-{color_byte_size}.avif");
    if color_byte_size <= 150 || color_byte_size >= 500 {
        std::fs::write(&tmp_path, &avif_file).expect(&tmp_path);
    }
    assert!(color_byte_size > 150 && color_byte_size < 500, "size = {color_byte_size}; expected ~= 215; see {tmp_path}");

    let parsed1 = avif_parse::read_avif(&mut avif_file.as_slice()).unwrap();
    assert_eq!(None, parsed1.alpha_item);

    let md = parsed1.primary_item_metadata().unwrap();
    assert_eq!(md.max_frame_width.get(), 129);
    assert_eq!(md.max_frame_height.get(), 101);
    assert!(md.still_picture);
    assert_eq!(md.bit_depth, 10);

    let img = img.map_buf(|b| b.into_iter().map(|px| px.rgb()).collect::<Vec<_>>());

    let enc = Encoder::new()
        .with_quality(33.0)
        .with_speed(10)
        .with_bit_depth(BitDepth::Ten)
        .with_alpha_quality(33.0)
        .with_alpha_color_mode(AlphaColorMode::UnassociatedDirty)
        .with_num_threads(Some(1));

    let EncodedImage { avif_file, color_byte_size, alpha_byte_size , .. } = enc.encode_rgb(img.as_ref()).unwrap();
    assert_eq!(0, alpha_byte_size); // the image must not have alpha
    assert!(color_byte_size > 50 && color_byte_size < 1000);

    let parsed2 = avif_parse::read_avif(&mut avif_file.as_slice()).unwrap();

    assert_eq!(parsed1.alpha_item, parsed2.alpha_item);
    assert_eq!(parsed1.primary_item, parsed2.primary_item); // both are the same pixels
}

#[test]
fn encode8_cleans_alpha() {
    let img = imgref::ImgVec::new((0..200).flat_map(|y| (0..256).map(move |x| {
        RGBA8::new((((x/ 5 + y ) & 0xF) << 4) as u8, (7 * x + y / 2) as u8, ((x * y) & 0x3) as u8, ((x + y) as u8 & 0x7F).saturating_sub(100))
    })).collect(), 256, 200);

    let enc = Encoder::new()
        .with_quality(66.0)
        .with_speed(6)
        .with_alpha_quality(88.0)
        .with_alpha_color_mode(AlphaColorMode::UnassociatedDirty)
        .with_num_threads(Some(1));

    let dirty = enc
        .encode_rgba(img.as_ref())
        .unwrap();

    let clean = enc
        .with_alpha_color_mode(AlphaColorMode::UnassociatedClean)
        .encode_rgba(img.as_ref())
        .unwrap();

    assert_eq!(clean.alpha_byte_size, dirty.alpha_byte_size); // same alpha on both
    assert!(clean.alpha_byte_size > 200 && clean.alpha_byte_size < 1000);
    assert!(clean.color_byte_size > 2000 && clean.color_byte_size < 6000);
    assert!(clean.color_byte_size < dirty.color_byte_size / 2); // significant reduction in color data
}
