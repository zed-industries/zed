use crate::libc;
use core::ops::Deref;
use core::ptr::{self, addr_of};

pub use self::{yaml_encoding_t::*, yaml_event_type_t::*, yaml_node_type_t::*};
pub use core::primitive::{i64 as ptrdiff_t, u64 as size_t, u8 as yaml_char_t};

/// The version directive data.
#[derive(Copy, Clone)]
#[repr(C)]
#[non_exhaustive]
pub struct yaml_version_directive_t {
    /// The major version number.
    pub major: libc::c_int,
    /// The minor version number.
    pub minor: libc::c_int,
}

/// The tag directive data.
#[derive(Copy, Clone)]
#[repr(C)]
#[non_exhaustive]
pub struct yaml_tag_directive_t {
    /// The tag handle.
    pub handle: *mut yaml_char_t,
    /// The tag prefix.
    pub prefix: *mut yaml_char_t,
}

/// The stream encoding.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
#[repr(u32)]
#[non_exhaustive]
pub enum yaml_encoding_t {
    /// Let the parser choose the encoding.
    YAML_ANY_ENCODING = 0,
    /// The default UTF-8 encoding.
    YAML_UTF8_ENCODING = 1,
    /// The UTF-16-LE encoding with BOM.
    YAML_UTF16LE_ENCODING = 2,
    /// The UTF-16-BE encoding with BOM.
    YAML_UTF16BE_ENCODING = 3,
}

/// Line break type.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
#[repr(u32)]
#[non_exhaustive]
pub enum yaml_break_t {
    /// Let the parser choose the break type.
    YAML_ANY_BREAK = 0,
    /// Use CR for line breaks (Mac style).
    YAML_CR_BREAK = 1,
    /// Use LN for line breaks (Unix style).
    YAML_LN_BREAK = 2,
    /// Use CR LN for line breaks (DOS style).
    YAML_CRLN_BREAK = 3,
}

/// Many bad things could happen with the parser and emitter.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
#[repr(u32)]
#[non_exhaustive]
pub enum yaml_error_type_t {
    /// No error is produced.
    YAML_NO_ERROR = 0,
    /// Cannot allocate or reallocate a block of memory.
    YAML_MEMORY_ERROR = 1,
    /// Cannot read or decode the input stream.
    YAML_READER_ERROR = 2,
    /// Cannot scan the input stream.
    YAML_SCANNER_ERROR = 3,
    /// Cannot parse the input stream.
    YAML_PARSER_ERROR = 4,
    /// Cannot compose a YAML document.
    YAML_COMPOSER_ERROR = 5,
    /// Cannot write to the output stream.
    YAML_WRITER_ERROR = 6,
    /// Cannot emit a YAML stream.
    YAML_EMITTER_ERROR = 7,
}

/// The pointer position.
#[derive(Copy, Clone)]
#[repr(C)]
#[non_exhaustive]
pub struct yaml_mark_t {
    /// The position index.
    pub index: size_t,
    /// The position line.
    pub line: size_t,
    /// The position column.
    pub column: size_t,
}

/// Scalar styles.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
#[repr(u32)]
#[non_exhaustive]
pub enum yaml_scalar_style_t {
    /// Let the emitter choose the style.
    YAML_ANY_SCALAR_STYLE = 0,
    /// The plain scalar style.
    YAML_PLAIN_SCALAR_STYLE = 1,
    /// The single-quoted scalar style.
    YAML_SINGLE_QUOTED_SCALAR_STYLE = 2,
    /// The double-quoted scalar style.
    YAML_DOUBLE_QUOTED_SCALAR_STYLE = 3,
    /// The literal scalar style.
    YAML_LITERAL_SCALAR_STYLE = 4,
    /// The folded scalar style.
    YAML_FOLDED_SCALAR_STYLE = 5,
}

/// Sequence styles.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
#[repr(u32)]
#[non_exhaustive]
pub enum yaml_sequence_style_t {
    /// Let the emitter choose the style.
    YAML_ANY_SEQUENCE_STYLE = 0,
    /// The block sequence style.
    YAML_BLOCK_SEQUENCE_STYLE = 1,
    /// The flow sequence style.
    YAML_FLOW_SEQUENCE_STYLE = 2,
}

/// Mapping styles.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
#[repr(u32)]
#[non_exhaustive]
pub enum yaml_mapping_style_t {
    /// Let the emitter choose the style.
    YAML_ANY_MAPPING_STYLE = 0,
    /// The block mapping style.
    YAML_BLOCK_MAPPING_STYLE = 1,
    /// The flow mapping style.
    YAML_FLOW_MAPPING_STYLE = 2,
}

/// Token types.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
#[repr(u32)]
#[non_exhaustive]
pub enum yaml_token_type_t {
    /// An empty token.
    YAML_NO_TOKEN = 0,
    /// A STREAM-START token.
    YAML_STREAM_START_TOKEN = 1,
    /// A STREAM-END token.
    YAML_STREAM_END_TOKEN = 2,
    /// A VERSION-DIRECTIVE token.
    YAML_VERSION_DIRECTIVE_TOKEN = 3,
    /// A TAG-DIRECTIVE token.
    YAML_TAG_DIRECTIVE_TOKEN = 4,
    /// A DOCUMENT-START token.
    YAML_DOCUMENT_START_TOKEN = 5,
    /// A DOCUMENT-END token.
    YAML_DOCUMENT_END_TOKEN = 6,
    /// A BLOCK-SEQUENCE-START token.
    YAML_BLOCK_SEQUENCE_START_TOKEN = 7,
    /// A BLOCK-MAPPING-START token.
    YAML_BLOCK_MAPPING_START_TOKEN = 8,
    /// A BLOCK-END token.
    YAML_BLOCK_END_TOKEN = 9,
    /// A FLOW-SEQUENCE-START token.
    YAML_FLOW_SEQUENCE_START_TOKEN = 10,
    /// A FLOW-SEQUENCE-END token.
    YAML_FLOW_SEQUENCE_END_TOKEN = 11,
    /// A FLOW-MAPPING-START token.
    YAML_FLOW_MAPPING_START_TOKEN = 12,
    /// A FLOW-MAPPING-END token.
    YAML_FLOW_MAPPING_END_TOKEN = 13,
    /// A BLOCK-ENTRY token.
    YAML_BLOCK_ENTRY_TOKEN = 14,
    /// A FLOW-ENTRY token.
    YAML_FLOW_ENTRY_TOKEN = 15,
    /// A KEY token.
    YAML_KEY_TOKEN = 16,
    /// A VALUE token.
    YAML_VALUE_TOKEN = 17,
    /// An ALIAS token.
    YAML_ALIAS_TOKEN = 18,
    /// An ANCHOR token.
    YAML_ANCHOR_TOKEN = 19,
    /// A TAG token.
    YAML_TAG_TOKEN = 20,
    /// A SCALAR token.
    YAML_SCALAR_TOKEN = 21,
}

/// The token structure.
#[derive(Copy, Clone)]
#[repr(C)]
#[non_exhaustive]
pub struct yaml_token_t {
    /// The token type.
    pub type_: yaml_token_type_t,
    /// The token data.
    ///
    /// ```
    /// # const _: &str = stringify! {
    /// union {
    ///     /// The stream start (for YAML_STREAM_START_TOKEN).
    ///     stream_start: struct {
    ///         /// The stream encoding.
    ///         encoding: yaml_encoding_t,
    ///     },
    ///     /// The alias (for YAML_ALIAS_TOKEN).
    ///     alias: struct {
    ///         /// The alias value.
    ///         value: *mut u8,
    ///     },
    ///     /// The anchor (for YAML_ANCHOR_TOKEN).
    ///     anchor: struct {
    ///         /// The anchor value.
    ///         value: *mut u8,
    ///     },
    ///     /// The tag (for YAML_TAG_TOKEN).
    ///     tag: struct {
    ///         /// The tag handle.
    ///         handle: *mut u8,
    ///         /// The tag suffix.
    ///         suffix: *mut u8,
    ///     },
    ///     /// The scalar value (for YAML_SCALAR_TOKEN).
    ///     scalar: struct {
    ///         /// The scalar value.
    ///         value: *mut u8,
    ///         /// The length of the scalar value.
    ///         length: u64,
    ///         /// The scalar style.
    ///         style: yaml_scalar_style_t,
    ///     },
    ///     /// The version directive (for YAML_VERSION_DIRECTIVE_TOKEN).
    ///     version_directive: struct {
    ///         /// The major version number.
    ///         major: i32,
    ///         /// The minor version number.
    ///         minor: i32,
    ///     },
    ///     /// The tag directive (for YAML_TAG_DIRECTIVE_TOKEN).
    ///     tag_directive: struct {
    ///         /// The tag handle.
    ///         handle: *mut u8,
    ///         /// The tag prefix.
    ///         prefix: *mut u8,
    ///     },
    /// }
    /// # };
    /// ```
    pub data: unnamed_yaml_token_t_data,
    /// The beginning of the token.
    pub start_mark: yaml_mark_t,
    /// The end of the token.
    pub end_mark: yaml_mark_t,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub union unnamed_yaml_token_t_data {
    /// The stream start (for YAML_STREAM_START_TOKEN).
    pub stream_start: unnamed_yaml_token_t_data_stream_start,
    /// The alias (for YAML_ALIAS_TOKEN).
    pub alias: unnamed_yaml_token_t_data_alias,
    /// The anchor (for YAML_ANCHOR_TOKEN).
    pub anchor: unnamed_yaml_token_t_data_anchor,
    /// The tag (for YAML_TAG_TOKEN).
    pub tag: unnamed_yaml_token_t_data_tag,
    /// The scalar value (for YAML_SCALAR_TOKEN).
    pub scalar: unnamed_yaml_token_t_data_scalar,
    /// The version directive (for YAML_VERSION_DIRECTIVE_TOKEN).
    pub version_directive: unnamed_yaml_token_t_data_version_directive,
    /// The tag directive (for YAML_TAG_DIRECTIVE_TOKEN).
    pub tag_directive: unnamed_yaml_token_t_data_tag_directive,
}

#[derive(Copy, Clone)]
#[repr(C)]
#[non_exhaustive]
pub struct unnamed_yaml_token_t_data_stream_start {
    /// The stream encoding.
    pub encoding: yaml_encoding_t,
}

#[derive(Copy, Clone)]
#[repr(C)]
#[non_exhaustive]
pub struct unnamed_yaml_token_t_data_alias {
    /// The alias value.
    pub value: *mut yaml_char_t,
}

#[derive(Copy, Clone)]
#[repr(C)]
#[non_exhaustive]
pub struct unnamed_yaml_token_t_data_anchor {
    /// The anchor value.
    pub value: *mut yaml_char_t,
}

#[derive(Copy, Clone)]
#[repr(C)]
#[non_exhaustive]
pub struct unnamed_yaml_token_t_data_tag {
    /// The tag handle.
    pub handle: *mut yaml_char_t,
    /// The tag suffix.
    pub suffix: *mut yaml_char_t,
}

#[derive(Copy, Clone)]
#[repr(C)]
#[non_exhaustive]
pub struct unnamed_yaml_token_t_data_scalar {
    /// The scalar value.
    pub value: *mut yaml_char_t,
    /// The length of the scalar value.
    pub length: size_t,
    /// The scalar style.
    pub style: yaml_scalar_style_t,
}

#[derive(Copy, Clone)]
#[repr(C)]
#[non_exhaustive]
pub struct unnamed_yaml_token_t_data_version_directive {
    /// The major version number.
    pub major: libc::c_int,
    /// The minor version number.
    pub minor: libc::c_int,
}

#[derive(Copy, Clone)]
#[repr(C)]
#[non_exhaustive]
pub struct unnamed_yaml_token_t_data_tag_directive {
    /// The tag handle.
    pub handle: *mut yaml_char_t,
    /// The tag prefix.
    pub prefix: *mut yaml_char_t,
}

/// Event types.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
#[repr(u32)]
#[non_exhaustive]
pub enum yaml_event_type_t {
    /// An empty event.
    YAML_NO_EVENT = 0,
    /// A STREAM-START event.
    YAML_STREAM_START_EVENT = 1,
    /// A STREAM-END event.
    YAML_STREAM_END_EVENT = 2,
    /// A DOCUMENT-START event.
    YAML_DOCUMENT_START_EVENT = 3,
    /// A DOCUMENT-END event.
    YAML_DOCUMENT_END_EVENT = 4,
    /// An ALIAS event.
    YAML_ALIAS_EVENT = 5,
    /// A SCALAR event.
    YAML_SCALAR_EVENT = 6,
    /// A SEQUENCE-START event.
    YAML_SEQUENCE_START_EVENT = 7,
    /// A SEQUENCE-END event.
    YAML_SEQUENCE_END_EVENT = 8,
    /// A MAPPING-START event.
    YAML_MAPPING_START_EVENT = 9,
    /// A MAPPING-END event.
    YAML_MAPPING_END_EVENT = 10,
}

/// The event structure.
#[derive(Copy, Clone)]
#[repr(C)]
#[non_exhaustive]
pub struct yaml_event_t {
    /// The event type.
    pub type_: yaml_event_type_t,
    /// The event data.
    ///
    /// ```
    /// # const _: &str = stringify! {
    /// union {
    ///     /// The stream parameters (for YAML_STREAM_START_EVENT).
    ///     stream_start: struct {
    ///         /// The document encoding.
    ///         encoding: yaml_encoding_t,
    ///     },
    ///     /// The document parameters (for YAML_DOCUMENT_START_EVENT).
    ///     document_start: struct {
    ///         /// The version directive.
    ///         version_directive: *mut yaml_version_directive_t,
    ///         /// The list of tag directives.
    ///         tag_directives: struct {
    ///             /// The beginning of the tag directives list.
    ///             start: *mut yaml_tag_directive_t,
    ///             /// The end of the tag directives list.
    ///             end: *mut yaml_tag_directive_t,
    ///         },
    ///         /// Is the document indicator implicit?
    ///         implicit: i32,
    ///     },
    ///     /// The document end parameters (for YAML_DOCUMENT_END_EVENT).
    ///     document_end: struct {
    ///         /// Is the document end indicator implicit?
    ///         implicit: i32,
    ///     },
    ///     /// The alias parameters (for YAML_ALIAS_EVENT).
    ///     alias: struct {
    ///         /// The anchor.
    ///         anchor: *mut u8,
    ///     },
    ///     /// The scalar parameters (for YAML_SCALAR_EVENT).
    ///     scalar: struct {
    ///         /// The anchor.
    ///         anchor: *mut u8,
    ///         /// The tag.
    ///         tag: *mut u8,
    ///         /// The scalar value.
    ///         value: *mut u8,
    ///         /// The length of the scalar value.
    ///         length: u64,
    ///         /// Is the tag optional for the plain style?
    ///         plain_implicit: i32,
    ///         /// Is the tag optional for any non-plain style?
    ///         quoted_implicit: i32,
    ///         /// The scalar style.
    ///         style: yaml_scalar_style_t,
    ///     },
    ///     /// The sequence parameters (for YAML_SEQUENCE_START_EVENT).
    ///     sequence_start: struct {
    ///         /// The anchor.
    ///         anchor: *mut u8,
    ///         /// The tag.
    ///         tag: *mut u8,
    ///         /// Is the tag optional?
    ///         implicit: i32,
    ///         /// The sequence style.
    ///         style: yaml_sequence_style_t,
    ///     },
    ///     /// The mapping parameters (for YAML_MAPPING_START_EVENT).
    ///     mapping_start: struct {
    ///         /// The anchor.
    ///         anchor: *mut u8,
    ///         /// The tag.
    ///         tag: *mut u8,
    ///         /// Is the tag optional?
    ///         implicit: i32,
    ///         /// The mapping style.
    ///         style: yaml_mapping_style_t,
    ///     },
    /// }
    /// # };
    /// ```
    pub data: unnamed_yaml_event_t_data,
    /// The beginning of the event.
    pub start_mark: yaml_mark_t,
    /// The end of the event.
    pub end_mark: yaml_mark_t,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub union unnamed_yaml_event_t_data {
    /// The stream parameters (for YAML_STREAM_START_EVENT).
    pub stream_start: unnamed_yaml_event_t_data_stream_start,
    /// The document parameters (for YAML_DOCUMENT_START_EVENT).
    pub document_start: unnamed_yaml_event_t_data_document_start,
    /// The document end parameters (for YAML_DOCUMENT_END_EVENT).
    pub document_end: unnamed_yaml_event_t_data_document_end,
    /// The alias parameters (for YAML_ALIAS_EVENT).
    pub alias: unnamed_yaml_event_t_data_alias,
    /// The scalar parameters (for YAML_SCALAR_EVENT).
    pub scalar: unnamed_yaml_event_t_data_scalar,
    /// The sequence parameters (for YAML_SEQUENCE_START_EVENT).
    pub sequence_start: unnamed_yaml_event_t_data_sequence_start,
    /// The mapping parameters (for YAML_MAPPING_START_EVENT).
    pub mapping_start: unnamed_yaml_event_t_data_mapping_start,
}

#[derive(Copy, Clone)]
#[repr(C)]
#[non_exhaustive]
pub struct unnamed_yaml_event_t_data_stream_start {
    /// The document encoding.
    pub encoding: yaml_encoding_t,
}

#[derive(Copy, Clone)]
#[repr(C)]
#[non_exhaustive]
pub struct unnamed_yaml_event_t_data_document_start {
    /// The version directive.
    pub version_directive: *mut yaml_version_directive_t,
    /// The list of tag directives.
    pub tag_directives: unnamed_yaml_event_t_data_document_start_tag_directives,
    /// Is the document indicator implicit?
    pub implicit: bool,
}

#[derive(Copy, Clone)]
#[repr(C)]
#[non_exhaustive]
pub struct unnamed_yaml_event_t_data_document_start_tag_directives {
    /// The beginning of the tag directives list.
    pub start: *mut yaml_tag_directive_t,
    /// The end of the tag directives list.
    pub end: *mut yaml_tag_directive_t,
}

#[derive(Copy, Clone)]
#[repr(C)]
#[non_exhaustive]
pub struct unnamed_yaml_event_t_data_document_end {
    /// Is the document end indicator implicit?
    pub implicit: bool,
}

#[derive(Copy, Clone)]
#[repr(C)]
#[non_exhaustive]
pub struct unnamed_yaml_event_t_data_alias {
    /// The anchor.
    pub anchor: *mut yaml_char_t,
}

#[derive(Copy, Clone)]
#[repr(C)]
#[non_exhaustive]
pub struct unnamed_yaml_event_t_data_scalar {
    /// The anchor.
    pub anchor: *mut yaml_char_t,
    /// The tag.
    pub tag: *mut yaml_char_t,
    /// The scalar value.
    pub value: *mut yaml_char_t,
    /// The length of the scalar value.
    pub length: size_t,
    /// Is the tag optional for the plain style?
    pub plain_implicit: bool,
    /// Is the tag optional for any non-plain style?
    pub quoted_implicit: bool,
    /// The scalar style.
    pub style: yaml_scalar_style_t,
}

#[derive(Copy, Clone)]
#[repr(C)]
#[non_exhaustive]
pub struct unnamed_yaml_event_t_data_sequence_start {
    /// The anchor.
    pub anchor: *mut yaml_char_t,
    /// The tag.
    pub tag: *mut yaml_char_t,
    /// Is the tag optional?
    pub implicit: bool,
    /// The sequence style.
    pub style: yaml_sequence_style_t,
}

#[derive(Copy, Clone)]
#[repr(C)]
#[non_exhaustive]
pub struct unnamed_yaml_event_t_data_mapping_start {
    /// The anchor.
    pub anchor: *mut yaml_char_t,
    /// The tag.
    pub tag: *mut yaml_char_t,
    /// Is the tag optional?
    pub implicit: bool,
    /// The mapping style.
    pub style: yaml_mapping_style_t,
}

/// Node types.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
#[repr(u32)]
#[non_exhaustive]
pub enum yaml_node_type_t {
    /// An empty node.
    YAML_NO_NODE = 0,
    /// A scalar node.
    YAML_SCALAR_NODE = 1,
    /// A sequence node.
    YAML_SEQUENCE_NODE = 2,
    /// A mapping node.
    YAML_MAPPING_NODE = 3,
}

/// The node structure.
#[derive(Copy, Clone)]
#[repr(C)]
#[non_exhaustive]
pub struct yaml_node_t {
    /// The node type.
    pub type_: yaml_node_type_t,
    /// The node tag.
    pub tag: *mut yaml_char_t,
    /// The node data.
    ///
    /// ```
    /// # const _: &str = stringify! {
    /// union {
    ///     /// The scalar parameters (for YAML_SCALAR_NODE).
    ///     scalar: struct {
    ///         /// The scalar value.
    ///         value: *mut u8,
    ///         /// The length of the scalar value.
    ///         length: u64,
    ///         /// The scalar style.
    ///         style: yaml_scalar_style_t,
    ///     },
    ///     /// The sequence parameters (for YAML_SEQUENCE_NODE).
    ///     sequence: struct {
    ///         /// The stack of sequence items.
    ///         items: yaml_stack_t<yaml_node_item_t>,
    ///         /// The sequence style.
    ///         style: yaml_sequence_style_t,
    ///     },
    ///     /// The mapping parameters (for YAML_MAPPING_NODE).
    ///     mapping: struct {
    ///         /// The stack of mapping pairs (key, value).
    ///         pairs: yaml_stack_t<yaml_node_pair_t>,
    ///         /// The mapping style.
    ///         style: yaml_mapping_style_t,
    ///     },
    /// }
    /// # };
    /// ```
    pub data: unnamed_yaml_node_t_data,
    /// The beginning of the node.
    pub start_mark: yaml_mark_t,
    /// The end of the node.
    pub end_mark: yaml_mark_t,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub union unnamed_yaml_node_t_data {
    /// The scalar parameters (for YAML_SCALAR_NODE).
    pub scalar: unnamed_yaml_node_t_data_scalar,
    /// The sequence parameters (for YAML_SEQUENCE_NODE).
    pub sequence: unnamed_yaml_node_t_data_sequence,
    /// The mapping parameters (for YAML_MAPPING_NODE).
    pub mapping: unnamed_yaml_node_t_data_mapping,
}

#[derive(Copy, Clone)]
#[repr(C)]
#[non_exhaustive]
pub struct unnamed_yaml_node_t_data_scalar {
    /// The scalar value.
    pub value: *mut yaml_char_t,
    /// The length of the scalar value.
    pub length: size_t,
    /// The scalar style.
    pub style: yaml_scalar_style_t,
}

/// An element of a sequence node.
pub type yaml_node_item_t = libc::c_int;

#[derive(Copy, Clone)]
#[repr(C)]
#[non_exhaustive]
pub struct unnamed_yaml_node_t_data_sequence {
    /// The stack of sequence items.
    pub items: yaml_stack_t<yaml_node_item_t>,
    /// The sequence style.
    pub style: yaml_sequence_style_t,
}

#[derive(Copy, Clone)]
#[repr(C)]
#[non_exhaustive]
pub struct unnamed_yaml_node_t_data_mapping {
    /// The stack of mapping pairs (key, value).
    pub pairs: yaml_stack_t<yaml_node_pair_t>,
    /// The mapping style.
    pub style: yaml_mapping_style_t,
}

/// An element of a mapping node.
#[derive(Copy, Clone)]
#[repr(C)]
#[non_exhaustive]
pub struct yaml_node_pair_t {
    /// The key of the element.
    pub key: libc::c_int,
    /// The value of the element.
    pub value: libc::c_int,
}

/// The document structure.
#[derive(Copy, Clone)]
#[repr(C)]
#[non_exhaustive]
pub struct yaml_document_t {
    /// The document nodes.
    pub nodes: yaml_stack_t<yaml_node_t>,
    /// The version directive.
    pub version_directive: *mut yaml_version_directive_t,
    /// The list of tag directives.
    ///
    /// ```
    /// # const _: &str = stringify! {
    /// struct {
    ///     /// The beginning of the tag directives list.
    ///     start: *mut yaml_tag_directive_t,
    ///     /// The end of the tag directives list.
    ///     end: *mut yaml_tag_directive_t,
    /// }
    /// # };
    /// ```
    pub tag_directives: unnamed_yaml_document_t_tag_directives,
    /// Is the document start indicator implicit?
    pub start_implicit: bool,
    /// Is the document end indicator implicit?
    pub end_implicit: bool,
    /// The beginning of the document.
    pub start_mark: yaml_mark_t,
    /// The end of the document.
    pub end_mark: yaml_mark_t,
}

#[derive(Copy, Clone)]
#[repr(C)]
#[non_exhaustive]
pub struct unnamed_yaml_document_t_tag_directives {
    /// The beginning of the tag directives list.
    pub start: *mut yaml_tag_directive_t,
    /// The end of the tag directives list.
    pub end: *mut yaml_tag_directive_t,
}

/// The prototype of a read handler.
///
/// The read handler is called when the parser needs to read more bytes from the
/// source. The handler should write not more than `size` bytes to the `buffer`.
/// The number of written bytes should be set to the `length` variable.
///
/// On success, the handler should return 1. If the handler failed, the returned
/// value should be 0. On EOF, the handler should set the `size_read` to 0 and
/// return 1.
pub type yaml_read_handler_t = unsafe fn(
    data: *mut libc::c_void,
    buffer: *mut libc::c_uchar,
    size: size_t,
    size_read: *mut size_t,
) -> libc::c_int;

/// This structure holds information about a potential simple key.
#[derive(Copy, Clone)]
#[repr(C)]
#[non_exhaustive]
pub struct yaml_simple_key_t {
    /// Is a simple key possible?
    pub possible: bool,
    /// Is a simple key required?
    pub required: bool,
    /// The number of the token.
    pub token_number: size_t,
    /// The position mark.
    pub mark: yaml_mark_t,
}

/// The states of the parser.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
#[repr(u32)]
#[non_exhaustive]
pub enum yaml_parser_state_t {
    /// Expect STREAM-START.
    YAML_PARSE_STREAM_START_STATE = 0,
    /// Expect the beginning of an implicit document.
    YAML_PARSE_IMPLICIT_DOCUMENT_START_STATE = 1,
    /// Expect DOCUMENT-START.
    YAML_PARSE_DOCUMENT_START_STATE = 2,
    /// Expect the content of a document.
    YAML_PARSE_DOCUMENT_CONTENT_STATE = 3,
    /// Expect DOCUMENT-END.
    YAML_PARSE_DOCUMENT_END_STATE = 4,
    /// Expect a block node.
    YAML_PARSE_BLOCK_NODE_STATE = 5,
    /// Expect a block node or indentless sequence.
    YAML_PARSE_BLOCK_NODE_OR_INDENTLESS_SEQUENCE_STATE = 6,
    /// Expect a flow node.
    YAML_PARSE_FLOW_NODE_STATE = 7,
    /// Expect the first entry of a block sequence.
    YAML_PARSE_BLOCK_SEQUENCE_FIRST_ENTRY_STATE = 8,
    /// Expect an entry of a block sequence.
    YAML_PARSE_BLOCK_SEQUENCE_ENTRY_STATE = 9,
    /// Expect an entry of an indentless sequence.
    YAML_PARSE_INDENTLESS_SEQUENCE_ENTRY_STATE = 10,
    /// Expect the first key of a block mapping.
    YAML_PARSE_BLOCK_MAPPING_FIRST_KEY_STATE = 11,
    /// Expect a block mapping key.
    YAML_PARSE_BLOCK_MAPPING_KEY_STATE = 12,
    /// Expect a block mapping value.
    YAML_PARSE_BLOCK_MAPPING_VALUE_STATE = 13,
    /// Expect the first entry of a flow sequence.
    YAML_PARSE_FLOW_SEQUENCE_FIRST_ENTRY_STATE = 14,
    /// Expect an entry of a flow sequence.
    YAML_PARSE_FLOW_SEQUENCE_ENTRY_STATE = 15,
    /// Expect a key of an ordered mapping.
    YAML_PARSE_FLOW_SEQUENCE_ENTRY_MAPPING_KEY_STATE = 16,
    /// Expect a value of an ordered mapping.
    YAML_PARSE_FLOW_SEQUENCE_ENTRY_MAPPING_VALUE_STATE = 17,
    /// Expect the and of an ordered mapping entry.
    YAML_PARSE_FLOW_SEQUENCE_ENTRY_MAPPING_END_STATE = 18,
    /// Expect the first key of a flow mapping.
    YAML_PARSE_FLOW_MAPPING_FIRST_KEY_STATE = 19,
    /// Expect a key of a flow mapping.
    YAML_PARSE_FLOW_MAPPING_KEY_STATE = 20,
    /// Expect a value of a flow mapping.
    YAML_PARSE_FLOW_MAPPING_VALUE_STATE = 21,
    /// Expect an empty value of a flow mapping.
    YAML_PARSE_FLOW_MAPPING_EMPTY_VALUE_STATE = 22,
    /// Expect nothing.
    YAML_PARSE_END_STATE = 23,
}

/// This structure holds aliases data.
#[derive(Copy, Clone)]
#[repr(C)]
#[non_exhaustive]
pub struct yaml_alias_data_t {
    /// The anchor.
    pub anchor: *mut yaml_char_t,
    /// The node id.
    pub index: libc::c_int,
    /// The anchor mark.
    pub mark: yaml_mark_t,
}

/// The parser structure.
///
/// All members are internal. Manage the structure using the `yaml_parser_`
/// family of functions.
#[derive(Copy, Clone)]
#[repr(C)]
#[non_exhaustive]
pub struct yaml_parser_t {
    /// Error type.
    #[cfg(doc)]
    pub error: yaml_error_type_t,
    #[cfg(not(doc))]
    pub(crate) error: yaml_error_type_t,
    /// Error description.
    #[cfg(doc)]
    pub problem: *const libc::c_char,
    #[cfg(not(doc))]
    pub(crate) problem: *const libc::c_char,
    /// The byte about which the problem occurred.
    #[cfg(doc)]
    pub problem_offset: size_t,
    #[cfg(not(doc))]
    pub(crate) problem_offset: size_t,
    /// The problematic value (-1 is none).
    #[cfg(doc)]
    pub problem_value: libc::c_int,
    #[cfg(not(doc))]
    pub(crate) problem_value: libc::c_int,
    /// The problem position.
    #[cfg(doc)]
    pub problem_mark: yaml_mark_t,
    #[cfg(not(doc))]
    pub(crate) problem_mark: yaml_mark_t,
    /// The error context.
    #[cfg(doc)]
    pub context: *const libc::c_char,
    #[cfg(not(doc))]
    pub(crate) context: *const libc::c_char,
    /// The context position.
    #[cfg(doc)]
    pub context_mark: yaml_mark_t,
    #[cfg(not(doc))]
    pub(crate) context_mark: yaml_mark_t,
    /// Read handler.
    pub(crate) read_handler: Option<yaml_read_handler_t>,
    /// A pointer for passing to the read handler.
    pub(crate) read_handler_data: *mut libc::c_void,
    /// Standard (string or file) input data.
    pub(crate) input: unnamed_yaml_parser_t_input,
    /// EOF flag
    pub(crate) eof: bool,
    /// The working buffer.
    pub(crate) buffer: yaml_buffer_t<yaml_char_t>,
    /// The number of unread characters in the buffer.
    pub(crate) unread: size_t,
    /// The raw buffer.
    pub(crate) raw_buffer: yaml_buffer_t<libc::c_uchar>,
    /// The input encoding.
    pub(crate) encoding: yaml_encoding_t,
    /// The offset of the current position (in bytes).
    pub(crate) offset: size_t,
    /// The mark of the current position.
    pub(crate) mark: yaml_mark_t,
    /// Have we started to scan the input stream?
    pub(crate) stream_start_produced: bool,
    /// Have we reached the end of the input stream?
    pub(crate) stream_end_produced: bool,
    /// The number of unclosed '[' and '{' indicators.
    pub(crate) flow_level: libc::c_int,
    /// The tokens queue.
    pub(crate) tokens: yaml_queue_t<yaml_token_t>,
    /// The number of tokens fetched from the queue.
    pub(crate) tokens_parsed: size_t,
    /// Does the tokens queue contain a token ready for dequeueing.
    pub(crate) token_available: bool,
    /// The indentation levels stack.
    pub(crate) indents: yaml_stack_t<libc::c_int>,
    /// The current indentation level.
    pub(crate) indent: libc::c_int,
    /// May a simple key occur at the current position?
    pub(crate) simple_key_allowed: bool,
    /// The stack of simple keys.
    pub(crate) simple_keys: yaml_stack_t<yaml_simple_key_t>,
    /// At least this many leading elements of simple_keys have possible=0.
    pub(crate) not_simple_keys: libc::c_int,
    /// The parser states stack.
    pub(crate) states: yaml_stack_t<yaml_parser_state_t>,
    /// The current parser state.
    pub(crate) state: yaml_parser_state_t,
    /// The stack of marks.
    pub(crate) marks: yaml_stack_t<yaml_mark_t>,
    /// The list of TAG directives.
    pub(crate) tag_directives: yaml_stack_t<yaml_tag_directive_t>,
    /// The alias data.
    pub(crate) aliases: yaml_stack_t<yaml_alias_data_t>,
    /// The currently parsed document.
    pub(crate) document: *mut yaml_document_t,
}

#[repr(C)]
#[non_exhaustive]
pub struct yaml_parser_t_prefix {
    /// Error type.
    pub error: yaml_error_type_t,
    /// Error description.
    pub problem: *const libc::c_char,
    /// The byte about which the problem occurred.
    pub problem_offset: size_t,
    /// The problematic value (-1 is none).
    pub problem_value: libc::c_int,
    /// The problem position.
    pub problem_mark: yaml_mark_t,
    /// The error context.
    pub context: *const libc::c_char,
    /// The context position.
    pub context_mark: yaml_mark_t,
}

#[doc(hidden)]
impl Deref for yaml_parser_t {
    type Target = yaml_parser_t_prefix;
    fn deref(&self) -> &Self::Target {
        unsafe { &*addr_of!(*self).cast() }
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub(crate) union unnamed_yaml_parser_t_input {
    /// String input data.
    pub string: unnamed_yaml_parser_t_input_string,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub(crate) struct unnamed_yaml_parser_t_input_string {
    /// The string start pointer.
    pub start: *const libc::c_uchar,
    /// The string end pointer.
    pub end: *const libc::c_uchar,
    /// The string current position.
    pub current: *const libc::c_uchar,
}

/// The prototype of a write handler.
///
/// The write handler is called when the emitter needs to flush the accumulated
/// characters to the output. The handler should write `size` bytes of the
/// `buffer` to the output.
///
/// On success, the handler should return 1. If the handler failed, the returned
/// value should be 0.
pub type yaml_write_handler_t =
    unsafe fn(data: *mut libc::c_void, buffer: *mut libc::c_uchar, size: size_t) -> libc::c_int;

/// The emitter states.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
#[repr(u32)]
#[non_exhaustive]
pub enum yaml_emitter_state_t {
    /// Expect STREAM-START.
    YAML_EMIT_STREAM_START_STATE = 0,
    /// Expect the first DOCUMENT-START or STREAM-END.
    YAML_EMIT_FIRST_DOCUMENT_START_STATE = 1,
    /// Expect DOCUMENT-START or STREAM-END.
    YAML_EMIT_DOCUMENT_START_STATE = 2,
    /// Expect the content of a document.
    YAML_EMIT_DOCUMENT_CONTENT_STATE = 3,
    /// Expect DOCUMENT-END.
    YAML_EMIT_DOCUMENT_END_STATE = 4,
    /// Expect the first item of a flow sequence.
    YAML_EMIT_FLOW_SEQUENCE_FIRST_ITEM_STATE = 5,
    /// Expect an item of a flow sequence.
    YAML_EMIT_FLOW_SEQUENCE_ITEM_STATE = 6,
    /// Expect the first key of a flow mapping.
    YAML_EMIT_FLOW_MAPPING_FIRST_KEY_STATE = 7,
    /// Expect a key of a flow mapping.
    YAML_EMIT_FLOW_MAPPING_KEY_STATE = 8,
    /// Expect a value for a simple key of a flow mapping.
    YAML_EMIT_FLOW_MAPPING_SIMPLE_VALUE_STATE = 9,
    /// Expect a value of a flow mapping.
    YAML_EMIT_FLOW_MAPPING_VALUE_STATE = 10,
    /// Expect the first item of a block sequence.
    YAML_EMIT_BLOCK_SEQUENCE_FIRST_ITEM_STATE = 11,
    /// Expect an item of a block sequence.
    YAML_EMIT_BLOCK_SEQUENCE_ITEM_STATE = 12,
    /// Expect the first key of a block mapping.
    YAML_EMIT_BLOCK_MAPPING_FIRST_KEY_STATE = 13,
    /// Expect the key of a block mapping.
    YAML_EMIT_BLOCK_MAPPING_KEY_STATE = 14,
    /// Expect a value for a simple key of a block mapping.
    YAML_EMIT_BLOCK_MAPPING_SIMPLE_VALUE_STATE = 15,
    /// Expect a value of a block mapping.
    YAML_EMIT_BLOCK_MAPPING_VALUE_STATE = 16,
    /// Expect nothing.
    YAML_EMIT_END_STATE = 17,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub(crate) struct yaml_anchors_t {
    /// The number of references.
    pub references: libc::c_int,
    /// The anchor id.
    pub anchor: libc::c_int,
    /// If the node has been emitted?
    pub serialized: bool,
}

/// The emitter structure.
///
/// All members are internal. Manage the structure using the `yaml_emitter_`
/// family of functions.
#[derive(Copy, Clone)]
#[repr(C)]
#[non_exhaustive]
pub struct yaml_emitter_t {
    /// Error type.
    #[cfg(doc)]
    pub error: yaml_error_type_t,
    #[cfg(not(doc))]
    pub(crate) error: yaml_error_type_t,
    /// Error description.
    #[cfg(doc)]
    pub problem: *const libc::c_char,
    #[cfg(not(doc))]
    pub(crate) problem: *const libc::c_char,
    /// Write handler.
    pub(crate) write_handler: Option<yaml_write_handler_t>,
    /// A pointer for passing to the write handler.
    pub(crate) write_handler_data: *mut libc::c_void,
    /// Standard (string or file) output data.
    pub(crate) output: unnamed_yaml_emitter_t_output,
    /// The working buffer.
    pub(crate) buffer: yaml_buffer_t<yaml_char_t>,
    /// The raw buffer.
    pub(crate) raw_buffer: yaml_buffer_t<libc::c_uchar>,
    /// The stream encoding.
    pub(crate) encoding: yaml_encoding_t,
    /// If the output is in the canonical style?
    pub(crate) canonical: bool,
    /// The number of indentation spaces.
    pub(crate) best_indent: libc::c_int,
    /// The preferred width of the output lines.
    pub(crate) best_width: libc::c_int,
    /// Allow unescaped non-ASCII characters?
    pub(crate) unicode: bool,
    /// The preferred line break.
    pub(crate) line_break: yaml_break_t,
    /// The stack of states.
    pub(crate) states: yaml_stack_t<yaml_emitter_state_t>,
    /// The current emitter state.
    pub(crate) state: yaml_emitter_state_t,
    /// The event queue.
    pub(crate) events: yaml_queue_t<yaml_event_t>,
    /// The stack of indentation levels.
    pub(crate) indents: yaml_stack_t<libc::c_int>,
    /// The list of tag directives.
    pub(crate) tag_directives: yaml_stack_t<yaml_tag_directive_t>,
    /// The current indentation level.
    pub(crate) indent: libc::c_int,
    /// The current flow level.
    pub(crate) flow_level: libc::c_int,
    /// Is it the document root context?
    pub(crate) root_context: bool,
    /// Is it a sequence context?
    pub(crate) sequence_context: bool,
    /// Is it a mapping context?
    pub(crate) mapping_context: bool,
    /// Is it a simple mapping key context?
    pub(crate) simple_key_context: bool,
    /// The current line.
    pub(crate) line: libc::c_int,
    /// The current column.
    pub(crate) column: libc::c_int,
    /// If the last character was a whitespace?
    pub(crate) whitespace: bool,
    /// If the last character was an indentation character (' ', '-', '?', ':')?
    pub(crate) indention: bool,
    /// If an explicit document end is required?
    pub(crate) open_ended: libc::c_int,
    /// Anchor analysis.
    pub(crate) anchor_data: unnamed_yaml_emitter_t_anchor_data,
    /// Tag analysis.
    pub(crate) tag_data: unnamed_yaml_emitter_t_tag_data,
    /// Scalar analysis.
    pub(crate) scalar_data: unnamed_yaml_emitter_t_scalar_data,
    /// If the stream was already opened?
    pub(crate) opened: bool,
    /// If the stream was already closed?
    pub(crate) closed: bool,
    /// The information associated with the document nodes.
    pub(crate) anchors: *mut yaml_anchors_t,
    /// The last assigned anchor id.
    pub(crate) last_anchor_id: libc::c_int,
    /// The currently emitted document.
    pub(crate) document: *mut yaml_document_t,
}

#[repr(C)]
#[non_exhaustive]
pub struct yaml_emitter_t_prefix {
    /// Error type.
    pub error: yaml_error_type_t,
    /// Error description.
    pub problem: *const libc::c_char,
}

#[doc(hidden)]
impl Deref for yaml_emitter_t {
    type Target = yaml_emitter_t_prefix;
    fn deref(&self) -> &Self::Target {
        unsafe { &*addr_of!(*self).cast() }
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub(crate) union unnamed_yaml_emitter_t_output {
    /// String output data.
    pub string: unnamed_yaml_emitter_t_output_string,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub(crate) struct unnamed_yaml_emitter_t_output_string {
    /// The buffer pointer.
    pub buffer: *mut libc::c_uchar,
    /// The buffer size.
    pub size: size_t,
    /// The number of written bytes.
    pub size_written: *mut size_t,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub(crate) struct unnamed_yaml_emitter_t_anchor_data {
    /// The anchor value.
    pub anchor: *mut yaml_char_t,
    /// The anchor length.
    pub anchor_length: size_t,
    /// Is it an alias?
    pub alias: bool,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub(crate) struct unnamed_yaml_emitter_t_tag_data {
    /// The tag handle.
    pub handle: *mut yaml_char_t,
    /// The tag handle length.
    pub handle_length: size_t,
    /// The tag suffix.
    pub suffix: *mut yaml_char_t,
    /// The tag suffix length.
    pub suffix_length: size_t,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub(crate) struct unnamed_yaml_emitter_t_scalar_data {
    /// The scalar value.
    pub value: *mut yaml_char_t,
    /// The scalar length.
    pub length: size_t,
    /// Does the scalar contain line breaks?
    pub multiline: bool,
    /// Can the scalar be expressed in the flow plain style?
    pub flow_plain_allowed: bool,
    /// Can the scalar be expressed in the block plain style?
    pub block_plain_allowed: bool,
    /// Can the scalar be expressed in the single quoted style?
    pub single_quoted_allowed: bool,
    /// Can the scalar be expressed in the literal or folded styles?
    pub block_allowed: bool,
    /// The output style.
    pub style: yaml_scalar_style_t,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub(crate) struct yaml_string_t {
    pub start: *mut yaml_char_t,
    pub end: *mut yaml_char_t,
    pub pointer: *mut yaml_char_t,
}

pub(crate) const NULL_STRING: yaml_string_t = yaml_string_t {
    start: ptr::null_mut::<yaml_char_t>(),
    end: ptr::null_mut::<yaml_char_t>(),
    pointer: ptr::null_mut::<yaml_char_t>(),
};

#[repr(C)]
pub(crate) struct yaml_buffer_t<T> {
    /// The beginning of the buffer.
    pub start: *mut T,
    /// The end of the buffer.
    pub end: *mut T,
    /// The current position of the buffer.
    pub pointer: *mut T,
    /// The last filled position of the buffer.
    pub last: *mut T,
}

impl<T> Copy for yaml_buffer_t<T> {}
impl<T> Clone for yaml_buffer_t<T> {
    fn clone(&self) -> Self {
        *self
    }
}

#[repr(C)]
pub struct yaml_stack_t<T> {
    /// The beginning of the stack.
    pub start: *mut T,
    /// The end of the stack.
    pub end: *mut T,
    /// The top of the stack.
    pub top: *mut T,
}

impl<T> Copy for yaml_stack_t<T> {}
impl<T> Clone for yaml_stack_t<T> {
    fn clone(&self) -> Self {
        *self
    }
}

#[repr(C)]
pub(crate) struct yaml_queue_t<T> {
    /// The beginning of the queue.
    pub start: *mut T,
    /// The end of the queue.
    pub end: *mut T,
    /// The head of the queue.
    pub head: *mut T,
    /// The tail of the queue.
    pub tail: *mut T,
}

impl<T> Copy for yaml_queue_t<T> {}
impl<T> Clone for yaml_queue_t<T> {
    fn clone(&self) -> Self {
        *self
    }
}
