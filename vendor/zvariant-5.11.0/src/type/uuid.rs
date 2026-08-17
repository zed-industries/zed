use crate::impl_type_with_repr;

impl_type_with_repr! {
    uuid::Uuid => &[u8] {
        uuid_ {
            signature = "ay",
            samples = [uuid::Uuid::parse_str("a1a2a3a4-b1b2-c1c2-d1d2-d3d4d5d6d7d8").unwrap()],
            repr(u) = u.as_bytes(),
        }
    }
}
