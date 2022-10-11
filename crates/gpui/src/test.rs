use crate::{
    elements::Empty, executor, platform, Element, ElementBox, Entity, FontCache, Handle,
    LeakDetector, MutableAppContext, Platform, RenderContext, Subscription, TestAppContext, View,
};
use futures::StreamExt;
use parking_lot::Mutex;
use smol::channel;
use std::{
    panic::{self, RefUnwindSafe},
    rc::Rc,
    sync::{
        atomic::{AtomicU64, Ordering::SeqCst},
        Arc,
    },
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
    test_fn: &mut (dyn RefUnwindSafe
              + Fn(
        &mut MutableAppContext,
        Rc<platform::test::ForegroundPlatform>,
        Arc<executor::Deterministic>,
        u64,
        bool,
    )),
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

            loop {
                let seed = atomic_seed.fetch_add(1, SeqCst);
                let is_last_iteration = seed + 1 >= starting_seed + num_iterations;

                if is_randomized {
                    dbg!(seed);
                }

                let deterministic = executor::Deterministic::new(seed);
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
                    test_fn(
                        cx,
                        foreground_platform.clone(),
                        deterministic.clone(),
                        seed,
                        is_last_iteration,
                    );
                });

                cx.update(|cx| cx.remove_all_windows());
                deterministic.run_until_parked();
                cx.update(|cx| cx.clear_globals());

                leak_detector.lock().detect();
                if is_last_iteration {
                    break;
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
                        eprintln!("failing seed: {}", atomic_seed.load(SeqCst) - 1);
                    }
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

    fn render(&mut self, _: &mut RenderContext<Self>) -> ElementBox {
        Element::boxed(Empty::new())
    }
}
