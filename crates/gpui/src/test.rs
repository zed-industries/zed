use std::{
    fmt::Write,
    panic::{self, RefUnwindSafe},
    rc::Rc,
    sync::{
        atomic::{AtomicU64, Ordering::SeqCst},
        Arc,
    },
};

use futures::StreamExt;
use parking_lot::Mutex;
use smol::channel;

use crate::{
    app::ref_counts::LeakDetector,
    elements::Empty,
    executor::{self, ExecutorEvent},
    platform,
    platform::Platform,
    util::CwdBacktrace,
    AnyElement, AppContext, Element, Entity, FontCache, Handle, Subscription, TestAppContext, View,
    ViewContext,
};

#[cfg(test)]
#[ctor::ctor]
fn init_logger() {
    if std::env::var("RUST_LOG").is_ok() {
        env_logger::init();
    }
}

// #[global_allocator]
// static ALLOC: dhat::Alloc = dhat::Alloc;

pub fn run_test(
    mut num_iterations: u64,
    mut starting_seed: u64,
    max_retries: usize,
    detect_nondeterminism: bool,
    test_fn: &mut (dyn RefUnwindSafe
              + Fn(
        &mut AppContext,
        Rc<platform::test::ForegroundPlatform>,
        Arc<executor::Deterministic>,
        u64,
    )),
    on_fail_fn: Option<fn()>,
    fn_name: String,
) {
    // let _profiler = dhat::Profiler::new_heap();

    let is_randomized = num_iterations > 1;
    if is_randomized {
        if let Ok(value) = std::env::var("SEED") {
            starting_seed = value.parse().expect("invalid SEED variable");
        }
        if let Ok(value) = std::env::var("ITERATIONS") {
            num_iterations = value.parse().expect("invalid ITERATIONS variable");
        }
    }

    let atomic_seed = AtomicU64::new(starting_seed as u64);
    let mut retries = 0;

    loop {
        let result = panic::catch_unwind(|| {
            let foreground_platform = Rc::new(platform::test::foreground_platform());
            let platform = Arc::new(platform::test::platform());
            let font_system = platform.fonts();
            let font_cache = Arc::new(FontCache::new(font_system));
            let mut prev_runnable_history: Option<Vec<ExecutorEvent>> = None;

            for _ in 0..num_iterations {
                let seed = atomic_seed.load(SeqCst);

                if is_randomized {
                    eprintln!("seed = {seed}");
                }

                let deterministic = executor::Deterministic::new(seed);
                if detect_nondeterminism {
                    deterministic.set_previous_execution_history(prev_runnable_history.clone());
                    deterministic.enable_runnable_backtrace();
                }

                let leak_detector = Arc::new(Mutex::new(LeakDetector::default()));
                let mut cx = TestAppContext::new(
                    foreground_platform.clone(),
                    platform.clone(),
                    deterministic.build_foreground(usize::MAX),
                    deterministic.build_background(),
                    font_cache.clone(),
                    leak_detector.clone(),
                    0,
                    fn_name.clone(),
                );
                cx.update(|cx| {
                    test_fn(cx, foreground_platform.clone(), deterministic.clone(), seed);
                });

                cx.remove_all_windows();
                deterministic.run_until_parked();
                cx.update(|cx| cx.clear_globals());

                leak_detector.lock().detect();

                if detect_nondeterminism {
                    let curr_runnable_history = deterministic.execution_history();
                    if let Some(prev_runnable_history) = prev_runnable_history {
                        let mut prev_entries = prev_runnable_history.iter().fuse();
                        let mut curr_entries = curr_runnable_history.iter().fuse();

                        let mut nondeterministic = false;
                        let mut common_history_prefix = Vec::new();
                        let mut prev_history_suffix = Vec::new();
                        let mut curr_history_suffix = Vec::new();
                        loop {
                            match (prev_entries.next(), curr_entries.next()) {
                                (None, None) => break,
                                (None, Some(curr_id)) => curr_history_suffix.push(*curr_id),
                                (Some(prev_id), None) => prev_history_suffix.push(*prev_id),
                                (Some(prev_id), Some(curr_id)) => {
                                    if nondeterministic {
                                        prev_history_suffix.push(*prev_id);
                                        curr_history_suffix.push(*curr_id);
                                    } else if prev_id == curr_id {
                                        common_history_prefix.push(*curr_id);
                                    } else {
                                        nondeterministic = true;
                                        prev_history_suffix.push(*prev_id);
                                        curr_history_suffix.push(*curr_id);
                                    }
                                }
                            }
                        }

                        if nondeterministic {
                            let mut error = String::new();
                            writeln!(&mut error, "Common prefix: {:?}", common_history_prefix)
                                .unwrap();
                            writeln!(&mut error, "Previous suffix: {:?}", prev_history_suffix)
                                .unwrap();
                            writeln!(&mut error, "Current suffix: {:?}", curr_history_suffix)
                                .unwrap();

                            let last_common_backtrace = common_history_prefix
                                .last()
                                .map(|event| deterministic.runnable_backtrace(event.id()));

                            writeln!(
                                &mut error,
                                "Last future that ran on both executions: {:?}",
                                last_common_backtrace.as_ref().map(CwdBacktrace)
                            )
                            .unwrap();
                            panic!("Detected non-determinism.\n{}", error);
                        }
                    }
                    prev_runnable_history = Some(curr_runnable_history);
                }

                if !detect_nondeterminism {
                    atomic_seed.fetch_add(1, SeqCst);
                }
            }
        });

        match result {
            Ok(_) => {
                break;
            }
            Err(error) => {
                if retries < max_retries {
                    retries += 1;
                    println!("retrying: attempt {}", retries);
                } else {
                    if is_randomized {
                        eprintln!("failing seed: {}", atomic_seed.load(SeqCst));
                    }
                    on_fail_fn.map(|f| f());
                    panic::resume_unwind(error);
                }
            }
        }
    }
}

pub struct Observation<T> {
    rx: channel::Receiver<T>,
    _subscription: Subscription,
}

impl<T> futures::Stream for Observation<T> {
    type Item = T;

    fn poll_next(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Self::Item>> {
        self.rx.poll_next_unpin(cx)
    }
}

pub fn observe<T: Entity>(entity: &impl Handle<T>, cx: &mut TestAppContext) -> Observation<()> {
    let (tx, rx) = smol::channel::unbounded();
    let _subscription = cx.update(|cx| {
        cx.observe(entity, move |_, _| {
            let _ = smol::block_on(tx.send(()));
        })
    });

    Observation { rx, _subscription }
}

pub fn subscribe<T: Entity>(
    entity: &impl Handle<T>,
    cx: &mut TestAppContext,
) -> Observation<T::Event>
where
    T::Event: Clone,
{
    let (tx, rx) = smol::channel::unbounded();
    let _subscription = cx.update(|cx| {
        cx.subscribe(entity, move |_, event, _| {
            let _ = smol::block_on(tx.send(event.clone()));
        })
    });

    Observation { rx, _subscription }
}

pub struct EmptyView;

impl Entity for EmptyView {
    type Event = ();
}

impl View for EmptyView {
    fn ui_name() -> &'static str {
        "empty view"
    }

    fn render(&mut self, _: &mut ViewContext<Self>) -> AnyElement<Self> {
        Empty::new().into_any()
    }
}
