// Take a look at the license at the top of the repository in the LICENSE file.

use std::{
    any::Any,
    cell::{Ref, RefMut},
    fmt,
};

use crate as glib;
use crate::{subclass::prelude::*, Object};

#[derive(Debug)]
pub enum BorrowError {
    InvalidType,
    AlreadyBorrowed(std::cell::BorrowError),
}

impl std::error::Error for BorrowError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::InvalidType => None,
            Self::AlreadyBorrowed(err) => Some(err),
        }
    }
}

impl fmt::Display for BorrowError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::InvalidType => fmt.write_str("type of the inner value is not as requested"),
            Self::AlreadyBorrowed(_) => fmt.write_str("value is already mutably borrowed"),
        }
    }
}

impl From<std::cell::BorrowError> for BorrowError {
    fn from(err: std::cell::BorrowError) -> Self {
        Self::AlreadyBorrowed(err)
    }
}

#[derive(Debug)]
pub enum BorrowMutError {
    InvalidType,
    AlreadyMutBorrowed(std::cell::BorrowMutError),
}

impl std::error::Error for BorrowMutError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::InvalidType => None,
            Self::AlreadyMutBorrowed(err) => Some(err),
        }
    }
}

impl fmt::Display for BorrowMutError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::InvalidType => fmt.write_str("type of the inner value is not as requested"),
            Self::AlreadyMutBorrowed(_) => fmt.write_str("value is already immutably borrowed"),
        }
    }
}

impl From<std::cell::BorrowMutError> for BorrowMutError {
    fn from(err: std::cell::BorrowMutError) -> Self {
        Self::AlreadyMutBorrowed(err)
    }
}

mod imp {
    use std::{any::Any, cell::RefCell};

    use crate as glib;
    use crate::subclass::prelude::*;

    #[derive(Debug)]
    pub struct BoxedAnyObject {
        pub value: RefCell<Box<dyn Any>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for BoxedAnyObject {
        const NAME: &'static str = "BoxedAnyObject";
        const ALLOW_NAME_CONFLICT: bool = true;
        type Type = super::BoxedAnyObject;
    }
    impl Default for BoxedAnyObject {
        fn default() -> Self {
            Self {
                value: RefCell::new(Box::new(None::<usize>)),
            }
        }
    }
    impl ObjectImpl for BoxedAnyObject {}
}

glib::wrapper! {
    // rustdoc-stripper-ignore-next
    /// This is a subclass of `glib::object::Object` capable of storing any Rust type.
    /// It let's you insert a Rust type anywhere a `glib::object::Object` is needed.
    /// The inserted value can then be borrowed as a Rust type, by using the various
    /// provided methods.
    ///
    /// # Examples
    /// ```
    /// use glib::prelude::*;
    /// use glib::BoxedAnyObject;
    /// use std::cell::Ref;
    ///
    /// struct Author {
    ///     name: String,
    ///     subscribers: usize
    /// }
    /// // BoxedAnyObject can contain any custom type
    /// let boxed = BoxedAnyObject::new(Author {
    ///     name: String::from("GLibAuthor"),
    ///     subscribers: 1000
    /// });
    ///
    /// // The value can be retrieved with `borrow`
    /// let author: Ref<Author> = boxed.borrow();
    /// ```
    ///
    /// ```ignore
    /// use gio::ListStore;
    ///
    /// // The boxed data can be stored as a `glib::object::Object`
    /// let list = ListStore::new::<BoxedAnyObject>();
    /// list.append(&boxed);
    /// ```
    pub struct BoxedAnyObject(ObjectSubclass<imp::BoxedAnyObject>);
}

impl BoxedAnyObject {
    // rustdoc-stripper-ignore-next
    /// Creates a new `BoxedAnyObject` containing `value`
    pub fn new<T: 'static>(value: T) -> Self {
        let obj: Self = Object::new();
        obj.replace(value);
        obj
    }

    // rustdoc-stripper-ignore-next
    /// Replaces the wrapped value with a new one, returning the old value, without deinitializing either one.
    /// The returned value is inside a `Box` and must be manually downcasted if needed.
    #[track_caller]
    pub fn replace<T: 'static>(&self, t: T) -> Box<dyn Any> {
        self.imp().value.replace(Box::new(t) as Box<dyn Any>)
    }

    // rustdoc-stripper-ignore-next
    /// Immutably borrows the wrapped value, returning an error if the value is currently mutably
    /// borrowed or if it's not of type `T`.
    ///
    /// The borrow lasts until the returned `Ref` exits scope. Multiple immutable borrows can be
    /// taken out at the same time.
    ///
    /// This is the non-panicking variant of [`borrow`](#method.borrow).
    pub fn try_borrow<T: 'static>(&self) -> Result<Ref<'_, T>, BorrowError> {
        // The required function is only available on nightly:
        // https://doc.rust-lang.org/std/cell/struct.Ref.html#method.filter_map.
        // As a workaround, I check if everything is safe, then I unwrap

        let borrowed = self.imp().value.try_borrow()?;
        borrowed
            .as_ref()
            .downcast_ref::<T>()
            .ok_or(BorrowError::InvalidType)?;
        Ok(self.borrow()) // Now this won't panic
    }

    // rustdoc-stripper-ignore-next
    /// Mutably borrows the wrapped value, returning an error if the value is currently borrowed.
    /// or if it's not of type `T`.
    ///
    /// The borrow lasts until the returned `RefMut` or all `RefMut`s derived
    /// from it exit scope. The value cannot be borrowed while this borrow is
    /// active.
    ///
    /// This is the non-panicking variant of [`borrow_mut`](#method.borrow_mut).
    pub fn try_borrow_mut<T: 'static>(&mut self) -> Result<RefMut<'_, T>, BorrowMutError> {
        // The required function is only available on nightly:
        // https://doc.rust-lang.org/std/cell/struct.Ref.html#method.filter_map
        // As a workaround, I check if everything is safe, then I unwrap.

        let mut borrowed_mut = self.imp().value.try_borrow_mut()?;
        borrowed_mut
            .as_mut()
            .downcast_mut::<T>()
            .ok_or(BorrowMutError::InvalidType)?;
        drop(borrowed_mut);
        Ok(self.borrow_mut()) // Now this won't panic
    }

    // rustdoc-stripper-ignore-next
    /// Immutably borrows the wrapped value.
    ///
    /// The borrow lasts until the returned `Ref` exits scope. Multiple
    /// immutable borrows can be taken out at the same time.
    ///
    /// # Panics
    ///
    /// Panics if the value is currently mutably borrowed or if it's not of type `T`.
    ///
    /// For a non-panicking variant, use
    /// [`try_borrow`](#method.try_borrow).
    #[track_caller]
    pub fn borrow<T: 'static>(&self) -> Ref<'_, T> {
        Ref::map(self.imp().value.borrow(), |value| {
            value
                .as_ref()
                .downcast_ref::<T>()
                .expect("can't downcast value to requested type")
        })
    }

    // rustdoc-stripper-ignore-next
    /// Mutably borrows the wrapped value.
    ///
    /// The borrow lasts until the returned `RefMut` or all `RefMut`s derived
    /// from it exit scope. The value cannot be borrowed while this borrow is
    /// active.
    ///
    /// # Panics
    ///
    /// Panics if the value is currently borrowed or if it's not of type `T`.
    ///
    /// For a non-panicking variant, use
    /// [`try_borrow_mut`](#method.try_borrow_mut).
    #[track_caller]
    pub fn borrow_mut<T: 'static>(&self) -> RefMut<'_, T> {
        RefMut::map(self.imp().value.borrow_mut(), |value| {
            value
                .as_mut()
                .downcast_mut::<T>()
                .expect("can't downcast value to requested type")
        })
    }
}
