use util::FlatCsv;

/// `Content-Language` header, defined in
/// [RFC7231](https://tools.ietf.org/html/rfc7231#section-3.1.3.2)
///
/// The `Content-Language` header field describes the natural language(s)
/// of the intended audience for the representation.  Note that this
/// might not be equivalent to all the languages used within the
/// representation.
///
/// # ABNF
///
/// ```text
/// Content-Language = 1#language-tag
/// ```
///
/// # Example values
///
/// * `da`
/// * `mi, en`
///
/// # Examples
///
/// ```
/// # extern crate headers;
/// #[macro_use] extern crate language_tags;
/// use headers::ContentLanguage;
/// #
/// # fn main() {
/// let con_lang = ContentLanguage::new([langtag!(en)])
/// # }
/// ```
#[derive(Clone, Debug, PartialEq, Header)]
pub struct ContentLanguage(FlatCsv);

