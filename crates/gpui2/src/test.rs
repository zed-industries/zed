use crate::TestDispatcher;
use rand::prelude::*;
use std::{
    env,
    panic::{self, RefUnwindSafe},
};

pub fn run_test(
    mut num_iterations: u64,
    max_retries: usize,
    test_fn: &mut (dyn RefUnwindSafe + Fn(TestDispatcher, u64)),
    on_fail_fn: Option<fn()>,
    _fn_name: String, // todo!("re-enable fn_name")
) {
    let starting_seed = env::var("SEED")
        .map(|seed| seed.parse().expect("invalid SEED variable"))
        .unwrap_or(0);
    let is_randomized = num_iterations > 1;
    if let Ok(iterations) = env::var("ITERATIONS") {
        num_iterations = iterations.parse().expect("invalid ITERATIONS variable");
    }

    for seed in starting_seed..starting_seed + num_iterations {
        let mut retry = 0;
        loop {
            if is_randomized {
                eprintln!("seed = {seed}");
            }
            let result = panic::catch_unwind(|| {
                let dispatcher = TestDispatcher::new(StdRng::seed_from_u64(seed));
                test_fn(dispatcher, seed);
            });

            match result {
                Ok(_) => break,
                Err(error) => {
                    if retry < max_retries {
                        println!("retrying: attempt {}", retry);
                        retry += 1;
                    } else {
                        if is_randomized {
                            eprintln!("failing seed: {}", seed);
                        }
                        on_fail_fn.map(|f| f());
                        panic::resume_unwind(error);
                    }
                }
            }
        }
    }
}
