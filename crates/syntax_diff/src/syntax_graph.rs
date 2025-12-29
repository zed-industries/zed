use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashMap;

use crate::{SyntaxCursor, SyntaxNode, SyntaxTree};

/// A vertex in the diff graph represents cursor positions in both trees.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Vertex<'a> {
    lhs: SyntaxCursor<'a>,
    rhs: SyntaxCursor<'a>,
}

impl<'a> Vertex<'a> {
    pub fn start(lhs_tree: &'a SyntaxTree, rhs_tree: &'a SyntaxTree) -> Self {
        Self {
            lhs: lhs_tree.cursor(),
            rhs: rhs_tree.cursor(),
        }
    }

    pub fn is_end(&self) -> bool {
        self.lhs.is_done() && self.rhs.is_done()
    }
}

/// The type of transition between vertices.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EdgeKind {
    /// Both nodes have identical subtrees (same structural hash).
    UnchangedSubtree,

    /// Both nodes are list nodes with matching delimiters.
    EnterBothDelimiters,

    /// Exit from both lists simultaneously.
    ExitBothDelimiters,

    /// LHS node has no match - mark as deleted.
    NovelLhs,

    /// RHS node has no match - mark as inserted.
    NovelRhs,

    /// Enter LHS list only (RHS stays put).
    EnterLhsDelimiter,

    /// Enter RHS list only (LHS stays put).
    EnterRhsDelimiter,

    /// Exit LHS list only.
    ExitLhsDelimiter,

    /// Exit RHS list only.
    ExitRhsDelimiter,
}

/// An edge in the diff graph.
#[derive(Clone, Copy, Debug)]
pub struct Edge<'a> {
    pub kind: EdgeKind,
    pub cost: u32,
    pub to: Vertex<'a>,
}

/// Cost constants for different operations.
mod costs {
    pub const UNCHANGED_SUBTREE: u32 = 0;
    pub const ENTER_BOTH: u32 = 1;
    pub const EXIT_BOTH: u32 = 1;
    pub const NOVEL_ATOM: u32 = 100;
    pub const NOVEL_LIST: u32 = 150;
    pub const ENTER_NOVEL: u32 = 50;
    pub const EXIT_NOVEL: u32 = 1;
}

/// Context for generating edges from a vertex.
pub struct DiffContext<'a> {
    pub lhs: &'a SyntaxTree,
    pub rhs: &'a SyntaxTree,
    pub lhs_text: &'a str,
    pub rhs_text: &'a str,
}

impl<'a> DiffContext<'a> {
    pub fn new(
        lhs: &'a SyntaxTree,
        rhs: &'a SyntaxTree,
        lhs_text: &'a str,
        rhs_text: &'a str,
    ) -> Self {
        Self {
            lhs,
            rhs,
            lhs_text,
            rhs_text,
        }
    }

    /// Generate all outgoing edges from a vertex.
    pub fn neighbors(&self, vertex: Vertex<'a>) -> Vec<Edge<'a>> {
        let mut edges = Vec::with_capacity(8);

        let lhs_node = vertex.lhs.node();
        let rhs_node = vertex.rhs.node();

        match (lhs_node, rhs_node) {
            (Some(lhs_node), Some(rhs_node)) => {
                // Check if subtrees are identical (structural hash match).
                if lhs_node.structural_hash == rhs_node.structural_hash {
                    let lhs_content = &self.lhs_text[lhs_node.byte_range.clone()];
                    let rhs_content = &self.rhs_text[rhs_node.byte_range.clone()];

                    if lhs_content == rhs_content {
                        edges.push(Edge {
                            kind: EdgeKind::UnchangedSubtree,
                            cost: costs::UNCHANGED_SUBTREE,
                            to: Vertex {
                                lhs: vertex.lhs.advance(),
                                rhs: vertex.rhs.advance(),
                            },
                        });
                        return edges;
                    }
                }

                let lhs_is_list = !lhs_node.is_leaf();
                let rhs_is_list = !rhs_node.is_leaf();

                // Both are lists - can enter both.
                if lhs_is_list && rhs_is_list {
                    let delimiters_match = self.delimiters_match(lhs_node, rhs_node);
                    let cost = if delimiters_match {
                        costs::ENTER_BOTH
                    } else {
                        costs::ENTER_NOVEL * 2
                    };

                    edges.push(Edge {
                        kind: EdgeKind::EnterBothDelimiters,
                        cost,
                        to: Vertex {
                            lhs: vertex.lhs.enter(),
                            rhs: vertex.rhs.enter(),
                        },
                    });
                }

                // Novel LHS - skip this node.
                let lhs_cost = if lhs_is_list {
                    costs::NOVEL_LIST
                } else {
                    costs::NOVEL_ATOM
                };
                edges.push(Edge {
                    kind: EdgeKind::NovelLhs,
                    cost: lhs_cost,
                    to: Vertex {
                        lhs: vertex.lhs.advance(),
                        rhs: vertex.rhs,
                    },
                });

                // Novel RHS - skip this node.
                let rhs_cost = if rhs_is_list {
                    costs::NOVEL_LIST
                } else {
                    costs::NOVEL_ATOM
                };
                edges.push(Edge {
                    kind: EdgeKind::NovelRhs,
                    cost: rhs_cost,
                    to: Vertex {
                        lhs: vertex.lhs,
                        rhs: vertex.rhs.advance(),
                    },
                });

                // Enter LHS list only.
                if lhs_is_list {
                    edges.push(Edge {
                        kind: EdgeKind::EnterLhsDelimiter,
                        cost: costs::ENTER_NOVEL,
                        to: Vertex {
                            lhs: vertex.lhs.enter(),
                            rhs: vertex.rhs,
                        },
                    });
                }

                // Enter RHS list only.
                if rhs_is_list {
                    edges.push(Edge {
                        kind: EdgeKind::EnterRhsDelimiter,
                        cost: costs::ENTER_NOVEL,
                        to: Vertex {
                            lhs: vertex.lhs,
                            rhs: vertex.rhs.enter(),
                        },
                    });
                }
            }

            (Some(lhs_node), None) => {
                let lhs_is_list = !lhs_node.is_leaf();

                let cost = if lhs_is_list {
                    costs::NOVEL_LIST
                } else {
                    costs::NOVEL_ATOM
                };
                edges.push(Edge {
                    kind: EdgeKind::NovelLhs,
                    cost,
                    to: Vertex {
                        lhs: vertex.lhs.advance(),
                        rhs: vertex.rhs,
                    },
                });

                if lhs_is_list {
                    edges.push(Edge {
                        kind: EdgeKind::EnterLhsDelimiter,
                        cost: costs::ENTER_NOVEL,
                        to: Vertex {
                            lhs: vertex.lhs.enter(),
                            rhs: vertex.rhs,
                        },
                    });
                }

                if vertex.rhs.can_exit() {
                    edges.push(Edge {
                        kind: EdgeKind::ExitRhsDelimiter,
                        cost: costs::EXIT_NOVEL,
                        to: Vertex {
                            lhs: vertex.lhs,
                            rhs: vertex.rhs.exit(),
                        },
                    });
                }
            }

            (None, Some(rhs_node)) => {
                let rhs_is_list = !rhs_node.is_leaf();

                let cost = if rhs_is_list {
                    costs::NOVEL_LIST
                } else {
                    costs::NOVEL_ATOM
                };
                edges.push(Edge {
                    kind: EdgeKind::NovelRhs,
                    cost,
                    to: Vertex {
                        lhs: vertex.lhs,
                        rhs: vertex.rhs.advance(),
                    },
                });

                if rhs_is_list {
                    edges.push(Edge {
                        kind: EdgeKind::EnterRhsDelimiter,
                        cost: costs::ENTER_NOVEL,
                        to: Vertex {
                            lhs: vertex.lhs,
                            rhs: vertex.rhs.enter(),
                        },
                    });
                }

                if vertex.lhs.can_exit() {
                    edges.push(Edge {
                        kind: EdgeKind::ExitLhsDelimiter,
                        cost: costs::EXIT_NOVEL,
                        to: Vertex {
                            lhs: vertex.lhs.exit(),
                            rhs: vertex.rhs,
                        },
                    });
                }
            }

            (None, None) => {
                let can_exit_lhs = vertex.lhs.can_exit();
                let can_exit_rhs = vertex.rhs.can_exit();

                if can_exit_lhs && can_exit_rhs {
                    edges.push(Edge {
                        kind: EdgeKind::ExitBothDelimiters,
                        cost: costs::EXIT_BOTH,
                        to: Vertex {
                            lhs: vertex.lhs.exit(),
                            rhs: vertex.rhs.exit(),
                        },
                    });
                }

                if can_exit_lhs {
                    edges.push(Edge {
                        kind: EdgeKind::ExitLhsDelimiter,
                        cost: costs::EXIT_NOVEL,
                        to: Vertex {
                            lhs: vertex.lhs.exit(),
                            rhs: vertex.rhs,
                        },
                    });
                }

                if can_exit_rhs {
                    edges.push(Edge {
                        kind: EdgeKind::ExitRhsDelimiter,
                        cost: costs::EXIT_NOVEL,
                        to: Vertex {
                            lhs: vertex.lhs,
                            rhs: vertex.rhs.exit(),
                        },
                    });
                }
            }
        }

        edges
    }

    fn delimiters_match(&self, lhs_node: &SyntaxNode, rhs_node: &SyntaxNode) -> bool {
        if lhs_node.kind_id != rhs_node.kind_id {
            return false;
        }

        let lhs_open = &self.lhs_text[lhs_node.open_delimiter()];
        let rhs_open = &self.rhs_text[rhs_node.open_delimiter()];
        let lhs_close = &self.lhs_text[lhs_node.close_delimiter()];
        let rhs_close = &self.rhs_text[rhs_node.close_delimiter()];

        lhs_open == rhs_open && lhs_close == rhs_close
    }
}

/// Result of running the diff algorithm.
#[derive(Debug)]
pub struct DiffResult<'a> {
    pub path: Vec<Edge<'a>>,
    pub total_cost: u32,
}

/// Run Dijkstra's algorithm to find the minimum-cost diff.
pub fn compute_diff<'a>(context: &DiffContext<'a>) -> Option<DiffResult<'a>> {
    let start = Vertex::start(context.lhs, context.rhs);

    if start.is_end() {
        return Some(DiffResult {
            path: vec![],
            total_cost: 0,
        });
    }

    let mut distances: HashMap<Vertex<'a>, u32> = HashMap::new();
    let mut predecessors: HashMap<Vertex<'a>, (Vertex<'a>, Edge<'a>)> = HashMap::new();
    let mut heap: BinaryHeap<Reverse<(u32, Vertex<'a>)>> = BinaryHeap::new();

    distances.insert(start, 0);
    heap.push(Reverse((0, start)));

    let mut end_vertex: Option<Vertex<'a>> = None;

    while let Some(Reverse((cost, vertex))) = heap.pop() {
        if let Some(&best) = distances.get(&vertex) {
            if cost > best {
                continue;
            }
        }

        if vertex.is_end() {
            end_vertex = Some(vertex);
            break;
        }

        for edge in context.neighbors(vertex) {
            let new_cost = cost + edge.cost;
            let is_better = distances
                .get(&edge.to)
                .map(|&d| new_cost < d)
                .unwrap_or(true);

            if is_better {
                distances.insert(edge.to, new_cost);
                predecessors.insert(edge.to, (vertex, edge));
                heap.push(Reverse((new_cost, edge.to)));
            }
        }
    }

    let end = end_vertex?;
    let total_cost = distances[&end];

    let mut path = Vec::new();
    let mut current = end;

    while let Some((prev, edge)) = predecessors.get(&current) {
        path.push(*edge);
        current = *prev;
    }

    path.reverse();

    Some(DiffResult { path, total_cost })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_test_tree(text: &str, language: tree_sitter::Language) -> (SyntaxTree, String) {
        let mut parser = tree_sitter::Parser::new();
        parser.set_language(&language).unwrap();
        let tree = parser.parse(text, None).unwrap();
        let syntax_tree = SyntaxTree::new(&mut tree.walk());
        (syntax_tree, text.to_string())
    }

    #[test]
    fn test_identical_trees() {
        let text = "fn main() {}";
        let (lhs, lhs_text) = make_test_tree(text, tree_sitter_rust::LANGUAGE.into());
        let (rhs, rhs_text) = make_test_tree(text, tree_sitter_rust::LANGUAGE.into());

        let context = DiffContext::new(&lhs, &rhs, &lhs_text, &rhs_text);
        let result = compute_diff(&context).unwrap();

        assert_eq!(result.total_cost, 0);
        assert_eq!(result.path.len(), 1);
        assert_eq!(result.path[0].kind, EdgeKind::UnchangedSubtree);
    }

    #[test]
    fn test_simple_change() {
        let (lhs, lhs_text) = make_test_tree("let x = 1;", tree_sitter_rust::LANGUAGE.into());
        let (rhs, rhs_text) = make_test_tree("let x = 2;", tree_sitter_rust::LANGUAGE.into());

        let context = DiffContext::new(&lhs, &rhs, &lhs_text, &rhs_text);
        let result = compute_diff(&context).unwrap();

        assert!(result.total_cost > 0);
    }

    #[test]
    fn test_nested_blocks() {
        let (lhs, lhs_text) = make_test_tree(
            "fn main() { if true { foo(); } }",
            tree_sitter_rust::LANGUAGE.into(),
        );
        let (rhs, rhs_text) = make_test_tree(
            "fn main() { if true { bar(); } }",
            tree_sitter_rust::LANGUAGE.into(),
        );

        let context = DiffContext::new(&lhs, &rhs, &lhs_text, &rhs_text);
        let result = compute_diff(&context).unwrap();

        assert!(result.total_cost > 0);
    }

    #[test]
    fn test_added_block() {
        let (lhs, lhs_text) =
            make_test_tree("fn main() { foo(); }", tree_sitter_rust::LANGUAGE.into());
        let (rhs, rhs_text) = make_test_tree(
            "fn main() { if true { foo(); } }",
            tree_sitter_rust::LANGUAGE.into(),
        );

        let context = DiffContext::new(&lhs, &rhs, &lhs_text, &rhs_text);
        let result = compute_diff(&context).unwrap();

        assert!(result.total_cost > 0);
    }

    #[test]
    fn test_empty_to_content() {
        let (lhs, lhs_text) = make_test_tree("fn main() {}", tree_sitter_rust::LANGUAGE.into());
        let (rhs, rhs_text) = make_test_tree(
            "fn main() { let x = 1; }",
            tree_sitter_rust::LANGUAGE.into(),
        );

        let context = DiffContext::new(&lhs, &rhs, &lhs_text, &rhs_text);
        let result = compute_diff(&context).unwrap();

        assert!(result.total_cost > 0);
    }
}
