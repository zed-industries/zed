use anyhow::{bail, Context, Error, Result};
use pretty_assertions::assert_eq;
use std::{borrow::Cow, fs, path::Path};
use wasm_encoder::{Encode, Section};
use wit_component::{ComponentEncoder, DecodedWasm, Linker, StringEncoding, WitPrinter};
use wit_parser::{PackageId, Resolve, UnresolvedPackage};

/// Tests the encoding of components.
///
/// This test looks in the `components/` directory for test cases.
///
/// The expected input files for a test case are:
///
/// * [required] `module.wat` *or* some combination of `lib-$name.wat` and
///   `dlopen-lib-$name.wat` - contains the core module definition(s) to be
///   encoded as a component.  If one or more `lib-$name.wat` and/or
///   `dlopen-lib-$name.wat` files exist, they will be linked using `Linker`
///   such that the `lib-` ones are not `dlopen`-able but the `dlopen-lib-` ones
///   are.
/// * [required] `module.wit` *or* `lib-$name.wat` and `dlopen-lib-$name.wat`
///   corresponding to the WAT files above - WIT package(s) describing the
///   interfaces of the `module.wat` or `lib-$name.wat` and
///   `dlopen-lib-$name.wat` files. Must have a `default world`
/// * [optional] `adapt-$name.wat` - optional adapter for the module name
///   `$name`, can be specified for multiple `$name`s
/// * [optional] `adapt-$name.wit` - required for each `*.wat` adapter to
///   describe imports/exports of the adapter.
/// * [optional] `stub-missing-functions` - if linking libraries and this file
///   exists, `Linker::stub_missing_functions` will be set to `true`.  The
///   contents of the file are ignored.
///
/// And the output files are one of the following:
///
/// * `component.wat` - the expected encoded component in text format if the
///   encoding is expected to succeed.
/// * `component.wit` - if `component.wat` exists this is the inferred interface
///   of the component.
/// * `error.txt` - the expected error message if the encoding is expected to
///   fail.
///
/// The test encodes a component based on the input files. If the encoding
/// succeeds, it expects the output to match `component.wat`. If the encoding
/// fails, it expects the output to match `error.txt`.
///
/// Run the test with the environment variable `BLESS` set to update
/// either `component.wat` or `error.txt` depending on the outcome of the encoding.
#[test]
fn component_encoding_via_flags() -> Result<()> {
    drop(env_logger::try_init());

    for entry in fs::read_dir("tests/components")? {
        let path = entry?.path();
        if !path.is_dir() {
            continue;
        }

        let test_case = path.file_stem().unwrap().to_str().unwrap();
        println!("testing {test_case}");

        let mut resolve = Resolve::default();
        let (pkg, _) = resolve.push_dir(&path)?;

        let module_path = path.join("module.wat");
        let mut adapters = glob::glob(path.join("adapt-*.wat").to_str().unwrap())?;
        let result = if module_path.is_file() {
            let module = read_core_module(&module_path, &resolve, pkg)?;
            adapters
                .try_fold(
                    ComponentEncoder::default().module(&module)?.validate(true),
                    |encoder, path| {
                        let (name, wasm) = read_name_and_module("adapt-", &path?, &resolve, pkg)?;
                        Ok::<_, Error>(encoder.adapter(&name, &wasm)?)
                    },
                )?
                .encode()
        } else {
            let mut libs = glob::glob(path.join("lib-*.wat").to_str().unwrap())?
                .map(|path| Ok(("lib-", path?, false)))
                .chain(
                    glob::glob(path.join("dlopen-lib-*.wat").to_str().unwrap())?
                        .map(|path| Ok(("dlopen-lib-", path?, true))),
                )
                .collect::<Result<Vec<_>>>()?;

            // Sort list to ensure deterministic order, which determines priority in cases of duplicate symbols:
            libs.sort_by(|(_, a, _), (_, b, _)| a.cmp(b));

            let mut linker = Linker::default().validate(true);

            if path.join("stub-missing-functions").is_file() {
                linker = linker.stub_missing_functions(true);
            }

            let linker =
                libs.into_iter()
                    .try_fold(linker, |linker, (prefix, path, dl_openable)| {
                        let (name, wasm) = read_name_and_module(prefix, &path, &resolve, pkg)?;
                        Ok::<_, Error>(linker.library(&name, &wasm, dl_openable)?)
                    })?;

            adapters
                .try_fold(linker, |linker, path| {
                    let (name, wasm) = read_name_and_module("adapt-", &path?, &resolve, pkg)?;
                    Ok::<_, Error>(linker.adapter(&name, &wasm)?)
                })?
                .encode()
        };
        let component_path = path.join("component.wat");
        let component_wit_path = path.join("component.wit.print");
        let error_path = path.join("error.txt");

        let bytes = match result {
            Ok(bytes) => {
                if test_case.starts_with("error-") {
                    bail!("expected an error but got success");
                }
                bytes
            }
            Err(err) => {
                if !test_case.starts_with("error-") {
                    return Err(err);
                }
                assert_output(&format!("{err:?}"), &error_path)?;
                continue;
            }
        };

        let wat = wasmprinter::print_bytes(&bytes)?;
        assert_output(&wat, &component_path)?;
        let (pkg, resolve) = match wit_component::decode(&bytes)? {
            DecodedWasm::WitPackage(..) => unreachable!(),
            DecodedWasm::Component(resolve, world) => {
                (resolve.worlds[world].package.unwrap(), resolve)
            }
        };
        let wit = WitPrinter::default().print(&resolve, pkg)?;
        assert_output(&wit, &component_wit_path)?;

        UnresolvedPackage::parse(&component_wit_path, &wit)
            .context("failed to parse printed WIT")?;

        // Check that the producer data got piped through properly
        let metadata = wasm_metadata::Metadata::from_binary(&bytes)?;
        match metadata {
            // Depends on the ComponentEncoder always putting the first module as the 0th child:
            wasm_metadata::Metadata::Component { children, .. } => match children[0].as_ref() {
                wasm_metadata::Metadata::Module { producers, .. } => {
                    let producers = producers.as_ref().expect("child module has producers");
                    let processed_by = producers
                        .get("processed-by")
                        .expect("child has processed-by section");
                    assert_eq!(
                        processed_by
                            .get("wit-component")
                            .expect("wit-component producer present"),
                        env!("CARGO_PKG_VERSION")
                    );
                    if module_path.is_file() {
                        assert_eq!(
                            processed_by
                                .get("my-fake-bindgen")
                                .expect("added bindgen field present"),
                            "123.45"
                        );
                    } else {
                        // Otherwise, we used `Linker`, which synthesizes the
                        // "main" module and thus won't have `my-fake-bindgen`
                    }
                }
                _ => panic!("expected child to be a module"),
            },
            _ => panic!("expected top level metadata of component"),
        }
    }

    Ok(())
}

fn read_name_and_module(
    prefix: &str,
    path: &Path,
    resolve: &Resolve,
    pkg: PackageId,
) -> Result<(String, Vec<u8>)> {
    let wasm = read_core_module(path, resolve, pkg)?;
    let stem = path.file_stem().unwrap().to_str().unwrap();
    let name = stem.trim_start_matches(prefix).to_owned();
    Ok((name, wasm))
}

/// Parses the core wasm module at `path`, expected as a `*.wat` file.
///
/// The `resolve` and `pkg` are the parsed WIT package from this test's
/// directory and the `path`'s filename is used to find a WIT document of the
/// corresponding name which should have a world that `path` ascribes to.
fn read_core_module(path: &Path, resolve: &Resolve, pkg: PackageId) -> Result<Vec<u8>> {
    let mut wasm = wat::parse_file(path)?;
    let name = path.file_stem().and_then(|s| s.to_str()).unwrap();
    let world = resolve
        .select_world(pkg, Some(name))
        .context("failed to select a world")?;

    // Add this producer data to the wit-component metadata so we can make sure it gets through the
    // translation:
    let mut producers = wasm_metadata::Producers::empty();
    producers.add("processed-by", "my-fake-bindgen", "123.45");

    let encoded =
        wit_component::metadata::encode(resolve, world, StringEncoding::UTF8, Some(&producers))?;

    let section = wasm_encoder::CustomSection {
        name: "component-type".into(),
        data: Cow::Borrowed(&encoded),
    };
    wasm.push(section.id());
    section.encode(&mut wasm);
    Ok(wasm)
}

fn assert_output(contents: &str, path: &Path) -> Result<()> {
    let contents = contents.replace("\r\n", "\n").replace(
        concat!("\"", env!("CARGO_PKG_VERSION"), "\""),
        "\"$CARGO_PKG_VERSION\"",
    );
    if std::env::var_os("BLESS").is_some() {
        fs::write(path, contents)?;
    } else {
        match fs::read_to_string(path) {
            Ok(expected) => {
                assert_eq!(
                    expected.replace("\r\n", "\n").trim(),
                    contents.trim(),
                    "failed baseline comparison ({})",
                    path.display(),
                );
            }
            Err(_) => {
                panic!("expected {path:?} to contain\n{contents}");
            }
        }
    }
    Ok(())
}
