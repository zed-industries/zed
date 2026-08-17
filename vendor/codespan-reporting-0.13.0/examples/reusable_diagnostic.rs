use codespan_reporting::diagnostic::{Diagnostic, Label};
use codespan_reporting::files::SimpleFile;
use codespan_reporting::term::{self, Config};
use core::ops::Range;

#[cfg(not(feature = "termcolor"))]
fn main() {
    panic!("this example requires termcolor feature");
}

#[cfg(feature = "termcolor")]
fn main() -> anyhow::Result<()> {
    use codespan_reporting::term::termcolor::{ColorChoice, StandardStream};

    #[derive(Debug)]
    pub struct Opts {
        color: ColorChoice,
    }

    fn parse_args() -> Result<Opts, pico_args::Error> {
        let mut pargs = pico_args::Arguments::from_env();
        let color = pargs
            .opt_value_from_str("--color")?
            .unwrap_or(ColorChoice::Auto);
        Ok(Opts { color })
    }

    let file = SimpleFile::new(
        "main.rs",
        unindent::unindent(
            r#"
                fn main() {
                    let foo: i32 = "hello, world";
                    foo += 1;
                }
            "#,
        ),
    );

    let errors = [
        Error::MismatchType(
            Item::new(20..23, "i32"),
            Item::new(31..45, "\"hello, world\""),
        ),
        Error::MutatingImmutable(Item::new(20..23, "foo"), Item::new(51..59, "foo += 1")),
    ];

    let Opts { color } = parse_args()?;

    let writer = StandardStream::stderr(color);
    let config = Config::default();

    for diagnostic in errors.iter().map(Error::report) {
        term::emit_to_write_style(&mut writer.lock(), &config, &file, &diagnostic)?;
    }

    Ok(())
}

/// An error enum that represent all possible errors within your program
enum Error {
    MismatchType(Item, Item),
    MutatingImmutable(Item, Item),
}

impl Error {
    fn report(&self) -> Diagnostic<()> {
        match self {
            Error::MismatchType(left, right) => Diagnostic::error()
                .with_code("E0308")
                .with_message("mismatch types")
                .with_labels(vec![
                    Label::primary((), right.range.clone()).with_message(format!(
                        "Expected `{}`, found: `{}`",
                        left.content, right.content,
                    )),
                    Label::secondary((), left.range.clone()).with_message("expected due to this"),
                ]),
            Error::MutatingImmutable(original, mutating) => Diagnostic::error()
                .with_code("E0384")
                .with_message(format!(
                    "cannot mutate immutable variable `{}`",
                    original.content,
                ))
                .with_labels(vec![
                    Label::secondary((), original.range.clone()).with_message(unindent::unindent(
                        &format!(
                            r#"
                                first assignment to `{0}`
                                help: make this binding mutable: `mut {0}`
                            "#,
                            original.content,
                        ),
                    )),
                    Label::primary((), mutating.range.clone())
                        .with_message("cannot assign twice to immutable variable"),
                ]),
        }
    }
}

/// An item in the source code to be used in the `Error` enum.
/// In a more complex program it could also contain a `files::FileId` to handle errors that occur inside multiple files.
struct Item {
    range: Range<usize>,
    content: String,
}

impl Item {
    fn new(range: Range<usize>, content: impl Into<String>) -> Item {
        let content = content.into();
        Item { range, content }
    }
}
