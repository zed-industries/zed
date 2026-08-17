use crate::Result;
use crate::config::{config_bool, config_f64, config_f64_css_px};
use crate::model::{
    Bounds, IshikawaDiagramLayout, IshikawaHeadLayout, IshikawaLabelBoxLayout, IshikawaLineLayout,
    IshikawaTextLayout,
};
use crate::text::{TextMeasurer, TextStyle, round_to_1_64_px_ties_to_even};
use merman_core::diagrams::ishikawa::{
    IshikawaDiagramRenderModel, IshikawaNodeRenderModel as IshikawaNode,
};
use serde_json::Value;
use std::collections::HashMap;

const FONT_SIZE_DEFAULT: f64 = 14.0;
const SPINE_BASE_LENGTH: f64 = 250.0;
const BONE_STUB: f64 = 30.0;
const BONE_BASE: f64 = 60.0;
const BONE_PER_CHILD: f64 = 5.0;
const COS_A: f64 = 0.139_173_100_960_065_47;
const SIN_A: f64 = 0.990_268_068_741_570_4;

#[derive(Debug, Clone)]
struct IshikawaConfig {
    padding: f64,
    use_max_width: bool,
    font_size: f64,
}

pub fn layout_ishikawa_diagram(
    semantic: &Value,
    effective_config: &Value,
    measurer: &dyn TextMeasurer,
) -> Result<IshikawaDiagramLayout> {
    let model: IshikawaDiagramRenderModel = crate::json::from_value_ref(semantic)?;
    layout_ishikawa_diagram_typed(&model, effective_config, measurer)
}

pub fn layout_ishikawa_diagram_typed(
    model: &IshikawaDiagramRenderModel,
    effective_config: &Value,
    measurer: &dyn TextMeasurer,
) -> Result<IshikawaDiagramLayout> {
    let cfg = ishikawa_config(effective_config);
    let Some(root) = model.root.as_ref() else {
        return Ok(IshikawaDiagramLayout {
            bounds: Some(Bounds {
                min_x: 0.0,
                min_y: 0.0,
                max_x: 1.0,
                max_y: 1.0,
            }),
            total_width: 1.0,
            total_height: 1.0,
            viewbox_x: 0.0,
            viewbox_y: 0.0,
            padding: cfg.padding,
            use_max_width: cfg.use_max_width,
            font_size: cfg.font_size,
            head: None,
            lines: Vec::new(),
            labels: Vec::new(),
            label_boxes: Vec::new(),
        });
    };

    let mut ctx = LayoutCtx {
        cfg: cfg.clone(),
        measurer,
        bounds: BoundsAcc::new(),
        head: None,
        lines: Vec::new(),
        labels: Vec::new(),
        label_boxes: Vec::new(),
    };

    let causes = root.children.as_slice();
    let mut spine_x = 0.0;
    let mut spine_y = SPINE_BASE_LENGTH;

    if causes.is_empty() {
        draw_head(&mut ctx, spine_x, spine_y, &root.text)?;
        draw_line(
            &mut ctx,
            spine_x,
            spine_y,
            spine_x,
            spine_y,
            "ishikawa-spine",
            false,
        );
        return Ok(ctx.into_layout());
    }

    spine_x -= 20.0;
    let upper_stats = side_stats(causes.iter().step_by(2));
    let lower_stats = side_stats(causes.iter().skip(1).step_by(2));
    let descendant_total = upper_stats.total + lower_stats.total;

    let mut upper_len = SPINE_BASE_LENGTH;
    let mut lower_len = SPINE_BASE_LENGTH;
    if descendant_total > 0 {
        let pool = SPINE_BASE_LENGTH * 2.0;
        let min_len = SPINE_BASE_LENGTH * 0.3;
        upper_len = (pool * (upper_stats.total as f64 / descendant_total as f64)).max(min_len);
        lower_len = (pool * (lower_stats.total as f64 / descendant_total as f64)).max(min_len);
    }

    let min_spacing = cfg.font_size * 2.0;
    upper_len = upper_len.max(upper_stats.max as f64 * min_spacing);
    lower_len = lower_len.max(lower_stats.max as f64 * min_spacing);

    spine_y = upper_len.max(SPINE_BASE_LENGTH);
    draw_head(&mut ctx, 0.0, spine_y, &root.text)?;

    let pair_count = causes.len().div_ceil(2);
    for pair in 0..pair_count {
        let mut pair_min_text_x = f64::INFINITY;
        if let Some(cause) = causes.get(pair * 2) {
            draw_branch(
                &mut ctx,
                cause,
                spine_x,
                spine_y,
                -1.0,
                upper_len,
                &mut pair_min_text_x,
            )?;
        }
        if let Some(cause) = causes.get(pair * 2 + 1) {
            draw_branch(
                &mut ctx,
                cause,
                spine_x,
                spine_y,
                1.0,
                lower_len,
                &mut pair_min_text_x,
            )?;
        }
        if pair_min_text_x.is_finite() {
            spine_x = pair_min_text_x;
        }
    }

    draw_line(
        &mut ctx,
        spine_x,
        spine_y,
        0.0,
        spine_y,
        "ishikawa-spine",
        false,
    );
    Ok(ctx.into_layout())
}

#[derive(Debug, Clone, Copy)]
struct SideStats {
    total: usize,
    max: usize,
}

fn side_stats<'a>(nodes: impl Iterator<Item = &'a IshikawaNode>) -> SideStats {
    nodes.fold(SideStats { total: 0, max: 0 }, |mut stats, node| {
        let descendants = count_descendants(node);
        stats.total += descendants;
        stats.max = stats.max.max(descendants);
        stats
    })
}

fn count_descendants(node: &IshikawaNode) -> usize {
    let mut count = 0usize;
    let mut stack: Vec<&IshikawaNode> = node.children.iter().collect();
    while let Some(child) = stack.pop() {
        count += 1;
        stack.extend(child.children.iter());
    }
    count
}

struct LayoutCtx<'a> {
    cfg: IshikawaConfig,
    measurer: &'a dyn TextMeasurer,
    bounds: BoundsAcc,
    head: Option<IshikawaHeadLayout>,
    lines: Vec<IshikawaLineLayout>,
    labels: Vec<IshikawaTextLayout>,
    label_boxes: Vec<IshikawaLabelBoxLayout>,
}

impl LayoutCtx<'_> {
    fn into_layout(self) -> IshikawaDiagramLayout {
        let padded = self.bounds.finish(self.cfg.padding).unwrap_or(Bounds {
            min_x: 0.0,
            min_y: 0.0,
            max_x: 1.0,
            max_y: 1.0,
        });
        let total_width = (padded.max_x - padded.min_x).max(1.0);
        let total_height = (padded.max_y - padded.min_y).max(1.0);
        let viewbox_x = padded.min_x;
        let viewbox_y = padded.min_y;
        IshikawaDiagramLayout {
            bounds: Some(padded),
            total_width,
            total_height,
            viewbox_x,
            viewbox_y,
            padding: self.cfg.padding,
            use_max_width: self.cfg.use_max_width,
            font_size: self.cfg.font_size,
            head: self.head,
            lines: self.lines,
            labels: self.labels,
            label_boxes: self.label_boxes,
        }
    }
}

fn draw_head(ctx: &mut LayoutCtx<'_>, x: f64, y: f64, label: &str) -> Result<()> {
    let max_chars = (110.0 / (ctx.cfg.font_size * 0.6)).floor().max(6.0) as usize;
    let wrapped = wrap_text(label, max_chars);
    let label_layout = text_layout(
        ctx,
        &wrapped,
        x + 42.0,
        y,
        "ishikawa-head-label",
        TextAnchor::Middle,
        VerticalMode::Middle,
    );
    let text_width = label_layout.bbox.max_x - label_layout.bbox.min_x;
    let text_height = label_layout.bbox.max_y - label_layout.bbox.min_y;
    let width = (text_width + 6.0).max(60.0);
    let height = (text_height * 2.0 + 40.0).max(40.0);
    let path_d = format!(
        "M 0 {} L 0 {} Q {} 0 0 {} Z",
        -height / 2.0,
        height / 2.0,
        width * 2.4,
        -height / 2.0
    );

    ctx.bounds
        .include_rect(x, y - height / 2.0, width * 1.2, height);
    ctx.bounds.include_bounds(&label_layout.bbox);
    ctx.head = Some(IshikawaHeadLayout {
        x,
        y,
        width,
        height,
        path_d,
        label: label_layout,
    });
    Ok(())
}

fn draw_branch(
    ctx: &mut LayoutCtx<'_>,
    node: &IshikawaNode,
    start_x: f64,
    start_y: f64,
    direction: f64,
    length: f64,
    pair_min_text_x: &mut f64,
) -> Result<()> {
    let children = node.children.as_slice();
    let line_len = length * if children.is_empty() { 0.2 } else { 1.0 };
    let dx = -COS_A * line_len;
    let dy = SIN_A * line_len * direction;
    let end_x = start_x + dx;
    let end_y = start_y + dy;

    draw_line(ctx, start_x, start_y, end_x, end_y, "ishikawa-branch", true);
    let cause_label = text_layout(
        ctx,
        &wrap_text(&node.text, 15),
        end_x,
        end_y + 11.0 * direction,
        "ishikawa-label cause",
        TextAnchor::Middle,
        VerticalMode::Middle,
    );
    *pair_min_text_x = pair_min_text_x.min(cause_label.bbox.min_x);
    add_label_with_box(ctx, cause_label);

    if children.is_empty() {
        return Ok(());
    }

    let flattened = flatten_tree(children, direction);
    let entry_count = flattened.entries.len();
    let mut ys = vec![start_y; entry_count];
    for (slot, entry_idx) in flattened.y_order.iter().enumerate() {
        ys[*entry_idx] = start_y + dy * ((slot + 1) as f64 / (entry_count + 1) as f64);
    }

    let mut bones = HashMap::new();
    bones.insert(
        -1isize,
        BoneInfo {
            x0: start_x,
            y0: start_y,
            x1: end_x,
            y1: end_y,
            child_count: children.len(),
            children_drawn: 0,
        },
    );

    let diagonal_x = -COS_A;
    let diagonal_y = SIN_A * direction;
    let odd_label_class = if direction < 0.0 {
        "ishikawa-label up"
    } else {
        "ishikawa-label down"
    };

    for (i, entry) in flattened.entries.iter().enumerate() {
        let y = ys[i];
        let parent = bones.get(&entry.parent_index).copied().unwrap_or(BoneInfo {
            x0: start_x,
            y0: start_y,
            x1: end_x,
            y1: end_y,
            child_count: children.len(),
            children_drawn: 0,
        });

        let (bx0, by0, bx1, text) = if entry.depth % 2 == 0 {
            let dy_parent = parent.y1 - parent.y0;
            let bx0 = lerp(
                parent.x0,
                parent.x1,
                if dy_parent != 0.0 {
                    (y - parent.y0) / dy_parent
                } else {
                    0.5
                },
            );
            let bx1 = bx0
                - if entry.child_count > 0 {
                    BONE_BASE + entry.child_count as f64 * BONE_PER_CHILD
                } else {
                    BONE_STUB
                };
            draw_line(ctx, bx0, y, bx1, y, "ishikawa-sub-branch", true);
            let text = text_layout(
                ctx,
                &entry.text,
                bx1,
                y,
                "ishikawa-label align",
                TextAnchor::End,
                VerticalMode::Middle,
            );
            (bx0, y, bx1, text)
        } else {
            let parent = bones.entry(entry.parent_index).or_insert(BoneInfo {
                x0: start_x,
                y0: start_y,
                x1: end_x,
                y1: end_y,
                child_count: children.len(),
                children_drawn: 0,
            });
            let k = parent.children_drawn;
            parent.children_drawn += 1;
            let bx0 = lerp(
                parent.x0,
                parent.x1,
                (parent.child_count - k) as f64 / (parent.child_count + 1) as f64,
            );
            let by0 = parent.y0;
            let bx1 = bx0 + diagonal_x * ((y - by0) / diagonal_y);
            draw_line(ctx, bx0, by0, bx1, y, "ishikawa-sub-branch", true);
            let text = text_layout(
                ctx,
                &entry.text,
                bx1,
                y,
                odd_label_class,
                TextAnchor::End,
                if direction < 0.0 {
                    VerticalMode::Baseline
                } else {
                    VerticalMode::Hanging
                },
            );
            (bx0, by0, bx1, text)
        };

        *pair_min_text_x = pair_min_text_x.min(text.bbox.min_x);
        ctx.bounds.include_bounds(&text.bbox);
        ctx.labels.push(text);

        if entry.child_count > 0 {
            bones.insert(
                i as isize,
                BoneInfo {
                    x0: bx0,
                    y0: by0,
                    x1: bx1,
                    y1: y,
                    child_count: entry.child_count,
                    children_drawn: 0,
                },
            );
        }
    }

    Ok(())
}

#[derive(Debug, Clone)]
struct FlattenedTree {
    entries: Vec<LabelEntry>,
    y_order: Vec<usize>,
}

#[derive(Debug, Clone)]
struct LabelEntry {
    text: String,
    depth: usize,
    parent_index: isize,
    child_count: usize,
}

fn flatten_tree(children: &[IshikawaNode], direction: f64) -> FlattenedTree {
    enum Action<'a> {
        Visit {
            node: &'a IshikawaNode,
            parent_index: isize,
            depth: usize,
        },
        PushY(usize),
    }

    fn push_nodes<'a>(
        stack: &mut Vec<Action<'a>>,
        nodes: &'a [IshikawaNode],
        parent_index: isize,
        depth: usize,
        direction: f64,
    ) {
        if direction < 0.0 {
            for node in nodes {
                stack.push(Action::Visit {
                    node,
                    parent_index,
                    depth,
                });
            }
        } else {
            for node in nodes.iter().rev() {
                stack.push(Action::Visit {
                    node,
                    parent_index,
                    depth,
                });
            }
        }
    }

    let mut out = FlattenedTree {
        entries: Vec::new(),
        y_order: Vec::new(),
    };
    let mut stack = Vec::new();
    push_nodes(&mut stack, children, -1, 2, direction);

    while let Some(action) = stack.pop() {
        match action {
            Action::Visit {
                node,
                parent_index,
                depth,
            } => {
                let idx = out.entries.len();
                let child_count = node.children.len();
                out.entries.push(LabelEntry {
                    text: wrap_text(&node.text, 15),
                    depth,
                    parent_index,
                    child_count,
                });
                if depth % 2 == 0 {
                    out.y_order.push(idx);
                    if child_count > 0 {
                        push_nodes(
                            &mut stack,
                            &node.children,
                            idx as isize,
                            depth + 1,
                            direction,
                        );
                    }
                } else {
                    stack.push(Action::PushY(idx));
                    if child_count > 0 {
                        push_nodes(
                            &mut stack,
                            &node.children,
                            idx as isize,
                            depth + 1,
                            direction,
                        );
                    }
                }
            }
            Action::PushY(idx) => out.y_order.push(idx),
        }
    }

    out
}

#[derive(Debug, Clone, Copy)]
struct BoneInfo {
    x0: f64,
    y0: f64,
    x1: f64,
    y1: f64,
    child_count: usize,
    children_drawn: usize,
}

#[derive(Debug, Clone, Copy)]
enum TextAnchor {
    Middle,
    End,
}

impl TextAnchor {
    fn as_str(self) -> &'static str {
        match self {
            Self::Middle => "middle",
            Self::End => "end",
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum VerticalMode {
    Middle,
    Baseline,
    Hanging,
}

fn add_label_with_box(ctx: &mut LayoutCtx<'_>, label: IshikawaTextLayout) {
    let box_layout = IshikawaLabelBoxLayout {
        x: label.bbox.min_x - 20.0,
        y: label.bbox.min_y - 2.0,
        width: (label.bbox.max_x - label.bbox.min_x) + 40.0,
        height: (label.bbox.max_y - label.bbox.min_y) + 4.0,
    };
    ctx.bounds.include_bounds(&label.bbox);
    ctx.bounds.include_rect(
        box_layout.x,
        box_layout.y,
        box_layout.width,
        box_layout.height,
    );
    ctx.label_boxes.push(box_layout);
    ctx.labels.push(label);
}

fn text_layout(
    ctx: &LayoutCtx<'_>,
    text: &str,
    x: f64,
    y: f64,
    class_name: &str,
    anchor: TextAnchor,
    vertical_mode: VerticalMode,
) -> IshikawaTextLayout {
    let lines = split_lines(text);
    let line_height = ctx.cfg.font_size * 1.05;
    let measure_style = ishikawa_text_measure_style(class_name, ctx.cfg.font_size);
    let horizontal = ishikawa_text_bbox_x(ctx, &lines, &measure_style, class_name, anchor);
    let width = horizontal.left + horizontal.right;
    let single_height = ishikawa_text_bbox_height_px(class_name, ctx.cfg.font_size);
    let height = if lines.is_empty() {
        single_height
    } else {
        (((lines.len() - 1) as f64 * line_height) + single_height).max(single_height)
    };

    let min_x = x - horizontal.left;
    let min_y = match vertical_mode {
        VerticalMode::Middle => {
            y - height / 2.0 - ishikawa_middle_bbox_y_offset_px(class_name, ctx.cfg.font_size)
        }
        VerticalMode::Baseline => y - height,
        VerticalMode::Hanging => y,
    };

    IshikawaTextLayout {
        text: text.to_string(),
        lines,
        class_name: class_name.to_string(),
        x,
        y,
        anchor: anchor.as_str().to_string(),
        line_height,
        font_size: ctx.cfg.font_size,
        bbox: Bounds {
            min_x,
            min_y,
            max_x: min_x + width,
            max_y: min_y + height,
        },
    }
}

fn ishikawa_text_measure_style(class_name: &str, font_size: f64) -> TextStyle {
    if class_name == "ishikawa-head-label" {
        TextStyle {
            font_size: 14.0,
            font_weight: Some("600".to_string()),
            ..Default::default()
        }
    } else {
        TextStyle {
            font_size,
            ..Default::default()
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct TextHorizontalBounds {
    left: f64,
    right: f64,
}

fn ishikawa_text_bbox_x(
    ctx: &LayoutCtx<'_>,
    lines: &[String],
    style: &TextStyle,
    class_name: &str,
    anchor: TextAnchor,
) -> TextHorizontalBounds {
    let mut out = TextHorizontalBounds {
        left: 0.0,
        right: 0.0,
    };

    for line in lines {
        let line_bounds = if class_name == "ishikawa-head-label" {
            let width = ctx.measurer.measure_svg_raw_text_bbox_width_px(line, style);
            TextHorizontalBounds {
                left: width / 2.0,
                right: width / 2.0,
            }
        } else {
            ishikawa_anchored_line_bbox_x(ctx, line, style, anchor)
        };
        out.left = out.left.max(line_bounds.left);
        out.right = out.right.max(line_bounds.right);
    }

    out
}

fn ishikawa_anchored_line_bbox_x(
    ctx: &LayoutCtx<'_>,
    line: &str,
    style: &TextStyle,
    anchor: TextAnchor,
) -> TextHorizontalBounds {
    let computed = ctx
        .measurer
        .measure_svg_text_computed_length_px(line, style)
        .max(0.0);
    let measured_width = ctx.measurer.measure(line, style).width.max(0.0);
    let anchored_advance = if (measured_width - computed).abs() <= 0.031_25 {
        round_to_1_64_px_ties_to_even(measured_width.max(computed))
    } else {
        computed
    };
    let (center_left, center_right) = ctx
        .measurer
        .measure_svg_text_bbox_x_with_ascii_overhang(line, style);
    let half = computed / 2.0;
    let start_overhang = (center_left - half).max(0.0);
    let end_overhang = (center_right - half).max(0.0);

    match anchor {
        TextAnchor::Middle => TextHorizontalBounds {
            left: center_left.max(0.0),
            right: center_right.max(0.0),
        },
        TextAnchor::End => TextHorizontalBounds {
            left: anchored_advance + start_overhang,
            right: end_overhang,
        },
    }
}

fn ishikawa_text_bbox_height_px(class_name: &str, font_size: f64) -> f64 {
    if class_name == "ishikawa-head-label" {
        16.0
    } else {
        (font_size.max(1.0) * 1.15).ceil()
    }
}

fn ishikawa_middle_bbox_y_offset_px(class_name: &str, font_size: f64) -> f64 {
    if class_name == "ishikawa-head-label" {
        0.0
    } else {
        font_size.max(1.0) * (21.0 / 256.0)
    }
}

fn draw_line(
    ctx: &mut LayoutCtx<'_>,
    x1: f64,
    y1: f64,
    x2: f64,
    y2: f64,
    class_name: &str,
    marker_start: bool,
) {
    ctx.bounds.include_point(x1, y1);
    ctx.bounds.include_point(x2, y2);
    ctx.lines.push(IshikawaLineLayout {
        x1,
        y1,
        x2,
        y2,
        class_name: class_name.to_string(),
        marker_start,
    });
}

fn wrap_text(text: &str, max_chars: usize) -> String {
    if text.len() <= max_chars {
        return text.to_string();
    }

    let mut lines: Vec<String> = Vec::new();
    for word in text.split_whitespace() {
        if let Some(last) = lines.last_mut() {
            if last.len() + 1 + word.len() <= max_chars {
                last.push(' ');
                last.push_str(word);
                continue;
            }
        }
        lines.push(word.to_string());
    }

    if lines.is_empty() {
        text.to_string()
    } else {
        lines.join("\n")
    }
}

fn split_lines(text: &str) -> Vec<String> {
    text.replace("<br/>", "\n")
        .replace("<br>", "\n")
        .lines()
        .map(str::to_string)
        .collect()
}

fn ishikawa_config(effective_config: &Value) -> IshikawaConfig {
    IshikawaConfig {
        padding: config_f64(effective_config, &["ishikawa", "diagramPadding"])
            .unwrap_or(20.0)
            .max(0.0),
        use_max_width: config_bool(effective_config, &["ishikawa", "useMaxWidth"]).unwrap_or(true),
        font_size: config_f64_css_px(effective_config, &["fontSize"])
            .unwrap_or(FONT_SIZE_DEFAULT)
            .max(1.0),
    }
}

fn lerp(a: f64, b: f64, t: f64) -> f64 {
    a + (b - a) * t
}

#[derive(Debug, Clone, Copy)]
struct BoundsAcc {
    min_x: f64,
    min_y: f64,
    max_x: f64,
    max_y: f64,
    has_value: bool,
}

impl BoundsAcc {
    fn new() -> Self {
        Self {
            min_x: 0.0,
            min_y: 0.0,
            max_x: 0.0,
            max_y: 0.0,
            has_value: false,
        }
    }

    fn include_point(&mut self, x: f64, y: f64) {
        if !self.has_value {
            self.min_x = x;
            self.max_x = x;
            self.min_y = y;
            self.max_y = y;
            self.has_value = true;
            return;
        }
        self.min_x = self.min_x.min(x);
        self.max_x = self.max_x.max(x);
        self.min_y = self.min_y.min(y);
        self.max_y = self.max_y.max(y);
    }

    fn include_rect(&mut self, x: f64, y: f64, width: f64, height: f64) {
        self.include_point(x, y);
        self.include_point(x + width, y + height);
    }

    fn include_bounds(&mut self, bounds: &Bounds) {
        self.include_point(bounds.min_x, bounds.min_y);
        self.include_point(bounds.max_x, bounds.max_y);
    }

    fn finish(self, padding: f64) -> Option<Bounds> {
        self.has_value.then_some(Bounds {
            min_x: self.min_x - padding,
            min_y: self.min_y - padding,
            max_x: self.max_x + padding,
            max_y: self.max_y + padding,
        })
    }
}
