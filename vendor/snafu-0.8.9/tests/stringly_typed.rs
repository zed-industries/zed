// TODO: ensure no-std still works
// THOUGHT: Source must always be an `Option`. This seems to force the `from()`

mod message_only {
    use snafu::prelude::*;

    #[derive(Debug, Snafu)]
    enum Error {
        #[snafu(whatever, display("{message}"))]
        Whatever { message: String },
    }

    // THOUGHT: Allow identify message field?

    type Result<T, E = Error> = std::result::Result<T, E>;

    #[test]
    fn implements_error() {
        fn check<T: std::error::Error>() {}
        check::<Error>();
    }

    #[test]
    fn can_use_a_formatted_string() {
        fn exercise(success: bool) -> Result<i32> {
            if !success {
                whatever!("This is a code {} error", 42);
            }

            Ok(1)
        }

        assert!(matches!(exercise(true), Ok(1)));
        let e = exercise(false).unwrap_err();
        assert_eq!("This is a code 42 error", e.to_string());
    }
}

// THOUGHT: Must it be boxed trait object?
// No, it *can* be a fixed type, but that's very limiting

mod message_and_source {
    use snafu::prelude::*;

    #[derive(Debug, Snafu)]
    struct UnderlyingError;

    fn underlying(success: bool) -> Result<i32, UnderlyingError> {
        ensure!(success, UnderlyingSnafu);
        Ok(1)
    }

    #[derive(Debug, Snafu)]
    enum Error {
        // THOUGHT: Should display automatically do message?
        #[snafu(whatever, display("{message}"))]
        Whatever {
            #[snafu(source(from(Box<dyn std::error::Error>, Some)))]
            source: Option<Box<dyn std::error::Error>>,
            message: String,
        },
    }

    type Result<T, E = Error> = std::result::Result<T, E>;

    #[test]
    fn implements_error() {
        fn check<T: std::error::Error>() {}
        check::<Error>();
    }

    #[test]
    fn can_use_a_formatted_string_via_macro() {
        fn exercise(success: bool) -> Result<i32> {
            let v = whatever!(underlying(success), "Something else happened {}", 42);
            Ok(v + 1)
        }

        assert!(matches!(exercise(true), Ok(2)));
        let e = exercise(false).unwrap_err();
        assert_eq!("Something else happened 42", e.to_string());
    }

    #[test]
    fn can_use_a_formatted_string_via_trait() {
        fn exercise(success: bool) -> Result<i32> {
            let v = underlying(success)
                .with_whatever_context(|_| format!("Something else happened {}", 42))?;
            Ok(v + 1)
        }

        assert!(matches!(exercise(true), Ok(2)));
        let e = exercise(false).unwrap_err();
        assert_eq!("Something else happened 42", e.to_string());
    }

    #[test]
    fn can_access_the_cause() {
        use std::error::Error as _;

        fn exercise(success: bool) -> Result<i32> {
            let v = whatever!(underlying(success), "Something else happened {}", 42);
            Ok(v + 1)
        }

        let e = exercise(false).unwrap_err();
        let src = e.source().expect("Must have a source");
        let src = src.downcast_ref::<UnderlyingError>();
        assert!(src.is_some());
    }

    #[test]
    fn does_not_need_a_cause() {
        use std::error::Error as _;

        fn exercise(success: bool) -> Result<i32> {
            if !success {
                whatever!("I caused a problem {}", 42);
            }
            Ok(1)
        }

        let e = exercise(false).unwrap_err();
        assert!(e.source().is_none());
    }

    #[test]
    fn can_be_recursive() {
        use std::error::Error as _;

        fn inner(success: bool) -> Result<i32> {
            if !success {
                whatever!("Inner error");
            }
            Ok(1)
        }

        fn outer(success: bool) -> Result<i32> {
            let v = whatever!(inner(success), "Outer error");
            Ok(v + 1)
        }

        assert!(matches!(outer(true), Ok(2)));
        let outer_error = outer(false).unwrap_err();
        let inner_error = outer_error.source().expect("Must have a source");
        assert!(inner_error.downcast_ref::<Error>().is_some());
        assert!(inner_error.source().is_none());
    }
}

mod message_source_and_backtrace {
    use snafu::{prelude::*, Backtrace, ErrorCompat};

    #[derive(Debug, Snafu)]
    struct UnderlyingError;

    fn underlying(success: bool) -> Result<i32, UnderlyingError> {
        ensure!(success, UnderlyingSnafu);
        Ok(1)
    }

    #[derive(Debug, Snafu)]
    enum Error {
        #[snafu(whatever, display("{}", message))]
        Whatever {
            #[snafu(source(from(Box<dyn std::error::Error>, Some)))]
            source: Option<Box<dyn std::error::Error>>,
            message: String,
            backtrace: Backtrace,
        },
    }

    type Result<T, E = Error> = std::result::Result<T, E>;

    #[test]
    fn implements_error() {
        fn check<T: std::error::Error>() {}
        check::<Error>();
    }

    #[test]
    fn can_use_a_formatted_string() {
        fn exercise(success: bool) -> Result<i32> {
            let v = whatever!(underlying(success), "Something else happened {}", 42);
            Ok(v + 1)
        }

        assert!(matches!(exercise(true), Ok(2)));
        let e = exercise(false).unwrap_err();
        assert_eq!("Something else happened 42", e.to_string());
    }

    #[test]
    fn has_a_backtrace() {
        use snafu::prelude::*;

        fn exercise(success: bool) -> Result<i32> {
            let v = whatever!(underlying(success), "Something else happened {}", 42);
            Ok(v + 1)
        }

        let e = exercise(false).unwrap_err();
        let bt = ErrorCompat::backtrace(&e).expect("Must have a backtrace");
        assert!(bt.to_string().contains("has_a_backtrace"));
    }
}

mod struck {
    mod message_source_and_backtrace {
        use snafu::{prelude::*, Backtrace};

        #[derive(Debug, Snafu)]
        #[snafu(whatever, display("{message}"))]
        struct Error {
            #[snafu(source(from(Box<dyn std::error::Error>, Some)))]
            source: Option<Box<dyn std::error::Error>>,
            message: String,
            backtrace: Backtrace,
        }

        type Result<T, E = Error> = std::result::Result<T, E>;

        #[test]
        fn implements_error() {
            fn check<T: std::error::Error>() {}
            check::<Error>();
        }

        #[test]
        fn can_use_a_formatted_string() {
            fn inner(success: bool) -> Result<i32> {
                if !success {
                    whatever!("inner went {}", "bang");
                }
                Ok(1)
            }

            fn outer(success: bool) -> Result<i32> {
                let v = whatever!(inner(success), "Something else happened {}", 42);
                Ok(v + 1)
            }

            assert!(matches!(outer(true), Ok(2)));
            let e = outer(false).unwrap_err();
            assert_eq!("Something else happened 42", e.to_string());
        }
    }
}

mod send_and_sync {
    use snafu::prelude::*;

    #[derive(Debug, Snafu)]
    enum Error {
        #[snafu(whatever, display("{message}"))]
        Whatever {
            #[snafu(source(from(Box<dyn std::error::Error + Send + Sync>, Some)))]
            source: Option<Box<dyn std::error::Error + Send + Sync>>,
            message: String,
        },
    }

    type Result<T, E = Error> = std::result::Result<T, E>;

    #[test]
    fn implements_error() {
        fn check<T: std::error::Error>() {}
        check::<Error>();
    }

    #[test]
    fn implements_send() {
        fn check<T: Send>() {}
        check::<Error>();
    }

    #[test]
    fn implements_sync() {
        fn check<T: Sync>() {}
        check::<Error>();
    }

    #[test]
    fn can_be_constructed() {
        fn inner() -> Result<()> {
            whatever!("The inner case")
        }

        fn outer() -> Result<()> {
            whatever!(inner(), "The outer case");
            Ok(())
        }

        assert!(outer().is_err());
    }
}
