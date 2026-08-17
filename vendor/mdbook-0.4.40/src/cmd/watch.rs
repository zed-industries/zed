use super::command_prelude::*;
use crate::{get_book_dir, open};
use mdbook::errors::Result;
use mdbook::MDBook;
use std::path::{Path, PathBuf};

mod native;
mod poller;

// Create clap subcommand arguments
pub fn make_subcommand() -> Command {
    Command::new("watch")
        .about("Watches a book's files and rebuilds it on changes")
        .arg_dest_dir()
        .arg_root_dir()
        .arg_open()
        .arg_watcher()
}

pub enum WatcherKind {
    Poll,
    Native,
}

impl WatcherKind {
    pub fn from_str(s: &str) -> WatcherKind {
        match s {
            "poll" => WatcherKind::Poll,
            "native" => WatcherKind::Native,
            _ => panic!("unsupported watcher {s}"),
        }
    }
}

// Watch command implementation
pub fn execute(args: &ArgMatches) -> Result<()> {
    let book_dir = get_book_dir(args);
    let mut book = MDBook::load(&book_dir)?;

    let update_config = |book: &mut MDBook| {
        if let Some(dest_dir) = args.get_one::<PathBuf>("dest-dir") {
            book.config.build.build_dir = dest_dir.into();
        }
    };
    update_config(&mut book);

    if args.get_flag("open") {
        book.build()?;
        let path = book.build_dir_for("html").join("index.html");
        if !path.exists() {
            error!("No chapter available to open");
            std::process::exit(1)
        }
        open(path);
    }

    let watcher = WatcherKind::from_str(args.get_one::<String>("watcher").unwrap());
    rebuild_on_change(watcher, &book_dir, &update_config, &|| {});

    Ok(())
}

pub fn rebuild_on_change(
    kind: WatcherKind,
    book_dir: &Path,
    update_config: &dyn Fn(&mut MDBook),
    post_build: &dyn Fn(),
) {
    match kind {
        WatcherKind::Poll => self::poller::rebuild_on_change(book_dir, update_config, post_build),
        WatcherKind::Native => self::native::rebuild_on_change(book_dir, update_config, post_build),
    }
}

fn find_gitignore(book_root: &Path) -> Option<PathBuf> {
    book_root
        .ancestors()
        .map(|p| p.join(".gitignore"))
        .find(|p| p.exists())
}
