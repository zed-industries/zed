//! Dagre-style layout pipeline.
//!
//! This pipeline follows upstream Dagre's structure more closely (ranking, normalization,
//! ordering, BK positioning, translation).

use crate::graphlib;
use crate::{
    EdgeLabel, GraphLabel, LabelPos, NodeLabel, Point, RankDir, acyclic, add_border_segments,
    coordinate_system, nesting_graph, normalize, order, parent_dummy_chains, position, rank,
    self_edges, util,
};

pub fn layout_dagreish(g: &mut graphlib::Graph<NodeLabel, EdgeLabel, GraphLabel>) {
    let timing_enabled = std::env::var("DUGONG_DAGREISH_TIMING")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(false);

    #[derive(Debug, Default, Clone)]
    struct DagreishTimings {
        total: web_time::Duration,
        preprocess: web_time::Duration,
        self_edges_remove: web_time::Duration,
        acyclic: web_time::Duration,
        nesting_run: web_time::Duration,
        rank: web_time::Duration,
        edge_label_proxies: web_time::Duration,
        assign_rank_min_max: web_time::Duration,
        normalize_run: web_time::Duration,
        compound_border: web_time::Duration,
        order: web_time::Duration,
        coord_adjust: web_time::Duration,
        self_edges_insert: web_time::Duration,
        layering_y: web_time::Duration,
        position_x: web_time::Duration,
        self_edges_position: web_time::Duration,
        remove_border_nodes: web_time::Duration,
        normalize_undo: web_time::Duration,
        translate: web_time::Duration,
        edge_points: web_time::Duration,
        acyclic_undo: web_time::Duration,
    }

    let total_start = timing_enabled.then(web_time::Instant::now);
    let mut timings = DagreishTimings::default();

    // Mirror Dagre's `makeSpaceForEdgeLabels` so edge-label proxy ranks become integers
    // (we later materialize label nodes in `normalize::run`).
    let preprocess_start = timing_enabled.then(web_time::Instant::now);
    g.graph_mut().ranksep /= 2.0;
    let rankdir = g.graph().rankdir;
    g.for_each_edge_mut(|_ek, e| {
        e.minlen = e.minlen.max(1).saturating_mul(2);
        if !matches!(e.labelpos, LabelPos::C) {
            match rankdir {
                RankDir::TB | RankDir::BT => e.width += e.labeloffset,
                RankDir::LR | RankDir::RL => e.height += e.labeloffset,
            }
        }
    });
    if let Some(s) = preprocess_start {
        timings.preprocess = s.elapsed();
    }

    // Dagre removes self-loops before ranking/normalization and re-inserts them during positioning
    // via dummy "selfedge" nodes. This avoids invalid rank constraints and gives self-loops a
    // deterministic, spacing-aware offset in BK positioning.
    let self_edges_remove_start = timing_enabled.then(web_time::Instant::now);
    self_edges::remove_self_edges(g);
    if let Some(s) = self_edges_remove_start {
        timings.self_edges_remove = s.elapsed();
    }

    let mut has_compound_structure = false;
    g.for_each_node(|id, _n| {
        if g.parent(id).is_some() || g.children_iter(id).next().is_some() {
            has_compound_structure = true;
        }
    });
    let tiny_simple_chain = g.node_count() == 2 && g.edge_count() == 1 && !has_compound_structure;
    let first_edge_pre = if tiny_simple_chain {
        g.edges().next().cloned()
    } else {
        None
    };

    let acyclic_start = timing_enabled.then(web_time::Instant::now);
    let ran_acyclic = if tiny_simple_chain || g.edge_count() <= 1 {
        false
    } else {
        acyclic::run(g);
        true
    };
    if let Some(s) = acyclic_start {
        timings.acyclic = s.elapsed();
    }

    // Mermaid's dagre adapter always enables `compound: true`, and Dagre's ranker expects a
    // connected graph. Nesting graph connects components (even if there are no explicit
    // subgraphs), preventing network-simplex from panicking on disconnected inputs.
    if g.options().compound {
        let nesting_start = timing_enabled.then(web_time::Instant::now);
        let ran_nesting = if tiny_simple_chain {
            false
        } else {
            nesting_graph::run(g);
            true
        };
        if let Some(s) = nesting_start {
            if ran_nesting {
                timings.nesting_run = s.elapsed();
            }
        }
    }

    // Match upstream Dagre: ranking runs on a non-compound view of the graph so cluster nodes
    // (nodes with children) do not participate in ranking / network-simplex connectivity.
    //
    // `nesting_graph::run` materializes border nodes and nesting edges; those border nodes are
    // leaf nodes and remain in the non-compound graph, providing the constraints Dagre expects.
    let rank_start = timing_enabled.then(web_time::Instant::now);
    if tiny_simple_chain {
        // For the smallest flowcharts (e.g. `A --> B`) network-simplex + ordering overhead
        // dominates total runtime. A deterministic direct rank assignment keeps behavior
        // identical for these cases while cutting fixed-cost work.
        g.for_each_node_mut(|_id, n| n.rank = Some(0));
        if let Some(ek) = first_edge_pre.clone() {
            let minlen = g
                .edge_by_key(&ek)
                .map(|e| e.minlen.max(1) as i32)
                .unwrap_or(1);
            if let Some(n) = g.node_mut(&ek.v) {
                n.rank = Some(0);
            }
            if let Some(n) = g.node_mut(&ek.w) {
                n.rank = Some(minlen);
            }
        }
    } else {
        let mut rank_graph = util::as_non_compound_graph(g);
        rank::rank(&mut rank_graph);
        // Mirror Dagre's JS behavior: `rank(asNonCompoundGraph(g))` mutates the same label objects
        // for leaf nodes, but does not propagate ranks to compound nodes (nodes with children).
        //
        // In Rust we don't share label objects between graphs, so we copy ranks explicitly for leaf
        // nodes only.
        for v in g.node_ids() {
            if g.children_iter(&v).next().is_some() {
                continue;
            }
            let Some(rank) = rank_graph.node(&v).and_then(|n| n.rank) else {
                continue;
            };
            if let Some(n) = g.node_mut(&v) {
                n.rank = Some(rank);
            }
        }
    }
    if let Some(s) = rank_start {
        timings.rank = s.elapsed();
    }

    // Mirror Dagre's `injectEdgeLabelProxies` / `removeEdgeLabelProxies` to compute label ranks.
    // These label ranks are used by `normalize::run` to materialize `edge-label` dummy nodes with
    // the correct width/height, letting BK positioning account for edge labels.
    let edge_proxy_start = timing_enabled.then(web_time::Instant::now);
    let mut edge_proxy_nodes: Vec<String> = Vec::new();
    // Only clone edge keys when a proxy is actually needed.
    let mut to_proxy: Vec<(graphlib::EdgeKey, i32)> = Vec::new();
    g.for_each_edge(|ek, edge| {
        if edge.width <= 0.0 || edge.height <= 0.0 {
            return;
        }
        let Some(v_rank) = g.node(&ek.v).and_then(|n| n.rank) else {
            return;
        };
        let Some(w_rank) = g.node(&ek.w).and_then(|n| n.rank) else {
            return;
        };
        let rank = (w_rank - v_rank) / 2 + v_rank;
        to_proxy.push((ek.clone(), rank));
    });

    for (ek, rank) in to_proxy {
        let id = util::unique_id("_ep");
        edge_proxy_nodes.push(id.clone());
        g.set_node(
            id,
            NodeLabel {
                rank: Some(rank),
                dummy: Some("edge-proxy".to_string()),
                edge_obj: Some(ek),
                ..Default::default()
            },
        );
    }

    util::remove_empty_ranks(g);

    // Match upstream Dagre: `nestingGraph.cleanup` must happen before ordering/positioning.
    if g.options().compound && !tiny_simple_chain {
        nesting_graph::cleanup(g);
    }

    util::normalize_ranks(g);

    // Remove edge label proxy nodes, storing their rank on the corresponding edge label.
    for v in std::mem::take(&mut edge_proxy_nodes) {
        let Some(node) = g.node(&v).cloned() else {
            let _ = g.remove_node(&v);
            continue;
        };
        if node.dummy.as_deref() != Some("edge-proxy") {
            let _ = g.remove_node(&v);
            continue;
        }
        let Some(edge_obj) = node.edge_obj else {
            let _ = g.remove_node(&v);
            continue;
        };
        if let Some(lbl) = g.edge_mut_by_key(&edge_obj) {
            lbl.label_rank = node.rank;
        }
        let _ = g.remove_node(&v);
    }

    // Defensive parity: if the caller-provided graph already contained edge-proxy nodes,
    // remove them as well to match the previous best-effort behavior.
    let mut leftovers: Vec<String> = Vec::new();
    g.for_each_node(|id, n| {
        if n.dummy.as_deref() == Some("edge-proxy") {
            leftovers.push(id.to_string());
        }
    });
    for v in leftovers {
        let Some(node) = g.node(&v).cloned() else {
            let _ = g.remove_node(&v);
            continue;
        };
        let Some(edge_obj) = node.edge_obj else {
            let _ = g.remove_node(&v);
            continue;
        };
        if let Some(lbl) = g.edge_mut_by_key(&edge_obj) {
            lbl.label_rank = node.rank;
        }
        let _ = g.remove_node(&v);
    }
    if let Some(s) = edge_proxy_start {
        timings.edge_label_proxies = s.elapsed();
    }

    // Dagre uses `assignRankMinMax` to annotate compound nodes with their rank span, derived from
    // the `nestingGraph` border top/bottom nodes. This rank span is later used by subgraph
    // ordering and border segment generation.
    if g.options().compound && !tiny_simple_chain {
        let span_start = timing_enabled.then(web_time::Instant::now);
        let node_ids = g.node_ids();
        for v in node_ids {
            let Some(node) = g.node(&v).cloned() else {
                continue;
            };
            let (Some(bt), Some(bb)) = (node.border_top.clone(), node.border_bottom.clone()) else {
                continue;
            };
            let (Some(min_rank), Some(max_rank)) = (
                g.node(&bt).and_then(|n| n.rank),
                g.node(&bb).and_then(|n| n.rank),
            ) else {
                continue;
            };
            if let Some(n) = g.node_mut(&v) {
                n.min_rank = Some(min_rank);
                n.max_rank = Some(max_rank);
            }
        }
        if let Some(s) = span_start {
            timings.assign_rank_min_max = s.elapsed();
        }
    }

    let normalize_run_start = timing_enabled.then(web_time::Instant::now);
    normalize::run(g);
    if let Some(s) = normalize_run_start {
        timings.normalize_run = s.elapsed();
    }
    if g.options().compound && !tiny_simple_chain {
        let border_start = timing_enabled.then(web_time::Instant::now);
        parent_dummy_chains::parent_dummy_chains(g);
        add_border_segments::add_border_segments(g);
        if let Some(s) = border_start {
            timings.compound_border = s.elapsed();
        }
    }
    let order_start = timing_enabled.then(web_time::Instant::now);
    if tiny_simple_chain {
        let mut nodes: Vec<(i32, String)> = Vec::with_capacity(g.node_count());
        g.for_each_node(|id, n| nodes.push((n.rank.unwrap_or(0), id.to_string())));
        nodes.sort_by(|a, b| (a.0, a.1.as_str()).cmp(&(b.0, b.1.as_str())));

        let mut cur_rank: Option<i32> = None;
        let mut next_order: usize = 0;
        for (rank, id) in nodes {
            if cur_rank != Some(rank) {
                cur_rank = Some(rank);
                next_order = 0;
            }
            if let Some(n) = g.node_mut(&id) {
                n.order = Some(next_order);
            }
            next_order += 1;
        }
    } else {
        order::order(
            g,
            order::OrderOptions {
                disable_optimal_order_heuristic: false,
            },
        );
    }
    if let Some(s) = order_start {
        timings.order = s.elapsed();
    }

    // Positioning runs in TB coordinates; `coordinate_system::adjust` maps LR/RL/BT into TB.
    let coord_adjust_start = timing_enabled.then(web_time::Instant::now);
    coordinate_system::adjust(g);
    if let Some(s) = coord_adjust_start {
        timings.coord_adjust = s.elapsed();
    }

    // Insert dummy self-edge nodes after ordering and rankdir transforms, so their sizes match the
    // active coordinate system (TB) and they can influence BK x-positioning.
    let self_edges_insert_start = timing_enabled.then(web_time::Instant::now);
    self_edges::insert_self_edges(g);
    if let Some(s) = self_edges_insert_start {
        timings.self_edges_insert = s.elapsed();
    }

    let rank_sep = g.graph().ranksep;
    let layering_start = timing_enabled.then(web_time::Instant::now);
    let layering = util::build_layer_matrix(g);

    let mut prev_y: f64 = 0.0;
    for (idx, layer) in layering.iter().enumerate() {
        let max_h = layer
            .iter()
            .filter_map(|id| g.node(id).map(|n| n.height))
            .fold(0.0_f64, f64::max);
        let y = prev_y + max_h / 2.0;
        for id in layer {
            if let Some(n) = g.node_mut(id) {
                n.y = Some(y);
            }
        }
        prev_y += max_h;
        if idx + 1 < layering.len() {
            prev_y += rank_sep;
        }
    }
    if let Some(s) = layering_start {
        timings.layering_y = s.elapsed();
    }

    let position_x_start = timing_enabled.then(web_time::Instant::now);
    let xs = position::bk::position_x_with_layering(g, &layering);
    g.for_each_node_mut(|id, n| {
        n.x = Some(xs.get(id).copied().unwrap_or(0.0));
    });
    if let Some(s) = position_x_start {
        timings.position_x = s.elapsed();
    }

    // Convert dummy self-edge nodes into self-loop edge point sequences and remove the dummy nodes.
    let self_edges_position_start = timing_enabled.then(web_time::Instant::now);
    self_edges::position_self_edges(g);
    if let Some(s) = self_edges_position_start {
        timings.self_edges_position = s.elapsed();
    }

    // Match upstream Dagre: `removeBorderNodes` runs after positioning and before `normalize.undo`.
    // It sets compound-node geometry (x/y/width/height) from border nodes, then removes all
    // border dummy nodes.
    if g.options().compound && !tiny_simple_chain {
        let remove_border_start = timing_enabled.then(web_time::Instant::now);
        super::compound::remove_border_nodes(g);
        if let Some(s) = remove_border_start {
            timings.remove_border_nodes = s.elapsed();
        }
    }

    let normalize_undo_start = timing_enabled.then(web_time::Instant::now);
    normalize::undo(g);
    coordinate_system::undo(g);
    if let Some(s) = normalize_undo_start {
        timings.normalize_undo = s.elapsed();
    }

    // Translate so the minimum top-left is at (marginx, marginy), matching Dagre's
    // `translateGraph(...)` behavior.
    let translate_start = timing_enabled.then(web_time::Instant::now);
    let mut min_x: f64 = f64::INFINITY;
    let mut min_y: f64 = f64::INFINITY;
    let mut max_x: f64 = f64::NEG_INFINITY;
    let mut max_y: f64 = f64::NEG_INFINITY;
    g.for_each_node(|_id, n| {
        let (Some(x), Some(y)) = (n.x, n.y) else {
            return;
        };
        min_x = min_x.min(x - n.width / 2.0);
        min_y = min_y.min(y - n.height / 2.0);
        max_x = max_x.max(x + n.width / 2.0);
        max_y = max_y.max(y + n.height / 2.0);
    });
    g.for_each_edge(|_ek, lbl| {
        // Match Dagre's `translateGraph(...)`: it computes min/max based on nodes and edge-label
        // boxes, but does not include intermediate edge points. This can leave some internal spline
        // control points with negative coordinates (which Mermaid preserves in `data-points`), while
        // the rendered path remains within the viewBox because `curveBasis` does not pass through
        // those interior points.
        if let (Some(x), Some(y)) = (lbl.x, lbl.y) {
            min_x = min_x.min(x - lbl.width / 2.0);
            min_y = min_y.min(y - lbl.height / 2.0);
            max_x = max_x.max(x + lbl.width / 2.0);
            max_y = max_y.max(y + lbl.height / 2.0);
        }
    });

    if min_x.is_finite() && min_y.is_finite() && max_x.is_finite() && max_y.is_finite() {
        // Dagre shifts the graph by `-(min - margin)` so the smallest x/y becomes `margin`.
        // This is observable in Mermaid flowchart-v2 SVG output where `diagramPadding: 0`
        // still yields a `viewBox` starting at x=8 (the Dagre margin).
        let margin_x = g.graph().marginx;
        let margin_y = g.graph().marginy;
        min_x -= margin_x;
        min_y -= margin_y;
        let graph_width = max_x - min_x + margin_x;
        let graph_height = max_y - min_y + margin_y;
        let dx = -min_x;
        let dy = -min_y;
        g.for_each_node_mut(|_id, n| {
            if let Some(x) = n.x {
                n.x = Some(x + dx);
            }
            if let Some(y) = n.y {
                n.y = Some(y + dy);
            }
        });
        g.for_each_edge_mut(|_ek, lbl| {
            for p in &mut lbl.points {
                p.x += dx;
                p.y += dy;
            }
            if let Some(x) = lbl.x {
                lbl.x = Some(x + dx);
            }
            if let Some(y) = lbl.y {
                lbl.y = Some(y + dy);
            }
        });
        g.graph_mut().width = graph_width;
        g.graph_mut().height = graph_height;
    }
    if let Some(s) = translate_start {
        timings.translate = s.elapsed();
    }

    // Ensure every edge has at least one internal point (so D3 `curveBasis` emits cubic beziers),
    // and add node intersection endpoints to better match Dagre/Mermaid edge point semantics.
    let edge_points_start = timing_enabled.then(web_time::Instant::now);
    let edge_keys: Vec<graphlib::EdgeKey> = g.edges().cloned().collect();
    for e in edge_keys {
        let Some((sx, sy, sw, sh)) = g
            .node(&e.v)
            .map(|n| (n.x.unwrap_or(0.0), n.y.unwrap_or(0.0), n.width, n.height))
        else {
            continue;
        };
        let Some((tx, ty, tw, th)) = g
            .node(&e.w)
            .map(|n| (n.x.unwrap_or(0.0), n.y.unwrap_or(0.0), n.width, n.height))
        else {
            continue;
        };

        let Some(lbl) = g.edge_mut(&e.v, &e.w, e.name.as_deref()) else {
            continue;
        };

        let mut internal: Vec<Point> = if lbl.points.is_empty() {
            vec![Point {
                x: (sx + tx) / 2.0,
                y: (sy + ty) / 2.0,
            }]
        } else {
            lbl.points.clone()
        };
        if internal.is_empty() {
            internal.push(Point {
                x: (sx + tx) / 2.0,
                y: (sy + ty) / 2.0,
            });
        }

        let Some(first) = internal.first().copied() else {
            continue;
        };
        let Some(last) = internal.last().copied() else {
            continue;
        };

        let mut pts: Vec<Point> = Vec::with_capacity(internal.len() + 2);

        pts.push(util::intersect_rect(
            util::Rect {
                x: sx,
                y: sy,
                width: sw,
                height: sh,
            },
            first,
        ));
        pts.extend(internal);
        pts.push(util::intersect_rect(
            util::Rect {
                x: tx,
                y: ty,
                width: tw,
                height: th,
            },
            last,
        ));

        lbl.points = pts;

        if (lbl.width > 0.0 || lbl.height > 0.0) && lbl.x.is_none() && lbl.y.is_none() {
            if let Some(mid) = lbl.points.get(lbl.points.len() / 2).copied() {
                let mut ex = mid.x;
                let ey = mid.y;
                match lbl.labelpos {
                    LabelPos::C => {}
                    LabelPos::L => ex -= lbl.labeloffset + lbl.width / 2.0,
                    LabelPos::R => ex += lbl.labeloffset + lbl.width / 2.0,
                }
                lbl.x = Some(ex);
                lbl.y = Some(ey);
            }
        }
    }
    if let Some(s) = edge_points_start {
        timings.edge_points = s.elapsed();
    }

    let acyclic_undo_start = timing_enabled.then(web_time::Instant::now);
    if ran_acyclic {
        acyclic::undo(g);
    }
    if let Some(s) = acyclic_undo_start {
        timings.acyclic_undo = s.elapsed();
    }

    if let Some(s) = total_start {
        timings.total = s.elapsed();
        eprintln!(
            "[dugong-timing] pipeline=dagreish nodes={} edges={} total={:?} preprocess={:?} self_edges_remove={:?} acyclic={:?} nesting_run={:?} rank={:?} edge_label_proxies={:?} assign_rank_min_max={:?} normalize_run={:?} compound_border={:?} order={:?} coord_adjust={:?} self_edges_insert={:?} layering_y={:?} position_x={:?} self_edges_position={:?} remove_border_nodes={:?} normalize_undo={:?} translate={:?} edge_points={:?} acyclic_undo={:?}",
            g.node_count(),
            g.edge_count(),
            timings.total,
            timings.preprocess,
            timings.self_edges_remove,
            timings.acyclic,
            timings.nesting_run,
            timings.rank,
            timings.edge_label_proxies,
            timings.assign_rank_min_max,
            timings.normalize_run,
            timings.compound_border,
            timings.order,
            timings.coord_adjust,
            timings.self_edges_insert,
            timings.layering_y,
            timings.position_x,
            timings.self_edges_position,
            timings.remove_border_nodes,
            timings.normalize_undo,
            timings.translate,
            timings.edge_points,
            timings.acyclic_undo,
        );
    }
}
