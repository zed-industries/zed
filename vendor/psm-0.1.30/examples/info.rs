extern crate psm;

psm::psm_stack_information! {
    yes {
        fn main() {
            println!("Stack is {:?} and is at {:p} currently",
                     psm::StackDirection::new(), psm::stack_pointer());
        }
    }
    no {
        fn main() {
            eprintln!("Stack information not supported by this target");
        }
    }
}

#[test]
fn run_example() {
    main();
}
