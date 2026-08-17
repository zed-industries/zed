use std::ops::Range;
use std::str::from_utf8;

use memchr::memchr;

use sqlx_core::bytes::Bytes;

use crate::error::Error;
use crate::io::ProtocolDecode;
use crate::message::{BackendMessage, BackendMessageFormat};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(u8)]
pub enum PgSeverity {
    Panic,
    Fatal,
    Error,
    Warning,
    Notice,
    Debug,
    Info,
    Log,
}

impl PgSeverity {
    #[inline]
    pub fn is_error(self) -> bool {
        matches!(self, Self::Panic | Self::Fatal | Self::Error)
    }
}

impl TryFrom<&str> for PgSeverity {
    type Error = Error;

    fn try_from(s: &str) -> Result<PgSeverity, Error> {
        let result = match s {
            "PANIC" => PgSeverity::Panic,
            "FATAL" => PgSeverity::Fatal,
            "ERROR" => PgSeverity::Error,
            "WARNING" => PgSeverity::Warning,
            "NOTICE" => PgSeverity::Notice,
            "DEBUG" => PgSeverity::Debug,
            "INFO" => PgSeverity::Info,
            "LOG" => PgSeverity::Log,

            severity => {
                return Err(err_protocol!("unknown severity: {:?}", severity));
            }
        };

        Ok(result)
    }
}

#[derive(Debug)]
pub struct Notice {
    storage: Bytes,
    severity: PgSeverity,
    message: Range<usize>,
    code: Range<usize>,
}

impl Notice {
    #[inline]
    pub fn severity(&self) -> PgSeverity {
        self.severity
    }

    #[inline]
    pub fn code(&self) -> &str {
        self.get_cached_str(self.code.clone())
    }

    #[inline]
    pub fn message(&self) -> &str {
        self.get_cached_str(self.message.clone())
    }

    // Field descriptions available here:
    //  https://www.postgresql.org/docs/current/protocol-error-fields.html

    #[inline]
    pub fn get(&self, ty: u8) -> Option<&str> {
        self.get_raw(ty).and_then(|v| from_utf8(v).ok())
    }

    pub fn get_raw(&self, ty: u8) -> Option<&[u8]> {
        self.fields()
            .filter(|(field, _)| *field == ty)
            .map(|(_, range)| &self.storage[range])
            .next()
    }
}

impl Notice {
    #[inline]
    fn fields(&self) -> Fields<'_> {
        Fields {
            storage: &self.storage,
            offset: 0,
        }
    }

    #[inline]
    fn get_cached_str(&self, cache: Range<usize>) -> &str {
        // unwrap: this cannot fail at this stage
        from_utf8(&self.storage[cache]).unwrap()
    }
}

impl ProtocolDecode<'_> for Notice {
    fn decode_with(buf: Bytes, _: ()) -> Result<Self, Error> {
        // In order to support PostgreSQL 9.5 and older we need to parse the localized S field.
        // Newer versions additionally come with the V field that is guaranteed to be in English.
        // We thus read both versions and prefer the unlocalized one if available.
        const DEFAULT_SEVERITY: PgSeverity = PgSeverity::Log;
        let mut severity_v = None;
        let mut severity_s = None;
        let mut message = 0..0;
        let mut code = 0..0;

        // we cache the three always present fields
        // this enables to keep the access time down for the fields most likely accessed

        let fields = Fields {
            storage: &buf,
            offset: 0,
        };

        for (field, v) in fields {
            if !(message.is_empty() || code.is_empty()) {
                // stop iterating when we have the 3 fields we were looking for
                // we assume V (severity) was the first field as it should be
                break;
            }

            match field {
                b'S' => {
                    severity_s = from_utf8(&buf[v.clone()])
                        // If the error string is not UTF-8, we have no hope of interpreting it,
                        // localized or not. The `V` field would likely fail to parse as well.
                        .map_err(|_| notice_protocol_err())?
                        .try_into()
                        // If we couldn't parse the severity here, it might just be localized.
                        .ok();
                }

                b'V' => {
                    // Propagate errors here, because V is not localized and
                    // thus we are missing a possible variant.
                    severity_v = Some(
                        from_utf8(&buf[v.clone()])
                            .map_err(|_| notice_protocol_err())?
                            .try_into()?,
                    );
                }

                b'M' => {
                    _ = from_utf8(&buf[v.clone()]).map_err(|_| notice_protocol_err())?;
                    message = v;
                }

                b'C' => {
                    _ = from_utf8(&buf[v.clone()]).map_err(|_| notice_protocol_err())?;
                    code = v;
                }

                // If more fields are added, make sure to check that they are valid UTF-8,
                // otherwise the get_cached_str method will panic.
                _ => {}
            }
        }

        Ok(Self {
            severity: severity_v.or(severity_s).unwrap_or(DEFAULT_SEVERITY),
            message,
            code,
            storage: buf,
        })
    }
}

impl BackendMessage for Notice {
    const FORMAT: BackendMessageFormat = BackendMessageFormat::NoticeResponse;

    fn decode_body(buf: Bytes) -> Result<Self, Error> {
        // Keeping both impls for now
        Self::decode_with(buf, ())
    }
}

/// An iterator over each field in the Error (or Notice) response.
struct Fields<'a> {
    storage: &'a [u8],
    offset: usize,
}

impl<'a> Iterator for Fields<'a> {
    type Item = (u8, Range<usize>);

    fn next(&mut self) -> Option<Self::Item> {
        // The fields in the response body are sequentially stored as [tag][string],
        // ending in a final, additional [nul]

        let ty = *self.storage.get(self.offset)?;

        if ty == 0 {
            return None;
        }

        // Consume the type byte
        self.offset = self.offset.checked_add(1)?;

        let start = self.offset;

        let len = memchr(b'\0', self.storage.get(start..)?)?;

        // Neither can overflow as they will always be `<= self.storage.len()`.
        let end = self.offset + len;
        self.offset = end + 1;

        Some((ty, start..end))
    }
}

fn notice_protocol_err() -> Error {
    // https://github.com/launchbadge/sqlx/issues/1144
    Error::Protocol(
        "Postgres returned a non-UTF-8 string for its error message. \
         This is most likely due to an error that occurred during authentication and \
         the default lc_messages locale is not binary-compatible with UTF-8. \
         See the server logs for the error details."
            .into(),
    )
}

#[test]
fn test_decode_error_response() {
    const DATA: &[u8] = b"SNOTICE\0VNOTICE\0C42710\0Mextension \"uuid-ossp\" already exists, skipping\0Fextension.c\0L1656\0RCreateExtension\0\0";

    let m = Notice::decode(Bytes::from_static(DATA)).unwrap();

    assert_eq!(
        m.message(),
        "extension \"uuid-ossp\" already exists, skipping"
    );

    assert!(matches!(m.severity(), PgSeverity::Notice));
    assert_eq!(m.code(), "42710");
}

#[cfg(all(test, not(debug_assertions)))]
#[bench]
fn bench_error_response_get_message(b: &mut test::Bencher) {
    const DATA: &[u8] = b"SNOTICE\0VNOTICE\0C42710\0Mextension \"uuid-ossp\" already exists, skipping\0Fextension.c\0L1656\0RCreateExtension\0\0";

    let res = Notice::decode(test::black_box(Bytes::from_static(DATA))).unwrap();

    b.iter(|| {
        let _ = test::black_box(&res).message();
    });
}

#[cfg(all(test, not(debug_assertions)))]
#[bench]
fn bench_decode_error_response(b: &mut test::Bencher) {
    const DATA: &[u8] = b"SNOTICE\0VNOTICE\0C42710\0Mextension \"uuid-ossp\" already exists, skipping\0Fextension.c\0L1656\0RCreateExtension\0\0";

    b.iter(|| {
        let _ = Notice::decode(test::black_box(Bytes::from_static(DATA)));
    });
}
