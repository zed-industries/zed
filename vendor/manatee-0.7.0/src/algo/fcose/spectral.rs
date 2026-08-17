#![allow(clippy::needless_range_loop)]

use rustc_hash::FxHashMap;

use super::{SimEdge, SimNode, XorShift64Star};

const INFINITY_HOPS: f64 = 100_000_000.0;
const SMALL: f64 = 1e-9;

const DEFAULT_SAMPLE_SIZE: usize = 25;
const DEFAULT_PI_TOL: f64 = 1e-7;

const MAX_POWER_ITERATIONS: usize = 10_000;

pub(super) fn apply_spectral_start_positions(
    nodes: &mut [SimNode],
    edges: &[SimEdge],
    compound_parent: &[Option<usize>],
    compound_ids_in_order: &[usize],
    node_separation: f64,
    rng: &mut XorShift64Star,
) -> bool {
    if nodes.is_empty() {
        return false;
    }

    let n_real = nodes.len();
    let (adjacency, node_size) =
        build_transformed_adjacency(nodes, edges, compound_parent, compound_ids_in_order);
    if node_size <= 1 {
        return false;
    }

    // Upstream skips spectral when the transformed graph has 1 or 2 nodes.
    if node_size == 2 {
        if n_real != 2 {
            return false;
        }
        // Place the second node to the right of the first node using an ideal edge length.
        // This matches upstream spectral.js' fallback path.
        let ideal = edges
            .iter()
            .map(|e| e.ideal_length)
            .find(|v| v.is_finite() && *v > 0.0)
            .unwrap_or(50.0);

        let (first, second) = (&nodes[0], &nodes[1]);
        let x1 = first.center_x();
        let y1 = first.center_y();
        let x2 = x1 + first.width / 2.0 + second.width / 2.0 + ideal;

        nodes[1].left = x2 - nodes[1].width / 2.0;
        nodes[1].top = y1 - nodes[1].height / 2.0;
        return true;
    }

    let sample_size = node_size.min(DEFAULT_SAMPLE_SIZE);
    if sample_size <= 1 {
        return false;
    }

    // Column sampling matrix (squared shortest-path distances).
    // Keep this as a plain Vec-backed matrix to match upstream JS operation order more closely.
    let mut c: Vec<Vec<f64>> = vec![vec![0.0; sample_size]; node_size];
    let mut samples: Vec<usize> = vec![0; sample_size];
    let mut min_dist: Vec<f64> = vec![INFINITY_HOPS; node_size];

    // Greedy sampling (Mermaid default): pick a random first sample, then repeatedly pick the node
    // that maximizes the minimum distance to the already-sampled set.
    //
    // Note: any "seed offset" to match upstream Mermaid baseline RNG consumption should be
    // applied *outside* spectral, at the layout invocation level, so reruns (`layout.run()` twice)
    // do not double-advance the RNG stream.
    let mut sample = rng.next_usize(node_size);
    min_dist.fill(INFINITY_HOPS);
    for (col, slot) in samples.iter_mut().enumerate().take(sample_size) {
        *slot = sample;
        sample = bfs_fill_column(
            sample,
            col,
            &adjacency,
            node_separation,
            &mut c,
            Some(&mut min_dist),
        );
    }

    // Square distances for C.
    for i in 0..node_size {
        for j in 0..sample_size {
            let v = c[i][j];
            c[i][j] = v * v;
        }
    }

    // PHI is the intersection of sampled rows/columns.
    let mut phi: Vec<Vec<f64>> = vec![vec![0.0; sample_size]; sample_size];
    for i in 0..sample_size {
        for j in 0..sample_size {
            phi[i][j] = c[samples[j]][i];
        }
    }

    let inv = match regularized_inverse_from_svd(&phi) {
        Some(m) => m,
        None => return false,
    };

    let (x_coords, y_coords) = match power_iteration(rng, &c, &inv, DEFAULT_PI_TOL) {
        Some(v) => v,
        None => return false,
    };
    if std::env::var("MANATEE_FCOSE_DEBUG_SPECTRAL_DUMP")
        .ok()
        .as_deref()
        == Some("1")
    {
        eprintln!("[manatee-fcose-spectral-dump] x_coords={x_coords:?}");
        eprintln!("[manatee-fcose-spectral-dump] y_coords={y_coords:?}");
    }

    for i in 0..n_real {
        let x = x_coords[i];
        let y = y_coords[i];
        if !(x.is_finite() && y.is_finite()) {
            return false;
        }
        nodes[i].left = x - nodes[i].width / 2.0;
        nodes[i].top = y - nodes[i].height / 2.0;
    }

    if std::env::var("MANATEE_FCOSE_DEBUG_SPECTRAL")
        .ok()
        .as_deref()
        == Some("1")
    {
        let mut min_x = f64::INFINITY;
        let mut min_y = f64::INFINITY;
        let mut max_x = f64::NEG_INFINITY;
        let mut max_y = f64::NEG_INFINITY;
        for n in nodes.iter().take(n_real) {
            min_x = min_x.min(n.center_x());
            min_y = min_y.min(n.center_y());
            max_x = max_x.max(n.center_x());
            max_y = max_y.max(n.center_y());
        }
        eprintln!(
            "[manatee-fcose-spectral] n_real={} transformed_n={} sample_size={} x=[{:.3},{:.3}] y=[{:.3},{:.3}]",
            n_real, node_size, sample_size, min_x, max_x, min_y, max_y
        );
    }

    if std::env::var("MANATEE_FCOSE_DEBUG_SPECTRAL_DUMP")
        .ok()
        .as_deref()
        == Some("1")
    {
        eprintln!("[manatee-fcose-spectral-dump] node_size={node_size} sample_size={sample_size}");
        eprintln!("[manatee-fcose-spectral-dump] samples={samples:?}");
        eprintln!("[manatee-fcose-spectral-dump] phi:");
        for row in &phi {
            eprintln!("[manatee-fcose-spectral-dump]   {:?}", row);
        }
        eprintln!("[manatee-fcose-spectral-dump] inv:");
        for row in &inv {
            eprintln!("[manatee-fcose-spectral-dump]   {:?}", row);
        }
    }

    true
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum ElemKey {
    Leaf(usize),
    Compound(usize),
}

fn build_transformed_adjacency(
    nodes: &[SimNode],
    edges: &[SimEdge],
    compound_parent: &[Option<usize>],
    compound_ids_in_order: &[usize],
) -> (Vec<Vec<usize>>, usize) {
    let n_real = nodes.len();

    // Transformed graph starts with all real (childless) nodes, then adds dummy nodes created by
    // `aux.connectComponents(...)` (top-level first, then for each parent in insertion order).
    let mut adjacency: Vec<Vec<usize>> = vec![Vec::new(); n_real];
    for e in edges {
        if e.a < n_real && e.b < n_real {
            adjacency[e.a].push(e.b);
            adjacency[e.b].push(e.a);
        }
    }
    for neigh in &mut adjacency {
        neigh.sort_unstable();
        neigh.dedup();
    }

    let leaf_deg: Vec<usize> = adjacency.iter().map(|v| v.len()).collect();

    // Match upstream spectral.js ordering:
    // - compound ids follow the Cytoscape insertion order (Mermaid adds groups first, then services)
    // - we keep that order as provided by the caller (derived from `graph.compounds`)
    let compound_ids: Vec<usize> = compound_ids_in_order
        .iter()
        .copied()
        .filter(|id| *id < compound_parent.len())
        .collect();
    let mut compound_id_to_ix: Vec<Option<usize>> = vec![None; compound_parent.len()];
    for (ix, &id) in compound_ids.iter().enumerate() {
        compound_id_to_ix[id] = Some(ix);
    }

    let mut compound_parent_ix: Vec<Option<usize>> = vec![None; compound_ids.len()];
    for (ix, &compound_id) in compound_ids.iter().enumerate() {
        let parent_ix = compound_parent
            .get(compound_id)
            .copied()
            .flatten()
            .and_then(|parent_id| compound_id_to_ix.get(parent_id).copied().flatten());
        compound_parent_ix[ix] = parent_ix;
    }

    let mut compound_children: Vec<Vec<usize>> = vec![Vec::new(); compound_ids.len()];
    for child_ix in 0..compound_ids.len() {
        if let Some(parent_ix) = compound_parent_ix[child_ix] {
            // Preserve Cytoscape insertion order (child compounds are appended in `compound_ids`
            // order, which is itself insertion order).
            compound_children[parent_ix].push(child_ix);
        }
    }

    let mut leaf_chain: Vec<Vec<usize>> = vec![Vec::new(); n_real];
    let mut leaf_immediate_parent: Vec<Option<usize>> = vec![None; n_real];
    let mut leaf_root_compound: Vec<Option<usize>> = vec![None; n_real];
    for i in 0..n_real {
        let mut cur = nodes[i].parent;
        while let Some(cid) = cur {
            let Some(cix) = compound_id_to_ix.get(cid).copied().flatten() else {
                break;
            };
            leaf_chain[i].push(cix);
            cur = compound_parent.get(cid).copied().flatten();
        }
        leaf_immediate_parent[i] = leaf_chain[i].first().copied();
        leaf_root_compound[i] = leaf_chain[i].last().copied();
    }

    // Track which compounds are actually referenced by any node so we don't treat unrelated
    // compound definitions as layout elements.
    let mut compound_used: Vec<bool> = vec![false; compound_ids.len()];
    for chain in &leaf_chain {
        for &cix in chain {
            compound_used[cix] = true;
        }
    }

    // Representative childless node per compound: mirror `spectral.js`'s `parentChildMap`.
    //
    // Important: upstream does *not* pick a global minimum-degree leaf from all descendants.
    // Instead, it descends along `children.nodes()[0]` while the current level has no childless
    // nodes, then picks the minimum-degree leaf among the childless nodes at that level.
    let mut compound_repr_leaf: Vec<Option<usize>> = vec![None; compound_ids.len()];
    for cix in 0..compound_ids.len() {
        if !compound_used[cix] {
            continue;
        }
        let mut current = cix;
        loop {
            // Direct childless nodes at this level.
            let mut best_leaf: Option<usize> = None;
            let mut best_deg: usize = usize::MAX;
            for leaf in 0..n_real {
                if leaf_immediate_parent[leaf] != Some(current) {
                    continue;
                }
                let deg = leaf_deg[leaf];
                match best_leaf {
                    None => {
                        best_leaf = Some(leaf);
                        best_deg = deg;
                    }
                    Some(_) if deg < best_deg => {
                        best_leaf = Some(leaf);
                        best_deg = deg;
                    }
                    _ => {}
                }
            }
            if best_leaf.is_some() {
                compound_repr_leaf[cix] = best_leaf;
                break;
            }

            // No direct leaves: descend into the first compound child (in insertion order).
            let mut next_compound: Option<usize> = None;
            for &child in &compound_children[current] {
                if compound_used.get(child).copied().unwrap_or(false) {
                    next_compound = Some(child);
                    break;
                }
            }
            let Some(next) = next_compound else {
                break;
            };
            current = next;
        }
    }

    let elem_degree = |e: ElemKey| -> usize {
        match e {
            ElemKey::Leaf(i) => leaf_deg.get(i).copied().unwrap_or(0),
            // In upstream Cytoscape, compound nodes do not have direct incident edges.
            ElemKey::Compound(_) => 0,
        }
    };

    let elem_repr_leaf = |e: ElemKey| -> Option<usize> {
        match e {
            ElemKey::Leaf(i) => Some(i),
            ElemKey::Compound(cix) => compound_repr_leaf.get(cix).copied().flatten(),
        }
    };

    // Mirror `spectral.js` preprocessing:
    // - `aux.connectComponents(cy, eles, aux.getTopMostNodes(nodes), dummyNodes);`
    // - `parentNodes.forEach(ele => aux.connectComponents(... topMostNodes(ele.descendants()) ...));`

    // Top-level connectComponents.
    {
        let mut top_most: Vec<ElemKey> = Vec::new();
        for cix in 0..compound_ids.len() {
            if compound_used[cix] && compound_parent_ix[cix].is_none() {
                top_most.push(ElemKey::Compound(cix));
            }
        }
        for leaf in 0..n_real {
            if leaf_immediate_parent[leaf].is_none() {
                top_most.push(ElemKey::Leaf(leaf));
            }
        }

        add_dummy_for_scope(
            &mut adjacency,
            edges,
            &top_most,
            |leaf| {
                Some(
                    leaf_root_compound[leaf]
                        .map(ElemKey::Compound)
                        .unwrap_or(ElemKey::Leaf(leaf)),
                )
            },
            &elem_degree,
            &elem_repr_leaf,
        );
    }

    // Per-compound connectComponents (in parent insertion order).
    for scope_cix in 0..compound_ids.len() {
        if !compound_used[scope_cix] {
            continue;
        }

        // `aux.getTopMostNodes(ele.descendants())` returns the immediate children of the parent.
        let mut top_most: Vec<ElemKey> = Vec::new();
        for &child in &compound_children[scope_cix] {
            if compound_used.get(child).copied().unwrap_or(false) {
                top_most.push(ElemKey::Compound(child));
            }
        }
        for leaf in 0..n_real {
            if leaf_immediate_parent[leaf] == Some(scope_cix) {
                top_most.push(ElemKey::Leaf(leaf));
            }
        }

        if top_most.len() <= 1 {
            continue;
        }

        add_dummy_for_scope(
            &mut adjacency,
            edges,
            &top_most,
            |leaf| {
                // Restrict traversal to this compound's descendants (neighbors outside the scope
                // are ignored because they don't intersect `topMostNodes` in upstream).
                if !leaf_chain
                    .get(leaf)
                    .is_some_and(|chain| chain.contains(&scope_cix))
                {
                    return None;
                }
                map_leaf_to_scope_top_most(scope_cix, leaf, &leaf_immediate_parent, &leaf_chain)
            },
            &elem_degree,
            &elem_repr_leaf,
        );
    }

    for neigh in &mut adjacency {
        neigh.sort_unstable();
        neigh.dedup();
    }

    let node_size = adjacency.len();
    (adjacency, node_size)
}

fn map_leaf_to_scope_top_most(
    scope_cix: usize,
    leaf: usize,
    leaf_immediate_parent: &[Option<usize>],
    leaf_chain: &[Vec<usize>],
) -> Option<ElemKey> {
    if leaf_immediate_parent.get(leaf).copied().flatten() == Some(scope_cix) {
        return Some(ElemKey::Leaf(leaf));
    }

    let chain = leaf_chain.get(leaf)?;
    let mut pos: Option<usize> = None;
    for (i, &cix) in chain.iter().enumerate() {
        if cix == scope_cix {
            pos = Some(i);
            break;
        }
    }
    let pos = pos?;
    if pos == 0 {
        // Immediate parent handled above; treat as a direct leaf child.
        return Some(ElemKey::Leaf(leaf));
    }
    Some(ElemKey::Compound(chain[pos - 1]))
}

fn add_dummy_for_scope(
    transformed_adj: &mut Vec<Vec<usize>>,
    edges: &[SimEdge],
    top_most: &[ElemKey],
    mut map_leaf_to_elem: impl FnMut(usize) -> Option<ElemKey>,
    elem_degree: &impl Fn(ElemKey) -> usize,
    elem_repr_leaf: &impl Fn(ElemKey) -> Option<usize>,
) {
    if top_most.len() <= 1 {
        return;
    }

    let mut elem_to_idx: FxHashMap<ElemKey, usize> = FxHashMap::default();
    for (i, &e) in top_most.iter().enumerate() {
        elem_to_idx.insert(e, i);
    }

    let mut elem_adj: Vec<Vec<usize>> = vec![Vec::new(); top_most.len()];
    for e in edges {
        let Some(a) = map_leaf_to_elem(e.a) else {
            continue;
        };
        let Some(b) = map_leaf_to_elem(e.b) else {
            continue;
        };
        if a == b {
            continue;
        }
        let Some(&ia) = elem_to_idx.get(&a) else {
            continue;
        };
        let Some(&ib) = elem_to_idx.get(&b) else {
            continue;
        };
        elem_adj[ia].push(ib);
        elem_adj[ib].push(ia);
    }
    for neigh in &mut elem_adj {
        neigh.sort_unstable();
        neigh.dedup();
    }

    let components = connected_components(&elem_adj);
    if components.len() <= 1 {
        return;
    }

    let dummy_idx = transformed_adj.len();
    transformed_adj.push(Vec::new());

    for comp in components {
        // Mirror `aux.connectComponents(...)` selection: pick the minimum-degree top-most node
        // in the component, and keep the first one on ties (JS uses `<`, not `<=`).
        let mut best = top_most[comp[0]];
        let mut best_deg = elem_degree(best);
        for &i in comp.iter().skip(1) {
            let e = top_most[i];
            let deg = elem_degree(e);
            if deg < best_deg {
                best = e;
                best_deg = deg;
            }
        }

        let Some(rep_leaf) = elem_repr_leaf(best) else {
            continue;
        };
        if rep_leaf >= transformed_adj.len() {
            continue;
        }
        transformed_adj[dummy_idx].push(rep_leaf);
        transformed_adj[rep_leaf].push(dummy_idx);
    }
}

fn connected_components(adjacency: &[Vec<usize>]) -> Vec<Vec<usize>> {
    let n = adjacency.len();
    let mut visited = vec![false; n];
    let mut out: Vec<Vec<usize>> = Vec::new();
    let mut q: std::collections::VecDeque<usize> = std::collections::VecDeque::new();

    for start in 0..n {
        if visited[start] {
            continue;
        }
        visited[start] = true;
        q.push_back(start);
        let mut comp: Vec<usize> = Vec::new();

        while let Some(v) = q.pop_front() {
            comp.push(v);
            for &u in &adjacency[v] {
                if !visited[u] {
                    visited[u] = true;
                    q.push_back(u);
                }
            }
        }

        out.push(comp);
    }

    out
}

fn bfs_fill_column(
    pivot: usize,
    col: usize,
    adjacency: &[Vec<usize>],
    node_separation: f64,
    c: &mut [Vec<f64>],
    mut min_dist: Option<&mut [f64]>,
) -> usize {
    let node_size = adjacency.len();
    let mut dist: Vec<i32> = vec![-1; node_size];
    let mut q: std::collections::VecDeque<usize> = std::collections::VecDeque::new();

    dist[pivot] = 0;
    q.push_back(pivot);

    while let Some(v) = q.pop_front() {
        for &u in &adjacency[v] {
            if dist[u] == -1 {
                dist[u] = dist[v].saturating_add(1);
                q.push_back(u);
            }
        }
    }

    let mut max_dist = 0.0;
    let mut max_idx = 0usize;
    for i in 0..node_size {
        let d = if dist[i] == -1 {
            INFINITY_HOPS
        } else {
            (dist[i] as f64) * node_separation
        };
        c[i][col] = d;

        if let Some(min_dist) = min_dist.as_deref_mut() {
            if d < min_dist[i] {
                min_dist[i] = d;
            }
            if min_dist[i] > max_dist {
                max_dist = min_dist[i];
                max_idx = i;
            }
        }
    }

    if min_dist.is_some() { max_idx } else { pivot }
}

#[derive(Debug, Clone)]
pub(super) struct SvdResult {
    pub(super) u: Vec<Vec<f64>>,
    pub(super) v: Vec<Vec<f64>>,
    pub(super) s: Vec<f64>,
}

// Port of layout-base `util/SVD.js` (JamaJS-derived) + `spectral.js` regularized inverse.
// This avoids relying on external linear algebra implementations whose numeric behavior can
// diverge enough to change the spectral basis on symmetric graphs (which cascades into different
// FCoSE results and parity-root viewports).
fn regularized_inverse_from_svd(phi: &[Vec<f64>]) -> Option<Vec<Vec<f64>>> {
    let n = phi.len();
    if n == 0 {
        return None;
    }
    if phi.iter().any(|r| r.len() != n) {
        return None;
    }

    let svd = svd_jama(phi)?;
    if svd.s.is_empty() {
        return None;
    }

    // layout-base spectral.js:
    // max_s = q[0]^3 where q is sorted descending by the SVD routine.
    let q0 = svd.s[0];
    let max_s = q0 * q0 * q0;

    // Diagonal regularization values (a_Sig[i][i]).
    let mut sig_diag: Vec<f64> = vec![0.0; n];
    for i in 0..n {
        let qi = svd.s.get(i).copied().unwrap_or(0.0);
        let qi2 = qi * qi;
        if qi2 == 0.0 {
            sig_diag[i] = 0.0;
            continue;
        }
        sig_diag[i] = qi / (qi2 + (max_s / qi2));
    }

    // INV = V * Sig * U^T
    let mut inv: Vec<Vec<f64>> = vec![vec![0.0; n]; n];
    for i in 0..n {
        for j in 0..n {
            let mut sum = 0.0;
            for k in 0..n {
                sum += svd.v[i][k] * sig_diag[k] * svd.u[j][k];
            }
            inv[i][j] = sum;
        }
    }
    Some(inv)
}

fn power_iteration(
    rng: &mut XorShift64Star,
    c: &[Vec<f64>],
    inv: &[Vec<f64>],
    pi_tol: f64,
) -> Option<(Vec<f64>, Vec<f64>)> {
    let n = c.len();
    if n == 0 {
        return None;
    }
    let sample_size = c[0].len();
    if inv.len() != sample_size || inv.iter().any(|r| r.len() != sample_size) {
        return None;
    }

    // Match upstream `spectral.js` RNG consumption order:
    //
    // ```
    // for(i=0; i<nodeSize; i++){
    //   Y1[i] = Math.random();
    //   Y2[i] = Math.random();
    // }
    // ```
    //
    // Interleaving matters on symmetric graphs: consuming all `Y1` values first and then all
    // `Y2` values yields a different RNG stream split and can rotate/reflect the spectral basis,
    // which cascades into different FCoSE results.
    let mut y1: Vec<f64> = vec![0.0; n];
    let mut y2: Vec<f64> = vec![0.0; n];
    for i in 0..n {
        y1[i] = rng.next_f64_unit();
        y2[i] = rng.next_f64_unit();
    }
    if std::env::var("MANATEE_FCOSE_DEBUG_SPECTRAL_DUMP")
        .ok()
        .as_deref()
        == Some("1")
    {
        eprintln!("[manatee-fcose-spectral-dump] y1_init={y1:?}");
        eprintln!("[manatee-fcose-spectral-dump] y2_init={y2:?}");
    }
    normalize_in_place(&mut y1);
    normalize_in_place(&mut y2);

    let (v1, theta1) = dominant_eigenvector(c, inv, y1, pi_tol)?;
    let (v2, theta2) = second_eigenvector(c, inv, &v1, y2, pi_tol)?;

    let s1 = theta1.abs().sqrt();
    let s2 = theta2.abs().sqrt();
    let x: Vec<f64> = v1.iter().map(|v| v * s1).collect();
    let y: Vec<f64> = v2.iter().map(|v| v * s2).collect();
    Some((x, y))
}

fn dominant_eigenvector(
    c: &[Vec<f64>],
    inv: &[Vec<f64>],
    mut y: Vec<f64>,
    pi_tol: f64,
) -> Option<(Vec<f64>, f64)> {
    let mut previous = SMALL;
    let mut theta = 0.0;

    for _ in 0..MAX_POWER_ITERATIONS {
        let v = y.clone();
        let t = mult_gamma(&v);
        let t = mult_l(&t, c, inv);
        let mut next = mult_gamma(&t);
        theta = dot(&v, &next);
        normalize_in_place(&mut next);

        let current = dot(&v, &next);
        let ratio = (current / previous).abs();

        y = next;
        if ratio <= 1.0 + pi_tol && ratio >= 1.0 {
            return Some((y, theta));
        }
        previous = current;
        if previous.abs() < SMALL {
            previous = SMALL;
        }
    }

    Some((y, theta))
}

fn second_eigenvector(
    c: &[Vec<f64>],
    inv: &[Vec<f64>],
    v1: &[f64],
    mut y: Vec<f64>,
    pi_tol: f64,
) -> Option<(Vec<f64>, f64)> {
    let mut previous = SMALL;
    let mut theta = 0.0;

    for _ in 0..MAX_POWER_ITERATIONS {
        let mut v = y.clone();
        let proj = dot(v1, &v);
        for i in 0..v.len() {
            v[i] -= v1[i] * proj;
        }

        let t = mult_gamma(&v);
        let t = mult_l(&t, c, inv);
        let mut next = mult_gamma(&t);
        theta = dot(&v, &next);
        normalize_in_place(&mut next);

        let current = dot(&v, &next);
        let ratio = (current / previous).abs();

        y = next;
        if ratio <= 1.0 + pi_tol && ratio >= 1.0 {
            return Some((y, theta));
        }
        previous = current;
        if previous.abs() < SMALL {
            previous = SMALL;
        }
    }

    Some((y, theta))
}

fn dot(a: &[f64], b: &[f64]) -> f64 {
    let n = a.len().min(b.len());
    let mut sum = 0.0;
    for i in 0..n {
        sum += a[i] * b[i];
    }
    sum
}

fn mult_gamma(v: &[f64]) -> Vec<f64> {
    let n = v.len();
    if n == 0 {
        return Vec::new();
    }
    let mut sum = 0.0;
    for &x in v {
        sum += x;
    }
    let mean = sum / (n as f64);
    let mut out = vec![0.0; n];
    for i in 0..n {
        out[i] = v[i] - mean;
    }
    out
}

fn mult_l(v: &[f64], c: &[Vec<f64>], inv: &[Vec<f64>]) -> Vec<f64> {
    // layout-base `Matrix.multL`:
    // result = -0.5 * C * INV * C^T * v
    let node_size = c.len();
    if node_size == 0 {
        return Vec::new();
    }
    let sample_size = c[0].len();

    let mut temp1 = vec![0.0; sample_size];
    for i in 0..sample_size {
        let mut sum = 0.0;
        for j in 0..node_size {
            sum += -0.5 * c[j][i] * v[j];
        }
        temp1[i] = sum;
    }

    let mut temp2 = vec![0.0; sample_size];
    for i in 0..sample_size {
        let mut sum = 0.0;
        for j in 0..sample_size {
            sum += inv[i][j] * temp1[j];
        }
        temp2[i] = sum;
    }

    let mut out = vec![0.0; node_size];
    for i in 0..node_size {
        let mut sum = 0.0;
        for j in 0..sample_size {
            sum += c[i][j] * temp2[j];
        }
        out[i] = sum;
    }
    out
}

fn normalize_in_place(v: &mut [f64]) {
    let mut sum_sq = 0.0;
    for &x in v.iter() {
        sum_sq += x * x;
    }
    let norm = sum_sq.sqrt();
    if norm.is_finite() && norm > 0.0 {
        for x in v.iter_mut() {
            *x /= norm;
        }
    }
}

fn svd_hypot(a: f64, b: f64) -> f64 {
    // layout-base `SVD.hypot`.
    if a.abs() > b.abs() {
        let r = b / a;
        a.abs() * (1.0 + r * r).sqrt()
    } else if b != 0.0 {
        let r = a / b;
        b.abs() * (1.0 + r * r).sqrt()
    } else {
        0.0
    }
}

pub(super) fn svd_jama(a_in: &[Vec<f64>]) -> Option<SvdResult> {
    let m = a_in.len();
    if m == 0 {
        return None;
    }
    let n = a_in[0].len();
    if n == 0 || a_in.iter().any(|r| r.len() != n) {
        return None;
    }

    let mut a: Vec<Vec<f64>> = a_in.to_vec();

    let nu = m.min(n);
    let mut s: Vec<f64> = vec![0.0; (m + 1).min(n)];
    let mut u: Vec<Vec<f64>> = vec![vec![0.0; nu]; m];
    let mut v: Vec<Vec<f64>> = vec![vec![0.0; n]; n];
    let mut e: Vec<f64> = vec![0.0; n];
    let mut work: Vec<f64> = vec![0.0; m];

    let wantu = true;
    let wantv = true;

    let nct = (m.saturating_sub(1)).min(n);
    let nrt = (n.saturating_sub(2)).min(m).max(0);

    let k_max = nct.max(nrt);
    for k in 0..k_max {
        if k < nct {
            s[k] = 0.0;
            for i in k..m {
                s[k] = svd_hypot(s[k], a[i][k]);
            }
            if s[k] != 0.0 {
                if a[k][k] < 0.0 {
                    s[k] = -s[k];
                }
                for i in k..m {
                    a[i][k] /= s[k];
                }
                a[k][k] += 1.0;
            }
            s[k] = -s[k];
        }

        for j in (k + 1)..n {
            if k < nct && s[k] != 0.0 {
                let mut t = 0.0;
                for i in k..m {
                    t += a[i][k] * a[i][j];
                }
                t = -t / a[k][k];
                for i in k..m {
                    a[i][j] += t * a[i][k];
                }
            }
            e[j] = a[k][j];
        }

        if wantu && k < nct {
            for i in k..m {
                u[i][k] = a[i][k];
            }
        }

        if k < nrt {
            e[k] = 0.0;
            for i in (k + 1)..n {
                e[k] = svd_hypot(e[k], e[i]);
            }
            if e[k] != 0.0 {
                if e[k + 1] < 0.0 {
                    e[k] = -e[k];
                }
                for i in (k + 1)..n {
                    e[i] /= e[k];
                }
                e[k + 1] += 1.0;
            }
            e[k] = -e[k];

            if (k + 1) < m && e[k] != 0.0 {
                for i in (k + 1)..m {
                    work[i] = 0.0;
                }
                for j in (k + 1)..n {
                    for i in (k + 1)..m {
                        work[i] += e[j] * a[i][j];
                    }
                }
                for j in (k + 1)..n {
                    let t = -e[j] / e[k + 1];
                    for i in (k + 1)..m {
                        a[i][j] += t * work[i];
                    }
                }
            }

            if wantv {
                for i in (k + 1)..n {
                    v[i][k] = e[i];
                }
            }
        }
    }

    let p = n.min(m + 1);
    if nct < n {
        s[nct] = a[nct][nct];
    }
    if m < p {
        s[p - 1] = 0.0;
    }
    if (nrt + 1) < p {
        e[nrt] = a[nrt][p - 1];
    }
    e[p - 1] = 0.0;

    if wantu {
        for j in nct..nu {
            for i in 0..m {
                u[i][j] = 0.0;
            }
            u[j][j] = 1.0;
        }

        let mut k = nct as i32 - 1;
        while k >= 0 {
            let kk = k as usize;
            if s[kk] != 0.0 {
                for j in (kk + 1)..nu {
                    let mut t = 0.0;
                    for i in kk..m {
                        t += u[i][kk] * u[i][j];
                    }
                    t = -t / u[kk][kk];
                    for i in kk..m {
                        u[i][j] += t * u[i][kk];
                    }
                }
                for i in kk..m {
                    u[i][kk] = -u[i][kk];
                }
                u[kk][kk] += 1.0;
                for i in 0..kk.saturating_sub(1) {
                    u[i][kk] = 0.0;
                }
            } else {
                for i in 0..m {
                    u[i][kk] = 0.0;
                }
                u[kk][kk] = 1.0;
            }
            k -= 1;
        }
    }

    if wantv {
        let mut k = n as i32 - 1;
        while k >= 0 {
            let kk = k as usize;
            if kk < nrt && e[kk] != 0.0 {
                for j in (kk + 1)..nu {
                    let mut t = 0.0;
                    for i in (kk + 1)..n {
                        t += v[i][kk] * v[i][j];
                    }
                    t = -t / v[kk + 1][kk];
                    for i in (kk + 1)..n {
                        v[i][j] += t * v[i][kk];
                    }
                }
            }
            for i in 0..n {
                v[i][kk] = 0.0;
            }
            v[kk][kk] = 1.0;
            k -= 1;
        }
    }

    let mut p_i32 = p as i32;
    let pp = (p - 1) as i32;
    let mut iter = 0i32;
    let eps = 2f64.powi(-52);
    let tiny = 2f64.powi(-966);

    while p_i32 > 0 {
        let mut k: i32;
        let kase: i32;

        k = p_i32 - 2;
        while k >= -1 {
            if k == -1 {
                break;
            }
            let kk = k as usize;
            if e[kk].abs() <= tiny + eps * (s[kk].abs() + s[kk + 1].abs()) {
                e[kk] = 0.0;
                break;
            }
            k -= 1;
        }

        if k == p_i32 - 2 {
            kase = 4;
        } else {
            let mut ks = p_i32 - 1;
            while ks >= k {
                if ks == k {
                    break;
                }
                let ksu = ks as usize;
                let t = (if ks != p_i32 { e[ksu].abs() } else { 0.0 })
                    + (if ks != k + 1 { e[ksu - 1].abs() } else { 0.0 });
                if s[ksu].abs() <= tiny + eps * t {
                    s[ksu] = 0.0;
                    break;
                }
                ks -= 1;
            }

            if ks == k {
                kase = 3;
            } else if ks == p_i32 - 1 {
                kase = 1;
            } else {
                // kase = 2
                k = ks;
                kase = 2;
            }
        }

        k += 1;
        match kase {
            1 => {
                let mut f = e[(p_i32 - 2) as usize];
                e[(p_i32 - 2) as usize] = 0.0;
                let mut j = p_i32 - 2;
                while j >= k {
                    let ju = j as usize;
                    let t = svd_hypot(s[ju], f);
                    let cs = s[ju] / t;
                    let sn = f / t;
                    s[ju] = t;
                    if j != k {
                        f = -sn * e[(j - 1) as usize];
                        e[(j - 1) as usize] *= cs;
                    }
                    if wantv {
                        for i in 0..n {
                            let t2 = cs * v[i][ju] + sn * v[i][(p_i32 - 1) as usize];
                            v[i][(p_i32 - 1) as usize] =
                                -sn * v[i][ju] + cs * v[i][(p_i32 - 1) as usize];
                            v[i][ju] = t2;
                        }
                    }
                    j -= 1;
                }
            }
            2 => {
                let mut f = e[(k - 1) as usize];
                e[(k - 1) as usize] = 0.0;
                let mut j = k;
                while j < p_i32 {
                    let ju = j as usize;
                    let t = svd_hypot(s[ju], f);
                    let cs = s[ju] / t;
                    let sn = f / t;
                    s[ju] = t;
                    f = -sn * e[ju];
                    e[ju] *= cs;
                    if wantu {
                        for i in 0..m {
                            let t2 = cs * u[i][ju] + sn * u[i][(k - 1) as usize];
                            u[i][(k - 1) as usize] = -sn * u[i][ju] + cs * u[i][(k - 1) as usize];
                            u[i][ju] = t2;
                        }
                    }
                    j += 1;
                }
            }
            3 => {
                let scale = s[(p_i32 - 1) as usize]
                    .abs()
                    .max(s[(p_i32 - 2) as usize].abs())
                    .max(e[(p_i32 - 2) as usize].abs())
                    .max(s[k as usize].abs())
                    .max(e[k as usize].abs());
                let sp = s[(p_i32 - 1) as usize] / scale;
                let spm1 = s[(p_i32 - 2) as usize] / scale;
                let epm1 = e[(p_i32 - 2) as usize] / scale;
                let sk = s[k as usize] / scale;
                let ek = e[k as usize] / scale;
                let b = ((spm1 + sp) * (spm1 - sp) + epm1 * epm1) / 2.0;
                let c_val = (sp * epm1) * (sp * epm1);
                let mut shift = 0.0;
                if b != 0.0 || c_val != 0.0 {
                    shift = (b * b + c_val).sqrt();
                    if b < 0.0 {
                        shift = -shift;
                    }
                    shift = c_val / (b + shift);
                }
                let mut f = (sk + sp) * (sk - sp) + shift;
                let mut g = sk * ek;

                let mut j = k;
                while j < p_i32 - 1 {
                    let ju = j as usize;
                    let mut t = svd_hypot(f, g);
                    let mut cs = f / t;
                    let mut sn = g / t;
                    if j != k {
                        e[(j - 1) as usize] = t;
                    }
                    f = cs * s[ju] + sn * e[ju];
                    e[ju] = cs * e[ju] - sn * s[ju];
                    g = sn * s[ju + 1];
                    s[ju + 1] *= cs;
                    if wantv {
                        for i in 0..n {
                            t = cs * v[i][ju] + sn * v[i][ju + 1];
                            v[i][ju + 1] = -sn * v[i][ju] + cs * v[i][ju + 1];
                            v[i][ju] = t;
                        }
                    }

                    t = svd_hypot(f, g);
                    cs = f / t;
                    sn = g / t;
                    s[ju] = t;
                    f = cs * e[ju] + sn * s[ju + 1];
                    s[ju + 1] = -sn * e[ju] + cs * s[ju + 1];
                    g = sn * e[ju + 1];
                    e[ju + 1] *= cs;
                    if wantu && (j as usize) < m.saturating_sub(1) {
                        for i in 0..m {
                            t = cs * u[i][ju] + sn * u[i][ju + 1];
                            u[i][ju + 1] = -sn * u[i][ju] + cs * u[i][ju + 1];
                            u[i][ju] = t;
                        }
                    }
                    j += 1;
                }
                e[(p_i32 - 2) as usize] = f;
                iter += 1;
            }
            4 => {
                let ku = k as usize;
                if s[ku] <= 0.0 {
                    s[ku] = if s[ku] < 0.0 { -s[ku] } else { 0.0 };
                    if wantv {
                        for i in 0..=pp.max(0) as usize {
                            v[i][ku] = -v[i][ku];
                        }
                    }
                }
                while k < pp {
                    let ku = k as usize;
                    if s[ku] >= s[ku + 1] {
                        break;
                    }
                    s.swap(ku, ku + 1);
                    if wantv && (k as usize) < n.saturating_sub(1) {
                        for i in 0..n {
                            v[i].swap(ku + 1, ku);
                        }
                    }
                    if wantu && (k as usize) < m.saturating_sub(1) {
                        for i in 0..m {
                            u[i].swap(ku + 1, ku);
                        }
                    }
                    k += 1;
                }
                iter = 0;
                p_i32 -= 1;
            }
            _ => {}
        }

        // Prevent pathological infinite loops.
        if iter > 10_000 {
            break;
        }
    }

    Some(SvdResult { u, v, s })
}
