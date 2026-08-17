struct Foo {
    #[cfg(windows)]
    x: i32,
}

pub fn foo() {
    Foo {
        #[cfg(windows)]
        x: 0,
    };
}
