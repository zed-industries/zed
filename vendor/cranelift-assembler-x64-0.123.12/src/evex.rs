//! Encoding logic for EVEX instructions.

use crate::api::CodeSink;

/// EVEX prefix is always 4 bytes, byte 0 is 0x62
pub struct EvexPrefix {
    byte1: u8,
    byte2: u8,
    byte3: u8,
}

/// The EVEX prefix only ever uses the top bit (bit 3--the fourth bit) of any
/// HW-encoded register.
#[inline(always)]
fn invert_top_bit(enc: u8) -> u8 {
    (!(enc >> 3)) & 1
}

//         ┌───┬───┬───┬───┬───┬───┬───┬───┐
// Byte 1: │ R │ X │ B │ R'│ 0 │ 0 │ m │ m │
//         ├───┼───┼───┼───┼───┼───┼───┼───┤
// Byte 2: │ W │ v │ v │ v │ v │ 1 │ p │ p │
//         ├───┼───┼───┼───┼───┼───┼───┼───┤
// Byte 3: │ z │ L'│ L │ b │ V'│ a │ a │ a │
//         └───┴───┴───┴───┴───┴───┴───┴───┘

impl EvexPrefix {
    /// Construct the [`EvexPrefix`] for an instruction.
    pub fn new(
        reg: u8,
        vvvv: u8,
        (b, x): (Option<u8>, Option<u8>),
        ll: u8,
        pp: u8,
        mmm: u8,
        w: bool,
        broadcast: bool,
    ) -> Self {
        let r = invert_top_bit(reg);
        let r_prime = invert_top_bit(reg >> 1);
        let b = invert_top_bit(b.unwrap_or(0));
        let x = invert_top_bit(x.unwrap_or(0));
        let vvvv_value = !vvvv & 0b1111;
        let v_prime = !(vvvv >> 4) & 0b1;

        // byte1
        debug_assert!(mmm <= 0b111);
        let byte1 = r << 7 | x << 6 | b << 5 | r_prime << 4 | mmm;

        // byte2
        debug_assert!(vvvv <= 0b11111);
        debug_assert!(pp <= 0b11);
        let byte2 = (w as u8) << 7 | vvvv_value << 3 | 0b100 | (pp & 0b11);

        // byte3
        debug_assert!(ll < 0b11, "bits 11b are reserved (#UD); must fit in 2 bits");
        let aaa = 0b000; // Force k0 masking register for now; eventually this should be configurable (TODO).
        let z = 0; // Masking kind bit; not used yet (TODO) so we default to merge-masking.
        let byte3 = z | ll << 5 | (broadcast as u8) << 4 | v_prime << 3 | aaa;

        Self {
            byte1,
            byte2,
            byte3,
        }
    }

    /// Construct the [`EvexPrefix`] for an instruction.
    pub fn two_op(
        reg: u8,
        (b, x): (Option<u8>, Option<u8>),
        ll: u8,
        pp: u8,
        mmm: u8,
        w: bool,
        broadcast: bool,
    ) -> Self {
        EvexPrefix::new(reg, 0, (b, x), ll, pp, mmm, w, broadcast)
    }

    /// Construct the [`EvexPrefix`] for an instruction.
    pub fn three_op(
        reg: u8,
        vvvv: u8,
        (b, x): (Option<u8>, Option<u8>),
        ll: u8,
        pp: u8,
        mmm: u8,
        w: bool,
        broadcast: bool,
    ) -> Self {
        EvexPrefix::new(reg, vvvv, (b, x), ll, pp, mmm, w, broadcast)
    }

    pub(crate) fn encode(&self, sink: &mut impl CodeSink) {
        sink.put1(0x62);
        sink.put1(self.byte1);
        sink.put1(self.byte2);
        sink.put1(self.byte3);
    }
}
