use crate::{
    internal::{self, Internal, InternalVisitor},
    std::{boxed::Box, sync::Arc},
    Error,
};

#[derive(Clone)]
pub(crate) enum OwnedInternal {
    // Primitive values
    BigSigned(i128),
    BigUnsigned(u128),
    Float(f64),
    Bool(bool),
    Char(char),
    #[cfg(feature = "inline-str")]
    StrSmall(inline_str::InlineStr),
    Str(Box<str>),
    None,

    // Buffered values
    Debug(internal::fmt::owned::OwnedFmt),
    Display(internal::fmt::owned::OwnedFmt),
    #[cfg(feature = "error")]
    Error(internal::error::owned::OwnedError),
    #[cfg(feature = "serde1")]
    Serde1(internal::serde::v1::owned::OwnedSerialize),
    #[cfg(feature = "sval2")]
    Sval2(internal::sval::v2::owned::OwnedValue),
    #[cfg(feature = "seq")]
    Seq(internal::seq::owned::OwnedSeq),

    // Shared values
    SharedDebug(Arc<dyn internal::fmt::DowncastDebug + Send + Sync>),
    SharedDisplay(Arc<dyn internal::fmt::DowncastDisplay + Send + Sync>),
    #[cfg(feature = "error")]
    SharedError(Arc<dyn internal::error::DowncastError + Send + Sync>),
    #[cfg(feature = "serde1")]
    SharedSerde1(Arc<dyn internal::serde::v1::DowncastSerialize + Send + Sync>),
    #[cfg(feature = "sval2")]
    SharedSval2(Arc<dyn internal::sval::v2::DowncastValue + Send + Sync>),
    #[cfg(feature = "seq")]
    SharedSeq(Arc<dyn internal::seq::DowncastSeq + Send + Sync>),

    // Poisoned value
    Poisoned(&'static str),
}

impl OwnedInternal {
    #[inline]
    pub(crate) const fn by_ref(&self) -> Internal<'_> {
        match self {
            #[cfg(not(feature = "inline-i128"))]
            OwnedInternal::BigSigned(v) => Internal::BigSigned(v),
            #[cfg(feature = "inline-i128")]
            OwnedInternal::BigSigned(v) => Internal::BigSigned(*v),
            #[cfg(not(feature = "inline-i128"))]
            OwnedInternal::BigUnsigned(v) => Internal::BigUnsigned(v),
            #[cfg(feature = "inline-i128")]
            OwnedInternal::BigUnsigned(v) => Internal::BigUnsigned(*v),
            OwnedInternal::Float(v) => Internal::Float(*v),
            OwnedInternal::Bool(v) => Internal::Bool(*v),
            OwnedInternal::Char(v) => Internal::Char(*v),
            OwnedInternal::Str(v) => Internal::Str(v),
            #[cfg(feature = "inline-str")]
            OwnedInternal::StrSmall(v) => Internal::Str(v.get()),
            OwnedInternal::None => Internal::None,

            OwnedInternal::Debug(v) => Internal::AnonDebug(v),
            OwnedInternal::Display(v) => Internal::AnonDisplay(v),
            #[cfg(feature = "error")]
            OwnedInternal::Error(v) => Internal::AnonError(v),
            #[cfg(feature = "serde1")]
            OwnedInternal::Serde1(v) => Internal::AnonSerde1(v),
            #[cfg(feature = "sval2")]
            OwnedInternal::Sval2(v) => Internal::AnonSval2(v),
            #[cfg(feature = "seq")]
            OwnedInternal::Seq(v) => Internal::AnonSeq(v),

            OwnedInternal::SharedDebug(ref value) => Internal::SharedRefDebug(value),
            OwnedInternal::SharedDisplay(ref value) => Internal::SharedRefDisplay(value),
            #[cfg(feature = "error")]
            OwnedInternal::SharedError(ref value) => Internal::SharedRefError(value),
            #[cfg(feature = "serde1")]
            OwnedInternal::SharedSerde1(ref value) => Internal::SharedRefSerde1(value),
            #[cfg(feature = "sval2")]
            OwnedInternal::SharedSval2(ref value) => Internal::SharedRefSval2(value),
            #[cfg(feature = "seq")]
            OwnedInternal::SharedSeq(ref value) => Internal::SharedRefSeq(value),

            OwnedInternal::Poisoned(msg) => Internal::Poisoned(msg),
        }
    }

    #[inline]
    pub(crate) fn into_shared(self) -> Self {
        match self {
            OwnedInternal::BigSigned(v) => OwnedInternal::BigSigned(v),
            OwnedInternal::BigUnsigned(v) => OwnedInternal::BigUnsigned(v),
            OwnedInternal::Float(v) => OwnedInternal::Float(v),
            OwnedInternal::Bool(v) => OwnedInternal::Bool(v),
            OwnedInternal::Char(v) => OwnedInternal::Char(v),
            OwnedInternal::Str(v) => OwnedInternal::Str(v),
            #[cfg(feature = "inline-str")]
            OwnedInternal::StrSmall(v) => OwnedInternal::StrSmall(v),
            OwnedInternal::None => OwnedInternal::None,

            OwnedInternal::Debug(v) => OwnedInternal::SharedDebug(Arc::new(v)),
            OwnedInternal::Display(v) => OwnedInternal::SharedDisplay(Arc::new(v)),
            #[cfg(feature = "error")]
            OwnedInternal::Error(v) => OwnedInternal::SharedError(Arc::new(v)),
            #[cfg(feature = "serde1")]
            OwnedInternal::Serde1(v) => OwnedInternal::SharedSerde1(Arc::new(v)),
            #[cfg(feature = "sval2")]
            OwnedInternal::Sval2(v) => OwnedInternal::SharedSval2(Arc::new(v)),
            #[cfg(feature = "seq")]
            OwnedInternal::Seq(v) => OwnedInternal::SharedSeq(Arc::new(v)),

            OwnedInternal::SharedDebug(v) => OwnedInternal::SharedDebug(v),
            OwnedInternal::SharedDisplay(v) => OwnedInternal::SharedDisplay(v),
            #[cfg(feature = "error")]
            OwnedInternal::SharedError(v) => OwnedInternal::SharedError(v),
            #[cfg(feature = "serde1")]
            OwnedInternal::SharedSerde1(v) => OwnedInternal::SharedSerde1(v),
            #[cfg(feature = "sval2")]
            OwnedInternal::SharedSval2(v) => OwnedInternal::SharedSval2(v),
            #[cfg(feature = "seq")]
            OwnedInternal::SharedSeq(v) => OwnedInternal::SharedSeq(v),

            OwnedInternal::Poisoned(msg) => OwnedInternal::Poisoned(msg),
        }
    }
}

impl<'v> Internal<'v> {
    pub(crate) fn to_owned(&self) -> OwnedInternal {
        struct OwnedVisitor(OwnedInternal);

        impl<'v> InternalVisitor<'v> for OwnedVisitor {
            fn fill(&mut self, v: &dyn crate::fill::Fill) -> Result<(), Error> {
                v.fill(crate::fill::Slot::new(self))
            }

            fn debug(&mut self, v: &dyn internal::fmt::Debug) -> Result<(), Error> {
                self.0 = OwnedInternal::Debug(internal::fmt::owned::buffer_debug(v));
                Ok(())
            }

            fn shared_debug(
                &mut self,
                v: &Arc<dyn internal::fmt::DowncastDebug + Send + Sync>,
            ) -> Result<(), Error> {
                self.0 = OwnedInternal::SharedDebug(v.clone());
                Ok(())
            }

            fn display(&mut self, v: &dyn internal::fmt::Display) -> Result<(), Error> {
                self.0 = OwnedInternal::Display(internal::fmt::owned::buffer_display(v));
                Ok(())
            }

            fn shared_display(
                &mut self,
                v: &Arc<dyn internal::fmt::DowncastDisplay + Send + Sync>,
            ) -> Result<(), Error> {
                self.0 = OwnedInternal::SharedDisplay(v.clone());
                Ok(())
            }

            fn u64(&mut self, v: u64) -> Result<(), Error> {
                self.0 = OwnedInternal::BigUnsigned(v as u128);
                Ok(())
            }

            fn i64(&mut self, v: i64) -> Result<(), Error> {
                self.0 = OwnedInternal::BigSigned(v as i128);
                Ok(())
            }

            fn u128(&mut self, v: &u128) -> Result<(), Error> {
                self.0 = OwnedInternal::BigUnsigned(*v);
                Ok(())
            }

            fn i128(&mut self, v: &i128) -> Result<(), Error> {
                self.0 = OwnedInternal::BigSigned(*v);
                Ok(())
            }

            fn f64(&mut self, v: f64) -> Result<(), Error> {
                self.0 = OwnedInternal::Float(v);
                Ok(())
            }

            fn bool(&mut self, v: bool) -> Result<(), Error> {
                self.0 = OwnedInternal::Bool(v);
                Ok(())
            }

            fn char(&mut self, v: char) -> Result<(), Error> {
                self.0 = OwnedInternal::Char(v);
                Ok(())
            }

            fn str(&mut self, v: &str) -> Result<(), Error> {
                #[cfg(feature = "inline-str")]
                {
                    if v.len() <= inline_str::MAX_INLINE_LEN {
                        self.0 = OwnedInternal::StrSmall(inline_str::InlineStr::copy_from(v));
                        return Ok(());
                    }
                }

                self.0 = OwnedInternal::Str(v.into());
                Ok(())
            }

            fn none(&mut self) -> Result<(), Error> {
                self.0 = OwnedInternal::None;
                Ok(())
            }

            #[cfg(feature = "error")]
            fn error(&mut self, v: &(dyn internal::error::Error + 'static)) -> Result<(), Error> {
                self.0 = OwnedInternal::Error(internal::error::owned::buffer(v));
                Ok(())
            }

            #[cfg(feature = "error")]
            fn shared_error(
                &mut self,
                v: &Arc<dyn internal::error::DowncastError + Send + Sync>,
            ) -> Result<(), Error> {
                self.0 = OwnedInternal::SharedError(v.clone());
                Ok(())
            }

            #[cfg(feature = "sval2")]
            fn sval2(&mut self, v: &dyn internal::sval::v2::Value) -> Result<(), Error> {
                self.0 = internal::sval::v2::owned::buffer(v)
                    .map(OwnedInternal::Sval2)
                    .unwrap_or(OwnedInternal::Poisoned("failed to buffer the value"));
                Ok(())
            }

            #[cfg(feature = "sval2")]
            fn shared_sval2(
                &mut self,
                v: &Arc<dyn internal::sval::v2::DowncastValue + Send + Sync>,
            ) -> Result<(), Error> {
                self.0 = OwnedInternal::SharedSval2(v.clone());
                Ok(())
            }

            #[cfg(feature = "serde1")]
            fn serde1(&mut self, v: &dyn internal::serde::v1::Serialize) -> Result<(), Error> {
                self.0 = internal::serde::v1::owned::buffer(v)
                    .map(OwnedInternal::Serde1)
                    .unwrap_or(OwnedInternal::Poisoned("failed to buffer the value"));
                Ok(())
            }

            #[cfg(feature = "serde1")]
            fn shared_serde1(
                &mut self,
                v: &Arc<dyn internal::serde::v1::DowncastSerialize + Send + Sync>,
            ) -> Result<(), Error> {
                self.0 = OwnedInternal::SharedSerde1(v.clone());
                Ok(())
            }

            #[cfg(feature = "seq")]
            fn seq(&mut self, v: &dyn internal::seq::Seq) -> Result<(), Error> {
                self.0 = internal::seq::owned::buffer(v)
                    .map(OwnedInternal::Seq)
                    .unwrap_or(OwnedInternal::Poisoned("failed to buffer the value"));
                Ok(())
            }

            #[cfg(feature = "seq")]
            fn shared_seq(
                &mut self,
                v: &Arc<dyn internal::seq::DowncastSeq + Send + Sync>,
            ) -> Result<(), Error> {
                self.0 = OwnedInternal::SharedSeq(v.clone());
                Ok(())
            }

            fn poisoned(&mut self, msg: &'static str) -> Result<(), Error> {
                self.0 = OwnedInternal::Poisoned(msg);
                Ok(())
            }
        }

        let mut visitor = OwnedVisitor(OwnedInternal::None);

        let _ = self.internal_visit(&mut visitor);

        visitor.0
    }
}

#[cfg(feature = "inline-str")]
mod inline_str {
    use crate::std::{fmt, slice, str};

    pub(super) const MAX_INLINE_LEN: usize = 22;

    #[derive(Clone, Copy)]
    pub(crate) struct InlineStr {
        data: [u8; MAX_INLINE_LEN],
        len: u8,
    }

    impl fmt::Debug for InlineStr {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            fmt::Debug::fmt(self.get(), f)
        }
    }

    impl fmt::Display for InlineStr {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            fmt::Display::fmt(self.get(), f)
        }
    }

    impl InlineStr {
        pub(super) fn copy_from(str: &str) -> Self {
            let str = str.as_bytes();

            // NOTE: This will panic if the string is too big
            let mut data = [0; MAX_INLINE_LEN];
            data[..str.len()].copy_from_slice(str);

            InlineStr {
                data,
                len: str.len() as u8,
            }
        }

        pub(super) const fn get(&self) -> &str {
            // NOTE: We can't slice data in `const` fns yet, so we do it this way
            // SAFETY: `data` contains valid UTF8, and `len` points within `data`
            unsafe {
                str::from_utf8_unchecked(slice::from_raw_parts(
                    &self.data as *const u8,
                    self.len as usize,
                ))
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        use std::string::ToString;

        #[test]
        fn inline_str() {
            let s = InlineStr::copy_from("abc");

            assert_eq!("abc", s.get());
            assert_eq!("abc", s.to_string());
        }
    }
}
