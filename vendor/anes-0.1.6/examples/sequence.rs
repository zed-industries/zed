/// An example how to create custom ANSI sequences.
use anes::{csi, esc, sequence};

fn static_unit_struct() {
    sequence!(
        /// Documentation string is also supported.
        struct Foo => csi!("foo")
    );

    assert_eq!(&format!("{}", Foo), "\x1B[foo");
}

fn dynamic_struct() {
    sequence!(
        /// Documentation string is also supported.
        struct Foo(u16, u16) =>
        |this, f| write!(f, esc!("{};{}"), this.0, this.1)
    );

    assert_eq!(&format!("{}", Foo(5, 10)), "\x1B5;10");
}

fn static_enum() {
    sequence!(
        /// Documentation string is also supported.
        enum Foo {
            /// Documentation string is also supported.
            Bar => esc!("bar"),
            /// Documentation string is also supported.
            Baz => csi!("baz"),
        }
    );

    assert_eq!(&format!("{}", Foo::Bar), "\x1Bbar");
    assert_eq!(&format!("{}", Foo::Baz), "\x1B[baz");
}

fn main() {
    static_unit_struct();
    dynamic_struct();
    static_enum();
}
