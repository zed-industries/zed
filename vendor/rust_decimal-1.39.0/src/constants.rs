// Sign mask for the flags field. A value of zero in this bit indicates a
// positive Decimal value, and a value of one in this bit indicates a
// negative Decimal value.
pub const SIGN_MASK: u32 = 0x8000_0000;
pub const UNSIGN_MASK: u32 = 0x4FFF_FFFF;

// Scale mask for the flags field. This byte in the flags field contains
// the power of 10 to divide the Decimal value by. The scale byte must
// contain a value between 0 and 28 inclusive.
pub const SCALE_MASK: u32 = 0x00FF_0000;
pub const U8_MASK: u32 = 0x0000_00FF;
pub const U32_MASK: u64 = u32::MAX as _;

// Number of bits scale is shifted by.
pub const SCALE_SHIFT: u32 = 16;
// Number of bits sign is shifted by.
pub const SIGN_SHIFT: u32 = 31;

// The maximum string buffer size used for serialization purposes. 31 is optimal, however we align
// to the byte boundary for simplicity.
pub const MAX_STR_BUFFER_SIZE: usize = 32;

// The maximum supported [`Decimal::scale`] value
pub const MAX_SCALE: u8 = 28;
#[cfg(not(feature = "legacy-ops"))]
// u8 to i32 is infallible, therefore, this cast will never overflow
pub const MAX_SCALE_I32: i32 = MAX_SCALE as _;
// u8 to u32 is infallible, therefore, this cast will never overflow
pub const MAX_SCALE_U32: u32 = MAX_SCALE as _;
// 79,228,162,514,264,337,593,543,950,335
pub const MAX_I128_REPR: i128 = 0x0000_0000_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF;

// Fast access for 10^n where n is 0-9
pub const POWERS_10: [u32; 10] = [
    1, 10, 100, 1000, 10000, 100000, 1000000, 10000000, 100000000, 1000000000,
];
// Fast access for 10^n where n is 1-19
pub const BIG_POWERS_10: [u64; 19] = [
    10,
    100,
    1000,
    10000,
    100000,
    1000000,
    10000000,
    100000000,
    1000000000,
    10000000000,
    100000000000,
    1000000000000,
    10000000000000,
    100000000000000,
    1000000000000000,
    10000000000000000,
    100000000000000000,
    1000000000000000000,
    10000000000000000000,
];

#[cfg(not(feature = "legacy-ops"))]
// The maximum power of 10 that a 32 bit integer can store
pub const MAX_I32_SCALE: i32 = 9;
#[cfg(not(feature = "legacy-ops"))]
// The maximum power of 10 that a 64 bit integer can store
pub const MAX_I64_SCALE: u32 = 19;
#[cfg(not(feature = "legacy-ops"))]
pub const U32_MAX: u64 = u32::MAX as u64;

// Determines potential overflow for 128 bit operations
pub const OVERFLOW_U96: u128 = 1u128 << 96;
pub const WILL_OVERFLOW_U64: u64 = u64::MAX / 10 - u8::MAX as u64;
pub const BYTES_TO_OVERFLOW_U64: usize = 18; // We can probably get away with less
