//! The various font tables

pub mod aat;
pub mod ankr;
pub mod avar;
pub mod base;
pub mod bitmap;
pub mod cbdt;
pub mod cblc;
pub mod cff;
pub mod cff2;
pub mod cmap;
pub mod colr;
pub mod cpal;
pub mod cvar;
pub mod dsig;
pub mod ebdt;
pub mod eblc;
pub mod feat;
pub mod fvar;
pub mod gasp;
pub mod gdef;
pub mod glyf;
pub mod gpos;
pub mod gsub;
pub mod gvar;
pub mod hdmx;
pub mod head;
pub mod hhea;
pub mod hmtx;
pub mod hvar;
pub mod kern;
pub mod kerx;
pub mod layout;
pub mod loca;
pub mod ltag;
pub mod maxp;
pub mod meta;
pub mod morx;
pub mod mvar;
pub mod name;
pub mod os2;
pub mod post;
pub mod postscript;
pub mod sbix;
pub mod stat;
pub mod svg;
pub mod trak;
pub mod varc;
pub mod variations;
pub mod vhea;
pub mod vmtx;
pub mod vorg;
pub mod vvar;

#[cfg(feature = "ift")]
pub mod ift;

/// Computes the table checksum for the given data.
///
/// See the OpenType [specification](https://learn.microsoft.com/en-us/typography/opentype/spec/otff#calculating-checksums)
/// for details.
pub fn compute_checksum(table: &[u8]) -> u32 {
    let mut sum = 0u32;
    let mut iter = table.chunks_exact(4);
    for quad in &mut iter {
        // this can't fail, and we trust the compiler to avoid a branch
        let array: [u8; 4] = quad.try_into().unwrap_or_default();
        sum = sum.wrapping_add(u32::from_be_bytes(array));
    }

    let rem = match *iter.remainder() {
        [a] => u32::from_be_bytes([a, 0, 0, 0]),
        [a, b] => u32::from_be_bytes([a, b, 0, 0]),
        [a, b, c] => u32::from_be_bytes([a, b, c, 0]),
        _ => 0,
    };

    sum.wrapping_add(rem)
}
