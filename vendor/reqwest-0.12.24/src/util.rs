use crate::header::{Entry, HeaderMap, HeaderValue, OccupiedEntry};
use std::fmt;

pub fn basic_auth<U, P>(username: U, password: Option<P>) -> HeaderValue
where
    U: std::fmt::Display,
    P: std::fmt::Display,
{
    use base64::prelude::BASE64_STANDARD;
    use base64::write::EncoderWriter;
    use std::io::Write;

    let mut buf = b"Basic ".to_vec();
    {
        let mut encoder = EncoderWriter::new(&mut buf, &BASE64_STANDARD);
        let _ = write!(encoder, "{username}:");
        if let Some(password) = password {
            let _ = write!(encoder, "{password}");
        }
    }
    let mut header = HeaderValue::from_maybe_shared(bytes::Bytes::from(buf))
        .expect("base64 is always valid HeaderValue");
    header.set_sensitive(true);
    header
}

#[cfg(not(target_arch = "wasm32"))]
pub(crate) fn fast_random() -> u64 {
    use std::cell::Cell;
    use std::collections::hash_map::RandomState;
    use std::hash::{BuildHasher, Hasher};

    thread_local! {
        static KEY: RandomState = RandomState::new();
        static COUNTER: Cell<u64> = Cell::new(0);
    }

    KEY.with(|key| {
        COUNTER.with(|ctr| {
            let n = ctr.get().wrapping_add(1);
            ctr.set(n);

            let mut h = key.build_hasher();
            h.write_u64(n);
            h.finish()
        })
    })
}

pub(crate) fn replace_headers(dst: &mut HeaderMap, src: HeaderMap) {
    // IntoIter of HeaderMap yields (Option<HeaderName>, HeaderValue).
    // The first time a name is yielded, it will be Some(name), and if
    // there are more values with the same name, the next yield will be
    // None.

    let mut prev_entry: Option<OccupiedEntry<_>> = None;
    for (key, value) in src {
        match key {
            Some(key) => match dst.entry(key) {
                Entry::Occupied(mut e) => {
                    e.insert(value);
                    prev_entry = Some(e);
                }
                Entry::Vacant(e) => {
                    let e = e.insert_entry(value);
                    prev_entry = Some(e);
                }
            },
            None => match prev_entry {
                Some(ref mut entry) => {
                    entry.append(value);
                }
                None => unreachable!("HeaderMap::into_iter yielded None first"),
            },
        }
    }
}

#[cfg(feature = "cookies")]
#[cfg(not(target_arch = "wasm32"))]
pub(crate) fn add_cookie_header(
    headers: &mut HeaderMap,
    cookie_store: &dyn crate::cookie::CookieStore,
    url: &url::Url,
) {
    if let Some(header) = cookie_store.cookies(url) {
        headers.insert(crate::header::COOKIE, header);
    }
}

pub(crate) struct Escape<'a>(&'a [u8]);

#[cfg(not(target_arch = "wasm32"))]
impl<'a> Escape<'a> {
    pub(crate) fn new(bytes: &'a [u8]) -> Self {
        Escape(bytes)
    }
}

impl fmt::Debug for Escape<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "b\"{}\"", self)?;
        Ok(())
    }
}

impl fmt::Display for Escape<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for &c in self.0 {
            // https://doc.rust-lang.org/reference.html#byte-escapes
            if c == b'\n' {
                write!(f, "\\n")?;
            } else if c == b'\r' {
                write!(f, "\\r")?;
            } else if c == b'\t' {
                write!(f, "\\t")?;
            } else if c == b'\\' || c == b'"' {
                write!(f, "\\{}", c as char)?;
            } else if c == b'\0' {
                write!(f, "\\0")?;
            // ASCII printable
            } else if c >= 0x20 && c < 0x7f {
                write!(f, "{}", c as char)?;
            } else {
                write!(f, "\\x{c:02x}")?;
            }
        }
        Ok(())
    }
}
