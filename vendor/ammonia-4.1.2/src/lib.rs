// Copyright (C) Michael Howell and others
// this library is released under the same terms as Rust itself.

#![deny(unsafe_code)]
#![deny(missing_docs)]

//! Ammonia is a whitelist-based HTML sanitization library. It is designed to
//! prevent cross-site scripting, layout breaking, and clickjacking caused
//! by untrusted user-provided HTML being mixed into a larger web page.
//!
//! Ammonia uses [html5ever] to parse and serialize document fragments the same way browsers do,
//! so it is extremely resilient to syntactic obfuscation.
//!
//! Ammonia parses its input exactly according to the HTML5 specification;
//! it will not linkify bare URLs, insert line or paragraph breaks, or convert `(C)` into &copy;.
//! If you want that, use a markup processor before running the sanitizer, like [pulldown-cmark].
//!
//! # Examples
//!
//! ```
//! let result = ammonia::clean(
//!     "<b><img src='' onerror=alert('hax')>I'm not trying to XSS you</b>"
//! );
//! assert_eq!(result, "<b><img src=\"\">I'm not trying to XSS you</b>");
//! ```
//!
//! [html5ever]: https://github.com/servo/html5ever "The HTML parser in Servo"
//! [pulldown-cmark]: https://github.com/google/pulldown-cmark "CommonMark parser"

#[cfg(ammonia_unstable)]
pub mod rcdom;

#[cfg(not(ammonia_unstable))]
mod rcdom;

mod style;

use html5ever::interface::Attribute;
use html5ever::serialize::{serialize, SerializeOpts};
use html5ever::tree_builder::{NodeOrText, TreeSink};
use html5ever::{driver as html, local_name, ns, QualName};
use maplit::{hashmap, hashset};
use std::sync::LazyLock;
use rcdom::{Handle, NodeData, RcDom, SerializableHandle};
use std::borrow::{Borrow, Cow};
use std::cell::Cell;
use std::cmp::max;
use std::collections::{HashMap, HashSet};
use std::fmt::{self, Display};
use std::io;
use std::iter::IntoIterator as IntoIter;
use std::mem;
use std::rc::Rc;
use std::str::FromStr;
use tendril::stream::TendrilSink;
use tendril::StrTendril;
use tendril::{format_tendril, ByteTendril};
pub use url::Url;

use html5ever::buffer_queue::BufferQueue;
use html5ever::tokenizer::{Token, TokenSink, TokenSinkResult, Tokenizer};
pub use url;

static AMMONIA: LazyLock<Builder<'static>> = LazyLock::new(Builder::default);

/// Clean HTML with a conservative set of defaults.
///
/// * [tags](struct.Builder.html#defaults)
/// * [`script` and `style` have their contents stripped](struct.Builder.html#defaults-1)
/// * [attributes on specific tags](struct.Builder.html#defaults-2)
/// * [attributes on all tags](struct.Builder.html#defaults-6)
/// * [url schemes](struct.Builder.html#defaults-7)
/// * [relative URLs are passed through, unchanged, by default](struct.Builder.html#defaults-8)
/// * [links are marked `noopener noreferrer` by default](struct.Builder.html#defaults-9)
/// * all `class=""` settings are blocked by default
/// * comments are stripped by default
/// * no generic attribute prefixes are turned on by default
/// * no specific tag-attribute-value settings are configured by default
///
/// [opener]: https://mathiasbynens.github.io/rel-noopener/
/// [referrer]: https://en.wikipedia.org/wiki/HTTP_referer
///
/// # Examples
///
///     assert_eq!(ammonia::clean("XSS<script>attack</script>"), "XSS")
pub fn clean(src: &str) -> String {
    AMMONIA.clean(src).to_string()
}

/// Turn an arbitrary string into unformatted HTML.
///
/// This function is roughly equivalent to PHP's `htmlspecialchars` and `htmlentities`.
/// It is as strict as possible, encoding every character that has special meaning to the
/// HTML parser.
///
/// # Warnings
///
/// This function cannot be used to package strings into a `<script>` or `<style>` tag;
/// you need a JavaScript or CSS escaper to do that.
///
///     // DO NOT DO THIS
///     # use ammonia::clean_text;
///     let untrusted = "Robert\"); abuse();//";
///     let html = format!("<script>invoke(\"{}\")</script>", clean_text(untrusted));
///
/// `<textarea>` tags will strip the first newline, if present, even if that newline is encoded.
/// If you want to build an editor that works the way most folks expect them to, you should put a
/// newline at the beginning of the tag, like this:
///
///     # use ammonia::{Builder, clean_text};
///     let untrusted = "\n\nhi!";
///     let mut b = Builder::new();
///     b.add_tags(&["textarea"]);
///     // This is the bad version
///     // The user put two newlines at the beginning, but the first one was removed
///     let sanitized = b.clean(&format!("<textarea>{}</textarea>", clean_text(untrusted))).to_string();
///     assert_eq!("<textarea>\nhi!</textarea>", sanitized);
///     // This is a good version
///     // The user put two newlines at the beginning, and we add a third one,
///     // so the result still has two
///     let sanitized = b.clean(&format!("<textarea>\n{}</textarea>", clean_text(untrusted))).to_string();
///     assert_eq!("<textarea>\n\nhi!</textarea>", sanitized);
///     // This version is also often considered good
///     // For many applications, leading and trailing whitespace is probably unwanted
///     let sanitized = b.clean(&format!("<textarea>{}</textarea>", clean_text(untrusted.trim()))).to_string();
///     assert_eq!("<textarea>hi!</textarea>", sanitized);
///
/// It also does not make user text safe for HTML attribute microsyntaxes such as `class` or `id`.
/// Only use this function for places where HTML accepts unrestricted text such as `title` attributes
/// and paragraph contents.
pub fn clean_text(src: &str) -> String {
    let mut ret_val = String::with_capacity(max(4, src.len()));
    for c in src.chars() {
        let replacement = match c {
            // this character, when confronted, will start a tag
            '<' => "&lt;",
            // in an unquoted attribute, will end the attribute value
            '>' => "&gt;",
            // in an attribute surrounded by double quotes, this character will end the attribute value
            '\"' => "&quot;",
            // in an attribute surrounded by single quotes, this character will end the attribute value
            '\'' => "&apos;",
            // in HTML5, returns a bogus parse error in an unquoted attribute, while in SGML/HTML, it will end an attribute value surrounded by backquotes
            '`' => "&grave;",
            // in an unquoted attribute, this character will end the attribute
            '/' => "&#47;",
            // starts an entity reference
            '&' => "&amp;",
            // if at the beginning of an unquoted attribute, will get ignored
            '=' => "&#61;",
            // will end an unquoted attribute
            ' ' => "&#32;",
            '\t' => "&#9;",
            '\n' => "&#10;",
            '\x0c' => "&#12;",
            '\r' => "&#13;",
            // a spec-compliant browser will perform this replacement anyway, but the middleware might not
            '\0' => "&#65533;",
            // ALL OTHER CHARACTERS ARE PASSED THROUGH VERBATIM
            _ => {
                ret_val.push(c);
                continue;
            }
        };
        ret_val.push_str(replacement);
    }
    ret_val
}

/// Determine if a given string contains HTML
///
/// This function is parses the full string into HTML and checks if the input contained any
/// HTML syntax.
///
/// # Note
/// This function will return positively for strings that contain invalid HTML syntax like
/// `<g>` and even `Vec::<u8>::new()`.
pub fn is_html(input: &str) -> bool {
    let santok = SanitizationTokenizer::new();
    let mut chunk = ByteTendril::new();
    chunk.push_slice(input.as_bytes());
    let mut input = BufferQueue::default();
    input.push_back(chunk.try_reinterpret().unwrap());

    let tok = Tokenizer::new(santok, Default::default());
    let _ = tok.feed(&mut input);
    tok.end();
    tok.sink.was_sanitized.get()
}

#[derive(Clone)]
struct SanitizationTokenizer {
    was_sanitized: Cell<bool>,
}

impl SanitizationTokenizer {
    pub fn new() -> SanitizationTokenizer {
        SanitizationTokenizer {
            was_sanitized: false.into(),
        }
    }
}

impl TokenSink for SanitizationTokenizer {
    type Handle = ();
    fn process_token(&self, token: Token, _line_number: u64) -> TokenSinkResult<()> {
        match token {
            Token::CharacterTokens(_) | Token::EOFToken | Token::ParseError(_) => {}
            _ => {
                self.was_sanitized.set(true);
            }
        }
        TokenSinkResult::Continue
    }
    fn end(&self) {}
}

/// An HTML sanitizer.
///
/// Given a fragment of HTML, Ammonia will parse it according to the HTML5
/// parsing algorithm and sanitize any disallowed tags or attributes. This
/// algorithm also takes care of things like unclosed and (some) misnested
/// tags.
///
/// # Examples
///
///     use ammonia::{Builder, UrlRelative};
///
///     let a = Builder::default()
///         .link_rel(None)
///         .url_relative(UrlRelative::PassThrough)
///         .clean("<a href=/>test")
///         .to_string();
///     assert_eq!(
///         a,
///         "<a href=\"/\">test</a>");
///
/// # Panics
///
/// Running [`clean`] or [`clean_from_reader`] may cause a panic if the builder is
/// configured with any of these (contradictory) settings:
///
///  * The `rel` attribute is added to [`generic_attributes`] or the
///    [`tag_attributes`] for the `<a>` tag, and [`link_rel`] is not set to `None`.
///
///    For example, this is going to panic, since [`link_rel`] is set  to
///    `Some("noopener noreferrer")` by default,
///    and it makes no sense to simultaneously say that the user is allowed to
///    set their own `rel` attribute while saying that every link shall be set to
///    a particular value:
///
///    ```should_panic
///    use ammonia::Builder;
///    use maplit::hashset;
///
///    # fn main() {
///    Builder::default()
///        .generic_attributes(hashset!["rel"])
///        .clean("");
///    # }
///    ```
///
///    This, however, is perfectly valid:
///
///    ```
///    use ammonia::Builder;
///    use maplit::hashset;
///
///    # fn main() {
///    Builder::default()
///        .generic_attributes(hashset!["rel"])
///        .link_rel(None)
///        .clean("");
///    # }
///    ```
///
///  * The `class` attribute is in [`allowed_classes`] and is in the
///    corresponding [`tag_attributes`] or in [`generic_attributes`].
///
///    This is done both to line up with the treatment of `rel`,
///    and to prevent people from accidentally allowing arbitrary
///    classes on a particular element.
///
///    This will panic:
///
///    ```should_panic
///    use ammonia::Builder;
///    use maplit::{hashmap, hashset};
///
///    # fn main() {
///    Builder::default()
///        .generic_attributes(hashset!["class"])
///        .allowed_classes(hashmap!["span" => hashset!["hidden"]])
///        .clean("");
///    # }
///    ```
///
///    This, however, is perfectly valid:
///
///    ```
///    use ammonia::Builder;
///    use maplit::{hashmap, hashset};
///
///    # fn main() {
///    Builder::default()
///        .allowed_classes(hashmap!["span" => hashset!["hidden"]])
///        .clean("");
///    # }
///    ```
///
///  * A tag is in either [`tags`] or [`tag_attributes`] while also
///    being in [`clean_content_tags`].
///
///    Both [`tags`] and [`tag_attributes`] are whitelists but
///    [`clean_content_tags`] is a blacklist, so it doesn't make sense
///    to have the same tag in both.
///
///    For example, this will panic, since the `aside` tag is in
///    [`tags`] by default:
///
///    ```should_panic
///    use ammonia::Builder;
///    use maplit::hashset;
///
///    # fn main() {
///    Builder::default()
///        .clean_content_tags(hashset!["aside"])
///        .clean("");
///    # }
///    ```
///
///    This, however, is valid:
///
///    ```
///    use ammonia::Builder;
///    use maplit::hashset;
///
///    # fn main() {
///    Builder::default()
///        .rm_tags(&["aside"])
///        .clean_content_tags(hashset!["aside"])
///        .clean("");
///    # }
///    ```
///
/// [`clean`]: #method.clean
/// [`clean_from_reader`]: #method.clean_from_reader
/// [`generic_attributes`]: #method.generic_attributes
/// [`tag_attributes`]: #method.tag_attributes
/// [`generic_attributes`]: #method.generic_attributes
/// [`link_rel`]: #method.link_rel
/// [`allowed_classes`]: #method.allowed_classes
/// [`id_prefix`]: #method.id_prefix
/// [`tags`]: #method.tags
/// [`clean_content_tags`]: #method.clean_content_tags
#[derive(Debug)]
pub struct Builder<'a> {
    tags: HashSet<&'a str>,
    clean_content_tags: HashSet<&'a str>,
    tag_attributes: HashMap<&'a str, HashSet<&'a str>>,
    tag_attribute_values: HashMap<&'a str, HashMap<&'a str, HashSet<&'a str>>>,
    set_tag_attribute_values: HashMap<&'a str, HashMap<&'a str, &'a str>>,
    generic_attributes: HashSet<&'a str>,
    url_schemes: HashSet<&'a str>,
    url_relative: UrlRelative<'a>,
    attribute_filter: Option<Box<dyn AttributeFilter>>,
    link_rel: Option<&'a str>,
    allowed_classes: HashMap<&'a str, HashSet<&'a str>>,
    strip_comments: bool,
    id_prefix: Option<&'a str>,
    generic_attribute_prefixes: Option<HashSet<&'a str>>,
    style_properties: Option<HashSet<&'a str>>,
}

impl<'a> Default for Builder<'a> {
    fn default() -> Self {
        #[rustfmt::skip]
        let tags = hashset![
            "a", "abbr", "acronym", "area", "article", "aside", "b", "bdi",
            "bdo", "blockquote", "br", "caption", "center", "cite", "code",
            "col", "colgroup", "data", "dd", "del", "details", "dfn", "div",
            "dl", "dt", "em", "figcaption", "figure", "footer", "h1", "h2",
            "h3", "h4", "h5", "h6", "header", "hgroup", "hr", "i", "img",
            "ins", "kbd", "li", "map", "mark", "nav", "ol", "p", "pre",
            "q", "rp", "rt", "rtc", "ruby", "s", "samp", "small", "span",
            "strike", "strong", "sub", "summary", "sup", "table", "tbody",
            "td", "th", "thead", "time", "tr", "tt", "u", "ul", "var", "wbr"
        ];
        let clean_content_tags = hashset!["script", "style"];
        let generic_attributes = hashset!["lang", "title"];
        let tag_attributes = hashmap![
            "a" => hashset![
                "href", "hreflang"
            ],
            "bdo" => hashset![
                "dir"
            ],
            "blockquote" => hashset![
                "cite"
            ],
            "col" => hashset![
                "align", "char", "charoff", "span"
            ],
            "colgroup" => hashset![
                "align", "char", "charoff", "span"
            ],
            "del" => hashset![
                "cite", "datetime"
            ],
            "hr" => hashset![
                "align", "size", "width"
            ],
            "img" => hashset![
                "align", "alt", "height", "src", "width"
            ],
            "ins" => hashset![
                "cite", "datetime"
            ],
            "ol" => hashset![
                "start"
            ],
            "q" => hashset![
                "cite"
            ],
            "table" => hashset![
                "align", "char", "charoff", "summary"
            ],
            "tbody" => hashset![
                "align", "char", "charoff"
            ],
            "td" => hashset![
                "align", "char", "charoff", "colspan", "headers", "rowspan"
            ],
            "tfoot" => hashset![
                "align", "char", "charoff"
            ],
            "th" => hashset![
                "align", "char", "charoff", "colspan", "headers", "rowspan", "scope"
            ],
            "thead" => hashset![
                "align", "char", "charoff"
            ],
            "tr" => hashset![
                "align", "char", "charoff"
            ],
        ];
        let tag_attribute_values = hashmap![];
        let set_tag_attribute_values = hashmap![];
        let url_schemes = hashset![
            "bitcoin",
            "ftp",
            "ftps",
            "geo",
            "http",
            "https",
            "im",
            "irc",
            "ircs",
            "magnet",
            "mailto",
            "mms",
            "mx",
            "news",
            "nntp",
            "openpgp4fpr",
            "sip",
            "sms",
            "smsto",
            "ssh",
            "tel",
            "url",
            "webcal",
            "wtai",
            "xmpp"
        ];
        let allowed_classes = hashmap![];

        Builder {
            tags,
            clean_content_tags,
            tag_attributes,
            tag_attribute_values,
            set_tag_attribute_values,
            generic_attributes,
            url_schemes,
            url_relative: UrlRelative::PassThrough,
            attribute_filter: None,
            link_rel: Some("noopener noreferrer"),
            allowed_classes,
            strip_comments: true,
            id_prefix: None,
            generic_attribute_prefixes: None,
            style_properties: None,
        }
    }
}

impl<'a> Builder<'a> {
    /// Sets the tags that are allowed.
    ///
    /// # Examples
    ///
    ///     use ammonia::Builder;
    ///     use maplit::hashset;
    ///
    ///     # fn main() {
    ///     let tags = hashset!["my-tag"];
    ///     let a = Builder::new()
    ///         .tags(tags)
    ///         .clean("<my-tag>")
    ///         .to_string();
    ///     assert_eq!(a, "<my-tag></my-tag>");
    ///     # }
    ///
    /// # Defaults
    ///
    /// ```notest
    /// a, abbr, acronym, area, article, aside, b, bdi,
    /// bdo, blockquote, br, caption, center, cite, code,
    /// col, colgroup, data, dd, del, details, dfn, div,
    /// dl, dt, em, figcaption, figure, footer, h1, h2,
    /// h3, h4, h5, h6, header, hgroup, hr, i, img,
    /// ins, kbd, li, map, mark, nav, ol, p, pre,
    /// q, rp, rt, rtc, ruby, s, samp, small, span,
    /// strike, strong, sub, summary, sup, table, tbody,
    /// td, th, thead, time, tr, tt, u, ul, var, wbr
    /// ```
    pub fn tags(&mut self, value: HashSet<&'a str>) -> &mut Self {
        self.tags = value;
        self
    }

    /// Add additonal whitelisted tags without overwriting old ones.
    ///
    /// Does nothing if the tag is already there.
    ///
    /// # Examples
    ///
    ///     let a = ammonia::Builder::default()
    ///         .add_tags(&["my-tag"])
    ///         .clean("<my-tag>test</my-tag> <span>mess</span>").to_string();
    ///     assert_eq!("<my-tag>test</my-tag> <span>mess</span>", a);
    pub fn add_tags<T: 'a + ?Sized + Borrow<str>, I: IntoIter<Item = &'a T>>(
        &mut self,
        it: I,
    ) -> &mut Self {
        self.tags.extend(it.into_iter().map(Borrow::borrow));
        self
    }

    /// Remove already-whitelisted tags.
    ///
    /// Does nothing if the tags is already gone.
    ///
    /// # Examples
    ///
    ///     let a = ammonia::Builder::default()
    ///         .rm_tags(&["span"])
    ///         .clean("<span></span>").to_string();
    ///     assert_eq!("", a);
    pub fn rm_tags<'b, T: 'b + ?Sized + Borrow<str>, I: IntoIter<Item = &'b T>>(
        &mut self,
        it: I,
    ) -> &mut Self {
        for i in it {
            self.tags.remove(i.borrow());
        }
        self
    }

    /// Returns a copy of the set of whitelisted tags.
    ///
    /// # Examples
    ///
    ///     use maplit::hashset;
    ///
    ///     let tags = hashset!["my-tag-1", "my-tag-2"];
    ///
    ///     let mut b = ammonia::Builder::default();
    ///     b.tags(Clone::clone(&tags));
    ///     assert_eq!(tags, b.clone_tags());
    pub fn clone_tags(&self) -> HashSet<&'a str> {
        self.tags.clone()
    }

    /// Sets the tags whose contents will be completely removed from the output.
    ///
    /// Adding tags which are whitelisted in `tags` or `tag_attributes` will cause
    /// a panic.
    ///
    /// # Examples
    ///
    ///     use ammonia::Builder;
    ///     use maplit::hashset;
    ///
    ///     # fn main() {
    ///     let tag_blacklist = hashset!["script", "style"];
    ///     let a = Builder::new()
    ///         .clean_content_tags(tag_blacklist)
    ///         .clean("<script>alert('hello')</script><style>a { background: #fff }</style>")
    ///         .to_string();
    ///     assert_eq!(a, "");
    ///     # }
    ///
    /// # Defaults
    ///
    /// ```notest
    /// script, style
    /// ```
    pub fn clean_content_tags(&mut self, value: HashSet<&'a str>) -> &mut Self {
        self.clean_content_tags = value;
        self
    }

    /// Add additonal blacklisted clean-content tags without overwriting old ones.
    ///
    /// Does nothing if the tag is already there.
    ///
    /// Adding tags which are whitelisted in `tags` or `tag_attributes` will cause
    /// a panic.
    ///
    /// # Examples
    ///
    ///     let a = ammonia::Builder::default()
    ///         .add_clean_content_tags(&["my-tag"])
    ///         .clean("<my-tag>test</my-tag><span>mess</span>").to_string();
    ///     assert_eq!("<span>mess</span>", a);
    pub fn add_clean_content_tags<T: 'a + ?Sized + Borrow<str>, I: IntoIter<Item = &'a T>>(
        &mut self,
        it: I,
    ) -> &mut Self {
        self.clean_content_tags
            .extend(it.into_iter().map(Borrow::borrow));
        self
    }

    /// Remove already-blacklisted clean-content tags.
    ///
    /// Does nothing if the tags aren't blacklisted.
    ///
    /// # Examples
    ///     use ammonia::Builder;
    ///     use maplit::hashset;
    ///
    ///     # fn main() {
    ///     let tag_blacklist = hashset!["script"];
    ///     let a = ammonia::Builder::default()
    ///         .clean_content_tags(tag_blacklist)
    ///         .rm_clean_content_tags(&["script"])
    ///         .clean("<script>XSS</script>").to_string();
    ///     assert_eq!("XSS", a);
    ///     # }
    pub fn rm_clean_content_tags<'b, T: 'b + ?Sized + Borrow<str>, I: IntoIter<Item = &'b T>>(
        &mut self,
        it: I,
    ) -> &mut Self {
        for i in it {
            self.clean_content_tags.remove(i.borrow());
        }
        self
    }

    /// Returns a copy of the set of blacklisted clean-content tags.
    ///
    /// # Examples
    ///     # use maplit::hashset;
    ///
    ///     let tags = hashset!["my-tag-1", "my-tag-2"];
    ///
    ///     let mut b = ammonia::Builder::default();
    ///     b.clean_content_tags(Clone::clone(&tags));
    ///     assert_eq!(tags, b.clone_clean_content_tags());
    pub fn clone_clean_content_tags(&self) -> HashSet<&'a str> {
        self.clean_content_tags.clone()
    }

    /// Sets the HTML attributes that are allowed on specific tags.
    ///
    /// The value is structured as a map from tag names to a set of attribute names.
    ///
    /// If a tag is not itself whitelisted, adding entries to this map will do nothing.
    ///
    /// # Examples
    ///
    ///     use ammonia::Builder;
    ///     use maplit::{hashmap, hashset};
    ///
    ///     # fn main() {
    ///     let tags = hashset!["my-tag"];
    ///     let tag_attributes = hashmap![
    ///         "my-tag" => hashset!["val"]
    ///     ];
    ///     let a = Builder::new().tags(tags).tag_attributes(tag_attributes)
    ///         .clean("<my-tag val=1>")
    ///         .to_string();
    ///     assert_eq!(a, "<my-tag val=\"1\"></my-tag>");
    ///     # }
    ///
    /// # Defaults
    ///
    /// ```notest
    /// a =>
    ///     href, hreflang
    /// bdo =>
    ///     dir
    /// blockquote =>
    ///     cite
    /// col =>
    ///     align, char, charoff, span
    /// colgroup =>
    ///     align, char, charoff, span
    /// del =>
    ///     cite, datetime
    /// hr =>
    ///     align, size, width
    /// img =>
    ///     align, alt, height, src, width
    /// ins =>
    ///     cite, datetime
    /// ol =>
    ///     start
    /// q =>
    ///     cite
    /// table =>
    ///     align, char, charoff, summary
    /// tbody =>
    ///     align, char, charoff
    /// td =>
    ///     align, char, charoff, colspan, headers, rowspan
    /// tfoot =>
    ///     align, char, charoff
    /// th =>
    ///     align, char, charoff, colspan, headers, rowspan, scope
    /// thead =>
    ///     align, char, charoff
    /// tr =>
    ///     align, char, charoff
    /// ```
    pub fn tag_attributes(&mut self, value: HashMap<&'a str, HashSet<&'a str>>) -> &mut Self {
        self.tag_attributes = value;
        self
    }

    /// Add additonal whitelisted tag-specific attributes without overwriting old ones.
    ///
    /// # Examples
    ///
    ///     let a = ammonia::Builder::default()
    ///         .add_tags(&["my-tag"])
    ///         .add_tag_attributes("my-tag", &["my-attr"])
    ///         .clean("<my-tag my-attr>test</my-tag> <span>mess</span>").to_string();
    ///     assert_eq!("<my-tag my-attr=\"\">test</my-tag> <span>mess</span>", a);
    pub fn add_tag_attributes<
        T: 'a + ?Sized + Borrow<str>,
        U: 'a + ?Sized + Borrow<str>,
        I: IntoIter<Item = &'a T>,
    >(
        &mut self,
        tag: &'a U,
        it: I,
    ) -> &mut Self {
        self.tag_attributes
            .entry(tag.borrow())
            .or_default()
            .extend(it.into_iter().map(Borrow::borrow));
        self
    }

    /// Remove already-whitelisted tag-specific attributes.
    ///
    /// Does nothing if the attribute is already gone.
    ///
    /// # Examples
    ///
    ///     let a = ammonia::Builder::default()
    ///         .rm_tag_attributes("a", &["href"])
    ///         .clean("<a href=\"/\"></a>").to_string();
    ///     assert_eq!("<a rel=\"noopener noreferrer\"></a>", a);
    pub fn rm_tag_attributes<
        'b,
        'c,
        T: 'b + ?Sized + Borrow<str>,
        U: 'c + ?Sized + Borrow<str>,
        I: IntoIter<Item = &'b T>,
    >(
        &mut self,
        tag: &'c U,
        it: I,
    ) -> &mut Self {
        if let Some(tag) = self.tag_attributes.get_mut(tag.borrow()) {
            for i in it {
                tag.remove(i.borrow());
            }
        }
        self
    }

    /// Returns a copy of the set of whitelisted tag-specific attributes.
    ///
    /// # Examples
    ///     use maplit::{hashmap, hashset};
    ///
    ///     let tag_attributes = hashmap![
    ///         "my-tag" => hashset!["my-attr-1", "my-attr-2"]
    ///     ];
    ///
    ///     let mut b = ammonia::Builder::default();
    ///     b.tag_attributes(Clone::clone(&tag_attributes));
    ///     assert_eq!(tag_attributes, b.clone_tag_attributes());
    pub fn clone_tag_attributes(&self) -> HashMap<&'a str, HashSet<&'a str>> {
        self.tag_attributes.clone()
    }

    /// Sets the values of HTML attributes that are allowed on specific tags.
    ///
    /// The value is structured as a map from tag names to a map from attribute names to a set of
    /// attribute values.
    ///
    /// If a tag is not itself whitelisted, adding entries to this map will do nothing.
    ///
    /// # Examples
    ///
    ///     use ammonia::Builder;
    ///     use maplit::{hashmap, hashset};
    ///
    ///     # fn main() {
    ///     let tags = hashset!["my-tag"];
    ///     let tag_attribute_values = hashmap![
    ///         "my-tag" => hashmap![
    ///             "my-attr" => hashset!["val"],
    ///         ],
    ///     ];
    ///     let a = Builder::new().tags(tags).tag_attribute_values(tag_attribute_values)
    ///         .clean("<my-tag my-attr=val>")
    ///         .to_string();
    ///     assert_eq!(a, "<my-tag my-attr=\"val\"></my-tag>");
    ///     # }
    ///
    /// # Defaults
    ///
    /// None.
    pub fn tag_attribute_values(
        &mut self,
        value: HashMap<&'a str, HashMap<&'a str, HashSet<&'a str>>>,
    ) -> &mut Self {
        self.tag_attribute_values = value;
        self
    }

    /// Add additonal whitelisted tag-specific attribute values without overwriting old ones.
    ///
    /// # Examples
    ///
    ///     let a = ammonia::Builder::default()
    ///         .add_tags(&["my-tag"])
    ///         .add_tag_attribute_values("my-tag", "my-attr", &[""])
    ///         .clean("<my-tag my-attr>test</my-tag> <span>mess</span>").to_string();
    ///     assert_eq!("<my-tag my-attr=\"\">test</my-tag> <span>mess</span>", a);
    pub fn add_tag_attribute_values<
        T: 'a + ?Sized + Borrow<str>,
        U: 'a + ?Sized + Borrow<str>,
        V: 'a + ?Sized + Borrow<str>,
        I: IntoIter<Item = &'a T>,
    >(
        &mut self,
        tag: &'a U,
        attribute: &'a V,
        it: I,
    ) -> &mut Self {
        self.tag_attribute_values
            .entry(tag.borrow())
            .or_default()
            .entry(attribute.borrow())
            .or_default()
            .extend(it.into_iter().map(Borrow::borrow));

        self
    }

    /// Remove already-whitelisted tag-specific attribute values.
    ///
    /// Does nothing if the attribute or the value is already gone.
    ///
    /// # Examples
    ///
    ///     let a = ammonia::Builder::default()
    ///         .rm_tag_attributes("a", &["href"])
    ///         .add_tag_attribute_values("a", "href", &["/"])
    ///         .rm_tag_attribute_values("a", "href", &["/"])
    ///         .clean("<a href=\"/\"></a>").to_string();
    ///     assert_eq!("<a rel=\"noopener noreferrer\"></a>", a);
    pub fn rm_tag_attribute_values<
        'b,
        'c,
        T: 'b + ?Sized + Borrow<str>,
        U: 'c + ?Sized + Borrow<str>,
        V: 'c + ?Sized + Borrow<str>,
        I: IntoIter<Item = &'b T>,
    >(
        &mut self,
        tag: &'c U,
        attribute: &'c V,
        it: I,
    ) -> &mut Self {
        if let Some(attrs) = self
            .tag_attribute_values
            .get_mut(tag.borrow())
            .and_then(|map| map.get_mut(attribute.borrow()))
        {
            for i in it {
                attrs.remove(i.borrow());
            }
        }
        self
    }

    /// Returns a copy of the set of whitelisted tag-specific attribute values.
    ///
    /// # Examples
    ///
    ///     use maplit::{hashmap, hashset};
    ///
    ///     let attribute_values = hashmap![
    ///         "my-attr-1" => hashset!["foo"],
    ///         "my-attr-2" => hashset!["baz", "bar"],
    ///     ];
    ///     let tag_attribute_values = hashmap![
    ///         "my-tag" => attribute_values
    ///     ];
    ///
    ///     let mut b = ammonia::Builder::default();
    ///     b.tag_attribute_values(Clone::clone(&tag_attribute_values));
    ///     assert_eq!(tag_attribute_values, b.clone_tag_attribute_values());
    pub fn clone_tag_attribute_values(
        &self,
    ) -> HashMap<&'a str, HashMap<&'a str, HashSet<&'a str>>> {
        self.tag_attribute_values.clone()
    }

    /// Sets the values of HTML attributes that are to be set on specific tags.
    ///
    /// The value is structured as a map from tag names to a map from attribute names to an
    /// attribute value.
    ///
    /// If a tag is not itself whitelisted, adding entries to this map will do nothing.
    ///
    /// # Examples
    ///
    ///     use ammonia::Builder;
    ///     use maplit::{hashmap, hashset};
    ///
    ///     # fn main() {
    ///     let tags = hashset!["my-tag"];
    ///     let set_tag_attribute_values = hashmap![
    ///         "my-tag" => hashmap![
    ///             "my-attr" => "val",
    ///         ],
    ///     ];
    ///     let a = Builder::new().tags(tags).set_tag_attribute_values(set_tag_attribute_values)
    ///         .clean("<my-tag>")
    ///         .to_string();
    ///     assert_eq!(a, "<my-tag my-attr=\"val\"></my-tag>");
    ///     # }
    ///
    /// # Defaults
    ///
    /// None.
    pub fn set_tag_attribute_values(
        &mut self,
        value: HashMap<&'a str, HashMap<&'a str, &'a str>>,
    ) -> &mut Self {
        self.set_tag_attribute_values = value;
        self
    }

    /// Add an attribute value to set on a specific element.
    ///
    /// # Examples
    ///
    ///     let a = ammonia::Builder::default()
    ///         .add_tags(&["my-tag"])
    ///         .set_tag_attribute_value("my-tag", "my-attr", "val")
    ///         .clean("<my-tag>test</my-tag> <span>mess</span>").to_string();
    ///     assert_eq!("<my-tag my-attr=\"val\">test</my-tag> <span>mess</span>", a);
    pub fn set_tag_attribute_value<
        T: 'a + ?Sized + Borrow<str>,
        A: 'a + ?Sized + Borrow<str>,
        V: 'a + ?Sized + Borrow<str>,
    >(
        &mut self,
        tag: &'a T,
        attribute: &'a A,
        value: &'a V,
    ) -> &mut Self {
        self.set_tag_attribute_values
            .entry(tag.borrow())
            .or_default()
            .insert(attribute.borrow(), value.borrow());
        self
    }

    /// Remove existing tag-specific attribute values to be set.
    ///
    /// Does nothing if the attribute is already gone.
    ///
    /// # Examples
    ///
    ///     let a = ammonia::Builder::default()
    ///         // this does nothing, since no value is set for this tag attribute yet
    ///         .rm_set_tag_attribute_value("a", "target")
    ///         .set_tag_attribute_value("a", "target", "_blank")
    ///         .rm_set_tag_attribute_value("a", "target")
    ///         .clean("<a href=\"/\"></a>").to_string();
    ///     assert_eq!("<a href=\"/\" rel=\"noopener noreferrer\"></a>", a);
    pub fn rm_set_tag_attribute_value<
        T: 'a + ?Sized + Borrow<str>,
        A: 'a + ?Sized + Borrow<str>,
    >(
        &mut self,
        tag: &'a T,
        attribute: &'a A,
    ) -> &mut Self {
        if let Some(attributes) = self.set_tag_attribute_values.get_mut(tag.borrow()) {
            attributes.remove(attribute.borrow());
        }
        self
    }

    /// Returns the value that will be set for the attribute on the element, if any.
    ///
    /// # Examples
    ///
    ///     let mut b = ammonia::Builder::default();
    ///     b.set_tag_attribute_value("a", "target", "_blank");
    ///     let value = b.get_set_tag_attribute_value("a", "target");
    ///     assert_eq!(value, Some("_blank"));
    pub fn get_set_tag_attribute_value<
        T: 'a + ?Sized + Borrow<str>,
        A: 'a + ?Sized + Borrow<str>,
    >(
        &self,
        tag: &'a T,
        attribute: &'a A,
    ) -> Option<&'a str> {
        self.set_tag_attribute_values
            .get(tag.borrow())
            .and_then(|map| map.get(attribute.borrow()))
            .copied()
    }

    /// Returns a copy of the set of tag-specific attribute values to be set.
    ///
    /// # Examples
    ///
    ///     use maplit::{hashmap, hashset};
    ///
    ///     let attribute_values = hashmap![
    ///         "my-attr-1" => "foo",
    ///         "my-attr-2" => "bar",
    ///     ];
    ///     let set_tag_attribute_values = hashmap![
    ///         "my-tag" => attribute_values,
    ///     ];
    ///
    ///     let mut b = ammonia::Builder::default();
    ///     b.set_tag_attribute_values(Clone::clone(&set_tag_attribute_values));
    ///     assert_eq!(set_tag_attribute_values, b.clone_set_tag_attribute_values());
    pub fn clone_set_tag_attribute_values(&self) -> HashMap<&'a str, HashMap<&'a str, &'a str>> {
        self.set_tag_attribute_values.clone()
    }

    /// Sets the prefix of attributes that are allowed on any tag.
    ///
    /// # Examples
    ///
    ///     use ammonia::Builder;
    ///     use maplit::hashset;
    ///
    ///     # fn main() {
    ///     let prefixes = hashset!["data-"];
    ///     let a = Builder::new()
    ///         .generic_attribute_prefixes(prefixes)
    ///         .clean("<b data-val=1>")
    ///         .to_string();
    ///     assert_eq!(a, "<b data-val=\"1\"></b>");
    ///     # }
    ///
    /// # Defaults
    ///
    /// No attribute prefixes are allowed by default.
    pub fn generic_attribute_prefixes(&mut self, value: HashSet<&'a str>) -> &mut Self {
        self.generic_attribute_prefixes = Some(value);
        self
    }

    /// Add additional whitelisted attribute prefix without overwriting old ones.
    ///
    /// # Examples
    ///
    ///     let a = ammonia::Builder::default()
    ///         .add_generic_attribute_prefixes(&["my-"])
    ///         .clean("<span my-attr>mess</span>").to_string();
    ///     assert_eq!("<span my-attr=\"\">mess</span>", a);
    pub fn add_generic_attribute_prefixes<
        T: 'a + ?Sized + Borrow<str>,
        I: IntoIter<Item = &'a T>,
    >(
        &mut self,
        it: I,
    ) -> &mut Self {
        self.generic_attribute_prefixes
            .get_or_insert_with(HashSet::new)
            .extend(it.into_iter().map(Borrow::borrow));
        self
    }

    /// Remove already-whitelisted attribute prefixes.
    ///
    /// Does nothing if the attribute prefix is already gone.
    ///
    /// # Examples
    ///
    ///     let a = ammonia::Builder::default()
    ///         .add_generic_attribute_prefixes(&["data-", "code-"])
    ///         .rm_generic_attribute_prefixes(&["data-"])
    ///         .clean("<span code-test=\"foo\" data-test=\"cool\"></span>").to_string();
    ///     assert_eq!("<span code-test=\"foo\"></span>", a);
    pub fn rm_generic_attribute_prefixes<
        'b,
        T: 'b + ?Sized + Borrow<str>,
        I: IntoIter<Item = &'b T>,
    >(
        &mut self,
        it: I,
    ) -> &mut Self {
        if let Some(true) = self.generic_attribute_prefixes.as_mut().map(|prefixes| {
            for i in it {
                let _ = prefixes.remove(i.borrow());
            }
            prefixes.is_empty()
        }) {
            self.generic_attribute_prefixes = None;
        }
        self
    }

    /// Returns a copy of the set of whitelisted attribute prefixes.
    ///
    /// # Examples
    ///
    ///     use maplit::hashset;
    ///
    ///     let generic_attribute_prefixes = hashset!["my-prfx-1-", "my-prfx-2-"];
    ///
    ///     let mut b = ammonia::Builder::default();
    ///     b.generic_attribute_prefixes(Clone::clone(&generic_attribute_prefixes));
    ///     assert_eq!(Some(generic_attribute_prefixes), b.clone_generic_attribute_prefixes());
    pub fn clone_generic_attribute_prefixes(&self) -> Option<HashSet<&'a str>> {
        self.generic_attribute_prefixes.clone()
    }

    /// Sets the attributes that are allowed on any tag.
    ///
    /// # Examples
    ///
    ///     use ammonia::Builder;
    ///     use maplit::hashset;
    ///
    ///     # fn main() {
    ///     let attributes = hashset!["data-val"];
    ///     let a = Builder::new()
    ///         .generic_attributes(attributes)
    ///         .clean("<b data-val=1>")
    ///         .to_string();
    ///     assert_eq!(a, "<b data-val=\"1\"></b>");
    ///     # }
    ///
    /// # Defaults
    ///
    /// ```notest
    /// lang, title
    /// ```
    pub fn generic_attributes(&mut self, value: HashSet<&'a str>) -> &mut Self {
        self.generic_attributes = value;
        self
    }

    /// Add additonal whitelisted attributes without overwriting old ones.
    ///
    /// # Examples
    ///
    ///     let a = ammonia::Builder::default()
    ///         .add_generic_attributes(&["my-attr"])
    ///         .clean("<span my-attr>mess</span>").to_string();
    ///     assert_eq!("<span my-attr=\"\">mess</span>", a);
    pub fn add_generic_attributes<T: 'a + ?Sized + Borrow<str>, I: IntoIter<Item = &'a T>>(
        &mut self,
        it: I,
    ) -> &mut Self {
        self.generic_attributes
            .extend(it.into_iter().map(Borrow::borrow));
        self
    }

    /// Remove already-whitelisted attributes.
    ///
    /// Does nothing if the attribute is already gone.
    ///
    /// # Examples
    ///
    ///     let a = ammonia::Builder::default()
    ///         .rm_generic_attributes(&["title"])
    ///         .clean("<span title=\"cool\"></span>").to_string();
    ///     assert_eq!("<span></span>", a);
    pub fn rm_generic_attributes<'b, T: 'b + ?Sized + Borrow<str>, I: IntoIter<Item = &'b T>>(
        &mut self,
        it: I,
    ) -> &mut Self {
        for i in it {
            self.generic_attributes.remove(i.borrow());
        }
        self
    }

    /// Returns a copy of the set of whitelisted attributes.
    ///
    /// # Examples
    ///
    ///     use maplit::hashset;
    ///
    ///     let generic_attributes = hashset!["my-attr-1", "my-attr-2"];
    ///
    ///     let mut b = ammonia::Builder::default();
    ///     b.generic_attributes(Clone::clone(&generic_attributes));
    ///     assert_eq!(generic_attributes, b.clone_generic_attributes());
    pub fn clone_generic_attributes(&self) -> HashSet<&'a str> {
        self.generic_attributes.clone()
    }

    /// Sets the URL schemes permitted on `href` and `src` attributes.
    ///
    /// # Examples
    ///
    ///     use ammonia::Builder;
    ///     use maplit::hashset;
    ///
    ///     # fn main() {
    ///     let url_schemes = hashset![
    ///         "http", "https", "mailto", "magnet"
    ///     ];
    ///     let a = Builder::new().url_schemes(url_schemes)
    ///         .clean("<a href=\"magnet:?xt=urn:ed2k:31D6CFE0D16AE931B73C59D7E0C089C0&xl=0&dn=zero_len.fil&xt=urn:bitprint:3I42H3S6NNFQ2MSVX7XZKYAYSCX5QBYJ.LWPNACQDBZRYXW3VHJVCJ64QBZNGHOHHHZWCLNQ&xt=urn:md5:D41D8CD98F00B204E9800998ECF8427E\">zero-length file</a>")
    ///         .to_string();
    ///
    ///     // See `link_rel` for information on the rel="noopener noreferrer" attribute
    ///     // in the cleaned HTML.
    ///     assert_eq!(a,
    ///       "<a href=\"magnet:?xt=urn:ed2k:31D6CFE0D16AE931B73C59D7E0C089C0&amp;xl=0&amp;dn=zero_len.fil&amp;xt=urn:bitprint:3I42H3S6NNFQ2MSVX7XZKYAYSCX5QBYJ.LWPNACQDBZRYXW3VHJVCJ64QBZNGHOHHHZWCLNQ&amp;xt=urn:md5:D41D8CD98F00B204E9800998ECF8427E\" rel=\"noopener noreferrer\">zero-length file</a>");
    ///     # }
    ///
    /// # Defaults
    ///
    /// ```notest
    /// bitcoin, ftp, ftps, geo, http, https, im, irc,
    /// ircs, magnet, mailto, mms, mx, news, nntp,
    /// openpgp4fpr, sip, sms, smsto, ssh, tel, url,
    /// webcal, wtai, xmpp
    /// ```
    pub fn url_schemes(&mut self, value: HashSet<&'a str>) -> &mut Self {
        self.url_schemes = value;
        self
    }

    /// Add additonal whitelisted URL schemes without overwriting old ones.
    ///
    /// # Examples
    ///
    ///     let a = ammonia::Builder::default()
    ///         .add_url_schemes(&["my-scheme"])
    ///         .clean("<a href=my-scheme:home>mess</span>").to_string();
    ///     assert_eq!("<a href=\"my-scheme:home\" rel=\"noopener noreferrer\">mess</a>", a);
    pub fn add_url_schemes<T: 'a + ?Sized + Borrow<str>, I: IntoIter<Item = &'a T>>(
        &mut self,
        it: I,
    ) -> &mut Self {
        self.url_schemes.extend(it.into_iter().map(Borrow::borrow));
        self
    }

    /// Remove already-whitelisted attributes.
    ///
    /// Does nothing if the attribute is already gone.
    ///
    /// # Examples
    ///
    ///     let a = ammonia::Builder::default()
    ///         .rm_url_schemes(&["ftp"])
    ///         .clean("<a href=\"ftp://ftp.mozilla.org/\"></a>").to_string();
    ///     assert_eq!("<a rel=\"noopener noreferrer\"></a>", a);
    pub fn rm_url_schemes<'b, T: 'b + ?Sized + Borrow<str>, I: IntoIter<Item = &'b T>>(
        &mut self,
        it: I,
    ) -> &mut Self {
        for i in it {
            self.url_schemes.remove(i.borrow());
        }
        self
    }

    /// Returns a copy of the set of whitelisted URL schemes.
    ///
    /// # Examples
    ///     use maplit::hashset;
    ///
    ///     let url_schemes = hashset!["my-scheme-1", "my-scheme-2"];
    ///
    ///     let mut b = ammonia::Builder::default();
    ///     b.url_schemes(Clone::clone(&url_schemes));
    ///     assert_eq!(url_schemes, b.clone_url_schemes());
    pub fn clone_url_schemes(&self) -> HashSet<&'a str> {
        self.url_schemes.clone()
    }

    /// Configures the behavior for relative URLs: pass-through, resolve-with-base, or deny.
    ///
    /// # Examples
    ///
    ///     use ammonia::{Builder, UrlRelative};
    ///
    ///     let a = Builder::new().url_relative(UrlRelative::PassThrough)
    ///         .clean("<a href=/>Home</a>")
    ///         .to_string();
    ///
    ///     // See `link_rel` for information on the rel="noopener noreferrer" attribute
    ///     // in the cleaned HTML.
    ///     assert_eq!(
    ///       a,
    ///       "<a href=\"/\" rel=\"noopener noreferrer\">Home</a>");
    ///
    /// # Defaults
    ///
    /// ```notest
    /// UrlRelative::PassThrough
    /// ```
    pub fn url_relative(&mut self, value: UrlRelative<'a>) -> &mut Self {
        self.url_relative = value;
        self
    }

    /// Allows rewriting of all attributes using a callback.
    ///
    /// The callback takes name of the element, attribute and its value.
    /// Returns `None` to remove the attribute, or a value to use.
    ///
    /// Rewriting of attributes with URLs is done before `url_relative()`.
    ///
    /// # Panics
    ///
    /// If more than one callback is set.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ammonia::Builder;
    /// let a = Builder::new()
    ///     .attribute_filter(|element, attribute, value| {
    ///         match (element, attribute) {
    ///             ("img", "src") => None,
    ///             _ => Some(value.into())
    ///         }
    ///     })
    ///     .link_rel(None)
    ///     .clean("<a href=/><img alt=Home src=foo></a>")
    ///     .to_string();
    /// assert_eq!(a,
    ///     r#"<a href="/"><img alt="Home"></a>"#);
    /// ```
    pub fn attribute_filter<'cb, CallbackFn>(&mut self, callback: CallbackFn) -> &mut Self
    where
        CallbackFn: for<'u> Fn(&str, &str, &'u str) -> Option<Cow<'u, str>> + Send + Sync + 'static,
    {
        assert!(
            self.attribute_filter.is_none(),
            "attribute_filter can be set only once"
        );
        self.attribute_filter = Some(Box::new(callback));
        self
    }

    /// Returns `true` if the relative URL resolver is set to `Deny`.
    ///
    /// # Examples
    ///
    ///     use ammonia::{Builder, UrlRelative};
    ///     let mut a = Builder::default();
    ///     a.url_relative(UrlRelative::Deny);
    ///     assert!(a.is_url_relative_deny());
    ///     a.url_relative(UrlRelative::PassThrough);
    ///     assert!(!a.is_url_relative_deny());
    pub fn is_url_relative_deny(&self) -> bool {
        matches!(self.url_relative, UrlRelative::Deny)
    }

    /// Returns `true` if the relative URL resolver is set to `PassThrough`.
    ///
    /// # Examples
    ///
    ///     use ammonia::{Builder, UrlRelative};
    ///     let mut a = Builder::default();
    ///     a.url_relative(UrlRelative::Deny);
    ///     assert!(!a.is_url_relative_pass_through());
    ///     a.url_relative(UrlRelative::PassThrough);
    ///     assert!(a.is_url_relative_pass_through());
    pub fn is_url_relative_pass_through(&self) -> bool {
        matches!(self.url_relative, UrlRelative::PassThrough)
    }

    /// Returns `true` if the relative URL resolver is set to `Custom`.
    ///
    /// # Examples
    ///
    ///     use ammonia::{Builder, UrlRelative};
    ///     use std::borrow::Cow;
    ///     fn test(a: &str) -> Option<Cow<str>> { None }
    ///     # fn main() {
    ///     let mut a = Builder::default();
    ///     a.url_relative(UrlRelative::Custom(Box::new(test)));
    ///     assert!(a.is_url_relative_custom());
    ///     a.url_relative(UrlRelative::PassThrough);
    ///     assert!(!a.is_url_relative_custom());
    ///     a.url_relative(UrlRelative::Deny);
    ///     assert!(!a.is_url_relative_custom());
    ///     # }
    pub fn is_url_relative_custom(&self) -> bool {
        matches!(self.url_relative, UrlRelative::Custom(_))
    }

    /// Configures a `rel` attribute that will be added on links.
    ///
    /// If `rel` is in the generic or tag attributes, this must be set to `None`.
    /// Common `rel` values to include:
    ///
    /// * `noopener`: This prevents [a particular type of XSS attack],
    ///   and should usually be turned on for untrusted HTML.
    /// * `noreferrer`: This prevents the browser from [sending the source URL]
    ///   to the website that is linked to.
    /// * `nofollow`: This prevents search engines from [using this link for
    ///   ranking], which disincentivizes spammers.
    ///
    /// To turn on rel-insertion, call this function with a space-separated list.
    /// Ammonia does not parse rel-attributes;
    /// it just puts the given string into the attribute directly.
    ///
    /// [a particular type of XSS attack]: https://mathiasbynens.github.io/rel-noopener/
    /// [sending the source URL]: https://en.wikipedia.org/wiki/HTTP_referer
    /// [using this link for ranking]: https://en.wikipedia.org/wiki/Nofollow
    ///
    /// # Examples
    ///
    ///     use ammonia::Builder;
    ///
    ///     let a = Builder::new().link_rel(None)
    ///         .clean("<a href=https://rust-lang.org/>Rust</a>")
    ///         .to_string();
    ///     assert_eq!(
    ///       a,
    ///       "<a href=\"https://rust-lang.org/\">Rust</a>");
    ///
    /// # Defaults
    ///
    /// ```notest
    /// Some("noopener noreferrer")
    /// ```
    pub fn link_rel(&mut self, value: Option<&'a str>) -> &mut Self {
        self.link_rel = value;
        self
    }

    /// Returns the settings for links' `rel` attribute, if one is set.
    ///
    /// # Examples
    ///
    ///     use ammonia::{Builder, UrlRelative};
    ///     let mut a = Builder::default();
    ///     a.link_rel(Some("a b"));
    ///     assert_eq!(a.get_link_rel(), Some("a b"));
    pub fn get_link_rel(&self) -> Option<&str> {
        self.link_rel
    }

    /// Sets the CSS classes that are allowed on specific tags.
    ///
    /// The values is structured as a map from tag names to a set of class names.
    ///
    /// If the `class` attribute is itself whitelisted for a tag, then adding entries to
    /// this map will cause a panic.
    ///
    /// # Examples
    ///
    ///     use ammonia::Builder;
    ///     use maplit::{hashmap, hashset};
    ///
    ///     # fn main() {
    ///     let allowed_classes = hashmap![
    ///         "code" => hashset!["rs", "ex", "c", "cxx", "js"]
    ///     ];
    ///     let a = Builder::new()
    ///         .allowed_classes(allowed_classes)
    ///         .clean("<code class=rs>fn main() {}</code>")
    ///         .to_string();
    ///     assert_eq!(
    ///       a,
    ///       "<code class=\"rs\">fn main() {}</code>");
    ///     # }
    ///
    /// # Defaults
    ///
    /// The set of allowed classes is empty by default.
    pub fn allowed_classes(&mut self, value: HashMap<&'a str, HashSet<&'a str>>) -> &mut Self {
        self.allowed_classes = value;
        self
    }

    /// Add additonal whitelisted classes without overwriting old ones.
    ///
    /// # Examples
    ///
    ///     let a = ammonia::Builder::default()
    ///         .add_allowed_classes("a", &["onebox"])
    ///         .clean("<a href=/ class=onebox>mess</span>").to_string();
    ///     assert_eq!("<a href=\"/\" class=\"onebox\" rel=\"noopener noreferrer\">mess</a>", a);
    pub fn add_allowed_classes<
        T: 'a + ?Sized + Borrow<str>,
        U: 'a + ?Sized + Borrow<str>,
        I: IntoIter<Item = &'a T>,
    >(
        &mut self,
        tag: &'a U,
        it: I,
    ) -> &mut Self {
        self.allowed_classes
            .entry(tag.borrow())
            .or_default()
            .extend(it.into_iter().map(Borrow::borrow));
        self
    }

    /// Remove already-whitelisted attributes.
    ///
    /// Does nothing if the attribute is already gone.
    ///
    /// # Examples
    ///
    ///     let a = ammonia::Builder::default()
    ///         .add_allowed_classes("span", &["active"])
    ///         .rm_allowed_classes("span", &["active"])
    ///         .clean("<span class=active>").to_string();
    ///     assert_eq!("<span class=\"\"></span>", a);
    pub fn rm_allowed_classes<
        'b,
        'c,
        T: 'b + ?Sized + Borrow<str>,
        U: 'c + ?Sized + Borrow<str>,
        I: IntoIter<Item = &'b T>,
    >(
        &mut self,
        tag: &'c U,
        it: I,
    ) -> &mut Self {
        if let Some(tag) = self.allowed_classes.get_mut(tag.borrow()) {
            for i in it {
                tag.remove(i.borrow());
            }
        }
        self
    }

    /// Returns a copy of the set of whitelisted class attributes.
    ///
    /// # Examples
    ///
    ///     use maplit::{hashmap, hashset};
    ///
    ///     let allowed_classes = hashmap![
    ///         "my-tag" => hashset!["my-class-1", "my-class-2"]
    ///     ];
    ///
    ///     let mut b = ammonia::Builder::default();
    ///     b.allowed_classes(Clone::clone(&allowed_classes));
    ///     assert_eq!(allowed_classes, b.clone_allowed_classes());
    pub fn clone_allowed_classes(&self) -> HashMap<&'a str, HashSet<&'a str>> {
        self.allowed_classes.clone()
    }

    /// Configures the handling of HTML comments.
    ///
    /// If this option is false, comments will be preserved.
    ///
    /// # Examples
    ///
    ///     use ammonia::Builder;
    ///
    ///     let a = Builder::new().strip_comments(false)
    ///         .clean("<!-- yes -->")
    ///         .to_string();
    ///     assert_eq!(
    ///       a,
    ///       "<!-- yes -->");
    ///
    /// # Defaults
    ///
    /// `true`
    pub fn strip_comments(&mut self, value: bool) -> &mut Self {
        self.strip_comments = value;
        self
    }

    /// Returns `true` if comment stripping is turned on.
    ///
    /// # Examples
    ///
    ///     let mut a = ammonia::Builder::new();
    ///     a.strip_comments(true);
    ///     assert!(a.will_strip_comments());
    ///     a.strip_comments(false);
    ///     assert!(!a.will_strip_comments());
    pub fn will_strip_comments(&self) -> bool {
        self.strip_comments
    }

    /// Prefixes all "id" attribute values with a given string.  Note that the tag and
    /// attribute themselves must still be whitelisted.
    ///
    /// # Examples
    ///
    ///     use ammonia::Builder;
    ///     use maplit::hashset;
    ///
    ///     # fn main() {
    ///     let attributes = hashset!["id"];
    ///     let a = Builder::new()
    ///         .generic_attributes(attributes)
    ///         .id_prefix(Some("safe-"))
    ///         .clean("<b id=42>")
    ///         .to_string();
    ///     assert_eq!(a, "<b id=\"safe-42\"></b>");
    ///     # }

    ///
    /// # Defaults
    ///
    /// `None`
    pub fn id_prefix(&mut self, value: Option<&'a str>) -> &mut Self {
        self.id_prefix = value;
        self
    }

    /// Only allows the specified properties in `style` attributes.
    ///
    /// Irrelevant if `style` is not an allowed attribute.
    ///
    /// Note that if style filtering is enabled style properties will be normalised e.g.
    /// invalid declarations and @rules will be removed, with only syntactically valid
    /// declarations kept.
    ///
    /// # Examples
    ///
    ///     use ammonia::Builder;
    ///     use maplit::hashset;
    ///
    ///     # fn main() {
    ///     let attributes = hashset!["style"];
    ///     let properties = hashset!["color"];
    ///     let a = Builder::new()
    ///         .generic_attributes(attributes)
    ///         .filter_style_properties(properties)
    ///         .clean("<p style=\"font-weight: heavy; color: red\">my html</p>")
    ///         .to_string();
    ///     assert_eq!(a, "<p style=\"color:red\">my html</p>");
    ///     # }
    pub fn filter_style_properties(&mut self, value: HashSet<&'a str>) -> &mut Self {
        self.style_properties = Some(value);
        self
    }

    /// Constructs a [`Builder`] instance configured with the [default options].
    ///
    /// # Examples
    ///
    ///     use ammonia::{Builder, Url, UrlRelative};
    ///     # use std::error::Error;
    ///
    ///     # fn do_main() -> Result<(), Box<dyn Error>> {
    ///     let input = "<!-- comments will be stripped -->This is an <a href=.>Ammonia</a> example using <a href=struct.Builder.html#method.new onclick=xss>the <code onmouseover=xss>new()</code> function</a>.";
    ///     let output = "This is an <a href=\"https://docs.rs/ammonia/1.0/ammonia/\" rel=\"noopener noreferrer\">Ammonia</a> example using <a href=\"https://docs.rs/ammonia/1.0/ammonia/struct.Builder.html#method.new\" rel=\"noopener noreferrer\">the <code>new()</code> function</a>.";
    ///
    ///     let result = Builder::new() // <--
    ///         .url_relative(UrlRelative::RewriteWithBase(Url::parse("https://docs.rs/ammonia/1.0/ammonia/")?))
    ///         .clean(input)
    ///         .to_string();
    ///     assert_eq!(result, output);
    ///     # Ok(())
    ///     # }
    ///     # fn main() { do_main().unwrap() }
    ///
    /// [default options]: fn.clean.html
    /// [`Builder`]: struct.Builder.html
    pub fn new() -> Self {
        Self::default()
    }

    /// Constructs a [`Builder`] instance configured with no allowed tags.
    ///
    /// # Examples
    ///
    ///     use ammonia::{Builder, Url, UrlRelative};
    ///     # use std::error::Error;
    ///
    ///     # fn do_main() -> Result<(), Box<dyn Error>> {
    ///     let input = "<!-- comments will be stripped -->This is an <a href=.>Ammonia</a> example using <a href=struct.Builder.html#method.new onclick=xss>the <code onmouseover=xss>empty()</code> function</a>.";
    ///     let output = "This is an Ammonia example using the empty() function.";
    ///
    ///     let result = Builder::empty() // <--
    ///         .url_relative(UrlRelative::RewriteWithBase(Url::parse("https://docs.rs/ammonia/1.0/ammonia/")?))
    ///         .clean(input)
    ///         .to_string();
    ///     assert_eq!(result, output);
    ///     # Ok(())
    ///     # }
    ///     # fn main() { do_main().unwrap() }
    ///
    /// [default options]: fn.clean.html
    /// [`Builder`]: struct.Builder.html
    pub fn empty() -> Self {
        Self {
            tags: hashset![],
            ..Self::default()
        }
    }

    /// Sanitizes an HTML fragment in a string according to the configured options.
    ///
    /// # Examples
    ///
    ///     use ammonia::{Builder, Url, UrlRelative};
    ///     # use std::error::Error;
    ///
    ///     # fn do_main() -> Result<(), Box<dyn Error>> {
    ///     let input = "<!-- comments will be stripped -->This is an <a href=.>Ammonia</a> example using <a href=struct.Builder.html#method.new onclick=xss>the <code onmouseover=xss>new()</code> function</a>.";
    ///     let output = "This is an <a href=\"https://docs.rs/ammonia/1.0/ammonia/\" rel=\"noopener noreferrer\">Ammonia</a> example using <a href=\"https://docs.rs/ammonia/1.0/ammonia/struct.Builder.html#method.new\" rel=\"noopener noreferrer\">the <code>new()</code> function</a>.";
    ///
    ///     let result = Builder::new()
    ///         .url_relative(UrlRelative::RewriteWithBase(Url::parse("https://docs.rs/ammonia/1.0/ammonia/")?))
    ///         .clean(input)
    ///         .to_string(); // <--
    ///     assert_eq!(result, output);
    ///     # Ok(())
    ///     # }
    ///     # fn main() { do_main().unwrap() }
    pub fn clean(&self, src: &str) -> Document {
        let parser = Self::make_parser();
        let dom = parser.one(src);
        self.clean_dom(dom)
    }

    /// Sanitizes an HTML fragment from a reader according to the configured options.
    ///
    /// The input should be in UTF-8 encoding, otherwise the decoding is lossy, just
    /// like when using [`String::from_utf8_lossy`].
    ///
    /// To avoid consuming the reader, a mutable reference can be passed to this method.
    ///
    /// # Examples
    ///
    ///     use ammonia::Builder;
    ///     # use std::error::Error;
    ///
    ///     # fn do_main() -> Result<(), Box<dyn Error>> {
    ///     let a = Builder::new()
    ///         .clean_from_reader(&b"<!-- no -->"[..])? // notice the `b`
    ///         .to_string();
    ///     assert_eq!(a, "");
    ///     # Ok(()) }
    ///     # fn main() { do_main().unwrap() }
    ///
    /// [`String::from_utf8_lossy`]: https://doc.rust-lang.org/std/string/struct.String.html#method.from_utf8_lossy
    pub fn clean_from_reader<R>(&self, mut src: R) -> io::Result<Document>
    where
        R: io::Read,
    {
        let parser = Self::make_parser().from_utf8();
        let dom = parser.read_from(&mut src)?;
        Ok(self.clean_dom(dom))
    }

    /// Clean a post-parsing DOM.
    ///
    /// This is not a public API because RcDom isn't really stable.
    /// We want to be able to take breaking changes to html5ever itself
    /// without having to break Ammonia's API.
    fn clean_dom(&self, dom: RcDom) -> Document {
        let mut stack = Vec::new();
        let mut removed = Vec::new();
        let link_rel = self
            .link_rel
            .map(|link_rel| format_tendril!("{}", link_rel));
        if link_rel.is_some() {
            assert!(self.generic_attributes.get("rel").is_none());
            assert!(self
                .tag_attributes
                .get("a")
                .and_then(|a| a.get("rel"))
                .is_none());
        }
        assert!(self.allowed_classes.is_empty() || !self.generic_attributes.contains("class"));
        for tag_name in self.allowed_classes.keys() {
            assert!(self
                .tag_attributes
                .get(tag_name)
                .and_then(|a| a.get("class"))
                .is_none());
        }
        for tag_name in &self.clean_content_tags {
            assert!(!self.tags.contains(tag_name), "`{tag_name}` appears in `clean_content_tags` and in `tags` at the same time");
            assert!(!self.tag_attributes.contains_key(tag_name), "`{tag_name}` appears in `clean_content_tags` and in `tag_attributes` at the same time");
        }
        let body = {
            let children = dom.document.children.borrow();
            children[0].clone()
        };
        stack.extend(
            mem::take(&mut *body.children.borrow_mut())
                .into_iter()
                .rev(),
        );
        // This design approach is used to prevent pathological content from producing
        // a stack overflow. The `stack` contains to-be-cleaned nodes, while `remove`,
        // of course, contains nodes that need to be dropped (we can't just drop them,
        // because they could have a very deep child tree).
        while let Some(mut node) = stack.pop() {
            let parent = node.parent
                .replace(None).expect("a node in the DOM will have a parent, except the root, which is not processed")
                .upgrade().expect("a node's parent will be pointed to by its parent (or the root pointer), and will not be dropped");
            if self.clean_node_content(&node) || !self.check_expected_namespace(&parent, &node) {
                removed.push(node);
                continue;
            }
            let pass = self.clean_child(&mut node);
            if pass {
                self.adjust_node_attributes(&mut node, &link_rel, self.id_prefix);
                dom.append(&parent.clone(), NodeOrText::AppendNode(node.clone()));
            } else {
                for sub in node.children.borrow_mut().iter_mut() {
                    sub.parent.replace(Some(Rc::downgrade(&parent)));
                }
            }
            stack.extend(
                mem::take(&mut *node.children.borrow_mut())
                    .into_iter()
                    .rev(),
            );
            if !pass {
                removed.push(node);
            }
        }
        // Now, imperatively clean up all of the child nodes.
        // Otherwise, we could wind up with a DoS, either caused by a memory leak,
        // or caused by a stack overflow.
        while let Some(node) = removed.pop() {
            removed.extend_from_slice(&mem::take(&mut *node.children.borrow_mut())[..]);
        }
        Document(dom)
    }

    /// Returns `true` if a node and all its content should be removed.
    fn clean_node_content(&self, node: &Handle) -> bool {
        match node.data {
            NodeData::Text { .. }
            | NodeData::Comment { .. }
            | NodeData::Doctype { .. }
            | NodeData::Document
            | NodeData::ProcessingInstruction { .. } => false,
            NodeData::Element { ref name, .. } => self.clean_content_tags.contains(&*name.local),
        }
    }

    /// Remove unwanted attributes, and check if the node should be kept or not.
    ///
    /// The root node doesn't need cleaning because we create the root node ourselves,
    /// and it doesn't get serialized, and ... it just exists to give the parser
    /// a context (in this case, a div-like block context).
    fn clean_child(&self, child: &mut Handle) -> bool {
        match child.data {
            NodeData::Text { .. } => true,
            NodeData::Comment { .. } => !self.strip_comments,
            NodeData::Doctype { .. }
            | NodeData::Document
            | NodeData::ProcessingInstruction { .. } => false,
            NodeData::Element {
                ref name,
                ref attrs,
                ..
            } => {
                if self.tags.contains(&*name.local) {
                    let attr_filter = |attr: &html5ever::Attribute| {
                        let whitelisted = self.generic_attributes.contains(&*attr.name.local)
                            || self.generic_attribute_prefixes.as_ref().map(|prefixes| {
                                prefixes.iter().any(|&p| attr.name.local.starts_with(p))
                            }) == Some(true)
                            || self
                                .tag_attributes
                                .get(&*name.local)
                                .map(|ta| ta.contains(&*attr.name.local))
                                == Some(true)
                            || self
                                .tag_attribute_values
                                .get(&*name.local)
                                .and_then(|tav| tav.get(&*attr.name.local))
                                .map(|vs| {
                                    let attr_val = attr.value.to_lowercase();
                                    vs.iter().any(|v| v.to_lowercase() == attr_val)
                                })
                                == Some(true);
                        if !whitelisted {
                            // If the class attribute is not whitelisted,
                            // but there is a whitelisted set of allowed_classes,
                            // do not strip out the class attribute.
                            // Banned classes will be filtered later.
                            &*attr.name.local == "class"
                                && self.allowed_classes.contains_key(&*name.local)
                        } else if is_url_attr(&name.local, &attr.name.local) {
                            let url = Url::parse(&attr.value);
                            if let Ok(url) = url {
                                self.url_schemes.contains(url.scheme())
                            } else if url == Err(url::ParseError::RelativeUrlWithoutBase) {
                                !matches!(self.url_relative, UrlRelative::Deny)
                            } else {
                                false
                            }
                        } else {
                            true
                        }
                    };
                    attrs.borrow_mut().retain(attr_filter);
                    true
                } else {
                    false
                }
            }
        }
    }

    // Check for unexpected namespace changes.
    //
    // The issue happens if developers added to the list of allowed tags any
    // tag which is parsed in RCDATA state, PLAINTEXT state or RAWTEXT state,
    // that is:
    //
    // * title
    // * textarea
    // * xmp
    // * iframe
    // * noembed
    // * noframes
    // * plaintext
    // * noscript
    // * style
    // * script
    //
    // An example in the wild is Plume, that allows iframe [1].  So in next
    // examples I'll assume the following policy:
    //
    //     Builder::new()
    //        .add_tags(&["iframe"])
    //
    // In HTML namespace `<iframe>` is parsed specially; that is, its content is
    // treated as text. For instance, the following html:
    //
    //     <iframe><a>test
    //
    // Is parsed into the following DOM tree:
    //
    //     iframe
    //     └─ #text: <a>test
    //
    // So iframe cannot have any children other than a text node.
    //
    // The same is not true, though, in "foreign content"; that is, within
    // <svg> or <math> tags. The following html:
    //
    //     <svg><iframe><a>test
    //
    // is parsed differently:
    //
    //    svg
    //    └─ iframe
    //       └─ a
    //          └─ #text: test
    //
    // So in SVG namespace iframe can have children.
    //
    // Ammonia disallows <svg> but it keeps its content after deleting it. And
    // the parser internally keeps track of the namespace of the element. So
    // assume we have the following snippet:
    //
    //     <svg><iframe><a title="</iframe><img src onerror=alert(1)>">test
    //
    // It is parsed into:
    //
    //     svg
    //     └─ iframe
    //        └─ a title="</iframe><img src onerror=alert(1)>"
    //           └─ #text: test
    //
    // This DOM tree is harmless from ammonia point of view because the piece
    // of code that looks like XSS is in a title attribute. Hence, the
    // resulting "safe" HTML from ammonia would be:
    //
    //     <iframe><a title="</iframe><img src onerror=alert(1)>" rel="noopener
    // noreferrer">test</a></iframe>
    //
    // However, at this point, the information about namespace is lost, which
    // means that the browser will parse this snippet into:
    //
    //     ├─ iframe
    //     │  └─ #text: <a title="
    //     ├─ img src="" onerror="alert(1)"
    //     └─ #text: " rel="noopener noreferrer">test
    //
    // Leading to XSS.
    //
    // To solve this issue, check for unexpected namespace switches after cleanup.
    // Elements which change namespace at an unexpected point are removed.
    // This function returns `true` if `child` should be kept, and `false` if it
    // should be removed.
    //
    // [1]: https://github.com/Plume-org/Plume/blob/main/plume-models/src/safe_string.rs#L21
    fn check_expected_namespace(&self, parent: &Handle, child: &Handle) -> bool {
        let (parent, child) = match (&parent.data, &child.data) {
            (NodeData::Element { name: pn, .. }, NodeData::Element { name: cn, .. }) => (pn, cn),
            _ => return true,
        };
        // The only way to switch from html to svg is with the <svg> tag
        if parent.ns == ns!(html) && child.ns == ns!(svg) {
            child.local == local_name!("svg")
        // The only way to switch from html to mathml is with the <math> tag
        } else if parent.ns == ns!(html) && child.ns == ns!(mathml) {
            child.local == local_name!("math")
        // The only way to switch from mathml to svg/html is with a text integration point
        } else if parent.ns == ns!(mathml) && child.ns != ns!(mathml) {
            // https://html.spec.whatwg.org/#mathml
            matches!(
                &*parent.local,
                "mi" | "mo" | "mn" | "ms" | "mtext" | "annotation-xml"
            ) && if child.ns == ns!(html) { is_html_tag(&child.local) } else { true }
        // The only way to switch from svg to mathml/html is with an html integration point
        } else if parent.ns == ns!(svg) && child.ns != ns!(svg) {
            // https://html.spec.whatwg.org/#svg-0
            matches!(&*parent.local, "foreignObject")
                && if child.ns == ns!(html) { is_html_tag(&child.local) } else { true }
        } else if child.ns == ns!(svg) {
            is_svg_tag(&child.local)
        } else if child.ns == ns!(mathml) {
            is_mathml_tag(&child.local)
        } else if child.ns == ns!(html) {
            is_html_tag(&child.local)
        } else {
            // There are no other supported ways to switch namespace
            parent.ns == child.ns
        }
    }

    /// Add and transform special-cased attributes and elements.
    ///
    /// This function handles:
    ///
    /// * relative URL rewriting
    /// * adding `<a rel>` attributes
    /// * filtering out banned style properties
    /// * filtering out banned classes
    fn adjust_node_attributes(
        &self,
        child: &mut Handle,
        link_rel: &Option<StrTendril>,
        id_prefix: Option<&'a str>,
    ) {
        if let NodeData::Element {
            ref name,
            ref attrs,
            ..
        } = child.data
        {
            if let Some(set_attrs) = self.set_tag_attribute_values.get(&*name.local) {
                let mut attrs = attrs.borrow_mut();
                for (&set_name, &set_value) in set_attrs {
                    // set the value of the attribute if the attribute is already present
                    if let Some(attr) = attrs.iter_mut().find(|attr| &*attr.name.local == set_name)
                    {
                        if &*attr.value != set_value {
                            attr.value = set_value.into();
                        }
                    } else {
                        // otherwise, add the attribute
                        let attr = Attribute {
                            name: QualName::new(None, ns!(), set_name.into()),
                            value: set_value.into(),
                        };
                        attrs.push(attr);
                    }
                }
            }
            if let Some(ref link_rel) = *link_rel {
                if &*name.local == "a" {
                    attrs.borrow_mut().push(Attribute {
                        name: QualName::new(None, ns!(), local_name!("rel")),
                        value: link_rel.clone(),
                    })
                }
            }
            if let Some(ref id_prefix) = id_prefix {
                for attr in &mut *attrs.borrow_mut() {
                    if &attr.name.local == "id" && !attr.value.starts_with(id_prefix) {
                        attr.value = format_tendril!("{}{}", id_prefix, attr.value);
                    }
                }
            }
            if let Some(ref attr_filter) = self.attribute_filter {
                let mut drop_attrs = Vec::new();
                let mut attrs = attrs.borrow_mut();
                for (i, attr) in &mut attrs.iter_mut().enumerate() {
                    let replace_with = if let Some(new) =
                        attr_filter.filter(&name.local, &attr.name.local, &attr.value)
                    {
                        if *new != *attr.value {
                            Some(format_tendril!("{}", new))
                        } else {
                            None // no need to replace the attr if filter returned the same value
                        }
                    } else {
                        drop_attrs.push(i);
                        None
                    };
                    if let Some(replace_with) = replace_with {
                        attr.value = replace_with;
                    }
                }
                for i in drop_attrs.into_iter().rev() {
                    attrs.swap_remove(i);
                }
            }
            {
                let mut drop_attrs = Vec::new();
                let mut attrs = attrs.borrow_mut();
                for (i, attr) in attrs.iter_mut().enumerate() {
                    if is_url_attr(&name.local, &attr.name.local) && is_url_relative(&attr.value) {
                        let new_value = self.url_relative.evaluate(&attr.value);
                        if let Some(new_value) = new_value {
                            attr.value = new_value;
                        } else {
                            drop_attrs.push(i);
                        }
                    }
                }
                // Swap remove scrambles the vector after the current point.
                // We will not do anything except with items before the current point.
                // The `rev()` is, as such, necessary for correctness.
                // We could use regular `remove(usize)` and a forward iterator,
                // but that's slower.
                for i in drop_attrs.into_iter().rev() {
                    attrs.swap_remove(i);
                }
            }
            if let Some(allowed_values) = &self.style_properties {
                for attr in &mut *attrs.borrow_mut() {
                    if &attr.name.local == "style" {
                        attr.value = style::filter_style_attribute(&attr.value, allowed_values).into();
                    }
                }
            }
            if let Some(allowed_values) = self.allowed_classes.get(&*name.local) {
                for attr in &mut *attrs.borrow_mut() {
                    if &attr.name.local == "class" {
                        let mut classes = vec![];
                        // https://html.spec.whatwg.org/#global-attributes:classes-2
                        for class in attr.value.split_ascii_whitespace() {
                            if allowed_values.contains(class) {
                                classes.push(class.to_owned());
                            }
                        }
                        attr.value = format_tendril!("{}", classes.join(" "));
                    }
                }
            }
        }
    }

    /// Initializes an HTML fragment parser.
    ///
    /// Ammonia conforms to the HTML5 fragment parsing rules,
    /// by parsing the given fragment as if it were included in a <div> tag.
    fn make_parser() -> html::Parser<RcDom> {
        html::parse_fragment(
            RcDom::default(),
            html::ParseOpts::default(),
            QualName::new(None, ns!(html), local_name!("div")),
            vec![],
            false,
        )
    }
}

/// Given an element name and attribute name, determine if the given attribute contains a URL.
fn is_url_attr(element: &str, attr: &str) -> bool {
    attr == "href"
        || attr == "src"
        || (element == "form" && attr == "action")
        || (element == "object" && attr == "data")
        || ((element == "button" || element == "input") && attr == "formaction")
        || (element == "a" && attr == "ping")
        || (element == "video" && attr == "poster")
}

fn is_html_tag(element: &str) -> bool {
    (!is_svg_tag(element) && !is_mathml_tag(element))
        || matches!(
            element,
            "title" | "style" | "font" | "a" | "script" | "span"
        )
}

/// Given an element name, check if it's SVG
fn is_svg_tag(element: &str) -> bool {
    // https://svgwg.org/svg2-draft/eltindex.html
    matches!(
        element,
        "a" | "animate"
            | "animateMotion"
            | "animateTransform"
            | "circle"
            | "clipPath"
            | "defs"
            | "desc"
            | "discard"
            | "ellipse"
            | "feBlend"
            | "feColorMatrix"
            | "feComponentTransfer"
            | "feComposite"
            | "feConvolveMatrix"
            | "feDiffuseLighting"
            | "feDisplacementMap"
            | "feDistantLight"
            | "feDropShadow"
            | "feFlood"
            | "feFuncA"
            | "feFuncB"
            | "feFuncG"
            | "feFuncR"
            | "feGaussianBlur"
            | "feImage"
            | "feMerge"
            | "feMergeNode"
            | "feMorphology"
            | "feOffset"
            | "fePointLight"
            | "feSpecularLighting"
            | "feSpotLight"
            | "feTile"
            | "feTurbulence"
            | "filter"
            | "foreignObject"
            | "g"
            | "image"
            | "line"
            | "linearGradient"
            | "marker"
            | "mask"
            | "metadata"
            | "mpath"
            | "path"
            | "pattern"
            | "polygon"
            | "polyline"
            | "radialGradient"
            | "rect"
            | "script"
            | "set"
            | "stop"
            | "style"
            | "svg"
            | "switch"
            | "symbol"
            | "text"
            | "textPath"
            | "title"
            | "tspan"
            | "use"
            | "view"
    )
}

/// Given an element name, check if it's Math
fn is_mathml_tag(element: &str) -> bool {
    // https://svgwg.org/svg2-draft/eltindex.html
    matches!(
        element,
        "abs"
            | "and"
            | "annotation"
            | "annotation-xml"
            | "apply"
            | "approx"
            | "arccos"
            | "arccosh"
            | "arccot"
            | "arccoth"
            | "arccsc"
            | "arccsch"
            | "arcsec"
            | "arcsech"
            | "arcsin"
            | "arcsinh"
            | "arctan"
            | "arctanh"
            | "arg"
            | "bind"
            | "bvar"
            | "card"
            | "cartesianproduct"
            | "cbytes"
            | "ceiling"
            | "cerror"
            | "ci"
            | "cn"
            | "codomain"
            | "complexes"
            | "compose"
            | "condition"
            | "conjugate"
            | "cos"
            | "cosh"
            | "cot"
            | "coth"
            | "cs"
            | "csc"
            | "csch"
            | "csymbol"
            | "curl"
            | "declare"
            | "degree"
            | "determinant"
            | "diff"
            | "divergence"
            | "divide"
            | "domain"
            | "domainofapplication"
            | "emptyset"
            | "eq"
            | "equivalent"
            | "eulergamma"
            | "exists"
            | "exp"
            | "exponentiale"
            | "factorial"
            | "factorof"
            | "false"
            | "floor"
            | "fn"
            | "forall"
            | "gcd"
            | "geq"
            | "grad"
            | "gt"
            | "ident"
            | "image"
            | "imaginary"
            | "imaginaryi"
            | "implies"
            | "in"
            | "infinity"
            | "int"
            | "integers"
            | "intersect"
            | "interval"
            | "inverse"
            | "lambda"
            | "laplacian"
            | "lcm"
            | "leq"
            | "limit"
            | "list"
            | "ln"
            | "log"
            | "logbase"
            | "lowlimit"
            | "lt"
            | "maction"
            | "maligngroup"
            | "malignmark"
            | "math"
            | "matrix"
            | "matrixrow"
            | "max"
            | "mean"
            | "median"
            | "menclose"
            | "merror"
            | "mfenced"
            | "mfrac"
            | "mglyph"
            | "mi"
            | "min"
            | "minus"
            | "mlabeledtr"
            | "mlongdiv"
            | "mmultiscripts"
            | "mn"
            | "mo"
            | "mode"
            | "moment"
            | "momentabout"
            | "mover"
            | "mpadded"
            | "mphantom"
            | "mprescripts"
            | "mroot"
            | "mrow"
            | "ms"
            | "mscarries"
            | "mscarry"
            | "msgroup"
            | "msline"
            | "mspace"
            | "msqrt"
            | "msrow"
            | "mstack"
            | "mstyle"
            | "msub"
            | "msubsup"
            | "msup"
            | "mtable"
            | "mtd"
            | "mtext"
            | "mtr"
            | "munder"
            | "munderover"
            | "naturalnumbers"
            | "neq"
            | "none"
            | "not"
            | "notanumber"
            | "notin"
            | "notprsubset"
            | "notsubset"
            | "or"
            | "otherwise"
            | "outerproduct"
            | "partialdiff"
            | "pi"
            | "piece"
            | "piecewise"
            | "plus"
            | "power"
            | "primes"
            | "product"
            | "prsubset"
            | "quotient"
            | "rationals"
            | "real"
            | "reals"
            | "reln"
            | "rem"
            | "root"
            | "scalarproduct"
            | "sdev"
            | "sec"
            | "sech"
            | "selector"
            | "semantics"
            | "sep"
            | "set"
            | "setdiff"
            | "share"
            | "sin"
            | "sinh"
            | "span"
            | "subset"
            | "sum"
            | "tan"
            | "tanh"
            | "tendsto"
            | "times"
            | "transpose"
            | "true"
            | "union"
            | "uplimit"
            | "variance"
            | "vector"
            | "vectorproduct"
            | "xor"
    )
}

fn is_url_relative(url: &str) -> bool {
    matches!(
        Url::parse(url),
        Err(url::ParseError::RelativeUrlWithoutBase)
    )
}

/// Policy for [relative URLs], that is, URLs that do not specify the scheme in full.
///
/// This policy kicks in, if set, for any attribute named `src` or `href`,
/// as well as the `data` attribute of an `object` tag.
///
/// [relative URLs]: struct.Builder.html#method.url_relative
///
/// # Examples
///
/// ## `Deny`
///
/// * `<a href="test">` is a file-relative URL, and will be removed
/// * `<a href="/test">` is a domain-relative URL, and will be removed
/// * `<a href="//example.com/test">` is a scheme-relative URL, and will be removed
/// * `<a href="http://example.com/test">` is an absolute URL, and will be kept
///
/// ## `PassThrough`
///
/// No changes will be made to any URLs, except if a disallowed scheme is used.
///
/// ## `RewriteWithBase`
///
/// If the base is set to `http://notriddle.com/some-directory/some-file`
///
/// * `<a href="test">` will be rewritten to `<a href="http://notriddle.com/some-directory/test">`
/// * `<a href="/test">` will be rewritten to `<a href="http://notriddle.com/test">`
/// * `<a href="//example.com/test">` will be rewritten to `<a href="http://example.com/test">`
/// * `<a href="http://example.com/test">` is an absolute URL, so it will be kept as-is
///
/// ## `Custom`
///
/// Pass the relative URL to a function.
/// If it returns `Some(string)`, then that one gets used.
/// Otherwise, it will remove the attribute (like `Deny` does).
///
///     use std::borrow::Cow;
///     fn is_absolute_path(url: &str) -> bool {
///         let u = url.as_bytes();
///         // `//a/b/c` is "protocol-relative", meaning "a" is a hostname
///         // `/a/b/c` is an absolute path, and what we want to do stuff to.
///         u.get(0) == Some(&b'/') && u.get(1) != Some(&b'/')
///     }
///     fn evaluate(url: &str) -> Option<Cow<str>> {
///         if is_absolute_path(url) {
///             Some(Cow::Owned(String::from("/root") + url))
///         } else {
///             Some(Cow::Borrowed(url))
///         }
///     }
///     fn main() {
///         let a = ammonia::Builder::new()
///             .url_relative(ammonia::UrlRelative::Custom(Box::new(evaluate)))
///             .clean("<a href=/test/path>fixed</a><a href=path>passed</a><a href=http://google.com/>skipped</a>")
///             .to_string();
///         assert_eq!(a, "<a href=\"/root/test/path\" rel=\"noopener noreferrer\">fixed</a><a href=\"path\" rel=\"noopener noreferrer\">passed</a><a href=\"http://google.com/\" rel=\"noopener noreferrer\">skipped</a>");
///     }
///
/// This function is only applied to relative URLs.
/// To filter all of the URLs,
/// use the not-yet-implemented Content Security Policy.
#[non_exhaustive]
pub enum UrlRelative<'a> {
    /// Relative URLs will be completely stripped from the document.
    Deny,
    /// Relative URLs will be passed through unchanged.
    PassThrough,
    /// Relative URLs will be changed into absolute URLs, based on this base URL.
    RewriteWithBase(Url),
    /// Force absolute and relative paths into a particular directory.
    ///
    /// Since the resolver does not affect fully-qualified URLs, it doesn't
    /// prevent users from linking wherever they want. This feature only
    /// serves to make content more portable.
    ///
    /// # Examples
    ///
    /// <table>
    /// <thead>
    /// <tr>
    ///     <th>root</th>
    ///     <th>path</th>
    ///     <th>url</th>
    ///     <th>result</th>
    /// </tr>
    /// </thead>
    /// <tbody>
    /// <tr>
    ///     <td>https://github.com/rust-ammonia/ammonia/blob/master/</td>
    ///     <td>README.md</td>
    ///     <td></td>
    ///     <td>https://github.com/rust-ammonia/ammonia/blob/master/README.md</td>
    /// </tr><tr>
    ///     <td>https://github.com/rust-ammonia/ammonia/blob/master/</td>
    ///     <td>README.md</td>
    ///     <td>/</td>
    ///     <td>https://github.com/rust-ammonia/ammonia/blob/master/</td>
    /// </tr><tr>
    ///     <td>https://github.com/rust-ammonia/ammonia/blob/master/</td>
    ///     <td>README.md</td>
    ///     <td>/CONTRIBUTING.md</td>
    ///     <td>https://github.com/rust-ammonia/ammonia/blob/master/CONTRIBUTING.md</td>
    /// </tr><tr>
    ///     <td>https://github.com/rust-ammonia/ammonia/blob/master</td>
    ///     <td>README.md</td>
    ///     <td></td>
    ///     <td>https://github.com/rust-ammonia/ammonia/blob/README.md</td>
    /// </tr><tr>
    ///     <td>https://github.com/rust-ammonia/ammonia/blob/master</td>
    ///     <td>README.md</td>
    ///     <td>/</td>
    ///     <td>https://github.com/rust-ammonia/ammonia/blob/</td>
    /// </tr><tr>
    ///     <td>https://github.com/rust-ammonia/ammonia/blob/master</td>
    ///     <td>README.md</td>
    ///     <td>/CONTRIBUTING.md</td>
    ///     <td>https://github.com/rust-ammonia/ammonia/blob/CONTRIBUTING.md</td>
    /// </tr><tr>
    ///     <td>https://github.com/rust-ammonia/ammonia/blob/master/</td>
    ///     <td></td>
    ///     <td></td>
    ///     <td>https://github.com/rust-ammonia/ammonia/blob/master/</td>
    /// </tr><tr>
    ///     <td>https://github.com/rust-ammonia/ammonia/blob/master/</td>
    ///     <td></td>
    ///     <td>/</td>
    ///     <td>https://github.com/rust-ammonia/ammonia/blob/master/</td>
    /// </tr><tr>
    ///     <td>https://github.com/rust-ammonia/ammonia/blob/master/</td>
    ///     <td></td>
    ///     <td>/CONTRIBUTING.md</td>
    ///     <td>https://github.com/rust-ammonia/ammonia/blob/master/CONTRIBUTING.md</td>
    /// </tr><tr>
    ///     <td>https://github.com/</td>
    ///     <td>rust-ammonia/ammonia/blob/master/README.md</td>
    ///     <td></td>
    ///     <td>https://github.com/rust-ammonia/ammonia/blob/master/README.md</td>
    /// </tr><tr>
    ///     <td>https://github.com/</td>
    ///     <td>rust-ammonia/ammonia/blob/master/README.md</td>
    ///     <td>/</td>
    ///     <td>https://github.com/</td>
    /// </tr><tr>
    ///     <td>https://github.com/</td>
    ///     <td>rust-ammonia/ammonia/blob/master/README.md</td>
    ///     <td>CONTRIBUTING.md</td>
    ///     <td>https://github.com/rust-ammonia/ammonia/blob/master/CONTRIBUTING.md</td>
    /// </tr><tr>
    ///     <td>https://github.com/</td>
    ///     <td>rust-ammonia/ammonia/blob/master/README.md</td>
    ///     <td>/CONTRIBUTING.md</td>
    ///     <td>https://github.com/CONTRIBUTING.md</td>
    /// </tr>
    /// </tbody>
    /// </table>
    RewriteWithRoot {
        /// The URL that is treated as the root by the resolver.
        root: Url,
        /// The "current path" used to resolve relative paths.
        path: String,
    },
    /// Rewrite URLs with a custom function.
    Custom(Box<dyn UrlRelativeEvaluate<'a>>),
}

impl<'a> UrlRelative<'a> {
    fn evaluate(&self, url: &str) -> Option<tendril::StrTendril> {
        match self {
            UrlRelative::RewriteWithBase(ref url_base) => url_base
                .join(url)
                .ok()
                .and_then(|x| StrTendril::from_str(x.as_str()).ok()),
            UrlRelative::RewriteWithRoot { ref root, ref path } => {
                (match url.as_bytes() {
                    // Scheme-relative URL
                    [b'/', b'/', ..] => root.join(url),
                    // Path-absolute URL
                    b"/" => root.join("."),
                    [b'/', ..] => root.join(&url[1..]),
                    // Path-relative URL
                    _ => root.join(path).and_then(|r| r.join(url)),
                })
                .ok()
                .and_then(|x| StrTendril::from_str(x.as_str()).ok())
            }
            UrlRelative::Custom(ref evaluate) => evaluate
                .evaluate(url)
                .as_ref()
                .map(Cow::as_ref)
                .map(StrTendril::from_str)
                .and_then(Result::ok),
            UrlRelative::PassThrough => StrTendril::from_str(url).ok(),
            UrlRelative::Deny => None,
        }
    }
}

impl<'a> fmt::Debug for UrlRelative<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            UrlRelative::Deny => write!(f, "UrlRelative::Deny"),
            UrlRelative::PassThrough => write!(f, "UrlRelative::PassThrough"),
            UrlRelative::RewriteWithBase(ref base) => {
                write!(f, "UrlRelative::RewriteWithBase({})", base)
            }
            UrlRelative::RewriteWithRoot { ref root, ref path } => {
                write!(
                    f,
                    "UrlRelative::RewriteWithRoot {{ root: {root}, path: {path} }}"
                )
            }
            UrlRelative::Custom(_) => write!(f, "UrlRelative::Custom"),
        }
    }
}

/// Types that implement this trait can be used to convert a relative URL into an absolute URL.
///
/// This evaluator is only called when the URL is relative; absolute URLs are not evaluated.
///
/// See [`url_relative`][url_relative] for more details.
///
/// [url_relative]: struct.Builder.html#method.url_relative
pub trait UrlRelativeEvaluate<'a>: Send + Sync + 'a {
    /// Return `None` to remove the attribute. Return `Some(str)` to replace it with a new string.
    fn evaluate<'url>(&self, _: &'url str) -> Option<Cow<'url, str>>;
}
impl<'a, T> UrlRelativeEvaluate<'a> for T
where
    T: Fn(&str) -> Option<Cow<'_, str>> + Send + Sync + 'a,
{
    fn evaluate<'url>(&self, url: &'url str) -> Option<Cow<'url, str>> {
        self(url)
    }
}

impl fmt::Debug for dyn AttributeFilter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("AttributeFilter")
    }
}

/// Types that implement this trait can be used to remove or rewrite arbitrary attributes.
///
/// See [`attribute_filter`][attribute_filter] for more details.
///
/// [attribute_filter]: struct.Builder.html#method.attribute_filter
pub trait AttributeFilter: Send + Sync {
    /// Return `None` to remove the attribute. Return `Some(str)` to replace it with a new string.
    fn filter<'a>(&self, _: &str, _: &str, _: &'a str) -> Option<Cow<'a, str>>;
}

impl<T> AttributeFilter for T
where
    T: for<'a> Fn(&str, &str, &'a str) -> Option<Cow<'a, str>> + Send + Sync + 'static,
{
    fn filter<'a>(&self, element: &str, attribute: &str, value: &'a str) -> Option<Cow<'a, str>> {
        self(element, attribute, value)
    }
}

/// A sanitized HTML document.
///
/// The `Document` type is an opaque struct representing an HTML fragment that was sanitized by
/// `ammonia`. It can be converted to a [`String`] or written to a [`Write`] instance. This allows
/// users to avoid buffering the serialized representation to a [`String`] when desired.
///
/// This type is opaque to insulate the caller from breaking changes in the `html5ever` interface.
///
/// Note that this type wraps an `html5ever` DOM tree. `ammonia` does not support streaming, so
/// the complete fragment needs to be stored in memory during processing.
///
/// [`String`]: https://doc.rust-lang.org/nightly/std/string/struct.String.html
/// [`Write`]: https://doc.rust-lang.org/nightly/std/io/trait.Write.html
///
/// # Examples
///
///     use ammonia::Builder;
///
///     let input = "<!-- comments will be stripped -->This is an Ammonia example.";
///     let output = "This is an Ammonia example.";
///
///     let document = Builder::new()
///         .clean(input);
///     assert_eq!(document.to_string(), output);
pub struct Document(RcDom);

impl Document {
    /// Serializes a `Document` instance to a writer.
    ///
    /// This method writes the sanitized HTML to a [`Write`] instance, avoiding a buffering step.
    ///
    /// To avoid consuming the writer, a mutable reference can be passed, like in the example below.
    ///
    /// Note that the in-memory representation of `Document` is larger than the serialized
    /// `String`.
    ///
    /// [`Write`]: https://doc.rust-lang.org/nightly/std/io/trait.Write.html
    ///
    /// # Examples
    ///
    ///     use ammonia::Builder;
    ///
    ///     let input = "Some <style></style>HTML here";
    ///     let expected = b"Some HTML here";
    ///
    ///     let document = Builder::new()
    ///         .clean(input);
    ///
    ///     let mut sanitized = Vec::new();
    ///     document.write_to(&mut sanitized)
    ///         .expect("Writing to a string should not fail (except on OOM)");
    ///     assert_eq!(sanitized, expected);
    pub fn write_to<W>(&self, writer: W) -> io::Result<()>
    where
        W: io::Write,
    {
        let opts = Self::serialize_opts();
        let inner: SerializableHandle = self.0.document.children.borrow()[0].clone().into();
        serialize(writer, &inner, opts)
    }

    /// Exposes the `Document` instance as an [`rcdom::Handle`].
    ///
    /// This method returns the inner object backing the `Document` instance. This allows
    /// making further changes to the DOM without introducing redundant serialization and
    /// parsing.
    ///
    /// Note that this method should be considered unstable and sits outside of the semver
    /// stability guarantees. It may change, break, or go away at any time, either because
    /// of `html5ever` changes or `ammonia` implementation changes.
    ///
    /// For this method to be accessible, a `cfg` flag is required. The easiest way is to
    /// use the `RUSTFLAGS` environment variable:
    ///
    /// ```text
    /// RUSTFLAGS='--cfg ammonia_unstable' cargo build
    /// ```
    ///
    /// on Unix-like platforms, or
    ///
    /// ```text
    /// set RUSTFLAGS=--cfg ammonia_unstable
    /// cargo build
    /// ```
    ///
    /// on Windows.
    ///
    /// This requirement also applies to crates that transitively depend on crates that use
    /// this flag.
    ///
    /// # Examples
    ///
    ///     use ammonia::Builder;
    ///     use maplit::hashset;
    ///     use html5ever::serialize::{serialize, SerializeOpts};
    ///
    ///     # use std::error::Error;
    ///     # fn do_main() -> Result<(), Box<dyn Error>> {
    ///     let input = "<a>one link</a> and <a>one more</a>";
    ///     let expected = "<a>one more</a> and <a>one link</a>";
    ///
    ///     let document = Builder::new()
    ///         .link_rel(None)
    ///         .clean(input);
    ///
    ///     let mut node = document.to_dom_node();
    ///     node.children.borrow_mut().reverse();
    ///
    ///     let mut buf = Vec::new();
    ///     serialize(&mut buf, &node, SerializeOpts::default())?;
    ///     let output = String::from_utf8(buf)?;
    ///
    ///     assert_eq!(output, expected);
    ///     # Ok(())
    ///     # }
    ///     # fn main() { do_main().unwrap() }
    #[cfg(ammonia_unstable)]
    pub fn to_dom_node(&self) -> Handle {
        self.0.document.children.borrow()[0].clone()
    }

    fn serialize_opts() -> SerializeOpts {
        SerializeOpts::default()
    }
}

impl Clone for Document {
    fn clone(&self) -> Self {
        let parser = Builder::make_parser();
        let dom = parser.one(&self.to_string()[..]);
        Document(dom)
    }
}

/// Convert a `Document` to stringified HTML.
///
/// Since [`Document`] implements [`Display`], it can be converted to a [`String`] using the
/// standard [`ToString::to_string`] method. This is the simplest way to use `ammonia`.
///
/// [`Document`]: ammonia::Document
/// [`Display`]: std::fmt::Display
/// [`ToString::to_string`]: std::string::ToString
///
/// # Examples
///
///     use ammonia::Builder;
///
///     let input = "Some <style></style>HTML here";
///     let output = "Some HTML here";
///
///     let document = Builder::new()
///         .clean(input);
///     assert_eq!(document.to_string(), output);
impl Display for Document {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let opts = Self::serialize_opts();
        let mut ret_val = Vec::new();
        let inner: SerializableHandle = self.0.document.children.borrow()[0].clone().into();
        serialize(&mut ret_val, &inner, opts)
            .expect("Writing to a string shouldn't fail (expect on OOM)");
        String::from_utf8(ret_val)
            .expect("html5ever only supports UTF8")
            .fmt(f)
    }
}

impl fmt::Debug for Document {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Document({})", self)
    }
}

impl From<Document> for String {
    fn from(document: Document) -> Self {
        document.to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn deeply_nested_whitelisted() {
        clean(&"<b>".repeat(60_000));
    }
    #[test]
    fn deeply_nested_blacklisted() {
        clean(&"<b-b>".repeat(60_000));
    }
    #[test]
    fn deeply_nested_alternating() {
        clean(&"<b-b>".repeat(35_000));
    }
    #[test]
    fn included_angles() {
        let fragment = "1 < 2";
        let result = clean(fragment);
        assert_eq!(result, "1 &lt; 2");
    }
    #[test]
    fn remove_script() {
        let fragment = "an <script>evil()</script> example";
        let result = clean(fragment);
        assert_eq!(result, "an  example");
    }
    #[test]
    fn ignore_link() {
        let fragment = "a <a href=\"http://www.google.com\">good</a> example";
        let expected = "a <a href=\"http://www.google.com\" rel=\"noopener noreferrer\">\
                        good</a> example";
        let result = clean(fragment);
        assert_eq!(result, expected);
    }
    #[test]
    fn remove_unsafe_link() {
        let fragment = "an <a onclick=\"evil()\" href=\"http://www.google.com\">evil</a> example";
        let result = clean(fragment);
        assert_eq!(
            result,
            "an <a href=\"http://www.google.com\" rel=\"noopener noreferrer\">evil</a> example"
        );
    }
    #[test]
    fn remove_js_link() {
        let fragment = "an <a href=\"javascript:evil()\">evil</a> example";
        let result = clean(fragment);
        assert_eq!(result, "an <a rel=\"noopener noreferrer\">evil</a> example");
    }
    #[test]
    fn tag_rebalance() {
        let fragment = "<b>AWESOME!";
        let result = clean(fragment);
        assert_eq!(result, "<b>AWESOME!</b>");
    }
    #[test]
    fn allow_url_relative() {
        let fragment = "<a href=test>Test</a>";
        let result = Builder::new()
            .url_relative(UrlRelative::PassThrough)
            .clean(fragment)
            .to_string();
        assert_eq!(
            result,
            "<a href=\"test\" rel=\"noopener noreferrer\">Test</a>"
        );
    }
    #[test]
    fn rewrite_url_relative() {
        let fragment = "<a href=test>Test</a>";
        let result = Builder::new()
            .url_relative(UrlRelative::RewriteWithBase(
                Url::parse("http://example.com/").unwrap(),
            ))
            .clean(fragment)
            .to_string();
        assert_eq!(
            result,
            "<a href=\"http://example.com/test\" rel=\"noopener noreferrer\">Test</a>"
        );
    }
    #[test]
    fn rewrite_url_relative_with_invalid_url() {
        // Reduced from https://github.com/Bauke/ammonia-crash-test
        let fragment = r##"<a href="\\"https://example.com\\"">test</a>"##;
        let result = Builder::new()
            .url_relative(UrlRelative::RewriteWithBase(
                Url::parse("http://example.com/").unwrap(),
            ))
            .clean(fragment)
            .to_string();
        assert_eq!(result, r##"<a rel="noopener noreferrer">test</a>"##);
    }
    #[test]
    fn attribute_filter_nop() {
        let fragment = "<a href=test>Test</a>";
        let result = Builder::new()
            .attribute_filter(|elem, attr, value| {
                assert_eq!("a", elem);
                assert!(
                    matches!(
                        (attr, value),
                        ("href", "test") | ("rel", "noopener noreferrer")
                    ),
                    "{}",
                    value.to_string()
                );
                Some(value.into())
            })
            .clean(fragment)
            .to_string();
        assert_eq!(
            result,
            "<a href=\"test\" rel=\"noopener noreferrer\">Test</a>"
        );
    }

    #[test]
    fn attribute_filter_drop() {
        let fragment = "Test<img alt=test src=imgtest>";
        let result = Builder::new()
            .attribute_filter(|elem, attr, value| {
                assert_eq!("img", elem);
                match (attr, value) {
                    ("src", "imgtest") => None,
                    ("alt", "test") => Some(value.into()),
                    _ => panic!("unexpected"),
                }
            })
            .clean(fragment)
            .to_string();
        assert_eq!(result, r#"Test<img alt="test">"#);
    }

    #[test]
    fn url_filter_absolute() {
        let fragment = "Test<img alt=test src=imgtest>";
        let result = Builder::new()
            .attribute_filter(|elem, attr, value| {
                assert_eq!("img", elem);
                match (attr, value) {
                    ("src", "imgtest") => {
                        Some(format!("https://example.com/images/{}", value).into())
                    }
                    ("alt", "test") => None,
                    _ => panic!("unexpected"),
                }
            })
            .url_relative(UrlRelative::RewriteWithBase(
                Url::parse("http://wrong.invalid/").unwrap(),
            ))
            .clean(fragment)
            .to_string();
        assert_eq!(
            result,
            r#"Test<img src="https://example.com/images/imgtest">"#
        );
    }

    #[test]
    fn url_filter_relative() {
        let fragment = "Test<img alt=test src=imgtest>";
        let result = Builder::new()
            .attribute_filter(|elem, attr, value| {
                assert_eq!("img", elem);
                match (attr, value) {
                    ("src", "imgtest") => Some("rewrite".into()),
                    ("alt", "test") => Some("altalt".into()),
                    _ => panic!("unexpected"),
                }
            })
            .url_relative(UrlRelative::RewriteWithBase(
                Url::parse("https://example.com/base/#").unwrap(),
            ))
            .clean(fragment)
            .to_string();
        assert_eq!(
            result,
            r#"Test<img alt="altalt" src="https://example.com/base/rewrite">"#
        );
    }

    #[test]
    fn rewrite_url_relative_no_rel() {
        let fragment = "<a href=test>Test</a>";
        let result = Builder::new()
            .url_relative(UrlRelative::RewriteWithBase(
                Url::parse("http://example.com/").unwrap(),
            ))
            .link_rel(None)
            .clean(fragment)
            .to_string();
        assert_eq!(result, "<a href=\"http://example.com/test\">Test</a>");
    }
    #[test]
    fn deny_url_relative() {
        let fragment = "<a href=test>Test</a>";
        let result = Builder::new()
            .url_relative(UrlRelative::Deny)
            .clean(fragment)
            .to_string();
        assert_eq!(result, "<a rel=\"noopener noreferrer\">Test</a>");
    }
    #[test]
    fn replace_rel() {
        let fragment = "<a href=test rel=\"garbage\">Test</a>";
        let result = Builder::new()
            .url_relative(UrlRelative::PassThrough)
            .clean(fragment)
            .to_string();
        assert_eq!(
            result,
            "<a href=\"test\" rel=\"noopener noreferrer\">Test</a>"
        );
    }
    #[test]
    fn consider_rel_still_banned() {
        let fragment = "<a href=test rel=\"garbage\">Test</a>";
        let result = Builder::new()
            .url_relative(UrlRelative::PassThrough)
            .link_rel(None)
            .clean(fragment)
            .to_string();
        assert_eq!(result, "<a href=\"test\">Test</a>");
    }
    #[test]
    fn object_data() {
        let fragment = "<span data=\"javascript:evil()\">Test</span>\
                        <object data=\"javascript:evil()\"></object>M";
        let expected = r#"<span data="javascript:evil()">Test</span><object></object>M"#;
        let result = Builder::new()
            .tags(hashset!["span", "object"])
            .generic_attributes(hashset!["data"])
            .clean(fragment)
            .to_string();
        assert_eq!(result, expected);
    }
    #[test]
    fn remove_attributes() {
        let fragment = "<table border=\"1\"><tr></tr></table>";
        let result = Builder::new().clean(fragment);
        assert_eq!(
            result.to_string(),
            "<table><tbody><tr></tr></tbody></table>"
        );
    }
    #[test]
    fn quotes_in_attrs() {
        let fragment = "<b title='\"'>contents</b>";
        let result = clean(fragment);
        assert_eq!(result, "<b title=\"&quot;\">contents</b>");
    }
    #[test]
    #[should_panic]
    fn panic_if_rel_is_allowed_and_replaced_generic() {
        Builder::new()
            .link_rel(Some("noopener noreferrer"))
            .generic_attributes(hashset!["rel"])
            .clean("something");
    }
    #[test]
    #[should_panic]
    fn panic_if_rel_is_allowed_and_replaced_a() {
        Builder::new()
            .link_rel(Some("noopener noreferrer"))
            .tag_attributes(hashmap![
                "a" => hashset!["rel"],
            ])
            .clean("something");
    }
    #[test]
    fn no_panic_if_rel_is_allowed_and_replaced_span() {
        Builder::new()
            .link_rel(Some("noopener noreferrer"))
            .tag_attributes(hashmap![
                "span" => hashset!["rel"],
            ])
            .clean("<span rel=\"what\">s</span>");
    }
    #[test]
    fn no_panic_if_rel_is_allowed_and_not_replaced_generic() {
        Builder::new()
            .link_rel(None)
            .generic_attributes(hashset!["rel"])
            .clean("<a rel=\"what\">s</a>");
    }
    #[test]
    fn no_panic_if_rel_is_allowed_and_not_replaced_a() {
        Builder::new()
            .link_rel(None)
            .tag_attributes(hashmap![
                "a" => hashset!["rel"],
            ])
            .clean("<a rel=\"what\">s</a>");
    }
    #[test]
    fn dont_close_void_elements() {
        let fragment = "<br>";
        let result = clean(fragment);
        assert_eq!(result.to_string(), "<br>");
    }
    #[should_panic]
    #[test]
    fn panic_on_allowed_classes_tag_attributes() {
        let fragment = "<p class=\"foo bar\"><a class=\"baz bleh\">Hey</a></p>";
        Builder::new()
            .link_rel(None)
            .tag_attributes(hashmap![
                "p" => hashset!["class"],
                "a" => hashset!["class"],
            ])
            .allowed_classes(hashmap![
                "p" => hashset!["foo", "bar"],
                "a" => hashset!["baz"],
            ])
            .clean(fragment);
    }
    #[should_panic]
    #[test]
    fn panic_on_allowed_classes_generic_attributes() {
        let fragment = "<p class=\"foo bar\"><a class=\"baz bleh\">Hey</a></p>";
        Builder::new()
            .link_rel(None)
            .generic_attributes(hashset!["class", "href", "some-foo"])
            .allowed_classes(hashmap![
                "p" => hashset!["foo", "bar"],
                "a" => hashset!["baz"],
            ])
            .clean(fragment);
    }
    #[test]
    fn remove_non_allowed_classes() {
        let fragment = "<p class=\"foo bar\"><a class=\"baz bleh\">Hey</a></p>";
        let result = Builder::new()
            .link_rel(None)
            .allowed_classes(hashmap![
                "p" => hashset!["foo", "bar"],
                "a" => hashset!["baz"],
            ])
            .clean(fragment);
        assert_eq!(
            result.to_string(),
            "<p class=\"foo bar\"><a class=\"baz\">Hey</a></p>"
        );
    }
    #[test]
    fn remove_non_allowed_classes_with_tag_class() {
        let fragment = "<p class=\"foo bar\"><a class=\"baz bleh\">Hey</a></p>";
        let result = Builder::new()
            .link_rel(None)
            .tag_attributes(hashmap![
                "div" => hashset!["class"],
            ])
            .allowed_classes(hashmap![
                "p" => hashset!["foo", "bar"],
                "a" => hashset!["baz"],
            ])
            .clean(fragment);
        assert_eq!(
            result.to_string(),
            "<p class=\"foo bar\"><a class=\"baz\">Hey</a></p>"
        );
    }
    #[test]
    fn allowed_classes_ascii_whitespace() {
        // According to https://infra.spec.whatwg.org/#ascii-whitespace,
        // TAB (\t), LF (\n), FF (\x0C), CR (\x0D) and SPACE (\x20) are
        // considered to be ASCII whitespace. Unicode whitespace characters
        // and VT (\x0B) aren't ASCII whitespace.
        let fragment = "<p class=\"a\tb\nc\x0Cd\re f\x0B g\u{2000}\">";
        let result = Builder::new()
            .allowed_classes(hashmap![
                "p" => hashset!["a", "b", "c", "d", "e", "f", "g"],
            ])
            .clean(fragment);
        assert_eq!(result.to_string(), r#"<p class="a b c d e"></p>"#);
    }
    #[test]
    fn remove_non_allowed_attributes_with_tag_attribute_values() {
        let fragment = "<p data-label=\"baz\" name=\"foo\"></p>";
        let result = Builder::new()
            .tag_attribute_values(hashmap![
                "p" => hashmap![
                    "data-label" => hashset!["bar"],
                ],
            ])
            .tag_attributes(hashmap![
                "p" => hashset!["name"],
            ])
            .clean(fragment);
        assert_eq!(result.to_string(), "<p name=\"foo\"></p>",);
    }
    #[test]
    fn keep_allowed_attributes_with_tag_attribute_values() {
        let fragment = "<p data-label=\"bar\" name=\"foo\"></p>";
        let result = Builder::new()
            .tag_attribute_values(hashmap![
                "p" => hashmap![
                    "data-label" => hashset!["bar"],
                ],
            ])
            .tag_attributes(hashmap![
                "p" => hashset!["name"],
            ])
            .clean(fragment);
        assert_eq!(
            result.to_string(),
            "<p data-label=\"bar\" name=\"foo\"></p>",
        );
    }
    #[test]
    fn tag_attribute_values_case_insensitive() {
        let fragment = "<input type=\"CHECKBOX\" name=\"foo\">";
        let result = Builder::new()
            .tags(hashset!["input"])
            .tag_attribute_values(hashmap![
                "input" => hashmap![
                    "type" => hashset!["checkbox"],
                ],
            ])
            .tag_attributes(hashmap![
                "input" => hashset!["name"],
            ])
            .clean(fragment);
        assert_eq!(result.to_string(), "<input type=\"CHECKBOX\" name=\"foo\">",);
    }
    #[test]
    fn set_tag_attribute_values() {
        let fragment = "<a href=\"https://example.com/\">Link</a>";
        let result = Builder::new()
            .link_rel(None)
            .add_tag_attributes("a", &["target"])
            .set_tag_attribute_value("a", "target", "_blank")
            .clean(fragment);
        assert_eq!(
            result.to_string(),
            "<a href=\"https://example.com/\" target=\"_blank\">Link</a>",
        );
    }
    #[test]
    fn update_existing_set_tag_attribute_values() {
        let fragment = "<a target=\"bad\" href=\"https://example.com/\">Link</a>";
        let result = Builder::new()
            .link_rel(None)
            .add_tag_attributes("a", &["target"])
            .set_tag_attribute_value("a", "target", "_blank")
            .clean(fragment);
        assert_eq!(
            result.to_string(),
            "<a target=\"_blank\" href=\"https://example.com/\">Link</a>",
        );
    }
    #[test]
    fn unwhitelisted_set_tag_attribute_values() {
        let fragment = "<span>hi</span><my-elem>";
        let result = Builder::new()
            .set_tag_attribute_value("my-elem", "my-attr", "val")
            .clean(fragment);
        assert_eq!(result.to_string(), "<span>hi</span>",);
    }
    #[test]
    fn remove_entity_link() {
        let fragment = "<a href=\"&#x6A&#x61&#x76&#x61&#x73&#x63&#x72&#x69&#x70&#x74&#x3A&#x61\
                        &#x6C&#x65&#x72&#x74&#x28&#x27&#x58&#x53&#x53&#x27&#x29\">Click me!</a>";
        let result = clean(fragment);
        assert_eq!(
            result.to_string(),
            "<a rel=\"noopener noreferrer\">Click me!</a>"
        );
    }
    #[test]
    fn remove_relative_url_evaluate() {
        fn is_absolute_path(url: &str) -> bool {
            let u = url.as_bytes();
            // `//a/b/c` is "protocol-relative", meaning "a" is a hostname
            // `/a/b/c` is an absolute path, and what we want to do stuff to.
            u.first() == Some(&b'/') && u.get(1) != Some(&b'/')
        }
        fn is_banned(url: &str) -> bool {
            let u = url.as_bytes();
            u.first() == Some(&b'b') && u.get(1) == Some(&b'a')
        }
        fn evaluate(url: &str) -> Option<Cow<'_, str>> {
            if is_absolute_path(url) {
                Some(Cow::Owned(String::from("/root") + url))
            } else if is_banned(url) {
                None
            } else {
                Some(Cow::Borrowed(url))
            }
        }
        let a = Builder::new()
            .url_relative(UrlRelative::Custom(Box::new(evaluate)))
            .clean("<a href=banned>banned</a><a href=/test/path>fixed</a><a href=path>passed</a><a href=http://google.com/>skipped</a>")
            .to_string();
        assert_eq!(a, "<a rel=\"noopener noreferrer\">banned</a><a href=\"/root/test/path\" rel=\"noopener noreferrer\">fixed</a><a href=\"path\" rel=\"noopener noreferrer\">passed</a><a href=\"http://google.com/\" rel=\"noopener noreferrer\">skipped</a>");
    }
    #[test]
    fn remove_relative_url_evaluate_b() {
        fn is_absolute_path(url: &str) -> bool {
            let u = url.as_bytes();
            // `//a/b/c` is "protocol-relative", meaning "a" is a hostname
            // `/a/b/c` is an absolute path, and what we want to do stuff to.
            u.first() == Some(&b'/') && u.get(1) != Some(&b'/')
        }
        fn is_banned(url: &str) -> bool {
            let u = url.as_bytes();
            u.first() == Some(&b'b') && u.get(1) == Some(&b'a')
        }
        fn evaluate(url: &str) -> Option<Cow<'_, str>> {
            if is_absolute_path(url) {
                Some(Cow::Owned(String::from("/root") + url))
            } else if is_banned(url) {
                None
            } else {
                Some(Cow::Borrowed(url))
            }
        }
        let a = Builder::new()
            .url_relative(UrlRelative::Custom(Box::new(evaluate)))
            .clean("<a href=banned>banned</a><a href=banned title=test>banned</a><a title=test href=banned>banned</a>")
            .to_string();
        assert_eq!(a, "<a rel=\"noopener noreferrer\">banned</a><a rel=\"noopener noreferrer\" title=\"test\">banned</a><a title=\"test\" rel=\"noopener noreferrer\">banned</a>");
    }
    #[test]
    fn remove_relative_url_evaluate_c() {
        // Don't run on absolute URLs.
        fn evaluate(_: &str) -> Option<Cow<'_, str>> {
            return Some(Cow::Owned(String::from("invalid")));
        }
        let a = Builder::new()
            .url_relative(UrlRelative::Custom(Box::new(evaluate)))
            .clean("<a href=\"https://www.google.com/\">google</a>")
            .to_string();
        assert_eq!(
            a,
            "<a href=\"https://www.google.com/\" rel=\"noopener noreferrer\">google</a>"
        );
    }
    #[test]
    fn clean_children_of_bad_element() {
        let fragment = "<bad><evil>a</evil>b</bad>";
        let result = Builder::new().clean(fragment);
        assert_eq!(result.to_string(), "ab");
    }
    #[test]
    fn reader_input() {
        let fragment = b"an <script>evil()</script> example";
        let result = Builder::new().clean_from_reader(&fragment[..]);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().to_string(), "an  example");
    }
    #[test]
    fn reader_non_utf8() {
        let fragment = b"non-utf8 \xF0\x90\x80string";
        let result = Builder::new().clean_from_reader(&fragment[..]);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().to_string(), "non-utf8 \u{fffd}string");
    }
    #[test]
    fn display_impl() {
        let fragment = r#"a <a>link</a>"#;
        let result = Builder::new().link_rel(None).clean(fragment);
        assert_eq!(format!("{}", result), "a <a>link</a>");
    }
    #[test]
    fn debug_impl() {
        let fragment = r#"a <a>link</a>"#;
        let result = Builder::new().link_rel(None).clean(fragment);
        assert_eq!(format!("{:?}", result), "Document(a <a>link</a>)");
    }
    #[cfg(ammonia_unstable)]
    #[test]
    fn to_dom_node() {
        let fragment = r#"a <a>link</a>"#;
        let result = Builder::new().link_rel(None).clean(fragment);
        let _node = result.to_dom_node();
    }
    #[test]
    fn string_from_document() {
        let fragment = r#"a <a>link"#;
        let result = String::from(Builder::new().link_rel(None).clean(fragment));
        assert_eq!(format!("{}", result), "a <a>link</a>");
    }
    fn require_sync<T: Sync>(_: T) {}
    fn require_send<T: Send>(_: T) {}
    #[test]
    fn require_sync_and_send() {
        require_sync(Builder::new());
        require_send(Builder::new());
    }
    #[test]
    fn id_prefixed() {
        let fragment = "<a id=\"hello\"></a><b id=\"hello\"></a>";
        let result = String::from(
            Builder::new()
                .tag_attributes(hashmap![
                    "a" => hashset!["id"],
                ])
                .id_prefix(Some("prefix-"))
                .clean(fragment),
        );
        assert_eq!(
            result.to_string(),
            "<a id=\"prefix-hello\" rel=\"noopener noreferrer\"></a><b></b>"
        );
    }
    #[test]
    fn id_already_prefixed() {
        let fragment = "<a id=\"prefix-hello\"></a>";
        let result = String::from(
            Builder::new()
                .tag_attributes(hashmap![
                    "a" => hashset!["id"],
                ])
                .id_prefix(Some("prefix-"))
                .clean(fragment),
        );
        assert_eq!(
            result.to_string(),
            "<a id=\"prefix-hello\" rel=\"noopener noreferrer\"></a>"
        );
    }
    #[test]
    fn clean_content_tags() {
        let fragment = "<script type=\"text/javascript\"><a>Hello!</a></script>";
        let result = String::from(
            Builder::new()
                .clean_content_tags(hashset!["script"])
                .clean(fragment),
        );
        assert_eq!(result.to_string(), "");
    }
    #[test]
    fn only_clean_content_tags() {
        let fragment = "<em>This is</em><script><a>Hello!</a></script><p>still here!</p>";
        let result = String::from(
            Builder::new()
                .clean_content_tags(hashset!["script"])
                .clean(fragment),
        );
        assert_eq!(result.to_string(), "<em>This is</em><p>still here!</p>");
    }
    #[test]
    fn clean_removed_default_tag() {
        let fragment = "<em>This is</em><script><a>Hello!</a></script><p>still here!</p>";
        let result = String::from(
            Builder::new()
                .rm_tags(hashset!["a"])
                .rm_tag_attributes("a", hashset!["href", "hreflang"])
                .clean_content_tags(hashset!["script"])
                .clean(fragment),
        );
        assert_eq!(result.to_string(), "<em>This is</em><p>still here!</p>");
    }
    #[test]
    #[should_panic]
    fn panic_on_clean_content_tag_attribute() {
        Builder::new()
            .rm_tags(std::iter::once("a"))
            .clean_content_tags(hashset!["a"])
            .clean("");
    }
    #[test]
    #[should_panic]
    fn panic_on_clean_content_tag() {
        Builder::new().clean_content_tags(hashset!["a"]).clean("");
    }

    #[test]
    fn clean_text_test() {
        assert_eq!(
            clean_text("<this> is <a test function"),
            "&lt;this&gt;&#32;is&#32;&lt;a&#32;test&#32;function"
        );
    }

    #[test]
    fn clean_text_spaces_test() {
        assert_eq!(clean_text("\x09\x0a\x0c\x20"), "&#9;&#10;&#12;&#32;");
    }

    #[test]
    fn ns_svg() {
        // https://github.com/cure53/DOMPurify/pull/495
        let fragment = r##"<svg><iframe><a title="</iframe><img src onerror=alert(1)>">test"##;
        let result = String::from(Builder::new().add_tags(&["iframe"]).clean(fragment));
        assert_eq!(result.to_string(), "");

        let fragment = "<svg><iframe>remove me</iframe></svg><iframe>keep me</iframe>";
        let result = String::from(Builder::new().add_tags(&["iframe"]).clean(fragment));
        assert_eq!(result.to_string(), "<iframe>keep me</iframe>");

        let fragment = "<svg><a>remove me</a></svg><iframe>keep me</iframe>";
        let result = String::from(Builder::new().add_tags(&["iframe"]).clean(fragment));
        assert_eq!(result.to_string(), "<iframe>keep me</iframe>");

        let fragment = "<svg><a>keep me</a></svg><iframe>keep me</iframe>";
        let result = String::from(Builder::new().add_tags(&["iframe", "svg"]).clean(fragment));
        assert_eq!(
            result.to_string(),
            "<svg><a rel=\"noopener noreferrer\">keep me</a></svg><iframe>keep me</iframe>"
        );
    }

    #[test]
    fn ns_svg_2() {
        let fragment = "<svg><foreignObject><table><path><xmp><!--</xmp><img title'--&gt;&lt;img src=1 onerror=alert(1)&gt;'>";
        let result =  Builder::default()
            .strip_comments(false)
            .add_tags(&["svg","foreignObject","table","path","xmp"])
            .clean(fragment);
        assert_eq!(
            result.to_string(),
            "<svg><foreignObject><table></table></foreignObject></svg>"
        );
    }

    #[test]
    fn ns_mathml() {
        // https://github.com/cure53/DOMPurify/pull/495
        let fragment = "<mglyph></mglyph>";
        let result = String::from(
            Builder::new()
                .add_tags(&["math", "mtext", "mglyph"])
                .clean(fragment),
        );
        assert_eq!(result.to_string(), "");
        let fragment = "<math><mtext><div><mglyph>";
        let result = String::from(
            Builder::new()
                .add_tags(&["math", "mtext", "mglyph"])
                .clean(fragment),
        );
        assert_eq!(
            result.to_string(),
            "<math><mtext><div></div></mtext></math>"
        );
        let fragment = "<math><mtext><mglyph>";
        let result = String::from(
            Builder::new()
                .add_tags(&["math", "mtext", "mglyph"])
                .clean(fragment),
        );
        assert_eq!(
            result.to_string(),
            "<math><mtext><mglyph></mglyph></mtext></math>"
        );
    }

    #[test]
    fn ns_mathml_2() {
        let fragment = "<math><mtext><table><mglyph><xmp><!--</xmp><img title='--&gt;&lt;img src=1 onerror=alert(1)&gt;'>";
        let result =  Builder::default()
            .strip_comments(false)
            .add_tags(&["math","mtext","table","mglyph","xmp"])
            .clean(fragment);
        assert_eq!(
            result.to_string(),
            "<math><mtext><table></table></mtext></math>"
        );
    }


    #[test]
    fn xml_processing_instruction() {
        // https://blog.slonser.info/posts/dompurify-node-type-confusion/
        let fragment = r##"<svg><?xml-stylesheet src='slonser' ?></svg>"##;
        let result = String::from(Builder::new().clean(fragment));
        assert_eq!(result.to_string(), "");

        let fragment = r##"<svg><?xml-stylesheet src='slonser' ?></svg>"##;
        let result = String::from(Builder::new().add_tags(&["svg"]).clean(fragment));
        assert_eq!(result.to_string(), "<svg></svg>");

        let fragment = r##"<svg><?xml-stylesheet ><img src=x onerror="alert('Ammonia bypassed!!!')"> ?></svg>"##;
        let result = String::from(Builder::new().add_tags(&["svg"]).clean(fragment));
        assert_eq!(result.to_string(), "<svg></svg><img src=\"x\"> ?&gt;");
    }

    #[test]
    fn generic_attribute_prefixes() {
        let prefix_data = ["data-"];
        let prefix_code = ["code-"];
        let mut b = Builder::new();
        let mut hs: HashSet<&'_ str> = HashSet::new();
        hs.insert("data-");
        assert!(b.generic_attribute_prefixes.is_none());
        b.generic_attribute_prefixes(hs);
        assert!(b.generic_attribute_prefixes.is_some());
        assert_eq!(b.generic_attribute_prefixes.as_ref().unwrap().len(), 1);
        b.add_generic_attribute_prefixes(&prefix_data);
        assert_eq!(b.generic_attribute_prefixes.as_ref().unwrap().len(), 1);
        b.add_generic_attribute_prefixes(&prefix_code);
        assert_eq!(b.generic_attribute_prefixes.as_ref().unwrap().len(), 2);
        b.rm_generic_attribute_prefixes(&prefix_code);
        assert_eq!(b.generic_attribute_prefixes.as_ref().unwrap().len(), 1);
        b.rm_generic_attribute_prefixes(&prefix_code);
        assert_eq!(b.generic_attribute_prefixes.as_ref().unwrap().len(), 1);
        b.rm_generic_attribute_prefixes(&prefix_data);
        assert!(b.generic_attribute_prefixes.is_none());
    }

    #[test]
    fn generic_attribute_prefixes_clean() {
        let fragment = r#"<a data-1 data-2 code-1 code-2><a>Hello!</a></a>"#;
        let result_cleaned = String::from(
            Builder::new()
                .add_tag_attributes("a", &["data-1"])
                .clean(fragment),
        );
        assert_eq!(
            result_cleaned,
            r#"<a data-1="" rel="noopener noreferrer"></a><a rel="noopener noreferrer">Hello!</a>"#
        );
        let result_allowed = String::from(
            Builder::new()
                .add_tag_attributes("a", &["data-1"])
                .add_generic_attribute_prefixes(&["data-"])
                .clean(fragment),
        );
        assert_eq!(
            result_allowed,
            r#"<a data-1="" data-2="" rel="noopener noreferrer"></a><a rel="noopener noreferrer">Hello!</a>"#
        );
        let result_allowed = String::from(
            Builder::new()
                .add_tag_attributes("a", &["data-1", "code-1"])
                .add_generic_attribute_prefixes(&["data-", "code-"])
                .clean(fragment),
        );
        assert_eq!(
            result_allowed,
            r#"<a data-1="" data-2="" code-1="" code-2="" rel="noopener noreferrer"></a><a rel="noopener noreferrer">Hello!</a>"#
        );
    }
    #[test]
    fn lesser_than_isnt_html() {
        let fragment = "1 < 2";
        assert!(!is_html(fragment));
    }
    #[test]
    fn dense_lesser_than_isnt_html() {
        let fragment = "1<2";
        assert!(!is_html(fragment));
    }
    #[test]
    fn what_about_number_elements() {
        let fragment = "foo<2>bar";
        assert!(!is_html(fragment));
    }
    #[test]
    fn turbofish_is_html_sadly() {
        let fragment = "Vec::<u8>::new()";
        assert!(is_html(fragment));
    }
    #[test]
    fn stop_grinning() {
        let fragment = "did you really believe me? <g>";
        assert!(is_html(fragment));
    }
    #[test]
    fn dont_be_bold() {
        let fragment = "<b>";
        assert!(is_html(fragment));
    }

    #[test]
    fn rewrite_with_root() {
        let tests = [
            (
                "https://github.com/rust-ammonia/ammonia/blob/master/",
                "README.md",
                "",
                "https://github.com/rust-ammonia/ammonia/blob/master/README.md",
            ),
            (
                "https://github.com/rust-ammonia/ammonia/blob/master/",
                "README.md",
                "/",
                "https://github.com/rust-ammonia/ammonia/blob/master/",
            ),
            (
                "https://github.com/rust-ammonia/ammonia/blob/master/",
                "README.md",
                "/CONTRIBUTING.md",
                "https://github.com/rust-ammonia/ammonia/blob/master/CONTRIBUTING.md",
            ),
            (
                "https://github.com/rust-ammonia/ammonia/blob/master",
                "README.md",
                "",
                "https://github.com/rust-ammonia/ammonia/blob/README.md",
            ),
            (
                "https://github.com/rust-ammonia/ammonia/blob/master",
                "README.md",
                "/",
                "https://github.com/rust-ammonia/ammonia/blob/",
            ),
            (
                "https://github.com/rust-ammonia/ammonia/blob/master",
                "README.md",
                "/CONTRIBUTING.md",
                "https://github.com/rust-ammonia/ammonia/blob/CONTRIBUTING.md",
            ),
            (
                "https://github.com/rust-ammonia/ammonia/blob/master/",
                "",
                "",
                "https://github.com/rust-ammonia/ammonia/blob/master/",
            ),
            (
                "https://github.com/rust-ammonia/ammonia/blob/master/",
                "",
                "/",
                "https://github.com/rust-ammonia/ammonia/blob/master/",
            ),
            (
                "https://github.com/rust-ammonia/ammonia/blob/master/",
                "",
                "/CONTRIBUTING.md",
                "https://github.com/rust-ammonia/ammonia/blob/master/CONTRIBUTING.md",
            ),
            (
                "https://github.com/",
                "rust-ammonia/ammonia/blob/master/README.md",
                "",
                "https://github.com/rust-ammonia/ammonia/blob/master/README.md",
            ),
            (
                "https://github.com/",
                "rust-ammonia/ammonia/blob/master/README.md",
                "/",
                "https://github.com/",
            ),
            (
                "https://github.com/",
                "rust-ammonia/ammonia/blob/master/README.md",
                "CONTRIBUTING.md",
                "https://github.com/rust-ammonia/ammonia/blob/master/CONTRIBUTING.md",
            ),
            (
                "https://github.com/",
                "rust-ammonia/ammonia/blob/master/README.md",
                "/CONTRIBUTING.md",
                "https://github.com/CONTRIBUTING.md",
            ),
        ];
        for (root, path, url, result) in tests {
            let h = format!(r#"<a href="{url}">test</a>"#);
            let r = format!(r#"<a href="{result}" rel="noopener noreferrer">test</a>"#);
            let a = Builder::new()
                .url_relative(UrlRelative::RewriteWithRoot {
                    root: Url::parse(root).unwrap(),
                    path: path.to_string(),
                })
                .clean(&h)
                .to_string();
            if r != a {
                println!(
                    "failed to check ({root}, {path}, {url}, {result})\n{r} != {a}",
                    r = r
                );
                assert_eq!(r, a);
            }
        }
    }
}
