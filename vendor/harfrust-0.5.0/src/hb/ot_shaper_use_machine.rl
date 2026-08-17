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

use core::cell::Cell;

use super::buffer::{hb_buffer_t, HB_BUFFER_SCRATCH_FLAG_HAS_BROKEN_SYLLABLE};
use super::GlyphInfo;
use super::machine_cursor::MachineCursor;
use super::ot_shaper_use::category;

%%{
  machine use_syllable_machine;
  alphtype u8;
  write data;
}%%

%%{

# Categories used in the Universal Shaping Engine spec:
# https://docs.microsoft.com/en-us/typography/script-development/use

O	= 0; # OTHER

B	= 1; # BASE
N	= 4; # BASE_NUM
GB	= 5; # BASE_OTHER
CGJ	= 6; # CGJ
SUB	= 11; # CONS_SUB
H	= 12; # HALANT

HN	= 13; # HALANT_NUM
ZWNJ	= 14; # Zero width non-joiner
WJ = 16; # Word joiner
R	= 18; # REPHA
CS	= 43; # CONS_WITH_STACKER
IS	= 44; # HALANT_OR_VOWEL_MODIFIER
Sk	= 48; # SAKOT
G	= 49; # HIEROGLYPH
J	= 50; # HIEROGLYPH_JOINER
SB	= 51; # HIEROGLYPH_SEGMENT_BEGIN
SE	= 52; # HIEROGLYPH_SEGMENT_END
HVM	= 53; # HALANT_OR_VOWEL_MODIFIER
HM	= 54; # HIEROGLYPH_MOD
HR	= 55; # HIEROGLYPH_MIRROR
RK	= 56; # REORDERING_KILLER

FAbv	= 24; # CONS_FINAL_ABOVE
FBlw	= 25; # CONS_FINAL_BELOW
FPst	= 26; # CONS_FINAL_POST
MAbv	= 27; # CONS_MED_ABOVE
MBlw	= 28; # CONS_MED_BELOW
MPst	= 29; # CONS_MED_POST
MPre	= 30; # CONS_MED_PRE
CMAbv	= 31; # CONS_MOD_ABOVE
CMBlw	= 32; # CONS_MOD_BELOW
VAbv	= 33; # VOWEL_ABOVE / VOWEL_ABOVE_BELOW / VOWEL_ABOVE_BELOW_POST / VOWEL_ABOVE_POST
VBlw	= 34; # VOWEL_BELOW / VOWEL_BELOW_POST
VPst	= 35; # VOWEL_POST	UIPC = Right
VPre	= 22; # VOWEL_PRE / VOWEL_PRE_ABOVE / VOWEL_PRE_ABOVE_POST / VOWEL_PRE_POST
VMAbv	= 37; # VOWEL_MOD_ABOVE
VMBlw	= 38; # VOWEL_MOD_BELOW
VMPst	= 39; # VOWEL_MOD_POST
VMPre	= 23; # VOWEL_MOD_PRE
SMAbv	= 41; # SYM_MOD_ABOVE
SMBlw	= 42; # SYM_MOD_BELOW
FMAbv	= 45; # CONS_FINAL_MOD	UIPC = Top
FMBlw	= 46; # CONS_FINAL_MOD	UIPC = Bottom
FMPst	= 47; # CONS_FINAL_MOD	UIPC = Not_Applicable

h = H | HVM | IS | Sk;

consonant_modifiers = CMAbv* CMBlw* ((h B | SUB) CMAbv* CMBlw*)*;
medial_consonants = MPre? MAbv? MBlw? MPst?;
dependent_vowels = VPre* VAbv* VBlw* VPst* | H;
vowel_modifiers = HVM? VMPre* VMAbv* VMBlw* VMPst*;
final_consonants = FAbv* FBlw* FPst*;
final_modifiers = FMAbv* FMBlw* | FMPst?;

complex_syllable_start = (R | CS)? (B | GB);
complex_syllable_middle =
	consonant_modifiers
	medial_consonants
	dependent_vowels
	vowel_modifiers
	(Sk B)*
;
complex_syllable_tail =
	complex_syllable_middle
	final_consonants
	final_modifiers
;
number_joiner_terminated_cluster_tail = (HN N)* HN;
numeral_cluster_tail = (HN N)+;
symbol_cluster_tail = SMAbv+ SMBlw* | SMBlw+;

virama_terminated_cluster_tail =
	consonant_modifiers
	(IS | RK)
;
virama_terminated_cluster =
	complex_syllable_start
	virama_terminated_cluster_tail
;
sakot_terminated_cluster_tail =
	complex_syllable_middle
	Sk
;
sakot_terminated_cluster =
	complex_syllable_start
	sakot_terminated_cluster_tail
;
standard_cluster =
	complex_syllable_start
	complex_syllable_tail
;
tail = complex_syllable_tail | sakot_terminated_cluster_tail | symbol_cluster_tail | virama_terminated_cluster_tail;
broken_cluster =
	R?
	(tail | number_joiner_terminated_cluster_tail | numeral_cluster_tail)
;

number_joiner_terminated_cluster = N number_joiner_terminated_cluster_tail;
numeral_cluster = N numeral_cluster_tail?;
symbol_cluster = (O | GB | SB) tail?;
hieroglyph_cluster = SB* G HR? HM? SE* (J SB* (G HR? HM? SE*)?)*;
other = any;

main := |*
	virama_terminated_cluster ZWNJ?		=> { found_syllable!(SyllableType::ViramaTerminatedCluster); };
	sakot_terminated_cluster ZWNJ?		=> { found_syllable!(SyllableType::SakotTerminatedCluster); };
	standard_cluster ZWNJ?			=> { found_syllable!(SyllableType::StandardCluster); };
	number_joiner_terminated_cluster ZWNJ?	=> { found_syllable!(SyllableType::NumberJoinerTerminatedCluster); };
	numeral_cluster ZWNJ?			=> { found_syllable!(SyllableType::NumeralCluster); };
	symbol_cluster ZWNJ?			=> { found_syllable!(SyllableType::SymbolCluster); };
	hieroglyph_cluster ZWNJ?		=> { found_syllable! (SyllableType::HieroglyphCluster); };
	FMPst					=> { found_syllable!(SyllableType::NonCluster); };
	broken_cluster ZWNJ?			=> { found_syllable!(SyllableType::BrokenCluster); buffer.scratch_flags |= HB_BUFFER_SCRATCH_FLAG_HAS_BROKEN_SYLLABLE; };
	other					=> { found_syllable!(SyllableType::NonCluster); };
*|;


}%%

#[derive(Clone, Copy)]
pub enum SyllableType {
    IndependentCluster,
    ViramaTerminatedCluster,
    SakotTerminatedCluster,
    StandardCluster,
    NumberJoinerTerminatedCluster,
    NumeralCluster,
    SymbolCluster,
    HieroglyphCluster,
    BrokenCluster,
    NonCluster,
}

pub fn find_syllables(buffer: &mut hb_buffer_t) {
    let mut cs = 0;
    let infos = Cell::as_slice_of_cells(Cell::from_mut(&mut buffer.info));
    let p0 = MachineCursor::new(infos, included);
    let mut p = p0;
    let mut ts = p0;
    let mut te = p0;
    let mut act = 0;
    let pe = p.end();
    let eof = p.end();
    let mut syllable_serial = 1u8;

    // Please manually replace assignments of 0 to p, ts, and te
    // to use p0 instead

    macro_rules! found_syllable {
        ($kind:expr) => {{
            found_syllable(ts.index(), te.index(), &mut syllable_serial, $kind, infos);
        }}
    }

    %%{
        write init;
        getkey (infos[p.index()].get().use_category() as u8);
        write exec;
    }%%
}

#[inline]
fn found_syllable(
    start: usize,
    end: usize,
    syllable_serial: &mut u8,
    kind: SyllableType,
    buffer: &[Cell<GlyphInfo>],
) {
    for i in start..end {
        let mut glyph = buffer[i].get();
        glyph.set_syllable((*syllable_serial << 4) | kind as u8);
        buffer[i].set(glyph);
    }

    *syllable_serial += 1;

    if *syllable_serial == 16 {
        *syllable_serial = 1;
    }
}

fn not_ccs_default_ignorable(i: &GlyphInfo) -> bool {
    i.use_category() != category::CGJ
}

fn included(infos: &[Cell<GlyphInfo>], i: usize) -> bool {
    let glyph = infos[i].get();
    if !not_ccs_default_ignorable(&glyph) {
        return false;
    }
    if glyph.use_category() == category::ZWNJ {
        for glyph2 in &infos[i + 1..] {
            if not_ccs_default_ignorable(&glyph2.get()) {
                return !glyph2.get().is_unicode_mark();
            }
        }
    }
    true
}
