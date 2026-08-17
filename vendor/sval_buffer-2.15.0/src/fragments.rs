use crate::{
    std::fmt::{self, Write as _},
    Error,
};

#[cfg(feature = "alloc")]
use crate::std::{
    borrow::{Cow, ToOwned},
    mem,
};

/**
Buffer text fragments into a single contiguous string.

In no-std environments, this buffer only supports a single
borrowed text fragment. Other methods will fail.
*/
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TextBuf<'sval> {
    buf: FragmentBuf<'sval, str>,
}

impl<'sval> TextBuf<'sval> {
    /**
    Create a new empty text buffer.
    */
    #[inline(always)]
    pub fn new() -> Self {
        TextBuf {
            buf: FragmentBuf::new(""),
        }
    }

    /**
    Buffer a text value into a contiguous string.
    */
    pub fn collect(value: &'sval (impl sval::Value + ?Sized)) -> Result<Self, Error> {
        let mut collector = TextCollector {
            buf: TextBuf::new(),
            err: None,
        };

        value
            .stream(&mut collector)
            .map_err(|_| collector.err.unwrap())?;

        Ok(collector.buf)
    }

    /**
    Buffer a displayable value into a contiguous string.
    */
    pub fn collect_display(value: impl fmt::Display) -> Result<Self, Error> {
        let mut buf = TextBuf::new();
        buf.push_display(value)?;

        Ok(buf)
    }

    /**
    Clear the text buffer so it can be re-used.
    */
    #[inline(always)]
    pub fn clear(&mut self) {
        *self = Default::default();
    }

    /**
    Push a borrowed text fragment onto the buffer.
    */
    #[inline(always)]
    pub fn push_fragment(&mut self, fragment: &'sval str) -> Result<(), Error> {
        self.buf.push(fragment)
    }

    /**
    Push a computed text fragment onto the buffer.

    If the `std` feature of this library is enabled, this method will
    buffer the fragment. In no-std environments this method will fail.
    */
    #[inline(always)]
    pub fn push_fragment_computed(&mut self, fragment: &str) -> Result<(), Error> {
        self.buf.push_computed(fragment)
    }

    /**
    Push a displayable value onto the buffer.

    If the `std` feature of htis library is enabled, this method will
    buffer the fragment. In no-std environments this method will fail.
    */
    pub fn push_display(&mut self, value: impl fmt::Display) -> Result<(), Error> {
        struct Writer<'a, 'sval> {
            buf: &'a mut TextBuf<'sval>,
            err: Option<Error>,
        }

        impl<'a, 'sval> fmt::Write for Writer<'a, 'sval> {
            fn write_str(&mut self, s: &str) -> fmt::Result {
                match self.buf.push_fragment_computed(s) {
                    Ok(()) => Ok(()),
                    Err(e) => {
                        self.err = Some(e);
                        Err(fmt::Error)
                    }
                }
            }
        }

        let mut writer = Writer {
            buf: self,
            err: None,
        };

        write!(&mut writer, "{}", value).map_err(|_| {
            writer
                .err
                .unwrap_or_else(|| Error::invalid_value("formatting failed"))
        })
    }

    /**
    Try get the contents of the buffer as a string borrowed for the `'sval` lifetime.
    */
    #[inline(always)]
    pub fn as_borrowed_str(&self) -> Option<&'sval str> {
        self.buf.as_borrowed_inner()
    }

    /**
    Get the contents of the buffer as a string.
    */
    #[inline(always)]
    pub fn as_str(&self) -> &str {
        self.buf.as_inner()
    }

    #[cfg(feature = "alloc")]
    pub(crate) fn into_owned_in_place(&mut self) -> &mut TextBuf<'static> {
        let TextBuf { ref mut buf } = self;

        crate::assert_static(buf.into_owned_in_place());

        // SAFETY: `self` no longer contains any data borrowed for `'sval`
        unsafe { mem::transmute::<&mut TextBuf<'sval>, &mut TextBuf<'static>>(self) }
    }
}

struct TextCollector<'a> {
    buf: TextBuf<'a>,
    err: Option<Error>,
}

impl<'a> TextCollector<'a> {
    fn try_catch(&mut self, f: impl FnOnce(&mut TextBuf<'a>) -> Result<(), Error>) -> sval::Result {
        match f(&mut self.buf) {
            Ok(()) => Ok(()),
            Err(e) => self.fail(e),
        }
    }

    fn fail(&mut self, err: Error) -> sval::Result {
        self.err = Some(err);
        sval::error()
    }
}

impl<'a> sval::Stream<'a> for TextCollector<'a> {
    fn text_begin(&mut self, _: Option<usize>) -> sval::Result {
        Ok(())
    }

    fn text_fragment(&mut self, fragment: &'a str) -> sval::Result {
        self.try_catch(|buf| buf.push_fragment(fragment))
    }

    fn text_fragment_computed(&mut self, fragment: &str) -> sval::Result {
        self.try_catch(|buf| buf.push_fragment_computed(fragment))
    }

    fn text_end(&mut self) -> sval::Result {
        Ok(())
    }

    fn null(&mut self) -> sval::Result {
        self.fail(Error::unsupported("text", "null"))
    }

    fn bool(&mut self, _: bool) -> sval::Result {
        self.fail(Error::unsupported("text", "boolean"))
    }

    fn i64(&mut self, _: i64) -> sval::Result {
        self.fail(Error::unsupported("text", "integer"))
    }

    fn f64(&mut self, _: f64) -> sval::Result {
        self.fail(Error::unsupported("text", "floating point"))
    }

    fn seq_begin(&mut self, _: Option<usize>) -> sval::Result {
        self.fail(Error::unsupported("text", "sequence"))
    }

    fn seq_value_begin(&mut self) -> sval::Result {
        self.fail(Error::unsupported("text", "sequence"))
    }

    fn seq_value_end(&mut self) -> sval::Result {
        self.fail(Error::unsupported("text", "sequence"))
    }

    fn seq_end(&mut self) -> sval::Result {
        self.fail(Error::unsupported("text", "sequence"))
    }
}

impl<'sval> fmt::Write for TextBuf<'sval> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.push_fragment_computed(s).map_err(|_| fmt::Error)
    }
}

impl<'sval> Default for TextBuf<'sval> {
    #[inline(always)]
    fn default() -> Self {
        TextBuf::new()
    }
}

impl<'sval> From<&'sval str> for TextBuf<'sval> {
    #[inline(always)]
    fn from(fragment: &'sval str) -> Self {
        TextBuf {
            buf: FragmentBuf::new(fragment),
        }
    }
}

impl<'sval> AsRef<str> for TextBuf<'sval> {
    #[inline(always)]
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl<'a> sval::Value for TextBuf<'a> {
    fn stream<'sval, S: sval::Stream<'sval> + ?Sized>(&'sval self, stream: &mut S) -> sval::Result {
        self.as_str().stream(stream)
    }
}

impl<'sval> sval_ref::ValueRef<'sval> for TextBuf<'sval> {
    fn stream_ref<S: sval::Stream<'sval> + ?Sized>(&self, stream: &mut S) -> sval::Result {
        match self.as_borrowed_str() {
            Some(v) => stream.value(v),
            None => {
                let v = self.as_str();

                stream.text_begin(Some(v.len()))?;
                stream.text_fragment_computed(v)?;
                stream.text_end()
            }
        }
    }
}

/**
Buffer binary fragments into a single contiguous slice.

In no-std environments, this buffer only supports a single
borrowed binary fragment. Other methods will fail.
*/
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BinaryBuf<'sval> {
    buf: FragmentBuf<'sval, [u8]>,
}

impl<'sval> BinaryBuf<'sval> {
    /**
    Create a new empty binary buffer.
    */
    #[inline(always)]
    pub fn new() -> Self {
        BinaryBuf {
            buf: FragmentBuf::new(&[]),
        }
    }

    /**
    Buffer a binary value into a contiguous slice.
    */
    pub fn collect(value: &'sval (impl sval::Value + ?Sized)) -> Result<Self, Error> {
        let mut collector = BinaryCollector {
            buf: BinaryBuf::new(),
            err: None,
        };

        value
            .stream(&mut collector)
            .map_err(|_| collector.err.unwrap())?;

        Ok(collector.buf)
    }

    /**
    Clear the binary buffer so it can be re-used.
    */
    #[inline(always)]
    pub fn clear(&mut self) {
        *self = Default::default();
    }

    /**
    Push a borrowed binary fragment onto the buffer.
    */
    #[inline(always)]
    pub fn push_fragment(&mut self, fragment: &'sval [u8]) -> Result<(), Error> {
        self.buf.push(fragment)
    }

    /**
    Push a computed binary fragment onto the buffer.

    If the `std` feature of this library is enabled, this method will
    buffer the fragment. In no-std environments this method will fail.
    */
    #[inline(always)]
    pub fn push_fragment_computed(&mut self, fragment: &[u8]) -> Result<(), Error> {
        self.buf.push_computed(fragment)
    }

    /**
    Try get the contents of the buffer as a slice borrowed for the `'sval` lifetime.
    */
    #[inline(always)]
    pub fn as_borrowed_slice(&self) -> Option<&'sval [u8]> {
        self.buf.as_borrowed_inner()
    }

    /**
    Get the contents of the buffer as a slice.
    */
    #[inline(always)]
    pub fn as_slice(&self) -> &[u8] {
        self.buf.as_inner()
    }

    #[cfg(feature = "alloc")]
    pub(crate) fn into_owned_in_place(&mut self) -> &mut BinaryBuf<'static> {
        let BinaryBuf { ref mut buf } = self;

        crate::assert_static(buf.into_owned_in_place());

        // SAFETY: `self` no longer contains any data borrowed for `'sval`
        unsafe { mem::transmute::<&mut BinaryBuf<'sval>, &mut BinaryBuf<'static>>(self) }
    }
}

struct BinaryCollector<'a> {
    buf: BinaryBuf<'a>,
    err: Option<Error>,
}

impl<'a> BinaryCollector<'a> {
    fn try_catch(
        &mut self,
        f: impl FnOnce(&mut BinaryBuf<'a>) -> Result<(), Error>,
    ) -> sval::Result {
        match f(&mut self.buf) {
            Ok(()) => Ok(()),
            Err(e) => self.fail(e),
        }
    }

    fn fail(&mut self, err: Error) -> sval::Result {
        self.err = Some(err);
        sval::error()
    }
}

impl<'a> sval::Stream<'a> for BinaryCollector<'a> {
    fn binary_begin(&mut self, _: Option<usize>) -> sval::Result {
        Ok(())
    }

    fn binary_fragment(&mut self, fragment: &'a [u8]) -> sval::Result {
        self.try_catch(|buf| buf.push_fragment(fragment))
    }

    fn binary_fragment_computed(&mut self, fragment: &[u8]) -> sval::Result {
        self.try_catch(|buf| buf.push_fragment_computed(fragment))
    }

    fn binary_end(&mut self) -> sval::Result {
        Ok(())
    }

    fn text_begin(&mut self, _: Option<usize>) -> sval::Result {
        self.fail(Error::unsupported("binary", "text"))
    }

    fn text_fragment_computed(&mut self, _: &str) -> sval::Result {
        self.fail(Error::unsupported("binary", "text"))
    }

    fn text_end(&mut self) -> sval::Result {
        self.fail(Error::unsupported("binary", "text"))
    }

    fn null(&mut self) -> sval::Result {
        self.fail(Error::unsupported("binary", "null"))
    }

    fn bool(&mut self, _: bool) -> sval::Result {
        self.fail(Error::unsupported("binary", "boolean"))
    }

    fn u8(&mut self, value: u8) -> sval::Result {
        self.try_catch(|buf| buf.push_fragment_computed(&[value]))
    }

    fn i64(&mut self, _: i64) -> sval::Result {
        self.fail(Error::unsupported("binary", "integer"))
    }

    fn f64(&mut self, _: f64) -> sval::Result {
        self.fail(Error::unsupported("binary", "floating point"))
    }

    fn map_begin(&mut self, _: Option<usize>) -> sval::Result {
        self.fail(Error::unsupported("binary", "map"))
    }

    fn seq_begin(&mut self, _: Option<usize>) -> sval::Result {
        Ok(())
    }

    fn seq_value_begin(&mut self) -> sval::Result {
        Ok(())
    }

    fn seq_value_end(&mut self) -> sval::Result {
        Ok(())
    }

    fn seq_end(&mut self) -> sval::Result {
        Ok(())
    }
}

impl<'sval> Default for BinaryBuf<'sval> {
    #[inline(always)]
    fn default() -> Self {
        BinaryBuf::new()
    }
}

impl<'sval> From<&'sval [u8]> for BinaryBuf<'sval> {
    #[inline(always)]
    fn from(fragment: &'sval [u8]) -> Self {
        BinaryBuf {
            buf: FragmentBuf::new(fragment),
        }
    }
}

impl<'sval, const N: usize> From<&'sval [u8; N]> for BinaryBuf<'sval> {
    fn from(fragment: &'sval [u8; N]) -> Self {
        BinaryBuf {
            buf: FragmentBuf::new(fragment),
        }
    }
}

impl<'sval> AsRef<[u8]> for BinaryBuf<'sval> {
    #[inline(always)]
    fn as_ref(&self) -> &[u8] {
        self.as_slice()
    }
}

impl<'a> sval::Value for BinaryBuf<'a> {
    fn stream<'sval, S: sval::Stream<'sval> + ?Sized>(&'sval self, stream: &mut S) -> sval::Result {
        sval::BinarySlice::new(self.as_slice()).stream(stream)
    }
}

impl<'sval> sval_ref::ValueRef<'sval> for BinaryBuf<'sval> {
    fn stream_ref<S: sval::Stream<'sval> + ?Sized>(&self, stream: &mut S) -> sval::Result {
        match self.as_borrowed_slice() {
            Some(v) => stream.value(sval::BinarySlice::new(v)),
            None => stream.value_computed(sval::BinarySlice::new(self.as_slice())),
        }
    }
}

#[cfg(not(feature = "alloc"))]
trait Fragment {
    #[inline(always)]
    fn to_fragment<'sval>(&'sval self) -> &'sval Self {
        self
    }

    fn can_replace(&self) -> bool;
}

#[cfg(feature = "alloc")]
trait Fragment: ToOwned {
    #[inline(always)]
    fn to_fragment<'sval>(&'sval self) -> Cow<'sval, Self> {
        Cow::Borrowed(self)
    }

    fn extend(buf: &mut Cow<Self>, fragment: &Self);

    fn can_replace(&self) -> bool;

    fn into_owned_in_place<'a, 'sval>(buf: &'a mut Cow<'sval, Self>) -> &'a mut Cow<'static, Self> {
        if let Cow::Borrowed(v) = buf {
            *buf = Cow::Owned(v.to_owned());
        }

        // SAFETY: `self` no longer contains any data borrowed for `'sval`
        unsafe { mem::transmute::<&mut Cow<'_, Self>, &mut Cow<'static, Self>>(buf) }
    }
}

impl Fragment for str {
    #[cfg(feature = "alloc")]
    #[inline(always)]
    fn extend(buf: &mut Cow<Self>, fragment: &Self) {
        buf.to_mut().push_str(fragment);
    }

    fn can_replace(&self) -> bool {
        self.len() == 0
    }
}

impl Fragment for [u8] {
    #[cfg(feature = "alloc")]
    #[inline(always)]
    fn extend(buf: &mut Cow<Self>, fragment: &Self) {
        buf.to_mut().extend(fragment);
    }

    fn can_replace(&self) -> bool {
        self.len() == 0
    }
}

struct FragmentBuf<'sval, T: ?Sized + Fragment> {
    #[cfg(not(feature = "alloc"))]
    value: &'sval T,
    #[cfg(feature = "alloc")]
    value: Cow<'sval, T>,
}

#[cfg(not(feature = "alloc"))]
impl<'sval, T: ?Sized + Fragment> Clone for FragmentBuf<'sval, T> {
    fn clone(&self) -> Self {
        FragmentBuf { value: self.value }
    }
}

#[cfg(feature = "alloc")]
impl<'sval, T: ?Sized + Fragment> Clone for FragmentBuf<'sval, T>
where
    T::Owned: Clone,
{
    fn clone(&self) -> Self {
        FragmentBuf {
            value: self.value.clone(),
        }
    }
}

#[cfg(not(feature = "alloc"))]
impl<'sval, T: ?Sized + Fragment + fmt::Debug> fmt::Debug for FragmentBuf<'sval, T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.value.fmt(f)
    }
}

#[cfg(feature = "alloc")]
impl<'sval, T: ?Sized + Fragment + fmt::Debug> fmt::Debug for FragmentBuf<'sval, T>
where
    T::Owned: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.value.fmt(f)
    }
}

#[cfg(not(feature = "alloc"))]
impl<'sval, T: ?Sized + Fragment + PartialEq> PartialEq for FragmentBuf<'sval, T> {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

#[cfg(feature = "alloc")]
impl<'sval, T: ?Sized + Fragment + PartialEq> PartialEq for FragmentBuf<'sval, T>
where
    T::Owned: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

#[cfg(not(feature = "alloc"))]
impl<'sval, T: ?Sized + Fragment + PartialEq> Eq for FragmentBuf<'sval, T> {}

#[cfg(feature = "alloc")]
impl<'sval, T: ?Sized + Fragment + Eq> Eq for FragmentBuf<'sval, T> where T::Owned: Eq {}

impl<'sval, T: ?Sized + Fragment> FragmentBuf<'sval, T> {
    #[inline(always)]
    fn new(value: &'sval T) -> Self {
        FragmentBuf {
            value: value.to_fragment(),
        }
    }

    #[inline(always)]
    fn push(&mut self, fragment: &'sval T) -> Result<(), Error> {
        if self.value.can_replace() {
            self.value = fragment.to_fragment();

            Ok(())
        } else {
            self.push_computed(fragment)
        }
    }

    #[inline(always)]
    fn push_computed(&mut self, fragment: &T) -> Result<(), Error> {
        #[cfg(feature = "alloc")]
        {
            Fragment::extend(&mut self.value, fragment);

            Ok(())
        }

        #[cfg(not(feature = "alloc"))]
        {
            let _ = fragment;
            Err(Error::no_alloc("computed fragment"))
        }
    }

    #[inline(always)]
    fn as_borrowed_inner(&self) -> Option<&'sval T> {
        #[cfg(feature = "alloc")]
        {
            match self.value {
                Cow::Borrowed(value) => Some(value),
                Cow::Owned(_) => None,
            }
        }

        #[cfg(not(feature = "alloc"))]
        {
            Some(self.value)
        }
    }

    #[inline(always)]
    fn as_inner(&self) -> &T {
        #[cfg(feature = "alloc")]
        {
            &*self.value
        }

        #[cfg(not(feature = "alloc"))]
        {
            self.value
        }
    }

    #[cfg(feature = "alloc")]
    pub(crate) fn into_owned_in_place(&mut self) -> &mut FragmentBuf<'static, T> {
        crate::assert_static(Fragment::into_owned_in_place(&mut self.value));

        // SAFETY: `self` no longer contains any data borrowed for `'sval`
        unsafe { mem::transmute::<&mut FragmentBuf<'sval, T>, &mut FragmentBuf<'static, T>>(self) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sval_ref::ValueRef;

    #[test]
    fn text_buf_empty() {
        assert_eq!("", TextBuf::new().as_borrowed_str().unwrap());
    }

    #[test]
    fn binary_buf_empty() {
        assert_eq!(&[] as &[u8], BinaryBuf::new().as_borrowed_slice().unwrap());
    }

    #[test]
    fn text_fragment_replace() {
        let mut buf = TextBuf::new();

        assert_eq!("", buf.as_str());
        assert_eq!(Some(""), buf.as_borrowed_str());

        buf.push_fragment("abc").unwrap();

        assert_eq!("abc", buf.as_str());
        assert_eq!(Some("abc"), buf.as_borrowed_str());
    }

    #[test]
    fn text_fragment_clear() {
        let mut buf = TextBuf::new();

        buf.push_fragment("abc").unwrap();
        buf.clear();

        assert_eq!("", buf.as_str());
    }

    #[test]
    fn binary_fragment_replace() {
        let mut buf = BinaryBuf::new();

        assert_eq!(b"" as &[u8], buf.as_slice());
        assert_eq!(Some(b"" as &[u8]), buf.as_borrowed_slice());

        buf.push_fragment(b"abc").unwrap();

        assert_eq!(b"abc", buf.as_slice());
        assert_eq!(Some(b"abc" as &[u8]), buf.as_borrowed_slice());
    }

    #[test]
    fn binary_fragment_clear() {
        let mut buf = BinaryBuf::new();

        buf.push_fragment(b"abc").unwrap();
        buf.clear();

        assert_eq!(b"", buf.as_slice());
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn text_fragment_computed() {
        let mut buf = TextBuf::new();

        buf.push_fragment_computed("abc").unwrap();

        assert_eq!("abc", buf.as_str());
        assert_eq!(None, buf.as_borrowed_str());
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn binary_fragment_computed() {
        let mut buf = BinaryBuf::new();

        buf.push_fragment_computed(b"abc").unwrap();

        assert_eq!(b"abc" as &[u8], buf.as_slice());
        assert_eq!(None, buf.as_borrowed_slice());
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn text_fragment_extend() {
        let mut buf = TextBuf::new();

        buf.push_fragment("abc").unwrap();
        buf.push_fragment("def").unwrap();

        assert_eq!("abcdef", buf.as_str());
        assert_eq!(None, buf.as_borrowed_str());
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn binary_fragment_extend() {
        let mut buf = BinaryBuf::new();

        buf.push_fragment(b"abc").unwrap();
        buf.push_fragment(b"def").unwrap();

        assert_eq!(b"abcdef" as &[u8], buf.as_slice());
        assert_eq!(None, buf.as_borrowed_slice());
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn collect_text_buf() {
        let buf = TextBuf::collect("a string").unwrap();

        assert_eq!(Some("a string"), buf.as_borrowed_str());
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn collect_binary_buf() {
        let buf = BinaryBuf::collect(sval::BinarySlice::new(b"a string")).unwrap();

        assert_eq!(Some(b"a string" as &[u8]), buf.as_borrowed_slice());
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn collect_binary_buf_seq() {
        let buf = BinaryBuf::collect(b"a string").unwrap();

        assert_eq!(b"a string" as &[u8], buf.as_slice());
    }

    #[test]
    fn stream_text_buf() {
        let mut buf = TextBuf::new();
        buf.push_fragment("abc").unwrap();

        sval_test::assert_tokens(&buf, {
            use sval_test::Token::*;

            &[TextBegin(Some(3)), TextFragment("abc"), TextEnd]
        });
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn stream_ref_text_buf_computed() {
        let mut buf = TextBuf::new();
        buf.push_fragment("123").unwrap();
        buf.push_fragment("()").unwrap();
        buf.push_fragment("data").unwrap();

        let mut tokens = sval_test::TokenBuf::new();
        buf.stream_ref(&mut tokens).unwrap();

        assert_eq!(tokens.as_tokens(), {
            use sval_test::Token::*;

            &[
                TextBegin(Some(9)),
                TextFragmentComputed("123()data".to_owned()),
                TextEnd,
            ]
        });
    }

    #[test]
    fn stream_ref_text_buf_borrowed() {
        let mut buf = TextBuf::new();
        buf.push_fragment("123").unwrap();

        let mut tokens = sval_test::TokenBuf::new();
        buf.stream_ref(&mut tokens).unwrap();

        assert_eq!(tokens.as_tokens(), {
            use sval_test::Token::*;

            &[TextBegin(Some(3)), TextFragment("123"), TextEnd]
        });
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn collect_text_buf_display() {
        let buf = TextBuf::collect_display(true).unwrap();

        assert_eq!("true", buf.as_str());
        assert!(buf.as_borrowed_str().is_none());
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn stream_text_buf_display() {
        let mut buf = TextBuf::new();

        buf.push_display(true).unwrap();
        buf.push_display(false).unwrap();

        assert_eq!("truefalse", buf.as_str());
        assert!(buf.as_borrowed_str().is_none());
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn stream_binary_buf_computed() {
        let mut buf = BinaryBuf::new();
        buf.push_fragment(b"abc").unwrap();
        buf.push_fragment_computed(b"def").unwrap();

        sval_test::assert_tokens(&buf, {
            use sval_test::Token::*;

            &[BinaryBegin(Some(6)), BinaryFragment(b"abcdef"), BinaryEnd]
        });
    }

    #[test]
    fn stream_ref_binary_buf_borrowed() {
        let mut buf = BinaryBuf::new();
        buf.push_fragment(b"abc").unwrap();

        let mut tokens = sval_test::TokenBuf::new();
        buf.stream_ref(&mut tokens).unwrap();

        assert_eq!(tokens.as_tokens(), {
            use sval_test::Token::*;

            &[BinaryBegin(Some(3)), BinaryFragment(b"abc"), BinaryEnd]
        });
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn stream_ref_binary_buf_computed() {
        let mut buf = BinaryBuf::new();
        buf.push_fragment(b"abc").unwrap();
        buf.push_fragment_computed(b"def").unwrap();

        let mut tokens = sval_test::TokenBuf::new();
        buf.stream_ref(&mut tokens).unwrap();

        assert_eq!(tokens.as_tokens(), {
            use sval_test::Token::*;

            &[
                BinaryBegin(Some(6)),
                BinaryFragmentComputed(b"abcdef".to_vec()),
                BinaryEnd,
            ]
        });
    }
}
