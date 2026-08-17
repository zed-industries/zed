use std::env;
use std::fs::File;
use std::io::prelude::*;
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

fn main() {
    let args: Vec<_> = env::args().collect();
    let mut f = File::open(&args[1]).unwrap();
    let mut s = String::new();
    f.read_to_string(&mut s).unwrap();

    let mut sink = NullSink {};
    let mut parser = Parser::new_from_str(&s);

    // Load events using our sink as the receiver.
    let begin = std::time::Instant::now();
    parser.load(&mut sink, true).unwrap();
    let end = std::time::Instant::now();

    if args.len() == 3 && args[2] == "--short" {
        println!("{}", (end - begin).as_nanos());
    } else {
        println!("Loaded {}MiB in {:?}", s.len() / 1024 / 1024, end - begin);
    }
}
