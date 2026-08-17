#![doc(html_logo_url = "https://nical.github.io/lyon-doc/lyon-logo.svg")]
#![deny(bare_trait_objects)]
#![allow(dead_code)]
#![allow(unused_variables)]

extern crate lyon_path as path;

pub use path::geom::euclid;
pub use path::math;

pub mod debugging;
pub mod parser;
pub mod rust_logo;
