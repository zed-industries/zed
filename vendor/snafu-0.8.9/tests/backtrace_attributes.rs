use snafu::{prelude::*, Backtrace};

#[derive(Debug, Snafu)]
enum Error {
    NoArgument {
        #[snafu(backtrace)]
        thing: Backtrace,
    },

    ExplicitTrue {
        #[snafu(backtrace(true))]
        thing: Backtrace,
    },

    ExplicitFalse {
        #[snafu(backtrace(false))]
        backtrace: i32,
    },
}

fn example() -> Result<(), Error> {
    NoArgumentSnafu.fail()?;
    ExplicitTrueSnafu.fail()?;
    ExplicitFalseSnafu { backtrace: 42 }.fail()?;
    Ok(())
}

#[test]
fn implements_error() {
    fn check<T: std::error::Error>() {}
    check::<Error>();
    example().unwrap_err();
}
