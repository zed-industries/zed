use super::*;

#[doc(hidden)]
pub trait RuntimeType: Type<Self> {
    const SIGNATURE: imp::ConstBuffer;
}

macro_rules! primitives {
    ($(($t:ty, $s:literal)),+) => {
        $(
            impl RuntimeType for $t {
                const SIGNATURE: imp::ConstBuffer = imp::ConstBuffer::from_slice($s);
            }
        )*
    };
}

primitives! {
    (bool, b"b1"),
    (i8, b"i1"),
    (u8, b"u1"),
    (i16, b"i2"),
    (u16, b"u2"),
    (i32, b"i4"),
    (u32, b"u4"),
    (i64, b"i8"),
    (u64, b"u8"),
    (f32, b"f4"),
    (f64, b"f8")
}
