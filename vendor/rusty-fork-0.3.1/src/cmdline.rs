//-
// Copyright 2018, 2020, 2025, Jason Lingle
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Internal module which parses and modifies the rust test command-line.

use std::env;

use crate::error::*;

/// How a hyphen-prefixed argument passed to the parent process should be
/// handled when constructing the command-line for the child process.
#[derive(Clone, Copy, Debug, PartialEq)]
enum FlagType {
    /// Pass the flag through unchanged. The boolean indicates whether the flag
    /// is followed by an argument.
    Pass(bool),
    /// Drop the flag entirely. The boolean indicates whether the flag is
    /// followed by an argument.
    Drop(bool),
    /// Indicates a known flag that should never be encountered. The string is
    /// a human-readable error message.
    Error(&'static str),
}

/// Table of all flags in the 2025-10-04 nightly build.
///
/// A number of these that affect output are dropped because we append our own
/// options.
static KNOWN_FLAGS: &[(&str, FlagType)] = &[
    ("--bench", FlagType::Pass(false)),
    ("--color", FlagType::Pass(true)),
    ("--ensure-time", FlagType::Drop(false)),
    ("--exact", FlagType::Drop(false)),
    ("--exclude-should-panic", FlagType::Pass(false)),
    ("--fail-fast", FlagType::Drop(false)),
    ("--force-run-in-process", FlagType::Pass(false)),
    ("--format", FlagType::Drop(true)),
    ("--help", FlagType::Error("Tests run but --help passed to process?")),
    ("--ignored", FlagType::Pass(false)),
    ("--include-ignored", FlagType::Pass(false)),
    ("--list", FlagType::Error("Tests run but --list passed to process?")),
    ("--logfile", FlagType::Drop(true)),
    ("--nocapture", FlagType::Drop(true)),
    ("--quiet", FlagType::Drop(false)),
    ("--report-time", FlagType::Drop(true)),
    ("--show-output", FlagType::Pass(false)),
    ("--shuffle", FlagType::Drop(false)),
    ("--shuffle-seed", FlagType::Drop(true)),
    ("--skip", FlagType::Drop(true)),
    ("--test", FlagType::Pass(false)),
    ("--test-threads", FlagType::Drop(true)),
    ("-Z", FlagType::Pass(true)),
    ("-h", FlagType::Error("Tests run but -h passed to process?")),
    ("-q", FlagType::Drop(false)),
];

fn look_up_flag_from_table(flag: &str) -> Option<FlagType> {
    KNOWN_FLAGS.iter().cloned().filter(|&(name, _)| name == flag)
        .map(|(_, typ)| typ).next()
}

pub(crate) fn env_var_for_flag(flag: &str) -> String {
    let mut var = "RUSTY_FORK_FLAG_".to_owned();
    var.push_str(
        &flag.trim_start_matches('-').to_uppercase().replace('-', "_"));
    var
}

fn look_up_flag_from_env(flag: &str) -> Option<FlagType> {
    env::var(&env_var_for_flag(flag)).ok().map(
        |value| match &*value {
            "pass" => FlagType::Pass(false),
            "pass-arg" => FlagType::Pass(true),
            "drop" => FlagType::Drop(false),
            "drop-arg" => FlagType::Drop(true),
            _ => FlagType::Error("incorrect flag type in environment; \
                                  must be one of `pass`, `pass-arg`, \
                                  `drop`, `drop-arg`"),
        })
}

fn look_up_flag(flag: &str) -> Option<FlagType> {
    look_up_flag_from_table(flag).or_else(|| look_up_flag_from_env(flag))
}

fn look_up_flag_or_err(flag: &str) -> Result<(bool, bool)> {
    match look_up_flag(flag) {
        None =>
            Err(Error::UnknownFlag(flag.to_owned())),
        Some(FlagType::Error(message)) =>
            Err(Error::DisallowedFlag(flag.to_owned(), message.to_owned())),
        Some(FlagType::Pass(has_arg)) => Ok((true, has_arg)),
        Some(FlagType::Drop(has_arg)) => Ok((false, has_arg)),
    }
}

/// Parse the full command line as would be given to the Rust test harness, and
/// strip out any flags that should be dropped as well as all filters. The
/// resulting argument list is also guaranteed to not have "--", so that new
/// flags can be appended.
///
/// The zeroth argument (the command name) is also dropped.
pub(crate) fn strip_cmdline<A : Iterator<Item = String>>
    (args: A) -> Result<Vec<String>>
{
    #[derive(Clone, Copy)]
    enum State {
        Ground, PassingArg, DroppingArg,
    }

    // Start in DroppingArg since we need to drop the exec name.
    let mut state = State::DroppingArg;
    let mut ret = Vec::new();

    for arg in args {
        match state {
            State::DroppingArg => {
                state = State::Ground;
            },

            State::PassingArg => {
                ret.push(arg);
                state = State::Ground;
            },

            State::Ground => {
                if &arg == "--" {
                    // Everything after this point is a filter
                    break;
                } else if &arg == "-" {
                    // "-" by itself is interpreted as a filter
                    continue;
                } else if arg.starts_with("--") {
                    let (pass, has_arg) = look_up_flag_or_err(
                        arg.split('=').next().expect("split returned empty"))?;
                    // If there's an = sign, the physical argument also
                    // contains the associated value, so don't pay attention to
                    // has_arg.
                    let has_arg = has_arg && !arg.contains('=');
                    if pass {
                        ret.push(arg);
                        if has_arg {
                            state = State::PassingArg;
                        }
                    } else if has_arg {
                        state = State::DroppingArg;
                    }
                } else if arg.starts_with("-") {
                    let mut chars = arg.chars();
                    let mut to_pass = "-".to_owned();

                    chars.next(); // skip initial '-'
                    while let Some(flag_ch) = chars.next() {
                        let flag = format!("-{}", flag_ch);
                        let (pass, has_arg) = look_up_flag_or_err(&flag)?;
                        if pass {
                            to_pass.push(flag_ch);
                            if has_arg {
                                if chars.clone().next().is_some() {
                                    // Arg is attached to this one
                                    to_pass.extend(chars);
                                } else {
                                    // Arg is separate
                                    state = State::PassingArg;
                                }
                                break;
                            }
                        } else if has_arg {
                            if chars.clone().next().is_none() {
                                // Arg is separate
                                state = State::DroppingArg;
                            }
                            break;
                        }
                    }

                    if "-" != &to_pass {
                        ret.push(to_pass);
                    }
                } else {
                    // It's a filter, drop
                }
            },
        }
    }

    Ok(ret)
}

/// Extra arguments to add after the stripped command line when running a
/// single test.
pub(crate) static RUN_TEST_ARGS: &[&str] = &[
    // --quiet because the test runner output is redundant
    "--quiet",
    // Single threaded because we get parallelism from the parent process
    "--test-threads", "1",
    // Disable capture since we want the output to be captured by the *parent*
    // process.
    "--nocapture",
    // Match our test filter exactly so we run exactly one test
    "--exact",
    // Ensure everything else is interpreted as filters
    "--",
];

#[cfg(test)]
mod test {
    use super::*;

    fn strip(cmdline: &str) -> Result<String> {
        strip_cmdline(cmdline.split_whitespace().map(|s| s.to_owned()))
            .map(|strs| strs.join(" "))
    }

    #[test]
    fn test_strip() {
        assert_eq!("", &strip("test").unwrap());
        assert_eq!("--ignored", &strip("test --ignored").unwrap());
        assert_eq!("", &strip("test --quiet").unwrap());
        assert_eq!("", &strip("test -q").unwrap());
        assert_eq!("", &strip("test -qq").unwrap());
        assert_eq!("", &strip("test --test-threads 42").unwrap());
        assert_eq!("-Z unstable-options",
                   &strip("test -Z unstable-options").unwrap());
        assert_eq!("-Zunstable-options",
                   &strip("test -Zunstable-options").unwrap());
        assert_eq!("-Zunstable-options",
                   &strip("test -qZunstable-options").unwrap());
        assert_eq!("--color auto", &strip("test --color auto").unwrap());
        assert_eq!("--color=auto", &strip("test --color=auto").unwrap());
        assert_eq!("", &strip("test filter filter2").unwrap());
        assert_eq!("", &strip("test -- --color=auto").unwrap());

        match strip("test --plugh").unwrap_err() {
            Error::UnknownFlag(ref flag) => assert_eq!("--plugh", flag),
            e => panic!("Unexpected error: {}", e),
        }
        match strip("test --help").unwrap_err() {
            Error::DisallowedFlag(ref flag, _) => assert_eq!("--help", flag),
            e => panic!("Unexpected error: {}", e),
        }
    }

    // Subprocess so we can change the environment without affecting other
    // tests
    rusty_fork_test! {
        #[test]
        fn define_args_via_env() {
            env::set_var("RUSTY_FORK_FLAG_X", "pass");
            env::set_var("RUSTY_FORK_FLAG_FOO", "pass-arg");
            env::set_var("RUSTY_FORK_FLAG_BAR", "drop");
            env::set_var("RUSTY_FORK_FLAG_BAZ", "drop-arg");

            assert_eq!("-X", &strip("test -X foo").unwrap());
            assert_eq!("--foo bar", &strip("test --foo bar").unwrap());
            assert_eq!("", &strip("test --bar").unwrap());
            assert_eq!("", &strip("test --baz --notaflag").unwrap());
        }
    }
}
