use zvariant::{LE, Result, serialized::Context, to_bytes};

#[test]
fn issue_59() {
    // Ensure we don't panic on deserializing tuple of smaller than expected length.
    let ctxt = Context::new_dbus(LE, 0);
    let encoded = to_bytes(ctxt, &("hello",)).unwrap();
    let result: Result<((&str, &str), _)> = encoded.deserialize();
    assert!(result.is_err());
}
