// Copyright 2019 TiKV Project Authors. Licensed under Apache-2.0.

fn main() {
    use std::thread;
    use std::time::Duration;

    use prometheus::Encoder;

    // A default ProcessCollector is registered automatically.
    let mut buffer = Vec::new();
    let encoder = prometheus::TextEncoder::new();
    for _ in 0..5 {
        let metric_families = prometheus::gather();
        encoder.encode(&metric_families, &mut buffer).unwrap();

        // Output to the standard output.
        println!("{}", String::from_utf8(buffer.clone()).unwrap());

        buffer.clear();
        thread::sleep(Duration::from_secs(1));
    }
}
