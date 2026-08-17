use zvariant::{Dict, Type, Value};

#[macro_use]
mod common {
    include!("common.rs");
}

#[test]
fn dict_compare() {
    // the order in which a dict has been constructed must not play a role
    // https://github.com/z-galaxy/zbus/issues/484
    let mut dict1 = Dict::new(<&str>::SIGNATURE, Value::SIGNATURE);
    dict1.add("first", Value::new("value")).unwrap();
    dict1.add("second", Value::new("value")).unwrap();

    let mut dict2 = Dict::new(<&str>::SIGNATURE, Value::SIGNATURE);
    dict2.add("second", Value::new("value")).unwrap();
    dict2.add("first", Value::new("value")).unwrap();

    assert_eq!(dict1, dict2);
}
