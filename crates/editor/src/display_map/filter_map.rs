use std::{cmp, ops::Range};

use language::Chunk;
use multi_buffer::{
    MultiBufferDiffHunk, MultiBufferPoint, MultiBufferRow, MultiBufferRows, MultiBufferSnapshot,
    RowInfo, ToOffset, ToPoint as _,
};
use rope::{Point, TextSummary};
use sum_tree::{Dimensions, SumTree};
use text::Bias;
use util::RangeExt as _;

use crate::display_map::{Highlights, custom_highlights::CustomHighlightsChunks};

/// A [`FilterTransform`] represents a (potentially filtered) region in the
/// [`FilterMap`].
///
/// The [`FilterMap`] maintains an invariant that no two consecutive transforms
/// are the same type (i.e. no two consecutive `Isomorphic`s or `Filter`s).
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
    #[cfg(test)]
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

pub(crate) struct FilterMap {
    snapshot: FilterSnapshot,
    mode: Option<FilterMode>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum FilterMode {
    RemoveDeletions,
    RemoveInsertions,
}

impl FilterMode {
    #[cfg(test)]
    fn should_remove(self, kind: buffer_diff::DiffHunkStatusKind) -> bool {
        use buffer_diff::DiffHunkStatusKind;

        match kind {
            DiffHunkStatusKind::Added => self == FilterMode::RemoveInsertions,
            DiffHunkStatusKind::Modified => {
                panic!("unexpected modified status in row infos");
            }
            DiffHunkStatusKind::Deleted => self == FilterMode::RemoveDeletions,
        }
    }
}

#[derive(Clone)]
pub(crate) struct FilterSnapshot {
    transforms: SumTree<FilterTransform>,
    pub(crate) buffer_snapshot: MultiBufferSnapshot,
    pub(crate) version: usize,
}

/// A byte index into the buffer (after ignored diff hunk lines are deleted)
#[derive(Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash, Default)]
pub(crate) struct FilterOffset(pub(crate) usize);

impl std::fmt::Debug for FilterOffset {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FilterOffset({})", self.0)
    }
}

impl FilterOffset {
    /// Convert a range of offsets to a range of [`FilterOffset`]s, given that
    /// there are no filtered lines in the buffer (for example, if no
    /// [`FilterMode`] is set).
    fn naive_range(Range { start, end }: Range<usize>) -> Range<Self> {
        FilterOffset(start)..FilterOffset(end)
    }
}

impl std::ops::Add for FilterOffset {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        FilterOffset(self.0 + rhs.0)
    }
}

impl std::ops::Sub for FilterOffset {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        FilterOffset(self.0 - rhs.0)
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
        _: <TransformSummary as sum_tree::Summary>::Context<'_>,
    ) {
        *self += &summary.input.lines;
    }
}

#[derive(Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Default)]
pub(crate) struct FilterPoint(pub(crate) Point);

impl std::ops::Add for FilterPoint {
    type Output = Self;

    fn add(mut self, rhs: Self) -> Self::Output {
        self += rhs;
        self
    }
}

impl std::ops::AddAssign<FilterPoint> for FilterPoint {
    fn add_assign(&mut self, rhs: FilterPoint) {
        self.0 += rhs.0;
    }
}

impl std::ops::Sub for FilterPoint {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        FilterPoint(self.0 - rhs.0)
    }
}

impl sum_tree::Dimension<'_, TransformSummary> for FilterPoint {
    fn zero(_: <TransformSummary as sum_tree::Summary>::Context<'_>) -> Self {
        FilterPoint(Point::zero())
    }

    fn add_summary(
        &mut self,
        summary: &'_ TransformSummary,
        _: <TransformSummary as sum_tree::Summary>::Context<'_>,
    ) {
        self.0 += summary.output.lines;
    }
}

impl sum_tree::Dimension<'_, TransformSummary> for usize {
    fn zero(_: <TransformSummary as sum_tree::Summary>::Context<'_>) -> Self {
        0
    }

    fn add_summary(
        &mut self,
        summary: &'_ TransformSummary,
        _: <TransformSummary as sum_tree::Summary>::Context<'_>,
    ) {
        *self += summary.input.len;
    }
}

pub(crate) type FilterEdit = text::Edit<FilterOffset>;

impl FilterMap {
    pub(crate) fn new(
        mode: Option<FilterMode>,
        buffer_snapshot: MultiBufferSnapshot,
    ) -> (Self, FilterSnapshot) {
        let mut this = Self {
            mode,
            snapshot: FilterSnapshot {
                version: 0,
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
        let snapshot = this.snapshot.clone();
        (this, snapshot)
    }

    #[cfg(test)]
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
    pub(crate) fn sync(
        &mut self,
        buffer_snapshot: MultiBufferSnapshot,
        mut buffer_edits: Vec<text::Edit<usize>>,
    ) -> (FilterSnapshot, Vec<FilterEdit>) {
        if buffer_edits.is_empty()
            && self
                .snapshot
                .buffer_snapshot
                .trailing_excerpt_update_count()
                != buffer_snapshot.trailing_excerpt_update_count()
        {
            buffer_edits.push(text::Edit {
                old: self.snapshot.buffer_snapshot.len()..self.snapshot.buffer_snapshot.len(),
                new: buffer_snapshot.len()..buffer_snapshot.len(),
            });
        }

        if buffer_edits.is_empty() {
            if self.snapshot.buffer_snapshot.edit_count() != buffer_snapshot.edit_count()
                || self.snapshot.buffer_snapshot.non_text_state_update_count()
                    != buffer_snapshot.non_text_state_update_count()
                || self
                    .snapshot
                    .buffer_snapshot
                    .trailing_excerpt_update_count()
                    != buffer_snapshot.trailing_excerpt_update_count()
            {
                self.snapshot.version += 1;
            }

            self.snapshot.buffer_snapshot = buffer_snapshot;
            return (self.snapshot.clone(), Vec::new());
        }

        // Passthrough case where nothing is filtered.
        let Some(mode) = self.mode else {
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

            // Reuse old transforms before the start of the edit.
            new_transforms.append(
                transform_cursor.slice(&buffer_edit.old.start, Bias::Left),
                (),
            );

            // Compute the start of the edit in the old snapshot.
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

            // Compute the start of the edit in the new snapshot.
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

            // Process the edited range based on diff hunks. Query for hunks
            // anywhere on the row containing the start of the edit, since these
            // can affect the diff status of the entire row.
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

            // Compute the end of the edit in the old snapshot.
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

        new_transforms.append(transform_cursor.suffix(), ());

        drop(transform_cursor);

        let new_snapshot = FilterSnapshot {
            transforms: new_transforms,
            buffer_snapshot,
            version: self.snapshot.version + 1,
        };
        #[cfg(test)]
        check_edits(&self.snapshot, &output_edits, &new_snapshot);
        self.snapshot = new_snapshot;
        #[cfg(test)]
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
            use sum_tree::Item as _;

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

    pub(crate) fn max_row(&self) -> FilterRow {
        FilterRow(self.max_point().0.row)
    }

    pub(crate) fn to_filter_point(&self, point: MultiBufferPoint) -> FilterPoint {
        todo!()
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
    pub(crate) fn to_filter_offset(&self, offset: usize) -> FilterOffset {
        let (start, _end, item) =
            self.transforms
                .find::<Dimensions<usize, FilterOffset>, _>((), &offset, Bias::Right);
        match item {
            Some(FilterTransform::Isomorphic { .. }) => {
                let overshoot = offset - start.0;
                start.1 + FilterOffset(overshoot)
            }
            Some(FilterTransform::Filter { .. }) | None => start.1,
        }
    }

    pub(crate) fn offset_to_point(&self, offset: FilterOffset) -> FilterPoint {
        let (start, _end, item) = self
            .transforms
            .find::<Dimensions<FilterOffset, FilterPoint, usize>, _>((), &offset, Bias::Right);
        match item {
            Some(FilterTransform::Isomorphic { .. }) => {
                let buffer_start = self.buffer_snapshot.offset_to_point(start.2);
                let overshoot = offset.0 - start.0.0;
                let buffer_end = self.buffer_snapshot.offset_to_point(start.2 + overshoot);
                FilterPoint(start.1.0 + (buffer_end - buffer_start))
            }
            Some(FilterTransform::Filter { .. }) => start.1,
            None => self.max_point(),
        }
    }

    pub(crate) fn point_to_offset(&self, point: FilterPoint) -> FilterOffset {
        type D = Dimensions<FilterPoint, FilterOffset, MultiBufferPoint>;
        let (start, _end, item) = self.transforms.find::<D, _>((), &point, Bias::Right);
        match item {
            Some(FilterTransform::Isomorphic { .. }) => {
                let buffer_start = self.buffer_snapshot.point_to_offset(start.2);
                let overshoot = point.0 - start.0.0;
                let buffer_end = self.buffer_snapshot.point_to_offset(start.2 + overshoot);
                FilterOffset(start.1.0 + (buffer_end - buffer_start))
            }
            Some(FilterTransform::Filter { .. }) => start.1,
            None => self.len(),
        }
    }

    pub(crate) fn max_point(&self) -> FilterPoint {
        FilterPoint(self.transforms.summary().output.lines)
    }

    pub(crate) fn len(&self) -> FilterOffset {
        FilterOffset(self.transforms.summary().output.len)
    }

    pub(crate) fn text_summary(&self) -> TextSummary {
        self.transforms.summary().output
    }

    pub(crate) fn text_summary_for_range(&self, range: Range<FilterOffset>) -> TextSummary {
        let mut summary = TextSummary::default();

        let mut cursor = self
            .transforms
            .cursor::<Dimensions<FilterOffset, usize>>(());
        cursor.seek(&range.start, Bias::Right);

        let overshoot = range.start.0 - cursor.start().0.0;
        match cursor.item() {
            Some(FilterTransform::Isomorphic { .. }) => {
                let buffer_start = cursor.start().1;
                let suffix_start = buffer_start + overshoot;
                let suffix_end =
                    buffer_start + (cmp::min(cursor.end().0, range.end).0 - cursor.start().0.0);
                summary = self
                    .buffer_snapshot
                    .text_summary_for_range(suffix_start..suffix_end);
                cursor.next();
            }
            Some(FilterTransform::Filter { .. }) | None => {}
        }

        if range.end > cursor.start().0 {
            summary += cursor
                .summary::<_, TransformSummary>(&range.end, Bias::Right)
                .output;

            let overshoot = range.end.0 - cursor.start().0.0;
            match cursor.item() {
                Some(FilterTransform::Isomorphic { .. }) => {
                    let prefix_start = cursor.start().1;
                    let prefix_end = prefix_start + overshoot;
                    summary += self
                        .buffer_snapshot
                        .text_summary_for_range::<TextSummary, _>(prefix_start..prefix_end);
                }
                Some(FilterTransform::Filter { .. }) | None => {}
            }
        }

        summary
    }

    // todo! check later
    /// Constrain a [`FilterPoint`] to a valid location in this snapshot.
    ///
    /// For example, if the first line in a snapshot had 50 characters, the
    /// point 0:100 would be clipped to 0:50 . This function trivially "takes
    /// into account" filtered lines, since the `point` parameter is already a
    /// [`FilterPoint`], which always refers to non-filtered lines.
    pub(crate) fn clip_point(&self, point: FilterPoint, bias: Bias) -> FilterPoint {
        let (start, _end, item) = self
            .transforms
            .find::<Dimensions<FilterPoint, MultiBufferPoint>, _>((), &point, Bias::Right);
        match item {
            Some(FilterTransform::Isomorphic { .. }) => {
                let overshoot = point.0 - start.0.0;
                let clipped_buffer_point =
                    self.buffer_snapshot.clip_point(start.1 + overshoot, bias);
                FilterPoint(start.0.0 + (clipped_buffer_point - start.1))
            }
            Some(FilterTransform::Filter { .. }) => {
                debug_assert_eq!(point, start.0);
                debug_assert!(
                    false,
                    "a cursor.find() searching for `FilterPoint`s should never stop on a `FilterTransform::Filter`"
                );
                start.0
            }
            None => self.max_point(),
        }
    }

    /// Returns true if the anchor resolves to an offset strictly inside a filtered region,
    /// or if it resolves to one end of a filtered region and the bias points into the filtered region.
    pub(crate) fn is_anchor_filtered(&self, anchor: multi_buffer::Anchor) -> bool {
        let offset = anchor.to_offset(&self.buffer_snapshot);
        let mut cursor = self.transforms.cursor::<usize>(());
        cursor.seek(&offset, Bias::Right);
        if let Some(prev_item) = cursor.prev_item()
            && matches!(prev_item, FilterTransform::Filter { .. })
        {
            cursor.prev();
        }
        match cursor.item() {
            Some(FilterTransform::Filter { .. }) => {
                *cursor.start() < offset
                    || (*cursor.start() == offset && anchor.bias() == Bias::Right)
                    || offset < cursor.end()
                    || (offset == cursor.end() && anchor.bias() == Bias::Left)
            }
            Some(FilterTransform::Isomorphic { .. }) | None => false,
        }
    }

    /// Translates a filter offset to a buffer offset. If there is a filtered region at the given offset,
    /// uses the bias to decide whether the start or end of the filtered region should be returned.
    pub(crate) fn to_buffer_offset(&self, offset: FilterOffset, bias: Bias) -> usize {
        let mut cursor = self
            .transforms
            .cursor::<Dimensions<FilterOffset, usize>>(());
        cursor.seek(&offset, Bias::Right);
        if let Some(prev_item) = cursor.prev_item()
            && matches!(prev_item, FilterTransform::Filter { .. })
        {
            cursor.prev();
        }
        match cursor.item() {
            Some(FilterTransform::Isomorphic { .. }) => {
                let overshoot = offset.0 - cursor.start().0.0;
                cursor.start().1 + overshoot
            }
            Some(FilterTransform::Filter { .. }) => match bias {
                Bias::Left => cursor.start().1,
                Bias::Right => cursor.end().1,
            },
            None => self.buffer_snapshot.len(),
        }
    }

    /// Translates a filter point to a buffer point. If there is a filtered region at the given point,
    /// uses the bias to decide whether the start or end of the filtered region should be returned.
    pub(crate) fn to_buffer_point(&self, point: FilterPoint, bias: Bias) -> MultiBufferPoint {
        let mut cursor = self
            .transforms
            .cursor::<Dimensions<FilterPoint, MultiBufferPoint>>(());
        cursor.seek(&point, Bias::Right);
        if let Some(prev_item) = cursor.prev_item()
            && matches!(prev_item, FilterTransform::Filter { .. })
        {
            cursor.prev();
        }
        match cursor.item() {
            Some(FilterTransform::Isomorphic { .. }) => {
                let overshoot = point.0 - cursor.start().0.0;
                cursor.start().1 + overshoot
            }
            Some(FilterTransform::Filter { .. }) => match bias {
                Bias::Left => cursor.start().1,
                Bias::Right => cursor.end().1,
            },
            None => self.buffer_snapshot.max_point(),
        }
    }

    pub(crate) fn row_infos(&self, start_row: FilterRow) -> FilterRows<'_> {
        FilterRows {
            transform_cursor: self.transforms.cursor(()),
            buffer_rows: self.buffer_snapshot.row_infos(MultiBufferRow(0)),
            next_row: start_row,
            snapshot: self,
        }
    }

    pub(crate) fn chunks<'a>(
        &'a self,
        range: Range<FilterOffset>,
        language_aware: bool,
        highlights: &Highlights<'a>,
    ) -> FilterChunks<'a> {
        let mut cursor = self
            .transforms
            .cursor::<'_, '_, Dimensions<FilterOffset, usize>>(());
        cursor.next();
        if matches!(cursor.item(), Some(FilterTransform::Filter { .. })) {
            cursor.next();
        }
        let buffer_chunks = CustomHighlightsChunks::new(
            cursor.start().1..cursor.end().1,
            language_aware,
            highlights.text_highlights,
            &self.buffer_snapshot,
        );
        let mut chunks = FilterChunks {
            cursor,
            buffer_chunks,
            snapshot: self,
        };
        chunks.seek(range);
        chunks
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct FilterRow(pub u32);

impl std::fmt::Debug for FilterRow {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FilterRow({})", self.0)
    }
}

#[derive(Clone)]
pub(crate) struct FilterRows<'a> {
    transform_cursor:
        sum_tree::Cursor<'a, 'static, FilterTransform, Dimensions<FilterPoint, MultiBufferPoint>>,
    snapshot: &'a FilterSnapshot,
    buffer_rows: MultiBufferRows<'a>,
    next_row: FilterRow,
}

impl<'a> Iterator for FilterRows<'a> {
    type Item = RowInfo;

    fn next(&mut self) -> Option<Self::Item> {
        let target = FilterPoint(Point::new(self.next_row.0, 0));
        self.transform_cursor.seek_forward(&target, Bias::Right);
        let mut buffer_start = self.transform_cursor.start().1;
        match self.transform_cursor.item()? {
            FilterTransform::Isomorphic { .. } => {
                let overshoot = target.0 - self.transform_cursor.start().0.0;
                buffer_start += overshoot;
            }
            FilterTransform::Filter { .. } => {}
        }
        self.buffer_rows.seek(MultiBufferRow(buffer_start.row));
        let info = self.buffer_rows.next()?;
        self.next_row.0 += 1;
        Some(info)
    }
}

impl FilterRows<'_> {
    pub fn seek(&mut self, row: FilterRow) {
        if row.0 < self.next_row.0 {
            self.transform_cursor = self.snapshot.transforms.cursor(());
        }
        self.next_row = row;
    }
}

pub(crate) struct FilterChunks<'a> {
    cursor: sum_tree::Cursor<'a, 'static, FilterTransform, Dimensions<FilterOffset, usize>>,
    buffer_chunks: CustomHighlightsChunks<'a>,
    snapshot: &'a FilterSnapshot,
}

impl FilterChunks<'_> {
    /// Ensures that `self.cursor.item()` returns a
    /// [`FilterTransform::Isomorphic`]. Returns whether `self.cursor` actually
    /// moved (`None` if the cursor is not currently on an item).
    ///
    /// This relies on the fact that the filter map maintains an invariant that
    /// there are no two consecutive transforms of different kinds.
    fn ensure_on_isomorphic(&mut self) -> Option<bool> {
        match self.cursor.item()? {
            FilterTransform::Filter { .. } => {
                self.cursor.next();
                Some(true)
            }
            FilterTransform::Isomorphic { .. } => Some(false),
        }
    }
}

impl<'a> Iterator for FilterChunks<'a> {
    type Item = Chunk<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        // Note: MultiBufferChunks ensures that no chunk crosses a diff transform boundary,
        // so we don't have to worry about splitting chunk here.
        let did_move = self.ensure_on_isomorphic()?;
        if did_move {
            let range = self.cursor.start().1..self.cursor.end().1;
            self.buffer_chunks.seek(range);
        }
        let buffer_chunk = loop {
            if let Some(chunk) = self.buffer_chunks.next() {
                break chunk;
            }

            // Exhausted the isomorphic transform, move on to the next one.
            self.cursor.next();
            self.ensure_on_isomorphic()?;

            // Note: this `.seek()` call will move the "iterable range" of
            // `self.buffer_chunks`, so `self.buffer_chunks.next()` may start
            // returning `Some` again.
            let range = self.cursor.start().1..self.cursor.end().1;
            self.buffer_chunks.seek(range);
        };

        Some(buffer_chunk)
    }
}

impl<'a> FilterChunks<'a> {
    pub(crate) fn seek(&mut self, range: Range<FilterOffset>) {
        self.cursor.seek(&range.start, Bias::Right);
        let overshoot = range.start.0 - self.cursor.start().0.0;
        let buffer_start = self.cursor.start().1 + overshoot;
        let buffer_end = self.cursor.end().1;
        self.buffer_chunks.seek(buffer_start..buffer_end);
    }

    // fn naive_seek(&mut self, range: Range<FilterOffset>) {
    //     *self = self
    //         .snapshot
    //         .chunks(range, self.language_aware, self.highlights);
    //     self.reset();
    //     while !range.contains(&self.cursor.start().0) {
    //         self.next();
    //     }
    // }

    // fn reset(&mut self) {
    //     *self = self.snapshot.chunks(range, language_aware, highlights)
    // }
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
        let (mut filter_map, _) = FilterMap::new(
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

        let (mut filter_map, _) = FilterMap::new(
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
