use zvariant::{LE, serialized::Context, to_bytes};

#[test]
fn issue_1145() {
    // Ensure f32::NAN can be encoded and decoded.
    let ctxt = Context::new_dbus(LE, 0);
    {
        let encoded = to_bytes(ctxt, &f32::NAN).unwrap();
        let result: f32 = encoded.deserialize().unwrap().0;
        assert!(result.is_nan());
    }
    // Ensure f32::INFINITY can be encoded and decoded.
    {
        let encoded = to_bytes(ctxt, &f32::INFINITY).unwrap();
        let result: f32 = encoded.deserialize().unwrap().0;
        assert!(result.is_infinite());
    }
    {
        let encoded = to_bytes(ctxt, &f32::NEG_INFINITY).unwrap();
        let result: f32 = encoded.deserialize().unwrap().0;
        assert!(result.is_infinite());
    }
}
