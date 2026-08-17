use crate::{
    std::fmt::{self, Write as _},
    Error, Result, Stream, Tag, Value,
};

/**
Adapt a [`fmt::Display`] into an [`sval::Value`].
*/
#[repr(transparent)]
pub struct Display<V: ?Sized>(V);

impl<V: fmt::Display> Display<V> {
    /**
    Adapt a [`fmt::Display`] into an [`sval::Value`].
    */
    pub fn new(value: V) -> Display<V> {
        Display(value)
    }

    /**
    Get a reference to the inner value.
    */
    pub fn inner(&self) -> &V {
        &self.0
    }

    /**
    Convert into the inner value.
    */
    pub fn into_inner(self) -> V {
        self.0
    }
}

impl<V: fmt::Display + ?Sized> Display<V> {
    /**
    Adapt a reference to a [`fmt::Display`] into an [`sval::Value`].
    */
    pub fn new_borrowed<'a>(value: &'a V) -> &'a Display<V> {
        // SAFETY: `&'a V` and `&'a Display<V>` have the same ABI
        unsafe { &*(value as *const _ as *const Display<V>) }
    }
}

impl<V: fmt::Display> Value for Display<V> {
    fn stream<'sval, S: Stream<'sval> + ?Sized>(&'sval self, stream: &mut S) -> Result {
        stream.text_begin(None)?;
        stream_display_fragments(stream, &self.0)?;
        stream.text_end()
    }
}

/**
Stream a [`fmt::Display`] as text into a [`Stream`].

This function can be used to stream a value as text using its `Display` implementation.
*/
pub fn stream_display<'sval>(
    stream: &mut (impl Stream<'sval> + ?Sized),
    value: impl fmt::Display,
) -> Result {
    stream.value_computed(&Display::new(value))
}

/**
Stream a [`fmt::Display`] as text fragments into a [`Stream`], without calling [`Stream::text_begin`] or [`Stream::text_end`].

This function can be used to stream a part of a larger value.
*/
pub fn stream_display_fragments<'sval>(
    stream: &mut (impl Stream<'sval> + ?Sized),
    value: impl fmt::Display,
) -> Result {
    write!(
        Writer(|fragment: &str| stream
            .text_fragment_computed(fragment)
            .map_err(|_| fmt::Error)),
        "{}",
        value
    )
    .map_err(|_| Error::new())
}

struct Writer<F>(F);

impl<F: FnMut(&str) -> fmt::Result> fmt::Write for Writer<F> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        (self.0)(s).map_err(|_| fmt::Error)
    }
}

impl Value for char {
    fn stream<'sval, S: Stream<'sval> + ?Sized>(&'sval self, stream: &mut S) -> Result {
        let mut buf = [0; 4];
        let value = &*self.encode_utf8(&mut buf);

        stream.text_begin(Some(value.len()))?;
        stream.text_fragment_computed(value)?;
        stream.text_end()
    }
}

impl Value for str {
    fn stream<'sval, S: Stream<'sval> + ?Sized>(&'sval self, stream: &mut S) -> Result {
        stream.text_begin(Some(self.len()))?;
        stream.text_fragment(self)?;
        stream.text_end()
    }

    fn tag(&self) -> Option<Tag> {
        None
    }

    fn to_text(&self) -> Option<&str> {
        Some(self)
    }
}

#[cfg(feature = "alloc")]
mod alloc_support {
    use super::*;

    use crate::std::string::String;

    impl Value for String {
        fn stream<'a, S: Stream<'a> + ?Sized>(&'a self, stream: &mut S) -> Result {
            (&**self).stream(stream)
        }

        fn tag(&self) -> Option<Tag> {
            None
        }

        #[inline]
        fn to_text(&self) -> Option<&str> {
            Some(self)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TextLike(&'static str);
    struct TextLikeComputed(&'static str);

    impl Value for TextLike {
        fn stream<'sval, S: Stream<'sval> + ?Sized>(&'sval self, stream: &mut S) -> Result {
            self.0.stream(stream)
        }
    }

    impl Value for TextLikeComputed {
        fn stream<'sval, S: Stream<'sval> + ?Sized>(&'sval self, stream: &mut S) -> Result {
            stream.text_begin(Some(self.0.len()))?;
            stream.text_fragment_computed(self.0)?;
            stream.text_end()
        }
    }

    #[test]
    fn string_cast() {
        assert_eq!(Some("a string"), "a string".to_text());
        assert_eq!(Some("a string"), TextLike("a string").to_text());
        assert_eq!(None, TextLikeComputed("123").to_text());
    }

    #[test]
    fn string_tag() {
        assert_eq!(None, TextLike("123").tag());
        assert_eq!(None, TextLikeComputed("123").tag());
    }
}
