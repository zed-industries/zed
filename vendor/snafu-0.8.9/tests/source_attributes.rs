use snafu::prelude::*;

#[derive(Debug, Snafu)]
enum InnerError {
    _Boom,
}

fn inner() -> Result<(), InnerError> {
    Ok(())
}

mod enabling {
    use super::*;

    #[derive(Debug, Snafu)]
    enum Error {
        NoArgument {
            #[snafu(source)]
            cause: InnerError,
        },

        ExplicitTrue {
            #[snafu(source(true))]
            cause: InnerError,
        },

        FromImpliesTrue {
            #[snafu(source(from(InnerError, Box::new)))]
            cause: Box<InnerError>,
        },

        ExplicitFalse {
            #[snafu(source(false))]
            source: i32,
        },
    }

    fn example() -> Result<(), Error> {
        inner().context(NoArgumentSnafu)?;
        inner().context(ExplicitTrueSnafu)?;
        inner().context(FromImpliesTrueSnafu)?;
        ExplicitFalseSnafu { source: 42 }.fail()?;
        Ok(())
    }

    #[test]
    fn implements_error() {
        fn check<T: std::error::Error>() {}
        check::<Error>();
        example().unwrap_err();
    }
}

mod transformation {
    use super::*;
    use std::io;

    #[derive(Debug, Snafu)]
    enum Error {
        TransformationViaClosure {
            #[snafu(source(from(InnerError, |e| io::Error::new(io::ErrorKind::InvalidData, e))))]
            source: io::Error,
        },

        TransformationViaFunction {
            #[snafu(source(from(InnerError, into_io)))]
            source: io::Error,
        },

        TransformationToTraitObject {
            #[snafu(source(from(InnerError, Box::new)))]
            source: Box<dyn std::error::Error>,
        },
    }

    fn into_io(e: InnerError) -> io::Error {
        io::Error::new(io::ErrorKind::InvalidData, e)
    }

    fn example() -> Result<(), Error> {
        inner().context(TransformationViaClosureSnafu)?;
        inner().context(TransformationViaFunctionSnafu)?;
        inner().context(TransformationToTraitObjectSnafu)?;
        Ok(())
    }

    #[test]
    fn implements_error() {
        fn check<T: std::error::Error>() {}
        check::<Error>();
        example().unwrap();
    }

    #[derive(Debug, Snafu)]
    #[snafu(source(from(Error, Box::new)))]
    struct ApiError(Box<Error>);

    fn api_example() -> Result<(), ApiError> {
        example()?;
        Ok(())
    }

    #[test]
    fn api_implements_error() {
        fn check<T: std::error::Error>() {}
        check::<ApiError>();
        api_example().unwrap();
    }
}
