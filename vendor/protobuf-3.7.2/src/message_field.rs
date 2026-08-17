use std::hash::Hash;
use std::ops::Deref;
use std::option;

use crate::Message;

/// Wrapper around `Option<Box<T>>`, convenient newtype.
///
/// # Examples
///
/// ```no_run
/// # use protobuf::MessageField;
/// # use std::ops::Add;
/// # struct Address {
/// # }
/// # struct Customer {
/// #     address: MessageField<Address>,
/// # }
/// # impl Customer {
/// #     fn new() -> Customer { unimplemented!() }
/// # }
/// #
/// #
/// # fn make_address() -> Address { unimplemented!() }
/// let mut customer = Customer::new();
///
/// // field of type `SingularPtrField` can be initialized like this
/// customer.address = MessageField::some(make_address());
/// // or using `Option` and `Into`
/// customer.address = Some(make_address()).into();
/// ```
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct MessageField<T>(pub Option<Box<T>>);

impl<T> MessageField<T> {
    /// Construct `SingularPtrField` from given object.
    #[inline]
    pub fn some(value: T) -> MessageField<T> {
        MessageField(Some(Box::new(value)))
    }

    /// Construct an empty `SingularPtrField`.
    #[inline]
    pub const fn none() -> MessageField<T> {
        MessageField(None)
    }

    /// Construct `SingularPtrField` from optional.
    #[inline]
    pub fn from_option(option: Option<T>) -> MessageField<T> {
        match option {
            Some(x) => MessageField::some(x),
            None => MessageField::none(),
        }
    }

    /// True iff this object contains data.
    #[inline]
    pub fn is_some(&self) -> bool {
        self.0.is_some()
    }

    /// True iff this object contains no data.
    #[inline]
    pub fn is_none(&self) -> bool {
        self.0.is_none()
    }

    /// Convert into `Option<T>`.
    #[inline]
    pub fn into_option(self) -> Option<T> {
        self.0.map(|v| *v)
    }

    /// View data as reference option.
    #[inline]
    pub fn as_ref(&self) -> Option<&T> {
        self.0.as_ref().map(|v| &**v)
    }

    /// View data as mutable reference option.
    #[inline]
    pub fn as_mut(&mut self) -> Option<&mut T> {
        self.0.as_mut().map(|v| &mut **v)
    }

    /// Take the data.
    /// Panics if empty
    #[inline]
    pub fn unwrap(self) -> T {
        *self.0.unwrap()
    }

    /// Take the data or return supplied default element if empty.
    #[inline]
    pub fn unwrap_or(self, def: T) -> T {
        self.0.map(|v| *v).unwrap_or(def)
    }

    /// Take the data or return supplied default element if empty.
    #[inline]
    pub fn unwrap_or_else<F>(self, f: F) -> T
    where
        F: FnOnce() -> T,
    {
        self.0.map(|v| *v).unwrap_or_else(f)
    }

    /// Apply given function to contained data to construct another `SingularPtrField`.
    /// Returns empty `SingularPtrField` if this object is empty.
    #[inline]
    pub fn map<U, F>(self, f: F) -> MessageField<U>
    where
        F: FnOnce(T) -> U,
    {
        MessageField::from_option(self.into_option().map(f))
    }

    /// View data as iterator.
    #[inline]
    pub fn iter(&self) -> option::IntoIter<&T> {
        self.as_ref().into_iter()
    }

    /// View data as mutable iterator.
    #[inline]
    pub fn mut_iter(&mut self) -> option::IntoIter<&mut T> {
        self.as_mut().into_iter()
    }

    /// Take data as option, leaving this object empty.
    #[inline]
    pub fn take(&mut self) -> Option<T> {
        self.0.take().map(|v| *v)
    }

    /// Clear this object, but do not call destructor of underlying data.
    #[inline]
    pub fn clear(&mut self) {
        self.0 = None;
    }
}

impl<T: Default> MessageField<T> {
    /// Get contained data, consume self. Return default value for type if this is empty.
    #[inline]
    pub fn unwrap_or_default(self) -> T {
        *self.0.unwrap_or_default()
    }
}

impl<M: Message> MessageField<M> {
    /// Get a reference to contained value or a default instance.
    pub fn get_or_default(&self) -> &M {
        self.as_ref().unwrap_or_else(|| M::default_instance())
    }

    /// Get a mutable reference to contained value, initialize if not initialized yet.
    pub fn mut_or_insert_default(&mut self) -> &mut M {
        if self.is_none() {
            *self = MessageField::some(Default::default());
        }
        self.as_mut().unwrap()
    }
}

/// Get a reference to contained value or a default instance if the field is not initialized.
impl<M: Message> Deref for MessageField<M> {
    type Target = M;

    fn deref(&self) -> &Self::Target {
        self.get_or_default()
    }
}

impl<T> Default for MessageField<T> {
    #[inline]
    fn default() -> MessageField<T> {
        MessageField::none()
    }
}

/// We don't have `From<Option<Box<T>>> for MessageField<T>` because
/// it would make type inference worse.
impl<T> From<Option<T>> for MessageField<T> {
    fn from(o: Option<T>) -> Self {
        MessageField::from_option(o)
    }
}

impl<'a, T> IntoIterator for &'a MessageField<T> {
    type Item = &'a T;
    type IntoIter = option::IntoIter<&'a T>;

    fn into_iter(self) -> option::IntoIter<&'a T> {
        self.iter()
    }
}
