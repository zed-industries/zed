use super::OrderEdgeWeight;
use crate::graphlib::Graph;
use rustc_hash::FxHashMap as HashMap;

pub fn cross_count<N, E, G>(g: &Graph<N, E, G>, layering: &[Vec<String>]) -> f64
where
    N: Default + 'static,
    E: Default + OrderEdgeWeight + 'static,
    G: Default,
{
    let mut cc: f64 = 0.0;
    for i in 1..layering.len() {
        cc += two_layer_cross_count(g, &layering[i - 1], &layering[i]);
    }
    cc
}

pub(crate) fn cross_count_ix<N, E, G>(g: &Graph<N, E, G>, layering: &[Vec<usize>]) -> f64
where
    N: Default + 'static,
    E: Default + OrderEdgeWeight + 'static,
    G: Default,
{
    let mut cc: f64 = 0.0;
    for i in 1..layering.len() {
        cc += two_layer_cross_count_ix(g, &layering[i - 1], &layering[i]);
    }
    cc
}

fn two_layer_cross_count<N, E, G>(g: &Graph<N, E, G>, north: &[String], south: &[String]) -> f64
where
    N: Default + 'static,
    E: Default + OrderEdgeWeight + 'static,
    G: Default,
{
    if south.is_empty() {
        return 0.0;
    }

    let mut south_pos: HashMap<&str, usize> = HashMap::default();
    for (i, v) in south.iter().enumerate() {
        south_pos.insert(v.as_str(), i);
    }

    #[derive(Debug, Clone)]
    struct SouthEntry {
        pos: usize,
        weight: f64,
    }

    let mut south_entries: Vec<SouthEntry> = Vec::new();
    for v in north {
        let mut entries: Vec<SouthEntry> = Vec::new();
        g.for_each_out_edge(v, None, |ek, lbl| {
            let Some(&pos) = south_pos.get(ek.w.as_str()) else {
                return;
            };
            entries.push(SouthEntry {
                pos,
                weight: lbl.weight(),
            });
        });
        entries.sort_by_key(|e| e.pos);
        south_entries.extend(entries);
    }

    let mut first_index: usize = 1;
    while first_index < south.len() {
        first_index <<= 1;
    }
    let tree_size = 2 * first_index - 1;
    first_index -= 1;
    let mut tree: Vec<f64> = vec![0.0; tree_size];

    let mut cc: f64 = 0.0;
    for entry in south_entries {
        let mut index = entry.pos + first_index;
        tree[index] += entry.weight;
        let mut weight_sum: f64 = 0.0;
        while index > 0 {
            if index % 2 == 1 {
                weight_sum += tree[index + 1];
            }
            index = (index - 1) >> 1;
            tree[index] += entry.weight;
        }
        cc += entry.weight * weight_sum;
    }

    cc
}

fn two_layer_cross_count_ix<N, E, G>(g: &Graph<N, E, G>, north: &[usize], south: &[usize]) -> f64
where
    N: Default + 'static,
    E: Default + OrderEdgeWeight + 'static,
    G: Default,
{
    if south.is_empty() {
        return 0.0;
    }

    let mut south_pos: HashMap<usize, usize> = HashMap::default();
    for (i, &v_ix) in south.iter().enumerate() {
        south_pos.insert(v_ix, i);
    }

    #[derive(Debug, Clone)]
    struct SouthEntry {
        pos: usize,
        weight: f64,
    }

    let mut south_entries: Vec<SouthEntry> = Vec::new();
    for &v_ix in north {
        let mut entries: Vec<SouthEntry> = Vec::new();
        g.for_each_out_edge_ix(v_ix, None, |_u_ix, w_ix, _ek, lbl| {
            let Some(&pos) = south_pos.get(&w_ix) else {
                return;
            };
            entries.push(SouthEntry {
                pos,
                weight: lbl.weight(),
            });
        });
        entries.sort_by_key(|e| e.pos);
        south_entries.extend(entries);
    }

    let mut first_index: usize = 1;
    while first_index < south.len() {
        first_index <<= 1;
    }
    let tree_size = 2 * first_index - 1;
    first_index -= 1;
    let mut tree: Vec<f64> = vec![0.0; tree_size];

    let mut cc: f64 = 0.0;
    for entry in south_entries {
        let mut index = entry.pos + first_index;
        tree[index] += entry.weight;
        let mut weight_sum: f64 = 0.0;
        while index > 0 {
            if index % 2 == 1 {
                weight_sum += tree[index + 1];
            }
            index = (index - 1) >> 1;
            tree[index] += entry.weight;
        }
        cc += entry.weight * weight_sum;
    }

    cc
}
