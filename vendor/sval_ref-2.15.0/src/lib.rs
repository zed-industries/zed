/*!
A variant of [`sval::Value`] for types that store references internally.
*/

#![cfg_attr(not(test), no_std)]
#![deny(missing_docs)]

#[cfg(feature = "std")]
extern crate std;

#[cfg(all(feature = "alloc", not(feature = "std")))]
extern crate alloc;
#[cfg(all(feature = "alloc", not(feature = "std")))]
extern crate core;

#[cfg(all(feature = "alloc", not(feature = "std")))]
mod std {
    pub use crate::{
        alloc::{borrow, boxed, collections, string, vec},
        core::{convert, fmt, hash, marker, mem, ops, result, str, write},
    };
}

#[cfg(all(not(feature = "alloc"), not(feature = "std")))]
extern crate core as std;

mod seq;

/**
Stream a value through a stream.
*/
pub fn stream_ref<'sval>(
    stream: &mut (impl Stream<'sval> + ?Sized),
    value: impl ValueRef<'sval>,
) -> Result {
    value.stream_ref(stream)
}

/**
Wrap an [`sval::Value`] in a [`ValueRef`]
*/
pub fn to_ref<'sval, V: sval::Value + ?Sized>(value: &'sval V) -> Ref<&'sval V> {
    Ref::new(value)
}

use sval::{Result, Stream, Value};

/**
Adapt an [`sval::Value`] into a [`ValueRef`].
*/
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
pub struct Ref<V: ?Sized>(V);

impl<V> Ref<V> {
    /**
    Wrap a value.
    */
    pub fn new(value: V) -> Self {
        Ref(value)
    }

    /**
    Get a reference to the underlying value.
    */
    pub fn inner(&self) -> &V {
        &self.0
    }

    /**
    Take ownership of the underlying value.
    */
    pub fn into_inner(self) -> V {
        self.0
    }
}

impl<V: ?Sized> Ref<V> {
    /**
    Get a borrowed wrapper over a borrowed value.
    */
    pub fn new_borrowed<'a>(value: &'a V) -> &'a Ref<V> {
        // SAFETY: `&'a V` and `&'a Ref<V>` have the same ABI
        unsafe { &*(value as *const _ as *const Ref<V>) }
    }
}

impl<V: sval::Value> sval::Value for Ref<V> {
    fn stream<'sval, S: Stream<'sval> + ?Sized>(&'sval self, stream: &mut S) -> Result {
        self.0.stream(stream)
    }

    fn tag(&self) -> Option<sval::Tag> {
        self.0.tag()
    }

    fn to_bool(&self) -> Option<bool> {
        self.0.to_bool()
    }

    fn to_f32(&self) -> Option<f32> {
        self.0.to_f32()
    }

    fn to_f64(&self) -> Option<f64> {
        self.0.to_f64()
    }

    fn to_i8(&self) -> Option<i8> {
        self.0.to_i8()
    }

    fn to_i16(&self) -> Option<i16> {
        self.0.to_i16()
    }

    fn to_i32(&self) -> Option<i32> {
        self.0.to_i32()
    }

    fn to_i64(&self) -> Option<i64> {
        self.0.to_i64()
    }

    fn to_i128(&self) -> Option<i128> {
        self.0.to_i128()
    }

    fn to_u8(&self) -> Option<u8> {
        self.0.to_u8()
    }

    fn to_u16(&self) -> Option<u16> {
        self.0.to_u16()
    }

    fn to_u32(&self) -> Option<u32> {
        self.0.to_u32()
    }

    fn to_u64(&self) -> Option<u64> {
        self.0.to_u64()
    }

    fn to_u128(&self) -> Option<u128> {
        self.0.to_u128()
    }

    fn to_text(&self) -> Option<&str> {
        self.0.to_text()
    }

    fn to_binary(&self) -> Option<&[u8]> {
        self.0.to_binary()
    }
}

impl<'sval, V: sval::Value + ?Sized> ValueRef<'sval> for Ref<&'sval V> {
    fn stream_ref<S: Stream<'sval> + ?Sized>(&self, stream: &mut S) -> Result {
        self.0.stream(stream)
    }
}

/**
A producer of structured data that stores a reference internally.

This trait is a variant of [`Value`] for wrapper types that keep a reference to a value internally.
In `Value`, the `'sval` lifetime comes from the borrow of `&'sval self`. In `ValueRef`, it comes
from the `'sval` lifetime in the trait itself.
*/
pub trait ValueRef<'sval>: Value {
    /**
    Stream this value through a [`Stream`].
    */
    fn stream_ref<S: Stream<'sval> + ?Sized>(&self, stream: &mut S) -> Result;
}

macro_rules! impl_value_ref_forward {
    ({ $($r:tt)* } => $bind:ident => { $($forward:tt)* }) => {
        $($r)* {
            fn stream_ref<S: Stream<'sval> + ?Sized>(&self, stream: &mut S) -> Result {
                let $bind = self;
                ($($forward)*).stream_ref(stream)
            }
        }
    };
}

impl_value_ref_forward!({impl<'sval, 'a, T: ValueRef<'sval> + ?Sized> ValueRef<'sval> for &'a T} => x => { **x });

#[cfg(feature = "alloc")]
mod alloc_support {
    use super::*;

    use crate::std::boxed::Box;

    impl_value_ref_forward!({impl<'sval, T: ValueRef<'sval> + ?Sized> ValueRef<'sval> for Box<T>} => x => { **x });
}

#[cfg(test)]
mod test {
    use crate::ValueRef;
    pub(crate) use sval_test::{assert_tokens, Token};

    pub(crate) fn assert_tokens_ref<'sval>(
        value: impl ValueRef<'sval>,
        tokens: &[sval_test::Token<'sval>],
    ) {
        let mut actual = sval_test::TokenBuf::new();
        value.stream_ref(&mut actual).unwrap();

        assert_eq!(tokens, actual.as_tokens());
    }

    pub(crate) struct Ref<T>(pub(crate) T);

    impl<T: sval::Value> sval::Value for Ref<T> {
        fn stream<'sval, S: sval::Stream<'sval> + ?Sized>(
            &'sval self,
            stream: &mut S,
        ) -> sval::Result {
            self.0.stream(stream)
        }
    }

    impl<'sval, T: sval::Value + ?Sized> ValueRef<'sval> for Ref<&'sval T> {
        fn stream_ref<S: sval::Stream<'sval> + ?Sized>(&self, stream: &mut S) -> sval::Result {
            self.0.stream(stream)
        }
    }

    pub(crate) fn compat_case<'sval>(
        v: &'sval (impl sval::Value + ValueRef<'sval> + ?Sized),
        tokens: &[Token<'sval>],
    ) {
        assert_tokens_ref(v, tokens);
        assert_tokens(v, tokens);
    }
}
