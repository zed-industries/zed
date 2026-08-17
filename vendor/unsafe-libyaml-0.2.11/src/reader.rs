use crate::externs::{memcmp, memmove};
use crate::ops::ForceAdd as _;
use crate::success::{Success, FAIL, OK};
use crate::yaml::{size_t, yaml_char_t};
use crate::{
    libc, yaml_parser_t, PointerExt, YAML_ANY_ENCODING, YAML_READER_ERROR, YAML_UTF16BE_ENCODING,
    YAML_UTF16LE_ENCODING, YAML_UTF8_ENCODING,
};
use core::ptr::addr_of_mut;

unsafe fn yaml_parser_set_reader_error(
    parser: *mut yaml_parser_t,
    problem: *const libc::c_char,
    offset: size_t,
    value: libc::c_int,
) -> Success {
    (*parser).error = YAML_READER_ERROR;
    let fresh0 = addr_of_mut!((*parser).problem);
    *fresh0 = problem;
    (*parser).problem_offset = offset;
    (*parser).problem_value = value;
    FAIL
}

const BOM_UTF8: *const libc::c_char = b"\xEF\xBB\xBF\0" as *const u8 as *const libc::c_char;
const BOM_UTF16LE: *const libc::c_char = b"\xFF\xFE\0" as *const u8 as *const libc::c_char;
const BOM_UTF16BE: *const libc::c_char = b"\xFE\xFF\0" as *const u8 as *const libc::c_char;

unsafe fn yaml_parser_determine_encoding(parser: *mut yaml_parser_t) -> Success {
    while !(*parser).eof
        && ((*parser)
            .raw_buffer
            .last
            .c_offset_from((*parser).raw_buffer.pointer) as libc::c_long)
            < 3_i64
    {
        if yaml_parser_update_raw_buffer(parser).fail {
            return FAIL;
        }
    }
    if (*parser)
        .raw_buffer
        .last
        .c_offset_from((*parser).raw_buffer.pointer) as libc::c_long
        >= 2_i64
        && memcmp(
            (*parser).raw_buffer.pointer as *const libc::c_void,
            BOM_UTF16LE as *const libc::c_void,
            2_u64,
        ) == 0
    {
        (*parser).encoding = YAML_UTF16LE_ENCODING;
        let fresh1 = addr_of_mut!((*parser).raw_buffer.pointer);
        *fresh1 = (*fresh1).wrapping_offset(2_isize);
        let fresh2 = addr_of_mut!((*parser).offset);
        *fresh2 = (*fresh2 as libc::c_ulong).force_add(2_u64) as size_t;
    } else if (*parser)
        .raw_buffer
        .last
        .c_offset_from((*parser).raw_buffer.pointer) as libc::c_long
        >= 2_i64
        && memcmp(
            (*parser).raw_buffer.pointer as *const libc::c_void,
            BOM_UTF16BE as *const libc::c_void,
            2_u64,
        ) == 0
    {
        (*parser).encoding = YAML_UTF16BE_ENCODING;
        let fresh3 = addr_of_mut!((*parser).raw_buffer.pointer);
        *fresh3 = (*fresh3).wrapping_offset(2_isize);
        let fresh4 = addr_of_mut!((*parser).offset);
        *fresh4 = (*fresh4 as libc::c_ulong).force_add(2_u64) as size_t;
    } else if (*parser)
        .raw_buffer
        .last
        .c_offset_from((*parser).raw_buffer.pointer) as libc::c_long
        >= 3_i64
        && memcmp(
            (*parser).raw_buffer.pointer as *const libc::c_void,
            BOM_UTF8 as *const libc::c_void,
            3_u64,
        ) == 0
    {
        (*parser).encoding = YAML_UTF8_ENCODING;
        let fresh5 = addr_of_mut!((*parser).raw_buffer.pointer);
        *fresh5 = (*fresh5).wrapping_offset(3_isize);
        let fresh6 = addr_of_mut!((*parser).offset);
        *fresh6 = (*fresh6 as libc::c_ulong).force_add(3_u64) as size_t;
    } else {
        (*parser).encoding = YAML_UTF8_ENCODING;
    }
    OK
}

unsafe fn yaml_parser_update_raw_buffer(parser: *mut yaml_parser_t) -> Success {
    let mut size_read: size_t = 0_u64;
    if (*parser).raw_buffer.start == (*parser).raw_buffer.pointer
        && (*parser).raw_buffer.last == (*parser).raw_buffer.end
    {
        return OK;
    }
    if (*parser).eof {
        return OK;
    }
    if (*parser).raw_buffer.start < (*parser).raw_buffer.pointer
        && (*parser).raw_buffer.pointer < (*parser).raw_buffer.last
    {
        memmove(
            (*parser).raw_buffer.start as *mut libc::c_void,
            (*parser).raw_buffer.pointer as *const libc::c_void,
            (*parser)
                .raw_buffer
                .last
                .c_offset_from((*parser).raw_buffer.pointer) as libc::c_long
                as libc::c_ulong,
        );
    }
    let fresh7 = addr_of_mut!((*parser).raw_buffer.last);
    *fresh7 = (*fresh7).wrapping_offset(
        -((*parser)
            .raw_buffer
            .pointer
            .c_offset_from((*parser).raw_buffer.start) as libc::c_long as isize),
    );
    let fresh8 = addr_of_mut!((*parser).raw_buffer.pointer);
    *fresh8 = (*parser).raw_buffer.start;
    if (*parser).read_handler.expect("non-null function pointer")(
        (*parser).read_handler_data,
        (*parser).raw_buffer.last,
        (*parser)
            .raw_buffer
            .end
            .c_offset_from((*parser).raw_buffer.last) as size_t,
        addr_of_mut!(size_read),
    ) == 0
    {
        return yaml_parser_set_reader_error(
            parser,
            b"input error\0" as *const u8 as *const libc::c_char,
            (*parser).offset,
            -1,
        );
    }
    let fresh9 = addr_of_mut!((*parser).raw_buffer.last);
    *fresh9 = (*fresh9).wrapping_offset(size_read as isize);
    if size_read == 0 {
        (*parser).eof = true;
    }
    OK
}

pub(crate) unsafe fn yaml_parser_update_buffer(
    parser: *mut yaml_parser_t,
    length: size_t,
) -> Success {
    let mut first = true;
    __assert!(((*parser).read_handler).is_some());
    if (*parser).eof && (*parser).raw_buffer.pointer == (*parser).raw_buffer.last {
        return OK;
    }
    if (*parser).unread >= length {
        return OK;
    }
    if (*parser).encoding == YAML_ANY_ENCODING {
        if yaml_parser_determine_encoding(parser).fail {
            return FAIL;
        }
    }
    if (*parser).buffer.start < (*parser).buffer.pointer
        && (*parser).buffer.pointer < (*parser).buffer.last
    {
        let size: size_t = (*parser)
            .buffer
            .last
            .c_offset_from((*parser).buffer.pointer) as size_t;
        memmove(
            (*parser).buffer.start as *mut libc::c_void,
            (*parser).buffer.pointer as *const libc::c_void,
            size,
        );
        let fresh10 = addr_of_mut!((*parser).buffer.pointer);
        *fresh10 = (*parser).buffer.start;
        let fresh11 = addr_of_mut!((*parser).buffer.last);
        *fresh11 = (*parser).buffer.start.wrapping_offset(size as isize);
    } else if (*parser).buffer.pointer == (*parser).buffer.last {
        let fresh12 = addr_of_mut!((*parser).buffer.pointer);
        *fresh12 = (*parser).buffer.start;
        let fresh13 = addr_of_mut!((*parser).buffer.last);
        *fresh13 = (*parser).buffer.start;
    }
    while (*parser).unread < length {
        if !first || (*parser).raw_buffer.pointer == (*parser).raw_buffer.last {
            if yaml_parser_update_raw_buffer(parser).fail {
                return FAIL;
            }
        }
        first = false;
        while (*parser).raw_buffer.pointer != (*parser).raw_buffer.last {
            let mut value: libc::c_uint = 0;
            let value2: libc::c_uint;
            let mut incomplete = false;
            let mut octet: libc::c_uchar;
            let mut width: libc::c_uint = 0;
            let low: libc::c_int;
            let high: libc::c_int;
            let mut k: size_t;
            let raw_unread: size_t = (*parser)
                .raw_buffer
                .last
                .c_offset_from((*parser).raw_buffer.pointer)
                as size_t;
            match (*parser).encoding {
                YAML_UTF8_ENCODING => {
                    octet = *(*parser).raw_buffer.pointer;
                    width = if octet & 0x80 == 0 {
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
                    if width == 0 {
                        return yaml_parser_set_reader_error(
                            parser,
                            b"invalid leading UTF-8 octet\0" as *const u8 as *const libc::c_char,
                            (*parser).offset,
                            octet as libc::c_int,
                        );
                    }
                    if width as libc::c_ulong > raw_unread {
                        if (*parser).eof {
                            return yaml_parser_set_reader_error(
                                parser,
                                b"incomplete UTF-8 octet sequence\0" as *const u8
                                    as *const libc::c_char,
                                (*parser).offset,
                                -1,
                            );
                        }
                        incomplete = true;
                    } else {
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
                        k = 1_u64;
                        while k < width as libc::c_ulong {
                            octet = *(*parser).raw_buffer.pointer.wrapping_offset(k as isize);
                            if octet & 0xC0 != 0x80 {
                                return yaml_parser_set_reader_error(
                                    parser,
                                    b"invalid trailing UTF-8 octet\0" as *const u8
                                        as *const libc::c_char,
                                    (*parser).offset.force_add(k),
                                    octet as libc::c_int,
                                );
                            }
                            value = (value << 6).force_add((octet & 0x3F) as libc::c_uint);
                            k = k.force_add(1);
                        }
                        if !(width == 1
                            || width == 2 && value >= 0x80
                            || width == 3 && value >= 0x800
                            || width == 4 && value >= 0x10000)
                        {
                            return yaml_parser_set_reader_error(
                                parser,
                                b"invalid length of a UTF-8 sequence\0" as *const u8
                                    as *const libc::c_char,
                                (*parser).offset,
                                -1,
                            );
                        }
                        if value >= 0xD800 && value <= 0xDFFF || value > 0x10FFFF {
                            return yaml_parser_set_reader_error(
                                parser,
                                b"invalid Unicode character\0" as *const u8 as *const libc::c_char,
                                (*parser).offset,
                                value as libc::c_int,
                            );
                        }
                    }
                }
                YAML_UTF16LE_ENCODING | YAML_UTF16BE_ENCODING => {
                    low = if (*parser).encoding == YAML_UTF16LE_ENCODING {
                        0
                    } else {
                        1
                    };
                    high = if (*parser).encoding == YAML_UTF16LE_ENCODING {
                        1
                    } else {
                        0
                    };
                    if raw_unread < 2_u64 {
                        if (*parser).eof {
                            return yaml_parser_set_reader_error(
                                parser,
                                b"incomplete UTF-16 character\0" as *const u8
                                    as *const libc::c_char,
                                (*parser).offset,
                                -1,
                            );
                        }
                        incomplete = true;
                    } else {
                        value = (*(*parser).raw_buffer.pointer.wrapping_offset(low as isize)
                            as libc::c_int
                            + ((*(*parser).raw_buffer.pointer.wrapping_offset(high as isize)
                                as libc::c_int)
                                << 8)) as libc::c_uint;
                        if value & 0xFC00 == 0xDC00 {
                            return yaml_parser_set_reader_error(
                                parser,
                                b"unexpected low surrogate area\0" as *const u8
                                    as *const libc::c_char,
                                (*parser).offset,
                                value as libc::c_int,
                            );
                        }
                        if value & 0xFC00 == 0xD800 {
                            width = 4;
                            if raw_unread < 4_u64 {
                                if (*parser).eof {
                                    return yaml_parser_set_reader_error(
                                        parser,
                                        b"incomplete UTF-16 surrogate pair\0" as *const u8
                                            as *const libc::c_char,
                                        (*parser).offset,
                                        -1,
                                    );
                                }
                                incomplete = true;
                            } else {
                                value2 = (*(*parser)
                                    .raw_buffer
                                    .pointer
                                    .wrapping_offset((low + 2) as isize)
                                    as libc::c_int
                                    + ((*(*parser)
                                        .raw_buffer
                                        .pointer
                                        .wrapping_offset((high + 2) as isize)
                                        as libc::c_int)
                                        << 8))
                                    as libc::c_uint;
                                if value2 & 0xFC00 != 0xDC00 {
                                    return yaml_parser_set_reader_error(
                                        parser,
                                        b"expected low surrogate area\0" as *const u8
                                            as *const libc::c_char,
                                        (*parser).offset.force_add(2_u64),
                                        value2 as libc::c_int,
                                    );
                                }
                                value = 0x10000_u32
                                    .force_add((value & 0x3FF) << 10)
                                    .force_add(value2 & 0x3FF);
                            }
                        } else {
                            width = 2;
                        }
                    }
                }
                _ => {}
            }
            if incomplete {
                break;
            }
            if !(value == 0x9
                || value == 0xA
                || value == 0xD
                || value >= 0x20 && value <= 0x7E
                || value == 0x85
                || value >= 0xA0 && value <= 0xD7FF
                || value >= 0xE000 && value <= 0xFFFD
                || value >= 0x10000 && value <= 0x10FFFF)
            {
                return yaml_parser_set_reader_error(
                    parser,
                    b"control characters are not allowed\0" as *const u8 as *const libc::c_char,
                    (*parser).offset,
                    value as libc::c_int,
                );
            }
            let fresh14 = addr_of_mut!((*parser).raw_buffer.pointer);
            *fresh14 = (*fresh14).wrapping_offset(width as isize);
            let fresh15 = addr_of_mut!((*parser).offset);
            *fresh15 = (*fresh15 as libc::c_ulong).force_add(width as libc::c_ulong) as size_t;
            if value <= 0x7F {
                let fresh16 = addr_of_mut!((*parser).buffer.last);
                let fresh17 = *fresh16;
                *fresh16 = (*fresh16).wrapping_offset(1);
                *fresh17 = value as yaml_char_t;
            } else if value <= 0x7FF {
                let fresh18 = addr_of_mut!((*parser).buffer.last);
                let fresh19 = *fresh18;
                *fresh18 = (*fresh18).wrapping_offset(1);
                *fresh19 = 0xC0_u32.force_add(value >> 6) as yaml_char_t;
                let fresh20 = addr_of_mut!((*parser).buffer.last);
                let fresh21 = *fresh20;
                *fresh20 = (*fresh20).wrapping_offset(1);
                *fresh21 = 0x80_u32.force_add(value & 0x3F) as yaml_char_t;
            } else if value <= 0xFFFF {
                let fresh22 = addr_of_mut!((*parser).buffer.last);
                let fresh23 = *fresh22;
                *fresh22 = (*fresh22).wrapping_offset(1);
                *fresh23 = 0xE0_u32.force_add(value >> 12) as yaml_char_t;
                let fresh24 = addr_of_mut!((*parser).buffer.last);
                let fresh25 = *fresh24;
                *fresh24 = (*fresh24).wrapping_offset(1);
                *fresh25 = 0x80_u32.force_add(value >> 6 & 0x3F) as yaml_char_t;
                let fresh26 = addr_of_mut!((*parser).buffer.last);
                let fresh27 = *fresh26;
                *fresh26 = (*fresh26).wrapping_offset(1);
                *fresh27 = 0x80_u32.force_add(value & 0x3F) as yaml_char_t;
            } else {
                let fresh28 = addr_of_mut!((*parser).buffer.last);
                let fresh29 = *fresh28;
                *fresh28 = (*fresh28).wrapping_offset(1);
                *fresh29 = 0xF0_u32.force_add(value >> 18) as yaml_char_t;
                let fresh30 = addr_of_mut!((*parser).buffer.last);
                let fresh31 = *fresh30;
                *fresh30 = (*fresh30).wrapping_offset(1);
                *fresh31 = 0x80_u32.force_add(value >> 12 & 0x3F) as yaml_char_t;
                let fresh32 = addr_of_mut!((*parser).buffer.last);
                let fresh33 = *fresh32;
                *fresh32 = (*fresh32).wrapping_offset(1);
                *fresh33 = 0x80_u32.force_add(value >> 6 & 0x3F) as yaml_char_t;
                let fresh34 = addr_of_mut!((*parser).buffer.last);
                let fresh35 = *fresh34;
                *fresh34 = (*fresh34).wrapping_offset(1);
                *fresh35 = 0x80_u32.force_add(value & 0x3F) as yaml_char_t;
            }
            let fresh36 = addr_of_mut!((*parser).unread);
            *fresh36 = (*fresh36).force_add(1);
        }
        if (*parser).eof {
            let fresh37 = addr_of_mut!((*parser).buffer.last);
            let fresh38 = *fresh37;
            *fresh37 = (*fresh37).wrapping_offset(1);
            *fresh38 = b'\0';
            let fresh39 = addr_of_mut!((*parser).unread);
            *fresh39 = (*fresh39).force_add(1);
            return OK;
        }
    }
    if (*parser).offset >= (!0_u64).wrapping_div(2_u64) {
        return yaml_parser_set_reader_error(
            parser,
            b"input is too long\0" as *const u8 as *const libc::c_char,
            (*parser).offset,
            -1,
        );
    }
    OK
}
