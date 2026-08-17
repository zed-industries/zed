//! Contains parser configuration structure.
use std::collections::HashMap;
use std::io::Read;

use crate::reader::EventReader;
use crate::util::Encoding;

/// Limits to defend from billion laughs attack
const DEFAULT_MAX_ENTITY_EXPANSION_LENGTH: usize = 1_000_000;
const DEFAULT_MAX_ENTITY_EXPANSION_DEPTH: u8 = 10;

/// Parser configuration structure. **There are more config methods than public fileds — see methods below**.
///
/// This structure contains various configuration options which affect
/// behavior of the parser.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ParserConfig {
    /// Whether or not should whitespace in textual events be removed. Default is false.
    ///
    /// When true, all standalone whitespace will be removed (this means no
    /// `Whitespace` events will be emitted), and leading and trailing whitespace
    /// from `Character` events will be deleted. If after trimming `Characters`
    /// event will be empty, it will also be omitted from output stream. This is
    /// possible, however, only if `whitespace_to_characters` or
    /// `cdata_to_characters` options are set.
    ///
    /// This option does not affect CDATA events, unless `cdata_to_characters`
    /// option is also set. In that case CDATA content will also be trimmed.
    pub trim_whitespace: bool,

    /// Whether or not should whitespace be converted to characters.
    /// Default is false.
    ///
    /// If true, instead of `Whitespace` events `Characters` events with the
    /// same content will be emitted. If `trim_whitespace` is also true, these
    /// events will be trimmed to nothing and, consequently, not emitted.
    pub whitespace_to_characters: bool,

    /// Whether or not should CDATA be converted to characters.
    /// Default is false.
    ///
    /// If true, instead of `CData` events `Characters` events with the same
    /// content will be emitted. If `trim_whitespace` is also true, these events
    /// will be trimmed. If corresponding CDATA contained nothing but whitespace,
    /// this event will be omitted from the stream.
    pub cdata_to_characters: bool,

    /// Whether or not should comments be omitted. Default is true.
    ///
    /// If true, `Comment` events will not be emitted at all.
    pub ignore_comments: bool,

    /// Whether or not should sequential `Characters` events be merged.
    /// Default is true.
    ///
    /// If true, multiple sequential `Characters` events will be merged into
    /// a single event, that is, their data will be concatenated.
    ///
    /// Multiple sequential `Characters` events are only possible if either
    /// `cdata_to_characters` or `ignore_comments` are set. Otherwise character
    /// events will always be separated by other events.
    pub coalesce_characters: bool,

    /// A map of extra entities recognized by the parser. Default is an empty map.
    ///
    /// By default the XML parser recognizes the entities defined in the XML spec. Sometimes,
    /// however, it is convenient to make the parser recognize additional entities which
    /// are also not available through the DTD definitions (especially given that at the moment
    /// DTD parsing is not supported).
    pub extra_entities: HashMap<String, String>,

    /// Whether or not the parser should ignore the end of stream. Default is false.
    ///
    /// By default the parser will either error out when it encounters a premature end of
    /// stream or complete normally if the end of stream was expected. If you want to continue
    /// reading from a stream whose input is supplied progressively, you can set this option to true.
    /// In this case the parser will allow you to invoke the `next()` method even if a supposed end
    /// of stream has happened.
    ///
    /// Note that support for this functionality is incomplete; for example, the parser will fail if
    /// the premature end of stream happens inside PCDATA. Therefore, use this option at your own risk.
    pub ignore_end_of_stream: bool,

    /// Whether or not non-unicode entity references get replaced with the replacement character
    ///
    /// When true, any decimal or hexadecimal character reference that cannot be converted from a
    /// u32 to a char using [std::char::from_u32](https://doc.rust-lang.org/std/char/fn.from_u32.html)
    /// will be converted into the unicode REPLACEMENT CHARACTER (U+FFFD).
    pub replace_unknown_entity_references: bool,

    /// Whether or not whitespace at the root level of the document is ignored. Default is true.
    ///
    /// By default any whitespace that is not enclosed within at least one level of elements will be
    /// ignored. Setting this value to false will cause root level whitespace events to be emitted.
    ///
    /// **There are configuration options – see methods below**
    pub ignore_root_level_whitespace: bool,
}

impl ParserConfig {
    /// Returns a new config with default values.
    ///
    /// You can tweak default values using builder-like pattern:
    ///
    /// ```rust
    /// use xml::reader::ParserConfig;
    ///
    /// let config = ParserConfig::new()
    ///     .trim_whitespace(true)
    ///     .ignore_comments(true)
    ///     .coalesce_characters(false);
    /// ```
    #[must_use]
    #[inline]
    pub fn new() -> Self {
        Self {
            trim_whitespace: false,
            whitespace_to_characters: false,
            cdata_to_characters: false,
            ignore_comments: true,
            coalesce_characters: true,
            extra_entities: HashMap::new(),
            ignore_end_of_stream: false,
            replace_unknown_entity_references: false,
            ignore_root_level_whitespace: true,
        }
    }

    /// Creates an XML reader with this configuration. The reader should be wrapped in a `BufReader`, otherwise parsing may be very slow.
    ///
    /// This is a convenience method for configuring and creating a reader at the same time:
    ///
    /// ```rust
    /// use xml::reader::ParserConfig;
    ///
    /// let mut source: &[u8] = b"...";
    ///
    /// let reader = ParserConfig::new()
    ///     .trim_whitespace(true)
    ///     .ignore_comments(true)
    ///     .coalesce_characters(false)
    ///     .create_reader(&mut source);
    /// ```
    ///
    /// This method is exactly equivalent to calling `EventReader::new_with_config()` with
    /// this configuration object.
    #[inline]
    pub fn create_reader<R: Read>(self, source: R) -> EventReader<R> {
        EventReader::new_with_config(source, self)
    }

    /// Adds a new entity mapping and returns an updated config object.
    ///
    /// This is a convenience method for adding external entities mappings to the XML parser.
    /// An example:
    ///
    /// ```rust
    /// use xml::reader::ParserConfig;
    ///
    /// let mut source: &[u8] = b"...";
    ///
    /// let reader = ParserConfig::new()
    ///     .add_entity("nbsp", " ")
    ///     .add_entity("copy", "©")
    ///     .add_entity("reg", "®")
    ///     .create_reader(&mut source);
    /// ```
    #[must_use]
    pub fn add_entity<S: Into<String>, T: Into<String>>(mut self, entity: S, value: T) -> Self {
        self.extra_entities.insert(entity.into(), value.into());
        self
    }
}

impl Default for ParserConfig {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

gen_setters! { ParserConfig,
    trim_whitespace: val bool,
    whitespace_to_characters: val bool,
    cdata_to_characters: val bool,
    ignore_comments: val bool,
    coalesce_characters: val bool,
    ignore_end_of_stream: val bool,
    replace_unknown_entity_references: val bool,
    ignore_root_level_whitespace: val bool
}

/// Backwards-compatible extension of `ParserConfig`, which will eventually be merged into the original `ParserConfig` struct
#[derive(Clone, PartialEq, Eq, Debug)]
#[non_exhaustive]
pub struct ParserConfig2 {
    pub(crate) c: ParserConfig,

    /// Use this encoding as the default. Necessary for UTF-16 files without BOM.
    pub override_encoding: Option<Encoding>,

    /// Allow `<?xml encoding="…">` to contain unsupported encoding names,
    /// and interpret them as Latin1 instead. This will mangle non-ASCII characters, but usually it won't fail parsing.
    pub ignore_invalid_encoding_declarations: bool,

    /// Documents with multiple root elements are ill-formed
    pub allow_multiple_root_elements: bool,

    /// Abort if custom entities create a string longer than this
    pub max_entity_expansion_length: usize,
    /// Entities can expand into other entities this many times (be careful about exponential cost!)
    pub max_entity_expansion_depth: u8,

    /// Maximum length of tag name or attribute name
    pub max_name_length: usize,

    /// Max number of attributes per element
    pub max_attributes: usize,

    /// Max number of bytes in each attribute
    pub max_attribute_length: usize,

    /// Maximum length of strings reprsenting characters, comments, and processing instructions
    pub max_data_length: usize,
}

impl Default for ParserConfig2 {
    fn default() -> Self {
        Self {
            c: ParserConfig::default(),
            override_encoding: None,
            ignore_invalid_encoding_declarations: false,
            allow_multiple_root_elements: true,
            max_entity_expansion_length: DEFAULT_MAX_ENTITY_EXPANSION_LENGTH,
            max_entity_expansion_depth: DEFAULT_MAX_ENTITY_EXPANSION_DEPTH,
            max_attributes: 1 << 16,
            max_attribute_length: 1 << 30,
            max_data_length: 1 << 30,
            max_name_length: 1 << 18,
        }
    }
}

impl ParserConfig2 {
    /// Create extended configuration struct
    #[inline]
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Read character encoding from `Content-Type` header.
    /// Set this when parsing XML documents fetched over HTTP.
    ///
    /// `text/*` MIME types do *not* imply latin1. UTF-8 is always the default fallback.
    #[must_use] pub fn content_type(mut self, mime_type: &str) -> Self {
        let charset = mime_type.split_once(';')
            .and_then(|(_, args)| args.split_once("charset"))
            .and_then(|(_, args)| args.split_once('='));
        if let Some((_, charset)) = charset {
            let name = charset.trim().trim_matches('"');
            if let Ok(enc) = name.parse() {
                self.override_encoding = Some(enc);
            }
        }
        self
    }

    /// Creates an XML reader with this configuration. The reader should be wrapped in a `BufReader`, otherwise parsing may be very slow.
    ///
    /// This is a convenience method for configuring and creating a reader at the same time:
    ///
    /// ```rust
    /// use xml::reader::ParserConfig;
    ///
    /// let mut source: &[u8] = b"...";
    ///
    /// let reader = ParserConfig::new()
    ///     .trim_whitespace(true)
    ///     .ignore_comments(true)
    ///     .coalesce_characters(false)
    ///     .create_reader(&mut source);
    /// ```
    ///
    /// This method is exactly equivalent to calling `EventReader::new_with_config()` with
    /// this configuration object.
    #[inline]
    pub fn create_reader<R: Read>(self, source: R) -> EventReader<R> {
        EventReader::new_with_config(source, self)
    }
}

impl From<ParserConfig> for ParserConfig2 {
    #[inline]
    fn from(c: ParserConfig) -> Self {
        Self { c, ..Default::default() }
    }
}

gen_setters! { ParserConfig2,
    /// Set if you got one in the HTTP header
    override_encoding: val Option<Encoding>,
    /// Allows invalid documents. There should be only a single root element in XML.
    allow_multiple_root_elements: val bool,
    /// Abort if custom entities create a string longer than this
    max_entity_expansion_length: val usize,
    /// Entities can expand into other entities this many times (be careful about exponential cost!)
    max_entity_expansion_depth: val u8,
    /// Max number of attributes per element
    max_attributes: val usize,
    /// Maximum length of tag name or attribute name
    max_name_length: val usize,
    /// Max number of bytes in each attribute
    max_attribute_length: val usize,
    /// Maximum length of strings reprsenting characters, comments, and processing instructions
    max_data_length: val usize,
    /// Allow `<?xml encoding="bogus"?>`
    ignore_invalid_encoding_declarations: val bool
}

gen_setters! { ParserConfig,
    /// Set if you got one in the HTTP header (see `content_type`)
    override_encoding: c2 Option<Encoding>,
    /// Allow `<?xml encoding="bogus"?>`
    ignore_invalid_encoding_declarations: c2 bool,
    /// Allows invalid documents. There should be only a single root element in XML.
    allow_multiple_root_elements: c2 bool,

    /// Abort if custom entities create a string longer than this
    max_entity_expansion_length: c2 usize,
    /// Entities can expand into other entities this many times (be careful about exponential cost!)
    max_entity_expansion_depth: c2 u8,
    /// Max number of attributes per element
    max_attributes: c2 usize,
    /// Maximum length of tag name or attribute name
    max_name_length: c2 usize,
    /// Max number of bytes in each attribute
    max_attribute_length: c2 usize,
    /// Maximum length of strings reprsenting characters, comments, and processing instructions
    max_data_length: c2 usize,

    /// Set encoding from the MIME type. Important for HTTP compatibility.
    content_type: c2 &str
}

gen_setters! { ParserConfig2,
    trim_whitespace: delegate bool,
    whitespace_to_characters: delegate bool,
    cdata_to_characters: delegate bool,
    ignore_comments: delegate bool,
    coalesce_characters: delegate bool,
    ignore_end_of_stream: delegate bool,
    replace_unknown_entity_references: delegate bool,
    /// Whether or not whitespace at the root level of the document is ignored. Default is true.
    ignore_root_level_whitespace: delegate bool
}

#[test]
fn mime_parse() {
    let c = ParserConfig2::new().content_type("text/xml;charset=Us-AScii").max_entity_expansion_length(1000);
    assert_eq!(c.override_encoding, Some(Encoding::Ascii));

    let c = ParserConfig2::new().max_entity_expansion_depth(3).content_type("text/xml;charset = \"UTF-16\"");
    assert_eq!(c.override_encoding, Some(Encoding::Utf16));
}
