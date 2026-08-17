#![allow(clippy::cast_possible_truncation, clippy::cast_precision_loss)]

use std::{env, fs::File, io::prelude::*};
use yaml_rust2::{
    parser::{MarkedEventReceiver, Parser},
    scanner::Marker,
    Event,
};

/// A sink which discards any event sent.
struct NullSink {}

impl MarkedEventReceiver for NullSink {
    fn on_event(&mut self, _: Event, _: Marker) {}
}

/// Parse the given input, returning elapsed time in nanoseconds.
fn do_parse(input: &str) -> u64 {
    let mut sink = NullSink {};
    let mut parser = Parser::new_from_str(input);
    let begin = std::time::Instant::now();
    parser.load(&mut sink, true).unwrap();
    let end = std::time::Instant::now();
    (end - begin).as_nanos() as u64
}

fn main() {
    let args: Vec<_> = env::args().collect();
    let iterations: u64 = args[2].parse().unwrap();
    let output_yaml = args.len() == 4 && args[3] == "--output-yaml";
    let mut f = File::open(&args[1]).unwrap();
    let mut s = String::new();
    f.read_to_string(&mut s).unwrap();

    // Warmup
    do_parse(&s);
    do_parse(&s);
    do_parse(&s);

    // Bench
    let times: Vec<_> = (0..iterations).map(|_| do_parse(&s)).collect();

    let mut sorted_times = times.clone();
    sorted_times.sort_unstable();

    // Compute relevant metrics.
    let sum: u64 = times.iter().sum();
    let avg = sum / iterations;
    let min = sorted_times[0];
    let max = sorted_times[(iterations - 1) as usize];
    let percentile95 = sorted_times[((95 * iterations) / 100) as usize];

    if output_yaml {
        println!("parser: yaml-rust2");
        println!("input: {}", args[1]);
        println!("average: {avg}");
        println!("min: {min}");
        println!("max: {max}");
        println!("percentile95: {percentile95}");
        println!("iterations: {iterations}");
        println!("times:");
        for time in &times {
            println!("  - {time}");
        }
    } else {
        println!("Average: {}s", (avg as f64) / 1_000_000_000.0);
        println!("Min: {}s", (min as f64) / 1_000_000_000.0);
        println!("Max: {}s", (max as f64) / 1_000_000_000.0);
        println!("95%: {}s", (percentile95 as f64) / 1_000_000_000.0);
    }
}
