#[macro_use]
extern crate clap;
#[macro_use]
extern crate log;

use anyhow::anyhow;
use chrono::Local;
use clap::{Arg, ArgMatches, Command};
use clap_complete::Shell;
use env_logger::Builder;
use log::LevelFilter;
use mdbook::utils;
use std::env;
use std::ffi::OsStr;
use std::io::Write;
use std::path::PathBuf;

mod cmd;

const VERSION: &str = concat!("v", crate_version!());

fn main() {
    init_logger();

    let command = create_clap_command();

    // Check which subcommand the user ran...
    let res = match command.get_matches().subcommand() {
        Some(("init", sub_matches)) => cmd::init::execute(sub_matches),
        Some(("build", sub_matches)) => cmd::build::execute(sub_matches),
        Some(("clean", sub_matches)) => cmd::clean::execute(sub_matches),
        #[cfg(feature = "watch")]
        Some(("watch", sub_matches)) => cmd::watch::execute(sub_matches),
        #[cfg(feature = "serve")]
        Some(("serve", sub_matches)) => cmd::serve::execute(sub_matches),
        Some(("test", sub_matches)) => cmd::test::execute(sub_matches),
        Some(("completions", sub_matches)) => (|| {
            let shell = sub_matches
                .get_one::<Shell>("shell")
                .ok_or_else(|| anyhow!("Shell name missing."))?;

            let mut complete_app = create_clap_command();
            clap_complete::generate(
                *shell,
                &mut complete_app,
                "mdbook",
                &mut std::io::stdout().lock(),
            );
            Ok(())
        })(),
        _ => unreachable!(),
    };

    if let Err(e) = res {
        utils::log_backtrace(&e);

        std::process::exit(101);
    }
}

/// Create a list of valid arguments and sub-commands
fn create_clap_command() -> Command {
    let app = Command::new(crate_name!())
        .about(crate_description!())
        .author("Mathieu David <mathieudavid@mathieudavid.org>")
        .version(VERSION)
        .propagate_version(true)
        .arg_required_else_help(true)
        .after_help(
            "For more information about a specific command, try `mdbook <command> --help`\n\
             The source code for mdBook is available at: https://github.com/rust-lang/mdBook",
        )
        .subcommand(cmd::init::make_subcommand())
        .subcommand(cmd::build::make_subcommand())
        .subcommand(cmd::test::make_subcommand())
        .subcommand(cmd::clean::make_subcommand())
        .subcommand(
            Command::new("completions")
                .about("Generate shell completions for your shell to stdout")
                .arg(
                    Arg::new("shell")
                        .value_parser(clap::value_parser!(Shell))
                        .help("the shell to generate completions for")
                        .value_name("SHELL")
                        .required(true),
                ),
        );

    #[cfg(feature = "watch")]
    let app = app.subcommand(cmd::watch::make_subcommand());
    #[cfg(feature = "serve")]
    let app = app.subcommand(cmd::serve::make_subcommand());

    app
}

fn init_logger() {
    let mut builder = Builder::new();

    builder.format(|formatter, record| {
        writeln!(
            formatter,
            "{} [{}] ({}): {}",
            Local::now().format("%Y-%m-%d %H:%M:%S"),
            record.level(),
            record.target(),
            record.args()
        )
    });

    if let Ok(var) = env::var("RUST_LOG") {
        builder.parse_filters(&var);
    } else {
        // if no RUST_LOG provided, default to logging at the Info level
        builder.filter(None, LevelFilter::Info);
        // Filter extraneous html5ever not-implemented messages
        builder.filter(Some("html5ever"), LevelFilter::Error);
    }

    builder.init();
}

fn get_book_dir(args: &ArgMatches) -> PathBuf {
    if let Some(p) = args.get_one::<PathBuf>("dir") {
        // Check if path is relative from current dir, or absolute...
        if p.is_relative() {
            env::current_dir().unwrap().join(p)
        } else {
            p.to_path_buf()
        }
    } else {
        env::current_dir().expect("Unable to determine the current directory")
    }
}

fn open<P: AsRef<OsStr>>(path: P) {
    info!("Opening web browser");
    if let Err(e) = opener::open(path) {
        error!("Error opening web browser: {}", e);
    }
}

#[test]
fn verify_app() {
    create_clap_command().debug_assert();
}
