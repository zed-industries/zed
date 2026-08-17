use criterion::{BatchSize, BenchmarkId, Criterion, criterion_group, criterion_main};
use dugong::graphlib::{Graph, GraphOptions};
use dugong::rank;
use dugong::{EdgeLabel, GraphLabel, NodeLabel};
use std::hint::black_box;
use std::time::Duration;

#[derive(Debug, Clone)]
struct GraphSpec {
    node_ids: Vec<String>,
    edges: Vec<(usize, usize, usize, f64)>,
}

impl GraphSpec {
    fn build(&self) -> Graph<NodeLabel, EdgeLabel, GraphLabel> {
        let mut g: Graph<NodeLabel, EdgeLabel, GraphLabel> = Graph::new(GraphOptions {
            directed: true,
            multigraph: true,
            ..Default::default()
        });
        g.set_graph(GraphLabel::default());
        g.set_default_node_label(NodeLabel::default);
        g.set_default_edge_label(|| EdgeLabel {
            minlen: 1,
            weight: 1.0,
            ..Default::default()
        });

        for id in &self.node_ids {
            g.set_node(id.clone(), NodeLabel::default());
        }

        for &(from, to, minlen, weight) in &self.edges {
            if from >= self.node_ids.len() || to >= self.node_ids.len() || from == to {
                continue;
            }
            g.set_edge_with_label(
                self.node_ids[from].clone(),
                self.node_ids[to].clone(),
                EdgeLabel {
                    minlen,
                    weight,
                    ..Default::default()
                },
            );
        }

        g
    }
}

fn build_dag_spec(name: &str, node_count: usize, fanout: usize) -> GraphSpec {
    let node_ids: Vec<String> = (0..node_count).map(|i| format!("{name}_n{i}")).collect();
    let mut edges: Vec<(usize, usize, usize, f64)> = Vec::new();

    // A spine to guarantee connectivity.
    for i in 0..node_count.saturating_sub(1) {
        edges.push((i, i + 1, 1, 2.0));
    }

    // Extra forward edges to create crossing pressure.
    for i in 0..node_count {
        for k in 2..=(fanout + 1) {
            let to = i.saturating_add(k);
            if to >= node_count {
                break;
            }
            edges.push((i, to, 1, 1.0));
        }

        // A longer edge that increases slack variation.
        let to = i.saturating_add(10);
        if to < node_count {
            edges.push((i, to, 2, 0.5));
        }
    }

    GraphSpec { node_ids, edges }
}

fn bench_network_simplex(c: &mut Criterion) {
    let mut group = c.benchmark_group("network_simplex");
    group.measurement_time(Duration::from_secs(10));

    let cases = [
        ("dag_50_f3", 50usize, 3usize),
        ("dag_200_f4", 200usize, 4usize),
        ("dag_400_f4", 400usize, 4usize),
    ];

    for (name, nodes, fanout) in cases {
        let spec = build_dag_spec(name, nodes, fanout);
        group.bench_with_input(
            BenchmarkId::new("rank::network_simplex", name),
            &spec,
            |b, spec| {
                b.iter_batched(
                    || spec.build(),
                    |mut g| {
                        rank::network_simplex::network_simplex(black_box(&mut g));
                        black_box(g.node_count());
                    },
                    BatchSize::LargeInput,
                )
            },
        );
    }

    group.finish();
}

criterion_group!(benches, bench_network_simplex);
criterion_main!(benches);
