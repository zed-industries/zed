// This test asserts that errors can be used across threads.

use std::thread;

use snafu::prelude::*;

#[derive(Debug, Snafu)]
enum InnerError {
    Boom,
}

#[derive(Debug, Snafu)]
enum Error {
    _Leaf {
        name: String,
    },

    Wrapper {
        source: InnerError,
    },

    _BoxedWrapper {
        source: Box<InnerError>,
    },

    _BoxedTraitObjectSend {
        source: Box<dyn std::error::Error + Send + 'static>,
    },

    _BoxedTraitObjectSendSync {
        source: Box<dyn std::error::Error + Send + Sync + 'static>,
    },
}

fn example() -> Result<(), Error> {
    BoomSnafu.fail().context(WrapperSnafu)
}

#[test]
fn implements_thread_safe_error() {
    fn check_error<E: std::error::Error>() {}
    check_error::<InnerError>();
    check_error::<Error>();

    fn check_send<E: Send>() {}
    check_send::<InnerError>();
    check_send::<Error>();

    let t = thread::spawn(move || example());

    let v = t.join().expect("Thread panicked");
    v.unwrap_err();
}
