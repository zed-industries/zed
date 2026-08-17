extern crate psm;

psm::psm_stack_manipulation! {
    yes {
        use std::alloc;

        const STACK_ALIGN: usize = 4096;
        const FRAME_SIZE: usize = 4096;
        const FIB_COUNTS: [(usize, u64); 3] = [
            (8, 21),
            (16, 987),
            (24, 46368),
        ];

        #[inline(never)]
        fn fib(n: usize) -> u64 {
            unsafe {
                let layout = alloc::Layout::from_size_align(FRAME_SIZE, STACK_ALIGN).unwrap();
                let new_stack = alloc::alloc(layout);
                assert!(!new_stack.is_null(), "allocations must succeed!");
                let r = match n {
                    0 => 0,
                    1 => 1,
                    _ => {
                        psm::on_stack(new_stack, FRAME_SIZE, || {
                            fib(n - 1) + fib(n - 2)
                        })
                    }
                };
                alloc::dealloc(new_stack, layout);
                r
            }
        }

        fn main() {
            for &(n, expected) in FIB_COUNTS.iter() {
                let res = fib(n);
                assert_eq!(res, expected);
                println!("fib({}) = {}", n, res);
            }
        }
    }
    no {
        fn main() {
            eprintln!("Stack manipulation not supported by this target");
        }
    }
}

#[test]
fn run_example() {
    main()
}
