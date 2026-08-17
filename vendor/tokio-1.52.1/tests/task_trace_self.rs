#![allow(unknown_lints, unexpected_cfgs)]
#![cfg(all(
    tokio_unstable,
    feature = "taskdump",
    target_os = "linux",
    any(target_arch = "aarch64", target_arch = "x86", target_arch = "x86_64")
))]

use std::future::Future;
use std::pin::Pin;
use std::ptr;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll};
use std::time::{Duration, Instant};

use tokio::runtime::dump::{trace_with, Root, Trace, TraceMeta};

pin_project_lite::pin_project! {
    pub struct PrettyFuture<F: Future> {
        #[pin]
        f: Root<F>,
        t_last: State,
        logs: Arc<Mutex<Vec<Trace>>>,
    }
}

enum State {
    NotStarted,
    Running { since: Instant },
    Alerted,
}

impl<F: Future> PrettyFuture<F> {
    pub fn pretty(f: F, logs: Arc<Mutex<Vec<Trace>>>) -> Self {
        PrettyFuture {
            f: Trace::root(f),
            t_last: State::NotStarted,
            logs,
        }
    }
}

impl<F: Future> Future for PrettyFuture<F> {
    type Output = F::Output;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<F::Output> {
        let mut this = self.project();
        let now = Instant::now();
        let t_last = match this.t_last {
            State::Running { since } => Some(*since),
            State::NotStarted => {
                *this.t_last = State::Running { since: now };
                None
            }
            State::Alerted => {
                // don't double-alert for the same future
                None
            }
        };
        if t_last.is_some_and(|t_last| now.duration_since(t_last) > Duration::from_millis(500)) {
            let (res, trace) = tokio::runtime::dump::Trace::capture(|| this.f.as_mut().poll(cx));
            this.logs.lock().unwrap().push(trace);
            *this.t_last = State::Alerted;
            return res;
        }
        this.f.poll(cx)
    }
}

#[tokio::test]
async fn task_trace_self() {
    let log = Arc::new(Mutex::new(vec![]));
    let log2 = Arc::new(Mutex::new(vec![]));
    let mut good_line = vec![];
    let mut bad_line = vec![];
    PrettyFuture::pretty(
        PrettyFuture::pretty(
            async {
                bad_line.push(line!() + 1);
                tokio::task::yield_now().await;
                bad_line.push(line!() + 1);
                tokio::time::sleep(Duration::from_millis(1)).await;
                for _ in 0..100 {
                    good_line.push(line!() + 1);
                    tokio::time::sleep(Duration::from_millis(10)).await;
                }
            },
            log.clone(),
        ),
        log2.clone(),
    )
    .await;
    for line in good_line {
        let s = format!("{}:{}:", file!(), line);
        assert!(log.lock().unwrap().iter().any(|x| {
            eprintln!("{x}");
            format!("{x}").contains(&s)
        }));
    }
    for line in bad_line {
        let s = format!("{}:{}:", file!(), line);
        assert!(!log
            .lock()
            .unwrap()
            .iter()
            .any(|x| format!("{x}").contains(&s)));
    }
}

/// Collect frames between `trace_leaf_for_test` and `root_addr` using
/// `backtrace::trace`, resolve them, and store pretty-printed symbol names
/// (with compiler hashes stripped) into `logs`.
#[inline(never)]
fn trace_leaf_for_test(meta: &TraceMeta, log: &mut Vec<Vec<String>>) {
    let mut frames: Vec<backtrace::BacktraceFrame> = vec![];
    let mut above_leaf = false;

    if let Some(root_addr) = meta.root_addr {
        backtrace::trace(|frame| {
            let below_root = !ptr::eq(frame.symbol_address(), root_addr);

            if above_leaf && below_root {
                frames.push(frame.to_owned().into());
            }

            if ptr::eq(frame.symbol_address(), meta.trace_leaf_addr) {
                above_leaf = true;
            }

            below_root
        });
    }

    // Resolve frames into human-readable symbol names with hashes stripped.
    let mut bt = backtrace::Backtrace::from(frames);
    bt.resolve();
    let mut names = vec![];
    for frame in bt.frames() {
        for symbol in frame.symbols() {
            if let Some(name) = symbol.name() {
                names.push(strip_symbol_hash(&format!("{name}")).to_owned());
            }
        }
    }

    log.push(names);
}

/// Strip the trailing `::h<hex>` hash that rustc appends to symbol names.
fn strip_symbol_hash(s: &str) -> &str {
    // Symbols end with "::h" followed by hex digits. Find the last "::h".
    if let Some(pos) = s.rfind("::h") {
        let suffix = &s[pos + 3..];
        if !suffix.is_empty() && suffix.chars().all(|c| c.is_ascii_hexdigit()) {
            return &s[..pos];
        }
    }
    s
}

pin_project_lite::pin_project! {
    /// A future wrapper that uses `trace_with` to capture a backtrace on every
    /// poll.
    /// The captured backtraces are stored in `logs`.
    pub struct TaskDump<F: Future> {
        #[pin]
        f: Root<F>,
        logs: Arc<Mutex<Vec<Vec<String>>>>,
    }
}

impl<F: Future> TaskDump<F> {
    pub fn new(f: F, logs: Arc<Mutex<Vec<Vec<String>>>>) -> Self {
        TaskDump {
            f: Trace::root(f),
            logs,
        }
    }
}

impl<F: Future> Future for TaskDump<F> {
    type Output = F::Output;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<F::Output> {
        let mut this = self.project();

        // Poll the future with a real waker. if it returns Ready, exit immediately
        match this.f.as_mut().poll(cx) {
            Poll::Ready(result) => return Poll::Ready(result),
            Poll::Pending => {}
        };

        let mut logs = Vec::new();

        // Tracing poll with a noop waker. If the future is at a yield
        // point, trace_leaf fires our callback and returns Pending. We discard
        // the result — this poll is purely for capturing the backtrace.
        let noop = futures::task::noop_waker();
        let mut noop_cx = Context::from_waker(&noop);
        let trace_poll = trace_with(
            || this.f.as_mut().poll(&mut noop_cx),
            |meta| trace_leaf_for_test(meta, &mut logs),
        );
        // trace should always produce poll pending
        assert!(
            matches!(trace_poll, Poll::Pending),
            "expected trace to produce Poll::Pending but it was ready"
        );

        // Drain any frames captured by trace_leaf_for_test into our log.
        this.logs.lock().unwrap().extend(logs);
        Poll::Pending
    }
}

#[inline(never)]
async fn inner_yield_point() {
    tokio::task::yield_now().await;
}

/// Validates that `trace_with` (via the `TaskDump` wrapper):
/// 1. Invokes the trace-leaf callback when the wrapped future is at a yield point.
/// 2. Produces a backtrace (via `backtrace::trace`) that contains the expected
///    intermediate symbols between the root and the leaf.
/// 3. Does not produce spurious callbacks when the future returns `Ready`.
#[tokio::test]
async fn trace_with_callback_and_backtrace() {
    let logs: Arc<Mutex<Vec<Vec<String>>>> = Arc::new(Mutex::new(vec![]));

    let result = TaskDump::new(
        async {
            inner_yield_point().await;
            42
        },
        logs.clone(),
    )
    .await;

    assert_eq!(result, 42);

    let captured = logs.lock().unwrap();

    assert_eq!(
        captured.len(),
        1,
        "expected exactly 1 traces, got {:#?}",
        *captured
    );

    // These symbols should appear in the trace in this exact order (substring match).
    // trace_leaf itself should NOT appear — it's the boundary frame.
    let expected_in_order = [
        "tokio::task::yield_now::yield_now",
        "core::future::poll_fn::PollFn",
        "tokio::task::yield_now::yield_now",
        "task_trace_self::inner_yield_point",
        "task_trace_self::trace_with_callback_and_backtrace",
    ];
    let trace = &captured[0];

    assert_eq!(
        expected_in_order.len(),
        trace.len(),
        "expected {} frames but got {}:\n{trace:#?}",
        expected_in_order.len(),
        trace.len()
    );

    for (expected, actual) in expected_in_order.iter().zip(trace.iter()) {
        assert!(
            actual.contains(expected),
            "expected frame containing {expected:?}, got {actual:?}\nfull trace:\n{trace:#?}"
        );
    }
}
