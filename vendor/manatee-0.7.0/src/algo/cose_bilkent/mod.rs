use crate::algo::CoseBilkentOptions;
use crate::error::Result;
use crate::graph::{Graph, LayoutResult, Point};
use rustc_hash::{FxHashMap as HashMap, FxHashSet as HashSet};
use std::collections::VecDeque;

#[derive(Debug, Clone, Copy)]
pub struct IndexedNode {
    pub width: f64,
    pub height: f64,
    pub x: f64,
    pub y: f64,
}

#[derive(Debug, Clone, Copy)]
pub struct IndexedEdge {
    pub a: usize,
    pub b: usize,
}

pub fn layout_indexed(
    nodes: &[IndexedNode],
    edges: &[IndexedEdge],
    _opts: &CoseBilkentOptions,
) -> Result<Vec<Point>> {
    if nodes.is_empty() {
        return Ok(Vec::new());
    }

    let timing_enabled = std::env::var("MANATEE_COSE_TIMING").ok().as_deref() == Some("1");
    #[derive(Debug, Default, Clone)]
    struct CoseLayoutTimings {
        total: web_time::Duration,
        from_indexed: web_time::Duration,
        flat_forest: web_time::Duration,
        radial: web_time::Duration,
        spring: web_time::Duration,
        transform: web_time::Duration,
        output: web_time::Duration,
    }
    let mut timings = CoseLayoutTimings::default();
    let total_start = timing_enabled.then(web_time::Instant::now);

    for (idx, e) in edges.iter().enumerate() {
        if e.a >= nodes.len() || e.b >= nodes.len() {
            return Err(crate::error::Error::MissingEndpoint {
                edge_id: format!("#{idx}"),
            });
        }
    }

    let from_indexed_start = timing_enabled.then(web_time::Instant::now);
    let mut sim = SimGraph::from_indexed(nodes, edges);
    if let Some(s) = from_indexed_start {
        timings.from_indexed = s.elapsed();
    }

    let flat_forest_start = timing_enabled.then(web_time::Instant::now);
    let forest = sim.get_flat_forest();
    if let Some(s) = flat_forest_start {
        timings.flat_forest = s.elapsed();
    }
    if !forest.is_empty() {
        let radial_start = timing_enabled.then(web_time::Instant::now);
        sim.position_nodes_radially(&forest);
        if let Some(s) = radial_start {
            timings.radial = s.elapsed();
        }
    }

    let spring_start = timing_enabled.then(web_time::Instant::now);
    if std::env::var("MANATEE_COSE_SKIP_SPRING").ok().as_deref() != Some("1") {
        sim.run_spring_embedder(timing_enabled);
    }
    if let Some(s) = spring_start {
        timings.spring = s.elapsed();
    }
    let transform_start = timing_enabled.then(web_time::Instant::now);
    if std::env::var("MANATEE_COSE_SKIP_TRANSFORM").ok().as_deref() != Some("1") {
        sim.transform_to_origin();
    }
    if let Some(s) = transform_start {
        timings.transform = s.elapsed();
    }

    let output_start = timing_enabled.then(web_time::Instant::now);
    let mut out: Vec<Point> = Vec::with_capacity(sim.nodes.len());
    for n in &sim.nodes {
        out.push(Point {
            x: n.center_x(),
            y: n.center_y(),
        });
    }
    if let Some(s) = output_start {
        timings.output = s.elapsed();
    }
    if let Some(s) = total_start {
        timings.total = s.elapsed();
        eprintln!(
            "[manatee-cose-timing] total={:?} from_indexed={:?} flat_forest={:?} radial={:?} spring={:?} transform={:?} output={:?} nodes={} edges={} components={}",
            timings.total,
            timings.from_indexed,
            timings.flat_forest,
            timings.radial,
            timings.spring,
            timings.transform,
            timings.output,
            sim.nodes.len(),
            sim.edges.len(),
            forest.len(),
        );
    }
    Ok(out)
}

pub fn layout(graph: &Graph, _opts: &CoseBilkentOptions) -> Result<LayoutResult> {
    graph.validate()?;

    let timing_enabled = std::env::var("MANATEE_COSE_TIMING").ok().as_deref() == Some("1");
    #[derive(Debug, Default, Clone)]
    struct CoseLayoutTimings {
        total: web_time::Duration,
        from_graph: web_time::Duration,
        flat_forest: web_time::Duration,
        radial: web_time::Duration,
        spring: web_time::Duration,
        transform: web_time::Duration,
        output: web_time::Duration,
    }
    let mut timings = CoseLayoutTimings::default();
    let total_start = timing_enabled.then(web_time::Instant::now);

    let from_graph_start = timing_enabled.then(web_time::Instant::now);
    let mut sim = SimGraph::from_graph(graph);
    if let Some(s) = from_graph_start {
        timings.from_graph = s.elapsed();
    }

    // COSE-Bilkent port for flat graphs (as used by Mermaid mindmap via Cytoscape).
    // This follows the upstream `cose-base` control flow:
    // - `getFlatForest()` + `positionNodesRadially(...)`
    // - `reduceTrees()` / `growTree()` scaffolding (currently disabled until parity is verified)
    // - spring embedder ticks
    // - `doPostLayout()` -> `transform(0,0)` to move the graph into positive coordinates
    let flat_forest_start = timing_enabled.then(web_time::Instant::now);
    let forest = sim.get_flat_forest();
    if let Some(s) = flat_forest_start {
        timings.flat_forest = s.elapsed();
    }
    if !forest.is_empty() {
        let radial_start = timing_enabled.then(web_time::Instant::now);
        sim.position_nodes_radially(&forest);
        if let Some(s) = radial_start {
            timings.radial = s.elapsed();
        }
    } else {
        // Fallback: keep all nodes at their provided initial positions (typically (0,0)).
        // The full port will use `scatter()` / `positionNodesRandomly()` for non-forest graphs.
    }
    let spring_start = timing_enabled.then(web_time::Instant::now);
    if std::env::var("MANATEE_COSE_SKIP_SPRING").ok().as_deref() != Some("1") {
        sim.run_spring_embedder(timing_enabled);
    }
    if let Some(s) = spring_start {
        timings.spring = s.elapsed();
    }
    let transform_start = timing_enabled.then(web_time::Instant::now);
    if std::env::var("MANATEE_COSE_SKIP_TRANSFORM").ok().as_deref() != Some("1") {
        sim.transform_to_origin();
    }
    if let Some(s) = transform_start {
        timings.transform = s.elapsed();
    }

    let output_start = timing_enabled.then(web_time::Instant::now);
    let node_count = sim.nodes.len();
    let edge_count = sim.edges.len();

    let mut positions: std::collections::BTreeMap<String, Point> =
        std::collections::BTreeMap::new();
    let nodes = std::mem::take(&mut sim.nodes);
    for n in nodes {
        let x = n.center_x();
        let y = n.center_y();
        positions.insert(n.id, Point { x, y });
    }
    if let Some(s) = output_start {
        timings.output = s.elapsed();
    }

    if let Some(s) = total_start {
        timings.total = s.elapsed();
        eprintln!(
            "[manatee-cose-timing] total={:?} from_graph={:?} flat_forest={:?} radial={:?} spring={:?} transform={:?} output={:?} nodes={} edges={} components={}",
            timings.total,
            timings.from_graph,
            timings.flat_forest,
            timings.radial,
            timings.spring,
            timings.transform,
            timings.output,
            node_count,
            edge_count,
            forest.len(),
        );
    }

    Ok(LayoutResult { positions })
}

#[derive(Debug, Clone)]
struct SimNode {
    id: String,
    width: f64,
    height: f64,
    half_width: f64,
    half_height: f64,
    // Top-left anchored rectangle, matching upstream `layout-base` `LNode.rect`.
    left: f64,
    top: f64,
    // Incident edge indices in insertion order, matching `LNode.edges` order.
    edges: Vec<usize>,
    // Cached repulsion candidates for the FR-grid variant (`FDLayoutNode.surrounding`).
    surrounding: Vec<usize>,
    active: bool,

    // FR-grid indices computed by `update_grid` for repulsion candidate lookup.
    start_x: i32,
    finish_x: i32,
    start_y: i32,
    finish_y: i32,

    // Forces (reset each iteration), matching `FDLayoutNode` / `CoSENode`.
    spring_fx: f64,
    spring_fy: f64,
    repulsion_fx: f64,
    repulsion_fy: f64,
    gravitation_fx: f64,
    gravitation_fy: f64,
}

impl SimNode {
    fn set_center(&mut self, cx: f64, cy: f64) {
        self.left = cx - self.half_width;
        self.top = cy - self.half_height;
    }

    fn center_x(&self) -> f64 {
        self.left + self.half_width
    }

    fn center_y(&self) -> f64 {
        self.top + self.half_height
    }

    fn diagonal(&self) -> f64 {
        (self.width * self.width + self.height * self.height).sqrt()
    }

    fn move_by(&mut self, dx: f64, dy: f64) {
        self.left += dx;
        self.top += dy;
    }

    fn half_w(&self) -> f64 {
        self.half_width
    }

    fn half_h(&self) -> f64 {
        self.half_height
    }

    fn right(&self) -> f64 {
        self.left + self.width
    }

    fn bottom(&self) -> f64 {
        self.top + self.height
    }
}

#[derive(Debug, Clone, Copy)]
struct SimEdge {
    a: usize,
    b: usize,
    active: bool,
}

#[derive(Debug, Clone, Copy)]
struct Bounds {
    min_x: f64,
    min_y: f64,
    max_x: f64,
    max_y: f64,
}

impl Bounds {
    fn from_nodes(nodes: &[SimNode], tree: &[usize]) -> Option<Self> {
        let mut min_x = f64::INFINITY;
        let mut min_y = f64::INFINITY;
        let mut max_x = f64::NEG_INFINITY;
        let mut max_y = f64::NEG_INFINITY;
        for &idx in tree {
            let n = &nodes[idx];
            min_x = min_x.min(n.left);
            min_y = min_y.min(n.top);
            max_x = max_x.max(n.left + n.width);
            max_y = max_y.max(n.top + n.height);
        }
        if !(min_x.is_finite() && min_y.is_finite() && max_x.is_finite() && max_y.is_finite()) {
            return None;
        }
        Some(Self {
            min_x,
            min_y,
            max_x,
            max_y,
        })
    }
}

#[derive(Debug, Default, Clone)]
struct SimGrid {
    size_x: usize,
    size_y: usize,
    cells: Vec<Vec<usize>>,
}

impl SimGrid {
    fn is_empty(&self) -> bool {
        self.size_x == 0 || self.size_y == 0
    }

    fn size_x(&self) -> usize {
        self.size_x
    }

    fn size_y(&self) -> usize {
        self.size_y
    }

    fn clear_cells(&mut self) {
        for cell in &mut self.cells {
            cell.clear();
        }
    }

    fn reset(&mut self, size_x: usize, size_y: usize, _left: f64, _top: f64, _range: f64) {
        if self.size_x != size_x || self.size_y != size_y {
            self.size_x = size_x;
            self.size_y = size_y;
            self.cells = vec![Vec::new(); size_x.saturating_mul(size_y)];
        } else {
            self.clear_cells();
        }
    }

    #[inline]
    fn idx(&self, x: usize, y: usize) -> usize {
        (x * self.size_y) + y
    }

    #[inline]
    fn push(&mut self, x: usize, y: usize, node_idx: usize) {
        let i = self.idx(x, y);
        self.cells[i].push(node_idx);
    }

    #[inline]
    fn cell(&self, x: usize, y: usize) -> &[usize] {
        let i = self.idx(x, y);
        self.cells[i].as_slice()
    }
}

#[derive(Debug)]
struct SimGraph {
    nodes: Vec<SimNode>,
    edges: Vec<SimEdge>,
    grid: SimGrid,
    repulsion_seen: Vec<u32>,
    repulsion_seen_gen: u32,
}

impl SimGraph {
    const DEFAULT_GRAPH_MARGIN: f64 = 15.0;
    const DEFAULT_COMPONENT_SEPERATION: f64 = 60.0; // upstream typo preserved
    const DEFAULT_EDGE_LENGTH: f64 = 50.0;
    const DEFAULT_RADIAL_SEPARATION: f64 = Self::DEFAULT_EDGE_LENGTH;

    // `layout-base` `LayoutConstants.WORLD_CENTER_X/Y`.
    const WORLD_CENTER_X: f64 = 1200.0;
    const WORLD_CENTER_Y: f64 = 900.0;

    const MAX_ITERATIONS: usize = 2500;
    const CONVERGENCE_CHECK_PERIOD: usize = 100;
    const MAX_NODE_DISPLACEMENT: f64 = 300.0;
    const MIN_REPULSION_DIST: f64 = Self::DEFAULT_EDGE_LENGTH / 10.0;
    const GRID_CALCULATION_CHECK_PERIOD: usize = 10; // `FDLayoutConstants.GRID_CALCULATION_CHECK_PERIOD`

    // cytoscape-cose-bilkent default options (Mermaid uses these in `cose-bilkent/cytoscape-setup.ts`).
    const DEFAULT_SPRING_STRENGTH: f64 = 0.45; // edgeElasticity
    const DEFAULT_REPULSION_STRENGTH: f64 = 4500.0; // nodeRepulsion
    const DEFAULT_GRAVITY_STRENGTH: f64 = 0.25; // gravity
    const DEFAULT_GRAVITY_RANGE_FACTOR: f64 = 3.8; // gravityRange

    #[inline]
    fn imath_sign(value: f64) -> f64 {
        // Port of `layout-base` `IMath.sign`: returns 0 for 0.
        if value > 0.0 {
            1.0
        } else if value < 0.0 {
            -1.0
        } else {
            0.0
        }
    }

    fn from_indexed(nodes_in: &[IndexedNode], edges_in: &[IndexedEdge]) -> Self {
        let mut nodes: Vec<SimNode> = Vec::with_capacity(nodes_in.len());
        for n in nodes_in {
            let w = n.width.max(1.0);
            let h = n.height.max(1.0);
            let hw = w * 0.5;
            let hh = h * 0.5;
            nodes.push(SimNode {
                id: String::new(),
                width: w,
                height: h,
                half_width: hw,
                half_height: hh,
                left: n.x - hw,
                top: n.y - hh,
                edges: Vec::new(),
                surrounding: Vec::new(),
                active: true,
                start_x: 0,
                finish_x: 0,
                start_y: 0,
                finish_y: 0,
                spring_fx: 0.0,
                spring_fy: 0.0,
                repulsion_fx: 0.0,
                repulsion_fy: 0.0,
                gravitation_fx: 0.0,
                gravitation_fy: 0.0,
            });
        }

        let mut seen_pairs: HashSet<(usize, usize)> =
            HashSet::with_capacity_and_hasher(edges_in.len(), Default::default());
        let mut edges: Vec<SimEdge> = Vec::with_capacity(edges_in.len());
        for e in edges_in {
            let (a, b) = (e.a, e.b);
            if a == b {
                continue;
            }
            if a >= nodes.len() || b >= nodes.len() {
                continue;
            }
            let (u, v) = if a < b { (a, b) } else { (b, a) };
            if !seen_pairs.insert((u, v)) {
                continue;
            }
            let ei = edges.len();
            edges.push(SimEdge { a, b, active: true });
            nodes[a].edges.push(ei);
            nodes[b].edges.push(ei);
        }

        Self {
            nodes,
            edges,
            grid: SimGrid::default(),
            repulsion_seen: vec![0u32; nodes_in.len()],
            repulsion_seen_gen: 1,
        }
    }

    fn from_graph(graph: &Graph) -> Self {
        let mut nodes: Vec<SimNode> = Vec::with_capacity(graph.nodes.len());
        for n in &graph.nodes {
            let w = n.width.max(1.0);
            let h = n.height.max(1.0);
            let hw = w * 0.5;
            let hh = h * 0.5;
            nodes.push(SimNode {
                id: n.id.clone(),
                width: w,
                height: h,
                half_width: hw,
                half_height: hh,
                left: n.x - hw,
                top: n.y - hh,
                edges: Vec::new(),
                surrounding: Vec::new(),
                active: true,
                start_x: 0,
                finish_x: 0,
                start_y: 0,
                finish_y: 0,
                spring_fx: 0.0,
                spring_fy: 0.0,
                repulsion_fx: 0.0,
                repulsion_fy: 0.0,
                gravitation_fx: 0.0,
                gravitation_fy: 0.0,
            });
        }

        let mut id_to_idx: HashMap<&str, usize> =
            HashMap::with_capacity_and_hasher(graph.nodes.len(), Default::default());
        for (idx, n) in graph.nodes.iter().enumerate() {
            id_to_idx.insert(n.id.as_str(), idx);
        }

        // Mirror the cytoscape-cose-bilkent behavior: only keep one edge between any two nodes.
        let mut seen_pairs: HashSet<(usize, usize)> =
            HashSet::with_capacity_and_hasher(graph.edges.len(), Default::default());
        let mut edges: Vec<SimEdge> = Vec::with_capacity(graph.edges.len());
        for e in &graph.edges {
            let Some(&a) = id_to_idx.get(e.source.as_str()) else {
                continue;
            };
            let Some(&b) = id_to_idx.get(e.target.as_str()) else {
                continue;
            };
            if a == b {
                continue;
            }
            let (u, v) = if a < b { (a, b) } else { (b, a) };
            if !seen_pairs.insert((u, v)) {
                continue;
            }
            let ei = edges.len();
            edges.push(SimEdge { a, b, active: true });
            nodes[a].edges.push(ei);
            nodes[b].edges.push(ei);
        }

        Self {
            nodes,
            edges,
            grid: SimGrid::default(),
            repulsion_seen: vec![0u32; graph.nodes.len()],
            repulsion_seen_gen: 1,
        }
    }

    fn edge_other_end(&self, edge_idx: usize, node_idx: usize) -> usize {
        let e = self.edges[edge_idx];
        if e.a == node_idx {
            e.b
        } else {
            debug_assert_eq!(e.b, node_idx);
            e.a
        }
    }

    fn for_each_active_neighbor(&self, node_idx: usize, mut f: impl FnMut(usize)) {
        for &ei in &self.nodes[node_idx].edges {
            if !self.edges[ei].active {
                continue;
            }
            let other = self.edge_other_end(ei, node_idx);
            if !self.nodes[other].active {
                continue;
            }
            f(other);
        }
    }

    fn active_edge_between(&self, a: usize, b: usize) -> Option<usize> {
        for &ei in &self.nodes[a].edges {
            if !self.edges[ei].active {
                continue;
            }
            if self.edge_other_end(ei, a) == b {
                return Some(ei);
            }
        }
        None
    }

    /// Port of `layout-base` `Layout.getFlatForest()` for flat graphs.
    fn get_flat_forest(&self) -> Vec<Vec<usize>> {
        let mut flat_forest: Vec<Vec<usize>> = Vec::new();
        let mut is_forest = true;

        // Root graph nodes in insertion order.
        let all_nodes: Vec<usize> = (0..self.nodes.len())
            .filter(|&idx| self.nodes[idx].active)
            .collect();

        // Graph is always flat in our current model (no compound nodes).

        // BFS for each component; reject if any component is not a tree.
        let mut to_be_visited: VecDeque<usize> = VecDeque::new();
        let mut parents: Vec<Option<usize>> = vec![None; self.nodes.len()];
        let mut parents_touched: Vec<usize> = Vec::new();
        let mut visited: Vec<bool> = vec![false; self.nodes.len()];
        let mut unprocessed_nodes: Vec<usize> = all_nodes;

        while !unprocessed_nodes.is_empty() && is_forest {
            to_be_visited.push_back(unprocessed_nodes[0]);

            let mut visited_order: Vec<usize> = Vec::new();

            while let Some(current_node) = to_be_visited.pop_front() {
                if !visited[current_node] {
                    visited[current_node] = true;
                    visited_order.push(current_node);
                }

                // Traverse all neighbors of this node, in edge insertion order.
                for &ei in &self.nodes[current_node].edges {
                    if !self.edges[ei].active {
                        continue;
                    }
                    let current_neighbor = self.edge_other_end(ei, current_node);
                    if !self.nodes[current_neighbor].active {
                        continue;
                    }

                    // If BFS is not growing from this neighbor.
                    if parents[current_node] != Some(current_neighbor) {
                        if !visited[current_neighbor] {
                            to_be_visited.push_back(current_neighbor);
                            if parents[current_neighbor].is_none() {
                                parents_touched.push(current_neighbor);
                            }
                            parents[current_neighbor] = Some(current_node);
                        } else {
                            is_forest = false;
                            break;
                        }
                    }
                }

                if !is_forest {
                    break;
                }
            }

            if !is_forest {
                flat_forest.clear();
            } else {
                // JS Set preserves insertion order; `visited_order` mimics `[...visited]`.
                flat_forest.push(visited_order.clone());

                // Remove all visited nodes from unProcessedNodes.
                unprocessed_nodes.retain(|&n| !visited[n]);

                // Clear per-component state (only touched indices).
                for &idx in &visited_order {
                    visited[idx] = false;
                }
                for idx in parents_touched.drain(..) {
                    parents[idx] = None;
                }

                to_be_visited.clear();
            }
        }

        flat_forest
    }

    fn active_degree(&self, node_idx: usize) -> usize {
        if !self.nodes[node_idx].active {
            return 0;
        }
        let mut d = 0usize;
        for &ei in &self.nodes[node_idx].edges {
            if !self.edges[ei].active {
                continue;
            }
            let other = self.edge_other_end(ei, node_idx);
            if self.nodes[other].active {
                d += 1;
            }
        }
        d
    }

    fn update_grid(&mut self, repulsion_range: f64) {
        self.grid.clear_cells();
        if self.nodes.iter().all(|n| !n.active) {
            return;
        }

        let mut min_left = f64::INFINITY;
        let mut min_top = f64::INFINITY;
        let mut max_right = f64::NEG_INFINITY;
        let mut max_bottom = f64::NEG_INFINITY;
        for n in &self.nodes {
            if !n.active {
                continue;
            }
            min_left = min_left.min(n.left);
            min_top = min_top.min(n.top);
            max_right = max_right.max(n.right());
            max_bottom = max_bottom.max(n.bottom());
        }
        if !(min_left.is_finite()
            && min_top.is_finite()
            && max_right.is_finite()
            && max_bottom.is_finite())
        {
            return;
        }

        // Match `layout-base` grid semantics:
        // - grid extents are based on the root graph bounds, which include `DEFAULT_GRAPH_MARGIN`
        //   (see `LGraph.updateBounds()` and `FDLayout.updateGrid()`).
        let left_with_margin = min_left - Self::DEFAULT_GRAPH_MARGIN;
        let top_with_margin = min_top - Self::DEFAULT_GRAPH_MARGIN;
        let right_with_margin = max_right + Self::DEFAULT_GRAPH_MARGIN;
        let bottom_with_margin = max_bottom + Self::DEFAULT_GRAPH_MARGIN;

        let size_x = ((right_with_margin - left_with_margin) / repulsion_range)
            .ceil()
            .max(1.0) as usize;
        let size_y = ((bottom_with_margin - top_with_margin) / repulsion_range)
            .ceil()
            .max(1.0) as usize;
        self.grid.reset(
            size_x,
            size_y,
            left_with_margin,
            top_with_margin,
            repulsion_range,
        );

        let clamp_x = |v: i32| v.clamp(0, (size_x as i32) - 1);
        let clamp_y = |v: i32| v.clamp(0, (size_y as i32) - 1);

        for (idx, n) in self.nodes.iter_mut().enumerate() {
            if !n.active {
                continue;
            }
            // `FDLayout.addNodeToGrid(v, left, top)` where `(left,top)` are root graph bounds
            // (already including `DEFAULT_GRAPH_MARGIN`).
            let start_x = ((n.left - left_with_margin) / repulsion_range).floor() as i32;
            let finish_x = ((n.right() - left_with_margin) / repulsion_range).floor() as i32;
            let start_y = ((n.top - top_with_margin) / repulsion_range).floor() as i32;
            let finish_y = ((n.bottom() - top_with_margin) / repulsion_range).floor() as i32;

            n.start_x = clamp_x(start_x);
            n.finish_x = clamp_x(finish_x);
            n.start_y = clamp_y(start_y);
            n.finish_y = clamp_y(finish_y);

            for gx in (n.start_x as usize)..=(n.finish_x as usize) {
                for gy in (n.start_y as usize)..=(n.finish_y as usize) {
                    self.grid.push(gx, gy, idx);
                }
            }
        }
    }

    /// Port of `layout-base` `Layout.findCenterOfTree(nodes)`.
    /// Note: this intentionally preserves the upstream loop's in-place removal behavior.
    fn find_center_of_tree(&self, nodes: &[usize]) -> usize {
        let mut list: Vec<usize> = nodes.to_vec();
        let mut removed: Vec<bool> = vec![false; self.nodes.len()];
        let mut remaining_degrees: Vec<usize> = vec![0; self.nodes.len()];
        let mut found_center = list.len() == 1 || list.len() == 2;
        let mut center_node = list[0];

        for &node in &list {
            let degree = self.active_degree(node);
            remaining_degrees[node] = degree;
            if degree == 1 {
                removed[node] = true;
            }
        }

        let mut temp_list: Vec<usize> = Vec::new();
        for &node in &list {
            if remaining_degrees[node] == 1 {
                temp_list.push(node);
            }
        }

        while !found_center {
            // Upstream bug-for-bug parity:
            // `Layout.findCenterOfTree()` creates `tempList2 = [...tempList]` but then iterates
            // over `list` (not `tempList2`) while removing from `list` in-place:
            //
            //   for (i=0; i<list.length; i++) { node=list[i]; list.splice(indexOf(node), 1); ... }
            //
            // This has the side-effect of skipping every other element. We replicate the exact
            // "remove then i++" semantics by using a `while` loop and `Vec::remove(i)`.
            temp_list.clear();
            let mut i = 0usize;
            while i < list.len() {
                let node = list[i];
                list.remove(i);

                self.for_each_active_neighbor(node, |neighbour| {
                    if removed[neighbour] {
                        return;
                    }
                    let other_degree = remaining_degrees[neighbour];
                    let new_degree = other_degree.saturating_sub(1);
                    if new_degree == 1 {
                        temp_list.push(neighbour);
                    }
                    remaining_degrees[neighbour] = new_degree;
                });

                i += 1;
            }

            for &v in &temp_list {
                removed[v] = true;
            }

            if list.len() == 1 || list.len() == 2 {
                found_center = true;
                center_node = list[0];
            }
        }

        center_node
    }

    fn max_diagonal_in_tree(&self, tree: &[usize]) -> f64 {
        let mut max_diag = f64::NEG_INFINITY;
        for &idx in tree {
            max_diag = max_diag.max(self.nodes[idx].diagonal());
        }
        if !max_diag.is_finite() { 0.0 } else { max_diag }
    }

    fn branch_radial_layout(
        &mut self,
        node: usize,
        parent: Option<usize>,
        start_angle: f64,
        end_angle: f64,
        distance: f64,
        radial_separation: f64,
    ) {
        struct BranchFrame {
            node: usize,
            parent: Option<usize>,
            start_angle: f64,
            end_angle: f64,
            distance: f64,
        }

        let mut stack = vec![BranchFrame {
            node,
            parent,
            start_angle,
            end_angle,
            distance,
        }];

        while let Some(frame) = stack.pop() {
            // First, position this node by finding its angle.
            let mut half_interval = ((frame.end_angle - frame.start_angle) + 1.0) / 2.0;
            if half_interval < 0.0 {
                half_interval += 180.0;
            }
            let node_angle = (half_interval + frame.start_angle).rem_euclid(360.0);
            let teta = (node_angle * std::f64::consts::TAU) / 360.0;
            let x_ = frame.distance * teta.cos();
            let y_ = frame.distance * teta.sin();
            self.nodes[frame.node].set_center(x_, y_);

            let neighbor_edges: Vec<usize> = self.nodes[frame.node].edges.clone();
            let inc_edges_count = neighbor_edges.len();
            let edge_to_parent = frame
                .parent
                .and_then(|parent| self.active_edge_between(frame.node, parent));
            let mut child_count = inc_edges_count;
            if edge_to_parent.is_some() {
                child_count = child_count.saturating_sub(1);
            }

            let start_index =
                if let Some(parent_edge) = edge_to_parent.filter(|_| inc_edges_count > 0) {
                    (neighbor_edges
                        .iter()
                        .position(|&edge| edge == parent_edge)
                        .unwrap_or(0)
                        + 1)
                        % inc_edges_count
                } else {
                    0
                };

            let step_angle = if child_count == 0 {
                0.0
            } else {
                (frame.end_angle - frame.start_angle).abs() / (child_count as f64)
            };

            if child_count == 0 || inc_edges_count == 0 {
                continue;
            }

            let mut child_frames = Vec::with_capacity(child_count);
            let mut branch_count = 0usize;
            let mut i = start_index;
            while branch_count != child_count {
                let current_neighbor = self.edge_other_end(neighbor_edges[i], frame.node);
                if Some(current_neighbor) == frame.parent {
                    i = (i + 1) % inc_edges_count;
                    continue;
                }

                let child_start_angle =
                    (frame.start_angle + (branch_count as f64) * step_angle).rem_euclid(360.0);
                let child_end_angle = (child_start_angle + step_angle).rem_euclid(360.0);
                child_frames.push(BranchFrame {
                    node: current_neighbor,
                    parent: Some(frame.node),
                    start_angle: child_start_angle,
                    end_angle: child_end_angle,
                    distance: frame.distance + radial_separation,
                });

                branch_count += 1;
                i = (i + 1) % inc_edges_count;
            }

            for child_frame in child_frames.into_iter().rev() {
                stack.push(child_frame);
            }
        }
    }

    fn radial_layout(
        &mut self,
        tree: &[usize],
        center_node: usize,
        starting_x: f64,
        starting_y: f64,
    ) -> (f64, f64) {
        let radial_sep = self
            .max_diagonal_in_tree(tree)
            .max(Self::DEFAULT_RADIAL_SEPARATION);

        self.branch_radial_layout(center_node, None, 0.0, 359.0, 0.0, radial_sep);

        let Some(bounds) = Bounds::from_nodes(&self.nodes, tree) else {
            return (starting_x, starting_y);
        };

        // `Transform` with extents 1.0 is a pure translation: worldOrg + (x - deviceOrg).
        let dx = starting_x - bounds.min_x;
        let dy = starting_y - bounds.min_y;
        for &idx in tree {
            self.nodes[idx].left += dx;
            self.nodes[idx].top += dy;
        }

        (bounds.max_x + dx, bounds.max_y + dy)
    }

    fn position_nodes_radially(&mut self, forest: &[Vec<usize>]) {
        // Tile the trees to a grid row by row; first tree starts at (0,0).
        let number_of_columns = (forest.len() as f64).sqrt().ceil().max(1.0) as usize;
        let mut height = 0.0;
        let mut current_y = 0.0;
        let mut current_x = 0.0;
        let mut point = (0.0, 0.0);

        for (i, tree) in forest.iter().enumerate() {
            if i % number_of_columns == 0 {
                current_x = 0.0;
                current_y = height;
                if i != 0 {
                    current_y += Self::DEFAULT_COMPONENT_SEPERATION;
                }
                height = 0.0;
            }

            let center_node = self.find_center_of_tree(tree);
            point = self.radial_layout(tree, center_node, current_x, current_y);

            if point.1 > height {
                height = point.1.floor();
            }

            current_x = (point.0 + Self::DEFAULT_COMPONENT_SEPERATION).floor();
        }

        // Match upstream `positionNodesRadially` final world-centering pass:
        // `this.transform(new PointD(WORLD_CENTER_X - point.x/2, WORLD_CENTER_Y - point.y/2))`
        // (layout-base). This is *not* equivalent to adding `(WORLD_CENTER - point/2)` directly:
        // `Layout.transform(...)` also subtracts the current root graph left/top (with margins),
        // and those exact floating-point cancellations affect downstream `===` checks.
        let world_org_x = Self::WORLD_CENTER_X - point.0 / 2.0;
        let world_org_y = Self::WORLD_CENTER_Y - point.1 / 2.0;

        let mut min_left = f64::INFINITY;
        let mut min_top = f64::INFINITY;
        for n in &self.nodes {
            if !n.active {
                continue;
            }
            min_left = min_left.min(n.left);
            min_top = min_top.min(n.top);
        }
        if min_left.is_finite() && min_top.is_finite() {
            let device_org_x = min_left - Self::DEFAULT_GRAPH_MARGIN;
            let device_org_y = min_top - Self::DEFAULT_GRAPH_MARGIN;
            let dx = world_org_x - device_org_x;
            let dy = world_org_y - device_org_y;
            for n in &mut self.nodes {
                n.move_by(dx, dy);
            }
        }
    }

    fn run_spring_embedder(&mut self, timing_enabled: bool) {
        if self.nodes.is_empty() {
            return;
        }

        #[derive(Debug, Default, Clone)]
        struct SpringEmbedderTimings {
            total: web_time::Duration,
            nodes_to_apply_gravitation: web_time::Duration,
            update_grid: web_time::Duration,
            spring_forces: web_time::Duration,
            repulsion_forces: web_time::Duration,
            gravitation_forces: web_time::Duration,
            move_nodes: web_time::Duration,
            iterations: usize,
            active_edges_spring: u64,
            repulsion_pairs_considered: u64,
            repulsion_pairs_in_range: u64,
        }
        let mut timings = SpringEmbedderTimings::default();
        let total_start = timing_enabled.then(web_time::Instant::now);

        // Mermaid's Cytoscape COSE-Bilkent applies gravitational forces only when the graph is
        // disconnected (`calculateNodesToApplyGravitationTo()` collects nodes from non-connected
        // graphs). For a connected mindmap tree this list is empty, so gravity is a no-op.
        let nodes_with_gravity_start = timing_enabled.then(web_time::Instant::now);
        let nodes_with_gravity = self.nodes_to_apply_gravitation();
        if let Some(s) = nodes_with_gravity_start {
            timings.nodes_to_apply_gravitation = s.elapsed();
        }

        fn nodes2_mut(nodes: &mut [SimNode], a: usize, b: usize) -> (&mut SimNode, &mut SimNode) {
            debug_assert!(a != b);
            if a < b {
                let (left, right) = nodes.split_at_mut(b);
                (&mut left[a], &mut right[0])
            } else {
                let (left, right) = nodes.split_at_mut(a);
                (&mut right[0], &mut left[b])
            }
        }

        // These are instance fields in upstream `FDLayout`/`CoSELayout`.
        let ideal_edge_length = Self::DEFAULT_EDGE_LENGTH.max(10.0);
        let spring_constant = Self::DEFAULT_SPRING_STRENGTH;
        let repulsion_constant = Self::DEFAULT_REPULSION_STRENGTH;
        let gravity_constant = Self::DEFAULT_GRAVITY_STRENGTH;
        let gravity_range_factor = Self::DEFAULT_GRAVITY_RANGE_FACTOR;
        let repulsion_range = 2.0 * ideal_edge_length;

        let active_n = self.nodes.iter().filter(|n| n.active).count().max(1) as f64;
        let displacement_threshold_per_node = (3.0 * Self::DEFAULT_EDGE_LENGTH) / 100.0;
        let total_displacement_threshold = displacement_threshold_per_node * active_n;

        // Non-incremental mode: coolingFactor starts at 1.0 for small graphs.
        let initial_cooling_factor = 1.0;
        let mut cooling_factor = initial_cooling_factor;
        let max_iterations = Self::MAX_ITERATIONS.max(active_n as usize * 5);
        let max_cooling_cycle = (max_iterations as f64) / (Self::CONVERGENCE_CHECK_PERIOD as f64);
        let final_temperature = (Self::CONVERGENCE_CHECK_PERIOD as f64) / (max_iterations as f64);
        let mut cooling_cycle = 0.0f64;
        // Mermaid (via `rendering-util/layout-algorithms/cose-bilkent/cytoscape-setup.ts`) uses
        // `quality: 'proof'` for COSE-Bilkent.
        let layout_quality = 2i32;

        let mut total_iterations = 0usize;
        let mut old_total_displacement = 0.0f64;
        let mut last_total_displacement = 0.0f64;

        let mut processed_repulsion: Vec<bool> = vec![false; self.nodes.len()];

        loop {
            total_iterations += 1;
            if timing_enabled {
                timings.iterations += 1;
            }

            if total_iterations == max_iterations {
                break;
            }

            if total_iterations.is_multiple_of(Self::CONVERGENCE_CHECK_PERIOD) {
                let oscilating = total_iterations > (max_iterations / 3)
                    && (last_total_displacement - old_total_displacement).abs() < 2.0;
                let converged = last_total_displacement < total_displacement_threshold;

                old_total_displacement = last_total_displacement;

                if converged || oscilating {
                    break;
                }

                cooling_cycle += 1.0;

                // cooling schedule 3 (see upstream comment in `CoSELayout.tick`)
                let numerator = (100.0 * (initial_cooling_factor - final_temperature)).ln();
                let denominator = max_cooling_cycle.ln().max(1e-9);
                let power = numerator / denominator;
                let cooling_adjuster = match layout_quality {
                    0 => cooling_cycle,
                    1 => cooling_cycle / 3.0,
                    _ => 1.0,
                };
                let schedule = cooling_cycle.powf(power) / 100.0 * cooling_adjuster;
                cooling_factor = (initial_cooling_factor - schedule).max(final_temperature);
            }

            let mut total_displacement = 0.0f64;

            // Spring forces
            let spring_start = timing_enabled.then(web_time::Instant::now);
            for e in &self.edges {
                if !e.active {
                    continue;
                }
                let (a, b) = (e.a, e.b);
                if !(self.nodes[a].active && self.nodes[b].active) {
                    continue;
                }
                if timing_enabled {
                    timings.active_edges_spring += 1;
                }

                // Upstream `FDLayout.calcSpringForce` uses clipping points on the node rectangles
                // (via `IGeometry.getIntersection`) so the "ideal edge length" applies between
                // node borders rather than between node centers.
                let (target_x, target_y, source_x, source_y, overlapped) =
                    rect_intersection_points(&self.nodes[b], &self.nodes[a]);
                if overlapped {
                    continue;
                }
                let mut lx = target_x - source_x;
                let mut ly = target_y - source_y;

                // Mirror `LEdge.updateLength(...)` from `layout-base`: very small components are
                // snapped to their sign (or 0 if the component is 0).
                if lx.abs() < 1.0 {
                    lx = Self::imath_sign(lx);
                }
                if ly.abs() < 1.0 {
                    ly = Self::imath_sign(ly);
                }

                let len = (lx * lx + ly * ly).sqrt();
                if len == 0.0 {
                    continue;
                }
                let spring_force = spring_constant * (len - ideal_edge_length);
                let sfx = spring_force * (lx / len);
                let sfy = spring_force * (ly / len);
                let (na, nb) = nodes2_mut(&mut self.nodes, a, b);
                na.spring_fx += sfx;
                na.spring_fy += sfy;
                nb.spring_fx -= sfx;
                nb.spring_fy -= sfy;
            }
            if let Some(s) = spring_start {
                timings.spring_forces += s.elapsed();
            }

            // Repulsion forces (FR-grid variant).
            //
            // Mirrors `FDLayout.calcRepulsionForces` + `calculateRepulsionForceOfANode`:
            // - rebuild the grid every `GRID_CALCULATION_CHECK_PERIOD` iterations (when allowed)
            // - cache `node.surrounding` between grid rebuilds
            // - candidate filtering uses *border distances* against `repulsionRange`
            let repulsion_start = timing_enabled.then(web_time::Instant::now);
            let rebuild_surrounding = total_iterations % Self::GRID_CALCULATION_CHECK_PERIOD == 1;

            if rebuild_surrounding {
                let update_grid_start = timing_enabled.then(web_time::Instant::now);
                self.update_grid(repulsion_range);
                if let Some(s) = update_grid_start {
                    timings.update_grid += s.elapsed();
                }
            }

            processed_repulsion.fill(false);

            if !self.grid.is_empty() {
                let size_x_i32 = self.grid.size_x().min(i32::MAX as usize) as i32;
                let size_y_i32 = self.grid.size_y().min(i32::MAX as usize) as i32;

                for a in 0..self.nodes.len() {
                    if !self.nodes[a].active {
                        continue;
                    }

                    if rebuild_surrounding {
                        self.nodes[a].surrounding.clear();

                        self.repulsion_seen_gen = self.repulsion_seen_gen.wrapping_add(1);
                        if self.repulsion_seen_gen == 0 {
                            self.repulsion_seen.fill(0);
                            self.repulsion_seen_gen = 1;
                        }
                        let seen_gen = self.repulsion_seen_gen;

                        let ni = &self.nodes[a];
                        let gx0 = (ni.start_x - 1).max(0) as usize;
                        let gy0 = (ni.start_y - 1).max(0) as usize;
                        let gx1 = (ni.finish_x + 1).min(size_x_i32.saturating_sub(1)) as usize;
                        let gy1 = (ni.finish_y + 1).min(size_y_i32.saturating_sub(1)) as usize;

                        for gx in gx0..=gx1 {
                            for gy in gy0..=gy1 {
                                for &b in self.grid.cell(gx, gy) {
                                    // `processedNodeSet` semantics: compute each pair once.
                                    if processed_repulsion[b] {
                                        continue;
                                    }
                                    if b == a || !self.nodes[b].active {
                                        continue;
                                    }
                                    // `surrounding.has(nodeB)` semantics: preserve first-hit insertion order.
                                    if self.repulsion_seen[b] == seen_gen {
                                        continue;
                                    }

                                    let na = &self.nodes[a];
                                    let nb = &self.nodes[b];
                                    let dist_x = (na.center_x() - nb.center_x()).abs()
                                        - (na.half_w() + nb.half_w());
                                    let dist_y = (na.center_y() - nb.center_y()).abs()
                                        - (na.half_h() + nb.half_h());
                                    if dist_x <= repulsion_range && dist_y <= repulsion_range {
                                        self.repulsion_seen[b] = seen_gen;
                                        self.nodes[a].surrounding.push(b);
                                    }
                                }
                            }
                        }
                    }

                    let surrounding = self.nodes[a].surrounding.clone();
                    for b in surrounding {
                        if timing_enabled {
                            timings.repulsion_pairs_considered += 1;
                            timings.repulsion_pairs_in_range += 1;
                        }
                        let (rfx, rfy) = self.calc_repulsion_force(a, b, repulsion_constant);
                        let (na, nb) = nodes2_mut(&mut self.nodes, a, b);
                        na.repulsion_fx += rfx;
                        na.repulsion_fy += rfy;
                        nb.repulsion_fx -= rfx;
                        nb.repulsion_fy -= rfy;
                    }

                    processed_repulsion[a] = true;
                }
            }

            if let Some(s) = repulsion_start {
                timings.repulsion_forces += s.elapsed();
            }

            // Gravitation (only for disconnected graphs).
            let gravitation_start = timing_enabled.then(web_time::Instant::now);
            if !nodes_with_gravity.is_empty() {
                if let Some((owner_center_x, owner_center_y, estimated_size)) =
                    self.gravitation_context(gravity_range_factor)
                {
                    for &idx in &nodes_with_gravity {
                        let n = &mut self.nodes[idx];
                        if !n.active {
                            continue;
                        }
                        let distance_x = n.center_x() - owner_center_x;
                        let distance_y = n.center_y() - owner_center_y;
                        let abs_distance_x = distance_x.abs() + n.width / 2.0;
                        let abs_distance_y = distance_y.abs() + n.height / 2.0;
                        if abs_distance_x > estimated_size || abs_distance_y > estimated_size {
                            n.gravitation_fx = -gravity_constant * distance_x;
                            n.gravitation_fy = -gravity_constant * distance_y;
                        }
                    }
                }
            }
            if let Some(s) = gravitation_start {
                timings.gravitation_forces += s.elapsed();
            }

            // Move nodes
            let move_start = timing_enabled.then(web_time::Instant::now);
            for n in &mut self.nodes {
                if !n.active {
                    continue;
                }
                let dx = cooling_factor * (n.spring_fx + n.repulsion_fx + n.gravitation_fx);
                let dy = cooling_factor * (n.spring_fy + n.repulsion_fy + n.gravitation_fy);

                let mut mdx = dx;
                let mut mdy = dy;
                let max_d = cooling_factor * Self::MAX_NODE_DISPLACEMENT;
                if mdx.abs() > max_d {
                    mdx = max_d * Self::imath_sign(mdx);
                }
                if mdy.abs() > max_d {
                    mdy = max_d * Self::imath_sign(mdy);
                }

                n.move_by(mdx, mdy);
                total_displacement += mdx.abs() + mdy.abs();

                // Reset forces
                n.spring_fx = 0.0;
                n.spring_fy = 0.0;
                n.repulsion_fx = 0.0;
                n.repulsion_fy = 0.0;
                n.gravitation_fx = 0.0;
                n.gravitation_fy = 0.0;
            }
            if let Some(s) = move_start {
                timings.move_nodes += s.elapsed();
            }

            last_total_displacement = total_displacement;
        }

        if let Some(s) = total_start {
            timings.total = s.elapsed();
            eprintln!(
                "[manatee-cose-spring] total={:?} iters={} gravity_select={:?} update_grid={:?} spring={:?} repulsion={:?} gravitation={:?} move={:?} spring_edges={} repulsion_pairs={} repulsion_in_range={}",
                timings.total,
                timings.iterations,
                timings.nodes_to_apply_gravitation,
                timings.update_grid,
                timings.spring_forces,
                timings.repulsion_forces,
                timings.gravitation_forces,
                timings.move_nodes,
                timings.active_edges_spring,
                timings.repulsion_pairs_considered,
                timings.repulsion_pairs_in_range,
            );
        }
    }

    #[cfg(test)]
    fn run_single_spring_tick_flat_graph(&mut self) {
        if self.nodes.is_empty() {
            return;
        }

        fn nodes2_mut(nodes: &mut [SimNode], a: usize, b: usize) -> (&mut SimNode, &mut SimNode) {
            debug_assert!(a != b);
            if a < b {
                let (left, right) = nodes.split_at_mut(b);
                (&mut left[a], &mut right[0])
            } else {
                let (left, right) = nodes.split_at_mut(a);
                (&mut right[0], &mut left[b])
            }
        }

        let ideal_edge_length = Self::DEFAULT_EDGE_LENGTH.max(10.0);
        let spring_constant = Self::DEFAULT_SPRING_STRENGTH;
        let repulsion_constant = Self::DEFAULT_REPULSION_STRENGTH;
        let repulsion_range = 2.0 * ideal_edge_length;

        // Tick #1 in upstream always triggers a grid rebuild (`totalIterations % 10 == 1`).
        self.update_grid(repulsion_range);

        // Spring forces.
        let mut spring_debug: Vec<(usize, usize, f64, f64, f64, f64)> = Vec::new(); // (a,b,lx,ly,len,sfy)
        for e in &self.edges {
            if !e.active {
                continue;
            }
            let (a, b) = (e.a, e.b);
            if !(self.nodes[a].active && self.nodes[b].active) {
                continue;
            }
            let (target_x, target_y, source_x, source_y, overlapped) =
                rect_intersection_points(&self.nodes[b], &self.nodes[a]);
            if overlapped {
                continue;
            }
            let mut lx = target_x - source_x;
            let mut ly = target_y - source_y;
            if lx.abs() < 1.0 {
                lx = Self::imath_sign(lx);
            }
            if ly.abs() < 1.0 {
                ly = Self::imath_sign(ly);
            }
            let len = (lx * lx + ly * ly).sqrt();
            if len == 0.0 {
                continue;
            }
            let spring_force = spring_constant * (len - ideal_edge_length);
            let sfx = spring_force * (lx / len);
            let sfy = spring_force * (ly / len);
            spring_debug.push((a, b, lx, ly, len, sfy));
            let (na, nb) = nodes2_mut(&mut self.nodes, a, b);
            na.spring_fx += sfx;
            na.spring_fy += sfy;
            nb.spring_fx -= sfx;
            nb.spring_fy -= sfy;
        }

        // Repulsion forces (FR-grid, tick #1: rebuild surrounding).
        let mut processed_repulsion: Vec<bool> = vec![false; self.nodes.len()];
        processed_repulsion.fill(false);
        if !self.grid.is_empty() {
            let size_x_i32 = self.grid.size_x().min(i32::MAX as usize) as i32;
            let size_y_i32 = self.grid.size_y().min(i32::MAX as usize) as i32;

            for a in 0..self.nodes.len() {
                if !self.nodes[a].active {
                    continue;
                }

                self.nodes[a].surrounding.clear();

                self.repulsion_seen_gen = self.repulsion_seen_gen.wrapping_add(1);
                if self.repulsion_seen_gen == 0 {
                    self.repulsion_seen.fill(0);
                    self.repulsion_seen_gen = 1;
                }
                let seen_gen = self.repulsion_seen_gen;

                let ni = &self.nodes[a];
                let gx0 = (ni.start_x - 1).max(0) as usize;
                let gy0 = (ni.start_y - 1).max(0) as usize;
                let gx1 = (ni.finish_x + 1).min(size_x_i32.saturating_sub(1)) as usize;
                let gy1 = (ni.finish_y + 1).min(size_y_i32.saturating_sub(1)) as usize;

                for gx in gx0..=gx1 {
                    for gy in gy0..=gy1 {
                        for &b in self.grid.cell(gx, gy) {
                            if processed_repulsion[b] {
                                continue;
                            }
                            if b == a || !self.nodes[b].active {
                                continue;
                            }
                            if self.repulsion_seen[b] == seen_gen {
                                continue;
                            }

                            let na = &self.nodes[a];
                            let nb = &self.nodes[b];
                            let dist_x =
                                (na.center_x() - nb.center_x()).abs() - (na.half_w() + nb.half_w());
                            let dist_y =
                                (na.center_y() - nb.center_y()).abs() - (na.half_h() + nb.half_h());
                            if dist_x <= repulsion_range && dist_y <= repulsion_range {
                                self.repulsion_seen[b] = seen_gen;
                                self.nodes[a].surrounding.push(b);
                            }
                        }
                    }
                }

                let surrounding = self.nodes[a].surrounding.clone();
                for b in surrounding {
                    let (rfx, rfy) = self.calc_repulsion_force(a, b, repulsion_constant);
                    let (na, nb) = nodes2_mut(&mut self.nodes, a, b);
                    na.repulsion_fx += rfx;
                    na.repulsion_fy += rfy;
                    nb.repulsion_fx -= rfx;
                    nb.repulsion_fy -= rfy;
                }

                processed_repulsion[a] = true;
            }
        }

        // For horizontal arrangements, y-forces should remain exactly zero.
        for (i, n) in self.nodes.iter().enumerate() {
            if !(n.spring_fy == 0.0 && n.repulsion_fy == 0.0 && n.gravitation_fy == 0.0) {
                panic!(
                    "unexpected y force before move: node[{i}] spring_fy={} repulsion_fy={} gravitation_fy={} spring_debug={:?}",
                    n.spring_fy, n.repulsion_fy, n.gravitation_fy, spring_debug
                );
            }
        }

        // Move nodes (coolingFactor=1.0 on tick #1 for small, non-incremental graphs).
        let cooling_factor = 1.0;
        for n in &mut self.nodes {
            if !n.active {
                continue;
            }
            let dx = cooling_factor * (n.spring_fx + n.repulsion_fx + n.gravitation_fx);
            let dy = cooling_factor * (n.spring_fy + n.repulsion_fy + n.gravitation_fy);

            let mut mdx = dx;
            let mut mdy = dy;
            let max_d = cooling_factor * Self::MAX_NODE_DISPLACEMENT;
            if mdx.abs() > max_d {
                mdx = max_d * Self::imath_sign(mdx);
            }
            if mdy.abs() > max_d {
                mdy = max_d * Self::imath_sign(mdy);
            }

            n.move_by(mdx, mdy);

            n.spring_fx = 0.0;
            n.spring_fy = 0.0;
            n.repulsion_fx = 0.0;
            n.repulsion_fy = 0.0;
            n.gravitation_fx = 0.0;
            n.gravitation_fy = 0.0;
        }
    }

    fn calc_repulsion_force(&self, a: usize, b: usize, repulsion_constant: f64) -> (f64, f64) {
        let na = &self.nodes[a];
        let nb = &self.nodes[b];

        if rects_intersect(na, nb) {
            let (ox, oy) = calc_separation_amount(na, nb, Self::DEFAULT_EDGE_LENGTH / 2.0);
            let repulsion_fx = 2.0 * ox;
            let repulsion_fy = 2.0 * oy;
            // `childrenConstant = 1*1/(1+1) = 0.5` for flat leaf nodes.
            (-0.5 * repulsion_fx, -0.5 * repulsion_fy)
        } else {
            // Use clipping points (approx) to account for node dimensions.
            // Avoid the redundant overlap check inside `rect_intersection_points`.
            let (ax, ay, bx, by) = rect_intersection_points_no_overlap_check(na, nb);
            let mut dx = bx - ax;
            let mut dy = by - ay;

            if dx.abs() < Self::MIN_REPULSION_DIST {
                dx = Self::imath_sign(dx) * Self::MIN_REPULSION_DIST;
            }
            if dy.abs() < Self::MIN_REPULSION_DIST {
                dy = Self::imath_sign(dy) * Self::MIN_REPULSION_DIST;
            }

            let dist_sq = dx * dx + dy * dy;
            let dist = dist_sq.sqrt();
            if dist_sq == 0.0 || dist == 0.0 {
                return (0.0, 0.0);
            }
            let repulsion_force = repulsion_constant / dist_sq;
            let rfx = repulsion_force * dx / dist;
            let rfy = repulsion_force * dy / dist;
            (-rfx, -rfy)
        }
    }

    /// Port of `Layout.transform(newLeftTop)` for the root graph and `newLeftTop = (0,0)`.
    /// This moves the layout into a positive coordinate space with a fixed margin (15px).
    fn transform_to_origin(&mut self) {
        if self.nodes.is_empty() {
            return;
        }

        let mut min_left = f64::INFINITY;
        let mut min_top = f64::INFINITY;
        for n in &self.nodes {
            if !n.active {
                continue;
            }
            min_left = min_left.min(n.left);
            min_top = min_top.min(n.top);
        }
        if !(min_left.is_finite() && min_top.is_finite()) {
            return;
        }

        let left_top_x = min_left - Self::DEFAULT_GRAPH_MARGIN;
        let left_top_y = min_top - Self::DEFAULT_GRAPH_MARGIN;

        // Translate so `left_top` becomes (0,0).
        let dx = -left_top_x;
        let dy = -left_top_y;
        for n in &mut self.nodes {
            if !n.active {
                continue;
            }
            n.left += dx;
            n.top += dy;
        }
    }

    fn nodes_to_apply_gravitation(&self) -> Vec<usize> {
        // Port of COSE `calculateNodesToApplyGravitationTo()` for a flat graph: apply gravity to
        // all nodes only if the graph is disconnected.
        let mut first_active: Option<usize> = None;
        for (i, n) in self.nodes.iter().enumerate() {
            if n.active {
                first_active = Some(i);
                break;
            }
        }
        let Some(start) = first_active else {
            return Vec::new();
        };

        let mut stack: Vec<usize> = vec![start];
        let mut seen: Vec<bool> = vec![false; self.nodes.len()];
        let mut seen_count: usize = 1;
        seen[start] = true;

        while let Some(u) = stack.pop() {
            for &ei in &self.nodes[u].edges {
                if !self.edges[ei].active {
                    continue;
                }
                let v = self.edge_other_end(ei, u);
                if !self.nodes[v].active {
                    continue;
                }
                if !seen[v] {
                    seen[v] = true;
                    seen_count += 1;
                    stack.push(v);
                }
            }
        }

        let active_count = self.nodes.iter().filter(|n| n.active).count();
        if seen_count == active_count {
            Vec::new()
        } else {
            (0..self.nodes.len())
                .filter(|&i| self.nodes[i].active)
                .collect()
        }
    }

    fn gravitation_context(&self, gravity_range_factor: f64) -> Option<(f64, f64, f64)> {
        // Port of `FDLayout.calcGravitationalForce` context:
        // - owner center = bbox center of the root graph
        // - estimatedSize = root.estimatedSize * gravityRangeFactor
        let mut min_left = f64::INFINITY;
        let mut max_right = f64::NEG_INFINITY;
        let mut min_top = f64::INFINITY;
        let mut max_bottom = f64::NEG_INFINITY;

        let mut size_sum = 0.0f64;
        let mut active_n = 0usize;

        for n in &self.nodes {
            if !n.active {
                continue;
            }
            active_n += 1;
            min_left = min_left.min(n.left);
            max_right = max_right.max(n.right());
            min_top = min_top.min(n.top);
            max_bottom = max_bottom.max(n.bottom());
            size_sum += (n.width + n.height) / 2.0;
        }

        if active_n == 0
            || !(min_left.is_finite()
                && max_right.is_finite()
                && min_top.is_finite()
                && max_bottom.is_finite())
        {
            return None;
        }

        let owner_center_x = (max_right + min_left) / 2.0;
        let owner_center_y = (max_bottom + min_top) / 2.0;

        let estimated_size_base = if size_sum == 0.0 {
            // `LayoutConstants.EMPTY_COMPOUND_NODE_SIZE`
            40.0
        } else {
            size_sum / (active_n as f64).sqrt()
        };
        let estimated_size = estimated_size_base * gravity_range_factor;
        if !estimated_size.is_finite() {
            return None;
        }

        Some((owner_center_x, owner_center_y, estimated_size))
    }
}

fn rects_intersect(a: &SimNode, b: &SimNode) -> bool {
    // Match `layout-base` `RectangleD.intersects` semantics:
    // - touching borders counts as intersection (uses `<`, not `<=`, in early-exit checks)
    if a.right() < b.left {
        return false;
    }
    if a.bottom() < b.top {
        return false;
    }
    if b.right() < a.left {
        return false;
    }
    if b.bottom() < a.top {
        return false;
    }
    true
}

/// Port of `layout-base` `IGeometry.getIntersection2(rectA, rectB, result)`.
///
/// Returns `(ax, ay, bx, by, overlapped)` where `(ax,ay)` is rectA's clip point and `(bx,by)` is
/// rectB's clip point on the line segment between their centers.
fn rect_intersection_points(a: &SimNode, b: &SimNode) -> (f64, f64, f64, f64, bool) {
    let p1x = a.center_x();
    let p1y = a.center_y();
    let p2x = b.center_x();
    let p2y = b.center_y();

    if rects_intersect(a, b) {
        return (p1x, p1y, p2x, p2y, true);
    }

    // NOTE: This intentionally mirrors the upstream `IGeometry.getIntersection2` implementation
    // from `layout-base` (including its branching structure) rather than a mathematically
    // equivalent closed-form intersection. Downstream convergence is sensitive to these
    // conditionals due to floating-point comparisons.

    // rectA corners
    let top_left_ax = a.left;
    let top_left_ay = a.top;
    let top_right_ax = a.right();
    let bottom_left_ax = a.left;
    let bottom_left_ay = a.bottom();
    let bottom_right_ax = a.right();
    let half_width_a = a.width / 2.0;
    let half_height_a = a.height / 2.0;

    // rectB corners
    let top_left_bx = b.left;
    let top_left_by = b.top;
    let top_right_bx = b.right();
    let bottom_left_bx = b.left;
    let bottom_left_by = b.bottom();
    let bottom_right_bx = b.right();
    let half_width_b = b.width / 2.0;
    let half_height_b = b.height / 2.0;

    // Line is vertical.
    if p1x == p2x {
        if p1y > p2y {
            return (p1x, top_left_ay, p2x, bottom_left_by, false);
        } else if p1y < p2y {
            return (p1x, bottom_left_ay, p2x, top_left_by, false);
        } else {
            return (p1x, p1y, p2x, p2y, false);
        }
    }

    // Line is horizontal.
    if p1y == p2y {
        if p1x > p2x {
            return (top_left_ax, p1y, top_right_bx, p2y, false);
        } else if p1x < p2x {
            return (top_right_ax, p1y, top_left_bx, p2y, false);
        } else {
            return (p1x, p1y, p2x, p2y, false);
        }
    }

    #[inline]
    fn get_cardinal_direction(slope: f64, slope_prime: f64, line: i32) -> i32 {
        if slope > slope_prime {
            line
        } else {
            1 + (line % 4)
        }
    }

    let slope_a = a.height / a.width;
    let slope_b = b.height / b.width;
    let slope_prime = (p2y - p1y) / (p2x - p1x);

    let mut ax = 0.0;
    let mut ay = 0.0;
    let mut bx = 0.0;
    let mut by = 0.0;
    let mut clip_a_found = false;
    let mut clip_b_found = false;

    // Determine whether clipping point is the corner of rectA.
    if -slope_a == slope_prime {
        if p1x > p2x {
            ax = bottom_left_ax;
            ay = bottom_left_ay;
            clip_a_found = true;
        } else {
            ax = top_right_ax;
            ay = top_left_ay;
            clip_a_found = true;
        }
    } else if slope_a == slope_prime {
        if p1x > p2x {
            ax = top_left_ax;
            ay = top_left_ay;
            clip_a_found = true;
        } else {
            ax = bottom_right_ax;
            ay = bottom_left_ay;
            clip_a_found = true;
        }
    }

    // Determine whether clipping point is the corner of rectB.
    if -slope_b == slope_prime {
        if p2x > p1x {
            bx = bottom_left_bx;
            by = bottom_left_by;
            clip_b_found = true;
        } else {
            bx = top_right_bx;
            by = top_left_by;
            clip_b_found = true;
        }
    } else if slope_b == slope_prime {
        if p2x > p1x {
            bx = top_left_bx;
            by = top_left_by;
            clip_b_found = true;
        } else {
            bx = bottom_right_bx;
            by = bottom_left_by;
            clip_b_found = true;
        }
    }

    if clip_a_found && clip_b_found {
        return (ax, ay, bx, by, false);
    }

    let (card_a, card_b) = if p1x > p2x {
        if p1y > p2y {
            (
                get_cardinal_direction(slope_a, slope_prime, 4),
                get_cardinal_direction(slope_b, slope_prime, 2),
            )
        } else {
            (
                get_cardinal_direction(-slope_a, slope_prime, 3),
                get_cardinal_direction(-slope_b, slope_prime, 1),
            )
        }
    } else if p1y > p2y {
        (
            get_cardinal_direction(-slope_a, slope_prime, 1),
            get_cardinal_direction(-slope_b, slope_prime, 3),
        )
    } else {
        (
            get_cardinal_direction(slope_a, slope_prime, 2),
            get_cardinal_direction(slope_b, slope_prime, 4),
        )
    };

    if !clip_a_found {
        match card_a {
            1 => {
                ay = top_left_ay;
                ax = p1x + (-half_height_a) / slope_prime;
            }
            2 => {
                ax = bottom_right_ax;
                ay = p1y + half_width_a * slope_prime;
            }
            3 => {
                ay = bottom_left_ay;
                ax = p1x + half_height_a / slope_prime;
            }
            _ => {
                ax = bottom_left_ax;
                ay = p1y + (-half_width_a) * slope_prime;
            }
        }
    }

    if !clip_b_found {
        match card_b {
            1 => {
                by = top_left_by;
                bx = p2x + (-half_height_b) / slope_prime;
            }
            2 => {
                bx = bottom_right_bx;
                by = p2y + half_width_b * slope_prime;
            }
            3 => {
                by = bottom_left_by;
                bx = p2x + half_height_b / slope_prime;
            }
            _ => {
                bx = bottom_left_bx;
                by = p2y + (-half_width_b) * slope_prime;
            }
        }
    }

    (ax, ay, bx, by, false)
}

#[inline]
fn rect_intersection_points_no_overlap_check(a: &SimNode, b: &SimNode) -> (f64, f64, f64, f64) {
    // Fast path for callers that already know `rects_intersect(a, b) == false`.
    let p1x = a.center_x();
    let p1y = a.center_y();
    let p2x = b.center_x();
    let p2y = b.center_y();

    // rectA corners
    let top_left_ax = a.left;
    let top_left_ay = a.top;
    let top_right_ax = a.right();
    let bottom_left_ax = a.left;
    let bottom_left_ay = a.bottom();
    let bottom_right_ax = a.right();
    let half_width_a = a.width / 2.0;
    let half_height_a = a.height / 2.0;

    // rectB corners
    let top_left_bx = b.left;
    let top_left_by = b.top;
    let top_right_bx = b.right();
    let bottom_left_bx = b.left;
    let bottom_left_by = b.bottom();
    let bottom_right_bx = b.right();
    let half_width_b = b.width / 2.0;
    let half_height_b = b.height / 2.0;

    if p1x == p2x {
        if p1y > p2y {
            return (p1x, top_left_ay, p2x, bottom_left_by);
        } else if p1y < p2y {
            return (p1x, bottom_left_ay, p2x, top_left_by);
        } else {
            return (p1x, p1y, p2x, p2y);
        }
    }

    if p1y == p2y {
        if p1x > p2x {
            return (top_left_ax, p1y, top_right_bx, p2y);
        } else if p1x < p2x {
            return (top_right_ax, p1y, top_left_bx, p2y);
        } else {
            return (p1x, p1y, p2x, p2y);
        }
    }

    #[inline]
    fn get_cardinal_direction(slope: f64, slope_prime: f64, line: i32) -> i32 {
        if slope > slope_prime {
            line
        } else {
            1 + (line % 4)
        }
    }

    let slope_a = a.height / a.width;
    let slope_b = b.height / b.width;
    let slope_prime = (p2y - p1y) / (p2x - p1x);

    let mut ax = 0.0;
    let mut ay = 0.0;
    let mut bx = 0.0;
    let mut by = 0.0;
    let mut clip_a_found = false;
    let mut clip_b_found = false;

    if -slope_a == slope_prime {
        if p1x > p2x {
            ax = bottom_left_ax;
            ay = bottom_left_ay;
            clip_a_found = true;
        } else {
            ax = top_right_ax;
            ay = top_left_ay;
            clip_a_found = true;
        }
    } else if slope_a == slope_prime {
        if p1x > p2x {
            ax = top_left_ax;
            ay = top_left_ay;
            clip_a_found = true;
        } else {
            ax = bottom_right_ax;
            ay = bottom_left_ay;
            clip_a_found = true;
        }
    }

    if -slope_b == slope_prime {
        if p2x > p1x {
            bx = bottom_left_bx;
            by = bottom_left_by;
            clip_b_found = true;
        } else {
            bx = top_right_bx;
            by = top_left_by;
            clip_b_found = true;
        }
    } else if slope_b == slope_prime {
        if p2x > p1x {
            bx = top_left_bx;
            by = top_left_by;
            clip_b_found = true;
        } else {
            bx = bottom_right_bx;
            by = bottom_left_by;
            clip_b_found = true;
        }
    }

    if !(clip_a_found && clip_b_found) {
        let (card_a, card_b) = if p1x > p2x {
            if p1y > p2y {
                (
                    get_cardinal_direction(slope_a, slope_prime, 4),
                    get_cardinal_direction(slope_b, slope_prime, 2),
                )
            } else {
                (
                    get_cardinal_direction(-slope_a, slope_prime, 3),
                    get_cardinal_direction(-slope_b, slope_prime, 1),
                )
            }
        } else if p1y > p2y {
            (
                get_cardinal_direction(-slope_a, slope_prime, 1),
                get_cardinal_direction(-slope_b, slope_prime, 3),
            )
        } else {
            (
                get_cardinal_direction(slope_a, slope_prime, 2),
                get_cardinal_direction(slope_b, slope_prime, 4),
            )
        };

        if !clip_a_found {
            match card_a {
                1 => {
                    ay = top_left_ay;
                    ax = p1x + (-half_height_a) / slope_prime;
                }
                2 => {
                    ax = bottom_right_ax;
                    ay = p1y + half_width_a * slope_prime;
                }
                3 => {
                    ay = bottom_left_ay;
                    ax = p1x + half_height_a / slope_prime;
                }
                _ => {
                    ax = bottom_left_ax;
                    ay = p1y + (-half_width_a) * slope_prime;
                }
            }
        }

        if !clip_b_found {
            match card_b {
                1 => {
                    by = top_left_by;
                    bx = p2x + (-half_height_b) / slope_prime;
                }
                2 => {
                    bx = bottom_right_bx;
                    by = p2y + half_width_b * slope_prime;
                }
                3 => {
                    by = bottom_left_by;
                    bx = p2x + half_height_b / slope_prime;
                }
                _ => {
                    bx = bottom_left_bx;
                    by = p2y + (-half_width_b) * slope_prime;
                }
            }
        }
    }

    (ax, ay, bx, by)
}

fn calc_separation_amount(a: &SimNode, b: &SimNode, separation_buffer: f64) -> (f64, f64) {
    debug_assert!(rects_intersect(a, b));

    let (dir_x, dir_y) = decide_directions_for_overlapping_nodes(a, b);

    let mut overlap_x = a.right().min(b.right()) - a.left.max(b.left);
    let mut overlap_y = a.bottom().min(b.bottom()) - a.top.max(b.top);

    if (a.left <= b.left) && (a.right() >= b.right()) {
        overlap_x += (b.left - a.left).min(a.right() - b.right());
    } else if (b.left <= a.left) && (b.right() >= a.right()) {
        overlap_x += (a.left - b.left).min(b.right() - a.right());
    }
    if (a.top <= b.top) && (a.bottom() >= b.bottom()) {
        overlap_y += (b.top - a.top).min(a.bottom() - b.bottom());
    } else if (b.top <= a.top) && (b.bottom() >= a.bottom()) {
        overlap_y += (a.top - b.top).min(b.bottom() - a.bottom());
    }

    let mut slope = ((b.center_y() - a.center_y()) / (b.center_x() - a.center_x())).abs();
    if (b.center_y() == a.center_y()) && (b.center_x() == a.center_x()) {
        slope = 1.0;
    }

    let mut move_by_y = slope * overlap_x;
    let mut move_by_x = overlap_y / slope;
    if overlap_x < move_by_x {
        move_by_x = overlap_x;
    } else {
        move_by_y = overlap_y;
    }

    let dx = -(dir_x as f64) * ((move_by_x / 2.0) + separation_buffer);
    let dy = -(dir_y as f64) * ((move_by_y / 2.0) + separation_buffer);
    (dx, dy)
}

fn decide_directions_for_overlapping_nodes(a: &SimNode, b: &SimNode) -> (i32, i32) {
    let dir_x = if a.center_x() < b.center_x() { -1 } else { 1 };
    let dir_y = if a.center_y() < b.center_y() { -1 } else { 1 };
    (dir_x, dir_y)
}

#[cfg(test)]
mod tests {
    use super::{IndexedEdge, IndexedNode, SimGraph, layout_indexed};

    fn assert_close(a: f64, b: f64) {
        let eps = 1e-3;
        assert!((a - b).abs() <= eps, "expected {a} ~= {b} (eps={eps})");
    }

    #[test]
    fn basic_three_node_tree_matches_upstream_positions() {
        // Oracle: cytoscape-cose-bilkent@4.1.0 + cytoscape@3.33.1 (Mermaid 11.12.3),
        // with the same node dimensions.
        //
        // This corresponds to `fixtures/upstream-svgs/mindmap/basic.svg`.
        let nodes = vec![
            IndexedNode {
                width: 69.734375,
                height: 34.0,
                x: 0.0,
                y: 0.0,
            },
            IndexedNode {
                width: 48.40625,
                height: 34.0,
                x: 0.0,
                y: 0.0,
            },
            IndexedNode {
                width: 48.921875,
                height: 34.0,
                x: 0.0,
                y: 0.0,
            },
        ];
        let edges = vec![IndexedEdge { a: 0, b: 1 }, IndexedEdge { a: 0, b: 2 }];

        let out = layout_indexed(&nodes, &edges, &Default::default()).expect("layout");

        assert_eq!(out.len(), 3);
        assert_close(out[0].x, 152.283539);
        assert_close(out[0].y, 32.0);
        assert_close(out[1].x, 264.848328);
        assert_close(out[1].y, 32.0);
        assert_close(out[2].x, 39.460938);
        assert_close(out[2].y, 32.0);
    }

    #[test]
    fn layout_indexed_handles_deep_tree_radial_layout_with_small_stack() {
        const DEPTH: usize = 2_048;
        let nodes = vec![
            IndexedNode {
                width: 48.0,
                height: 24.0,
                x: 0.0,
                y: 0.0,
            };
            DEPTH
        ];
        let edges = (1..DEPTH)
            .map(|idx| IndexedEdge { a: idx - 1, b: idx })
            .collect::<Vec<_>>();

        let handle = std::thread::Builder::new()
            .name("cose-bilkent-deep-tree-radial-layout".to_string())
            .stack_size(64 * 1024)
            .spawn(move || {
                let out = layout_indexed(&nodes, &edges, &Default::default())
                    .expect("deep tree layout should not depend on recursive stack growth");
                assert_eq!(out.len(), DEPTH);
                assert!(
                    out.iter()
                        .all(|point| point.x.is_finite() && point.y.is_finite()),
                    "deep tree layout should emit finite positions"
                );
            })
            .expect("spawn COSE-Bilkent deep tree layout test");
        handle
            .join()
            .expect("COSE-Bilkent deep tree layout should finish without stack overflow");
    }

    #[test]
    fn basic_three_node_tree_radial_init_has_equal_y() {
        let nodes = vec![
            IndexedNode {
                width: 69.734375,
                height: 34.0,
                x: 0.0,
                y: 0.0,
            },
            IndexedNode {
                width: 48.40625,
                height: 34.0,
                x: 0.0,
                y: 0.0,
            },
            IndexedNode {
                width: 48.921875,
                height: 34.0,
                x: 0.0,
                y: 0.0,
            },
        ];
        let edges = vec![IndexedEdge { a: 0, b: 1 }, IndexedEdge { a: 0, b: 2 }];

        let mut sim = SimGraph::from_indexed(&nodes, &edges);
        let forest = sim.get_flat_forest();
        assert_eq!(forest.len(), 1);
        sim.position_nodes_radially(&forest);

        let y0 = sim.nodes[0].center_y();
        for (i, n) in sim.nodes.iter().enumerate() {
            assert!(
                n.center_y() == y0,
                "radial init center_y mismatch: node[{i}] y={} vs y0={}",
                n.center_y(),
                y0
            );
        }
    }

    #[test]
    fn basic_three_node_tree_tick1_keeps_equal_y() {
        let nodes = vec![
            IndexedNode {
                width: 69.734375,
                height: 34.0,
                x: 0.0,
                y: 0.0,
            },
            IndexedNode {
                width: 48.40625,
                height: 34.0,
                x: 0.0,
                y: 0.0,
            },
            IndexedNode {
                width: 48.921875,
                height: 34.0,
                x: 0.0,
                y: 0.0,
            },
        ];
        let edges = vec![IndexedEdge { a: 0, b: 1 }, IndexedEdge { a: 0, b: 2 }];

        let mut sim = SimGraph::from_indexed(&nodes, &edges);
        assert_eq!(sim.edges.len(), 2);
        assert_eq!(sim.edges[0].a, 0);
        assert_eq!(sim.edges[0].b, 1);
        assert_eq!(sim.edges[1].a, 0);
        assert_eq!(sim.edges[1].b, 2);
        let forest = sim.get_flat_forest();
        sim.position_nodes_radially(&forest);

        let y0 = sim.nodes[0].center_y();
        for n in &sim.nodes {
            assert_eq!(n.center_y(), y0);
        }
        // Mirror the spring embedder's tick#1 grid rebuild (should not affect geometry).
        sim.update_grid(2.0 * super::SimGraph::DEFAULT_EDGE_LENGTH);
        // Sanity: for a horizontal arrangement, clipping points should preserve equal y.
        {
            let (t1x, t1y, s1x, s1y, ov1) =
                super::rect_intersection_points(&sim.nodes[1], &sim.nodes[0]);
            assert!(!ov1);
            assert_eq!(
                t1y, s1y,
                "edge(0->1) clip y differs: t=({t1x},{t1y}) s=({s1x},{s1y})"
            );
            let (t2x, t2y, s2x, s2y, ov2) =
                super::rect_intersection_points(&sim.nodes[2], &sim.nodes[0]);
            assert!(!ov2);
            assert_eq!(
                t2y, s2y,
                "edge(0->2) clip y differs: t=({t2x},{t2y}) s=({s2x},{s2y})"
            );
        }
        sim.run_single_spring_tick_flat_graph();

        for (i, n) in sim.nodes.iter().enumerate() {
            assert!(
                n.center_y() == y0,
                "tick1 center_y mismatch: node[{i}] y={} vs y0={}",
                n.center_y(),
                y0
            );
        }
    }

    #[test]
    fn find_center_of_tree_matches_layout_base_buggy_semantics() {
        // Star-shaped tree: node 0 connected to all others.
        // With the upstream `findCenterOfTree()` bug (removing from `list` while iterating over it),
        // the result depends on insertion order and ends up not being the actual tree center.
        let n = 21usize;
        let nodes: Vec<IndexedNode> = (0..n)
            .map(|_| IndexedNode {
                width: 10.0,
                height: 10.0,
                x: 0.0,
                y: 0.0,
            })
            .collect();
        let edges: Vec<IndexedEdge> = (1..n).map(|i| IndexedEdge { a: 0, b: i }).collect();

        let sim = SimGraph::from_indexed(&nodes, &edges);
        let list: Vec<usize> = (0..n).collect();

        assert_eq!(sim.find_center_of_tree(&list), 7);
    }
}
