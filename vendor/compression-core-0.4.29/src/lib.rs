//! Abstractions for compression algorithms.

#![cfg_attr(docsrs, feature(doc_auto_cfg, doc_cfg))]

mod level;
pub mod unshared;
pub mod util;

pub use level::Level;
