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
