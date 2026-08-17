//! Flowchart hierarchy helpers (clusters, LCA, edge selection).

use super::*;

pub(in crate::svg::parity) fn flowchart_is_strict_descendant(
    parent: &FxHashMap<&str, &str>,
    node_id: &str,
    cluster_id: &str,
) -> bool {
    if node_id == cluster_id {
        return false;
    }
    let mut cur: Option<&str> = Some(node_id);
    while let Some(id) = cur {
        if id == cluster_id {
            return true;
        }
        cur = parent.get(id).copied();
    }
    false
}

pub(in crate::svg::parity) fn flowchart_effective_parent<'a>(
    ctx: &'a FlowchartRenderCtx<'_>,
    id: &str,
) -> Option<&'a str> {
    let mut cur = ctx.parent.get(id).copied();
    while let Some(p) = cur {
        if ctx.subgraphs_by_id.contains_key(p) && !ctx.recursive_clusters.contains(p) {
            cur = ctx.parent.get(p).copied();
            continue;
        }
        return Some(p);
    }
    None
}

// Mermaid flowchart-v2 uses nested `.root` groups for extracted clusters. The `<g class="root">`
// is positioned by the cluster node transform, and its internal content starts at a fixed 8px
// margin (graph marginx/marginy in Mermaid's Dagre config).
pub(in crate::svg::parity) fn flowchart_cluster_root_offsets(
    ctx: &FlowchartRenderCtx<'_>,
    cid: &str,
) -> Option<FlowchartRootOffsets> {
    const ROOT_MARGIN_PX: f64 = 8.0;
    let cluster = ctx.layout_clusters_by_id.get(cid)?;

    let abs_left = if cluster.diff > 0.0 {
        (cluster.x + cluster.diff - cluster.width / 2.0) + ctx.tx
    } else {
        (cluster.x - cluster.width / 2.0) + ctx.tx - ROOT_MARGIN_PX
    };
    let title_total_margin = (cluster.title_margin_top + cluster.title_margin_bottom).max(0.0);
    let title_y_shift = title_total_margin / 2.0;

    let my_parent = flowchart_effective_parent(ctx, cid);
    let has_empty_sibling = ctx.subgraphs_by_id.iter().any(|(id, sg)| {
        *id != cid
            && sg.nodes.is_empty()
            && ctx.layout_clusters_by_id.contains_key(id)
            && flowchart_effective_parent(ctx, id) == my_parent
    });

    let base_top = (cluster.y - cluster.height / 2.0) + ctx.ty - ROOT_MARGIN_PX;
    let extra_transform_y = if has_empty_sibling {
        cluster.offset_y.max(0.0) * 2.0
    } else {
        0.0
    };

    let abs_top_transform = base_top + extra_transform_y;
    let abs_top_content = base_top + title_y_shift;

    Some(FlowchartRootOffsets {
        origin_x: abs_left,
        origin_y: abs_top_content,
        abs_top_transform,
    })
}

pub(super) fn flowchart_node_dom_indices<'a>(
    model: &'a crate::flowchart::FlowchartV2Model,
) -> FxHashMap<&'a str, usize> {
    if !model.vertex_calls.is_empty() {
        let mut out: FxHashMap<&'a str, usize> = FxHashMap::default();
        out.reserve(model.vertex_calls.len());
        for (vertex_counter, id) in model.vertex_calls.iter().enumerate() {
            let id: &'a str = id.as_str();
            let _ = out.entry(id).or_insert(vertex_counter);
        }
        return out;
    }

    let mut out: FxHashMap<&'a str, usize> = FxHashMap::default();
    out.reserve(model.edges.len().saturating_mul(2) + model.nodes.len());
    let mut vertex_counter: usize = 0;

    // Mermaid FlowDB assigns `domId` when a vertex is first created, but increments the internal
    // `vertexCounter` on every `addVertex(...)` call (even for repeated references). This means the
    // domId suffix depends on the full "first-use" order + repeat uses.
    fn touch<'a>(id: &'a str, out: &mut FxHashMap<&'a str, usize>, c: &mut usize) {
        let _ = out.entry(id).or_insert(*c);
        *c += 1;
    }

    for e in &model.edges {
        touch(e.from.as_str(), &mut out, &mut vertex_counter);
        touch(e.to.as_str(), &mut out, &mut vertex_counter);
    }

    for n in &model.nodes {
        touch(n.id.as_str(), &mut out, &mut vertex_counter);
    }

    out
}

pub(in crate::svg::parity) fn flowchart_root_children_clusters<'a>(
    ctx: &'a FlowchartRenderCtx<'a>,
    parent_cluster: Option<&str>,
) -> Vec<&'a str> {
    let mut out = Vec::new();
    for id in ctx.subgraphs_by_id.keys() {
        if !ctx.recursive_clusters.contains(id) {
            continue;
        }
        let parent = flowchart_effective_parent(ctx, id);
        if parent == parent_cluster {
            out.push(*id);
        }
    }
    out.sort_by(|a, b| {
        let a_idx = ctx.subgraph_order.iter().position(|id| id == a);
        let b_idx = ctx.subgraph_order.iter().position(|id| id == b);

        let aa = ctx.layout_clusters_by_id.get(a);
        let bb = ctx.layout_clusters_by_id.get(b);
        let (al, at) = aa
            .map(|c| (c.x - c.width / 2.0, c.y - c.height / 2.0))
            .unwrap_or((0.0, 0.0));
        let (bl, bt) = bb
            .map(|c| (c.x - c.width / 2.0, c.y - c.height / 2.0))
            .unwrap_or((0.0, 0.0));
        if let (Some(ai), Some(bi)) = (a_idx, b_idx) {
            // Mirror Mermaid's Dagre graph registration behavior: sibling cluster roots tend to
            // appear in reverse subgraph definition order.
            bi.cmp(&ai)
                .then_with(|| al.total_cmp(&bl))
                .then_with(|| at.total_cmp(&bt))
                .then_with(|| a.cmp(b))
        } else {
            al.total_cmp(&bl)
                .then_with(|| at.total_cmp(&bt))
                .then_with(|| a.cmp(b))
        }
    });
    out
}

pub(in crate::svg::parity) fn flowchart_root_children_nodes<'a>(
    ctx: &'a FlowchartRenderCtx<'a>,
    parent_cluster: Option<&str>,
) -> Vec<&'a str> {
    let cluster_ids: std::collections::HashSet<&str> = ctx
        .subgraphs_by_id
        .iter()
        .filter(|(_, sg)| !sg.nodes.is_empty())
        .map(|(k, _)| *k)
        .collect();
    let mut out = Vec::new();
    for (id, n) in &ctx.nodes_by_id {
        if cluster_ids.contains(id) {
            continue;
        }
        let parent = flowchart_effective_parent(ctx, id);
        if parent == parent_cluster {
            out.push(n.id.as_str());
        }
    }
    for (id, sg) in &ctx.subgraphs_by_id {
        if !sg.nodes.is_empty() {
            continue;
        }
        let parent = flowchart_effective_parent(ctx, id);
        if parent == parent_cluster {
            out.push(*id);
        }
    }

    let dom_order_idx: Option<std::collections::HashMap<&str, usize>> = ctx
        .dom_node_order_by_root
        .get(parent_cluster.unwrap_or(""))
        .map(|ids| {
            let mut m: std::collections::HashMap<&str, usize> = std::collections::HashMap::new();
            for (i, id) in ids.iter().enumerate() {
                m.insert(id.as_str(), i);
            }
            m
        });

    fn cluster_nesting_depth(
        ctx: &FlowchartRenderCtx<'_>,
        id: &str,
        parent_cluster: Option<&str>,
    ) -> usize {
        let mut depth: usize = 0;
        let mut cur = ctx.parent.get(id).copied();
        while let Some(p) = cur {
            let count = if parent_cluster.is_some() {
                // Within an extracted root, Mermaid's node insertion/DOM ordering is sensitive
                // to the full cluster nesting (including non-recursive clusters).
                ctx.subgraphs_by_id.contains_key(p)
            } else {
                // At the top-level root, only extracted clusters introduce additional nesting.
                ctx.recursive_clusters.contains(p)
            };
            if count {
                depth = depth.saturating_add(1);
            }
            cur = ctx.parent.get(p).copied();
        }
        depth
    }

    fn nearest_cluster_id<'a>(
        ctx: &'a FlowchartRenderCtx<'_>,
        id: &str,
        parent_cluster: Option<&str>,
    ) -> Option<&'a str> {
        let mut cur = ctx.parent.get(id).copied();
        while let Some(p) = cur {
            let keep = if parent_cluster.is_some() {
                ctx.subgraphs_by_id
                    .get(p)
                    .is_some_and(|sg| !sg.nodes.is_empty())
            } else {
                ctx.recursive_clusters.contains(p)
            };
            if keep {
                return Some(p);
            }
            cur = ctx.parent.get(p).copied();
        }
        None
    }

    fn dir_sort_key(primary_dir: &str, x: f64, y: f64) -> (f64, f64) {
        match primary_dir {
            "BT" => (-y, x),
            "LR" => (x, y),
            "RL" => (-x, y),
            _ => (y, x), // TB (default)
        }
    }

    out.sort_by(|a, b| {
        if let Some(ref dom) = dom_order_idx {
            let adi = dom.get(a).copied().unwrap_or(usize::MAX);
            let bdi = dom.get(b).copied().unwrap_or(usize::MAX);
            if adi != bdi {
                return adi.cmp(&bdi);
            }
        }

        let ai = ctx.node_dom_index.get(a).copied().unwrap_or(usize::MAX);
        let bi = ctx.node_dom_index.get(b).copied().unwrap_or(usize::MAX);

        let aa = ctx.layout_nodes_by_id.get(a);
        let bb = ctx.layout_nodes_by_id.get(b);
        let (ax, ay) = aa.map(|n| (n.x, n.y)).unwrap_or((0.0, 0.0));
        let (bx, by) = bb.map(|n| (n.x, n.y)).unwrap_or((0.0, 0.0));
        let ad = cluster_nesting_depth(ctx, a, parent_cluster);
        let bd = cluster_nesting_depth(ctx, b, parent_cluster);
        bd.cmp(&ad)
            .then_with(|| {
                if ad == 0 && bd == 0 {
                    // For nodes not nested in any subgraph, upstream Mermaid keeps the graph
                    // insertion order as the primary key, then uses position to stabilize ties.
                    ai.cmp(&bi)
                        .then_with(|| ay.total_cmp(&by))
                        .then_with(|| ax.total_cmp(&bx))
                } else {
                    // For nodes that are nested in subgraphs, upstream Mermaid's DOM ordering is
                    // closer to “flow direction” ordering within the nearest cluster.
                    let ag = nearest_cluster_id(ctx, a, parent_cluster);
                    let bg = nearest_cluster_id(ctx, b, parent_cluster);
                    if ag == bg {
                        let dir = ag
                            .and_then(|id| ctx.layout_clusters_by_id.get(id))
                            .map(|c| c.effective_dir.as_str())
                            .unwrap_or("TB");
                        let (ap, as_) = dir_sort_key(dir, ax, ay);
                        let (bp, bs) = dir_sort_key(dir, bx, by);
                        ap.total_cmp(&bp)
                            .then_with(|| as_.total_cmp(&bs))
                            .then_with(|| ai.cmp(&bi))
                    } else {
                        // Different clusters at the same nesting depth: keep insertion order stable.
                        ai.cmp(&bi)
                            .then_with(|| ay.total_cmp(&by))
                            .then_with(|| ax.total_cmp(&bx))
                    }
                }
            })
            .then_with(|| a.cmp(b))
    });
    out
}

pub(in crate::svg::parity) fn flowchart_lca(
    ctx: &FlowchartRenderCtx<'_>,
    a: &str,
    b: &str,
) -> Option<String> {
    let mut ancestors: std::collections::HashSet<String> = std::collections::HashSet::new();
    let mut cur = flowchart_effective_parent(ctx, a).map(|s| s.to_string());
    while let Some(p) = cur {
        ancestors.insert(p.clone());
        cur = flowchart_effective_parent(ctx, &p).map(|s| s.to_string());
    }

    let mut cur = flowchart_effective_parent(ctx, b).map(|s| s.to_string());
    while let Some(p) = cur {
        if ancestors.contains(&p) {
            return Some(p);
        }
        cur = flowchart_effective_parent(ctx, &p).map(|s| s.to_string());
    }
    None
}

pub(in crate::svg::parity) fn flowchart_edges_for_root<'a>(
    ctx: &'a FlowchartRenderCtx<'a>,
    cluster_id: Option<&str>,
) -> Vec<&'a crate::flowchart::FlowEdge> {
    let mut out = Vec::new();
    for edge_id in &ctx.edge_order {
        let Some(&e) = ctx.edges_by_id.get(edge_id) else {
            continue;
        };
        let lca = flowchart_lca(ctx, e.from.as_str(), e.to.as_str());
        if lca.as_deref() == cluster_id {
            out.push(e);
        }
    }
    out
}
