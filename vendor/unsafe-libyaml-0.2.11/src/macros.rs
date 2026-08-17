macro_rules! BUFFER_INIT {
    ($buffer:expr, $size:expr) => {{
        let start = addr_of_mut!($buffer.start);
        *start = yaml_malloc($size as size_t) as *mut yaml_char_t;
        let pointer = addr_of_mut!($buffer.pointer);
        *pointer = $buffer.start;
        let last = addr_of_mut!($buffer.last);
        *last = *pointer;
        let end = addr_of_mut!($buffer.end);
        *end = $buffer.start.wrapping_add($size as usize);
    }};
}

macro_rules! BUFFER_DEL {
    ($buffer:expr) => {{
        yaml_free($buffer.start as *mut libc::c_void);
        let end = addr_of_mut!($buffer.end);
        *end = ptr::null_mut::<yaml_char_t>();
        let pointer = addr_of_mut!($buffer.pointer);
        *pointer = *end;
        let start = addr_of_mut!($buffer.start);
        *start = *pointer;
    }};
}

macro_rules! STRING_ASSIGN {
    ($string:expr, $length:expr) => {
        yaml_string_t {
            start: $string,
            end: $string.wrapping_offset($length as isize),
            pointer: $string,
        }
    };
}

macro_rules! STRING_INIT {
    ($string:expr) => {{
        $string.start = yaml_malloc(16) as *mut yaml_char_t;
        $string.pointer = $string.start;
        $string.end = $string.start.wrapping_add(16);
        memset($string.start as *mut libc::c_void, 0, 16);
    }};
}

macro_rules! STRING_DEL {
    ($string:expr) => {{
        yaml_free($string.start as *mut libc::c_void);
        $string.end = ptr::null_mut::<yaml_char_t>();
        $string.pointer = $string.end;
        $string.start = $string.pointer;
    }};
}

macro_rules! STRING_EXTEND {
    ($string:expr) => {
        if $string.pointer.wrapping_add(5) >= $string.end {
            yaml_string_extend(
                addr_of_mut!($string.start),
                addr_of_mut!($string.pointer),
                addr_of_mut!($string.end),
            );
        }
    };
}

macro_rules! CLEAR {
    ($string:expr) => {{
        $string.pointer = $string.start;
        memset(
            $string.start as *mut libc::c_void,
            0,
            $string.end.c_offset_from($string.start) as libc::c_ulong,
        );
    }};
}

macro_rules! JOIN {
    ($string_a:expr, $string_b:expr) => {{
        yaml_string_join(
            addr_of_mut!($string_a.start),
            addr_of_mut!($string_a.pointer),
            addr_of_mut!($string_a.end),
            addr_of_mut!($string_b.start),
            addr_of_mut!($string_b.pointer),
            addr_of_mut!($string_b.end),
        );
        $string_b.pointer = $string_b.start;
    }};
}

macro_rules! CHECK_AT {
    ($string:expr, $octet:expr, $offset:expr) => {
        *$string.pointer.offset($offset as isize) == $octet
    };
}

macro_rules! CHECK {
    ($string:expr, $octet:expr) => {
        *$string.pointer == $octet
    };
}

macro_rules! IS_ALPHA {
    ($string:expr) => {
        *$string.pointer >= b'0' && *$string.pointer <= b'9'
            || *$string.pointer >= b'A' && *$string.pointer <= b'Z'
            || *$string.pointer >= b'a' && *$string.pointer <= b'z'
            || *$string.pointer == b'_'
            || *$string.pointer == b'-'
    };
}

macro_rules! IS_DIGIT {
    ($string:expr) => {
        *$string.pointer >= b'0' && *$string.pointer <= b'9'
    };
}

macro_rules! AS_DIGIT {
    ($string:expr) => {
        (*$string.pointer - b'0') as libc::c_int
    };
}

macro_rules! IS_HEX_AT {
    ($string:expr, $offset:expr) => {
        *$string.pointer.wrapping_offset($offset) >= b'0'
            && *$string.pointer.wrapping_offset($offset) <= b'9'
            || *$string.pointer.wrapping_offset($offset) >= b'A'
                && *$string.pointer.wrapping_offset($offset) <= b'F'
            || *$string.pointer.wrapping_offset($offset) >= b'a'
                && *$string.pointer.wrapping_offset($offset) <= b'f'
    };
}

macro_rules! AS_HEX_AT {
    ($string:expr, $offset:expr) => {
        if *$string.pointer.wrapping_offset($offset) >= b'A'
            && *$string.pointer.wrapping_offset($offset) <= b'F'
        {
            *$string.pointer.wrapping_offset($offset) - b'A' + 10
        } else if *$string.pointer.wrapping_offset($offset) >= b'a'
            && *$string.pointer.wrapping_offset($offset) <= b'f'
        {
            *$string.pointer.wrapping_offset($offset) - b'a' + 10
        } else {
            *$string.pointer.wrapping_offset($offset) - b'0'
        } as libc::c_int
    };
}

macro_rules! IS_ASCII {
    ($string:expr) => {
        *$string.pointer <= b'\x7F'
    };
}

macro_rules! IS_PRINTABLE {
    ($string:expr) => {
        match *$string.pointer {
            // ASCII
            0x0A | 0x20..=0x7E => true,
            // U+A0 ... U+BF
            0xC2 => match *$string.pointer.wrapping_offset(1) {
                0xA0..=0xBF => true,
                _ => false,
            },
            // U+C0 ... U+CFFF
            0xC3..=0xEC => true,
            // U+D000 ... U+D7FF
            0xED => match *$string.pointer.wrapping_offset(1) {
                0x00..=0x9F => true,
                _ => false,
            },
            // U+E000 ... U+EFFF
            0xEE => true,
            // U+F000 ... U+FFFD
            0xEF => match *$string.pointer.wrapping_offset(1) {
                0xBB => match *$string.pointer.wrapping_offset(2) {
                    // except U+FEFF
                    0xBF => false,
                    _ => true,
                },
                0xBF => match *$string.pointer.wrapping_offset(2) {
                    0xBE | 0xBF => false,
                    _ => true,
                },
                _ => true,
            },
            // U+10000 ... U+10FFFF
            0xF0..=0xF4 => true,
            _ => false,
        }
    };
}

macro_rules! IS_Z_AT {
    ($string:expr, $offset:expr) => {
        CHECK_AT!($string, b'\0', $offset)
    };
}

macro_rules! IS_Z {
    ($string:expr) => {
        IS_Z_AT!($string, 0)
    };
}

macro_rules! IS_BOM {
    ($string:expr) => {
        CHECK_AT!($string, b'\xEF', 0)
            && CHECK_AT!($string, b'\xBB', 1)
            && CHECK_AT!($string, b'\xBF', 2)
    };
}

macro_rules! IS_SPACE_AT {
    ($string:expr, $offset:expr) => {
        CHECK_AT!($string, b' ', $offset)
    };
}

macro_rules! IS_SPACE {
    ($string:expr) => {
        IS_SPACE_AT!($string, 0)
    };
}

macro_rules! IS_TAB_AT {
    ($string:expr, $offset:expr) => {
        CHECK_AT!($string, b'\t', $offset)
    };
}

macro_rules! IS_TAB {
    ($string:expr) => {
        IS_TAB_AT!($string, 0)
    };
}

macro_rules! IS_BLANK_AT {
    ($string:expr, $offset:expr) => {
        IS_SPACE_AT!($string, $offset) || IS_TAB_AT!($string, $offset)
    };
}

macro_rules! IS_BLANK {
    ($string:expr) => {
        IS_BLANK_AT!($string, 0)
    };
}

macro_rules! IS_BREAK_AT {
    ($string:expr, $offset:expr) => {
        CHECK_AT!($string, b'\r', $offset)
            || CHECK_AT!($string, b'\n', $offset)
            || CHECK_AT!($string, b'\xC2', $offset) && CHECK_AT!($string, b'\x85', $offset + 1)
            || CHECK_AT!($string, b'\xE2', $offset)
                && CHECK_AT!($string, b'\x80', $offset + 1)
                && CHECK_AT!($string, b'\xA8', $offset + 2)
            || CHECK_AT!($string, b'\xE2', $offset)
                && CHECK_AT!($string, b'\x80', $offset + 1)
                && CHECK_AT!($string, b'\xA9', $offset + 2)
    };
}

macro_rules! IS_BREAK {
    ($string:expr) => {
        IS_BREAK_AT!($string, 0)
    };
}

macro_rules! IS_CRLF {
    ($string:expr) => {
        CHECK_AT!($string, b'\r', 0) && CHECK_AT!($string, b'\n', 1)
    };
}

macro_rules! IS_BREAKZ_AT {
    ($string:expr, $offset:expr) => {
        IS_BREAK_AT!($string, $offset) || IS_Z_AT!($string, $offset)
    };
}

macro_rules! IS_BREAKZ {
    ($string:expr) => {
        IS_BREAKZ_AT!($string, 0)
    };
}

macro_rules! IS_BLANKZ_AT {
    ($string:expr, $offset:expr) => {
        IS_BLANK_AT!($string, $offset) || IS_BREAKZ_AT!($string, $offset)
    };
}

macro_rules! IS_BLANKZ {
    ($string:expr) => {
        IS_BLANKZ_AT!($string, 0)
    };
}

macro_rules! WIDTH_AT {
    ($string:expr, $offset:expr) => {
        if *$string.pointer.wrapping_offset($offset as isize) & 0x80 == 0x00 {
            1
        } else if *$string.pointer.wrapping_offset($offset as isize) & 0xE0 == 0xC0 {
            2
        } else if *$string.pointer.wrapping_offset($offset as isize) & 0xF0 == 0xE0 {
            3
        } else if *$string.pointer.wrapping_offset($offset as isize) & 0xF8 == 0xF0 {
            4
        } else {
            0
        }
    };
}

macro_rules! WIDTH {
    ($string:expr) => {
        WIDTH_AT!($string, 0)
    };
}

macro_rules! MOVE {
    ($string:expr) => {
        $string.pointer = $string.pointer.wrapping_offset(WIDTH!($string) as isize)
    };
}

macro_rules! COPY {
    ($string_a:expr, $string_b:expr) => {
        if *$string_b.pointer & 0x80 == 0x00 {
            *$string_a.pointer = *$string_b.pointer;
            $string_a.pointer = $string_a.pointer.wrapping_offset(1);
            $string_b.pointer = $string_b.pointer.wrapping_offset(1);
        } else if *$string_b.pointer & 0xE0 == 0xC0 {
            *$string_a.pointer = *$string_b.pointer;
            $string_a.pointer = $string_a.pointer.wrapping_offset(1);
            $string_b.pointer = $string_b.pointer.wrapping_offset(1);
            *$string_a.pointer = *$string_b.pointer;
            $string_a.pointer = $string_a.pointer.wrapping_offset(1);
            $string_b.pointer = $string_b.pointer.wrapping_offset(1);
        } else if *$string_b.pointer & 0xF0 == 0xE0 {
            *$string_a.pointer = *$string_b.pointer;
            $string_a.pointer = $string_a.pointer.wrapping_offset(1);
            $string_b.pointer = $string_b.pointer.wrapping_offset(1);
            *$string_a.pointer = *$string_b.pointer;
            $string_a.pointer = $string_a.pointer.wrapping_offset(1);
            $string_b.pointer = $string_b.pointer.wrapping_offset(1);
            *$string_a.pointer = *$string_b.pointer;
            $string_a.pointer = $string_a.pointer.wrapping_offset(1);
            $string_b.pointer = $string_b.pointer.wrapping_offset(1);
        } else if *$string_b.pointer & 0xF8 == 0xF0 {
            *$string_a.pointer = *$string_b.pointer;
            $string_a.pointer = $string_a.pointer.wrapping_offset(1);
            $string_b.pointer = $string_b.pointer.wrapping_offset(1);
            *$string_a.pointer = *$string_b.pointer;
            $string_a.pointer = $string_a.pointer.wrapping_offset(1);
            $string_b.pointer = $string_b.pointer.wrapping_offset(1);
            *$string_a.pointer = *$string_b.pointer;
            $string_a.pointer = $string_a.pointer.wrapping_offset(1);
            $string_b.pointer = $string_b.pointer.wrapping_offset(1);
            *$string_a.pointer = *$string_b.pointer;
            $string_a.pointer = $string_a.pointer.wrapping_offset(1);
            $string_b.pointer = $string_b.pointer.wrapping_offset(1);
        }
    };
}

macro_rules! STACK_INIT {
    ($stack:expr, $type:ty) => {{
        $stack.start = yaml_malloc(16 * size_of::<$type>() as libc::c_ulong) as *mut $type;
        $stack.top = $stack.start;
        $stack.end = $stack.start.offset(16_isize);
    }};
}

macro_rules! STACK_DEL {
    ($stack:expr) => {
        yaml_free($stack.start as *mut libc::c_void);
        $stack.end = ptr::null_mut();
        $stack.top = ptr::null_mut();
        $stack.start = ptr::null_mut();
    };
}

macro_rules! STACK_EMPTY {
    ($stack:expr) => {
        $stack.start == $stack.top
    };
}

macro_rules! STACK_LIMIT {
    ($context:expr, $stack:expr) => {
        if $stack.top.c_offset_from($stack.start) < libc::c_int::MAX as isize - 1 {
            OK
        } else {
            (*$context).error = YAML_MEMORY_ERROR;
            FAIL
        }
    };
}

macro_rules! PUSH {
    (do $stack:expr, $push:expr) => {{
        if $stack.top == $stack.end {
            yaml_stack_extend(
                addr_of_mut!($stack.start) as *mut *mut libc::c_void,
                addr_of_mut!($stack.top) as *mut *mut libc::c_void,
                addr_of_mut!($stack.end) as *mut *mut libc::c_void,
            );
        }
        $push;
        $stack.top = $stack.top.wrapping_offset(1);
    }};
    ($stack:expr, *$value:expr) => {
        PUSH!(do $stack, ptr::copy_nonoverlapping($value, $stack.top, 1))
    };
    ($stack:expr, $value:expr) => {
        PUSH!(do $stack, ptr::write($stack.top, $value))
    };
}

macro_rules! POP {
    ($stack:expr) => {
        *{
            $stack.top = $stack.top.offset(-1);
            $stack.top
        }
    };
}

macro_rules! QUEUE_INIT {
    ($queue:expr, $type:ty) => {{
        $queue.start = yaml_malloc(16 * size_of::<$type>() as libc::c_ulong) as *mut $type;
        $queue.tail = $queue.start;
        $queue.head = $queue.tail;
        $queue.end = $queue.start.offset(16_isize);
    }};
}

macro_rules! QUEUE_DEL {
    ($queue:expr) => {
        yaml_free($queue.start as *mut libc::c_void);
        $queue.end = ptr::null_mut();
        $queue.tail = ptr::null_mut();
        $queue.head = ptr::null_mut();
        $queue.start = ptr::null_mut();
    };
}

macro_rules! QUEUE_EMPTY {
    ($queue:expr) => {
        $queue.head == $queue.tail
    };
}

macro_rules! ENQUEUE {
    (do $queue:expr, $enqueue:expr) => {{
        if $queue.tail == $queue.end {
            yaml_queue_extend(
                addr_of_mut!($queue.start) as *mut *mut libc::c_void,
                addr_of_mut!($queue.head) as *mut *mut libc::c_void,
                addr_of_mut!($queue.tail) as *mut *mut libc::c_void,
                addr_of_mut!($queue.end) as *mut *mut libc::c_void,
            );
        }
        $enqueue;
        $queue.tail = $queue.tail.wrapping_offset(1);
    }};
    ($queue:expr, *$value:expr) => {
        ENQUEUE!(do $queue, ptr::copy_nonoverlapping($value, $queue.tail, 1))
    };
    ($queue:expr, $value:expr) => {
        ENQUEUE!(do $queue, ptr::write($queue.tail, $value))
    };
}

macro_rules! DEQUEUE {
    ($queue:expr) => {
        *{
            let head = $queue.head;
            $queue.head = $queue.head.wrapping_offset(1);
            head
        }
    };
}

macro_rules! QUEUE_INSERT {
    ($queue:expr, $index:expr, $value:expr) => {{
        if $queue.tail == $queue.end {
            yaml_queue_extend(
                addr_of_mut!($queue.start) as *mut *mut libc::c_void,
                addr_of_mut!($queue.head) as *mut *mut libc::c_void,
                addr_of_mut!($queue.tail) as *mut *mut libc::c_void,
                addr_of_mut!($queue.end) as *mut *mut libc::c_void,
            );
        }
        memmove(
            $queue
                .head
                .wrapping_offset($index as isize)
                .wrapping_offset(1_isize) as *mut libc::c_void,
            $queue.head.wrapping_offset($index as isize) as *const libc::c_void,
            ($queue.tail.c_offset_from($queue.head) as libc::c_ulong)
                .wrapping_sub($index)
                .wrapping_mul(size_of::<yaml_token_t>() as libc::c_ulong),
        );
        *$queue.head.wrapping_offset($index as isize) = $value;
        let fresh14 = addr_of_mut!($queue.tail);
        *fresh14 = (*fresh14).wrapping_offset(1);
    }};
}
