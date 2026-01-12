//! A graph representation for computing tree diffs.

use std::cmp::{Reverse, min};
use std::collections::BinaryHeap;
use std::collections::hash_map::Entry;
use std::fmt::Debug;
use std::hash::{Hash, Hasher};

use arrayvec::ArrayVec;
use collections::FxHashMap;

use crate::SyntaxTree;
use crate::syntax_tree::{SyntaxHint, SyntaxTreeCursor};

/// A path segment in the diff graph.
///
/// Represents a transition from one vertex to another via an edge.
/// - `from` is the source vertex (None for the start of the path)
/// - `edge` is the transition taken (None for the start of the path)
/// - `into` is the destination vertex
/// - `cost` is the cumulative cost to reach `into` from the start
#[derive(Debug, Clone)]
pub struct SyntaxPath<'a> {
    pub from: Option<SyntaxVertex<'a>>,
    pub edge: Option<SyntaxEdge>,
    pub into: SyntaxVertex<'a>,
    pub cost: u32,
}

/// Result of running Dijkstra's algorithm on two syntax trees.
pub struct SyntaxRoute<'a>(pub Vec<SyntaxPath<'a>>);

impl<'a> Debug for SyntaxRoute<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SyntaxRoute [")?;
        for path in &self.0 {
            let lhs_range = path.into.lhs.node().map(|n| &n.byte_range);
            let rhs_range = path.into.rhs.node().map(|n| &n.byte_range);
            writeln!(f, "  {:?} {:?} {:?}", path.edge, lhs_range, rhs_range)?;
        }
        write!(f, "]")
    }
}

impl<'a> PartialEq for SyntaxPath<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost
    }
}

impl<'a> Eq for SyntaxPath<'a> {}

impl<'a> PartialOrd for SyntaxPath<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<'a> Ord for SyntaxPath<'a> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.cost.cmp(&other.cost)
    }
}

/// A vertex in the diff graph.
///
/// Each vertex represents positions in both the LHS and RHS syntax trees,
/// along with a cursor into the delimiter stack that tracks how we got here.
#[derive(Debug, Clone)]
pub struct SyntaxVertex<'a> {
    pub lhs: SyntaxTreeCursor<'a>,
    pub rhs: SyntaxTreeCursor<'a>,
    pub delimiters: SyntaxDelimiters,
}

impl<'a> SyntaxVertex<'a> {
    pub fn new(
        lhs: SyntaxTreeCursor<'a>,
        rhs: SyntaxTreeCursor<'a>,
        delimiters: SyntaxDelimiters,
    ) -> Self {
        Self {
            lhs,
            rhs,
            delimiters,
        }
    }

    pub fn is_end(&self) -> bool {
        self.lhs.is_end() && self.rhs.is_end() && self.delimiters.is_empty()
    }

    fn can_pop_either(&self) -> bool {
        self.delimiters.can_pop_lhs() || self.delimiters.can_pop_rhs()
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
    Unchanged {
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
    EnterNovelDelimiterBoth,
}

impl SyntaxEdge {
    pub fn cost(self) -> u32 {
        match self {
            SyntaxEdge::Unchanged {
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
            SyntaxEdge::EnterNovelDelimiterBoth => 550,
            SyntaxEdge::Replaced { levenshtein_pct } => 500 + u32::from(100 - levenshtein_pct),
        }
    }
}

#[derive(Debug, Default, Clone, Copy, Hash, PartialEq, Eq)]
pub struct SyntaxDelimiters {
    // TODO: SmallVec<[(u64, u64)]; 16> to avoid limits
    lhs_depths: u128,
    rhs_depths: u128,
    both_depth: u8,
}

impl SyntaxDelimiters {
    const LAYER_SIZE: u8 = 8;

    pub fn is_empty(&self) -> bool {
        self.both_depth == 0 && self.lhs_depths == 0 && self.rhs_depths == 0
    }

    pub fn push_both(self) -> Self {
        Self {
            lhs_depths: self.lhs_depths,
            rhs_depths: self.rhs_depths,
            both_depth: self.both_depth + 1,
        }
    }

    pub fn push_lhs(self) -> Self {
        Self {
            lhs_depths: self.lhs_depths + self.shift(),
            rhs_depths: self.rhs_depths,
            both_depth: self.both_depth,
        }
    }

    pub fn push_rhs(self) -> Self {
        Self {
            rhs_depths: self.rhs_depths + self.shift(),
            lhs_depths: self.lhs_depths,
            both_depth: self.both_depth,
        }
    }

    pub fn pop_lhs(self) -> Option<Self> {
        if self.lhs_depth() == 0 {
            return None;
        }

        Some(Self {
            lhs_depths: self.lhs_depths - self.shift(),
            rhs_depths: self.rhs_depths,
            both_depth: self.both_depth,
        })
    }

    pub fn pop_rhs(self) -> Option<Self> {
        if self.rhs_depth() == 0 {
            return None;
        }

        Some(Self {
            rhs_depths: self.rhs_depths - self.shift(),
            lhs_depths: self.lhs_depths,
            both_depth: self.both_depth,
        })
    }

    pub fn pop_both(self) -> Option<Self> {
        if self.both_depth == 0 || self.lhs_depth() != 0 || self.rhs_depth() != 0 {
            return None;
        }

        Some(Self {
            both_depth: self.both_depth - 1,
            lhs_depths: self.lhs_depths,
            rhs_depths: self.rhs_depths,
        })
    }

    pub fn can_pop_lhs(&self) -> bool {
        self.lhs_depth() > 0
    }

    pub fn can_pop_rhs(&self) -> bool {
        self.rhs_depth() > 0
    }

    fn shift(&self) -> u128 {
        1 << (self.both_depth * Self::LAYER_SIZE)
    }

    fn lhs_depth(&self) -> u8 {
        ((self.lhs_depths >> (self.both_depth * Self::LAYER_SIZE)) & 0xFF) as u8
    }

    fn rhs_depth(&self) -> u8 {
        ((self.rhs_depths >> (self.both_depth * Self::LAYER_SIZE)) & 0xFF) as u8
    }
}

/// Pop as many delimiters as possible when cursors reach end of their current level.
fn pop_all_delimiters<'a>(
    mut lhs: SyntaxTreeCursor<'a>,
    mut rhs: SyntaxTreeCursor<'a>,
    mut delimiters: SyntaxDelimiters,
) -> (SyntaxTreeCursor<'a>, SyntaxTreeCursor<'a>, SyntaxDelimiters) {
    loop {
        let mut popped = false;

        // Try popping LHS delimiters while LHS cursor is at end
        while lhs.is_end() {
            if let Some(new_delimiters) = delimiters.pop_lhs() {
                lhs = lhs.last().parent().next_sibling();
                delimiters = new_delimiters;
                popped = true;
            } else {
                break;
            }
        }

        // Try popping RHS delimiters while RHS cursor is at end
        while rhs.is_end() {
            if let Some(new_delimiters) = delimiters.pop_rhs() {
                rhs = rhs.last().parent().next_sibling();
                delimiters = new_delimiters;
                popped = true;
            } else {
                break;
            }
        }

        // Try popping Both when both cursors are at end
        if lhs.is_end() && rhs.is_end() {
            if let Some(new_delimiters) = delimiters.pop_both() {
                lhs = lhs.last().parent().next_sibling();
                rhs = rhs.last().parent().next_sibling();
                delimiters = new_delimiters;
                popped = true;
            }
        }

        if !popped {
            break;
        }
    }

    (lhs, rhs, delimiters)
}

/// Compute all possible neighbor vertices from the current vertex.
pub fn compute_neighbours<'a>(v: &SyntaxVertex<'a>) -> ArrayVec<(SyntaxEdge, SyntaxVertex<'a>), 8> {
    let mut neighbours = ArrayVec::new();

    if let (Some(lhs_node), Some(rhs_node)) = (v.lhs.node(), v.rhs.node()) {
        // Both nodes have same structure - unchanged
        if lhs_node.structural_hash == rhs_node.structural_hash {
            let depth_difference = (v.lhs.depth() as i32 - v.rhs.depth() as i32).unsigned_abs();
            let probably_punctuation = v
                .lhs
                .node()
                .is_some_and(|node| node.hint == Some(SyntaxHint::Punctuation));
            let (lhs, rhs, delimiters) =
                pop_all_delimiters(v.lhs.next_sibling(), v.rhs.next_sibling(), v.delimiters);
            neighbours.push((
                SyntaxEdge::Unchanged {
                    depth_difference,
                    probably_punctuation,
                },
                SyntaxVertex {
                    lhs,
                    rhs,
                    delimiters,
                },
            ));
        } else {
            if let (
                Some(SyntaxHint::Comment(lhs_comment)),
                Some(SyntaxHint::Comment(rhs_comment)),
            ) = (lhs_node.hint.as_ref(), rhs_node.hint.as_ref())
            {
                let levenshtein_pct = (strsim::normalized_levenshtein(lhs_comment, rhs_comment)
                    * 100.0)
                    .round() as u8;
                let (lhs, rhs, delimiters) =
                    pop_all_delimiters(v.lhs.next_sibling(), v.rhs.next_sibling(), v.delimiters);
                neighbours.push((
                    SyntaxEdge::Replaced { levenshtein_pct },
                    SyntaxVertex {
                        lhs,
                        rhs,
                        delimiters,
                    },
                ));
            }
        }

        // Both are lists - check if delimiters match
        if lhs_node.is_list() && rhs_node.is_list() {
            let delimiters_match = lhs_node.has_delimiters()
                && rhs_node.has_delimiters()
                && lhs_node.open_delimiter() == rhs_node.open_delimiter()
                && lhs_node.close_delimiter() == rhs_node.close_delimiter();

            if delimiters_match {
                // Both are lists with matching delimiters - enter them together
                let depth_difference = (v.lhs.depth() as i32 - v.rhs.depth() as i32).unsigned_abs();
                let delimiters = v.delimiters.push_both();
                let (lhs, rhs, delimiters) =
                    pop_all_delimiters(v.lhs.first_child(), v.rhs.first_child(), delimiters);
                neighbours.push((
                    SyntaxEdge::EnterUnchangedDelimiter { depth_difference },
                    SyntaxVertex {
                        lhs,
                        rhs,
                        delimiters,
                    },
                ));
            } else {
                // Both are lists with non-matching delimiters - enter both as novel
                let delimiters = v.delimiters.push_lhs().push_rhs();
                let (lhs, rhs, delimiters) =
                    pop_all_delimiters(v.lhs.first_child(), v.rhs.first_child(), delimiters);
                neighbours.push((
                    SyntaxEdge::EnterNovelDelimiterBoth,
                    SyntaxVertex {
                        lhs,
                        rhs,
                        delimiters,
                    },
                ));
            }
        }
    }

    // Novel LHS atom
    if let Some(lhs_node) = v.lhs.node() {
        if lhs_node.is_atom() {
            let (lhs, rhs, delimiters) =
                pop_all_delimiters(v.lhs.next_sibling(), v.rhs, v.delimiters);
            neighbours.push((
                SyntaxEdge::NovelAtomLHS,
                SyntaxVertex {
                    lhs,
                    rhs,
                    delimiters,
                },
            ));
        } else {
            // Enter novel LHS list
            let delimiters = v.delimiters.push_lhs();
            let (lhs, rhs, delimiters) = pop_all_delimiters(v.lhs.first_child(), v.rhs, delimiters);
            neighbours.push((
                SyntaxEdge::EnterNovelDelimiterLHS,
                SyntaxVertex {
                    lhs,
                    rhs,
                    delimiters,
                },
            ));
        }
    }

    // Novel RHS atom
    if let Some(rhs_node) = v.rhs.node() {
        if rhs_node.is_atom() {
            let (lhs, rhs, delimiters) =
                pop_all_delimiters(v.lhs, v.rhs.next_sibling(), v.delimiters);
            neighbours.push((
                SyntaxEdge::NovelAtomRHS,
                SyntaxVertex {
                    lhs,
                    rhs,
                    delimiters,
                },
            ));
        } else {
            // Enter novel RHS list
            let delimiters = v.delimiters.push_rhs();
            let (lhs, rhs, delimiters) = pop_all_delimiters(v.lhs, v.rhs.first_child(), delimiters);
            neighbours.push((
                SyntaxEdge::EnterNovelDelimiterRHS,
                SyntaxVertex {
                    lhs,
                    rhs,
                    delimiters,
                },
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
) -> Option<SyntaxRoute<'a>> {
    let lhs_cursor = lhs_tree.cursor();
    let rhs_cursor = rhs_tree.cursor();

    let graph_limit = std::cmp::min(2 * lhs_tree.len() * rhs_tree.len(), graph_limit);

    let mut heap: BinaryHeap<Reverse<SyntaxPath<'a>>> = BinaryHeap::default();
    let mut visited: FxHashMap<SyntaxVertex<'a>, SyntaxPath<'a>> =
        FxHashMap::with_capacity_and_hasher(graph_limit, Default::default());

    let start = SyntaxVertex::new(lhs_cursor, rhs_cursor, SyntaxDelimiters::default());

    heap.push(Reverse(SyntaxPath {
        from: None,
        edge: None,
        into: start,
        cost: 0,
    }));

    let end_vertex = loop {
        let Reverse(path) = heap.pop()?;
        let current_vertex = path.into.clone();

        match visited.entry(current_vertex.clone()) {
            Entry::Occupied(e) if path.cost >= e.get().cost => continue,
            Entry::Occupied(mut e) => {
                e.insert(path.clone());
            }
            Entry::Vacant(e) => {
                e.insert(path.clone());
            }
        };

        if current_vertex.is_end() {
            break current_vertex;
        }

        if visited.len() > graph_limit {
            return None;
        }

        let neighbours = compute_neighbours(&current_vertex);
        for (edge, next_vertex) in neighbours {
            let next_cost = path.cost + edge.cost();

            let dominated = visited
                .get(&next_vertex)
                .is_some_and(|v| next_cost >= v.cost);

            if !dominated {
                heap.push(Reverse(SyntaxPath {
                    from: Some(current_vertex.clone()),
                    edge: Some(edge),
                    into: next_vertex,
                    cost: next_cost,
                }));
            }
        }
    };

    Some(SyntaxRoute(reconstruct_path(end_vertex, &visited)))
}

fn reconstruct_path<'a>(
    end: SyntaxVertex<'a>,
    visited: &FxHashMap<SyntaxVertex<'a>, SyntaxPath<'a>>,
) -> Vec<SyntaxPath<'a>> {
    let mut route = Vec::new();
    let mut current = end;

    while let Some(segment) = visited.get(&current) {
        let Some(predecessor) = segment.from.clone() else {
            break;
        };

        route.push(SyntaxPath {
            from: Some(predecessor.clone()),
            edge: segment.edge,
            into: current,
            cost: segment.cost,
        });

        current = predecessor;
    }

    route.reverse();
    route
}
