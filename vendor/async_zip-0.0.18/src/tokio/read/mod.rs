// Copyright (c) 2023 Harry [Majored] [hello@majored.pw]
// MIT License (https://github.com/Majored/rs-async-zip/blob/main/LICENSE)

//! A module which supports reading ZIP files.

use tokio_util::compat::Compat;

#[cfg(feature = "tokio-fs")]
pub mod fs;
#[cfg(doc)]
use crate::base;
#[cfg(doc)]
use tokio;

/// A [`tokio`]-specific type alias for [`base::read::ZipEntryReader`];
pub type ZipEntryReader<'a, R, E> = crate::base::read::ZipEntryReader<'a, Compat<R>, E>;

pub mod seek {
    //! A ZIP reader which acts over a seekable source.
    use tokio_util::compat::Compat;

    #[cfg(doc)]
    use crate::base;
    #[cfg(doc)]
    use tokio;

    /// A [`tokio`]-specific type alias for [`base::read::seek::ZipFileReader`];
    pub type ZipFileReader<R> = crate::base::read::seek::ZipFileReader<Compat<R>>;
}

pub mod stream {
    //! A ZIP reader which acts over a non-seekable source.

    #[cfg(doc)]
    use crate::base;
    #[cfg(doc)]
    use tokio;
    use tokio_util::compat::Compat;

    /// A [`tokio`]-specific type alias for [`base::read::stream::Reading`];
    pub type Reading<'a, R, E> = crate::base::read::stream::Reading<'a, Compat<R>, E>;
    /// A [`tokio`]-specific type alias for [`base::read::stream::Ready`];
    pub type Ready<R> = crate::base::read::stream::Ready<Compat<R>>;
}
