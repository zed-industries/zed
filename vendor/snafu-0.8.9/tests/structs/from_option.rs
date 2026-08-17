use snafu::{prelude::*, Backtrace};

#[derive(Debug, Snafu)]
#[snafu(display("The argument at index {idx} was missing"))]
struct Error {
    idx: usize,
    backtrace: Backtrace,
}

type Result<T, E = Error> = std::result::Result<T, E>;

#[test]
fn can_be_used_as_context_on_an_option() {
    fn example(values: &[i32], idx: usize) -> Result<i32> {
        values.get(idx).copied().context(Snafu { idx })
    }

    let actual = example(&[], 42);
    assert!(matches!(actual, Err(Error { idx: 42, .. })));
}
