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
  machine myanmar_syllable_machine;
  alphtype u8;
  write data;
}%%

// IMPORTANT: Before updating any values here, make sure to read the comment in `ot_category_t`.
%%{

# Spec category D is folded into GB; D0 is not implemented by Uniscribe and as such folded into D
# Spec category P is folded into GB

C    = 1;
IV   = 2;
DB   = 3;	# Dot below	     = OT_N
H    = 4;
ZWNJ = 5;
ZWJ  = 6;
SM    = 8;	# Visarga and Shan tones
GB   = 10;	# 		     = OT_PLACEHOLDER
DOTTEDCIRCLE    = 11;
A    = 9;
Ra   = 15;
CS   = 18;
SMPst = 57;

VAbv = 20;
VBlw = 21;
VPre = 22;
VPst = 23;

# 32+ are for Myanmar-specific values
As   = 32;	# Asat
MH   = 35;	# Medial Ha
MR   = 36;	# Medial Ra
MW   = 37;	# Medial Wa, Shan Wa
MY   = 38;	# Medial Ya, Mon Na, Mon Ma
PT   = 39;	# Pwo and other tones
VS   = 40;	# Variation selectors
ML   = 41;	# Medial Mon La

j = ZWJ|ZWNJ;			# Joiners
k = (Ra As H);			# Kinzi

sm = SM | SMPst;
c = C|Ra;			# is_consonant

medial_group = MY? As? MR? ((MW MH? ML? | MH ML? | ML) As?)?;
main_vowel_group = (VPre.VS?)* VAbv* VBlw* A* (DB As?)?;
post_vowel_group = VPst MH? ML? As* VAbv* A* (DB As?)?;
tone_group = sm | PT A* DB? As?;

complex_syllable_tail = As* medial_group main_vowel_group post_vowel_group* tone_group* j?;
syllable_tail = (H (c|IV).VS?)* (H | complex_syllable_tail);

consonant_syllable =	(k|CS)? (c|IV|GB|DOTTEDCIRCLE).VS? syllable_tail;
broken_cluster =	k? VS? syllable_tail;
other =			any;

main := |*
	consonant_syllable	=> { found_syllable!(SyllableType::ConsonantSyllable); };
	j | SMPst		=> { found_syllable!(SyllableType::NonMyanmarCluster); };
	broken_cluster		=> { found_syllable!(SyllableType::BrokenCluster); buffer.scratch_flags |= HB_BUFFER_SCRATCH_FLAG_HAS_BROKEN_SYLLABLE; };
	other			=> { found_syllable!(SyllableType::NonMyanmarCluster); };
*|;


}%%

#[derive(Clone, Copy)]
pub enum SyllableType {
    ConsonantSyllable = 0,
    PunctuationCluster,
    BrokenCluster,
    NonMyanmarCluster,
}

pub fn find_syllables_myanmar(buffer: &mut hb_buffer_t) {
    let mut cs = 0;
    let mut ts = 0;
    let mut te;
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
        getkey (buffer.info[p].myanmar_category() as u8);
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
