//! Implementations of comparison traits copied and modified from The Rust
//! Programming Language.
//!
//! Sources:
//! - <https://github.com/rust-lang/rust/blob/b1277d04db0dc8009037e872a1be7cdc2bd74a43/library/std/src/path.rs#L2635-L2726>
//!
//! Copyrights:
//! - Copyrights in the Rust project are retained by their contributors. No
//!   copyright assignment is required to contribute to the Rust project.
//!
//!   Some files include explicit copyright notices and/or license notices.
//!   For full authorship information, see the version control history or
//!   <https://thanks.rust-lang.org>
//!
//!   <https://github.com/rust-lang/rust/blob/b1277d04db0dc8009037e872a1be7cdc2bd74a43/COPYRIGHT>
//! - Modifications copyright (c) 2020 dylni (<https://github.com/dylni>)<br>
//!   <https://github.com/dylni/normpath/blob/master/COPYRIGHT>

use std::borrow::Cow;
use std::cmp::Ordering;
use std::path::Path;
use std::path::PathBuf;

use super::BasePath;
use super::BasePathBuf;

macro_rules! r#impl {
    ( $left:ty , $right:ty ) => {
        impl PartialEq<$right> for $left {
            #[inline]
            fn eq(&self, other: &$right) -> bool {
                <BasePath as PartialEq<Path>>::eq(self, other.as_ref())
            }
        }

        impl PartialEq<$left> for $right {
            #[inline]
            fn eq(&self, other: &$left) -> bool {
                other == self
            }
        }

        impl PartialOrd<$right> for $left {
            #[inline]
            fn partial_cmp(&self, other: &$right) -> Option<Ordering> {
                <BasePath as PartialOrd<Path>>::partial_cmp(
                    self,
                    other.as_ref(),
                )
            }
        }

        impl PartialOrd<$left> for $right {
            #[inline]
            fn partial_cmp(&self, other: &$left) -> Option<Ordering> {
                other.partial_cmp(self)
            }
        }
    };
}

r#impl!(BasePathBuf, BasePath);
r#impl!(BasePathBuf, &BasePath);
r#impl!(Cow<'_, BasePath>, BasePath);
r#impl!(Cow<'_, BasePath>, &BasePath);
r#impl!(Cow<'_, BasePath>, BasePathBuf);

r#impl!(BasePathBuf, Path);
r#impl!(BasePathBuf, &Path);
r#impl!(BasePathBuf, Cow<'_, Path>);
r#impl!(BasePathBuf, PathBuf);
r#impl!(BasePath, &Path);
r#impl!(BasePath, Cow<'_, Path>);
r#impl!(BasePath, PathBuf);
r#impl!(&BasePath, Path);
r#impl!(&BasePath, Cow<'_, Path>);
r#impl!(&BasePath, PathBuf);
