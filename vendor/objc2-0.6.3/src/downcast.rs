use crate::ClassType;

/// Classes that can be safely downcasted to.
///
/// [`DowncastTarget`] is an unsafe marker trait that can be implemented on
/// types that also implement [`ClassType`].
///
/// Ideally, every type that implements `ClassType` would also be a valid
/// downcast target, however this would be unsound when used with generics,
/// because we can only trivially decide whether the "base container" is an
/// instance of some class type, but anything related to the generic arguments
/// is unknown.
///
/// This trait is implemented automatically by the [`extern_class!`] and
/// [`define_class!`] macros.
///
/// [`extern_class!`]: crate::extern_class
/// [`define_class!`]: crate::define_class
///
///
/// # Safety
///
/// The type must not have any generic arguments other than [`AnyObject`].
///
/// [`AnyObject`]: crate::runtime::AnyObject
///
///
/// # Examples
///
/// Implementing [`DowncastTarget`] for `NSString`:
///
/// ```ignore
/// // SAFETY: NSString does not have any generic parameters.
/// unsafe impl DowncastTarget for NSString {}
/// ```
///
/// However, implementing it for `NSArray` can only be done when the object
/// type is `AnyObject`.
///
/// ```ignore
/// // SAFETY: NSArray does not have any generic parameters set (the generic
/// // defaults to `AnyObject`).
/// unsafe impl DowncastTarget for NSArray {}
///
/// // This would not be valid, since downcasting can only trivially determine
/// // whether the base class (in this case `NSArray`) matches the receiver
/// // class type.
/// // unsafe impl<T: Message> DowncastTarget for NSArray<T> {}
/// ```
pub unsafe trait DowncastTarget: ClassType + 'static {}
