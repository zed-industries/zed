// Copyright 2019 TiKV Project Authors. Licensed under Apache-2.0.

use prometheus::register_counter;

/// small example that uses rust 2018 style macro imports

#[allow(unused)]
fn main() {
    register_counter!("test_macro_3", "help");
    prometheus::register_counter_vec!("errorz", "errors", &["error"]).unwrap();
    prometheus::register_gauge!("test_macro_gauge_2", "help");
    prometheus::register_gauge_vec!("test_macro_gauge_vec_3", "help", &["a", "b"]);
    prometheus::register_histogram!("test_macro_histogram_4", "help", vec![1.0, 2.0]);
    prometheus::register_histogram_vec!(
        "test_macro_histogram_4",
        "help",
        &["a", "b"],
        vec![1.0, 2.0]
    );
    prometheus::register_int_counter!("meh", "foo");
    prometheus::register_int_counter_vec!("errorz", "errors", &["error"]).unwrap();
    prometheus::register_int_gauge!("test_macro_gauge_2", "help");
    prometheus::register_int_gauge_vec!("test_macro_gauge_vec_4", "help", &["a", "b"]);
}
