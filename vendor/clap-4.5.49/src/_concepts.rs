//! ## CLI Concepts
//!
//! Note: this will be speaking towards the general case.
//!
//! ### Environmental context
//!
//! When you run a command line application, it is inside a terminal emulator, or terminal.
//! This handles integration with the rest of your system including user input,
//! rendering, etc.
//!
//! The terminal will run inside of itself an interactive shell.
//! The shell is responsible for showing the prompt, receiving input including the command you are writing,
//! letting that command take over until completion, and then repeating.
//! This is called a read-eval-print loop, or REPL.
//! Typically the shell will take the command you typed and split it into separate arguments,
//! including handling of quoting, escaping, and globbing.
//! The parsing and evaluation of the command is shell specific.
//! The shell will then determine which application to run and then pass the full command-line as
//! individual arguments to your program.
//! These arguments are exposed in Rust as [`std::env::args_os`].
//!
//! Windows is an exception in Shell behavior in that the command is passed as an individual
//! string, verbatim, and the application must split the arguments.
//! [`std::env::args_os`] will handle the splitting for you but will not handle globs.
//!
//! Takeaways:
//! - Your application will only see quotes that have been escaped within the shell
//!   - e.g. to receive `message="hello world"`, you may need to type `'message="hello world"'` or `message=\"hello world\"`
//! - If your applications needs to parse a string into arguments,
//!   you will need to pick a syntax and do it yourself
//!   - POSIX's shell syntax is a common choice and available in packages like [shlex](https://docs.rs/shlex)
//!   - See also our [REPL cookbook entry][crate::_cookbook::repl]
//! - On Windows, you will need to handle globbing yourself if desired
//!   - [`wild`](https://docs.rs/wild) can help with that
//!
//! ### Argument Parsing
//!
//! The first argument of [`std::env::args_os`] is the [`Command::bin_name`]
//! which is usually limited to affecting [`Command::render_usage`].
//! [`Command::no_binary_name`] and [`Command::multicall`] exist for rare cases when this assumption is not valid.
//!
//! Command-lines are a context-sensitive grammar,
//! meaning the interpretation of an argument is based on the arguments that came before.
//! Arguments come in one of several flavors:
//! - Values
//! - Flags
//! - Subcommands
//!
//! When examining the next argument,
//! 1. If it starts with a `--`,
//!    then that is a long Flag and all remaining text up to a `=` or the end is
//!    matched to a [`Arg::long`], [`Command::long_flag`], or alias.
//!    - Everything after the `=` is taken as a Value and parsing a new argument is examined.
//!    - If no `=` is present, then Values will be taken according to [`Arg::num_args`]
//!    - We generally call a Flag that takes a Value an Option
//! 2. If it starts with a `-`,
//!    then that is a sequence of short Flags where each character is matched against a [`Arg::short`], [`Command::short_flag`] or
//!    alias until `=`, the end, or a short Flag takes Values (see [`Arg::num_args`])
//! 3. If its a `--`, that is an escape and all future arguments are considered to be a Value, even if
//!    they start with `--` or `-`
//! 4. If it matches a [`Command::name`],
//!    then the argument is a subcommand
//! 5. If there is an [`Arg`] at the next [`Arg::index`],
//!    then the argument is considered a Positional argument
//!
//! When a subcommand matches,
//! all further arguments are parsed by that [`Command`].
//!
//! There are many settings that tweak this behavior, including:
//! - [`Arg::last`]: a positional that can only come after `--`
//! - [`Arg::trailing_var_arg`]: all further arguments are captured as additional Values
//! - [`Arg::allow_hyphen_values`] and [`Arg::allow_negative_numbers`]: assumes arguments
//!   starting with `-` are Values and not Flags.
//! - [`Command::subcommand_precedence_over_arg`]: when an [`Arg::num_args`] takes Values,
//!   stop if one matches a subCommand
//! - [`Command::allow_missing_positional`]: in limited cases a [`Arg::index`] may be skipped
//! - [`Command::allow_external_subcommands`]: treat any unknown argument as a subcommand, capturing
//!   all remaining arguments.
//!
//! Takeaways
//! - Values that start with a `-` either need to be escaped by the user with `--`
//!   (if a positional),
//!   or you need to set [`Arg::allow_hyphen_values`] or [`Arg::allow_negative_numbers`]
//! - [`Arg::num_args`],
//!   [`ArgAction::Append`] (on a positional),
//!   [`Arg::trailing_var_arg`],
//!   and [`Command::allow_external_subcommands`]
//!   all affect the parser in similar but slightly different ways and which to use depends on your
//!   application
//!
//! ### Value Parsing
//!
//! When reacting to a Flag (no Value),
//! [`Arg::default_missing_values`] will be applied.
//!
//! The Value will be split by [`Arg::value_delimiter`].
//!
//! The Value will then be stored according to its [`ArgAction`].
//! For most [`ArgAction`]s,
//! the Value will be parsed according to [`ValueParser`]
//! and stored in the [`ArgMatches`].

#![allow(unused_imports)]
use clap_builder::builder::ValueParser;
use clap_builder::Arg;
use clap_builder::ArgAction;
use clap_builder::ArgMatches;
use clap_builder::Command;
