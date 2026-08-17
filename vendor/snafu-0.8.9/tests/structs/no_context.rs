use snafu::prelude::*;

#[derive(Debug, Snafu)]
struct InnerError;

fn inner() -> Result<(), InnerError> {
    Ok(())
}

#[derive(Debug, Snafu)]
#[snafu(context(false))]
struct OuterError {
    source: InnerError,
}

#[test]
fn does_not_need_context_method() {
    fn exercise() -> Result<(), OuterError> {
        inner()?;
        Ok(())
    }

    let _ = exercise();
}

mod with_source_transformation {
    use super::*;

    #[derive(Debug, Snafu)]
    #[snafu(context(false))]
    struct OuterError {
        #[snafu(source(from(InnerError, Box::new)))]
        source: Box<InnerError>,
    }

    #[test]
    fn does_not_need_context_method() {
        fn exercise() -> Result<(), OuterError> {
            inner()?;
            Ok(())
        }

        let _ = exercise();
    }
}
