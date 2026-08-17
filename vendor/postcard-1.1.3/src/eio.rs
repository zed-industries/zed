// We disable all embedded-io versions but the most recent in docs.rs, because we use
// --all-features which doesn't work with non-additive features.
#[cfg(all(feature = "embedded-io-04", feature = "embedded-io-06", not(docsrs)))]
compile_error!("Only one version of `embedded-io` must be enabled through features");

#[cfg(all(feature = "embedded-io-04", not(docsrs)))]
mod version_impl {
    pub use embedded_io_04 as embedded_io;
    pub use embedded_io_04::blocking::{Read, Write};
}

#[cfg(feature = "embedded-io-06")]
mod version_impl {
    pub use embedded_io_06 as embedded_io;
    pub use embedded_io_06::{Read, Write};
}

// All versions should export the appropriate items
#[cfg(any(feature = "embedded-io-04", feature = "embedded-io-06"))]
pub use version_impl::{embedded_io, Read, Write};
