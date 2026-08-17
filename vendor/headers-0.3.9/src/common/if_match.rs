use super::ETag;
use util::EntityTagRange;
use HeaderValue;

/// `If-Match` header, defined in
/// [RFC7232](https://tools.ietf.org/html/rfc7232#section-3.1)
///
/// The `If-Match` header field makes the request method conditional on
/// the recipient origin server either having at least one current
/// representation of the target resource, when the field-value is "*",
/// or having a current representation of the target resource that has an
/// entity-tag matching a member of the list of entity-tags provided in
/// the field-value.
///
/// An origin server MUST use the strong comparison function when
/// comparing entity-tags for `If-Match`, since the client
/// intends this precondition to prevent the method from being applied if
/// there have been any changes to the representation data.
///
/// # ABNF
///
/// ```text
/// If-Match = "*" / 1#entity-tag
/// ```
///
/// # Example values
///
/// * `"xyzzy"`
/// * "xyzzy", "r2d2xxxx", "c3piozzzz"
///
/// # Examples
///
/// ```
/// # extern crate headers;
/// use headers::IfMatch;
///
/// let if_match = IfMatch::any();
/// ```
#[derive(Clone, Debug, PartialEq)]
pub struct IfMatch(EntityTagRange);

derive_header! {
    IfMatch(_),
    name: IF_MATCH
}

impl IfMatch {
    /// Create a new `If-Match: *` header.
    pub fn any() -> IfMatch {
        IfMatch(EntityTagRange::Any)
    }

    /// Returns whether this is `If-Match: *`, matching any entity tag.
    pub fn is_any(&self) -> bool {
        match self.0 {
            EntityTagRange::Any => true,
            EntityTagRange::Tags(..) => false,
        }
    }

    /// Checks whether the `ETag` strongly matches.
    pub fn precondition_passes(&self, etag: &ETag) -> bool {
        self.0.matches_strong(&etag.0)
    }
}

impl From<ETag> for IfMatch {
    fn from(etag: ETag) -> IfMatch {
        IfMatch(EntityTagRange::Tags(HeaderValue::from(etag.0).into()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_any() {
        assert!(IfMatch::any().is_any());
        assert!(!IfMatch::from(ETag::from_static("\"yolo\"")).is_any());
    }

    #[test]
    fn precondition_fails() {
        let if_match = IfMatch::from(ETag::from_static("\"foo\""));

        let bar = ETag::from_static("\"bar\"");
        let weak_foo = ETag::from_static("W/\"foo\"");

        assert!(!if_match.precondition_passes(&bar));
        assert!(!if_match.precondition_passes(&weak_foo));
    }

    #[test]
    fn precondition_passes() {
        let foo = ETag::from_static("\"foo\"");

        let if_match = IfMatch::from(foo.clone());

        assert!(if_match.precondition_passes(&foo));
    }

    #[test]
    fn precondition_any() {
        let foo = ETag::from_static("\"foo\"");

        let if_match = IfMatch::any();

        assert!(if_match.precondition_passes(&foo));
    }
}
