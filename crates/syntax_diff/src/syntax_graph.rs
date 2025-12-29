//! A graph representation for computing tree diffs.

use std::cmp::{Reverse, min};
use std::collections::{BinaryHeap, HashMap};
use std::hash::{Hash, Hasher};

use crate::SyntaxTree;
use crate::syntax_tree::{SyntaxId, SyntaxTreeCursor};

/// Error when the graph search exceeds the configured limit.
#[derive(Debug)]
pub struct ExceededGraphLimit;

/// Result of running Dijkstra's algorithm on two syntax trees.
///
/// The route as (vertex_before, edge) pairs.
/// Each entry represents: from vertex_before, take edge.
pub struct SyntaxRoute<'a>(pub Vec<(SyntaxVertex<'a>, SyntaxEdge)>);

#[derive(Clone)]
// TODO: revisit
struct VertexState<'a> {
    vertex: SyntaxVertex<'a>,
    cost: u32,
    predecessor: Option<(SyntaxEdge, Box<VertexState<'a>>)>,
}

impl<'a> PartialEq for VertexState<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost
    }
}

impl<'a> Eq for VertexState<'a> {}

impl<'a> PartialOrd for VertexState<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<'a> Ord for VertexState<'a> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.cost.cmp(&other.cost)
    }
}

/// Tracks how we entered list delimiters.
#[derive(Clone, PartialEq, Eq)]
pub enum EnteredDelimiter {
    /// We entered LHS and RHS lists together - must pop together.
    PopBoth { lhs: SyntaxId, rhs: SyntaxId },
    /// We entered LHS and RHS separately - can pop independently.
    PopEither {
        lhs: Vec<SyntaxId>,
        rhs: Vec<SyntaxId>,
    },
}

/// A vertex in the diff graph.
///
/// Each vertex represents positions in both the LHS and RHS syntax trees,
/// along with a stack of entered delimiters that tracks how we got here.
#[derive(Clone)]
pub struct SyntaxVertex<'a> {
    pub lhs: SyntaxTreeCursor<'a>,
    pub rhs: SyntaxTreeCursor<'a>,
    parents: Vec<EnteredDelimiter>,
}

impl<'a> SyntaxVertex<'a> {
    pub fn new(lhs: SyntaxTreeCursor<'a>, rhs: SyntaxTreeCursor<'a>) -> Self {
        Self {
            lhs,
            rhs,
            parents: Vec::new(),
        }
    }

    pub fn is_end(&self) -> bool {
        self.lhs.is_end() && self.rhs.is_end() && self.parents.is_empty()
    }

    fn can_pop_either(&self) -> bool {
        matches!(
            self.parents.last(),
            Some(EnteredDelimiter::PopEither { .. })
        )
    }

    fn push_both_delimiters(&self, lhs_id: SyntaxId, rhs_id: SyntaxId) -> Vec<EnteredDelimiter> {
        let mut parents = self.parents.clone();
        parents.push(EnteredDelimiter::PopBoth {
            lhs: lhs_id,
            rhs: rhs_id,
        });
        parents
    }

    fn push_lhs_delimiter(&self, id: SyntaxId) -> Vec<EnteredDelimiter> {
        let mut parents = self.parents.clone();
        match parents.last_mut() {
            Some(EnteredDelimiter::PopEither { lhs, .. }) => {
                lhs.push(id);
            }
            _ => {
                parents.push(EnteredDelimiter::PopEither {
                    lhs: vec![id],
                    rhs: vec![],
                });
            }
        }
        parents
    }

    fn push_rhs_delimiter(&self, id: SyntaxId) -> Vec<EnteredDelimiter> {
        let mut parents = self.parents.clone();
        match parents.last_mut() {
            Some(EnteredDelimiter::PopEither { rhs, .. }) => {
                rhs.push(id);
            }
            _ => {
                parents.push(EnteredDelimiter::PopEither {
                    lhs: vec![],
                    rhs: vec![id],
                });
            }
        }
        parents
    }
}

impl PartialEq for SyntaxVertex<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.lhs == other.lhs
            && self.rhs == other.rhs
            && self.can_pop_either() == other.can_pop_either()
    }
}

impl Eq for SyntaxVertex<'_> {}

impl Hash for SyntaxVertex<'_> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.lhs.hash(state);
        self.rhs.hash(state);
        self.can_pop_either().hash(state);
    }
}

/// An edge in the diff graph with an associated cost.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum SyntaxEdge {
    UnchangedNode {
        depth_difference: u32,
        probably_punctuation: bool,
    },
    EnterUnchangedDelimiter {
        depth_difference: u32,
    },
    Replaced {
        levenshtein_pct: u8,
    },
    NovelAtomLHS,
    NovelAtomRHS,
    EnterNovelDelimiterLHS,
    EnterNovelDelimiterRHS,
}

impl SyntaxEdge {
    pub fn cost(self) -> u32 {
        match self {
            SyntaxEdge::UnchangedNode {
                depth_difference,
                probably_punctuation,
            } => {
                let base = min(40, depth_difference + 1);
                base + if probably_punctuation { 200 } else { 0 }
            }
            SyntaxEdge::EnterUnchangedDelimiter { depth_difference } => {
                100 + min(40, depth_difference)
            }
            SyntaxEdge::NovelAtomLHS | SyntaxEdge::NovelAtomRHS => 300,
            SyntaxEdge::EnterNovelDelimiterLHS | SyntaxEdge::EnterNovelDelimiterRHS => 300,
            SyntaxEdge::Replaced { levenshtein_pct } => 500 + u32::from(100 - levenshtein_pct),
        }
    }
}

/// Pop as many parents as possible when cursors reach end of their current level.
fn pop_all_parents<'a>(
    mut lhs: SyntaxTreeCursor<'a>,
    mut rhs: SyntaxTreeCursor<'a>,
    mut parents: Vec<EnteredDelimiter>,
) -> (
    SyntaxTreeCursor<'a>,
    SyntaxTreeCursor<'a>,
    Vec<EnteredDelimiter>,
) {
    loop {
        if lhs.is_end() {
            if let Some(EnteredDelimiter::PopEither {
                lhs: lhs_stack,
                rhs: rhs_stack,
            }) = parents.last_mut()
            {
                if let Some(lhs_parent_id) = lhs_stack.pop() {
                    lhs = lhs.tree().cursor_at(lhs_parent_id).next_sibling();
                    if lhs_stack.is_empty() && rhs_stack.is_empty() {
                        parents.pop();
                    }
                    continue;
                }
            }
        }

        if rhs.is_end() {
            if let Some(EnteredDelimiter::PopEither {
                lhs: lhs_stack,
                rhs: rhs_stack,
            }) = parents.last_mut()
            {
                if let Some(rhs_parent_id) = rhs_stack.pop() {
                    rhs = rhs.tree().cursor_at(rhs_parent_id).next_sibling();
                    if lhs_stack.is_empty() && rhs_stack.is_empty() {
                        parents.pop();
                    }
                    continue;
                }
            }
        }

        if lhs.is_end() && rhs.is_end() {
            if let Some(EnteredDelimiter::PopBoth {
                lhs: lhs_id,
                rhs: rhs_id,
            }) = parents.last()
            {
                let lhs_id = *lhs_id;
                let rhs_id = *rhs_id;
                parents.pop();
                lhs = lhs.tree().cursor_at(lhs_id).next_sibling();
                rhs = rhs.tree().cursor_at(rhs_id).next_sibling();
                continue;
            }
        }

        break;
    }

    (lhs, rhs, parents)
}

/// Compute all possible neighbor vertices from the current vertex.
pub fn compute_neighbours<'a>(v: &SyntaxVertex<'a>) -> Vec<(SyntaxEdge, SyntaxVertex<'a>)> {
    let mut neighbours = Vec::with_capacity(7);

    if let (Some(lhs_node), Some(rhs_node)) = (v.lhs.node(), v.rhs.node()) {
        let lhs_id = v.lhs.id().unwrap();
        let rhs_id = v.rhs.id().unwrap();

        // Both nodes have same structure - unchanged
        if lhs_node.structural_hash == rhs_node.structural_hash {
            let depth_difference = (v.lhs.depth() as i32 - v.rhs.depth() as i32).unsigned_abs();
            // TODO: https://github.com/Wilfred/difftastic/blob/cba6cc5d5a0b47b36fdb028a87af03c89d1908b4/src/diff/graph.rs#L422
            let probably_punctuation = false;

            let (lhs, rhs, parents) = pop_all_parents(
                v.lhs.next_sibling(),
                v.rhs.next_sibling(),
                v.parents.clone(),
            );

            neighbours.push((
                SyntaxEdge::UnchangedNode {
                    depth_difference,
                    probably_punctuation,
                },
                SyntaxVertex { lhs, rhs, parents },
            ));
        }

        // Both are lists with matching delimiters - enter them together
        if lhs_node.is_list() && rhs_node.is_list() {
            if lhs_node.open_delimiter() == rhs_node.open_delimiter()
                && lhs_node.close_delimiter() == rhs_node.close_delimiter()
            {
                let depth_difference = (v.lhs.depth() as i32 - v.rhs.depth() as i32).unsigned_abs();
                let parents = v.push_both_delimiters(lhs_id, rhs_id);

                let (lhs, rhs, parents) =
                    pop_all_parents(v.lhs.first_child(), v.rhs.first_child(), parents);

                neighbours.push((
                    SyntaxEdge::EnterUnchangedDelimiter { depth_difference },
                    SyntaxVertex { lhs, rhs, parents },
                ));
            }
        }
    }

    // Novel LHS atom
    if let Some(lhs_node) = v.lhs.node() {
        if lhs_node.is_atom() {
            let (lhs, rhs, parents) =
                pop_all_parents(v.lhs.next_sibling(), v.rhs, v.parents.clone());
            neighbours.push((SyntaxEdge::NovelAtomLHS, SyntaxVertex { lhs, rhs, parents }));
        } else {
            // Enter novel LHS list
            let lhs_id = v.lhs.id().unwrap();
            let parents = v.push_lhs_delimiter(lhs_id);

            let (lhs, rhs, parents) = pop_all_parents(v.lhs.first_child(), v.rhs, parents);
            neighbours.push((
                SyntaxEdge::EnterNovelDelimiterLHS,
                SyntaxVertex { lhs, rhs, parents },
            ));
        }
    }

    // Novel RHS atom
    if let Some(rhs_node) = v.rhs.node() {
        if rhs_node.is_atom() {
            let (lhs, rhs, parents) =
                pop_all_parents(v.lhs, v.rhs.next_sibling(), v.parents.clone());
            neighbours.push((SyntaxEdge::NovelAtomRHS, SyntaxVertex { lhs, rhs, parents }));
        } else {
            // Enter novel RHS list
            let rhs_id = v.rhs.id().unwrap();
            let parents = v.push_rhs_delimiter(rhs_id);

            let (lhs, rhs, parents) = pop_all_parents(v.lhs, v.rhs.first_child(), parents);
            neighbours.push((
                SyntaxEdge::EnterNovelDelimiterRHS,
                SyntaxVertex { lhs, rhs, parents },
            ));
        }
    }

    neighbours
}

/// Find the shortest path between two syntax trees.
///
/// Returns a sequence of edges representing the optimal diff.
pub fn shortest_path<'a>(
    lhs_tree: &'a SyntaxTree,
    rhs_tree: &'a SyntaxTree,
    graph_limit: usize,
) -> Result<SyntaxRoute<'a>, ExceededGraphLimit> {
    let lhs_cursor = lhs_tree.cursor();
    let rhs_cursor = rhs_tree.cursor();
    let start = SyntaxVertex::new(lhs_cursor, rhs_cursor);

    Ok(SyntaxRoute(find_shortest_path(start, graph_limit)?))
}

fn find_shortest_path<'a>(
    start: SyntaxVertex<'a>,
    graph_limit: usize,
) -> Result<Vec<(SyntaxVertex<'a>, SyntaxEdge)>, ExceededGraphLimit> {
    let mut heap: BinaryHeap<Reverse<VertexState<'a>>> = BinaryHeap::new();
    let mut best_cost: HashMap<SyntaxVertex<'a>, u32> = HashMap::new();

    heap.push(Reverse(VertexState {
        vertex: start,
        cost: 0,
        predecessor: None,
    }));

    let end_state = loop {
        let Reverse(current) = match heap.pop() {
            Some(state) => state,
            None => panic!("Ran out of graph nodes before reaching end"),
        };

        if current.vertex.is_end() {
            break current;
        }

        if let Some(&existing_cost) = best_cost.get(&current.vertex) {
            if current.cost >= existing_cost {
                continue;
            }
        }

        best_cost.insert(current.vertex.clone(), current.cost);

        if best_cost.len() > graph_limit {
            return Err(ExceededGraphLimit);
        }

        let neighbours = compute_neighbours(&current.vertex);
        for (edge, next_vertex) in neighbours {
            let next_cost = current.cost + edge.cost();

            let dominated = best_cost.get(&next_vertex).is_some_and(|&c| next_cost >= c);

            if !dominated {
                heap.push(Reverse(VertexState {
                    vertex: next_vertex,
                    cost: next_cost,
                    predecessor: Some((edge, Box::new(current.clone()))),
                }));
            }
        }
    };

    Ok(reconstruct_path(end_state))
}

fn reconstruct_path<'a>(end_state: VertexState<'a>) -> Vec<(SyntaxVertex<'a>, SyntaxEdge)> {
    let mut path = Vec::new();
    let mut current = end_state;

    // Walk backwards through predecessors
    while let Some((edge, predecessor)) = current.predecessor {
        // predecessor.vertex is the state BEFORE taking edge
        // current.vertex is the state AFTER taking edge
        path.push((predecessor.vertex.clone(), edge));
        current = *predecessor;
    }

    path.reverse();
    path
}
