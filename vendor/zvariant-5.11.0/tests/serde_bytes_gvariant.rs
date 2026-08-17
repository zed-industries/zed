#[test]
#[cfg(all(feature = "serde_bytes", feature = "gvariant"))]
fn serde_bytes_gvariant() {
    use serde::{Deserialize, Serialize};
    use serde_bytes::*;
    use zvariant::{LE, Type, serialized::Context, to_bytes};

    #[macro_use]
    mod common {
        include!("common.rs");
    }

    let ctxt = Context::new_gvariant(LE, 0);
    let ay = Bytes::new(&[77u8; 1_000_000]);
    let encoded = to_bytes(ctxt, &ay).unwrap();
    assert_eq!(encoded.len(), 1_000_000);
    let decoded: ByteBuf = encoded.deserialize().unwrap().0;
    assert_eq!(decoded.len(), 1_000_000);

    #[derive(Deserialize, Serialize, Type, PartialEq, Debug)]
    struct Struct<'s> {
        field1: u16,
        #[serde(with = "serde_bytes")]
        field2: &'s [u8],
        field3: i64,
    }
    assert_eq!(Struct::SIGNATURE, "(qayx)");
    let s = Struct {
        field1: 0xFF_FF,
        field2: &[77u8; 512],
        field3: 0xFF_FF_FF_FF_FF_FF,
    };
    let encoded = to_bytes(ctxt, &s).unwrap();
    assert_eq!(encoded.len(), 530);
    let decoded: Struct<'_> = encoded.deserialize().unwrap().0;
    assert_eq!(decoded, s);
}
