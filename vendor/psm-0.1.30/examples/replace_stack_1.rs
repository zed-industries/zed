extern crate psm;

psm::psm_stack_manipulation! {
    yes {
        use std::alloc;

        const STACK_SIZE: usize = 4096 * 64;
        const STACK_ALIGN: usize = 4096;

        fn main() {
            println!("current stack pointer is {:p}", psm::stack_pointer());
            unsafe {
                let new_stack = alloc::alloc(alloc::Layout::from_size_align(STACK_SIZE, STACK_ALIGN).unwrap());
                println!("new stack is between {:p} and {:p}", new_stack, new_stack.offset(STACK_SIZE as isize));
                psm::replace_stack(new_stack, STACK_SIZE, || {
                    println!("after replacement stack pointer is {:p}", psm::stack_pointer());
                    ::std::process::exit(0);
                });
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
    // NOTE: intentionally out-of-processes, as the example exits with `process::exit(0)`.
    main()
}
