//! The Nfa we construct for each regex. Since the states are not
//! really of interest, we represent this just as a vector of labeled
//! edges.

use crate::lexer::re::Regex;
use regex_syntax::hir::{
    Class, ClassBytesRange, ClassUnicodeRange, Hir, HirKind, Literal, Repetition,
};
use std::char;
use std::fmt::{Debug, Error as FmtError, Formatter};
use std::ops::RangeInclusive;
use std::usize;

#[cfg(test)]
mod interpret;

#[cfg(test)]
mod test;

#[derive(Debug)]
pub struct Nfa {
    states: Vec<State>,
    edges: Edges,
}

#[allow(clippy::upper_case_acronyms)]
#[deprecated(since = "1.0.0", note = "Use `Nfa` instead")]
pub type NFA = Nfa;

/// An edge label representing a range of characters, inclusive. Note
/// that this range may contain some endpoints that are not valid
/// unicode, hence we store u32.
#[derive(Clone, Hash, PartialEq, Eq)]
pub struct Test {
    range: RangeInclusive<u32>,
}

impl PartialOrd for Test {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Test {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.start().cmp(&other.start()) {
            std::cmp::Ordering::Equal => self.end().cmp(&other.end()),
            ord => ord,
        }
    }
}

/// An "epsilon" edge -- no input
#[derive(Debug, PartialEq, Eq)]
pub struct Noop;

/// An "other" edge -- fallback if no other edges apply
#[derive(Debug, PartialEq, Eq)]
pub struct Other;

/// For each state, we just store the indices of the first char and
/// test edges, or usize::MAX if no such edge. You can then find all
/// edges by enumerating subsequent edges in the vectors until you
/// find one with a different `from` value.
#[derive(Debug)]
pub struct State {
    kind: StateKind,
    first_noop_edge: usize,
    first_test_edge: usize,
    first_other_edge: usize,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum StateKind {
    Accept,
    Reject,
    Neither,
}

#[derive(Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct NfaStateIndex(usize);

#[deprecated(since = "1.0.0", note = "Use `NfaStateIndex` instead")]
pub type NFAStateIndex = NfaStateIndex;

/// A set of edges for the state machine. Edges are kept sorted by the
/// type of label they have. Within a vector, all edges with the same
/// `from` are grouped together so they can be enumerated later (for
/// now we just ensure this during construction, but one could easily
/// sort).
#[derive(Debug)]
pub struct Edges {
    noop_edges: Vec<Edge<Noop>>,

    // edges where we are testing the character in some way; for any
    // given state, there should not be multiple edges with the same
    // test
    test_edges: Vec<Edge<Test>>,

    // fallback rules if no test_edge applies
    other_edges: Vec<Edge<Other>>,
}

#[derive(PartialEq, Eq)]
pub struct Edge<L> {
    pub from: NfaStateIndex,
    pub label: L,
    pub to: NfaStateIndex,
}

pub const ACCEPT: NfaStateIndex = NfaStateIndex(0);
pub const REJECT: NfaStateIndex = NfaStateIndex(1);
pub const START: NfaStateIndex = NfaStateIndex(2);

#[derive(Debug, PartialEq, Eq)]
pub enum NfaConstructionError {
    NamedCaptures,
    NonGreedy,
    LookAround,
    ByteRegex,
}

#[deprecated(since = "1.0.0", note = "Use `NfaConstructionError` instead")]
pub type NFAConstructionError = NfaConstructionError;

impl Nfa {
    pub fn from_re(regex: &Regex) -> Result<Nfa, NfaConstructionError> {
        let mut nfa = Nfa::new();
        let s0 = nfa.expr(regex, ACCEPT, REJECT)?;
        nfa.push_edge(START, Noop, s0);
        Ok(nfa)
    }

    ///////////////////////////////////////////////////////////////////////////
    // Public methods for querying an Nfa

    pub fn edges<L: EdgeLabel>(&self, from: NfaStateIndex) -> EdgeIterator<L> {
        let vec = L::vec(&self.edges);
        let first = *L::first(&self.states[from.0]);
        EdgeIterator {
            edges: vec,
            from,
            index: first,
        }
    }

    pub fn kind(&self, from: NfaStateIndex) -> StateKind {
        self.states[from.0].kind
    }

    pub fn is_accepting_state(&self, from: NfaStateIndex) -> bool {
        self.states[from.0].kind == StateKind::Accept
    }

    pub fn is_rejecting_state(&self, from: NfaStateIndex) -> bool {
        self.states[from.0].kind == StateKind::Reject
    }

    ///////////////////////////////////////////////////////////////////////////
    // Private methods for building an Nfa

    fn new() -> Nfa {
        let mut nfa = Nfa {
            states: vec![],
            edges: Edges {
                noop_edges: vec![],
                test_edges: vec![],
                other_edges: vec![],
            },
        };

        // reserve the ACCEPT, REJECT, and START states ahead of time
        assert!(nfa.new_state(StateKind::Accept) == ACCEPT);
        assert!(nfa.new_state(StateKind::Reject) == REJECT);
        assert!(nfa.new_state(StateKind::Neither) == START);

        // the ACCEPT state, given another token, becomes a REJECT
        nfa.push_edge(ACCEPT, Other, REJECT);

        // the REJECT state loops back to itself no matter what
        nfa.push_edge(REJECT, Other, REJECT);

        nfa
    }

    fn new_state(&mut self, kind: StateKind) -> NfaStateIndex {
        let index = self.states.len();

        // these edge indices will be patched later by patch_edges()
        self.states.push(State {
            kind,
            first_noop_edge: usize::MAX,
            first_test_edge: usize::MAX,
            first_other_edge: usize::MAX,
        });

        NfaStateIndex(index)
    }

    // pushes an edge: note that all outgoing edges from a particular
    // state should be pushed together, so that the edge vectors are
    // suitably sorted
    fn push_edge<L: EdgeLabel>(&mut self, from: NfaStateIndex, label: L, to: NfaStateIndex) {
        let edge_vec = L::vec_mut(&mut self.edges);
        let edge_index = edge_vec.len();
        edge_vec.push(Edge { from, label, to });

        // if this is the first edge from the `from` state, set the
        // index
        let first_index = L::first_mut(&mut self.states[from.0]);
        if *first_index == usize::MAX {
            *first_index = edge_index;
        } else {
            // otherwise, check that all edges are continuous
            assert_eq!(edge_vec[edge_index - 1].from, from);
        }
    }

    fn expr(
        &mut self,
        expr: &Hir,
        accept: NfaStateIndex,
        reject: NfaStateIndex,
    ) -> Result<NfaStateIndex, NfaConstructionError> {
        match expr.kind() {
            HirKind::Empty => Ok(accept),

            HirKind::Literal(Literal(l)) => Ok(l.iter().rev().fold(accept, |accept, &b| {
                let s0 = self.new_state(StateKind::Neither);
                self.push_edge(s0, Test::byte(b), accept);
                self.push_edge(s0, Other, reject);
                s0
            })),

            HirKind::Class(ref class) => {
                match *class {
                    Class::Unicode(ref uc) =>
                    // [s0] --c0--> [accept]
                    //  | |            ^
                    //  | |   ...      |
                    //  | |            |
                    //  | +---cn-------+
                    //  +---------------> [reject]
                    {
                        let s0 = self.new_state(StateKind::Neither);
                        for &range in uc.iter() {
                            let test: Test = range.into();
                            self.push_edge(s0, test, accept);
                        }
                        self.push_edge(s0, Other, reject);
                        Ok(s0)
                    }
                    //// Bytes are not supported
                    Class::Bytes(ref byte) => {
                        let s0 = self.new_state(StateKind::Neither);
                        for &range in byte.iter() {
                            let test: Test = range.into();
                            self.push_edge(s0, test, accept);
                        }
                        self.push_edge(s0, Other, reject);
                        Ok(s0)
                    }
                }
            }

            // currently we don't support any boundaries because
            // I was too lazy to code them up or think about them
            // Akin to anchors or wordboundaries
            HirKind::Look(_) => Err(NfaConstructionError::LookAround),

            // currently we treat all capture groups the same, whether they
            // capture or not; but we don't permit named groups,
            // in case we want to give them significance in the future
            HirKind::Capture(c) => match c.name {
                Some(_) => Err(NfaConstructionError::NamedCaptures),
                None => self.expr(&c.sub, accept, reject),
            },

            HirKind::Repetition(Repetition {
                min,
                max,
                greedy,
                sub,
            }) => {
                if !greedy {
                    // currently we always report the longest match possible
                    Err(NfaConstructionError::NonGreedy)
                } else {
                    match (min, max) {
                        (0, Some(1)) => self.optional_expr(sub, accept, reject),
                        (0, None) => self.star_expr(sub, accept, reject),
                        (1, None) => self.plus_expr(sub, accept, reject),
                        (_, Some(max)) if min == max => {
                            (0..*max).try_fold(accept, |s, _| self.expr(sub, s, reject))
                        }
                        (_, Some(max)) => {
                            let s = (*min..*max)
                                .try_fold(accept, |s, _| self.optional_expr(sub, s, reject))?;
                            (0..*min).try_fold(s, |s, _| self.expr(sub, s, reject))
                        }
                        (_, None) => {
                            // +---min times----+
                            // |                |
                            //
                            // [s0] --..e..-- [s1] --..e*..--> [accept]
                            //          |      |
                            //          |      v
                            //          +-> [reject]
                            self.star_expr(sub, accept, reject).and_then(|s| {
                                (0..*min).try_fold(s, |s, _| self.expr(sub, s, reject))
                            })
                        }
                    }
                }
            }

            HirKind::Concat(ref exprs) => {
                let mut s = accept;
                for expr in exprs.iter().rev() {
                    s = self.expr(expr, s, reject)?;
                }
                Ok(s)
            }

            HirKind::Alternation(ref exprs) => {
                // [s0] --exprs[0]--> [accept/reject]
                //   |                   ^
                //   |                   |
                //   +----exprs[..]------+
                //   |                   |
                //   |                   |
                //   +----exprs[n-1]-----+

                let s0 = self.new_state(StateKind::Neither);
                let targets: Vec<_> = exprs
                    .iter()
                    .map(|expr| self.expr(expr, accept, reject))
                    .collect::<Result<Vec<_>, _>>()?;

                // push edges from s0 all together so they are
                // adjacant in the edge array
                for target in targets {
                    self.push_edge(s0, Noop, target);
                }
                Ok(s0)
            }
        }
    }

    fn optional_expr(
        &mut self,
        expr: &Hir,
        accept: NfaStateIndex,
        reject: NfaStateIndex,
    ) -> Result<NfaStateIndex, NfaConstructionError> {
        // [s0] ----> [accept]
        //   |           ^
        //   v           |
        // [s1] --...----+
        //         |
        //         v
        //      [reject]

        let s1 = self.expr(expr, accept, reject)?;

        let s0 = self.new_state(StateKind::Neither);
        self.push_edge(s0, Noop, accept); // they might supply nothing
        self.push_edge(s0, Noop, s1);

        Ok(s0)
    }

    fn star_expr(
        &mut self,
        expr: &Hir,
        accept: NfaStateIndex,
        reject: NfaStateIndex,
    ) -> Result<NfaStateIndex, NfaConstructionError> {
        // [s0] ----> [accept]
        //  | ^
        //  | |
        //  | +----------+
        //  v            |
        // [s1] --...----+
        //         |
        //         v
        //      [reject]

        let s0 = self.new_state(StateKind::Neither);

        let s1 = self.expr(expr, s0, reject)?;

        self.push_edge(s0, Noop, accept);
        self.push_edge(s0, Noop, s1);

        Ok(s0)
    }

    fn plus_expr(
        &mut self,
        expr: &Hir,
        accept: NfaStateIndex,
        reject: NfaStateIndex,
    ) -> Result<NfaStateIndex, NfaConstructionError> {
        //            [accept]
        //               ^
        //               |
        //    +----------+
        //    v          |
        // [s0] --...--[s1]
        //         |
        //         v
        //      [reject]

        let s1 = self.new_state(StateKind::Neither);

        let s0 = self.expr(expr, s1, reject)?;

        self.push_edge(s1, Noop, accept);
        self.push_edge(s1, Noop, s0);

        Ok(s0)
    }
}

pub trait EdgeLabel: Sized {
    fn vec_mut(nfa: &mut Edges) -> &mut Vec<Edge<Self>>;
    fn vec(nfa: &Edges) -> &Vec<Edge<Self>>;
    fn first_mut(state: &mut State) -> &mut usize;
    fn first(state: &State) -> &usize;
}

impl EdgeLabel for Noop {
    fn vec_mut(nfa: &mut Edges) -> &mut Vec<Edge<Noop>> {
        &mut nfa.noop_edges
    }
    fn first_mut(state: &mut State) -> &mut usize {
        &mut state.first_noop_edge
    }
    fn vec(nfa: &Edges) -> &Vec<Edge<Noop>> {
        &nfa.noop_edges
    }
    fn first(state: &State) -> &usize {
        &state.first_noop_edge
    }
}

impl EdgeLabel for Other {
    fn vec_mut(nfa: &mut Edges) -> &mut Vec<Edge<Other>> {
        &mut nfa.other_edges
    }
    fn first_mut(state: &mut State) -> &mut usize {
        &mut state.first_other_edge
    }
    fn vec(nfa: &Edges) -> &Vec<Edge<Other>> {
        &nfa.other_edges
    }
    fn first(state: &State) -> &usize {
        &state.first_other_edge
    }
}

impl EdgeLabel for Test {
    fn vec_mut(nfa: &mut Edges) -> &mut Vec<Edge<Test>> {
        &mut nfa.test_edges
    }
    fn first_mut(state: &mut State) -> &mut usize {
        &mut state.first_test_edge
    }
    fn vec(nfa: &Edges) -> &Vec<Edge<Test>> {
        &nfa.test_edges
    }
    fn first(state: &State) -> &usize {
        &state.first_test_edge
    }
}

pub struct EdgeIterator<'nfa, L: EdgeLabel + 'nfa> {
    edges: &'nfa [Edge<L>],
    from: NfaStateIndex,
    index: usize,
}

impl<'nfa, L: EdgeLabel> Iterator for EdgeIterator<'nfa, L> {
    type Item = &'nfa Edge<L>;

    fn next(&mut self) -> Option<&'nfa Edge<L>> {
        let index = self.index;
        if index == usize::MAX {
            return None;
        }

        let next_index = index + 1;
        if next_index >= self.edges.len() || self.edges[next_index].from != self.from {
            self.index = usize::MAX;
        } else {
            self.index = next_index;
        }

        Some(&self.edges[index])
    }
}

impl Test {
    pub fn new(range: RangeInclusive<u32>) -> Test {
        Test { range }
    }

    pub fn start(&self) -> u32 {
        *self.range.start()
    }

    pub fn end(&self) -> u32 {
        *self.range.end()
    }

    pub fn char(c: char) -> Test {
        let c = c as u32;
        Test { range: c..=c }
    }

    pub fn byte(b: u8) -> Test {
        let b = b as u32;
        Test { range: b..=b }
    }

    pub fn inclusive_range(s: char, e: char) -> Test {
        Test {
            range: s as u32..=e as u32,
        }
    }

    pub fn inclusive_byte_range(s: u8, e: u8) -> Test {
        Test {
            range: s as u32..=e as u32,
        }
    }

    pub fn exclusive_range(s: char, e: char) -> Test {
        Test {
            range: s as u32..=e as u32 - 1,
        }
    }

    pub fn is_char(&self) -> bool {
        self.len() == 1
    }

    pub fn len(&self) -> u32 {
        // The reason we don't have a RangeInclusive::len is because it panics if the range is 0..=u32::max
        // Akin to https://github.com/rust-lang/rust/issues/36386
        // Plus one because the range is inclusive
        self.end() + 1 - self.start()
    }

    pub fn contains_u32(&self, c: u32) -> bool {
        self.range.contains(&c)
    }

    pub fn contains_char(&self, c: char) -> bool {
        self.contains_u32(c as u32)
    }

    pub fn intersects(&self, r: &Test) -> bool {
        !self.is_empty()
            && !r.is_empty()
            && (self.contains_u32(r.start()) || r.contains_u32(self.start()))
    }

    pub fn is_disjoint(&self, r: &Test) -> bool {
        !self.intersects(r)
    }

    pub fn is_empty(&self) -> bool {
        self.range.is_empty()
    }
}

impl From<ClassUnicodeRange> for Test {
    fn from(range: ClassUnicodeRange) -> Test {
        Test::inclusive_range(range.start(), range.end())
    }
}

impl From<ClassBytesRange> for Test {
    fn from(range: ClassBytesRange) -> Test {
        Test::inclusive_byte_range(range.start(), range.end())
    }
}

impl Debug for Test {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), FmtError> {
        match (char::from_u32(self.start()), char::from_u32(self.end())) {
            (Some(start), Some(end)) => {
                if self.is_char() {
                    if ".[]()?+*!".contains(start) {
                        write!(fmt, "\\{}", start)
                    } else {
                        write!(fmt, "{}", start)
                    }
                } else {
                    write!(fmt, "[{:?}..={:?}]", start, end)
                }
            }
            _ => write!(fmt, "[{:?}..]{:?}]", self.start(), self.end()),
        }
    }
}

impl Debug for NfaStateIndex {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), FmtError> {
        write!(fmt, "Nfa{}", self.0)
    }
}

impl<L: Debug> Debug for Edge<L> {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), FmtError> {
        write!(fmt, "{:?} -{:?}-> {:?}", self.from, self.label, self.to)
    }
}
