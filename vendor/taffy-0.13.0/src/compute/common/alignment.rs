//! Generic CSS alignment code that is shared between both the Flexbox and CSS Grid algorithms.
use crate::style::{AlignContent, AlignContentKeyword, AlignItems, AlignItemsKeyword, AlignmentSafety};

/// Resolve the `safe`/`unsafe` overflow-position fallback for a self-level alignment value
/// (used by `align-self` / `justify-self`-style sites and by absolutely-positioned items in
/// flex/grid). If the alignment subject overflows its alignment container and the requested
/// alignment is `safe`, fall back to logical `Start` per CSS Box Alignment
/// <https://www.w3.org/TR/css-align-3/#overflow-values>. Otherwise drop the safety modifier
/// and return the bare keyword.
#[inline]
pub(crate) fn resolve_self_alignment_safety(alignment: AlignItems, overflows: bool) -> AlignItemsKeyword {
    if matches!(alignment.safety, AlignmentSafety::Safe) && overflows {
        AlignItemsKeyword::Start
    } else {
        alignment.keyword
    }
}

/// Resolve any spec-defined fallbacks for the given [`AlignContent`] value, returning the
/// bare position keyword the alignment math should use.
///
/// In addition to the spec at <https://www.w3.org/TR/css-align-3/> this implementation follows
/// the resolution of <https://github.com/w3c/csswg-drafts/issues/10154>.
pub(crate) fn apply_alignment_fallback(
    free_space: f32,
    num_items: usize,
    alignment_mode: AlignContent,
) -> AlignContentKeyword {
    let mut keyword = alignment_mode.keyword;
    let mut is_safe = matches!(alignment_mode.safety, AlignmentSafety::Safe);

    // 1. If there is only a single item being aligned or the items overflow the container, the
    //    distributed alignment keywords (`stretch`, `space-*`) fall back to a positional keyword
    //    and gain implicit `safe` semantics so step 2 can flip them to `Start` on overflow.
    //    https://www.w3.org/TR/css-align-3/#distribution-values
    if num_items <= 1 || free_space <= 0.0 {
        (keyword, is_safe) = match keyword {
            AlignContentKeyword::Stretch | AlignContentKeyword::SpaceBetween => (AlignContentKeyword::FlexStart, true),
            AlignContentKeyword::SpaceAround | AlignContentKeyword::SpaceEvenly => (AlignContentKeyword::Center, true),
            other => (other, is_safe),
        };
    }

    // 2. Safe alignment falls back to `Start` whenever the alignment subject would overflow the
    //    alignment container.
    if free_space <= 0.0 && is_safe {
        keyword = AlignContentKeyword::Start;
    }

    keyword
}

/// Generic alignment function that is used:
///   - For both align-content and justify-content alignment
///   - For both the Flexbox and CSS Grid algorithms
///
/// CSS Grid does not apply gaps as part of alignment, so the gap parameter should
/// always be set to zero for CSS Grid.
pub(crate) fn compute_alignment_offset(
    free_space: f32,
    num_items: usize,
    gap: f32,
    alignment_mode: AlignContentKeyword,
    layout_is_flex_reversed: bool,
    is_first: bool,
) -> f32 {
    if is_first {
        match alignment_mode {
            AlignContentKeyword::Start => 0.0,
            AlignContentKeyword::FlexStart => {
                if layout_is_flex_reversed {
                    free_space
                } else {
                    0.0
                }
            }
            AlignContentKeyword::End => free_space,
            AlignContentKeyword::FlexEnd => {
                if layout_is_flex_reversed {
                    0.0
                } else {
                    free_space
                }
            }
            AlignContentKeyword::Center => free_space / 2.0,
            AlignContentKeyword::Stretch => 0.0,
            AlignContentKeyword::SpaceBetween => 0.0,
            AlignContentKeyword::SpaceAround => {
                if free_space >= 0.0 {
                    (free_space / num_items as f32) / 2.0
                } else {
                    free_space / 2.0
                }
            }
            AlignContentKeyword::SpaceEvenly => {
                if free_space >= 0.0 {
                    free_space / (num_items + 1) as f32
                } else {
                    free_space / 2.0
                }
            }
        }
    } else {
        let free_space = free_space.max(0.0);
        gap + match alignment_mode {
            AlignContentKeyword::Start
            | AlignContentKeyword::FlexStart
            | AlignContentKeyword::End
            | AlignContentKeyword::FlexEnd
            | AlignContentKeyword::Center
            | AlignContentKeyword::Stretch => 0.0,
            AlignContentKeyword::SpaceBetween => free_space / (num_items - 1) as f32,
            AlignContentKeyword::SpaceAround => free_space / num_items as f32,
            AlignContentKeyword::SpaceEvenly => free_space / (num_items + 1) as f32,
        }
    }
}
