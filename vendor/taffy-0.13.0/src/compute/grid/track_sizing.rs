//! Implements the track sizing algorithm
//! <https://www.w3.org/TR/css-grid-1/#layout-algorithm>
use super::types::{GridItem, GridTrack, TrackCounts};
use crate::geometry::{AbstractAxis, Line, Size};
use crate::style::{AlignContent, AlignContentKeyword, AlignSelf, AvailableSpace};
use crate::style_helpers::TaffyMinContent;
use crate::tree::{LayoutPartialTree, LayoutPartialTreeExt, SizingMode};
use crate::util::sys::{f32_max, f32_min, Vec};
use crate::util::{MaybeMath, ResolveOrZero};
use crate::CompactLength;
use core::cmp::Ordering;

/// Takes an axis, and a list of grid items sorted firstly by whether they cross a flex track
/// in the specified axis (items that don't cross a flex track first) and then by the number
/// of tracks they cross in specified axis (ascending order).
struct ItemBatcher {
    /// The axis in which the ItemBatcher is operating. Used when querying properties from items.
    axis: AbstractAxis,
    /// The starting index of the current batch
    index_offset: usize,
    /// The span of the items in the current batch
    current_span: u16,
    /// Whether the current batch of items cross a flexible track
    current_is_flex: bool,
}

impl ItemBatcher {
    /// Create a new ItemBatcher for the specified axis
    #[inline(always)]
    fn new(axis: AbstractAxis) -> Self {
        ItemBatcher { index_offset: 0, axis, current_span: 1, current_is_flex: false }
    }

    /// This is basically a manual version of Iterator::next which passes `items`
    /// in as a parameter on each iteration to work around borrow checker rules
    #[inline]
    fn next<'items>(&mut self, items: &'items mut [GridItem]) -> Option<(&'items mut [GridItem], bool)> {
        if self.current_is_flex || self.index_offset >= items.len() {
            return None;
        }

        let item = &items[self.index_offset];
        self.current_span = item.span(self.axis);
        self.current_is_flex = item.crosses_flexible_track(self.axis);

        let next_index_offset = if self.current_is_flex {
            items.len()
        } else {
            items
                .iter()
                .position(|item: &GridItem| {
                    item.crosses_flexible_track(self.axis) || item.span(self.axis) > self.current_span
                })
                .unwrap_or(items.len())
        };

        let batch_range = self.index_offset..next_index_offset;
        self.index_offset = next_index_offset;

        let batch = &mut items[batch_range];
        Some((batch, self.current_is_flex))
    }
}

/// This struct captures a bunch of variables which are used to compute the intrinsic sizes of children so that those variables
/// don't have to be passed around all over the place below. It then has methods that implement the intrinsic sizing computations
struct IntrinsicSizeMeasurer<'tree, 'oat, Tree, EstimateFunction>
where
    Tree: LayoutPartialTree,
    EstimateFunction: Fn(&GridTrack, Option<f32>, &Tree) -> Option<f32>,
{
    /// The layout tree
    tree: &'tree mut Tree,
    /// The tracks in the opposite axis to the one we are currently sizing
    other_axis_tracks: &'oat [GridTrack],
    /// A function that computes an estimate of an other-axis track's size which is passed to
    /// the child size measurement functions
    get_track_size_estimate: EstimateFunction,
    /// The axis we are currently sizing
    axis: AbstractAxis,
    /// The available grid space
    inner_node_size: Size<Option<f32>>,
}

impl<Tree, EstimateFunction> IntrinsicSizeMeasurer<'_, '_, Tree, EstimateFunction>
where
    Tree: LayoutPartialTree,
    EstimateFunction: Fn(&GridTrack, Option<f32>, &Tree) -> Option<f32>,
{
    /// Compute the available_space to be passed to the child sizing functions
    /// These are estimates based on either the max track sizing function or the provisional base size in the opposite
    /// axis to the one currently being sized.
    /// https://www.w3.org/TR/css-grid-1/#algo-overview
    #[inline(always)]
    fn grid_area_size(&self, item: &mut GridItem, axis_tracks: &[GridTrack]) -> Size<Option<f32>> {
        item.grid_area_size_cached(
            self.axis,
            axis_tracks,
            self.other_axis_tracks,
            self.inner_node_size,
            |track, basis| (self.get_track_size_estimate)(track, basis, self.tree),
            &|val, basis| self.tree.calc(val, basis),
        )
    }

    /// Compute the item's resolved margins for size contributions. Horizontal percentage margins always resolve
    /// to zero if the container size is indefinite as otherwise this would introduce a cyclic dependency.
    #[inline(always)]
    fn margins_axis_sums_with_baseline_shims(&self, item: &GridItem, percentage_basis: Option<f32>) -> Size<f32> {
        item.margins_axis_sums_with_baseline_shims(percentage_basis, self.tree)
    }

    /// Simple pass-through function to `LayoutPartialTreeExt::calc`
    #[inline(always)]
    fn calc(&self, val: *const (), basis: f32) -> f32 {
        self.tree.calc(val, basis)
    }

    /// Retrieve the item's min content contribution from the cache or compute it using the provided parameters
    #[inline(always)]
    fn min_content_contribution(&mut self, item: &mut GridItem, axis_tracks: &[GridTrack]) -> f32 {
        let grid_area_size = self.grid_area_size(item, axis_tracks);
        let available_space = grid_area_size.with(self.axis, None);
        let margin_axis_sums = self.margins_axis_sums_with_baseline_shims(item, available_space.width);
        let contribution = item.min_content_contribution_cached(self.axis, self.tree, grid_area_size, available_space);
        contribution + margin_axis_sums.get(self.axis)
    }

    /// Retrieve the item's max content contribution from the cache or compute it using the provided parameters
    #[inline(always)]
    fn max_content_contribution(&mut self, item: &mut GridItem, axis_tracks: &[GridTrack]) -> f32 {
        let grid_area_size = self.grid_area_size(item, axis_tracks);
        let available_space = grid_area_size.with(self.axis, None);
        let margin_axis_sums = self.margins_axis_sums_with_baseline_shims(item, available_space.width);
        let contribution = item.max_content_contribution_cached(self.axis, self.tree, grid_area_size, available_space);
        contribution + margin_axis_sums.get(self.axis)
    }

    /// The minimum contribution of an item is the smallest outer size it can have.
    /// Specifically:
    ///   - If the item’s computed preferred size behaves as auto or depends on the size of its containing block in the relevant axis:
    ///     Its minimum contribution is the outer size that would result from assuming the item’s used minimum size as its preferred size;
    ///   - Else the item’s minimum contribution is its min-content contribution.
    ///
    /// Because the minimum contribution often depends on the size of the item’s content, it is considered a type of intrinsic size contribution.
    #[inline(always)]
    fn minimum_contribution(&mut self, item: &mut GridItem, axis_tracks: &[GridTrack]) -> f32 {
        let grid_area_size = self.grid_area_size(item, axis_tracks);
        let available_space = grid_area_size.with(self.axis, None);
        let margin_axis_sums = self.margins_axis_sums_with_baseline_shims(item, available_space.width);
        let contribution =
            item.minimum_contribution_cached(self.tree, self.axis, axis_tracks, grid_area_size, self.inner_node_size);
        contribution + margin_axis_sums.get(self.axis)
    }
}

/// To make track sizing efficient we want to order tracks
/// Here a placement is either a Line<i16> representing a row-start/row-end or a column-start/column-end
#[inline(always)]
pub(super) fn cmp_by_cross_flex_then_span_then_start(
    axis: AbstractAxis,
) -> impl FnMut(&GridItem, &GridItem) -> Ordering {
    move |item_a: &GridItem, item_b: &GridItem| -> Ordering {
        match (item_a.crosses_flexible_track(axis), item_b.crosses_flexible_track(axis)) {
            (false, true) => Ordering::Less,
            (true, false) => Ordering::Greater,
            _ => {
                let placement_a = item_a.placement(axis);
                let placement_b = item_b.placement(axis);
                match placement_a.span().cmp(&placement_b.span()) {
                    Ordering::Less => Ordering::Less,
                    Ordering::Greater => Ordering::Greater,
                    Ordering::Equal => placement_a.start.cmp(&placement_b.start),
                }
            }
        }
    }
}

/// When applying the track sizing algorithm and estimating the size in the other axis for content sizing items
/// we should take into account align-content/justify-content if both the grid container and all items in the
/// other axis have definite sizes. This function computes such a per-gutter additional size adjustment.
#[inline(always)]
pub(super) fn compute_alignment_gutter_adjustment(
    alignment: AlignContent,
    axis_inner_node_size: Option<f32>,
    get_track_size_estimate: impl Fn(&GridTrack, Option<f32>) -> Option<f32>,
    tracks: &[GridTrack],
) -> f32 {
    if tracks.len() <= 1 {
        return 0.0;
    }

    // As items never cross the outermost gutters in a grid, we can simplify our calculations by
    // treating Start and End the same. The safety modifier doesn't influence gutter weight;
    // overflow fallback is handled when offsets are computed.
    let outer_gutter_weight = match alignment.keyword() {
        AlignContentKeyword::Start
        | AlignContentKeyword::FlexStart
        | AlignContentKeyword::End
        | AlignContentKeyword::FlexEnd
        | AlignContentKeyword::Center => 1,
        AlignContentKeyword::Stretch => 0,
        AlignContentKeyword::SpaceBetween => 0,
        AlignContentKeyword::SpaceAround => 1,
        AlignContentKeyword::SpaceEvenly => 1,
    };

    let inner_gutter_weight = match alignment.keyword() {
        AlignContentKeyword::FlexStart
        | AlignContentKeyword::Start
        | AlignContentKeyword::FlexEnd
        | AlignContentKeyword::End
        | AlignContentKeyword::Center
        | AlignContentKeyword::Stretch => 0,
        AlignContentKeyword::SpaceBetween => 1,
        AlignContentKeyword::SpaceAround => 2,
        AlignContentKeyword::SpaceEvenly => 1,
    };

    if inner_gutter_weight == 0 {
        return 0.0;
    }

    if let Some(axis_inner_node_size) = axis_inner_node_size {
        let free_space = tracks
            .iter()
            .map(|track| get_track_size_estimate(track, Some(axis_inner_node_size)))
            .sum::<Option<f32>>()
            .map(|track_size_sum| f32_max(0.0, axis_inner_node_size - track_size_sum))
            .unwrap_or(0.0);

        let weighted_track_count =
            (((tracks.len() - 3) / 2) * inner_gutter_weight as usize) + (2 * outer_gutter_weight as usize);

        return (free_space / weighted_track_count as f32) * inner_gutter_weight as f32;
    }

    0.0
}

/// Convert origin-zero coordinates track placement in grid track vector indexes
#[inline(always)]
pub(super) fn resolve_item_track_indexes(items: &mut [GridItem], column_counts: TrackCounts, row_counts: TrackCounts) {
    for item in items {
        item.column_indexes = item.column.map(|line| line.into_track_vec_index(column_counts) as u16);
        item.row_indexes = item.row.map(|line| line.into_track_vec_index(row_counts) as u16);
    }
}

/// Determine (in each axis) whether the item crosses any flexible tracks
#[inline(always)]
pub(super) fn determine_if_item_crosses_flexible_or_intrinsic_tracks(
    items: &mut Vec<GridItem>,
    columns: &[GridTrack],
    rows: &[GridTrack],
) {
    for item in items {
        item.crosses_flexible_column =
            item.track_range_excluding_lines(AbstractAxis::Inline).any(|i| columns[i].is_flexible());
        item.crosses_intrinsic_column =
            item.track_range_excluding_lines(AbstractAxis::Inline).any(|i| columns[i].has_intrinsic_sizing_function());
        item.crosses_flexible_row =
            item.track_range_excluding_lines(AbstractAxis::Block).any(|i| rows[i].is_flexible());
        item.crosses_intrinsic_row =
            item.track_range_excluding_lines(AbstractAxis::Block).any(|i| rows[i].has_intrinsic_sizing_function());
    }
}

/// Track sizing algorithm
/// Note: Gutters are treated as empty fixed-size tracks for the purpose of the track sizing algorithm.
#[allow(clippy::too_many_arguments)]
pub(super) fn track_sizing_algorithm<Tree: LayoutPartialTree>(
    tree: &mut Tree,
    axis: AbstractAxis,
    axis_min_size: Option<f32>,
    axis_max_size: Option<f32>,
    axis_alignment: AlignContent,
    other_axis_alignment: AlignContent,
    available_grid_space: Size<AvailableSpace>,
    inner_node_size: Size<Option<f32>>,
    axis_tracks: &mut [GridTrack],
    other_axis_tracks: &mut [GridTrack],
    items: &mut [GridItem],
    get_track_size_estimate: fn(&GridTrack, Option<f32>, &Tree) -> Option<f32>,
    has_baseline_aligned_item: bool,
) {
    // 11.4 Initialise Track sizes
    // Initialize each track’s base size and growth limit.
    let percentage_basis = inner_node_size.get(axis).or(axis_min_size);
    initialize_track_sizes(tree, axis_tracks, percentage_basis);

    // 11.5.1 Shim item baselines
    if has_baseline_aligned_item {
        resolve_item_baselines(tree, axis, items, inner_node_size);
    }

    // If all tracks have base_size = growth_limit, then skip the rest of this function.
    // Note: this can only happen both track sizing function have the same fixed track sizing function
    if axis_tracks.iter().all(|track| track.base_size == track.growth_limit) {
        return;
    }

    // Pre-computations for 11.5 Resolve Intrinsic Track Sizes

    // Compute an additional amount to add to each spanned gutter when computing item's estimated size in the
    // in the opposite axis based on the alignment, container size, and estimated track sizes in that axis
    let gutter_alignment_adjustment = compute_alignment_gutter_adjustment(
        other_axis_alignment,
        inner_node_size.get(axis.other()),
        |track, basis| get_track_size_estimate(track, basis, tree),
        other_axis_tracks,
    );
    if other_axis_tracks.len() > 3 {
        let len = other_axis_tracks.len();
        let inner_gutter_tracks = other_axis_tracks[2..len].iter_mut().step_by(2);
        for track in inner_gutter_tracks {
            track.content_alignment_adjustment = gutter_alignment_adjustment;
        }
    }

    // 11.5 Resolve Intrinsic Track Sizes
    resolve_intrinsic_track_sizes(
        tree,
        axis,
        axis_tracks,
        other_axis_tracks,
        items,
        available_grid_space.get(axis),
        inner_node_size,
        get_track_size_estimate,
    );

    // 11.6. Maximise Tracks
    // Distributes free space (if any) to tracks with FINITE growth limits, up to their limits.
    maximise_tracks(axis_tracks, inner_node_size.get(axis), available_grid_space.get(axis));

    // For the purpose of the final two expansion steps ("Expand Flexible Tracks" and "Stretch auto Tracks"), we only want to expand
    // into space generated by the grid container's size (as defined by either it's preferred size style or by it's parent node through
    // something like stretch alignment), not just any available space. To do this we map definite available space to AvailableSpace::MaxContent
    // in the case that inner_node_size is None
    let axis_available_space_for_expansion = if let Some(available_space) = inner_node_size.get(axis) {
        AvailableSpace::Definite(available_space)
    } else {
        match available_grid_space.get(axis) {
            AvailableSpace::MinContent => AvailableSpace::MinContent,
            AvailableSpace::MaxContent | AvailableSpace::Definite(_) => AvailableSpace::MaxContent,
        }
    };

    // 11.7. Expand Flexible Tracks
    // This step sizes flexible tracks using the largest value it can assign to an fr without exceeding the available space.
    expand_flexible_tracks(
        tree,
        axis,
        axis_tracks,
        items,
        axis_min_size,
        axis_max_size,
        axis_available_space_for_expansion,
    );

    // 11.8. Stretch auto Tracks
    // This step expands tracks that have an auto max track sizing function by dividing any remaining positive, definite free space equally amongst them.
    if axis_alignment == AlignContent::STRETCH {
        stretch_auto_tracks(axis_tracks, axis_min_size, axis_available_space_for_expansion);
    }
}

/// Whether it is a minimum or maximum size's space being distributed
/// This controls behaviour of the space distribution algorithm when distributing beyond limits
/// See "distributing space beyond limits" at https://www.w3.org/TR/css-grid-1/#extra-space
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum IntrinsicContributionType {
    /// It's a minimum size's space being distributed
    Minimum,
    /// It's a maximum size's space being distributed
    Maximum,
}

/// Add any planned base size increases to the base size after a round of distributing space to base sizes
/// Reset the planed base size increase to zero ready for the next round.
#[inline(always)]
fn flush_planned_base_size_increases(tracks: &mut [GridTrack]) {
    for track in tracks {
        track.base_size += track.base_size_planned_increase;
        track.base_size_planned_increase = 0.0;
    }
}

/// Add any planned growth limit increases to the growth limit after a round of distributing space to growth limits
/// Reset the planed growth limit increase to zero ready for the next round.
#[inline(always)]
fn flush_planned_growth_limit_increases(tracks: &mut [GridTrack], set_infinitely_growable: bool) {
    for track in tracks {
        if track.growth_limit_planned_increase > 0.0 {
            track.growth_limit = if track.growth_limit == f32::INFINITY {
                track.base_size + track.growth_limit_planned_increase
            } else {
                track.growth_limit + track.growth_limit_planned_increase
            };
            track.infinitely_growable = set_infinitely_growable;
        } else {
            track.infinitely_growable = false;
        }
        track.growth_limit_planned_increase = 0.0
    }
}

/// 11.4 Initialise Track sizes
/// Initialize each track’s base size and growth limit.
#[inline(always)]
fn initialize_track_sizes(
    tree: &impl LayoutPartialTree,
    axis_tracks: &mut [GridTrack],
    axis_inner_node_size: Option<f32>,
) {
    for track in axis_tracks.iter_mut() {
        // For each track, if the track’s min track sizing function is:
        // - A fixed sizing function
        //     Resolve to an absolute length and use that size as the track’s initial base size.
        //     Note: Indefinite lengths cannot occur, as they’re treated as auto.
        // - An intrinsic sizing function
        //     Use an initial base size of zero.
        track.base_size = track
            .min_track_sizing_function
            .definite_value(axis_inner_node_size, |val, basis| tree.calc(val, basis))
            .unwrap_or(0.0);

        // For each track, if the track’s max track sizing function is:
        // - A fixed sizing function
        //     Resolve to an absolute length and use that size as the track’s initial growth limit.
        // - An intrinsic sizing function
        //     Use an initial growth limit of infinity.
        // - A flexible sizing function
        //     Use an initial growth limit of infinity.
        track.growth_limit = track
            .max_track_sizing_function
            .definite_value(axis_inner_node_size, |val, basis| tree.calc(val, basis))
            .unwrap_or(f32::INFINITY);

        // In all cases, if the growth limit is less than the base size, increase the growth limit to match the base size.
        if track.growth_limit < track.base_size {
            track.growth_limit = track.base_size;
        }
    }
}

/// 11.5.1 Shim baseline-aligned items so their intrinsic size contributions reflect their baseline alignment.
fn resolve_item_baselines(
    tree: &mut impl LayoutPartialTree,
    axis: AbstractAxis,
    items: &mut [GridItem],
    inner_node_size: Size<Option<f32>>,
) {
    // Sort items by track in the other axis (row) start position so that we can iterate items in groups which
    // are in the same track in the other axis (row)
    let other_axis = axis.other();
    items.sort_by_key(|item| item.placement(other_axis).start);

    // Iterate over grid rows
    let mut remaining_items = &mut items[0..];
    while !remaining_items.is_empty() {
        // Get the row index of the current row
        let current_row = remaining_items[0].placement(other_axis).start;

        // Find the item index of the first item that is in a different row (or None if we've reached the end of the list)
        let next_row_first_item =
            remaining_items.iter().position(|item| item.placement(other_axis).start != current_row);

        // Use this index to split the `remaining_items` slice in two slices:
        //    - A `row_items` slice containing the items (that start) in the current row
        //    - A new `remaining_items` consisting of the remainder of the `remaining_items` slice
        //      that hasn't been split off into `row_items
        let row_items = if let Some(index) = next_row_first_item {
            let (row_items, tail) = remaining_items.split_at_mut(index);
            remaining_items = tail;
            row_items
        } else {
            let row_items = remaining_items;
            remaining_items = &mut [];
            row_items
        };

        // Count how many items in *this row* are baseline aligned
        // If a row has one or zero items participating in baseline alignment then baseline alignment is a no-op
        // for those items and we skip further computations for that row
        let row_baseline_item_count = row_items.iter().filter(|item| item.align_self == AlignSelf::BASELINE).count();
        if row_baseline_item_count <= 1 {
            continue;
        }

        // Compute the baselines of all items in the row
        for item in row_items.iter_mut() {
            let measured_size_and_baselines = tree.perform_child_layout(
                item.node,
                Size::NONE,
                inner_node_size,
                Size::MIN_CONTENT,
                SizingMode::InherentSize,
                Line::FALSE,
            );

            let baseline = measured_size_and_baselines.first_baselines.y;
            let height = measured_size_and_baselines.size.height;

            item.baseline = Some(
                baseline.unwrap_or(height)
                    + item.margin.top.resolve_or_zero(inner_node_size.width, |val, basis| tree.calc(val, basis)),
            );
        }

        // Compute the max baseline of all items in the row
        let row_max_baseline =
            row_items.iter().map(|item| item.baseline.unwrap_or(0.0)).max_by(|a, b| a.total_cmp(b)).unwrap();

        // Compute the baseline shim for each item in the row
        for item in row_items.iter_mut() {
            item.baseline_shim = row_max_baseline - item.baseline.unwrap_or(0.0);
        }
    }
}

/// 11.5 Resolve Intrinsic Track Sizes
#[allow(clippy::too_many_arguments)]
fn resolve_intrinsic_track_sizes<Tree: LayoutPartialTree>(
    tree: &mut Tree,
    axis: AbstractAxis,
    axis_tracks: &mut [GridTrack],
    other_axis_tracks: &[GridTrack],
    items: &mut [GridItem],
    axis_available_grid_space: AvailableSpace,
    inner_node_size: Size<Option<f32>>,
    get_track_size_estimate: impl Fn(&GridTrack, Option<f32>, &Tree) -> Option<f32>,
) {
    // Step 1. Shim baseline-aligned items so their intrinsic size contributions reflect their baseline alignment.

    // Already done at this point. See resolve_item_baselines function.

    // Step 2.

    // The track sizing algorithm requires us to iterate through the items in ascending order of the number of
    // tracks they span (first items that span 1 track, then items that span 2 tracks, etc).
    // To avoid having to do multiple iterations of the items, we pre-sort them into this order.
    items.sort_by(cmp_by_cross_flex_then_span_then_start(axis));

    // Step 2, Step 3 and Step 4
    // 2 & 3. Iterate over items that don't cross a flex track. Items should have already been sorted in ascending order
    // of the number of tracks they span. Step 2 is the 1 track case and has an optimised implementation
    // 4. Next, repeat the previous step instead considering (together, rather than grouped by span size) all items
    // that do span a track with a flexible sizing function while

    // Compute item's intrinsic (content-based) sizes
    // Note: For items with a specified minimum size of auto (the initial value), the minimum contribution is usually equivalent
    // to the min-content contribution—but can differ in some cases, see §6.6 Automatic Minimum Size of Grid Items.
    // Also, minimum contribution <= min-content contribution <= max-content contribution.

    let axis_inner_node_size = inner_node_size.get(axis);
    let mut item_sizer =
        IntrinsicSizeMeasurer { tree, other_axis_tracks, axis, inner_node_size, get_track_size_estimate };

    let mut batched_item_iterator = ItemBatcher::new(axis);
    while let Some((batch, is_flex)) = batched_item_iterator.next(items) {
        // 2. Size tracks to fit non-spanning items: For each track with an intrinsic track sizing function and not a flexible sizing function,
        // consider the items in it with a span of 1:
        let batch_span = batch[0].placement(axis).span();
        if !is_flex && batch_span == 1 {
            for item in batch.iter_mut() {
                let track_index = item.placement_indexes(axis).start + 1;
                let track = &axis_tracks[track_index as usize];

                // Handle base sizes
                let new_base_size = match track.min_track_sizing_function.0.tag() {
                    CompactLength::MIN_CONTENT_TAG => {
                        f32_max(track.base_size, item_sizer.min_content_contribution(item, axis_tracks))
                    }
                    // If the container size is indefinite and has not yet been resolved then percentage sized
                    // tracks should be treated as min-content (this matches Chrome's behaviour and seems sensible)
                    CompactLength::PERCENT_TAG => {
                        if axis_inner_node_size.is_none() {
                            f32_max(track.base_size, item_sizer.min_content_contribution(item, axis_tracks))
                        } else {
                            track.base_size
                        }
                    }
                    CompactLength::MAX_CONTENT_TAG => {
                        f32_max(track.base_size, item_sizer.max_content_contribution(item, axis_tracks))
                    }
                    CompactLength::AUTO_TAG => {
                        let space = match axis_available_grid_space {
                            // QUIRK: The spec says that:
                            //
                            //   If the grid container is being sized under a min- or max-content constraint, use the items’ limited
                            //   min-content contributions in place of their minimum contributions here.
                            //
                            // However, in practice browsers only seem to apply this rule if the item is not a scroll container
                            // (note that overflow:hidden counts as a scroll container), giving the automatic minimum size of scroll
                            // containers (zero) precedence over the min-content contributions.
                            AvailableSpace::MinContent | AvailableSpace::MaxContent
                                if !item.overflow.get(axis).is_scroll_container() =>
                            {
                                let axis_minimum_size = item_sizer.minimum_contribution(item, axis_tracks);
                                let axis_min_content_size = item_sizer.min_content_contribution(item, axis_tracks);
                                let limit = track
                                    .max_track_sizing_function
                                    .definite_limit(axis_inner_node_size, |val, basis| item_sizer.calc(val, basis));
                                axis_min_content_size.maybe_min(limit).max(axis_minimum_size)
                            }
                            _ => item_sizer.minimum_contribution(item, axis_tracks),
                        };
                        f32_max(track.base_size, space)
                    }
                    CompactLength::LENGTH_TAG => {
                        // Do nothing as it's not an intrinsic track sizing function
                        track.base_size
                    }
                    // Handle calc() like percentage
                    #[cfg(feature = "calc")]
                    _ if track.min_track_sizing_function.0.is_calc() => {
                        if axis_inner_node_size.is_none() {
                            f32_max(track.base_size, item_sizer.min_content_contribution(item, axis_tracks))
                        } else {
                            track.base_size
                        }
                    }
                    _ => unreachable!(),
                };
                let growth_limit_min_content_contribution = if !item.overflow.get(axis).is_scroll_container() {
                    Some(item_sizer.min_content_contribution(item, axis_tracks))
                } else {
                    None
                };
                let growth_limit_max_content_contribution = item_sizer.max_content_contribution(item, axis_tracks);
                let growth_limit_intrinsic_min_content_contribution =
                    item_sizer.min_content_contribution(item, axis_tracks);
                let track = &mut axis_tracks[track_index as usize];
                track.base_size = new_base_size;

                // Handle growth limits
                if track.max_track_sizing_function.is_fit_content() {
                    // If item is not a scroll container, then increase the growth limit to at least the
                    // size of the min-content contribution
                    if let Some(min_content_contribution) = growth_limit_min_content_contribution {
                        track.growth_limit_planned_increase =
                            f32_max(track.growth_limit_planned_increase, min_content_contribution);
                    }

                    // Always increase the growth limit to at least the size of the *fit-content limited*
                    // max-content contribution
                    let fit_content_limit = track.fit_content_limit(axis_inner_node_size);
                    let max_content_contribution = f32_min(growth_limit_max_content_contribution, fit_content_limit);
                    track.growth_limit_planned_increase =
                        f32_max(track.growth_limit_planned_increase, max_content_contribution);
                } else if track.max_track_sizing_function.is_max_content_alike()
                    || track.max_track_sizing_function.uses_percentage() && axis_inner_node_size.is_none()
                {
                    // If the container size is indefinite and has not yet been resolved then percentage sized
                    // tracks should be treated as auto (this matches Chrome's behaviour and seems sensible)
                    track.growth_limit_planned_increase =
                        f32_max(track.growth_limit_planned_increase, growth_limit_max_content_contribution);
                } else if track.max_track_sizing_function.is_intrinsic() {
                    track.growth_limit_planned_increase =
                        f32_max(track.growth_limit_planned_increase, growth_limit_intrinsic_min_content_contribution);
                }
            }

            for track in axis_tracks.iter_mut() {
                if track.growth_limit_planned_increase > 0.0 {
                    track.growth_limit = if track.growth_limit == f32::INFINITY {
                        track.growth_limit_planned_increase
                    } else {
                        f32_max(track.growth_limit, track.growth_limit_planned_increase)
                    };
                }
                track.infinitely_growable = false;
                track.growth_limit_planned_increase = 0.0;
                if track.growth_limit < track.base_size {
                    track.growth_limit = track.base_size;
                }
            }

            continue;
        }

        // 1. For intrinsic minimums:
        // First increase the base size of tracks with an intrinsic min track sizing function
        for item in batch.iter_mut().filter(|item| item.crosses_intrinsic_track(axis)) {
            // ...by distributing extra space as needed to accommodate these items’ minimum contributions.
            //
            // QUIRK: The spec says that:
            //
            //   If the grid container is being sized under a min- or max-content constraint, use the items’ limited min-content contributions
            //   in place of their minimum contributions here.
            //
            // However, in practice browsers only seem to apply this rule if the item is not a scroll container (note that overflow:hidden counts as
            // a scroll container), giving the automatic minimum size of scroll containers (zero) precedence over the min-content contributions.
            let space = match axis_available_grid_space {
                AvailableSpace::MinContent | AvailableSpace::MaxContent
                    if !item.overflow.get(axis).is_scroll_container() =>
                {
                    let axis_minimum_size = item_sizer.minimum_contribution(item, axis_tracks);
                    let axis_min_content_size = item_sizer.min_content_contribution(item, axis_tracks);
                    let limit = item.spanned_track_limit(axis, axis_tracks, axis_inner_node_size, &|val, basis| {
                        item_sizer.calc(val, basis)
                    });
                    axis_min_content_size.maybe_min(limit).max(axis_minimum_size)
                }
                _ => item_sizer.minimum_contribution(item, axis_tracks),
            };
            let tracks = &mut axis_tracks[item.track_range_excluding_lines(axis)];
            if space > 0.0 {
                let has_intrinsic_min_track_sizing_function = |track: &GridTrack| {
                    track
                        .min_track_sizing_function
                        .definite_value(axis_inner_node_size, |val, basis| item_sizer.calc(val, basis))
                        .is_none()
                };
                if item.overflow.get(axis).is_scroll_container() {
                    let fit_content_limit =
                        move |track: &GridTrack| track.fit_content_limited_growth_limit(axis_inner_node_size);
                    distribute_item_space_to_base_size(
                        is_flex,
                        space,
                        tracks,
                        has_intrinsic_min_track_sizing_function,
                        fit_content_limit,
                        IntrinsicContributionType::Minimum,
                        axis_inner_node_size,
                    );
                } else {
                    distribute_item_space_to_base_size(
                        is_flex,
                        space,
                        tracks,
                        has_intrinsic_min_track_sizing_function,
                        |track| track.growth_limit,
                        IntrinsicContributionType::Minimum,
                        axis_inner_node_size,
                    );
                }
            }
        }
        flush_planned_base_size_increases(axis_tracks);

        // 2. For content-based minimums:
        // Next continue to increase the base size of tracks with a min track sizing function of min-content or max-content
        // by distributing extra space as needed to account for these items' min-content contributions.
        let has_min_or_max_content_min_track_sizing_function =
            move |track: &GridTrack| track.min_track_sizing_function.is_min_or_max_content();
        for item in batch.iter_mut() {
            let space = item_sizer.min_content_contribution(item, axis_tracks);
            let tracks = &mut axis_tracks[item.track_range_excluding_lines(axis)];
            if space > 0.0 {
                if item.overflow.get(axis).is_scroll_container() {
                    let fit_content_limit =
                        move |track: &GridTrack| track.fit_content_limited_growth_limit(axis_inner_node_size);
                    distribute_item_space_to_base_size(
                        is_flex,
                        space,
                        tracks,
                        has_min_or_max_content_min_track_sizing_function,
                        fit_content_limit,
                        IntrinsicContributionType::Minimum,
                        axis_inner_node_size,
                    );
                } else {
                    distribute_item_space_to_base_size(
                        is_flex,
                        space,
                        tracks,
                        has_min_or_max_content_min_track_sizing_function,
                        |track| track.growth_limit,
                        IntrinsicContributionType::Minimum,
                        axis_inner_node_size,
                    );
                }
            }
        }
        flush_planned_base_size_increases(axis_tracks);

        // 3. For max-content minimums:

        // If the grid container is being sized under a max-content constraint, continue to increase the base size of tracks with
        // a min track sizing function of auto or max-content by distributing extra space as needed to account for these items'
        // limited max-content contributions.

        // Define fit_content_limited_growth_limit function. This is passed to the distribute_space_up_to_limits
        // helper function, and is used to compute the limit to distribute up to for each track.
        // Wrapping the method on GridTrack is necessary in order to resolve percentage fit-content arguments.
        if axis_available_grid_space == AvailableSpace::MaxContent {
            /// Whether a track:
            ///   - has an Auto MIN track sizing function
            ///   - Does not have a MinContent MAX track sizing function
            ///
            /// The latter condition was added in order to match Chrome. But I believe it is due to the provision
            /// under minmax here https://www.w3.org/TR/css-grid-1/#track-sizes which states that:
            ///
            ///    "If the max is less than the min, then the max will be floored by the min (essentially yielding minmax(min, min))"
            #[inline(always)]
            fn has_auto_min_track_sizing_function(track: &GridTrack) -> bool {
                track.min_track_sizing_function.is_auto() && !track.max_track_sizing_function.is_min_content()
            }

            /// Whether a track has a MaxContent min track sizing function
            #[inline(always)]
            fn has_max_content_min_track_sizing_function(track: &GridTrack) -> bool {
                track.min_track_sizing_function.is_max_content()
            }

            for item in batch.iter_mut() {
                let axis_max_content_size = item_sizer.max_content_contribution(item, axis_tracks);
                let limit = item.spanned_track_limit(axis, axis_tracks, axis_inner_node_size, &|val, basis| {
                    item_sizer.calc(val, basis)
                });
                let space = axis_max_content_size.maybe_min(limit);
                let tracks = &mut axis_tracks[item.track_range_excluding_lines(axis)];
                if space > 0.0 {
                    // If any of the tracks spanned by the item have a MaxContent min track sizing function then
                    // distribute space only to those tracks. Otherwise distribute space to tracks with an Auto min
                    // track sizing function.
                    //
                    // Note: this prioritisation of MaxContent over Auto is not mentioned in the spec (which suggests that
                    // we ought to distribute space evenly between MaxContent and Auto tracks). But it is implemented like
                    // this in both Chrome and Firefox (and it does have a certain logic to it), so we implement it too for
                    // compatibility.
                    //
                    // See: https://www.w3.org/TR/css-grid-1/#track-size-max-content-min
                    if tracks.iter().any(has_max_content_min_track_sizing_function) {
                        distribute_item_space_to_base_size(
                            is_flex,
                            space,
                            tracks,
                            has_max_content_min_track_sizing_function,
                            |_| f32::INFINITY,
                            IntrinsicContributionType::Maximum,
                            axis_inner_node_size,
                        );
                    } else {
                        let fit_content_limited_growth_limit =
                            move |track: &GridTrack| track.fit_content_limited_growth_limit(axis_inner_node_size);
                        distribute_item_space_to_base_size(
                            is_flex,
                            space,
                            tracks,
                            has_auto_min_track_sizing_function,
                            fit_content_limited_growth_limit,
                            IntrinsicContributionType::Maximum,
                            axis_inner_node_size,
                        );
                    }
                }
            }
            flush_planned_base_size_increases(axis_tracks);
        }

        // In all cases, continue to increase the base size of tracks with a min track sizing function of max-content by distributing
        // extra space as needed to account for these items' max-content contributions.
        let has_max_content_min_track_sizing_function =
            move |track: &GridTrack| track.min_track_sizing_function.is_max_content();
        for item in batch.iter_mut() {
            let axis_max_content_size = item_sizer.max_content_contribution(item, axis_tracks);
            let space = axis_max_content_size;
            let tracks = &mut axis_tracks[item.track_range_excluding_lines(axis)];
            if space > 0.0 {
                distribute_item_space_to_base_size(
                    is_flex,
                    space,
                    tracks,
                    has_max_content_min_track_sizing_function,
                    |track| track.growth_limit,
                    IntrinsicContributionType::Maximum,
                    axis_inner_node_size,
                );
            }
        }
        flush_planned_base_size_increases(axis_tracks);

        // 4. If at this point any track’s growth limit is now less than its base size, increase its growth limit to match its base size.
        for track in axis_tracks.iter_mut() {
            if track.growth_limit < track.base_size {
                track.growth_limit = track.base_size;
            }
        }

        // If a track is a flexible track, then it has flexible max track sizing function
        // It cannot also have an intrinsic max track sizing function, so these steps do not apply.
        if !is_flex {
            // 5. For intrinsic maximums: Next increase the growth limit of tracks with an intrinsic max track sizing function by
            // distributing extra space as needed to account for these items' min-content contributions.
            let has_intrinsic_max_track_sizing_function =
                move |track: &GridTrack| !track.max_track_sizing_function.has_definite_value(axis_inner_node_size);
            for item in batch.iter_mut() {
                let axis_min_content_size = item_sizer.min_content_contribution(item, axis_tracks);
                let space = axis_min_content_size;
                let tracks = &mut axis_tracks[item.track_range_excluding_lines(axis)];
                if space > 0.0 {
                    distribute_item_space_to_growth_limit(
                        space,
                        tracks,
                        has_intrinsic_max_track_sizing_function,
                        inner_node_size.get(axis),
                    );
                }
            }
            // Mark any tracks whose growth limit changed from infinite to finite in this step as infinitely growable for the next step.
            flush_planned_growth_limit_increases(axis_tracks, true);

            // 6. For max-content maximums: Lastly continue to increase the growth limit of tracks with a max track sizing function of max-content
            // by distributing extra space as needed to account for these items' max-content contributions. However, limit the growth of any
            // fit-content() tracks by their fit-content() argument.
            let has_max_content_max_track_sizing_function = |track: &GridTrack| {
                track.max_track_sizing_function.is_max_content_alike()
                    || (track.max_track_sizing_function.uses_percentage() && axis_inner_node_size.is_none())
            };
            for item in batch.iter_mut() {
                let axis_max_content_size = item_sizer.max_content_contribution(item, axis_tracks);
                let space = axis_max_content_size;
                let tracks = &mut axis_tracks[item.track_range_excluding_lines(axis)];
                if space > 0.0 {
                    distribute_item_space_to_growth_limit(
                        space,
                        tracks,
                        has_max_content_max_track_sizing_function,
                        inner_node_size.get(axis),
                    );
                }
            }
            // Mark any tracks whose growth limit changed from infinite to finite in this step as infinitely growable for the next step.
            flush_planned_growth_limit_increases(axis_tracks, false);
        }
    }

    // Step 5. If any track still has an infinite growth limit (because, for example, it had no items placed
    // in it or it is a flexible track), set its growth limit to its base size.
    // NOTE: this step is super-important to ensure that the "Maximise Tracks" step doesn't affect flexible tracks
    axis_tracks
        .iter_mut()
        .filter(|track| track.growth_limit == f32::INFINITY)
        .for_each(|track| track.growth_limit = track.base_size);
}

/// 11.5.1. Distributing Extra Space Across Spanned Tracks
/// https://www.w3.org/TR/css-grid-1/#extra-space
#[inline(always)]
fn distribute_item_space_to_base_size(
    is_flex: bool,
    space: f32,
    tracks: &mut [GridTrack],
    track_is_affected: impl Fn(&GridTrack) -> bool,
    track_limit: impl Fn(&GridTrack) -> f32,
    intrinsic_contribution_type: IntrinsicContributionType,
    axis_inner_node_size: Option<f32>,
) {
    if is_flex {
        let filter = |track: &GridTrack| track.is_flexible() && track_is_affected(track);

        // If the sum of the flex factors of the affected tracks is greater than zero, distribute
        // space according to the ratios of the tracks' flex factors. Otherwise distribute space equally.
        //
        // Note: the spec says to compute the sum over all flexible tracks spanned by the item, and to
        // blend flex-factor-proportional and equal distribution when the sum is below one. But Chrome
        // computes the sum over only the affected tracks and uses pure flex-factor ratios whenever
        // that sum is non-zero, so we do the same for compatibility.
        let flex_factor_sum: f32 = tracks.iter().filter(|track| filter(track)).map(|track| track.flex_factor()).sum();
        if flex_factor_sum > 0.0 {
            distribute_item_space_to_base_size_inner(
                space,
                tracks,
                filter,
                |track| track.flex_factor(),
                track_limit,
                intrinsic_contribution_type,
                axis_inner_node_size,
            )
        } else {
            distribute_item_space_to_base_size_inner(
                space,
                tracks,
                filter,
                |_| 1.0,
                track_limit,
                intrinsic_contribution_type,
                axis_inner_node_size,
            )
        }
    } else {
        distribute_item_space_to_base_size_inner(
            space,
            tracks,
            track_is_affected,
            |_| 1.0,
            track_limit,
            intrinsic_contribution_type,
            axis_inner_node_size,
        )
    }

    /// Inner function that doesn't account for differences due to distributing to flex items
    /// This difference is handled by the closure passed in above
    fn distribute_item_space_to_base_size_inner(
        space: f32,
        tracks: &mut [GridTrack],
        track_is_affected: impl Fn(&GridTrack) -> bool,
        track_distribution_proportion: impl Fn(&GridTrack) -> f32,
        track_limit: impl Fn(&GridTrack) -> f32,
        intrinsic_contribution_type: IntrinsicContributionType,
        axis_inner_node_size: Option<f32>,
    ) {
        // Skip this distribution if there is either
        //   - no space to distribute
        //   - no affected tracks to distribute space to
        if space == 0.0 || !tracks.iter().any(&track_is_affected) {
            return;
        }

        // Define get_base_size function. This is passed to the distribute_space_up_to_limits helper function
        // to indicate that it is the base size that is being distributed to.
        let get_base_size = |track: &GridTrack| track.base_size;

        // 1. Find the space to distribute
        let track_sizes: f32 = tracks.iter().map(|track| track.base_size).sum();
        let extra_space: f32 = f32_max(0.0, space - track_sizes);

        // 2. Distribute space up to limits:
        // Note: there are two exit conditions to this loop:
        //   - We run out of space to distribute (extra_space falls below THRESHOLD)
        //   - We run out of growable tracks to distribute to

        /// Define a small constant to avoid infinite loops due to rounding errors. Rather than stopping distributing
        /// extra space when it gets to exactly zero, we will stop when it falls below this amount
        const THRESHOLD: f32 = 0.000001;

        let extra_space = distribute_space_up_to_limits(
            extra_space,
            tracks,
            &track_is_affected,
            &track_distribution_proportion,
            get_base_size,
            &track_limit,
        );

        // 3. Distribute remaining span beyond limits (if any)
        if extra_space > THRESHOLD {
            // When accommodating minimum contributions or accommodating min-content contributions:
            //   - any affected track that happens to also have an intrinsic max track sizing function;
            // When accommodating max-content contributions:
            //   - any affected track that happens to also have a max-content max track sizing function
            let mut filter = match intrinsic_contribution_type {
                IntrinsicContributionType::Minimum => {
                    (|track: &GridTrack| track.max_track_sizing_function.is_intrinsic()) as fn(&GridTrack) -> bool
                }
                IntrinsicContributionType::Maximum => {
                    (|track: &GridTrack| track.max_track_sizing_function.is_max_or_fit_content())
                        as fn(&GridTrack) -> bool
                }
            };

            // If there are no such tracks (matching filter above), then use all affected tracks.
            let number_of_tracks =
                tracks.iter().filter(|track| track_is_affected(track)).filter(|track| filter(track)).count();
            if number_of_tracks == 0 {
                filter = (|_| true) as fn(&GridTrack) -> bool;
            }

            // When distributing beyond limits, growth limits are ignored, but the argument
            // to any fit-content() max track sizing function still caps growth.
            distribute_space_up_to_limits(
                extra_space,
                tracks,
                |track| track_is_affected(track) && filter(track),
                &track_distribution_proportion,
                get_base_size,
                |track| track.fit_content_limit(axis_inner_node_size),
            );
        }

        // 4. For each affected track, if the track’s item-incurred increase is larger than the track’s planned increase
        // set the track’s planned increase to that value.
        for track in tracks.iter_mut() {
            if track.item_incurred_increase > track.base_size_planned_increase {
                track.base_size_planned_increase = track.item_incurred_increase;
            }

            // Reset the item_incurresed increase ready for the next space distribution
            track.item_incurred_increase = 0.0;
        }
    }
}

/// 11.5.1. Distributing Extra Space Across Spanned Tracks
/// This is simplified (and faster) version of the algorithm for growth limits
/// https://www.w3.org/TR/css-grid-1/#extra-space
fn distribute_item_space_to_growth_limit(
    space: f32,
    tracks: &mut [GridTrack],
    track_is_affected: impl Fn(&GridTrack) -> bool,
    axis_inner_node_size: Option<f32>,
) {
    // Skip this distribution if there is either
    //   - no space to distribute
    //   - no affected tracks to distribute space to
    if space == 0.0 || tracks.iter().filter(|track| track_is_affected(track)).count() == 0 {
        return;
    }

    // 1. Find the space to distribute
    let track_sizes: f32 = tracks
        .iter()
        .map(|track| if track.growth_limit == f32::INFINITY { track.base_size } else { track.growth_limit })
        .sum();
    let extra_space: f32 = f32_max(0.0, space - track_sizes);

    // 2. Distribute space up to limits:
    // For growth limits the limit is either Infinity, or the growth limit itself. Which means that:
    //   - If there are any tracks with infinite limits then all space will be distributed to those track(s).
    //   - Otherwise no space will be distributed as part of this step
    let number_of_growable_tracks = tracks
        .iter()
        .filter(|track| track_is_affected(track))
        .filter(|track| {
            track.infinitely_growable || track.fit_content_limited_growth_limit(axis_inner_node_size) == f32::INFINITY
        })
        .count();
    if number_of_growable_tracks > 0 {
        let item_incurred_increase = extra_space / number_of_growable_tracks as f32;
        for track in tracks.iter_mut().filter(|track| track_is_affected(track)).filter(|track| {
            track.infinitely_growable || track.fit_content_limited_growth_limit(axis_inner_node_size) == f32::INFINITY
        }) {
            track.item_incurred_increase = item_incurred_increase;
        }
    } else {
        // 3. Distribute space beyond limits
        // If space remains after all tracks are frozen, unfreeze and continue to distribute space to the item-incurred increase
        // ...when handling any intrinsic growth limit: all affected tracks.
        distribute_space_up_to_limits(
            extra_space,
            tracks,
            track_is_affected,
            |_| 1.0,
            |track| if track.growth_limit == f32::INFINITY { track.base_size } else { track.growth_limit },
            move |track| track.fit_content_limit(axis_inner_node_size),
        );
    };

    // 4. For each affected track, if the track’s item-incurred increase is larger than the track’s planned increase
    // set the track’s planned increase to that value.
    for track in tracks.iter_mut() {
        if track.item_incurred_increase > track.growth_limit_planned_increase {
            track.growth_limit_planned_increase = track.item_incurred_increase;
        }

        // Reset the item_incurresed increase ready for the next space distribution
        track.item_incurred_increase = 0.0;
    }
}

/// 11.6 Maximise Tracks
/// Distributes free space (if any) to tracks with FINITE growth limits, up to their limits.
#[inline(always)]
fn maximise_tracks(
    axis_tracks: &mut [GridTrack],
    axis_inner_node_size: Option<f32>,
    axis_available_grid_space: AvailableSpace,
) {
    let used_space: f32 = axis_tracks.iter().map(|track| track.base_size).sum();
    let free_space = axis_available_grid_space.compute_free_space(used_space);
    if free_space == f32::INFINITY {
        axis_tracks.iter_mut().for_each(|track| track.base_size = track.growth_limit);
    } else if free_space > 0.0 {
        distribute_space_up_to_limits(
            free_space,
            axis_tracks,
            |_| true,
            |_| 1.0,
            |track| track.base_size,
            move |track: &GridTrack| track.fit_content_limited_growth_limit(axis_inner_node_size),
        );
        for track in axis_tracks.iter_mut() {
            track.base_size += track.item_incurred_increase;
            track.item_incurred_increase = 0.0;
        }
    }
}

/// 11.7. Expand Flexible Tracks
/// This step sizes flexible tracks using the largest value it can assign to an fr without exceeding the available space.
#[allow(clippy::too_many_arguments)]
#[inline(always)]
fn expand_flexible_tracks(
    tree: &mut impl LayoutPartialTree,
    axis: AbstractAxis,
    axis_tracks: &mut [GridTrack],
    items: &mut [GridItem],
    axis_min_size: Option<f32>,
    axis_max_size: Option<f32>,
    axis_available_space_for_expansion: AvailableSpace,
) {
    // First, find the grid’s used flex fraction:
    let flex_fraction = match axis_available_space_for_expansion {
        // If the free space is zero:
        //    The used flex fraction is zero.
        // Otherwise, if the free space is a definite length:
        //   The used flex fraction is the result of finding the size of an fr using all of the grid tracks and
        //   a space to fill of the available grid space.
        AvailableSpace::Definite(available_space) => {
            let used_space: f32 = axis_tracks.iter().map(|track| track.base_size).sum();
            let free_space = available_space - used_space;
            if free_space <= 0.0 {
                0.0
            } else {
                find_size_of_fr(axis_tracks, available_space)
            }
        }
        // If ... sizing the grid container under a min-content constraint the used flex fraction is zero.
        AvailableSpace::MinContent => 0.0,
        // Otherwise, if the free space is an indefinite length:
        AvailableSpace::MaxContent => {
            // The used flex fraction is the maximum of:
            let flex_fraction = f32_max(
                // For each flexible track, if the flexible track’s flex factor is greater than one,
                // the result of dividing the track’s base size by its flex factor; otherwise, the track’s base size.
                axis_tracks
                    .iter()
                    .filter(|track| track.max_track_sizing_function.is_fr())
                    .map(|track| {
                        let flex_factor = track.flex_factor();
                        if flex_factor > 1.0 {
                            track.base_size / flex_factor
                        } else {
                            track.base_size
                        }
                    })
                    .max_by(|a, b| a.total_cmp(b))
                    .unwrap_or(0.0),
                // For each grid item that crosses a flexible track, the result of finding the size of an fr using all the grid tracks
                // that the item crosses and a space to fill of the item’s max-content contribution.
                items
                    .iter_mut()
                    .filter(|item| item.crosses_flexible_track(axis))
                    .map(|item| {
                        let tracks = &axis_tracks[item.track_range_excluding_lines(axis)];
                        // TODO: plumb estimate of other axis size (known_dimensions) in here rather than just passing Size::NONE?
                        let max_content_contribution =
                            item.max_content_contribution_cached(axis, tree, Size::NONE, Size::NONE);
                        find_size_of_fr(tracks, max_content_contribution)
                    })
                    .max_by(|a, b| a.total_cmp(b))
                    .unwrap_or(0.0),
            );

            // If using this flex fraction would cause the grid to be smaller than the grid container’s min-width/height (or larger than the
            // grid container’s max-width/height), then redo this step, treating the free space as definite and the available grid space as equal
            // to the grid container’s inner size when it’s sized to its min-width/height (max-width/height).
            // (Note: min_size takes precedence over max_size)
            let hypothetical_grid_size: f32 = axis_tracks
                .iter()
                .map(|track| {
                    if track.max_track_sizing_function.is_fr() {
                        let track_flex_factor = track.max_track_sizing_function.0.value();
                        f32_max(track.base_size, track_flex_factor * flex_fraction)
                    } else {
                        track.base_size
                    }
                })
                .sum();
            let axis_min_size = axis_min_size.unwrap_or(0.0);
            let axis_max_size = axis_max_size.unwrap_or(f32::INFINITY);
            if hypothetical_grid_size < axis_min_size {
                find_size_of_fr(axis_tracks, axis_min_size)
            } else if hypothetical_grid_size > axis_max_size {
                find_size_of_fr(axis_tracks, axis_max_size)
            } else {
                flex_fraction
            }
        }
    };

    // For each flexible track, if the product of the used flex fraction and the track’s flex factor is greater
    // than the track’s base size, set its base size to that product.
    for track in axis_tracks.iter_mut().filter(|track| track.max_track_sizing_function.is_fr()) {
        let track_flex_factor = track.max_track_sizing_function.0.value();
        track.base_size = f32_max(track.base_size, track_flex_factor * flex_fraction);
    }
}

/// 11.7.1. Find the Size of an fr
/// This algorithm finds the largest size that an fr unit can be without exceeding the target size.
/// It must be called with a set of grid tracks and some quantity of space to fill.
#[inline(always)]
fn find_size_of_fr(tracks: &[GridTrack], space_to_fill: f32) -> f32 {
    // Handle the trivial case where there is no space to fill
    // Do not remove as otherwise the loop below will loop infinitely
    if space_to_fill == 0.0 {
        return 0.0;
    }

    // If the product of the hypothetical fr size (computed below) and any flexible track’s flex factor
    // is less than the track’s base size, then we must restart this algorithm treating all such tracks as inflexible.
    // We therefore wrap the entire algorithm in a loop, with an hypothetical_fr_size of INFINITY such that the above
    // condition can never be true for the first iteration.
    let mut hypothetical_fr_size = f32::INFINITY;
    let mut previous_iter_hypothetical_fr_size;
    loop {
        // Let leftover space be the space to fill minus the base sizes of the non-flexible grid tracks.
        // Let flex factor sum be the sum of the flex factors of the flexible tracks. If this value is less than 1, set it to 1 instead.
        // We compute both of these in a single loop to avoid iterating over the data twice
        let mut used_space = 0.0;
        let mut naive_flex_factor_sum = 0.0;
        for track in tracks.iter() {
            // Tracks for which flex_factor * hypothetical_fr_size < track.base_size are treated as inflexible
            if track.max_track_sizing_function.is_fr()
                && track.max_track_sizing_function.0.value() * hypothetical_fr_size >= track.base_size
            {
                naive_flex_factor_sum += track.max_track_sizing_function.0.value();
            } else {
                used_space += track.base_size;
            };
        }
        let leftover_space = space_to_fill - used_space;
        let flex_factor = f32_max(naive_flex_factor_sum, 1.0);

        // Let the hypothetical fr size be the leftover space divided by the flex factor sum.
        previous_iter_hypothetical_fr_size = hypothetical_fr_size;
        hypothetical_fr_size = leftover_space / flex_factor;

        // If the product of the hypothetical fr size and a flexible track’s flex factor is less than the track’s base size,
        // restart this algorithm treating all such tracks as inflexible.
        // We keep track of the hypothetical_fr_size
        let hypothetical_fr_size_is_valid = tracks.iter().all(|track| {
            if track.max_track_sizing_function.is_fr() {
                let flex_factor = track.max_track_sizing_function.0.value();
                flex_factor * hypothetical_fr_size >= track.base_size
                    || flex_factor * previous_iter_hypothetical_fr_size < track.base_size
            } else {
                true
            }
        });
        if hypothetical_fr_size_is_valid {
            break;
        }
    }

    // Return the hypothetical fr size.
    hypothetical_fr_size
}

/// 11.8. Stretch auto Tracks
/// This step expands tracks that have an auto max track sizing function by dividing any remaining positive, definite free space equally amongst them.
#[inline(always)]
fn stretch_auto_tracks(
    axis_tracks: &mut [GridTrack],
    axis_min_size: Option<f32>,
    axis_available_space_for_expansion: AvailableSpace,
) {
    let num_auto_tracks = axis_tracks.iter().filter(|track| track.max_track_sizing_function.is_auto()).count();
    if num_auto_tracks > 0 {
        let used_space: f32 = axis_tracks.iter().map(|track| track.base_size).sum();

        // If the free space is indefinite, but the grid container has a definite min-width/height
        // use that size to calculate the free space for this step instead.
        let free_space = if axis_available_space_for_expansion.is_definite() {
            axis_available_space_for_expansion.compute_free_space(used_space)
        } else {
            match axis_min_size {
                Some(size) => size - used_space,
                None => 0.0,
            }
        };
        if free_space > 0.0 {
            let extra_space_per_auto_track = free_space / num_auto_tracks as f32;
            axis_tracks
                .iter_mut()
                .filter(|track| track.max_track_sizing_function.is_auto())
                .for_each(|track| track.base_size += extra_space_per_auto_track);
        }
    }
}

/// Helper function for distributing space to tracks evenly
/// Used by both distribute_item_space_to_base_size and maximise_tracks steps
#[inline(always)]
fn distribute_space_up_to_limits(
    space_to_distribute: f32,
    tracks: &mut [GridTrack],
    track_is_affected: impl Fn(&GridTrack) -> bool,
    track_distribution_proportion: impl Fn(&GridTrack) -> f32,
    track_affected_property: impl Fn(&GridTrack) -> f32,
    track_limit: impl Fn(&GridTrack) -> f32,
) -> f32 {
    /// Define a small constant to avoid infinite loops due to rounding errors. Rather than stopping distributing
    /// extra space when it gets to exactly zero, we will stop when it falls below this amount
    const THRESHOLD: f32 = 0.01;

    let mut space_to_distribute = space_to_distribute;
    while space_to_distribute > THRESHOLD {
        let track_distribution_proportion_sum: f32 = tracks
            .iter()
            .filter(|track| track_affected_property(track) + track.item_incurred_increase < track_limit(track))
            .filter(|track| track_is_affected(track))
            .map(&track_distribution_proportion)
            .sum();

        if track_distribution_proportion_sum == 0.0 {
            break;
        }

        // Compute item-incurred increase for this iteration
        let min_increase_limit = tracks
            .iter()
            .filter(|track| track_affected_property(track) + track.item_incurred_increase < track_limit(track))
            .filter(|track| track_is_affected(track))
            .map(|track| {
                (track_limit(track) - track_affected_property(track) - track.item_incurred_increase)
                    / track_distribution_proportion(track)
            })
            .min_by(|a, b| a.total_cmp(b))
            .unwrap(); // We will never pass an empty track list to this function
        let iteration_item_incurred_increase =
            f32_min(min_increase_limit, space_to_distribute / track_distribution_proportion_sum);

        for track in tracks.iter_mut().filter(|track| track_is_affected(track)) {
            let increase = iteration_item_incurred_increase * track_distribution_proportion(track);
            if increase > 0.0
                && track_affected_property(track) + track.item_incurred_increase + increase
                    <= track_limit(track) + THRESHOLD
            {
                track.item_incurred_increase += increase;
                space_to_distribute -= increase;
            }
        }
    }

    space_to_distribute
}
