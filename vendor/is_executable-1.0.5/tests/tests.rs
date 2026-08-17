extern crate diff;
extern crate is_executable;

use is_executable::is_executable;

#[cfg(unix)]
mod unix {
    use super::*;

    #[test]
    fn executable() {
        assert!(is_executable("./tests/i_am_executable"));
    }

    #[test]
    fn executable_symlink() {
        assert!(is_executable("./tests/i_am_executable_and_symlink"));
    }

    #[test]
    fn not_executable_symlink() {
        assert!(!is_executable("./tests/i_am_not_executable_and_symlink"));
    }

    #[test]
    fn not_executable_directory() {
        assert!(!is_executable("."));
    }
}

#[cfg(target_os = "windows")]
mod windows {
    use super::*;

    #[test]
    fn executable() {
        assert!(is_executable("C:\\Windows\\explorer.exe"));
    }

    #[test]
    fn by_extension() {
        assert!(is_executable("./tests/i_am_executable_on_windows.bat"));
    }

    #[test]
    fn non_existent_correct_extension() {
        assert!(!is_executable("./tests/non_existent.exe"));
    }
}

#[cfg(any(target_os = "wasi", target_family = "wasm"))]
mod wasm {
    use super::*;

    #[test]
    fn executable() {
        assert!(!is_executable("./tests/i_am_executable"));
    }

    #[test]
    fn executable_symlink() {
        assert!(!is_executable("./tests/i_am_executable_and_symlink"));
    }

    #[test]
    fn not_executable_symlink() {
        assert!(!is_executable("./tests/i_am_not_executable_and_symlink"));
    }

    #[test]
    fn not_executable_directory() {
        assert!(!is_executable("."));
    }
}

#[test]
fn not_executable() {
    assert!(!is_executable("./tests/i_am_not_executable"));
}

#[test]
fn non_existant() {
    assert!(!is_executable("./tests/this-file-does-not-exist"));
}
