#![allow(missing_debug_implementations)]

//! Percent-encoders for URI/IRI components.

use super::{table::*, Encoder, Table};

/// An encoder for URI userinfo.
#[derive(Clone, Copy)]
pub struct Userinfo(());

impl Encoder for Userinfo {
    const TABLE: &'static Table = USERINFO;
}

/// An encoder for IRI userinfo.
#[derive(Clone, Copy)]
pub struct IUserinfo(());

impl Encoder for IUserinfo {
    const TABLE: &'static Table = IUSERINFO;
}

/// An encoder for URI registered name.
#[derive(Clone, Copy)]
#[cfg_attr(fuzzing, derive(PartialEq, Eq))]
pub struct RegName(());

impl Encoder for RegName {
    const TABLE: &'static Table = REG_NAME;
}

/// An encoder for IRI registered name.
#[derive(Clone, Copy)]
pub struct IRegName(());

impl Encoder for IRegName {
    const TABLE: &'static Table = IREG_NAME;
}

/// An encoder for URI/IRI port.
#[derive(Clone, Copy)]
pub struct Port(());

impl Encoder for Port {
    const TABLE: &'static Table = DIGIT;
}

/// An encoder for URI path.
///
/// `EStr` has [extension methods] for the path component.
///
/// [extension methods]: super::EStr#impl-EStr<E>-1
#[derive(Clone, Copy)]
pub struct Path(());

impl Encoder for Path {
    const TABLE: &'static Table = PATH;
}

/// An encoder for IRI path.
///
/// `EStr` has [extension methods] for the path component.
///
/// [extension methods]: super::EStr#impl-EStr<E>-1
#[derive(Clone, Copy)]
pub struct IPath(());

impl Encoder for IPath {
    const TABLE: &'static Table = IPATH;
}

/// An encoder for URI query.
#[derive(Clone, Copy)]
pub struct Query(());

impl Encoder for Query {
    const TABLE: &'static Table = QUERY;
}

/// An encoder for IRI query.
#[derive(Clone, Copy)]
pub struct IQuery(());

impl Encoder for IQuery {
    const TABLE: &'static Table = IQUERY;
}

/// An encoder for URI fragment.
#[derive(Clone, Copy)]
pub struct Fragment(());

impl Encoder for Fragment {
    const TABLE: &'static Table = FRAGMENT;
}

/// An encoder for IRI fragment.
#[derive(Clone, Copy)]
pub struct IFragment(());

impl Encoder for IFragment {
    const TABLE: &'static Table = IFRAGMENT;
}

/// An encoder for URI data which preserves only [unreserved] characters
/// and encodes the others.
///
/// [unreserved]: https://datatracker.ietf.org/doc/html/rfc3986#section-2.3
#[derive(Clone, Copy)]
pub struct Data(());

impl Encoder for Data {
    const TABLE: &'static Table = &UNRESERVED.or_pct_encoded();
}

/// An encoder for IRI data which preserves only [unreserved] characters
/// and encodes the others.
///
/// [unreserved]: https://datatracker.ietf.org/doc/html/rfc3987#section-2.1
#[derive(Clone, Copy)]
pub struct IData(());

impl Encoder for IData {
    const TABLE: &'static Table = &UNRESERVED.or_pct_encoded().or_ucschar();
}
