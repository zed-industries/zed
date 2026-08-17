use rmpv::Value;

#[test]
fn display_nil() {
    assert_eq!("nil", format!("{}", Value::Nil));
}

#[test]
fn display_bool() {
    assert_eq!("true", format!("{}", Value::Boolean(true)));
    assert_eq!("false", format!("{}", Value::Boolean(false)));
}

#[test]
fn display_int() {
    assert_eq!("42", format!("{}", Value::from(42)));
    assert_eq!("42", format!("{}", Value::from(42)));
}

#[test]
fn display_float() {
    assert_eq!("3.1415", format!("{}", Value::F32(3.1415)));
    assert_eq!("3.1415", format!("{}", Value::F64(3.1415)));
}

#[test]
fn display_string() {
    assert_eq!("\"le string\"", format!("{}", Value::String("le string".into())));
}

#[test]
fn display_binary() {
    assert_eq!("[108, 101, 32, 115, 116, 114, 105, 110, 103]", format!("{}",
        Value::Binary(b"le string".to_vec())));
}

#[test]
fn display_array() {
    assert_eq!("[]", format!("{}", Value::Array(vec![])));
    assert_eq!("[nil]", format!("{}", Value::Array(vec![Value::Nil])));
    assert_eq!("[nil, nil]", format!("{}", Value::Array(vec![Value::Nil, Value::Nil])));
}

#[test]
fn display_map() {
    assert_eq!("{}", format!("{}", Value::Map(vec![])));
    assert_eq!("{nil: nil}", format!("{}", Value::Map(vec![(Value::Nil, Value::Nil)])));
    assert_eq!("{nil: nil, true: false}", format!("{}", Value::Map(vec![(Value::Nil, Value::Nil),
        (Value::Boolean(true), Value::Boolean(false))])));
}

#[test]
fn display_ext() {
    assert_eq!("[1, []]", format!("{}", Value::Ext(1, vec![])));
    assert_eq!("[1, [100]]", format!("{}", Value::Ext(1, vec![100])));
    assert_eq!("[1, [100, 42]]", format!("{}", Value::Ext(1, vec![100, 42])));
}

#[test]
fn from_bool() {
    assert_eq!(Value::Boolean(true), Value::from(true));
    assert_eq!(Value::Boolean(false), Value::from(false));
}

#[test]
fn from_u8() {
    assert_eq!(Value::from(42), Value::from(42u8));
}

#[test]
fn from_u16() {
    assert_eq!(Value::from(42), Value::from(42u16));
}

#[test]
fn from_u32() {
    assert_eq!(Value::from(42), Value::from(42u32));
}

#[test]
fn from_u64() {
    assert_eq!(Value::from(42), Value::from(42u64));
}

#[test]
fn from_usize() {
    assert_eq!(Value::from(42), Value::from(42usize));
}

#[test]
fn from_i8() {
    assert_eq!(Value::from(-42), Value::from(-42i8));
}

#[test]
fn from_i16() {
    assert_eq!(Value::from(-42), Value::from(-42i16));
}

#[test]
fn from_i32() {
    assert_eq!(Value::from(-42), Value::from(-42i32));
}

#[test]
fn from_i64() {
    assert_eq!(Value::from(-42), Value::from(-42i64));
}

#[test]
fn from_isize() {
    assert_eq!(Value::from(-42), Value::from(-42isize));
}

#[test]
fn from_f32() {
    assert_eq!(Value::F32(3.1415), Value::from(3.1415f32));
}

#[test]
fn from_f64() {
    assert_eq!(Value::F64(3.1415), Value::from(3.1415f64));
}

#[test]
fn from_iterator() {
    let v: Vec<u8> = vec![0u8, 1u8, 2u8];
    let w: Value = v.into_iter().collect();

    let w2 = Value::Array(vec![
                         Value::from(0u8),
                         Value::from(1u8),
                         Value::from(2u8)
                         ]); 

    assert_eq!(w, w2);

    let w3 = Value::Binary(vec![0u8, 1u8, 2u8]);

    assert!(w != w3);
}

#[test]
fn is_nil() {
    assert!(Value::Nil.is_nil());
    assert!(!Value::Boolean(true).is_nil());
}

#[test]
fn monadic_index() {
    let val = Value::Array(vec![
        Value::Array(vec![
            Value::String("value".into()),
            Value::Boolean(true),
        ]),
        Value::Boolean(false),
    ]);

    assert_eq!("value", val[0][0].as_str().unwrap());
    assert!(val[0][1].as_bool().unwrap());
    assert!(!val[1].as_bool().unwrap());

    assert!(val[0][0][0].is_nil());
    assert!(val[2].is_nil());
    assert!(val[1][2][3][4][5].is_nil());
}

#[test]
fn index_into_map() {
    let val = Value::Map(vec![
        ( Value::String("a".into()), Value::from(1) ),
        ( Value::String("b".into()), Value::Array(vec![
            Value::from(3),
            Value::from(4),
            Value::from(5)
        ])),
        ( Value::String("c".into()), Value::Map(vec![
            ( Value::String("d".into()), Value::from(8) ),
            ( Value::String("e".into()), Value::from(9) )
        ]))
    ]);        
    assert_eq!(1, val["a"].as_i64().unwrap());
    assert_eq!(5, val["b"][2].as_i64().unwrap());
    assert_eq!(9, val["c"]["e"].as_i64().unwrap());
    assert!(val["b"][3].is_nil());
    assert!(val["d"][4].is_nil());
}

#[test]
fn try_from_val() {
    use rmpv::Utf8String;
    use std::convert::TryInto;

  assert_eq!(false, Value::Boolean(false).try_into().unwrap());
  assert_eq!(Utf8String::from("spook"), Value::from("spook").try_into().unwrap());
  assert_eq!(String::from("spook"), TryInto::<String>::try_into(Value::from("spook")).unwrap());
  assert_eq!(vec![0], TryInto::<Vec<u8>>::try_into(Value::Binary(vec![0u8])).unwrap());
}
