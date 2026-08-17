#[allow(unused, deprecated)]
use std::ascii::AsciiExt;
use std::cmp;
use std::default::Default;
use std::fmt;
use std::str;

#[cfg(test)]
use self::internal::IntoQuality;

/// Represents a quality used in quality values.
///
/// Can be created with the `q` function.
///
/// # Implementation notes
///
/// The quality value is defined as a number between 0 and 1 with three decimal places. This means
/// there are 1001 possible values. Since floating point numbers are not exact and the smallest
/// floating point data type (`f32`) consumes four bytes, hyper uses an `u16` value to store the
/// quality internally. For performance reasons you may set quality directly to a value between
/// 0 and 1000 e.g. `Quality(532)` matches the quality `q=0.532`.
///
/// [RFC7231 Section 5.3.1](https://tools.ietf.org/html/rfc7231#section-5.3.1)
/// gives more information on quality values in HTTP header fields.
#[derive(Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Quality(u16);

impl Default for Quality {
    fn default() -> Quality {
        Quality(1000)
    }
}

/// Represents an item with a quality value as defined in
/// [RFC7231](https://tools.ietf.org/html/rfc7231#section-5.3.1).
#[derive(Clone, PartialEq, Debug)]
pub struct QualityValue<T> {
    /// The actual contents of the field.
    value: T,
    /// The quality (client or server preference) for the value.
    quality: Quality,
}

impl<T> QualityValue<T> {
    /// Creates a new `QualityValue` from an item and a quality.
    pub fn new(value: T, quality: Quality) -> QualityValue<T> {
        QualityValue {
            value,
            quality,
        }
    }

    /*
    /// Convenience function to set a `Quality` from a float or integer.
    ///
    /// Implemented for `u16` and `f32`.
    ///
    /// # Panic
    ///
    /// Panics if value is out of range.
    pub fn with_q<Q: IntoQuality>(mut self, q: Q) -> QualityValue<T> {
        self.quality = q.into_quality();
        self
    }
    */
}

impl<T> From<T> for QualityValue<T> {
    fn from(value: T) -> QualityValue<T> {
        QualityValue {
            value,
            quality: Quality::default(),
        }
    }
}

impl<T: PartialEq> cmp::PartialOrd for QualityValue<T> {
    fn partial_cmp(&self, other: &QualityValue<T>) -> Option<cmp::Ordering> {
        self.quality.partial_cmp(&other.quality)
    }
}

impl<T: fmt::Display> fmt::Display for QualityValue<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.value, f)?;
        match self.quality.0 {
            1000 => Ok(()),
            0 => f.write_str("; q=0"),
            x => write!(f, "; q=0.{}", format!("{:03}", x).trim_right_matches('0'))
        }
    }
}

impl<T: str::FromStr> str::FromStr for QualityValue<T> {
    type Err = ::Error;
    fn from_str(s: &str) -> ::Result<QualityValue<T>> {
        // Set defaults used if parsing fails.
        let mut raw_item = s;
        let mut quality = 1f32;

        let parts: Vec<&str> = s.rsplitn(2, ';').map(|x| x.trim()).collect();
        if parts.len() == 2 {
            if parts[0].len() < 2 {
                return Err(::Error::invalid());
            }
            if parts[0].starts_with("q=") || parts[0].starts_with("Q=") {
                let q_part = &parts[0][2..parts[0].len()];
                if q_part.len() > 5 {
                    return Err(::Error::invalid());
                }
                match q_part.parse::<f32>() {
                    Ok(q_value) => {
                        if 0f32 <= q_value && q_value <= 1f32 {
                            quality = q_value;
                            raw_item = parts[1];
                            } else {
                                return Err(::Error::invalid());
                            }
                        },
                    Err(_) => {
                        return Err(::Error::invalid())
                    },
                }
            }
        }
        match raw_item.parse::<T>() {
            // we already checked above that the quality is within range
            Ok(item) => Ok(QualityValue::new(item, from_f32(quality))),
            Err(_) => {
                Err(::Error::invalid())
            },
        }
    }
}

#[inline]
fn from_f32(f: f32) -> Quality {
    // this function is only used internally. A check that `f` is within range
    // should be done before calling this method. Just in case, this
    // debug_assert should catch if we were forgetful
    debug_assert!(f >= 0f32 && f <= 1f32, "q value must be between 0.0 and 1.0");
    Quality((f * 1000f32) as u16)
}

#[cfg(test)]
fn q<T: IntoQuality>(val: T) -> Quality {
    val.into_quality()
}

mod internal {
    use super::Quality;

    // TryFrom is probably better, but it's not stable. For now, we want to
    // keep the functionality of the `q` function, while allowing it to be
    // generic over `f32` and `u16`.
    //
    // `q` would panic before, so keep that behavior. `TryFrom` can be
    // introduced later for a non-panicking conversion.

    pub trait IntoQuality: Sealed + Sized {
        fn into_quality(self) -> Quality;
    }

    impl IntoQuality for f32 {
        fn into_quality(self) -> Quality {
            assert!(self >= 0f32 && self <= 1f32, "float must be between 0.0 and 1.0");
            super::from_f32(self)
        }
    }

    impl IntoQuality for u16 {
        fn into_quality(self) -> Quality {
            assert!(self <= 1000, "u16 must be between 0 and 1000");
            Quality(self)
        }
    }


    pub trait Sealed {}
    impl Sealed for u16 {}
    impl Sealed for f32 {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quality_item_fmt_q_1() {
        let x = QualityValue::from("foo");
        assert_eq!(format!("{}", x), "foo");
    }
    #[test]
    fn test_quality_item_fmt_q_0001() {
        let x = QualityValue::new("foo", Quality(1));
        assert_eq!(format!("{}", x), "foo; q=0.001");
    }
    #[test]
    fn test_quality_item_fmt_q_05() {
        let x = QualityValue::new("foo", Quality(500));
        assert_eq!(format!("{}", x), "foo; q=0.5");
    }

    #[test]
    fn test_quality_item_fmt_q_0() {
        let x = QualityValue::new("foo", Quality(0));
        assert_eq!(x.to_string(), "foo; q=0");
    }

    #[test]
    fn test_quality_item_from_str1() {
        let x: QualityValue<String> = "chunked".parse().unwrap();
        assert_eq!(x, QualityValue { value: "chunked".to_owned(), quality: Quality(1000), });
    }
    #[test]
    fn test_quality_item_from_str2() {
        let x: QualityValue<String> = "chunked; q=1".parse().unwrap();
        assert_eq!(x, QualityValue { value: "chunked".to_owned(), quality: Quality(1000), });
    }
    #[test]
    fn test_quality_item_from_str3() {
        let x: QualityValue<String> = "gzip; q=0.5".parse().unwrap();
        assert_eq!(x, QualityValue { value: "gzip".to_owned(), quality: Quality(500), });
    }
    #[test]
    fn test_quality_item_from_str4() {
        let x: QualityValue<String> = "gzip; q=0.273".parse().unwrap();
        assert_eq!(x, QualityValue { value: "gzip".to_owned(), quality: Quality(273), });
    }
    #[test]
    fn test_quality_item_from_str5() {
        assert!("gzip; q=0.2739999".parse::<QualityValue<String>>().is_err());
    }

    #[test]
    fn test_quality_item_from_str6() {
        assert!("gzip; q=2".parse::<QualityValue<String>>().is_err());
    }
    #[test]
    fn test_quality_item_ordering() {
        let x: QualityValue<String> = "gzip; q=0.5".parse().unwrap();
        let y: QualityValue<String> = "gzip; q=0.273".parse().unwrap();
        assert!(x > y)
    }

    #[test]
    fn test_quality() {
        assert_eq!(q(0.5), Quality(500));
    }

    #[test]
    #[should_panic]
    fn test_quality_invalid() {
        q(-1.0);
    }

    #[test]
    #[should_panic]
    fn test_quality_invalid2() {
        q(2.0);
    }

    #[test]
    fn test_fuzzing_bugs() {
        assert!("99999;".parse::<QualityValue<String>>().is_err());
        assert!("\x0d;;;=\u{d6aa}==".parse::<QualityValue<String>>().is_ok())
    }
}
