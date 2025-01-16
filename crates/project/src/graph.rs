use ::collections::HashMap;

/// A minimal graph implementation
/// Only useful for detecting cycles and reported the identities of the nodes that form the cycle
#[derive(Debug)]
pub(crate) struct Graph {
    // This might end up being more cache efficient if implemented with Vec<Vec<NodeIndex>> instead
    // and we assuming continuous indexing but this enables arbitrary node identities
    adjacencies: HashMap<u32, Vec<u32>>
}

#[derive(Debug)]
pub(crate) struct Cycle {
    pub src_node: u32,
    pub dst_node: u32
}

impl Graph {
    /// Creates a new, empty `[Graph]`
    pub fn new() -> Self {
        Self { adjacencies: HashMap::default() }
    }

    /// Adds an edge to the graph, adding the nodes if not present
    pub fn add_edge(&mut self, src_node: u32, dst_node: u32) {
        match self.adjacencies.get_mut(&src_node) {
            Some(neighbors) if !neighbors.contains(&dst_node) => {
                neighbors.push(dst_node);
            }
            None => {
                self.adjacencies.insert(src_node, vec![dst_node]);
            }
            _ => ()
        }
    }

    /// Adds an arbitrarily identified node to the graph
    pub fn add_node(&mut self, node: u32) {
        if let None = self.adjacencies.get(&node) {
            self.adjacencies.insert(node, vec![]);
        }
    }

    /// If a cycle is present, returns the node as `Cycle::src_node` and it's pointee as `[Cycle::dst_node]`
    pub fn has_cycle(&self, start_node: u32) -> Option<Cycle> {
        match self.adjacencies.get(&start_node) {
            Some(neighbors) if !neighbors.is_empty() => {
                let mut visited = vec![];
                let mut stack = vec![];
                self.dfs(start_node, &mut visited, &mut stack)
            }
            _ => None
        }
    }

    fn dfs(
        &self,
        node: u32,
        visited: &mut Vec<u32>,
        stack: &mut Vec<u32>
    ) -> Option<Cycle> {
        if !visited.contains(&node) {
            visited.push(node);

            let Some(neighbors) = self.adjacencies.get(&node) else {
                return None;
            };

            stack.push(node);

            for neighbor in neighbors {
                if !visited.contains(&neighbor) {
                    if let cycle @ Some(_) = self.dfs(*neighbor, visited, stack) {
                        return cycle;
                    }
                } else if stack.contains(&neighbor) {
                    return Some(Cycle { src_node: node, dst_node: *neighbor });
                }
            }

            stack.pop();
        }

        None
    }
}

#[cfg(test)]
mod graph_test {
    use pretty_assertions::assert_matches;

    use super::{Graph, Cycle};

    #[test]
    fn finds_no_cycle() {
        const GRAPH: [[u32; 2]; 2] = [
            [1, 2],
            [2, 3]
        ];

        let mut graph = Graph::new();

        for edge in GRAPH.iter() {
            graph.add_edge(edge[0], edge[1]);
        }

        assert_matches!(graph.has_cycle(1), None);
        assert_matches!(graph.has_cycle(2), None);
    }

    #[test]
    fn finds_cycle() {
        const GRAPH: [[u32; 2]; 3] = [
            [1, 2],
            [2, 3],
            [3, 1]
        ];

        let mut graph = Graph::new();
        for edge in GRAPH {
            graph.add_edge(edge[0], edge[1]);
        }
        dbg!(&graph);

        assert_matches!(graph.has_cycle(1), Some(Cycle { src_node: 3, dst_node: 1 }));
        assert_matches!(graph.has_cycle(2), Some(Cycle { src_node: 1, dst_node: 2 }));
    }
}
