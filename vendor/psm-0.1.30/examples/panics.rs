extern crate psm;

use std::panic;

const CHAIN_DEPTH: usize = 16;

psm::psm_stack_manipulation! {
    yes {
        use std::alloc;
        const STACK_ALIGN: usize = 4096;
        // Generating backraces (because of RUST_BACKTRACE) create a few quite large frames, so it is
        // important, that all frames have sufficient amount of available memory to not run over the
        // stack...
        const FRAME_SIZE: usize = 4096 * 10;

        fn panic_chain(depth: usize) {
            if depth == 0 {
                panic!("full chain!");
            } else {
                unsafe {
                    let layout = alloc::Layout::from_size_align(FRAME_SIZE, STACK_ALIGN).unwrap();
                    let new_stack = alloc::alloc(layout);
                    assert!(!new_stack.is_null(), "allocations must succeed!");
                    let p = psm::on_stack(new_stack, FRAME_SIZE, || {
                        panic::catch_unwind(|| {
                            panic_chain(depth - 1);
                        })
                    });
                    alloc::dealloc(new_stack, layout);
                    p.map_err(panic::resume_unwind).unwrap()
                }
            }
        }

        fn main() {
            panic_chain(CHAIN_DEPTH);
        }

        #[test]
        fn run_example() {
            assert!(panic::catch_unwind(|| {
                panic_chain(CHAIN_DEPTH);
            }).is_err(), "Panic did not propagate!");
        }
    }

    no {
        fn main() {
            eprintln!("Stack manipulation not supported by this target");
        }
    }
}
