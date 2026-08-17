use snafu::{prelude::*, Backtrace, ErrorCompat};

type AnotherError = Box<dyn std::error::Error>;

#[derive(Debug, Snafu)]
enum Error {
    #[snafu(display("Invalid user {user_id}:\n{backtrace}"))]
    _InvalidUser {
        user_id: i32,
        backtrace: Backtrace,
    },

    _WithSource {
        source: AnotherError,
        backtrace: Backtrace,
    },

    _WithSourceAndOtherInfo {
        user_id: i32,
        source: AnotherError,
        backtrace: Backtrace,
    },

    WithBacktrace {
        backtrace: Backtrace,
    },
}

type Result<T, E = Error> = std::result::Result<T, E>;

fn example() -> Result<()> {
    WithBacktraceSnafu.fail()
}

#[test]
fn is_compatible_with_std_error_trait() {
    fn expects_std_trait<E: std::error::Error>() {}

    expects_std_trait::<Error>();
}

#[test]
fn is_compatible_with_std_backtrace_type() {
    fn expects_std_type(_: &std::backtrace::Backtrace) {}

    let error = example().unwrap_err();
    let backtrace = ErrorCompat::backtrace(&error).unwrap();
    expects_std_type(&backtrace);
}

#[test]
fn backtrace_contains_function_names() {
    let error = example().unwrap_err();
    let backtrace = ErrorCompat::backtrace(&error).unwrap();
    assert!(backtrace.to_string().contains("::example"));
}

mod delegation {
    use snafu::{prelude::*, ErrorCompat};

    mod house {
        use snafu::{prelude::*, Backtrace};

        #[derive(Debug, Snafu)]
        pub struct FatalError {
            backtrace: Backtrace,
        }

        pub fn answer_telephone() -> Result<(), FatalError> {
            FatalSnafu.fail()
        }
    }

    #[derive(Debug, Snafu)]
    enum Error {
        MovieTrope {
            #[snafu(backtrace)]
            source: house::FatalError,
        },

        SourceAndBacktraceAttrs {
            // Testing source and backtrace attributes; the field should be recognized as a source,
            // and allow us to get a backtrace delegated from the source error
            #[snafu(source, backtrace)]
            cause: house::FatalError,
        },
    }

    fn delegate_example() -> Result<(), Error> {
        house::answer_telephone().context(MovieTropeSnafu)?;

        Ok(())
    }

    #[test]
    fn backtrace_comes_from_delegated_error() {
        let e = delegate_example().unwrap_err();
        let text = ErrorCompat::backtrace(&e)
            .map(ToString::to_string)
            .unwrap_or_default();
        assert!(
            text.contains("answer_telephone"),
            "{:?} does not contain `answer_telephone`",
            text,
        );
    }

    fn delegate_and_rename_example() -> Result<(), Error> {
        house::answer_telephone().context(SourceAndBacktraceAttrsSnafu)
    }

    #[test]
    fn backtrace_comes_from_renamed_delegated_error() {
        let e = delegate_and_rename_example().unwrap_err();
        let text = ErrorCompat::backtrace(&e)
            .map(ToString::to_string)
            .unwrap_or_default();
        assert!(
            text.contains("answer_telephone"),
            "{:?} does not contain `answer_telephone`",
            text,
        );
    }
}

mod whatever_nested {
    use snafu::{prelude::*, Whatever};

    fn inner_outer() -> Result<(), Whatever> {
        not_a_whatever().with_whatever_context(|_| format!("Outer failure"))
    }

    fn not_a_whatever() -> Result<(), Box<dyn std::error::Error>> {
        inner_whatever().map_err(Into::into)
    }

    fn inner_whatever() -> Result<(), Whatever> {
        whatever!("Inner failure");
    }

    #[test]
    fn backtrace_method_delegates_to_nested_whatever() {
        let e = inner_outer().unwrap_err();
        let bt = e.backtrace().expect("Must have a backtrace");
        let text = bt.to_string();
        assert!(
            text.contains("::inner_whatever"),
            "{:?} does not contain `::inner_whatever`",
            text,
        );
    }
}

mod boxed {
    use snafu::{prelude::*, Backtrace, ErrorCompat};
    use std::{rc::Rc, sync::Arc};

    #[derive(Debug, Snafu)]
    struct BoxBacktrace {
        backtrace: Box<Backtrace>,
    }

    #[test]
    fn box_is_backtrace() {
        let e = BoxBacktraceSnafu.build();
        let text = ErrorCompat::backtrace(&e)
            .map(ToString::to_string)
            .unwrap_or_default();
        assert!(text.contains("::box_is_backtrace"));
    }

    #[derive(Debug, Snafu)]
    struct RcBacktrace {
        backtrace: Rc<Backtrace>,
    }

    #[test]
    fn rc_is_backtrace() {
        let e = RcBacktraceSnafu.build();
        let text = ErrorCompat::backtrace(&e)
            .map(ToString::to_string)
            .unwrap_or_default();
        assert!(text.contains("::rc_is_backtrace"));
    }

    #[derive(Debug, Snafu)]
    struct ArcBacktrace {
        backtrace: Arc<Backtrace>,
    }

    #[test]
    fn arc_is_backtrace() {
        let e = ArcBacktraceSnafu.build();
        let text = ErrorCompat::backtrace(&e)
            .map(ToString::to_string)
            .unwrap_or_default();
        assert!(text.contains("::arc_is_backtrace"));
    }
}
