//! Implements placing items in the grid and resolving the implicit grid.
//! <https://www.w3.org/TR/css-grid-1/#placement>
use super::types::{CellOccupancyMatrix, CellOccupancyState, GridItem};
use super::{NamedLineResolver, OriginZeroLine, MAX_OZ_LINE, MIN_OZ_LINE};
use crate::geometry::Line;
use crate::geometry::{AbsoluteAxis, InBothAbsAxis};
use crate::style::{AlignItems, GridAutoFlow, OriginZeroGridPlacement};
use crate::tree::NodeId;
use crate::util::sys::Vec;
use crate::{CoreStyle, Direction, GridItemStyle};

#[inline]
/// Returns whether placement/search should run in reverse for this axis.
fn axis_is_reversed(direction: Direction, axis: AbsoluteAxis) -> bool {
    direction.is_rtl() && axis == AbsoluteAxis::Horizontal
}

#[inline]
/// Advances the cursor by one track in the active search direction.
fn advance_position(position: OriginZeroLine, axis_is_reversed: bool) -> OriginZeroLine {
    if axis_is_reversed {
        OriginZeroLine(position.0.saturating_sub(1))
    } else {
        OriginZeroLine(position.0.saturating_add(1))
    }
}

#[inline]
/// Returns the initial search line for sparse/dense placement in the given axis direction.
fn search_start_line(
    grid_start_line: OriginZeroLine,
    grid_end_line: OriginZeroLine,
    axis_is_reversed: bool,
) -> OriginZeroLine {
    if axis_is_reversed {
        grid_end_line - 1
    } else {
        grid_start_line
    }
}

#[inline]
/// Resolves an indefinite span at `position`, respecting the active axis direction.
fn resolve_indefinite_grid_span(position: OriginZeroLine, span: u16, axis_is_reversed: bool) -> Line<OriginZeroLine> {
    let position = position.0 as i32;
    let span = span as i32;
    let line = |value: i32| OriginZeroLine(value.clamp(i16::MIN as i32, i16::MAX as i32) as i16);
    if axis_is_reversed {
        Line { start: line(position - span + 1), end: line(position + 1) }
    } else {
        Line { start: line(position), end: line(position + span) }
    }
}

#[inline]
/// Mirrors a horizontal span around the explicit grid width.
fn mirror_horizontal_span(span: Line<OriginZeroLine>, explicit_col_count: u16) -> Line<OriginZeroLine> {
    let explicit_col_end_line = explicit_col_count as i16;
    Line {
        start: OriginZeroLine(explicit_col_end_line - span.end.0),
        end: OriginZeroLine(explicit_col_end_line - span.start.0),
    }
}

#[inline]
/// Mirrors horizontal spans for RTL while leaving all other spans unchanged.
fn maybe_mirror_span(
    span: Line<OriginZeroLine>,
    axis: AbsoluteAxis,
    direction: Direction,
    explicit_col_count: u16,
) -> Line<OriginZeroLine> {
    if axis == AbsoluteAxis::Horizontal && direction.is_rtl() {
        // Clamp into the limited grid before mirroring so that placements outside of the limited
        // grid mirror to the same tracks as their clamped equivalents
        mirror_horizontal_span(clamp_span_to_limited_grid(span, MIN_OZ_LINE, MAX_OZ_LINE), explicit_col_count)
    } else {
        span
    }
}

/// 8.5. Grid Item Placement Algorithm
/// Place items into the grid, generating new rows/column into the implicit grid as required
///
/// [Specification](https://www.w3.org/TR/css-grid-2/#auto-placement-algo)
#[allow(clippy::too_many_arguments)]
pub(super) fn place_grid_items<'a, S, ChildIter>(
    cell_occupancy_matrix: &mut CellOccupancyMatrix,
    items: &mut Vec<GridItem>,
    children_iter: impl Fn() -> ChildIter,
    direction: Direction,
    grid_auto_flow: GridAutoFlow,
    align_items: AlignItems,
    justify_items: AlignItems,
    named_line_resolver: &NamedLineResolver<<S as CoreStyle>::CustomIdent>,
) where
    S: GridItemStyle + 'a,
    ChildIter: Iterator<Item = (usize, NodeId, S)>,
{
    let primary_axis = grid_auto_flow.primary_axis();
    let secondary_axis = primary_axis.other_axis();
    let explicit_col_count = cell_occupancy_matrix.track_counts(AbsoluteAxis::Horizontal).explicit;

    let map_child_style_to_origin_zero_placement = {
        let explicit_row_count = cell_occupancy_matrix.track_counts(AbsoluteAxis::Vertical).explicit;
        move |(index, node, style): (usize, NodeId, S)| -> (_, _, _, S) {
            let origin_zero_placement = InBothAbsAxis {
                horizontal: named_line_resolver
                    .resolve_column_names(&style.grid_column())
                    .map(|placement| placement.into_origin_zero_placement(explicit_col_count)),
                vertical: named_line_resolver
                    .resolve_row_names(&style.grid_row())
                    .map(|placement| placement.into_origin_zero_placement(explicit_row_count)),
            };
            (index, node, origin_zero_placement, style)
        }
    };

    // 1. Place children with definite positions
    let mut idx = 0;
    children_iter()
        .map(map_child_style_to_origin_zero_placement)
        .filter(|(_, _, placement, _)| placement.horizontal.is_definite() && placement.vertical.is_definite())
        .for_each(|(index, child_node, child_placement, style)| {
            idx += 1;
            #[cfg(test)]
            println!("Definite Item {idx}\n==============");

            let (row_span, col_span) =
                place_definite_grid_item(child_placement, primary_axis, direction, explicit_col_count);
            record_grid_placement(
                cell_occupancy_matrix,
                items,
                child_node,
                index,
                style,
                align_items,
                justify_items,
                primary_axis,
                direction,
                explicit_col_count,
                row_span,
                col_span,
                CellOccupancyState::DefinitelyPlaced,
            );
        });

    // 2. Place remaining children with definite secondary axis positions
    let mut idx = 0;
    children_iter()
        .map(map_child_style_to_origin_zero_placement)
        .filter(|(_, _, placement, _)| {
            placement.get(secondary_axis).is_definite() && !placement.get(primary_axis).is_definite()
        })
        .for_each(|(index, child_node, child_placement, style)| {
            idx += 1;
            #[cfg(test)]
            println!("Definite Secondary Item {idx}\n==============");

            let (primary_span, secondary_span) = place_definite_secondary_axis_item(
                &*cell_occupancy_matrix,
                child_placement,
                grid_auto_flow,
                direction,
                explicit_col_count,
            );

            record_grid_placement(
                cell_occupancy_matrix,
                items,
                child_node,
                index,
                style,
                align_items,
                justify_items,
                primary_axis,
                direction,
                explicit_col_count,
                primary_span,
                secondary_span,
                CellOccupancyState::AutoPlaced,
            );
        });

    // 3. Determine the number of columns in the implicit grid
    // By the time we get to this point in the execution, this is actually already accounted for:
    //
    // 3.1 Start with the columns from the explicit grid
    //        => Handled by grid size estimate which is used to pre-size the GridOccupancyMatrix
    //
    // 3.2 Among all the items with a definite column position (explicitly positioned items, items positioned in the previous step,
    //     and items not yet positioned but with a definite column) add columns to the beginning and end of the implicit grid as necessary
    //     to accommodate those items.
    //        => Handled by expand_to_fit_range which expands the GridOccupancyMatrix as necessary
    //            -> Called by mark_area_as
    //            -> Called by record_grid_placement
    //
    // 3.3 If the largest column span among all the items without a definite column position is larger than the width of
    //     the implicit grid, add columns to the end of the implicit grid to accommodate that column span.
    //        => Handled by grid size estimate which is used to pre-size the GridOccupancyMatrix

    // 4. Position the remaining grid items
    // (which either have definite position only in the secondary axis or indefinite positions in both axis)
    let primary_axis = grid_auto_flow.primary_axis();
    let secondary_axis = primary_axis.other_axis();
    let primary_axis_grid_start_line = cell_occupancy_matrix.track_counts(primary_axis).implicit_start_line();
    let primary_axis_grid_end_line = cell_occupancy_matrix.track_counts(primary_axis).implicit_end_line();
    let secondary_axis_grid_start_line = cell_occupancy_matrix.track_counts(secondary_axis).implicit_start_line();
    let secondary_axis_grid_end_line = cell_occupancy_matrix.track_counts(secondary_axis).implicit_end_line();
    let primary_axis_is_reversed = axis_is_reversed(direction, primary_axis);
    let grid_start_position = (
        search_start_line(primary_axis_grid_start_line, primary_axis_grid_end_line, primary_axis_is_reversed),
        search_start_line(
            secondary_axis_grid_start_line,
            secondary_axis_grid_end_line,
            axis_is_reversed(direction, secondary_axis),
        ),
    );
    let mut grid_position = grid_start_position;
    let mut idx = 0;
    children_iter()
        .map(map_child_style_to_origin_zero_placement)
        .filter(|(_, _, placement, _)| !placement.get(secondary_axis).is_definite())
        .for_each(|(index, child_node, child_placement, style)| {
            idx += 1;
            #[cfg(test)]
            println!("\nAuto Item {idx}\n==============");

            // Compute placement
            let (primary_span, secondary_span) = place_indefinitely_positioned_item(
                &*cell_occupancy_matrix,
                child_placement,
                grid_auto_flow,
                grid_position,
                direction,
                explicit_col_count,
            );

            // Record item
            record_grid_placement(
                cell_occupancy_matrix,
                items,
                child_node,
                index,
                style,
                align_items,
                justify_items,
                primary_axis,
                direction,
                explicit_col_count,
                primary_span,
                secondary_span,
                CellOccupancyState::AutoPlaced,
            );

            // If using the "dense" placement algorithm then reset the grid position back to grid_start_position ready for the next item
            // Otherwise set it to the position of the current item so that the next item it placed after it.
            grid_position = match (grid_auto_flow.is_dense(), primary_axis_is_reversed) {
                (true, _) => grid_start_position,
                (false, false) => (primary_span.end, secondary_span.start),
                (false, true) => (primary_span.start, secondary_span.start),
            };
        });
}

/// 8.5. Grid Item Placement Algorithm
/// Place a single definitely placed item into the grid
fn place_definite_grid_item(
    placement: InBothAbsAxis<Line<OriginZeroGridPlacement>>,
    primary_axis: AbsoluteAxis,
    direction: Direction,
    explicit_col_count: u16,
) -> (Line<OriginZeroLine>, Line<OriginZeroLine>) {
    // Resolve spans to tracks
    let primary_span = maybe_mirror_span(
        placement.get(primary_axis).resolve_definite_grid_lines(),
        primary_axis,
        direction,
        explicit_col_count,
    );
    let secondary_span = maybe_mirror_span(
        placement.get(primary_axis.other_axis()).resolve_definite_grid_lines(),
        primary_axis.other_axis(),
        direction,
        explicit_col_count,
    );

    (primary_span, secondary_span)
}

/// 8.5. Grid Item Placement Algorithm
/// Step 2. Place remaining children with definite secondary axis positions
fn place_definite_secondary_axis_item(
    cell_occupancy_matrix: &CellOccupancyMatrix,
    placement: InBothAbsAxis<Line<OriginZeroGridPlacement>>,
    auto_flow: GridAutoFlow,
    direction: Direction,
    explicit_col_count: u16,
) -> (Line<OriginZeroLine>, Line<OriginZeroLine>) {
    let primary_axis = auto_flow.primary_axis();
    let secondary_axis = primary_axis.other_axis();
    let primary_axis_is_reversed = axis_is_reversed(direction, primary_axis);
    let primary_axis_grid_start_line = cell_occupancy_matrix.track_counts(primary_axis).implicit_start_line();
    let primary_axis_grid_end_line = cell_occupancy_matrix.track_counts(primary_axis).implicit_end_line();

    let secondary_axis_placement = maybe_mirror_span(
        placement.get(secondary_axis).resolve_definite_grid_lines(),
        secondary_axis,
        direction,
        explicit_col_count,
    );
    let starting_position = match auto_flow.is_dense() {
        true => search_start_line(primary_axis_grid_start_line, primary_axis_grid_end_line, primary_axis_is_reversed),
        false => {
            let lookup_result = if primary_axis_is_reversed {
                cell_occupancy_matrix.first_of_type(
                    primary_axis,
                    secondary_axis_placement.start,
                    CellOccupancyState::AutoPlaced,
                )
            } else {
                cell_occupancy_matrix.last_of_type(
                    primary_axis,
                    secondary_axis_placement.start,
                    CellOccupancyState::AutoPlaced,
                )
            };
            lookup_result.unwrap_or(search_start_line(
                primary_axis_grid_start_line,
                primary_axis_grid_end_line,
                primary_axis_is_reversed,
            ))
        }
    };
    let primary_axis_span = placement.get(primary_axis).indefinite_span();

    let mut position: OriginZeroLine = starting_position;
    loop {
        let primary_axis_placement =
            resolve_indefinite_grid_span(position, primary_axis_span, primary_axis_is_reversed);

        let collision = cell_occupancy_matrix.line_area_collision_jump(
            primary_axis,
            primary_axis_placement,
            secondary_axis_placement,
            primary_axis_is_reversed,
        );

        match collision {
            None => return (primary_axis_placement, secondary_axis_placement),
            Some(next_position) => position = next_position,
        }
    }
}

/// 8.5. Grid Item Placement Algorithm
/// Step 4. Position the remaining grid items.
fn place_indefinitely_positioned_item(
    cell_occupancy_matrix: &CellOccupancyMatrix,
    placement: InBothAbsAxis<Line<OriginZeroGridPlacement>>,
    auto_flow: GridAutoFlow,
    grid_position: (OriginZeroLine, OriginZeroLine),
    direction: Direction,
    explicit_col_count: u16,
) -> (Line<OriginZeroLine>, Line<OriginZeroLine>) {
    let primary_axis = auto_flow.primary_axis();
    let secondary_axis = primary_axis.other_axis();
    let primary_axis_is_reversed = axis_is_reversed(direction, primary_axis);
    let secondary_axis_is_reversed = axis_is_reversed(direction, secondary_axis);

    let primary_placement_style = placement.get(primary_axis);
    let secondary_placement_style = placement.get(secondary_axis);

    let secondary_span = secondary_placement_style.indefinite_span();
    let has_definite_primary_axis_position = primary_placement_style.is_definite();
    let primary_axis_grid_start_line = cell_occupancy_matrix.track_counts(primary_axis).implicit_start_line();
    let primary_axis_grid_end_line = cell_occupancy_matrix.track_counts(primary_axis).implicit_end_line();
    let secondary_axis_grid_start_line = cell_occupancy_matrix.track_counts(secondary_axis).implicit_start_line();
    let secondary_axis_grid_end_line = cell_occupancy_matrix.track_counts(secondary_axis).implicit_end_line();
    let primary_start_position =
        search_start_line(primary_axis_grid_start_line, primary_axis_grid_end_line, primary_axis_is_reversed);
    let secondary_start_position =
        search_start_line(secondary_axis_grid_start_line, secondary_axis_grid_end_line, secondary_axis_is_reversed);

    let (mut primary_idx, mut secondary_idx) = grid_position;

    if has_definite_primary_axis_position {
        let primary_span = maybe_mirror_span(
            primary_placement_style.resolve_definite_grid_lines(),
            primary_axis,
            direction,
            explicit_col_count,
        );

        // Compute secondary axis starting position for search
        secondary_idx = match auto_flow.is_dense() {
            // If auto-flow is dense then we always search from the first track
            true => secondary_start_position,
            false => {
                let should_advance_secondary = if primary_axis_is_reversed {
                    primary_span.start > primary_idx
                } else {
                    primary_span.start < primary_idx
                };
                if should_advance_secondary {
                    advance_position(secondary_idx, secondary_axis_is_reversed)
                } else {
                    secondary_idx
                }
            }
        };

        // Item has fixed primary axis position: so we simply increment the secondary axis position
        // until we find a space that the item fits in
        loop {
            let secondary_span =
                resolve_indefinite_grid_span(secondary_idx, secondary_span, secondary_axis_is_reversed);

            // If area is occupied, jump the index past the collision and try again
            let collision = cell_occupancy_matrix.line_area_collision_jump(
                secondary_axis,
                secondary_span,
                primary_span,
                secondary_axis_is_reversed,
            );
            if let Some(next_position) = collision {
                secondary_idx = next_position;
                continue;
            }

            // Once we find a free space, return that position
            return (primary_span, secondary_span);
        }
    } else {
        let primary_span = primary_placement_style.indefinite_span();

        // Whether the item spans every track in the primary axis. Such an item can only be
        // placed at the primary axis grid start, in a stripe of entirely unoccupied tracks.
        let spans_all_primary_tracks = primary_span as usize >= cell_occupancy_matrix.track_counts(primary_axis).len();

        // Item does not have any fixed axis, so we search along the primary axis until we hit the end of the already
        // existent tracks, and then we reset the primary axis back to zero and increment the secondary axis index.
        // We continue in this vein until we find a space that the item fits in.
        loop {
            let primary_span = resolve_indefinite_grid_span(primary_idx, primary_span, primary_axis_is_reversed);
            let secondary_span =
                resolve_indefinite_grid_span(secondary_idx, secondary_span, secondary_axis_is_reversed);

            // If the primary index is out of bounds, then increment the secondary index and reset the primary
            // index back to the start of the grid
            let primary_out_of_bounds = if primary_axis_is_reversed {
                primary_span.start < primary_axis_grid_start_line
            } else {
                primary_span.end > primary_axis_grid_end_line
            };
            if primary_out_of_bounds {
                secondary_idx = advance_position(secondary_idx, secondary_axis_is_reversed);
                primary_idx = primary_start_position;
                continue;
            }

            // If the item spans every primary axis track, it fits if and only if all of the
            // secondary axis tracks it spans are entirely unoccupied. Jump the secondary index
            // past any non-empty tracks in the spanned stripe.
            if spans_all_primary_tracks {
                match cell_occupancy_matrix.occupied_track_jump(
                    secondary_axis,
                    secondary_span,
                    secondary_axis_is_reversed,
                ) {
                    Some(next_position) => {
                        secondary_idx = next_position;
                        primary_idx = primary_start_position;
                        continue;
                    }
                    None => return (primary_span, secondary_span),
                }
            }

            // If area is occupied, jump the primary index past the collision and try again
            let collision = cell_occupancy_matrix.line_area_collision_jump(
                primary_axis,
                primary_span,
                secondary_span,
                primary_axis_is_reversed,
            );
            if let Some(next_position) = collision {
                primary_idx = next_position;
                continue;
            }

            // Once we find a free space that's in bounds, return that position
            return (primary_span, secondary_span);
        }
    }
}

/// Clamp a placement into the limited grid, preserving a span of at least 1 track.
/// Items placed outside of the limited grid are clamped into it.
///
/// See: <https://www.w3.org/TR/css-grid-1/#overlarge-grids>
fn clamp_span_to_limited_grid(span: Line<OriginZeroLine>, min_line: i16, max_line: i16) -> Line<OriginZeroLine> {
    let start = span.start.0.clamp(min_line, max_line - 1);
    let end = span.end.0.clamp(start + 1, max_line);
    Line { start: OriginZeroLine(start), end: OriginZeroLine(end) }
}

/// Clamps a span using bounds transformed into the axis's placement coordinates.
fn clamp_span_for_axis(
    span: Line<OriginZeroLine>,
    axis: AbsoluteAxis,
    direction: Direction,
    explicit_col_count: u16,
) -> Line<OriginZeroLine> {
    let (min_line, max_line) = if axis == AbsoluteAxis::Horizontal && direction.is_rtl() {
        let explicit_end = explicit_col_count as i16;
        (explicit_end - MAX_OZ_LINE, explicit_end - MIN_OZ_LINE)
    } else {
        (MIN_OZ_LINE, MAX_OZ_LINE)
    };
    clamp_span_to_limited_grid(span, min_line, max_line)
}

/// Record the grid item in both CellOccupancyMatric and the GridItems list
/// once a definite placement has been determined
#[allow(clippy::too_many_arguments)]
fn record_grid_placement<S: GridItemStyle>(
    cell_occupancy_matrix: &mut CellOccupancyMatrix,
    items: &mut Vec<GridItem>,
    node: NodeId,
    index: usize,
    style: S,
    parent_align_items: AlignItems,
    parent_justify_items: AlignItems,
    primary_axis: AbsoluteAxis,
    direction: Direction,
    explicit_col_count: u16,
    primary_span: Line<OriginZeroLine>,
    secondary_span: Line<OriginZeroLine>,
    placement_type: CellOccupancyState,
) {
    #[cfg(test)]
    println!("BEFORE placement:");
    #[cfg(test)]
    println!("{cell_occupancy_matrix:?}");

    // Clamp placements into the limited grid to prevent arithmetic overflow when growing the
    // implicit grid (https://www.w3.org/TR/css-grid-1/#overlarge-grids)
    let primary_span = clamp_span_for_axis(primary_span, primary_axis, direction, explicit_col_count);
    let secondary_span = clamp_span_for_axis(secondary_span, primary_axis.other_axis(), direction, explicit_col_count);

    // Mark area of grid as occupied
    cell_occupancy_matrix.mark_area_as(primary_axis, primary_span, secondary_span, placement_type);

    // Create grid item
    let (col_span, row_span) = match primary_axis {
        AbsoluteAxis::Horizontal => (primary_span, secondary_span),
        AbsoluteAxis::Vertical => (secondary_span, primary_span),
    };
    items.push(GridItem::new_with_placement_style_and_order(
        node,
        col_span,
        row_span,
        style,
        parent_align_items,
        parent_justify_items,
        index as u16,
    ));

    #[cfg(test)]
    println!("AFTER placement:");
    #[cfg(test)]
    println!("{cell_occupancy_matrix:?}");
    #[cfg(test)]
    println!("\n");
}

#[cfg(test)]
mod tests {
    use super::*;

    mod test_placement_algorithm {
        use crate::compute::grid::implicit_grid::compute_grid_size_estimate;
        use crate::compute::grid::types::TrackCounts;
        use crate::compute::grid::util::*;
        use crate::compute::grid::CellOccupancyMatrix;
        use crate::compute::grid::NamedLineResolver;
        use crate::compute::grid::OriginZeroLine;
        use crate::prelude::*;
        use crate::style::GridAutoFlow;
        use crate::Direction;

        use super::super::place_grid_items;

        type ExpectedPlacement = (i16, i16, i16, i16);

        fn placement_test_runner(
            explicit_col_count: u16,
            explicit_row_count: u16,
            children: Vec<(usize, Style, ExpectedPlacement)>,
            expected_col_counts: TrackCounts,
            expected_row_counts: TrackCounts,
            flow: GridAutoFlow,
        ) {
            // Setup test
            let children_iter = || children.iter().map(|(index, style, _)| (*index, NodeId::from(*index), style));
            let child_styles_iter = children.iter().map(|(_, style, _)| style);
            let estimated_sizes =
                compute_grid_size_estimate(explicit_col_count, explicit_row_count, Direction::Ltr, child_styles_iter);
            let mut items = Vec::new();
            let mut cell_occupancy_matrix =
                CellOccupancyMatrix::with_track_counts(estimated_sizes.0, estimated_sizes.1);
            let mut name_resolver = NamedLineResolver::new(&Style::DEFAULT, 0, 0);
            name_resolver.set_explicit_column_count(explicit_col_count);
            name_resolver.set_explicit_row_count(explicit_row_count);

            // Run placement algorithm
            place_grid_items(
                &mut cell_occupancy_matrix,
                &mut items,
                children_iter,
                Direction::Ltr,
                flow,
                AlignSelf::START,
                AlignSelf::START,
                // TODO: actually test named line resolution
                &name_resolver,
            );

            // Assert that each item has been placed in the right location
            let mut sorted_children = children.clone();
            sorted_children.sort_by_key(|child| child.0);
            for (idx, ((id, _style, expected_placement), item)) in sorted_children.iter().zip(items.iter()).enumerate()
            {
                assert_eq!(item.node, NodeId::from(*id));
                let actual_placement = (item.column.start, item.column.end, item.row.start, item.row.end);
                assert_eq!(actual_placement, (*expected_placement).into_oz(), "Item {idx} (0-indexed)");
            }

            // Assert that the correct number of implicit rows have been generated
            let actual_row_counts = *cell_occupancy_matrix.track_counts(crate::compute::grid::AbsoluteAxis::Vertical);
            assert_eq!(actual_row_counts, expected_row_counts, "row track counts");
            let actual_col_counts = *cell_occupancy_matrix.track_counts(crate::compute::grid::AbsoluteAxis::Horizontal);
            assert_eq!(actual_col_counts, expected_col_counts, "column track counts");
        }

        #[test]
        fn test_only_fixed_placement() {
            let flow = GridAutoFlow::Row;
            let explicit_col_count = 2;
            let explicit_row_count = 2;
            let children = {
                vec![
                    // node, style (grid coords), expected_placement (oz coords)
                    (1, (line(1), auto(), line(1), auto()).into_grid_child(), (0, 1, 0, 1)),
                    (2, (line(-4), auto(), line(-3), auto()).into_grid_child(), (-1, 0, 0, 1)),
                    (3, (line(-3), auto(), line(-4), auto()).into_grid_child(), (0, 1, -1, 0)),
                    (4, (line(3), span(2), line(5), auto()).into_grid_child(), (2, 4, 4, 5)),
                ]
            };
            let expected_cols = TrackCounts { negative_implicit: 1, explicit: 2, positive_implicit: 2 };
            let expected_rows = TrackCounts { negative_implicit: 1, explicit: 2, positive_implicit: 3 };
            placement_test_runner(explicit_col_count, explicit_row_count, children, expected_cols, expected_rows, flow);
        }

        #[test]
        fn test_placement_spanning_origin() {
            let flow = GridAutoFlow::Row;
            let explicit_col_count = 2;
            let explicit_row_count = 2;
            let children = {
                vec![
                    // node, style (grid coords), expected_placement (oz coords)
                    (1, (line(-1), line(-1), line(-1), line(-1)).into_grid_child(), (2, 3, 2, 3)),
                    (2, (line(-1), span(2), line(-1), span(2)).into_grid_child(), (2, 4, 2, 4)),
                    (3, (line(-4), line(-4), line(-4), line(-4)).into_grid_child(), (-1, 0, -1, 0)),
                    (4, (line(-4), span(2), line(-4), span(2)).into_grid_child(), (-1, 1, -1, 1)),
                ]
            };
            let expected_cols = TrackCounts { negative_implicit: 1, explicit: 2, positive_implicit: 2 };
            let expected_rows = TrackCounts { negative_implicit: 1, explicit: 2, positive_implicit: 2 };
            placement_test_runner(explicit_col_count, explicit_row_count, children, expected_cols, expected_rows, flow);
        }

        #[test]
        fn test_only_auto_placement_row_flow() {
            let flow = GridAutoFlow::Row;
            let explicit_col_count = 2;
            let explicit_row_count = 2;
            let children = {
                let auto_child = (auto(), auto(), auto(), auto()).into_grid_child();
                vec![
                    // output order, node, style (grid coords), expected_placement (oz coords)
                    (1, auto_child.clone(), (0, 1, 0, 1)),
                    (2, auto_child.clone(), (1, 2, 0, 1)),
                    (3, auto_child.clone(), (0, 1, 1, 2)),
                    (4, auto_child.clone(), (1, 2, 1, 2)),
                    (5, auto_child.clone(), (0, 1, 2, 3)),
                    (6, auto_child.clone(), (1, 2, 2, 3)),
                    (7, auto_child.clone(), (0, 1, 3, 4)),
                    (8, auto_child.clone(), (1, 2, 3, 4)),
                ]
            };
            let expected_cols = TrackCounts { negative_implicit: 0, explicit: 2, positive_implicit: 0 };
            let expected_rows = TrackCounts { negative_implicit: 0, explicit: 2, positive_implicit: 2 };
            placement_test_runner(explicit_col_count, explicit_row_count, children, expected_cols, expected_rows, flow);
        }

        #[test]
        fn test_only_auto_placement_column_flow() {
            let flow = GridAutoFlow::Column;
            let explicit_col_count = 2;
            let explicit_row_count = 2;
            let children = {
                let auto_child = (auto(), auto(), auto(), auto()).into_grid_child();
                vec![
                    // output order, node, style (grid coords), expected_placement (oz coords)
                    (1, auto_child.clone(), (0, 1, 0, 1)),
                    (2, auto_child.clone(), (0, 1, 1, 2)),
                    (3, auto_child.clone(), (1, 2, 0, 1)),
                    (4, auto_child.clone(), (1, 2, 1, 2)),
                    (5, auto_child.clone(), (2, 3, 0, 1)),
                    (6, auto_child.clone(), (2, 3, 1, 2)),
                    (7, auto_child.clone(), (3, 4, 0, 1)),
                    (8, auto_child.clone(), (3, 4, 1, 2)),
                ]
            };
            let expected_cols = TrackCounts { negative_implicit: 0, explicit: 2, positive_implicit: 2 };
            let expected_rows = TrackCounts { negative_implicit: 0, explicit: 2, positive_implicit: 0 };
            placement_test_runner(explicit_col_count, explicit_row_count, children, expected_cols, expected_rows, flow);
        }

        #[test]
        fn test_oversized_item() {
            let flow = GridAutoFlow::Row;
            let explicit_col_count = 2;
            let explicit_row_count = 2;
            let children = {
                vec![
                    // output order, node, style (grid coords), expected_placement (oz coords)
                    (1, (span(5), auto(), auto(), auto()).into_grid_child(), (0, 5, 0, 1)),
                ]
            };
            let expected_cols = TrackCounts { negative_implicit: 0, explicit: 2, positive_implicit: 3 };
            let expected_rows = TrackCounts { negative_implicit: 0, explicit: 2, positive_implicit: 0 };
            placement_test_runner(explicit_col_count, explicit_row_count, children, expected_cols, expected_rows, flow);
        }

        #[test]
        fn test_fixed_in_secondary_axis() {
            let flow = GridAutoFlow::Row;
            let explicit_col_count = 2;
            let explicit_row_count = 2;
            let children = {
                vec![
                    // output order, node, style (grid coords), expected_placement (oz coords)
                    (1, (span(2), auto(), line(1), auto()).into_grid_child(), (0, 2, 0, 1)),
                    (2, (auto(), auto(), line(2), auto()).into_grid_child(), (0, 1, 1, 2)),
                    (3, (auto(), auto(), line(1), auto()).into_grid_child(), (2, 3, 0, 1)),
                    (4, (auto(), auto(), line(4), auto()).into_grid_child(), (0, 1, 3, 4)),
                ]
            };
            let expected_cols = TrackCounts { negative_implicit: 0, explicit: 2, positive_implicit: 1 };
            let expected_rows = TrackCounts { negative_implicit: 0, explicit: 2, positive_implicit: 2 };
            placement_test_runner(explicit_col_count, explicit_row_count, children, expected_cols, expected_rows, flow);
        }

        #[test]
        fn test_definite_in_secondary_axis_with_fully_definite_negative() {
            let flow = GridAutoFlow::Row;
            let explicit_col_count = 2;
            let explicit_row_count = 2;
            let children = {
                vec![
                    // output order, node, style (grid coords), expected_placement (oz coords)
                    (2, (auto(), auto(), line(2), auto()).into_grid_child(), (0, 1, 1, 2)),
                    (1, (line(-4), auto(), line(2), auto()).into_grid_child(), (-1, 0, 1, 2)),
                    (3, (auto(), auto(), line(1), auto()).into_grid_child(), (-1, 0, 0, 1)),
                ]
            };
            let expected_cols = TrackCounts { negative_implicit: 1, explicit: 2, positive_implicit: 0 };
            let expected_rows = TrackCounts { negative_implicit: 0, explicit: 2, positive_implicit: 0 };
            placement_test_runner(explicit_col_count, explicit_row_count, children, expected_cols, expected_rows, flow);
        }

        #[test]
        fn test_dense_packing_algorithm() {
            let flow = GridAutoFlow::RowDense;
            let explicit_col_count = 4;
            let explicit_row_count = 4;
            let children = {
                vec![
                    // output order, node, style (grid coords), expected_placement (oz coords)
                    (1, (line(2), auto(), line(1), auto()).into_grid_child(), (1, 2, 0, 1)), // Definitely positioned in column 2
                    (2, (span(2), auto(), auto(), auto()).into_grid_child(), (2, 4, 0, 1)), // Spans 2 columns, so positioned after item 1
                    (3, (auto(), auto(), auto(), auto()).into_grid_child(), (0, 1, 0, 1)), // Spans 1 column, so should be positioned before item 1
                ]
            };
            let expected_cols = TrackCounts { negative_implicit: 0, explicit: 4, positive_implicit: 0 };
            let expected_rows = TrackCounts { negative_implicit: 0, explicit: 4, positive_implicit: 0 };
            placement_test_runner(explicit_col_count, explicit_row_count, children, expected_cols, expected_rows, flow);
        }

        #[test]
        fn test_sparse_packing_algorithm() {
            let flow = GridAutoFlow::Row;
            let explicit_col_count = 4;
            let explicit_row_count = 4;
            let children = {
                vec![
                    // output order, node, style (grid coords), expected_placement (oz coords)
                    (1, (auto(), span(3), auto(), auto()).into_grid_child(), (0, 3, 0, 1)), // Width 3
                    (2, (auto(), span(3), auto(), auto()).into_grid_child(), (0, 3, 1, 2)), // Width 3 (wraps to next row)
                    (3, (auto(), span(1), auto(), auto()).into_grid_child(), (3, 4, 1, 2)), // Width 1 (uses second row as we're already on it)
                ]
            };
            let expected_cols = TrackCounts { negative_implicit: 0, explicit: 4, positive_implicit: 0 };
            let expected_rows = TrackCounts { negative_implicit: 0, explicit: 4, positive_implicit: 0 };
            placement_test_runner(explicit_col_count, explicit_row_count, children, expected_cols, expected_rows, flow);
        }

        #[test]
        fn test_auto_placement_in_negative_tracks() {
            let flow = GridAutoFlow::RowDense;
            let explicit_col_count = 2;
            let explicit_row_count = 2;
            let children = {
                vec![
                    // output order, node, style (grid coords), expected_placement (oz coords)
                    (1, (line(-5), auto(), line(1), auto()).into_grid_child(), (-2, -1, 0, 1)), // Row 1. Definitely positioned in column -2
                    (2, (auto(), auto(), line(2), auto()).into_grid_child(), (-2, -1, 1, 2)), // Row 2. Auto positioned in column -2
                    (3, (auto(), auto(), auto(), auto()).into_grid_child(), (-1, 0, 0, 1)), // Row 1. Auto positioned in column -1
                ]
            };
            let expected_cols = TrackCounts { negative_implicit: 2, explicit: 2, positive_implicit: 0 };
            let expected_rows = TrackCounts { negative_implicit: 0, explicit: 2, positive_implicit: 0 };
            placement_test_runner(explicit_col_count, explicit_row_count, children, expected_cols, expected_rows, flow);
        }

        #[test]
        fn test_rtl_overlarge_placement_uses_mirrored_limits() {
            let explicit_col_count = 9_000;
            let explicit_row_count = 0;
            let style = (line(-19_005), auto(), auto(), auto()).into_grid_child();
            let children = [(0, style)];
            let estimated_sizes = compute_grid_size_estimate(
                explicit_col_count,
                explicit_row_count,
                Direction::Rtl,
                children.iter().map(|(_, style)| style),
            );
            let mut items = Vec::new();
            let mut cell_occupancy_matrix =
                CellOccupancyMatrix::with_track_counts(estimated_sizes.0, estimated_sizes.1);
            let mut name_resolver = NamedLineResolver::new(&Style::DEFAULT, 0, 0);
            name_resolver.set_explicit_column_count(explicit_col_count);
            name_resolver.set_explicit_row_count(explicit_row_count);
            place_grid_items(
                &mut cell_occupancy_matrix,
                &mut items,
                || children.iter().map(|(index, style)| (*index, NodeId::from(*index), style)),
                Direction::Rtl,
                GridAutoFlow::Row,
                AlignSelf::START,
                AlignSelf::START,
                &name_resolver,
            );
            assert_eq!(items[0].column, Line { start: OriginZeroLine(18_999), end: OriginZeroLine(19_000) });
        }
    }

    #[test]
    fn auto_placement_cursor_saturates_at_integer_bounds() {
        assert_eq!(advance_position(OriginZeroLine(i16::MAX), false), OriginZeroLine(i16::MAX));
        assert_eq!(advance_position(OriginZeroLine(i16::MIN), true), OriginZeroLine(i16::MIN));
    }

    #[test]
    fn indefinite_spans_saturate_at_integer_bounds() {
        assert_eq!(
            resolve_indefinite_grid_span(OriginZeroLine(i16::MAX), 1, false),
            Line { start: OriginZeroLine(i16::MAX), end: OriginZeroLine(i16::MAX) }
        );
        assert_eq!(
            resolve_indefinite_grid_span(OriginZeroLine(i16::MIN), 1, true),
            Line { start: OriginZeroLine(i16::MIN), end: OriginZeroLine(i16::MIN + 1) }
        );
    }

    #[test]
    fn rtl_spans_are_clamped_in_mirrored_coordinates() {
        let logical_span = Line { start: OriginZeroLine(-10_000), end: OriginZeroLine(-9_999) };
        let mirrored_span = maybe_mirror_span(logical_span, AbsoluteAxis::Horizontal, Direction::Rtl, 9_000);
        assert_eq!(mirrored_span, Line { start: OriginZeroLine(18_999), end: OriginZeroLine(19_000) });
        assert_eq!(clamp_span_for_axis(mirrored_span, AbsoluteAxis::Horizontal, Direction::Rtl, 9_000), mirrored_span);
    }
}
