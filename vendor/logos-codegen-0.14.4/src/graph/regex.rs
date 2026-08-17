use std::fmt::Debug;

use regex_syntax::utf8::Utf8Sequences;

use crate::graph::{Disambiguate, Fork, Graph, Node, NodeId, Range, ReservedId, Rope};
use crate::mir::{Class, ClassUnicode, Literal, Mir};

impl<Leaf: Disambiguate + Debug> Graph<Leaf> {
    pub fn regex(&mut self, mir: Mir, then: NodeId) -> NodeId {
        self.parse_mir(&mir, then, None, None, false)
    }

    fn parse_mir(
        &mut self,
        mir: &Mir,
        then: NodeId,
        miss: Option<NodeId>,
        reserved: Option<ReservedId>,
        repeated: bool,
    ) -> NodeId {
        match mir {
            Mir::Empty => then,
            Mir::Loop(mir) => {
                let reserved_first = reserved.unwrap_or_else(|| self.reserve());

                let (new_then, new_miss);
                if let Some(old_miss) = miss {
                    // We have to separate the first iteration from the other iterations,
                    // because the `old_miss` path must only be taken if we miss the first
                    // iteration.
                    let reserved_next = self.reserve();
                    new_then = self.parse_mir(
                        mir,
                        reserved_next.get(),
                        Some(then),
                        Some(reserved_next),
                        true,
                    );
                    new_miss = self.merge(old_miss, then);
                } else {
                    new_then = reserved_first.get();
                    new_miss = then;
                }

                self.parse_mir(mir, new_then, Some(new_miss), Some(reserved_first), true)
            }
            Mir::Maybe(mir) => {
                let miss = match miss {
                    Some(id) => self.merge(id, then),
                    None => then,
                };

                self.parse_mir(mir, then, Some(miss), reserved, true)
            }
            Mir::Alternation(alternation) => {
                let mut fork = Fork::new().miss(miss);

                for mir in alternation {
                    let id = self.parse_mir(mir, then, None, None, repeated);
                    let alt = self.fork_off(id);

                    fork.merge(alt, self);
                }

                self.insert_or_push(reserved, fork)
            }
            Mir::Literal(literal) => {
                let pattern = literal.0.to_vec();

                self.insert_or_push(reserved, Rope::new(pattern, then).miss(miss))
            }
            Mir::Concat(concat) => {
                // Take an initial guess at the capacity - estimates a little worse than an average case
                // scenario by assuming every concat element is singular but has a full code-point unicode literal.
                // The only way to get the actual size of the Vec is if every sub-concat node is added up.
                let mut ropebuf: Vec<Range> = Vec::with_capacity(concat.len() * 4);
                let mut then = then;

                let mut handle_bytes = |graph: &mut Self, mir: &Mir, then: &mut NodeId| match mir {
                    Mir::Literal(Literal(bytes)) => {
                        ropebuf.extend(bytes.iter().rev().map(Into::<Range>::into));
                        true
                    }
                    Mir::Class(Class::Unicode(class)) if is_one_ascii(class, repeated) => {
                        ropebuf.push(class.ranges()[0].into());
                        true
                    }
                    Mir::Class(Class::Bytes(class)) if class.ranges().len() == 1 => {
                        ropebuf.push(class.ranges()[0].into());
                        true
                    }
                    _ => {
                        if !ropebuf.is_empty() {
                            let rope =
                                Rope::new(ropebuf.iter().cloned().rev().collect::<Vec<_>>(), *then);

                            *then = graph.push(rope);
                            ropebuf = Vec::with_capacity(concat.len() * 4);
                        }
                        false
                    }
                };

                for mir in concat[1..].iter().rev() {
                    if !handle_bytes(self, mir, &mut then) {
                        then = self.parse_mir(mir, then, None, None, false);
                    }
                }

                let first_mir = &concat[0];
                if handle_bytes(self, first_mir, &mut then) {
                    let rope = Rope::new(ropebuf.iter().cloned().rev().collect::<Vec<_>>(), then)
                        .miss(miss);
                    self.insert_or_push(reserved, rope)
                } else {
                    self.parse_mir(first_mir, then, miss, reserved, false)
                }
            }
            Mir::Class(Class::Unicode(class)) if !is_ascii(class, repeated) => {
                let mut ropes = class
                    .iter()
                    .flat_map(|range| Utf8Sequences::new(range.start(), range.end()))
                    .map(|sequence| Rope::new(sequence.as_slice(), then))
                    .collect::<Vec<_>>();

                if ropes.len() == 1 {
                    let rope = ropes.remove(0);

                    return self.insert_or_push(reserved, rope.miss(miss));
                }

                let mut root = Fork::new().miss(miss);

                for rope in ropes {
                    let fork = rope.into_fork(self);
                    root.merge(fork, self);
                }

                self.insert_or_push(reserved, root)
            }
            Mir::Class(class) => {
                let mut fork = Fork::new().miss(miss);

                let class: Vec<Range> = match class {
                    Class::Unicode(u) => u.iter().copied().map(Into::into).collect(),
                    Class::Bytes(b) => b.iter().copied().map(Into::into).collect(),
                };

                for range in class {
                    fork.add_branch(range, then, self);
                }

                self.insert_or_push(reserved, fork)
            }
        }
    }

    fn insert_or_push<N>(&mut self, id: Option<ReservedId>, node: N) -> NodeId
    where
        N: Into<Node<Leaf>>,
    {
        match id {
            Some(id) => self.insert(id, node),
            None => self.push(node),
        }
    }
}

/// Return whether current class unicode is ascii.
///
/// Because unicode ranges are iterated in increasing order,
/// it is only necessary to check the last range.
///
/// If the check is performed in a repetition,
/// a fast path is used by checking if end of range is 0x0010_FFFF.
fn is_ascii(class: &ClassUnicode, repeated: bool) -> bool {
    class.iter().last().map_or(true, |range| {
        let start = range.start() as u32;
        let end = range.end() as u32;
        end < 128 || (repeated && start < 128 && end == 0x0010_FFFF)
    })
}

/// Return whether current class unicode is ascii and only contains
/// one range.
///
/// See [`is_ascii`] function for more details.
fn is_one_ascii(class: &ClassUnicode, repeated: bool) -> bool {
    if class.ranges().len() != 1 {
        return false;
    }

    let range = &class.ranges()[0];
    let start = range.start() as u32;
    let end = range.end() as u32;
    end < 128 || (repeated && start < 128 && end == 0x0010_FFFF)
}

#[cfg(test)]
mod tests {
    use std::num::NonZeroU32;

    use super::*;
    use crate::graph::Node;
    use pretty_assertions::assert_eq;

    #[test]
    fn rope() {
        let mut graph = Graph::new();

        let mir = Mir::utf8("foobar").unwrap();

        assert_eq!(mir.priority(), 12);

        let leaf = graph.push(Node::Leaf("LEAF"));
        let id = graph.regex(mir, leaf);

        assert_eq!(graph[id], Node::Rope(Rope::new("foobar", leaf)),)
    }

    #[test]
    fn alternation() {
        let mut graph = Graph::new();

        let mir = Mir::utf8("a|b").unwrap();

        assert_eq!(mir.priority(), 2);

        let leaf = graph.push(Node::Leaf("LEAF"));
        let id = graph.regex(mir, leaf);

        assert_eq!(
            graph[id],
            Node::Fork(Fork::new().branch(b'a', leaf).branch(b'b', leaf)),
        );
    }

    #[test]
    fn repeat() {
        let mut graph = Graph::new();

        let mir = Mir::utf8("[a-z]*").unwrap();

        assert_eq!(mir.priority(), 0);

        let leaf = graph.push(Node::Leaf("LEAF"));
        let id = graph.regex(mir, leaf);

        assert_eq!(
            graph[id],
            Node::Fork(
                Fork::new()
                    .branch('a'..='z', id) // goto self == loop
                    .miss(leaf)
            ),
        );
    }

    #[test]
    fn maybe() {
        let mut graph = Graph::new();

        let mir = Mir::utf8("[a-z]?").unwrap();

        assert_eq!(mir.priority(), 0);

        let leaf = graph.push(Node::Leaf("LEAF"));
        let id = graph.regex(mir, leaf);

        assert_eq!(
            graph[id],
            Node::Fork(Fork::new().branch('a'..='z', leaf).miss(leaf)),
        );
    }

    #[test]
    fn long_concat_389() {
        let mut graph = Graph::new();

        let mir = Mir::utf8("abcdefghijklmnopqrstuvwxyz*").unwrap();

        assert_eq!(mir.priority(), 50);

        let leaf = graph.push(Node::Leaf("LEAF"));
        let id = graph.regex(mir, leaf);
        let sub_id = NodeId(NonZeroU32::new(2).unwrap());

        assert_eq!(
            graph[id],
            Node::Rope(Rope::new("abcdefghijklmnopqrstuvwxy", sub_id))
        );

        assert_eq!(graph[sub_id], Node::Rope(Rope::new("z", sub_id).miss(leaf)))
    }
}
