use anyhow::{Context, Result};
use clap::{Arg, ArgMatches, Command};
use docs_preprocessor::ZedDocsPreprocessor;
use mdbook::preprocess::{CmdPreprocessor, Preprocessor};
use std::io::{self, Read};
use std::process;

pub fn make_app() -> Command {
    Command::new("zed-docs-preprocessor")
        .about("Preprocesses Zed Docs content to provide rich action & keybinding support and more")
        .subcommand(
            Command::new("supports")
                .arg(Arg::new("renderer").required(true))
                .about("Check whether a renderer is supported by this preprocessor"),
        )
}

fn main() -> Result<()> {
    let matches = make_app().get_matches();

    let preprocessor =
        ZedDocsPreprocessor::new().context("Failed to create ZedDocsPreprocessor")?;

    if let Some(sub_args) = matches.subcommand_matches("supports") {
        handle_supports(&preprocessor, sub_args);
    } else {
        handle_preprocessing(&preprocessor)?;
    }

    Ok(())
}

fn handle_preprocessing(pre: &dyn Preprocessor) -> Result<()> {
    let mut stdin = io::stdin();
    let mut input = String::new();
    stdin.read_to_string(&mut input)?;

    let (ctx, book) = CmdPreprocessor::parse_input(input.as_bytes())?;

    let processed_book = pre.run(&ctx, book)?;

    serde_json::to_writer(io::stdout(), &processed_book)?;

    Ok(())
}

fn handle_supports(pre: &dyn Preprocessor, sub_args: &ArgMatches) -> ! {
    let renderer = sub_args
        .get_one::<String>("renderer")
        .expect("Required argument");
    let supported = pre.supports_renderer(renderer);

    if supported {
        process::exit(0);
    } else {
        process::exit(1);
    }
}
