use super::*;

/// Where a segment sits in [`plan_breadcrumb_layout`]'s drop order when the bar can't fit
/// everything.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum BreadcrumbSegmentKind {
    /// The leading project-root segment — kept only once every [`Middle`](Self::Middle) segment
    /// is gone, since it's the one segment whose dropdown reaches top-level entries.
    Root,
    /// A directory component strictly between the root (or the start of the trail) and the file
    /// segment — dropped first, since the "⋯" is exactly what stands in for these.
    Middle,
    /// The open file, or the directory navigated into, dropped only once every `Root` and
    /// `Middle` segment is gone.
    File,
    /// An ancestor symbol, outermost first. The last one is nearest the cursor and never dropped.
    Symbol,
}

/// Assigns each segment its kind purely from its position relative to `file_segment_index`.
pub(crate) fn classify_breadcrumb_segment_kinds(
    segment_count: usize,
    file_segment_index: usize,
    has_root_segment: bool,
) -> Vec<BreadcrumbSegmentKind> {
    (0..segment_count)
        .map(|index| match index.cmp(&file_segment_index) {
            Ordering::Greater => BreadcrumbSegmentKind::Symbol,
            Ordering::Equal => BreadcrumbSegmentKind::File,
            Ordering::Less if has_root_segment && index == 0 => BreadcrumbSegmentKind::Root,
            Ordering::Less => BreadcrumbSegmentKind::Middle,
        })
        .collect()
}

/// Aligns `symbol_segments` 1:1 with `segments`, replacing it wholesale if the lengths disagree:
/// later steps assume equal length and would otherwise panic in `Vec::splice`.
pub(crate) fn align_symbol_segments(
    segments: &[HighlightedText],
    symbol_segments: Vec<Option<BreadcrumbSegmentTarget>>,
) -> Vec<Option<BreadcrumbSegmentTarget>> {
    if symbol_segments.len() == segments.len() {
        symbol_segments
    } else {
        vec![None; segments.len()]
    }
}

/// A safety net against a pathologically deep path. Ordinary breadcrumbs never approach it; the
/// width comparison in `plan_breadcrumb_layout` is what actually fires.
const MAX_BREADCRUMB_SEGMENTS_HARD_CAP: usize = 64;

/// Trims a pathologically long run of `Middle` segments to a bounded prefix and suffix before the
/// width-based planner sees it. The run is always contiguous, so this is a single splice that
/// never touches `Root`, `File` or `Symbol`.
pub(crate) fn hard_cap_breadcrumb_middle_segments(
    mut segments: Vec<HighlightedText>,
    mut symbol_segments: Vec<Option<BreadcrumbSegmentTarget>>,
    mut kinds: Vec<BreadcrumbSegmentKind>,
    mut file_segment_index: usize,
) -> (
    Vec<HighlightedText>,
    Vec<Option<BreadcrumbSegmentTarget>>,
    Vec<BreadcrumbSegmentKind>,
    usize,
) {
    let middle_start = kinds
        .iter()
        .position(|kind| *kind == BreadcrumbSegmentKind::Middle);
    let middle_end = kinds
        .iter()
        .rposition(|kind| *kind == BreadcrumbSegmentKind::Middle)
        .map(|index| index + 1);
    let (Some(middle_start), Some(middle_end)) = (middle_start, middle_end) else {
        return (segments, symbol_segments, kinds, file_segment_index);
    };
    if middle_end - middle_start <= MAX_BREADCRUMB_SEGMENTS_HARD_CAP {
        return (segments, symbol_segments, kinds, file_segment_index);
    }

    let half = MAX_BREADCRUMB_SEGMENTS_HARD_CAP / 2;
    let splice_start = middle_start + half;
    let splice_end = middle_end - half;

    segments.splice(
        splice_start..splice_end,
        Some(HighlightedText {
            text: "⋯".into(),
            highlights: vec![],
        }),
    );
    symbol_segments.splice(splice_start..splice_end, Some(None));
    kinds.splice(
        splice_start..splice_end,
        Some(BreadcrumbSegmentKind::Middle),
    );

    // `File` always follows every `Middle` segment, so this splice can only shift its index left.
    file_segment_index -= (splice_end - splice_start) - 1;

    (segments, symbol_segments, kinds, file_segment_index)
}

/// What [`plan_breadcrumb_layout`] decided: `visible` and `ellipses` together partition
/// `0..segment_count`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct BreadcrumbLayoutPlan {
    pub(crate) visible: Vec<usize>,
    pub(crate) ellipses: Vec<Range<usize>>,
}

fn total_breadcrumb_layout_width(
    widths: &[Pixels],
    dropped: &[bool],
    ellipsis_width: Pixels,
) -> Pixels {
    let mut total = Pixels::ZERO;
    let mut in_dropped_run = false;
    for (index, &is_dropped) in dropped.iter().enumerate() {
        if is_dropped {
            if !in_dropped_run {
                total += ellipsis_width;
                in_dropped_run = true;
            }
        } else {
            total += widths[index];
            in_dropped_run = false;
        }
    }
    total
}

fn breadcrumb_layout_plan_from_dropped(dropped: &[bool]) -> BreadcrumbLayoutPlan {
    let mut visible = Vec::new();
    let mut ellipses = Vec::new();
    let mut run_start = None;
    for (index, &is_dropped) in dropped.iter().enumerate() {
        if is_dropped {
            run_start.get_or_insert(index);
        } else {
            if let Some(start) = run_start.take() {
                ellipses.push(start..index);
            }
            visible.push(index);
        }
    }
    if let Some(start) = run_start {
        ellipses.push(start..dropped.len());
    }
    BreadcrumbLayoutPlan { visible, ellipses }
}

/// Drops segments cheapest first until the row fits: every `Middle`, then `Root`, then `File`,
/// then `Symbol` outermost first. The last segment is never a candidate, so the bar never empties.
/// A pure function of the measured widths, so it is testable without a `Window`.
pub(crate) fn plan_breadcrumb_layout(
    widths: &[Pixels],
    kinds: &[BreadcrumbSegmentKind],
    ellipsis_width: Pixels,
    available_width: Pixels,
) -> BreadcrumbLayoutPlan {
    debug_assert_eq!(widths.len(), kinds.len());
    let segment_count = widths.len();
    if segment_count == 0 {
        return BreadcrumbLayoutPlan {
            visible: Vec::new(),
            ellipses: Vec::new(),
        };
    }

    let mut dropped = vec![false; segment_count];
    if total_breadcrumb_layout_width(widths, &dropped, ellipsis_width) <= available_width {
        return breadcrumb_layout_plan_from_dropped(&dropped);
    }

    let last_index = segment_count - 1;
    let mut drop_order = Vec::with_capacity(segment_count - 1);
    for kind in [
        BreadcrumbSegmentKind::Middle,
        BreadcrumbSegmentKind::Root,
        BreadcrumbSegmentKind::File,
        BreadcrumbSegmentKind::Symbol,
    ] {
        drop_order.extend(
            kinds
                .iter()
                .enumerate()
                .filter(|(index, segment_kind)| *index != last_index && **segment_kind == kind)
                .map(|(index, _)| index),
        );
    }

    for index in drop_order {
        dropped[index] = true;
        if total_breadcrumb_layout_width(widths, &dropped, ellipsis_width) <= available_width {
            break;
        }
    }

    breadcrumb_layout_plan_from_dropped(&dropped)
}

pub(crate) fn breadcrumb_layout_plan_width(
    widths: &[Pixels],
    plan: &BreadcrumbLayoutPlan,
    ellipsis_width: Pixels,
) -> Pixels {
    let mut dropped = vec![false; widths.len()];
    for range in &plan.ellipses {
        for index in range.clone() {
            dropped[index] = true;
        }
    }
    total_breadcrumb_layout_width(widths, &dropped, ellipsis_width)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_align_symbol_segments_realigns_divergent_lengths() {
        let segments: Vec<HighlightedText> = (0..3)
            .map(|i| HighlightedText {
                text: format!("segment-{i}").into(),
                highlights: vec![],
            })
            .collect();
        // Shorter than `segments` on purpose: models a navigation whose worktree failed to resolve.
        let symbol_segments = vec![Some(BreadcrumbSegmentTarget::Symbol {
            buffer_id: BufferId::new(1).unwrap(),
            item: None,
        })];

        let symbol_segments = align_symbol_segments(&segments, symbol_segments);

        assert_eq!(symbol_segments.len(), 3);
        assert!(symbol_segments.iter().all(Option::is_none));
    }

    #[test]
    fn test_classify_breadcrumb_segment_kinds() {
        // Root, two directory components, file, two ancestor symbols.
        let kinds = classify_breadcrumb_segment_kinds(6, 3, true);
        assert_eq!(
            kinds,
            vec![
                BreadcrumbSegmentKind::Root,
                BreadcrumbSegmentKind::Middle,
                BreadcrumbSegmentKind::Middle,
                BreadcrumbSegmentKind::File,
                BreadcrumbSegmentKind::Symbol,
                BreadcrumbSegmentKind::Symbol,
            ]
        );

        // No root segment (`Siblings` mode, or the path wasn't split at all): the first segment
        // is `Middle`, not `Root`.
        let kinds = classify_breadcrumb_segment_kinds(3, 1, false);
        assert_eq!(
            kinds,
            vec![
                BreadcrumbSegmentKind::Middle,
                BreadcrumbSegmentKind::File,
                BreadcrumbSegmentKind::Symbol,
            ]
        );

        // The path wasn't split (unsaved buffer): a single `File` segment, `file_segment_index`
        // stays `0`.
        let kinds = classify_breadcrumb_segment_kinds(1, 0, false);
        assert_eq!(kinds, vec![BreadcrumbSegmentKind::File]);
    }

    /// Without `align_symbol_segments`, a short `symbol_segments` (from a worktree that failed to
    /// resolve) panics: splice ranges below are computed from `segments.len()` alone.
    #[test]
    fn test_hard_cap_breadcrumb_middle_segments_does_not_panic_on_divergent_symbol_segments() {
        let segments: Vec<HighlightedText> = (0..100)
            .map(|i| HighlightedText {
                text: format!("segment-{i}").into(),
                highlights: vec![],
            })
            .collect();
        // Only one entry — far shorter than `segments` — modeling the divergence described above.
        let symbol_segments = vec![Some(BreadcrumbSegmentTarget::Symbol {
            buffer_id: BufferId::new(1).unwrap(),
            item: None,
        })];
        let symbol_segments = align_symbol_segments(&segments, symbol_segments);
        assert_eq!(symbol_segments.len(), segments.len());

        // A root segment, 98 middle components, then the file at the end.
        let kinds = classify_breadcrumb_segment_kinds(segments.len(), 99, true);

        let (segments, symbol_segments, kinds, file_segment_index) =
            hard_cap_breadcrumb_middle_segments(segments, symbol_segments, kinds, 99);

        // Root (1) + hard-capped middle (32 kept prefix + 1 "⋯" + 32 kept suffix = 65) + file (1)
        // = 67.
        assert_eq!(segments.len(), 67);
        assert_eq!(symbol_segments.len(), segments.len());
        assert_eq!(kinds.len(), segments.len());
        assert_eq!(file_segment_index, 66);
        assert_eq!(kinds[file_segment_index], BreadcrumbSegmentKind::File);
    }

    #[test]
    fn test_hard_cap_breadcrumb_middle_segments_leaves_ordinary_input_untouched() {
        let segments: Vec<HighlightedText> = (0..6)
            .map(|i| HighlightedText {
                text: format!("segment-{i}").into(),
                highlights: vec![],
            })
            .collect();
        let symbol_segments = vec![None; segments.len()];
        let kinds = classify_breadcrumb_segment_kinds(segments.len(), 3, true);

        let (segments, symbol_segments, kinds, file_segment_index) =
            hard_cap_breadcrumb_middle_segments(segments, symbol_segments, kinds, 3);

        assert_eq!(segments.len(), 6);
        assert_eq!(symbol_segments.len(), 6);
        assert_eq!(kinds.len(), 6);
        assert_eq!(file_segment_index, 3);
    }

    /// Widths (in pixels) for a synthetic six-segment trail modeling the report's own example:
    /// root, four middle directory components, the file, then two ancestor symbols (outermost
    /// first, cursor-nearest last) — `root, a, b, c, d, file.kt, Class, fun method`.
    fn sample_breadcrumb_widths_and_kinds() -> (Vec<Pixels>, Vec<BreadcrumbSegmentKind>) {
        use BreadcrumbSegmentKind::*;
        let widths = vec![
            px(60.),  // root
            px(30.),  // a
            px(30.),  // b
            px(30.),  // c
            px(30.),  // d
            px(80.),  // file.kt
            px(90.),  // Class
            px(120.), // fun method
        ];
        let kinds = vec![Root, Middle, Middle, Middle, Middle, File, Symbol, Symbol];
        (widths, kinds)
    }

    #[test]
    fn test_plan_breadcrumb_layout_everything_fits() {
        let (widths, kinds) = sample_breadcrumb_widths_and_kinds();
        let total: Pixels = widths.iter().fold(Pixels::ZERO, |sum, w| sum + *w);

        let plan = plan_breadcrumb_layout(&widths, &kinds, px(20.), total);

        assert_eq!(plan.visible, (0..widths.len()).collect::<Vec<_>>());
        assert!(plan.ellipses.is_empty());
    }

    #[test]
    fn test_plan_breadcrumb_layout_drops_middle_before_root_before_file_before_outer_symbols() {
        let (widths, kinds) = sample_breadcrumb_widths_and_kinds();

        // Narrow enough that all 4 middle components must go (dropping only 3 still leaves it too
        // wide), but root, file, and both symbols still fit once all 4 are gone.
        let plan = plan_breadcrumb_layout(&widths, &kinds, px(20.), px(380.));
        assert_eq!(plan.visible, vec![0, 5, 6, 7]);
        assert_eq!(plan.ellipses, vec![1..5]);

        // Narrow enough that root has to go too, but file and both symbols still fit.
        let plan = plan_breadcrumb_layout(&widths, &kinds, px(20.), px(340.));
        assert_eq!(plan.visible, vec![5, 6, 7]);
        assert_eq!(plan.ellipses, vec![0..5]);

        // Narrower still: file goes too, leaving just the symbol chain.
        let plan = plan_breadcrumb_layout(&widths, &kinds, px(20.), px(230.));
        assert_eq!(plan.visible, vec![6, 7]);
        assert_eq!(plan.ellipses, vec![0..6]);

        // Narrower yet: the outer symbol goes too, leaving only the innermost — never dropped.
        let plan = plan_breadcrumb_layout(&widths, &kinds, px(20.), px(140.));
        assert_eq!(plan.visible, vec![7]);
        assert_eq!(plan.ellipses, vec![0..7]);
    }

    #[test]
    fn test_plan_breadcrumb_layout_degenerate_case_always_keeps_the_last_segment() {
        let (widths, kinds) = sample_breadcrumb_widths_and_kinds();

        // Not even the innermost symbol alone fits — still renders it rather than nothing.
        let plan = plan_breadcrumb_layout(&widths, &kinds, px(20.), px(1.));
        assert_eq!(plan.visible, vec![7]);
        assert_eq!(plan.ellipses, vec![0..7]);
    }

    #[test]
    fn test_plan_breadcrumb_layout_single_segment_never_collapses() {
        let plan =
            plan_breadcrumb_layout(&[px(500.)], &[BreadcrumbSegmentKind::File], px(20.), px(1.));
        assert_eq!(plan.visible, vec![0]);
        assert!(plan.ellipses.is_empty());
    }

    #[test]
    fn test_plan_breadcrumb_layout_empty_input() {
        let plan = plan_breadcrumb_layout(&[], &[], px(20.), px(500.));
        assert!(plan.visible.is_empty());
        assert!(plan.ellipses.is_empty());
    }
}
