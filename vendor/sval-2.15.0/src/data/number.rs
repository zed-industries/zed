use crate::{std::fmt, tags, Result, Stream, Value};

macro_rules! stream_default {
    ($($fi:ident => $i:ty,)*) => {
        $(
            pub(crate) fn $fi<'sval>(v: $i, stream: &mut (impl Stream<'sval> + ?Sized)) -> crate::Result {
                stream_number(stream, v)
            }
        )*
    };
}

macro_rules! impl_value {
    ($(
        $convert:ident => $ty:ident,
    )+) => {
        $(
            impl Value for $ty {
                fn stream<'sval, S: Stream<'sval> + ?Sized>(&'sval self, stream: &mut S) -> crate::Result {
                    stream.$ty(*self)
                }

                fn $convert(&self) -> Option<$ty> {
                    Some(*self)
                }
            }
        )+
    };
}

stream_default!(
    stream_i128 => i128,
    stream_u128 => u128,
    stream_f64 => f64,
);

impl_value!(
    to_u8 => u8,
    to_u16 => u16,
    to_u32 => u32,
    to_u64 => u64,
    to_u128 => u128,
    to_i8 => i8,
    to_i16 => i16,
    to_i32 => i32,
    to_i64 => i64,
    to_i128 => i128,
    to_f32 => f32,
    to_f64 => f64,
);

/**
Stream an arbitrary precision number conforming to [`tags::NUMBER`]
using its [`fmt::Display`] implementation.
*/
pub fn stream_number<'sval>(
    stream: &mut (impl Stream<'sval> + ?Sized),
    number: impl fmt::Display,
) -> Result {
    stream.tagged_begin(Some(&tags::NUMBER), None, None)?;
    stream.value_computed(&crate::Display::new(number))?;
    stream.tagged_end(Some(&tags::NUMBER), None, None)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn number_cast() {
        assert_eq!(Some(1u8), 1u8.to_u8());
        assert_eq!(Some(2u16), 2u16.to_u16());
        assert_eq!(Some(3u32), 3u32.to_u32());
        assert_eq!(Some(4u64), 4u64.to_u64());
        assert_eq!(Some(42u128), 42u128.to_u128());

        assert_eq!(Some(1i8), 1i8.to_i8());
        assert_eq!(Some(2i16), 2i16.to_i16());
        assert_eq!(Some(3i32), 3i32.to_i32());
        assert_eq!(Some(4i64), 4i64.to_i64());
        assert_eq!(Some(42i128), 42i128.to_i128());

        assert_eq!(Some(3f32), 3f32.to_f32());
        assert_eq!(Some(4f64), 4f64.to_f64());
    }

    #[test]
    fn number_tag() {
        struct Number(&'static str);

        impl Value for Number {
            fn stream<'sval, S: Stream<'sval> + ?Sized>(&'sval self, stream: &mut S) -> Result {
                stream.tagged_begin(Some(&tags::NUMBER), None, None)?;
                stream.value(self.0)?;
                stream.tagged_end(Some(&tags::NUMBER), None, None)
            }
        }

        assert_eq!(Some(tags::NUMBER), 1u8.tag());
        assert_eq!(Some(tags::NUMBER), 1u16.tag());
        assert_eq!(Some(tags::NUMBER), 1u32.tag());
        assert_eq!(Some(tags::NUMBER), 1u64.tag());
        assert_eq!(Some(tags::NUMBER), 1u128.tag());

        assert_eq!(Some(tags::NUMBER), 1i8.tag());
        assert_eq!(Some(tags::NUMBER), 1i16.tag());
        assert_eq!(Some(tags::NUMBER), 1i32.tag());
        assert_eq!(Some(tags::NUMBER), 1i64.tag());
        assert_eq!(Some(tags::NUMBER), 1i128.tag());

        assert_eq!(Some(tags::NUMBER), 1f32.tag());
        assert_eq!(Some(tags::NUMBER), 1f64.tag());

        assert_eq!(Some(tags::NUMBER), Number("42").tag());
    }
}
