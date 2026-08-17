use link_section::{section, in_section, TypedSection};

#[section(typed, unsafe, name = my_crate::FOO)]
static FOO: TypedSection<fn()>;

#[in_section(unsafe, name = my_crate::FOO, type = typed)]
fn foo() {
    println!("foo");
}

#[section(typed, unsafe, name = package::BAR, aux(main = my_crate::FOO))]
static BAR: TypedSection<fn()>;

#[in_section(unsafe, type = typed, name = package::BAR, aux(main = my_crate::FOO))]
fn foo() {
    println!("foo");
}

fn main() {
}
