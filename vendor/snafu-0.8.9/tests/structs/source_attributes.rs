use snafu::prelude::*;

mod enabling {
    use super::*;

    #[test]
    fn no_argument_treated_as_source() {
        #[derive(Debug, Snafu)]
        struct Error {
            #[snafu(source)]
            cause: InnerError,
        }

        let _ = inner().context(Snafu);
    }

    #[test]
    fn true_argument_treated_as_source() {
        #[derive(Debug, Snafu)]
        struct Error {
            #[snafu(source(true))]
            cause: InnerError,
        }

        let _ = inner().context(Snafu);
    }

    #[test]
    fn from_argument_treated_as_source() {
        #[derive(Debug, Snafu)]
        struct Error {
            #[snafu(source(from(InnerError, Box::new)))]
            cause: Box<InnerError>,
        }

        let _ = inner().context(Snafu);
    }

    #[test]
    fn false_argument_not_treated_as_source() {
        #[derive(Debug, Snafu)]
        struct Error {
            #[snafu(source(false))]
            source: i32,
        }

        let _ = Snafu { source: 42 }.build();
    }
}

mod transformation {
    use super::*;
    use std::{error::Error as StdError, io};

    #[test]
    fn transformation_via_closure() {
        #[derive(Debug, Snafu)]
        struct Error {
            #[snafu(source(from(InnerError, |e| io::Error::new(io::ErrorKind::InvalidData, e))))]
            source: io::Error,
        }

        let _ = inner().context(Snafu);
    }

    #[test]
    fn transformation_via_function() {
        fn into_io(e: InnerError) -> io::Error {
            io::Error::new(io::ErrorKind::InvalidData, e)
        }

        #[derive(Debug, Snafu)]
        struct Error {
            #[snafu(source(from(InnerError, into_io)))]
            source: io::Error,
        }

        let _ = inner().context(Snafu);
    }

    #[test]
    fn transformation_to_trait_object() {
        #[derive(Debug, Snafu)]
        struct Error {
            #[snafu(source(from(InnerError, Box::new)))]
            source: Box<dyn StdError>,
        }

        let _ = inner().context(Snafu);
    }
}

#[derive(Debug, Snafu)]
struct InnerError;

fn inner() -> Result<(), InnerError> {
    Ok(())
}
