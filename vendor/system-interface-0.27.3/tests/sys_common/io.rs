use cap_tempfile::{ambient_authority, tempdir};

pub use cap_tempfile::TempDir;

#[allow(unused)]
pub fn tmpdir() -> TempDir {
    // It's ok to wrap this in an unsafe block, rather than an unsafe function,
    // because this function is only used by tests.
    unsafe { tempdir(ambient_authority()) }
        .expect("expected to be able to create a temporary directory")
}
