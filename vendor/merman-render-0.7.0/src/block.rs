use crate::config::config_f64;
use crate::model::{BlockDiagramLayout, Bounds, LayoutEdge, LayoutLabel, LayoutNode, LayoutPoint};
use crate::text::{TextMeasurer, TextStyle, WrapMode};
use crate::{Error, Result};
use serde_json::{Map, Value};
use std::collections::HashMap;

pub(crate) type BlockDiagramModel = merman_core::diagrams::block::BlockDiagramRenderModel;
pub(crate) type BlockNode = merman_core::diagrams::block::BlockNodeRenderModel;

#[derive(Debug, Clone)]
struct SizedBlock {
    id: String,
    block_type: String,
    children: Vec<SizedBlock>,
    columns: i64,
    width_in_columns: i64,
    width: f64,
    height: f64,
    label_width: f64,
    label_height: f64,
    x: f64,
    y: f64,
}

fn decode_block_label_html(raw: &str) -> String {
    raw.replace("&nbsp;", "\u{00A0}")
}

pub(crate) fn block_label_is_effectively_empty(text: &str) -> bool {
    !text.is_empty()
        && text
            .chars()
            .all(|ch| ch != '\u{00A0}' && ch.is_whitespace())
}

fn block_html_label_metrics_px(
    text: &str,
    measurer: &dyn TextMeasurer,
    style: &TextStyle,
) -> (f64, f64) {
    let html_metrics = measurer.measure_wrapped(text, style, None, WrapMode::HtmlLike);
    let width =
        crate::generated::block_text_overrides_11_12_2::lookup_html_width_px(style.font_size, text)
            .unwrap_or(html_metrics.width)
            .max(0.0);
    let height = html_metrics.height.max(0.0);
    (width, height)
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct BlockArrowPoint {
    pub(crate) x: f64,
    pub(crate) y: f64,
}

pub(crate) fn block_arrow_points(
    directions: &[String],
    bbox_w: f64,
    bbox_h: f64,
    node_padding: f64,
) -> Vec<BlockArrowPoint> {
    fn expand_and_dedup(directions: &[String]) -> std::collections::BTreeSet<String> {
        let mut out = std::collections::BTreeSet::new();
        for d in directions {
            match d.trim() {
                "x" => {
                    out.insert("right".to_string());
                    out.insert("left".to_string());
                }
                "y" => {
                    out.insert("up".to_string());
                    out.insert("down".to_string());
                }
                other if !other.is_empty() => {
                    out.insert(other.to_string());
                }
                _ => {}
            }
        }
        out
    }

    let dirs = expand_and_dedup(directions);
    let height = bbox_h + 2.0 * node_padding;
    let midpoint = height / 2.0;
    let width = bbox_w + 2.0 * midpoint + node_padding;
    let pad = node_padding / 2.0;

    let has = |name: &str| dirs.contains(name);

    if has("right") && has("left") && has("up") && has("down") {
        return vec![
            BlockArrowPoint { x: 0.0, y: 0.0 },
            BlockArrowPoint {
                x: midpoint,
                y: 0.0,
            },
            BlockArrowPoint {
                x: width / 2.0,
                y: 2.0 * pad,
            },
            BlockArrowPoint {
                x: width - midpoint,
                y: 0.0,
            },
            BlockArrowPoint { x: width, y: 0.0 },
            BlockArrowPoint {
                x: width,
                y: -height / 3.0,
            },
            BlockArrowPoint {
                x: width + 2.0 * pad,
                y: -height / 2.0,
            },
            BlockArrowPoint {
                x: width,
                y: (-2.0 * height) / 3.0,
            },
            BlockArrowPoint {
                x: width,
                y: -height,
            },
            BlockArrowPoint {
                x: width - midpoint,
                y: -height,
            },
            BlockArrowPoint {
                x: width / 2.0,
                y: -height - 2.0 * pad,
            },
            BlockArrowPoint {
                x: midpoint,
                y: -height,
            },
            BlockArrowPoint { x: 0.0, y: -height },
            BlockArrowPoint {
                x: 0.0,
                y: (-2.0 * height) / 3.0,
            },
            BlockArrowPoint {
                x: -2.0 * pad,
                y: -height / 2.0,
            },
            BlockArrowPoint {
                x: 0.0,
                y: -height / 3.0,
            },
        ];
    }
    if has("right") && has("left") && has("up") {
        return vec![
            BlockArrowPoint {
                x: midpoint,
                y: 0.0,
            },
            BlockArrowPoint {
                x: width - midpoint,
                y: 0.0,
            },
            BlockArrowPoint {
                x: width,
                y: -height / 2.0,
            },
            BlockArrowPoint {
                x: width - midpoint,
                y: -height,
            },
            BlockArrowPoint {
                x: midpoint,
                y: -height,
            },
            BlockArrowPoint {
                x: 0.0,
                y: -height / 2.0,
            },
        ];
    }
    if has("right") && has("left") && has("down") {
        return vec![
            BlockArrowPoint { x: 0.0, y: 0.0 },
            BlockArrowPoint {
                x: midpoint,
                y: -height,
            },
            BlockArrowPoint {
                x: width - midpoint,
                y: -height,
            },
            BlockArrowPoint { x: width, y: 0.0 },
        ];
    }
    if has("right") && has("up") && has("down") {
        return vec![
            BlockArrowPoint { x: 0.0, y: 0.0 },
            BlockArrowPoint {
                x: width,
                y: -midpoint,
            },
            BlockArrowPoint {
                x: width,
                y: -height + midpoint,
            },
            BlockArrowPoint { x: 0.0, y: -height },
        ];
    }
    if has("left") && has("up") && has("down") {
        return vec![
            BlockArrowPoint { x: width, y: 0.0 },
            BlockArrowPoint {
                x: 0.0,
                y: -midpoint,
            },
            BlockArrowPoint {
                x: 0.0,
                y: -height + midpoint,
            },
            BlockArrowPoint {
                x: width,
                y: -height,
            },
        ];
    }
    if has("right") && has("left") {
        return vec![
            BlockArrowPoint {
                x: midpoint,
                y: 0.0,
            },
            BlockArrowPoint {
                x: midpoint,
                y: -pad,
            },
            BlockArrowPoint {
                x: width - midpoint,
                y: -pad,
            },
            BlockArrowPoint {
                x: width - midpoint,
                y: 0.0,
            },
            BlockArrowPoint {
                x: width,
                y: -height / 2.0,
            },
            BlockArrowPoint {
                x: width - midpoint,
                y: -height,
            },
            BlockArrowPoint {
                x: width - midpoint,
                y: -height + pad,
            },
            BlockArrowPoint {
                x: midpoint,
                y: -height + pad,
            },
            BlockArrowPoint {
                x: midpoint,
                y: -height,
            },
            BlockArrowPoint {
                x: 0.0,
                y: -height / 2.0,
            },
        ];
    }
    if has("up") && has("down") {
        return vec![
            BlockArrowPoint {
                x: width / 2.0,
                y: 0.0,
            },
            BlockArrowPoint { x: 0.0, y: -pad },
            BlockArrowPoint {
                x: midpoint,
                y: -pad,
            },
            BlockArrowPoint {
                x: midpoint,
                y: -height + pad,
            },
            BlockArrowPoint {
                x: 0.0,
                y: -height + pad,
            },
            BlockArrowPoint {
                x: width / 2.0,
                y: -height,
            },
            BlockArrowPoint {
                x: width,
                y: -height + pad,
            },
            BlockArrowPoint {
                x: width - midpoint,
                y: -height + pad,
            },
            BlockArrowPoint {
                x: width - midpoint,
                y: -pad,
            },
            BlockArrowPoint { x: width, y: -pad },
        ];
    }
    if has("right") && has("up") {
        return vec![
            BlockArrowPoint { x: 0.0, y: 0.0 },
            BlockArrowPoint {
                x: width,
                y: -midpoint,
            },
            BlockArrowPoint { x: 0.0, y: -height },
        ];
    }
    if has("right") && has("down") {
        return vec![
            BlockArrowPoint { x: 0.0, y: 0.0 },
            BlockArrowPoint { x: width, y: 0.0 },
            BlockArrowPoint { x: 0.0, y: -height },
        ];
    }
    if has("left") && has("up") {
        return vec![
            BlockArrowPoint { x: width, y: 0.0 },
            BlockArrowPoint {
                x: 0.0,
                y: -midpoint,
            },
            BlockArrowPoint {
                x: width,
                y: -height,
            },
        ];
    }
    if has("left") && has("down") {
        return vec![
            BlockArrowPoint { x: width, y: 0.0 },
            BlockArrowPoint { x: 0.0, y: 0.0 },
            BlockArrowPoint {
                x: width,
                y: -height,
            },
        ];
    }
    if has("right") {
        return vec![
            BlockArrowPoint {
                x: midpoint,
                y: -pad,
            },
            BlockArrowPoint {
                x: midpoint,
                y: -pad,
            },
            BlockArrowPoint {
                x: width - midpoint,
                y: -pad,
            },
            BlockArrowPoint {
                x: width - midpoint,
                y: 0.0,
            },
            BlockArrowPoint {
                x: width,
                y: -height / 2.0,
            },
            BlockArrowPoint {
                x: width - midpoint,
                y: -height,
            },
            BlockArrowPoint {
                x: width - midpoint,
                y: -height + pad,
            },
            BlockArrowPoint {
                x: midpoint,
                y: -height + pad,
            },
            BlockArrowPoint {
                x: midpoint,
                y: -height + pad,
            },
        ];
    }
    if has("left") {
        return vec![
            BlockArrowPoint {
                x: midpoint,
                y: 0.0,
            },
            BlockArrowPoint {
                x: midpoint,
                y: -pad,
            },
            BlockArrowPoint {
                x: width - midpoint,
                y: -pad,
            },
            BlockArrowPoint {
                x: width - midpoint,
                y: -height + pad,
            },
            BlockArrowPoint {
                x: midpoint,
                y: -height + pad,
            },
            BlockArrowPoint {
                x: midpoint,
                y: -height,
            },
            BlockArrowPoint {
                x: 0.0,
                y: -height / 2.0,
            },
        ];
    }
    if has("up") {
        return vec![
            BlockArrowPoint {
                x: midpoint,
                y: -pad,
            },
            BlockArrowPoint {
                x: midpoint,
                y: -height + pad,
            },
            BlockArrowPoint {
                x: 0.0,
                y: -height + pad,
            },
            BlockArrowPoint {
                x: width / 2.0,
                y: -height,
            },
            BlockArrowPoint {
                x: width,
                y: -height + pad,
            },
            BlockArrowPoint {
                x: width - midpoint,
                y: -height + pad,
            },
            BlockArrowPoint {
                x: width - midpoint,
                y: -pad,
            },
        ];
    }
    if has("down") {
        return vec![
            BlockArrowPoint {
                x: width / 2.0,
                y: 0.0,
            },
            BlockArrowPoint { x: 0.0, y: -pad },
            BlockArrowPoint {
                x: midpoint,
                y: -pad,
            },
            BlockArrowPoint {
                x: midpoint,
                y: -height + pad,
            },
            BlockArrowPoint {
                x: width - midpoint,
                y: -height + pad,
            },
            BlockArrowPoint {
                x: width - midpoint,
                y: -pad,
            },
            BlockArrowPoint { x: width, y: -pad },
        ];
    }

    vec![BlockArrowPoint { x: 0.0, y: 0.0 }]
}

fn polygon_bounds(points: &[BlockArrowPoint]) -> (f64, f64) {
    if points.is_empty() {
        return (0.0, 0.0);
    }

    let mut min_x = points[0].x;
    let mut max_x = points[0].x;
    let mut min_y = points[0].y;
    let mut max_y = points[0].y;
    for point in &points[1..] {
        min_x = min_x.min(point.x);
        max_x = max_x.max(point.x);
        min_y = min_y.min(point.y);
        max_y = max_y.max(point.y);
    }

    ((max_x - min_x).max(0.0), (max_y - min_y).max(0.0))
}

fn block_shape_size(
    block_type: &str,
    directions: &[String],
    label_width: f64,
    label_height: f64,
    padding: f64,
    has_label: bool,
) -> Option<(f64, f64)> {
    let rect_w = (label_width + padding).max(1.0);
    let rect_h = (label_height + padding).max(1.0);

    match block_type {
        "composite" => has_label.then(|| (label_width.max(1.0), (label_height + padding).max(1.0))),
        "group" => has_label.then_some((rect_w, rect_h)),
        "space" => None,
        "circle" => Some((rect_w, rect_w)),
        "doublecircle" => {
            let outer_diameter = rect_w + 10.0;
            Some((outer_diameter, outer_diameter))
        }
        "stadium" => Some(((label_width + rect_h / 4.0 + padding).max(1.0), rect_h)),
        "cylinder" => {
            let rx = rect_w / 2.0;
            let ry = rx / (2.5 + rect_w / 50.0);
            let body_h = (label_height + ry + padding).max(1.0);
            Some((rect_w, body_h + 2.0 * ry))
        }
        "diamond" => {
            let side = (rect_w + rect_h).max(1.0);
            Some((side, side))
        }
        "hexagon" => {
            let shoulder = rect_h / 4.0;
            Some(((label_width + 2.0 * shoulder + padding).max(1.0), rect_h))
        }
        "rect_left_inv_arrow" => Some((rect_w + rect_h / 2.0, rect_h)),
        "subroutine" => Some((rect_w + 16.0, rect_h)),
        "lean_right" | "trapezoid" | "inv_trapezoid" => {
            Some((rect_w + (2.0 * rect_h) / 3.0, rect_h))
        }
        "lean_left" => Some((rect_w + rect_h / 3.0, rect_h)),
        "block_arrow" => Some(polygon_bounds(&block_arrow_points(
            directions,
            label_width,
            label_height,
            padding,
        ))),
        _ => Some((rect_w, rect_h)),
    }
}

fn to_sized_block_shallow(
    node: &BlockNode,
    padding: f64,
    measurer: &dyn TextMeasurer,
    text_style: &TextStyle,
    children: Vec<SizedBlock>,
) -> SizedBlock {
    let columns = node.columns.unwrap_or(-1);
    let width_in_columns = node.width_in_columns.unwrap_or(1).max(1);

    let mut width = 0.0;
    let mut height = 0.0;

    // Mermaid renders block diagram labels via `labelHelper(...)`, which decodes HTML entities
    // and measures the resulting HTML content (`getBoundingClientRect()` for width/height).
    //
    // Block diagrams frequently use `&nbsp;` placeholders (notably for block arrows), so we must
    // decode those before measuring; otherwise node widths drift drastically.
    let label_decoded = decode_block_label_html(&node.label);
    let label_effectively_empty = block_label_is_effectively_empty(&label_decoded);
    let (label_width, label_height) = if label_effectively_empty {
        (0.0, 0.0)
    } else {
        block_html_label_metrics_px(&label_decoded, measurer, text_style)
    };
    let shape_label_height = label_height;

    if let Some((computed_width, computed_height)) = block_shape_size(
        node.block_type.as_str(),
        &node.directions,
        label_width,
        shape_label_height,
        padding,
        !label_effectively_empty && !label_decoded.trim().is_empty(),
    ) {
        width = computed_width;
        height = computed_height;
    }

    SizedBlock {
        id: node.id.clone(),
        block_type: node.block_type.clone(),
        children,
        columns,
        width_in_columns,
        width,
        height,
        label_width,
        label_height,
        x: 0.0,
        y: 0.0,
    }
}

fn to_sized_block(
    node: &BlockNode,
    padding: f64,
    measurer: &dyn TextMeasurer,
    text_style: &TextStyle,
) -> SizedBlock {
    let mut stack: Vec<(&BlockNode, bool)> = vec![(node, false)];
    let mut completed: HashMap<*const BlockNode, SizedBlock> = HashMap::new();

    while let Some((block, visited)) = stack.pop() {
        if visited {
            let children = block
                .children
                .iter()
                .filter_map(|child| completed.remove(&(child as *const BlockNode)))
                .collect();
            completed.insert(
                block as *const BlockNode,
                to_sized_block_shallow(block, padding, measurer, text_style, children),
            );
        } else {
            stack.push((block, true));
            for child in block.children.iter().rev() {
                stack.push((child, false));
            }
        }
    }

    completed
        .remove(&(node as *const BlockNode))
        .unwrap_or_else(|| to_sized_block_shallow(node, padding, measurer, text_style, Vec::new()))
}

fn get_max_child_size(block: &SizedBlock) -> (f64, f64) {
    let mut max_width = 0.0;
    let mut max_height = 0.0;
    for child in &block.children {
        if child.block_type == "space" {
            continue;
        }
        if child.width > max_width {
            max_width = child.width / (block.width_in_columns as f64);
        }
        if child.height > max_height {
            max_height = child.height;
        }
    }
    (max_width, max_height)
}

fn block_ref_at_path<'a>(root: &'a SizedBlock, path: &[usize]) -> &'a SizedBlock {
    let mut block = root;
    for &index in path {
        block = &block.children[index];
    }
    block
}

fn block_mut_at_path<'a>(root: &'a mut SizedBlock, path: &[usize]) -> &'a mut SizedBlock {
    let mut block = root;
    for &index in path {
        block = &mut block.children[index];
    }
    block
}

fn set_block_sizes_shallow(block: &mut SizedBlock, padding: f64) {
    if block.width <= 0.0 {
        block.width = 0.0;
        block.height = 0.0;
        block.x = 0.0;
        block.y = 0.0;
    }

    if block.children.is_empty() {
        return;
    }

    let (mut max_width, mut max_height) = get_max_child_size(block);

    for child in &mut block.children {
        child.width = max_width * (child.width_in_columns as f64)
            + padding * ((child.width_in_columns as f64) - 1.0);
        child.height = max_height;
        child.x = 0.0;
        child.y = 0.0;
    }

    for child in &mut block.children {
        child.x = 0.0;
        child.y = 0.0;
    }

    let (x_size, y_size) = block_grid_size(block);

    let mut width = (x_size as f64) * (max_width + padding) + padding;
    let height = (y_size as f64) * (max_height + padding) + padding;

    if width < block.width {
        width = block.width;
        let num = if block.columns > 0 {
            (block.children.len() as i64).min(block.columns)
        } else {
            block.children.len() as i64
        };
        if num > 0 {
            let child_width = (width - (num as f64) * padding - padding) / (num as f64);
            for child in &mut block.children {
                child.width = child_width;
            }
        }
    }

    block.width = width;
    block.height = height;
    block.x = 0.0;
    block.y = 0.0;

    // Keep behavior consistent with Mermaid even when all children were `space`.
    max_width = max_width.max(0.0);
    max_height = max_height.max(0.0);
    let _ = (max_width, max_height);
}

fn block_grid_size(block: &SizedBlock) -> (i64, i64) {
    let columns = block.columns;
    let mut num_items = 0i64;
    for child in &block.children {
        num_items += child.width_in_columns.max(1);
    }

    let mut x_size = block.children.len() as i64;
    if columns > 0 && columns < num_items {
        x_size = columns;
    }
    let y_size = ((num_items as f64) / (x_size.max(1) as f64)).ceil() as i64;
    (x_size, y_size)
}

fn propagate_parent_size_to_children(block: &mut SizedBlock, padding: f64) {
    if block.children.is_empty() {
        return;
    }

    let (max_width, _max_height) = get_max_child_size(block);
    let (x_size, y_size) = block_grid_size(block);
    let grid_width = (x_size as f64) * (max_width + padding) + padding;

    if grid_width < block.width {
        let child_width = (block.width - (x_size as f64) * padding - padding) / (x_size as f64);
        let child_height = (block.height - (y_size as f64) * padding - padding) / (y_size as f64);
        for child in &mut block.children {
            child.width = child_width;
            child.height = child_height;
            child.x = 0.0;
            child.y = 0.0;
        }
    }
}

fn set_block_sizes(block: &mut SizedBlock, padding: f64) {
    let mut stack: Vec<(Vec<usize>, bool)> = vec![(Vec::new(), false)];
    while let Some((path, visited)) = stack.pop() {
        if visited {
            let block = block_mut_at_path(block, &path);
            set_block_sizes_shallow(block, padding);
            continue;
        }

        let child_count = block_ref_at_path(block, &path).children.len();
        stack.push((path.clone(), true));
        for index in (0..child_count).rev() {
            let mut child_path = path.clone();
            child_path.push(index);
            stack.push((child_path, false));
        }
    }

    let mut stack: Vec<Vec<usize>> = vec![Vec::new()];
    while let Some(path) = stack.pop() {
        let child_count = {
            let block = block_mut_at_path(block, &path);
            propagate_parent_size_to_children(block, padding);
            block.children.len()
        };

        for index in (0..child_count).rev() {
            let mut child_path = path.clone();
            child_path.push(index);
            stack.push(child_path);
        }
    }
}

fn calculate_block_position(columns: i64, position: i64) -> (i64, i64) {
    if columns < 0 {
        return (position, 0);
    }
    if columns == 1 {
        return (0, position);
    }
    (position % columns, position / columns)
}

fn layout_blocks(block: &mut SizedBlock, padding: f64) {
    let mut stack: Vec<Vec<usize>> = vec![Vec::new()];
    while let Some(path) = stack.pop() {
        let child_count = {
            let block = block_mut_at_path(block, &path);
            if block.children.is_empty() {
                0
            } else {
                let columns = block.columns;
                let mut column_pos = 0i64;

                // JS truthiness: treat `0` as falsy (Mermaid uses `block?.size?.x ? ... : -padding`).
                let mut starting_pos_x = if block.x != 0.0 {
                    block.x + (-block.width / 2.0)
                } else {
                    -padding
                };
                let mut row_pos = 0i64;

                for child in &mut block.children {
                    let (px, py) = calculate_block_position(columns, column_pos);

                    if py != row_pos {
                        row_pos = py;
                        starting_pos_x = if block.x != 0.0 {
                            block.x + (-block.width / 2.0)
                        } else {
                            -padding
                        };
                    }

                    let half_width = child.width / 2.0;
                    child.x = starting_pos_x + padding + half_width;
                    starting_pos_x = child.x + half_width;

                    child.y = block.y - block.height / 2.0
                        + (py as f64) * (child.height + padding)
                        + child.height / 2.0
                        + padding;

                    let mut columns_filled = child.width_in_columns.max(1);
                    if columns > 0 {
                        let rem = columns - (column_pos % columns);
                        columns_filled = columns_filled.min(rem.max(1));
                    }
                    column_pos += columns_filled;

                    let _ = px;
                }
                block.children.len()
            }
        };

        for index in (0..child_count).rev() {
            let mut child_path = path.clone();
            child_path.push(index);
            stack.push(child_path);
        }
    }
}

fn find_bounds(block: &SizedBlock, b: &mut Bounds) {
    let mut stack = vec![block];
    while let Some(block) = stack.pop() {
        if block.id != "root" {
            b.min_x = b.min_x.min(block.x - block.width / 2.0);
            b.min_y = b.min_y.min(block.y - block.height / 2.0);
            b.max_x = b.max_x.max(block.x + block.width / 2.0);
            b.max_y = b.max_y.max(block.y + block.height / 2.0);
        }
        for child in block.children.iter().rev() {
            stack.push(child);
        }
    }
}

fn collect_nodes(block: &SizedBlock, out: &mut Vec<LayoutNode>) {
    let mut stack = vec![block];
    while let Some(block) = stack.pop() {
        if block.id != "root" && block.block_type != "space" {
            out.push(LayoutNode {
                id: block.id.clone(),
                x: block.x,
                y: block.y,
                width: block.width,
                height: block.height,
                is_cluster: false,
                label_width: Some(block.label_width.max(0.0)),
                label_height: Some(block.label_height.max(0.0)),
            });
        }
        for child in block.children.iter().rev() {
            stack.push(child);
        }
    }
}

fn invalid_block_model(message: impl Into<String>) -> Error {
    Error::InvalidModel {
        message: message.into(),
    }
}

fn required_string_field(obj: &Map<String, Value>, key: &str) -> Result<String> {
    match obj.get(key) {
        Some(Value::String(value)) => Ok(value.clone()),
        Some(other) => Err(invalid_block_model(format!(
            "block node field `{key}` must be a string, got {other:?}"
        ))),
        None => Err(invalid_block_model(format!(
            "block node missing required field `{key}`"
        ))),
    }
}

fn optional_string_field(obj: &Map<String, Value>, key: &str) -> Result<String> {
    match obj.get(key) {
        Some(Value::String(value)) => Ok(value.clone()),
        Some(other) => Err(invalid_block_model(format!(
            "block node field `{key}` must be a string, got {other:?}"
        ))),
        None => Ok(String::new()),
    }
}

fn optional_i64_field(obj: &Map<String, Value>, key: &str) -> Result<Option<i64>> {
    match obj.get(key) {
        Some(Value::Number(value)) => value.as_i64().map(Some).ok_or_else(|| {
            invalid_block_model(format!("block node field `{key}` must be an integer"))
        }),
        Some(Value::Null) | None => Ok(None),
        Some(other) => Err(invalid_block_model(format!(
            "block node field `{key}` must be an integer, got {other:?}"
        ))),
    }
}

fn string_array_field(obj: &Map<String, Value>, key: &str) -> Result<Vec<String>> {
    let Some(value) = obj.get(key) else {
        return Ok(Vec::new());
    };
    let Value::Array(items) = value else {
        return Err(invalid_block_model(format!(
            "block node field `{key}` must be an array"
        )));
    };

    items
        .iter()
        .map(|item| match item {
            Value::String(value) => Ok(value.clone()),
            other => Err(invalid_block_model(format!(
                "block node field `{key}` must contain strings, got {other:?}"
            ))),
        })
        .collect()
}

fn block_children_values<'a>(value: &'a Value) -> Result<&'a [Value]> {
    let obj = value
        .as_object()
        .ok_or_else(|| invalid_block_model("block node must be an object"))?;
    match obj.get("children") {
        Some(Value::Array(children)) => Ok(children),
        Some(other) => Err(invalid_block_model(format!(
            "block node field `children` must be an array, got {other:?}"
        ))),
        None => Ok(&[]),
    }
}

fn block_node_from_value_nonrecursive(value: &Value) -> Result<BlockNode> {
    let mut stack = vec![(value, false)];
    let mut completed: HashMap<*const Value, BlockNode> = HashMap::new();

    while let Some((current, visited)) = stack.pop() {
        if visited {
            let obj = current
                .as_object()
                .ok_or_else(|| invalid_block_model("block node must be an object"))?;
            let children = block_children_values(current)?
                .iter()
                .map(|child| {
                    completed.remove(&std::ptr::from_ref(child)).ok_or_else(|| {
                        invalid_block_model("block child node was not completed before parent")
                    })
                })
                .collect::<Result<Vec<_>>>()?;

            completed.insert(
                std::ptr::from_ref(current),
                BlockNode {
                    id: required_string_field(obj, "id")?,
                    label: optional_string_field(obj, "label")?,
                    block_type: optional_string_field(obj, "type")?,
                    children,
                    columns: optional_i64_field(obj, "columns")?,
                    width_in_columns: optional_i64_field(obj, "widthInColumns")?,
                    width: optional_i64_field(obj, "width")?,
                    classes: string_array_field(obj, "classes")?,
                    styles: string_array_field(obj, "styles")?,
                    directions: string_array_field(obj, "directions")?,
                },
            );
        } else {
            stack.push((current, true));
            for child in block_children_values(current)?.iter().rev() {
                stack.push((child, false));
            }
        }
    }

    completed
        .remove(&std::ptr::from_ref(value))
        .ok_or_else(|| invalid_block_model("block root node was not completed"))
}

pub(crate) fn block_model_from_semantic(semantic: &Value) -> Result<BlockDiagramModel> {
    let blocks_flat = match semantic.get("blocksFlat") {
        Some(Value::Array(items)) => items
            .iter()
            .map(block_node_from_value_nonrecursive)
            .collect::<Result<Vec<_>>>()?,
        Some(other) => {
            return Err(invalid_block_model(format!(
                "block semantic field `blocksFlat` must be an array, got {other:?}"
            )));
        }
        None => Vec::new(),
    };
    let edges = semantic
        .get("edges")
        .map(crate::json::from_value_ref)
        .transpose()?
        .unwrap_or_default();

    Ok(BlockDiagramModel { blocks_flat, edges })
}

pub fn layout_block_diagram(
    semantic: &Value,
    effective_config: &Value,
    measurer: &dyn TextMeasurer,
) -> Result<BlockDiagramLayout> {
    let model = block_model_from_semantic(semantic)?;
    layout_block_diagram_typed(&model, effective_config, measurer)
}

pub fn layout_block_diagram_typed(
    model: &merman_core::diagrams::block::BlockDiagramRenderModel,
    effective_config: &Value,
    measurer: &dyn TextMeasurer,
) -> Result<BlockDiagramLayout> {
    let padding = config_f64(effective_config, &["block", "padding"]).unwrap_or(8.0);
    let text_style = crate::text::TextStyle {
        font_family: Some(crate::config::config_font_family_or_first_array_css(
            effective_config,
        )),
        font_size: crate::config::config_theme_or_root_font_size_px(effective_config, 16.0)
            .max(1.0),
        font_weight: None,
    };

    let root = model
        .blocks_flat
        .iter()
        .find(|b| b.id == "root" && b.block_type == "composite")
        .ok_or_else(|| Error::InvalidModel {
            message: "missing block root composite".to_string(),
        })?;

    let mut root = to_sized_block(root, padding, measurer, &text_style);
    set_block_sizes(&mut root, padding);
    layout_blocks(&mut root, padding);

    let mut nodes: Vec<LayoutNode> = Vec::new();
    collect_nodes(&root, &mut nodes);

    let mut bounds = Bounds {
        min_x: 0.0,
        min_y: 0.0,
        max_x: 0.0,
        max_y: 0.0,
    };
    find_bounds(&root, &mut bounds);
    let bounds = if nodes.is_empty() { None } else { Some(bounds) };

    let nodes_by_id: HashMap<String, LayoutNode> =
        nodes.iter().cloned().map(|n| (n.id.clone(), n)).collect();

    let mut edges: Vec<LayoutEdge> = Vec::new();
    for e in &model.edges {
        let Some(from) = nodes_by_id.get(&e.start) else {
            continue;
        };
        let Some(to) = nodes_by_id.get(&e.end) else {
            continue;
        };

        let start = LayoutPoint {
            x: from.x,
            y: from.y,
        };
        let end = LayoutPoint { x: to.x, y: to.y };
        let mid = LayoutPoint {
            x: start.x + (end.x - start.x) / 2.0,
            y: start.y + (end.y - start.y) / 2.0,
        };

        let label = if e.label.trim().is_empty() {
            None
        } else {
            let edge_label = decode_block_label_html(&e.label);
            let (label_width, label_height) =
                block_html_label_metrics_px(&edge_label, measurer, &text_style);
            Some(LayoutLabel {
                x: mid.x,
                y: mid.y,
                width: label_width.max(1.0),
                height: label_height.max(1.0),
            })
        };

        edges.push(LayoutEdge {
            id: e.id.clone(),
            from: e.start.clone(),
            to: e.end.clone(),
            from_cluster: None,
            to_cluster: None,
            points: vec![start, mid, end],
            label,
            start_label_left: None,
            start_label_right: None,
            end_label_left: None,
            end_label_right: None,
            start_marker: e.arrow_type_start.clone(),
            end_marker: e.arrow_type_end.clone(),
            stroke_dasharray: None,
        });
    }

    Ok(BlockDiagramLayout {
        nodes,
        edges,
        bounds,
    })
}

#[cfg(test)]
mod tests {
    use crate::text::{TextStyle, VendoredFontMetricsTextMeasurer};

    fn default_style(font_size: f64) -> TextStyle {
        TextStyle {
            font_family: Some("\"trebuchet ms\", verdana, arial, sans-serif".to_string()),
            font_size,
            font_weight: None,
        }
    }

    #[test]
    fn block_label_metrics_use_block_owned_width_and_height_overrides() {
        let measurer = VendoredFontMetricsTextMeasurer::default();
        let style = default_style(24.0);

        let (width, height) = super::block_html_label_metrics_px(
            "Font size precedence should widen this block",
            &measurer,
            &style,
        );

        assert_eq!(width, 487.890625);
        assert_eq!(height, 36.0);
    }
}
