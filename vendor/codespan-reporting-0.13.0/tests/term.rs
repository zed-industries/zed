use codespan_reporting::diagnostic::{Diagnostic, Label};
use codespan_reporting::files::{SimpleFile, SimpleFiles};
use codespan_reporting::term::{Chars, Config, DisplayStyle};
use std::sync::LazyLock;

mod support;

use self::support::TestData;

static TEST_CONFIG: LazyLock<Config> = LazyLock::new(Config::default);

type LazyTestData<'a, T> = LazyLock<TestData<'a, T>>;

macro_rules! test_emit {
    (rich_color) => {
        #[test]
        #[cfg(feature = "termcolor")]
        fn rich_color() {
            let config = Config {
                display_style: DisplayStyle::Rich,
                ..TEST_CONFIG.clone()
            };

            insta::assert_snapshot!(TEST_DATA.emit_color(&config));
        }
    };
    (medium_color) => {
        #[test]
        #[cfg(feature = "termcolor")]
        fn medium_color() {
            let config = Config {
                display_style: DisplayStyle::Medium,
                ..TEST_CONFIG.clone()
            };

            insta::assert_snapshot!(TEST_DATA.emit_color(&config));
        }
    };
    (short_color) => {
        #[test]
        #[cfg(feature = "termcolor")]
        fn short_color() {
            let config = Config {
                display_style: DisplayStyle::Short,
                ..TEST_CONFIG.clone()
            };

            insta::assert_snapshot!(TEST_DATA.emit_color(&config));
        }
    };
    (rich_no_color) => {
        #[test]
        fn rich_no_color() {
            let config = Config {
                display_style: DisplayStyle::Rich,
                ..TEST_CONFIG.clone()
            };

            insta::assert_snapshot!(TEST_DATA.emit_no_color(&config));
        }
    };
    (medium_no_color) => {
        #[test]
        fn medium_no_color() {
            let config = Config {
                display_style: DisplayStyle::Medium,
                ..TEST_CONFIG.clone()
            };

            insta::assert_snapshot!(TEST_DATA.emit_no_color(&config));
        }
    };
    (short_no_color) => {
        #[test]
        fn short_no_color() {
            let config = Config {
                display_style: DisplayStyle::Short,
                ..TEST_CONFIG.clone()
            };

            insta::assert_snapshot!(TEST_DATA.emit_no_color(&config));
        }
    };
    (rich_ascii_no_color) => {
        #[test]
        fn rich_ascii_no_color() {
            let config = Config {
                display_style: DisplayStyle::Rich,
                chars: Chars::ascii(),
                ..TEST_CONFIG.clone()
            };

            insta::assert_snapshot!(TEST_DATA.emit_no_color(&config));
        }
    };
}

mod empty {
    use super::*;

    static TEST_DATA: LazyTestData<'static, SimpleFiles<&'static str, &'static str>> =
        LazyLock::new(|| {
            let files = SimpleFiles::new();

            let diagnostics = vec![
                Diagnostic::bug(),
                Diagnostic::error(),
                Diagnostic::warning(),
                Diagnostic::note(),
                Diagnostic::help(),
                Diagnostic::bug(),
            ];

            TestData { files, diagnostics }
        });

    test_emit!(rich_color);
    test_emit!(medium_color);
    test_emit!(short_color);
    test_emit!(rich_no_color);
    test_emit!(medium_no_color);
    test_emit!(short_no_color);
    test_emit!(rich_ascii_no_color);
}

/// Based on:
/// - https://github.com/rust-lang/rust/blob/c20d7eecbc0928b57da8fe30b2ef8528e2bdd5be/src/test/ui/codemap_tests/one_line.stderr
mod same_line {
    use super::*;

    static TEST_DATA: LazyTestData<'static, SimpleFiles<&'static str, String>> =
        LazyLock::new(|| {
            let mut files = SimpleFiles::new();

            let file_id1 = files.add(
                "one_line.rs",
                unindent::unindent(
                    r#"
                    fn main() {
                        let mut v = vec![Some("foo"), Some("bar")];
                        v.push(v.pop().unwrap());
                    }
                "#,
                ),
            );

            let diagnostics = vec![
                Diagnostic::error()
                    .with_code("E0499")
                    .with_message("cannot borrow `v` as mutable more than once at a time")
                    .with_labels(vec![
                        Label::primary(file_id1, 71..72)
                            .with_message("second mutable borrow occurs here"),
                        Label::secondary(file_id1, 64..65)
                            .with_message("first borrow later used by call"),
                        Label::secondary(file_id1, 66..70)
                            .with_message("first mutable borrow occurs here"),
                    ]),
                Diagnostic::error()
                    .with_message("aborting due to previous error")
                    .with_notes(vec![
                        "For more information about this error, try `rustc --explain E0499`."
                            .to_owned(),
                    ]),
            ];

            TestData { files, diagnostics }
        });

    test_emit!(rich_color);
    test_emit!(medium_color);
    test_emit!(short_color);
    test_emit!(rich_no_color);
    test_emit!(medium_no_color);
    test_emit!(short_no_color);
    test_emit!(rich_ascii_no_color);
}

/// Based on:
/// - https://github.com/rust-lang/rust/blob/c20d7eecbc0928b57da8fe30b2ef8528e2bdd5be/src/test/ui/nested_impl_trait.stderr
/// - https://github.com/rust-lang/rust/blob/c20d7eecbc0928b57da8fe30b2ef8528e2bdd5be/src/test/ui/typeck/typeck_type_placeholder_item.stderr
/// - https://github.com/rust-lang/rust/blob/c20d7eecbc0928b57da8fe30b2ef8528e2bdd5be/src/test/ui/no_send_res_ports.stderr
mod overlapping {
    use super::*;

    static TEST_DATA: LazyTestData<'static, SimpleFiles<&'static str, String>> = LazyLock::new(
        || {
            let mut files = SimpleFiles::new();

            let file_id1 = files.add(
                "nested_impl_trait.rs",
                unindent::unindent(
                    r#"
                    use std::fmt::Debug;

                    fn fine(x: impl Into<u32>) -> impl Into<u32> { x }

                    fn bad_in_ret_position(x: impl Into<u32>) -> impl Into<impl Debug> { x }
                "#,
                ),
            );
            let file_id2 = files.add(
                "typeck_type_placeholder_item.rs",
                unindent::unindent(
                    r#"
                    fn fn_test1() -> _ { 5 }
                    fn fn_test2(x: i32) -> (_, _) { (x, x) }
                "#,
                ),
            );
            let file_id3 = files.add(
                "libstd/thread/mod.rs",
                unindent::unindent(
                    r#"
                    #[stable(feature = "rust1", since = "1.0.0")]
                    pub fn spawn<F, T>(self, f: F) -> io::Result<JoinHandle<T>>
                    where
                        F: FnOnce() -> T,
                        F: Send + 'static,
                        T: Send + 'static,
                    {
                        unsafe { self.spawn_unchecked(f) }
                    }
                "#,
                ),
            );
            let file_id4 = files.add(
                "no_send_res_ports.rs",
                unindent::unindent(
                    r#"
                    use std::thread;
                    use std::rc::Rc;

                    #[derive(Debug)]
                    struct Port<T>(Rc<T>);

                    fn main() {
                        #[derive(Debug)]
                        struct Foo {
                            _x: Port<()>,
                        }

                        impl Drop for Foo {
                            fn drop(&mut self) {}
                        }

                        fn foo(x: Port<()>) -> Foo {
                            Foo {
                                _x: x
                            }
                        }

                        let x = foo(Port(Rc::new(())));

                        thread::spawn(move|| {
                            let y = x;
                            println!("{:?}", y);
                        });
                    }
                "#,
                ),
            );

            let diagnostics = vec![
                Diagnostic::error()
                    .with_code("E0666")
                    .with_message("nested `impl Trait` is not allowed")
                    .with_labels(vec![
                        Label::primary(file_id1, 129..139)
                            .with_message("nested `impl Trait` here"),
                        Label::secondary(file_id1, 119..140)
                            .with_message("outer `impl Trait`"),
                    ]),
                Diagnostic::error()
                    .with_code("E0121")
                    .with_message("the type placeholder `_` is not allowed within types on item signatures")
                        .with_labels(vec![
                            Label::primary(file_id2, 17..18)
                                .with_message("not allowed in type signatures"),
                            Label::secondary(file_id2, 17..18)
                                .with_message("help: replace with the correct return type: `i32`"),
                        ]),
                Diagnostic::error()
                    .with_code("E0121")
                    .with_message("the type placeholder `_` is not allowed within types on item signatures")
                        .with_labels(vec![
                            Label::primary(file_id2, 49..50)
                                .with_message("not allowed in type signatures"),
                            Label::primary(file_id2, 52..53)
                                .with_message("not allowed in type signatures"),
                            Label::secondary(file_id2, 48..54)
                                .with_message("help: replace with the correct return type: `(i32, i32)`"),
                        ]),
                Diagnostic::error()
                    .with_code("E0277")
                    .with_message("`std::rc::Rc<()>` cannot be sent between threads safely")
                    .with_labels(vec![
                        Label::primary(file_id4, 339..352)
                            .with_message("`std::rc::Rc<()>` cannot be sent between threads safely"),
                        Label::secondary(file_id4, 353..416)
                            .with_message("within this `[closure@no_send_res_ports.rs:29:19: 33:6 x:main::Foo]`"),
                        Label::secondary(file_id3, 141..145)
                            .with_message("required by this bound in `std::thread::spawn`"),
                    ])
                    .with_notes(vec![
                        "help: within `[closure@no_send_res_ports.rs:29:19: 33:6 x:main::Foo]`, the trait `std::marker::Send` is not implemented for `std::rc::Rc<()>`".to_owned(),
                        "note: required because it appears within the type `Port<()>`".to_owned(),
                        "note: required because it appears within the type `main::Foo`".to_owned(),
                        "note: required because it appears within the type `[closure@no_send_res_ports.rs:29:19: 33:6 x:main::Foo]`".to_owned(),
                    ]),
                Diagnostic::error()
                    .with_message("aborting due 5 previous errors")
                    .with_notes(vec![
                        "Some errors have detailed explanations: E0121, E0277, E0666.".to_owned(),
                        "For more information about an error, try `rustc --explain E0121`.".to_owned(),
                    ]),
            ];

            TestData { files, diagnostics }
        },
    );

    test_emit!(rich_color);
    test_emit!(medium_color);
    test_emit!(short_color);
    test_emit!(rich_no_color);
    test_emit!(medium_no_color);
    test_emit!(short_no_color);
    test_emit!(rich_ascii_no_color);
}

mod message {
    use super::*;

    static TEST_DATA: LazyTestData<'static, SimpleFiles<&'static str, &'static str>> =
        LazyLock::new(|| {
            let files = SimpleFiles::new();

            let diagnostics = vec![
                Diagnostic::error().with_message("a message"),
                Diagnostic::warning().with_message("a message"),
                Diagnostic::note().with_message("a message"),
                Diagnostic::help().with_message("a message"),
            ];

            TestData { files, diagnostics }
        });

    test_emit!(rich_color);
    test_emit!(medium_color);
    test_emit!(short_color);
    test_emit!(rich_no_color);
    test_emit!(medium_no_color);
    test_emit!(short_no_color);
    test_emit!(rich_ascii_no_color);
}

mod message_and_notes {
    use super::*;

    static TEST_DATA: LazyTestData<'static, SimpleFiles<&'static str, &'static str>> =
        LazyLock::new(|| {
            let files = SimpleFiles::new();

            let diagnostics = vec![
                Diagnostic::error()
                    .with_message("a message")
                    .with_notes(vec!["a note".to_owned()]),
                Diagnostic::warning()
                    .with_message("a message")
                    .with_notes(vec!["a note".to_owned()]),
                Diagnostic::note()
                    .with_message("a message")
                    .with_notes(vec!["a note".to_owned()]),
                Diagnostic::help()
                    .with_message("a message")
                    .with_notes(vec!["a note".to_owned()]),
            ];

            TestData { files, diagnostics }
        });

    test_emit!(rich_color);
    test_emit!(medium_color);
    test_emit!(short_color);
    test_emit!(rich_no_color);
    test_emit!(medium_no_color);
    test_emit!(short_no_color);
    test_emit!(rich_ascii_no_color);
}

mod message_errorcode {
    use super::*;

    static TEST_DATA: LazyTestData<'static, SimpleFiles<&'static str, &'static str>> =
        LazyLock::new(|| {
            let files = SimpleFiles::new();

            let diagnostics = vec![
                Diagnostic::error()
                    .with_message("a message")
                    .with_code("E0001"),
                Diagnostic::warning()
                    .with_message("a message")
                    .with_code("W001"),
                Diagnostic::note()
                    .with_message("a message")
                    .with_code("N0815"),
                Diagnostic::help()
                    .with_message("a message")
                    .with_code("H4711"),
                Diagnostic::error()
                    .with_message("where did my errorcode go?")
                    .with_code(""),
                Diagnostic::warning()
                    .with_message("where did my errorcode go?")
                    .with_code(""),
                Diagnostic::note()
                    .with_message("where did my errorcode go?")
                    .with_code(""),
                Diagnostic::help()
                    .with_message("where did my errorcode go?")
                    .with_code(""),
            ];

            TestData { files, diagnostics }
        });

    test_emit!(rich_no_color);
    test_emit!(short_no_color);
    test_emit!(rich_ascii_no_color);
}

mod empty_ranges {
    use super::*;

    static TEST_DATA: LazyTestData<'static, SimpleFile<&'static str, &'static str>> =
        LazyLock::new(|| {
            let file = SimpleFile::new("hello", "Hello world!\nBye world!\n   ");
            let eof = file.source().len();

            let diagnostics = vec![
                Diagnostic::note()
                    .with_message("middle")
                    .with_labels(vec![Label::primary((), 6..6).with_message("middle")]),
                Diagnostic::note()
                    .with_message("end of line")
                    .with_labels(vec![Label::primary((), 12..12).with_message("end of line")]),
                Diagnostic::note()
                    .with_message("end of line")
                    .with_labels(vec![Label::primary((), 23..23).with_message("end of line")]),
                Diagnostic::note()
                    .with_message("end of file")
                    .with_labels(vec![
                        Label::primary((), eof..eof).with_message("end of file")
                    ]),
            ];

            TestData {
                files: file,
                diagnostics,
            }
        });

    test_emit!(rich_color);
    test_emit!(medium_color);
    test_emit!(short_color);
    test_emit!(rich_no_color);
    test_emit!(medium_no_color);
    test_emit!(short_no_color);
    test_emit!(rich_ascii_no_color);
}

mod same_ranges {
    use super::*;

    static TEST_DATA: LazyTestData<'static, SimpleFile<&'static str, &'static str>> =
        LazyLock::new(|| {
            let file = SimpleFile::new("same_range", "::S { }");

            let diagnostics = vec![Diagnostic::error()
                .with_message("Unexpected token")
                .with_labels(vec![
                    Label::primary((), 4..4).with_message("Unexpected '{'"),
                    Label::secondary((), 4..4).with_message("Expected '('"),
                ])];

            TestData {
                files: file,
                diagnostics,
            }
        });

    test_emit!(rich_color);
    test_emit!(medium_color);
    test_emit!(short_color);
    test_emit!(rich_no_color);
    test_emit!(medium_no_color);
    test_emit!(short_no_color);
    test_emit!(rich_ascii_no_color);
}

mod multifile {
    use super::*;

    static TEST_DATA: LazyTestData<'static, SimpleFiles<&'static str, String>> =
        LazyLock::new(|| {
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

            let diagnostics = vec![
                // Unknown builtin error
                Diagnostic::error()
                    .with_message("unknown builtin: `NATRAL`")
                    .with_labels(vec![
                        Label::primary(file_id1, 96..102).with_message("unknown builtin")
                    ])
                    .with_notes(vec![
                        "there is a builtin with a similar name: `NATURAL`".to_owned()
                    ]),
                // Unused parameter warning
                Diagnostic::warning()
                    .with_message("unused parameter pattern: `n₂`")
                    .with_labels(vec![
                        Label::primary(file_id1, 285..289).with_message("unused parameter")
                    ])
                    .with_notes(vec!["consider using a wildcard pattern: `_`".to_owned()]),
                // Unexpected type error
                Diagnostic::error()
                    .with_message("unexpected type in application of `_+_`")
                    .with_code("E0001")
                    .with_labels(vec![
                        Label::primary(file_id2, 37..44)
                            .with_message("expected `Nat`, found `String`"),
                        Label::secondary(file_id1, 130..155)
                            .with_message("based on the definition of `_+_`"),
                    ])
                    .with_notes(vec![unindent::unindent(
                        "
                            expected type `Nat`
                               found type `String`
                        ",
                    )]),
            ];

            TestData { files, diagnostics }
        });

    test_emit!(rich_color);
    test_emit!(medium_color);
    test_emit!(short_color);
    test_emit!(rich_no_color);
    test_emit!(medium_no_color);
    test_emit!(short_no_color);
    test_emit!(rich_ascii_no_color);
}

mod fizz_buzz {
    use super::*;

    static TEST_DATA: LazyTestData<'static, SimpleFiles<&'static str, String>> =
        LazyLock::new(|| {
            let mut files = SimpleFiles::new();

            let file_id = files.add(
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

            let diagnostics = vec![
                // Incompatible match clause error
                Diagnostic::error()
                    .with_message("`case` clauses have incompatible types")
                    .with_code("E0308")
                    .with_labels(vec![
                        Label::primary(file_id, 163..166)
                            .with_message("expected `String`, found `Nat`"),
                        Label::secondary(file_id, 62..166)
                            .with_message("`case` clauses have incompatible types"),
                        Label::secondary(file_id, 41..47)
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
                        Label::primary(file_id, 328..331)
                            .with_message("expected `String`, found `Nat`"),
                        Label::secondary(file_id, 211..331)
                            .with_message("`case` clauses have incompatible types"),
                        Label::secondary(file_id, 258..268)
                            .with_message("this is found to be of type `String`"),
                        Label::secondary(file_id, 284..290)
                            .with_message("this is found to be of type `String`"),
                        Label::secondary(file_id, 306..312)
                            .with_message("this is found to be of type `String`"),
                        Label::secondary(file_id, 186..192)
                            .with_message("expected type `String` found here"),
                    ])
                    .with_notes(vec![unindent::unindent(
                        "
                            expected type `String`
                               found type `Nat`
                        ",
                    )]),
            ];

            TestData { files, diagnostics }
        });

    test_emit!(rich_color);
    test_emit!(medium_color);
    test_emit!(short_color);
    test_emit!(rich_no_color);
    test_emit!(medium_no_color);
    test_emit!(short_no_color);
    test_emit!(rich_ascii_no_color);
}

mod multiline_overlapping {
    use super::*;

    static TEST_DATA: LazyTestData<'static, SimpleFile<&'static str, String>> =
        LazyLock::new(|| {
            let file = SimpleFile::new(
                "codespan/src/file.rs",
                "        match line_index.compare(self.last_line_index()) {
            Ordering::Less => Ok(self.line_starts()[line_index.to_usize()]),
            Ordering::Equal => Ok(self.source_span().end()),
            Ordering::Greater => LineIndexOutOfBoundsError {
                given: line_index,
                max: self.last_line_index(),
            },
        }"
                .to_owned(),
            );

            let diagnostics = vec![Diagnostic::error()
                .with_message("match arms have incompatible types")
                .with_code("E0308")
                .with_labels(vec![
                // this secondary label is before the primary label to test the locus calculation (see issue #259)
                Label::secondary((), 89..134).with_message(
                    "this is found to be of type `Result<ByteIndex, LineIndexOutOfBoundsError>`",
                ),
                Label::primary((), 230..351).with_message(
                    "expected enum `Result`, found struct `LineIndexOutOfBoundsError`",
                ),
                Label::secondary((), 8..362).with_message("`match` arms have incompatible types"),
                Label::secondary((), 167..195).with_message(
                    "this is found to be of type `Result<ByteIndex, LineIndexOutOfBoundsError>`",
                ),
            ])
                .with_notes(vec![unindent::unindent(
                    "
                            expected type `Result<ByteIndex, LineIndexOutOfBoundsError>`
                               found type `LineIndexOutOfBoundsError`
                        ",
                )])];

            TestData {
                files: file,
                diagnostics,
            }
        });

    test_emit!(rich_color);
    test_emit!(medium_color);
    test_emit!(short_color);
    test_emit!(rich_no_color);
    test_emit!(medium_no_color);
    test_emit!(short_no_color);
    test_emit!(rich_ascii_no_color);
}

mod tabbed {
    use super::*;

    static TEST_DATA: LazyTestData<'static, SimpleFiles<&'static str, String>> =
        LazyLock::new(|| {
            let mut files = SimpleFiles::new();

            let file_id = files.add(
                "tabbed",
                unindent::unindent(
                    "
                Entity:
                \tArmament:
                \t\tWeapon: DogJaw
                \t\tReloadingCondition:\tattack-cooldown
                \tFoo: Bar",
                ),
            );

            let diagnostics = vec![
                Diagnostic::warning()
                    .with_message("unknown weapon `DogJaw`")
                    .with_labels(vec![
                        Label::primary(file_id, 29..35).with_message("the weapon")
                    ]),
                Diagnostic::warning()
                    .with_message("unknown condition `attack-cooldown`")
                    .with_labels(vec![
                        Label::primary(file_id, 58..73).with_message("the condition")
                    ]),
                Diagnostic::warning()
                    .with_message("unknown field `Foo`")
                    .with_labels(vec![
                        Label::primary(file_id, 75..78).with_message("the field")
                    ]),
            ];

            TestData { files, diagnostics }
        });

    #[test]
    fn tab_width_default_no_color() {
        let config = TEST_CONFIG.clone();

        insta::assert_snapshot!(TEST_DATA.emit_no_color(&config));
    }

    #[test]
    fn tab_width_3_no_color() {
        let config = Config {
            tab_width: 3,
            ..TEST_CONFIG.clone()
        };

        insta::assert_snapshot!(TEST_DATA.emit_no_color(&config));
    }

    #[test]
    fn tab_width_6_no_color() {
        let config = Config {
            tab_width: 6,
            ..TEST_CONFIG.clone()
        };

        insta::assert_snapshot!(TEST_DATA.emit_no_color(&config));
    }
}

mod tab_columns {
    use super::*;

    static TEST_DATA: LazyTestData<'static, SimpleFiles<&'static str, String>> =
        LazyLock::new(|| {
            let mut files = SimpleFiles::new();

            let source = unindent::unindent(
                "\thello
                ∙\thello
                ∙∙\thello
                ∙∙∙\thello
                ∙∙∙∙\thello
                ∙∙∙∙∙\thello
                ∙∙∙∙∙∙\thello",
            );
            let hello_ranges = source
                .match_indices("hello")
                .map(|(start, hello)| start..(start + hello.len()))
                .collect::<Vec<_>>();

            let file_id = files.add("tab_columns", source);

            let diagnostics = vec![Diagnostic::warning().with_message("tab test").with_labels(
                hello_ranges
                    .into_iter()
                    .map(|range| Label::primary(file_id, range))
                    .collect(),
            )];

            TestData { files, diagnostics }
        });

    #[test]
    fn tab_width_default_no_color() {
        let config = TEST_CONFIG.clone();

        insta::assert_snapshot!(TEST_DATA.emit_no_color(&config));
    }

    #[test]
    fn tab_width_2_no_color() {
        let config = Config {
            tab_width: 2,
            ..TEST_CONFIG.clone()
        };

        insta::assert_snapshot!(TEST_DATA.emit_no_color(&config));
    }

    #[test]
    fn tab_width_3_no_color() {
        let config = Config {
            tab_width: 3,
            ..TEST_CONFIG.clone()
        };

        insta::assert_snapshot!(TEST_DATA.emit_no_color(&config));
    }

    #[test]
    fn tab_width_6_no_color() {
        let config = Config {
            tab_width: 6,
            ..TEST_CONFIG.clone()
        };

        insta::assert_snapshot!(TEST_DATA.emit_no_color(&config));
    }
}

/// Based on:
/// - https://github.com/TheSamsa/rust/blob/75cf41afb468152611212271bae026948cd3ba46/src/test/ui/codemap_tests/unicode.stderr
mod unicode {
    use super::*;

    static TEST_DATA: LazyTestData<'static, SimpleFile<&'static str, String>> =
        LazyLock::new(|| {
            let prefix = r#"extern "#;
            let abi = r#""路濫狼á́́""#;
            let suffix = r#" fn foo() {}"#;

            let file = SimpleFile::new("unicode.rs", format!("{}{}{}", prefix, abi, suffix));

            let diagnostics = vec![
                Diagnostic::error()
                    .with_code("E0703")
                    .with_message("invalid ABI: found `路濫狼á́́`")
                    .with_labels(vec![Label::primary(
                        (),
                        prefix.len()..(prefix.len() + abi.len()),
                    )
                    .with_message("invalid ABI")])
                    .with_notes(vec![unindent::unindent(
                        "
                            valid ABIs:
                              - aapcs
                              - amdgpu-kernel
                              - C
                              - cdecl
                              - efiapi
                              - fastcall
                              - msp430-interrupt
                              - platform-intrinsic
                              - ptx-kernel
                              - Rust
                              - rust-call
                              - rust-intrinsic
                              - stdcall
                              - system
                              - sysv64
                              - thiscall
                              - unadjusted
                              - vectorcall
                              - win64
                              - x86-interrupt
                        ",
                    )]),
                Diagnostic::error()
                    .with_message("aborting due to previous error")
                    .with_notes(vec![
                        "For more information about this error, try `rustc --explain E0703`."
                            .to_owned(),
                    ]),
            ];

            TestData {
                files: file,
                diagnostics,
            }
        });

    test_emit!(rich_no_color);
    test_emit!(medium_no_color);
    test_emit!(short_no_color);
}

mod unicode_spans {
    use super::*;

    static TEST_DATA: LazyTestData<'static, SimpleFile<&'static str, String>> =
        LazyLock::new(|| {
            let moon_phases = r#"🐄🌑🐄🌒🐄🌓🐄🌔🐄🌕🐄🌖🐄🌗🐄🌘🐄"#.to_string();
            let invalid_start = 1;
            let invalid_end = "🐄".len() - 1;
            assert!(!moon_phases.is_char_boundary(invalid_start));
            assert!(!moon_phases.is_char_boundary(invalid_end));
            assert_eq!("🐄".len(), 4);
            let file = SimpleFile::new("moon_jump.rs", moon_phases);
            let diagnostics = vec![
                Diagnostic::error()
                    .with_code("E01")
                    .with_message("cow may not jump during new moon.")
                    .with_labels(vec![
                        Label::primary((), invalid_start..invalid_end).with_message("Invalid jump")
                    ]),
                Diagnostic::note()
                    .with_message("invalid unicode range")
                    .with_labels(vec![Label::secondary((), invalid_start.."🐄".len())
                        .with_message("Cow range does not start at boundary.")]),
                Diagnostic::note()
                    .with_message("invalid unicode range")
                    .with_labels(vec![Label::secondary((), "🐄🌑".len().."🐄🌑🐄".len() - 1)
                        .with_message("Cow range does not end at boundary.")]),
                Diagnostic::note()
                    .with_message("invalid unicode range")
                    .with_labels(vec![Label::secondary(
                        (),
                        invalid_start.."🐄🌑🐄".len() - 1,
                    )
                    .with_message("Cow does not start or end at boundary.")]),
            ];
            TestData {
                files: file,
                diagnostics,
            }
        });

    test_emit!(rich_no_color);
    test_emit!(medium_no_color);
    test_emit!(short_no_color);
}

mod position_indicator {
    use super::*;

    static TEST_DATA: LazyTestData<'static, SimpleFile<&'static str, String>> = LazyLock::new(
        || {
            let file = SimpleFile::new(
                "tests/main.js",
                unindent::unindent(
                    "\"use strict\";
                    let zero=0;
                    function foo() {
                      \"use strict\";
                      one=1;
                    }",
                ),
            );
            let diagnostics = vec![
                Diagnostic::warning()
                    .with_code("ParserWarning")
                    .with_message("The strict mode declaration in the body of function `foo` is redundant, as the outer scope is already in strict mode")
                    .with_labels(vec![
                        Label::primary((), 45..57)
                            .with_message("This strict mode declaration is redundant"),
                        Label::secondary((), 0..12)
                            .with_message("Strict mode is first declared here"),
                    ]),
            ];
            TestData {
                files: file,
                diagnostics,
            }
        },
    );

    test_emit!(rich_no_color);
    test_emit!(medium_no_color);
    test_emit!(short_no_color);
    test_emit!(rich_ascii_no_color);
}

mod multiline_omit {
    use super::*;

    static TEST_CONFIG: LazyLock<Config> = LazyLock::new(|| Config {
        start_context_lines: 2,
        end_context_lines: 1,
        ..Config::default()
    });

    static TEST_DATA: LazyTestData<'static, SimpleFiles<&'static str, String>> =
        LazyLock::new(|| {
            let mut files = SimpleFiles::new();

            let file_id1 = files.add(
                "empty_if_comments.lua",
                [
                    "elseif 3 then", // primary label starts here
                    "",              // context line
                    "",
                    "",
                    "",
                    "",
                    "",
                    "",
                    "",     // context line
                    "else", // primary label ends here
                ]
                .join("\n"),
            );

            let file_id2 = files.add(
                "src/lib.rs",
                [
                    "fn main() {",
                    "    1",   // primary label starts here
                    "    + 1", // context line
                    "    + 1", // skip
                    "    + 1", // skip
                    "    + 1", // skip
                    "    +1",  // secondary label here
                    "    + 1", // this single line will not be skipped; the previously filtered out label must be retrieved
                    "    + 1", // context line
                    "    + 1", // primary label ends here
                    "}",
                ]
                .join("\n"),
            );

            let diagnostics = vec![
                Diagnostic::error()
                    .with_message("empty elseif block")
                    .with_code("empty_if")
                    .with_labels(vec![
                        Label::primary(file_id1, 0..23),
                        Label::secondary(file_id1, 15..21)
                            .with_message("content should be in here"),
                    ]),
                Diagnostic::error()
                    .with_message("mismatched types")
                    .with_code("E0308")
                    .with_labels(vec![
                        Label::primary(file_id2, 17..80).with_message("expected (), found integer"),
                        Label::secondary(file_id2, 55..55).with_message("missing whitespace"),
                    ])
                    .with_notes(vec![
                        "note:\texpected type `()`\n\tfound type `{integer}`".to_owned()
                    ]),
            ];

            TestData { files, diagnostics }
        });

    test_emit!(rich_no_color);
}

mod surrounding_lines {
    use super::*;

    static TEST_CONFIG: LazyLock<Config> = LazyLock::new(|| Config {
        before_label_lines: 2,
        after_label_lines: 1,
        ..Config::default()
    });

    static TEST_DATA: LazyLock<TestData<'static, SimpleFiles<&'static str, String>>> =
        LazyLock::new(|| {
            let mut files = SimpleFiles::new();

            let file_id = files.add(
                "surroundingLines.fun",
                unindent::unindent(
                    r#"
                #[foo]
                fn main() {
                    println!(
                        "{}",
                        Foo
                    );
                }

                struct Foo"#,
                ),
            );

            let diagnostics = vec![
                Diagnostic::error()
                    .with_message("Unknown attribute macro")
                    .with_labels(vec![Label::primary(file_id, 2..5)
                        .with_message("No attribute macro `foo` known")]),
                Diagnostic::error()
                    .with_message("Missing argument for format")
                    .with_labels(vec![
                        Label::primary(file_id, 55..58)
                            .with_message("No instance of std::fmt::Display exists for type Foo"),
                        Label::secondary(file_id, 42..44)
                            .with_message("Unable to use `{}`-directive to display `Foo`"),
                    ]),
                Diagnostic::error()
                    .with_message("Syntax error")
                    .with_labels(vec![
                        Label::primary(file_id, 79..79).with_message("Missing a semicolon")
                    ]),
            ];

            TestData { files, diagnostics }
        });

    test_emit!(rich_no_color);
}
