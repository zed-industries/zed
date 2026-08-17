use std::error::Error;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

#[test]
fn trybuild() {
    let directory = PathBuf::from("tests/try_build");

    let mut _renamer = None;

    let compile_fail_dir = directory.join("compile_fail");

    // Sometimes error messages change on beta/nightly - allow alternate errors on those.
    _renamer = Some(Renamer::rename(compile_fail_dir.clone()).unwrap());

    let fail = trybuild::TestCases::new();
    fail.compile_fail(compile_fail_dir.join("*.rs"));
    add_feature_dirs(&compile_fail_dir, &fail, ExpectedResult::Fail);

    let pass = trybuild::TestCases::new();
    let pass_dir = directory.join("pass");
    pass.pass(pass_dir.join("*.rs"));
    add_feature_dirs(&pass_dir, &pass, ExpectedResult::Pass);
}

enum ExpectedResult {
    Pass,
    Fail,
}

fn add_feature_dirs(
    parent_dir: &Path,
    test_cases: &trybuild::TestCases,
    expected_result: ExpectedResult,
) {
    let features_dir = parent_dir.join("features");
    let feature_specific_dir = if cfg!(feature = "complex-expressions") {
        features_dir.join("complex-expressions")
    } else {
        features_dir.join("!complex-expressions")
    };
    let tests = feature_specific_dir.join("*.rs");
    match expected_result {
        ExpectedResult::Pass => test_cases.pass(tests),
        ExpectedResult::Fail => test_cases.compile_fail(tests),
    }
}

struct Renamer(Vec<PathBuf>);

impl Renamer {
    const STDERR_EXTENSION: &'static str = "stderr";

    #[rustversion::all(beta)]
    const VERSION_SPECIFIC_EXTENSION: &'static str = "stderr_beta";

    #[rustversion::all(nightly)]
    const VERSION_SPECIFIC_EXTENSION: &'static str = "stderr_nightly";

    #[rustversion::all(not(beta), not(nightly))]
    const VERSION_SPECIFIC_EXTENSION: &'static str = "stderr_doesnotexist";

    const NON_VERSION_SPECIFIC_BACKUP_EXTENSION: &'static str =
        "stderr_non_version_specific_backup";

    fn rename(dir: PathBuf) -> anyhow::Result<Self> {
        let nightly_paths = WalkDir::new(dir)
            .max_depth(1)
            .into_iter()
            .filter_map(|dir_entry| {
                let dir_entry = match dir_entry {
                    Ok(dir_entry) => dir_entry,
                    Err(err) => return Some(Err(err)),
                };
                let path = dir_entry.path();
                if let Some(file_name) = path.file_name() {
                    if Path::new(file_name).extension()
                        == Some(Renamer::VERSION_SPECIFIC_EXTENSION.as_ref())
                    {
                        return Some(Ok(path.to_path_buf()));
                    }
                }
                None
            })
            .collect::<Result<Vec<_>, _>>()?;
        // Create early so that if we end up returning an error this gets dropped and undoes any
        // already-done renames.
        let renamer = Renamer(nightly_paths);

        for nightly_path in &renamer.0 {
            std::fs::rename(
                nightly_path.with_extension(Renamer::STDERR_EXTENSION),
                nightly_path.with_extension(Renamer::NON_VERSION_SPECIFIC_BACKUP_EXTENSION),
            )?;
            std::fs::rename(
                nightly_path.with_extension(Renamer::VERSION_SPECIFIC_EXTENSION),
                nightly_path.with_extension(Renamer::STDERR_EXTENSION),
            )?;
        }
        Ok(renamer)
    }
}

impl Drop for Renamer {
    fn drop(&mut self) {
        for path in &self.0 {
            ignore_error(std::fs::rename(
                path.with_extension(Renamer::STDERR_EXTENSION),
                path.with_extension(Renamer::VERSION_SPECIFIC_EXTENSION),
            ));
            ignore_error(std::fs::rename(
                path.with_extension(Renamer::NON_VERSION_SPECIFIC_BACKUP_EXTENSION),
                path.with_extension(Renamer::STDERR_EXTENSION),
            ));
        }
    }
}

fn ignore_error<T, E: Error>(result: Result<T, E>) {
    if let Err(err) = result {
        eprintln!("Ignoring error: {}", err);
    }
}
