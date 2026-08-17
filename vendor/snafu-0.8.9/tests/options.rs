use snafu::{prelude::*, Backtrace};
use std::collections::HashMap;

#[derive(Debug, Snafu)]
enum Error {
    #[snafu(display("The left-hand argument {id} was missing"))]
    LeftHandMissing { id: i32, backtrace: Backtrace },
    #[snafu(display("The right-hand argument {id} was missing"))]
    RightHandMissing { id: i32, backtrace: Backtrace },
}

type Result<T, E = Error> = std::result::Result<T, E>;

fn example(values: &HashMap<i32, i32>, left: i32, right: i32) -> Result<i32> {
    let l = values
        .get(&left)
        .context(LeftHandMissingSnafu { id: left })?;
    let r = values
        .get(&right)
        .context(RightHandMissingSnafu { id: right })?;

    Ok(l + r)
}

#[test]
fn implements_error() {
    fn check<T: std::error::Error>() {}
    check::<Error>();
    example(&Default::default(), 1, 2).unwrap_err();
}
