use std::cmp::Reverse;

use bumpalo::Bump;
use radix_heap::RadixHeapMap;

use crate::{
    changes::ChangeMap,
    graph::{Edge, Vertex, populate_change_map, set_neighbours},
    hash::DftHashMap,
    syntax::Syntax,
};

#[derive(Debug)]
pub struct ExceededGraphLimit;

fn shortest_vertex_path<'s, 'v>(
    start: &'v Vertex<'s, 'v>,
    vertex_arena: &'v Bump,
    size_hint: usize,
    graph_limit: usize,
) -> Result<Vec<&'v Vertex<'s, 'v>>, ExceededGraphLimit> {
    // RadixHeapMap is a max-heap, so wrap with Reverse to get min-heap behavior.
    let mut heap: RadixHeapMap<Reverse<_>, &'v Vertex<'s, 'v>> = RadixHeapMap::new();

    heap.push(Reverse(0), start);

    let mut seen = DftHashMap::default();
    seen.reserve(size_hint);

    let end: &'v Vertex<'s, 'v> = loop {
        match heap.pop() {
            Some((Reverse(distance), current)) => {
                if current.is_end() {
                    break current;
                }

                set_neighbours(current, vertex_arena, &mut seen);
                for neighbour in *current
                    .neighbours
                    .borrow()
                    .as_ref()
                    .expect("Neighbours should be set before traversal")
                {
                    let (edge, next) = neighbour;
                    let distance_to_next = distance + edge.cost();

                    let found_shorter_route = match next.predecessor.get() {
                        Some((prev_shortest, _)) => distance_to_next < prev_shortest,
                        None => true,
                    };

                    if found_shorter_route {
                        next.predecessor.replace(Some((distance_to_next, current)));
                        heap.push(Reverse(distance_to_next), next);
                    }
                }

                if seen.len() > graph_limit {
                    log::info!(
                        "Reached graph limit, arena consumed {} bytes",
                        vertex_arena.allocated_bytes(),
                    );
                    return Err(ExceededGraphLimit);
                }
            }
            None => panic!("Ran out of graph nodes before reaching end"),
        }
    };

    log::info!(
        "Saw {} vertices (a Vertex is {} bytes), arena consumed {} bytes, with {} vertices left on heap.",
        seen.len(),
        std::mem::size_of::<Vertex>(),
        vertex_arena.allocated_bytes(),
        heap.len(),
    );

    let mut current = Some((0, end));
    let mut vertex_route: Vec<&'v Vertex<'s, 'v>> = vec![];
    while let Some((_, node)) = current {
        vertex_route.push(node);
        current = node.predecessor.get();
    }

    vertex_route.reverse();
    Ok(vertex_route)
}

fn shortest_path_with_edges<'s, 'v>(
    route: &[&'v Vertex<'s, 'v>],
) -> Vec<(Edge, &'v Vertex<'s, 'v>)> {
    let mut prev = route.first().expect("Expected non-empty route");

    let mut cost = 0;
    let mut res = vec![];

    for vertex in route.iter().skip(1) {
        let edge = edge_between(prev, vertex);
        res.push((edge, *prev));
        cost += edge.cost();

        prev = vertex;
    }
    log::debug!("Found a path of {} with cost {}.", route.len(), cost);

    res
}

fn shortest_path<'s, 'v>(
    start: Vertex<'s, 'v>,
    vertex_arena: &'v Bump,
    size_hint: usize,
    graph_limit: usize,
) -> Result<Vec<(Edge, &'v Vertex<'s, 'v>)>, ExceededGraphLimit> {
    let start: &'v Vertex<'s, 'v> = vertex_arena.alloc(start);
    let vertex_path = shortest_vertex_path(start, vertex_arena, size_hint, graph_limit)?;
    Ok(shortest_path_with_edges(&vertex_path))
}

fn edge_between<'s, 'v>(before: &Vertex<'s, 'v>, after: &Vertex<'s, 'v>) -> Edge {
    assert_ne!(before, after);

    let mut shortest_edge: Option<Edge> = None;
    if let Some(neighbours) = &*before.neighbours.borrow() {
        for neighbour in *neighbours {
            let (edge, next) = *neighbour;
            // If there are multiple edges that can take us to `next`,
            // prefer the shortest.
            if *next == *after {
                let is_shorter = match shortest_edge {
                    Some(prev_edge) => edge.cost() < prev_edge.cost(),
                    None => true,
                };

                if is_shorter {
                    shortest_edge = Some(edge);
                }
            }
        }
    }

    if let Some(edge) = shortest_edge {
        return edge;
    }

    panic!(
        "Expected a route between the two vertices {:#?} and {:#?}",
        before, after
    );
}

fn node_count(root: Option<&Syntax>) -> u32 {
    let iter = std::iter::successors(root, |node| node.next_sibling());

    iter.map(|node| match node {
        Syntax::List {
            num_descendants, ..
        } => *num_descendants,
        Syntax::Atom { .. } => 1,
    })
    .sum::<u32>()
}

fn tree_count(root: Option<&Syntax>) -> u32 {
    std::iter::successors(root, |node| node.next_sibling()).count() as _
}

pub(crate) fn mark_syntax<'a>(
    lhs_syntax: Option<&'a Syntax<'a>>,
    rhs_syntax: Option<&'a Syntax<'a>>,
    change_map: &mut ChangeMap<'a>,
    graph_limit: usize,
) -> Result<(), ExceededGraphLimit> {
    let lhs_node_count = node_count(lhs_syntax) as usize;
    let rhs_node_count = node_count(rhs_syntax) as usize;
    log::info!(
        "LHS nodes: {} ({} toplevel), RHS nodes: {} ({} toplevel)",
        lhs_node_count,
        tree_count(lhs_syntax),
        rhs_node_count,
        tree_count(rhs_syntax),
    );

    let size_hint = std::cmp::min(lhs_node_count * rhs_node_count, graph_limit);

    let start = Vertex::new(lhs_syntax, rhs_syntax);
    let vertex_arena = Bump::new();

    let route = shortest_path(start, &vertex_arena, size_hint, graph_limit)?;

    log::debug!(
        "Initial 5 items on path: {:#?}",
        route
            .iter()
            .map(|(edge, v)| {
                format!(
                    "{:20} {:20} --- {:3} {:?}",
                    v.lhs_syntax
                        .map_or_else(|| "None".into(), Syntax::dbg_content),
                    v.rhs_syntax
                        .map_or_else(|| "None".into(), Syntax::dbg_content),
                    edge.cost(),
                    edge,
                )
            })
            .take(5)
            .collect::<Vec<_>>()
    );

    populate_change_map(&route, change_map);
    Ok(())
}
