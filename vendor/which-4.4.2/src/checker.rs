use crate::finder::Checker;
use std::fs;
use std::path::Path;

pub struct ExecutableChecker;

impl ExecutableChecker {
    pub fn new() -> ExecutableChecker {
        ExecutableChecker
    }
}

impl Checker for ExecutableChecker {
    #[cfg(any(unix, target_os = "wasi"))]
    fn is_valid(&self, path: &Path) -> bool {
        use rustix::fs as rfs;
        rfs::access(path, rfs::Access::EXEC_OK).is_ok()
    }

    #[cfg(windows)]
    fn is_valid(&self, _path: &Path) -> bool {
        true
    }
}

pub struct ExistedChecker;

impl ExistedChecker {
    pub fn new() -> ExistedChecker {
        ExistedChecker
    }
}

impl Checker for ExistedChecker {
    #[cfg(target_os = "windows")]
    fn is_valid(&self, path: &Path) -> bool {
        fs::symlink_metadata(path)
            .map(|metadata| {
                let file_type = metadata.file_type();
                file_type.is_file() || file_type.is_symlink()
            })
            .unwrap_or(false)
    }

    #[cfg(not(target_os = "windows"))]
    fn is_valid(&self, path: &Path) -> bool {
        fs::metadata(path)
            .map(|metadata| metadata.is_file())
            .unwrap_or(false)
    }
}

pub struct CompositeChecker {
    checkers: Vec<Box<dyn Checker>>,
}

impl CompositeChecker {
    pub fn new() -> CompositeChecker {
        CompositeChecker {
            checkers: Vec::new(),
        }
    }

    pub fn add_checker(mut self, checker: Box<dyn Checker>) -> CompositeChecker {
        self.checkers.push(checker);
        self
    }
}

impl Checker for CompositeChecker {
    fn is_valid(&self, path: &Path) -> bool {
        self.checkers.iter().all(|checker| checker.is_valid(path))
    }
}
