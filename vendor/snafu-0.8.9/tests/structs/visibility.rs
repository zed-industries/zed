// There are also sad-path tests

pub mod inner {
    use snafu::prelude::*;

    #[derive(Debug, Snafu)]
    #[snafu(visibility(pub(crate)))]
    pub(crate) struct Error;
}

#[test]
fn can_set_visibility() {
    let _ = inner::Snafu.build();
}
