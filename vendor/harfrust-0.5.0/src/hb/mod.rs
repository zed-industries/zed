// Match harfbuzz code style.
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(clippy::collapsible_if)]
#![allow(clippy::collapsible_else_if)]
#![allow(clippy::comparison_chain)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::non_canonical_partial_ord_impl)]
#![allow(clippy::upper_case_acronyms)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::wildcard_in_or_patterns)]
#![allow(clippy::identity_op)]
#![allow(clippy::inline_always)]
#![allow(clippy::mut_range_bound)]
#![allow(clippy::enum_variant_names)]
#![allow(clippy::manual_range_patterns)]
#![allow(clippy::type_complexity)]
#![allow(clippy::wrong_self_convention)]
#![allow(clippy::match_like_matches_macro)]
#![allow(clippy::manual_range_contains)]

mod algs;
#[macro_use]
pub mod buffer;
mod aat;
mod cache;
mod charmap;
pub mod common;
pub mod face;
mod glyph_metrics;
mod glyph_names;
mod kerning;
mod machine_cursor;
mod ot;
mod ot_layout;
mod ot_layout_common;
mod ot_layout_gpos_table;
mod ot_layout_gsub_table;
mod ot_layout_gsubgpos;
mod ot_map;
mod ot_shape;
mod ot_shape_fallback;
mod ot_shape_normalize;
pub mod ot_shape_plan;
mod ot_shaper;
mod ot_shaper_arabic;
mod ot_shaper_arabic_table;
mod ot_shaper_hangul;
mod ot_shaper_hebrew;
mod ot_shaper_indic;
mod ot_shaper_indic_machine;
#[rustfmt::skip]
mod ot_shaper_indic_table;
mod ot_shaper_khmer;
mod ot_shaper_khmer_machine;
mod ot_shaper_myanmar;
mod ot_shaper_myanmar_machine;
mod ot_shaper_syllabic;
mod ot_shaper_thai;
mod ot_shaper_use;
mod ot_shaper_use_machine;
#[rustfmt::skip]
mod ot_shaper_use_table;
mod ot_shaper_vowel_constraints;
pub(crate) mod set_digest;
mod tables;
mod tag;
mod tag_table;
mod text_parser;
#[rustfmt::skip]
mod ucd_table;
mod unicode;

use read_fonts::types::Tag as hb_tag_t;

use self::buffer::GlyphInfo;
use self::face::hb_font_t;

type hb_mask_t = u32;

use self::common::{script, Direction, Feature, Language, Script};
