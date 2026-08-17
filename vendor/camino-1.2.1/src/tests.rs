// Copyright (c) The camino Contributors
// SPDX-License-Identifier: MIT OR Apache-2.0

// Test that all required impls exist.

use crate::{Utf8Path, Utf8PathBuf};
use std::{
    borrow::Cow,
    path::{Path, PathBuf},
    rc::Rc,
    sync::Arc,
};

macro_rules! all_into {
    ($t:ty, $x:ident) => {
        test_into::<$t, Utf8PathBuf>($x.clone());
        test_into::<$t, Box<Utf8Path>>($x.clone());
        test_into::<$t, Arc<Utf8Path>>($x.clone());
        test_into::<$t, Rc<Utf8Path>>($x.clone());
        test_into::<$t, Cow<'_, Utf8Path>>($x.clone());
        test_into::<$t, PathBuf>($x.clone());
        test_into::<$t, Box<Path>>($x.clone());
        test_into::<$t, Arc<Path>>($x.clone());
        test_into::<$t, Rc<Path>>($x.clone());
        test_into::<$t, Cow<'_, Path>>($x.clone());
    };
}

#[test]
fn test_borrowed_into() {
    let utf8_path = Utf8Path::new("test/path");
    all_into!(&Utf8Path, utf8_path);
}

#[test]
fn test_owned_into() {
    let utf8_path_buf = Utf8PathBuf::from("test/path");
    all_into!(Utf8PathBuf, utf8_path_buf);
}

fn test_into<T, U>(orig: T)
where
    T: Into<U>,
{
    let _ = orig.into();
}

#[cfg(path_buf_deref_mut)]
#[test]
fn test_deref_mut() {
    // This test is mostly for miri.
    let mut path_buf = Utf8PathBuf::from("foobar");
    let _: &mut Utf8Path = &mut path_buf;
}
