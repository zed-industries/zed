use std::str::FromStr;
use util::EntityTag;

/// `ETag` header, defined in [RFC7232](http://tools.ietf.org/html/rfc7232#section-2.3)
///
/// The `ETag` header field in a response provides the current entity-tag
/// for the selected representation, as determined at the conclusion of
/// handling the request.  An entity-tag is an opaque validator for
/// differentiating between multiple representations of the same
/// resource, regardless of whether those multiple representations are
/// due to resource state changes over time, content negotiation
/// resulting in multiple representations being valid at the same time,
/// or both.  An entity-tag consists of an opaque quoted string, possibly
/// prefixed by a weakness indicator.
///
/// # ABNF
///
/// ```text
/// ETag       = entity-tag
/// ```
///
/// # Example values
///
/// * `"xyzzy"`
/// * `W/"xyzzy"`
/// * `""`
///
/// # Examples
///
/// ```
/// let etag = "\"xyzzy\"".parse::<headers::ETag>().unwrap();
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ETag(pub(super) EntityTag);

derive_header! {
    ETag(_),
    name: ETAG
}

impl ETag {
    #[cfg(test)]
    pub(crate) fn from_static(src: &'static str) -> ETag {
        ETag(EntityTag::from_static(src))
    }
}

error_type!(InvalidETag);

impl FromStr for ETag {
    type Err = InvalidETag;
    fn from_str(src: &str) -> Result<Self, Self::Err> {
        let val = src.parse().map_err(|_| InvalidETag { _inner: () })?;

        EntityTag::from_owned(val)
            .map(ETag)
            .ok_or_else(|| InvalidETag { _inner: () })
    }
}

/*
test_etag {
    // From the RFC
    test_header!(test1,
        vec![b"\"xyzzy\""],
        Some(ETag(EntityTag::new(false, "xyzzy".to_owned()))));
    test_header!(test2,
        vec![b"W/\"xyzzy\""],
        Some(ETag(EntityTag::new(true, "xyzzy".to_owned()))));
    test_header!(test3,
        vec![b"\"\""],
        Some(ETag(EntityTag::new(false, "".to_owned()))));
    // Own tests
    test_header!(test4,
        vec![b"\"foobar\""],
        Some(ETag(EntityTag::new(false, "foobar".to_owned()))));
    test_header!(test5,
        vec![b"\"\""],
        Some(ETag(EntityTag::new(false, "".to_owned()))));
    test_header!(test6,
        vec![b"W/\"weak-etag\""],
        Some(ETag(EntityTag::new(true, "weak-etag".to_owned()))));
    test_header!(test7,
        vec![b"W/\"\x65\x62\""],
        Some(ETag(EntityTag::new(true, "\u{0065}\u{0062}".to_owned()))));
    test_header!(test8,
        vec![b"W/\"\""],
        Some(ETag(EntityTag::new(true, "".to_owned()))));
    test_header!(test9,
        vec![b"no-dquotes"],
        None::<ETag>);
    test_header!(test10,
        vec![b"w/\"the-first-w-is-case-sensitive\""],
        None::<ETag>);
    test_header!(test11,
        vec![b""],
        None::<ETag>);
    test_header!(test12,
        vec![b"\"unmatched-dquotes1"],
        None::<ETag>);
    test_header!(test13,
        vec![b"unmatched-dquotes2\""],
        None::<ETag>);
    test_header!(test14,
        vec![b"matched-\"dquotes\""],
        None::<ETag>);
    test_header!(test15,
        vec![b"\""],
        None::<ETag>);
}
*/
