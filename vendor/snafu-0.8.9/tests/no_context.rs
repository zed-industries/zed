use snafu::prelude::*;

#[derive(Debug, Snafu)]
enum AlphaError {
    _AlphaDummy,
}

#[derive(Debug, Snafu)]
enum BetaError {
    _BetaDummy,
}

#[derive(Debug, Snafu)]
enum Error {
    #[snafu(context(false))]
    Alpha { source: AlphaError },

    #[snafu(context(false))]
    Beta { source: BetaError },
}

fn alpha() -> Result<i32, AlphaError> {
    Ok(1)
}

fn beta() -> Result<i32, BetaError> {
    Ok(2)
}

fn example() -> Result<i32, Error> {
    let a = alpha()?;
    let b = beta()?;
    Ok(a * 10 + b)
}

fn check<T: std::error::Error>() {}

#[test]
fn implements_error() {
    check::<Error>();

    assert_eq!(12, example().unwrap());
}

mod with_backtraces {
    use super::*;
    use snafu::Backtrace;

    #[derive(Debug, Snafu)]
    enum Error {
        #[snafu(context(false))]
        Alpha {
            source: AlphaError,
            backtrace: Backtrace,
        },
    }

    #[test]
    fn implements_error() {
        check::<Error>();
    }
}

mod with_bounds {
    use super::*;
    use std::fmt::{Debug, Display};

    #[derive(Debug, Snafu)]
    enum GenericError<T, U = i32> {
        _Something { things: T, other: U },
    }

    #[derive(Debug, Snafu)]
    enum Error<T: 'static>
    where
        T: Debug + Display + Copy,
    {
        #[snafu(context(false))]
        Generic { source: GenericError<T> },
    }

    #[test]
    fn implements_error() {
        check::<Error<i32>>();
    }
}
