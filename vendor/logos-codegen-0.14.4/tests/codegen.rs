use std::{error::Error, io, path::PathBuf};

#[rstest::rstest]
#[case("simple")]
#[case("no_error_lut")]
pub fn test_codegen(#[case] fixture: &str) -> Result<(), Box<dyn Error>> {
    let mut fixture_dir = PathBuf::new();
    fixture_dir.push(env!("CARGO_MANIFEST_DIR"));
    fixture_dir.push("tests");
    fixture_dir.push("data");
    fixture_dir.push(fixture);

    let input = fixture_dir.join("input.rs");
    fixture_dir.push("output.rs");
    let output_file = fixture_dir;

    let input = std::fs::read_to_string(input)?;
    let output = std::fs::read_to_string(&output_file)?;

    let generated = logos_codegen::generate(input.parse()?);
    let generated = generated.to_string();

    if std::env::var("BLESS_CODEGEN").is_ok_and(|value| value == "1") {
        std::fs::write(&output_file, &generated)?;
        return Ok(());
    }

    assert_eq!(generated, output, "Codegen test failed: `{fixture}`, run tests again with env var `BLESS_CODEGEN=1` to bless these changes");

    Ok(())
}
