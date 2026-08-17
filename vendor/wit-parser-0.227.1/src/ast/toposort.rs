use crate::ast::{Id, Span};
use anyhow::Result;
use indexmap::IndexMap;
use std::collections::BinaryHeap;
use std::fmt;
use std::mem;

#[derive(Default, Clone)]
struct State {
    /// Number of outbound edges from this node which have still not been
    /// processed into the topological ordering.
    outbound_remaining: usize,

    /// Indices of nodes that depend on this one, used when this node is added
    /// to the binary heap to decrement `outbound_remaining`.
    reverse_deps: Vec<usize>,
}

/// Performs a topological sort of the `deps` provided, returning the order in
/// which to visit the nodes in reverse-dep order.
///
/// This sort goes one level further as well to produce a stable ordering
/// regardless of the input edges so long as the structure of the graph has
/// changed. Notably the nodes are sorted, by name, in the output in addition to
/// being sorted in dependency order. This is done to assist with round-tripping
/// documents where new edges are discovered during world elaboration that
/// doesn't change the dependency graph but can change the dependency listings
/// between serializations.
///
/// The algorithm chosen here to do this is:
///
/// * Build some metadata about all nodes including their count of outbound
///   edges remaining to be added to the order and a reverse dependency list.
/// * Collect all nodes with 0 outbound edges into a binary heap.
/// * Pop from the binary heap and decrement outbound edges that depend on
///   this node.
/// * Iterate until the dependency ordering is the same size as the dependency
///   array.
///
/// This sort will also detect when dependencies are missing or when cycles are
/// present and return an error.
pub fn toposort<'a>(
    kind: &str,
    deps: &IndexMap<&'a str, Vec<Id<'a>>>,
) -> Result<Vec<&'a str>, Error> {
    // Initialize a `State` per-node with the number of outbound edges and
    // additionally filling out the `reverse_deps` array.
    let mut states = vec![State::default(); deps.len()];
    for (i, (_, edges)) in deps.iter().enumerate() {
        states[i].outbound_remaining = edges.len();
        for edge in edges {
            let (j, _, _) = deps
                .get_full(edge.name)
                .ok_or_else(|| Error::NonexistentDep {
                    span: edge.span,
                    name: edge.name.to_string(),
                    kind: kind.to_string(),
                    highlighted: None,
                })?;
            states[j].reverse_deps.push(i);
        }
    }

    let mut order = Vec::new();
    let mut heap = BinaryHeap::new();

    // Seed the `heap` with edges that have no outbound edges
    //
    // The heap here is keyed by `(usize, &str, usize)` where the first `usize`
    // is unique which is what determines the order of the heap. The other two
    // fields are metadata used when pulling from the heap. The first `usize` is
    // the index of the item within the original dependency map which should
    // reflect the original source order of the item. Note that this is stored
    // in reverse order to ensure that when there are multiple items in the heap
    // the first item in the original order is popped first.
    for (i, dep) in deps.keys().enumerate() {
        if states[i].outbound_remaining == 0 {
            heap.push((deps.len() - i, *dep, i));
        }
    }

    // Drain the binary heap which represents all nodes that have had all their
    // dependencies processed. Iteratively add to the heap as well as nodes are
    // removed.
    while let Some((_order, node, i)) = heap.pop() {
        order.push(node);
        for i in mem::take(&mut states[i].reverse_deps) {
            states[i].outbound_remaining -= 1;
            if states[i].outbound_remaining == 0 {
                let (dep, _) = deps.get_index(i).unwrap();
                heap.push((deps.len() - i, *dep, i));
            }
        }
    }

    // If all nodes are present in order then a topological ordering was
    // achieved and it can be returned.
    if order.len() == deps.len() {
        return Ok(order);
    }

    // ... otherwise there are still dependencies with remaining edges which
    // means that a cycle must be present, so find the cycle and report the
    // error.
    for (i, state) in states.iter().enumerate() {
        if state.outbound_remaining == 0 {
            continue;
        }
        let (_, edges) = deps.get_index(i).unwrap();
        for dep in edges {
            let (j, _, _) = deps.get_full(dep.name).unwrap();
            if states[j].outbound_remaining == 0 {
                continue;
            }
            return Err(Error::Cycle {
                span: dep.span,
                name: dep.name.to_string(),
                kind: kind.to_string(),
                highlighted: None,
            });
        }
    }

    unreachable!()
}

#[derive(Debug)]
pub enum Error {
    NonexistentDep {
        span: Span,
        name: String,
        kind: String,
        highlighted: Option<String>,
    },
    Cycle {
        span: Span,
        name: String,
        kind: String,
        highlighted: Option<String>,
    },
}

impl Error {
    pub(crate) fn highlighted(&self) -> Option<&str> {
        match self {
            Error::NonexistentDep { highlighted, .. } | Error::Cycle { highlighted, .. } => {
                highlighted.as_deref()
            }
        }
    }
    pub(crate) fn set_highlighted(&mut self, string: String) {
        match self {
            Error::NonexistentDep { highlighted, .. } | Error::Cycle { highlighted, .. } => {
                *highlighted = Some(string);
            }
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(s) = self.highlighted() {
            return f.write_str(s);
        }
        match self {
            Error::NonexistentDep { kind, name, .. } => {
                write!(f, "{kind} `{name}` does not exist")
            }
            Error::Cycle { kind, name, .. } => {
                write!(f, "{kind} `{name}` depends on itself")
            }
        }
    }
}

impl std::error::Error for Error {}

#[cfg(test)]
mod tests {
    use super::*;

    fn id(name: &str) -> Id<'_> {
        Id {
            name,
            span: Span { start: 0, end: 0 },
        }
    }

    #[test]
    fn smoke() {
        let empty: Vec<&str> = Vec::new();
        assert_eq!(toposort("", &IndexMap::new()).unwrap(), empty);

        let mut nonexistent = IndexMap::new();
        nonexistent.insert("a", vec![id("b")]);
        assert!(matches!(
            toposort("", &nonexistent),
            Err(Error::NonexistentDep { .. })
        ));

        let mut one = IndexMap::new();
        one.insert("a", vec![]);
        assert_eq!(toposort("", &one).unwrap(), ["a"]);

        let mut two = IndexMap::new();
        two.insert("a", vec![]);
        two.insert("b", vec![id("a")]);
        assert_eq!(toposort("", &two).unwrap(), ["a", "b"]);

        let mut two = IndexMap::new();
        two.insert("a", vec![id("b")]);
        two.insert("b", vec![]);
        assert_eq!(toposort("", &two).unwrap(), ["b", "a"]);
    }

    #[test]
    fn cycles() {
        let mut cycle = IndexMap::new();
        cycle.insert("a", vec![id("a")]);
        assert!(matches!(toposort("", &cycle), Err(Error::Cycle { .. })));

        let mut cycle = IndexMap::new();
        cycle.insert("a", vec![id("b")]);
        cycle.insert("b", vec![id("c")]);
        cycle.insert("c", vec![id("a")]);
        assert!(matches!(toposort("", &cycle), Err(Error::Cycle { .. })));
    }

    #[test]
    fn depend_twice() {
        let mut two = IndexMap::new();
        two.insert("b", vec![id("a"), id("a")]);
        two.insert("a", vec![]);
        assert_eq!(toposort("", &two).unwrap(), ["a", "b"]);
    }

    #[test]
    fn preserve_order() {
        let mut order = IndexMap::new();
        order.insert("a", vec![]);
        order.insert("b", vec![]);
        assert_eq!(toposort("", &order).unwrap(), ["a", "b"]);

        let mut order = IndexMap::new();
        order.insert("b", vec![]);
        order.insert("a", vec![]);
        assert_eq!(toposort("", &order).unwrap(), ["b", "a"]);
    }
}
