use crate::{
    fill::Slot,
    internal::{Internal, InternalVisitor},
    std::{any::Any, fmt, marker::PhantomData, mem, ops::ControlFlow},
    Error, ValueBag,
};

#[cfg(feature = "alloc")]
use crate::std::{string::String, vec::Vec};

impl<'v> ValueBag<'v> {
    /// Get a value from a sequence of values without capturing support.
    pub fn from_seq_slice<I, T>(value: &'v I) -> Self
    where
        I: AsRef<[T]>,
        &'v T: Into<ValueBag<'v>> + 'v,
    {
        ValueBag {
            inner: Internal::AnonSeq(SeqSlice::new_ref(value)),
        }
    }

    pub(crate) const fn from_dyn_seq(value: &'v dyn Seq) -> Self {
        ValueBag {
            inner: Internal::AnonSeq(value),
        }
    }

    /// Try get a collection `S` of `u64`s from this value.
    ///
    /// If this value is a sequence then the collection `S` will be extended
    /// with the attempted conversion of each of its elements.
    ///
    /// If this value is not a sequence then this method will return `None`.
    pub fn to_u64_seq<S: Default + Extend<Option<u64>>>(&self) -> Option<S> {
        self.inner
            .extend::<ExtendPrimitive<S, u64>>()
            .map(|seq| seq.into_inner())
    }

    /// Try get a collection `S` of `i64`s from this value.
    ///
    /// If this value is a sequence then the collection `S` will be extended
    /// with the attempted conversion of each of its elements.
    ///
    /// If this value is not a sequence then this method will return `None`.
    pub fn to_i64_seq<S: Default + Extend<Option<i64>>>(&self) -> Option<S> {
        self.inner
            .extend::<ExtendPrimitive<S, i64>>()
            .map(|seq| seq.into_inner())
    }

    /// Try get a collection `S` of `u128`s from this value.
    ///
    /// If this value is a sequence then the collection `S` will be extended
    /// with the attempted conversion of each of its elements.
    ///
    /// If this value is not a sequence then this method will return `None`.
    pub fn to_u128_seq<S: Default + Extend<Option<u128>>>(&self) -> Option<S> {
        self.inner
            .extend::<ExtendPrimitive<S, u128>>()
            .map(|seq| seq.into_inner())
    }

    /// Try get a collection `S` of `i128`s from this value.
    ///
    /// If this value is a sequence then the collection `S` will be extended
    /// with the attempted conversion of each of its elements.
    ///
    /// If this value is not a sequence then this method will return `None`.
    pub fn to_i128_seq<S: Default + Extend<Option<i128>>>(&self) -> Option<S> {
        self.inner
            .extend::<ExtendPrimitive<S, i128>>()
            .map(|seq| seq.into_inner())
    }

    /// Try get a collection `S` of `f64`s from this value.
    ///
    /// If this value is a sequence then the collection `S` will be extended
    /// with the attempted conversion of each of its elements.
    ///
    /// If this value is not a sequence then this method will return `None`.
    pub fn to_f64_seq<S: Default + Extend<Option<f64>>>(&self) -> Option<S> {
        self.inner
            .extend::<ExtendPrimitive<S, f64>>()
            .map(|seq| seq.into_inner())
    }

    /// Get a collection `S` of `f64`s from this value.
    ///
    /// If this value is a sequence then the collection `S` will be extended
    /// with the conversion of each of its elements. The conversion is the
    /// same as [`ValueBag::as_f64`].
    ///
    /// If this value is not a sequence then this method will return an
    /// empty collection.
    ///
    /// This is similar to [`ValueBag::to_f64_seq`], but can be more
    /// convenient when there's no need to distinguish between an empty
    /// collection and a non-collection, or between `f64` and non-`f64` elements.
    pub fn as_f64_seq<S: Default + Extend<f64>>(&self) -> S {
        #[derive(Default)]
        struct ExtendF64<S>(S);

        impl<'a, S: Extend<f64>> ExtendValue<'a> for ExtendF64<S> {
            fn extend(&mut self, inner: Internal<'_>) {
                self.0.extend(Some(ValueBag { inner }.as_f64()))
            }
        }

        self.inner
            .extend::<ExtendF64<S>>()
            .map(|seq| seq.0)
            .unwrap_or_default()
    }

    /// Try get a collection `S` of `bool`s from this value.
    ///
    /// If this value is a sequence then the collection `S` will be extended
    /// with the attempted conversion of each of its elements.
    ///
    /// If this value is not a sequence then this method will return `None`.
    pub fn to_bool_seq<S: Default + Extend<Option<bool>>>(&self) -> Option<S> {
        self.inner
            .extend::<ExtendPrimitive<S, bool>>()
            .map(|seq| seq.into_inner())
    }

    /// Try get a collection `S` of `char`s from this value.
    ///
    /// If this value is a sequence then the collection `S` will be extended
    /// with the attempted conversion of each of its elements.
    ///
    /// If this value is not a sequence then this method will return `None`.
    pub fn to_char_seq<S: Default + Extend<Option<char>>>(&self) -> Option<S> {
        self.inner
            .extend::<ExtendPrimitive<S, char>>()
            .map(|seq| seq.into_inner())
    }

    /// Try get a collection `S` of strings from this value.
    ///
    /// If this value is a sequence then the collection `S` will be extended
    /// with the attempted conversion of each of its elements.
    ///
    /// If this value is not a sequence then this method will return `None`.
    #[inline]
    pub fn to_borrowed_str_seq<S: Default + Extend<Option<&'v str>>>(&self) -> Option<S> {
        #[derive(Default)]
        struct ExtendStr<'a, S>(S, PhantomData<&'a str>);

        impl<'a, S: Extend<Option<&'a str>>> ExtendValue<'a> for ExtendStr<'a, S> {
            fn extend<'b>(&mut self, _: Internal<'b>) {
                self.0.extend(Some(None::<&'a str>))
            }

            fn extend_borrowed(&mut self, inner: Internal<'a>) {
                self.0.extend(Some(ValueBag { inner }.to_borrowed_str()))
            }
        }

        self.inner.extend::<ExtendStr<'v, S>>().map(|seq| seq.0)
    }
}

impl<'s, 'f> Slot<'s, 'f> {
    /// Fill the slot with a sequence of values.
    ///
    /// The given value doesn't need to satisfy any particular lifetime constraints.
    pub fn fill_seq_slice<I, T>(self, value: &'f I) -> Result<(), Error>
    where
        I: AsRef<[T]>,
        &'f T: Into<ValueBag<'f>> + 'f,
    {
        self.fill(|visitor| visitor.seq(SeqSlice::new_ref(value)))
    }
}

/*
This is a bit of an ugly way of working around the gulf between
lifetimes expressed externally as bounds, and lifetimes implied
on methods.
*/

#[repr(transparent)]
struct SeqSlice<'a, I: ?Sized, T>(PhantomData<&'a [T]>, I);

impl<'a, I: AsRef<[T]> + ?Sized + 'a, T> SeqSlice<'a, I, T> {
    fn new_ref(v: &'a I) -> &'a SeqSlice<'a, I, T> {
        // SAFETY: `SeqSlice<'a, I, T>` and `I` have the same ABI
        unsafe { &*(v as *const I as *const SeqSlice<'a, I, T>) }
    }

    fn as_ref<'b>(&'b self) -> &'a [T] {
        // SAFETY: `new_ref` requires there's a borrow of `&'a I`
        // on the borrow stack, so we can safely borrow it for `'a` here
        let inner = unsafe { mem::transmute::<&'b I, &'a I>(&self.1) };

        inner.as_ref()
    }
}

impl<'a, I, T> Seq for SeqSlice<'a, I, T>
where
    I: AsRef<[T]> + ?Sized + 'a,
    &'a T: Into<ValueBag<'a>>,
{
    fn visit(&self, visitor: &mut dyn Visitor<'_>) {
        for v in self.as_ref().iter() {
            if let ControlFlow::Break(()) = visitor.element(v.into()) {
                return;
            }
        }
    }

    fn borrowed_visit<'v>(&'v self, visitor: &mut dyn Visitor<'v>) {
        for v in self.as_ref().iter() {
            if let ControlFlow::Break(()) = visitor.borrowed_element(v.into()) {
                return;
            }
        }
    }
}

pub(crate) trait Seq {
    fn visit(&self, visitor: &mut dyn Visitor<'_>);

    fn borrowed_visit<'v>(&'v self, visitor: &mut dyn Visitor<'v>) {
        self.visit(visitor)
    }
}

impl<'a, S: Seq + ?Sized> Seq for &'a S {
    fn visit(&self, visitor: &mut dyn Visitor<'_>) {
        (**self).visit(visitor)
    }

    fn borrowed_visit<'v>(&'v self, visitor: &mut dyn Visitor<'v>) {
        (**self).borrowed_visit(visitor)
    }
}

pub(crate) trait Visitor<'v> {
    fn element(&mut self, v: ValueBag) -> ControlFlow<()>;

    fn borrowed_element(&mut self, v: ValueBag<'v>) -> ControlFlow<()> {
        self.element(v)
    }
}

impl<'a, 'v, T: Visitor<'v> + ?Sized> Visitor<'v> for &'a mut T {
    fn element(&mut self, v: ValueBag) -> ControlFlow<()> {
        (**self).element(v)
    }

    fn borrowed_element(&mut self, v: ValueBag<'v>) -> ControlFlow<()> {
        (**self).borrowed_element(v)
    }
}

pub(crate) trait DowncastSeq {
    // Currently only used when `owned` is also available
    #[allow(dead_code)]
    fn as_any(&self) -> &dyn Any;
    fn as_super(&self) -> &dyn Seq;
}

impl<T: Seq + 'static> DowncastSeq for T {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_super(&self) -> &dyn Seq {
        self
    }
}

impl<'a> Seq for dyn DowncastSeq + Send + Sync + 'a {
    fn visit(&self, visitor: &mut dyn Visitor<'_>) {
        self.as_super().visit(visitor)
    }
}

macro_rules! convert_primitive(
    ($($t:ty,)*) => {
        $(
            impl<'v, const N: usize> From<&'v [$t; N]> for ValueBag<'v> {
                fn from(v: &'v [$t; N]) -> Self {
                    ValueBag::from_seq_slice(v)
                }
            }

            impl<'v, const N: usize> From<Option<&'v [$t; N]>> for ValueBag<'v> {
                fn from(v: Option<&'v [$t; N]>) -> Self {
                    ValueBag::from_option(v)
                }
            }

            impl<'a, 'v> From<&'v &'a [$t]> for ValueBag<'v> {
                fn from(v: &'v &'a [$t]) -> Self {
                    ValueBag::from_seq_slice(v)
                }
            }

            impl<'a, 'v> From<Option<&'v &'a [$t]>> for ValueBag<'v> {
                fn from(v: Option<&'v &'a [$t]>) -> Self {
                    ValueBag::from_option(v)
                }
            }

            #[cfg(feature = "alloc")]
            impl<'v> From<&'v Vec<$t>> for ValueBag<'v> {
                fn from(v: &'v Vec<$t>) -> Self {
                    ValueBag::from_seq_slice(v)
                }
            }

            #[cfg(feature = "alloc")]
            impl<'v> From<Option<&'v Vec<$t>>> for ValueBag<'v> {
                fn from(v: Option<&'v Vec<$t>>) -> Self {
                    ValueBag::from_option(v)
                }
            }
        )*
    }
);

convert_primitive![
    u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64, bool, char,
];

impl<'v, 'a, const N: usize> From<&'v [&'a str; N]> for ValueBag<'v> {
    fn from(v: &'v [&'a str; N]) -> Self {
        ValueBag::from_seq_slice(v)
    }
}

impl<'v, 'a, const N: usize> From<Option<&'v [&'a str; N]>> for ValueBag<'v> {
    fn from(v: Option<&'v [&'a str; N]>) -> Self {
        ValueBag::from_option(v)
    }
}

impl<'v, 'a, 'b> From<&'v &'a [&'b str]> for ValueBag<'v> {
    fn from(v: &'v &'a [&'b str]) -> Self {
        ValueBag::from_seq_slice(v)
    }
}

impl<'v, 'a, 'b> From<Option<&'v &'a [&'b str]>> for ValueBag<'v> {
    fn from(v: Option<&'v &'a [&'b str]>) -> Self {
        ValueBag::from_option(v)
    }
}

#[cfg(feature = "alloc")]
impl<'v> From<&'v Vec<String>> for ValueBag<'v> {
    fn from(v: &'v Vec<String>) -> Self {
        ValueBag::from_seq_slice(v)
    }
}

#[cfg(feature = "alloc")]
impl<'v> From<Option<&'v Vec<String>>> for ValueBag<'v> {
    fn from(v: Option<&'v Vec<String>>) -> Self {
        ValueBag::from_option(v)
    }
}

#[derive(Default)]
pub(crate) struct ExtendPrimitive<S, T>(S, PhantomData<T>);

impl<S, T> ExtendPrimitive<S, T> {
    pub fn into_inner(self) -> S {
        self.0
    }
}

impl<'a, S: Extend<Option<T>>, T: for<'b> TryFrom<ValueBag<'b>>> ExtendValue<'a>
    for ExtendPrimitive<S, T>
{
    fn extend(&mut self, inner: Internal) {
        self.0.extend(Some(ValueBag { inner }.try_into().ok()))
    }
}

#[allow(dead_code)]
pub(crate) trait ExtendValue<'v> {
    fn extend(&mut self, v: Internal);

    fn extend_borrowed(&mut self, v: Internal<'v>) {
        self.extend(v);
    }
}

struct ExtendVisitor<S>(S);

impl<'v, S: ExtendValue<'v>> Visitor<'v> for ExtendVisitor<S> {
    fn element(&mut self, v: ValueBag) -> ControlFlow<()> {
        self.0.extend(v.inner);
        ControlFlow::Continue(())
    }

    fn borrowed_element(&mut self, v: ValueBag<'v>) -> ControlFlow<()> {
        self.0.extend_borrowed(v.inner);
        ControlFlow::Continue(())
    }
}

impl<'v> Internal<'v> {
    #[inline]
    pub(crate) fn extend<S: Default + ExtendValue<'v>>(&self) -> Option<S> {
        struct SeqVisitor<S>(Option<S>);

        impl<'v, S: Default + ExtendValue<'v>> InternalVisitor<'v> for SeqVisitor<S> {
            #[inline]
            fn fill(&mut self, v: &dyn crate::fill::Fill) -> Result<(), Error> {
                v.fill(Slot::new(self))
            }

            #[inline]
            fn debug(&mut self, _: &dyn fmt::Debug) -> Result<(), Error> {
                Ok(())
            }

            #[inline]
            fn display(&mut self, _: &dyn fmt::Display) -> Result<(), Error> {
                Ok(())
            }

            #[inline]
            fn u64(&mut self, _: u64) -> Result<(), Error> {
                Ok(())
            }

            #[inline]
            fn i64(&mut self, _: i64) -> Result<(), Error> {
                Ok(())
            }

            #[inline]
            fn u128(&mut self, _: &u128) -> Result<(), Error> {
                Ok(())
            }

            #[inline]
            fn i128(&mut self, _: &i128) -> Result<(), Error> {
                Ok(())
            }

            #[inline]
            fn f64(&mut self, _: f64) -> Result<(), Error> {
                Ok(())
            }

            #[inline]
            fn bool(&mut self, _: bool) -> Result<(), Error> {
                Ok(())
            }

            #[inline]
            fn char(&mut self, _: char) -> Result<(), Error> {
                Ok(())
            }

            #[inline]
            fn str(&mut self, _: &str) -> Result<(), Error> {
                Ok(())
            }

            #[inline]
            fn none(&mut self) -> Result<(), Error> {
                Ok(())
            }

            #[cfg(feature = "error")]
            #[inline]
            fn error(&mut self, _: &dyn crate::internal::error::Error) -> Result<(), Error> {
                Ok(())
            }

            #[cfg(feature = "sval2")]
            #[inline]
            fn sval2(&mut self, v: &dyn crate::internal::sval::v2::Value) -> Result<(), Error> {
                self.0 = crate::internal::sval::v2::seq::extend(v);

                Ok(())
            }

            #[cfg(feature = "sval2")]
            #[inline]
            fn borrowed_sval2(
                &mut self,
                v: &'v dyn crate::internal::sval::v2::Value,
            ) -> Result<(), Error> {
                self.0 = crate::internal::sval::v2::seq::extend_borrowed(v);

                Ok(())
            }

            #[cfg(feature = "serde1")]
            #[inline]
            fn serde1(
                &mut self,
                v: &dyn crate::internal::serde::v1::Serialize,
            ) -> Result<(), Error> {
                self.0 = crate::internal::serde::v1::seq::extend(v);

                Ok(())
            }

            fn seq(&mut self, seq: &dyn Seq) -> Result<(), Error> {
                let mut s = ExtendVisitor(S::default());
                seq.visit(&mut s);
                self.0 = Some(s.0);

                Ok(())
            }

            fn borrowed_seq(&mut self, seq: &'v dyn Seq) -> Result<(), Error> {
                let mut s = ExtendVisitor(S::default());
                seq.borrowed_visit(&mut s);
                self.0 = Some(s.0);

                Ok(())
            }

            fn poisoned(&mut self, _: &'static str) -> Result<(), Error> {
                Ok(())
            }
        }

        let mut visitor = SeqVisitor(None);
        let _ = self.internal_visit(&mut visitor);

        visitor.0
    }
}

#[cfg(feature = "alloc")]
mod alloc_support {
    use super::*;

    use crate::std::borrow::Cow;

    impl<'v> ValueBag<'v> {
        /// Try get a collection `S` of strings from this value.
        ///
        /// If this value is a sequence then the collection `S` will be extended
        /// with the attempted conversion of each of its elements.
        ///
        /// If this value is not a sequence then this method will return `None`.
        #[inline]
        pub fn to_str_seq<S: Default + Extend<Option<Cow<'v, str>>>>(&self) -> Option<S> {
            #[derive(Default)]
            struct ExtendStr<'a, S>(S, PhantomData<Cow<'a, str>>);

            impl<'a, S: Extend<Option<Cow<'a, str>>>> ExtendValue<'a> for ExtendStr<'a, S> {
                fn extend(&mut self, inner: Internal<'_>) {
                    self.0.extend(Some(
                        ValueBag { inner }
                            .to_str()
                            .map(|s| Cow::Owned(s.into_owned())),
                    ))
                }

                fn extend_borrowed(&mut self, inner: Internal<'a>) {
                    self.0.extend(Some(ValueBag { inner }.to_str()))
                }
            }

            self.inner.extend::<ExtendStr<'v, S>>().map(|seq| seq.0)
        }
    }
}

#[cfg(feature = "owned")]
pub(crate) mod owned {
    use super::*;

    use crate::{owned::OwnedValueBag, std::boxed::Box};

    #[derive(Clone)]
    pub(crate) struct OwnedSeq(Box<[OwnedValueBag]>);

    impl Seq for OwnedSeq {
        fn visit(&self, visitor: &mut dyn Visitor<'_>) {
            for item in self.0.iter() {
                if let ControlFlow::Break(()) = visitor.element(item.by_ref()) {
                    return;
                }
            }
        }
    }

    pub(crate) fn buffer(v: &dyn Seq) -> Result<OwnedSeq, Error> {
        struct BufferVisitor(Vec<OwnedValueBag>);

        impl<'v> Visitor<'v> for BufferVisitor {
            fn element(&mut self, v: ValueBag) -> ControlFlow<()> {
                self.0.push(v.to_owned());
                ControlFlow::Continue(())
            }
        }

        let mut buf = BufferVisitor(Vec::new());
        v.visit(&mut buf);
        Ok(OwnedSeq(buf.0.into_boxed_slice()))
    }
}

#[cfg(test)]
mod tests {
    #[cfg(target_arch = "wasm32")]
    use wasm_bindgen_test::*;

    use std::vec::Vec;

    use super::*;

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
    fn to_u64_seq() {
        assert_eq!(
            Some(vec![Some(1u64), Some(2u64), Some(3u64)]),
            ValueBag::from(&[1u8, 2u8, 3u8]).to_u64_seq::<Vec<Option<u64>>>()
        );

        assert_eq!(
            Some(vec![Some(1u64), Some(2u64), Some(3u64)]),
            ValueBag::from(&[1u16, 2u16, 3u16]).to_u64_seq::<Vec<Option<u64>>>()
        );

        assert_eq!(
            Some(vec![Some(1u64), Some(2u64), Some(3u64)]),
            ValueBag::from(&[1u32, 2u32, 3u32]).to_u64_seq::<Vec<Option<u64>>>()
        );

        assert_eq!(
            Some(vec![Some(1u64), Some(2u64), Some(3u64)]),
            ValueBag::from(&[1u64, 2u64, 3u64]).to_u64_seq::<Vec<Option<u64>>>()
        );
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
    fn to_i64_seq() {
        assert_eq!(
            Some(vec![Some(1i64), Some(2i64), Some(3i64)]),
            ValueBag::from(&[1i8, 2i8, 3i8]).to_i64_seq::<Vec<Option<i64>>>()
        );

        assert_eq!(
            Some(vec![Some(1i64), Some(2i64), Some(3i64)]),
            ValueBag::from(&[1i16, 2i16, 3i16]).to_i64_seq::<Vec<Option<i64>>>()
        );

        assert_eq!(
            Some(vec![Some(1i64), Some(2i64), Some(3i64)]),
            ValueBag::from(&[1i32, 2i32, 3i32]).to_i64_seq::<Vec<Option<i64>>>()
        );

        assert_eq!(
            Some(vec![Some(1i64), Some(2i64), Some(3i64)]),
            ValueBag::from(&[1i64, 2i64, 3i64]).to_i64_seq::<Vec<Option<i64>>>()
        );
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
    fn to_f64_seq() {
        assert_eq!(
            Some(vec![Some(1.0f64), Some(2.0f64), Some(3.0f64)]),
            ValueBag::from(&[1.0f64, 2.0f64, 3.0f64]).to_f64_seq::<Vec<Option<f64>>>()
        );
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
    fn as_f64_seq() {
        assert_eq!(
            Vec::<f64>::new(),
            ValueBag::from(1.0f64).as_f64_seq::<Vec<f64>>()
        );

        assert_eq!(
            vec![1.0f64, 2.0f64, 3.0f64],
            ValueBag::from(&[1, 2, 3]).as_f64_seq::<Vec<f64>>()
        );
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
    fn to_u128_seq() {
        assert_eq!(
            Some(vec![Some(1u128), Some(2u128), Some(3u128)]),
            ValueBag::from(&[1u128, 2u128, 3u128]).to_u128_seq::<Vec<Option<u128>>>()
        );
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
    fn to_i128_seq() {
        assert_eq!(
            Some(vec![Some(1i128), Some(2i128), Some(3i128)]),
            ValueBag::from(&[1i128, 2i128, 3i128]).to_i128_seq::<Vec<Option<i128>>>()
        );
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
    fn to_bool_seq() {
        assert_eq!(
            Some(vec![Some(true), Some(false)]),
            ValueBag::from(&[true, false]).to_bool_seq::<Vec<Option<bool>>>()
        );
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
    fn to_char_seq() {
        assert_eq!(
            Some(vec![Some('a'), Some('b'), Some('c')]),
            ValueBag::from(&['a', 'b', 'c']).to_char_seq::<Vec<Option<char>>>()
        );
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
    fn to_borrowed_str_seq() {
        let v = ["a", "b", "c"];
        let v = ValueBag::from(&v);

        assert_eq!(
            Some(vec![Some("a"), Some("b"), Some("c")]),
            v.to_borrowed_str_seq::<Vec<Option<&str>>>()
        );

        let v = ValueBag::from_fill(&|slot: Slot| slot.fill_seq_slice(&["a", "b", "c"]));

        assert_eq!(
            Some(vec![None, None, None]),
            v.to_borrowed_str_seq::<Vec<Option<&str>>>()
        );
    }

    #[cfg(feature = "alloc")]
    mod alloc_support {
        use super::*;

        use crate::std::borrow::Cow;

        #[test]
        #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
        fn to_str_seq() {
            let v = ["a", "b", "c"];
            let v = ValueBag::from(&v);

            assert_eq!(
                Some(vec![
                    Some(Cow::Borrowed("a")),
                    Some(Cow::Borrowed("b")),
                    Some(Cow::Borrowed("c"))
                ]),
                v.to_str_seq::<Vec<Option<Cow<str>>>>()
            );

            let v = ValueBag::from_fill(&|slot: Slot| slot.fill_seq_slice(&["a", "b", "c"]));

            assert_eq!(
                Some(vec![
                    Some(Cow::Owned("a".into())),
                    Some(Cow::Owned("b".into())),
                    Some(Cow::Owned("c".into()))
                ]),
                v.to_str_seq::<Vec<Option<Cow<str>>>>()
            );
        }
    }
}
