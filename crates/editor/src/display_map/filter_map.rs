use std::{cmp, collections::VecDeque, ops::Range};

use buffer_diff::DiffHunkStatusKind;
use multi_buffer::{
    AnchorRangeExt as _, MultiBufferDiffHunk, MultiBufferPoint, MultiBufferRow,
    MultiBufferSnapshot, ToOffset as _, ToPoint as _,
};
use rope::{Point, TextSummary};
use sum_tree::{Dimensions, Item, SumTree};
use text::{Bias, OffsetRangeExt as _};
use util::{RangeExt as _, debug_panic};

#[derive(Debug, Clone)]
enum FilterTransform {
    Isomorphic {
        summary: TextSummary,
        #[cfg(test)]
        text: String,
    },
    Filter {
        summary: TextSummary,
        #[cfg(test)]
        text: String,
    },
}

impl FilterTransform {
    fn is_isomorphic(&self) -> bool {
        matches!(self, FilterTransform::Isomorphic { .. })
    }

    #[cfg(test)]
    fn text(&self) -> &str {
        match self {
            Self::Isomorphic { text, .. } => text,
            Self::Filter { text, .. } => text,
        }
    }

    #[cfg(test)]
    fn text_escaped(&self) -> String {
        self.text().replace("\n", r"\n").replace("\t", r"\t")
    }
}

impl sum_tree::Item for FilterTransform {
    type Summary = TransformSummary;

    fn summary(&self, _: <Self::Summary as sum_tree::Summary>::Context<'_>) -> Self::Summary {
        match self {
            Self::Isomorphic { summary, .. } => TransformSummary {
                input: *summary,
                output: *summary,
            },
            Self::Filter { summary, .. } => TransformSummary {
                input: *summary,
                output: TextSummary::default(),
            },
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct TransformSummary {
    input: TextSummary,
    output: TextSummary,
}

impl sum_tree::ContextLessSummary for TransformSummary {
    fn zero() -> Self {
        TransformSummary {
            input: TextSummary::default(),
            output: TextSummary::default(),
        }
    }

    fn add_summary(&mut self, summary: &Self) {
        self.input += summary.input;
        self.output += summary.output;
    }
}

struct FilterMap {
    snapshot: FilterSnapshot,
    mode: Option<FilterMode>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum FilterMode {
    RemoveDeletions,
    RemoveInsertions,
}

impl FilterMode {
    fn should_remove(self, kind: DiffHunkStatusKind) -> bool {
        match kind {
            DiffHunkStatusKind::Added => self == FilterMode::RemoveInsertions,
            DiffHunkStatusKind::Modified => {
                debug_panic!(
                    "should not have an unexpanded modified hunk in multibuffer when filter map is active"
                );
                false
            }
            DiffHunkStatusKind::Deleted => self == FilterMode::RemoveDeletions,
        }
    }
}

#[derive(Clone)]
struct FilterSnapshot {
    transforms: SumTree<FilterTransform>,
    buffer_snapshot: MultiBufferSnapshot,
}

/// A byte index into the buffer (after ignored diff hunk lines are deleted)
#[derive(Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
struct FilterOffset(usize);

impl std::fmt::Debug for FilterOffset {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FilterOffset({})", self.0)
    }
}

impl FilterOffset {
    /// Convert a range of offsets to a range of [`FilterOffset`]s, given that
    /// there are no filtered lines in the buffer (for example, if no
    /// [`FilterMode`] is set).
    pub fn naive_range(Range { start, end }: Range<usize>) -> Range<Self> {
        FilterOffset(start)..FilterOffset(end)
    }
}

impl sum_tree::Dimension<'_, TransformSummary> for FilterOffset {
    fn zero(_: <TransformSummary as sum_tree::Summary>::Context<'_>) -> Self {
        FilterOffset(0)
    }

    fn add_summary(
        &mut self,
        summary: &'_ TransformSummary,
        _: <TransformSummary as sum_tree::Summary>::Context<'_>,
    ) {
        self.0 += summary.output.len;
    }
}

impl sum_tree::Dimension<'_, TransformSummary> for MultiBufferPoint {
    fn zero(_: <TransformSummary as sum_tree::Summary>::Context<'_>) -> Self {
        MultiBufferPoint::zero()
    }

    fn add_summary(
        &mut self,
        summary: &'_ TransformSummary,
        cx: <TransformSummary as sum_tree::Summary>::Context<'_>,
    ) {
        *self += &summary.input.lines;
    }
}

#[derive(Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq)]
struct FilterPoint(Point);

impl sum_tree::Dimension<'_, TransformSummary> for FilterPoint {
    fn zero(cx: <TransformSummary as sum_tree::Summary>::Context<'_>) -> Self {
        FilterPoint(Point::zero())
    }

    fn add_summary(
        &mut self,
        summary: &'_ TransformSummary,
        cx: <TransformSummary as sum_tree::Summary>::Context<'_>,
    ) {
        self.0 += summary.output.lines;
    }
}

type FilterEdit = text::Edit<FilterOffset>;

impl FilterMap {
    fn new(mode: Option<FilterMode>, buffer_snapshot: MultiBufferSnapshot) -> Self {
        let mut this = Self {
            mode,
            snapshot: FilterSnapshot {
                buffer_snapshot: buffer_snapshot.clone(),
                transforms: SumTree::from_item(
                    FilterTransform::Isomorphic {
                        summary: buffer_snapshot.text_summary(),
                        #[cfg(test)]
                        text: buffer_snapshot.text(),
                    },
                    (),
                ),
            },
        };
        this.sync(
            buffer_snapshot.clone(),
            vec![text::Edit {
                old: 0..buffer_snapshot.len(),
                new: 0..buffer_snapshot.len(),
            }],
        );
        this
    }

    #[cfg(debug_assertions)]
    fn check_invariants(&self) {
        use itertools::Itertools;
        use multi_buffer::MultiBufferRow;

        #[cfg(test)]
        pretty_assertions::assert_eq!(
            self.snapshot.input_text(),
            self.snapshot.buffer_snapshot.text(),
            "wrong input text"
        );

        pretty_assertions::assert_eq!(
            self.snapshot.transforms.summary().input,
            self.snapshot.buffer_snapshot.text_summary(),
            "input summary does not match buffer snapshot"
        );

        self.snapshot
            .transforms
            .iter()
            .tuple_windows()
            .for_each(|(left, right)| {
                assert!(
                    left.is_isomorphic() || right.is_isomorphic(),
                    "two consecutive non-isomorphic transforms"
                );
                assert!(
                    !left.is_isomorphic() || !right.is_isomorphic(),
                    "two consecutive isomorphic transforms"
                );
            });

        let Some(mode) = self.mode else {
            pretty_assertions::assert_eq!(
                self.snapshot.transforms.iter().count(),
                1,
                "more than one transform in a trivial map"
            );
            pretty_assertions::assert_eq!(
                self.snapshot.transforms.summary().output,
                self.snapshot.buffer_snapshot.text_summary(),
                "output summary for trivial map does not match buffer snapshot"
            );
            return;
        };

        dbg!(&mode);

        #[cfg(test)]
        self.snapshot.print_transforms();

        #[cfg(test)]
        log::info!("filter map output text:\n{}", self.snapshot.text());

        let mut expected_summary = TextSummary::default();
        let mut expected_text = String::new();

        let mut row_infos = self
            .snapshot
            .buffer_snapshot
            .row_infos(MultiBufferRow(0))
            .peekable();
        while let Some(row_info) = row_infos.next() {
            let Some(row) = row_info.multibuffer_row else {
                continue;
            };
            if let Some(status) = row_info.diff_status
                && mode.should_remove(status.kind)
            {
                continue;
            }

            let row_range = Point::new(row.0, 0)
                ..Point::new(row.0 + 1, 0).min(self.snapshot.buffer_snapshot.max_point());
            expected_summary += self
                .snapshot
                .buffer_snapshot
                .text_summary_for_range::<TextSummary, _>(row_range.clone());
            let row_text = self
                .snapshot
                .buffer_snapshot
                .text_for_range(row_range)
                .collect::<String>();
            expected_text += &row_text;
        }

        #[cfg(test)]
        pretty_assertions::assert_eq!(
            self.snapshot.text(),
            expected_text,
            "wrong output text for nontrivial map"
        );

        pretty_assertions::assert_eq!(
            self.snapshot.transforms.summary().output,
            expected_summary,
            "wrong output summary for nontrivial map"
        )
    }
}

impl FilterMap {
    fn sync(
        &mut self,
        buffer_snapshot: MultiBufferSnapshot,
        buffer_edits: Vec<text::Edit<usize>>,
    ) -> (FilterSnapshot, Vec<FilterEdit>) {
        if buffer_edits.is_empty() {
            return (self.snapshot.clone(), Vec::new());
        }

        let Some(mode) = self.mode else {
            // If we're not filtering out anything, edits can be passed through
            // unchanged and we only need one isomorphic transform.
            self.snapshot.buffer_snapshot = buffer_snapshot.clone();
            self.snapshot.transforms = SumTree::from_item(
                FilterTransform::Isomorphic {
                    summary: buffer_snapshot.text_summary(),
                    #[cfg(test)]
                    text: buffer_snapshot.text(),
                },
                (),
            );
            return (
                self.snapshot.clone(),
                buffer_edits
                    .into_iter()
                    .map(|edit| text::Edit {
                        old: FilterOffset::naive_range(edit.old),
                        new: FilterOffset::naive_range(edit.new),
                    })
                    .collect(),
            );
        };

        dbg!(&buffer_edits);

        // Extend the new range of each edit to the next line boundary--most convenient to work in terms of points for this.
        let buffer_edits = {
            let mut buffer_point_edits = buffer_edits
                .into_iter()
                .map(|buffer_edit| text::Edit {
                    old: buffer_edit
                        .old
                        .start
                        .to_point(&self.snapshot.buffer_snapshot)
                        ..buffer_edit.old.end.to_point(&self.snapshot.buffer_snapshot),
                    new: buffer_edit.new.start.to_point(&buffer_snapshot)
                        ..buffer_edit.new.end.to_point(&buffer_snapshot),
                })
                .peekable();
            let mut merged_buffer_point_edits = Vec::new();
            while let Some(mut buffer_edit) = buffer_point_edits.next() {
                let start_of_next_line =
                    Point::new(buffer_edit.new.end.row + 1, 0).min(buffer_snapshot.max_point());

                if let Some(next_buffer_edit) = buffer_point_edits.peek_mut()
                    && next_buffer_edit.new.start < start_of_next_line
                {
                    next_buffer_edit.old.start = buffer_edit.old.start;
                    next_buffer_edit.new.start = buffer_edit.new.start;
                    continue;
                }

                buffer_edit.old.end += start_of_next_line - buffer_edit.new.end;
                buffer_edit.new.end = start_of_next_line;
                merged_buffer_point_edits.push(buffer_edit);
            }
            merged_buffer_point_edits
        };

        let mut new_transforms: SumTree<FilterTransform> = SumTree::new(());
        // TODO might be worth having a usize dimension here too to help with overshoot calculations
        let mut transform_cursor =
            self.snapshot
                .transforms
                .cursor::<Dimensions<MultiBufferPoint, FilterPoint, FilterOffset>>(());
        let mut output_edits: Vec<text::Edit<FilterOffset>> = Vec::new();

        // TODO in what follows we repeatedly call text_summary_for_range,
        // could use a persistent usize cursor over buffer_snapshot instead.

        transform_cursor.next();

        let mut buffer_edits = buffer_edits.into_iter().peekable();

        while let Some(buffer_edit) = buffer_edits.next() {
            dbg!(&buffer_edit);

            log::info!("append old transforms before edit");
            log::info!(
                "input len before append is {}",
                new_transforms.summary().input.len
            );
            log::info!(
                "output len before append is {}",
                new_transforms.summary().output.len
            );
            new_transforms.append(
                transform_cursor.slice(&buffer_edit.old.start, Bias::Left),
                (),
            );
            log::info!(
                "input len after append is {}",
                new_transforms.summary().input.len
            );
            log::info!(
                "output len after append is {}",
                new_transforms.summary().output.len
            );

            let mut edit_old_start = transform_cursor.start().2;
            if buffer_edit.old.start > transform_cursor.start().0
                && let Some(FilterTransform::Isomorphic { .. }) = transform_cursor.item()
            {
                // TODO perf
                let buffer_edit_old_start_offset = buffer_edit
                    .old
                    .start
                    .to_offset(&self.snapshot.buffer_snapshot);
                let transform_cursor_start_offset = transform_cursor
                    .start()
                    .0
                    .to_offset(&self.snapshot.buffer_snapshot);
                edit_old_start.0 += buffer_edit_old_start_offset - transform_cursor_start_offset;
            }

            let mut edit_new_start = FilterOffset(new_transforms.summary().output.len);
            if buffer_edit.new.start > new_transforms.summary().input.lines {
                let range = new_transforms.summary().input.lines..buffer_edit.new.start;
                match transform_cursor.item() {
                    Some(FilterTransform::Isomorphic { .. }) => {
                        let summary = push_isomorphic(&mut new_transforms, range, &buffer_snapshot);
                        edit_new_start.0 += summary.len;
                    }
                    Some(FilterTransform::Filter { .. }) => {
                        push_filter(&mut new_transforms, range, &buffer_snapshot);
                    }
                    None => {}
                }
            }

            // Process the edited range based on diff hunks. Extend the range of iteration a bit
            // to catch hunks before the start of the edit that nonetheless affect the diff status
            // of that row.
            let mut query_range_start = buffer_edit.new.start;
            query_range_start.column = 0;
            for hunk in buffer_snapshot.diff_hunks_in_range(query_range_start..buffer_edit.new.end)
            {
                let (deletion_range, addition_range) = diff_hunk_bounds(&hunk, &buffer_snapshot);
                let deletion_range = deletion_range.clamp(buffer_edit.new.clone());
                let addition_range = addition_range.clamp(buffer_edit.new.clone());
                let prefix_range = new_transforms.summary().input.lines..deletion_range.start;
                log::info!("push isomorphic content before hunk");
                push_isomorphic(&mut new_transforms, prefix_range, &buffer_snapshot);

                match mode {
                    FilterMode::RemoveDeletions => {
                        log::info!("filter hunk deletion");
                        push_filter(&mut new_transforms, deletion_range, &buffer_snapshot);
                        push_isomorphic(&mut new_transforms, addition_range, &buffer_snapshot);
                    }
                    FilterMode::RemoveInsertions => {
                        log::info!("filter hunk insertion");
                        push_isomorphic(&mut new_transforms, deletion_range, &buffer_snapshot);
                        push_filter(&mut new_transforms, addition_range, &buffer_snapshot);
                    }
                }
            }

            log::info!("push isomorphic content after last hunk");
            if buffer_edit.new.end > new_transforms.summary().input.lines {
                let suffix_range = new_transforms.summary().input.lines..buffer_edit.new.end;
                push_isomorphic(&mut new_transforms, suffix_range, &buffer_snapshot);
            }

            transform_cursor.seek(&buffer_edit.old.end, Bias::Right);
            let mut edit_old_end = transform_cursor.start().2;
            let edit_new_end = FilterOffset(new_transforms.summary().output.len);
            if buffer_edit.old.end > transform_cursor.start().0 {
                match transform_cursor.item() {
                    Some(FilterTransform::Isomorphic { .. }) => {
                        // TODO perf
                        let buffer_edit_old_end_offset = buffer_edit
                            .old
                            .end
                            .to_offset(&self.snapshot.buffer_snapshot);
                        let transform_cursor_start_offset = transform_cursor
                            .start()
                            .0
                            .to_offset(&self.snapshot.buffer_snapshot);
                        edit_old_end.0 +=
                            buffer_edit_old_end_offset - transform_cursor_start_offset;
                    }
                    Some(FilterTransform::Filter { .. }) => {}
                    None => {}
                }
            }

            if buffer_edits.peek().is_none_or(|next_buffer_edit| {
                next_buffer_edit.old.start >= transform_cursor.end().0
            }) {
                log::info!(
                    "consume remainder of old transform since this is the last intersecting edit"
                );
                let suffix_start = new_transforms.summary().input.lines;
                let suffix_len = transform_cursor.end().0 - buffer_edit.old.end;
                match transform_cursor.item() {
                    Some(FilterTransform::Isomorphic { .. }) => {
                        push_isomorphic(
                            &mut new_transforms,
                            suffix_start
                                ..std::cmp::min(
                                    suffix_start + suffix_len,
                                    buffer_snapshot.max_point(),
                                ),
                            &buffer_snapshot,
                        );
                        transform_cursor.next();
                    }
                    Some(FilterTransform::Filter { .. }) => {
                        push_filter(
                            &mut new_transforms,
                            suffix_start
                                ..std::cmp::min(
                                    suffix_start + suffix_len,
                                    buffer_snapshot.max_point(),
                                ),
                            &buffer_snapshot,
                        );
                        transform_cursor.next();
                    }
                    None => {}
                }
            }

            let edit = text::Edit {
                old: edit_old_start..edit_old_end,
                new: edit_new_start..edit_new_end,
            };
            debug_assert!(
                edit.old.start <= edit_old_end && edit.new.start <= edit_new_end,
                "inverted edit: {edit:?}",
            );
            if let Some(prev_edit) = output_edits.last() {
                debug_assert!(
                    prev_edit.old.end <= edit.old.start && prev_edit.new.end <= edit.new.start,
                    "unordered edits: {prev_edit:?}, {edit:?}"
                );
            }
            output_edits.push(edit);
        }

        log::info!("append old transforms after last edit");
        log::info!(
            "input len before suffix is {}",
            new_transforms.summary().input.len
        );
        let suffix = transform_cursor.suffix();
        log::info!("suffix summary is {:?}", suffix.summary());
        new_transforms.append(suffix, ());
        log::info!(
            "input len after suffix is {}",
            new_transforms.summary().input.len
        );

        drop(transform_cursor);

        let new_snapshot = FilterSnapshot {
            transforms: new_transforms,
            buffer_snapshot,
        };
        #[cfg(test)]
        check_edits(&self.snapshot, &output_edits, &new_snapshot);
        self.snapshot = new_snapshot;
        #[cfg(debug_assertions)]
        self.check_invariants();
        (self.snapshot.clone(), output_edits)
    }
}

#[cfg(test)]
fn check_edits(
    old_snapshot: &FilterSnapshot,
    output_edits: &[text::Edit<FilterOffset>],
    new_snapshot: &FilterSnapshot,
) {
    let mut edited_old_text = old_snapshot.text();
    let new_text = new_snapshot.text();

    dbg!(&output_edits);

    for edit in output_edits.iter().rev() {
        let edit_old_range = edit.old.start.0..edit.old.end.0;
        let edit_new_range = edit.new.start.0..edit.new.end.0;
        assert!(
            edited_old_text.get(edit_old_range.clone()).is_some(),
            "old range too large for old text (range is {:?}, old length {}, new length: {}, old text:\n{})",
            edit_old_range,
            edited_old_text.len(),
            new_text.len(),
            edited_old_text,
        );
        assert!(
            new_text.get(edit_new_range.clone()).is_some(),
            "new range too large for old text (range is {:?}, old length {}, new length: {}, new text: \n{})",
            edit_new_range,
            edited_old_text.len(),
            new_text.len(),
            new_text,
        );
        edited_old_text.replace_range(edit_old_range, &new_text[edit_new_range]);
    }

    pretty_assertions::assert_eq!(
        edited_old_text,
        new_text,
        "edits don't transform old snapshot into new snapshot"
    );
}

fn diff_hunk_bounds(
    hunk: &MultiBufferDiffHunk,
    buffer_snapshot: &MultiBufferSnapshot,
) -> (Range<MultiBufferPoint>, Range<MultiBufferPoint>) {
    let start_of_hunk = Point::new(hunk.row_range.start.0, 0);
    let switch_point = hunk
        .multi_buffer_range()
        .start
        .bias_right(&buffer_snapshot)
        .to_point(&buffer_snapshot);
    let end_of_hunk = MultiBufferPoint::new(hunk.row_range.end.0, 0);
    (start_of_hunk..switch_point, switch_point..end_of_hunk)
}

fn text_summaries_for_diff_hunk(
    hunk: &multi_buffer::MultiBufferDiffHunk,
    buffer_snapshot: &MultiBufferSnapshot,
) -> (TextSummary, TextSummary) {
    let start_of_hunk = hunk
        .multi_buffer_range()
        .start
        .bias_left(&buffer_snapshot)
        .to_point(&buffer_snapshot);
    let switch_point = hunk
        .multi_buffer_range()
        .start
        .bias_right(&buffer_snapshot)
        .to_point(&buffer_snapshot);
    let end_of_hunk = hunk.row_range.end.0;
    let end_of_hunk = Point::new(end_of_hunk, 0);
    let deletion_summary = buffer_snapshot.text_summary_for_range(start_of_hunk..switch_point);
    let addition_summary = buffer_snapshot.text_summary_for_range(switch_point..end_of_hunk);
    (deletion_summary, addition_summary)
}

impl FilterSnapshot {
    #[cfg(test)]
    fn text(&self) -> String {
        self.transforms
            .iter()
            .filter_map(|t| match t {
                FilterTransform::Isomorphic { text, .. } => Some(text.as_str()),
                FilterTransform::Filter { .. } => None,
            })
            .collect()
    }

    #[cfg(test)]
    fn input_text(&self) -> String {
        self.transforms.iter().map(|t| t.text()).collect()
    }

    #[cfg(test)]
    fn print_transforms(&self) {
        if self.transforms.is_empty() {
            println!("<empty>");
            return;
        }
        let mut offset = 0;

        for transform in self.transforms.iter() {
            let new_offset = offset + transform.summary(()).input.len;
            let ty = match transform {
                FilterTransform::Filter { .. } => "F",
                FilterTransform::Isomorphic { .. } => "I",
            };
            println!(
                "{offset:0>3}->{new_offset:0>3} ({ty}): {}",
                transform.text_escaped(),
            );
            offset = new_offset;
        }
    }
}

fn push_isomorphic(
    transforms: &mut SumTree<FilterTransform>,
    range: Range<MultiBufferPoint>,
    snapshot: &MultiBufferSnapshot,
) -> TextSummary {
    if range.is_empty() {
        return TextSummary::default();
    }

    let summary_to_add = snapshot.text_summary_for_range::<TextSummary, _>(range.clone());
    #[cfg(test)]
    let text_to_add = snapshot.text_for_range(range).collect::<String>();

    #[cfg(test)]
    log::info!("| push_isomorphic({text_to_add:?})");

    let mut merged = false;
    transforms.update_last(
        |transform| {
            if let FilterTransform::Isomorphic {
                summary,
                #[cfg(test)]
                text,
            } = transform
            {
                *summary += summary_to_add;
                #[cfg(test)]
                text.push_str(&text_to_add);
                merged = true;
            }
        },
        (),
    );
    if !merged {
        transforms.push(
            FilterTransform::Isomorphic {
                summary: summary_to_add,
                #[cfg(test)]
                text: text_to_add,
            },
            (),
        );
    }
    summary_to_add
}

fn push_filter(
    transforms: &mut SumTree<FilterTransform>,
    range: Range<MultiBufferPoint>,
    snapshot: &MultiBufferSnapshot,
) {
    if range.is_empty() {
        return;
    }

    let summary_to_add = snapshot.text_summary_for_range::<TextSummary, _>(range.clone());
    #[cfg(test)]
    let text_to_add = snapshot.text_for_range(range).collect::<String>();

    #[cfg(test)]
    log::info!("| push_filter({text_to_add:?})");

    let mut merged = false;
    transforms.update_last(
        |transform| {
            if let FilterTransform::Filter {
                summary,
                #[cfg(test)]
                text,
            } = transform
            {
                *summary += summary_to_add;
                #[cfg(test)]
                text.push_str(&text_to_add);
                merged = true;
            }
        },
        (),
    );
    if !merged {
        transforms.push(
            FilterTransform::Filter {
                summary: summary_to_add,
                #[cfg(test)]
                text: text_to_add,
            },
            (),
        );
    }
}

impl FilterSnapshot {
    // fn text_summary_for_range(&self, range: Range<FilterOffset>) -> TextSummary {
    //     let mut summary = TextSummary::default();

    //     let mut cursor = self
    //         .transforms
    //         .cursor::<Dimensions<FilterOffset, usize>>(());
    //     cursor.seek(&range.start, Bias::Right);

    //     let overshoot = range.start.0 - cursor.start().0.0;
    //     match cursor.item() {
    //         Some(FilterTransform::Isomorphic { .. }) => {
    //             let buffer_start = cursor.start().1;
    //             let suffix_start = buffer_start + overshoot;
    //             let suffix_end =
    //                 buffer_start + (cmp::min(cursor.end().0, range.end).0 - cursor.start().0.0);
    //             summary = self
    //                 .buffer_snapshot
    //                 .text_summary_for_range(suffix_start..suffix_end);
    //             cursor.next();
    //         }
    //         Some(FilterTransform::Filter { .. }) | None => {}
    //     }

    //     if range.end > cursor.start().0 {
    //         summary += cursor
    //             .summary::<_, TransformSummary>(&range.end, Bias::Right)
    //             .output;

    //         let overshoot = range.end.0 - cursor.start().0.0;
    //         match cursor.item() {
    //             Some(FilterTransform::Isomorphic { .. }) => {
    //                 let prefix_start = cursor.start().1;
    //                 let prefix_end = prefix_start + overshoot;
    //                 summary += self
    //                     .buffer_snapshot
    //                     .text_summary_for_range::<TextSummary, _>(prefix_start..prefix_end);
    //             }
    //             Some(FilterTransform::Filter { .. }) | None => {}
    //         }
    //     }
    //
    //     summary
    // }

    // fn to_point(&self, offset: FilterOffset) -> FilterPoint {
    //     let (start, _, item) = self
    //         .transforms
    //         .find::<Dimensions<FilterOffset, FilterPoint, usize>, _>((), &offset, Bias::Right);
    //     let overshoot = offset.0 - start.0.0;
    //     match item {
    //         Some(FilterTransform::Isomorphic { .. }) => {
    //             let buffer_offset_start = start.2;
    //             let buffer_offset_end = buffer_offset_start + overshoot;
    //             let buffer_start = self.buffer_snapshot.offset_to_point(buffer_offset_start);
    //             let buffer_end = self.buffer_snapshot.offset_to_point(buffer_offset_end);
    //             FilterPoint(start.1.0 + (buffer_end - buffer_start))
    //         }
    //         Some(FilterTransform::Filter { .. }) | None => self.max_point(),
    //     }
    // }

    fn max_point(&self) -> FilterPoint {
        FilterPoint(self.transforms.summary().output.lines)
    }

    fn point_to_offset(&self, point: FilterPoint) -> FilterOffset {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use buffer_diff::BufferDiff;
    use collections::HashMap;
    use gpui::{AppContext as _, Entity};
    use language::{Buffer, Capability};
    use multi_buffer::{MultiBuffer, randomly_mutate_multibuffer_with_diffs};
    use rand::{Rng as _, rngs::StdRng};
    use text::{BufferId, Point};

    use crate::display_map::filter_map::{FilterMap, FilterMode};

    #[gpui::test(iterations = 100)]
    fn test_random_filter_map(cx: &mut gpui::TestAppContext, mut rng: StdRng) {
        let operations = std::env::var("OPERATIONS")
            .map(|i| i.parse().expect("invalid `OPERATIONS` variable"))
            .unwrap_or(10);

        let mut buffers: Vec<Entity<Buffer>> = Vec::new();
        let mut base_texts: HashMap<BufferId, String> = HashMap::default();
        let multibuffer = cx.new(|cx| {
            let mut multibuffer = MultiBuffer::new(Capability::ReadWrite);
            multibuffer.set_all_diff_hunks_expanded(cx);
            multibuffer
        });
        let mut needs_diff_calculation = false;

        let mode = if rng.random() {
            FilterMode::RemoveDeletions
        } else {
            FilterMode::RemoveInsertions
        };
        let mut filter_map = FilterMap::new(
            Some(mode),
            multibuffer.read_with(cx, |multibuffer, cx| multibuffer.snapshot(cx)),
        );

        for _ in 0..operations {
            let subscription = multibuffer.update(cx, |multibuffer, cx| multibuffer.subscribe());
            randomly_mutate_multibuffer_with_diffs(
                multibuffer.clone(),
                &mut buffers,
                &mut base_texts,
                &mut needs_diff_calculation,
                &mut rng,
                cx,
            );
            let buffer_snapshot =
                multibuffer.read_with(cx, |multibuffer, cx| multibuffer.snapshot(cx));
            let buffer_edits = subscription.consume().into_inner();
            filter_map.sync(buffer_snapshot, buffer_edits);
        }
    }

    #[gpui::test]
    fn test_deleting_hunk_start_anchor(cx: &mut gpui::TestAppContext) {
        let base_text = "old line\n";
        let multibuffer = cx.update(|cx| MultiBuffer::build_simple("old line\nnew line\n", cx));
        let buffer = multibuffer.update(cx, |multibuffer, cx| {
            multibuffer.set_all_diff_hunks_expanded(cx);
            multibuffer.all_buffers().into_iter().next().unwrap()
        });
        let diff = cx.new(|cx| BufferDiff::new_with_base_text(base_text, &buffer, cx));
        multibuffer.update(cx, |multibuffer, cx| {
            multibuffer.add_diff(diff.clone(), cx);
        });
        cx.run_until_parked();

        let mut filter_map = FilterMap::new(
            Some(FilterMode::RemoveInsertions),
            multibuffer.read_with(cx, |multibuffer, cx| multibuffer.snapshot(cx)),
        );

        let subscription = multibuffer.update(cx, |multibuffer, _| multibuffer.subscribe());
        let snapshot = multibuffer.update(cx, |multibuffer, cx| {
            multibuffer.edit([(Point::new(0, 0)..Point::new(1, 1), "")], None, cx);
            multibuffer.snapshot(cx)
        });
        let edits = subscription.consume();

        filter_map.sync(snapshot, edits.into_inner());
    }
}
