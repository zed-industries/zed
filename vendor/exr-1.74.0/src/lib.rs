

//! Read and write OpenEXR images.
//! This library uses no foreign code or unsafe Rust.
//!
//! See the [README.md](https://github.com/johannesvollmer/exrs/blob/master/README.md) for crate information.
//! Read __the [GUIDE.md](https://github.com/johannesvollmer/exrs/blob/master/GUIDE.md) for a API introduction__.
//! Check out the [examples](https://github.com/johannesvollmer/exrs/tree/master/examples) for a first impression.

#![warn(
    rust_2018_idioms,
    future_incompatible,
    unused_extern_crates,
    unused,

    missing_copy_implementations,
    missing_debug_implementations,

    clippy::all,
    clippy::restriction,
    clippy::pedantic,
    clippy::nursery,
    clippy::cargo,
)]

#![deny(
    unused_variables,
    unused_assignments,
    dead_code,
    unused_must_use,
    missing_copy_implementations,
    trivial_numeric_casts,
    redundant_semicolons
)]

#![forbid(unsafe_code)]
#![warn(missing_docs)]

pub mod io; // public to allow for custom attribute byte parsing

pub mod math;
pub mod compression;
pub mod meta;
pub mod image;

pub mod error;
pub mod block;

#[macro_use]
extern crate smallvec;

/// Export the most important items from `exrs`.
/// _Note: This includes a type called `Result`, possibly overwriting the default `std::Result` type usage._
pub mod prelude {

    /// Import this specifically if you want to be explicit but still use the extension traits.
    pub mod traits {
        pub use crate::image::write::{WritableImage, channels::GetPixel};
        pub use crate::image::read::{
            read, any_channels::ReadSamples, image::ReadLayers,
            image::ReadImage, layers::ReadChannels,
            specific_channels::{ReadSpecificChannel}
        };

        pub use crate::image::crop::{Crop, CropWhere, CropResult, InspectSample, CroppedChannels, ApplyCroppedView};
    }

    pub use traits::*;

    pub use crate::image::write::{write_rgb_file, write_rgba_file};
    pub use crate::image::read::{
        read_first_rgba_layer_from_file,
        read_all_rgba_layers_from_file,
        read_all_data_from_file,
        read_all_flat_layers_from_file,
        read_first_flat_layer_from_file
    };

    // image data structures
    pub use crate::image::*;
    pub use crate::meta::{ attribute, MetaData, header::{ LayerAttributes, ImageAttributes } };
    pub use crate::block::samples::Sample;
    pub use crate::meta::attribute::{
        AttributeValue, Compression, Text, IntegerBounds,
        LineOrder, SampleType, TileDescription, ChannelDescription
    };

    // common math
    pub use crate::math::Vec2;

    // error handling
    pub use crate::error::{ Result, Error };

    // re-export external stuff
    pub use half::f16;
    pub use smallvec::SmallVec;
}



