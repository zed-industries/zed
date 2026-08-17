extern crate peg;

const FOO: i32 = 42;

peg::parser!(grammar test_grammar() for str {
    use super::FOO as F1;
    use super::{FOO as F2};
    pub rule renamed_imports() -> (i32, i32) = { (F1, F2) }
});

use self::test_grammar::*;

fn main() {
    assert_eq!(renamed_imports("").unwrap(), (42, 42));
}