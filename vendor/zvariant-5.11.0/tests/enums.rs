use zvariant::{serialized::Context, to_bytes_for_signature};

#[macro_use]
mod common {
    include!("common.rs");
}

#[test]
fn enums() {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
    enum Unit {
        Variant1,
        Variant2,
        Variant3,
    }

    let ctxts_n_expected_lens = [
        // Unit variants are encoded as u32 and that has the same encoding in both formats.
        [
            (Context::new_dbus(zvariant::BE, 0), 4usize),
            (Context::new_dbus(zvariant::BE, 1), 7),
            (Context::new_dbus(zvariant::BE, 2), 6),
            (Context::new_dbus(zvariant::BE, 3), 5),
            (Context::new_dbus(zvariant::BE, 4), 4),
        ],
        #[cfg(feature = "gvariant")]
        [
            (Context::new_gvariant(zvariant::BE, 0), 4usize),
            (Context::new_gvariant(zvariant::BE, 1), 7),
            (Context::new_gvariant(zvariant::BE, 2), 6),
            (Context::new_gvariant(zvariant::BE, 3), 5),
            (Context::new_gvariant(zvariant::BE, 4), 4),
        ],
    ];
    for ctxts_n_expected_len in ctxts_n_expected_lens {
        for (ctxt, expected_len) in ctxts_n_expected_len {
            let encoded = to_bytes_for_signature(ctxt, "u", &Unit::Variant2).unwrap();
            assert_eq!(encoded.len(), expected_len);
            let decoded: Unit = encoded.deserialize_for_signature("u").unwrap().0;
            assert_eq!(decoded, Unit::Variant2);
        }
    }

    #[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
    enum NewType<'s> {
        Variant1(&'s str),
        Variant2(&'s str),
        Variant3(&'s str),
    }

    let ctxts_n_expected_lens = [
        [
            (Context::new_dbus(zvariant::BE, 0), 14usize),
            (Context::new_dbus(zvariant::BE, 1), 21),
            (Context::new_dbus(zvariant::BE, 2), 20),
            (Context::new_dbus(zvariant::BE, 3), 19),
            (Context::new_dbus(zvariant::BE, 4), 18),
        ],
        #[cfg(feature = "gvariant")]
        [
            (Context::new_gvariant(zvariant::BE, 0), 10usize),
            (Context::new_gvariant(zvariant::BE, 1), 13),
            (Context::new_gvariant(zvariant::BE, 2), 12),
            (Context::new_gvariant(zvariant::BE, 3), 11),
            (Context::new_gvariant(zvariant::BE, 4), 10),
        ],
    ];
    for ctxts_n_expected_len in ctxts_n_expected_lens {
        for (ctxt, expected_len) in ctxts_n_expected_len {
            let encoded =
                to_bytes_for_signature(ctxt, "(us)", &NewType::Variant2("hello")).unwrap();
            assert_eq!(encoded.len(), expected_len);
            let decoded: NewType<'_> = encoded.deserialize_for_signature("(us)").unwrap().0;
            assert_eq!(decoded, NewType::Variant2("hello"));
        }
    }

    #[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
    enum Structs {
        Tuple(u8, u32),
        Struct { y: u8, t: u32 },
    }

    let ctxts_n_expected_lens = [
        [
            (Context::new_dbus(zvariant::BE, 0), 16usize),
            (Context::new_dbus(zvariant::BE, 1), 23),
            (Context::new_dbus(zvariant::BE, 2), 22),
            (Context::new_dbus(zvariant::BE, 3), 21),
            (Context::new_dbus(zvariant::BE, 4), 20),
        ],
        #[cfg(feature = "gvariant")]
        [
            (Context::new_gvariant(zvariant::BE, 0), 12usize),
            (Context::new_gvariant(zvariant::BE, 1), 15),
            (Context::new_gvariant(zvariant::BE, 2), 14),
            (Context::new_gvariant(zvariant::BE, 3), 13),
            (Context::new_gvariant(zvariant::BE, 4), 12),
        ],
    ];
    // TODO: Provide convenience API to create complex signatures
    let signature = "(u(yu))";
    for ctxts_n_expected_len in ctxts_n_expected_lens {
        for (ctxt, expected_len) in ctxts_n_expected_len {
            let encoded = to_bytes_for_signature(ctxt, signature, &Structs::Tuple(42, 42)).unwrap();
            assert_eq!(encoded.len(), expected_len);
            let decoded: Structs = encoded.deserialize_for_signature(signature).unwrap().0;
            assert_eq!(decoded, Structs::Tuple(42, 42));

            let s = Structs::Struct { y: 42, t: 42 };
            let encoded = to_bytes_for_signature(ctxt, signature, &s).unwrap();
            assert_eq!(encoded.len(), expected_len);
            let decoded: Structs = encoded.deserialize_for_signature(signature).unwrap().0;
            assert_eq!(decoded, Structs::Struct { y: 42, t: 42 });
        }
    }
}
