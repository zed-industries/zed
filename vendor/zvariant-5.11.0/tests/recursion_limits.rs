use zvariant::{Error, LE, MaxDepthExceeded, Value, serialized::Context, to_bytes};

#[macro_use]
mod common {
    include!("common.rs");
}

#[test]
fn recursion_limits() {
    let ctxt = Context::new_dbus(LE, 0);
    // Total container depth exceeds limit (64)
    let mut value = Value::from(0u8);
    for _ in 0..64 {
        value = Value::Value(Box::new(value));
    }
    assert!(matches!(
        to_bytes(ctxt, &value),
        Err(Error::MaxDepthExceeded(MaxDepthExceeded::Container))
    ));

    // Array depth exceeds limit (32)
    let vec = vec![vec![vec![vec![vec![vec![vec![vec![vec![vec![vec![
        vec![vec![vec![vec![vec![vec![vec![vec![vec![vec![vec![
            vec![vec![vec![vec![vec![vec![vec![vec![vec![vec![vec![
                0u8,
            ]]]]]]]]]]],
        ]]]]]]]]]]],
    ]]]]]]]]]]];
    assert!(matches!(
        to_bytes(ctxt, &vec),
        Err(Error::MaxDepthExceeded(MaxDepthExceeded::Array))
    ));

    // Struct depth exceeds limit (32)
    let tuple = ((((((((((((((((((
        (((((((((((((((0u8,),),),),),),),),),),),),),),),
    ),),),),),),),),),),),),),),),),),);
    assert!(matches!(
        to_bytes(ctxt, &tuple),
        Err(Error::MaxDepthExceeded(MaxDepthExceeded::Structure))
    ));

    // total depth exceeds limit (64) with struct, array and variant.
    let mut value = Value::from(0u8);
    for _ in 0..32 {
        value = Value::Value(Box::new(value));
    }
    let tuple_array =
        (
            ((((((((((((((((vec![vec![vec![vec![vec![vec![vec![vec![vec![
                vec![vec![vec![vec![vec![vec![vec![value]]]]]]],
            ]]]]]]]]],),),),),),),),),),),),),),),),),
        );
    assert!(matches!(
        to_bytes(ctxt, &tuple_array),
        Err(Error::MaxDepthExceeded(MaxDepthExceeded::Container))
    ));

    // TODO:
    //
    // * Test deserializers.
    // * Test gvariant format.
}
