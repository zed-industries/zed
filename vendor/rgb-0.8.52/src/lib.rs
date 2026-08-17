//! Basic struct for `RGB` and `RGBA` pixels. Packed, with red first, alpha last.
//!
//! This crate is intended to be the lowest common denominator for sharing `RGB`/`RGBA` bitmaps between other crates.
//!
//! The crate includes convenience functions for converting between the struct and bytes,
//! and overloaded operators that work on all channels at once.
//!
//! This crate intentionally doesn't implement color management (due to complexity of the problem),
//! but the structs can be parametrized to implement this if necessary. Other colorspaces are out of scope.
//!
#![cfg_attr(feature = "as-bytes", doc = "```rust")]
#![cfg_attr(not(feature = "as-bytes"), doc = "```ignore")]
//! # use rgb::*;
//! let pixel = RGB8 {r:0, g:100, b:255};
//!
//! let pixel_rgba = pixel.alpha(255);
//! let pixel = pixel_rgba.rgb();
//!
//! let pixels = vec![pixel; 100];
//! /// Can be converted to a type-less slice without copying
//! let bytes: &[u8] = rgb::bytemuck::cast_slice(&pixels);
//!
//! use rgb::prelude::*; // Import pixel map trait
//! let half_bright = pixel.map(|channel| channel / 2);
//! let doubled = half_bright * 2;
//! # let _ = doubled;
//! ```
#![doc(html_logo_url = "https://kornel.ski/rgb-logo.png")]
#![warn(missing_docs)]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![no_std]

// std is required to run unit tests
#[cfg(test)]
#[macro_use] extern crate std;
/// Re-export of the [`bytemuck` crate](https://lib.rs/bytemuck). [See docs](https://docs.rs/bytemuck).
///
/// Use [`::bytemuck::cast_slice()`] or [`::bytemuck::from_bytes()`] to convert
/// pixels to/from `&[u8]`.
#[cfg(feature = "bytemuck")]
#[doc(alias = "ComponentSlice")]
#[doc(alias = "as_bytes")]
#[doc(alias = "Pod")]
pub use ::bytemuck;

pub(crate) mod formats {
    pub mod abgr;
    pub mod argb;
    pub mod bgr;
    pub mod bgra;
    pub mod gray;
    pub mod gray_a;
    pub mod gray_alpha;
    pub mod grb;
    pub mod rgb;
    pub mod rgba;
}

/// Use as `use rgb::prelude::*`
///
/// The prelude will contain commonly used traits, and will be expanded in the future.
///
/// Using a glob import is recommended for forward compatibility with the next major version of the crate.
pub mod prelude {
    pub use crate::legacy::internal::pixel::ComponentMap;
    pub use crate::legacy::internal::pixel::ColorComponentMap;
}

pub use formats::abgr::Abgr;
pub use formats::argb::Argb;
pub use formats::bgr::Bgr;
pub use formats::bgra::Bgra;
#[cfg(not(feature = "unstable-experimental"))]
pub use formats::gray_alpha::GrayAlpha_v08 as GrayAlpha;
#[cfg(not(feature = "unstable-experimental"))]
pub use formats::gray::Gray_v08 as Gray;
pub use formats::grb::Grb;
pub use formats::rgb::Rgb;
pub use formats::rgba::Rgba;

mod inherent_impls;

pub(crate) mod legacy {
    pub(crate) mod internal {
        pub mod convert;
        pub mod ops;
        pub mod pixel;
        pub mod rgb;
        pub mod rgba;
    }
    /// BGR/BGRA alernative layouts & grayscale
    ///
    /// BGR might be useful for some Windows or OpenGL APIs.
    pub mod alt;
}

pub use legacy::alt;

#[cfg(all(feature = "bytemuck", not(feature = "as-bytes")))]
mod bytemuck_impl;
#[cfg(feature = "as-bytes")]
mod as_bytes;

/// Re-export from `bytemuck` crate
#[cfg(feature = "as-bytes")]
pub use ::bytemuck::Pod;
/// Re-export from `bytemuck` crate
#[cfg(feature = "as-bytes")]
pub use ::bytemuck::Zeroable;

pub use crate::legacy::internal::convert::*;
pub use crate::legacy::internal::pixel::*;

#[doc(hidden)]
/// Renamed to `Rgb`
pub use formats::rgb::Rgb as RGB;
#[doc(hidden)]
/// Renamed to `Rgba`
pub use formats::rgba::Rgba as RGBA;

#[doc(hidden)]
/// Incompatible replacement for the `GrayAlpha` type
pub use formats::gray_a::GrayA;

#[cfg(feature = "unstable-experimental")]
pub use formats::gray::Gray_v09 as Gray;

/// 8-bit RGB
///
/// The colorspace is technically undefined, but generally sRGB is assumed.
pub type RGB8 = RGB<u8>;

/// 16-bit RGB in machine's native endian
///
/// Be careful to perform byte-swapping when reading from files.
pub type RGB16 = RGB<u16>;

/// 8-bit RGBA, alpha is last. 0 = transparent, 255 = opaque.
pub type RGBA8 = RGBA<u8>;

/// 16-bit RGB in machine's native endian. 0 = transparent, 65535 = opaque.
///
/// Alpha is last.
pub type RGBA16 = RGBA<u16>;

#[test]
fn rgb_works() {
    let rgb = RGB{r:0u8,g:128,b:255}.clone();
    assert_eq!(rgb.b, 255);

    assert_eq!(rgb, rgb.iter().map(|ch| ch).collect());

    #[cfg(feature = "as-bytes")]
    {
        assert_eq!(0, [rgb].as_bytes()[0]);
        assert_eq!(128, [rgb].as_bytes()[1]);
        assert_eq!(255, [rgb].as_bytes()[2]);
    }

    let rgb = RGB16{r:0u16,g:0x7F7F,b:65535};
    assert_eq!(rgb.b, 65535);
    assert_eq!(rgb.as_slice()[1], 0x7F7F);

    #[cfg(feature = "as-bytes")]
    {
        assert_eq!(0, [rgb].as_bytes()[0]);
        assert_eq!(0, [rgb].as_bytes()[1]);
        assert_eq!(0x7F, [rgb].as_bytes()[2]);
        assert_eq!(0x7F, [rgb].as_bytes()[3]);
        assert_eq!(0xFF, [rgb].as_bytes()[4]);
        assert_eq!(0xFF, [rgb].as_bytes()[5]);
    }

    assert_eq!("rgb(1,2,3)", format!("{}", RGB::new(1,2,3)));
}

#[test]
fn sub_floats() {
    assert_eq!(RGBA{r:2.5_f64, g:-1.5, b:0., a:5.}, RGBA{r:3.5_f64, g:-0.5, b:-2., a:0.} - RGBA{r:1.0_f64, g:1., b:-2., a:-5.});
}

#[test]
fn into() {
    let a:RGB8 = RGB{r:0,g:1,b:2};
    let b:RGB<i16> = a.into();
    let c:RGB<f32> = b.into();
    let d:RGB<f32> = a.into();
    assert_eq!(c, d);
}

#[test]
fn rgba_works() {
    let rgba = RGBA{r:0u8,g:128,b:255,a:33}.clone();
    assert_eq!(rgba.b, 255);
    assert_eq!(rgba.a, 33);

    assert_eq!(rgba, rgba.iter().map(|ch| ch).collect());

    assert_eq!("rgba(1,2,3,4)", format!("{}", RGBA::new(1,2,3,4)));

    assert_eq!(rgba - rgba, RGBA::new(0,0,0,0));
}

#[test]
fn bytes() {
    let rgb = RGB8::new(1,2,3);

    #[cfg(feature = "as-bytes")]
    {
        let rgb_arr = [rgb];
        let rgb_bytes = rgb_arr.as_bytes();
        assert_eq!(&[1,2,3], rgb_bytes);
        assert_eq!(rgb_bytes.as_rgba().len(), 0);
        assert_eq!({let t: &[RGBA8] = rgb_bytes.as_pixels(); t}.len(), 0);
        assert_eq!(rgb, rgb_bytes.into_iter().cloned().collect());
        assert_eq!(&[rgb], rgb_bytes.as_rgb());
        assert_eq!(&[rgb], rgb_bytes.as_pixels());
    }
    let mut rgb2 = [rgb];
    assert_eq!(rgb2[..].as_mut_slice().as_rgb_mut(), &mut [rgb]);
    assert_eq!(&mut [rgb], rgb2[..].as_mut_slice().as_pixels_mut());


    #[cfg(feature = "as-bytes")]
    {
        let rgba = RGBA8::new(1,2,3,4);
        let mut rgba_arr = [rgba];
        let rgba_bytes = rgba_arr.as_bytes_mut();
        assert_eq!(&[1,2,3,4], rgba_bytes);
        assert_eq!(&[rgba], rgba_bytes.as_rgba());
        rgba_bytes[3] = 99;
        assert_eq!(RGBA8::new(1,2,3,99), rgba_arr.as_bytes().into_iter().cloned().collect());
    }

    let rgb = RGB16::new(1,2,3);
    let rgb_slice = rgb.as_slice();
    assert_eq!(&[1,2,3], rgb_slice);
    assert_eq!(rgb_slice.as_rgba(), &[]);
    assert_eq!(&[rgb], rgb_slice.as_rgb());
    assert_eq!(rgb, rgb_slice.into_iter().cloned().collect());

    let rgba = RGBA16::new(1,2,3,4);
    let rgba_slice = rgba.as_slice();
    assert_eq!(&[1,2,3,4], rgba_slice);
    assert_eq!(&[1,2,3], rgba_slice.as_rgb()[0].as_slice());
    assert_eq!(&[rgba], rgba_slice.as_rgba());
    assert_eq!(rgba, rgba_slice.iter().copied().collect());
    let mut rgba2 = [rgba];
    assert_eq!(rgba2[..].as_mut_slice().as_rgba_mut(), &mut [rgba]);

    let mut foo = vec![0u8; 8];
    foo.as_rgba_mut()[1] = RGBA::new(1,2,3,4);
    assert_eq!(&[0u8,0,0,0,1,2,3,4], &foo[..]);
}
