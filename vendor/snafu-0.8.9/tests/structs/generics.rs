mod lifetimes {
    use snafu::prelude::*;

    #[derive(Debug, Snafu)]
    struct Error<'a> {
        key: &'a i32,
    }

    #[test]
    fn are_allowed() {
        let key = 42;
        let e = Snafu { key: &key }.build();
        assert_eq!(*e.key, key);
    }
}

mod types {
    use snafu::prelude::*;

    #[derive(Debug, Snafu)]
    struct Error<T> {
        key: T,
    }

    #[test]
    fn are_allowed() {
        let key = 42;
        let e: Error<i32> = Snafu { key }.build();
        assert_eq!(e.key, key);
    }

    mod with_defaults {
        use snafu::{prelude::*, AsErrorSource};
        use std::{error::Error as StdError, fmt::Debug, io};

        #[derive(Debug, Snafu)]
        struct Error<S = io::Error, T = String>
        where
            S: StdError + AsErrorSource,
            T: Debug,
        {
            source: S,
            key: T,
        }

        #[test]
        fn allows_non_default_types() {
            #[derive(Debug, Snafu)]
            struct AnotherError;

            let r = AnotherSnafu.fail::<()>();
            let _e: Error<_, u8> = r.context(Snafu { key: 42 }).unwrap_err();
        }
    }
}

mod bounds {
    mod inline {
        use snafu::prelude::*;
        use std::fmt::Display;

        #[derive(Debug, Snafu)]
        #[snafu(display("key: {key}"))]
        struct Error<T: Display> {
            key: T,
        }

        #[test]
        fn are_preserved() {
            let e: Error<bool> = Snafu { key: true }.build();
            let display = e.to_string();
            assert_eq!(display, "key: true");
        }
    }

    mod where_clause {
        use snafu::prelude::*;
        use std::fmt::Display;

        #[derive(Debug, Snafu)]
        #[snafu(display("key: {key}"))]
        struct Error<T>
        where
            T: Display,
        {
            key: T,
        }

        #[test]
        fn are_preserved() {
            let e: Error<bool> = Snafu { key: true }.build();
            let display = e.to_string();
            assert_eq!(display, "key: true");
        }
    }
}
