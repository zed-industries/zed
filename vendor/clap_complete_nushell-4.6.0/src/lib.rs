//! Generates [Nushell](https://github.com/nushell/nushell) completions for [`clap`](https://github.com/clap-rs/clap) based CLIs
//!
//! ## Example
//!
//! ```
//! use clap::Command;
//! use clap_complete::generate;
//! use clap_complete_nushell::Nushell;
//! use std::io;
//!
//! let mut cmd = Command::new("myapp")
//!     .subcommand(Command::new("test").subcommand(Command::new("config")))
//!     .subcommand(Command::new("hello"));
//!
//! generate(Nushell, &mut cmd, "myapp", &mut io::stdout());
//! ```

#![doc(html_logo_url = "https://raw.githubusercontent.com/clap-rs/clap/master/assets/clap.png")]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![forbid(unsafe_code)]
#![warn(missing_docs)]
#![warn(clippy::print_stderr)]
#![warn(clippy::print_stdout)]

use clap::ValueHint;
use clap::builder::StyledStr;
use clap::{Arg, ArgAction, Command, builder::PossibleValue};
use clap_complete::Generator;

/// Generate Nushell complete file
pub struct Nushell;

impl Generator for Nushell {
    fn file_name(&self, name: &str) -> String {
        format!("{name}.nu")
    }

    fn generate(&self, cmd: &Command, buf: &mut dyn std::io::Write) {
        self.try_generate(cmd, buf)
            .expect("failed to write completion file");
    }

    fn try_generate(
        &self,
        cmd: &Command,
        buf: &mut dyn std::io::Write,
    ) -> Result<(), std::io::Error> {
        let mut completions = String::new();

        completions.push_str("module completions {\n\n");

        generate_completion(&mut completions, cmd, false);

        for sub in cmd.get_subcommands() {
            generate_completion(&mut completions, sub, true);
        }

        completions.push_str("}\n\n");
        completions.push_str("export use completions *\n");

        buf.write_all(completions.as_bytes())
    }
}

fn append_value_completion_and_help(
    arg: &Arg,
    name: &str,
    possible_values: &[PossibleValue],
    s: &mut String,
) {
    let takes_values = arg
        .get_num_args()
        .map(|r| r.takes_values())
        .unwrap_or(false);

    if takes_values {
        let nu_type = match arg.get_value_hint() {
            ValueHint::Unknown => "string",
            ValueHint::Other => "string",
            ValueHint::AnyPath => "path",
            ValueHint::FilePath => "path",
            ValueHint::DirPath => "path",
            ValueHint::ExecutablePath => "path",
            ValueHint::CommandName => "string",
            ValueHint::CommandString => "string",
            ValueHint::CommandWithArguments => "string",
            ValueHint::Username => "string",
            ValueHint::Hostname => "string",
            ValueHint::Url => "string",
            ValueHint::EmailAddress => "string",
            _ => "string",
        };
        s.push_str(format!(": {nu_type}").as_str());

        if !possible_values.is_empty() {
            s.push_str(format!(r#"@"nu-complete {} {}""#, name, arg.get_id()).as_str());
        }
    }

    if let Some(help) = arg.get_help() {
        let indent: usize = 30;
        let width = match s.lines().last() {
            Some(line) => indent.saturating_sub(line.len()),
            None => 0,
        };

        s.push_str(format!("{:>width$}# {}", ' ', single_line_styled_str(help)).as_str());
    }

    s.push('\n');
}

fn append_value_completion_defs(arg: &Arg, name: &str, s: &mut String) {
    let possible_values = arg.get_possible_values();
    if possible_values.is_empty() {
        return;
    }

    s.push_str(format!(r#"  def "nu-complete {} {}" [] {{"#, name, arg.get_id()).as_str());
    s.push_str("\n    [");

    for value in possible_values {
        let vname = value.get_name();
        if vname.contains(|c: char| c.is_whitespace()) {
            s.push_str(format!(r#" "\"{vname}\"""#).as_str());
        } else {
            s.push_str(format!(r#" "{vname}""#).as_str());
        }
    }

    s.push_str(" ]\n  }\n\n");
}

fn append_argument(arg: &Arg, name: &str, s: &mut String) {
    let possible_values = arg.get_possible_values();

    if arg.is_positional() {
        // rest arguments
        if matches!(arg.get_action(), ArgAction::Append) {
            s.push_str(format!("    ...{}", arg.get_id()).as_str());
        } else {
            s.push_str(format!("    {}", arg.get_id()).as_str());

            if !arg.is_required_set() {
                s.push('?');
            }
        }

        append_value_completion_and_help(arg, name, &possible_values, s);

        return;
    }

    let shorts = arg.get_short_and_visible_aliases();
    let longs = arg.get_long_and_visible_aliases();

    match shorts {
        Some(shorts) => match longs {
            Some(longs) => {
                // short options and long options
                s.push_str(
                    format!(
                        "    --{}(-{})",
                        longs.first().expect("At least one long option expected"),
                        shorts.first().expect("At lease one short option expected")
                    )
                    .as_str(),
                );
                append_value_completion_and_help(arg, name, &possible_values, s);

                // long alias
                for long in longs.iter().skip(1) {
                    s.push_str(format!("    --{long}").as_str());
                    append_value_completion_and_help(arg, name, &possible_values, s);
                }

                // short alias
                for short in shorts.iter().skip(1) {
                    s.push_str(format!("    -{short}").as_str());
                    append_value_completion_and_help(arg, name, &possible_values, s);
                }
            }
            None => {
                // short options only
                for short in shorts {
                    s.push_str(format!("    -{short}").as_str());
                    append_value_completion_and_help(arg, name, &possible_values, s);
                }
            }
        },
        None => match longs {
            Some(longs) => {
                // long options only
                for long in longs {
                    s.push_str(format!("    --{long}").as_str());
                    append_value_completion_and_help(arg, name, &possible_values, s);
                }
            }
            None => unreachable!("No short or long options found"),
        },
    }
}

fn generate_completion(completions: &mut String, cmd: &Command, is_subcommand: bool) {
    let name = cmd.get_bin_name().expect("Failed to get bin name");

    for arg in cmd.get_arguments() {
        append_value_completion_defs(arg, name, completions);
    }

    if let Some(about) = cmd.get_about() {
        let about = single_line_styled_str(about);
        completions.push_str(format!("  # {about}\n").as_str());
    }

    if is_subcommand {
        completions.push_str(format!("  export extern \"{name}\" [\n").as_str());
    } else {
        completions.push_str(format!("  export extern {name} [\n").as_str());
    }

    let flags: Vec<_> = cmd.get_arguments().filter(|a| !a.is_positional()).collect();
    let mut positionals: Vec<_> = cmd.get_positionals().collect();

    positionals.sort_by_key(|arg| arg.get_index());

    for arg in flags {
        append_argument(arg, name, completions);
    }

    for arg in positionals {
        append_argument(arg, name, completions);
    }

    completions.push_str("  ]\n\n");

    if is_subcommand {
        for sub in cmd.get_subcommands() {
            generate_completion(completions, sub, true);
        }
    }
}

fn single_line_styled_str(text: &StyledStr) -> String {
    text.to_string().replace('\n', " ")
}

#[doc = include_str!("../README.md")]
#[cfg(doctest)]
pub struct ReadmeDoctests;
