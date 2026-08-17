use crate::prelude::*;
use core::panic::Location;

#[derive(Debug)]
#[allow(dead_code)]
struct LineCol {
    line: u32,
    col: u32,
}

impl From<&'_ Location<'_>> for LineCol {
    fn from(value: &'_ Location<'_>) -> Self {
        Self {
            line: value.line(),
            col: value.column(),
        }
    }
}

#[test]
fn track_caller_function() {
    #[builder]
    #[track_caller]
    fn dont_brick(_x: u32) -> LineCol {
        Location::caller().into()
    }

    let location = dont_brick().x(10).call();
    assert_debug_eq(location, expect!["LineCol { line: 28, col: 39 }"]);

    #[track_caller]
    #[builder]
    fn dont_brick_2(_x: u32) -> LineCol {
        Location::caller().into()
    }

    let location = dont_brick_2().x(10).call();
    assert_debug_eq(location, expect!["LineCol { line: 37, col: 41 }"]);
}

#[test]
fn track_caller_method() {
    struct Brick;
    struct Senti;
    #[bon]
    impl Senti {
        #[builder(finish_fn = yatta)]
        #[track_caller]
        fn new(brick: Brick) -> LineCol {
            let Brick = brick;
            Location::caller().into()
        }
    }
    let yatta = Senti::builder().brick(Brick).yatta();
    assert_debug_eq(yatta, expect!["LineCol { line: 54, col: 47 }"]);
}
