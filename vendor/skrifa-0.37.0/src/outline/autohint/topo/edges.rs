//! Edge detection.
//!
//! Edges are sets of segments that all lie within a threshold based on
//! stem widths.
//!
//! Here we compute edges from the segment list, assign properties (round,
//! serif, links) and then associate them with blue zones.

use super::{
    super::{
        metrics::{fixed_div, fixed_mul, Scale, ScaledAxisMetrics, ScaledBlue, UnscaledBlue},
        outline::Direction,
        style::ScriptGroup,
    },
    Axis, Edge, Segment,
};

/// Links segments to edges, using feature analysis for selection.
///
/// See <https://gitlab.freedesktop.org/freetype/freetype/-/blob/57617782464411201ce7bbc93b086c1b4d7d84a5/src/autofit/aflatin.c#L2128>
pub(crate) fn compute_edges(
    axis: &mut Axis,
    metrics: &ScaledAxisMetrics,
    top_to_bottom_hinting: bool,
    y_scale: i32,
    group: ScriptGroup,
) {
    axis.edges.clear();
    let scale = metrics.scale;
    // This is always passed as 0 in functions that take hinting direction
    // in CJK
    // See <https://gitlab.freedesktop.org/freetype/freetype/-/blob/57617782464411201ce7bbc93b086c1b4d7d84a5/src/autofit/afcjk.c#L1114>
    let top_to_bottom_hinting = if axis.dim == Axis::HORIZONTAL || group != ScriptGroup::Default {
        false
    } else {
        top_to_bottom_hinting
    };
    // Ignore horizontal segments less than 1 pixel in length
    let segment_length_threshold = if axis.dim == Axis::HORIZONTAL {
        fixed_div(64, y_scale)
    } else {
        0
    };
    // Also ignore segments with a width delta larger than 0.5 pixels
    let segment_width_threshold = fixed_div(32, scale);
    // Ensure that edge distance threshold is less than or equal to
    // 0.25 pixels
    let initial_threshold = metrics.width_metrics.edge_distance_threshold;
    const EDGE_DISTANCE_THRESHOLD_MAX: i32 = 64 / 4;
    let edge_distance_threshold = if group == ScriptGroup::Default {
        fixed_div(
            fixed_mul(initial_threshold, scale).min(EDGE_DISTANCE_THRESHOLD_MAX),
            scale,
        )
    } else {
        // CJK uses a slightly different computation here
        // See <https://gitlab.freedesktop.org/freetype/freetype/-/blob/57617782464411201ce7bbc93b086c1b4d7d84a5/src/autofit/afcjk.c#L1043>
        let threshold = fixed_mul(initial_threshold, scale);
        if threshold > EDGE_DISTANCE_THRESHOLD_MAX {
            fixed_div(EDGE_DISTANCE_THRESHOLD_MAX, scale)
        } else {
            initial_threshold
        }
    };
    // Now build the sorted table of edges by looping over all segments
    // to find a matching edge, adding a new one if not found.
    // We can't iterate segments because we make mutable calls on `axis`
    // below which causes overlapping borrows
    for segment_ix in 0..axis.segments.len() {
        let segment = &axis.segments[segment_ix];
        if group == ScriptGroup::Default {
            // Ignore segments that are too short, too wide or direction-less
            if (segment.height as i32) < segment_length_threshold
                || (segment.delta as i32 > segment_width_threshold)
                || segment.dir == Direction::None
            {
                continue;
            }
            // Ignore serif edges that are smaller than 1.5 pixels
            if segment.serif_ix.is_some()
                && (2 * segment.height as i32) < (3 * segment_length_threshold)
            {
                continue;
            }
        }
        // Look for a corresponding edge for this segment
        let mut best_dist = i32::MAX;
        let mut best_edge_ix = None;
        for edge_ix in 0..axis.edges.len() {
            let edge = &axis.edges[edge_ix];
            let dist = (segment.pos as i32 - edge.fpos as i32).abs();
            if dist < edge_distance_threshold && edge.dir == segment.dir && dist < best_dist {
                if group == ScriptGroup::Default {
                    best_edge_ix = Some(edge_ix);
                    break;
                }
                // For CJK, we add some additional checks
                // See <https://gitlab.freedesktop.org/freetype/freetype/-/blob/57617782464411201ce7bbc93b086c1b4d7d84a5/src/autofit/afcjk.c#L1073>
                if let Some(link) = segment.link(&axis.segments).copied() {
                    // Check whether all linked segments of the candidate edge
                    // can make a single edge
                    let first_ix = edge.first_ix as usize;
                    let mut seg1 = &axis.segments[first_ix];
                    let mut dist2 = 0;
                    loop {
                        if let Some(link1) = seg1.link(&axis.segments).copied() {
                            dist2 = (link.pos as i32 - link1.pos as i32).abs();
                            if dist2 >= edge_distance_threshold {
                                break;
                            }
                        }
                        if seg1.edge_next_ix == Some(first_ix as u16) {
                            break;
                        }
                        if let Some(next) = seg1.next_in_edge(&axis.segments) {
                            seg1 = next;
                        } else {
                            break;
                        }
                    }
                    if dist2 >= edge_distance_threshold {
                        continue;
                    }
                }
                best_dist = dist;
                best_edge_ix = Some(edge_ix);
            }
        }
        if let Some(edge_ix) = best_edge_ix {
            axis.append_segment_to_edge(segment_ix, edge_ix);
        } else {
            // We couldn't find an edge, so add a new one for this segment
            let opos = fixed_mul(segment.pos as i32, scale);
            let edge = Edge {
                fpos: segment.pos,
                opos,
                pos: opos,
                dir: segment.dir,
                first_ix: segment_ix as u16,
                last_ix: segment_ix as u16,
                ..Default::default()
            };
            axis.insert_edge(edge, top_to_bottom_hinting);
            axis.segments[segment_ix].edge_next_ix = Some(segment_ix as u16);
        }
    }
    if group == ScriptGroup::Default {
        // Loop again to find single point segments without a direction and
        // associate them with an existing edge if possible
        for segment_ix in 0..axis.segments.len() {
            let segment = &axis.segments[segment_ix];
            if segment.dir != Direction::None {
                continue;
            }
            // Try to find an edge that coincides with this segment within the
            // threshold
            if let Some(edge_ix) = axis
                .edges
                .iter()
                .enumerate()
                .filter_map(|(ix, edge)| {
                    ((segment.pos as i32 - edge.fpos as i32).abs() < edge_distance_threshold)
                        .then_some(ix)
                })
                .next()
            {
                // We found an edge, link everything up
                axis.append_segment_to_edge(segment_ix, edge_ix);
            }
        }
    }
    link_segments_to_edges(axis);
    compute_edge_properties(axis);
}

/// Edges get reordered as they're built so we need to assign edge indices to
/// segments in a second pass.
fn link_segments_to_edges(axis: &mut Axis) {
    let segments = axis.segments.as_mut_slice();
    for edge_ix in 0..axis.edges.len() {
        let edge = &axis.edges[edge_ix];
        let mut ix = edge.first_ix as usize;
        let last_ix = edge.last_ix as usize;
        loop {
            let segment = &mut segments[ix];
            segment.edge_ix = Some(edge_ix as u16);
            if ix == last_ix {
                break;
            }
            ix = segment
                .edge_next_ix
                .map(|ix| ix as usize)
                .unwrap_or(last_ix);
        }
    }
}

/// Compute the edge properties based on the series of segments that make
/// up the edge.
///
/// See <https://gitlab.freedesktop.org/freetype/freetype/-/blob/57617782464411201ce7bbc93b086c1b4d7d84a5/src/autofit/aflatin.c#L2339>
fn compute_edge_properties(axis: &mut Axis) {
    let edges = axis.edges.as_mut_slice();
    let segments = axis.segments.as_slice();
    for edge_ix in 0..edges.len() {
        let mut roundness = 0;
        let mut straightness = 0;
        let edge = edges[edge_ix];
        let mut segment_ix = edge.first_ix as usize;
        let last_segment_ix = edge.last_ix as usize;
        loop {
            // This loop can modify the current edge, so make sure we
            // reload it here
            let edge = edges[edge_ix];
            let segment = &segments[segment_ix];
            let next_segment_ix = segment.edge_next_ix;
            // Check roundness
            if segment.flags & Segment::ROUND != 0 {
                roundness += 1;
            } else {
                straightness += 1;
            }
            // Check for serifs
            let is_serif = if let Some(serif_ix) = segment.serif_ix {
                let serif = &segments[serif_ix as usize];
                serif.edge_ix.is_some() && serif.edge_ix != Some(edge_ix as u16)
            } else {
                false
            };
            // Check for links
            if is_serif
                || (segment.link_ix.is_some()
                    && segments[segment.link_ix.unwrap() as usize]
                        .edge_ix
                        .is_some())
            {
                let (edge2_ix, segment2_ix) = if is_serif {
                    (edge.serif_ix, segment.serif_ix)
                } else {
                    (edge.link_ix, segment.link_ix)
                };
                let edge2_ix = if let (Some(edge2_ix), Some(segment2_ix)) = (edge2_ix, segment2_ix)
                {
                    let edge2 = &edges[edge2_ix as usize];
                    let edge_delta = (edge.fpos as i32 - edge2.fpos as i32).abs();
                    let segment2 = &segments[segment2_ix as usize];
                    let segment_delta = (segment.pos as i32 - segment2.pos as i32).abs();
                    if segment_delta < edge_delta {
                        segment2.edge_ix
                    } else {
                        Some(edge2_ix)
                    }
                } else if let Some(segment2_ix) = segment2_ix {
                    segments[segment2_ix as usize].edge_ix
                } else {
                    edge2_ix
                };
                if is_serif {
                    edges[edge_ix].serif_ix = edge2_ix;
                    edges[edge2_ix.unwrap() as usize].flags |= Edge::SERIF;
                } else {
                    edges[edge_ix].link_ix = edge2_ix;
                }
            }
            if segment_ix == last_segment_ix {
                break;
            }
            segment_ix = next_segment_ix
                .map(|ix| ix as usize)
                .unwrap_or(last_segment_ix);
        }
        let edge = &mut edges[edge_ix];
        edge.flags = Edge::NORMAL;
        if roundness > 0 && roundness >= straightness {
            edge.flags |= Edge::ROUND;
        }
        // Drop serifs for linked edges
        if edge.serif_ix.is_some() && edge.link_ix.is_some() {
            edge.serif_ix = None;
        }
    }
}

/// Compute all edges which lie within blue zones.
///
/// For Latin, this is only done for the vertical axis.
///
/// See <https://gitlab.freedesktop.org/freetype/freetype/-/blob/57617782464411201ce7bbc93b086c1b4d7d84a5/src/autofit/aflatin.c#L2503>
pub(crate) fn compute_blue_edges(
    axis: &mut Axis,
    scale: &Scale,
    unscaled_blues: &[UnscaledBlue],
    blues: &[ScaledBlue],
    group: ScriptGroup,
) {
    // For the default script group, don't compute blues in the horizontal
    // direction
    // See <https://gitlab.freedesktop.org/freetype/freetype/-/blob/57617782464411201ce7bbc93b086c1b4d7d84a5/src/autofit/aflatin.c#L3572>
    if axis.dim != Axis::VERTICAL && group == ScriptGroup::Default {
        return;
    }
    let axis_scale = if axis.dim == Axis::HORIZONTAL {
        scale.x_scale
    } else {
        scale.y_scale
    };
    // Initial threshold
    let initial_best_dest = fixed_mul(scale.units_per_em / 40, axis_scale).min(64 / 2);
    for edge in &mut axis.edges {
        let mut best_blue = None;
        let mut best_is_neutral = false;
        // Initial threshold as a fraction of em size with a max distance
        // of 0.5 pixels
        let mut best_dist = initial_best_dest;
        for (unscaled_blue, blue) in unscaled_blues.iter().zip(blues) {
            // Ignore inactive blue zones
            if !blue.is_active {
                continue;
            }
            let is_top = blue.zones.is_top_like();
            let is_neutral = blue.zones.is_neutral();
            let is_major_dir = edge.dir == axis.major_dir;
            // Both directions are handled for neutral blues
            if is_top ^ is_major_dir || is_neutral {
                // Compare to reference position
                let (ref_pos, matching_blue) = if group == ScriptGroup::Default {
                    (unscaled_blue.position, blue.position)
                } else {
                    // For CJK, we take the blue with the smallest delta
                    // from the edge
                    // See <https://gitlab.freedesktop.org/freetype/freetype/-/blob/57617782464411201ce7bbc93b086c1b4d7d84a5/src/autofit/afcjk.c#L1356>
                    if (edge.fpos as i32 - unscaled_blue.position).abs()
                        > (edge.fpos as i32 - unscaled_blue.overshoot).abs()
                    {
                        (unscaled_blue.overshoot, blue.overshoot)
                    } else {
                        (unscaled_blue.position, blue.position)
                    }
                };
                let dist = fixed_mul((edge.fpos as i32 - ref_pos).abs(), axis_scale);
                if dist < best_dist {
                    best_dist = dist;
                    best_blue = Some(matching_blue);
                    best_is_neutral = is_neutral;
                }
                if group == ScriptGroup::Default {
                    // Now compare to overshoot position for the default script
                    // group
                    // See <https://gitlab.freedesktop.org/freetype/freetype/-/blob/57617782464411201ce7bbc93b086c1b4d7d84a5/src/autofit/aflatin.c#L2579>
                    if edge.flags & Edge::ROUND != 0 && dist != 0 && !is_neutral {
                        let is_under_ref = (edge.fpos as i32) < unscaled_blue.position;
                        if is_top ^ is_under_ref {
                            let dist = fixed_mul(
                                (edge.fpos as i32 - unscaled_blue.overshoot).abs(),
                                axis_scale,
                            );
                            if dist < best_dist {
                                best_dist = dist;
                                best_blue = Some(blue.overshoot);
                                best_is_neutral = is_neutral;
                            }
                        }
                    }
                }
            }
        }
        if let Some(best_blue) = best_blue {
            edge.blue_edge = Some(best_blue);
            if best_is_neutral {
                edge.flags |= Edge::NEUTRAL;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{
        super::super::{
            metrics::{self, ScaledWidth},
            outline::Outline,
            shape::{Shaper, ShaperMode},
            style,
        },
        super::segments,
        *,
    };
    use crate::{attribute::Style, MetadataProvider};
    use raw::{types::GlyphId, FontRef, TableProvider};

    #[test]
    fn edges_default() {
        let expected_h_edges = [
            Edge {
                fpos: 15,
                opos: 15,
                pos: 15,
                flags: Edge::ROUND,
                dir: Direction::Up,
                blue_edge: None,
                link_ix: Some(3),
                serif_ix: None,
                scale: 0,
                first_ix: 1,
                last_ix: 1,
            },
            Edge {
                fpos: 123,
                opos: 126,
                pos: 126,
                flags: 0,
                dir: Direction::Up,
                blue_edge: None,
                link_ix: Some(2),
                serif_ix: None,
                scale: 0,
                first_ix: 0,
                last_ix: 0,
            },
            Edge {
                fpos: 186,
                opos: 190,
                pos: 190,
                flags: 0,
                dir: Direction::Down,
                blue_edge: None,
                link_ix: Some(1),
                serif_ix: None,
                scale: 0,
                first_ix: 4,
                last_ix: 4,
            },
            Edge {
                fpos: 205,
                opos: 210,
                pos: 210,
                flags: Edge::ROUND,
                dir: Direction::Down,
                blue_edge: None,
                link_ix: Some(0),
                serif_ix: None,
                scale: 0,
                first_ix: 3,
                last_ix: 3,
            },
        ];
        let expected_v_edges = [
            Edge {
                fpos: -240,
                opos: -246,
                pos: -246,
                flags: 0,
                dir: Direction::Left,
                blue_edge: Some(ScaledWidth {
                    scaled: -246,
                    fitted: -256,
                }),
                link_ix: None,
                serif_ix: Some(1),
                scale: 0,
                first_ix: 3,
                last_ix: 3,
            },
            Edge {
                fpos: 481,
                opos: 493,
                pos: 493,
                flags: 0,
                dir: Direction::Left,
                blue_edge: None,
                link_ix: Some(2),
                serif_ix: None,
                scale: 0,
                first_ix: 0,
                last_ix: 0,
            },
            Edge {
                fpos: 592,
                opos: 606,
                pos: 606,
                flags: Edge::ROUND | Edge::SERIF,
                dir: Direction::Right,
                blue_edge: Some(ScaledWidth {
                    scaled: 606,
                    fitted: 576,
                }),
                link_ix: Some(1),
                serif_ix: None,
                scale: 0,
                first_ix: 2,
                last_ix: 2,
            },
            Edge {
                fpos: 647,
                opos: 663,
                pos: 663,
                flags: 0,
                dir: Direction::Right,
                blue_edge: None,
                link_ix: None,
                serif_ix: Some(2),
                scale: 0,
                first_ix: 1,
                last_ix: 1,
            },
        ];
        check_edges(
            font_test_data::NOTOSERIFHEBREW_AUTOHINT_METRICS,
            GlyphId::new(9),
            style::StyleClass::HEBR,
            &expected_h_edges,
            &expected_v_edges,
        );
    }

    #[test]
    fn edges_cjk() {
        let expected_h_edges = [
            Edge {
                fpos: 138,
                opos: 141,
                pos: 141,
                flags: 0,
                dir: Direction::Up,
                blue_edge: None,
                link_ix: Some(1),
                serif_ix: None,
                scale: 0,
                first_ix: 8,
                last_ix: 8,
            },
            Edge {
                fpos: 201,
                opos: 206,
                pos: 206,
                flags: 0,
                dir: Direction::Down,
                blue_edge: None,
                link_ix: Some(0),
                serif_ix: None,
                scale: 0,
                first_ix: 7,
                last_ix: 7,
            },
            Edge {
                fpos: 458,
                opos: 469,
                pos: 469,
                flags: 0,
                dir: Direction::Down,
                blue_edge: None,
                link_ix: None,
                serif_ix: None,
                scale: 0,
                first_ix: 2,
                last_ix: 2,
            },
            Edge {
                fpos: 569,
                opos: 583,
                pos: 583,
                flags: 0,
                dir: Direction::Down,
                blue_edge: None,
                link_ix: None,
                serif_ix: None,
                scale: 0,
                first_ix: 6,
                last_ix: 6,
            },
            Edge {
                fpos: 670,
                opos: 686,
                pos: 686,
                flags: 0,
                dir: Direction::Up,
                blue_edge: None,
                link_ix: Some(6),
                serif_ix: None,
                scale: 0,
                first_ix: 1,
                last_ix: 1,
            },
            Edge {
                fpos: 693,
                opos: 710,
                pos: 710,
                flags: 0,
                dir: Direction::Up,
                blue_edge: None,
                link_ix: None,
                serif_ix: Some(7),
                scale: 0,
                first_ix: 4,
                last_ix: 4,
            },
            Edge {
                fpos: 731,
                opos: 749,
                pos: 749,
                flags: 0,
                dir: Direction::Down,
                blue_edge: None,
                link_ix: Some(4),
                serif_ix: None,
                scale: 0,
                first_ix: 0,
                last_ix: 0,
            },
            Edge {
                fpos: 849,
                opos: 869,
                pos: 869,
                flags: 0,
                dir: Direction::Up,
                blue_edge: None,
                link_ix: Some(8),
                serif_ix: None,
                scale: 0,
                first_ix: 5,
                last_ix: 5,
            },
            Edge {
                fpos: 911,
                opos: 933,
                pos: 933,
                flags: 0,
                dir: Direction::Down,
                blue_edge: None,
                link_ix: Some(7),
                serif_ix: None,
                scale: 0,
                first_ix: 3,
                last_ix: 3,
            },
        ];
        let expected_v_edges = [
            Edge {
                fpos: -78,
                opos: -80,
                pos: -80,
                flags: Edge::ROUND,
                dir: Direction::Left,
                blue_edge: Some(ScaledWidth {
                    scaled: -80,
                    fitted: -64,
                }),
                link_ix: None,
                serif_ix: None,
                scale: 0,
                first_ix: 8,
                last_ix: 8,
            },
            Edge {
                fpos: 3,
                opos: 3,
                pos: 3,
                flags: Edge::ROUND,
                dir: Direction::Right,
                blue_edge: None,
                link_ix: None,
                serif_ix: None,
                scale: 0,
                first_ix: 4,
                last_ix: 4,
            },
            Edge {
                fpos: 133,
                opos: 136,
                pos: 136,
                flags: Edge::ROUND,
                dir: Direction::Left,
                blue_edge: None,
                link_ix: None,
                serif_ix: None,
                scale: 0,
                first_ix: 2,
                last_ix: 2,
            },
            Edge {
                fpos: 547,
                opos: 560,
                pos: 560,
                flags: 0,
                dir: Direction::Left,
                blue_edge: None,
                link_ix: None,
                serif_ix: Some(5),
                scale: 0,
                first_ix: 6,
                last_ix: 6,
            },
            Edge {
                fpos: 576,
                opos: 590,
                pos: 590,
                flags: 0,
                dir: Direction::Right,
                blue_edge: None,
                link_ix: Some(5),
                serif_ix: None,
                scale: 0,
                first_ix: 5,
                last_ix: 5,
            },
            Edge {
                fpos: 576,
                opos: 590,
                pos: 590,
                flags: 0,
                dir: Direction::Left,
                blue_edge: None,
                link_ix: Some(4),
                serif_ix: None,
                scale: 0,
                first_ix: 7,
                last_ix: 7,
            },
            Edge {
                fpos: 729,
                opos: 746,
                pos: 746,
                flags: 0,
                dir: Direction::Left,
                blue_edge: None,
                link_ix: Some(7),
                serif_ix: None,
                scale: 0,
                first_ix: 1,
                last_ix: 1,
            },
            Edge {
                fpos: 758,
                opos: 776,
                pos: 776,
                flags: 0,
                dir: Direction::Right,
                blue_edge: None,
                link_ix: Some(6),
                serif_ix: None,
                scale: 0,
                first_ix: 0,
                last_ix: 3,
            },
            Edge {
                fpos: 788,
                opos: 807,
                pos: 807,
                flags: Edge::ROUND,
                dir: Direction::Left,
                blue_edge: None,
                link_ix: None,
                serif_ix: None,
                scale: 0,
                first_ix: 9,
                last_ix: 9,
            },
        ];
        check_edges(
            font_test_data::NOTOSERIFTC_AUTOHINT_METRICS,
            GlyphId::new(9),
            style::StyleClass::HANI,
            &expected_h_edges,
            &expected_v_edges,
        );
    }

    fn check_edges(
        font_data: &[u8],
        glyph_id: GlyphId,
        style_class: usize,
        expected_h_edges: &[Edge],
        expected_v_edges: &[Edge],
    ) {
        let font = FontRef::new(font_data).unwrap();
        let shaper = Shaper::new(&font, ShaperMode::Nominal);
        let class = &style::STYLE_CLASSES[style_class];
        let unscaled_metrics =
            metrics::compute_unscaled_style_metrics(&shaper, Default::default(), class);
        let scale = metrics::Scale::new(
            16.0,
            font.head().unwrap().units_per_em() as i32,
            Style::Normal,
            Default::default(),
            class.script.group,
        );
        let scaled_metrics = metrics::scale_style_metrics(&unscaled_metrics, scale);
        let glyphs = font.outline_glyphs();
        let glyph = glyphs.get(glyph_id).unwrap();
        let mut outline = Outline::default();
        outline.fill(&glyph, Default::default()).unwrap();
        let mut axes = [
            Axis::new(Axis::HORIZONTAL, outline.orientation),
            Axis::new(Axis::VERTICAL, outline.orientation),
        ];
        for (dim, axis) in axes.iter_mut().enumerate() {
            segments::compute_segments(&mut outline, axis, class.script.group);
            segments::link_segments(
                &outline,
                axis,
                scaled_metrics.axes[dim].scale,
                class.script.group,
                unscaled_metrics.axes[dim].max_width(),
            );
            compute_edges(
                axis,
                &scaled_metrics.axes[dim],
                class.script.hint_top_to_bottom,
                scaled_metrics.axes[1].scale,
                class.script.group,
            );
            compute_blue_edges(
                axis,
                &scale,
                &unscaled_metrics.axes[dim].blues,
                &scaled_metrics.axes[dim].blues,
                class.script.group,
            );
        }
        assert_eq!(axes[Axis::HORIZONTAL].edges.as_slice(), expected_h_edges);
        assert_eq!(axes[Axis::VERTICAL].edges.as_slice(), expected_v_edges);
    }
}
