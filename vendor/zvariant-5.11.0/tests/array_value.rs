use std::vec;
use zvariant::{Array, LE, Type, Value, serialized::Context, to_bytes};

#[macro_use]
mod common {
    include!("common.rs");
}

#[test]
fn array_value() {
    // Let's use D-Bus/GVariant terms

    //
    // Array of u8
    //
    // First a normal Rust array that is actually serialized as a struct (thank you Serde!)
    assert_eq!(<[u8; 2]>::SIGNATURE, "(yy)");
    let ay = [77u8, 88];
    let ctxt = Context::new_dbus(LE, 0);
    let encoded = to_bytes(ctxt, &ay).unwrap();
    assert_eq!(encoded.len(), 2);
    let decoded: [u8; 2] = encoded.deserialize().unwrap().0;
    assert_eq!(&decoded, &[77u8, 88]);

    // Then rest of the tests just use ArrayVec, heapless::Vec or Vec
    #[cfg(feature = "arrayvec")]
    let ay = arrayvec::ArrayVec::from([77u8, 88]);
    #[cfg(all(not(feature = "arrayvec"), feature = "heapless"))]
    let ay = heapless::Vec::<_, 2>::from_slice(&[77u8, 88]).unwrap();
    #[cfg(all(not(feature = "arrayvec"), not(feature = "heapless")))]
    let ay = vec![77u8, 88];
    let ctxt = Context::new_dbus(LE, 0);
    let encoded = to_bytes(ctxt, &ay).unwrap();
    assert_eq!(encoded.len(), 6);

    #[cfg(feature = "arrayvec")]
    let decoded: arrayvec::ArrayVec<u8, 2> = encoded.deserialize().unwrap().0;
    #[cfg(all(not(feature = "arrayvec"), feature = "heapless"))]
    let decoded: heapless::Vec<u8, 2> = encoded.deserialize().unwrap().0;
    #[cfg(all(not(feature = "arrayvec"), not(feature = "heapless")))]
    let decoded: Vec<u8> = encoded.deserialize().unwrap().0;
    assert_eq!(&decoded.as_slice(), &[77u8, 88]);

    // GVariant format now
    #[cfg(feature = "gvariant")]
    {
        let ctxt = Context::new_gvariant(LE, 0);
        let gv_encoded = to_bytes(ctxt, &ay).unwrap();
        assert_eq!(gv_encoded.len(), 2);
    }
    let ctxt = Context::new_dbus(LE, 0);

    // As Value
    let v: Value<'_> = ay[..].into();
    assert_eq!(v.value_signature(), "ay");
    let encoded = to_bytes(ctxt, &v).unwrap();
    assert_eq!(encoded.len(), 10);
    let v = encoded.deserialize::<Value<'_>>().unwrap().0;
    if let Value::Array(array) = v {
        assert_eq!(*array.element_signature(), "y");
        assert_eq!(array.len(), 2);
        assert_eq!(array.get(0).unwrap(), Some(77u8));
        assert_eq!(array.get(1).unwrap(), Some(88u8));
    } else {
        panic!();
    }

    // Now try as Vec
    let vec = ay.to_vec();
    let encoded = to_bytes(ctxt, &vec).unwrap();
    assert_eq!(encoded.len(), 6);

    // Vec as Value
    let v: Value<'_> = Array::from(&vec).into();
    assert_eq!(v.value_signature(), "ay");
    let encoded = to_bytes(ctxt, &v).unwrap();
    assert_eq!(encoded.len(), 10);

    // Empty array
    let at: Vec<u64> = vec![];
    let encoded = to_bytes(ctxt, &at).unwrap();
    assert_eq!(encoded.len(), 8);

    // GVariant format now
    #[cfg(feature = "gvariant")]
    {
        let ctxt = Context::new_gvariant(LE, 0);
        let gv_encoded = to_bytes(ctxt, &at).unwrap();
        assert_eq!(gv_encoded.len(), 0);
        let at = encoded.deserialize::<Vec<u64>>().unwrap().0;
        assert_eq!(at.len(), 0);
    }
    let ctxt = Context::new_dbus(LE, 0);

    // As Value
    let v: Value<'_> = at[..].into();
    assert_eq!(v.value_signature(), "at");
    let encoded = to_bytes(ctxt, &v).unwrap();
    assert_eq!(encoded.len(), 8);
    let v = encoded.deserialize::<Value<'_>>().unwrap().0;
    if let Value::Array(array) = v {
        assert_eq!(*array.element_signature(), "t");
        assert_eq!(array.len(), 0);
    } else {
        panic!();
    }

    // GVariant format now
    #[cfg(feature = "gvariant")]
    {
        let ctxt = Context::new_gvariant(LE, 0);
        let v: Value<'_> = at[..].into();
        let gv_encoded = to_bytes(ctxt, &v).unwrap();
        assert_eq!(gv_encoded.len(), 3);
        let v = gv_encoded.deserialize::<Value<'_>>().unwrap().0;
        if let Value::Array(array) = v {
            assert_eq!(*array.element_signature(), "t");
            assert_eq!(array.len(), 0);
        } else {
            panic!();
        }
    }
    let ctxt = Context::new_dbus(LE, 0);

    //
    // Array of strings
    //
    // Can't use 'as' as it's a keyword
    let as_ = vec!["Hello", "World", "Now", "Bye!"];
    let encoded = to_bytes(ctxt, &as_).unwrap();
    assert_eq!(encoded.len(), 45);
    let decoded = encoded.deserialize::<Vec<&str>>().unwrap().0;
    assert_eq!(decoded.len(), 4);
    assert_eq!(decoded[0], "Hello");
    assert_eq!(decoded[1], "World");

    let decoded = encoded.deserialize::<Vec<String>>().unwrap().0;
    assert_eq!(decoded.as_slice(), as_.as_slice());

    // Decode just the second string
    let slice = encoded.slice(14..);
    let decoded: &str = slice.deserialize().unwrap().0;
    assert_eq!(decoded, "World");

    // As Value
    let v: Value<'_> = as_[..].into();
    assert_eq!(v.value_signature(), "as");
    let encoded = to_bytes(ctxt, &v).unwrap();
    assert_eq!(encoded.len(), 49);
    let v = encoded.deserialize().unwrap().0;
    if let Value::Array(array) = v {
        assert_eq!(*array.element_signature(), "s");
        assert_eq!(array.len(), 4);
        assert_eq!(array[0], Value::new("Hello"));
        assert_eq!(array[1], Value::new("World"));
    } else {
        panic!();
    }

    let v: Value<'_> = as_[..].into();
    let a: Array<'_> = v.try_into().unwrap();
    let _ve: Vec<String> = a.try_into().unwrap();

    // GVariant format now
    #[cfg(feature = "gvariant")]
    {
        let ctxt = Context::new_gvariant(LE, 0);
        let v: Value<'_> = as_[..].into();
        let gv_encoded = to_bytes(ctxt, &v).unwrap();
        assert_eq!(gv_encoded.len(), 28);
    }

    // Array of Struct, which in turn containin an Array (We gotta go deeper!)
    // Signature: "a(yu(xbxas)s)");
    let ar = vec![(
        // top-most simple fields
        u8::MAX,
        u32::MAX,
        (
            // 2nd level simple fields
            i64::MAX,
            true,
            i64::MAX,
            // 2nd level array field
            &["Hello", "World"][..],
        ),
        // one more top-most simple field
        "hello",
    )];
    let ctxt = Context::new_dbus(LE, 0);
    let encoded = to_bytes(ctxt, &ar).unwrap();
    assert_eq!(encoded.len(), 78);
    #[allow(clippy::type_complexity)]
    let decoded: Vec<(u8, u32, (i64, bool, i64, Vec<&str>), &str)> =
        encoded.deserialize().unwrap().0;
    assert_eq!(decoded.len(), 1);
    let r = &decoded[0];
    assert_eq!(r.0, u8::MAX);
    assert_eq!(r.1, u32::MAX);
    let inner_r = &r.2;
    assert_eq!(inner_r.0, i64::MAX);
    assert!(inner_r.1);
    assert_eq!(inner_r.2, i64::MAX);
    let as_ = &inner_r.3;
    assert_eq!(as_.len(), 2);
    assert_eq!(as_[0], "Hello");
    assert_eq!(as_[1], "World");
    assert_eq!(r.3, "hello");

    // GVariant format now
    #[cfg(feature = "gvariant")]
    {
        let ctxt = Context::new_gvariant(LE, 0);
        let gv_encoded = to_bytes(ctxt, &ar).unwrap();
        assert_eq!(gv_encoded.len(), 54);
        #[allow(clippy::type_complexity)]
        let decoded: Vec<(u8, u32, (i64, bool, i64, Vec<&str>), &str)> =
            gv_encoded.deserialize().unwrap().0;
        assert_eq!(decoded.len(), 1);
        let r = &decoded[0];
        assert_eq!(r.0, u8::MAX);
        assert_eq!(r.1, u32::MAX);
        let inner_r = &r.2;
        assert_eq!(inner_r.0, i64::MAX);
        assert!(inner_r.1);
        assert_eq!(inner_r.2, i64::MAX);
        let as_ = &inner_r.3;
        assert_eq!(as_.len(), 2);
        assert_eq!(as_[0], "Hello");
        assert_eq!(as_[1], "World");
        assert_eq!(r.3, "hello");
    }
    let ctxt = Context::new_dbus(LE, 0);

    // As Value
    let v: Value<'_> = ar[..].into();
    assert_eq!(v.value_signature(), "a(yu(xbxas)s)");
    let encoded = to_bytes(ctxt, &v).unwrap();
    assert_eq!(encoded.len(), 94);
    let v = encoded.deserialize::<Value<'_>>().unwrap().0;
    if let Value::Array(array) = v.try_clone().unwrap() {
        assert_eq!(*array.element_signature(), "(yu(xbxas)s)");
        assert_eq!(array.len(), 1);
        let r = &array[0];
        if let Value::Structure(r) = r {
            let fields = r.fields();
            assert_eq!(fields[0], Value::U8(u8::MAX));
            assert_eq!(fields[1], Value::U32(u32::MAX));
            if let Value::Structure(r) = &fields[2] {
                let fields = r.fields();
                assert_eq!(fields[0], Value::I64(i64::MAX));
                assert_eq!(fields[1], Value::Bool(true));
                assert_eq!(fields[2], Value::I64(i64::MAX));
                if let Value::Array(as_) = &fields[3] {
                    assert_eq!(as_.len(), 2);
                    assert_eq!(as_[0], Value::new("Hello"));
                    assert_eq!(as_[1], Value::new("World"));
                } else {
                    panic!();
                }
            } else {
                panic!();
            }
            assert_eq!(fields[3], Value::new("hello"));
        } else {
            panic!();
        }
    } else {
        panic!();
    }

    // GVariant format now
    #[cfg(feature = "gvariant")]
    {
        let ctxt = Context::new_gvariant(LE, 0);
        let gv_encoded = to_bytes(ctxt, &v).unwrap();
        assert_eq!(gv_encoded.len(), 68);
        let v: Value<'_> = gv_encoded.deserialize().unwrap().0;
        if let Value::Array(array) = v {
            assert_eq!(*array.element_signature(), "(yu(xbxas)s)");
            assert_eq!(array.len(), 1);
            let r = &array.get(0).unwrap().unwrap();
            if let Value::Structure(r) = r {
                let fields = r.fields();
                assert_eq!(fields[0], Value::U8(u8::MAX));
                assert_eq!(fields[1], Value::U32(u32::MAX));
                if let Value::Structure(r) = &fields[2] {
                    let fields = r.fields();
                    assert_eq!(fields[0], Value::I64(i64::MAX));
                    assert_eq!(fields[1], Value::Bool(true));
                    assert_eq!(fields[2], Value::I64(i64::MAX));
                    if let Value::Array(as_) = &fields[3] {
                        assert_eq!(as_.len(), 2);
                        assert_eq!(as_.get(0).unwrap(), Some("Hello"));
                        assert_eq!(as_.get(1).unwrap(), Some("World"));
                    } else {
                        panic!();
                    }
                } else {
                    panic!();
                }
                assert_eq!(fields[3], Value::new("hello"));
            } else {
                panic!();
            }
        } else {
            panic!();
        }

        let mut rng = fastrand::Rng::new();
        // Let's test GVariant ser/de of a 254 byte array with variable-width elements as to
        // ensure no problems with non-normal BS of GVariant.
        let as_ = vec![
            std::iter::repeat_with(|| rng.alphanumeric())
                .take(126)
                .collect::<String>(),
            std::iter::repeat_with(|| rng.alphanumeric())
                .take(126)
                .collect::<String>(),
        ];
        let gv_encoded = to_bytes(ctxt, &as_).unwrap();
        // 252 chars + 2 null terminator bytes doesn't leave room for 2 framing offset bytes so
        // a 2-byte offset is chosen by the serializer.
        assert_eq!(gv_encoded.len(), 258);

        // Check if our own deserializer does the right thing
        let as2: Vec<String> = gv_encoded.deserialize().unwrap().0;
        assert_eq!(as2, as_);

        // Test conversion of Array of Value to Vec<Value>
        let v = Value::new(vec![Value::new(43), Value::new("bonjour")]);
        let av = <Array<'_>>::try_from(v).unwrap();
        let av = <Vec<Value<'_>>>::try_from(av).unwrap();
        assert_eq!(av[0], Value::new(43));
        assert_eq!(av[1], Value::new("bonjour"));

        let vec = vec![1, 2];
        let val = Value::new(&vec);
        assert_eq!(TryInto::<Vec<i32>>::try_into(val).unwrap(), vec);
    }

    // Empty array should be treated as a unit type, which is encoded as a u8.
    assert_eq!(<[u64; 0]>::SIGNATURE, &zvariant::Signature::U8);
    let array: [u64; 0] = [];
    let encoded = to_bytes(ctxt, &array).unwrap();
    assert_eq!(encoded.len(), 1);
    assert_eq!(encoded[0], 0);
    let _decoded: [u64; 0] = encoded.deserialize().unwrap().0;
}
