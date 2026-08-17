//! Traits and functions for casting colors to and from other data types.
//!
//! The functions and traits in this module cast without changing the underlying
//! data. See the [`convert`](crate::convert) module for how to convert between
//! color spaces.
//!
//! # Traits or Functions?
//!
//! This module provides both a set of traits ([`FromComponents`],
//! [`IntoUints`], etc.) and a set of functions ([`from_component_slice`],
//! [`into_uint_array`], etc.) that effectively implement the same
//! functionality. The traits are all implemented using the functions, so they
//! can be seen as bits of more implicit syntax sugar for the more explicit
//! functions.
//!
//! A general recommendation is to use the traits, since they provide mostly
//! better ergonomics.
//!
//! # Arrays and Slices
//!
//! Types that implement [`ArrayCast`] can be cast to and from arrays and slices
//! with little to no overhead. This makes it easy to work with image buffers
//! and types from other crates without having to copy the data first.
//!
//! Casting can be either be done with the free functions in this module, or
//! using the helper traits.
//!
//! ## Casting Arrays
//!
//! Arrays can be type checked to have the correct size at compile time, making
//! casting free after optimization has removed the overhead from asserts. The
//! same is true for arrays in slices and `Vec`s, because the length stays the
//! same after casting.
//!
//! ```
//! use palette::{cast::{self, ArraysAsMut}, Srgb, IntoColor};
//!
//! let color = cast::from_array::<Srgb<u8>>([23, 198, 76]).into_linear();
//! // Note: `Srgb::<u8>::from([23, 198, 76])` works too.
//!
//! let mut buffer = [[64, 139, 10], [93, 18, 214]];
//! let color_buffer: &mut [Srgb<u8>] = buffer.arrays_as_mut();
//!
//! for destination in color_buffer {
//!     let linear_dst = destination.into_linear::<f32>();
//!     *destination = (linear_dst + color).into_encoding();
//! }
//! ```
//!
//! Trying to cast a single array of the wrong size will not compile:
//!
//! ```compile_fail
//! use palette::{cast, Srgb};
//!
//! let color = cast::from_array::<Srgb<u8>>([23, 198]); // Too few components.
//! ```
//!
//! ## Casting Component Buffers
//!
//! This is a common situation is image processing, where you have an image
//! buffer, such as `&mut [u8]`, `&mut [f32]`, `Vec<u8>` or `Vec<f32>`, that you
//! want to work with as colors. This buffer may, for example, be the content of
//! an image file or shared with the GPU.
//!
//! The downside, compared to having fixed size arrays, is that the length
//! cannot be statically known to be a multiple of the color type's array
//! length. This adds a bit of error handling overhead, as well as for dividing
//! or multiplying the length.
//!
//! ```
//! use palette::{cast::{self, TryComponentsAs}, Srgb};
//!
//! let correct_buffer = [64, 139, 10, 93, 18, 214];
//! let should_be_ok: Result<&[Srgb<u8>], _> = correct_buffer.try_components_as();
//! assert!(should_be_ok.is_ok());
//!
//! let incorrect_buffer = [64, 139, 10, 93, 18, 214, 198, 76];
//! let should_be_err: Result<&[Srgb<u8>], _> = incorrect_buffer.try_components_as();
//! assert!(should_be_err.is_err());
//! ```
//!
//! An alternative, for when the length can be trusted to be correct, is to use
//! methods without the `try_*` prefix, such as `ComponentsAs::components_as`,
//! or the `from_component_*` functions, that panic on error.
//!
//! This works:
//!
//! ```
//! use palette::{cast::ComponentsAs, Srgb};
//!
//! let correct_buffer = [64, 139, 10, 93, 18, 214];
//! let color_buffer: &[Srgb<u8>] = correct_buffer.components_as();
//! ```
//!
//! But this panics:
//!
//! ```should_panic
//! use palette::{cast::ComponentsAs, Srgb};
//!
//! let incorrect_buffer = [64, 139, 10, 93, 18, 214, 198, 76];
//! let color_buffer: &[Srgb<u8>] = incorrect_buffer.components_as();
//! ```
//!
//! ## Casting Single Colors
//!
//! The built-in color types implement `AsRef`, `AsMut`, `From`, `Into`,
//! `TryFrom` and `TryInto` in addition to `ArrayCast` for convenient casting of
//! single colors:
//!
//! ```
//! use core::convert::TryFrom;
//! use palette::Srgb;
//!
//! let color = Srgb::from([23u8, 198, 76]);
//! let array: [u8; 3] = color.into();
//!
//! let slice: &[u8] = color.as_ref();
//! assert!(<&Srgb<u8>>::try_from(slice).is_ok());
//!
//! let short_slice: &[f32] = &[0.1, 0.5];
//! assert!(<&Srgb>::try_from(short_slice).is_err()); // Too few components.
//! ```
//!
//! ## Component Order
//!
//! The component order in an array or slice is not always the same as in the
//! color types. For example, a byte buffer that is encoded as ARGB will not
//! cast to correct `Rgba` values. The components can be reordered after casting
//! by using the [`Packed`] wrapper as an intermediate representation.
//!
//! ```
//! // `PackedArgb` is an alias for `Packed<rgb::channels::Argb, P = u32>`.
//! use palette::{rgb::PackedArgb, cast::ComponentsAs, Srgba};
//!
//! let components = [1.0f32, 0.8, 0.2, 0.3, 1.0, 0.5, 0.7, 0.6];
//! let colors: &[PackedArgb<_>] = components.components_as();
//!
//! // Notice how the alpha values have moved from the beginning to the end:
//! assert_eq!(Srgba::from(colors[0]), Srgba::new(0.8, 0.2, 0.3, 1.0));
//! assert_eq!(Srgba::from(colors[1]), Srgba::new(0.5, 0.7, 0.6, 1.0));
//! ```
//!
//! # Unsigned Integers
//!
//! Types that implement [`UintCast`] can be cast to and from unsigned integers
//! of the same size. It's a bit more limited than slices and arrays but it's
//! useful for common patterns like representing RGBA values as hexadecimal
//! unsigned integers.
//!
//! The [`Packed`] wrapper can be used as an intermediate format to make
//! unpacking the values as simple as `from` or `into`. It's also possible to
//! choose a channel order to be something other than what the default `From`
//! implementations would use.
//!
//! ```
//! // `PackedArgb` is an alias for `Packed<rgb::channels::Argb, P = u32>`.
//! use palette::{rgb::PackedArgb, cast::UintsAs, Srgba};
//!
//! let raw = [0xFF7F0080u32, 0xFF60BBCC];
//! let colors: &[PackedArgb] = raw.uints_as();
//!
//! assert_eq!(colors.len(), 2);
//! assert_eq!(Srgba::from(colors[0]), Srgba::new(0x7F, 0x00, 0x80, 0xFF));
//! assert_eq!(Srgba::from(colors[1]), Srgba::new(0x60, 0xBB, 0xCC, 0xFF));
//! ```

mod array;
mod as_arrays_traits;
mod as_components_traits;
mod as_uints_traits;
mod from_into_arrays_traits;
mod from_into_components_traits;
mod from_into_uints_traits;
mod packed;
mod uint;

pub use self::{
    array::*, as_arrays_traits::*, as_components_traits::*, as_uints_traits::*,
    from_into_arrays_traits::*, from_into_components_traits::*, from_into_uints_traits::*,
    packed::*, uint::*,
};
