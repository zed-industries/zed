//! Encoding logic for VEX instructions.

use crate::api::CodeSink;

/// Construct and emit the VEX prefix bytes.
pub enum VexPrefix {
    TwoByte(u8),
    ThreeByte(u8, u8),
}

/// The VEX prefix only ever uses the top bit (bit 3--the fourth bit) of any
/// HW-encoded register.
#[inline(always)]
fn invert_top_bit(enc: u8) -> u8 {
    (!(enc >> 3)) & 1
}

fn use_2byte_prefix(x: u8, b: u8, w: bool, mmmmm: u8) -> bool {
    // These bits are only represented on the 3 byte prefix, so their presence
    // implies the use of the 3 byte prefix
    b == 1 && x == 1 &&
    // The presence of W1 in the opcode column implies the opcode must be
    // encoded using the 3-byte form of the VEX prefix.
    w == false &&
    // The presence of 0F3A and 0F38 in the opcode column implies that
    // opcode can only be encoded by the three-byte form of VEX.
    !(mmmmm == 0b10 || mmmmm == 0b11)
}

impl VexPrefix {
    /// Construct the [`VexPrefix`] for a ternary instruction.
    ///
    /// Used with a single register operand:
    /// - `reg` and `vvvv` hold HW-encoded registers.
    /// - `b` and `x` hold the (optional) HW-encoded registers for the `rm`
    ///   operand.
    /// - the other fields (`l`, `pp`, `mmmmm`, `w`) correspond directly to
    ///   fields in the VEX prefix.
    #[inline]
    #[must_use]
    pub fn three_op(
        reg: u8,
        vvvv: u8,
        (b, x): (Option<u8>, Option<u8>),
        l: u8,
        pp: u8,
        mmmmm: u8,
        w: bool,
    ) -> Self {
        let r = invert_top_bit(reg);
        let b = invert_top_bit(b.unwrap_or(0));
        let x = invert_top_bit(x.unwrap_or(0));

        if use_2byte_prefix(x, b, w, mmmmm) {
            // 2-byte VEX prefix.
            //
            // +-----+ +-------------------+
            // | C5h | | R | vvvv | L | pp |
            // +-----+ +-------------------+
            debug_assert!(vvvv <= 0b1111);
            debug_assert!(l <= 0b1);
            debug_assert!(pp <= 0b11);
            let last_byte = r << 7 | (!vvvv & 0b1111) << 3 | (l & 0b1) << 2 | (pp & 0b11);

            Self::TwoByte(last_byte)
        } else {
            // 3-byte VEX prefix.
            //
            // +-----+ +--------------+ +-------------------+
            // | C4h | | RXB | m-mmmm | | W | vvvv | L | pp |
            // +-----+ +--------------+ +-------------------+
            debug_assert!(mmmmm >= 0b01 && mmmmm <= 0b11);
            let second_byte = r << 7 | x << 6 | b << 5 | mmmmm;

            debug_assert!(vvvv <= 0b1111);
            debug_assert!(l <= 0b1);
            debug_assert!(pp <= 0b11);
            let last_byte = (w as u8) << 7 | (!vvvv & 0b1111) << 3 | (l & 0b1) << 2 | (pp & 0b11);

            Self::ThreeByte(second_byte, last_byte)
        }
    }

    /// Construct the [`VexPrefix`] for a binary instruction.
    ///
    /// This simply but conveniently reuses [`VexPrefix::three_op`] with a
    /// `vvvv` value of `0`.
    #[inline]
    #[must_use]
    pub fn two_op(
        reg: u8,
        (b, x): (Option<u8>, Option<u8>),
        l: u8,
        pp: u8,
        mmmmm: u8,
        w: bool,
    ) -> Self {
        Self::three_op(reg, 0, (b, x), l, pp, mmmmm, w)
    }

    pub(crate) fn encode(&self, sink: &mut impl CodeSink) {
        match self {
            VexPrefix::TwoByte(last_byte) => {
                sink.put1(0xC5);
                sink.put1(*last_byte);
            }
            VexPrefix::ThreeByte(second_byte, last_byte) => {
                sink.put1(0xC4);
                sink.put1(*second_byte);
                sink.put1(*last_byte);
            }
        }
    }
}
