use anyhow::{Context, Result};
use pretty_assertions::assert_eq;
use std::{fs, path::Path};
use wit_component::WitPrinter;
use wit_parser::Resolve;

/// This is a test which iterates over the `tests/merge` directory and treats
/// each subdirectory as its own test. Each subdirectory has an `into` and a
/// `from` folder which represent two different `Resolve` sets, each with their
/// own set of dependencies as well.
///
/// Each test will merge the `from` into the `into` and assert that everything
/// is valid along the way. On successful merge the resulting documents
/// are printed and asserted against expectations. Failures assert the
/// correct error message.
#[test]
fn merging() -> Result<()> {
    drop(env_logger::try_init());

    for entry in fs::read_dir("tests/merge")? {
        let path = entry?.path();
        if !path.is_dir() {
            continue;
        }

        let test_case = path.file_stem().unwrap().to_str().unwrap();
        println!("testing {test_case}");

        let mut from = Resolve::default();
        from.push_dir(&path.join("from"))?;
        let mut into = Resolve::default();
        into.push_dir(&path.join("into"))?;

        from.assert_valid();
        into.assert_valid();

        match into.merge(from) {
            Ok(_) => {
                assert!(
                    !test_case.starts_with("bad-"),
                    "should have failed to merge"
                );
                into.assert_valid();
                for (id, pkg) in into.packages.iter() {
                    let expected = path
                        .join("merge")
                        .join(&pkg.name.name)
                        .with_extension("wit");
                    let mut printer = WitPrinter::default();
                    printer.print(&into, id, &[])?;
                    let output = printer.output.to_string();
                    assert_output(&expected, &output)?;
                }
            }
            Err(e) => {
                assert!(test_case.starts_with("bad-"), "failed to merge with {e:?}");
                assert_output(&path.join("error.txt"), &format!("{e:#}"))?;
            }
        }
    }

    Ok(())
}

fn assert_output(expected: &Path, actual: &str) -> Result<()> {
    if std::env::var_os("BLESS").is_some() {
        fs::create_dir_all(expected.parent().unwrap())?;
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
