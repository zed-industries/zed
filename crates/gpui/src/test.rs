use std::{
    panic::{self, RefUnwindSafe},
    rc::Rc,
    sync::{
        atomic::{AtomicU64, Ordering::SeqCst},
        Arc,
    },
};

use crate::{executor, platform, FontCache, MutableAppContext, Platform, TestAppContext};

#[cfg(test)]
#[ctor::ctor]
fn init_logger() {
    env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .init();
}

pub fn run_sync_test(
    mut num_iterations: u64,
    mut starting_seed: u64,
    max_retries: usize,
    test_fn: &mut (dyn RefUnwindSafe + Fn(&mut MutableAppContext, u64)),
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
                let seed = atomic_seed.load(SeqCst);
                if seed >= starting_seed + num_iterations {
                    break;
                }

                if is_randomized {
                    dbg!(seed);
                }

                let (foreground, background) = executor::deterministic(seed);
                let mut cx = TestAppContext::new(
                    foreground_platform.clone(),
                    platform.clone(),
                    foreground.clone(),
                    background.clone(),
                    font_cache.clone(),
                    0,
                );
                cx.update(|cx| test_fn(cx, seed));

                atomic_seed.fetch_add(1, SeqCst);
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
                    panic::resume_unwind(error);
                }
            }
        }
    }
}
