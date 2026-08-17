use super::command_prelude::*;
use crate::get_book_dir;
use anyhow::Context;
use mdbook::MDBook;
use std::fs;
use std::path::PathBuf;

// Create clap subcommand arguments
pub fn make_subcommand() -> Command {
    Command::new("clean")
        .about("Deletes a built book")
        .arg_dest_dir()
        .arg_root_dir()
}

// Clean command implementation
pub fn execute(args: &ArgMatches) -> mdbook::errors::Result<()> {
    let book_dir = get_book_dir(args);
    let book = MDBook::load(book_dir)?;

    let dir_to_remove = match args.get_one::<PathBuf>("dest-dir") {
        Some(dest_dir) => dest_dir.into(),
        None => book.root.join(&book.config.build.build_dir),
    };

    if dir_to_remove.exists() {
        fs::remove_dir_all(&dir_to_remove)
            .with_context(|| "Unable to remove the build directory")?;
    }

    Ok(())
}
