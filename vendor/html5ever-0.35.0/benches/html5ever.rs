#[macro_use]
extern crate criterion;
extern crate html5ever;

use std::fs;
use std::path::PathBuf;

use criterion::{BatchSize, Criterion};

use html5ever::tokenizer::{BufferQueue, Token, TokenSink, TokenSinkResult, Tokenizer};
use html5ever::{tendril::*, TokenizerResult};

struct Sink;

impl TokenSink for Sink {
    type Handle = ();

    fn process_token(&self, token: Token, _line_number: u64) -> TokenSinkResult<()> {
        // Don't use the token, but make sure we don't get
        // optimized out entirely.
        std::hint::black_box(token);
        TokenSinkResult::Continue
    }
}

fn run_bench(c: &mut Criterion, name: &str) {
    let mut path = PathBuf::from("./");
    path.push("data/bench/");
    path.push(name);
    let mut file = fs::File::open(&path).expect("can't open file");

    // Read the file and treat it as an infinitely repeating sequence of characters.
    let mut file_input = ByteTendril::new();
    file.read_to_tendril(&mut file_input)
        .expect("can't read file");
    let file_input: StrTendril = file_input.try_reinterpret().unwrap();
    let size = file_input.len();
    let mut stream = file_input.chars().cycle();

    // Break the input into chunks of 1024 chars (= a few kB).
    // This simulates reading from the network.
    let mut input = vec![];
    let mut total = 0usize;
    while total < size {
        // The by_ref() call is important, otherwise we get wrong results!
        // See rust-lang/rust#18045.
        let sz = std::cmp::min(1024, size - total);
        input.push(stream.by_ref().take(sz).collect::<String>().to_tendril());
        total += sz;
    }

    let test_name = format!("html tokenizing {name}");

    // Construct a buffer queue to feed to the tokenizer
    let buffer_queue = BufferQueue::default();
    for buf in input.into_iter() {
        buffer_queue.push_back(buf);
    }

    c.bench_function(&test_name, move |b| {
        b.iter_batched(
            || buffer_queue.clone(),
            |buffer_queue| {
                let tok = Tokenizer::new(Sink, Default::default());

                // Tokenize the entire input, ignoring any <script> elements we find along the way
                while tok.feed(&buffer_queue) != TokenizerResult::Done {}

                tok.end();
            },
            BatchSize::SmallInput,
        )
    });
}

fn html5ever_benchmark(c: &mut Criterion) {
    run_bench(c, "lipsum.html");
    run_bench(c, "lipsum-zh.html");
    run_bench(c, "medium-fragment.html");
    run_bench(c, "small-fragment.html");
    run_bench(c, "tiny-fragment.html");
    run_bench(c, "strong.html");
}

criterion_group!(benches, html5ever_benchmark);
criterion_main!(benches);
