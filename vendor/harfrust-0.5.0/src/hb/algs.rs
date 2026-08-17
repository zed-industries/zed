// FLAG macro in harfbuzz.
#[inline]
pub const fn rb_flag(x: u32) -> u32 {
    1 << x
}

// FLAG_UNSAFE macro in harfbuzz.
#[inline]
pub fn rb_flag_unsafe(x: u32) -> u32 {
    if x < 32 {
        1 << x
    } else {
        0
    }
}

// FLAG_RANGE macro in harfbuzz.
#[inline]
pub fn rb_flag_range(x: u32, y: u32) -> u32 {
    (x < y) as u32 + rb_flag(y + 1) - rb_flag(x)
}

// FLAG64 macro in harfbuzz.
#[inline]
pub const fn rb_flag64(x: u32) -> u64 {
    1 << x as u64
}

// FLAG64_UNSAFE macro in harfbuzz.
#[inline]
pub fn rb_flag64_unsafe(x: u32) -> u64 {
    if x < 64 {
        1 << (x as u64)
    } else {
        0
    }
}

/* Encodes three unsigned integers in one 64-bit number.  If the inputs have more than 21 bits,
 * values will be truncated / overlap, and might not decode exactly. */
#[allow(non_snake_case, dead_code)]
pub const fn HB_CODEPOINT_ENCODE3(x: u32, y: u32, z: u32) -> u64 {
    ((x as u64) << 42) | ((y as u64) << 21) | (z as u64)
}
#[allow(non_snake_case, dead_code)]
pub const fn HB_CODEPOINT_DECODE3_1(v: u64) -> u32 {
    (v >> 42) as u32
}
#[allow(non_snake_case, dead_code)]
pub const fn HB_CODEPOINT_DECODE3_2(v: u64) -> u32 {
    ((v >> 21) & 0x001F_FFFF) as u32
}
#[allow(non_snake_case, dead_code)]
pub const fn HB_CODEPOINT_DECODE3_3(v: u64) -> u32 {
    (v & 0x001F_FFFF) as u32
}

/* Custom encoding used by hb-ucd. */
#[allow(non_snake_case, dead_code)]
pub const fn HB_CODEPOINT_ENCODE3_11_7_14(x: u32, y: u32, z: u32) -> u32 {
    ((x & 0x07FF) << 21) | ((y & 0x007F) << 14) | (z & 0x3FFF)
}
#[allow(non_snake_case, dead_code)]
pub const fn HB_CODEPOINT_DECODE3_11_7_14_1(v: u32) -> u32 {
    v >> 21
}
#[allow(non_snake_case, dead_code)]
pub const fn HB_CODEPOINT_DECODE3_11_7_14_2(v: u32) -> u32 {
    ((v >> 14) & 0x007F) | 0x0300
}
#[allow(non_snake_case, dead_code)]
pub const fn HB_CODEPOINT_DECODE3_11_7_14_3(v: u32) -> u32 {
    v & 0x3FFF
}
