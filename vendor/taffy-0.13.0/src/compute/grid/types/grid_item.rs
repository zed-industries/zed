//! Contains GridItem used to represent a single grid item during layout
use super::GridTrack;
use crate::compute::grid::OriginZeroLine;
use crate::geometry::AbstractAxis;
use crate::geometry::{Line, Point, Rect, Size};
use crate::style::{AlignItems, AlignSelf, AvailableSpace, Dimension, LengthPercentageAuto, Overflow};
use crate::tree::{LayoutPartialTree, LayoutPartialTreeExt, NodeId, SizingMode};
use crate::util::{MaybeMath, MaybeResolve, ResolveOrZero};
use crate::{BoxSizing, GridItemStyle, LengthPercentage};
use core::ops::Range;

/// Represents a single grid item
#[derive(Debug)]
pub(in super::super) struct GridItem {
    /// The id of the node that this item represents
    pub node: NodeId,

    /// The order of the item in the children array
    ///
    /// We sort the list of grid items during track sizing. This field allows us to sort back the original order
    /// for final positioning
    pub source_order: u16,

    /// The item's definite row-start and row-end, as resolved by the placement algorithm
    /// (in origin-zero coordinates)
    pub row: Line<OriginZeroLine>,
    /// The items definite column-start and column-end, as resolved by the placement algorithm
    /// (in origin-zero coordinates)
    pub column: Line<OriginZeroLine>,

    /// Is it a compressible replaced element?
    /// https://drafts.csswg.org/css-sizing-3/#min-content-zero
    pub is_compressible_replaced: bool,
    /// The item's overflow style
    pub overflow: Point<Overflow>,
    /// The item's box_sizing style
    pub box_sizing: BoxSizing,
    /// The item's size style
    pub size: Size<Dimension>,
    /// The item's min_size style
    pub min_size: Size<Dimension>,
    /// The item's max_size style
    pub max_size: Size<Dimension>,
    /// The item's aspect_ratio style
    pub aspect_ratio: Option<f32>,
    /// The item's padding style
    pub padding: Rect<LengthPercentage>,
    /// The item's border style
    pub border: Rect<LengthPercentage>,
    /// The item's margin style
    pub margin: Rect<LengthPercentageAuto>,
    /// The item's align_self property, or the parent's align_items property is not set
    pub align_self: AlignSelf,
    /// The item's justify_self property, or the parent's justify_items property is not set
    pub justify_self: AlignSelf,
    /// The items first baseline (horizontal)
    pub baseline: Option<f32>,
    /// Shim for baseline alignment that acts like an extra top margin
    /// TODO: Support last baseline and vertical text baselines
    pub baseline_shim: f32,

    /// The item's definite row-start and row-end (same as `row` field, except in a different coordinate system)
    /// (as indexes into the Vec<GridTrack> stored in a grid's AbstractAxisTracks)
    pub row_indexes: Line<u16>,
    /// The items definite column-start and column-end (same as `column` field, except in a different coordinate system)
    /// (as indexes into the Vec<GridTrack> stored in a grid's AbstractAxisTracks)
    pub column_indexes: Line<u16>,

    /// Whether the item crosses a flexible row
    pub crosses_flexible_row: bool,
    /// Whether the item crosses a flexible column
    pub crosses_flexible_column: bool,
    /// Whether the item crosses a intrinsic row
    pub crosses_intrinsic_row: bool,
    /// Whether the item crosses a intrinsic column
    pub crosses_intrinsic_column: bool,

    // Caches for intrinsic size computation. These caches are only valid for a single run of the track-sizing algorithm.
    /// Cache for the known_dimensions input to intrinsic sizing computation
    pub grid_area_size_cache: Option<Size<Option<f32>>>,
    /// Cache for the min-content size
    pub min_content_contribution_cache: Size<Option<f32>>,
    /// Cache for the minimum contribution
    pub minimum_contribution_cache: Size<Option<f32>>,
    /// Cache for the max-content size
    pub max_content_contribution_cache: Size<Option<f32>>,

    /// Final y position. Used to compute baseline alignment for the container.
    pub y_position: f32,
    /// Final height. Used to compute baseline alignment for the container.
    pub height: f32,
}

impl GridItem {
    /// Create a new item given a concrete placement in both axes
    pub fn new_with_placement_style_and_order<S: GridItemStyle>(
        node: NodeId,
        col_span: Line<OriginZeroLine>,
        row_span: Line<OriginZeroLine>,
        style: S,
        parent_align_items: AlignItems,
        parent_justify_items: AlignItems,
        source_order: u16,
    ) -> Self {
        GridItem {
            node,
            source_order,
            row: row_span,
            column: col_span,
            is_compressible_replaced: style.is_compressible_replaced(),
            overflow: style.overflow(),
            box_sizing: style.box_sizing(),
            size: style.size(),
            min_size: style.min_size(),
            max_size: style.max_size(),
            aspect_ratio: style.aspect_ratio(),
            padding: style.padding(),
            border: style.border(),
            margin: style.margin(),
            align_self: style.align_self().unwrap_or(parent_align_items),
            justify_self: style.justify_self().unwrap_or(parent_justify_items),
            baseline: None,
            baseline_shim: 0.0,
            row_indexes: Line { start: 0, end: 0 }, // Properly initialised later
            column_indexes: Line { start: 0, end: 0 }, // Properly initialised later
            crosses_flexible_row: false,            // Properly initialised later
            crosses_flexible_column: false,         // Properly initialised later
            crosses_intrinsic_row: false,           // Properly initialised later
            crosses_intrinsic_column: false,        // Properly initialised later
            grid_area_size_cache: None,
            min_content_contribution_cache: Size::NONE,
            max_content_contribution_cache: Size::NONE,
            minimum_contribution_cache: Size::NONE,
            y_position: 0.0,
            height: 0.0,
        }
    }

    /// This item's placement in the specified axis in OriginZero coordinates
    pub fn placement(&self, axis: AbstractAxis) -> Line<OriginZeroLine> {
        match axis {
            AbstractAxis::Block => self.row,
            AbstractAxis::Inline => self.column,
        }
    }

    /// This item's placement in the specified axis as GridTrackVec indices
    pub fn placement_indexes(&self, axis: AbstractAxis) -> Line<u16> {
        match axis {
            AbstractAxis::Block => self.row_indexes,
            AbstractAxis::Inline => self.column_indexes,
        }
    }

    /// Returns a range which can be used as an index into the GridTrackVec in the specified axis
    /// which will produce a sub-slice of covering all the tracks and lines that this item spans
    /// excluding the lines that bound it.
    pub fn track_range_excluding_lines(&self, axis: AbstractAxis) -> Range<usize> {
        let indexes = self.placement_indexes(axis);
        (indexes.start as usize + 1)..(indexes.end as usize)
    }

    /// Returns the number of tracks that this item spans in the specified axis
    pub fn span(&self, axis: AbstractAxis) -> u16 {
        match axis {
            AbstractAxis::Block => self.row.span(),
            AbstractAxis::Inline => self.column.span(),
        }
    }

    /// Returns the pre-computed value indicating whether the grid item crosses a flexible track in
    /// the specified axis
    pub fn crosses_flexible_track(&self, axis: AbstractAxis) -> bool {
        match axis {
            AbstractAxis::Inline => self.crosses_flexible_column,
            AbstractAxis::Block => self.crosses_flexible_row,
        }
    }

    /// Returns the pre-computed value indicating whether the grid item crosses an intrinsic track in
    /// the specified axis
    pub fn crosses_intrinsic_track(&self, axis: AbstractAxis) -> bool {
        match axis {
            AbstractAxis::Inline => self.crosses_intrinsic_column,
            AbstractAxis::Block => self.crosses_intrinsic_row,
        }
    }

    /// For an item spanning multiple tracks, the upper limit used to calculate its limited min-/max-content contribution is the
    /// sum of the fixed max track sizing functions of any tracks it spans, and is applied if it only spans such tracks.
    pub fn spanned_track_limit(
        &mut self,
        axis: AbstractAxis,
        axis_tracks: &[GridTrack],
        axis_parent_size: Option<f32>,
        resolve_calc_value: &dyn Fn(*const (), f32) -> f32,
    ) -> Option<f32> {
        let spanned_tracks = &axis_tracks[self.track_range_excluding_lines(axis)];
        let tracks_all_fixed = spanned_tracks.iter().all(|track| {
            track.max_track_sizing_function.definite_limit(axis_parent_size, resolve_calc_value).is_some()
        });
        if tracks_all_fixed {
            let limit: f32 = spanned_tracks
                .iter()
                .map(|track| {
                    track.max_track_sizing_function.definite_limit(axis_parent_size, resolve_calc_value).unwrap()
                })
                .sum();
            Some(limit)
        } else {
            None
        }
    }

    /// Similar to the spanned_track_limit, but excludes FitContent arguments from the limit.
    /// Used to clamp the automatic minimum contributions of an item
    pub fn spanned_fixed_track_limit(
        &mut self,
        axis: AbstractAxis,
        axis_tracks: &[GridTrack],
        axis_parent_size: Option<f32>,
        resolve_calc_value: &dyn Fn(*const (), f32) -> f32,
    ) -> Option<f32> {
        let spanned_tracks = &axis_tracks[self.track_range_excluding_lines(axis)];
        let tracks_all_fixed = spanned_tracks.iter().all(|track| {
            track.max_track_sizing_function.definite_value(axis_parent_size, resolve_calc_value).is_some()
        });
        if tracks_all_fixed {
            let limit: f32 = spanned_tracks
                .iter()
                .map(|track| {
                    track.max_track_sizing_function.definite_value(axis_parent_size, resolve_calc_value).unwrap()
                })
                .sum();
            Some(limit)
        } else {
            None
        }
    }

    /// Compute the known_dimensions to be passed to the child sizing functions
    /// The key thing that is being done here is applying stretch alignment, which is necessary to
    /// allow percentage sizes further down the tree to resolve properly in some cases
    fn known_dimensions(
        &self,
        tree: &mut impl LayoutPartialTree,
        grid_area_size: Size<Option<f32>>,
    ) -> Size<Option<f32>> {
        let margins = self.margins_axis_sums_with_baseline_shims(grid_area_size.width, tree);

        let aspect_ratio = self.aspect_ratio;
        // CSS resolves percentage padding and border against the inline size of the containing
        // block. For a grid item under intrinsic measurement, that inline-size basis is the grid
        // area's width when it is definite.
        // Spec:
        // https://www.w3.org/TR/css-grid-1/#item-margins
        // https://www.w3.org/TR/CSS22/box.html#padding-properties
        let padding = self.padding.resolve_or_zero(grid_area_size.width, |val, basis| tree.calc(val, basis));
        let border = self.border.resolve_or_zero(grid_area_size.width, |val, basis| tree.calc(val, basis));
        let padding_border_size = (padding + border).sum_axes();
        let box_sizing_adjustment =
            if self.box_sizing == BoxSizing::ContentBox { padding_border_size } else { Size::ZERO };
        let inherent_size = self
            .size
            .maybe_resolve(grid_area_size, |val, basis| tree.calc(val, basis))
            .maybe_apply_aspect_ratio(aspect_ratio)
            .maybe_add(box_sizing_adjustment);
        let min_size = self
            .min_size
            .maybe_resolve(grid_area_size, |val, basis| tree.calc(val, basis))
            .maybe_apply_aspect_ratio(aspect_ratio)
            .maybe_add(box_sizing_adjustment);
        let max_size = self
            .max_size
            .maybe_resolve(grid_area_size, |val, basis| tree.calc(val, basis))
            .maybe_apply_aspect_ratio(aspect_ratio)
            .maybe_add(box_sizing_adjustment);

        let grid_area_minus_item_margins_size = grid_area_size.maybe_sub(margins);

        // If node is absolutely positioned and width is not set explicitly, then deduce it
        // from left, right and container_content_box if both are set.
        let width = inherent_size.width.or_else(|| {
            // Apply width based on stretch alignment if:
            //  - Alignment style is "stretch"
            //  - The node is not absolutely positioned
            //  - The node does not have auto margins in this axis.
            if !self.margin.left.is_auto() && !self.margin.right.is_auto() && self.justify_self == AlignSelf::STRETCH {
                return grid_area_minus_item_margins_size.width;
            }

            None
        });
        // Reapply aspect ratio after stretch and absolute position width adjustments
        let Size { width, height } =
            Size { width, height: inherent_size.height }.maybe_apply_aspect_ratio(aspect_ratio);

        let height = height.or_else(|| {
            // Apply height based on stretch alignment if:
            //  - Alignment style is "stretch"
            //  - The node is not absolutely positioned
            //  - The node does not have auto margins in this axis.
            if !self.margin.top.is_auto() && !self.margin.bottom.is_auto() && self.align_self == AlignSelf::STRETCH {
                return grid_area_minus_item_margins_size.height;
            }

            None
        });
        // Reapply aspect ratio after stretch and absolute position height adjustments
        let Size { width, height } = Size { width, height }.maybe_apply_aspect_ratio(aspect_ratio);

        // Clamp size by min and max width/height
        let Size { width, height } = Size { width, height }.maybe_clamp(min_size, max_size);

        Size { width, height }
    }

    /// Returns the grid area's size in the specified axis when every spanned track has a definite fixed size.
    ///
    /// During intrinsic sizing, percentages on grid items resolve against the size of the grid area,
    /// not the grid container. If the spanned tracks in an axis are not all definite yet, the grid
    /// area is still indefinite in that axis and percentage-dependent values must stay unresolved here.
    ///
    /// Spec:
    /// https://www.w3.org/TR/css-grid-1/#grid-item-sizing
    /// https://www.w3.org/TR/css-grid-1/#algo-overview
    ///
    /// Compute the available_space to be passed to the child sizing functions
    /// These are estimates based on either the max track sizing function or the provisional base size in the opposite
    /// axis to the one currently being sized.
    /// https://www.w3.org/TR/css-grid-1/#algo-overview
    pub fn grid_area_size(
        &self,
        axis: AbstractAxis,
        axis_tracks: &[GridTrack],
        other_axis_tracks: &[GridTrack],
        available_space: Size<Option<f32>>,
        get_track_size_estimate: impl Fn(&GridTrack, Option<f32>) -> Option<f32>,
        resolve_calc_value: &impl Fn(*const (), f32) -> f32,
    ) -> Size<Option<f32>> {
        let mut size = Size::NONE;
        size.set(
            axis,
            axis_tracks[self.track_range_excluding_lines(axis)]
                .iter()
                .map(|track| {
                    let min_size = track
                        .min_track_sizing_function
                        .definite_value(available_space.get(axis), resolve_calc_value)?;
                    let max_size = track
                        .max_track_sizing_function
                        .definite_value(available_space.get(axis), resolve_calc_value)?;

                    if min_size == max_size {
                        Some(track.base_size)
                    } else {
                        None
                    }
                })
                .sum::<Option<f32>>(),
        );

        size.set(
            axis.other(),
            other_axis_tracks[self.track_range_excluding_lines(axis.other())]
                .iter()
                .map(|track| {
                    get_track_size_estimate(track, available_space.get(axis.other()))
                        .map(|size| size + track.content_alignment_adjustment)
                })
                .sum::<Option<f32>>(),
        );

        size
    }

    /// Retrieve the available_space from the cache or compute them using the passed parameters
    pub fn grid_area_size_cached(
        &mut self,
        axis: AbstractAxis,
        axis_tracks: &[GridTrack],
        other_axis_tracks: &[GridTrack],
        available_space: Size<Option<f32>>,
        get_track_size_estimate: impl Fn(&GridTrack, Option<f32>) -> Option<f32>,
        resolve_calc_value: &impl Fn(*const (), f32) -> f32,
    ) -> Size<Option<f32>> {
        self.grid_area_size_cache.unwrap_or_else(|| {
            let grid_area_size = self.grid_area_size(
                axis,
                axis_tracks,
                other_axis_tracks,
                available_space,
                get_track_size_estimate,
                resolve_calc_value,
            );
            self.grid_area_size_cache = Some(grid_area_size);
            grid_area_size
        })
    }

    /// Compute the item's resolved margins for size contributions. Horizontal percentage margins always resolve
    /// to zero if the container size is indefinite as otherwise this would introduce a cyclic dependency.
    #[inline(always)]
    pub fn margins_axis_sums_with_baseline_shims(
        &self,
        inner_node_width: Option<f32>,
        tree: &impl LayoutPartialTree,
    ) -> Size<f32> {
        Rect {
            left: self.margin.left.resolve_or_zero(Some(0.0), |val, basis| tree.calc(val, basis)),
            right: self.margin.right.resolve_or_zero(Some(0.0), |val, basis| tree.calc(val, basis)),
            top: self.margin.top.resolve_or_zero(inner_node_width, |val, basis| tree.calc(val, basis))
                + self.baseline_shim,
            bottom: self.margin.bottom.resolve_or_zero(inner_node_width, |val, basis| tree.calc(val, basis)),
        }
        .sum_axes()
    }

    /// Compute the item's min content contribution from the provided parameters
    pub fn min_content_contribution(
        &self,
        axis: AbstractAxis,
        tree: &mut impl LayoutPartialTree,
        grid_area_size: Size<Option<f32>>,
        available_space: Size<Option<f32>>,
    ) -> f32 {
        let known_dimensions = self.known_dimensions(tree, grid_area_size);
        // The child sees the grid area as its containing block during intrinsic measurement, so
        // percentage box properties resolve against the grid area when that size is definite.
        // Spec:
        // https://www.w3.org/TR/css-grid-1/#grid-item-sizing
        // https://www.w3.org/TR/css-grid-1/#algo-overview
        tree.measure_child_size(
            self.node,
            known_dimensions,
            grid_area_size,
            available_space.map(|opt| match opt {
                Some(size) => AvailableSpace::Definite(size),
                None => AvailableSpace::MinContent,
            }),
            SizingMode::InherentSize,
            axis.as_abs_naive(),
            Line::FALSE,
        )
    }

    /// Retrieve the item's min content contribution from the cache or compute it using the provided parameters
    #[inline(always)]
    pub fn min_content_contribution_cached(
        &mut self,
        axis: AbstractAxis,
        tree: &mut impl LayoutPartialTree,
        grid_area_size: Size<Option<f32>>,
        available_space: Size<Option<f32>>,
    ) -> f32 {
        self.min_content_contribution_cache.get(axis).unwrap_or_else(|| {
            let size = self.min_content_contribution(axis, tree, grid_area_size, available_space);
            self.min_content_contribution_cache.set(axis, Some(size));
            size
        })
    }

    /// Compute the item's max content contribution from the provided parameters
    pub fn max_content_contribution(
        &self,
        axis: AbstractAxis,
        tree: &mut impl LayoutPartialTree,
        grid_area_size: Size<Option<f32>>,
        available_space: Size<Option<f32>>,
    ) -> f32 {
        let known_dimensions = self.known_dimensions(tree, grid_area_size);
        // See the min-content path above. Max-content measurement uses the same containing-block
        // basis so percentage-dependent item geometry is measured from the grid area rather than
        // from the container.
        tree.measure_child_size(
            self.node,
            known_dimensions,
            grid_area_size,
            available_space.map(|opt| match opt {
                Some(size) => AvailableSpace::Definite(size),
                None => AvailableSpace::MaxContent,
            }),
            SizingMode::InherentSize,
            axis.as_abs_naive(),
            Line::FALSE,
        )
    }

    /// Retrieve the item's max content contribution from the cache or compute it using the provided parameters
    #[inline(always)]
    pub fn max_content_contribution_cached(
        &mut self,
        axis: AbstractAxis,
        tree: &mut impl LayoutPartialTree,
        grid_area_size: Size<Option<f32>>,
        available_space: Size<Option<f32>>,
    ) -> f32 {
        self.max_content_contribution_cache.get(axis).unwrap_or_else(|| {
            let size = self.max_content_contribution(axis, tree, grid_area_size, available_space);
            self.max_content_contribution_cache.set(axis, Some(size));
            size
        })
    }

    /// The minimum contribution of an item is the smallest outer size it can have.
    /// Specifically:
    ///   - If the item’s computed preferred size behaves as auto or depends on the size of its containing block in the relevant axis:
    ///     Its minimum contribution is the outer size that would result from assuming the item’s used minimum size as its preferred size;
    ///   - Else the item’s minimum contribution is its min-content contribution.
    ///
    /// Because the minimum contribution often depends on the size of the item’s content, it is considered a type of intrinsic size contribution.
    /// See: https://www.w3.org/TR/css-grid-1/#min-size-auto
    pub fn minimum_contribution(
        &mut self,
        tree: &mut impl LayoutPartialTree,
        axis: AbstractAxis,
        axis_tracks: &[GridTrack],
        grid_area_size: Size<Option<f32>>,
        inner_node_size: Size<Option<f32>>,
    ) -> f32 {
        let padding = self.padding.resolve_or_zero(grid_area_size.width, |val, basis| tree.calc(val, basis));
        let border = self.border.resolve_or_zero(grid_area_size.width, |val, basis| tree.calc(val, basis));
        let padding_border_size = (padding + border).sum_axes();
        let box_sizing_adjustment =
            if self.box_sizing == BoxSizing::ContentBox { padding_border_size } else { Size::ZERO };
        self.size
            .maybe_resolve(grid_area_size, |val, basis| tree.calc(val, basis))
            .maybe_apply_aspect_ratio(self.aspect_ratio)
            .maybe_add(box_sizing_adjustment)
            .get(axis)
            .or_else(|| {
                self.min_size
                    .maybe_resolve(grid_area_size, |val, basis| tree.calc(val, basis))
                    .maybe_apply_aspect_ratio(self.aspect_ratio)
                    .maybe_add(box_sizing_adjustment)
                    .get(axis)
            })
            .or_else(|| self.overflow.get(axis).maybe_into_automatic_min_size())
            .unwrap_or_else(|| {
                // Automatic minimum size. See https://www.w3.org/TR/css-grid-1/#min-size-auto

                // To provide a more reasonable default minimum size for grid items, the used value of its automatic minimum size
                // in a given axis is the content-based minimum size if all of the following are true:
                let item_axis_tracks = &axis_tracks[self.track_range_excluding_lines(axis)];

                // it is not a scroll container
                // TODO: support overflow property

                // it spans at least one track in that axis whose min track sizing function is auto
                let spans_auto_min_track = axis_tracks
                    .iter()
                    // TODO: should this be 'behaves as auto' rather than just literal auto?
                    .any(|track| track.min_track_sizing_function.is_auto());

                // if it spans more than one track in that axis, none of those tracks are flexible
                let only_span_one_track = item_axis_tracks.len() == 1;
                let spans_a_flexible_track = axis_tracks.iter().any(|track| track.max_track_sizing_function.is_fr());

                let use_content_based_minimum =
                    spans_auto_min_track && (only_span_one_track || !spans_a_flexible_track);

                // Otherwise, the automatic minimum size is zero, as usual.
                if use_content_based_minimum {
                    let mut minimum_contribution =
                        self.min_content_contribution_cached(axis, tree, grid_area_size, grid_area_size);

                    // If the item is a compressible replaced element, and has a definite preferred size or maximum size in the
                    // relevant axis, the size suggestion is capped by those sizes; for this purpose, any indefinite percentages
                    // in these sizes are resolved against zero (and considered definite).
                    if self.is_compressible_replaced {
                        let size = self.size.get(axis).maybe_resolve(Some(0.0), |val, basis| tree.calc(val, basis));
                        let max_size =
                            self.max_size.get(axis).maybe_resolve(Some(0.0), |val, basis| tree.calc(val, basis));
                        minimum_contribution = minimum_contribution.maybe_min(size).maybe_min(max_size);
                    }

                    // The content-based minimum size is additionally clamped by the sum of any fixed max track sizing
                    // functions of the tracks the item spans. Note that this clamp does not apply to explicitly specified
                    // preferred or minimum sizes, and that the argument to fit-content() does not clamp the content-based
                    // minimum size in the same way as a fixed max track sizing function.
                    let limit =
                        self.spanned_fixed_track_limit(axis, axis_tracks, inner_node_size.get(axis), &|val, basis| {
                            tree.resolve_calc_value(val, basis)
                        });
                    minimum_contribution.maybe_min(limit)
                } else {
                    0.0
                }
            })
    }

    /// Retrieve the item's minimum contribution from the cache or compute it using the provided parameters
    #[inline(always)]
    pub fn minimum_contribution_cached(
        &mut self,
        tree: &mut impl LayoutPartialTree,
        axis: AbstractAxis,
        axis_tracks: &[GridTrack],
        grid_area_size: Size<Option<f32>>,
        inner_node_size: Size<Option<f32>>,
    ) -> f32 {
        self.minimum_contribution_cache.get(axis).unwrap_or_else(|| {
            let size = self.minimum_contribution(tree, axis, axis_tracks, grid_area_size, inner_node_size);
            self.minimum_contribution_cache.set(axis, Some(size));
            size
        })
    }
}
