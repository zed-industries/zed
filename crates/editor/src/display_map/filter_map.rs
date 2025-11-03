use std::{cmp, ops::Range};

use buffer_diff::{DiffHunkStatus, DiffHunkStatusKind};
use multi_buffer::{AnchorRangeExt as _, MultiBufferSnapshot, ToPoint as _};
use rope::{Point, TextSummary};
use schemars::transform;
use sum_tree::{Dimensions, SumTree};
use text::Bias;
use util::debug_panic;

/// All summaries are an integral number of multibuffer rows.
#[derive(Debug, Clone, Copy)]
struct WholeLineTextSummary(pub TextSummary);

impl WholeLineTextSummary {
    pub fn empty() -> Self {
        Self(TextSummary::default())
    }
}

#[derive(Debug, Clone, Copy)]
enum Transform {
    Isomorphic { summary: WholeLineTextSummary },
    Filter { summary: WholeLineTextSummary },
}

impl Transform {
    fn is_isomorphic(&self) -> bool {
        matches!(self, Transform::Isomorphic { .. })
    }
}

impl sum_tree::Item for Transform {
    type Summary = TransformSummary;

    fn summary(&self, cx: <Self::Summary as sum_tree::Summary>::Context<'_>) -> Self::Summary {
        match self {
            Self::Isomorphic { summary } => TransformSummary {
                input: *summary,
                output: *summary,
            },
            Self::Filter { summary } => TransformSummary {
                input: *summary,
                output: WholeLineTextSummary::empty(),
            },
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct TransformSummary {
    input: WholeLineTextSummary,
    output: WholeLineTextSummary,
}

impl sum_tree::ContextLessSummary for TransformSummary {
    fn zero() -> Self {
        TransformSummary {
            input: WholeLineTextSummary::empty(),
            output: WholeLineTextSummary::empty(),
        }
    }

    fn add_summary(&mut self, summary: &Self) {
        self.input.0 += summary.input.0;
        self.output.0 += summary.output.0;
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
    transforms: SumTree<Transform>,
    buffer_snapshot: MultiBufferSnapshot,
}

/// A byte index into the buffer (after ignored diff hunk lines are deleted)
#[derive(Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq)]
struct FilterOffset(usize);

impl FilterOffset {
    /// Convert a range of offsets to a range of [`FilterOffset`]s, given that
    /// there are no filtered lines in the buffer (for example, if no
    /// [`FilterMode`] is set).
    pub fn naive_range(Range { start, end }: Range<usize>) -> Range<Self> {
        FilterOffset(start)..FilterOffset(end)
    }
}

impl sum_tree::Dimension<'_, TransformSummary> for FilterOffset {
    fn zero(cx: <TransformSummary as sum_tree::Summary>::Context<'_>) -> Self {
        FilterOffset(0)
    }

    fn add_summary(
        &mut self,
        summary: &'_ TransformSummary,
        cx: <TransformSummary as sum_tree::Summary>::Context<'_>,
    ) {
        self.0 += summary.output.0.len;
    }
}

impl sum_tree::Dimension<'_, TransformSummary> for usize {
    fn zero(cx: <TransformSummary as sum_tree::Summary>::Context<'_>) -> Self {
        0
    }

    fn add_summary(
        &mut self,
        summary: &'_ TransformSummary,
        cx: <TransformSummary as sum_tree::Summary>::Context<'_>,
    ) {
        *self += summary.input.0.len;
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
        self.0 += summary.output.0.lines;
    }
}

type FilterEdit = text::Edit<FilterOffset>;

impl FilterMap {
    fn new(mode: Option<FilterMode>, buffer_snapshot: MultiBufferSnapshot) -> Self {
        let mut this = Self {
            mode,
            snapshot: FilterSnapshot {
                buffer_snapshot: buffer_snapshot.clone(),
                transforms: SumTree::default(),
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

        #[cfg(any(test, feature = "test-support"))]
        log::info!("filter map output text:\n{}", self.snapshot.text());

        pretty_assertions::assert_eq!(
            self.snapshot.transforms.summary().input.0,
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
                self.snapshot.transforms.summary().output.0,
                self.snapshot.buffer_snapshot.text_summary(),
                "output summary for trivial map does not match buffer snapshot"
            );
            return;
        };

        let mut expected_summary = TextSummary::default();
        let mut anchor = multi_buffer::Anchor::min();
        for hunk in self.snapshot.buffer_snapshot.diff_hunks() {
            dbg!(&hunk);

            expected_summary += self
                .snapshot
                .buffer_snapshot
                .text_summary_for_range::<TextSummary, _>(
                    anchor
                        ..hunk
                            .multi_buffer_range()
                            .start
                            .bias_left(&self.snapshot.buffer_snapshot),
                );
            let (deletion_summary, addition_summary) =
                text_summaries_for_diff_hunk(&hunk, &self.snapshot.buffer_snapshot);

            dbg!(deletion_summary.len, addition_summary.len,);

            match mode {
                FilterMode::RemoveDeletions => {
                    expected_summary += addition_summary;
                }
                FilterMode::RemoveInsertions => {
                    expected_summary += deletion_summary;
                }
            }
            // if !mode.should_remove(hunk.status().kind) {
            //     expected_summary += self
            //         .snapshot
            //         .buffer_snapshot
            //         .text_summary_for_range::<TextSummary, _>(hunk.multi_buffer_range());
            // }
            anchor = hunk.multi_buffer_range().end;
        }
        expected_summary += self
            .snapshot
            .buffer_snapshot
            .text_summary_for_range::<TextSummary, _>(anchor..multi_buffer::Anchor::max());

        dbg!(self.snapshot.transforms.iter().collect::<Vec<_>>());

        pretty_assertions::assert_eq!(
            self.snapshot.transforms.summary().output.0,
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
        let Some(mode) = self.mode else {
            // If we're not filtering out anything, edits can be passed through
            // unchanged and we only need one isomorphic transform.
            self.snapshot.buffer_snapshot = buffer_snapshot.clone();
            self.snapshot.transforms = SumTree::from_item(
                Transform::Isomorphic {
                    summary: WholeLineTextSummary(buffer_snapshot.text_summary()),
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

        let mut new_transforms: SumTree<Transform> = SumTree::new(());
        let mut transform_cursor = self
            .snapshot
            .transforms
            .cursor::<Dimensions<usize, FilterOffset>>(());
        let mut output_edits = Vec::new();

        // TODO in what follows we repeatedly call text_summary_for_range,
        // could use a persistent usize cursor over buffer_snapshot instead.

        transform_cursor.next();

        // for e1 in &buffer_edits {
        //     for e2 in &buffer_edits {
        //         assert!(!e1.old.overlaps(e2.old));
        //     }
        // }
        //
        // Edit { old: 3..6, new: 10..20 }    -> Edit { old: 10..20, new: something_else }
        // break at old = 4
        // old = 3..4, 4..6
        // new = 10..20, 10..20
        // new = 10..20, 20..20
        // new = ??????

        // convert vec<edit> to vecdeque<edit>
        // iterate through edits
        // check if an edit crosses a transform boundary
        // if it does, truncate, and push the other half to the front of the queue

        // |--------------------------------------| len = 216           self.snapshot.transforms (cursor) SumTree<FilterTransform>
        //    <-----> <--------------> <------> <-------> <---->
        //    <-----> <--------------> <------> <><-----> <---->
        //    <-->
        //
        //    |
        //            |
        //                             |
        //
        // |---------| |-----------| |------------|                      new_transforms: SumTree<FilterTransform>
        //
        // first iteration should give us new transforms like:
        //
        //

        let mut buffer_edits = buffer_edits.into_iter().peekable();

        while let Some(buffer_edit) = buffer_edits.next() {
            debug_assert!(transform_cursor.start().0 <= buffer_edit.old.end);

            // Reuse any old transforms that strictly precede the start of the edit.
            log::info!(
                "input len before append is {}",
                new_transforms.summary().input.0.len
            );
            new_transforms.append(transform_cursor.slice(&buffer_edit.old.end, Bias::Left), ());
            log::info!(
                "input len after append is {}",
                new_transforms.summary().input.0.len
            );

            let mut edit_old_start = transform_cursor.start().1;
            let mut edit_new_start = FilterOffset(new_transforms.summary().output.0.len);

            // If the edit starts in the middle of a transform, split the transform and push the unaffected portion.
            if buffer_edit.new.start > new_transforms.summary().input.0.len {
                let summary = buffer_snapshot.text_summary_for_range(
                    new_transforms.summary().input.0.len..buffer_edit.new.start,
                );
                match transform_cursor.item() {
                    Some(Transform::Isomorphic { .. }) => {
                        push_isomorphic(&mut new_transforms, summary);
                        edit_old_start.0 += summary.len;
                        edit_new_start.0 += summary.len;
                    }
                    Some(Transform::Filter { .. }) => {
                        push_filter(&mut new_transforms, summary);
                    }
                    None => {}
                }
                // let summary = self
                //     .snapshot
                //     .buffer_snapshot
                //     .text_summary_for_range(transform_cursor.start().0..buffer_edit.old.start);
                // match transform_cursor.item() {
                //     Some(Transform::Isomorphic { .. }) => {
                //         push_isomorphic(&mut new_transforms, summary);
                //         edit_old_start.0 += summary.len;
                //         edit_new_start.0 += summary.len;
                //         // transform_cursor.next();
                //         dbg!(transform_cursor.start(), transform_cursor.end());
                //     }
                //     Some(Transform::Filter { .. }) => {
                //         push_filter(&mut new_transforms, summary);
                //         // transform_cursor.next();
                //     }
                //     None => {}
                // }
            }

            // For each hunk in the edit, push the non-hunk region preceding it, then
            // possibly filter the hunk depending on the mode.
            for hunk in buffer_snapshot.diff_hunks_in_range(buffer_edit.new.clone()) {
                let mut hunk_range = hunk.multi_buffer_range().to_offset(&buffer_snapshot);
                hunk_range.start = std::cmp::max(hunk_range.start, buffer_edit.new.start);
                hunk_range.end = std::cmp::min(hunk_range.end, buffer_edit.new.end);
                let prefix_range = new_transforms.summary().input.0.len..hunk_range.start;
                push_isomorphic(
                    &mut new_transforms,
                    buffer_snapshot.text_summary_for_range(prefix_range),
                );

                let (deletion_summary, addition_summary) =
                    text_summaries_for_diff_hunk(&hunk, &buffer_snapshot);

                // let hunk_summary = buffer_snapshot.text_summary_for_range(hunk_range);
                // let start_of_hunk_line = ...;
                // let switch_mode_line = ...;
                // let end_of_hunk_line = ...;
                match mode {
                    FilterMode::RemoveDeletions => {
                        push_filter(&mut new_transforms, deletion_summary);
                        push_isomorphic(&mut new_transforms, addition_summary);
                    }
                    FilterMode::RemoveInsertions => {
                        push_isomorphic(&mut new_transforms, deletion_summary);
                        push_filter(&mut new_transforms, addition_summary);
                    }
                }
                // if (mode == FilterMode::RemoveDeletions)
                //     == (hunk.status().kind == DiffHunkStatusKind::Deleted)
                // {
                //     push_filter(&mut new_transforms, hunk_summary);
                // } else {
                //     push_isomorphic(&mut new_transforms, hunk_summary);
                // }
            }

            // Push any non-hunk content after the last hunk.
            if buffer_edit.new.end > new_transforms.summary().input.0.len {
                let suffix_range = new_transforms.summary().input.0.len..buffer_edit.new.end;
                push_isomorphic(
                    &mut new_transforms,
                    buffer_snapshot.text_summary_for_range(suffix_range),
                );
            }

            //           transforms_cursor.start()
            //           v
            //           |----------------------------| old transforms
            // -------------->    <------------->       edits
            //               ^ buffer_edit.old.end

            transform_cursor.seek(&buffer_edit.old.end, Bias::Right);
            let mut edit_old_end = transform_cursor.end().1;
            let edit_new_end = FilterOffset(new_transforms.summary().output.0.len);
            if buffer_edit.old.end > transform_cursor.start().0 {
                // let summary = self
                //     .snapshot
                //     .buffer_snapshot
                //     .text_summary_for_range(buffer_edit.old.end..transform_cursor.end().0);
                match transform_cursor.item() {
                    Some(Transform::Isomorphic { .. }) => {
                        // push_isomorphic(&mut new_transforms, summary);
                        edit_old_end.0 += buffer_edit.old.end - transform_cursor.start().0;
                        // edit_new_end.0 += buffer_edit.old.end - transform_cursor.start().0;
                        // transform_cursor.next();
                    }
                    Some(Transform::Filter { .. }) => {
                        // push_filter(&mut new_transforms, summary);
                        // transform_cursor.next();
                    }
                    None => {}
                }
            }

            // If this is the last edit that intersects the current transform, consume the remainder of the transform and advance.
            if buffer_edits.peek().is_none_or(|next_buffer_edit| {
                next_buffer_edit.old.start >= transform_cursor.end().0
            }) {
                let suffix_start = new_transforms.summary().input.0.len;
                let suffix_len = transform_cursor.end().0 - buffer_edit.old.end;
                let summary = buffer_snapshot.text_summary_for_range(
                    suffix_start..std::cmp::min(suffix_start + suffix_len, buffer_snapshot.len()),
                );
                push_isomorphic(&mut new_transforms, summary);
                transform_cursor.next();
            }

            output_edits.push(text::Edit {
                old: edit_old_start..edit_old_end,
                new: edit_new_start..edit_new_end,
            })
        }

        // Append old transforms after the last edit.

        log::info!(
            "input len before suffix is {}",
            new_transforms.summary().input.0.len
        );
        let suffix = transform_cursor.suffix();
        log::info!("suffix summary is {:?}", suffix.summary());
        new_transforms.append(suffix, ());
        log::info!(
            "input len after suffix is {}",
            new_transforms.summary().input.0.len
        );

        drop(transform_cursor);

        self.snapshot.transforms = new_transforms;
        self.snapshot.buffer_snapshot = buffer_snapshot;
        #[cfg(debug_assertions)]
        self.check_invariants();
        (self.snapshot.clone(), output_edits)
    }
}

fn text_summaries_for_diff_hunk(
    hunk: &multi_buffer::MultiBufferDiffHunk,
    buffer_snapshot: &MultiBufferSnapshot,
) -> (TextSummary, TextSummary) {
    // TODO does it make sense to do this in terms of points?
    let start_of_hunk = Point::new(hunk.row_range.start.0, 0);
    let switch_point = hunk
        .multi_buffer_range()
        .start
        .bias_right(&buffer_snapshot)
        .to_point(&buffer_snapshot);
    let end_of_hunk = Point::new(hunk.row_range.end.0, 0);
    let deletion_summary = buffer_snapshot.text_summary_for_range(start_of_hunk..switch_point);
    let addition_summary = buffer_snapshot.text_summary_for_range(switch_point..end_of_hunk);
    (deletion_summary, addition_summary)
}

impl FilterSnapshot {
    #[cfg(any(test, feature = "test-support"))]
    fn text(&self) -> String {
        let mut offset = 0;
        let mut output = String::new();

        for transform in self.transforms.iter() {
            match transform {
                Transform::Isomorphic { summary } => {
                    let strs = self.buffer_snapshot.text_for_range(
                        offset.min(self.buffer_snapshot.len())
                            ..(offset + summary.0.len).min(self.buffer_snapshot.len()),
                    );

                    for s in strs {
                        output.push_str(s);
                    }

                    offset += summary.0.len;
                }
                Transform::Filter { summary } => {
                    offset += summary.0.len;
                }
            }
        }

        output
    }
}

fn push_isomorphic(transforms: &mut SumTree<Transform>, summary_to_add: TextSummary) {
    log::info!(
        "push_isomorphic, input len after push is {}",
        transforms.summary().input.0.len + summary_to_add.len
    );
    let mut merged = false;
    transforms.update_last(
        |transform| {
            if let Transform::Isomorphic { summary } = transform {
                summary.0 += summary_to_add;
                merged = true;
            }
        },
        (),
    );
    if !merged {
        transforms.push(
            Transform::Isomorphic {
                summary: WholeLineTextSummary(summary_to_add),
            },
            (),
        );
    }
}

fn push_filter(transforms: &mut SumTree<Transform>, summary_to_add: TextSummary) {
    log::info!(
        "push_filter, input len after push is {}",
        transforms.summary().input.0.len + summary_to_add.len
    );
    let mut merged = false;
    transforms.update_last(
        |transform| {
            if let Transform::Filter { summary } = transform {
                summary.0 += summary_to_add;
                merged = true;
            }
        },
        (),
    );
    if !merged {
        transforms.push(
            Transform::Filter {
                summary: WholeLineTextSummary(summary_to_add),
            },
            (),
        );
    }
}

impl FilterSnapshot {
    fn text_summary_for_range(&self, range: Range<FilterOffset>) -> TextSummary {
        let mut summary = TextSummary::default();

        let mut cursor = self
            .transforms
            .cursor::<Dimensions<FilterOffset, usize>>(());
        cursor.seek(&range.start, Bias::Right);

        let overshoot = range.start.0 - cursor.start().0.0;
        match cursor.item() {
            Some(Transform::Isomorphic { .. }) => {
                let buffer_start = cursor.start().1;
                let suffix_start = buffer_start + overshoot;
                let suffix_end =
                    buffer_start + (cmp::min(cursor.end().0, range.end).0 - cursor.start().0.0);
                summary = self
                    .buffer_snapshot
                    .text_summary_for_range(suffix_start..suffix_end);
                cursor.next();
            }
            Some(Transform::Filter { .. }) | None => {}
        }

        if range.end > cursor.start().0 {
            summary += cursor
                .summary::<_, TransformSummary>(&range.end, Bias::Right)
                .output
                .0;

            let overshoot = range.end.0 - cursor.start().0.0;
            match cursor.item() {
                Some(Transform::Isomorphic { .. }) => {
                    let prefix_start = cursor.start().1;
                    let prefix_end = prefix_start + overshoot;
                    summary += self
                        .buffer_snapshot
                        .text_summary_for_range::<TextSummary, _>(prefix_start..prefix_end);
                }
                Some(Transform::Filter { .. }) | None => {}
            }
        }

        summary
    }

    fn to_point(&self, offset: FilterOffset) -> FilterPoint {
        let (start, _, item) = self
            .transforms
            .find::<Dimensions<FilterOffset, FilterPoint, usize>, _>((), &offset, Bias::Right);
        let overshoot = offset.0 - start.0.0;
        match item {
            Some(Transform::Isomorphic { .. }) => {
                let buffer_offset_start = start.2;
                let buffer_offset_end = buffer_offset_start + overshoot;
                let buffer_start = self.buffer_snapshot.offset_to_point(buffer_offset_start);
                let buffer_end = self.buffer_snapshot.offset_to_point(buffer_offset_end);
                FilterPoint(start.1.0 + (buffer_end - buffer_start))
            }
            Some(Transform::Filter { .. }) | None => self.max_point(),
        }
    }

    fn max_point(&self) -> FilterPoint {
        FilterPoint(self.transforms.summary().output.0.lines)
    }
}

#[cfg(test)]
mod tests {
    use collections::HashMap;
    use gpui::{AppContext as _, Entity};
    use language::{Buffer, Capability};
    use multi_buffer::{MultiBuffer, MultiBufferSnapshot, randomly_mutate_multibuffer_with_diffs};
    use rand::{Rng as _, rngs::StdRng};
    use text::BufferId;

    use crate::display_map::filter_map::{FilterMap, FilterMode, FilterSnapshot};

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
}
