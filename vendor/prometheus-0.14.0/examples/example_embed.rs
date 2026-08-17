// Copyright 2019 TiKV Project Authors. Licensed under Apache-2.0.

use std::thread;
use std::time::Duration;

use prometheus::{Counter, CounterVec, Encoder, Gauge, GaugeVec, Opts, Registry, TextEncoder};

fn main() {
    let r = Registry::new();

    let counter_opts = Opts::new("test_counter", "test counter help")
        .const_label("a", "1")
        .const_label("b", "2");
    let counter = Counter::with_opts(counter_opts).unwrap();
    let counter_vec_opts = Opts::new("test_counter_vec", "test counter vector help")
        .const_label("a", "1")
        .const_label("b", "2");
    let counter_vec = CounterVec::new(counter_vec_opts, &["c", "d"]).unwrap();

    r.register(Box::new(counter.clone())).unwrap();
    r.register(Box::new(counter_vec.clone())).unwrap();

    let gauge_opts = Opts::new("test_gauge", "test gauge help")
        .const_label("a", "1")
        .const_label("b", "2");
    let gauge = Gauge::with_opts(gauge_opts).unwrap();
    let gauge_vec_opts = Opts::new("test_gauge_vec", "test gauge vector help")
        .const_label("a", "1")
        .const_label("b", "2");
    let gauge_vec = GaugeVec::new(gauge_vec_opts, &["c", "d"]).unwrap();

    r.register(Box::new(gauge.clone())).unwrap();
    r.register(Box::new(gauge_vec.clone())).unwrap();

    counter.inc();
    assert_eq!(counter.get() as u64, 1);
    counter.inc_by(42.0);
    assert_eq!(counter.get() as u64, 43);

    counter_vec.with_label_values(&["3", "4"]).inc();
    assert_eq!(counter_vec.with_label_values(&["3", "4"]).get() as u64, 1);

    counter_vec.with_label_values(&["3", "4"]).inc_by(42.0);
    assert_eq!(counter_vec.with_label_values(&["3", "4"]).get() as u64, 43);

    gauge.inc();
    assert_eq!(gauge.get() as u64, 1);
    gauge.add(42.0);
    assert_eq!(gauge.get() as u64, 43);

    gauge_vec.with_label_values(&["3", "4"]).inc();
    assert_eq!(gauge_vec.with_label_values(&["3", "4"]).get() as u64, 1);

    gauge_vec.with_label_values(&["3", "4"]).set(42.0);
    assert_eq!(gauge_vec.with_label_values(&["3", "4"]).get() as u64, 42);

    let c2 = counter.clone();
    let cv2 = counter_vec.clone();
    let g2 = gauge.clone();
    let gv2 = gauge_vec.clone();
    thread::spawn(move || {
        for _ in 0..10 {
            thread::sleep(Duration::from_millis(500));
            c2.inc();
            cv2.with_label_values(&["3", "4"]).inc();
            g2.inc();
            gv2.with_label_values(&["3", "4"]).inc();
        }
    });

    thread::spawn(move || {
        for _ in 0..5 {
            thread::sleep(Duration::from_secs(1));
            counter.inc();
            counter_vec.with_label_values(&["3", "4"]).inc();
            gauge.dec();
            gauge_vec.with_label_values(&["3", "4"]).set(42.0);
        }
    });

    // Choose your writer and encoder.
    let mut buffer = Vec::<u8>::new();
    let encoder = TextEncoder::new();
    for _ in 0..5 {
        let metric_families = r.gather();
        encoder.encode(&metric_families, &mut buffer).unwrap();

        // Output to the standard output.
        println!("{}", String::from_utf8(buffer.clone()).unwrap());

        buffer.clear();
        thread::sleep(Duration::from_secs(1));
    }
}
