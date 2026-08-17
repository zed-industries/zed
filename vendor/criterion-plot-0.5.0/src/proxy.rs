//! Generic constructors for newtypes

#![allow(non_snake_case)]

use crate::{Font as FontType, Label as LabelType, Output as OutputType, Title as TitleType};
use std::borrow::Cow;
use std::path::Path;

/// Generic constructor for `Font`
#[cfg_attr(feature = "cargo-clippy", allow(clippy::inline_always))]
#[inline(always)]
pub fn Font<S>(string: S) -> FontType
where
    S: Into<Cow<'static, str>>,
{
    FontType(string.into())
}

/// Generic constructor for `Label`
#[cfg_attr(feature = "cargo-clippy", allow(clippy::inline_always))]
#[inline(always)]
pub fn Label<S>(string: S) -> LabelType
where
    S: Into<Cow<'static, str>>,
{
    LabelType(string.into())
}

/// Generic constructor for `Title`
#[cfg_attr(feature = "cargo-clippy", allow(clippy::inline_always))]
#[inline(always)]
pub fn Title<S>(string: S) -> TitleType
where
    S: Into<Cow<'static, str>>,
{
    TitleType(string.into())
}

/// Generic constructor for `Output`
#[cfg_attr(feature = "cargo-clippy", allow(clippy::inline_always))]
#[inline(always)]
pub fn Output<P>(path: P) -> OutputType
where
    P: Into<Cow<'static, Path>>,
{
    OutputType(path.into())
}
