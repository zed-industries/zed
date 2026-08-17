use snafu::prelude::*;

#[derive(Debug, Snafu)]
enum Error {
    #[snafu(context(suffix(Moo)))]
    Alpha,

    TrimsWhenEndingInError,

    #[snafu(context(suffix(false)))]
    CanOptOutOfSuffix,

    #[snafu(context(name(ButThisOneIsBetter)))]
    ThisNameIsGood,
}

fn alpha_usage() -> Result<(), Error> {
    AlphaMoo.fail()
}

fn trimming_usage() -> Result<(), Error> {
    TrimsWhenEndingInSnafu.fail()
}

fn no_suffix_usage() -> Result<(), Error> {
    CanOptOutOfSuffix.fail()
}

fn directly_named_usage() -> Result<(), Error> {
    ButThisOneIsBetter.fail()
}

#[test]
fn implements_error() {
    fn check<T: std::error::Error>() {}
    check::<Error>();

    alpha_usage().unwrap_err();
    trimming_usage().unwrap_err();
    no_suffix_usage().unwrap_err();
    directly_named_usage().unwrap_err();
}

mod applied_to_enum {
    use super::*;

    #[derive(Debug, Snafu)]
    #[snafu(context(suffix(false)))]
    enum Error {
        Alpha,
        Beta,
        #[snafu(context(suffix(Cow)))]
        Gamma,
    }

    fn alpha_usage() -> Result<(), Error> {
        Alpha.fail()
    }

    fn beta_usage() -> Result<(), Error> {
        Beta.fail()
    }

    fn gamma_usage() -> Result<(), Error> {
        GammaCow.fail()
    }

    #[test]
    fn implements_error() {
        fn check<T: std::error::Error>() {}
        check::<Error>();

        alpha_usage().unwrap_err();
        beta_usage().unwrap_err();
        gamma_usage().unwrap_err();
    }
}
