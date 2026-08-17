use snafu::{prelude::*, Backtrace};

type BoxError = Box<dyn std::error::Error>;

#[derive(Debug, Snafu)]
enum Error<'a, 'x, A, Y> {
    Everything {
        source: BoxError,
        name: &'a str,
        length: A,
        backtrace: Backtrace,
    },
    Lifetime {
        key: &'x i32,
    },
    Type {
        value: Y,
    },
}

fn cause_error() -> Result<(), BoxError> {
    Ok(())
}

fn example<'s, 'k, V>(name: &'s str, key: &'k i32, value: V) -> Result<(), Error<'s, 'k, usize, V>>
where
    V: std::fmt::Debug,
{
    let length = name.len();

    cause_error().context(EverythingSnafu { name, length })?;

    if name == "alice" {
        return LifetimeSnafu { key }.fail();
    }

    if name == "bob" {
        return TypeSnafu { value }.fail();
    }

    Ok(())
}

#[test]
fn implements_error() {
    let name = String::from("hello");
    let key = Box::new(42);
    let value = vec![false];

    example(&name, &key, value).unwrap();
}

mod bounds {
    mod inline {
        use snafu::prelude::*;
        use std::fmt::{Debug, Display};

        #[derive(Debug, Snafu)]
        pub struct ApiError<T: Debug + Display>(Error<T>);

        #[derive(Debug, Snafu)]
        enum Error<T: Display> {
            #[snafu(display("Boom: {value}"))]
            _Boom { value: T },

            #[snafu(whatever, display("{message}"))]
            Whatever {
                message: String,
                #[snafu(source(from(Box<dyn std::error::Error>, Some)))]
                source: Option<Box<dyn std::error::Error>>,
            },
        }

        #[test]
        fn implements_error() {
            fn check_bounds<T: std::error::Error>() {}
            check_bounds::<Error<i32>>();
            check_bounds::<ApiError<i32>>();
        }
    }

    mod where_clause {
        use snafu::prelude::*;
        use std::fmt::{Debug, Display};

        #[derive(Debug, Snafu)]
        pub struct ApiError<T>(Error<T>)
        where
            T: Debug + Display;

        #[derive(Debug, Snafu)]
        enum Error<T>
        where
            T: Display,
        {
            #[snafu(display("Boom: {value}"))]
            _Boom { value: T },

            #[snafu(whatever, display("{message}"))]
            Whatever {
                message: String,
                #[snafu(source(from(Box<dyn std::error::Error>, Some)))]
                source: Option<Box<dyn std::error::Error>>,
            },
        }

        #[test]
        fn implements_error() {
            fn check_bounds<T: std::error::Error>() {}
            check_bounds::<Error<i32>>();
            check_bounds::<ApiError<i32>>();
        }
    }
}

mod const_generics {
    use snafu::prelude::*;

    #[derive(Debug, Snafu)]
    #[snafu(display("Exceeded {N}"))]
    pub struct Error<const N: i32>;

    #[test]
    fn implements_error() {
        fn check_bounds<T: std::error::Error>() {}
        check_bounds::<Error<1>>();
        check_bounds::<Error<2>>();
    }

    #[test]
    fn can_be_constructed() {
        fn make_one() -> Result<(), Error<1>> {
            Snafu.fail()
        }

        fn make_two() -> Result<(), Error<2>> {
            Snafu.fail()
        }

        assert!(make_one().is_err());
        assert!(make_two().is_err());
    }

    #[test]
    fn can_use_const_in_display() {
        let e: Error<42> = Snafu.build();
        assert_eq!(e.to_string(), "Exceeded 42");
    }

    mod with_default {
        use snafu::prelude::*;

        #[derive(Debug, Snafu)]
        #[snafu(display("Exceeded {N}"))]
        pub struct Error<const N: i32 = 42>;

        #[test]
        fn implements_error() {
            fn check_bounds<T: std::error::Error>() {}
            check_bounds::<Error>();
            check_bounds::<Error<99>>();
        }

        #[test]
        fn can_be_constructed() {
            fn make_forty_two() -> Result<(), Error> {
                Snafu.fail()
            }

            fn make_ninety_nine() -> Result<(), Error<99>> {
                Snafu.fail()
            }

            assert!(make_forty_two().is_err());
            assert!(make_ninety_nine().is_err());
        }

        #[test]
        fn can_use_const_in_display() {
            let e: Error = Snafu.build();
            assert_eq!(e.to_string(), "Exceeded 42");
        }
    }
}
