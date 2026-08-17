/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

use std::any::Any;
use std::error::Error as StdError;
use std::fmt;
use std::sync::Arc;

/// Abstraction over `Box<dyn T + Send + Sync>` that provides `Debug` and optionally `Clone`.
///
/// The orchestrator uses `TypeErasedBox` to avoid the complication of six or more generic parameters
/// and to avoid the monomorphization that brings with it.
///
/// # Examples
///
/// Creating a box:
/// ```no_run
/// use aws_smithy_types::type_erasure::TypeErasedBox;
///
/// let boxed = TypeErasedBox::new("some value");
/// ```
///
/// Downcasting a box:
/// ```no_run
/// # use aws_smithy_types::type_erasure::TypeErasedBox;
/// # let boxed = TypeErasedBox::new("some value".to_string());
/// let value: Option<&String> = boxed.downcast_ref::<String>();
/// ```
///
/// Converting a box back into its value:
/// ```
/// # use aws_smithy_types::type_erasure::TypeErasedBox;
/// let boxed = TypeErasedBox::new("some value".to_string());
/// let value: String = *boxed.downcast::<String>().expect("it is a string");
/// ```
pub struct TypeErasedBox {
    field: Box<dyn Any + Send + Sync>,
    #[allow(clippy::type_complexity)]
    debug: Arc<
        dyn Fn(&Box<dyn Any + Send + Sync>, &mut fmt::Formatter<'_>) -> fmt::Result + Send + Sync,
    >,
    #[allow(clippy::type_complexity)]
    clone: Option<Arc<dyn Fn(&Box<dyn Any + Send + Sync>) -> TypeErasedBox + Send + Sync>>,
}

#[cfg(feature = "test-util")]
impl TypeErasedBox {
    /// Often, when testing the orchestrator or its components, it's necessary to provide a
    /// `TypeErasedBox` to serve as an `Input` for `invoke`. In cases where the type won't actually
    /// be accessed during testing, use this method to generate a `TypeErasedBox` that makes it
    /// clear that "for the purpose of this test, the `Input` doesn't matter."
    pub fn doesnt_matter() -> Self {
        Self::new("doesn't matter")
    }
}

impl fmt::Debug for TypeErasedBox {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("TypeErasedBox[")?;
        if self.clone.is_some() {
            f.write_str("Clone")?;
        } else {
            f.write_str("!Clone")?;
        }
        f.write_str("]:")?;
        (self.debug)(&self.field, f)
    }
}

impl TypeErasedBox {
    /// Create a new `TypeErasedBox` from `value` of type `T`
    pub fn new<T: Send + Sync + fmt::Debug + 'static>(value: T) -> Self {
        let debug = |value: &Box<dyn Any + Send + Sync>, f: &mut fmt::Formatter<'_>| {
            fmt::Debug::fmt(value.downcast_ref::<T>().expect("type-checked"), f)
        };
        Self {
            field: Box::new(value),
            debug: Arc::new(debug),
            clone: None,
        }
    }

    /// Create a new cloneable `TypeErasedBox` from the given `value`.
    pub fn new_with_clone<T: Send + Sync + Clone + fmt::Debug + 'static>(value: T) -> Self {
        let debug = |value: &Box<dyn Any + Send + Sync>, f: &mut fmt::Formatter<'_>| {
            fmt::Debug::fmt(value.downcast_ref::<T>().expect("type-checked"), f)
        };
        let clone = |value: &Box<dyn Any + Send + Sync>| {
            TypeErasedBox::new_with_clone(value.downcast_ref::<T>().expect("typechecked").clone())
        };
        Self {
            field: Box::new(value),
            debug: Arc::new(debug),
            clone: Some(Arc::new(clone)),
        }
    }

    /// Attempts to clone this box.
    ///
    /// Note: this will only ever succeed if the box was created with [`TypeErasedBox::new_with_clone`].
    pub fn try_clone(&self) -> Option<Self> {
        Some((self.clone.as_ref()?)(&self.field))
    }

    /// Downcast into a `Box<T>`, or return `Self` if it is not a `T`.
    pub fn downcast<T: fmt::Debug + Send + Sync + 'static>(self) -> Result<Box<T>, Self> {
        let TypeErasedBox {
            field,
            debug,
            clone,
        } = self;
        field.downcast().map_err(|field| Self {
            field,
            debug,
            clone,
        })
    }

    /// Downcast as a `&T`, or return `None` if it is not a `T`.
    pub fn downcast_ref<T: fmt::Debug + Send + Sync + 'static>(&self) -> Option<&T> {
        self.field.downcast_ref()
    }

    /// Downcast as a `&mut T`, or return `None` if it is not a `T`.
    pub fn downcast_mut<T: fmt::Debug + Send + Sync + 'static>(&mut self) -> Option<&mut T> {
        self.field.downcast_mut()
    }
}

impl From<TypeErasedError> for TypeErasedBox {
    fn from(value: TypeErasedError) -> Self {
        TypeErasedBox {
            field: value.field,
            debug: value.debug,
            clone: None,
        }
    }
}

/// A new-type around `Box<dyn Error + Debug + Send + Sync>` that also implements `Error`
pub struct TypeErasedError {
    field: Box<dyn Any + Send + Sync>,
    #[allow(clippy::type_complexity)]
    debug: Arc<
        dyn Fn(&Box<dyn Any + Send + Sync>, &mut fmt::Formatter<'_>) -> fmt::Result + Send + Sync,
    >,
    #[allow(clippy::type_complexity)]
    as_error: Box<dyn for<'a> Fn(&'a TypeErasedError) -> &'a (dyn StdError) + Send + Sync>,
}

impl fmt::Debug for TypeErasedError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("TypeErasedError:")?;
        (self.debug)(&self.field, f)
    }
}

impl fmt::Display for TypeErasedError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt((self.as_error)(self), f)
    }
}

impl StdError for TypeErasedError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        (self.as_error)(self).source()
    }
}

impl TypeErasedError {
    /// Create a new `TypeErasedError` from `value` of type `T`
    pub fn new<T: StdError + Send + Sync + fmt::Debug + 'static>(value: T) -> Self {
        let debug = |value: &Box<dyn Any + Send + Sync>, f: &mut fmt::Formatter<'_>| {
            fmt::Debug::fmt(value.downcast_ref::<T>().expect("typechecked"), f)
        };
        Self {
            field: Box::new(value),
            debug: Arc::new(debug),
            as_error: Box::new(|value: &TypeErasedError| {
                value.downcast_ref::<T>().expect("typechecked") as _
            }),
        }
    }

    /// Downcast into a `Box<T>`, or return `Self` if it is not a `T`.
    pub fn downcast<T: StdError + fmt::Debug + Send + Sync + 'static>(
        self,
    ) -> Result<Box<T>, Self> {
        let TypeErasedError {
            field,
            debug,
            as_error,
        } = self;
        field.downcast().map_err(|field| Self {
            field,
            debug,
            as_error,
        })
    }

    /// Downcast as a `&T`, or return `None` if it is not a `T`.
    pub fn downcast_ref<T: StdError + fmt::Debug + Send + Sync + 'static>(&self) -> Option<&T> {
        self.field.downcast_ref()
    }

    /// Downcast as a `&mut T`, or return `None` if it is not a `T`.
    pub fn downcast_mut<T: StdError + fmt::Debug + Send + Sync + 'static>(
        &mut self,
    ) -> Option<&mut T> {
        self.field.downcast_mut()
    }

    /// Returns a `TypeErasedError` with a fake/test value with the expectation that it won't be downcast in the test.
    #[cfg(feature = "test-util")]
    pub fn doesnt_matter() -> Self {
        #[derive(Debug)]
        struct FakeError;
        impl fmt::Display for FakeError {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "FakeError")
            }
        }
        impl StdError for FakeError {}
        Self::new(FakeError)
    }
}

#[cfg(test)]
mod tests {
    use super::{TypeErasedBox, TypeErasedError};
    use std::fmt;

    #[derive(Debug, Clone, PartialEq, Eq)]
    struct TestErr {
        inner: &'static str,
    }

    impl TestErr {
        fn new(inner: &'static str) -> Self {
            Self { inner }
        }
    }

    impl fmt::Display for TestErr {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "Error: {}", self.inner)
        }
    }

    impl std::error::Error for TestErr {}

    #[test]
    fn test_typed_erased_errors_can_be_downcast() {
        let test_err = TestErr::new("something failed!");
        let type_erased_test_err = TypeErasedError::new(test_err.clone());
        let actual = type_erased_test_err
            .downcast::<TestErr>()
            .expect("type erased error can be downcast into original type");
        assert_eq!(test_err, *actual);
    }

    #[test]
    fn test_typed_erased_errors_can_be_unwrapped() {
        let test_err = TestErr::new("something failed!");
        let type_erased_test_err = TypeErasedError::new(test_err.clone());
        let actual = *type_erased_test_err
            .downcast::<TestErr>()
            .expect("type erased error can be downcast into original type");
        assert_eq!(test_err, actual);
    }

    #[test]
    fn test_typed_cloneable_boxes() {
        let expected_str = "I can be cloned";
        let cloneable = TypeErasedBox::new_with_clone(expected_str.to_owned());
        // ensure it can be cloned
        let cloned = cloneable.try_clone().unwrap();
        let actual_str = cloned.downcast_ref::<String>().unwrap();
        assert_eq!(expected_str, actual_str);
        // they should point to different addresses
        assert_ne!(format!("{expected_str:p}"), format! {"{actual_str:p}"});
    }
}
