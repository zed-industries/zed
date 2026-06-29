use criterion::{Criterion, Throughput, black_box, criterion_group, criterion_main};
use language::intra_line_diff;

/// Representative line pairs we might see on every changed line in a large hunk.
fn sample_pairs() -> Vec<(String, String)> {
    let templates: &[(&str, &str)] = &[
        ("let x = 1", "let x = 42"),
        ("hello world foo bar", "hello WORLD foo BAR"),
        ("fn compute_total(items: &[Item])", "fn compute_sum(items: &[Item])"),
        ("    foo bar", "        foo bar"),
        ("old = value", "new = value"),
        ("prefix alpha", "prefix beta"),
        ("greet 日", "greet 日本語"),
        ("🎉 party", "🎉🎊 party"),
        ("a,b", "x,y"),
        (
            "one two three four five six seven eight nine ten",
            "one two THREE four five SIX seven eight NINE ten",
        ),
        (
            "pub async fn handle_request(ctx: &mut Context) -> Result<Response>",
            "pub async fn handle_request(cx: &mut Context) -> Result<Response>",
        ),
        ("completely different line on the left", "totally unrelated content here"),
    ];

    // Expand to a large batch so the benchmark reflects per-file / multi-hunk cost.
    templates
        .iter()
        .cycle()
        .take(10_000)
        .enumerate()
        .map(|(i, (old, new))| {
            // Vary slightly so we don't only measure identical-cache effects.
            let old = format!("{old} // {i}");
            let new = if i % 7 == 0 {
                old.clone()
            } else {
                format!("{new} // {i}")
            };
            (old, new)
        })
        .collect()
}

fn bench_intra_line_diff_batch(c: &mut Criterion) {
    let pairs = sample_pairs();
    let mut group = c.benchmark_group("intra_line_diff");
    group.throughput(Throughput::Elements(pairs.len() as u64));
    group.bench_function("batch_10k_line_pairs", |b| {
        b.iter(|| {
            for (old, new) in &pairs {
                black_box(intra_line_diff(old, new));
            }
        });
    });
    group.finish();
}

criterion_group!(benches, bench_intra_line_diff_batch);
criterion_main!(benches);
