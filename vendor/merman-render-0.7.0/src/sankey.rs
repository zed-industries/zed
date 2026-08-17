use crate::config::{config_bool, config_f64, config_string};
use crate::json::from_value_ref;
use crate::model::{Bounds, SankeyDiagramLayout, SankeyLinkLayout, SankeyNodeLayout};
use crate::text::TextMeasurer;
use crate::{Error, Result};
use merman_core::diagrams::sankey::SankeyDiagramRenderModel;
use serde_json::Value;
use std::cmp::Ordering;
use std::collections::HashMap;

const SANKEY_NODE_WIDTH_PX: f64 = 10.0;
const SANKEY_NODE_PADDING_BASE_PX: f64 = 12.0;
const SANKEY_NODE_PADDING_SHOW_VALUES_EXTRA_PX: f64 = 15.0;

#[derive(Debug, Clone)]
struct Node {
    id: String,
    index: usize,
    source_links: Vec<usize>,
    target_links: Vec<usize>,
    value: f64,
    depth: usize,
    height: usize,
    layer: usize,
    x0: f64,
    x1: f64,
    y0: f64,
    y1: f64,
}

#[derive(Debug, Clone)]
struct Link {
    index: usize,
    source: usize,
    target: usize,
    value: f64,
    width: f64,
    y0: f64,
    y1: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum NodeAlign {
    Left,
    Right,
    Justify,
    Center,
}

fn has_ref_object(v: &Value) -> bool {
    v.as_object().is_some_and(|m| m.contains_key("$ref"))
}

fn parse_align(cfg: &Value) -> NodeAlign {
    match config_string(cfg, &["sankey", "nodeAlignment"]).as_deref() {
        Some("left") => NodeAlign::Left,
        Some("right") => NodeAlign::Right,
        Some("center") => NodeAlign::Center,
        _ => NodeAlign::Justify,
    }
}

fn f64_cmp(a: f64, b: f64) -> Ordering {
    a.partial_cmp(&b).unwrap_or(Ordering::Equal)
}

fn sankey_node_padding_px_with_base(base: f64, show_values: bool) -> f64 {
    base + if show_values {
        SANKEY_NODE_PADDING_SHOW_VALUES_EXTRA_PX
    } else {
        0.0
    }
}

pub fn layout_sankey_diagram(
    semantic: &Value,
    effective_config: &Value,
    _text_measurer: &dyn TextMeasurer,
) -> Result<SankeyDiagramLayout> {
    let model: SankeyDiagramRenderModel = from_value_ref(semantic)?;
    layout_sankey_diagram_typed(&model, effective_config, _text_measurer)
}

pub fn layout_sankey_diagram_typed(
    model: &SankeyDiagramRenderModel,
    effective_config: &Value,
    _text_measurer: &dyn TextMeasurer,
) -> Result<SankeyDiagramLayout> {
    let width = config_f64(effective_config, &["sankey", "width"]).unwrap_or(600.0);
    let height = config_f64(effective_config, &["sankey", "height"]).unwrap_or(400.0);

    let sankey_cfg = effective_config.get("sankey");
    let sankey_cfg_missing = sankey_cfg.is_none() || sankey_cfg.is_some_and(has_ref_object);
    let show_values = if sankey_cfg_missing {
        true
    } else {
        config_bool(effective_config, &["sankey", "showValues"]).unwrap_or(true)
    };
    let align = parse_align(effective_config);

    let dx = if sankey_cfg_missing {
        SANKEY_NODE_WIDTH_PX
    } else {
        config_f64(effective_config, &["sankey", "nodeWidth"]).unwrap_or(SANKEY_NODE_WIDTH_PX)
    };
    let node_padding_base = if sankey_cfg_missing {
        SANKEY_NODE_PADDING_BASE_PX
    } else {
        config_f64(effective_config, &["sankey", "nodePadding"])
            .unwrap_or(SANKEY_NODE_PADDING_BASE_PX)
    };
    let dy = sankey_node_padding_px_with_base(node_padding_base, show_values);
    let iterations = 6usize;

    let mut nodes: Vec<Node> = model
        .graph
        .nodes
        .iter()
        .enumerate()
        .map(|(i, n)| Node {
            id: n.id.clone(),
            index: i,
            source_links: Vec::new(),
            target_links: Vec::new(),
            value: 0.0,
            depth: 0,
            height: 0,
            layer: 0,
            x0: 0.0,
            x1: 0.0,
            y0: 0.0,
            y1: 0.0,
        })
        .collect();

    let mut node_by_id: HashMap<String, usize> = HashMap::new();
    for (i, n) in model.graph.nodes.iter().enumerate() {
        node_by_id.insert(n.id.clone(), i);
    }

    let mut links: Vec<Link> = Vec::with_capacity(model.graph.links.len());
    for (i, l) in model.graph.links.iter().enumerate() {
        let source = node_by_id
            .get(&l.source)
            .copied()
            .ok_or_else(|| Error::InvalidModel {
                message: format!("missing node id {}", l.source),
            })?;
        let target = node_by_id
            .get(&l.target)
            .copied()
            .ok_or_else(|| Error::InvalidModel {
                message: format!("missing node id {}", l.target),
            })?;

        let value = l.value.as_f64().unwrap_or(0.0);
        links.push(Link {
            index: i,
            source,
            target,
            value,
            width: 0.0,
            y0: 0.0,
            y1: 0.0,
        });

        nodes[source].source_links.push(i);
        nodes[target].target_links.push(i);
    }

    for n in &mut nodes {
        let out_sum: f64 = n.source_links.iter().map(|&li| links[li].value).sum();
        let in_sum: f64 = n.target_links.iter().map(|&li| links[li].value).sum();
        n.value = out_sum.max(in_sum);
    }

    fn compute_node_depths(nodes: &mut [Node], links: &[Link]) -> Result<()> {
        let n = nodes.len();
        let mut current: Vec<usize> = (0..n).collect();
        let mut next: Vec<usize> = Vec::new();
        let mut next_seen = vec![false; n];
        let mut x: usize = 0;
        while !current.is_empty() {
            for &node_idx in &current {
                nodes[node_idx].depth = x;
                for &li in &nodes[node_idx].source_links {
                    let t = links[li].target;
                    if !next_seen[t] {
                        next_seen[t] = true;
                        next.push(t);
                    }
                }
            }
            x += 1;
            if x > n {
                return Err(Error::InvalidModel {
                    message: "circular link".to_string(),
                });
            }
            current = next;
            next = Vec::new();
            next_seen.fill(false);
        }
        Ok(())
    }

    fn compute_node_heights(nodes: &mut [Node], links: &[Link]) -> Result<()> {
        let n = nodes.len();
        let mut current: Vec<usize> = (0..n).collect();
        let mut next: Vec<usize> = Vec::new();
        let mut next_seen = vec![false; n];
        let mut x: usize = 0;
        while !current.is_empty() {
            for &node_idx in &current {
                nodes[node_idx].height = x;
                for &li in &nodes[node_idx].target_links {
                    let s = links[li].source;
                    if !next_seen[s] {
                        next_seen[s] = true;
                        next.push(s);
                    }
                }
            }
            x += 1;
            if x > n {
                return Err(Error::InvalidModel {
                    message: "circular link".to_string(),
                });
            }
            current = next;
            next = Vec::new();
            next_seen.fill(false);
        }
        Ok(())
    }

    compute_node_depths(&mut nodes, &links)?;
    compute_node_heights(&mut nodes, &links)?;

    let max_depth = nodes.iter().map(|n| n.depth).max().unwrap_or(0);
    let column_count = max_depth + 1;
    let kx = if column_count <= 1 {
        0.0
    } else {
        (width - dx) / (column_count as f64 - 1.0)
    };

    let mut columns: Vec<Vec<usize>> = vec![Vec::new(); column_count.max(1)];
    for i in 0..nodes.len() {
        let x = column_count.max(1);
        let raw_layer = match align {
            NodeAlign::Left => nodes[i].depth as i64,
            NodeAlign::Right => x as i64 - 1 - nodes[i].height as i64,
            NodeAlign::Justify => {
                if nodes[i].source_links.is_empty() {
                    x as i64 - 1
                } else {
                    nodes[i].depth as i64
                }
            }
            NodeAlign::Center => {
                if !nodes[i].target_links.is_empty() {
                    nodes[i].depth as i64
                } else if !nodes[i].source_links.is_empty() {
                    let min_target_depth = nodes[i]
                        .source_links
                        .iter()
                        .map(|&li| nodes[links[li].target].depth)
                        .min()
                        .unwrap_or(0);
                    min_target_depth as i64 - 1
                } else {
                    0
                }
            }
        };
        let layer = raw_layer.clamp(0, x as i64 - 1) as usize;
        nodes[i].layer = layer;
        nodes[i].x0 = layer as f64 * kx;
        nodes[i].x1 = nodes[i].x0 + dx;
        columns[layer].push(i);
    }

    let max_len = columns.iter().map(|c| c.len()).max().unwrap_or(0);
    let py = if max_len <= 1 {
        dy
    } else {
        dy.min(height / (max_len as f64 - 1.0))
    };

    let mut ky = f64::INFINITY;
    for col in &columns {
        if col.is_empty() {
            continue;
        }
        let sum_values: f64 = col.iter().map(|&ni| nodes[ni].value).sum();
        if sum_values <= 0.0 {
            continue;
        }
        let denom = height - (col.len() as f64 - 1.0) * py;
        ky = ky.min(denom / sum_values);
    }
    if !ky.is_finite() {
        ky = 0.0;
    }

    fn sort_source_links_by_target_y0(node_y0: &[f64], links: &[Link], link_indices: &mut [usize]) {
        link_indices.sort_by(|&a, &b| {
            let ta = node_y0[links[a].target];
            let tb = node_y0[links[b].target];
            f64_cmp(ta, tb).then_with(|| links[a].index.cmp(&links[b].index))
        });
    }

    fn sort_target_links_by_source_y0(node_y0: &[f64], links: &[Link], link_indices: &mut [usize]) {
        link_indices.sort_by(|&a, &b| {
            let sa = node_y0[links[a].source];
            let sb = node_y0[links[b].source];
            f64_cmp(sa, sb).then_with(|| links[a].index.cmp(&links[b].index))
        });
    }

    fn reorder_links(nodes: &mut [Node], links: &[Link], column: &[usize]) {
        let node_y0 = nodes.iter().map(|n| n.y0).collect::<Vec<_>>();
        for &ni in column {
            sort_source_links_by_target_y0(&node_y0, links, &mut nodes[ni].source_links);
            sort_target_links_by_source_y0(&node_y0, links, &mut nodes[ni].target_links);
        }
    }

    for col in &columns {
        let mut y = 0.0;
        for &ni in col {
            nodes[ni].y0 = y;
            nodes[ni].y1 = y + nodes[ni].value * ky;
            y = nodes[ni].y1 + py;
            for &li in &nodes[ni].source_links {
                links[li].width = links[li].value * ky;
            }
        }
        let n = col.len();
        if n > 0 {
            let offset = (height - y + py) / (n as f64 + 1.0);
            for (i, &ni) in col.iter().enumerate() {
                let adj = offset * (i as f64 + 1.0);
                nodes[ni].y0 += adj;
                nodes[ni].y1 += adj;
            }
            reorder_links(&mut nodes, &links, col);
        }
    }

    fn target_top(nodes: &[Node], links: &[Link], py: f64, source: usize, target: usize) -> f64 {
        let source_link_count = nodes[source].source_links.len() as f64;
        let mut y = nodes[source].y0 - (source_link_count - 1.0) * py / 2.0;
        for &li in &nodes[source].source_links {
            let node = links[li].target;
            if node == target {
                break;
            }
            y += links[li].width + py;
        }
        for &li in &nodes[target].target_links {
            let node = links[li].source;
            if node == source {
                break;
            }
            y -= links[li].width;
        }
        y
    }

    fn source_top(nodes: &[Node], links: &[Link], py: f64, source: usize, target: usize) -> f64 {
        let target_link_count = nodes[target].target_links.len() as f64;
        let mut y = nodes[target].y0 - (target_link_count - 1.0) * py / 2.0;
        for &li in &nodes[target].target_links {
            let node = links[li].source;
            if node == source {
                break;
            }
            y += links[li].width + py;
        }
        for &li in &nodes[source].source_links {
            let node = links[li].target;
            if node == target {
                break;
            }
            y -= links[li].width;
        }
        y
    }

    fn reorder_node_links(nodes: &mut [Node], links: &[Link], node_idx: usize) {
        let node_y0 = nodes.iter().map(|n| n.y0).collect::<Vec<_>>();

        let target_links = nodes[node_idx].target_links.clone();
        for li in target_links {
            let source = links[li].source;
            sort_source_links_by_target_y0(&node_y0, links, &mut nodes[source].source_links);
        }

        let source_links = nodes[node_idx].source_links.clone();
        for li in source_links {
            let target = links[li].target;
            sort_target_links_by_source_y0(&node_y0, links, &mut nodes[target].target_links);
        }
    }

    fn resolve_collisions_top_to_bottom(
        nodes: &mut [Node],
        column: &[usize],
        py: f64,
        mut y: f64,
        mut i: isize,
        alpha: f64,
    ) {
        while i < column.len() as isize {
            let ni = column[i as usize];
            let dy = (y - nodes[ni].y0) * alpha;
            if dy > 1e-6 {
                nodes[ni].y0 += dy;
                nodes[ni].y1 += dy;
            }
            y = nodes[ni].y1 + py;
            i += 1;
        }
    }

    fn resolve_collisions_bottom_to_top(
        nodes: &mut [Node],
        column: &[usize],
        py: f64,
        mut y: f64,
        mut i: isize,
        alpha: f64,
    ) {
        while i >= 0 {
            let ni = column[i as usize];
            let dy = (nodes[ni].y1 - y) * alpha;
            if dy > 1e-6 {
                nodes[ni].y0 -= dy;
                nodes[ni].y1 -= dy;
            }
            y = nodes[ni].y0 - py;
            i -= 1;
        }
    }

    fn resolve_collisions(
        nodes: &mut [Node],
        column: &[usize],
        py: f64,
        y0_extent: f64,
        y1_extent: f64,
        alpha: f64,
    ) {
        if column.is_empty() {
            return;
        }
        let i = column.len() >> 1;
        let subject = column[i];
        resolve_collisions_bottom_to_top(
            nodes,
            column,
            py,
            nodes[subject].y0 - py,
            i as isize - 1,
            alpha,
        );
        resolve_collisions_top_to_bottom(
            nodes,
            column,
            py,
            nodes[subject].y1 + py,
            i as isize + 1,
            alpha,
        );
        resolve_collisions_bottom_to_top(
            nodes,
            column,
            py,
            y1_extent,
            column.len() as isize - 1,
            alpha,
        );
        resolve_collisions_top_to_bottom(nodes, column, py, y0_extent, 0, alpha);
    }

    #[derive(Debug, Clone, Copy)]
    struct RelaxParams {
        py: f64,
        alpha: f64,
        beta: f64,
        y0_extent: f64,
        y1_extent: f64,
    }

    fn relax_left_to_right(
        nodes: &mut [Node],
        links: &[Link],
        columns: &mut [Vec<usize>],
        params: RelaxParams,
    ) {
        for column in columns.iter_mut().skip(1) {
            for &target in column.iter() {
                let mut y = 0.0;
                let mut w = 0.0;
                for &li in &nodes[target].target_links {
                    let source = links[li].source;
                    let value = links[li].value;
                    let v = value * (nodes[target].layer as f64 - nodes[source].layer as f64);
                    y += target_top(nodes, links, params.py, source, target) * v;
                    w += v;
                }
                if w <= 0.0 {
                    continue;
                }
                let dy = (y / w - nodes[target].y0) * params.alpha;
                nodes[target].y0 += dy;
                nodes[target].y1 += dy;
                reorder_node_links(nodes, links, target);
            }
            column.sort_by(|&a, &b| f64_cmp(nodes[a].y0, nodes[b].y0).then_with(|| a.cmp(&b)));
            resolve_collisions(
                nodes,
                column,
                params.py,
                params.y0_extent,
                params.y1_extent,
                params.beta,
            );
        }
    }

    fn relax_right_to_left(
        nodes: &mut [Node],
        links: &[Link],
        columns: &mut [Vec<usize>],
        params: RelaxParams,
    ) {
        if columns.len() < 2 {
            return;
        }
        for i in (0..=(columns.len() - 2)).rev() {
            let column = &mut columns[i];
            for &source in column.iter() {
                let mut y = 0.0;
                let mut w = 0.0;
                for &li in &nodes[source].source_links {
                    let target = links[li].target;
                    let value = links[li].value;
                    let v = value * (nodes[target].layer as f64 - nodes[source].layer as f64);
                    y += source_top(nodes, links, params.py, source, target) * v;
                    w += v;
                }
                if w <= 0.0 {
                    continue;
                }
                let dy = (y / w - nodes[source].y0) * params.alpha;
                nodes[source].y0 += dy;
                nodes[source].y1 += dy;
                reorder_node_links(nodes, links, source);
            }
            column.sort_by(|&a, &b| f64_cmp(nodes[a].y0, nodes[b].y0).then_with(|| a.cmp(&b)));
            resolve_collisions(
                nodes,
                column,
                params.py,
                params.y0_extent,
                params.y1_extent,
                params.beta,
            );
        }
    }

    let mut columns_for_relax = columns.clone();
    for i in 0..iterations {
        let alpha = 0.99_f64.powi(i as i32);
        let beta = (1.0 - alpha).max((i as f64 + 1.0) / iterations as f64);
        let params = RelaxParams {
            py,
            alpha,
            beta,
            y0_extent: 0.0,
            y1_extent: height,
        };
        relax_right_to_left(&mut nodes, &links, &mut columns_for_relax, params);
        relax_left_to_right(&mut nodes, &links, &mut columns_for_relax, params);
    }

    for node in &mut nodes {
        let mut y0 = node.y0;
        let mut y1 = node.y0;
        for &li in &node.source_links {
            links[li].y0 = y0 + links[li].width / 2.0;
            y0 += links[li].width;
        }
        for &li in &node.target_links {
            links[li].y1 = y1 + links[li].width / 2.0;
            y1 += links[li].width;
        }
    }

    let layout_nodes: Vec<SankeyNodeLayout> = nodes
        .iter()
        .map(|n| SankeyNodeLayout {
            id: n.id.clone(),
            index: n.index,
            depth: n.depth,
            height: n.height,
            layer: n.layer,
            value: n.value,
            x0: n.x0,
            x1: n.x1,
            y0: n.y0,
            y1: n.y1,
        })
        .collect();

    let layout_links: Vec<SankeyLinkLayout> = links
        .iter()
        .map(|l| SankeyLinkLayout {
            index: l.index,
            source: nodes[l.source].id.clone(),
            target: nodes[l.target].id.clone(),
            value: l.value,
            width: l.width,
            y0: l.y0,
            y1: l.y1,
        })
        .collect();

    Ok(SankeyDiagramLayout {
        bounds: Some(Bounds {
            min_x: 0.0,
            min_y: 0.0,
            max_x: width,
            max_y: height,
        }),
        width,
        height,
        node_width: dx,
        node_padding: py,
        nodes: layout_nodes,
        links: layout_links,
    })
}

#[cfg(test)]
mod tests {
    use super::layout_sankey_diagram;
    use crate::text::DeterministicTextMeasurer;
    use serde_json::json;

    #[test]
    fn sankey_node_geometry_constants_match_mermaid() {
        assert_eq!(super::SANKEY_NODE_WIDTH_PX, 10.0);
        assert_eq!(
            super::sankey_node_padding_px_with_base(super::SANKEY_NODE_PADDING_BASE_PX, true),
            27.0
        );
        assert_eq!(
            super::sankey_node_padding_px_with_base(super::SANKEY_NODE_PADDING_BASE_PX, false),
            12.0
        );
    }

    #[test]
    fn sankey_layout_uses_mermaid_node_geometry() {
        let semantic = json!({
            "graph": {
                "nodes": [{"id": "A"}, {"id": "B"}],
                "links": [{"source": "A", "target": "B", "value": 1.0}]
            }
        });
        let measurer = DeterministicTextMeasurer {
            char_width_factor: 8.0,
            line_height_factor: 16.0,
        };

        let default_layout = layout_sankey_diagram(&semantic, &json!({}), &measurer).unwrap();
        assert_eq!(default_layout.node_width, super::SANKEY_NODE_WIDTH_PX);
        assert_eq!(
            default_layout.node_padding,
            super::sankey_node_padding_px_with_base(super::SANKEY_NODE_PADDING_BASE_PX, true)
        );

        let hidden_values_layout = layout_sankey_diagram(
            &semantic,
            &json!({"sankey": {"showValues": false}}),
            &measurer,
        )
        .unwrap();
        assert_eq!(
            hidden_values_layout.node_padding,
            super::sankey_node_padding_px_with_base(super::SANKEY_NODE_PADDING_BASE_PX, false)
        );
    }

    #[test]
    fn sankey_layout_uses_configured_node_width_and_padding() {
        let semantic = json!({
            "graph": {
                "nodes": [{"id": "A"}, {"id": "B"}],
                "links": [{"source": "A", "target": "B", "value": 1.0}]
            }
        });
        let measurer = DeterministicTextMeasurer {
            char_width_factor: 8.0,
            line_height_factor: 16.0,
        };

        let layout = layout_sankey_diagram(
            &semantic,
            &json!({"sankey": {"nodeWidth": 24, "nodePadding": 18}}),
            &measurer,
        )
        .unwrap();

        assert_eq!(layout.node_width, 24.0);
        assert_eq!(layout.node_padding, 33.0);
        assert_eq!(layout.nodes[0].x1 - layout.nodes[0].x0, 24.0);

        let hidden_values_layout = layout_sankey_diagram(
            &semantic,
            &json!({"sankey": {"nodeWidth": 24, "nodePadding": 18, "showValues": false}}),
            &measurer,
        )
        .unwrap();
        assert_eq!(hidden_values_layout.node_padding, 18.0);
    }
}
