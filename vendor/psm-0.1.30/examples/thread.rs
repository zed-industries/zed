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
            for (n, expected, handle) in FIB_COUNTS.iter().map(|&(n, expected)|
                (n, expected, std::thread::spawn(move || {
                    fib(n)
                }))
            ) {
                if let Ok(res) = handle.join() {
                    assert_eq!(res, expected);
                    println!("fib({}) = {}", n, res);
                } else {
                    panic!("joining a thread returned an Err");
                }
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
