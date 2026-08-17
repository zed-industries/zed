use crate::{tags, Index, Label, Result, Stream, Value};

/**
The absence of any meaningful value.
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Null;

impl Value for Null {
    fn stream<'sval, S: Stream<'sval> + ?Sized>(&'sval self, stream: &mut S) -> Result {
        stream.null()
    }
}

impl<T: Value> Value for Option<T> {
    fn stream<'a, S: Stream<'a> + ?Sized>(&'a self, stream: &mut S) -> Result {
        if let Some(some) = self {
            stream.tagged_begin(
                Some(&tags::RUST_OPTION_SOME),
                Some(&Label::new("Some").with_tag(&tags::VALUE_IDENT)),
                Some(&Index::new(1).with_tag(&tags::VALUE_OFFSET)),
            )?;

            stream.value(some)?;

            stream.tagged_end(
                Some(&tags::RUST_OPTION_SOME),
                Some(&Label::new("Some").with_tag(&tags::VALUE_IDENT)),
                Some(&Index::new(1).with_tag(&tags::VALUE_OFFSET)),
            )
        } else {
            stream.tag(
                Some(&tags::RUST_OPTION_NONE),
                Some(&Label::new("None").with_tag(&tags::VALUE_IDENT)),
                Some(&Index::new(0).with_tag(&tags::VALUE_OFFSET)),
            )
        }
    }

    #[inline]
    fn to_bool(&self) -> Option<bool> {
        self.as_ref().and_then(|value| value.to_bool())
    }

    #[inline]
    fn to_f32(&self) -> Option<f32> {
        self.as_ref().and_then(|value| value.to_f32())
    }

    #[inline]
    fn to_f64(&self) -> Option<f64> {
        self.as_ref().and_then(|value| value.to_f64())
    }

    #[inline]
    fn to_i8(&self) -> Option<i8> {
        self.as_ref().and_then(|value| value.to_i8())
    }

    #[inline]
    fn to_i16(&self) -> Option<i16> {
        self.as_ref().and_then(|value| value.to_i16())
    }

    #[inline]
    fn to_i32(&self) -> Option<i32> {
        self.as_ref().and_then(|value| value.to_i32())
    }

    #[inline]
    fn to_i64(&self) -> Option<i64> {
        self.as_ref().and_then(|value| value.to_i64())
    }

    #[inline]
    fn to_i128(&self) -> Option<i128> {
        self.as_ref().and_then(|value| value.to_i128())
    }

    #[inline]
    fn to_u8(&self) -> Option<u8> {
        self.as_ref().and_then(|value| value.to_u8())
    }

    #[inline]
    fn to_u16(&self) -> Option<u16> {
        self.as_ref().and_then(|value| value.to_u16())
    }

    #[inline]
    fn to_u32(&self) -> Option<u32> {
        self.as_ref().and_then(|value| value.to_u32())
    }

    #[inline]
    fn to_u64(&self) -> Option<u64> {
        self.as_ref().and_then(|value| value.to_u64())
    }

    #[inline]
    fn to_u128(&self) -> Option<u128> {
        self.as_ref().and_then(|value| value.to_u128())
    }

    #[inline]
    fn to_text(&self) -> Option<&str> {
        self.as_ref().and_then(|value| value.to_text())
    }

    #[inline]
    fn to_binary(&self) -> Option<&[u8]> {
        self.as_ref().and_then(|value| value.to_binary())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn option_cast() {
        assert_eq!(Some(1u8), Some(1u8).to_u8());
        assert_eq!(Some(2u16), Some(2u16).to_u16());
        assert_eq!(Some(3u32), Some(3u32).to_u32());
        assert_eq!(Some(4u64), Some(4u64).to_u64());
        assert_eq!(Some(42u128), Some(42u128).to_u128());

        assert_eq!(Some(1i8), Some(1i8).to_i8());
        assert_eq!(Some(2i16), Some(2i16).to_i16());
        assert_eq!(Some(3i32), Some(3i32).to_i32());
        assert_eq!(Some(4i64), Some(4i64).to_i64());
        assert_eq!(Some(42i128), Some(42i128).to_i128());

        assert_eq!(Some(3f32), Some(3f32).to_f32());
        assert_eq!(Some(4f64), Some(4f64).to_f64());

        assert_eq!(Some(true), Some(true).to_bool());

        assert_eq!(Some("a string"), Some("a string").to_text());
    }

    #[test]
    fn option_tag() {
        assert_eq!(Some(tags::RUST_OPTION_SOME), Some(42).tag());
        assert_eq!(Some(tags::RUST_OPTION_NONE), None::<i32>.tag());
    }
}
