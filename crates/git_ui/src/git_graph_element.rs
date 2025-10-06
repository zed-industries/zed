use gpui::{
    AnyElement, Bounds, Element, ElementId, GlobalElementId, Hsla, InspectorElementId, LayoutId,
    Path, PathBuilder, Pixels, Point, Style, UniformListDecoration, Window, point, px,
};
use std::ops::Range;

#[derive(Clone, Copy, Debug)]
pub struct Coordinate {
    pub x: f32,
    pub y: f32,
}

#[derive(Clone, Debug)]
pub struct PositionedCommit {
    pub oid: String,
    pub author: String,
    pub message: String,
    pub date: String,
    pub position: Coordinate,
    pub color: Hsla,
    pub branches: Vec<String>,
    pub tags: Vec<String>,
    pub is_head: bool,
}

#[derive(Clone, Debug)]
pub struct BranchPath {
    pub coordinates: Vec<Coordinate>,
    pub color: Hsla,
}

pub struct GitGraphDecoration {
    all_commits: Vec<PositionedCommit>,
    all_paths: Vec<BranchPath>,
    partial_paths: Vec<BranchPath>,
    scroll_x: f32,
    graph_width: f32,
}

impl GitGraphDecoration {
    pub fn new(
        commits: Vec<PositionedCommit>,
        paths: Vec<BranchPath>,
        partial_paths: Vec<BranchPath>,
        scroll_x: f32,
        graph_width: f32,
    ) -> Self {
        Self {
            all_commits: commits,
            all_paths: paths,
            partial_paths,
            scroll_x,
            graph_width,
        }
    }
}

impl UniformListDecoration for GitGraphDecoration {
    fn compute(
        &self,
        visible_range: Range<usize>,
        bounds: Bounds<Pixels>,
        scroll_offset: Point<Pixels>,
        item_height: Pixels,
        item_count: usize,
        _window: &mut Window,
        _cx: &mut gpui::App,
    ) -> AnyElement {
        GitGraphElement {
            visible_range,
            all_commits: self.all_commits.clone(),
            all_paths: self.all_paths.clone(),
            partial_paths: self.partial_paths.clone(),
            bounds,
            item_height,
            scroll_x: self.scroll_x,
            graph_width: self.graph_width,
            scroll_offset_y: scroll_offset.y.0,
            total_item_count: item_count,
        }
        .into_any_element()
    }
}

pub struct GitGraphElement {
    visible_range: Range<usize>,
    all_commits: Vec<PositionedCommit>,
    all_paths: Vec<BranchPath>,
    partial_paths: Vec<BranchPath>,
    bounds: Bounds<Pixels>,
    item_height: Pixels,
    scroll_x: f32,
    graph_width: f32,
    scroll_offset_y: f32,
    total_item_count: usize,
}

impl Element for GitGraphElement {
    type RequestLayoutState = ();
    type PrepaintState = ();

    fn id(&self) -> Option<ElementId> {
        None
    }

    fn source_location(&self) -> Option<&'static core::panic::Location<'static>> {
        None
    }

    fn request_layout(
        &mut self,
        _id: Option<&GlobalElementId>,
        _inspector_id: Option<&InspectorElementId>,
        window: &mut Window,
        cx: &mut gpui::App,
    ) -> (LayoutId, Self::RequestLayoutState) {
        (window.request_layout(Style::default(), [], cx), ())
    }

    fn prepaint(
        &mut self,
        _id: Option<&GlobalElementId>,
        _inspector_id: Option<&InspectorElementId>,
        _bounds: Bounds<Pixels>,
        _request_layout: &mut Self::RequestLayoutState,
        _window: &mut Window,
        _cx: &mut gpui::App,
    ) -> Self::PrepaintState {
    }

    fn paint(
        &mut self,
        _id: Option<&GlobalElementId>,
        _inspector_id: Option<&InspectorElementId>,
        _bounds: Bounds<Pixels>,
        _request_layout: &mut Self::RequestLayoutState,
        _prepaint: &mut Self::PrepaintState,
        window: &mut Window,
        _cx: &mut gpui::App,
    ) {
        // Apply horizontal scroll offset only
        // Note: bounds.origin.y already accounts for vertical scroll in the decoration system
        let offset = point(
            self.bounds.origin.x - px(self.scroll_x),
            self.bounds.origin.y,
        );

        let graph_max_x = self.bounds.origin.x + px(self.graph_width);

        // Calculate total content height
        let total_content_height = self.total_item_count as f32 * self.item_height.0;

        // Clip to the visible graph area
        // The origin.y needs to account for scroll offset to extend the clipping area for paths
        let clip_bounds = Bounds {
            origin: point(
                self.bounds.origin.x,
                self.bounds.origin.y - px(self.scroll_offset_y),
            ),
            size: gpui::Size {
                width: px(self.graph_width),
                height: px(total_content_height + 1000.0), // Add padding for continuation lines
            },
        };

        window.with_content_mask(
            Some(gpui::ContentMask {
                bounds: clip_bounds,
            }),
            |window| {
                // Paint all regular paths (they handle their own visibility)
                for branch_path in &self.all_paths {
                    if let Some(path) =
                        Self::create_path_for_branch(&branch_path.coordinates, offset)
                    {
                        window.paint_path(path, branch_path.color);
                    }
                }

                // Paint partial paths (paths extending beyond loaded commits)
                // These represent branches that continue outside the visible range
                for partial_path in &self.partial_paths {
                    if let Some(path) =
                        Self::create_path_for_branch(&partial_path.coordinates, offset)
                    {
                        window.paint_path(path, partial_path.color);
                    }
                }

                // Paint commit circles for visible commits
                for (i, positioned) in self.all_commits.iter().enumerate() {
                    if i >= self.visible_range.start && i < self.visible_range.end {
                        let x = offset.x + px(positioned.position.x);
                        let y = offset.y + px(positioned.position.y);

                        if x.0 >= self.bounds.origin.x.0 && x < graph_max_x {
                            let center = point(x, y);
                            let radius = px(4.0);

                            let mut builder = PathBuilder::fill();
                            builder.move_to(point(center.x + radius, center.y));
                            builder.arc_to(
                                point(radius, radius),
                                px(0.),
                                false,
                                false,
                                point(center.x - radius, center.y),
                            );
                            builder.arc_to(
                                point(radius, radius),
                                px(0.),
                                false,
                                false,
                                point(center.x + radius, center.y),
                            );
                            builder.close();

                            if let Ok(path) = builder.build() {
                                window.paint_path(path, positioned.color);
                            }
                        }
                    }
                }
            },
        );
    }
}

impl GitGraphElement {
    fn create_path_for_branch(
        coordinates: &[Coordinate],
        offset: Point<Pixels>,
    ) -> Option<Path<Pixels>> {
        if coordinates.is_empty() {
            return None;
        }

        const CURVE_DISTANCE: f32 = 25.0;

        let mut builder = PathBuilder::stroke(px(2.5));
        let first = coordinates[0];
        builder.move_to(point(offset.x + px(first.x), offset.y + px(first.y)));

        for (i, coord) in coordinates.iter().enumerate().skip(1) {
            let prev = coordinates[i - 1];

            if prev.x != coord.x {
                let vertical_distance = (coord.y - prev.y).abs();

                if vertical_distance > CURVE_DISTANCE * 2.0 {
                    let curve_start_y = if coord.y > prev.y {
                        coord.y - CURVE_DISTANCE
                    } else {
                        coord.y + CURVE_DISTANCE
                    };

                    builder.line_to(point(offset.x + px(prev.x), offset.y + px(curve_start_y)));

                    let control_y = curve_start_y + (coord.y - curve_start_y) * 0.5;
                    let cp1 = Coordinate {
                        x: prev.x,
                        y: control_y,
                    };
                    let cp2 = Coordinate {
                        x: coord.x,
                        y: control_y,
                    };

                    builder.cubic_bezier_to(
                        point(offset.x + px(coord.x), offset.y + px(coord.y)),
                        point(offset.x + px(cp1.x), offset.y + px(cp1.y)),
                        point(offset.x + px(cp2.x), offset.y + px(cp2.y)),
                    );
                } else {
                    let middle_y = (prev.y + coord.y) / 2.0;
                    let p1 = Coordinate {
                        x: prev.x,
                        y: middle_y,
                    };
                    let p2 = Coordinate {
                        x: coord.x,
                        y: middle_y,
                    };

                    builder.cubic_bezier_to(
                        point(offset.x + px(coord.x), offset.y + px(coord.y)),
                        point(offset.x + px(p1.x), offset.y + px(p1.y)),
                        point(offset.x + px(p2.x), offset.y + px(p2.y)),
                    );
                }
            } else {
                builder.line_to(point(offset.x + px(coord.x), offset.y + px(coord.y)));
            }
        }

        builder.build().ok()
    }
}

impl gpui::IntoElement for GitGraphElement {
    type Element = Self;

    fn into_element(self) -> Self::Element {
        self
    }
}

impl GitGraphElement {
    pub fn into_any_element(self) -> gpui::AnyElement {
        self.into_any()
    }
}
