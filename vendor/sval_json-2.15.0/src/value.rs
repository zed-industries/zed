use core::{borrow::Borrow, fmt};

/**
A string containing encoded JSON.

Streaming a `JsonStr` will embed its contents directly rather
than treating them as a string.
*/
#[repr(transparent)]
#[derive(PartialEq, Eq, Hash)]
pub struct JsonStr(str);

impl JsonStr {
    /**
    Treat a string as native JSON.
    */
    pub const fn new<'a>(json: &'a str) -> &'a Self {
        // SAFETY: `JsonStr` and `str` have the same ABI
        unsafe { &*(json as *const _ as *const JsonStr) }
    }

    /**
    Get a reference to the underlying string.
    */
    pub const fn as_str(&self) -> &str {
        &self.0
    }

    /**
    Get a reference to the bytes of the underlying string.
    */
    pub const fn as_bytes(&self) -> &[u8] {
        self.0.as_bytes()
    }
}

impl<'a> From<&'a str> for &'a JsonStr {
    fn from(value: &'a str) -> Self {
        JsonStr::new(value)
    }
}

impl fmt::Debug for JsonStr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(&self.0, f)
    }
}

impl fmt::Display for JsonStr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}

impl PartialEq<str> for JsonStr {
    fn eq(&self, other: &str) -> bool {
        self.as_str() == other
    }
}

impl sval::Value for JsonStr {
    fn stream<'sval, S: sval::Stream<'sval> + ?Sized>(&'sval self, stream: &mut S) -> sval::Result {
        stream.tagged_begin(Some(&crate::tags::JSON_VALUE), None, None)?;
        stream.value(&self.0)?;
        stream.tagged_end(Some(&crate::tags::JSON_VALUE), None, None)
    }
}

impl AsRef<str> for JsonStr {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Borrow<str> for JsonStr {
    fn borrow(&self) -> &str {
        &self.0
    }
}

#[cfg(feature = "alloc")]
mod alloc_support {
    use super::*;

    use alloc::boxed::Box;

    impl JsonStr {
        /**
        Treat a string as native JSON.
        */
        pub fn boxed(json: impl Into<Box<str>>) -> Box<Self> {
            let json = json.into();

            // SAFETY: `JsonStr` and `str` have the same ABI
            unsafe { Box::from_raw(Box::into_raw(json) as *mut str as *mut JsonStr) }
        }
    }

    impl From<Box<str>> for Box<JsonStr> {
        fn from(value: Box<str>) -> Self {
            // SAFETY: `JsonStr` and `str` have the same ABI
            unsafe { Box::from_raw(Box::into_raw(value) as *mut JsonStr) }
        }
    }
}
