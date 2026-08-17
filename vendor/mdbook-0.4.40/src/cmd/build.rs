use super::command_prelude::*;
use crate::{get_book_dir, open};
use mdbook::errors::Result;
use mdbook::MDBook;
use std::path::PathBuf;

// Create clap subcommand arguments
pub fn make_subcommand() -> Command {
    Command::new("build")
        .about("Builds a book from its markdown files")
        .arg_dest_dir()
        .arg_root_dir()
        .arg_open()
}

// Build command implementation
pub fn execute(args: &ArgMatches) -> Result<()> {
    let book_dir = get_book_dir(args);
    let mut book = MDBook::load(book_dir)?;

    if let Some(dest_dir) = args.get_one::<PathBuf>("dest-dir") {
        book.config.build.build_dir = dest_dir.into();
    }

    book.build()?;

    if args.get_flag("open") {
        // FIXME: What's the right behaviour if we don't use the HTML renderer?
        let path = book.build_dir_for("html").join("index.html");
        if !path.exists() {
            error!("No chapter available to open");
            std::process::exit(1)
        }
        open(path);
    }

    Ok(())
}
