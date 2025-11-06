use std::{cmp, ops::Range};

use buffer_diff::DiffHunkStatusKind;
use multi_buffer::{
    AnchorRangeExt as _, MultiBufferDiffHunk, MultiBufferSnapshot, ToOffset as _, ToPoint as _,
};
use rope::{Point, TextSummary};
use sum_tree::{Dimensions, SumTree};
use text::Bias;
use util::{RangeExt as _, debug_panic};

#[derive(Debug, Clone)]
enum Transform {
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

impl Transform {
    fn is_isomorphic(&self) -> bool {
        matches!(self, Transform::Isomorphic { .. })
    }

    #[cfg(test)]
    fn text(&self) -> &str {
        match self {
            Self::Isomorphic { text, .. } => text,
            Self::Filter { text, .. } => text,
        }
    }
}

impl sum_tree::Item for Transform {
    type Summary = TransformSummary;

    fn summary(&self, cx: <Self::Summary as sum_tree::Summary>::Context<'_>) -> Self::Summary {
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
        self.0 += summary.output.len;
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
        *self += summary.input.len;
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
                    Transform::Isomorphic {
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
            self.snapshot.buffer_snapshot.text()
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
        let Some(mode) = self.mode else {
            // If we're not filtering out anything, edits can be passed through
            // unchanged and we only need one isomorphic transform.
            self.snapshot.buffer_snapshot = buffer_snapshot.clone();
            self.snapshot.transforms = SumTree::from_item(
                Transform::Isomorphic {
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

        let mut new_transforms: SumTree<Transform> = SumTree::new(());
        let mut transform_cursor = self
            .snapshot
            .transforms
            .cursor::<Dimensions<usize, FilterOffset>>(());
        let mut output_edits = Vec::new();

        // TODO in what follows we repeatedly call text_summary_for_range,
        // could use a persistent usize cursor over buffer_snapshot instead.

        transform_cursor.next();

        let mut buffer_edits = buffer_edits.into_iter().peekable();

        // We may need to extend/remove some of the buffer edits depending on
        // how they line up with the hunks. Consider the following case:
        //
        // hunks: |--unchanged---|-----deleted----|-----added------|
        // edits: <-1->     <-----------2----------->    <-3->  <---4--->
        //
        // Edit 2 overlaps the start of the added region, but doesn't cover the
        // whole region. This means that the anchor for that region will be
        // invalidated, but not all of the region will be covered by an edit,
        // and so will not have its hunk info invalidated.
        //
        // To solve this, we need to extend edit 2 so that it covers all of the
        // added region, while ensuring it doesn't overlap any other edits. For
        // edit 3, we can just delete it entirely, since it's fully contained
        // within the hunk. However, for edit 4, we "merge" edit 2 with it,
        // extending edit 2 so that it covers all of edit 4. However, we need to
        // be careful, since edit 4 might ALSO be a bad edit (i.e. it overlaps
        // with a hunk in the same way edit 2 does). In this case, we need to
        // repeat the process until we get a clean hunk end.
        while let Some(mut buffer_edit) = buffer_edits.next() {
            //                         hunk buffer range start
            //                         v
            //         |-deleted range-|-added range-| hunk
            //                            <------>
            //  <------------------------->            buffer_edit.old (what we currently invalidate)
            //                             <-------------------------->
            //
            //
            //                 <------------------------------->

            // while let Some(prev_diff_hunk_start) = self
            //     .snapshot
            //     .buffer_snapshot
            //     .diff_hunk_before(buffer_edit.old.end)
            //     && buffer_edit
            //         .old
            //         .contains(&prev_diff_hunk_start.to_offset(&self.snapshot.buffer_snapshot))
            // {
            //     let mut revised_edit_old_end: usize = prev_diff_hunk.end;
            //     let mut revised_edit_new_end: usize =
            //         buffer_edit.new.end + (revised_edit_old_end - buffer_edit.old.end);
            //     while let Some(next_edit) = buffer_edits.peek()
            //         && next_edit.old.start < revised_edit_old_end
            //     {
            //         revised_edit_old_end = revised_edit_old_end.max(next_edit.old.end);
            //         revised_edit_new_end = revised_edit_new_end.max(next_edit.new.end);
            //         buffer_edits.next();
            //     }
            //     buffer_edit.old.end = revised_edit_old_end;
            //     buffer_edit.new.end = revised_edit_new_end;
            // }

            debug_assert!(transform_cursor.start().0 <= buffer_edit.old.end);

            // Reuse any old transforms that strictly precede the start of the edit.
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

            let mut edit_old_start = transform_cursor.start().1;
            let mut edit_new_start = FilterOffset(new_transforms.summary().output.len);

            // If the edit starts in the middle of a transform, split the transform and push the unaffected portion.
            if buffer_edit.new.start > new_transforms.summary().input.len {
                let range = new_transforms.summary().input.len..buffer_edit.new.start;
                match transform_cursor.item() {
                    Some(Transform::Isomorphic { .. }) => {
                        let summary = push_isomorphic(&mut new_transforms, range, &buffer_snapshot);
                        edit_old_start.0 += summary.len;
                        edit_new_start.0 += summary.len;
                    }
                    Some(Transform::Filter { .. }) => {
                        push_filter(&mut new_transforms, range, &buffer_snapshot);
                    }
                    None => {}
                }
            }

            // Process the edited range based on diff hunks.
            for hunk in buffer_snapshot.diff_hunks_in_range(buffer_edit.new.clone()) {
                let (deletion_range, addition_range) = diff_hunk_bounds(&hunk, &buffer_snapshot);
                let deletion_range = deletion_range.clamp(buffer_edit.new.clone());
                let addition_range = addition_range.clamp(buffer_edit.new.clone());
                // Push an isomorphic transform for any content preceding this hunk.
                let prefix_range = new_transforms.summary().input.len..deletion_range.start;
                push_isomorphic(&mut new_transforms, prefix_range, &buffer_snapshot);

                match mode {
                    FilterMode::RemoveDeletions => {
                        push_filter(&mut new_transforms, deletion_range, &buffer_snapshot);
                        push_isomorphic(&mut new_transforms, addition_range, &buffer_snapshot);
                    }
                    FilterMode::RemoveInsertions => {
                        push_isomorphic(&mut new_transforms, deletion_range, &buffer_snapshot);
                        push_filter(&mut new_transforms, addition_range, &buffer_snapshot);
                    }
                }
            }

            // Push any non-hunk content after the last hunk.
            if buffer_edit.new.end > new_transforms.summary().input.len {
                let suffix_range = new_transforms.summary().input.len..buffer_edit.new.end;
                push_isomorphic(&mut new_transforms, suffix_range, &buffer_snapshot);
            }

            transform_cursor.seek(&buffer_edit.old.end, Bias::Right);
            let mut edit_old_end = transform_cursor.end().1;
            let edit_new_end = FilterOffset(new_transforms.summary().output.len);
            if buffer_edit.old.end > transform_cursor.start().0 {
                match transform_cursor.item() {
                    Some(Transform::Isomorphic { .. }) => {
                        edit_old_end.0 += buffer_edit.old.end - transform_cursor.start().0;
                    }
                    Some(Transform::Filter { .. }) => {}
                    None => {}
                }
            }

            // If this is the last edit that intersects the current transform, consume the remainder of the transform and advance.
            if buffer_edits.peek().is_none_or(|next_buffer_edit| {
                next_buffer_edit.old.start >= transform_cursor.end().0
            }) {
                let suffix_start = new_transforms.summary().input.len;
                let suffix_len = transform_cursor.end().0 - buffer_edit.old.end;
                match transform_cursor.item() {
                    Some(Transform::Isomorphic { .. }) => {
                        push_isomorphic(
                            &mut new_transforms,
                            suffix_start
                                ..std::cmp::min(suffix_start + suffix_len, buffer_snapshot.len()),
                            &buffer_snapshot,
                        );
                        transform_cursor.next();
                    }
                    Some(Transform::Filter { .. }) => {
                        push_filter(
                            &mut new_transforms,
                            suffix_start
                                ..std::cmp::min(suffix_start + suffix_len, buffer_snapshot.len()),
                            &buffer_snapshot,
                        );
                        transform_cursor.next();
                    }
                    None => {}
                }
            }

            output_edits.push(text::Edit {
                old: edit_old_start..edit_old_end,
                new: edit_new_start..edit_new_end,
            })
        }

        // Append old transforms after the last edit.

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

        self.snapshot.transforms = new_transforms;
        self.snapshot.buffer_snapshot = buffer_snapshot;
        #[cfg(debug_assertions)]
        self.check_invariants();
        // TODO assert that the output edits are well-formed and that applying them transforms the old snapshot into the new snapshot
        (self.snapshot.clone(), output_edits)
    }
}

fn diff_hunk_bounds(
    hunk: &MultiBufferDiffHunk,
    buffer_snapshot: &MultiBufferSnapshot,
) -> (Range<usize>, Range<usize>) {
    let start_of_hunk = hunk
        .multi_buffer_range()
        .start
        .bias_left(&buffer_snapshot)
        .to_offset(&buffer_snapshot);
    let switch_point = hunk
        .multi_buffer_range()
        .start
        .bias_right(&buffer_snapshot)
        .to_offset(&buffer_snapshot);
    let end_of_hunk = buffer_snapshot.point_to_offset(Point::new(hunk.row_range.end.0, 0));
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
                Transform::Isomorphic { text, .. } => Some(text.as_str()),
                Transform::Filter { .. } => None,
            })
            .collect()
    }

    #[cfg(test)]
    fn input_text(&self) -> String {
        self.transforms.iter().map(|t| t.text()).collect()
    }
}

fn push_isomorphic(
    transforms: &mut SumTree<Transform>,
    range: Range<usize>,
    snapshot: &MultiBufferSnapshot,
) -> TextSummary {
    if range.is_empty() {
        return TextSummary::default();
    }

    let summary_to_add = snapshot.text_summary_for_range::<TextSummary, _>(range.clone());
    #[cfg(test)]
    let text_to_add = snapshot.text_for_range(range).collect::<String>();

    #[cfg(test)]
    log::info!("push_isomorphic, pushing:\n{text_to_add:?}",);

    let mut merged = false;
    transforms.update_last(
        |transform| {
            if let Transform::Isomorphic {
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
            Transform::Isomorphic {
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
    transforms: &mut SumTree<Transform>,
    range: Range<usize>,
    snapshot: &MultiBufferSnapshot,
) {
    if range.is_empty() {
        return;
    }

    let summary_to_add = snapshot.text_summary_for_range::<TextSummary, _>(range.clone());
    #[cfg(test)]
    let text_to_add = snapshot.text_for_range(range).collect::<String>();

    #[cfg(test)]
    log::info!("push_filter, pushing:\n{text_to_add:?}",);

    let mut merged = false;
    transforms.update_last(
        |transform| {
            if let Transform::Filter {
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
            Transform::Filter {
                summary: summary_to_add,
                #[cfg(test)]
                text: text_to_add,
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
                .output;

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
        FilterPoint(self.transforms.summary().output.lines)
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
