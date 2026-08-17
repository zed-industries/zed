// Copyright 2019 TiKV Project Authors. Licensed under Apache-2.0.

/// Create labels with specified name-value pairs.
///
/// # Examples
///
/// ```
/// # use std::collections::HashMap;
/// # use prometheus::labels;
/// # fn main() {
/// let labels = labels!{
///     "test" => "hello",
///     "foo" => "bar",
/// };
/// assert_eq!(labels.len(), 2);
/// assert!(labels.get("test").is_some());
/// assert_eq!(*(labels.get("test").unwrap()), "hello");
///
/// let labels: HashMap<&str, &str> = labels!{};
/// assert!(labels.is_empty());
/// # }
/// ```
#[macro_export]
macro_rules! labels {
    ( $( $ KEY : expr => $ VALUE : expr ),* $(,)? ) => {
        {
            use std::collections::HashMap;

            let mut lbs = HashMap::new();
            $(
                lbs.insert($KEY, $VALUE);
            )*

            lbs
        }
    };
}

#[test]
fn test_labels_without_trailing_comma() {
    let labels = labels! {
        "test" => "hello",
        "foo" => "bar"
    };
    assert_eq!(labels.len(), 2);
    assert!(labels.contains_key("test"));
    assert_eq!(*(labels.get("test").unwrap()), "hello");
}

/// Create an [`Opts`][crate::Opts].
///
/// # Examples
///
/// ```
/// # use prometheus::{labels, opts};
/// # fn main() {
/// let name = "test_opts";
/// let help = "test opts help";
///
/// let opts = opts!(name, help);
/// assert_eq!(opts.name, name);
/// assert_eq!(opts.help, help);
///
/// let opts = opts!(name, help, labels!{"test" => "hello", "foo" => "bar",});
/// assert_eq!(opts.const_labels.len(), 2);
/// assert!(opts.const_labels.get("foo").is_some());
/// assert_eq!(opts.const_labels.get("foo").unwrap(), "bar");
///
/// let opts = opts!(name,
///                  help,
///                  labels!{"test" => "hello", "foo" => "bar",},
///                  labels!{"ans" => "42",});
/// assert_eq!(opts.const_labels.len(), 3);
/// assert!(opts.const_labels.get("ans").is_some());
/// assert_eq!(opts.const_labels.get("ans").unwrap(), "42");
/// # }
/// ```
#[macro_export]
macro_rules! opts {
    ( $ NAME : expr , $ HELP : expr $ ( , $ CONST_LABELS : expr ) * $ ( , ) ? ) => {
        {
            use std::collections::HashMap;

            let opts = $crate::Opts::new($NAME, $HELP);
            let lbs = HashMap::<String, String>::new();
            $(
                #[allow(clippy::redundant_locals)]
                let mut lbs = lbs;
                lbs.extend($CONST_LABELS.iter().map(|(k, v)| ((*k).into(), (*v).into())));
            )*

            opts.const_labels(lbs)
        }
    }
}

#[test]
fn test_opts_trailing_comma() {
    let name = "test_opts";
    let help = "test opts help";

    let opts = opts!(name, help,);
    assert_eq!(opts.name, name);
    assert_eq!(opts.help, help);

    let opts = opts!(name, help, labels! {"test" => "hello", "foo" => "bar",},);
    assert_eq!(opts.const_labels.len(), 2);
    assert!(opts.const_labels.contains_key("foo"));
    assert_eq!(opts.const_labels.get("foo").unwrap(), "bar");

    let opts = opts!(
        name,
        help,
        labels! {"test" => "hello", "foo" => "bar",},
        labels! {"ans" => "42",},
    );
    assert_eq!(opts.const_labels.len(), 3);
    assert!(opts.const_labels.contains_key("ans"));
    assert_eq!(opts.const_labels.get("ans").unwrap(), "42");
}

/// Create a [`HistogramOpts`][crate::HistogramOpts].
///
/// # Examples
///
/// ```
/// # use prometheus::linear_buckets;
/// # use prometheus::{histogram_opts, labels};
/// # fn main() {
/// let name = "test_histogram_opts";
/// let help = "test opts help";
///
/// let opts = histogram_opts!(name, help);
/// assert_eq!(opts.common_opts.name, name);
/// assert_eq!(opts.common_opts.help, help);
///
/// let opts = histogram_opts!(name, help, linear_buckets(1.0, 0.5, 4).unwrap());
/// assert_eq!(opts.common_opts.name, name);
/// assert_eq!(opts.common_opts.help, help);
/// assert_eq!(opts.buckets.len(), 4);
///
/// let opts = histogram_opts!(name,
///                            help,
///                            vec![1.0, 2.0],
///                            labels!{"key".to_string() => "value".to_string(),});
/// assert_eq!(opts.common_opts.name, name);
/// assert_eq!(opts.common_opts.help, help);
/// assert_eq!(opts.buckets.len(), 2);
/// assert!(opts.common_opts.const_labels.get("key").is_some());
/// assert_eq!(opts.common_opts.const_labels.get("key").unwrap(), "value");
/// # }
/// ```
#[macro_export(local_inner_macros)]
macro_rules! histogram_opts {
    ($NAME:expr, $HELP:expr $(,)?) => {{
        $crate::HistogramOpts::new($NAME, $HELP)
    }};

    ($NAME:expr, $HELP:expr, $BUCKETS:expr $(,)?) => {{
        let hopts = histogram_opts!($NAME, $HELP);
        hopts.buckets($BUCKETS)
    }};

    ($NAME:expr, $HELP:expr, $BUCKETS:expr, $CONST_LABELS:expr $(,)?) => {{
        let hopts = histogram_opts!($NAME, $HELP, $BUCKETS);
        hopts.const_labels($CONST_LABELS)
    }};
}

#[test]
fn test_histogram_opts_trailing_comma() {
    use crate::linear_buckets;

    let name = "test_histogram_opts";
    let help = "test opts help";

    let opts = histogram_opts!(name, help,);
    assert_eq!(opts.common_opts.name, name);
    assert_eq!(opts.common_opts.help, help);

    let opts = histogram_opts!(name, help, linear_buckets(1.0, 0.5, 4).unwrap(),);
    assert_eq!(opts.common_opts.name, name);
    assert_eq!(opts.common_opts.help, help);
    assert_eq!(opts.buckets.len(), 4);

    let opts = histogram_opts!(
        name,
        help,
        vec![1.0, 2.0],
        labels! {"key".to_string() => "value".to_string(),},
    );
    assert_eq!(opts.common_opts.name, name);
    assert_eq!(opts.common_opts.help, help);
    assert_eq!(opts.buckets.len(), 2);
    assert!(opts.common_opts.const_labels.contains_key("key"));
    assert_eq!(opts.common_opts.const_labels.get("key").unwrap(), "value");
}

/// Create a [`Counter`][crate::Counter] and registers to default registry.
///
/// # Examples
///
/// ```
/// # use prometheus::{opts, register_counter};
/// # fn main() {
/// let opts = opts!("test_macro_counter_1", "help");
/// let res1 = register_counter!(opts);
/// assert!(res1.is_ok());
///
/// let res2 = register_counter!("test_macro_counter_2", "help");
/// assert!(res2.is_ok());
/// # }
/// ```
#[macro_export(local_inner_macros)]
macro_rules! register_counter {
    (@of_type $TYPE:ident, $OPTS:expr) => {{
        let counter = $crate::$TYPE::with_opts($OPTS).unwrap();
        $crate::register(Box::new(counter.clone())).map(|()| counter)
    }};

    ($OPTS:expr $(,)?) => {{
        register_counter!(@of_type Counter, $OPTS)
    }};

    ($NAME:expr, $HELP:expr $(,)?) => {{
        register_counter!(opts!($NAME, $HELP))
    }};
}

#[test]
fn test_register_counter_trailing_comma() {
    let opts = opts!("test_macro_counter_1", "help",);
    let res1 = register_counter!(opts,);
    assert!(res1.is_ok());

    let res2 = register_counter!("test_macro_counter_2", "help",);
    assert!(res2.is_ok());
}

/// Create a [`Counter`][crate::Counter] and registers to a custom registry.
///
/// # Examples
///
/// ```
/// # use prometheus::{register_counter_with_registry, opts};
/// # use prometheus::Registry;
/// # use std::collections::HashMap;
/// # fn main() {
/// let mut labels = HashMap::new();
/// labels.insert("mykey".to_string(), "myvalue".to_string());
/// let custom_registry = Registry::new_custom(Some("myprefix".to_string()), Some(labels)).unwrap();
/// let opts = opts!("test_macro_counter_1", "help");
/// let res1 = register_counter_with_registry!(opts, custom_registry);
/// assert!(res1.is_ok());
///
/// let res2 = register_counter_with_registry!("test_macro_counter_2", "help", custom_registry);
/// assert!(res2.is_ok());
/// # }
/// ```
#[macro_export(local_inner_macros)]
macro_rules! register_counter_with_registry {
    (@of_type $TYPE: ident, $OPTS:expr, $REGISTRY:expr) => {{
        let counter = $crate::$TYPE::with_opts($OPTS).unwrap();
        $REGISTRY.register(Box::new(counter.clone())).map(|()| counter)
    }};

    ($OPTS:expr, $REGISTRY:expr $(,)?) => {{
        register_counter_with_registry!(@of_type Counter, $OPTS, $REGISTRY)
    }};

    ($NAME:expr, $HELP:expr, $REGISTRY:expr $(,)?) => {{
        register_counter_with_registry!(opts!($NAME, $HELP), $REGISTRY)
    }};
}

#[test]
fn test_register_counter_with_registry_trailing_comma() {
    use crate::Registry;
    use std::collections::HashMap;

    let mut labels = HashMap::new();
    labels.insert("mykey".to_string(), "myvalue".to_string());
    let custom_registry = Registry::new_custom(Some("myprefix".to_string()), Some(labels)).unwrap();
    let opts = opts!("test_macro_counter_1", "help",);
    let res1 = register_counter_with_registry!(opts, custom_registry,);
    assert!(res1.is_ok());

    let res2 = register_counter_with_registry!("test_macro_counter_2", "help", custom_registry,);
    assert!(res2.is_ok());
}

/// Create an [`IntCounter`][crate::IntCounter] and registers to default registry.
///
/// View docs of `register_counter` for examples.
#[macro_export(local_inner_macros)]
macro_rules! register_int_counter {
    ($OPTS:expr $(,)?) => {{
        register_counter!(@of_type IntCounter, $OPTS)
    }};

    ($NAME:expr, $HELP:expr $(,)?) => {{
        register_int_counter!(opts!($NAME, $HELP))
    }};
}

/// Create an [`IntCounter`][crate::IntCounter] and registers to a custom registry.
///
/// View docs of `register_counter_with_registry` for examples.
#[macro_export(local_inner_macros)]
macro_rules! register_int_counter_with_registry {
    ($OPTS:expr, $REGISTRY:expr $(,)?) => {{
        register_counter_with_registry!(@of_type IntCounter, $OPTS, $REGISTRY)
    }};

    ($NAME:expr, $HELP:expr, $REGISTRY:expr $(,)?) => {{
        register_int_counter_with_registry!(opts!($NAME, $HELP), $REGISTRY)
    }};
}

#[test]
fn test_register_int_counter() {
    use crate::Registry;
    use std::collections::HashMap;

    let opts = opts!("test_opts_int_counter_1", "help");
    let res = register_int_counter!(opts);
    assert!(res.is_ok());

    let res = register_int_counter!("test_opts_int_counter_2", "help");
    assert!(res.is_ok());

    let opts = opts!("test_opts_int_counter_3", "help",);
    let res = register_int_counter!(opts,);
    assert!(res.is_ok());

    let res = register_int_counter!("test_opts_int_counter_4", "help",);
    assert!(res.is_ok());

    let mut labels = HashMap::new();
    labels.insert("mykey".to_string(), "myvalue".to_string());
    let custom_registry = Registry::new_custom(Some("myprefix".to_string()), Some(labels)).unwrap();

    let opts = opts!("test_opts_int_counter_1", "help");
    let res = register_int_counter_with_registry!(opts, custom_registry);
    assert!(res.is_ok());

    let res =
        register_int_counter_with_registry!("test_opts_int_counter_2", "help", custom_registry);
    assert!(res.is_ok());

    let opts = opts!("test_opts_int_counter_3", "help");
    let res = register_int_counter_with_registry!(opts, custom_registry,);
    assert!(res.is_ok());

    let res =
        register_int_counter_with_registry!("test_opts_int_counter_4", "help", custom_registry,);
    assert!(res.is_ok());
}

#[macro_export]
#[doc(hidden)]
macro_rules! __register_counter_vec {
    ($TYPE:ident, $OPTS:expr, $LABELS_NAMES:expr) => {{
        let counter_vec = $crate::$TYPE::new($OPTS, $LABELS_NAMES).unwrap();
        $crate::register(Box::new(counter_vec.clone())).map(|()| counter_vec)
    }};

    ($TYPE:ident, $OPTS:expr, $LABELS_NAMES:expr, $REGISTRY:expr) => {{
        let counter_vec = $crate::$TYPE::new($OPTS, $LABELS_NAMES).unwrap();
        $REGISTRY
            .register(Box::new(counter_vec.clone()))
            .map(|()| counter_vec)
    }};
}

/// Create a [`CounterVec`][crate::CounterVec] and registers to default registry.
///
/// # Examples
///
/// ```
/// # use prometheus::{opts, register_counter_vec};
/// # fn main() {
/// let opts = opts!("test_macro_counter_vec_1", "help");
/// let counter_vec = register_counter_vec!(opts, &["a", "b"]);
/// assert!(counter_vec.is_ok());
///
/// let counter_vec = register_counter_vec!("test_macro_counter_vec_2", "help", &["a", "b"]);
/// assert!(counter_vec.is_ok());
/// # }
/// ```
#[macro_export(local_inner_macros)]
macro_rules! register_counter_vec {
    ($OPTS:expr, $LABELS_NAMES:expr $(,)?) => {{
        __register_counter_vec!(CounterVec, $OPTS, $LABELS_NAMES)
    }};

    ($NAME:expr, $HELP:expr, $LABELS_NAMES:expr $(,)?) => {{
        register_counter_vec!(opts!($NAME, $HELP), $LABELS_NAMES)
    }};
}

#[test]
fn test_register_counter_vec_trailing_comma() {
    let opts = opts!("test_macro_counter_vec_1", "help",);
    let counter_vec = register_counter_vec!(opts, &["a", "b"],);
    assert!(counter_vec.is_ok());

    let counter_vec = register_counter_vec!("test_macro_counter_vec_2", "help", &["a", "b"],);
    assert!(counter_vec.is_ok());
}

/// Create a [`CounterVec`][crate::CounterVec] and registers to a custom registry.
///
/// # Examples
///
/// ```
/// # use prometheus::{register_counter_vec_with_registry, opts};
/// # use prometheus::Registry;
/// # use std::collections::HashMap;
/// # fn main() {
/// let mut labels = HashMap::new();
/// labels.insert("mykey".to_string(), "myvalue".to_string());
/// let custom_registry = Registry::new_custom(Some("myprefix".to_string()), Some(labels)).unwrap();
///
/// let opts = opts!("test_macro_counter_vec_1", "help");
/// let counter_vec = register_counter_vec_with_registry!(opts, &["a", "b"], custom_registry);
/// assert!(counter_vec.is_ok());
///
/// let counter_vec = register_counter_vec_with_registry!("test_macro_counter_vec_2", "help", &["a", "b"], custom_registry);
/// assert!(counter_vec.is_ok());
/// # }
/// ```
#[macro_export(local_inner_macros)]
macro_rules! register_counter_vec_with_registry {
    ($OPTS:expr, $LABELS_NAMES:expr, $REGISTRY:expr $(,)?) => {{
        __register_counter_vec!(CounterVec, $OPTS, $LABELS_NAMES, $REGISTRY)
    }};

    ($NAME:expr, $HELP:expr, $LABELS_NAMES:expr, $REGISTRY:expr $(,)?) => {{
        register_counter_vec_with_registry!(opts!($NAME, $HELP), $LABELS_NAMES, $REGISTRY)
    }};
}

#[test]
fn test_register_counter_vec_with_registry_trailing_comma() {
    use crate::Registry;
    use std::collections::HashMap;

    let mut labels = HashMap::new();
    labels.insert("mykey".to_string(), "myvalue".to_string());
    let custom_registry = Registry::new_custom(Some("myprefix".to_string()), Some(labels)).unwrap();

    let opts = opts!("test_macro_counter_vec_1", "help",);
    let counter_vec = register_counter_vec_with_registry!(opts, &["a", "b"], custom_registry,);
    assert!(counter_vec.is_ok());

    let counter_vec = register_counter_vec_with_registry!(
        "test_macro_counter_vec_2",
        "help",
        &["a", "b"],
        custom_registry,
    );
    assert!(counter_vec.is_ok());
}

/// Create an [`IntCounterVec`][crate::IntCounterVec] and registers to default registry.
///
/// View docs of `register_counter_vec` for examples.
#[macro_export(local_inner_macros)]
macro_rules! register_int_counter_vec {
    ($OPTS:expr, $LABELS_NAMES:expr $(,)?) => {{
        __register_counter_vec!(IntCounterVec, $OPTS, $LABELS_NAMES)
    }};

    ($NAME:expr, $HELP:expr, $LABELS_NAMES:expr $(,)?) => {{
        register_int_counter_vec!(opts!($NAME, $HELP), $LABELS_NAMES)
    }};
}

/// Create an [`IntCounterVec`][crate::IntCounterVec] and registers to a custom registry.
///
/// View docs of `register_counter_vec_with_registry` for examples.
#[macro_export(local_inner_macros)]
macro_rules! register_int_counter_vec_with_registry {
    ($OPTS:expr, $LABELS_NAMES:expr, $REGISTRY:expr $(,)?) => {{
        __register_counter_vec!(IntCounterVec, $OPTS, $LABELS_NAMES, $REGISTRY)
    }};

    ($NAME:expr, $HELP:expr, $LABELS_NAMES:expr, $REGISTRY:expr $(,)?) => {{
        register_int_counter_vec_with_registry!(opts!($NAME, $HELP), $LABELS_NAMES, $REGISTRY)
    }};
}

#[test]
fn test_register_int_counter_vec() {
    use crate::Registry;
    use std::collections::HashMap;

    let opts = opts!("test_opts_int_counter_vec_1", "help");
    let res = register_int_counter_vec!(opts, &["a", "b"]);
    assert!(res.is_ok());

    let res = register_int_counter_vec!("test_opts_int_counter_vec_2", "help", &["a", "b"]);
    assert!(res.is_ok());

    let opts = opts!("test_opts_int_counter_vec_3", "help",);
    let res = register_int_counter_vec!(opts, &["a", "b"],);
    assert!(res.is_ok());

    let res = register_int_counter_vec!("test_opts_int_counter_vec_4", "help", &["a", "b"],);
    assert!(res.is_ok());

    let mut labels = HashMap::new();
    labels.insert("mykey".to_string(), "myvalue".to_string());
    let custom_registry = Registry::new_custom(Some("myprefix".to_string()), Some(labels)).unwrap();

    let opts = opts!("test_opts_int_counter_vec_1", "help");
    let res = register_int_counter_vec_with_registry!(opts, &["a", "b"], custom_registry);
    assert!(res.is_ok());

    let res = register_int_counter_vec_with_registry!(
        "test_opts_int_counter_vec_2",
        "help",
        &["a", "b"],
        custom_registry
    );
    assert!(res.is_ok());

    let opts = opts!("test_opts_int_counter_vec_3", "help");
    let res = register_int_counter_vec_with_registry!(opts, &["a", "b"], custom_registry,);
    assert!(res.is_ok());

    let res = register_int_counter_vec_with_registry!(
        "test_opts_int_counter_vec_4",
        "help",
        &["a", "b"],
        custom_registry,
    );
    assert!(res.is_ok());
}

#[macro_export]
#[doc(hidden)]
macro_rules! __register_gauge {
    ($TYPE:ident, $OPTS:expr) => {{
        let gauge = $crate::$TYPE::with_opts($OPTS).unwrap();
        $crate::register(Box::new(gauge.clone())).map(|()| gauge)
    }};

    ($TYPE:ident, $OPTS:expr, $REGISTRY:expr) => {{
        let gauge = $crate::$TYPE::with_opts($OPTS).unwrap();
        $REGISTRY.register(Box::new(gauge.clone())).map(|()| gauge)
    }};
}

/// Create a [`Gauge`][crate::Gauge] and registers to default registry.
///
/// # Examples
///
/// ```
/// # use prometheus::{opts, register_gauge};
/// # fn main() {
/// let opts = opts!("test_macro_gauge", "help");
/// let res1 = register_gauge!(opts);
/// assert!(res1.is_ok());
///
/// let res2 = register_gauge!("test_macro_gauge_2", "help");
/// assert!(res2.is_ok());
/// # }
/// ```
#[macro_export(local_inner_macros)]
macro_rules! register_gauge {
    ($OPTS:expr $(,)?) => {{
        __register_gauge!(Gauge, $OPTS)
    }};

    ($NAME:expr, $HELP:expr $(,)?) => {{
        register_gauge!(opts!($NAME, $HELP))
    }};
}

#[test]
fn test_register_gauge_trailing_comma() {
    let opts = opts!("test_macro_gauge", "help",);
    let res1 = register_gauge!(opts,);
    assert!(res1.is_ok());

    let res2 = register_gauge!("test_macro_gauge_2", "help",);
    assert!(res2.is_ok());
}

/// Create a [`Gauge`][crate::Gauge] and registers to a custom registry.
///
/// # Examples
///
/// ```
/// # use prometheus::{register_gauge_with_registry, opts};
/// # use prometheus::Registry;
/// # use std::collections::HashMap;
/// # fn main() {
/// let mut labels = HashMap::new();
/// labels.insert("mykey".to_string(), "myvalue".to_string());
/// let custom_registry = Registry::new_custom(Some("myprefix".to_string()), Some(labels)).unwrap();
///
/// let opts = opts!("test_macro_gauge", "help");
/// let res1 = register_gauge_with_registry!(opts, custom_registry);
/// assert!(res1.is_ok());
///
/// let res2 = register_gauge_with_registry!("test_macro_gauge_2", "help", custom_registry);
/// assert!(res2.is_ok());
/// # }
/// ```
#[macro_export(local_inner_macros)]
macro_rules! register_gauge_with_registry {
    ($OPTS:expr, $REGISTRY:expr $(,)?) => {{
        __register_gauge!(Gauge, $OPTS, $REGISTRY)
    }};

    ($NAME:expr, $HELP:expr, $REGISTRY:expr $(,)?) => {{
        register_gauge_with_registry!(opts!($NAME, $HELP), $REGISTRY)
    }};
}

#[test]
fn test_register_gauge_with_registry_trailing_comma() {
    use crate::Registry;
    use std::collections::HashMap;

    let mut labels = HashMap::new();
    labels.insert("mykey".to_string(), "myvalue".to_string());
    let custom_registry = Registry::new_custom(Some("myprefix".to_string()), Some(labels)).unwrap();

    let opts = opts!("test_macro_gauge", "help",);
    let res1 = register_gauge_with_registry!(opts, custom_registry,);
    assert!(res1.is_ok());

    let res2 = register_gauge_with_registry!("test_macro_gauge_2", "help", custom_registry,);
    assert!(res2.is_ok());
}

/// Create an [`IntGauge`][crate::IntGauge] and registers to default registry.
///
/// View docs of `register_gauge` for examples.
#[macro_export(local_inner_macros)]
macro_rules! register_int_gauge {
    ($OPTS:expr $(,)?) => {{
        __register_gauge!(IntGauge, $OPTS)
    }};

    ($NAME:expr, $HELP:expr $(,)?) => {{
        register_int_gauge!(opts!($NAME, $HELP))
    }};
}

/// Create an [`IntGauge`][crate::IntGauge] and registers to a custom registry.
///
/// View docs of `register_gauge_with_registry` for examples.
#[macro_export(local_inner_macros)]
macro_rules! register_int_gauge_with_registry {
    ($OPTS:expr, $REGISTRY:expr $(,)?) => {{
        __register_gauge!(IntGauge, $OPTS, $REGISTRY)
    }};

    ($NAME:expr, $HELP:expr, $REGISTRY:expr $(,)?) => {{
        register_int_gauge_with_registry!(opts!($NAME, $HELP), $REGISTRY)
    }};
}

#[macro_export]
#[doc(hidden)]
macro_rules! __register_gauge_vec {
    ($TYPE:ident, $OPTS:expr, $LABELS_NAMES:expr $(,)?) => {{
        let gauge_vec = $crate::$TYPE::new($OPTS, $LABELS_NAMES).unwrap();
        $crate::register(Box::new(gauge_vec.clone())).map(|()| gauge_vec)
    }};

    ($TYPE:ident, $OPTS:expr, $LABELS_NAMES:expr, $REGISTRY:expr $(,)?) => {{
        let gauge_vec = $crate::$TYPE::new($OPTS, $LABELS_NAMES).unwrap();
        $REGISTRY
            .register(Box::new(gauge_vec.clone()))
            .map(|()| gauge_vec)
    }};
}

#[test]
fn test_register_int_gauge() {
    use crate::Registry;
    use std::collections::HashMap;

    let opts = opts!("test_opts_int_gauge_1", "help");
    let res = register_int_gauge!(opts);
    assert!(res.is_ok());

    let res = register_int_gauge!("test_opts_int_gauge_2", "help");
    assert!(res.is_ok());

    let opts = opts!("test_opts_int_gauge_3", "help",);
    let res = register_int_gauge!(opts,);
    assert!(res.is_ok());

    let res = register_int_gauge!("test_opts_int_gauge_4", "help",);
    assert!(res.is_ok());

    let mut labels = HashMap::new();
    labels.insert("mykey".to_string(), "myvalue".to_string());
    let custom_registry = Registry::new_custom(Some("myprefix".to_string()), Some(labels)).unwrap();

    let opts = opts!("test_opts_int_gauge_1", "help");
    let res = register_int_gauge_with_registry!(opts, custom_registry);
    assert!(res.is_ok());

    let res = register_int_gauge_with_registry!("test_opts_int_gauge_2", "help", custom_registry);
    assert!(res.is_ok());

    let opts = opts!("test_opts_int_gauge_3", "help");
    let res = register_int_gauge_with_registry!(opts, custom_registry,);
    assert!(res.is_ok());

    let res = register_int_gauge_with_registry!("test_opts_int_gauge_4", "help", custom_registry,);
    assert!(res.is_ok());
}

/// Create a [`GaugeVec`][crate::GaugeVec] and registers to default registry.
///
/// # Examples
///
/// ```
/// # use prometheus::{opts, register_gauge_vec};
/// # fn main() {
/// let opts = opts!("test_macro_gauge_vec_1", "help");
/// let gauge_vec = register_gauge_vec!(opts, &["a", "b"]);
/// assert!(gauge_vec.is_ok());
///
/// let gauge_vec = register_gauge_vec!("test_macro_gauge_vec_2", "help", &["a", "b"]);
/// assert!(gauge_vec.is_ok());
/// # }
/// ```
#[macro_export(local_inner_macros)]
macro_rules! register_gauge_vec {
    ($OPTS:expr, $LABELS_NAMES:expr $(,)?) => {{
        __register_gauge_vec!(GaugeVec, $OPTS, $LABELS_NAMES)
    }};

    ($NAME:expr, $HELP:expr, $LABELS_NAMES:expr $(,)?) => {{
        register_gauge_vec!(opts!($NAME, $HELP), $LABELS_NAMES)
    }};
}

#[test]
fn test_register_gauge_vec_trailing_comma() {
    let opts = opts!("test_macro_gauge_vec_1", "help",);
    let gauge_vec = register_gauge_vec!(opts, &["a", "b"],);
    assert!(gauge_vec.is_ok());

    let gauge_vec = register_gauge_vec!("test_macro_gauge_vec_2", "help", &["a", "b"],);
    assert!(gauge_vec.is_ok());
}

/// Create a [`GaugeVec`][crate::GaugeVec] and registers to a custom registry.
///
/// # Examples
///
/// ```
/// # use prometheus::{register_gauge_vec_with_registry, opts};
/// # use prometheus::Registry;
/// # use std::collections::HashMap;
/// # fn main() {
/// let mut labels = HashMap::new();
/// labels.insert("mykey".to_string(), "myvalue".to_string());
/// let custom_registry = Registry::new_custom(Some("myprefix".to_string()), Some(labels)).unwrap();
///
/// let opts = opts!("test_macro_gauge_vec_1", "help");
/// let gauge_vec = register_gauge_vec_with_registry!(opts, &["a", "b"], custom_registry);
/// assert!(gauge_vec.is_ok());
///
/// let gauge_vec = register_gauge_vec_with_registry!("test_macro_gauge_vec_2", "help", &["a", "b"], custom_registry);
/// assert!(gauge_vec.is_ok());
/// # }
/// ```
#[macro_export(local_inner_macros)]
macro_rules! register_gauge_vec_with_registry {
    ($OPTS:expr, $LABELS_NAMES:expr, $REGISTRY:expr $(,)?) => {{
        __register_gauge_vec!(GaugeVec, $OPTS, $LABELS_NAMES, $REGISTRY)
    }};

    ($NAME:expr, $HELP:expr, $LABELS_NAMES:expr, $REGISTRY:expr $(,)?) => {{
        register_gauge_vec_with_registry!(opts!($NAME, $HELP), $LABELS_NAMES, $REGISTRY)
    }};
}

#[test]
fn test_register_gauge_vec_with_registry_trailing_comma() {
    use crate::Registry;
    use std::collections::HashMap;

    let mut labels = HashMap::new();
    labels.insert("mykey".to_string(), "myvalue".to_string());
    let custom_registry = Registry::new_custom(Some("myprefix".to_string()), Some(labels)).unwrap();

    let opts = opts!("test_macro_gauge_vec_1", "help",);
    let gauge_vec = register_gauge_vec_with_registry!(opts, &["a", "b"], custom_registry,);
    assert!(gauge_vec.is_ok());

    let gauge_vec = register_gauge_vec_with_registry!(
        "test_macro_gauge_vec_2",
        "help",
        &["a", "b"],
        custom_registry,
    );
    assert!(gauge_vec.is_ok());
}

/// Create an [`IntGaugeVec`][crate::IntGaugeVec] and registers to default registry.
///
/// View docs of `register_gauge_vec` for examples.
#[macro_export(local_inner_macros)]
macro_rules! register_int_gauge_vec {
    ($OPTS:expr, $LABELS_NAMES:expr $(,)?) => {{
        __register_gauge_vec!(IntGaugeVec, $OPTS, $LABELS_NAMES)
    }};

    ($NAME:expr, $HELP:expr, $LABELS_NAMES:expr $(,)?) => {{
        register_int_gauge_vec!(opts!($NAME, $HELP), $LABELS_NAMES)
    }};
}

/// Create an [`IntGaugeVec`][crate::IntGaugeVec] and registers to a custom registry.
///
/// View docs of `register_gauge_vec_with_registry` for examples.
#[macro_export(local_inner_macros)]
macro_rules! register_int_gauge_vec_with_registry {
    ($OPTS:expr, $LABELS_NAMES:expr, $REGISTRY:expr $(,)?) => {{
        __register_gauge_vec!(IntGaugeVec, $OPTS, $LABELS_NAMES, $REGISTRY)
    }};

    ($NAME:expr, $HELP:expr, $LABELS_NAMES:expr, $REGISTRY:expr $(,)?) => {{
        register_int_gauge_vec_with_registry!(opts!($NAME, $HELP), $LABELS_NAMES, $REGISTRY)
    }};
}

#[test]
fn test_register_int_gauge_vec() {
    use crate::Registry;
    use std::collections::HashMap;

    let opts = opts!("test_opts_int_gauge_vec_1", "help");
    let res = register_int_gauge_vec!(opts, &["a", "b"]);
    assert!(res.is_ok());

    let res = register_int_gauge_vec!("test_opts_int_gauge_vec_2", "help", &["a", "b"]);
    assert!(res.is_ok());

    let opts = opts!("test_opts_int_gauge_vec_3", "help",);
    let res = register_int_gauge_vec!(opts, &["a", "b"],);
    assert!(res.is_ok());

    let res = register_int_gauge_vec!("test_opts_int_gauge_vec_4", "help", &["a", "b"],);
    assert!(res.is_ok());

    let mut labels = HashMap::new();
    labels.insert("mykey".to_string(), "myvalue".to_string());
    let custom_registry = Registry::new_custom(Some("myprefix".to_string()), Some(labels)).unwrap();

    let opts = opts!("test_opts_int_gauge_vec_1", "help");
    let res = register_int_gauge_vec_with_registry!(opts, &["a", "b"], custom_registry);
    assert!(res.is_ok());

    let res = register_int_gauge_vec_with_registry!(
        "test_opts_int_gauge_vec_2",
        "help",
        &["a", "b"],
        custom_registry
    );
    assert!(res.is_ok());

    let opts = opts!("test_opts_int_gauge_vec_3", "help");
    let res = register_int_gauge_vec_with_registry!(opts, &["a", "b"], custom_registry,);
    assert!(res.is_ok());

    let res = register_int_gauge_vec_with_registry!(
        "test_opts_int_gauge_vec_4",
        "help",
        &["a", "b"],
        custom_registry,
    );
    assert!(res.is_ok());
}

/// Create a [`Histogram`][crate::Histogram] and registers to default registry.
///
/// # Examples
///
/// ```
/// # use prometheus::{histogram_opts, register_histogram};
/// # fn main() {
/// let opts = histogram_opts!("test_macro_histogram", "help");
/// let res1 = register_histogram!(opts);
/// assert!(res1.is_ok());
///
/// let res2 = register_histogram!("test_macro_histogram_2", "help");
/// assert!(res2.is_ok());
///
/// let res3 = register_histogram!("test_macro_histogram_4",
///                                 "help",
///                                 vec![1.0, 2.0]);
/// assert!(res3.is_ok());
/// # }
/// ```
#[macro_export(local_inner_macros)]
macro_rules! register_histogram {
    ($NAME:expr, $HELP:expr $(,)?) => {
        register_histogram!(histogram_opts!($NAME, $HELP))
    };

    ($NAME:expr, $HELP:expr, $BUCKETS:expr $(,)?) => {
        register_histogram!(histogram_opts!($NAME, $HELP, $BUCKETS))
    };

    ($HOPTS:expr $(,)?) => {{
        let histogram = $crate::Histogram::with_opts($HOPTS).unwrap();
        $crate::register(Box::new(histogram.clone())).map(|()| histogram)
    }};
}

#[test]
fn test_register_histogram_trailing_comma() {
    let opts = histogram_opts!("test_macro_histogram", "help",);
    let res1 = register_histogram!(opts,);
    assert!(res1.is_ok());

    let res2 = register_histogram!("test_macro_histogram_2", "help",);
    assert!(res2.is_ok());

    let res3 = register_histogram!("test_macro_histogram_4", "help", vec![1.0, 2.0],);
    assert!(res3.is_ok());
}

/// Create a [`Histogram`][crate::Histogram] and registers to a custom registry.
///
/// # Examples
///
/// ```
/// # use prometheus::{register_histogram_with_registry, histogram_opts};
/// # use prometheus::Registry;
/// # use std::collections::HashMap;
/// # fn main() {
/// let mut labels = HashMap::new();
/// labels.insert("mykey".to_string(), "myvalue".to_string());
/// let custom_registry = Registry::new_custom(Some("myprefix".to_string()), Some(labels)).unwrap();
///
/// let opts = histogram_opts!("test_macro_histogram", "help");
/// let res1 = register_histogram_with_registry!(opts, custom_registry);
/// assert!(res1.is_ok());
///
/// let res2 = register_histogram_with_registry!("test_macro_histogram_2", "help", custom_registry);
/// assert!(res2.is_ok());
///
/// let res3 = register_histogram_with_registry!("test_macro_histogram_4",
///                                 "help",
///                                 vec![1.0, 2.0], custom_registry);
/// assert!(res3.is_ok());
/// # }
/// ```
#[macro_export(local_inner_macros)]
macro_rules! register_histogram_with_registry {
    ($NAME:expr, $HELP:expr, $REGISTRY:expr $(,)?) => {
        register_histogram_with_registry!(histogram_opts!($NAME, $HELP), $REGISTRY)
    };

    ($NAME:expr, $HELP:expr, $BUCKETS:expr, $REGISTRY:expr $(,)?) => {
        register_histogram_with_registry!(histogram_opts!($NAME, $HELP, $BUCKETS), $REGISTRY)
    };

    ($HOPTS:expr, $REGISTRY:expr $(,)?) => {{
        let histogram = $crate::Histogram::with_opts($HOPTS).unwrap();
        $REGISTRY
            .register(Box::new(histogram.clone()))
            .map(|()| histogram)
    }};
}

#[test]
fn test_register_histogram_with_registry_trailing_comma() {
    use crate::Registry;
    use std::collections::HashMap;

    let mut labels = HashMap::new();
    labels.insert("mykey".to_string(), "myvalue".to_string());
    let custom_registry = Registry::new_custom(Some("myprefix".to_string()), Some(labels)).unwrap();

    let opts = histogram_opts!("test_macro_histogram", "help",);
    let res1 = register_histogram_with_registry!(opts, custom_registry,);
    assert!(res1.is_ok());

    let res2 =
        register_histogram_with_registry!("test_macro_histogram_2", "help", custom_registry,);
    assert!(res2.is_ok());

    let res3 = register_histogram_with_registry!(
        "test_macro_histogram_4",
        "help",
        vec![1.0, 2.0],
        custom_registry,
    );
    assert!(res3.is_ok());
}

/// Create a [`HistogramVec`][crate::HistogramVec] and registers to default registry.
///
/// # Examples
///
/// ```
/// # use prometheus::{histogram_opts, register_histogram_vec};
/// # fn main() {
/// let opts = histogram_opts!("test_macro_histogram_vec_1", "help");
/// let histogram_vec = register_histogram_vec!(opts, &["a", "b"]);
/// assert!(histogram_vec.is_ok());
///
/// let histogram_vec =
///     register_histogram_vec!("test_macro_histogram_vec_2", "help", &["a", "b"]);
/// assert!(histogram_vec.is_ok());
///
/// let histogram_vec = register_histogram_vec!("test_macro_histogram_vec_3",
///                                             "help",
///                                             &["test_label"],
///                                             vec![0.0, 1.0, 2.0]);
/// assert!(histogram_vec.is_ok());
/// # }
/// ```
#[macro_export(local_inner_macros)]
macro_rules! register_histogram_vec {
    ($HOPTS:expr, $LABELS_NAMES:expr $(,)?) => {{
        let histogram_vec = $crate::HistogramVec::new($HOPTS, $LABELS_NAMES).unwrap();
        $crate::register(Box::new(histogram_vec.clone())).map(|()| histogram_vec)
    }};

    ($NAME:expr, $HELP:expr, $LABELS_NAMES:expr $(,)?) => {{
        register_histogram_vec!(histogram_opts!($NAME, $HELP), $LABELS_NAMES)
    }};

    ($NAME:expr, $HELP:expr, $LABELS_NAMES:expr, $BUCKETS:expr $(,)?) => {{
        register_histogram_vec!(histogram_opts!($NAME, $HELP, $BUCKETS), $LABELS_NAMES)
    }};
}

#[test]
fn test_register_histogram_vec_trailing_comma() {
    let opts = histogram_opts!("test_macro_histogram_vec_1", "help",);
    let histogram_vec = register_histogram_vec!(opts, &["a", "b"],);
    assert!(histogram_vec.is_ok());

    let histogram_vec = register_histogram_vec!("test_macro_histogram_vec_2", "help", &["a", "b"],);
    assert!(histogram_vec.is_ok());

    let histogram_vec = register_histogram_vec!(
        "test_macro_histogram_vec_3",
        "help",
        &["test_label"],
        vec![0.0, 1.0, 2.0],
    );
    assert!(histogram_vec.is_ok());
}

/// Create a [`HistogramVec`][crate::HistogramVec] and registers to a custom registry.
///
/// # Examples
///
/// ```
/// # use prometheus::{register_histogram_vec_with_registry, histogram_opts};
/// # use prometheus::Registry;
/// # use std::collections::HashMap;
/// # fn main() {
/// let mut labels = HashMap::new();
/// labels.insert("mykey".to_string(), "myvalue".to_string());
/// let custom_registry = Registry::new_custom(Some("myprefix".to_string()), Some(labels)).unwrap();
///
/// let opts = histogram_opts!("test_macro_histogram_vec_1", "help");
/// let histogram_vec = register_histogram_vec_with_registry!(opts, &["a", "b"], custom_registry);
/// assert!(histogram_vec.is_ok());
///
/// let histogram_vec =
///     register_histogram_vec_with_registry!("test_macro_histogram_vec_2", "help", &["a", "b"], custom_registry);
/// assert!(histogram_vec.is_ok());
///
/// let histogram_vec = register_histogram_vec_with_registry!("test_macro_histogram_vec_3",
///                                             "help",
///                                             &["test_label"],
///                                             vec![0.0, 1.0, 2.0], custom_registry);
/// assert!(histogram_vec.is_ok());
/// # }
/// ```
#[macro_export(local_inner_macros)]
macro_rules! register_histogram_vec_with_registry {
    ($HOPTS:expr, $LABELS_NAMES:expr, $REGISTRY:expr $(,)?) => {{
        let histogram_vec = $crate::HistogramVec::new($HOPTS, $LABELS_NAMES).unwrap();
        $REGISTRY
            .register(Box::new(histogram_vec.clone()))
            .map(|()| histogram_vec)
    }};

    ($NAME:expr, $HELP:expr, $LABELS_NAMES:expr, $REGISTRY:expr $(,)?) => {{
        register_histogram_vec_with_registry!(
            histogram_opts!($NAME, $HELP),
            $LABELS_NAMES,
            $REGISTRY
        )
    }};

    ($NAME:expr, $HELP:expr, $LABELS_NAMES:expr, $BUCKETS:expr, $REGISTRY:expr $(,)?) => {{
        register_histogram_vec_with_registry!(
            histogram_opts!($NAME, $HELP, $BUCKETS),
            $LABELS_NAMES,
            $REGISTRY
        )
    }};
}

#[test]
fn test_register_histogram_vec_with_registry_trailing_comma() {
    use crate::Registry;
    use std::collections::HashMap;

    let mut labels = HashMap::new();
    labels.insert("mykey".to_string(), "myvalue".to_string());
    let custom_registry = Registry::new_custom(Some("myprefix".to_string()), Some(labels)).unwrap();

    let opts = histogram_opts!("test_macro_histogram_vec_1", "help",);
    let histogram_vec = register_histogram_vec_with_registry!(opts, &["a", "b"], custom_registry,);
    assert!(histogram_vec.is_ok());

    let histogram_vec = register_histogram_vec_with_registry!(
        "test_macro_histogram_vec_2",
        "help",
        &["a", "b"],
        custom_registry,
    );
    assert!(histogram_vec.is_ok());

    let histogram_vec = register_histogram_vec_with_registry!(
        "test_macro_histogram_vec_3",
        "help",
        &["test_label"],
        vec![0.0, 1.0, 2.0],
        custom_registry,
    );
    assert!(histogram_vec.is_ok());
}
