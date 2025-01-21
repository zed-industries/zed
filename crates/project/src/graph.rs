use nohash_hasher::BuildNoHashHasher;

type HashMap = std::collections::hash_map::HashMap<u32, Vec<u32>, BuildNoHashHasher<u32>>;

/// A minimal graph implementation
/// Only useful for detecting cycles and reported the identities of the nodes that form the cycle
#[derive(Debug)]
pub(crate) struct Graph {
    // This might end up being more cache efficient if implemented with Vec<Vec<NodeIndex>> instead
    // and we assuming continuous indexing but this enables arbitrary node identities
    adjacencies: HashMap,
}

#[derive(Debug)]
pub(crate) struct Cycle {
    pub src_node: u32,
    pub dst_node: u32,
}

impl Into<anyhow::Error> for Cycle {
    fn into(self) -> anyhow::Error {
        anyhow::anyhow!(
            "cycle found: src: {}, dst: {}",
            self.src_node,
            self.dst_node
        )
    }
}

impl Graph {
    /// Creates a new, empty [`Graph`]
    pub fn new() -> Self {
        Self {
            adjacencies: HashMap::default(),
        }
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
            _ => (),
        }
    }

    /// Adds an arbitrarily identified node to the graph
    pub fn add_node(&mut self, node: u32) {
        if let None = self.adjacencies.get(&node) {
            self.adjacencies.insert(node, vec![]);
        }
    }

    fn dfs(&self, node: u32, visited: &mut Vec<u32>, stack: &mut Vec<u32>) -> Option<Cycle> {
        if visited.contains(&node) {
            return None;
        }

        let Some(neighbors) = self.adjacencies.get(&node) else {
            visited.push(node);
            return None;
        };

        stack.push(node);

        for neighbor in neighbors {
            if stack.contains(&neighbor) {
                return Some(Cycle {
                    src_node: *neighbor,
                    dst_node: node,
                });
            } else if let cycle @ Some(_) = self.dfs(*neighbor, visited, stack) {
                return cycle;
            }
        }

        stack.pop();
        visited.push(node);

        None
    }

    /// Build a subgraph starting from `start_node` from the nodes of this grpah
    pub fn subgraph(&self, start_node: u32) -> Graph {
        let Some(_) = self.adjacencies.get(&start_node) else {
            let mut graph = Graph::new();
            graph.add_node(start_node);
            return graph;
        };

        let mut graph = Self::new();

        self.collect_subgraph_nodes(&mut graph, start_node);

        graph
    }

    fn collect_subgraph_nodes(&self, other: &mut Self, node: u32) {
        if !other.adjacencies.contains_key(&node) {
            other.add_node(node);

            let Some(neighbors) = self.adjacencies.get(&node) else {
                return;
            };

            for neighbor in neighbors {
                self.collect_subgraph_nodes(other, *neighbor);
                other.add_edge(node, *neighbor);
            }
        }
    }

    /// Topologically sorts nodes, or return the nodes that form a cycle
    pub fn topo_sort(&self) -> Result<Vec<u32>, Cycle> {
        let mut visited = vec![];
        let mut stack = vec![];

        for node in self.adjacencies.keys() {
            if let Some(cycle) = self.dfs(*node, &mut visited, &mut stack) {
                return Err(cycle);
            }
        }

        visited.reverse();

        Ok(visited)
    }
}

#[cfg(test)]
mod graph_test {
    use itertools::Itertools;
    use pretty_assertions::assert_matches;

    use super::Graph;

    #[test]
    fn finds_no_cycle() {
        const GRAPH: [[u32; 2]; 2] = [[1, 2], [2, 3]];

        let mut graph = Graph::new();

        for edge in GRAPH.iter() {
            graph.add_edge(edge[0], edge[1]);
        }

        assert_matches!(graph.topo_sort(), Ok(_));
    }

    #[test]
    fn subgraph_correct() {
        const GRAPH: [[u32; 2]; 6] = [[3, 1], [2, 1], [1, 4], [4, 5], [3, 4], [2, 4]];

        let mut graph = Graph::new();
        for [src, dst] in &GRAPH {
            graph.add_edge(*src, *dst);
        }

        let subgraph = graph.subgraph(2);
        for i in &[1, 2, 4, 5] {
            assert!(
                subgraph.adjacencies.contains_key(i),
                "subgraph didn't contain key {i}; subgraph keys: {:#?}",
                subgraph.adjacencies.keys().collect_vec()
            );
        }
    }

    #[test]
    fn finds_cycle() {
        const GRAPH: [[u32; 2]; 3] = [[1, 2], [2, 3], [3, 1]];

        let mut graph = Graph::new();
        for edge in GRAPH {
            graph.add_edge(edge[0], edge[1]);
        }

        assert_matches!(graph.topo_sort(), Err(_));
    }

    #[test]
    fn sorts_correctly() {
        const GRAPH: [[u32; 2]; 6] = [[2, 3], [2, 1], [4, 1], [2, 4], [5, 4], [5, 1]];

        let mut graph = Graph::new();

        for [src, dst] in &GRAPH {
            graph.add_edge(*src, *dst);
        }

        assert_eq!(graph.topo_sort().unwrap(), vec![2, 3, 5, 4, 1]);
    }
}
