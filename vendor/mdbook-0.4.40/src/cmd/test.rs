use super::command_prelude::*;
use crate::get_book_dir;
use clap::builder::NonEmptyStringValueParser;
use clap::ArgAction;
use mdbook::errors::Result;
use mdbook::MDBook;
use std::path::PathBuf;

// Create clap subcommand arguments
pub fn make_subcommand() -> Command {
    Command::new("test")
        .about("Tests that a book's Rust code samples compile")
        // FIXME: --dest-dir is unused by the test command, it should be removed
        .arg_dest_dir()
        .arg_root_dir()
        .arg(
            Arg::new("chapter")
                .short('c')
                .long("chapter")
                .value_name("chapter"),
        )
        .arg(
            Arg::new("library-path")
                .short('L')
                .long("library-path")
                .value_name("dir")
                .value_delimiter(',')
                .value_parser(NonEmptyStringValueParser::new())
                .action(ArgAction::Append)
                .help(
                    "A comma-separated list of directories to add to the crate \
                    search path when building tests",
                ),
        )
}

// test command implementation
pub fn execute(args: &ArgMatches) -> Result<()> {
    let library_paths: Vec<&str> = args
        .get_many("library-path")
        .map(|it| it.map(String::as_str).collect())
        .unwrap_or_default();

    let chapter: Option<&str> = args.get_one::<String>("chapter").map(|s| s.as_str());

    let book_dir = get_book_dir(args);
    let mut book = MDBook::load(book_dir)?;

    if let Some(dest_dir) = args.get_one::<PathBuf>("dest-dir") {
        book.config.build.build_dir = dest_dir.to_path_buf();
    }
    match chapter {
        Some(_) => book.test_chapter(library_paths, chapter),
        None => book.test(library_paths),
    }?;

    Ok(())
}
