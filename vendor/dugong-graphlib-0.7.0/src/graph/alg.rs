//! Helper algorithms re-exported for Dagre compatibility.

use super::Graph;
use std::collections::{BTreeMap, BTreeSet, VecDeque};

pub fn preorder<N, E, G>(g: &Graph<N, E, G>, roots: &[&str]) -> Vec<String>
where
    N: Default + 'static,
    E: Default + 'static,
    G: Default,
{
    let mut visited: BTreeSet<String> = BTreeSet::new();
    let mut out: Vec<String> = Vec::new();
    for r in roots {
        assert!(g.has_node(r), "preorder root is not in the graph: {r}");
        let mut stack = vec![r.to_string()];
        while let Some(v) = stack.pop() {
            if !visited.insert(v.clone()) {
                continue;
            }
            out.push(v.clone());
            let successors = g.successors(&v);
            for w in successors.into_iter().rev() {
                stack.push(w.to_string());
            }
        }
    }
    out
}

pub fn postorder<N, E, G>(g: &Graph<N, E, G>, roots: &[&str]) -> Vec<String>
where
    N: Default + 'static,
    E: Default + 'static,
    G: Default,
{
    let mut visited: BTreeSet<String> = BTreeSet::new();
    let mut out: Vec<String> = Vec::new();
    for r in roots {
        assert!(g.has_node(r), "postorder root is not in the graph: {r}");
        let mut stack = vec![(r.to_string(), false)];
        while let Some((v, expanded)) = stack.pop() {
            if expanded {
                out.push(v);
                continue;
            }
            if !visited.insert(v.clone()) {
                continue;
            }
            stack.push((v.clone(), true));
            let successors = g.successors(&v);
            for w in successors.into_iter().rev() {
                stack.push((w.to_string(), false));
            }
        }
    }
    out
}

pub fn components<N, E, G>(g: &Graph<N, E, G>) -> Vec<Vec<String>>
where
    N: Default + 'static,
    E: Default + 'static,
    G: Default,
{
    let mut seen: BTreeSet<String> = BTreeSet::new();
    let ids = g.node_ids();
    let mut out: Vec<Vec<String>> = Vec::new();

    for start in ids {
        if !seen.insert(start.clone()) {
            continue;
        }
        let mut comp: Vec<String> = Vec::new();
        let mut q: VecDeque<String> = VecDeque::new();
        q.push_back(start);
        while let Some(v) = q.pop_front() {
            comp.push(v.clone());
            for n in g.successors(&v) {
                if seen.insert(n.to_string()) {
                    q.push_back(n.to_string());
                }
            }
            for n in g.predecessors(&v) {
                if seen.insert(n.to_string()) {
                    q.push_back(n.to_string());
                }
            }
        }
        out.push(comp);
    }

    out
}

pub fn find_cycles<N, E, G>(g: &Graph<N, E, G>) -> Vec<Vec<String>>
where
    N: Default + 'static,
    E: Default + 'static,
    G: Default,
{
    // Strongly connected components (Tarjan). Report SCCs with size > 1, or self-loops.
    let node_ids = g.node_ids();
    struct TarjanFrame {
        v: String,
        successors: Vec<String>,
        next_successor: usize,
    }

    struct Tarjan<'a, N, E, G>
    where
        N: Default + 'static,
        E: Default + 'static,
        G: Default,
    {
        g: &'a Graph<N, E, G>,
        index: usize,
        stack: Vec<String>,
        on_stack: BTreeSet<String>,
        indices: BTreeMap<String, usize>,
        lowlink: BTreeMap<String, usize>,
        sccs: Vec<Vec<String>>,
    }

    impl<N, E, G> Tarjan<'_, N, E, G>
    where
        N: Default + 'static,
        E: Default + 'static,
        G: Default,
    {
        fn push_frame(&mut self, v: String, frames: &mut Vec<TarjanFrame>) {
            self.indices.insert(v.to_string(), self.index);
            self.lowlink.insert(v.to_string(), self.index);
            self.index += 1;
            self.stack.push(v.to_string());
            self.on_stack.insert(v.to_string());
            frames.push(TarjanFrame {
                successors: self
                    .g
                    .successors(&v)
                    .into_iter()
                    .map(str::to_string)
                    .collect(),
                v,
                next_successor: 0,
            });
        }

        fn strongconnect_iterative(&mut self, root: &str) {
            let mut frames: Vec<TarjanFrame> = Vec::new();
            self.push_frame(root.to_string(), &mut frames);

            while !frames.is_empty() {
                let next = {
                    let frame = match frames.last_mut() {
                        Some(frame) => frame,
                        None => break,
                    };
                    if frame.next_successor < frame.successors.len() {
                        let w = frame.successors[frame.next_successor].clone();
                        frame.next_successor += 1;
                        Some(w)
                    } else {
                        None
                    }
                };

                if let Some(w) = next {
                    let v = match frames.last() {
                        Some(frame) => frame.v.clone(),
                        None => break,
                    };
                    if !self.indices.contains_key(&w) {
                        self.push_frame(w, &mut frames);
                        continue;
                    }
                    if self.on_stack.contains(&w) {
                        let Some(v_low) = self.lowlink.get(&v).copied() else {
                            debug_assert!(false, "tarjan lowlink missing for v");
                            continue;
                        };
                        let Some(w_idx) = self.indices.get(&w).copied() else {
                            debug_assert!(false, "tarjan index missing for w");
                            continue;
                        };
                        self.lowlink.insert(v, v_low.min(w_idx));
                    }
                    continue;
                }

                let Some(frame) = frames.pop() else {
                    break;
                };
                let v = frame.v;
                let Some(v_low) = self.lowlink.get(&v).copied() else {
                    debug_assert!(false, "tarjan lowlink missing for v");
                    continue;
                };
                let Some(v_idx) = self.indices.get(&v).copied() else {
                    debug_assert!(false, "tarjan index missing for v");
                    continue;
                };
                if v_low == v_idx {
                    let mut scc: Vec<String> = Vec::new();
                    loop {
                        let Some(w) = self.stack.pop() else {
                            debug_assert!(false, "tarjan stack underflow");
                            break;
                        };
                        self.on_stack.remove(&w);
                        scc.push(w.clone());
                        if w == v {
                            break;
                        }
                    }
                    self.sccs.push(scc);
                }

                if let Some(parent) = frames.last_mut() {
                    let Some(parent_low) = self.lowlink.get(&parent.v).copied() else {
                        debug_assert!(false, "tarjan lowlink missing for parent");
                        continue;
                    };
                    self.lowlink.insert(parent.v.clone(), parent_low.min(v_low));
                }
            }
        }
    }

    let mut tarjan = Tarjan {
        g,
        index: 0,
        stack: Vec::new(),
        on_stack: BTreeSet::new(),
        indices: BTreeMap::new(),
        lowlink: BTreeMap::new(),
        sccs: Vec::new(),
    };

    for v in &node_ids {
        if !tarjan.indices.contains_key(v) {
            tarjan.strongconnect_iterative(v);
        }
    }

    let mut cycles: Vec<Vec<String>> = Vec::new();
    for mut scc in tarjan.sccs {
        if scc.len() > 1 {
            // Deterministic node order: use original insertion order.
            let order: BTreeMap<String, usize> = node_ids
                .iter()
                .cloned()
                .enumerate()
                .map(|(i, v)| (v, i))
                .collect();
            scc.sort_by_key(|v| order.get(v).copied().unwrap_or(usize::MAX));
            cycles.push(scc);
        } else {
            let v = &scc[0];
            if g.has_edge(v, v, None) || !g.out_edges(v, Some(v)).is_empty() {
                cycles.push(vec![v.clone()]);
            }
        }
    }

    cycles.sort_by(|a, b| a.first().cmp(&b.first()));
    cycles
}
