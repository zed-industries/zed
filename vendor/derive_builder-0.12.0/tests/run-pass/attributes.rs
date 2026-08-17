#[macro_use]
extern crate derive_builder;

/// This is a doc comment for the struct
#[warn(missing_docs)]
#[allow(non_snake_case, dead_code)]
#[derive(Builder)]
struct Lorem {
    /// This is a doc comment for a field
    field_with_doc_comment: String,
    #[allow(missing_docs)]
    undocumented: String,
    #[allow(non_snake_case)]
    CamelCase: i32,
    #[cfg(target_os = "macos")]
    mac_only: bool,
    #[allow(non_snake_case)]
    #[cfg(target_os = "linux")]
    LinuxOnly: (),
}

fn main() { }
