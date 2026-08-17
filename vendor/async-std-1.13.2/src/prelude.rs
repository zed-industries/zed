//! The async prelude.
//!
//! The prelude re-exports most commonly used traits and macros from this crate.
//!
//! # Examples
//!
//! Import the prelude with:
//!
//! ```
//! # #[allow(unused_imports)]
//! use async_std::prelude::*;
//! ```

cfg_std! {
    #[doc(no_inline)]
    pub use std::future::Future;
    #[doc(no_inline)]
    pub use crate::stream::Stream;

    #[doc(inline)]
    pub use crate::future::future::FutureExt;
    #[doc(inline)]
    pub use crate::stream::stream::StreamExt;
    #[doc(no_inline)]
    pub use crate::io::BufRead as _;
    #[doc(no_inline)]
    pub use crate::io::Read as _;
    #[doc(no_inline)]
    pub use crate::io::Seek as _;
    #[doc(no_inline)]
    pub use crate::io::Write as _;

    #[doc(no_inline)]
    pub use crate::io::prelude::BufReadExt as _;
    #[doc(no_inline)]
    pub use crate::io::prelude::ReadExt as _;
    #[doc(no_inline)]
    pub use crate::io::prelude::SeekExt as _;
    #[doc(no_inline)]
    pub use crate::io::prelude::WriteExt as _;
}

cfg_default! {
    #[doc(no_inline)]
    pub use crate::task_local;
}

cfg_unstable! {
    #[doc(no_inline)]
    pub use crate::stream::DoubleEndedStream;
    #[doc(no_inline)]
    pub use crate::stream::ExactSizeStream;
}
