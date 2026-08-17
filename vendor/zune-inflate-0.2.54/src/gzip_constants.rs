#![cfg(feature = "gzip")]
pub const GZIP_ID1: u8 = 0x1F;
pub const GZIP_ID2: u8 = 0x8B;
pub const GZIP_CM_DEFLATE: u8 = 8;

pub const GZIP_FRESERVED: u8 = 0xE0;
pub const GZIP_FEXTRA: u8 = 0x04;

pub const GZIP_FOOTER_SIZE: usize = 8;

pub const GZIP_FHCRC: u8 = 0x02;
pub const GZIP_FNAME: u8 = 0x08;
pub const GZIP_FCOMMENT: u8 = 0x10;
