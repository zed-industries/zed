// Take a look at the license at the top of the repository in the LICENSE file.

use std::fs::{self, File};
use std::io::Read;
use std::path::Path;

pub struct TestResult {
    pub nb_tests: usize,
    pub nb_errors: usize,
}

impl std::ops::AddAssign for TestResult {
    fn add_assign(&mut self, other: Self) {
        self.nb_tests += other.nb_tests;
        self.nb_errors += other.nb_errors;
    }
}

pub fn read_dirs<P: AsRef<Path>, F: FnMut(&Path, &str)>(dirs: &[P], callback: &mut F) {
    for dir in dirs {
        read_dir(dir, callback);
    }
}

fn read_dir<P: AsRef<Path>, F: FnMut(&Path, &str)>(dir: P, callback: &mut F) {
    for entry in fs::read_dir(dir).expect("read_dir failed") {
        let entry = entry.expect("entry failed");
        let file_type = entry.file_type().expect("file_type failed");
        let path = entry.path();
        if file_type.is_dir() {
            read_dir(path, callback);
        } else if path
            .extension()
            .map(|ext| ext == "rs" || ext == "c" || ext == "h")
            .unwrap_or(false)
        {
            let content = read_file(&path);
            callback(&path, &content);
        }
    }
}

fn read_file<P: AsRef<Path>>(p: P) -> String {
    let mut f = File::open(&p).expect("read_file::open failed");
    let mut content =
        String::with_capacity(f.metadata().map(|m| m.len() as usize + 1).unwrap_or(0));
    if let Err(e) = f.read_to_string(&mut content) {
        panic!(
            "read_file::read_to_end failed for `{}: {e:?}",
            p.as_ref().display()
        );
    }
    content
}

pub fn show_error(p: &Path, err: &str) {
    eprintln!("=> [{}]: {err}", p.display());
}
