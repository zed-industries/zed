use crate::externs::{free, malloc, memcpy, memmove, memset, realloc, strdup, strlen};
use crate::ops::{ForceAdd as _, ForceMul as _};
use crate::success::{Success, FAIL, OK};
use crate::yaml::{size_t, yaml_char_t};
use crate::{
    libc, yaml_break_t, yaml_document_t, yaml_emitter_state_t, yaml_emitter_t, yaml_encoding_t,
    yaml_event_t, yaml_mapping_style_t, yaml_mark_t, yaml_node_item_t, yaml_node_pair_t,
    yaml_node_t, yaml_parser_state_t, yaml_parser_t, yaml_read_handler_t, yaml_scalar_style_t,
    yaml_sequence_style_t, yaml_simple_key_t, yaml_tag_directive_t, yaml_token_t,
    yaml_version_directive_t, yaml_write_handler_t, PointerExt, YAML_ALIAS_EVENT, YAML_ALIAS_TOKEN,
    YAML_ANCHOR_TOKEN, YAML_ANY_ENCODING, YAML_DOCUMENT_END_EVENT, YAML_DOCUMENT_START_EVENT,
    YAML_MAPPING_END_EVENT, YAML_MAPPING_NODE, YAML_MAPPING_START_EVENT, YAML_SCALAR_EVENT,
    YAML_SCALAR_NODE, YAML_SCALAR_TOKEN, YAML_SEQUENCE_END_EVENT, YAML_SEQUENCE_NODE,
    YAML_SEQUENCE_START_EVENT, YAML_STREAM_END_EVENT, YAML_STREAM_START_EVENT,
    YAML_TAG_DIRECTIVE_TOKEN, YAML_TAG_TOKEN,
};
use core::mem::{size_of, MaybeUninit};
use core::ptr::{self, addr_of_mut};

const INPUT_RAW_BUFFER_SIZE: usize = 16384;
const INPUT_BUFFER_SIZE: usize = INPUT_RAW_BUFFER_SIZE * 3;
const OUTPUT_BUFFER_SIZE: usize = 16384;
const OUTPUT_RAW_BUFFER_SIZE: usize = OUTPUT_BUFFER_SIZE * 2 + 2;

pub(crate) unsafe fn yaml_malloc(size: size_t) -> *mut libc::c_void {
    malloc(size)
}

pub(crate) unsafe fn yaml_realloc(ptr: *mut libc::c_void, size: size_t) -> *mut libc::c_void {
    if !ptr.is_null() {
        realloc(ptr, size)
    } else {
        malloc(size)
    }
}

pub(crate) unsafe fn yaml_free(ptr: *mut libc::c_void) {
    if !ptr.is_null() {
        free(ptr);
    }
}

pub(crate) unsafe fn yaml_strdup(str: *const yaml_char_t) -> *mut yaml_char_t {
    if str.is_null() {
        return ptr::null_mut::<yaml_char_t>();
    }
    strdup(str as *mut libc::c_char) as *mut yaml_char_t
}

pub(crate) unsafe fn yaml_string_extend(
    start: *mut *mut yaml_char_t,
    pointer: *mut *mut yaml_char_t,
    end: *mut *mut yaml_char_t,
) {
    let new_start: *mut yaml_char_t = yaml_realloc(
        *start as *mut libc::c_void,
        (((*end).c_offset_from(*start) as libc::c_long).force_mul(2_i64)) as size_t,
    ) as *mut yaml_char_t;
    memset(
        new_start.wrapping_offset((*end).c_offset_from(*start) as libc::c_long as isize)
            as *mut libc::c_void,
        0,
        (*end).c_offset_from(*start) as libc::c_ulong,
    );
    *pointer = new_start.wrapping_offset((*pointer).c_offset_from(*start) as libc::c_long as isize);
    *end = new_start.wrapping_offset(
        (((*end).c_offset_from(*start) as libc::c_long).force_mul(2_i64)) as isize,
    );
    *start = new_start;
}

pub(crate) unsafe fn yaml_string_join(
    a_start: *mut *mut yaml_char_t,
    a_pointer: *mut *mut yaml_char_t,
    a_end: *mut *mut yaml_char_t,
    b_start: *mut *mut yaml_char_t,
    b_pointer: *mut *mut yaml_char_t,
    _b_end: *mut *mut yaml_char_t,
) {
    if *b_start == *b_pointer {
        return;
    }
    while (*a_end).c_offset_from(*a_pointer) as libc::c_long
        <= (*b_pointer).c_offset_from(*b_start) as libc::c_long
    {
        yaml_string_extend(a_start, a_pointer, a_end);
    }
    memcpy(
        *a_pointer as *mut libc::c_void,
        *b_start as *const libc::c_void,
        (*b_pointer).c_offset_from(*b_start) as libc::c_ulong,
    );
    *a_pointer =
        (*a_pointer).wrapping_offset((*b_pointer).c_offset_from(*b_start) as libc::c_long as isize);
}

pub(crate) unsafe fn yaml_stack_extend(
    start: *mut *mut libc::c_void,
    top: *mut *mut libc::c_void,
    end: *mut *mut libc::c_void,
) {
    let new_start: *mut libc::c_void = yaml_realloc(
        *start,
        (((*end as *mut libc::c_char).c_offset_from(*start as *mut libc::c_char) as libc::c_long)
            .force_mul(2_i64)) as size_t,
    );
    *top = (new_start as *mut libc::c_char).wrapping_offset(
        (*top as *mut libc::c_char).c_offset_from(*start as *mut libc::c_char) as libc::c_long
            as isize,
    ) as *mut libc::c_void;
    *end = (new_start as *mut libc::c_char).wrapping_offset(
        (((*end as *mut libc::c_char).c_offset_from(*start as *mut libc::c_char) as libc::c_long)
            .force_mul(2_i64)) as isize,
    ) as *mut libc::c_void;
    *start = new_start;
}

pub(crate) unsafe fn yaml_queue_extend(
    start: *mut *mut libc::c_void,
    head: *mut *mut libc::c_void,
    tail: *mut *mut libc::c_void,
    end: *mut *mut libc::c_void,
) {
    if *start == *head && *tail == *end {
        let new_start: *mut libc::c_void = yaml_realloc(
            *start,
            (((*end as *mut libc::c_char).c_offset_from(*start as *mut libc::c_char)
                as libc::c_long)
                .force_mul(2_i64)) as size_t,
        );
        *head = (new_start as *mut libc::c_char).wrapping_offset(
            (*head as *mut libc::c_char).c_offset_from(*start as *mut libc::c_char) as libc::c_long
                as isize,
        ) as *mut libc::c_void;
        *tail = (new_start as *mut libc::c_char).wrapping_offset(
            (*tail as *mut libc::c_char).c_offset_from(*start as *mut libc::c_char) as libc::c_long
                as isize,
        ) as *mut libc::c_void;
        *end = (new_start as *mut libc::c_char).wrapping_offset(
            (((*end as *mut libc::c_char).c_offset_from(*start as *mut libc::c_char)
                as libc::c_long)
                .force_mul(2_i64)) as isize,
        ) as *mut libc::c_void;
        *start = new_start;
    }
    if *tail == *end {
        if *head != *tail {
            memmove(
                *start,
                *head,
                (*tail as *mut libc::c_char).c_offset_from(*head as *mut libc::c_char)
                    as libc::c_ulong,
            );
        }
        *tail = (*start as *mut libc::c_char).wrapping_offset(
            (*tail as *mut libc::c_char).c_offset_from(*head as *mut libc::c_char) as libc::c_long
                as isize,
        ) as *mut libc::c_void;
        *head = *start;
    }
}

/// Initialize a parser.
///
/// This function creates a new parser object. An application is responsible
/// for destroying the object using the yaml_parser_delete() function.
pub unsafe fn yaml_parser_initialize(parser: *mut yaml_parser_t) -> Success {
    __assert!(!parser.is_null());
    memset(
        parser as *mut libc::c_void,
        0,
        size_of::<yaml_parser_t>() as libc::c_ulong,
    );
    BUFFER_INIT!((*parser).raw_buffer, INPUT_RAW_BUFFER_SIZE);
    BUFFER_INIT!((*parser).buffer, INPUT_BUFFER_SIZE);
    QUEUE_INIT!((*parser).tokens, yaml_token_t);
    STACK_INIT!((*parser).indents, libc::c_int);
    STACK_INIT!((*parser).simple_keys, yaml_simple_key_t);
    STACK_INIT!((*parser).states, yaml_parser_state_t);
    STACK_INIT!((*parser).marks, yaml_mark_t);
    STACK_INIT!((*parser).tag_directives, yaml_tag_directive_t);
    OK
}

/// Destroy a parser.
pub unsafe fn yaml_parser_delete(parser: *mut yaml_parser_t) {
    __assert!(!parser.is_null());
    BUFFER_DEL!((*parser).raw_buffer);
    BUFFER_DEL!((*parser).buffer);
    while !QUEUE_EMPTY!((*parser).tokens) {
        yaml_token_delete(addr_of_mut!(DEQUEUE!((*parser).tokens)));
    }
    QUEUE_DEL!((*parser).tokens);
    STACK_DEL!((*parser).indents);
    STACK_DEL!((*parser).simple_keys);
    STACK_DEL!((*parser).states);
    STACK_DEL!((*parser).marks);
    while !STACK_EMPTY!((*parser).tag_directives) {
        let tag_directive = POP!((*parser).tag_directives);
        yaml_free(tag_directive.handle as *mut libc::c_void);
        yaml_free(tag_directive.prefix as *mut libc::c_void);
    }
    STACK_DEL!((*parser).tag_directives);
    memset(
        parser as *mut libc::c_void,
        0,
        size_of::<yaml_parser_t>() as libc::c_ulong,
    );
}

unsafe fn yaml_string_read_handler(
    data: *mut libc::c_void,
    buffer: *mut libc::c_uchar,
    mut size: size_t,
    size_read: *mut size_t,
) -> libc::c_int {
    let parser: *mut yaml_parser_t = data as *mut yaml_parser_t;
    if (*parser).input.string.current == (*parser).input.string.end {
        *size_read = 0_u64;
        return 1;
    }
    if size
        > (*parser)
            .input
            .string
            .end
            .c_offset_from((*parser).input.string.current) as size_t
    {
        size = (*parser)
            .input
            .string
            .end
            .c_offset_from((*parser).input.string.current) as size_t;
    }
    memcpy(
        buffer as *mut libc::c_void,
        (*parser).input.string.current as *const libc::c_void,
        size,
    );
    let fresh80 = addr_of_mut!((*parser).input.string.current);
    *fresh80 = (*fresh80).wrapping_offset(size as isize);
    *size_read = size;
    1
}

/// Set a string input.
///
/// Note that the `input` pointer must be valid while the `parser` object
/// exists. The application is responsible for destroying `input` after
/// destroying the `parser`.
pub unsafe fn yaml_parser_set_input_string(
    parser: *mut yaml_parser_t,
    input: *const libc::c_uchar,
    size: size_t,
) {
    __assert!(!parser.is_null());
    __assert!(((*parser).read_handler).is_none());
    __assert!(!input.is_null());
    let fresh81 = addr_of_mut!((*parser).read_handler);
    *fresh81 = Some(
        yaml_string_read_handler
            as unsafe fn(*mut libc::c_void, *mut libc::c_uchar, size_t, *mut size_t) -> libc::c_int,
    );
    let fresh82 = addr_of_mut!((*parser).read_handler_data);
    *fresh82 = parser as *mut libc::c_void;
    let fresh83 = addr_of_mut!((*parser).input.string.start);
    *fresh83 = input;
    let fresh84 = addr_of_mut!((*parser).input.string.current);
    *fresh84 = input;
    let fresh85 = addr_of_mut!((*parser).input.string.end);
    *fresh85 = input.wrapping_offset(size as isize);
}

/// Set a generic input handler.
pub unsafe fn yaml_parser_set_input(
    parser: *mut yaml_parser_t,
    handler: yaml_read_handler_t,
    data: *mut libc::c_void,
) {
    __assert!(!parser.is_null());
    __assert!(((*parser).read_handler).is_none());
    let fresh89 = addr_of_mut!((*parser).read_handler);
    *fresh89 = Some(handler);
    let fresh90 = addr_of_mut!((*parser).read_handler_data);
    *fresh90 = data;
}

/// Set the source encoding.
pub unsafe fn yaml_parser_set_encoding(parser: *mut yaml_parser_t, encoding: yaml_encoding_t) {
    __assert!(!parser.is_null());
    __assert!((*parser).encoding == YAML_ANY_ENCODING);
    (*parser).encoding = encoding;
}

/// Initialize an emitter.
///
/// This function creates a new emitter object. An application is responsible
/// for destroying the object using the yaml_emitter_delete() function.
pub unsafe fn yaml_emitter_initialize(emitter: *mut yaml_emitter_t) -> Success {
    __assert!(!emitter.is_null());
    memset(
        emitter as *mut libc::c_void,
        0,
        size_of::<yaml_emitter_t>() as libc::c_ulong,
    );
    BUFFER_INIT!((*emitter).buffer, OUTPUT_BUFFER_SIZE);
    BUFFER_INIT!((*emitter).raw_buffer, OUTPUT_RAW_BUFFER_SIZE);
    STACK_INIT!((*emitter).states, yaml_emitter_state_t);
    QUEUE_INIT!((*emitter).events, yaml_event_t);
    STACK_INIT!((*emitter).indents, libc::c_int);
    STACK_INIT!((*emitter).tag_directives, yaml_tag_directive_t);
    OK
}

/// Destroy an emitter.
pub unsafe fn yaml_emitter_delete(emitter: *mut yaml_emitter_t) {
    __assert!(!emitter.is_null());
    BUFFER_DEL!((*emitter).buffer);
    BUFFER_DEL!((*emitter).raw_buffer);
    STACK_DEL!((*emitter).states);
    while !QUEUE_EMPTY!((*emitter).events) {
        yaml_event_delete(addr_of_mut!(DEQUEUE!((*emitter).events)));
    }
    QUEUE_DEL!((*emitter).events);
    STACK_DEL!((*emitter).indents);
    while !STACK_EMPTY!((*emitter).tag_directives) {
        let tag_directive = POP!((*emitter).tag_directives);
        yaml_free(tag_directive.handle as *mut libc::c_void);
        yaml_free(tag_directive.prefix as *mut libc::c_void);
    }
    STACK_DEL!((*emitter).tag_directives);
    yaml_free((*emitter).anchors as *mut libc::c_void);
    memset(
        emitter as *mut libc::c_void,
        0,
        size_of::<yaml_emitter_t>() as libc::c_ulong,
    );
}

unsafe fn yaml_string_write_handler(
    data: *mut libc::c_void,
    buffer: *mut libc::c_uchar,
    size: size_t,
) -> libc::c_int {
    let emitter: *mut yaml_emitter_t = data as *mut yaml_emitter_t;
    if (*emitter)
        .output
        .string
        .size
        .wrapping_sub(*(*emitter).output.string.size_written)
        < size
    {
        memcpy(
            (*emitter)
                .output
                .string
                .buffer
                .wrapping_offset(*(*emitter).output.string.size_written as isize)
                as *mut libc::c_void,
            buffer as *const libc::c_void,
            (*emitter)
                .output
                .string
                .size
                .wrapping_sub(*(*emitter).output.string.size_written),
        );
        *(*emitter).output.string.size_written = (*emitter).output.string.size;
        return 0;
    }
    memcpy(
        (*emitter)
            .output
            .string
            .buffer
            .wrapping_offset(*(*emitter).output.string.size_written as isize)
            as *mut libc::c_void,
        buffer as *const libc::c_void,
        size,
    );
    let fresh153 = addr_of_mut!((*(*emitter).output.string.size_written));
    *fresh153 = (*fresh153 as libc::c_ulong).force_add(size) as size_t;
    1
}

/// Set a string output.
///
/// The emitter will write the output characters to the `output` buffer of the
/// size `size`. The emitter will set `size_written` to the number of written
/// bytes. If the buffer is smaller than required, the emitter produces the
/// YAML_WRITE_ERROR error.
pub unsafe fn yaml_emitter_set_output_string(
    emitter: *mut yaml_emitter_t,
    output: *mut libc::c_uchar,
    size: size_t,
    size_written: *mut size_t,
) {
    __assert!(!emitter.is_null());
    __assert!(((*emitter).write_handler).is_none());
    __assert!(!output.is_null());
    let fresh154 = addr_of_mut!((*emitter).write_handler);
    *fresh154 = Some(
        yaml_string_write_handler
            as unsafe fn(*mut libc::c_void, *mut libc::c_uchar, size_t) -> libc::c_int,
    );
    let fresh155 = addr_of_mut!((*emitter).write_handler_data);
    *fresh155 = emitter as *mut libc::c_void;
    let fresh156 = addr_of_mut!((*emitter).output.string.buffer);
    *fresh156 = output;
    (*emitter).output.string.size = size;
    let fresh157 = addr_of_mut!((*emitter).output.string.size_written);
    *fresh157 = size_written;
    *size_written = 0_u64;
}

/// Set a generic output handler.
pub unsafe fn yaml_emitter_set_output(
    emitter: *mut yaml_emitter_t,
    handler: yaml_write_handler_t,
    data: *mut libc::c_void,
) {
    __assert!(!emitter.is_null());
    __assert!(((*emitter).write_handler).is_none());
    let fresh161 = addr_of_mut!((*emitter).write_handler);
    *fresh161 = Some(handler);
    let fresh162 = addr_of_mut!((*emitter).write_handler_data);
    *fresh162 = data;
}

/// Set the output encoding.
pub unsafe fn yaml_emitter_set_encoding(emitter: *mut yaml_emitter_t, encoding: yaml_encoding_t) {
    __assert!(!emitter.is_null());
    __assert!((*emitter).encoding == YAML_ANY_ENCODING);
    (*emitter).encoding = encoding;
}

/// Set if the output should be in the "canonical" format as in the YAML
/// specification.
pub unsafe fn yaml_emitter_set_canonical(emitter: *mut yaml_emitter_t, canonical: bool) {
    __assert!(!emitter.is_null());
    (*emitter).canonical = canonical;
}

/// Set the indentation increment.
pub unsafe fn yaml_emitter_set_indent(emitter: *mut yaml_emitter_t, indent: libc::c_int) {
    __assert!(!emitter.is_null());
    (*emitter).best_indent = if 1 < indent && indent < 10 { indent } else { 2 };
}

/// Set the preferred line width. -1 means unlimited.
pub unsafe fn yaml_emitter_set_width(emitter: *mut yaml_emitter_t, width: libc::c_int) {
    __assert!(!emitter.is_null());
    (*emitter).best_width = if width >= 0 { width } else { -1 };
}

/// Set if unescaped non-ASCII characters are allowed.
pub unsafe fn yaml_emitter_set_unicode(emitter: *mut yaml_emitter_t, unicode: bool) {
    __assert!(!emitter.is_null());
    (*emitter).unicode = unicode;
}

/// Set the preferred line break.
pub unsafe fn yaml_emitter_set_break(emitter: *mut yaml_emitter_t, line_break: yaml_break_t) {
    __assert!(!emitter.is_null());
    (*emitter).line_break = line_break;
}

/// Free any memory allocated for a token object.
pub unsafe fn yaml_token_delete(token: *mut yaml_token_t) {
    __assert!(!token.is_null());
    match (*token).type_ {
        YAML_TAG_DIRECTIVE_TOKEN => {
            yaml_free((*token).data.tag_directive.handle as *mut libc::c_void);
            yaml_free((*token).data.tag_directive.prefix as *mut libc::c_void);
        }
        YAML_ALIAS_TOKEN => {
            yaml_free((*token).data.alias.value as *mut libc::c_void);
        }
        YAML_ANCHOR_TOKEN => {
            yaml_free((*token).data.anchor.value as *mut libc::c_void);
        }
        YAML_TAG_TOKEN => {
            yaml_free((*token).data.tag.handle as *mut libc::c_void);
            yaml_free((*token).data.tag.suffix as *mut libc::c_void);
        }
        YAML_SCALAR_TOKEN => {
            yaml_free((*token).data.scalar.value as *mut libc::c_void);
        }
        _ => {}
    }
    memset(
        token as *mut libc::c_void,
        0,
        size_of::<yaml_token_t>() as libc::c_ulong,
    );
}

unsafe fn yaml_check_utf8(start: *const yaml_char_t, length: size_t) -> Success {
    let end: *const yaml_char_t = start.wrapping_offset(length as isize);
    let mut pointer: *const yaml_char_t = start;
    while pointer < end {
        let mut octet: libc::c_uchar;
        let mut value: libc::c_uint;
        let mut k: size_t;
        octet = *pointer;
        let width: libc::c_uint = if octet & 0x80 == 0 {
            1
        } else if octet & 0xE0 == 0xC0 {
            2
        } else if octet & 0xF0 == 0xE0 {
            3
        } else if octet & 0xF8 == 0xF0 {
            4
        } else {
            0
        } as libc::c_uint;
        value = if octet & 0x80 == 0 {
            octet & 0x7F
        } else if octet & 0xE0 == 0xC0 {
            octet & 0x1F
        } else if octet & 0xF0 == 0xE0 {
            octet & 0xF
        } else if octet & 0xF8 == 0xF0 {
            octet & 0x7
        } else {
            0
        } as libc::c_uint;
        if width == 0 {
            return FAIL;
        }
        if pointer.wrapping_offset(width as isize) > end {
            return FAIL;
        }
        k = 1_u64;
        while k < width as libc::c_ulong {
            octet = *pointer.wrapping_offset(k as isize);
            if octet & 0xC0 != 0x80 {
                return FAIL;
            }
            value = (value << 6).force_add((octet & 0x3F) as libc::c_uint);
            k = k.force_add(1);
        }
        if !(width == 1
            || width == 2 && value >= 0x80
            || width == 3 && value >= 0x800
            || width == 4 && value >= 0x10000)
        {
            return FAIL;
        }
        pointer = pointer.wrapping_offset(width as isize);
    }
    OK
}

/// Create the STREAM-START event.
pub unsafe fn yaml_stream_start_event_initialize(
    event: *mut yaml_event_t,
    encoding: yaml_encoding_t,
) -> Success {
    let mark = yaml_mark_t {
        index: 0_u64,
        line: 0_u64,
        column: 0_u64,
    };
    __assert!(!event.is_null());
    memset(
        event as *mut libc::c_void,
        0,
        size_of::<yaml_event_t>() as libc::c_ulong,
    );
    (*event).type_ = YAML_STREAM_START_EVENT;
    (*event).start_mark = mark;
    (*event).end_mark = mark;
    (*event).data.stream_start.encoding = encoding;
    OK
}

/// Create the STREAM-END event.
pub unsafe fn yaml_stream_end_event_initialize(event: *mut yaml_event_t) -> Success {
    let mark = yaml_mark_t {
        index: 0_u64,
        line: 0_u64,
        column: 0_u64,
    };
    __assert!(!event.is_null());
    memset(
        event as *mut libc::c_void,
        0,
        size_of::<yaml_event_t>() as libc::c_ulong,
    );
    (*event).type_ = YAML_STREAM_END_EVENT;
    (*event).start_mark = mark;
    (*event).end_mark = mark;
    OK
}

/// Create the DOCUMENT-START event.
///
/// The `implicit` argument is considered as a stylistic parameter and may be
/// ignored by the emitter.
pub unsafe fn yaml_document_start_event_initialize(
    event: *mut yaml_event_t,
    version_directive: *mut yaml_version_directive_t,
    tag_directives_start: *mut yaml_tag_directive_t,
    tag_directives_end: *mut yaml_tag_directive_t,
    implicit: bool,
) -> Success {
    let current_block: u64;
    let mark = yaml_mark_t {
        index: 0_u64,
        line: 0_u64,
        column: 0_u64,
    };
    let mut version_directive_copy: *mut yaml_version_directive_t =
        ptr::null_mut::<yaml_version_directive_t>();
    struct TagDirectivesCopy {
        start: *mut yaml_tag_directive_t,
        end: *mut yaml_tag_directive_t,
        top: *mut yaml_tag_directive_t,
    }
    let mut tag_directives_copy = TagDirectivesCopy {
        start: ptr::null_mut::<yaml_tag_directive_t>(),
        end: ptr::null_mut::<yaml_tag_directive_t>(),
        top: ptr::null_mut::<yaml_tag_directive_t>(),
    };
    let mut value = yaml_tag_directive_t {
        handle: ptr::null_mut::<yaml_char_t>(),
        prefix: ptr::null_mut::<yaml_char_t>(),
    };
    __assert!(!event.is_null());
    __assert!(
        !tag_directives_start.is_null() && !tag_directives_end.is_null()
            || tag_directives_start == tag_directives_end
    );
    if !version_directive.is_null() {
        version_directive_copy = yaml_malloc(size_of::<yaml_version_directive_t>() as libc::c_ulong)
            as *mut yaml_version_directive_t;
        (*version_directive_copy).major = (*version_directive).major;
        (*version_directive_copy).minor = (*version_directive).minor;
    }
    if tag_directives_start != tag_directives_end {
        let mut tag_directive: *mut yaml_tag_directive_t;
        STACK_INIT!(tag_directives_copy, yaml_tag_directive_t);
        tag_directive = tag_directives_start;
        loop {
            if !(tag_directive != tag_directives_end) {
                current_block = 16203760046146113240;
                break;
            }
            __assert!(!((*tag_directive).handle).is_null());
            __assert!(!((*tag_directive).prefix).is_null());
            if yaml_check_utf8(
                (*tag_directive).handle,
                strlen((*tag_directive).handle as *mut libc::c_char),
            )
            .fail
            {
                current_block = 14964981520188694172;
                break;
            }
            if yaml_check_utf8(
                (*tag_directive).prefix,
                strlen((*tag_directive).prefix as *mut libc::c_char),
            )
            .fail
            {
                current_block = 14964981520188694172;
                break;
            }
            value.handle = yaml_strdup((*tag_directive).handle);
            value.prefix = yaml_strdup((*tag_directive).prefix);
            if value.handle.is_null() || value.prefix.is_null() {
                current_block = 14964981520188694172;
                break;
            }
            PUSH!(tag_directives_copy, value);
            value.handle = ptr::null_mut::<yaml_char_t>();
            value.prefix = ptr::null_mut::<yaml_char_t>();
            tag_directive = tag_directive.wrapping_offset(1);
        }
    } else {
        current_block = 16203760046146113240;
    }
    if current_block != 14964981520188694172 {
        memset(
            event as *mut libc::c_void,
            0,
            size_of::<yaml_event_t>() as libc::c_ulong,
        );
        (*event).type_ = YAML_DOCUMENT_START_EVENT;
        (*event).start_mark = mark;
        (*event).end_mark = mark;
        let fresh164 = addr_of_mut!((*event).data.document_start.version_directive);
        *fresh164 = version_directive_copy;
        let fresh165 = addr_of_mut!((*event).data.document_start.tag_directives.start);
        *fresh165 = tag_directives_copy.start;
        let fresh166 = addr_of_mut!((*event).data.document_start.tag_directives.end);
        *fresh166 = tag_directives_copy.top;
        (*event).data.document_start.implicit = implicit;
        return OK;
    }
    yaml_free(version_directive_copy as *mut libc::c_void);
    while !STACK_EMPTY!(tag_directives_copy) {
        let value = POP!(tag_directives_copy);
        yaml_free(value.handle as *mut libc::c_void);
        yaml_free(value.prefix as *mut libc::c_void);
    }
    STACK_DEL!(tag_directives_copy);
    yaml_free(value.handle as *mut libc::c_void);
    yaml_free(value.prefix as *mut libc::c_void);
    FAIL
}

/// Create the DOCUMENT-END event.
///
/// The `implicit` argument is considered as a stylistic parameter and may be
/// ignored by the emitter.
pub unsafe fn yaml_document_end_event_initialize(
    event: *mut yaml_event_t,
    implicit: bool,
) -> Success {
    let mark = yaml_mark_t {
        index: 0_u64,
        line: 0_u64,
        column: 0_u64,
    };
    __assert!(!event.is_null());
    memset(
        event as *mut libc::c_void,
        0,
        size_of::<yaml_event_t>() as libc::c_ulong,
    );
    (*event).type_ = YAML_DOCUMENT_END_EVENT;
    (*event).start_mark = mark;
    (*event).end_mark = mark;
    (*event).data.document_end.implicit = implicit;
    OK
}

/// Create an ALIAS event.
pub unsafe fn yaml_alias_event_initialize(
    event: *mut yaml_event_t,
    anchor: *const yaml_char_t,
) -> Success {
    let mark = yaml_mark_t {
        index: 0_u64,
        line: 0_u64,
        column: 0_u64,
    };
    __assert!(!event.is_null());
    __assert!(!anchor.is_null());
    if yaml_check_utf8(anchor, strlen(anchor as *mut libc::c_char)).fail {
        return FAIL;
    }
    let anchor_copy: *mut yaml_char_t = yaml_strdup(anchor);
    if anchor_copy.is_null() {
        return FAIL;
    }
    memset(
        event as *mut libc::c_void,
        0,
        size_of::<yaml_event_t>() as libc::c_ulong,
    );
    (*event).type_ = YAML_ALIAS_EVENT;
    (*event).start_mark = mark;
    (*event).end_mark = mark;
    let fresh167 = addr_of_mut!((*event).data.alias.anchor);
    *fresh167 = anchor_copy;
    OK
}

/// Create a SCALAR event.
///
/// The `style` argument may be ignored by the emitter.
///
/// Either the `tag` attribute or one of the `plain_implicit` and
/// `quoted_implicit` flags must be set.
///
pub unsafe fn yaml_scalar_event_initialize(
    event: *mut yaml_event_t,
    anchor: *const yaml_char_t,
    tag: *const yaml_char_t,
    value: *const yaml_char_t,
    mut length: libc::c_int,
    plain_implicit: bool,
    quoted_implicit: bool,
    style: yaml_scalar_style_t,
) -> Success {
    let mut current_block: u64;
    let mark = yaml_mark_t {
        index: 0_u64,
        line: 0_u64,
        column: 0_u64,
    };
    let mut anchor_copy: *mut yaml_char_t = ptr::null_mut::<yaml_char_t>();
    let mut tag_copy: *mut yaml_char_t = ptr::null_mut::<yaml_char_t>();
    let mut value_copy: *mut yaml_char_t = ptr::null_mut::<yaml_char_t>();
    __assert!(!event.is_null());
    __assert!(!value.is_null());
    if !anchor.is_null() {
        if yaml_check_utf8(anchor, strlen(anchor as *mut libc::c_char)).fail {
            current_block = 16285396129609901221;
        } else {
            anchor_copy = yaml_strdup(anchor);
            if anchor_copy.is_null() {
                current_block = 16285396129609901221;
            } else {
                current_block = 8515828400728868193;
            }
        }
    } else {
        current_block = 8515828400728868193;
    }
    if current_block == 8515828400728868193 {
        if !tag.is_null() {
            if yaml_check_utf8(tag, strlen(tag as *mut libc::c_char)).fail {
                current_block = 16285396129609901221;
            } else {
                tag_copy = yaml_strdup(tag);
                if tag_copy.is_null() {
                    current_block = 16285396129609901221;
                } else {
                    current_block = 12800627514080957624;
                }
            }
        } else {
            current_block = 12800627514080957624;
        }
        if current_block != 16285396129609901221 {
            if length < 0 {
                length = strlen(value as *mut libc::c_char) as libc::c_int;
            }
            if yaml_check_utf8(value, length as size_t).ok {
                value_copy = yaml_malloc(length.force_add(1) as size_t) as *mut yaml_char_t;
                memcpy(
                    value_copy as *mut libc::c_void,
                    value as *const libc::c_void,
                    length as libc::c_ulong,
                );
                *value_copy.wrapping_offset(length as isize) = b'\0';
                memset(
                    event as *mut libc::c_void,
                    0,
                    size_of::<yaml_event_t>() as libc::c_ulong,
                );
                (*event).type_ = YAML_SCALAR_EVENT;
                (*event).start_mark = mark;
                (*event).end_mark = mark;
                let fresh168 = addr_of_mut!((*event).data.scalar.anchor);
                *fresh168 = anchor_copy;
                let fresh169 = addr_of_mut!((*event).data.scalar.tag);
                *fresh169 = tag_copy;
                let fresh170 = addr_of_mut!((*event).data.scalar.value);
                *fresh170 = value_copy;
                (*event).data.scalar.length = length as size_t;
                (*event).data.scalar.plain_implicit = plain_implicit;
                (*event).data.scalar.quoted_implicit = quoted_implicit;
                (*event).data.scalar.style = style;
                return OK;
            }
        }
    }
    yaml_free(anchor_copy as *mut libc::c_void);
    yaml_free(tag_copy as *mut libc::c_void);
    yaml_free(value_copy as *mut libc::c_void);
    FAIL
}

/// Create a SEQUENCE-START event.
///
/// The `style` argument may be ignored by the emitter.
///
/// Either the `tag` attribute or the `implicit` flag must be set.
pub unsafe fn yaml_sequence_start_event_initialize(
    event: *mut yaml_event_t,
    anchor: *const yaml_char_t,
    tag: *const yaml_char_t,
    implicit: bool,
    style: yaml_sequence_style_t,
) -> Success {
    let mut current_block: u64;
    let mark = yaml_mark_t {
        index: 0_u64,
        line: 0_u64,
        column: 0_u64,
    };
    let mut anchor_copy: *mut yaml_char_t = ptr::null_mut::<yaml_char_t>();
    let mut tag_copy: *mut yaml_char_t = ptr::null_mut::<yaml_char_t>();
    __assert!(!event.is_null());
    if !anchor.is_null() {
        if yaml_check_utf8(anchor, strlen(anchor as *mut libc::c_char)).fail {
            current_block = 8817775685815971442;
        } else {
            anchor_copy = yaml_strdup(anchor);
            if anchor_copy.is_null() {
                current_block = 8817775685815971442;
            } else {
                current_block = 11006700562992250127;
            }
        }
    } else {
        current_block = 11006700562992250127;
    }
    match current_block {
        11006700562992250127 => {
            if !tag.is_null() {
                if yaml_check_utf8(tag, strlen(tag as *mut libc::c_char)).fail {
                    current_block = 8817775685815971442;
                } else {
                    tag_copy = yaml_strdup(tag);
                    if tag_copy.is_null() {
                        current_block = 8817775685815971442;
                    } else {
                        current_block = 7651349459974463963;
                    }
                }
            } else {
                current_block = 7651349459974463963;
            }
            if current_block != 8817775685815971442 {
                memset(
                    event as *mut libc::c_void,
                    0,
                    size_of::<yaml_event_t>() as libc::c_ulong,
                );
                (*event).type_ = YAML_SEQUENCE_START_EVENT;
                (*event).start_mark = mark;
                (*event).end_mark = mark;
                let fresh171 = addr_of_mut!((*event).data.sequence_start.anchor);
                *fresh171 = anchor_copy;
                let fresh172 = addr_of_mut!((*event).data.sequence_start.tag);
                *fresh172 = tag_copy;
                (*event).data.sequence_start.implicit = implicit;
                (*event).data.sequence_start.style = style;
                return OK;
            }
        }
        _ => {}
    }
    yaml_free(anchor_copy as *mut libc::c_void);
    yaml_free(tag_copy as *mut libc::c_void);
    FAIL
}

/// Create a SEQUENCE-END event.
pub unsafe fn yaml_sequence_end_event_initialize(event: *mut yaml_event_t) -> Success {
    let mark = yaml_mark_t {
        index: 0_u64,
        line: 0_u64,
        column: 0_u64,
    };
    __assert!(!event.is_null());
    memset(
        event as *mut libc::c_void,
        0,
        size_of::<yaml_event_t>() as libc::c_ulong,
    );
    (*event).type_ = YAML_SEQUENCE_END_EVENT;
    (*event).start_mark = mark;
    (*event).end_mark = mark;
    OK
}

/// Create a MAPPING-START event.
///
/// The `style` argument may be ignored by the emitter.
///
/// Either the `tag` attribute or the `implicit` flag must be set.
pub unsafe fn yaml_mapping_start_event_initialize(
    event: *mut yaml_event_t,
    anchor: *const yaml_char_t,
    tag: *const yaml_char_t,
    implicit: bool,
    style: yaml_mapping_style_t,
) -> Success {
    let mut current_block: u64;
    let mark = yaml_mark_t {
        index: 0_u64,
        line: 0_u64,
        column: 0_u64,
    };
    let mut anchor_copy: *mut yaml_char_t = ptr::null_mut::<yaml_char_t>();
    let mut tag_copy: *mut yaml_char_t = ptr::null_mut::<yaml_char_t>();
    __assert!(!event.is_null());
    if !anchor.is_null() {
        if yaml_check_utf8(anchor, strlen(anchor as *mut libc::c_char)).fail {
            current_block = 14748279734549812740;
        } else {
            anchor_copy = yaml_strdup(anchor);
            if anchor_copy.is_null() {
                current_block = 14748279734549812740;
            } else {
                current_block = 11006700562992250127;
            }
        }
    } else {
        current_block = 11006700562992250127;
    }
    if current_block == 11006700562992250127 {
        if !tag.is_null() {
            if yaml_check_utf8(tag, strlen(tag as *mut libc::c_char)).fail {
                current_block = 14748279734549812740;
            } else {
                tag_copy = yaml_strdup(tag);
                if tag_copy.is_null() {
                    current_block = 14748279734549812740;
                } else {
                    current_block = 7651349459974463963;
                }
            }
        } else {
            current_block = 7651349459974463963;
        }
        if current_block != 14748279734549812740 {
            memset(
                event as *mut libc::c_void,
                0,
                size_of::<yaml_event_t>() as libc::c_ulong,
            );
            (*event).type_ = YAML_MAPPING_START_EVENT;
            (*event).start_mark = mark;
            (*event).end_mark = mark;
            let fresh173 = addr_of_mut!((*event).data.mapping_start.anchor);
            *fresh173 = anchor_copy;
            let fresh174 = addr_of_mut!((*event).data.mapping_start.tag);
            *fresh174 = tag_copy;
            (*event).data.mapping_start.implicit = implicit;
            (*event).data.mapping_start.style = style;
            return OK;
        }
    }
    yaml_free(anchor_copy as *mut libc::c_void);
    yaml_free(tag_copy as *mut libc::c_void);
    FAIL
}

/// Create a MAPPING-END event.
pub unsafe fn yaml_mapping_end_event_initialize(event: *mut yaml_event_t) -> Success {
    let mark = yaml_mark_t {
        index: 0_u64,
        line: 0_u64,
        column: 0_u64,
    };
    __assert!(!event.is_null());
    memset(
        event as *mut libc::c_void,
        0,
        size_of::<yaml_event_t>() as libc::c_ulong,
    );
    (*event).type_ = YAML_MAPPING_END_EVENT;
    (*event).start_mark = mark;
    (*event).end_mark = mark;
    OK
}

/// Free any memory allocated for an event object.
pub unsafe fn yaml_event_delete(event: *mut yaml_event_t) {
    let mut tag_directive: *mut yaml_tag_directive_t;
    __assert!(!event.is_null());
    match (*event).type_ {
        YAML_DOCUMENT_START_EVENT => {
            yaml_free((*event).data.document_start.version_directive as *mut libc::c_void);
            tag_directive = (*event).data.document_start.tag_directives.start;
            while tag_directive != (*event).data.document_start.tag_directives.end {
                yaml_free((*tag_directive).handle as *mut libc::c_void);
                yaml_free((*tag_directive).prefix as *mut libc::c_void);
                tag_directive = tag_directive.wrapping_offset(1);
            }
            yaml_free((*event).data.document_start.tag_directives.start as *mut libc::c_void);
        }
        YAML_ALIAS_EVENT => {
            yaml_free((*event).data.alias.anchor as *mut libc::c_void);
        }
        YAML_SCALAR_EVENT => {
            yaml_free((*event).data.scalar.anchor as *mut libc::c_void);
            yaml_free((*event).data.scalar.tag as *mut libc::c_void);
            yaml_free((*event).data.scalar.value as *mut libc::c_void);
        }
        YAML_SEQUENCE_START_EVENT => {
            yaml_free((*event).data.sequence_start.anchor as *mut libc::c_void);
            yaml_free((*event).data.sequence_start.tag as *mut libc::c_void);
        }
        YAML_MAPPING_START_EVENT => {
            yaml_free((*event).data.mapping_start.anchor as *mut libc::c_void);
            yaml_free((*event).data.mapping_start.tag as *mut libc::c_void);
        }
        _ => {}
    }
    memset(
        event as *mut libc::c_void,
        0,
        size_of::<yaml_event_t>() as libc::c_ulong,
    );
}

/// Create a YAML document.
pub unsafe fn yaml_document_initialize(
    document: *mut yaml_document_t,
    version_directive: *mut yaml_version_directive_t,
    tag_directives_start: *mut yaml_tag_directive_t,
    tag_directives_end: *mut yaml_tag_directive_t,
    start_implicit: bool,
    end_implicit: bool,
) -> Success {
    let current_block: u64;
    struct Nodes {
        start: *mut yaml_node_t,
        end: *mut yaml_node_t,
        top: *mut yaml_node_t,
    }
    let mut nodes = Nodes {
        start: ptr::null_mut::<yaml_node_t>(),
        end: ptr::null_mut::<yaml_node_t>(),
        top: ptr::null_mut::<yaml_node_t>(),
    };
    let mut version_directive_copy: *mut yaml_version_directive_t =
        ptr::null_mut::<yaml_version_directive_t>();
    struct TagDirectivesCopy {
        start: *mut yaml_tag_directive_t,
        end: *mut yaml_tag_directive_t,
        top: *mut yaml_tag_directive_t,
    }
    let mut tag_directives_copy = TagDirectivesCopy {
        start: ptr::null_mut::<yaml_tag_directive_t>(),
        end: ptr::null_mut::<yaml_tag_directive_t>(),
        top: ptr::null_mut::<yaml_tag_directive_t>(),
    };
    let mut value = yaml_tag_directive_t {
        handle: ptr::null_mut::<yaml_char_t>(),
        prefix: ptr::null_mut::<yaml_char_t>(),
    };
    let mark = yaml_mark_t {
        index: 0_u64,
        line: 0_u64,
        column: 0_u64,
    };
    __assert!(!document.is_null());
    __assert!(
        !tag_directives_start.is_null() && !tag_directives_end.is_null()
            || tag_directives_start == tag_directives_end
    );
    STACK_INIT!(nodes, yaml_node_t);
    if !version_directive.is_null() {
        version_directive_copy = yaml_malloc(size_of::<yaml_version_directive_t>() as libc::c_ulong)
            as *mut yaml_version_directive_t;
        (*version_directive_copy).major = (*version_directive).major;
        (*version_directive_copy).minor = (*version_directive).minor;
    }
    if tag_directives_start != tag_directives_end {
        let mut tag_directive: *mut yaml_tag_directive_t;
        STACK_INIT!(tag_directives_copy, yaml_tag_directive_t);
        tag_directive = tag_directives_start;
        loop {
            if !(tag_directive != tag_directives_end) {
                current_block = 14818589718467733107;
                break;
            }
            __assert!(!((*tag_directive).handle).is_null());
            __assert!(!((*tag_directive).prefix).is_null());
            if yaml_check_utf8(
                (*tag_directive).handle,
                strlen((*tag_directive).handle as *mut libc::c_char),
            )
            .fail
            {
                current_block = 8142820162064489797;
                break;
            }
            if yaml_check_utf8(
                (*tag_directive).prefix,
                strlen((*tag_directive).prefix as *mut libc::c_char),
            )
            .fail
            {
                current_block = 8142820162064489797;
                break;
            }
            value.handle = yaml_strdup((*tag_directive).handle);
            value.prefix = yaml_strdup((*tag_directive).prefix);
            if value.handle.is_null() || value.prefix.is_null() {
                current_block = 8142820162064489797;
                break;
            }
            PUSH!(tag_directives_copy, value);
            value.handle = ptr::null_mut::<yaml_char_t>();
            value.prefix = ptr::null_mut::<yaml_char_t>();
            tag_directive = tag_directive.wrapping_offset(1);
        }
    } else {
        current_block = 14818589718467733107;
    }
    if current_block != 8142820162064489797 {
        memset(
            document as *mut libc::c_void,
            0,
            size_of::<yaml_document_t>() as libc::c_ulong,
        );
        let fresh176 = addr_of_mut!((*document).nodes.start);
        *fresh176 = nodes.start;
        let fresh177 = addr_of_mut!((*document).nodes.end);
        *fresh177 = nodes.end;
        let fresh178 = addr_of_mut!((*document).nodes.top);
        *fresh178 = nodes.start;
        let fresh179 = addr_of_mut!((*document).version_directive);
        *fresh179 = version_directive_copy;
        let fresh180 = addr_of_mut!((*document).tag_directives.start);
        *fresh180 = tag_directives_copy.start;
        let fresh181 = addr_of_mut!((*document).tag_directives.end);
        *fresh181 = tag_directives_copy.top;
        (*document).start_implicit = start_implicit;
        (*document).end_implicit = end_implicit;
        (*document).start_mark = mark;
        (*document).end_mark = mark;
        return OK;
    }
    STACK_DEL!(nodes);
    yaml_free(version_directive_copy as *mut libc::c_void);
    while !STACK_EMPTY!(tag_directives_copy) {
        let value = POP!(tag_directives_copy);
        yaml_free(value.handle as *mut libc::c_void);
        yaml_free(value.prefix as *mut libc::c_void);
    }
    STACK_DEL!(tag_directives_copy);
    yaml_free(value.handle as *mut libc::c_void);
    yaml_free(value.prefix as *mut libc::c_void);
    FAIL
}

/// Delete a YAML document and all its nodes.
pub unsafe fn yaml_document_delete(document: *mut yaml_document_t) {
    let mut tag_directive: *mut yaml_tag_directive_t;
    __assert!(!document.is_null());
    while !STACK_EMPTY!((*document).nodes) {
        let mut node = POP!((*document).nodes);
        yaml_free(node.tag as *mut libc::c_void);
        match node.type_ {
            YAML_SCALAR_NODE => {
                yaml_free(node.data.scalar.value as *mut libc::c_void);
            }
            YAML_SEQUENCE_NODE => {
                STACK_DEL!(node.data.sequence.items);
            }
            YAML_MAPPING_NODE => {
                STACK_DEL!(node.data.mapping.pairs);
            }
            _ => {
                __assert!(false);
            }
        }
    }
    STACK_DEL!((*document).nodes);
    yaml_free((*document).version_directive as *mut libc::c_void);
    tag_directive = (*document).tag_directives.start;
    while tag_directive != (*document).tag_directives.end {
        yaml_free((*tag_directive).handle as *mut libc::c_void);
        yaml_free((*tag_directive).prefix as *mut libc::c_void);
        tag_directive = tag_directive.wrapping_offset(1);
    }
    yaml_free((*document).tag_directives.start as *mut libc::c_void);
    memset(
        document as *mut libc::c_void,
        0,
        size_of::<yaml_document_t>() as libc::c_ulong,
    );
}

/// Get a node of a YAML document.
///
/// The pointer returned by this function is valid until any of the functions
/// modifying the documents are called.
///
/// Returns the node objct or NULL if `node_id` is out of range.
pub unsafe fn yaml_document_get_node(
    document: *mut yaml_document_t,
    index: libc::c_int,
) -> *mut yaml_node_t {
    __assert!(!document.is_null());
    if index > 0 && (*document).nodes.start.wrapping_offset(index as isize) <= (*document).nodes.top
    {
        return (*document)
            .nodes
            .start
            .wrapping_offset(index as isize)
            .wrapping_offset(-1_isize);
    }
    ptr::null_mut::<yaml_node_t>()
}

/// Get the root of a YAML document node.
///
/// The root object is the first object added to the document.
///
/// The pointer returned by this function is valid until any of the functions
/// modifying the documents are called.
///
/// An empty document produced by the parser signifies the end of a YAML stream.
///
/// Returns the node object or NULL if the document is empty.
pub unsafe fn yaml_document_get_root_node(document: *mut yaml_document_t) -> *mut yaml_node_t {
    __assert!(!document.is_null());
    if (*document).nodes.top != (*document).nodes.start {
        return (*document).nodes.start;
    }
    ptr::null_mut::<yaml_node_t>()
}

/// Create a SCALAR node and attach it to the document.
///
/// The `style` argument may be ignored by the emitter.
///
/// Returns the node id or 0 on error.
#[must_use]
pub unsafe fn yaml_document_add_scalar(
    document: *mut yaml_document_t,
    mut tag: *const yaml_char_t,
    value: *const yaml_char_t,
    mut length: libc::c_int,
    style: yaml_scalar_style_t,
) -> libc::c_int {
    let mark = yaml_mark_t {
        index: 0_u64,
        line: 0_u64,
        column: 0_u64,
    };
    let mut tag_copy: *mut yaml_char_t = ptr::null_mut::<yaml_char_t>();
    let mut value_copy: *mut yaml_char_t = ptr::null_mut::<yaml_char_t>();
    let mut node = MaybeUninit::<yaml_node_t>::uninit();
    let node = node.as_mut_ptr();
    __assert!(!document.is_null());
    __assert!(!value.is_null());
    if tag.is_null() {
        tag = b"tag:yaml.org,2002:str\0" as *const u8 as *const libc::c_char as *mut yaml_char_t;
    }
    if yaml_check_utf8(tag, strlen(tag as *mut libc::c_char)).ok {
        tag_copy = yaml_strdup(tag);
        if !tag_copy.is_null() {
            if length < 0 {
                length = strlen(value as *mut libc::c_char) as libc::c_int;
            }
            if yaml_check_utf8(value, length as size_t).ok {
                value_copy = yaml_malloc(length.force_add(1) as size_t) as *mut yaml_char_t;
                memcpy(
                    value_copy as *mut libc::c_void,
                    value as *const libc::c_void,
                    length as libc::c_ulong,
                );
                *value_copy.wrapping_offset(length as isize) = b'\0';
                memset(
                    node as *mut libc::c_void,
                    0,
                    size_of::<yaml_node_t>() as libc::c_ulong,
                );
                (*node).type_ = YAML_SCALAR_NODE;
                (*node).tag = tag_copy;
                (*node).start_mark = mark;
                (*node).end_mark = mark;
                (*node).data.scalar.value = value_copy;
                (*node).data.scalar.length = length as size_t;
                (*node).data.scalar.style = style;
                PUSH!((*document).nodes, *node);
                return (*document).nodes.top.c_offset_from((*document).nodes.start) as libc::c_int;
            }
        }
    }
    yaml_free(tag_copy as *mut libc::c_void);
    yaml_free(value_copy as *mut libc::c_void);
    0
}

/// Create a SEQUENCE node and attach it to the document.
///
/// The `style` argument may be ignored by the emitter.
///
/// Returns the node id or 0 on error.
#[must_use]
pub unsafe fn yaml_document_add_sequence(
    document: *mut yaml_document_t,
    mut tag: *const yaml_char_t,
    style: yaml_sequence_style_t,
) -> libc::c_int {
    let mark = yaml_mark_t {
        index: 0_u64,
        line: 0_u64,
        column: 0_u64,
    };
    let mut tag_copy: *mut yaml_char_t = ptr::null_mut::<yaml_char_t>();
    struct Items {
        start: *mut yaml_node_item_t,
        end: *mut yaml_node_item_t,
        top: *mut yaml_node_item_t,
    }
    let mut items = Items {
        start: ptr::null_mut::<yaml_node_item_t>(),
        end: ptr::null_mut::<yaml_node_item_t>(),
        top: ptr::null_mut::<yaml_node_item_t>(),
    };
    let mut node = MaybeUninit::<yaml_node_t>::uninit();
    let node = node.as_mut_ptr();
    __assert!(!document.is_null());
    if tag.is_null() {
        tag = b"tag:yaml.org,2002:seq\0" as *const u8 as *const libc::c_char as *mut yaml_char_t;
    }
    if yaml_check_utf8(tag, strlen(tag as *mut libc::c_char)).ok {
        tag_copy = yaml_strdup(tag);
        if !tag_copy.is_null() {
            STACK_INIT!(items, yaml_node_item_t);
            memset(
                node as *mut libc::c_void,
                0,
                size_of::<yaml_node_t>() as libc::c_ulong,
            );
            (*node).type_ = YAML_SEQUENCE_NODE;
            (*node).tag = tag_copy;
            (*node).start_mark = mark;
            (*node).end_mark = mark;
            (*node).data.sequence.items.start = items.start;
            (*node).data.sequence.items.end = items.end;
            (*node).data.sequence.items.top = items.start;
            (*node).data.sequence.style = style;
            PUSH!((*document).nodes, *node);
            return (*document).nodes.top.c_offset_from((*document).nodes.start) as libc::c_int;
        }
    }
    STACK_DEL!(items);
    yaml_free(tag_copy as *mut libc::c_void);
    0
}

/// Create a MAPPING node and attach it to the document.
///
/// The `style` argument may be ignored by the emitter.
///
/// Returns the node id or 0 on error.
#[must_use]
pub unsafe fn yaml_document_add_mapping(
    document: *mut yaml_document_t,
    mut tag: *const yaml_char_t,
    style: yaml_mapping_style_t,
) -> libc::c_int {
    let mark = yaml_mark_t {
        index: 0_u64,
        line: 0_u64,
        column: 0_u64,
    };
    let mut tag_copy: *mut yaml_char_t = ptr::null_mut::<yaml_char_t>();
    struct Pairs {
        start: *mut yaml_node_pair_t,
        end: *mut yaml_node_pair_t,
        top: *mut yaml_node_pair_t,
    }
    let mut pairs = Pairs {
        start: ptr::null_mut::<yaml_node_pair_t>(),
        end: ptr::null_mut::<yaml_node_pair_t>(),
        top: ptr::null_mut::<yaml_node_pair_t>(),
    };
    let mut node = MaybeUninit::<yaml_node_t>::uninit();
    let node = node.as_mut_ptr();
    __assert!(!document.is_null());
    if tag.is_null() {
        tag = b"tag:yaml.org,2002:map\0" as *const u8 as *const libc::c_char as *mut yaml_char_t;
    }
    if yaml_check_utf8(tag, strlen(tag as *mut libc::c_char)).ok {
        tag_copy = yaml_strdup(tag);
        if !tag_copy.is_null() {
            STACK_INIT!(pairs, yaml_node_pair_t);
            memset(
                node as *mut libc::c_void,
                0,
                size_of::<yaml_node_t>() as libc::c_ulong,
            );
            (*node).type_ = YAML_MAPPING_NODE;
            (*node).tag = tag_copy;
            (*node).start_mark = mark;
            (*node).end_mark = mark;
            (*node).data.mapping.pairs.start = pairs.start;
            (*node).data.mapping.pairs.end = pairs.end;
            (*node).data.mapping.pairs.top = pairs.start;
            (*node).data.mapping.style = style;
            PUSH!((*document).nodes, *node);
            return (*document).nodes.top.c_offset_from((*document).nodes.start) as libc::c_int;
        }
    }
    STACK_DEL!(pairs);
    yaml_free(tag_copy as *mut libc::c_void);
    0
}

/// Add an item to a SEQUENCE node.
pub unsafe fn yaml_document_append_sequence_item(
    document: *mut yaml_document_t,
    sequence: libc::c_int,
    item: libc::c_int,
) -> Success {
    __assert!(!document.is_null());
    __assert!(
        sequence > 0
            && ((*document).nodes.start).wrapping_offset(sequence as isize)
                <= (*document).nodes.top
    );
    __assert!(
        (*((*document).nodes.start).wrapping_offset((sequence - 1) as isize)).type_
            == YAML_SEQUENCE_NODE
    );
    __assert!(
        item > 0
            && ((*document).nodes.start).wrapping_offset(item as isize) <= (*document).nodes.top
    );
    PUSH!(
        (*((*document).nodes.start).wrapping_offset((sequence - 1) as isize))
            .data
            .sequence
            .items,
        item
    );
    OK
}

/// Add a pair of a key and a value to a MAPPING node.
pub unsafe fn yaml_document_append_mapping_pair(
    document: *mut yaml_document_t,
    mapping: libc::c_int,
    key: libc::c_int,
    value: libc::c_int,
) -> Success {
    __assert!(!document.is_null());
    __assert!(
        mapping > 0
            && ((*document).nodes.start).wrapping_offset(mapping as isize) <= (*document).nodes.top
    );
    __assert!(
        (*((*document).nodes.start).wrapping_offset((mapping - 1) as isize)).type_
            == YAML_MAPPING_NODE
    );
    __assert!(
        key > 0 && ((*document).nodes.start).wrapping_offset(key as isize) <= (*document).nodes.top
    );
    __assert!(
        value > 0
            && ((*document).nodes.start).wrapping_offset(value as isize) <= (*document).nodes.top
    );
    let pair = yaml_node_pair_t { key, value };
    PUSH!(
        (*((*document).nodes.start).wrapping_offset((mapping - 1) as isize))
            .data
            .mapping
            .pairs,
        pair
    );
    OK
}
