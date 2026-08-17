use clap_complete::generate;
use clap_complete_nushell::Nushell;

fn main() {
    let matches = cli().get_matches();
    if matches.contains_id("generate") {
        let mut cmd = cli();
        generate(Nushell, &mut cmd, "test", &mut std::io::stdout());
    } else {
        println!("{matches:?}");
    }
}

fn cli() -> clap::Command {
    clap::Command::new("test")
        .version("3.0")
        .propagate_version(true)
        .args([
            clap::Arg::new("global")
                .long("global")
                .global(true)
                .action(clap::ArgAction::SetTrue)
                .help("everywhere"),
            clap::Arg::new("generate").long("generate").help("generate"),
        ])
        .subcommands([
            clap::Command::new("action").args([
                clap::Arg::new("set-true")
                    .long("set-true")
                    .action(clap::ArgAction::SetTrue)
                    .help("bool"),
                clap::Arg::new("set")
                    .long("set")
                    .action(clap::ArgAction::Set)
                    .help("value"),
                clap::Arg::new("count")
                    .long("count")
                    .action(clap::ArgAction::Count)
                    .help("number"),
                clap::Arg::new("choice")
                    .long("choice")
                    .value_parser(["first", "second"])
                    .help("enum"),
            ]),
            clap::Command::new("quote")
                .args([
                    clap::Arg::new("single-quotes")
                        .long("single-quotes")
                        .action(clap::ArgAction::SetTrue)
                        .help("Can be 'always', 'auto', or 'never'"),
                    clap::Arg::new("double-quotes")
                        .long("double-quotes")
                        .action(clap::ArgAction::SetTrue)
                        .help("Can be \"always\", \"auto\", or \"never\""),
                    clap::Arg::new("backticks")
                        .long("backticks")
                        .action(clap::ArgAction::SetTrue)
                        .help("For more information see `echo test`"),
                    clap::Arg::new("backslash")
                        .long("backslash")
                        .action(clap::ArgAction::SetTrue)
                        .help("Avoid '\\n'"),
                    clap::Arg::new("brackets")
                        .long("brackets")
                        .action(clap::ArgAction::SetTrue)
                        .help("List packages [filter]"),
                    clap::Arg::new("expansions")
                        .long("expansions")
                        .action(clap::ArgAction::SetTrue)
                        .help("Execute the shell command with $SHELL"),
                ])
                .subcommands([
                    clap::Command::new("cmd-single-quotes")
                        .about("Can be 'always', 'auto', or 'never'"),
                    clap::Command::new("cmd-double-quotes")
                        .about("Can be \"always\", \"auto\", or \"never\""),
                    clap::Command::new("cmd-backticks")
                        .about("For more information see `echo test`"),
                    clap::Command::new("cmd-backslash").about("Avoid '\\n'"),
                    clap::Command::new("cmd-brackets").about("List packages [filter]"),
                    clap::Command::new("cmd-expansions")
                        .about("Execute the shell command with $SHELL"),
                ]),
            clap::Command::new("value").args([
                clap::Arg::new("delim").long("delim").value_delimiter(','),
                clap::Arg::new("tuple").long("tuple").num_args(2),
                clap::Arg::new("require-eq")
                    .long("require-eq")
                    .require_equals(true),
                clap::Arg::new("term").num_args(1..).value_terminator(";"),
            ]),
            clap::Command::new("pacman").subcommands([
                clap::Command::new("one").long_flag("one").short_flag('o'),
                clap::Command::new("two").long_flag("two").short_flag('t'),
            ]),
            clap::Command::new("last")
                .args([clap::Arg::new("first"), clap::Arg::new("free").last(true)]),
            clap::Command::new("alias").args([
                clap::Arg::new("flag")
                    .short('f')
                    .visible_short_alias('F')
                    .long("flag")
                    .action(clap::ArgAction::SetTrue)
                    .visible_alias("flg")
                    .help("cmd flag"),
                clap::Arg::new("option")
                    .short('o')
                    .visible_short_alias('O')
                    .long("option")
                    .visible_alias("opt")
                    .help("cmd option")
                    .action(clap::ArgAction::Set),
                clap::Arg::new("positional"),
            ]),
            clap::Command::new("hint").args([
                clap::Arg::new("choice")
                    .long("choice")
                    .action(clap::ArgAction::Set)
                    .value_parser(["bash", "fish", "zsh"]),
                clap::Arg::new("unknown")
                    .long("unknown")
                    .value_hint(clap::ValueHint::Unknown),
                clap::Arg::new("other")
                    .long("other")
                    .value_hint(clap::ValueHint::Other),
                clap::Arg::new("path")
                    .long("path")
                    .short('p')
                    .value_hint(clap::ValueHint::AnyPath),
                clap::Arg::new("file")
                    .long("file")
                    .short('f')
                    .value_hint(clap::ValueHint::FilePath),
                clap::Arg::new("dir")
                    .long("dir")
                    .short('d')
                    .value_hint(clap::ValueHint::DirPath),
                clap::Arg::new("exe")
                    .long("exe")
                    .short('e')
                    .value_hint(clap::ValueHint::ExecutablePath),
                clap::Arg::new("cmd_name")
                    .long("cmd-name")
                    .value_hint(clap::ValueHint::CommandName),
                clap::Arg::new("cmd")
                    .long("cmd")
                    .short('c')
                    .value_hint(clap::ValueHint::CommandString),
                clap::Arg::new("command_with_args")
                    .action(clap::ArgAction::Set)
                    .num_args(1..)
                    .trailing_var_arg(true)
                    .value_hint(clap::ValueHint::CommandWithArguments),
                clap::Arg::new("user")
                    .short('u')
                    .long("user")
                    .value_hint(clap::ValueHint::Username),
                clap::Arg::new("host")
                    .short('H')
                    .long("host")
                    .value_hint(clap::ValueHint::Hostname),
                clap::Arg::new("url")
                    .long("url")
                    .value_hint(clap::ValueHint::Url),
                clap::Arg::new("email")
                    .long("email")
                    .value_hint(clap::ValueHint::EmailAddress),
            ]),
        ])
}
