use crate::ops::ForceAdd as _;
use crate::success::{Success, FAIL, OK};
use crate::yaml::size_t;
use crate::{
    libc, yaml_emitter_t, PointerExt, YAML_ANY_ENCODING, YAML_UTF16LE_ENCODING, YAML_UTF8_ENCODING,
    YAML_WRITER_ERROR,
};
use core::ptr::addr_of_mut;

unsafe fn yaml_emitter_set_writer_error(
    emitter: *mut yaml_emitter_t,
    problem: *const libc::c_char,
) -> Success {
    (*emitter).error = YAML_WRITER_ERROR;
    let fresh0 = addr_of_mut!((*emitter).problem);
    *fresh0 = problem;
    FAIL
}

/// Flush the accumulated characters to the output.
pub unsafe fn yaml_emitter_flush(emitter: *mut yaml_emitter_t) -> Success {
    __assert!(!emitter.is_null());
    __assert!(((*emitter).write_handler).is_some());
    __assert!((*emitter).encoding != YAML_ANY_ENCODING);
    let fresh1 = addr_of_mut!((*emitter).buffer.last);
    *fresh1 = (*emitter).buffer.pointer;
    let fresh2 = addr_of_mut!((*emitter).buffer.pointer);
    *fresh2 = (*emitter).buffer.start;
    if (*emitter).buffer.start == (*emitter).buffer.last {
        return OK;
    }
    if (*emitter).encoding == YAML_UTF8_ENCODING {
        if (*emitter).write_handler.expect("non-null function pointer")(
            (*emitter).write_handler_data,
            (*emitter).buffer.start,
            (*emitter)
                .buffer
                .last
                .c_offset_from((*emitter).buffer.start) as size_t,
        ) != 0
        {
            let fresh3 = addr_of_mut!((*emitter).buffer.last);
            *fresh3 = (*emitter).buffer.start;
            let fresh4 = addr_of_mut!((*emitter).buffer.pointer);
            *fresh4 = (*emitter).buffer.start;
            return OK;
        } else {
            return yaml_emitter_set_writer_error(
                emitter,
                b"write error\0" as *const u8 as *const libc::c_char,
            );
        }
    }
    let low: libc::c_int = if (*emitter).encoding == YAML_UTF16LE_ENCODING {
        0
    } else {
        1
    };
    let high: libc::c_int = if (*emitter).encoding == YAML_UTF16LE_ENCODING {
        1
    } else {
        0
    };
    while (*emitter).buffer.pointer != (*emitter).buffer.last {
        let mut octet: libc::c_uchar;
        let mut value: libc::c_uint;
        let mut k: size_t;
        octet = *(*emitter).buffer.pointer;
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
        k = 1_u64;
        while k < width as libc::c_ulong {
            octet = *(*emitter).buffer.pointer.wrapping_offset(k as isize);
            value = (value << 6).force_add((octet & 0x3F) as libc::c_uint);
            k = k.force_add(1);
        }
        let fresh5 = addr_of_mut!((*emitter).buffer.pointer);
        *fresh5 = (*fresh5).wrapping_offset(width as isize);
        if value < 0x10000 {
            *(*emitter).raw_buffer.last.wrapping_offset(high as isize) =
                (value >> 8) as libc::c_uchar;
            *(*emitter).raw_buffer.last.wrapping_offset(low as isize) =
                (value & 0xFF) as libc::c_uchar;
            let fresh6 = addr_of_mut!((*emitter).raw_buffer.last);
            *fresh6 = (*fresh6).wrapping_offset(2_isize);
        } else {
            value = value.wrapping_sub(0x10000);
            *(*emitter).raw_buffer.last.wrapping_offset(high as isize) =
                0xD8_u32.force_add(value >> 18) as libc::c_uchar;
            *(*emitter).raw_buffer.last.wrapping_offset(low as isize) =
                (value >> 10 & 0xFF) as libc::c_uchar;
            *(*emitter)
                .raw_buffer
                .last
                .wrapping_offset((high + 2) as isize) =
                0xDC_u32.force_add(value >> 8 & 0xFF) as libc::c_uchar;
            *(*emitter)
                .raw_buffer
                .last
                .wrapping_offset((low + 2) as isize) = (value & 0xFF) as libc::c_uchar;
            let fresh7 = addr_of_mut!((*emitter).raw_buffer.last);
            *fresh7 = (*fresh7).wrapping_offset(4_isize);
        }
    }
    if (*emitter).write_handler.expect("non-null function pointer")(
        (*emitter).write_handler_data,
        (*emitter).raw_buffer.start,
        (*emitter)
            .raw_buffer
            .last
            .c_offset_from((*emitter).raw_buffer.start) as size_t,
    ) != 0
    {
        let fresh8 = addr_of_mut!((*emitter).buffer.last);
        *fresh8 = (*emitter).buffer.start;
        let fresh9 = addr_of_mut!((*emitter).buffer.pointer);
        *fresh9 = (*emitter).buffer.start;
        let fresh10 = addr_of_mut!((*emitter).raw_buffer.last);
        *fresh10 = (*emitter).raw_buffer.start;
        let fresh11 = addr_of_mut!((*emitter).raw_buffer.pointer);
        *fresh11 = (*emitter).raw_buffer.start;
        OK
    } else {
        yaml_emitter_set_writer_error(
            emitter,
            b"write error\0" as *const u8 as *const libc::c_char,
        )
    }
}
