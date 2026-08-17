#![feature(used_with_arg)]

struct Thing;

inventory::collect!(Thing);

inventory::submit! {
    #[used(linker)]
    Thing
}

fn main() {}
