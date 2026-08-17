use super::OrderNodeRange;
use crate::graphlib::Graph;
use rustc_hash::FxHashMap as HashMap;

pub fn init_order<N, E, G>(g: &Graph<N, E, G>) -> Vec<Vec<String>>
where
    N: Default + OrderNodeRange + 'static,
    E: Default + 'static,
    G: Default,
{
    let mut visited: HashMap<String, bool> = HashMap::default();

    let simple_nodes: Vec<String> = g
        .nodes()
        .filter(|v| g.children_iter(v).next().is_none())
        .map(|v| v.to_string())
        .collect();

    let mut max_rank: i32 = i32::MIN;
    for v in &simple_nodes {
        let Some(rank) = g.node(v).and_then(|n| n.rank()) else {
            continue;
        };
        max_rank = max_rank.max(rank);
    }

    if max_rank == i32::MIN {
        return Vec::new();
    }

    let mut layers: Vec<Vec<String>> = vec![Vec::new(); (max_rank + 1).max(0) as usize];

    let mut ordered_vs = simple_nodes.clone();

    let mut insertion_idx: HashMap<String, usize> = HashMap::default();
    for (idx, v) in simple_nodes.iter().enumerate() {
        insertion_idx.insert(v.to_string(), idx);
    }

    // Dagre's `initOrder` is effectively stable for nodes within the same rank (Graphlib/JS
    // preserves insertion order in `g.nodes()`). Rust's `sort_by_key` is unstable, so we must
    // include insertion order as a tie-breaker to avoid mirrored / drifted layouts on graphs
    // with symmetric constraints.
    ordered_vs.sort_by_key(|v| {
        let rank = g.node(v).and_then(|n| n.rank()).unwrap_or(i32::MAX);
        let idx = insertion_idx.get(v).copied().unwrap_or(usize::MAX);
        (rank, idx)
    });
    for v in ordered_vs {
        let mut stack = vec![v];
        while let Some(v) = stack.pop() {
            if visited.get(v.as_str()).copied().unwrap_or(false) {
                continue;
            }
            visited.insert(v.clone(), true);

            let Some(rank) = g.node(&v).and_then(|n| n.rank()) else {
                continue;
            };
            let idx = rank.max(0) as usize;
            if let Some(layer) = layers.get_mut(idx) {
                layer.push(v.clone());
            }

            let successors = g.successors(&v);
            for w in successors.into_iter().rev() {
                stack.push(w.to_string());
            }
        }
    }

    layers
}
