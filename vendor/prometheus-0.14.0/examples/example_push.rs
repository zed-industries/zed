// Copyright 2019 TiKV Project Authors. Licensed under Apache-2.0.

use std::env;
use std::thread;
use std::time;

use getopts::Options;
use prometheus::{Counter, Histogram};

use lazy_static::lazy_static;
use prometheus::{labels, register_counter, register_histogram};

lazy_static! {
    static ref PUSH_COUNTER: Counter = register_counter!(
        "example_push_total",
        "Total number of prometheus client pushed."
    )
    .unwrap();
    static ref PUSH_REQ_HISTOGRAM: Histogram = register_histogram!(
        "example_push_request_duration_seconds",
        "The push request latencies in seconds."
    )
    .unwrap();
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optopt(
        "A",
        "addr",
        "prometheus pushgateway address",
        "default is 127.0.0.1:9091",
    );
    opts.optflag("h", "help", "print this help menu");

    let matches = opts.parse(&args).unwrap();
    if matches.opt_present("h") || !matches.opt_present("A") {
        let brief = format!("Usage: {} [options]", program);
        print!("{}", opts.usage(&brief));
        return;
    }
    println!("Pushing, please start Pushgateway first.");

    let address = matches.opt_str("A").unwrap_or("127.0.0.1:9091".to_owned());
    for _ in 0..5 {
        thread::sleep(time::Duration::from_secs(2));
        PUSH_COUNTER.inc();
        let metric_families = prometheus::gather();
        let _timer = PUSH_REQ_HISTOGRAM.start_timer(); // drop as observe
        prometheus::push_metrics(
            "example_push",
            labels! {"instance".to_owned() => "HAL-9000".to_owned(),},
            &address,
            metric_families,
            Some(prometheus::BasicAuthentication {
                username: "user".to_owned(),
                password: "pass".to_owned(),
            }),
        )
        .unwrap();
    }

    println!("Okay, please check the Pushgateway.");
}
