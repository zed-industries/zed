use anyhow::{Context, Result};
use libtest_mimic::{Arguments, Trial};
use pretty_assertions::assert_eq;
use std::fs;
use std::path::Path;
use wasmparser::WasmFeatures;
use wit_component::WitPrinter;
use wit_parser::{PackageId, Resolve, UnresolvedPackageGroup};

/// Tests the encoding of a WIT package as a WebAssembly binary.
///
/// This test looks in the `interfaces/` directory for test cases. Each test
/// case is a `*.wit` file or a folder which contains `*.wit` files as part of a
/// multi-file package. Each tests `foo.wit` is accompanied with a `foo.wat` for
/// the WebAssembly text format encoding of the package. Additionally each test
/// has a `foo.print.wit` which is the machine-printed version of the WIT
/// document as decoded from the binary encoded interface.
///
/// Run the test with the environment variable `BLESS` set to update
/// the baseline files.
fn main() -> Result<()> {
    env_logger::init();

    let mut trials = Vec::new();
    for entry in fs::read_dir("tests/interfaces")? {
        let path = entry?.path();
        let name = match path.file_name().and_then(|s| s.to_str()) {
            Some(s) => s,
            None => continue,
        };
        let is_dir = path.is_dir();
        let is_test = is_dir || name.ends_with(".wit");
        if is_test {
            trials.push(Trial::test(name.to_string(), move || {
                run_test(&path, is_dir)
                    .context(format!("failed test `{}`", path.display()))
                    .map_err(|e| format!("{e:?}").into())
            }));
        }
    }

    let mut args = Arguments::from_args();
    if cfg!(target_family = "wasm") && !cfg!(target_feature = "atomics") {
        args.test_threads = Some(1);
    }
    libtest_mimic::run(&args, trials).exit();
}

fn run_test(path: &Path, is_dir: bool) -> Result<()> {
    let mut resolve = Resolve::new();
    let package = if is_dir {
        resolve.push_dir(path)?.0
    } else {
        resolve.push_file(path)?
    };

    assert_print(&resolve, package, path, is_dir)?;

    // First convert the WIT package to a binary WebAssembly output, then
    // convert that binary wasm to textual wasm, then assert it matches the
    // expectation.
    let wasm = wit_component::encode(&resolve, package)?;
    let wat = wasmprinter::print_bytes(&wasm)?;
    assert_output(&path.with_extension("wat"), &wat)?;
    wasmparser::Validator::new_with_features(WasmFeatures::all())
        .validate_all(&wasm)
        .context("failed to validate wasm output")?;

    // Next decode a fresh WIT package from the WebAssembly generated. Print
    // this package's documents and assert they all match the expectations.
    let decoded = wit_component::decode(&wasm)?;

    let decoded_package = decoded.package();
    let resolve = decoded.resolve();

    assert_print(resolve, decoded.package(), path, is_dir)?;

    // Finally convert the decoded package to wasm again and make sure it
    // matches the prior wasm.
    let wasm2 = wit_component::encode(resolve, decoded_package)?;
    if wasm != wasm2 {
        let wat2 = wasmprinter::print_bytes(&wasm)?;
        assert_eq!(wat, wat2, "document did not roundtrip correctly");
    }

    Ok(())
}

fn assert_print(resolve: &Resolve, pkg_id: PackageId, path: &Path, is_dir: bool) -> Result<()> {
    let mut printer = WitPrinter::default();
    printer.print(resolve, pkg_id, &[])?;
    let output = printer.output.to_string();
    let pkg = &resolve.packages[pkg_id];
    let expected = if is_dir {
        path.join(format!("{}.wit.print", &pkg.name.name))
    } else {
        path.with_extension("wit.print")
    };
    assert_output(&expected, &output)?;

    UnresolvedPackageGroup::parse("foo.wit", &output).context("failed to parse printed output")?;
    Ok(())
}

fn assert_output(expected: &Path, actual: &str) -> Result<()> {
    let actual = actual.replace(
        concat!("\"", env!("CARGO_PKG_VERSION"), "\""),
        "\"$CARGO_PKG_VERSION\"",
    );
    if std::env::var_os("BLESS").is_some() {
        fs::write(expected, actual).with_context(|| format!("failed to write {expected:?}"))?;
    } else {
        assert_eq!(
            fs::read_to_string(expected)
                .with_context(|| format!("failed to read {expected:?}"))?
                .replace("\r\n", "\n"),
            actual,
            "expectation `{}` did not match actual",
            expected.display(),
        );
    }
    Ok(())
}
