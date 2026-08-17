use std::fmt;
use std::borrow::Cow;
use std::str::FromStr;
#[allow(unused, deprecated)]
use std::ascii::AsciiExt;

use mime::Mime;
use language_tags::LanguageTag;

use parsing;
use {Header, Raw};

/// The `Link` header, defined in
/// [RFC5988](http://tools.ietf.org/html/rfc5988#section-5)
///
/// # ABNF
///
/// ```text
/// Link           = "Link" ":" #link-value
/// link-value     = "<" URI-Reference ">" *( ";" link-param )
/// link-param     = ( ( "rel" "=" relation-types )
///                | ( "anchor" "=" <"> URI-Reference <"> )
///                | ( "rev" "=" relation-types )
///                | ( "hreflang" "=" Language-Tag )
///                | ( "media" "=" ( MediaDesc | ( <"> MediaDesc <"> ) ) )
///                | ( "title" "=" quoted-string )
///                | ( "title*" "=" ext-value )
///                | ( "type" "=" ( media-type | quoted-mt ) )
///                | ( link-extension ) )
/// link-extension = ( parmname [ "=" ( ptoken | quoted-string ) ] )
///                | ( ext-name-star "=" ext-value )
/// ext-name-star  = parmname "*" ; reserved for RFC2231-profiled
/// ; extensions.  Whitespace NOT
/// ; allowed in between.
/// ptoken         = 1*ptokenchar
/// ptokenchar     = "!" | "#" | "$" | "%" | "&" | "'" | "("
///                | ")" | "*" | "+" | "-" | "." | "/" | DIGIT
///                | ":" | "<" | "=" | ">" | "?" | "@" | ALPHA
///                | "[" | "]" | "^" | "_" | "`" | "{" | "|"
///                | "}" | "~"
/// media-type     = type-name "/" subtype-name
/// quoted-mt      = <"> media-type <">
/// relation-types = relation-type
///                | <"> relation-type *( 1*SP relation-type ) <">
/// relation-type  = reg-rel-type | ext-rel-type
/// reg-rel-type   = LOALPHA *( LOALPHA | DIGIT | "." | "-" )
/// ext-rel-type   = URI
/// ```
///
/// # Example values
///
/// `Link: <http://example.com/TheBook/chapter2>; rel="previous";
///        title="previous chapter"`
///
/// `Link: </TheBook/chapter2>; rel="previous"; title*=UTF-8'de'letztes%20Kapitel,
///        </TheBook/chapter4>; rel="next"; title*=UTF-8'de'n%c3%a4chstes%20Kapitel`
///
/// # Examples
///
/// ```
/// use headers::{Headers, Link, LinkValue, RelationType};
///
/// let link_value = LinkValue::new("http://example.com/TheBook/chapter2")
///     .push_rel(RelationType::Previous)
///     .set_title("previous chapter");
///
/// let mut headers = Headers::new();
/// headers.set(
///     Link::new(vec![link_value])
/// );
/// ```
#[derive(Clone, PartialEq, Debug)]
pub struct Link {
    /// A list of the `link-value`s of the Link entity-header.
    values: Vec<LinkValue>
}

/// A single `link-value` of a `Link` header, based on:
/// [RFC5988](http://tools.ietf.org/html/rfc5988#section-5)
#[derive(Clone, PartialEq, Debug)]
pub struct LinkValue {
    /// Target IRI: `link-value`.
    link: Cow<'static, str>,

    /// Forward Relation Types: `rel`.
    rel: Option<Vec<RelationType>>,

    /// Context IRI: `anchor`.
    anchor: Option<String>,

    /// Reverse Relation Types: `rev`.
    rev: Option<Vec<RelationType>>,

    /// Hint on the language of the result of dereferencing
    /// the link: `hreflang`.
    href_lang: Option<Vec<LanguageTag>>,

    /// Destination medium or media: `media`.
    media_desc: Option<Vec<MediaDesc>>,

    /// Label of the destination of a Link: `title`.
    title: Option<String>,

    /// The `title` encoded in a different charset: `title*`.
    title_star: Option<String>,

    /// Hint on the media type of the result of dereferencing
    /// the link: `type`.
    media_type: Option<Mime>,
}

/// A Media Descriptors Enum based on:
/// [https://www.w3.org/TR/html401/types.html#h-6.13][url]
///
/// [url]: https://www.w3.org/TR/html401/types.html#h-6.13
#[derive(Clone, PartialEq, Debug)]
pub enum MediaDesc {
    /// screen.
    Screen,
    /// tty.
    Tty,
    /// tv.
    Tv,
    /// projection.
    Projection,
    /// handheld.
    Handheld,
    /// print.
    Print,
    /// braille.
    Braille,
    /// aural.
    Aural,
    /// all.
    All,
    /// Unrecognized media descriptor extension.
    Extension(String)
}

/// A Link Relation Type Enum based on:
/// [RFC5988](https://tools.ietf.org/html/rfc5988#section-6.2.2)
#[derive(Clone, PartialEq, Debug)]
pub enum RelationType {
    /// alternate.
    Alternate,
    /// appendix.
    Appendix,
    /// bookmark.
    Bookmark,
    /// chapter.
    Chapter,
    /// contents.
    Contents,
    /// copyright.
    Copyright,
    /// current.
    Current,
    /// describedby.
    DescribedBy,
    /// edit.
    Edit,
    /// edit-media.
    EditMedia,
    /// enclosure.
    Enclosure,
    /// first.
    First,
    /// glossary.
    Glossary,
    /// help.
    Help,
    /// hub.
    Hub,
    /// index.
    Index,
    /// last.
    Last,
    /// latest-version.
    LatestVersion,
    /// license.
    License,
    /// next.
    Next,
    /// next-archive.
    NextArchive,
    /// payment.
    Payment,
    /// prev.
    Prev,
    /// predecessor-version.
    PredecessorVersion,
    /// previous.
    Previous,
    /// prev-archive.
    PrevArchive,
    /// related.
    Related,
    /// replies.
    Replies,
    /// section.
    Section,
    /// self.
    RelationTypeSelf,
    /// service.
    Service,
    /// start.
    Start,
    /// stylesheet.
    Stylesheet,
    /// subsection.
    Subsection,
    /// successor-version.
    SuccessorVersion,
    /// up.
    Up,
    /// versionHistory.
    VersionHistory,
    /// via.
    Via,
    /// working-copy.
    WorkingCopy,
    /// working-copy-of.
    WorkingCopyOf,
    /// ext-rel-type.
    ExtRelType(String)
}

////////////////////////////////////////////////////////////////////////////////
// Struct methods
////////////////////////////////////////////////////////////////////////////////

impl Link {
    /// Create `Link` from a `Vec<LinkValue>`.
    pub fn new(link_values: Vec<LinkValue>) -> Link {
        Link { values: link_values }
    }

    /// Get the `Link` header's `LinkValue`s.
    pub fn values(&self) -> &[LinkValue] {
        self.values.as_ref()
    }

    /// Add a `LinkValue` instance to the `Link` header's values.
    pub fn push_value(&mut self, link_value: LinkValue) {
        self.values.push(link_value);
    }
}

impl LinkValue {
    /// Create `LinkValue` from URI-Reference.
    pub fn new<T>(uri: T) -> LinkValue
        where T: Into<Cow<'static, str>> {
        LinkValue {
            link: uri.into(),
            rel: None,
            anchor: None,
            rev: None,
            href_lang: None,
            media_desc: None,
            title: None,
            title_star: None,
            media_type: None,
        }
    }

    /// Get the `LinkValue`'s value.
    pub fn link(&self) -> &str {
        self.link.as_ref()
    }

    /// Get the `LinkValue`'s `rel` parameter(s).
    pub fn rel(&self) -> Option<&[RelationType]> {
        self.rel.as_ref().map(AsRef::as_ref)
    }

    /// Get the `LinkValue`'s `anchor` parameter.
    pub fn anchor(&self) -> Option<&str> {
        self.anchor.as_ref().map(AsRef::as_ref)
    }

    /// Get the `LinkValue`'s `rev` parameter(s).
    pub fn rev(&self) -> Option<&[RelationType]> {
        self.rev.as_ref().map(AsRef::as_ref)
    }

    /// Get the `LinkValue`'s `hreflang` parameter(s).
    pub fn href_lang(&self) -> Option<&[LanguageTag]> {
        self.href_lang.as_ref().map(AsRef::as_ref)
    }

    /// Get the `LinkValue`'s `media` parameter(s).
    pub fn media_desc(&self) -> Option<&[MediaDesc]> {
        self.media_desc.as_ref().map(AsRef::as_ref)
    }

    /// Get the `LinkValue`'s `title` parameter.
    pub fn title(&self) -> Option<&str> {
        self.title.as_ref().map(AsRef::as_ref)
    }

    /// Get the `LinkValue`'s `title*` parameter.
    pub fn title_star(&self) -> Option<&str> {
        self.title_star.as_ref().map(AsRef::as_ref)
    }

    /// Get the `LinkValue`'s `type` parameter.
    pub fn media_type(&self) -> Option<&Mime> {
        self.media_type.as_ref()
    }

    /// Add a `RelationType` to the `LinkValue`'s `rel` parameter.
    pub fn push_rel(mut self, rel: RelationType) -> LinkValue {
        let mut v = self.rel.take().unwrap_or(Vec::new());

        v.push(rel);

        self.rel = Some(v);

        self
    }

    /// Set `LinkValue`'s `anchor` parameter.
    pub fn set_anchor<T: Into<String>>(mut self, anchor: T) -> LinkValue {
        self.anchor = Some(anchor.into());

        self
    }

    /// Add a `RelationType` to the `LinkValue`'s `rev` parameter.
    pub fn push_rev(mut self, rev: RelationType) -> LinkValue {
        let mut v = self.rev.take().unwrap_or(Vec::new());

        v.push(rev);

        self.rev = Some(v);

        self
    }

    /// Add a `LanguageTag` to the `LinkValue`'s `hreflang` parameter.
    pub fn push_href_lang(mut self, language_tag: LanguageTag) -> LinkValue {
        let mut v = self.href_lang.take().unwrap_or(Vec::new());

        v.push(language_tag);

        self.href_lang = Some(v);

        self
    }

    /// Add a `MediaDesc` to the `LinkValue`'s `media_desc` parameter.
    pub fn push_media_desc(mut self, media_desc: MediaDesc) -> LinkValue {
        let mut v = self.media_desc.take().unwrap_or(Vec::new());

        v.push(media_desc);

        self.media_desc = Some(v);

        self
    }

    /// Set `LinkValue`'s `title` parameter.
    pub fn set_title<T: Into<String>>(mut self, title: T) -> LinkValue {
        self.title = Some(title.into());

        self
    }

    /// Set `LinkValue`'s `title*` parameter.
    pub fn set_title_star<T: Into<String>>(mut self, title_star: T) -> LinkValue {
        self.title_star = Some(title_star.into());

        self
    }

    /// Set `LinkValue`'s `type` parameter.
    pub fn set_media_type(mut self, media_type: Mime) -> LinkValue {
        self.media_type = Some(media_type);

        self
    }
}

////////////////////////////////////////////////////////////////////////////////
// Trait implementations
////////////////////////////////////////////////////////////////////////////////

impl Header for Link {
    fn header_name() -> &'static str {
        static NAME: &'static str = "Link";
        NAME
    }

    fn parse_header(raw: &Raw) -> ::Result<Link> {
        // If more that one `Link` headers are present in a request's
        // headers they are combined in a single `Link` header containing
        // all the `link-value`s present in each of those `Link` headers.
        raw.iter()
            .map(parsing::from_raw_str::<Link>)
            .fold(None, |p, c| {
                match (p, c) {
                    (None, c) => Some(c),
                    (e @ Some(Err(_)), _) => e,
                    (Some(Ok(mut p)), Ok(c)) => {
                        p.values.extend(c.values);

                        Some(Ok(p))
                    },
                    _ => Some(Err(::Error::Header)),
                }
            })
            .unwrap_or(Err(::Error::Header))
    }

    fn fmt_header(&self, f: &mut ::Formatter) -> fmt::Result {
        f.fmt_line(self)
    }
}

impl fmt::Display for Link {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt_delimited(f, self.values.as_slice(), ", ", ("", ""))
    }
}

impl fmt::Display for LinkValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(write!(f, "<{}>", self.link));

        if let Some(ref rel) = self.rel {
            try!(fmt_delimited(f, rel.as_slice(), " ", ("; rel=\"", "\"")));
        }
        if let Some(ref anchor) = self.anchor {
            try!(write!(f, "; anchor=\"{}\"", anchor));
        }
        if let Some(ref rev) = self.rev {
            try!(fmt_delimited(f, rev.as_slice(), " ", ("; rev=\"", "\"")));
        }
        if let Some(ref href_lang) = self.href_lang {
            for tag in href_lang {
                try!(write!(f, "; hreflang={}", tag));
            }
        }
        if let Some(ref media_desc) = self.media_desc {
            try!(fmt_delimited(f, media_desc.as_slice(), ", ", ("; media=\"", "\"")));
        }
        if let Some(ref title) = self.title {
            try!(write!(f, "; title=\"{}\"", title));
        }
        if let Some(ref title_star) = self.title_star {
            try!(write!(f, "; title*={}", title_star));
        }
        if let Some(ref media_type) = self.media_type {
            try!(write!(f, "; type=\"{}\"", media_type));
        }

        Ok(())
    }
}

impl FromStr for Link {
    type Err = ::Error;

    fn from_str(s: &str) -> ::Result<Link> {
        // Create a split iterator with delimiters: `;`, `,`
        let link_split = SplitAsciiUnquoted::new(s, ";,");

        let mut link_values: Vec<LinkValue> = Vec::new();

        // Loop over the splits parsing the Link header into
        // a `Vec<LinkValue>`
        for segment in link_split {
            // Parse the `Target IRI`
            // https://tools.ietf.org/html/rfc5988#section-5.1
            if segment.trim().starts_with('<') {
                link_values.push(
                    match verify_and_trim(segment.trim(), (b'<', b'>')) {
                        Err(_) => return Err(::Error::Header),
                        Ok(s) => {
                            LinkValue {
                                link: s.to_owned().into(),
                                rel: None,
                                anchor: None,
                                rev: None,
                                href_lang: None,
                                media_desc: None,
                                title: None,
                                title_star: None,
                                media_type: None,
                            }
                        },
                    }
                );
            } else {
                // Parse the current link-value's parameters
                let mut link_param_split = segment.splitn(2, '=');

                let link_param_name = match link_param_split.next() {
                    None => return Err(::Error::Header),
                    Some(p) => p.trim(),
                };

                let link_header = match link_values.last_mut() {
                    None => return Err(::Error::Header),
                    Some(l) => l,
                };

                if "rel".eq_ignore_ascii_case(link_param_name) {
                    // Parse relation type: `rel`.
                    // https://tools.ietf.org/html/rfc5988#section-5.3
                    if link_header.rel.is_none() {
                        link_header.rel = match link_param_split.next() {
                            None | Some("") => return Err(::Error::Header),
                            Some(s) => {
                                s.trim_matches(|c: char| c == '"' || c.is_whitespace())
                                    .split(' ')
                                    .map(|t| t.trim().parse())
                                    .collect::<Result<Vec<RelationType>, _>>()
                                    .or_else(|_| Err(::Error::Header))
                                    .ok()
                            },
                        };
                    }
                } else if "anchor".eq_ignore_ascii_case(link_param_name) {
                    // Parse the `Context IRI`.
                    // https://tools.ietf.org/html/rfc5988#section-5.2
                    link_header.anchor = match link_param_split.next() {
                        None | Some("") => return Err(::Error::Header),
                        Some(s) => match verify_and_trim(s.trim(), (b'"', b'"')) {
                            Err(_) => return Err(::Error::Header),
                            Ok(a) => Some(String::from(a)),
                        },
                    };
                } else if "rev".eq_ignore_ascii_case(link_param_name) {
                    // Parse relation type: `rev`.
                    // https://tools.ietf.org/html/rfc5988#section-5.3
                    if link_header.rev.is_none() {
                        link_header.rev = match link_param_split.next() {
                            None | Some("") => return Err(::Error::Header),
                            Some(s) => {
                                s.trim_matches(|c: char| c == '"' || c.is_whitespace())
                                    .split(' ')
                                    .map(|t| t.trim().parse())
                                    .collect::<Result<Vec<RelationType>, _>>()
                                    .or_else(|_| Err(::Error::Header))
                                    .ok()
                            },
                        }
                    }
                } else if "hreflang".eq_ignore_ascii_case(link_param_name) {
                    // Parse target attribute: `hreflang`.
                    // https://tools.ietf.org/html/rfc5988#section-5.4
                    let mut v = link_header.href_lang.take().unwrap_or(Vec::new());

                    v.push(
                        match link_param_split.next() {
                            None | Some("") => return Err(::Error::Header),
                            Some(s) => match s.trim().parse() {
                                Err(_) => return Err(::Error::Header),
                                Ok(t) => t,
                            },
                        }
                    );

                    link_header.href_lang = Some(v);
                } else if "media".eq_ignore_ascii_case(link_param_name) {
                    // Parse target attribute: `media`.
                    // https://tools.ietf.org/html/rfc5988#section-5.4
                    if link_header.media_desc.is_none() {
                        link_header.media_desc = match link_param_split.next() {
                            None | Some("") => return Err(::Error::Header),
                            Some(s) => {
                                s.trim_matches(|c: char| c == '"' || c.is_whitespace())
                                    .split(',')
                                    .map(|t| t.trim().parse())
                                    .collect::<Result<Vec<MediaDesc>, _>>()
                                    .or_else(|_| Err(::Error::Header))
                                    .ok()
                            },
                        };
                    }
                } else if "title".eq_ignore_ascii_case(link_param_name) {
                    // Parse target attribute: `title`.
                    // https://tools.ietf.org/html/rfc5988#section-5.4
                    if link_header.title.is_none() {
                        link_header.title = match link_param_split.next() {
                            None | Some("") => return Err(::Error::Header),
                            Some(s) => match verify_and_trim(s.trim(), (b'"', b'"')) {
                                Err(_) => return Err(::Error::Header),
                                Ok(t) => Some(String::from(t)),
                            },
                        };
                    }
                } else if "title*".eq_ignore_ascii_case(link_param_name) {
                    // Parse target attribute: `title*`.
                    // https://tools.ietf.org/html/rfc5988#section-5.4
                    //
                    // Definition of `ext-value`:
                    //       https://tools.ietf.org/html/rfc5987#section-3.2.1
                    if link_header.title_star.is_none() {
                        link_header.title_star = match link_param_split.next() {
                            None | Some("") => return Err(::Error::Header),
                            Some(s) => Some(String::from(s.trim())),
                        };
                    }
                } else if "type".eq_ignore_ascii_case(link_param_name) {
                    // Parse target attribute: `type`.
                    // https://tools.ietf.org/html/rfc5988#section-5.4
                    if link_header.media_type.is_none() {
                        link_header.media_type = match link_param_split.next() {
                            None | Some("") => return Err(::Error::Header),
                            Some(s) => match verify_and_trim(s.trim(), (b'"', b'"')) {
                                Err(_) => return Err(::Error::Header),
                                Ok(t) => match t.parse() {
                                    Err(_) => return Err(::Error::Header),
                                    Ok(m) => Some(m),
                                },
                            },

                        };
                    }
                } else {
                    return Err(::Error::Header);
                }
            }
        }

        Ok(Link::new(link_values))
    }
}

impl fmt::Display for MediaDesc {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            MediaDesc::Screen => write!(f, "screen"),
            MediaDesc::Tty => write!(f, "tty"),
            MediaDesc::Tv => write!(f, "tv"),
            MediaDesc::Projection => write!(f, "projection"),
            MediaDesc::Handheld => write!(f, "handheld"),
            MediaDesc::Print => write!(f, "print"),
            MediaDesc::Braille => write!(f, "braille"),
            MediaDesc::Aural => write!(f, "aural"),
            MediaDesc::All => write!(f, "all"),
            MediaDesc::Extension(ref other) => write!(f, "{}", other),
         }
    }
}

impl FromStr for MediaDesc {
    type Err = ::Error;

    fn from_str(s: &str) -> ::Result<MediaDesc> {
        match s {
            "screen" => Ok(MediaDesc::Screen),
            "tty" => Ok(MediaDesc::Tty),
            "tv" => Ok(MediaDesc::Tv),
            "projection" => Ok(MediaDesc::Projection),
            "handheld" => Ok(MediaDesc::Handheld),
            "print" => Ok(MediaDesc::Print),
            "braille" => Ok(MediaDesc::Braille),
            "aural" => Ok(MediaDesc::Aural),
            "all" => Ok(MediaDesc::All),
             _ => Ok(MediaDesc::Extension(String::from(s))),
        }
    }
}

impl fmt::Display for RelationType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RelationType::Alternate => write!(f, "alternate"),
            RelationType::Appendix => write!(f, "appendix"),
            RelationType::Bookmark => write!(f, "bookmark"),
            RelationType::Chapter => write!(f, "chapter"),
            RelationType::Contents => write!(f, "contents"),
            RelationType::Copyright => write!(f, "copyright"),
            RelationType::Current => write!(f, "current"),
            RelationType::DescribedBy => write!(f, "describedby"),
            RelationType::Edit => write!(f, "edit"),
            RelationType::EditMedia => write!(f, "edit-media"),
            RelationType::Enclosure => write!(f, "enclosure"),
            RelationType::First => write!(f, "first"),
            RelationType::Glossary => write!(f, "glossary"),
            RelationType::Help => write!(f, "help"),
            RelationType::Hub => write!(f, "hub"),
            RelationType::Index => write!(f, "index"),
            RelationType::Last => write!(f, "last"),
            RelationType::LatestVersion => write!(f, "latest-version"),
            RelationType::License => write!(f, "license"),
            RelationType::Next => write!(f, "next"),
            RelationType::NextArchive => write!(f, "next-archive"),
            RelationType::Payment => write!(f, "payment"),
            RelationType::Prev => write!(f, "prev"),
            RelationType::PredecessorVersion => write!(f, "predecessor-version"),
            RelationType::Previous => write!(f, "previous"),
            RelationType::PrevArchive => write!(f, "prev-archive"),
            RelationType::Related => write!(f, "related"),
            RelationType::Replies => write!(f, "replies"),
            RelationType::Section => write!(f, "section"),
            RelationType::RelationTypeSelf => write!(f, "self"),
            RelationType::Service => write!(f, "service"),
            RelationType::Start => write!(f, "start"),
            RelationType::Stylesheet => write!(f, "stylesheet"),
            RelationType::Subsection => write!(f, "subsection"),
            RelationType::SuccessorVersion => write!(f, "successor-version"),
            RelationType::Up => write!(f, "up"),
            RelationType::VersionHistory => write!(f, "version-history"),
            RelationType::Via => write!(f, "via"),
            RelationType::WorkingCopy => write!(f, "working-copy"),
            RelationType::WorkingCopyOf => write!(f, "working-copy-of"),
            RelationType::ExtRelType(ref uri) => write!(f, "{}", uri),
         }
    }
}

impl FromStr for RelationType {
    type Err = ::Error;

    fn from_str(s: &str) -> ::Result<RelationType> {
        if "alternate".eq_ignore_ascii_case(s) {
            Ok(RelationType::Alternate)
        } else if "appendix".eq_ignore_ascii_case(s) {
            Ok(RelationType::Appendix)
        } else if "bookmark".eq_ignore_ascii_case(s) {
            Ok(RelationType::Bookmark)
        } else if "chapter".eq_ignore_ascii_case(s) {
            Ok(RelationType::Chapter)
        } else if "contents".eq_ignore_ascii_case(s) {
            Ok(RelationType::Contents)
        } else if "copyright".eq_ignore_ascii_case(s) {
            Ok(RelationType::Copyright)
        } else if "current".eq_ignore_ascii_case(s) {
            Ok(RelationType::Current)
        } else if "describedby".eq_ignore_ascii_case(s) {
            Ok(RelationType::DescribedBy)
        } else if "edit".eq_ignore_ascii_case(s) {
            Ok(RelationType::Edit)
        } else if "edit-media".eq_ignore_ascii_case(s) {
            Ok(RelationType::EditMedia)
        } else if "enclosure".eq_ignore_ascii_case(s) {
            Ok(RelationType::Enclosure)
        } else if "first".eq_ignore_ascii_case(s) {
            Ok(RelationType::First)
        } else if "glossary".eq_ignore_ascii_case(s) {
            Ok(RelationType::Glossary)
        } else if "help".eq_ignore_ascii_case(s) {
            Ok(RelationType::Help)
        } else if "hub".eq_ignore_ascii_case(s) {
            Ok(RelationType::Hub)
        } else if "index".eq_ignore_ascii_case(s) {
            Ok(RelationType::Index)
        } else if "last".eq_ignore_ascii_case(s) {
            Ok(RelationType::Last)
        } else if "latest-version".eq_ignore_ascii_case(s) {
            Ok(RelationType::LatestVersion)
        } else if "license".eq_ignore_ascii_case(s) {
            Ok(RelationType::License)
        } else if "next".eq_ignore_ascii_case(s) {
            Ok(RelationType::Next)
        } else if "next-archive".eq_ignore_ascii_case(s) {
            Ok(RelationType::NextArchive)
        } else if "payment".eq_ignore_ascii_case(s) {
            Ok(RelationType::Payment)
        } else if "prev".eq_ignore_ascii_case(s) {
            Ok(RelationType::Prev)
        } else if "predecessor-version".eq_ignore_ascii_case(s) {
            Ok(RelationType::PredecessorVersion)
        } else if "previous".eq_ignore_ascii_case(s) {
            Ok(RelationType::Previous)
        } else if "prev-archive".eq_ignore_ascii_case(s) {
            Ok(RelationType::PrevArchive)
        } else if "related".eq_ignore_ascii_case(s) {
            Ok(RelationType::Related)
        } else if "replies".eq_ignore_ascii_case(s) {
            Ok(RelationType::Replies)
        } else if "section".eq_ignore_ascii_case(s) {
            Ok(RelationType::Section)
        } else if "self".eq_ignore_ascii_case(s) {
            Ok(RelationType::RelationTypeSelf)
        } else if "service".eq_ignore_ascii_case(s) {
            Ok(RelationType::Service)
        } else if "start".eq_ignore_ascii_case(s) {
            Ok(RelationType::Start)
        } else if "stylesheet".eq_ignore_ascii_case(s) {
            Ok(RelationType::Stylesheet)
        } else if "subsection".eq_ignore_ascii_case(s) {
            Ok(RelationType::Subsection)
        } else if "successor-version".eq_ignore_ascii_case(s) {
            Ok(RelationType::SuccessorVersion)
        } else if "up".eq_ignore_ascii_case(s) {
            Ok(RelationType::Up)
        } else if "version-history".eq_ignore_ascii_case(s) {
            Ok(RelationType::VersionHistory)
        } else if "via".eq_ignore_ascii_case(s) {
            Ok(RelationType::Via)
        } else if "working-copy".eq_ignore_ascii_case(s) {
            Ok(RelationType::WorkingCopy)
        } else if "working-copy-of".eq_ignore_ascii_case(s) {
            Ok(RelationType::WorkingCopyOf)
        } else {
            Ok(RelationType::ExtRelType(String::from(s)))
        }
    }
}

////////////////////////////////////////////////////////////////////////////////
// Utilities
////////////////////////////////////////////////////////////////////////////////

struct SplitAsciiUnquoted<'a> {
    src: &'a str,
    pos: usize,
    del: &'a str
}

impl<'a> SplitAsciiUnquoted<'a> {
    fn new(s: &'a str, d: &'a str) -> SplitAsciiUnquoted<'a> {
        SplitAsciiUnquoted{
            src: s,
            pos: 0,
            del: d,
        }
    }
}

impl<'a> Iterator for SplitAsciiUnquoted<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<&'a str> {
        if self.pos < self.src.len() {
            let prev_pos = self.pos;
            let mut pos = self.pos;

            let mut in_quotes = false;

            for c in self.src[prev_pos..].as_bytes().iter() {
                in_quotes ^= *c == b'"';

                // Ignore `c` if we're `in_quotes`.
                if !in_quotes && self.del.as_bytes().contains(c) {
                    break;
                }

                pos += 1;
            }

            self.pos = pos + 1;

            Some(&self.src[prev_pos..pos])
        } else {
            None
        }
    }
}

fn fmt_delimited<T: fmt::Display>(f: &mut fmt::Formatter, p: &[T], d: &str, b: (&str, &str)) -> fmt::Result {
    if p.len() != 0 {
        // Write a starting string `b.0` before the first element
        try!(write!(f, "{}{}", b.0, p[0]));

        for i in &p[1..] {
            // Write the next element preceded by the delimiter `d`
            try!(write!(f, "{}{}", d, i));
        }

        // Write a ending string `b.1` before the first element
        try!(write!(f, "{}", b.1));
    }

    Ok(())
}

fn verify_and_trim(s: &str, b: (u8, u8)) -> ::Result<&str> {
    let length = s.len();
    let byte_array = s.as_bytes();

    // Verify that `s` starts with `b.0` and ends with `b.1` and return
    // the contained substring after trimming whitespace.
    if length > 1 && b.0 == byte_array[0] && b.1 == byte_array[length - 1] {
        Ok(s.trim_matches(
            |c: char| c == b.0 as char || c == b.1 as char || c.is_whitespace())
        )
    } else {
        Err(::Error::Header)
    }
}

////////////////////////////////////////////////////////////////////////////////
// Tests
////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use std::fmt;
    use std::fmt::Write;

    use super::{Link, LinkValue, MediaDesc, RelationType, SplitAsciiUnquoted};
    use super::{fmt_delimited, verify_and_trim};

    use Header;

    // use proto::ServerTransaction;
    use bytes::BytesMut;

    use mime;

    #[test]
    fn test_link() {
        let link_value = LinkValue::new("http://example.com/TheBook/chapter2")
            .push_rel(RelationType::Previous)
            .push_rev(RelationType::Next)
            .set_title("previous chapter");

        let link_header = b"<http://example.com/TheBook/chapter2>; \
            rel=\"previous\"; rev=next; title=\"previous chapter\"";

        let expected_link = Link::new(vec![link_value]);

        let link = Header::parse_header(&vec![link_header.to_vec()].into());
        assert_eq!(link.ok(), Some(expected_link));
    }

    #[test]
    fn test_link_multiple_values() {
        let first_link = LinkValue::new("/TheBook/chapter2")
            .push_rel(RelationType::Previous)
            .set_title_star("UTF-8'de'letztes%20Kapitel");

        let second_link = LinkValue::new("/TheBook/chapter4")
            .push_rel(RelationType::Next)
            .set_title_star("UTF-8'de'n%c3%a4chstes%20Kapitel");

        let link_header = b"</TheBook/chapter2>; \
            rel=\"previous\"; title*=UTF-8'de'letztes%20Kapitel, \
            </TheBook/chapter4>; \
            rel=\"next\"; title*=UTF-8'de'n%c3%a4chstes%20Kapitel";

        let expected_link = Link::new(vec![first_link, second_link]);

        let link = Header::parse_header(&vec![link_header.to_vec()].into());
        assert_eq!(link.ok(), Some(expected_link));
    }

    #[test]
    fn test_link_all_attributes() {
        let link_value = LinkValue::new("http://example.com/TheBook/chapter2")
            .push_rel(RelationType::Previous)
            .set_anchor("../anchor/example/")
            .push_rev(RelationType::Next)
            .push_href_lang("de".parse().unwrap())
            .push_media_desc(MediaDesc::Screen)
            .set_title("previous chapter")
            .set_title_star("title* unparsed")
            .set_media_type(mime::TEXT_PLAIN);

        let link_header = b"<http://example.com/TheBook/chapter2>; \
            rel=\"previous\"; anchor=\"../anchor/example/\"; \
            rev=\"next\"; hreflang=de; media=\"screen\"; \
            title=\"previous chapter\"; title*=title* unparsed; \
            type=\"text/plain\"";

        let expected_link = Link::new(vec![link_value]);

        let link = Header::parse_header(&vec![link_header.to_vec()].into());
        assert_eq!(link.ok(), Some(expected_link));
    }

    // TODO
    // #[test]
    // fn test_link_multiple_link_headers() {
    //     let first_link = LinkValue::new("/TheBook/chapter2")
    //         .push_rel(RelationType::Previous)
    //         .set_title_star("UTF-8'de'letztes%20Kapitel");

    //     let second_link = LinkValue::new("/TheBook/chapter4")
    //         .push_rel(RelationType::Next)
    //         .set_title_star("UTF-8'de'n%c3%a4chstes%20Kapitel");

    //     let third_link = LinkValue::new("http://example.com/TheBook/chapter2")
    //         .push_rel(RelationType::Previous)
    //         .push_rev(RelationType::Next)
    //         .set_title("previous chapter");

    //     let expected_link = Link::new(vec![first_link, second_link, third_link]);

    //     let mut raw = BytesMut::from(b"GET /super_short_uri/and_whatever HTTP/1.1\r\nHost: \
    //                               hyper.rs\r\nAccept: a lot of things\r\nAccept-Charset: \
    //                               utf8\r\nAccept-Encoding: *\r\nLink: </TheBook/chapter2>; \
    //                               rel=\"previous\"; title*=UTF-8'de'letztes%20Kapitel, \
    //                               </TheBook/chapter4>; rel=\"next\"; title*=\
    //                               UTF-8'de'n%c3%a4chstes%20Kapitel\r\n\
    //                               Access-Control-Allow-Credentials: None\r\nLink: \
    //                               <http://example.com/TheBook/chapter2>; \
    //                               rel=\"previous\"; rev=next; title=\"previous chapter\"\
    //                               \r\n\r\n".to_vec());

    //     let (mut res, _) = ServerTransaction::parse(&mut raw).unwrap().unwrap();

    //     let link = res.headers.remove::<Link>().unwrap();

    //     assert_eq!(link, expected_link);
    // }

    #[test]
    fn test_link_display() {
        let link_value = LinkValue::new("http://example.com/TheBook/chapter2")
            .push_rel(RelationType::Previous)
            .set_anchor("/anchor/example/")
            .push_rev(RelationType::Next)
            .push_href_lang("de".parse().unwrap())
            .push_media_desc(MediaDesc::Screen)
            .set_title("previous chapter")
            .set_title_star("title* unparsed")
            .set_media_type(mime::TEXT_PLAIN);

        let link = Link::new(vec![link_value]);

        let mut link_header = String::new();
        write!(&mut link_header, "{}", link).unwrap();

        let expected_link_header = "<http://example.com/TheBook/chapter2>; \
            rel=\"previous\"; anchor=\"/anchor/example/\"; \
            rev=\"next\"; hreflang=de; media=\"screen\"; \
            title=\"previous chapter\"; title*=title* unparsed; \
            type=\"text/plain\"";

        assert_eq!(link_header, expected_link_header);
    }

    #[test]
    fn test_link_parsing_errors() {
        let link_a  = b"http://example.com/TheBook/chapter2; \
            rel=\"previous\"; rev=next; title=\"previous chapter\"";

        let mut err: Result<Link, _> = Header::parse_header(&vec![link_a.to_vec()].into());
        assert_eq!(err.is_err(), true);

        let link_b = b"<http://example.com/TheBook/chapter2>; \
            =\"previous\"; rev=next; title=\"previous chapter\"";

        err = Header::parse_header(&vec![link_b.to_vec()].into());
        assert_eq!(err.is_err(), true);

        let link_c = b"<http://example.com/TheBook/chapter2>; \
            rel=; rev=next; title=\"previous chapter\"";

        err = Header::parse_header(&vec![link_c.to_vec()].into());
        assert_eq!(err.is_err(), true);

        let link_d = b"<http://example.com/TheBook/chapter2>; \
            rel=\"previous\"; rev=next; title=";

        err = Header::parse_header(&vec![link_d.to_vec()].into());
        assert_eq!(err.is_err(), true);

        let link_e = b"<http://example.com/TheBook/chapter2>; \
            rel=\"previous\"; rev=next; attr=unknown";

        err = Header::parse_header(&vec![link_e.to_vec()].into());
        assert_eq!(err.is_err(), true);
     }

    #[test]
    fn test_link_split_ascii_unquoted_iterator() {
        let string = "some, text; \"and, more; in quotes\", or not";
        let mut string_split = SplitAsciiUnquoted::new(string, ";,");

        assert_eq!(Some("some"), string_split.next());
        assert_eq!(Some(" text"), string_split.next());
        assert_eq!(Some(" \"and, more; in quotes\""), string_split.next());
        assert_eq!(Some(" or not"), string_split.next());
        assert_eq!(None, string_split.next());
    }

    #[test]
    fn test_link_fmt_delimited() {
        struct TestFormatterStruct<'a> { v: Vec<&'a str> };

        impl<'a> fmt::Display for TestFormatterStruct<'a> {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                fmt_delimited(f, self.v.as_slice(), ", ", (">>", "<<"))
            }
        }

        let test_formatter = TestFormatterStruct { v: vec!["first", "second"] };

        let mut string = String::new();
        write!(&mut string, "{}", test_formatter).unwrap();

        let expected_string = ">>first, second<<";

        assert_eq!(string, expected_string);
    }

    #[test]
    fn test_link_verify_and_trim() {
        let string = verify_and_trim(">  some string   <", (b'>', b'<'));
        assert_eq!(string.ok(), Some("some string"));

        let err = verify_and_trim(" >  some string   <", (b'>', b'<'));
        assert_eq!(err.is_err(), true);
    }
}

bench_header!(bench_link, Link, { vec![b"<http://example.com/TheBook/chapter2>; rel=\"previous\"; rev=next; title=\"previous chapter\"; type=\"text/html\"; media=\"screen, tty\"".to_vec()] });
