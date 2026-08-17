use cfg_if::cfg_if;

/// A type of parsed text.
///
/// Parsing context adds certain limitations for the textual content. E.g., it's unsafe to
/// rewrite text inside `<script>` element with a string that contains `"</script>"` substring as
/// this will preemptively close the `<script>` element, possibly introducing an XSS attack vector.
/// As other example, some parsing contexts don't allow [HTML entities] in text. Thus, rewriting
/// content of a `<style>` element with text that contains HTML entities may cause a CSS parsing
/// error in a browser, because entities won't be decoded by a browser in this context.
///
/// Text type provides users of the rewriter with a capability to assess the context in which text
/// parsing is hapenning and make informed decision about preprocessing of the textual content
/// replacement.
///
/// The names of the text types are taken from the [HTML parsing specification].
///
/// [HTML entities]: https://developer.mozilla.org/en-US/docs/Glossary/Entity
/// [HTML parsing specification]: https://html.spec.whatwg.org/multipage/parsing.html
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum TextType {
    /// Text inside a `<plaintext>` element.
    ///
    /// All text is interpreter literally. There's no escaping possible.
    PlainText,
    /// Text inside `<title>` and `<textarea>` elements.
    ///
    /// It may contain HTML entities. It's similar to `Data`, but syntax of tags (other than the closing tag for the element) is interpreted as text.
    RCData,
    /// Text inside `<style>`, `<xmp>`, `<iframe>`, `<noembed>`, `<noframes>` and
    /// `<noscript>` elements.
    ///
    /// This text does not support escaping with HTML entities. It must not contain any text that looks like a closing tag.
    RawText,
    /// Text inside a `<script>` element.
    ///
    /// This text does not support escaping with HTML entities. It must not contain any text that looks like a closing tag.
    ScriptData,
    /// Regular text.
    ///
    /// It may contain HTML entities. `<` should be escaped as `&lt;`, and literal `&` should be escaped as `&amp;`.
    Data,
    /// Text inside a [CDATA section].
    ///
    /// This text does not support escaping with HTML entities. `]]>` must be escaped, e.g. with `]]]]><![CDATA[>`.
    ///
    /// [CDATA section]: https://developer.mozilla.org/en-US/docs/Web/API/CDATASection
    CDataSection,
}

impl TextType {
    /// Returns `true` if the text type allows [HTML entities].
    ///
    /// [HTML entities]: https://developer.mozilla.org/en-US/docs/Glossary/Entity
    #[inline]
    #[must_use]
    pub fn allows_html_entities(self) -> bool {
        self == Self::Data || self == Self::RCData
    }
}

cfg_if! {
    if #[cfg(feature = "integration_test")] {
        impl TextType {
            #[must_use] pub fn should_replace_unsafe_null_in_text(self) -> bool {
                self != Self::Data && self != Self::CDataSection
            }
        }

        #[allow(clippy::fallible_impl_from)]
        impl<'s> From<&'s str> for TextType {
            fn from(text_type: &'s str) -> Self {
                match text_type {
                    "Data state" => Self::Data,
                    "PLAINTEXT state" => Self::PlainText,
                    "RCDATA state" => Self::RCData,
                    "RAWTEXT state" => Self::RawText,
                    "Script data state" => Self::ScriptData,
                    "CDATA section state" => Self::CDataSection,
                    _ => panic!("Unknown text type"),
                }
            }
        }
    }
}
