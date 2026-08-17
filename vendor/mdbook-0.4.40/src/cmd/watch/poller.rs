//! A simple poll-based filesystem watcher.
//!
//! This exists because the native change notifications have historically had
//! lots of problems. Various operating systems and different filesystems have
//! had problems correctly reporting changes.

use ignore::gitignore::Gitignore;
use mdbook::MDBook;
use pathdiff::diff_paths;
use std::collections::HashMap;
use std::fs::FileType;
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant, SystemTime};
use walkdir::WalkDir;

/// Calls the closure when a book source file is changed, blocking indefinitely.
pub fn rebuild_on_change(
    book_dir: &Path,
    update_config: &dyn Fn(&mut MDBook),
    post_build: &dyn Fn(),
) {
    let mut book = MDBook::load(book_dir).unwrap_or_else(|e| {
        error!("failed to load book: {e}");
        std::process::exit(1);
    });

    let mut watcher = Watcher::new(book_dir);

    info!("Watching for changes...");
    // Scan once to initialize the starting point.
    watcher.set_roots(&book);
    watcher.scan();

    // Track average scan time, to help investigate if the poller is taking
    // undesirably long. This is not a rigorous benchmark, just a rough
    // estimate.
    const AVG_SIZE: usize = 60;
    let mut avgs = vec![0.0; AVG_SIZE];
    let mut avg_i = 0;

    loop {
        std::thread::sleep(Duration::new(1, 0));
        watcher.set_roots(&book);
        let start = Instant::now();
        let paths = watcher.scan();
        let elapsed = start.elapsed().as_secs_f64();
        avgs[avg_i] = elapsed;
        avg_i += 1;
        if avg_i >= AVG_SIZE {
            avg_i = 0;
            let avg = avgs.iter().sum::<f64>() / (avgs.len() as f64);
            trace!(
                "scan average time: {avg:.2}s, scan size is {}",
                watcher.path_data.len()
            );
        }

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

#[derive(PartialEq)]
struct PathData {
    file_type: FileType,
    mtime: SystemTime,
    size: u64,
}

/// A very simple poll-watcher that scans for modified files.
#[derive(Default)]
struct Watcher {
    /// The root paths where it will recursively scan for changes.
    root_paths: Vec<PathBuf>,
    /// Data about files on disk.
    path_data: HashMap<PathBuf, PathData>,
    /// Filters paths that will be watched.
    ignore: Option<(PathBuf, Gitignore)>,
}

impl Watcher {
    fn new(book_root: &Path) -> Watcher {
        // FIXME: ignore should be reloaded when it changes.
        let ignore = super::find_gitignore(book_root).map(|gitignore_path| {
            let (ignore, err) = Gitignore::new(&gitignore_path);
            if let Some(err) = err {
                warn!(
                    "error reading gitignore `{}`: {err}",
                    gitignore_path.display()
                );
            }
            // Note: The usage of `canonicalize` may encounter occasional
            // failures on the Windows platform, presenting a potential risk.
            // For more details, refer to [Pull Request
            // #2229](https://github.com/rust-lang/mdBook/pull/2229#discussion_r1408665981).
            let ignore_path = ignore
                .path()
                .canonicalize()
                .expect("ignore root canonicalize error");
            (ignore_path, ignore)
        });

        Watcher {
            ignore,
            ..Default::default()
        }
    }

    /// Sets the root directories where scanning will start.
    fn set_roots(&mut self, book: &MDBook) {
        let mut root_paths = vec![
            book.source_dir(),
            book.theme_dir(),
            book.root.join("book.toml"),
        ];
        root_paths.extend(
            book.config
                .build
                .extra_watch_dirs
                .iter()
                .map(|path| book.root.join(path)),
        );
        if let Some(html_config) = book.config.html_config() {
            root_paths.extend(
                html_config
                    .additional_css
                    .iter()
                    .chain(html_config.additional_js.iter())
                    .map(|path| book.root.join(path)),
            );
        }

        self.root_paths = root_paths;
    }

    /// Scans for changes.
    ///
    /// Returns the paths that have changed.
    fn scan(&mut self) -> Vec<PathBuf> {
        let ignore = &self.ignore;
        let new_path_data: HashMap<_, _> = self
            .root_paths
            .iter()
            .filter(|root| root.exists())
            .flat_map(|root| {
                WalkDir::new(root)
                    .follow_links(true)
                    .into_iter()
                    .filter_entry(|entry| {
                        if let Some((ignore_path, ignore)) = ignore {
                            let path = entry.path();
                            // Canonicalization helps with removing `..` and
                            // `.` entries, which can cause issues with
                            // diff_paths.
                            let path = path.canonicalize().unwrap_or_else(|_| path.to_path_buf());
                            let relative_path = diff_paths(&path, &ignore_path)
                                .expect("One of the paths should be an absolute");
                            if ignore
                                .matched_path_or_any_parents(&relative_path, relative_path.is_dir())
                                .is_ignore()
                            {
                                trace!("ignoring {path:?}");
                                return false;
                            }
                        }
                        true
                    })
                    .filter_map(move |entry| {
                        let entry = match entry {
                            Ok(e) => e,
                            Err(e) => {
                                debug!("failed to scan {root:?}: {e}");
                                return None;
                            }
                        };
                        if entry.file_type().is_dir() {
                            // Changes to directories themselves aren't
                            // particularly interesting.
                            return None;
                        }
                        let path = entry.path().to_path_buf();

                        let meta = match entry.metadata() {
                            Ok(meta) => meta,
                            Err(e) => {
                                debug!("failed to scan {path:?}: {e}");
                                return None;
                            }
                        };
                        let mtime = meta.modified().unwrap_or(SystemTime::UNIX_EPOCH);
                        let pd = PathData {
                            file_type: meta.file_type(),
                            mtime,
                            size: meta.len(),
                        };
                        Some((path, pd))
                    })
            })
            .collect();
        let mut paths = Vec::new();
        for (new_path, new_data) in &new_path_data {
            match self.path_data.get(new_path) {
                Some(old_data) => {
                    if new_data != old_data {
                        paths.push(new_path.to_path_buf());
                    }
                }
                None => {
                    paths.push(new_path.clone());
                }
            }
        }
        for old_path in self.path_data.keys() {
            if !new_path_data.contains_key(old_path) {
                paths.push(old_path.to_path_buf());
            }
        }
        self.path_data = new_path_data;
        paths
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Helper for testing the watcher.
    fn check_watch_behavior(
        gitignore_path: &str,
        gitignore: &str,
        book_root_path: &str,
        ignored: &[&str],
        not_ignored: &[&str],
        extra_setup: &dyn Fn(&Path),
    ) {
        // Create the book and initialize things.
        let temp = tempfile::Builder::new()
            .prefix("mdbook-")
            .tempdir()
            .unwrap();
        let root = temp.path();
        let book_root = root.join(book_root_path);
        // eprintln!("book_root={book_root:?}",);
        MDBook::init(&book_root).build().unwrap();
        std::fs::write(root.join(gitignore_path), gitignore).unwrap();
        let create = |paths: &[&str]| {
            let mut paths = paths
                .iter()
                .map(|path| root.join(path))
                .inspect(|path| {
                    std::fs::create_dir_all(path.parent().unwrap()).unwrap();
                    std::fs::write(path, "initial content").unwrap();
                })
                .map(|path| path.canonicalize().unwrap())
                .collect::<Vec<_>>();
            paths.sort();
            paths
        };
        let ignored = create(ignored);
        let not_ignored = create(not_ignored);
        extra_setup(&book_root);
        // Create a watcher and check its behavior.
        let book = MDBook::load(&book_root).unwrap();
        let mut watcher = Watcher::new(&book_root);
        watcher.set_roots(&book);
        // Do an initial scan to initialize its state.
        watcher.scan();
        // Verify the steady state is empty.
        let changed = watcher.scan();
        assert_eq!(changed, Vec::<PathBuf>::new());
        // Modify all files, and verify that only not_ignored are detected.
        for path in ignored.iter().chain(not_ignored.iter()) {
            std::fs::write(path, "modified").unwrap();
        }
        let changed = watcher.scan();
        let mut changed = changed
            .into_iter()
            .map(|p| p.canonicalize().unwrap())
            .collect::<Vec<_>>();
        changed.sort();
        assert_eq!(changed, not_ignored);
        // Verify again that steady state is empty.
        let changed = watcher.scan();
        assert_eq!(changed, Vec::<PathBuf>::new());
    }

    #[test]
    fn test_ignore() {
        // Basic gitignore test.
        check_watch_behavior(
            "foo/.gitignore",
            "*.tmp",
            "foo",
            &["foo/src/somefile.tmp"],
            &["foo/src/chapter.md"],
            &|_book_root| {},
        );
    }

    #[test]
    fn test_ignore_in_parent() {
        // gitignore is in the parent of the book
        check_watch_behavior(
            ".gitignore",
            "*.tmp\nsomedir/\n/inroot\n/foo/src/inbook\n",
            "foo",
            &[
                "foo/src/somefile.tmp",
                "foo/src/somedir/somefile",
                "inroot/somefile",
                "foo/src/inbook/somefile",
            ],
            &["foo/src/inroot/somefile"],
            &|_book_root| {},
        );
    }

    #[test]
    fn test_ignore_canonical() {
        // test with path with ..
        check_watch_behavior(
            ".gitignore",
            "*.tmp\nsomedir/\n/foo/src/inbook\n",
            "bar/../foo",
            &[
                "foo/src/somefile.tmp",
                "foo/src/somedir/somefile",
                "foo/src/inbook/somefile",
            ],
            &["foo/src/chapter.md"],
            &|_book_root| {},
        );
    }

    #[test]
    fn test_scan_extra_watch() {
        // Check behavior with extra-watch-dirs
        check_watch_behavior(
            ".gitignore",
            "*.tmp\n/outside-root/ignoreme\n/foo/examples/ignoreme\n",
            "foo",
            &[
                "foo/src/somefile.tmp",
                "foo/examples/example.tmp",
                "outside-root/somefile.tmp",
                "outside-root/ignoreme",
                "foo/examples/ignoreme",
            ],
            &[
                "foo/src/chapter.md",
                "foo/examples/example.rs",
                "foo/examples/example2.rs",
                "outside-root/image.png",
            ],
            &|book_root| {
                std::fs::write(
                    book_root.join("book.toml"),
                    r#"
                        [book]
                        title = "foo"

                        [build]
                        extra-watch-dirs = [
                            "examples",
                            "../outside-root",
                        ]
                    "#,
                )
                .unwrap();
            },
        );
    }
}
