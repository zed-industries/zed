extern crate psm;

psm::psm_stack_manipulation! {
    yes {
        use std::alloc;

        #[inline(never)]
        fn fib(n: usize, stack_limit: *mut u8) -> Option<u64> {
            // match psm::StackDirection::new() {
            //     psm::StackDirection::Ascending => if psm::stack_pointer() > stack_limit {
            //         return None;
            //     }
            //     psm::StackDirection::Descending => if psm::stack_pointer() < stack_limit {
            //         return None;
            //     }
            // }

            match n {
                0 => Some(0),
                1 => Some(1),
                _ => fib(n - 1, stack_limit).and_then(|x| fib(n - 2, stack_limit).map(|y| x + y)),
            }
        }

        const STACK_ALIGN: usize = 4096;
        const STACK_REDLINE: usize = 512;
        const FIB_COUNTS: [(usize, u64); 3] = [
            (8, 21),
            (16, 987),
            (32, 2178309),
        ];


        fn main() {
            let mut stack_size = 1024 * 128;
            unsafe {
                for &(n, expected) in FIB_COUNTS.iter() {
                    loop {
                        println!("fib({}) with {} bytes of stack", n, stack_size - STACK_REDLINE);
                        let layout = alloc::Layout::from_size_align(stack_size, STACK_ALIGN).unwrap();
                        let new_stack = alloc::alloc(layout);
                        assert!(!new_stack.is_null(), "allocations must succeed!");
                        let max_stack = match psm::StackDirection::new() {
                            psm::StackDirection::Ascending =>
                                new_stack.offset((stack_size - STACK_REDLINE) as isize),
                            psm::StackDirection::Descending =>
                                new_stack.offset(STACK_REDLINE as isize),
                        };
                        let result = psm::on_stack(new_stack, stack_size, || {
                            fib(n, max_stack)
                        });
                        alloc::dealloc(new_stack, layout);
                        if let Some(res) = result {
                            assert_eq!(res, expected);
                            println!("fib({}) = {}", n, res);
                            break;
                        } else {
                            println!("Stack not large enough!");
                            stack_size *= 2;
                        }
                    }
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
