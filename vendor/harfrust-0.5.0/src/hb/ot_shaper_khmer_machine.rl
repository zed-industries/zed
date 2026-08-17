#![allow(
    dead_code,
    non_upper_case_globals,
    unused_assignments,
    unused_parens,
    while_true,
    clippy::assign_op_pattern,
    clippy::collapsible_if,
    clippy::comparison_chain,
    clippy::double_parens,
    clippy::unnecessary_cast,
    clippy::single_match,
    clippy::never_loop
)]

use super::buffer::{HB_BUFFER_SCRATCH_FLAG_HAS_BROKEN_SYLLABLE, hb_buffer_t};

%%{
  machine khmer_syllable_machine;
  alphtype u8;
  write data;
}%%

// IMPORTANT: Before updating any values here, make sure to read the comment in `ot_category_t`.
%%{

# We use category H for spec category Coeng


C    = 1;
V    = 2;
H    = 4;
ZWNJ = 5;
ZWJ  = 6;
PLACEHOLDER = 10;
DOTTEDCIRCLE = 11;
Ra   = 15;

VAbv = 20;
VBlw = 21;
VPre = 22;
VPst = 23;

Robatic = 25;
Xgroup  = 26;
Ygroup  = 27;

c = (C | Ra | V);
cn = c.((ZWJ|ZWNJ)?.Robatic)?;
joiner = (ZWJ | ZWNJ);
xgroup = (joiner*.Xgroup)*;
ygroup = Ygroup*;

# This grammar was experimentally extracted from what Uniscribe allows.

matra_group = VPre? xgroup VBlw? xgroup (joiner?.VAbv)? xgroup VPst?;
syllable_tail = xgroup matra_group xgroup (H.c)? ygroup;


broken_cluster =	Robatic? (H.cn)* (H | syllable_tail);
consonant_syllable =	(cn|PLACEHOLDER|DOTTEDCIRCLE) broken_cluster;
other =			any;

main := |*
	consonant_syllable	=> { found_syllable!(SyllableType::ConsonantSyllable); };
	broken_cluster		=> { found_syllable!(SyllableType::BrokenCluster); buffer.scratch_flags |= HB_BUFFER_SCRATCH_FLAG_HAS_BROKEN_SYLLABLE; };
	other			=> { found_syllable!(SyllableType::NonKhmerCluster); };
*|;


}%%

#[derive(Clone, Copy)]
pub enum SyllableType {
    ConsonantSyllable = 0,
    BrokenCluster,
    NonKhmerCluster,
}

pub fn find_syllables_khmer(buffer: &mut hb_buffer_t) {
    let mut cs = 0;
    let mut ts = 0;
    let mut te = 0;
    let mut act = 0;
    let mut p = 0;
    let pe = buffer.len;
    let eof = buffer.len;
    let mut syllable_serial = 1u8;

    macro_rules! found_syllable {
        ($kind:expr) => {{
            found_syllable(ts, te, &mut syllable_serial, $kind, buffer);
        }}
    }

    %%{
        write init;
        getkey (buffer.info[p].khmer_category() as u8);
        write exec;
    }%%
}

#[inline]
fn found_syllable(
    start: usize,
    end: usize,
    syllable_serial: &mut u8,
    kind: SyllableType,
    buffer: &mut hb_buffer_t,
) {
    for i in start..end {
        buffer.info[i].set_syllable((*syllable_serial << 4) | kind as u8);
    }

    *syllable_serial += 1;

    if *syllable_serial == 16 {
        *syllable_serial = 1;
    }
}
