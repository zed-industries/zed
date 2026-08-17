//! To run this example, execute the following command from the top level of
//! this repository:
//!
//! ```sh
//! cargo run --example term
//! ```

use codespan_reporting::diagnostic::{Diagnostic, Label};
use codespan_reporting::files::SimpleFiles;
use codespan_reporting::term::{self, Config};

#[cfg(not(feature = "termcolor"))]
fn main() {
    panic!("this example requires termcolor feature");
}

#[cfg(feature = "termcolor")]
fn main() -> anyhow::Result<()> {
    use codespan_reporting::term::termcolor::{ColorChoice, StandardStream};

    #[derive(Debug)]
    pub struct Opts {
        /// Configure coloring of output
        pub color: ColorChoice,
    }

    fn parse_args() -> Result<Opts, pico_args::Error> {
        let mut pargs = pico_args::Arguments::from_env();
        let color = pargs
            .opt_value_from_str("--color")?
            .unwrap_or(ColorChoice::Auto);
        Ok(Opts { color })
    }

    let Opts { color } = parse_args()?;

    let mut files = SimpleFiles::new();

    let file_id1 = files.add(
        "Data/Nat.fun",
        unindent::unindent(
            "
                module Data.Nat where

                data Nat : Type where
                    zero : Nat
                    succ : Nat → Nat

                {-# BUILTIN NATRAL Nat #-}

                infixl 6 _+_ _-_

                _+_ : Nat → Nat → Nat
                zero    + n₂ = n₂
                succ n₁ + n₂ = succ (n₁ + n₂)

                _-_ : Nat → Nat → Nat
                n₁      - zero    = n₁
                zero    - succ n₂ = zero
                succ n₁ - succ n₂ = n₁ - n₂
            ",
        ),
    );

    let file_id2 = files.add(
        "Test.fun",
        unindent::unindent(
            r#"
                module Test where

                _ : Nat
                _ = 123 + "hello"
            "#,
        ),
    );

    let file_id3 = files.add(
        "FizzBuzz.fun",
        unindent::unindent(
            r#"
                module FizzBuzz where

                fizz₁ : Nat → String
                fizz₁ num = case (mod num 5) (mod num 3) of
                    0 0 => "FizzBuzz"
                    0 _ => "Fizz"
                    _ 0 => "Buzz"
                    _ _ => num

                fizz₂ : Nat → String
                fizz₂ num =
                    case (mod num 5) (mod num 3) of
                        0 0 => "FizzBuzz"
                        0 _ => "Fizz"
                        _ 0 => "Buzz"
                        _ _ => num
            "#,
        ),
    );

    let diagnostics = [
        // Unknown builtin error
        Diagnostic::error()
            .with_message("unknown builtin: `NATRAL`")
            .with_label(Label::primary(file_id1, 96..102).with_message("unknown builtin"))
            .with_notes(vec![
                "there is a builtin with a similar name: `NATURAL`".to_owned()
            ]),
        // Unused parameter warning
        Diagnostic::warning()
            .with_message("unused parameter pattern: `n₂`")
            .with_label(Label::primary(file_id1, 285..289).with_message("unused parameter"))
            .with_notes(vec!["consider using a wildcard pattern: `_`".to_owned()]),
        // Unexpected type error
        Diagnostic::error()
            .with_message("unexpected type in application of `_+_`")
            .with_code("E0001")
            .with_labels(vec![
                Label::primary(file_id2, 37..44).with_message("expected `Nat`, found `String`"),
                Label::secondary(file_id1, 130..155)
                    .with_message("based on the definition of `_+_`"),
            ])
            .with_notes(vec![unindent::unindent(
                "
                    expected type `Nat`
                       found type `String`
                ",
            )]),
        // Incompatible match clause error
        Diagnostic::error()
            .with_message("`case` clauses have incompatible types")
            .with_code("E0308")
            .with_labels(vec![
                Label::primary(file_id3, 163..166).with_message("expected `String`, found `Nat`"),
                Label::secondary(file_id3, 62..166)
                    .with_message("`case` clauses have incompatible types"),
                Label::secondary(file_id3, 41..47)
                    .with_message("expected type `String` found here"),
            ])
            .with_notes(vec![unindent::unindent(
                "
                    expected type `String`
                       found type `Nat`
                ",
            )]),
        // Incompatible match clause error
        Diagnostic::error()
            .with_message("`case` clauses have incompatible types")
            .with_code("E0308")
            .with_labels(vec![
                Label::primary(file_id3, 328..331).with_message("expected `String`, found `Nat`"),
                Label::secondary(file_id3, 211..331)
                    .with_message("`case` clauses have incompatible types"),
                Label::secondary(file_id3, 258..268)
                    .with_message("this is found to be of type `String`"),
                Label::secondary(file_id3, 284..290)
                    .with_message("this is found to be of type `String`"),
                Label::secondary(file_id3, 306..312)
                    .with_message("this is found to be of type `String`"),
                Label::secondary(file_id3, 186..192)
                    .with_message("expected type `String` found here"),
            ])
            .with_notes(vec![unindent::unindent(
                "
                    expected type `String`
                       found type `Nat`
                ",
            )]),
    ];

    let writer = StandardStream::stderr(color);
    let config = Config::default();
    for diagnostic in &diagnostics {
        term::emit_to_write_style(&mut writer.lock(), &config, &files, diagnostic)?;
    }

    Ok(())
}
