use link_section::declarative::{section, in_section};
use link_section::TypedSection;

struct Driver {
    name: &'static str,
    f: fn(),
}

impl Driver {
    const fn new(name: &'static str, f: fn()) -> Self {
        Self { name, f }
    }
}

section! {
    #[section(unsafe, type = typed)]
    static FOO: TypedSection<Driver>;
}

in_section! {
    #[in_section(unsafe, type = typed, name = FOO)]
    const DRIVER: Driver = Driver::new("driver", || ());
}

fn main() {
}
