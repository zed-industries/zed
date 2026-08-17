mod default_with_lifetime {
    use snafu::{prelude::*, AsErrorSource};
    use std::fmt::{Debug, Display};

    #[derive(Debug, Snafu)]
    pub struct ApiError<'a, S, T>(Error<'a, S, T>)
    where
        T: Debug + Display,
        S: std::error::Error + AsErrorSource;

    #[derive(Debug, Snafu)]
    enum Error<'a, S = std::io::Error, T = String>
    where
        T: Display,
        S: std::error::Error + AsErrorSource,
    {
        #[snafu(display("Boom: {value}"))]
        _Boom {
            value: T,
            name: &'a str,
        },

        _WithSource {
            source: S,
        },

        _Empty,
    }

    #[test]
    fn implements_error() {
        fn check_bounds<T: std::error::Error>() {}
        check_bounds::<Error<std::io::Error>>();
        check_bounds::<ApiError<std::io::Error, i32>>();
    }
}
