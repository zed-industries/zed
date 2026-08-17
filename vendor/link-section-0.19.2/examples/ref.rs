//! Reference-section example for `link-section`.
#![warn(missing_docs)]

mod operations {
    use link_section::{in_section, section};

    /// Operations.
    #[section(typed)]
    pub static OPERATIONS: link_section::TypedSection<&'static str>;

    #[in_section(OPERATIONS)]
    static OPERATION_1: &'static str = "operation_1";

    #[in_section(OPERATIONS)]
    static OPERATION_2: &'static str = "operation_2";

    #[in_section(OPERATIONS)]
    static OPERATION_3: &'static str = "operation_3";
}

mod referenced_operations {
    use link_section::{in_section, section};

    #[section(reference)]
    pub static OPERATIONS: link_section::TypedReferenceSection<&'static str>;

    #[in_section(OPERATIONS)]
    pub static REF_OPERATION_1: &'static str = "ref_operation_1";
}

fn main() {
    for op in operations::OPERATIONS {
        println!("Operation: {}", op);
    }
    for op in referenced_operations::OPERATIONS {
        println!("Referenced Operation: {}", op);
    }
    println!("OPERATIONS: {:?}", operations::OPERATIONS);
    println!("REF_OPERATIONS: {:?}", referenced_operations::OPERATIONS);
    println!(
        "REF_OPERATION_1: {}",
        *referenced_operations::REF_OPERATION_1
    );
}
