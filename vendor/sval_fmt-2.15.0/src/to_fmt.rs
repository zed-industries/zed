use core::fmt;

use crate::writer::Writer;

/**
Adapt an [`sval::Value`] into a [`fmt::Debug`] or [`fmt::Display`].
*/
#[repr(transparent)]
pub struct ToFmt<V: ?Sized>(V);

impl<V: sval::Value> ToFmt<V> {
    /**
    Adapt an [`sval::Value`] into a [`fmt::Debug`] or [`fmt::Display`].
    */
    pub fn new(value: V) -> ToFmt<V> {
        ToFmt(value)
    }
}

impl<V: sval::Value + ?Sized> ToFmt<V> {
    /**
    Adapt a reference to an [`sval::Value`] into a [`fmt::Debug`] or [`fmt::Display`].
    */
    pub fn new_borrowed<'a>(value: &'a V) -> &'a ToFmt<V> {
        // SAFETY: `&'a V` and `&'a ToDebug<V>` have the same ABI
        unsafe { &*(value as *const _ as *const ToFmt<V>) }
    }
}

/**
Format a value into an underlying formatter.
*/
pub fn stream_to_fmt(fmt: &mut fmt::Formatter, v: impl sval::Value) -> fmt::Result {
    v.stream(&mut Writer::new(fmt)).map_err(|_| fmt::Error)
}

impl<V: sval::Value> fmt::Debug for ToFmt<V> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

impl<V: sval::Value> fmt::Display for ToFmt<V> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // If the `Value` impl fails then swallow the error rather than
        // propagate it; Traits like `ToString` expect formatting to be
        // infallible unless the writer itself fails
        match stream_to_fmt(f, &self.0) {
            Ok(()) => Ok(()),
            Err(e) => write!(f, "<{}>", e),
        }
    }
}
