use link_section::declarative::{section, in_section};
use link_section::TypedSection;

section! {
    #[section(unsafe, type = typed)]
    static FOO: TypedSection<fn()>;
}

in_section! {
    #[in_section(unsafe, type = typed, name = FOO)]
    fn foo() {
        
    }
}

fn main() {
}
