use alloc::{boxed::Box, string::String, sync::Arc, vec::Vec};
use core::{error::Error, fmt};

use thiserror::Error;

#[cfg(send_sync)]
pub type ContextErrorSource = Box<dyn Error + Send + Sync + 'static>;
#[cfg(not(send_sync))]
pub type ContextErrorSource = Box<dyn Error + 'static>;

#[derive(Debug, Error)]
#[error(
    "In {fn_ident}{}{}{}",
    if self.label.is_empty() { "" } else { ", label = '" },
    self.label,
    if self.label.is_empty() { "" } else { "'" }
)]
pub struct ContextError {
    pub fn_ident: &'static str,
    #[source]
    pub source: ContextErrorSource,
    pub label: String,
}

/// Don't use this error type with thiserror's #[error(transparent)]
#[derive(Clone)]
pub struct MultiError {
    inner: Vec<Arc<dyn Error + Send + Sync + 'static>>,
}

impl MultiError {
    pub fn new<T: Error + Send + Sync + 'static>(
        iter: impl ExactSizeIterator<Item = T>,
    ) -> Option<Self> {
        if iter.len() == 0 {
            return None;
        }
        Some(Self {
            inner: iter.map(Box::from).map(Arc::from).collect(),
        })
    }

    pub fn errors(&self) -> Box<dyn Iterator<Item = &(dyn Error + Send + Sync + 'static)> + '_> {
        Box::new(self.inner.iter().map(|e| e.as_ref()))
    }
}

impl fmt::Debug for MultiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        fmt::Debug::fmt(&self.inner[0], f)
    }
}

impl fmt::Display for MultiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        fmt::Display::fmt(&self.inner[0], f)
    }
}

impl Error for MultiError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.inner[0].source()
    }
}
