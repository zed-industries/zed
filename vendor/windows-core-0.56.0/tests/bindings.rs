use windows_bindgen::*;

#[test]
fn bindings() -> Result<()> {
    bindgen(["--etc", "tests/bindings.txt"])?;
    bindgen(["--etc", "tests/com_bindings.txt"])?;
    Ok(())
}
