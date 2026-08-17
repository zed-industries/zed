//! A filesystem watcher using native operating system facilities.

use ignore::gitignore::Gitignore;
use mdbook::MDBook;
use std::path::{Path, PathBuf};
use std::sync::mpsc::channel;
use std::thread::sleep;
use std::time::Duration;

pub fn rebuild_on_change(
    book_dir: &Path,
    update_config: &dyn Fn(&mut MDBook),
    post_build: &dyn Fn(),
) {
    use notify::RecursiveMode::*;

    let mut book = MDBook::load(book_dir).unwrap_or_else(|e| {
        error!("failed to load book: {e}");
        std::process::exit(1);
    });

    // Create a channel to receive the events.
    let (tx, rx) = channel();

    let mut debouncer = match notify_debouncer_mini::new_debouncer(Duration::from_secs(1), tx) {
        Ok(d) => d,
        Err(e) => {
            error!("Error while trying to watch the files:\n\n\t{:?}", e);
            std::process::exit(1)
        }
    };
    let watcher = debouncer.watcher();

    // Add the source directory to the watcher
    if let Err(e) = watcher.watch(&book.source_dir(), Recursive) {
        error!("Error while watching {:?}:\n    {:?}", book.source_dir(), e);
        std::process::exit(1);
    };

    let _ = watcher.watch(&book.theme_dir(), Recursive);

    // Add the book.toml file to the watcher if it exists
    let _ = watcher.watch(&book.root.join("book.toml"), NonRecursive);

    for dir in &book.config.build.extra_watch_dirs {
        let path = book.root.join(dir);
        let canonical_path = path.canonicalize().unwrap_or_else(|e| {
            error!("Error while watching extra directory {path:?}:\n    {e}");
            std::process::exit(1);
        });

        if let Err(e) = watcher.watch(&canonical_path, Recursive) {
            error!(
                "Error while watching extra directory {:?}:\n    {:?}",
                canonical_path, e
            );
            std::process::exit(1);
        }
    }

    info!("Listening for changes...");

    loop {
        let first_event = rx.recv().unwrap();
        sleep(Duration::from_millis(50));
        let other_events = rx.try_iter();

        let all_events = std::iter::once(first_event).chain(other_events);

        let paths: Vec<_> = all_events
            .filter_map(|event| match event {
                Ok(events) => Some(events),
                Err(error) => {
                    log::warn!("error while watching for changes: {error}");
                    None
                }
            })
            .flatten()
            .map(|event| event.path)
            .collect();

        // If we are watching files outside the current repository (via extra-watch-dirs), then they are definitionally
        // ignored by gitignore. So we handle this case by including such files into the watched paths list.
        let any_external_paths = paths.iter().filter(|p| !p.starts_with(&book.root)).cloned();
        let mut paths = remove_ignored_files(&book.root, &paths[..]);
        paths.extend(any_external_paths);

        if !paths.is_empty() {
            info!("Files changed: {paths:?}");
            match MDBook::load(book_dir) {
                Ok(mut b) => {
                    update_config(&mut b);
                    if let Err(e) = b.build() {
                        error!("failed to build the book: {e:?}");
                    } else {
                        post_build();
                    }
                    book = b;
                }
                Err(e) => error!("failed to load book config: {e:?}"),
            }
        }
    }
}

fn remove_ignored_files(book_root: &Path, paths: &[PathBuf]) -> Vec<PathBuf> {
    if paths.is_empty() {
        return vec![];
    }

    match super::find_gitignore(book_root) {
        Some(gitignore_path) => {
            let (ignore, err) = Gitignore::new(&gitignore_path);
            if let Some(err) = err {
                warn!(
                    "error reading gitignore `{}`: {err}",
                    gitignore_path.display()
                );
            }
            filter_ignored_files(ignore, paths)
        }
        None => {
            // There is no .gitignore file.
            paths.iter().map(|path| path.to_path_buf()).collect()
        }
    }
}

// Note: The usage of `canonicalize` may encounter occasional failures on the Windows platform, presenting a potential risk.
// For more details, refer to [Pull Request #2229](https://github.com/rust-lang/mdBook/pull/2229#discussion_r1408665981).
fn filter_ignored_files(ignore: Gitignore, paths: &[PathBuf]) -> Vec<PathBuf> {
    let ignore_root = ignore
        .path()
        .canonicalize()
        .expect("ignore root canonicalize error");

    paths
        .iter()
        .filter(|path| {
            let relative_path = pathdiff::diff_paths(&path, &ignore_root)
                .expect("One of the paths should be an absolute");
            !ignore
                .matched_path_or_any_parents(&relative_path, relative_path.is_dir())
                .is_ignore()
        })
        .map(|path| path.to_path_buf())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use ignore::gitignore::GitignoreBuilder;
    use std::env;

    #[test]
    fn test_filter_ignored_files() {
        let current_dir = env::current_dir().unwrap();

        let ignore = GitignoreBuilder::new(&current_dir)
            .add_line(None, "*.html")
            .unwrap()
            .build()
            .unwrap();
        let should_remain = current_dir.join("record.text");
        let should_filter = current_dir.join("index.html");

        let remain = filter_ignored_files(ignore, &[should_remain.clone(), should_filter]);
        assert_eq!(remain, vec![should_remain])
    }

    #[test]
    fn filter_ignored_files_should_handle_parent_dir() {
        let current_dir = env::current_dir().unwrap();

        let ignore = GitignoreBuilder::new(&current_dir)
            .add_line(None, "*.html")
            .unwrap()
            .build()
            .unwrap();

        let parent_dir = current_dir.join("..");
        let should_remain = parent_dir.join("record.text");
        let should_filter = parent_dir.join("index.html");

        let remain = filter_ignored_files(ignore, &[should_remain.clone(), should_filter]);
        assert_eq!(remain, vec![should_remain])
    }
}
