use clap::{Arg, ArgAction, Command, ValueHint, builder::PossibleValue};
use clap_complete::generate;
use clap_complete_nushell::Nushell;
use std::io;

fn main() {
    let mut cmd = Command::new("myapp")
        .version("3.0")
        .propagate_version(true)
        .about("Tests completions")
        .arg(
            Arg::new("file")
                .value_hint(ValueHint::FilePath)
                .help("some input file"),
        )
        .arg(
            Arg::new("config")
                .action(ArgAction::Count)
                .help("some config file")
                .short('c')
                .visible_short_alias('C')
                .long("config")
                .visible_alias("conf"),
        )
        .arg(Arg::new("choice").value_parser(["first", "second"]))
        .subcommand(
            Command::new("test").about("tests things").arg(
                Arg::new("case")
                    .long("case")
                    .action(ArgAction::Set)
                    .help("the case to test"),
            ),
        )
        .subcommand(
            Command::new("some_cmd")
                .about("top level subcommand")
                .subcommand(
                    Command::new("sub_cmd").about("sub-subcommand").arg(
                        Arg::new("config")
                            .long("config")
                            .action(ArgAction::Set)
                            .value_parser([PossibleValue::new("Lest quotes aren't escaped.")])
                            .help("the other case to test"),
                    ),
                ),
        );

    generate(Nushell, &mut cmd, "myapp", &mut io::stdout());
}
