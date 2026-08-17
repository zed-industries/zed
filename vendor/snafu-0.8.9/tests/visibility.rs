// There are also sad-path tests

mod outer {
    pub mod inner {
        use snafu::prelude::*;

        #[derive(Debug, Snafu)]
        #[snafu(visibility(pub(crate)))]
        pub(crate) enum Error {
            PubCrate {
                id: i32,
            },

            #[snafu(visibility(pub(in crate::outer)))]
            PubInPath {
                id: i32,
            },

            #[snafu(visibility)]
            _Private {
                id: i32,
            },
        }
    }

    #[test]
    fn can_set_default_visibility() {
        let _ = self::inner::PubCrateSnafu { id: 42 }.build();
    }

    #[test]
    fn can_set_visibility() {
        let _ = self::inner::PubInPathSnafu { id: 42 }.build();
    }
}

#[test]
fn can_set_default_visibility() {
    let _ = self::outer::inner::PubCrateSnafu { id: 42 }.build();
}
