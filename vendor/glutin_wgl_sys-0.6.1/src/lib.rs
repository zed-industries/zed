#![cfg(windows)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::missing_safety_doc)]
#![allow(clippy::manual_non_exhaustive)]
#![allow(clippy::unnecessary_cast)]
#![allow(unused_imports)]
#![allow(unknown_lints, clippy::missing_transmute_annotations)]

/// WGL bindings
pub mod wgl {
    include!(concat!(env!("OUT_DIR"), "/wgl_bindings.rs"));
}

/// Functions that are not necessarily always available
pub mod wgl_extra {
    include!(concat!(env!("OUT_DIR"), "/wgl_extra_bindings.rs"));
}

#[link(name = "opengl32")]
extern "C" {}
