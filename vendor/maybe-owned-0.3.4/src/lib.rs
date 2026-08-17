//! This crate only provides the `MaybeOwned` and `MaybeOwnedMut` enums
//!
//! Take a look at their documentation for more information.
//!
#![warn(missing_docs)]
#[cfg(feature = "serde")]
extern crate serde;

#[cfg(feature = "serde")]
mod serde_impls;

mod transitive_impl;

use std::borrow::{Borrow, BorrowMut, Cow};
use std::cmp::Ordering;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::ops::{Deref, DerefMut};
use std::str::FromStr;

/// This type provides a way to store data to which you either have a
/// reference to or which you do own.
///
/// It provides `From<T>`, `From<&'a T>` implementations and, in difference
/// to `Cow` does _not_ require `ToOwned` to be implemented which makes it
/// compatible with non cloneable data, as a draw back of this it does not
/// know about `ToOwned`. As a consequence of it can't know that `&str` should
/// be the borrowed version of `String` and not `&String` this is especially
/// bad wrt. `Box` as the borrowed version of `Box<T>` would be `&Box<T>`.
///
/// While this crate has some drawbacks compared to `Cow` is has the benefit,
/// that it works with Types which neither implement `Clone` nor `ToOwned`.
/// Another benefit lies in the ability to write API functions which accept
/// a generic parameter `E: Into<MaybeOwned<'a, T>>` as the API consumer can
/// pass `T`, `&'a T` and `MaybeOwned<'a, T>` as argument, without requiring
/// a explicit `Cow::Owned` or a split into two functions one accepting
/// owed and the other borrowed values.
///
/// # Alternatives
///
/// If you mainly have values implementing `ToOwned` like `&str`/`String`, `Path`/`PathBuf` or
/// `&[T]`/`Vec<T>` using `std::borrow::Cow` might be preferable.
///
/// If you want to be able to treat `&T`, `&mut T`, `Box<T>` and `Arc<T>` the same
/// consider using [`reffers::rbma::RBMA`](https://docs.rs/reffers)
/// (through not all types/platforms are supported because
/// as it relies on the fact that for many pointers the lowest two bits are 0, and stores
/// the discriminant in them, nevertheless this is can only be used with 32bit-aligned data,
/// e.g. using a &u8 _might_ fail). RBMA also allows you to recover a `&mut T` if it was created
/// from `Box<T>`, `&mut T` or a unique `Arc`.
///
///
/// # Examples
///
/// ```
/// # use maybe_owned::MaybeOwned;
/// struct PseudoBigData(u8);
/// fn pseudo_register_fn<'a, E>(_val: E) where E: Into<MaybeOwned<'a, PseudoBigData>> { }
///
/// let data = PseudoBigData(12);
/// let data2 = PseudoBigData(13);
///
/// pseudo_register_fn(&data);
/// pseudo_register_fn(&data);
/// pseudo_register_fn(data2);
/// pseudo_register_fn(MaybeOwned::Owned(PseudoBigData(111)));
/// ```
///
/// ```
/// # use maybe_owned::MaybeOwned;
/// #[repr(C)]
/// struct OpaqueFFI {
///     ref1:  * const u8
///     //we also might want to have PhantomData etc.
/// }
///
/// // does not work as it does not implement `ToOwned`
/// // let _ = Cow::Owned(OpaqueFFI { ref1: 0 as *const u8});
///
/// // ok, MaybeOwned can do this (but can't do &str<->String as tread of)
/// let _ = MaybeOwned::Owned(OpaqueFFI { ref1: 0 as *const u8 });
/// ```
///
/// ```
/// # #[macro_use]
/// # extern crate serde_derive;
/// # extern crate serde_json;
/// # extern crate maybe_owned;
/// # #[cfg(feature = "serde")]
/// # fn main() {
/// # use maybe_owned::MaybeOwned;
/// use std::collections::HashMap;
///
/// #[derive(Serialize, Deserialize)]
/// struct SerializedData<'a> {
///     data: MaybeOwned<'a, HashMap<String, i32>>,
/// }
///
/// let mut map = HashMap::new();
/// map.insert("answer".to_owned(), 42);
///
/// // serializing can use borrowed data to avoid unnecessary copying
/// let bytes = serde_json::to_vec(&SerializedData { data: (&map).into() }).unwrap();
///
/// // deserializing creates owned data
/// let deserialized: SerializedData = serde_json::from_slice(&bytes).unwrap();
/// assert_eq!(deserialized.data["answer"], 42);
/// # }
/// # #[cfg(not(feature = "serde"))] fn main() {}
/// ```
///
/// # Transitive `std::ops` implementations
///
/// There are transitive implementations for most operator in `std::ops`.
///
/// A Op between a `MaybeOwned<L>` and `MaybeOwned<R>` is implemented if:
///
/// - L impl the Op with R
/// - L impl the Op with &R
/// - &L impl the Op with R
/// - &L impl the Op with &R
/// - the `Output` of all aboves implementations is
///   the same type
///
///
/// The `Neg` (`-` prefix) op is implemented for `V` if:
///
/// - `V` impl `Neg`
/// - `&V` impl `Neg`
/// - both have the same `Output`
///
///
/// The `Not` (`!` prefix) op is implemented for `V` if:
///
/// - `V` impl `Not`
/// - `&V` impl `Not`
/// - both have the same `Output`
///
/// Adding implementations for Ops which add a `MaybeOwned` to
/// a non `MaybeOwned` value (like `MaybeOwned<T> + T`) requires
/// far reaching specialization in rust and is therefore not done
/// for now.
#[derive(Debug)]
pub enum MaybeOwned<'a, T: 'a> {
    /// owns T
    Owned(T),
    /// has a reference to T
    Borrowed(&'a T),
}

/// This type is basically the same as `MaybeOwned`,
/// but works with mutable references.
///
/// Note that while you can se `MaybeOwned` as a alternative
/// implementation for a Cow (Copy-On-Write) type this isn't
/// really the case for `MaybeOwnedMut` as changing it will
/// potentially change the source through the given `&mut`
/// reference. For example the transitive add assign (+=)
/// implementation for `MaybeOwned` does (need to) convert
/// the given instance into a owned variant before using
/// `+=` on the contained type. But for `MaybeOwnedMut` it
/// can directly use `+=` on the `&mut` contained in the
/// `Borrowed` variant!
#[derive(Debug)]
pub enum MaybeOwnedMut<'a, T: 'a> {
    /// owns T
    Owned(T),
    /// has a reference to T
    Borrowed(&'a mut T),
}

macro_rules! common_impls {
    ($Name:ident) => {
        impl<T> $Name<'_, T> {
            /// Returns true if the data is owned else false.
            pub fn is_owned(&self) -> bool {
                match self {
                    Self::Owned(_) => true,
                    Self::Borrowed(_) => false,
                }
            }
        }

        impl<T: Clone> $Name<'_, T> {

            /// Return the contained data in it's owned form.
            ///
            /// If it's borrowed this will clone it.
            pub fn into_owned(self) -> T {
                match self {
                    Self::Owned(v) => v,
                    Self::Borrowed(v) => v.clone(),
                }
            }

            /// Internally converts the type into it's owned variant.
            ///
            /// Conversion from a reference to the owned variant is done by cloning.
            ///
            /// *This returns a `&mut T` and as such can be used to "unconditionally"
            ///  get an `&mut T`*. Be aware that while this works with both `MaybeOwned`
            ///  and `MaybeOwnedMut` it also converts it to an owned variant in both
            ///  cases. So while it's the best way to get a `&mut T` for `MaybeOwned`
            ///  for `MaybeOwnedMut` it's preferable to use `as_mut` from `AsMut`.
            ///
            /// ## Example
            ///
            /// ```
            /// use maybe_owned::MaybeOwned;
            ///
            /// #[derive(Clone, Debug, PartialEq, Eq)]
            /// struct PseudoBigData(u8);
            ///
            /// let data = PseudoBigData(12);
            ///
            /// let mut maybe: MaybeOwned<PseudoBigData> = (&data).into();
            /// assert_eq!(false, maybe.is_owned());
            ///
            /// {
            ///     let reference = maybe.make_owned();
            ///     assert_eq!(&mut PseudoBigData(12), reference);
            /// }
            /// assert!(maybe.is_owned());
            /// ```
            pub fn make_owned(&mut self) -> &mut T {
                match self {
                    Self::Owned(v) => v,
                    Self::Borrowed(v) => {
                        *self = Self::Owned(v.clone());
                        match self {
                            Self::Owned(v) => v,
                            Self::Borrowed(..) => unreachable!(),
                        }
                    }
                }
            }
        }

        impl<T> Deref for $Name<'_, T> {
            type Target = T;

            fn deref(&self) -> &T {
                match self {
                    Self::Owned(v) => v,
                    Self::Borrowed(v) => v,
                }
            }
        }

        impl<T> AsRef<T> for $Name<'_, T> {
            fn as_ref(&self) -> &T {
                self
            }
        }

        impl<T> From<T> for $Name<'_, T> {
            fn from(v: T) -> Self {
                Self::Owned(v)
            }
        }

        impl<T> Borrow<T> for $Name<'_, T> {
            fn borrow(&self) -> &T {
                self
            }
        }

        impl<T: Default> Default for $Name<'_, T> {
            fn default() -> Self {
                Self::Owned(T::default())
            }
        }

        impl<'b, A: PartialEq<B>, B> PartialEq<$Name<'b, B>> for $Name<'_, A> {
            #[inline]
            fn eq(&self, other: &$Name<'b, B>) -> bool {
                PartialEq::eq(self.deref(), other.deref())
            }
        }

        impl<'a, T: Eq> Eq for $Name<'a, T> {}

        impl<T: FromStr> FromStr for $Name<'_, T> {
            type Err = T::Err;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                Ok(Self::Owned(T::from_str(s)?))
            }
        }

        // TODO: Specify RHS
        impl<T: PartialOrd> PartialOrd for $Name<'_, T> {
            #[inline]
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                PartialOrd::partial_cmp(self.deref(), other.deref())
            }
        }

        impl<T: Ord> Ord for $Name<'_, T> {
            #[inline]
            fn cmp(&self, other: &Self) -> Ordering {
                Ord::cmp(self.deref(), other.deref())
            }
        }

        impl<T: Hash> Hash for $Name<'_, T> {
            #[inline]
            fn hash<H: Hasher>(&self, state: &mut H) {
                Hash::hash(self.deref(), state)
            }
        }

        impl<'a, T: fmt::Display> fmt::Display for $Name<'a, T> {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                match self {
                    Self::Owned(o) => fmt::Display::fmt(o, f),
                    Self::Borrowed(b) => fmt::Display::fmt(b, f),
                }
            }
        }
    };
}

common_impls!(MaybeOwned);
common_impls!(MaybeOwnedMut);

impl<'a, T> From<&'a T> for MaybeOwned<'a, T> {
    fn from(v: &'a T) -> Self {
        Self::Borrowed(v)
    }
}

impl<'a, T> From<&'a mut T> for MaybeOwnedMut<'a, T> {
    fn from(v: &'a mut T) -> Self {
        Self::Borrowed(v)
    }
}

impl<'a, T: ToOwned<Owned = T>> From<Cow<'a, T>> for MaybeOwned<'a, T> {
    fn from(cow: Cow<'a, T>) -> MaybeOwned<'a, T> {
        match cow {
            Cow::Owned(v) => MaybeOwned::Owned(v),
            Cow::Borrowed(v) => MaybeOwned::Borrowed(v),
        }
    }
}

impl<'a, T: ToOwned<Owned = T>> Into<Cow<'a, T>> for MaybeOwned<'a, T> {
    fn into(self) -> Cow<'a, T> {
        match self {
            MaybeOwned::Owned(v) => Cow::Owned(v),
            MaybeOwned::Borrowed(v) => Cow::Borrowed(v),
        }
    }
}

impl<T: Clone> Clone for MaybeOwned<'_, T> {
    fn clone(&self) -> Self {
        match self {
            Self::Owned(v) => Self::Owned(v.clone()),
            Self::Borrowed(v) => Self::Borrowed(v),
        }
    }
}

impl<T> MaybeOwned<'_, T> {
    /// Returns a `&mut` if possible.
    ///
    /// If the internal representation is borrowed (`&T`) then
    /// this method will return `None`
    pub fn as_mut(&mut self) -> Option<&mut T> {
        match self {
            MaybeOwned::Owned(value) => Some(value),
            MaybeOwned::Borrowed(_) => None
        }
    }
}

impl<T: Clone> MaybeOwned<'_, T> {
    /// Acquires a mutable reference to owned data.
    ///
    /// Clones data if it is not already owned.
    ///
    /// ## Example
    ///
    /// ```
    /// use maybe_owned::MaybeOwned;
    ///
    /// #[derive(Clone, Debug, PartialEq, Eq)]
    /// struct PseudoBigData(u8);
    ///
    /// let data = PseudoBigData(12);
    ///
    /// let mut maybe: MaybeOwned<PseudoBigData> = (&data).into();
    /// assert_eq!(false, maybe.is_owned());
    ///
    /// {
    ///     let reference = maybe.to_mut();
    ///     assert_eq!(&mut PseudoBigData(12), reference);
    /// }
    /// assert!(maybe.is_owned());
    /// ```
    ///
    #[deprecated = "use `make_owned` instead"]
    pub fn to_mut(&mut self) -> &mut T {
        match *self {
            Self::Owned(ref mut v) => v,
            Self::Borrowed(v) => {
                *self = Self::Owned(v.clone());
                match *self {
                    Self::Owned(ref mut v) => v,
                    Self::Borrowed(..) => unreachable!(),
                }
            }
        }
    }
}

impl<T> DerefMut for MaybeOwnedMut<'_, T> {
    fn deref_mut(&mut self) -> &mut T {
        match self {
            Self::Owned(v) => v,
            Self::Borrowed(v) => v,
        }
    }
}

impl<T> AsMut<T> for MaybeOwnedMut<'_, T> {
    fn as_mut(&mut self) -> &mut T {
        match self {
            Self::Owned(v) => v,
            Self::Borrowed(v) => v,
        }
    }
}

impl<T> BorrowMut<T> for MaybeOwnedMut<'_, T> {
    fn borrow_mut(&mut self) -> &mut T {
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    type TestType = Vec<()>;

    fn with_into<'a, I: Into<MaybeOwned<'a, TestType>>>(v: I) -> MaybeOwned<'a, TestType> {
        v.into()
    }

    #[test]
    fn is_owned() {
        let data = TestType::default();
        assert!(MaybeOwned::Owned(data).is_owned());
    }

    #[test]
    fn make_owned() {
        let mut a = MaybeOwned::Borrowed(&12u8);
        assert!(!a.is_owned());
        a.make_owned();
        assert!(a.is_owned());
        assert_eq!(&*a, &12);
    }

    #[test]
    fn into_with_owned() {
        //ty check if it accepts references
        let data = TestType::default();
        assert!(with_into(data).is_owned())
    }
    #[test]
    fn into_with_borrow() {
        //ty check if it accepts references
        let data = TestType::default();
        assert!(!with_into(&data).is_owned());
    }

    #[test]
    fn clone_owned() {
        let maybe = MaybeOwned::<TestType>::default();
        assert!(maybe.clone().is_owned());
    }

    #[test]
    fn clone_borrow() {
        let data = TestType::default();
        let maybe: MaybeOwned<TestType> = (&data).into();
        assert!(!maybe.clone().is_owned());
    }

    #[test]
    fn to_mut() {
        let data = TestType::default();
        let mut maybe: MaybeOwned<TestType> = (&data).into();
        assert!(!maybe.is_owned());
        {
            #[allow(deprecated)]
            let _mut_ref = maybe.to_mut();
        }
        assert!(maybe.is_owned());
    }

    #[test]
    fn into_inner() {
        let data = vec![1u32, 2];
        let maybe: MaybeOwned<Vec<u32>> = (&data).into();
        assert_eq!(data, maybe.into_owned());
    }

    #[test]
    fn has_default() {
        #[derive(Default)]
        struct TestType(u8);
        let _x: MaybeOwned<TestType> = Default::default();
    }

    #[test]
    fn has_clone() {
        #[derive(Clone)]
        struct TestType(u8);
        let _x = TestType(12).clone();
    }

    #[test]
    fn has_deref() {
        let a = MaybeOwned::Owned(vec![1u8]);
        let _ = a.len();

        let a = MaybeOwnedMut::Owned(vec![1u8]);
        let _ = a.len();
    }

    #[test]
    fn has_deref_mut() {
        let mut a = MaybeOwnedMut::Owned(vec![1u8]);
        a[0] = 12u8;
    }

    #[test]
    fn has_partial_eq() {
        #[derive(PartialEq)]
        struct TestType(f32);

        let n = TestType(33.0);
        let a = MaybeOwned::Owned(TestType(42.0));
        let b = MaybeOwned::Borrowed(&n);
        let c = MaybeOwned::Owned(TestType(33.0));

        assert_eq!(a == b, false);
        assert_eq!(b == c, true);
        assert_eq!(c == a, false);
    }

    #[test]
    fn has_eq() {
        #[derive(PartialEq, Eq)]
        struct TestType(i32);

        let n = TestType(33);
        let a = MaybeOwned::Owned(TestType(42));
        let b = MaybeOwned::Borrowed(&n);
        let c = MaybeOwned::Owned(TestType(33));

        assert_eq!(a == b, false);
        assert_eq!(b == c, true);
        assert_eq!(c == a, false);
    }

    #[test]
    fn has_partial_ord() {
        #[derive(PartialEq, PartialOrd)]
        struct TestType(f32);

        let n = TestType(33.0);
        let a = MaybeOwned::Owned(TestType(42.0));
        let b = MaybeOwned::Borrowed(&n);
        let c = MaybeOwned::Owned(TestType(33.0));

        assert_eq!(a > b, true);
        assert_eq!(b > c, false);
        assert_eq!(a < c, false);
    }

    #[test]
    fn has_ord() {
        #[derive(PartialEq, Eq, PartialOrd, Ord)]
        struct TestType(i32);

        let n = TestType(33);
        let a = MaybeOwned::Owned(TestType(42));
        let b = MaybeOwned::Borrowed(&n);
        let c = MaybeOwned::Owned(TestType(33));

        assert_eq!(a > b, true);
        assert_eq!(b > c, false);
        assert_eq!(a < c, false);
    }

    #[test]
    fn has_hash() {
        use std::collections::HashMap;

        let mut map = HashMap::new();
        map.insert(MaybeOwned::Owned(42), 33);

        assert_eq!(map.get(&MaybeOwned::Borrowed(&42)), Some(&33));
    }

    #[test]
    fn has_borrow() {
        let v = MaybeOwned::Owned(42);
        let _ = Borrow::<u8>::borrow(&v);

        let v = MaybeOwnedMut::Owned(42);
        let _ = Borrow::<u8>::borrow(&v);
    }

    #[test]
    fn has_borrow_mut() {
        let mut v = MaybeOwnedMut::Owned(42);
        let _ = BorrowMut::<u8>::borrow_mut(&mut v);
    }

    #[test]
    fn has_as_ref() {
        let v = MaybeOwned::Owned(42);
        let _ = AsRef::<u8>::borrow(&v);

        let v = MaybeOwnedMut::Owned(42);
        let _ = AsRef::<u8>::borrow(&v);
    }

    #[test]
    fn has_as_mut() {
        // uses a as_mut method
        let mut v: MaybeOwned<u8> = (&11).into();
        assert_eq!(v.as_mut(), None);

        let mut v: MaybeOwned<u8> = 12.into();
        assert_eq!(v.as_mut(), Some(&mut 12));

        // uses AsMut
        let mut v = MaybeOwnedMut::Owned(42);
        let _ = AsMut::<u8>::borrow_mut(&mut v);
    }

    #[test]
    fn has_display() {
        let n = 33;
        let a = MaybeOwned::Owned(42);
        let b = MaybeOwned::Borrowed(&n);

        let s = format!("{} {}", a, b);

        assert_eq!(s, "42 33");
    }

    #[test]
    fn from_cow() {
        use std::borrow::Cow;

        fn test<'a, V: Into<MaybeOwned<'a, i32>>>(v: V, n: i32) {
            assert_eq!(*v.into(), n)
        }

        let n = 33;
        test(Cow::Owned(42), 42);
        test(Cow::Borrowed(&n), n);
    }

    #[test]
    fn into_cow() {
        use std::borrow::Cow;

        fn test<'a, V: Into<Cow<'a, i32>>>(v: V, n: i32) {
            assert_eq!(*v.into(), n)
        }

        let n = 33;
        test(MaybeOwned::Owned(42), 42);
        test(MaybeOwned::Borrowed(&n), n);
    }

    #[test]
    fn from_str() {
        let as_string = "12";
        //assumption as_string is convertable to u32
        assert_eq!(12u32, as_string.parse().unwrap());
        assert_eq!(MaybeOwned::Owned(12u32), as_string.parse().unwrap());
    }

    #[test]
    fn as_ref() {
        let data = TestType::default();
        let maybe_owned = MaybeOwned::Borrowed(&data);
        let _ref: &TestType = maybe_owned.as_ref();
        assert_eq!(&data as *const _ as usize, _ref as *const _ as usize);
    }

    #[test]
    fn borrow() {
        use std::borrow::Borrow;

        let data = TestType::default();
        let maybe_owned = MaybeOwned::Borrowed(&data);
        let _ref: &TestType = maybe_owned.borrow();
        assert_eq!(&data as *const _ as usize, _ref as *const _ as usize);
    }

    #[test]
    fn reborrow_mut() {
        let value = vec![0u32];
        let mut value = MaybeOwnedMut::Owned(value);
        let mut reborrow = MaybeOwnedMut::Borrowed(value.deref_mut());
        reborrow.push(1);
        assert_eq!(&[0, 1], &value[..]);
    }
}
