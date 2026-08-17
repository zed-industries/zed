use core::fmt::{self, Write as _};

/**
Stream a [`fmt::Debug`] into an [`sval::Stream`].
*/
pub fn stream_debug<'sval>(
    stream: &mut (impl sval::Stream<'sval> + ?Sized),
    value: impl fmt::Debug,
) -> sval::Result {
    stream.value_computed(&DebugToValue::new(value))
}

/**
Stream a [`fmt::Display`] into an [`sval::Stream`].
*/
pub fn stream_display<'sval>(
    stream: &mut (impl sval::Stream<'sval> + ?Sized),
    value: impl fmt::Display,
) -> sval::Result {
    stream.value_computed(&DisplayToValue::new(value))
}

/**
Adapt a [`fmt::Debug`] into an [`sval::Value`].
*/
#[repr(transparent)]
pub struct DebugToValue<V: ?Sized>(V);

impl<V: fmt::Debug> DebugToValue<V> {
    /**
    Adapt a [`fmt::Debug`] into an [`sval::Value`].
    */
    pub const fn new(value: V) -> DebugToValue<V> {
        DebugToValue(value)
    }
}

impl<V: fmt::Debug + ?Sized> DebugToValue<V> {
    /**
    Adapt a reference to a [`fmt::Debug`] into an [`sval::Value`].
    */
    pub const fn new_borrowed<'a>(value: &'a V) -> &'a DebugToValue<V> {
        // SAFETY: `&'a V` and `&'a DebugToValue<V>` have the same ABI
        unsafe { &*(value as *const _ as *const DebugToValue<V>) }
    }
}

/**
Adapt a [`fmt::Display`] into an [`sval::Value`].
*/
#[repr(transparent)]
pub struct DisplayToValue<V: ?Sized>(V);

impl<V: fmt::Display> DisplayToValue<V> {
    /**
    Adapt a [`fmt::Display`] into an [`sval::Value`].
    */
    pub const fn new(value: V) -> DisplayToValue<V> {
        DisplayToValue(value)
    }
}

impl<V: fmt::Display + ?Sized> DisplayToValue<V> {
    /**
    Adapt a reference to a [`fmt::Display`] into an [`sval::Value`].
    */
    pub const fn new_borrowed<'a>(value: &'a V) -> &'a DisplayToValue<V> {
        // SAFETY: `&'a V` and `&'a DisplayToValue<V>` have the same ABI
        unsafe { &*(value as *const _ as *const DisplayToValue<V>) }
    }
}

impl<T: fmt::Debug> sval::Value for DebugToValue<T> {
    fn stream<'sval, S: sval::Stream<'sval> + ?Sized>(&'sval self, stream: &mut S) -> sval::Result {
        struct Writer<S>(S);

        impl<'a, S: sval::Stream<'a>> fmt::Write for Writer<S> {
            fn write_str(&mut self, s: &str) -> fmt::Result {
                self.0.text_fragment_computed(s).map_err(|_| fmt::Error)?;

                Ok(())
            }
        }

        stream.text_begin(None)?;

        write!(Writer(&mut *stream), "{:?}", self.0).map_err(|_| sval::Error::new())?;

        stream.text_end()
    }
}

impl<T: fmt::Display> sval::Value for DisplayToValue<T> {
    fn stream<'sval, S: sval::Stream<'sval> + ?Sized>(&'sval self, stream: &mut S) -> sval::Result {
        struct Writer<S>(S);

        impl<'a, S: sval::Stream<'a>> fmt::Write for Writer<S> {
            fn write_str(&mut self, s: &str) -> fmt::Result {
                self.0.text_fragment_computed(s).map_err(|_| fmt::Error)?;

                Ok(())
            }
        }

        stream.text_begin(None)?;

        write!(Writer(&mut *stream), "{}", self.0).map_err(|_| sval::Error::new())?;

        stream.text_end()
    }
}
