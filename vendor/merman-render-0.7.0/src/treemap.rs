use crate::config::{config_bool, config_f64, config_string, json_f64};
use crate::model::{TreemapDiagramLayout, TreemapLeafLayout, TreemapSectionLayout};
use crate::{Error, Result};
use merman_core::diagrams::treemap::{
    TreemapDiagramRenderModel, TreemapNodeRenderModel as TreemapNode,
};
use serde_json::Value;
use std::collections::HashMap;

pub(crate) const TREEMAP_SECTION_INNER_PADDING_PX: f64 = 10.0;
pub(crate) const TREEMAP_SECTION_HEADER_HEIGHT_PX: f64 = 25.0;

#[derive(Debug, Clone)]
struct HierNode {
    name: String,
    own_value: f64,
    value: f64,
    class_selector: Option<String>,
    css_compiled_styles: Option<Vec<String>>,
    parent: Option<usize>,
    children: Vec<usize>,
    depth: usize,
    x0: f64,
    y0: f64,
    x1: f64,
    y1: f64,
}

#[derive(Debug, Clone)]
struct TreemapConfig {
    use_max_width: bool,
    padding: f64,
    diagram_padding: f64,
    show_values: bool,
    node_width: f64,
    node_height: f64,
    value_format: String,
}

fn treemap_config(effective_config: &Value) -> TreemapConfig {
    // Mermaid 11.12.2 treemap defaults live in `defaultConfig.ts`, not in the YAML schema.
    // Keep these in sync with:
    // - `repo-ref/mermaid/packages/mermaid/src/defaultConfig.ts`
    // - `repo-ref/mermaid/packages/mermaid/src/diagrams/treemap/renderer.ts`
    let use_max_width = config_bool(effective_config, &["treemap", "useMaxWidth"]).unwrap_or(true);
    let padding = config_f64(effective_config, &["treemap", "padding"]).unwrap_or(10.0);
    let diagram_padding =
        config_f64(effective_config, &["treemap", "diagramPadding"]).unwrap_or(8.0);
    let show_values = config_bool(effective_config, &["treemap", "showValues"]).unwrap_or(true);
    let node_width = config_f64(effective_config, &["treemap", "nodeWidth"]).unwrap_or(100.0);
    let node_height = config_f64(effective_config, &["treemap", "nodeHeight"]).unwrap_or(40.0);
    let value_format = config_string(effective_config, &["treemap", "valueFormat"])
        .unwrap_or_else(|| ",".to_string());

    TreemapConfig {
        use_max_width,
        padding,
        diagram_padding,
        show_values,
        node_width,
        node_height,
        value_format,
    }
}

fn push_node(nodes: &mut Vec<HierNode>, node: &TreemapNode, parent: Option<usize>, depth: usize) {
    let mut stack = vec![(node, parent, depth)];
    while let Some((current, parent_idx, current_depth)) = stack.pop() {
        let own_value = current.value.as_ref().and_then(json_f64).unwrap_or(0.0);
        let idx = nodes.len();
        nodes.push(HierNode {
            name: current.name.clone(),
            own_value,
            value: 0.0,
            class_selector: current.class_selector.clone(),
            css_compiled_styles: current.css_compiled_styles.clone(),
            parent: parent_idx,
            children: Vec::new(),
            depth: current_depth,
            x0: 0.0,
            y0: 0.0,
            x1: 0.0,
            y1: 0.0,
        });

        if let Some(parent_idx) = parent_idx {
            if let Some(parent_node) = nodes.get_mut(parent_idx) {
                parent_node.children.push(idx);
            }
        }

        if let Some(children) = current.children.as_ref() {
            for child in children.iter().rev() {
                stack.push((child, Some(idx), current_depth.saturating_add(1)));
            }
        }
    }
}

fn compute_sum(nodes: &mut [HierNode], idx: usize) -> f64 {
    let mut stack = vec![(idx, false)];
    while let Some((node_idx, visited)) = stack.pop() {
        let Some(node) = nodes.get(node_idx) else {
            continue;
        };

        if visited {
            let sum = node.own_value
                + node
                    .children
                    .iter()
                    .filter_map(|&child_idx| nodes.get(child_idx).map(|child| child.value))
                    .sum::<f64>();
            if let Some(node) = nodes.get_mut(node_idx) {
                node.value = sum;
            }
        } else {
            stack.push((node_idx, true));
            for &child_idx in node.children.iter().rev() {
                stack.push((child_idx, false));
            }
        }
    }

    nodes.get(idx).map(|node| node.value).unwrap_or(0.0)
}

fn sort_children_by_value(nodes: &mut [HierNode], idx: usize) {
    let mut stack = vec![idx];
    while let Some(node_idx) = stack.pop() {
        if node_idx >= nodes.len() {
            continue;
        }

        let mut items = nodes[node_idx]
            .children
            .iter()
            .copied()
            .enumerate()
            .map(|(pos, child)| (child, pos))
            .collect::<Vec<_>>();
        items.sort_by(|(a, a_pos), (b, b_pos)| {
            let av = nodes[*a].value;
            let bv = nodes[*b].value;
            bv.partial_cmp(&av)
                .unwrap_or(std::cmp::Ordering::Equal)
                .then_with(|| a_pos.cmp(b_pos))
        });
        nodes[node_idx].children = items.into_iter().map(|(child, _pos)| child).collect();

        let children = nodes[node_idx].children.clone();
        for child_idx in children.into_iter().rev() {
            stack.push(child_idx);
        }
    }
}

fn each_before(nodes: &[HierNode], root: usize) -> Vec<usize> {
    let mut out = Vec::new();
    let mut stack = vec![root];
    while let Some(idx) = stack.pop() {
        out.push(idx);
        let children = &nodes[idx].children;
        for &c in children.iter().rev() {
            stack.push(c);
        }
    }
    out
}

fn descendants_bfs(nodes: &[HierNode], root: usize) -> Vec<usize> {
    let mut out = Vec::new();
    let mut next = vec![root];
    while !next.is_empty() {
        let mut current = next;
        current.reverse();
        next = Vec::new();
        while let Some(idx) = current.pop() {
            out.push(idx);
            for &c in &nodes[idx].children {
                next.push(c);
            }
        }
    }
    out
}

fn leaves_each_before(nodes: &[HierNode], root: usize) -> Vec<usize> {
    let mut out = Vec::new();
    for idx in each_before(nodes, root) {
        if nodes[idx].children.is_empty() {
            out.push(idx);
        }
    }
    out
}

fn treemap_round_node(nodes: &mut [HierNode], idx: usize) {
    nodes[idx].x0 = nodes[idx].x0.round();
    nodes[idx].y0 = nodes[idx].y0.round();
    nodes[idx].x1 = nodes[idx].x1.round();
    nodes[idx].y1 = nodes[idx].y1.round();
}

fn treemap_dice(
    nodes: &mut [HierNode],
    children: &[usize],
    row_value: f64,
    x0: f64,
    y0: f64,
    x1: f64,
    y1: f64,
) {
    let mut x = x0;
    let k = if row_value != 0.0 {
        (x1 - x0) / row_value
    } else {
        0.0
    };
    for &child in children {
        nodes[child].y0 = y0;
        nodes[child].y1 = y1;
        nodes[child].x0 = x;
        x += nodes[child].value * k;
        nodes[child].x1 = x;
    }
}

fn treemap_slice(
    nodes: &mut [HierNode],
    children: &[usize],
    row_value: f64,
    x0: f64,
    y0: f64,
    x1: f64,
    y1: f64,
) {
    let mut y = y0;
    let k = if row_value != 0.0 {
        (y1 - y0) / row_value
    } else {
        0.0
    };
    for &child in children {
        nodes[child].x0 = x0;
        nodes[child].x1 = x1;
        nodes[child].y0 = y;
        y += nodes[child].value * k;
        nodes[child].y1 = y;
    }
}

fn squarify(nodes: &mut [HierNode], parent: usize, mut x0: f64, mut y0: f64, x1: f64, y1: f64) {
    const PHI: f64 = (1.0 + 2.23606797749979) / 2.0;
    let ratio = PHI;

    let children = nodes[parent].children.clone();
    if children.is_empty() {
        return;
    }

    let n = children.len();
    let mut i0 = 0usize;
    let mut i1 = 0usize;
    let mut value = nodes[parent].value;

    while i0 < n {
        let dx = x1 - x0;
        let dy = y1 - y0;

        let mut sum_value;
        loop {
            if i1 >= n {
                return;
            }
            sum_value = nodes[children[i1]].value;
            i1 += 1;
            if sum_value != 0.0 || i1 >= n {
                break;
            }
        }

        let mut min_value = sum_value;
        let mut max_value = sum_value;

        let alpha = (dy / dx).max(dx / dy) / (value * ratio);
        let mut beta = sum_value * sum_value * alpha;
        let mut min_ratio = (max_value / beta).max(beta / min_value);

        while i1 < n {
            let node_value = nodes[children[i1]].value;
            sum_value += node_value;
            if node_value < min_value {
                min_value = node_value;
            }
            if node_value > max_value {
                max_value = node_value;
            }
            beta = sum_value * sum_value * alpha;
            let new_ratio = (max_value / beta).max(beta / min_value);
            if new_ratio > min_ratio {
                sum_value -= node_value;
                break;
            }
            min_ratio = new_ratio;
            i1 += 1;
        }

        let dice = dx < dy;
        let row_children = &children[i0..i1];
        if dice {
            let y2 = if value != 0.0 {
                y0 + dy * sum_value / value
            } else {
                y1
            };
            treemap_dice(nodes, row_children, sum_value, x0, y0, x1, y2);
            y0 = y2;
        } else {
            let x2 = if value != 0.0 {
                x0 + dx * sum_value / value
            } else {
                x1
            };
            treemap_slice(nodes, row_children, sum_value, x0, y0, x2, y1);
            x0 = x2;
        }

        value -= sum_value;
        i0 = i1;
    }
}

fn position_node(
    nodes: &mut [HierNode],
    idx: usize,
    padding_stack: &mut Vec<f64>,
    padding_inner: f64,
) {
    let depth = nodes[idx].depth;
    if padding_stack.len() <= depth {
        padding_stack.resize(depth + 1, 0.0);
    }
    let mut p = padding_stack[depth];
    let mut x0 = nodes[idx].x0 + p;
    let mut y0 = nodes[idx].y0 + p;
    let mut x1 = nodes[idx].x1 - p;
    let mut y1 = nodes[idx].y1 - p;
    if x1 < x0 {
        x0 = (x0 + x1) / 2.0;
        x1 = x0;
    }
    if y1 < y0 {
        y0 = (y0 + y1) / 2.0;
        y1 = y0;
    }
    nodes[idx].x0 = x0;
    nodes[idx].y0 = y0;
    nodes[idx].x1 = x1;
    nodes[idx].y1 = y1;

    if nodes[idx].children.is_empty() {
        return;
    }

    p = padding_inner / 2.0;
    if padding_stack.len() <= depth + 1 {
        padding_stack.resize(depth + 2, 0.0);
    }
    padding_stack[depth + 1] = p;

    let has_children = true;
    let padding_top = if has_children {
        TREEMAP_SECTION_HEADER_HEIGHT_PX + TREEMAP_SECTION_INNER_PADDING_PX
    } else {
        0.0
    };
    let padding_lr = if has_children {
        TREEMAP_SECTION_INNER_PADDING_PX
    } else {
        0.0
    };
    let padding_bottom = if has_children {
        TREEMAP_SECTION_INNER_PADDING_PX
    } else {
        0.0
    };

    x0 += padding_lr - p;
    y0 += padding_top - p;
    x1 -= padding_lr - p;
    y1 -= padding_bottom - p;
    if x1 < x0 {
        x0 = (x0 + x1) / 2.0;
        x1 = x0;
    }
    if y1 < y0 {
        y0 = (y0 + y1) / 2.0;
        y1 = y0;
    }

    squarify(nodes, idx, x0, y0, x1, y1);
}

pub fn layout_treemap_diagram(
    semantic: &Value,
    effective_config: &Value,
    _measurer: &dyn crate::text::TextMeasurer,
) -> Result<TreemapDiagramLayout> {
    let model = treemap_model_from_semantic(semantic)?;
    layout_treemap_diagram_typed(&model, effective_config, _measurer)
}

fn treemap_model_from_semantic(semantic: &Value) -> Result<TreemapDiagramRenderModel> {
    let root = semantic
        .get("root")
        .ok_or_else(|| invalid_treemap_model("missing root"))?;

    Ok(TreemapDiagramRenderModel {
        acc_title: optional_string_field(semantic, "accTitle")?,
        acc_descr: optional_string_field(semantic, "accDescr")?,
        title: optional_string_field(semantic, "title")?,
        root: treemap_node_from_value(root)?,
    })
}

fn treemap_node_from_value(root: &Value) -> Result<TreemapNode> {
    let mut models: HashMap<*const Value, TreemapNode> = HashMap::new();
    let mut stack = vec![(root, false)];

    while let Some((node_value, visited)) = stack.pop() {
        let node_ptr = std::ptr::from_ref(node_value);
        if visited {
            let children = optional_children_array(node_value)?.map(|children| {
                children
                    .iter()
                    .filter_map(|child| models.remove(&std::ptr::from_ref(child)))
                    .collect::<Vec<_>>()
            });
            models.insert(
                node_ptr,
                TreemapNode {
                    name: required_string_field(node_value, "name")?,
                    children,
                    value: optional_value_field(node_value, "value"),
                    class_selector: optional_string_field(node_value, "classSelector")?,
                    css_compiled_styles: optional_string_vec_field(
                        node_value,
                        "cssCompiledStyles",
                    )?,
                },
            );
        } else {
            stack.push((node_value, true));
            if let Some(children) = optional_children_array(node_value)? {
                for child in children.iter().rev() {
                    stack.push((child, false));
                }
            }
        }
    }

    models
        .remove(&std::ptr::from_ref(root))
        .ok_or_else(|| invalid_treemap_model("root projection failed"))
}

fn optional_children_array(value: &Value) -> Result<Option<&Vec<Value>>> {
    match value.get("children") {
        None | Some(Value::Null) => Ok(None),
        Some(Value::Array(children)) => Ok(Some(children)),
        Some(_) => Err(invalid_treemap_model("children must be an array")),
    }
}

fn required_string_field(value: &Value, field: &str) -> Result<String> {
    match value.get(field) {
        Some(Value::String(v)) => Ok(v.clone()),
        _ => Err(invalid_treemap_model(format!("{field} must be a string"))),
    }
}

fn optional_string_field(value: &Value, field: &str) -> Result<Option<String>> {
    match value.get(field) {
        None | Some(Value::Null) => Ok(None),
        Some(Value::String(v)) => Ok(Some(v.clone())),
        Some(_) => Err(invalid_treemap_model(format!("{field} must be a string"))),
    }
}

fn optional_string_vec_field(value: &Value, field: &str) -> Result<Option<Vec<String>>> {
    match value.get(field) {
        None | Some(Value::Null) => Ok(None),
        Some(Value::Array(values)) => values
            .iter()
            .map(|v| {
                v.as_str()
                    .map(str::to_string)
                    .ok_or_else(|| invalid_treemap_model(format!("{field} must contain strings")))
            })
            .collect::<Result<Vec<_>>>()
            .map(Some),
        Some(_) => Err(invalid_treemap_model(format!("{field} must be an array"))),
    }
}

fn optional_value_field(value: &Value, field: &str) -> Option<Value> {
    value
        .get(field)
        .filter(|v| !v.is_null())
        .map(crate::json::clone_value_nonrecursive)
}

fn invalid_treemap_model(message: impl Into<String>) -> Error {
    Error::InvalidModel {
        message: format!("invalid treemap semantic model: {}", message.into()),
    }
}

pub fn layout_treemap_diagram_typed(
    model: &TreemapDiagramRenderModel,
    effective_config: &Value,
    _measurer: &dyn crate::text::TextMeasurer,
) -> Result<TreemapDiagramLayout> {
    let cfg = treemap_config(effective_config);

    let title_height = if model.title.as_deref().is_some_and(|t| !t.trim().is_empty()) {
        30.0
    } else {
        0.0
    };

    let width = if cfg.node_width > 0.0 {
        cfg.node_width * TREEMAP_SECTION_INNER_PADDING_PX
    } else {
        960.0
    };
    let height = if cfg.node_height > 0.0 {
        cfg.node_height * TREEMAP_SECTION_INNER_PADDING_PX
    } else {
        500.0
    };

    let mut nodes: Vec<HierNode> = Vec::new();
    push_node(&mut nodes, &model.root, None, 0);
    if nodes.is_empty() {
        return Err(Error::InvalidModel {
            message: "treemap root produced no nodes".to_string(),
        });
    }
    let root_idx = 0usize;

    compute_sum(&mut nodes, root_idx);
    sort_children_by_value(&mut nodes, root_idx);

    nodes[root_idx].x0 = 0.0;
    nodes[root_idx].y0 = 0.0;
    nodes[root_idx].x1 = width;
    nodes[root_idx].y1 = height;

    let mut padding_stack = vec![0.0];
    for idx in each_before(&nodes, root_idx) {
        position_node(&mut nodes, idx, &mut padding_stack, cfg.padding.max(0.0));
    }

    for idx in each_before(&nodes, root_idx) {
        treemap_round_node(&mut nodes, idx);
    }

    let branch_nodes = descendants_bfs(&nodes, root_idx)
        .into_iter()
        .filter(|&idx| !nodes[idx].children.is_empty())
        .collect::<Vec<_>>();

    let leaf_nodes = leaves_each_before(&nodes, root_idx);

    let mut sections = Vec::new();
    for idx in &branch_nodes {
        let n = &nodes[*idx];
        sections.push(TreemapSectionLayout {
            name: n.name.clone(),
            depth: n.depth as i64,
            value: n.value,
            x0: n.x0,
            y0: n.y0,
            x1: n.x1,
            y1: n.y1,
            class_selector: n.class_selector.clone(),
            css_compiled_styles: n.css_compiled_styles.clone(),
        });
    }

    let mut leaves = Vec::new();
    for idx in &leaf_nodes {
        let n = &nodes[*idx];
        leaves.push(TreemapLeafLayout {
            name: n.name.clone(),
            value: n.value,
            parent_name: n.parent.map(|p| nodes[p].name.clone()),
            x0: n.x0,
            y0: n.y0,
            x1: n.x1,
            y1: n.y1,
            class_selector: n.class_selector.clone(),
            css_compiled_styles: n.css_compiled_styles.clone(),
        });
    }

    Ok(TreemapDiagramLayout {
        title_height,
        width,
        height,
        use_max_width: cfg.use_max_width,
        diagram_padding: cfg.diagram_padding.max(0.0),
        show_values: cfg.show_values,
        value_format: cfg.value_format,
        acc_title: model.acc_title.clone(),
        acc_descr: model.acc_descr.clone(),
        title: model.title.clone(),
        sections,
        leaves,
    })
}

#[cfg(test)]
mod tests {
    #[test]
    fn treemap_geometry_constants_match_mermaid() {
        assert_eq!(super::TREEMAP_SECTION_INNER_PADDING_PX, 10.0);
        assert_eq!(super::TREEMAP_SECTION_HEADER_HEIGHT_PX, 25.0);
    }
}
