//! A graph representation for computing tree diffs.

use std::cmp::min;
use std::hash::{Hash, Hasher};

use crate::syntax_tree::{AtomKind, SyntaxId, SyntaxTreeCursor};

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
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub enum SyntaxEdge {
    UnchangedNode {
        depth_difference: u32,
        probably_punctuation: bool,
    },
    EnterUnchangedDelimiter {
        depth_difference: u32,
    },
    ReplacedComment {
        levenshtein_pct: u8,
    },
    ReplacedString {
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
            SyntaxEdge::ReplacedComment { levenshtein_pct }
            | SyntaxEdge::ReplacedString { levenshtein_pct } => {
                500 + u32::from(100 - levenshtein_pct)
            }
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
                    lhs = SyntaxTreeCursor::at(lhs.tree(), lhs_parent_id).next_sibling();
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
                    rhs = SyntaxTreeCursor::at(rhs.tree(), rhs_parent_id).next_sibling();
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
                lhs = SyntaxTreeCursor::at(lhs.tree(), lhs_id).next_sibling();
                rhs = SyntaxTreeCursor::at(rhs.tree(), rhs_id).next_sibling();
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
        if lhs_node.structural_hash() == rhs_node.structural_hash() {
            let depth_difference = (v.lhs.depth() as i32 - v.rhs.depth() as i32).unsigned_abs();
            let probably_punctuation = lhs_node.is_atom();

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

        // Both are comments or strings - check for replacement
        if let (Some(lhs_kind), Some(rhs_kind)) = (lhs_node.atom_kind(), rhs_node.atom_kind()) {
            let is_comment_or_string =
                |k: AtomKind| matches!(k, AtomKind::Comment | AtomKind::String);

            if is_comment_or_string(lhs_kind)
                && is_comment_or_string(rhs_kind)
                && lhs_kind == rhs_kind
            {
                if lhs_node.structural_hash() != rhs_node.structural_hash() {
                    // TODO: compute actual levenshtein when we have content access
                    let levenshtein_pct = 50;

                    let edge = if lhs_kind == AtomKind::Comment {
                        SyntaxEdge::ReplacedComment { levenshtein_pct }
                    } else {
                        SyntaxEdge::ReplacedString { levenshtein_pct }
                    };

                    let (lhs, rhs, parents) = pop_all_parents(
                        v.lhs.next_sibling(),
                        v.rhs.next_sibling(),
                        v.parents.clone(),
                    );

                    neighbours.push((edge, SyntaxVertex { lhs, rhs, parents }));
                }
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
