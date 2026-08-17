use crate::{Config, DefaultConfig};

use super::TracyLayer;
use criterion::Criterion;
use futures::future::join_all;
use tracing::{debug, event, info, info_span, span, Level};
use tracing_attributes::instrument;
use tracing_subscriber::layer::SubscriberExt;

fn it_works() {
    let span = span!(Level::TRACE, "a sec");
    let _enter = span.enter();
    event!(Level::INFO, "EXPLOSION!");
}

fn it_works_2() {
    let span = span!(Level::TRACE, "2 secs");
    let _enter = span.enter();
    event!(
        Level::INFO,
        message = "DOUBLE THE EXPLOSION!",
        tracy.frame_mark = true
    );
}

fn multiple_entries() {
    let span = span!(Level::INFO, "multiple_entries");
    span.in_scope(|| {});
    span.in_scope(|| {});

    let span = span!(Level::INFO, "multiple_entries 2");
    span.in_scope(|| span.in_scope(|| {}));
}

fn out_of_order() {
    let span1 = span!(Level::INFO, "out of order exits 1");
    let span2 = span!(Level::INFO, "out of order exits 2");
    let span3 = span!(Level::INFO, "out of order exits 3");
    let entry1 = span1.enter();
    let entry2 = span2.enter();
    let entry3 = span3.enter();
    drop(entry2);
    drop(entry3);
    drop(entry1);
}

fn exit_in_different_thread() {
    let span = Box::new(span!(Level::INFO, "exit in different thread"));
    let entry = span.enter();
    std::thread::scope(|scope| {
        let thread = scope.spawn(move || drop(entry));
        thread.join().unwrap();
    });
}

async fn parent_task(subtasks: usize) {
    info!("spawning subtasks...");
    let subtasks = (1..=subtasks)
        .map(|number| {
            debug!(message = "creating subtask;", number);
            subtask(number)
        })
        .collect::<Vec<_>>();

    let result = join_all(subtasks).await;

    debug!("all subtasks completed");
    let sum: usize = result.into_iter().sum();
    info!(sum);
}

#[instrument]
async fn subtask(number: usize) -> usize {
    info!("sleeping in subtask {}...", number);
    tokio::time::sleep(std::time::Duration::from_millis(10)).await;
    info!("sleeping in subtask {}...", number);
    tokio::time::sleep(std::time::Duration::from_millis(number as _)).await;
    info!("sleeping in subtask {}...", number);
    tokio::time::sleep(std::time::Duration::from_millis(10)).await;
    number
}

// Test based on the spawny_things example from the tracing repository.
async fn async_futures() {
    parent_task(5).await;
}

fn message_too_long() {
    info!("{}", "a".repeat(u16::MAX.into()));
}

fn long_span_data() {
    let data = "c".repeat(u16::MAX.into());
    info_span!("some span name", "{}", data).in_scope(|| {});
}

fn span_with_fields() {
    let span = span!(
        Level::TRACE,
        "wait",
        duration = 0,
        reason = "testing fields"
    );
    let _enter = span.enter();
}

pub(crate) fn test() {
    tracing::subscriber::set_global_default(
        tracing_subscriber::registry().with(TracyLayer::default()),
    )
    .expect("setup the subscriber");
    it_works();
    it_works_2();
    multiple_entries();
    out_of_order();
    exit_in_different_thread();
    message_too_long();
    long_span_data();
    span_with_fields();
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("tokio runtime");
    runtime.block_on(async_futures());
}

#[derive(Default)]
struct CallstackConfig(DefaultConfig);
impl Config for CallstackConfig {
    type Formatter = <DefaultConfig as Config>::Formatter;
    fn formatter(&self) -> &Self::Formatter {
        self.0.formatter()
    }
    fn stack_depth(&self, _: &tracing_core::Metadata<'_>) -> u16 {
        100
    }
}

fn benchmark_span(c: &mut Criterion) {
    c.bench_function("span/callstack", |bencher| {
        let layer =
            tracing_subscriber::registry().with(TracyLayer::new(CallstackConfig::default()));
        tracing::subscriber::with_default(layer, || {
            bencher.iter(|| {
                let _span =
                    tracing::error_span!("message", field1 = "first", field2 = "second").entered();
            });
        });
    });

    c.bench_function("span/no_callstack", |bencher| {
        let layer = tracing_subscriber::registry().with(TracyLayer::default());
        tracing::subscriber::with_default(layer, || {
            bencher.iter(|| {
                let _span =
                    tracing::error_span!("message", field1 = "first", field2 = "second").entered();
            });
        });
    });
}

fn benchmark_message(c: &mut Criterion) {
    c.bench_function("event/callstack", |bencher| {
        let layer =
            tracing_subscriber::registry().with(TracyLayer::new(CallstackConfig::default()));
        tracing::subscriber::with_default(layer, || {
            bencher.iter(|| {
                tracing::error!(field1 = "first", field2 = "second", "message");
            });
        });
    });

    c.bench_function("event/no_callstack", |bencher| {
        let layer = tracing_subscriber::registry().with(TracyLayer::default());
        tracing::subscriber::with_default(layer, || {
            bencher.iter(|| {
                tracing::error!(field1 = "first", field2 = "second", "message");
            });
        });
    });
}

pub(crate) fn bench() {
    criterion::criterion_group!(benches, benchmark_span, benchmark_message);
    benches();
    Criterion::default().configure_from_args().final_summary();
}
