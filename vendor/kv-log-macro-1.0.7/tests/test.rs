use std::error::Error;

#[test]
fn should_work() -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    Ok(())
}
