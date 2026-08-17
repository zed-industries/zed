//! Random Number Generator Protocol
//!
//! This protocol is used to provide random numbers for use in applications, or
//! entropy for seeding other random number generators.

pub const PROTOCOL_GUID: crate::base::Guid = crate::base::Guid::from_fields(
    0x3152bca5,
    0xeade,
    0x433d,
    0x86,
    0x2e,
    &[0xc0, 0x1c, 0xdc, 0x29, 0x1f, 0x44],
);

pub type Algorithm = crate::base::Guid;

pub const ALGORITHM_SP800_90_HASH_256_GUID: Algorithm = crate::base::Guid::from_fields(
    0xa7af67cb,
    0x603b,
    0x4d42,
    0xba,
    0x21,
    &[0x70, 0xbf, 0xb6, 0x29, 0x3f, 0x96],
);
pub const ALGORITHM_SP800_90_HMAC_256_GUID: Algorithm = crate::base::Guid::from_fields(
    0xc5149b43,
    0xae85,
    0x4f53,
    0x99,
    0x82,
    &[0xb9, 0x43, 0x35, 0xd3, 0xa9, 0xe7],
);
pub const ALGORITHM_SP800_90_CTR_256_GUID: Algorithm = crate::base::Guid::from_fields(
    0x44f0de6e,
    0x4d8c,
    0x4045,
    0xa8,
    0xc7,
    &[0x4d, 0xd1, 0x68, 0x85, 0x6b, 0x9e],
);
pub const ALGORITHM_X9_31_3DES_GUID: Algorithm = crate::base::Guid::from_fields(
    0x63c4785a,
    0xca34,
    0x4012,
    0xa3,
    0xc8,
    &[0x0b, 0x6a, 0x32, 0x4f, 0x55, 0x46],
);
pub const ALGORITHM_X9_31_AES_GUID: Algorithm = crate::base::Guid::from_fields(
    0xacd03321,
    0x777e,
    0x4d3d,
    0xb1,
    0xc8,
    &[0x20, 0xcf, 0xd8, 0x88, 0x20, 0xc9],
);
pub const ALGORITHM_RAW: Algorithm = crate::base::Guid::from_fields(
    0xe43176d7,
    0xb6e8,
    0x4827,
    0xb7,
    0x84,
    &[0x7f, 0xfd, 0xc4, 0xb6, 0x85, 0x61],
);

pub type ProtocolGetInfo = eficall! {fn(
    *mut Protocol,
    *mut usize,
    *mut Algorithm,
) -> crate::base::Status};

pub type ProtocolGetRng = eficall! {fn(
    *mut Protocol,
    *mut Algorithm,
    usize,
    *mut u8,
) -> crate::base::Status};

#[repr(C)]
pub struct Protocol {
    pub get_info: ProtocolGetInfo,
    pub get_rng: ProtocolGetRng,
}
