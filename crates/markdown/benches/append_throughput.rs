//! `Markdown::append` throughput microbench.
//!
//! Isolates the string-accumulation cost of `Markdown::append` from the
//! rest of the widget (background parse task, GPUI app context, etc.) so
//! the algorithmic improvement is reproducible in a single command.
//!
//! - `pre_fix_concat`: reproduces the historical body of `append`, which
//!   was `self.source = SharedString::new(self.source.to_string() + text)`
//!   on every call. O(n) per call -> O(n^2) on the streamed total.
//! - `post_fix_buffered`: reproduces the new body, which accumulates into
//!   a `String` buffer with amortised `push_str` and promotes the buffer
//!   to a `SharedString` once at the end of the stream (the throttle in
//!   the real widget collapses many appends into one promotion per parse
//!   cycle; the bench uses one promotion per stream as the best-case
//!   bound).
//!
//! Fixture matches the Paneflow hot path observed via flamegraph on the
//! ACP streaming buffer drain loop: 60 KB total across 100 chunks of
//! 600 B. The ratio reported by the criterion HTML report is the load-
//! bearing artefact for the perf claim.
//!
//! Run via `cargo bench -p markdown --bench append_throughput`. Output
//! lands under `target/criterion/markdown_append/`.

use criterion::{BenchmarkId, Criterion, Throughput, black_box, criterion_group, criterion_main};
use gpui::SharedString;

const CHUNKS: usize = 100;
const CHUNK_BYTES: usize = 600;
const TOTAL_BYTES: usize = CHUNKS * CHUNK_BYTES;

fn make_chunks() -> Vec<String> {
    const FIXTURE_BYTES: &[u8] = b"markdown append streaming fixture with code blocks and prose\n";

    (0..CHUNKS)
        .map(|chunk_ix| {
            (0..CHUNK_BYTES)
                .map(|byte_ix| FIXTURE_BYTES[(chunk_ix + byte_ix) % FIXTURE_BYTES.len()] as char)
                .collect()
        })
        .collect()
}

fn pre_fix_concat(chunks: &[String]) -> SharedString {
    let mut source = SharedString::new("");

    for chunk in chunks {
        // Exact body of the historical `Markdown::append`.
        source = SharedString::new(source.to_string() + black_box(chunk.as_str()));
    }

    source
}

fn post_fix_buffered(chunks: &[String]) -> SharedString {
    let mut buf = String::new();

    for chunk in chunks {
        // Exact body of the new `Markdown::append` after the buffer is
        // initialised.
        buf.push_str(black_box(chunk.as_str()));
    }

    // One promotion per stream — the best-case bound the throttle in
    // `parse()` converges towards when many appends land between parse
    // cycles.
    SharedString::from(buf)
}

fn bench_append_throughput(c: &mut Criterion) {
    let chunks = make_chunks();
    let mut group = c.benchmark_group("markdown_append");
    group.throughput(Throughput::Bytes(TOTAL_BYTES as u64));

    group.bench_function(BenchmarkId::new("pre_fix_concat", "60kb_100x600b"), |b| {
        b.iter(|| {
            let source = pre_fix_concat(black_box(&chunks));
            black_box(source);
        });
    });

    group.bench_function(
        BenchmarkId::new("post_fix_buffered", "60kb_100x600b"),
        |b| {
            b.iter(|| {
                let source = post_fix_buffered(black_box(&chunks));
                black_box(source);
            });
        },
    );

    group.finish();
}

criterion_group!(markdown_append, bench_append_throughput);
criterion_main!(markdown_append);
