use crate::{Index, Label, Result, Stream, Tag};

/**
A producer of structured data.

Each call to [`Value::stream`] is expected to produce a single complete value.
*/
pub trait Value {
    /**
    Stream this value through a [`Stream`].
    */
    fn stream<'sval, S: Stream<'sval> + ?Sized>(&'sval self, stream: &mut S) -> Result;

    /**
    Get the tag of this value, if there is one.
    */
    #[inline]
    fn tag(&self) -> Option<Tag> {
        default_value::tag(self)
    }

    /**
    Try convert this value into a boolean.
    */
    #[inline]
    fn to_bool(&self) -> Option<bool> {
        default_value::to_bool(self)
    }

    /**
    Try convert this value into a 32bit binary floating point number.
    */
    #[inline]
    fn to_f32(&self) -> Option<f32> {
        default_value::to_f32(self)
    }

    /**
    Try convert this value into a 64bit binary floating point number.
    */
    #[inline]
    fn to_f64(&self) -> Option<f64> {
        default_value::to_f64(self)
    }

    /**
    Try convert this value into a signed 8bit integer.
    */
    #[inline]
    fn to_i8(&self) -> Option<i8> {
        default_value::to_i8(self)
    }

    /**
    Try convert this value into a signed 16bit integer.
    */
    #[inline]
    fn to_i16(&self) -> Option<i16> {
        default_value::to_i16(self)
    }

    /**
    Try convert this value into a signed 32bit integer.
    */
    #[inline]
    fn to_i32(&self) -> Option<i32> {
        default_value::to_i32(self)
    }

    /**
    Try convert this value into a signed 64bit integer.
    */
    #[inline]
    fn to_i64(&self) -> Option<i64> {
        default_value::to_i64(self)
    }

    /**
    Try convert this value into a signed 128bit integer.
    */
    #[inline]
    fn to_i128(&self) -> Option<i128> {
        default_value::to_i128(self)
    }

    /**
    Try convert this value into an unsigned 8bit integer.
    */
    #[inline]
    fn to_u8(&self) -> Option<u8> {
        default_value::to_u8(self)
    }

    /**
    Try convert this value into an unsigned 16bit integer.
    */
    #[inline]
    fn to_u16(&self) -> Option<u16> {
        default_value::to_u16(self)
    }

    /**
    Try convert this value into an unsigned 32bit integer.
    */
    #[inline]
    fn to_u32(&self) -> Option<u32> {
        default_value::to_u32(self)
    }

    /**
    Try convert this value into an unsigned 64bit integer.
    */
    #[inline]
    fn to_u64(&self) -> Option<u64> {
        default_value::to_u64(self)
    }

    /**
    Try convert this value into an unsigned 128bit integer.
    */
    #[inline]
    fn to_u128(&self) -> Option<u128> {
        default_value::to_u128(self)
    }

    /**
    Try convert this value into a text string.
    */
    #[inline]
    fn to_text(&self) -> Option<&str> {
        default_value::to_text(self)
    }

    /**
    Try convert this value into a bitstring.
    */
    #[inline]
    fn to_binary(&self) -> Option<&[u8]> {
        default_value::to_binary(self)
    }
}

macro_rules! impl_value_forward {
    ({ $($r:tt)* } => $bind:ident => { $($forward:tt)* }) => {
        $($r)* {
            fn stream<'sval, S: Stream<'sval> + ?Sized>(&'sval self, stream: &mut S) -> Result {
                let $bind = self;
                ($($forward)*).stream(stream)
            }

            #[inline]
            fn tag(&self) -> Option<Tag> {
                let $bind = self;
                ($($forward)*).tag()
            }

            #[inline]
            fn to_bool(&self) -> Option<bool> {
                let $bind = self;
                ($($forward)*).to_bool()
            }

            #[inline]
            fn to_f32(&self) -> Option<f32> {
                let $bind = self;
                ($($forward)*).to_f32()
            }

            #[inline]
            fn to_f64(&self) -> Option<f64> {
                let $bind = self;
                ($($forward)*).to_f64()
            }

            #[inline]
            fn to_i8(&self) -> Option<i8> {
                let $bind = self;
                ($($forward)*).to_i8()
            }

            #[inline]
            fn to_i16(&self) -> Option<i16> {
                let $bind = self;
                ($($forward)*).to_i16()
            }

            #[inline]
            fn to_i32(&self) -> Option<i32> {
                let $bind = self;
                ($($forward)*).to_i32()
            }

            #[inline]
            fn to_i64(&self) -> Option<i64> {
                let $bind = self;
                ($($forward)*).to_i64()
            }

            #[inline]
            fn to_i128(&self) -> Option<i128> {
                let $bind = self;
                ($($forward)*).to_i128()
            }

            #[inline]
            fn to_u8(&self) -> Option<u8> {
                let $bind = self;
                ($($forward)*).to_u8()
            }

            #[inline]
            fn to_u16(&self) -> Option<u16> {
                let $bind = self;
                ($($forward)*).to_u16()
            }

            #[inline]
            fn to_u32(&self) -> Option<u32> {
                let $bind = self;
                ($($forward)*).to_u32()
            }

            #[inline]
            fn to_u64(&self) -> Option<u64> {
                let $bind = self;
                ($($forward)*).to_u64()
            }

            #[inline]
            fn to_u128(&self) -> Option<u128> {
                let $bind = self;
                ($($forward)*).to_u128()
            }

            #[inline]
            fn to_text(&self) -> Option<&str> {
                let $bind = self;
                ($($forward)*).to_text()
            }

            #[inline]
            fn to_binary(&self) -> Option<&[u8]> {
                let $bind = self;
                ($($forward)*).to_binary()
            }
        }
    };
}

impl_value_forward!({impl<'a, T: Value + ?Sized> Value for &'a T} => x => { **x });

#[cfg(feature = "alloc")]
mod alloc_support {
    use super::*;

    use crate::std::boxed::Box;

    impl_value_forward!({impl<T: Value + ?Sized> Value for Box<T>} => x => { **x });
}

pub mod default_value {
    /*!
    Default method implementations for [`Value`]s.
    */

    use super::*;

    /**
    Get the tag of this value, if there is one.
    */
    pub fn tag(value: &(impl Value + ?Sized)) -> Option<Tag> {
        struct Extract {
            value: Option<Tag>,
            set: bool,
        }

        impl Extract {
            fn set(&mut self, tag: Option<&Tag>) -> Result {
                if self.set {
                    return crate::error();
                }

                self.set = true;
                self.value = tag.cloned();

                Ok(())
            }
        }

        impl<'sval> Stream<'sval> for Extract {
            fn tag(&mut self, tag: Option<&Tag>, _: Option<&Label>, _: Option<&Index>) -> Result {
                self.set(tag)
            }

            fn tagged_begin(
                &mut self,
                tag: Option<&Tag>,
                _: Option<&Label>,
                _: Option<&Index>,
            ) -> Result {
                self.set(tag)
            }

            fn enum_begin(
                &mut self,
                tag: Option<&Tag>,
                _: Option<&Label>,
                _: Option<&Index>,
            ) -> Result {
                self.set(tag)
            }

            fn record_begin(
                &mut self,
                tag: Option<&Tag>,
                _: Option<&Label>,
                _: Option<&Index>,
                _: Option<usize>,
            ) -> Result {
                self.set(tag)
            }

            fn tuple_begin(
                &mut self,
                tag: Option<&Tag>,
                _: Option<&Label>,
                _: Option<&Index>,
                _: Option<usize>,
            ) -> Result {
                self.set(tag)
            }

            fn bool(&mut self, _: bool) -> Result {
                crate::error()
            }

            fn null(&mut self) -> Result {
                crate::error()
            }

            fn text_begin(&mut self, _: Option<usize>) -> Result {
                crate::error()
            }

            fn text_fragment_computed(&mut self, _: &str) -> Result {
                crate::error()
            }

            fn text_end(&mut self) -> Result {
                crate::error()
            }

            fn i64(&mut self, _: i64) -> Result {
                self.set(Some(&crate::tags::NUMBER))
            }

            fn f64(&mut self, _: f64) -> Result {
                self.set(Some(&crate::tags::NUMBER))
            }

            fn seq_begin(&mut self, _: Option<usize>) -> Result {
                crate::error()
            }

            fn seq_value_begin(&mut self) -> Result {
                crate::error()
            }

            fn seq_value_end(&mut self) -> Result {
                crate::error()
            }

            fn seq_end(&mut self) -> Result {
                crate::error()
            }
        }

        let mut extract = Extract {
            value: None,
            set: false,
        };

        let _ = value.stream(&mut extract);

        extract.value
    }

    /**
    Try convert this value into a boolean.
    */
    pub fn to_bool(value: &(impl Value + ?Sized)) -> Option<bool> {
        struct Extract(Option<bool>);

        impl<'sval> Stream<'sval> for Extract {
            fn bool(&mut self, value: bool) -> Result {
                self.0 = Some(value);
                Ok(())
            }

            fn null(&mut self) -> Result {
                crate::error()
            }

            fn text_begin(&mut self, _: Option<usize>) -> Result {
                crate::error()
            }

            fn text_fragment_computed(&mut self, _: &str) -> Result {
                crate::error()
            }

            fn text_end(&mut self) -> Result {
                crate::error()
            }

            fn i64(&mut self, _: i64) -> Result {
                crate::error()
            }

            fn f64(&mut self, _: f64) -> Result {
                crate::error()
            }

            fn seq_begin(&mut self, _: Option<usize>) -> Result {
                crate::error()
            }

            fn seq_value_begin(&mut self) -> Result {
                crate::error()
            }

            fn seq_value_end(&mut self) -> Result {
                crate::error()
            }

            fn seq_end(&mut self) -> Result {
                crate::error()
            }
        }

        let mut extract = Extract(None);
        value.stream(&mut extract).ok()?;
        extract.0
    }

    /**
    Try convert this value into a 32bit binary floating point number.
    */
    pub fn to_f32(value: &(impl Value + ?Sized)) -> Option<f32> {
        struct Extract(Option<f32>);

        impl<'sval> Stream<'sval> for Extract {
            fn f32(&mut self, value: f32) -> Result {
                self.0 = Some(value);
                Ok(())
            }

            fn null(&mut self) -> Result {
                crate::error()
            }

            fn bool(&mut self, _: bool) -> Result {
                crate::error()
            }

            fn text_begin(&mut self, _: Option<usize>) -> Result {
                crate::error()
            }

            fn text_fragment_computed(&mut self, _: &str) -> Result {
                crate::error()
            }

            fn text_end(&mut self) -> Result {
                crate::error()
            }

            fn i64(&mut self, _: i64) -> Result {
                crate::error()
            }

            fn f64(&mut self, _: f64) -> Result {
                crate::error()
            }

            fn seq_begin(&mut self, _: Option<usize>) -> Result {
                crate::error()
            }

            fn seq_value_begin(&mut self) -> Result {
                crate::error()
            }

            fn seq_value_end(&mut self) -> Result {
                crate::error()
            }

            fn seq_end(&mut self) -> Result {
                crate::error()
            }
        }

        let mut extract = Extract(None);
        value.stream(&mut extract).ok()?;
        extract.0
    }

    /**
    Try convert this value into a 64bit binary floating point number.
    */
    pub fn to_f64(value: &(impl Value + ?Sized)) -> Option<f64> {
        struct Extract(Option<f64>);

        impl<'sval> Stream<'sval> for Extract {
            fn f64(&mut self, value: f64) -> Result {
                self.0 = Some(value);
                Ok(())
            }

            fn null(&mut self) -> Result {
                crate::error()
            }

            fn bool(&mut self, _: bool) -> Result {
                crate::error()
            }

            fn text_begin(&mut self, _: Option<usize>) -> Result {
                crate::error()
            }

            fn text_fragment_computed(&mut self, _: &str) -> Result {
                crate::error()
            }

            fn text_end(&mut self) -> Result {
                crate::error()
            }

            fn i64(&mut self, _: i64) -> Result {
                crate::error()
            }

            fn seq_begin(&mut self, _: Option<usize>) -> Result {
                crate::error()
            }

            fn seq_value_begin(&mut self) -> Result {
                crate::error()
            }

            fn seq_value_end(&mut self) -> Result {
                crate::error()
            }

            fn seq_end(&mut self) -> Result {
                crate::error()
            }
        }

        let mut extract = Extract(None);
        value.stream(&mut extract).ok()?;
        extract.0
    }

    /**
    Try convert this value into a signed 8bit integer.
    */
    pub fn to_i8(value: &(impl Value + ?Sized)) -> Option<i8> {
        value.to_i128().and_then(|value| value.try_into().ok())
    }

    /**
    Try convert this value into a signed 16bit integer.
    */
    pub fn to_i16(value: &(impl Value + ?Sized)) -> Option<i16> {
        value.to_i128().and_then(|value| value.try_into().ok())
    }

    /**
    Try convert this value into a signed 32bit integer.
    */
    pub fn to_i32(value: &(impl Value + ?Sized)) -> Option<i32> {
        value.to_i128().and_then(|value| value.try_into().ok())
    }

    /**
    Try convert this value into a signed 64bit integer.
    */
    pub fn to_i64(value: &(impl Value + ?Sized)) -> Option<i64> {
        value.to_i128().and_then(|value| value.try_into().ok())
    }

    /**
    Try convert this value into a signed 128bit integer.
    */
    pub fn to_i128(value: &(impl Value + ?Sized)) -> Option<i128> {
        struct Extract(Option<i128>);

        impl<'sval> Stream<'sval> for Extract {
            fn i128(&mut self, value: i128) -> Result {
                self.0 = Some(value);
                Ok(())
            }

            fn null(&mut self) -> Result {
                crate::error()
            }

            fn bool(&mut self, _: bool) -> Result {
                crate::error()
            }

            fn text_begin(&mut self, _: Option<usize>) -> Result {
                crate::error()
            }

            fn text_fragment_computed(&mut self, _: &str) -> Result {
                crate::error()
            }

            fn text_end(&mut self) -> Result {
                crate::error()
            }

            fn i64(&mut self, _: i64) -> Result {
                crate::error()
            }

            fn f64(&mut self, _: f64) -> Result {
                crate::error()
            }

            fn seq_begin(&mut self, _: Option<usize>) -> Result {
                crate::error()
            }

            fn seq_value_begin(&mut self) -> Result {
                crate::error()
            }

            fn seq_value_end(&mut self) -> Result {
                crate::error()
            }

            fn seq_end(&mut self) -> Result {
                crate::error()
            }
        }

        let mut extract = Extract(None);
        value.stream(&mut extract).ok()?;
        extract.0
    }

    /**
    Try convert this value into an unsigned 8bit integer.
    */
    pub fn to_u8(value: &(impl Value + ?Sized)) -> Option<u8> {
        value.to_u128().and_then(|value| value.try_into().ok())
    }

    /**
    Try convert this value into an unsigned 16bit integer.
    */
    pub fn to_u16(value: &(impl Value + ?Sized)) -> Option<u16> {
        value.to_u128().and_then(|value| value.try_into().ok())
    }

    /**
    Try convert this value into an unsigned 32bit integer.
    */
    pub fn to_u32(value: &(impl Value + ?Sized)) -> Option<u32> {
        value.to_u128().and_then(|value| value.try_into().ok())
    }

    /**
    Try convert this value into an unsigned 64bit integer.
    */
    pub fn to_u64(value: &(impl Value + ?Sized)) -> Option<u64> {
        value.to_u128().and_then(|value| value.try_into().ok())
    }

    /**
    Try convert this value into an unsigned 128bit integer.
    */
    pub fn to_u128(value: &(impl Value + ?Sized)) -> Option<u128> {
        struct Extract(Option<u128>);

        impl<'sval> Stream<'sval> for Extract {
            fn u128(&mut self, value: u128) -> Result {
                self.0 = Some(value);
                Ok(())
            }

            fn null(&mut self) -> Result {
                crate::error()
            }

            fn bool(&mut self, _: bool) -> Result {
                crate::error()
            }

            fn text_begin(&mut self, _: Option<usize>) -> Result {
                crate::error()
            }

            fn text_fragment_computed(&mut self, _: &str) -> Result {
                crate::error()
            }

            fn text_end(&mut self) -> Result {
                crate::error()
            }

            fn i64(&mut self, _: i64) -> Result {
                crate::error()
            }

            fn f64(&mut self, _: f64) -> Result {
                crate::error()
            }

            fn seq_begin(&mut self, _: Option<usize>) -> Result {
                crate::error()
            }

            fn seq_value_begin(&mut self) -> Result {
                crate::error()
            }

            fn seq_value_end(&mut self) -> Result {
                crate::error()
            }

            fn seq_end(&mut self) -> Result {
                crate::error()
            }
        }

        let mut extract = Extract(None);
        value.stream(&mut extract).ok()?;
        extract.0
    }

    /**
    Try convert this value into a text string.
    */
    pub fn to_text(value: &(impl Value + ?Sized)) -> Option<&str> {
        struct Extract<'sval> {
            extracted: Option<&'sval str>,
            seen_fragment: bool,
        }

        impl<'sval> Stream<'sval> for Extract<'sval> {
            fn text_begin(&mut self, _: Option<usize>) -> Result {
                Ok(())
            }

            fn text_fragment(&mut self, fragment: &'sval str) -> Result {
                // Allow either independent strings, or fragments of a single borrowed string
                if !self.seen_fragment {
                    self.extracted = Some(fragment);
                    self.seen_fragment = true;
                } else {
                    self.extracted = None;
                }

                Ok(())
            }

            fn text_fragment_computed(&mut self, _: &str) -> Result {
                self.extracted = None;
                self.seen_fragment = true;

                crate::error()
            }

            fn text_end(&mut self) -> Result {
                Ok(())
            }

            fn null(&mut self) -> Result {
                crate::error()
            }

            fn bool(&mut self, _: bool) -> Result {
                crate::error()
            }

            fn i64(&mut self, _: i64) -> Result {
                crate::error()
            }

            fn f64(&mut self, _: f64) -> Result {
                crate::error()
            }

            fn seq_begin(&mut self, _: Option<usize>) -> Result {
                crate::error()
            }

            fn seq_value_begin(&mut self) -> Result {
                crate::error()
            }

            fn seq_value_end(&mut self) -> Result {
                crate::error()
            }

            fn seq_end(&mut self) -> Result {
                crate::error()
            }
        }

        let mut extract = Extract {
            extracted: None,
            seen_fragment: false,
        };

        value.stream(&mut extract).ok()?;
        extract.extracted
    }

    /**
    Try convert this value into a bitstring.
    */
    pub fn to_binary(value: &(impl Value + ?Sized)) -> Option<&[u8]> {
        struct Extract<'sval> {
            extracted: Option<&'sval [u8]>,
            seen_fragment: bool,
        }

        impl<'sval> Stream<'sval> for Extract<'sval> {
            fn binary_begin(&mut self, _: Option<usize>) -> Result {
                Ok(())
            }

            fn binary_fragment(&mut self, fragment: &'sval [u8]) -> Result {
                // Allow either independent bytes, or fragments of a single borrowed byte stream
                if !self.seen_fragment {
                    self.extracted = Some(fragment);
                    self.seen_fragment = true;
                } else {
                    self.extracted = None;
                }

                Ok(())
            }

            fn binary_fragment_computed(&mut self, _: &[u8]) -> Result {
                self.extracted = None;
                self.seen_fragment = true;

                crate::error()
            }

            fn binary_end(&mut self) -> Result {
                Ok(())
            }

            fn null(&mut self) -> Result {
                crate::error()
            }

            fn bool(&mut self, _: bool) -> Result {
                crate::error()
            }

            fn text_begin(&mut self, _: Option<usize>) -> Result {
                crate::error()
            }

            fn text_fragment_computed(&mut self, _: &str) -> Result {
                crate::error()
            }

            fn text_end(&mut self) -> Result {
                crate::error()
            }

            fn i64(&mut self, _: i64) -> Result {
                crate::error()
            }

            fn f64(&mut self, _: f64) -> Result {
                crate::error()
            }

            fn seq_begin(&mut self, _: Option<usize>) -> Result {
                crate::error()
            }

            fn seq_value_begin(&mut self) -> Result {
                crate::error()
            }

            fn seq_value_end(&mut self) -> Result {
                crate::error()
            }

            fn seq_end(&mut self) -> Result {
                crate::error()
            }
        }

        let mut extract = Extract {
            extracted: None,
            seen_fragment: false,
        };

        value.stream(&mut extract).ok()?;
        extract.extracted
    }
}
