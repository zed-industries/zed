//! A graph representation for computing tree diffs.

use std::{
    cell::{Cell, RefCell},
    cmp::min,
    fmt,
    hash::{Hash, Hasher},
};

use bumpalo::Bump;
use hashbrown::hash_map::RawEntryMut;
use smallvec::{smallvec, SmallVec};
use strsim::normalized_levenshtein;

use self::Edge::*;
use crate::{
    diff::{
        changes::{insert_deep_unchanged, ChangeKind, ChangeMap},
        stack::Stack,
    },
    hash::DftHashMap,
    parse::syntax::{AtomKind, Syntax, SyntaxId},
};

/// A vertex in a directed acyclic graph that represents a diff.
///
/// Each vertex represents two pointers: one to the next unmatched LHS
/// syntax, and one to the next unmatched RHS syntax.
///
/// For example, suppose we have `X A` on the LHS and `A` on the
/// RHS. Our start vertex looks like this.
///
/// ```text
/// LHS: X A     RHS: A
///      ^            ^
/// ```
///
/// From this vertex, we could take [`Edge::NovelAtomLHS`], bringing
/// us to this vertex.
///
/// ```text
/// LHS: X A     RHS: A
///        ^          ^
/// ```
///
/// Alternatively, we could take the [`Edge::NovelAtomRHS`], bringing us
/// to this vertex.
///
/// ```text
/// LHS: X A     RHS: A
///      ^              ^
/// ```
///
/// Vertices are arena allocated (the 'v lifetime) and have references
/// to syntax nodes (the 's lifetime).
#[derive(Debug, Clone)]
pub(crate) struct Vertex<'s, 'v> {
    pub(crate) neighbours: RefCell<Option<&'v [(Edge, &'v Vertex<'s, 'v>)]>>,
    pub(crate) predecessor: Cell<Option<(u32, &'v Vertex<'s, 'v>)>>,
    // TODO: experiment with storing SyntaxId only, and have a HashMap
    // from SyntaxId to &Syntax.
    pub(crate) lhs_syntax: Option<&'s Syntax<'s>>,
    pub(crate) rhs_syntax: Option<&'s Syntax<'s>>,
    parents: Stack<'v, EnteredDelimiter<'s, 'v>>,
    lhs_parent_id: Option<SyntaxId>,
    rhs_parent_id: Option<SyntaxId>,
}

impl PartialEq for Vertex<'_, '_> {
    fn eq(&self, other: &Self) -> bool {
        // Strictly speaking, we should compare the whole
        // EnteredDelimiter stack, not just the immediate
        // parents. By taking the immediate parent, we have
        // vertices with different stacks that are 'equal'.
        //
        // This makes the graph traversal path dependent: the
        // first vertex we see 'wins', and we use it for deciding
        // how we can pop.
        //
        // In practice this seems to work well. The first vertex
        // has the lowest cost, so has the most PopBoth
        // occurrences, which is the best outcome.
        //
        // Handling this properly would require considering many
        // more vertices to be distinct, exponentially increasing
        // the graph size relative to tree depth.
        let b0 = match (self.lhs_syntax, other.lhs_syntax) {
            (Some(s0), Some(s1)) => s0.id() == s1.id(),
            (None, None) => self.lhs_parent_id == other.lhs_parent_id,
            _ => false,
        };
        let b1 = match (self.rhs_syntax, other.rhs_syntax) {
            (Some(s0), Some(s1)) => s0.id() == s1.id(),
            (None, None) => self.rhs_parent_id == other.rhs_parent_id,
            _ => false,
        };
        // We do want to distinguish whether we can pop each side
        // independently though. Without this, if we find a case
        // where we can pop sides together, we don't consider the
        // case where we get a better diff by popping each side
        // separately.
        let b2 = can_pop_either_parent(&self.parents) == can_pop_either_parent(&other.parents);

        b0 && b1 && b2
    }
}
impl Eq for Vertex<'_, '_> {}

impl Hash for Vertex<'_, '_> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.lhs_syntax.map(|node| node.id()).hash(state);
        self.rhs_syntax.map(|node| node.id()).hash(state);

        self.lhs_parent_id.hash(state);
        self.rhs_parent_id.hash(state);
        can_pop_either_parent(&self.parents).hash(state);
    }
}

/// Tracks entering syntax List nodes.
#[derive(Clone, PartialEq)]
enum EnteredDelimiter<'s, 'v> {
    /// If we've entered the LHS or RHS separately, we can pop either
    /// side independently.
    ///
    /// Assumes that at least one stack is non-empty.
    PopEither((Stack<'v, &'s Syntax<'s>>, Stack<'v, &'s Syntax<'s>>)),
    /// If we've entered the LHS and RHS together, we must pop both
    /// sides together too. Otherwise we'd consider the following case to have no changes.
    ///
    /// ```text
    /// Old: (a b c)
    /// New: (a b) c
    /// ```
    PopBoth((&'s Syntax<'s>, &'s Syntax<'s>)),
}

impl fmt::Debug for EnteredDelimiter<'_, '_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let desc = match self {
            Self::PopEither((lhs_delims, rhs_delims)) => {
                format!(
                    "PopEither(lhs count: {}, rhs count: {})",
                    lhs_delims.size(),
                    rhs_delims.size()
                )
            }
            Self::PopBoth(_) => "PopBoth".to_owned(),
        };
        f.write_str(&desc)
    }
}

fn push_both_delimiters<'s, 'v>(
    entered: &Stack<'v, EnteredDelimiter<'s, 'v>>,
    lhs_delim: &'s Syntax<'s>,
    rhs_delim: &'s Syntax<'s>,
    alloc: &'v Bump,
) -> Stack<'v, EnteredDelimiter<'s, 'v>> {
    entered.push(EnteredDelimiter::PopBoth((lhs_delim, rhs_delim)), alloc)
}

fn can_pop_either_parent(entered: &Stack<EnteredDelimiter>) -> bool {
    matches!(entered.peek(), Some(EnteredDelimiter::PopEither(_)))
}

fn try_pop_both<'s, 'v>(
    entered: &Stack<'v, EnteredDelimiter<'s, 'v>>,
) -> Option<(
    &'s Syntax<'s>,
    &'s Syntax<'s>,
    Stack<'v, EnteredDelimiter<'s, 'v>>,
)> {
    match entered.peek() {
        Some(EnteredDelimiter::PopBoth((lhs_delim, rhs_delim))) => {
            Some((lhs_delim, rhs_delim, entered.pop().unwrap()))
        }
        _ => None,
    }
}

fn try_pop_lhs<'s, 'v>(
    entered: &Stack<'v, EnteredDelimiter<'s, 'v>>,
    alloc: &'v Bump,
) -> Option<(&'s Syntax<'s>, Stack<'v, EnteredDelimiter<'s, 'v>>)> {
    match entered.peek() {
        Some(EnteredDelimiter::PopEither((lhs_delims, rhs_delims))) => match lhs_delims.peek() {
            Some(lhs_delim) => {
                let mut entered = entered.pop().unwrap();
                let new_lhs_delims = lhs_delims.pop().unwrap();

                if !new_lhs_delims.is_empty() || !rhs_delims.is_empty() {
                    entered = entered.push(
                        EnteredDelimiter::PopEither((new_lhs_delims, rhs_delims.clone())),
                        alloc,
                    );
                }

                Some((lhs_delim, entered))
            }
            None => None,
        },
        _ => None,
    }
}

fn try_pop_rhs<'s, 'v>(
    entered: &Stack<'v, EnteredDelimiter<'s, 'v>>,
    alloc: &'v Bump,
) -> Option<(&'s Syntax<'s>, Stack<'v, EnteredDelimiter<'s, 'v>>)> {
    match entered.peek() {
        Some(EnteredDelimiter::PopEither((lhs_delims, rhs_delims))) => match rhs_delims.peek() {
            Some(rhs_delim) => {
                let mut entered = entered.pop().unwrap();
                let new_rhs_delims = rhs_delims.pop().unwrap();

                if !lhs_delims.is_empty() || !new_rhs_delims.is_empty() {
                    entered = entered.push(
                        EnteredDelimiter::PopEither((lhs_delims.clone(), new_rhs_delims)),
                        alloc,
                    );
                }

                Some((rhs_delim, entered))
            }
            None => None,
        },
        _ => None,
    }
}

fn push_lhs_delimiter<'s, 'v>(
    entered: &Stack<'v, EnteredDelimiter<'s, 'v>>,
    delimiter: &'s Syntax<'s>,
    alloc: &'v Bump,
) -> Stack<'v, EnteredDelimiter<'s, 'v>> {
    match entered.peek() {
        Some(EnteredDelimiter::PopEither((lhs_delims, rhs_delims))) => entered.pop().unwrap().push(
            EnteredDelimiter::PopEither((lhs_delims.push(delimiter, alloc), rhs_delims.clone())),
            alloc,
        ),
        _ => entered.push(
            EnteredDelimiter::PopEither((Stack::new().push(delimiter, alloc), Stack::new())),
            alloc,
        ),
    }
}

fn push_rhs_delimiter<'s, 'v>(
    entered: &Stack<'v, EnteredDelimiter<'s, 'v>>,
    delimiter: &'s Syntax<'s>,
    alloc: &'v Bump,
) -> Stack<'v, EnteredDelimiter<'s, 'v>> {
    match entered.peek() {
        Some(EnteredDelimiter::PopEither((lhs_delims, rhs_delims))) => entered.pop().unwrap().push(
            EnteredDelimiter::PopEither((lhs_delims.clone(), rhs_delims.push(delimiter, alloc))),
            alloc,
        ),
        _ => entered.push(
            EnteredDelimiter::PopEither((Stack::new(), Stack::new().push(delimiter, alloc))),
            alloc,
        ),
    }
}

impl<'s, 'v> Vertex<'s, 'v> {
    pub(crate) fn is_end(&self) -> bool {
        self.lhs_syntax.is_none() && self.rhs_syntax.is_none() && self.parents.is_empty()
    }

    pub(crate) fn new(
        lhs_syntax: Option<&'s Syntax<'s>>,
        rhs_syntax: Option<&'s Syntax<'s>>,
    ) -> Self {
        let parents = Stack::new();
        Vertex {
            neighbours: RefCell::new(None),
            predecessor: Cell::new(None),
            lhs_syntax,
            rhs_syntax,
            parents,
            lhs_parent_id: None,
            rhs_parent_id: None,
        }
    }
}

/// An edge in our graph, with an associated [`cost`](Edge::cost).
///
/// A syntax node can always be marked as novel, so a vertex will have
/// at least a NovelFoo edge. Depending on the syntax nodes of the
/// current [`Vertex`], other edges may also be available.
///
/// See [`set_neighbours`] for all the edges available for a given `Vertex`.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub(crate) enum Edge {
    UnchangedNode {
        depth_difference: u32,
        /// Is this node just punctuation? We penalise this case,
        /// because it's more useful to match e.g. a variable name
        /// than a comma.
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
    NovelAtomLHS {},
    NovelAtomRHS {},
    // TODO: An EnterNovelDelimiterBoth edge might help performance
    // rather doing LHS and RHS separately.
    EnterNovelDelimiterLHS {},
    EnterNovelDelimiterRHS {},
}

impl Edge {
    pub(crate) fn cost(self) -> u32 {
        match self {
            // Matching nodes is always best.
            UnchangedNode {
                depth_difference,
                probably_punctuation,
            } => {
                // TODO: Perhaps prefer matching longer strings? It's
                // probably easier to read.

                // The cost for unchanged nodes can be as low as 1,
                // but we penalise nodes that have a different depth
                // difference, capped at 40.
                let base = min(40, depth_difference + 1);

                // If the node is only punctuation, increase the
                // cost. It's better to have unchanged variable names
                // and novel punctuation than the reverse.
                //
                // We want a sufficiently large punctuation cost such
                // that unchanged variables always win, even if there
                // are replacement edges elsewhere.
                //
                // Replacement edges have a cost between 500 and 600,
                // so they can be up to 100 less than two novel nodes.
                // If we have replacements either side of a node
                // (e.g. see comma_and_comment_1.js), then that's
                // potentially a cost difference of 200.
                base + if probably_punctuation { 200 } else { 0 }
            }
            // Matching an outer delimiter is good.
            EnterUnchangedDelimiter { depth_difference } => 100 + min(40, depth_difference),

            // Otherwise, we've added/removed a node.
            NovelAtomLHS {} | NovelAtomRHS {} => 300,
            EnterNovelDelimiterLHS { .. } | EnterNovelDelimiterRHS { .. } => 300,
            // Replacing a comment is better than treating it as
            // novel. However, since ReplacedComment is an alternative
            // to NovelAtomLHS and NovelAtomRHS, we need to be
            // slightly less than 2 * 300.
            ReplacedComment { levenshtein_pct } | ReplacedString { levenshtein_pct } => {
                500 + u32::from(100 - levenshtein_pct)
            }
        }
    }
}

fn allocate_if_new<'s, 'v>(
    v: Vertex<'s, 'v>,
    alloc: &'v Bump,
    seen: &mut DftHashMap<&Vertex<'s, 'v>, SmallVec<[&'v Vertex<'s, 'v>; 2]>>,
) -> &'v Vertex<'s, 'v> {
    // We use the entry API so that we only need to do a single lookup
    // for access and insert.
    match seen.raw_entry_mut().from_key(&v) {
        RawEntryMut::Occupied(mut occupied) => {
            let existing = occupied.get_mut();

            // Don't explore more than two possible parenthesis
            // nestings for each syntax node pair.
            if let Some(allocated) = existing.last() {
                if existing.len() >= 2 {
                    return allocated;
                }
            }

            // If we have seen exactly this graph node before, even
            // considering parenthesis matching, return it.
            for existing_node in existing.iter() {
                if existing_node.parents == v.parents {
                    return existing_node;
                }
            }

            // We haven't reached the graph node limit yet, allocate a
            // new one.
            let allocated = alloc.alloc(v);
            existing.push(allocated);
            allocated
        }
        RawEntryMut::Vacant(vacant) => {
            let allocated = alloc.alloc(v);

            // We know that this vec will never have more than 2
            // nodes, and this code is very hot, so use a smallvec.
            //
            // We still use a vec to enable experiments with the value
            // of how many possible parenthesis nestings to explore.
            let existing: SmallVec<[&'v Vertex<'s, 'v>; 2]> = smallvec![&*allocated];

            vacant.insert(allocated, existing);
            allocated
        }
    }
}

/// Does this node look like punctuation?
///
/// This check is deliberately conservative, because it's hard to
/// accurately recognise punctuation in a language-agnostic way.
fn looks_like_punctuation(node: &Syntax) -> bool {
    match node {
        Syntax::Atom { content, .. } => content == "," || content == ";" || content == ".",
        _ => false,
    }
}

/// Pop as many parents of `lhs_node` and `rhs_node` as
/// possible. Return the new syntax nodes and parents.
fn pop_all_parents<'s, 'v>(
    lhs_node: Option<&'s Syntax<'s>>,
    rhs_node: Option<&'s Syntax<'s>>,
    lhs_parent_id: Option<SyntaxId>,
    rhs_parent_id: Option<SyntaxId>,
    parents: &Stack<'v, EnteredDelimiter<'s, 'v>>,
    alloc: &'v Bump,
) -> (
    Option<&'s Syntax<'s>>,
    Option<&'s Syntax<'s>>,
    Option<SyntaxId>,
    Option<SyntaxId>,
    Stack<'v, EnteredDelimiter<'s, 'v>>,
) {
    let mut lhs_node = lhs_node;
    let mut rhs_node = rhs_node;
    let mut lhs_parent_id = lhs_parent_id;
    let mut rhs_parent_id = rhs_parent_id;
    let mut parents = parents.clone();

    loop {
        if lhs_node.is_none() {
            if let Some((lhs_parent, parents_next)) = try_pop_lhs(&parents, alloc) {
                // Move to next after LHS parent.

                // Continue from sibling of parent.
                lhs_node = lhs_parent.next_sibling();
                lhs_parent_id = lhs_parent.parent().map(Syntax::id);
                parents = parents_next;
                continue;
            }
        }

        if rhs_node.is_none() {
            if let Some((rhs_parent, parents_next)) = try_pop_rhs(&parents, alloc) {
                // Move to next after RHS parent.

                // Continue from sibling of parent.
                rhs_node = rhs_parent.next_sibling();
                rhs_parent_id = rhs_parent.parent().map(Syntax::id);
                parents = parents_next;
                continue;
            }
        }

        if lhs_node.is_none() && rhs_node.is_none() {
            // We have exhausted all the nodes on both lists, so we can
            // move up to the parent node.

            // Continue from sibling of parent.
            if let Some((lhs_parent, rhs_parent, parents_next)) = try_pop_both(&parents) {
                lhs_node = lhs_parent.next_sibling();
                rhs_node = rhs_parent.next_sibling();
                lhs_parent_id = lhs_parent.parent().map(Syntax::id);
                rhs_parent_id = rhs_parent.parent().map(Syntax::id);
                parents = parents_next;
                continue;
            }
        }

        break;
    }

    (lhs_node, rhs_node, lhs_parent_id, rhs_parent_id, parents)
}

/// Compute the neighbours of `v` if we haven't previously done so,
/// and write them to the .neighbours cell inside `v`.
pub(crate) fn set_neighbours<'s, 'v>(
    v: &Vertex<'s, 'v>,
    alloc: &'v Bump,
    seen: &mut DftHashMap<&Vertex<'s, 'v>, SmallVec<[&'v Vertex<'s, 'v>; 2]>>,
) {
    if v.neighbours.borrow().is_some() {
        return;
    }

    // There are only seven pushes in this function, so that's sufficient.
    let mut neighbours: Vec<(Edge, &Vertex)> = Vec::with_capacity(7);

    if let (Some(lhs_syntax), Some(rhs_syntax)) = (&v.lhs_syntax, &v.rhs_syntax) {
        if lhs_syntax == rhs_syntax {
            let depth_difference = (lhs_syntax.num_ancestors() as i32
                - rhs_syntax.num_ancestors() as i32)
                .unsigned_abs();

            let probably_punctuation = looks_like_punctuation(lhs_syntax);

            // Both nodes are equal, the happy case.
            let (lhs_syntax, rhs_syntax, lhs_parent_id, rhs_parent_id, parents) = pop_all_parents(
                lhs_syntax.next_sibling(),
                rhs_syntax.next_sibling(),
                v.lhs_parent_id,
                v.rhs_parent_id,
                &v.parents,
                alloc,
            );

            neighbours.push((
                UnchangedNode {
                    depth_difference,
                    probably_punctuation,
                },
                allocate_if_new(
                    Vertex {
                        neighbours: RefCell::new(None),
                        predecessor: Cell::new(None),
                        lhs_syntax,
                        rhs_syntax,
                        parents,
                        lhs_parent_id,
                        rhs_parent_id,
                    },
                    alloc,
                    seen,
                ),
            ));
        }

        if let (
            Syntax::List {
                open_content: lhs_open_content,
                close_content: lhs_close_content,
                children: lhs_children,
                ..
            },
            Syntax::List {
                open_content: rhs_open_content,
                close_content: rhs_close_content,
                children: rhs_children,
                ..
            },
        ) = (lhs_syntax, rhs_syntax)
        {
            // The list delimiters are equal, but children may not be.
            if lhs_open_content == rhs_open_content && lhs_close_content == rhs_close_content {
                let lhs_next = lhs_children.first().copied();
                let rhs_next = rhs_children.first().copied();

                // TODO: be consistent between parents_next and next_parents.
                let parents_next = push_both_delimiters(&v.parents, lhs_syntax, rhs_syntax, alloc);

                let depth_difference = (lhs_syntax.num_ancestors() as i32
                    - rhs_syntax.num_ancestors() as i32)
                    .unsigned_abs();

                // When we enter a list, we may need to immediately
                // pop several levels if the list has no children.
                let (lhs_syntax, rhs_syntax, lhs_parent_id, rhs_parent_id, parents) =
                    pop_all_parents(
                        lhs_next,
                        rhs_next,
                        Some(lhs_syntax.id()),
                        Some(rhs_syntax.id()),
                        &parents_next,
                        alloc,
                    );

                neighbours.push((
                    EnterUnchangedDelimiter { depth_difference },
                    allocate_if_new(
                        Vertex {
                            neighbours: RefCell::new(None),
                            predecessor: Cell::new(None),
                            lhs_syntax,
                            rhs_syntax,
                            parents,
                            lhs_parent_id,
                            rhs_parent_id,
                        },
                        alloc,
                        seen,
                    ),
                ));
            }
        }

        if let (
            Syntax::Atom {
                content: lhs_content,
                kind: lhs_kind @ AtomKind::Comment | lhs_kind @ AtomKind::String(_),
                ..
            },
            Syntax::Atom {
                content: rhs_content,
                kind: rhs_kind @ AtomKind::Comment | rhs_kind @ AtomKind::String(_),
                ..
            },
        ) = (lhs_syntax, rhs_syntax)
        {
            // Both sides are comments/both sides are strings and
            // their content is reasonably similar.
            if lhs_kind == rhs_kind && lhs_content != rhs_content {
                let levenshtein_pct =
                    (normalized_levenshtein(lhs_content, rhs_content) * 100.0).round() as u8;
                let edge = if lhs_kind == &AtomKind::Comment {
                    ReplacedComment { levenshtein_pct }
                } else {
                    ReplacedString { levenshtein_pct }
                };

                let (lhs_syntax, rhs_syntax, lhs_parent_id, rhs_parent_id, parents) =
                    pop_all_parents(
                        lhs_syntax.next_sibling(),
                        rhs_syntax.next_sibling(),
                        v.lhs_parent_id,
                        v.rhs_parent_id,
                        &v.parents,
                        alloc,
                    );
                neighbours.push((
                    edge,
                    allocate_if_new(
                        Vertex {
                            neighbours: RefCell::new(None),
                            predecessor: Cell::new(None),
                            lhs_syntax,
                            rhs_syntax,
                            parents,
                            lhs_parent_id,
                            rhs_parent_id,
                        },
                        alloc,
                        seen,
                    ),
                ));
            }
        }
    }

    if let Some(lhs_syntax) = &v.lhs_syntax {
        match lhs_syntax {
            // Step over this novel atom.
            Syntax::Atom { .. } => {
                let (lhs_syntax, rhs_syntax, lhs_parent_id, rhs_parent_id, parents) =
                    pop_all_parents(
                        lhs_syntax.next_sibling(),
                        v.rhs_syntax,
                        v.lhs_parent_id,
                        v.rhs_parent_id,
                        &v.parents,
                        alloc,
                    );

                neighbours.push((
                    NovelAtomLHS {},
                    allocate_if_new(
                        Vertex {
                            neighbours: RefCell::new(None),
                            predecessor: Cell::new(None),
                            lhs_syntax,
                            rhs_syntax,
                            parents,
                            lhs_parent_id,
                            rhs_parent_id,
                        },
                        alloc,
                        seen,
                    ),
                ));
            }
            // Step into this partially/fully novel list.
            Syntax::List { children, .. } => {
                let lhs_next = children.first().copied();

                let parents_next = push_lhs_delimiter(&v.parents, lhs_syntax, alloc);

                let (lhs_syntax, rhs_syntax, lhs_parent_id, rhs_parent_id, parents) =
                    pop_all_parents(
                        lhs_next,
                        v.rhs_syntax,
                        Some(lhs_syntax.id()),
                        v.rhs_parent_id,
                        &parents_next,
                        alloc,
                    );

                neighbours.push((
                    EnterNovelDelimiterLHS {},
                    allocate_if_new(
                        Vertex {
                            neighbours: RefCell::new(None),
                            predecessor: Cell::new(None),
                            lhs_syntax,
                            rhs_syntax,
                            parents,
                            lhs_parent_id,
                            rhs_parent_id,
                        },
                        alloc,
                        seen,
                    ),
                ));
            }
        }
    }

    if let Some(rhs_syntax) = &v.rhs_syntax {
        match rhs_syntax {
            // Step over this novel atom.
            Syntax::Atom { .. } => {
                let (lhs_syntax, rhs_syntax, lhs_parent_id, rhs_parent_id, parents) =
                    pop_all_parents(
                        v.lhs_syntax,
                        rhs_syntax.next_sibling(),
                        v.lhs_parent_id,
                        v.rhs_parent_id,
                        &v.parents,
                        alloc,
                    );

                neighbours.push((
                    NovelAtomRHS {},
                    allocate_if_new(
                        Vertex {
                            neighbours: RefCell::new(None),
                            predecessor: Cell::new(None),
                            lhs_syntax,
                            rhs_syntax,
                            parents,
                            lhs_parent_id,
                            rhs_parent_id,
                        },
                        alloc,
                        seen,
                    ),
                ));
            }
            // Step into this partially/fully novel list.
            Syntax::List { children, .. } => {
                let rhs_next = children.first().copied();
                let parents_next = push_rhs_delimiter(&v.parents, rhs_syntax, alloc);

                let (lhs_syntax, rhs_syntax, lhs_parent_id, rhs_parent_id, parents) =
                    pop_all_parents(
                        v.lhs_syntax,
                        rhs_next,
                        v.lhs_parent_id,
                        Some(rhs_syntax.id()),
                        &parents_next,
                        alloc,
                    );

                neighbours.push((
                    EnterNovelDelimiterRHS {},
                    allocate_if_new(
                        Vertex {
                            neighbours: RefCell::new(None),
                            predecessor: Cell::new(None),
                            lhs_syntax,
                            rhs_syntax,
                            parents,
                            lhs_parent_id,
                            rhs_parent_id,
                        },
                        alloc,
                        seen,
                    ),
                ));
            }
        }
    }
    assert!(
        !neighbours.is_empty(),
        "Must always find some next steps if node is not the end"
    );

    v.neighbours
        .replace(Some(alloc.alloc_slice_copy(neighbours.as_slice())));
}

pub(crate) fn populate_change_map<'s, 'v>(
    route: &[(Edge, &'v Vertex<'s, 'v>)],
    change_map: &mut ChangeMap<'s>,
) {
    for (e, v) in route {
        match e {
            UnchangedNode { .. } => {
                // No change on this node or its children.
                let lhs = v.lhs_syntax.unwrap();
                let rhs = v.rhs_syntax.unwrap();

                insert_deep_unchanged(lhs, rhs, change_map);
                insert_deep_unchanged(rhs, lhs, change_map);
            }
            EnterUnchangedDelimiter { .. } => {
                // No change on the outer delimiter, but children may
                // have changed.
                let lhs = v.lhs_syntax.unwrap();
                let rhs = v.rhs_syntax.unwrap();
                change_map.insert(lhs, ChangeKind::Unchanged(rhs));
                change_map.insert(rhs, ChangeKind::Unchanged(lhs));
            }
            ReplacedComment { levenshtein_pct } | ReplacedString { levenshtein_pct } => {
                let lhs = v.lhs_syntax.unwrap();
                let rhs = v.rhs_syntax.unwrap();
                let change_kind = |first, second| {
                    if let ReplacedComment { .. } = e {
                        ChangeKind::ReplacedComment(first, second)
                    } else {
                        ChangeKind::ReplacedString(first, second)
                    }
                };

                if *levenshtein_pct > 20 {
                    change_map.insert(lhs, change_kind(lhs, rhs));
                    change_map.insert(rhs, change_kind(rhs, lhs));
                } else {
                    change_map.insert(lhs, ChangeKind::Novel);
                    change_map.insert(rhs, ChangeKind::Novel);
                }
            }
            NovelAtomLHS { .. } | EnterNovelDelimiterLHS { .. } => {
                let lhs = v.lhs_syntax.unwrap();
                change_map.insert(lhs, ChangeKind::Novel);
            }
            NovelAtomRHS { .. } | EnterNovelDelimiterRHS { .. } => {
                let rhs = v.rhs_syntax.unwrap();
                change_map.insert(rhs, ChangeKind::Novel);
            }
        }
    }
}
