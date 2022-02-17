use std::{
    panic::{self, RefUnwindSafe},
    rc::Rc,
    sync::{
        atomic::{AtomicU64, Ordering::SeqCst},
        Arc,
    },
};

use futures::StreamExt;
use smol::channel;

use crate::{
    executor, platform, Entity, FontCache, Handle, MutableAppContext, Platform, Subscription,
    TestAppContext,
};

#[cfg(test)]
#[ctor::ctor]
fn init_logger() {
    if std::env::var("RUST_LOG").is_ok() {
        env_logger::init();
    }
}

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
) {
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
                let mut cx = TestAppContext::new(
                    foreground_platform.clone(),
                    platform.clone(),
                    deterministic.build_foreground(usize::MAX),
                    deterministic.build_background(),
                    font_cache.clone(),
                    0,
                );
                cx.update(|cx| {
                    test_fn(
                        cx,
                        foreground_platform.clone(),
                        deterministic,
                        seed,
                        is_last_iteration,
                    )
                });

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
