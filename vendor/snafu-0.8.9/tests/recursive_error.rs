use snafu::prelude::*;

#[derive(Debug, Snafu)]
enum Error {
    Leaf {
        name: String,
    },

    BoxedSelf {
        #[snafu(source(from(Error, Box::new)))]
        source: Box<Error>,
    },

    BoxedPublic {
        #[snafu(source(from(ApiError, Box::new)))]
        source: Box<ApiError>,
    },
}

#[derive(Debug, Snafu)]
#[snafu(source(from(Error, Box::new)))]
struct ApiError(Box<Error>);

type Result<T, E = Error> = std::result::Result<T, E>;

fn lookup() -> Result<()> {
    LeafSnafu { name: "foo" }.fail()
}

fn add() -> Result<()> {
    lookup().context(BoxedSelfSnafu)
}

fn public() -> Result<(), ApiError> {
    add()?;
    Ok(())
}

fn re_private() -> Result<()> {
    public().context(BoxedPublicSnafu)
}

#[test]
fn implements_error() {
    fn check<T: std::error::Error>() {}
    check::<Error>();
    re_private().unwrap_err();
}
