use crate::fs::{open_unchecked, OpenOptions};
use maybe_owned::MaybeOwned;
use std::ops::Deref;
use std::path::Component;
use std::{fmt, fs, io, mem};
#[cfg(racy_asserts)]
use {crate::fs::file_path, std::path::PathBuf};

/// Several places in the code need to be able to handle either owned or
/// borrowed [`std::fs::File]`s. Cloning a `File` to let them always have an
/// owned `File` is expensive and fallible, so use this `struct` to hold either
/// one, and implement [`Deref`] to allow them to be handled in a uniform way.
///
/// This is similar to [`Cow`], except without the copy-on-write part ;-).
/// `Cow` requires a `Clone` implementation, which `File` doesn't have, and
/// most users of this type don't need copy-on-write behavior.
///
/// And, this type has the special `descend_to`, which just does an assignment,
/// but also some useful assertion checks.
///
/// [`Deref`]: std::ops::Deref
/// [`Cow`]: std::borrow::Cow
pub(super) struct MaybeOwnedFile<'borrow> {
    inner: MaybeOwned<'borrow, fs::File>,

    #[cfg(racy_asserts)]
    path: Option<PathBuf>,
}

impl<'borrow> MaybeOwnedFile<'borrow> {
    /// Constructs a new `MaybeOwnedFile` which is not owned.
    pub(super) fn borrowed(file: &'borrow fs::File) -> Self {
        #[cfg(racy_asserts)]
        let path = file_path(file);

        Self {
            inner: MaybeOwned::Borrowed(file),

            #[cfg(racy_asserts)]
            path,
        }
    }

    /// Constructs a new `MaybeOwnedFile` which is owned.
    pub(super) fn owned(file: fs::File) -> Self {
        #[cfg(racy_asserts)]
        let path = file_path(&file);

        Self {
            inner: MaybeOwned::Owned(file),

            #[cfg(racy_asserts)]
            path,
        }
    }

    /// Like `borrowed` but does not do path checks.
    #[allow(dead_code)]
    pub(super) const fn borrowed_noassert(file: &'borrow fs::File) -> Self {
        Self {
            inner: MaybeOwned::Borrowed(file),

            #[cfg(racy_asserts)]
            path: None,
        }
    }

    /// Like `owned` but does not do path checks.
    #[allow(dead_code)]
    pub(super) const fn owned_noassert(file: fs::File) -> Self {
        Self {
            inner: MaybeOwned::Owned(file),

            #[cfg(racy_asserts)]
            path: None,
        }
    }

    /// Set this `MaybeOwnedFile` to a new owned file which is from a subtree
    /// of the current file. Return a `MaybeOwnedFile` representing the
    /// previous state.
    pub(super) fn descend_to(&mut self, to: MaybeOwnedFile<'borrow>) -> Self {
        #[cfg(racy_asserts)]
        let path = self.path.clone();

        #[cfg(racy_asserts)]
        if let Some(to_path) = file_path(&to) {
            if let Some(current_path) = &self.path {
                assert!(
                    to_path.starts_with(current_path),
                    "attempted to descend from {:?} to {:?}",
                    to_path.display(),
                    current_path.display()
                );
            }
            self.path = Some(to_path);
        }

        Self {
            inner: mem::replace(&mut self.inner, to.inner),

            #[cfg(racy_asserts)]
            path,
        }
    }

    /// Produce an owned `File`. This uses `open` on "." if needed to convert a
    /// borrowed `File` to an owned one.
    #[cfg_attr(windows, allow(dead_code))]
    pub(super) fn into_file(self, options: &OpenOptions) -> io::Result<fs::File> {
        match self.inner {
            MaybeOwned::Owned(file) => Ok(file),
            MaybeOwned::Borrowed(file) => {
                // The only situation in which we'd be asked to produce an owned
                // `File` is when there's a need to open "." within a directory
                // to obtain a new handle.
                open_unchecked(file, Component::CurDir.as_ref(), options).map_err(Into::into)
            }
        }
    }

    /// Assuming `self` holds an owned `File`, return it.
    #[cfg_attr(any(windows, target_os = "freebsd"), allow(dead_code))]
    pub(super) fn unwrap_owned(self) -> fs::File {
        match self.inner {
            MaybeOwned::Owned(file) => file,
            MaybeOwned::Borrowed(_) => panic!("expected owned file"),
        }
    }
}

impl<'borrow> Deref for MaybeOwnedFile<'borrow> {
    type Target = fs::File;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.inner.as_ref()
    }
}

impl<'borrow> fmt::Debug for MaybeOwnedFile<'borrow> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.deref().fmt(f)
    }
}
