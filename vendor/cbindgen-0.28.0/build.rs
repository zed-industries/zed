fn generate_tests() {
    use std::env;
    use std::ffi::OsStr;
    use std::fs::{self, File};
    use std::io::Write;
    use std::path::{Path, PathBuf};

    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let mut dst = File::create(Path::new(&out_dir).join("tests.rs")).unwrap();

    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let tests_dir = manifest_dir.join("tests").join("rust");
    let tests = fs::read_dir(&tests_dir).unwrap();

    let entries = tests.map(|t| t.expect("Couldn't read test file"));

    println!("cargo:rerun-if-changed={}", tests_dir.display());

    for entry in entries {
        let path_segment = if entry.file_type().unwrap().is_file() {
            match entry.path().extension().and_then(OsStr::to_str) {
                Some("rs") => {}
                _ => continue,
            };

            entry
                .path()
                .file_stem()
                .unwrap()
                .to_str()
                .unwrap()
                .to_owned()
        } else {
            entry.file_name().to_str().unwrap().to_owned()
        };

        let identifier = path_segment
            .replace(|c| !char::is_alphanumeric(c), "_")
            .replace("__", "_");

        writeln!(
            dst,
            "test_file!(test_{}, {:?}, {:?});",
            identifier,
            path_segment,
            entry.path(),
        )
        .unwrap();
    }

    dst.flush().unwrap();
}

fn generate_depfile_tests() {
    use std::env;
    use std::fs::{self, File};
    use std::io::Write;
    use std::path::{Path, PathBuf};

    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let mut dst = File::create(Path::new(&out_dir).join("depfile_tests.rs")).unwrap();

    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let tests_dir = manifest_dir.join("tests").join("depfile");
    let tests = fs::read_dir(&tests_dir).unwrap();

    let entries = tests.map(|t| t.expect("Couldn't read test file"));

    println!("cargo:rerun-if-changed={}", tests_dir.display());

    for entry in entries {
        if entry.file_type().unwrap().is_file() {
            continue;
        };
        let path_segment = entry.file_name().to_str().unwrap().to_owned();

        let identifier = path_segment
            .replace(|c| !char::is_alphanumeric(c), "_")
            .replace("__", "_");

        writeln!(
            dst,
            "test_file!(test_depfile_{}, {:?}, {:?});",
            identifier,
            path_segment,
            entry.path(),
        )
        .unwrap();
    }

    dst.flush().unwrap();
}

fn main() {
    generate_tests();
    generate_depfile_tests();
}
