use core::fmt;
use core::hash::Hash;
use std::error::Error;

use crate::encode::{Encoding, EncodingBox};
use crate::runtime::{EncodingParseError, Method};

#[derive(Debug, PartialEq, Eq, Hash)]
pub(crate) enum Inner {
    MethodNotFound,
    EncodingParseError(EncodingParseError),
    MismatchedReturn(EncodingBox, Encoding),
    MismatchedArgumentsCount(usize, usize),
    MismatchedArgument(usize, EncodingBox, Encoding),
}

impl fmt::Display for Inner {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MethodNotFound => write!(f, "method not found"),
            Self::EncodingParseError(e) => write!(f, "{e}"),
            Self::MismatchedReturn(expected, actual) => {
                write!(
                    f,
                    "expected return to have type code '{expected}', but found '{actual}'",
                )
            }
            Self::MismatchedArgumentsCount(expected, actual) => {
                write!(f, "expected {expected} arguments, but {actual} were given",)
            }
            Self::MismatchedArgument(i, expected, actual) => {
                write!(
                    f,
                    "expected argument at index {i} to have type code '{expected}', but found '{actual}'",
                )
            }
        }
    }
}

/// Failed verifying selector on a class.
///
/// This is returned in the error case of [`AnyClass::verify_sel`], see that
/// for details.
///
/// This implements [`Error`], and a description of the error can be retrieved
/// using [`fmt::Display`].
///
/// [`AnyClass::verify_sel`]: crate::runtime::AnyClass::verify_sel
#[derive(Debug, PartialEq, Eq, Hash)]
pub struct VerificationError(Inner);

impl From<EncodingParseError> for VerificationError {
    fn from(e: EncodingParseError) -> Self {
        Self(Inner::EncodingParseError(e))
    }
}

impl From<Inner> for VerificationError {
    fn from(inner: Inner) -> Self {
        Self(inner)
    }
}

impl fmt::Display for VerificationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Delegate to inner
        fmt::Display::fmt(&self.0, f)
    }
}

impl Error for VerificationError {}

/// Relaxed version of `Encoding::equivalent_to_box` that allows
/// `*mut c_void` and `*const c_void` to be used in place of other pointers,
/// and allows signed types where unsigned types are excepted.
///
/// Note: This is a top-level comparison; `*mut *mut c_void` or structures
/// containing `*mut c_void` are not allowed differently than usual.
fn relaxed_equivalent_to_box(encoding: &Encoding, expected: &EncodingBox) -> bool {
    if cfg!(feature = "relax-void-encoding")
        && matches!(encoding, Encoding::Pointer(&Encoding::Void))
        && matches!(expected, EncodingBox::Pointer(_))
    {
        return true;
    }

    if cfg!(feature = "relax-sign-encoding") {
        let actual_signed = match encoding {
            Encoding::UChar => &Encoding::Char,
            Encoding::UShort => &Encoding::Short,
            Encoding::UInt => &Encoding::Int,
            Encoding::ULong => &Encoding::Long,
            Encoding::ULongLong => &Encoding::LongLong,
            enc => enc,
        };
        let expected_signed = match expected {
            EncodingBox::UChar => &EncodingBox::Char,
            EncodingBox::UShort => &EncodingBox::Short,
            EncodingBox::UInt => &EncodingBox::Int,
            EncodingBox::ULong => &EncodingBox::Long,
            EncodingBox::ULongLong => &EncodingBox::LongLong,
            enc => enc,
        };
        if actual_signed == expected_signed {
            return true;
        }
    }

    encoding.equivalent_to_box(expected)
}

pub(crate) fn verify_method_signature(
    method: &Method,
    args: &[Encoding],
    ret: &Encoding,
) -> Result<(), VerificationError> {
    let mut iter = method.types();

    // TODO: Verify stack layout
    let (expected, _stack_layout) = iter.extract_return()?;
    if !relaxed_equivalent_to_box(ret, &expected) {
        return Err(Inner::MismatchedReturn(expected, ret.clone()).into());
    }

    iter.verify_receiver()?;
    iter.verify_sel()?;

    let actual_count = args.len();

    for (i, actual) in args.iter().enumerate() {
        if let Some(res) = iter.next() {
            // TODO: Verify stack layout
            let (expected, _stack_layout) = res?;
            if !relaxed_equivalent_to_box(actual, &expected) {
                return Err(Inner::MismatchedArgument(i, expected, actual.clone()).into());
            }
        } else {
            return Err(Inner::MismatchedArgumentsCount(i, actual_count).into());
        }
    }

    let remaining = iter.count();
    if remaining != 0 {
        return Err(Inner::MismatchedArgumentsCount(actual_count + remaining, actual_count).into());
    }

    let expected_count = method.name().number_of_arguments();
    if expected_count != actual_count {
        return Err(Inner::MismatchedArgumentsCount(expected_count, actual_count).into());
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ffi;
    use crate::runtime::Sel;
    use crate::test_utils;
    use crate::{msg_send, sel};
    use alloc::string::ToString;
    use core::ffi::c_void;
    use core::panic::{RefUnwindSafe, UnwindSafe};

    #[test]
    fn test_verify_message() {
        let cls = test_utils::custom_class();

        assert!(cls.verify_sel::<(), u32>(sel!(foo)).is_ok());
        assert!(cls.verify_sel::<(u32,), ()>(sel!(setFoo:)).is_ok());

        let metaclass = cls.metaclass();
        metaclass
            .verify_sel::<(i32, i32), i32>(sel!(addNumber:toNumber:))
            .unwrap();
    }

    #[test]
    fn test_verify_message_errors() {
        let cls = test_utils::custom_class();

        // Unimplemented selector (missing colon)
        let err = cls.verify_sel::<(), ()>(sel!(setFoo)).unwrap_err();
        assert_eq!(err.to_string(), "method not found");

        // Incorrect return type
        let err = cls.verify_sel::<(u32,), u64>(sel!(setFoo:)).unwrap_err();
        assert_eq!(
            err.to_string(),
            "expected return to have type code 'v', but found 'Q'"
        );

        // Too many arguments
        let err = cls.verify_sel::<(u32, i8), ()>(sel!(setFoo:)).unwrap_err();
        assert_eq!(err.to_string(), "expected 1 arguments, but 2 were given");

        // Too few arguments
        let err = cls.verify_sel::<(), ()>(sel!(setFoo:)).unwrap_err();
        assert_eq!(err.to_string(), "expected 1 arguments, but 0 were given");

        // Incorrect argument type
        let err = cls.verify_sel::<(Sel,), ()>(sel!(setFoo:)).unwrap_err();
        assert_eq!(
            err.to_string(),
            "expected argument at index 0 to have type code 'I', but found ':'"
        );

        // <https://github.com/madsmtm/objc2/issues/566>
        let res = cls.verify_sel::<(), ffi::NSUInteger>(sel!(getNSInteger));
        let expected = if cfg!(feature = "relax-sign-encoding") {
            Ok(())
        } else if cfg!(target_pointer_width = "64") {
            Err("expected return to have type code 'q', but found 'Q'".to_string())
        } else {
            Err("expected return to have type code 'i', but found 'I'".to_string())
        };
        assert_eq!(res.map_err(|e| e.to_string()), expected);

        // Metaclass
        let metaclass = cls.metaclass();
        let err = metaclass
            .verify_sel::<(i32, i32, i32), i32>(sel!(addNumber:toNumber:))
            .unwrap_err();
        assert_eq!(err.to_string(), "expected 2 arguments, but 3 were given");
    }

    #[test]
    #[cfg(debug_assertions)]
    #[should_panic = "invalid message send to -[CustomObject foo]: expected return to have type code 'I', but found '^i'"]
    fn test_send_message_verified() {
        let obj = test_utils::custom_object();
        let _: *const i32 = unsafe { msg_send![&obj, foo] };
    }

    #[test]
    #[cfg(debug_assertions)]
    #[should_panic = "invalid message send to +[CustomObject abcDef]: method not found"]
    fn test_send_message_verified_to_class() {
        let cls = test_utils::custom_class();
        let _: i32 = unsafe { msg_send![cls, abcDef] };
    }

    #[test]
    fn test_marker_traits() {
        fn assert_marker_traits<T: Send + Sync + UnwindSafe + RefUnwindSafe + Unpin>() {}
        assert_marker_traits::<VerificationError>();
    }

    #[test]
    fn test_get_reference() {
        let mut obj = test_utils::custom_object();
        let _: () = unsafe { msg_send![&mut obj, setFoo: 42u32] };

        let res: &u32 = unsafe { msg_send![&obj, fooReference] };
        assert_eq!(*res, 42);
        let res: *const u32 = unsafe { msg_send![&obj, fooReference] };
        assert_eq!(unsafe { *res }, 42);
        let res: *mut u32 = unsafe { msg_send![&obj, fooReference] };
        assert_eq!(unsafe { *res }, 42);
    }

    #[test]
    #[cfg_attr(
        all(debug_assertions, not(feature = "relax-void-encoding")),
        should_panic = "invalid message send to -[CustomObject fooReference]: expected return to have type code '^I', but found '^v'"
    )]
    fn test_get_reference_void() {
        let mut obj = test_utils::custom_object();
        let _: () = unsafe { msg_send![&mut obj, setFoo: 42u32] };

        let res: *mut c_void = unsafe { msg_send![&obj, fooReference] };
        let res: *mut u32 = res.cast();
        assert_eq!(unsafe { *res }, 42);
    }

    #[test]
    #[cfg(debug_assertions)]
    #[should_panic = "invalid message send to -[CustomObject foo]: expected return to have type code 'I', but found '^v'"]
    fn test_get_integer_void() {
        let obj = test_utils::custom_object();
        let _: *mut c_void = unsafe { msg_send![&obj, foo] };
    }
}
