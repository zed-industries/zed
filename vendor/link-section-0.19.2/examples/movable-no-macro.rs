//! Reference-section example for `link-section`.
#![warn(missing_docs)]

use link_section::section;

/// Operations.
#[section(movable, unsafe, name = OPERATIONS)]
static OPERATIONS: link_section::TypedMovableSection<Operation>;

#[derive(Debug, PartialEq, Eq, Ord, PartialOrd)]
struct Operation(u32);

pub(crate) mod operations {
    use super::Operation;
    use link_section::in_section;

    #[in_section(unsafe, name = OPERATIONS, type = movable)]
    pub static OPERATION_A: Operation = Operation(1);

    #[in_section(unsafe, name = OPERATIONS, type = movable)]
    pub static OPERATION_B: Operation = Operation(3);

    #[in_section(unsafe, name = OPERATIONS, type = movable)]
    pub static OPERATION_C: Operation = Operation(6);

    #[in_section(unsafe, name = OPERATIONS, type = movable)]
    pub static OPERATION_D: Operation = Operation(10);

    #[in_section(unsafe, name = OPERATIONS, type = movable)]
    pub static OPERATION_E: Operation = Operation(8);

    #[in_section(unsafe, name = OPERATIONS, type = movable)]
    pub static OPERATION_F: Operation = Operation(2);

    #[in_section(unsafe, name = OPERATIONS, type = movable)]
    pub static OPERATION_G: Operation = Operation(4);

    #[in_section(unsafe, name = OPERATIONS, type = movable)]
    pub static OPERATION_H: Operation = Operation(5);

    #[in_section(unsafe, name = OPERATIONS, type = movable)]
    pub static OPERATION_I: Operation = Operation(9);

    #[in_section(unsafe, name = OPERATIONS, type = movable)]
    pub static OPERATION_J: Operation = Operation(7);
}

#[allow(unsafe_code)]
fn sort_operations() {
    unsafe { OPERATIONS.sort_unstable() };
}

fn main() {
    // This should normally be done in a `ctor`, but for this example we know
    // there are no other live threads and we do it here.
    sort_operations();

    for op in OPERATIONS {
        println!("Operation: {op:?}");
    }

    println!("OPERATION_A: {:?}", *operations::OPERATION_A);
    println!("OPERATION_B: {:?}", *operations::OPERATION_B);
    println!("OPERATION_C: {:?}", *operations::OPERATION_C);
    println!("OPERATION_D: {:?}", *operations::OPERATION_D);
    println!("OPERATION_E: {:?}", *operations::OPERATION_E);
    println!("OPERATION_F: {:?}", *operations::OPERATION_F);
    println!("OPERATION_G: {:?}", *operations::OPERATION_G);
    println!("OPERATION_H: {:?}", *operations::OPERATION_H);
    println!("OPERATION_I: {:?}", *operations::OPERATION_I);
    println!("OPERATION_J: {:?}", *operations::OPERATION_J);
}
