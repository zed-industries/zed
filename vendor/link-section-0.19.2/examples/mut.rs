//! Reference-section example for `link-section`.
#![warn(missing_docs)]

use link_section::section;

/// Operations.
#[section(mutable)]
pub static OPERATIONS: link_section::TypedMutableSection<&'static str>;

mod operations {
    use link_section::in_section;

    #[in_section(super::OPERATIONS)]
    const OPERATION_1: &'static str = "operation_1";

    #[in_section(super::OPERATIONS)]
    const OPERATION_2: &'static str = "operation_2";

    #[in_section(super::OPERATIONS)]
    const OPERATION_3: &'static str = "operation_3";
}

#[allow(unsafe_code)]
fn main() {
    // This should normally be done in a `ctor`, but for this example we know
    // there are no other live threads and we do it here.
    {
        let ops = unsafe { OPERATIONS.as_mut_slice() };
        ops.sort_unstable();
    }

    for op in OPERATIONS {
        println!("Operation: {}", op);
    }
}
