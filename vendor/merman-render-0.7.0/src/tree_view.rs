use crate::config::{config_bool, config_f64};
use crate::model::{Bounds, TreeViewDiagramLayout, TreeViewLineLayout, TreeViewNodeLayout};
use crate::text::{TextMeasurer, TextStyle};
use crate::theme::PresentationTheme;
use crate::{Error, Result};
use merman_core::MAX_DIAGRAM_NESTING_DEPTH;
use merman_core::diagrams::tree_view::{
    TreeViewDiagramRenderModel, TreeViewNodeRenderModel as TreeViewNode,
};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug, Clone)]
struct TreeViewConfig {
    row_indent: f64,
    padding_x: f64,
    padding_y: f64,
    line_thickness: f64,
    use_max_width: bool,
    label_font_size: f64,
}

pub fn layout_tree_view_diagram(
    semantic: &Value,
    effective_config: &Value,
    measurer: &dyn TextMeasurer,
) -> Result<TreeViewDiagramLayout> {
    let model: TreeViewDiagramRenderModel = crate::json::from_value_ref(semantic)?;
    layout_tree_view_diagram_typed(&model, effective_config, measurer)
}

pub fn layout_tree_view_diagram_typed(
    model: &TreeViewDiagramRenderModel,
    effective_config: &Value,
    measurer: &dyn TextMeasurer,
) -> Result<TreeViewDiagramLayout> {
    let cfg = tree_view_config(effective_config);
    validate_tree_view_render_depth(&model.root)?;
    let style = TextStyle {
        font_size: cfg.label_font_size,
        ..Default::default()
    };
    let mut ctx = LayoutCtx {
        cfg: cfg.clone(),
        measurer,
        style,
        total_height: 0.0,
        total_width: 0.0,
        nodes: Vec::new(),
        lines: Vec::new(),
    };

    layout_tree(&mut ctx, &model.root);

    let min_x = -ctx.cfg.line_thickness / 2.0;
    let total_width = ctx.total_width.max(1.0);
    let total_height = ctx.total_height.max(1.0);
    Ok(TreeViewDiagramLayout {
        bounds: Some(Bounds {
            min_x,
            min_y: 0.0,
            max_x: total_width,
            max_y: total_height,
        }),
        total_width,
        total_height,
        row_indent: ctx.cfg.row_indent,
        padding_x: ctx.cfg.padding_x,
        padding_y: ctx.cfg.padding_y,
        line_thickness: ctx.cfg.line_thickness,
        use_max_width: ctx.cfg.use_max_width,
        label_font_size: ctx.cfg.label_font_size,
        nodes: ctx.nodes,
        lines: ctx.lines,
    })
}

fn validate_tree_view_render_depth(root: &TreeViewNode) -> Result<()> {
    let mut stack = vec![(root, 0usize)];
    while let Some((node, depth)) = stack.pop() {
        if depth > MAX_DIAGRAM_NESTING_DEPTH {
            return Err(Error::InvalidModel {
                message: format!("treeView nesting depth exceeds {MAX_DIAGRAM_NESTING_DEPTH}"),
            });
        }
        for child in &node.children {
            stack.push((child, depth.saturating_add(1)));
        }
    }
    Ok(())
}

struct LayoutCtx<'a> {
    cfg: TreeViewConfig,
    measurer: &'a dyn TextMeasurer,
    style: TextStyle,
    total_height: f64,
    total_width: f64,
    nodes: Vec<TreeViewNodeLayout>,
    lines: Vec<TreeViewLineLayout>,
}

enum LayoutFrame<'a> {
    Enter {
        node: &'a TreeViewNode,
        depth: usize,
    },
    Exit {
        node: &'a TreeViewNode,
        node_index: usize,
    },
}

fn layout_tree(ctx: &mut LayoutCtx<'_>, root: &TreeViewNode) {
    let mut stack = vec![LayoutFrame::Enter {
        node: root,
        depth: 0,
    }];
    let mut node_indices: HashMap<*const TreeViewNode, usize> = HashMap::new();

    while let Some(frame) = stack.pop() {
        match frame {
            LayoutFrame::Enter { node, depth } => {
                let node_index = push_node_layout(ctx, node, depth);
                node_indices.insert(std::ptr::from_ref(node), node_index);
                stack.push(LayoutFrame::Exit { node, node_index });
                for child in node.children.iter().rev() {
                    stack.push(LayoutFrame::Enter {
                        node: child,
                        depth: depth.saturating_add(1),
                    });
                }
            }
            LayoutFrame::Exit { node, node_index } => {
                if let Some(last_child) = node.children.last() {
                    if let Some(last_child_idx) =
                        node_indices.get(&std::ptr::from_ref(last_child)).copied()
                    {
                        push_vertical_line(ctx, node_index, last_child_idx);
                    }
                }
            }
        }
    }
}

fn push_node_layout(ctx: &mut LayoutCtx<'_>, node: &TreeViewNode, depth: usize) -> usize {
    let indent = depth as f64 * (ctx.cfg.row_indent + ctx.cfg.padding_x);
    let label_width = tree_view_label_bbox_width_px(ctx.measurer, &node.name, &ctx.style);
    let label_height = tree_view_label_bbox_height_px(ctx.cfg.label_font_size);
    let height = label_height + ctx.cfg.padding_y * 2.0;
    let width = label_width + ctx.cfg.padding_x * 2.0;
    let y = ctx.total_height;
    let idx = ctx.nodes.len();

    ctx.nodes.push(TreeViewNodeLayout {
        id: node.id,
        level: node.level,
        name: node.name.clone(),
        depth,
        x: indent,
        y,
        width,
        height,
        label_x: indent + ctx.cfg.padding_x,
        label_y: y + height / 2.0,
        label_width,
        label_height,
    });
    ctx.lines.push(TreeViewLineLayout {
        x1: indent - ctx.cfg.row_indent,
        y1: y + height / 2.0,
        x2: indent,
        y2: y + height / 2.0,
        stroke_width: ctx.cfg.line_thickness,
        kind: "horizontal".to_string(),
    });

    ctx.total_width = ctx.total_width.max(indent + width);
    ctx.total_height += height;

    idx
}

fn push_vertical_line(ctx: &mut LayoutCtx<'_>, node_index: usize, last_child_idx: usize) {
    let Some(current) = ctx.nodes.get(node_index) else {
        return;
    };
    let Some(last_child) = ctx.nodes.get(last_child_idx) else {
        return;
    };
    let current_x = current.x;
    let current_y = current.y;
    let current_height = current.height;
    let last_child_y = last_child.y;
    let last_child_height = last_child.height;

    ctx.lines.push(TreeViewLineLayout {
        x1: current_x + ctx.cfg.padding_x,
        y1: current_y + current_height,
        x2: current_x + ctx.cfg.padding_x,
        y2: last_child_y + last_child_height / 2.0 + ctx.cfg.line_thickness / 2.0,
        stroke_width: ctx.cfg.line_thickness,
        kind: "vertical".to_string(),
    });
}

fn tree_view_config(effective_config: &Value) -> TreeViewConfig {
    let theme = PresentationTheme::new(effective_config).tree_view();
    TreeViewConfig {
        row_indent: config_f64(effective_config, &["treeView", "rowIndent"])
            .unwrap_or(10.0)
            .max(0.0),
        padding_x: config_f64(effective_config, &["treeView", "paddingX"])
            .unwrap_or(5.0)
            .max(0.0),
        padding_y: config_f64(effective_config, &["treeView", "paddingY"])
            .unwrap_or(5.0)
            .max(0.0),
        line_thickness: config_f64(effective_config, &["treeView", "lineThickness"])
            .unwrap_or(1.0)
            .max(0.0),
        use_max_width: config_bool(effective_config, &["treeView", "useMaxWidth"]).unwrap_or(true),
        label_font_size: theme.label_font_size,
    }
}

fn tree_view_label_bbox_height_px(font_size: f64) -> f64 {
    (font_size.max(1.0) * 1.15).ceil()
}

fn tree_view_label_bbox_width_px(
    measurer: &dyn TextMeasurer,
    label: &str,
    style: &TextStyle,
) -> f64 {
    if style.font_size > 16.0 {
        measurer.measure(label, style).width.max(0.0)
    } else {
        measurer
            .measure_svg_raw_text_bbox_width_px(label, style)
            .max(0.0)
    }
}
