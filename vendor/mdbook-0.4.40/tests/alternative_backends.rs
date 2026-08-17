//! Integration tests to make sure alternative backends work.

use mdbook::config::Config;
use mdbook::MDBook;
use std::fs;
use std::path::Path;
use tempfile::{Builder as TempFileBuilder, TempDir};

#[test]
fn passing_alternate_backend() {
    let (md, _temp) = dummy_book_with_backend("passing", success_cmd(), false);

    md.build().unwrap();
}

#[test]
fn failing_alternate_backend() {
    let (md, _temp) = dummy_book_with_backend("failing", fail_cmd(), false);

    md.build().unwrap_err();
}

#[test]
fn missing_backends_are_fatal() {
    let (md, _temp) = dummy_book_with_backend("missing", "trduyvbhijnorgevfuhn", false);
    assert!(md.build().is_err());
}

#[test]
fn missing_optional_backends_are_not_fatal() {
    let (md, _temp) = dummy_book_with_backend("missing", "trduyvbhijnorgevfuhn", true);
    assert!(md.build().is_ok());
}

#[test]
fn alternate_backend_with_arguments() {
    let (md, _temp) = dummy_book_with_backend("arguments", "echo Hello World!", false);

    md.build().unwrap();
}

#[test]
fn backends_receive_render_context_via_stdin() {
    use mdbook::renderer::RenderContext;
    use std::fs::File;

    let (md, temp) = dummy_book_with_backend("cat-to-file", "renderers/myrenderer", false);

    let renderers = temp.path().join("renderers");
    fs::create_dir(&renderers).unwrap();
    rust_exe(
        &renderers,
        "myrenderer",
        r#"fn main() {
            use std::io::Read;
            let mut s = String::new();
            std::io::stdin().read_to_string(&mut s).unwrap();
            std::fs::write("out.txt", s).unwrap();
        }"#,
    );

    let out_file = temp.path().join("book/out.txt");

    assert!(!out_file.exists());
    md.build().unwrap();
    assert!(out_file.exists());

    let got = RenderContext::from_json(File::open(&out_file).unwrap());
    assert!(got.is_ok());
}

#[test]
fn relative_command_path() {
    // Checks behavior of relative paths for the `command` setting.
    let temp = TempFileBuilder::new().prefix("mdbook").tempdir().unwrap();
    let renderers = temp.path().join("renderers");
    fs::create_dir(&renderers).unwrap();
    rust_exe(
        &renderers,
        "myrenderer",
        r#"fn main() {
            std::fs::write("output", "test").unwrap();
        }"#,
    );
    let do_test = |cmd_path| {
        let mut config = Config::default();
        config
            .set("output.html", toml::value::Table::new())
            .unwrap();
        config.set("output.myrenderer.command", cmd_path).unwrap();
        let md = MDBook::init(temp.path())
            .with_config(config)
            .build()
            .unwrap();
        let output = temp.path().join("book/myrenderer/output");
        assert!(!output.exists());
        md.build().unwrap();
        assert!(output.exists());
        fs::remove_file(output).unwrap();
    };
    // Legacy paths work, relative to the output directory.
    if cfg!(windows) {
        do_test("../../renderers/myrenderer.exe");
    } else {
        do_test("../../renderers/myrenderer");
    }
    // Modern path, relative to the book directory.
    do_test("renderers/myrenderer");
}

fn dummy_book_with_backend(
    name: &str,
    command: &str,
    backend_is_optional: bool,
) -> (MDBook, TempDir) {
    let temp = TempFileBuilder::new().prefix("mdbook").tempdir().unwrap();

    let mut config = Config::default();
    config
        .set(format!("output.{}.command", name), command)
        .unwrap();

    if backend_is_optional {
        config
            .set(format!("output.{}.optional", name), true)
            .unwrap();
    }

    let md = MDBook::init(temp.path())
        .with_config(config)
        .build()
        .unwrap();

    (md, temp)
}

fn fail_cmd() -> &'static str {
    if cfg!(windows) {
        r#"cmd.exe /c "exit 1""#
    } else {
        "false"
    }
}

fn success_cmd() -> &'static str {
    if cfg!(windows) {
        r#"cmd.exe /c "exit 0""#
    } else {
        "true"
    }
}

fn rust_exe(temp: &Path, name: &str, src: &str) {
    let rs = temp.join(name).with_extension("rs");
    fs::write(&rs, src).unwrap();
    let status = std::process::Command::new("rustc")
        .arg(rs)
        .current_dir(temp)
        .status()
        .expect("rustc should run");
    assert!(status.success());
}
