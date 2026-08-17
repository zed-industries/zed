#![allow(dead_code, unused_imports)]
use core::marker::PhantomData;
use core::panic::{RefUnwindSafe, UnwindSafe};

use objc2::rc::Retained;
use objc2::runtime::{AnyObject, NSObject};
use objc2::{__inner_extern_class, mutability, ClassType, Message};

#[derive(PartialEq, Eq, Hash, PartialOrd, Ord, Clone, Copy, Debug)]
#[repr(transparent)]
struct UnsafeIgnoreAutoTraits<T: ?Sized>(T);
// SAFETY: Moved to the name of the struct
unsafe impl<T: ?Sized> Send for UnsafeIgnoreAutoTraits<T> {}
// SAFETY: Moved to the name of the struct
unsafe impl<T: ?Sized> Sync for UnsafeIgnoreAutoTraits<T> {}
impl<T: ?Sized> RefUnwindSafe for UnsafeIgnoreAutoTraits<T> {}
impl<T: ?Sized> UnwindSafe for UnsafeIgnoreAutoTraits<T> {}
// TODO
// impl<T: ?Sized> Unpin for UnsafeIgnoreAutoTraits<T> {}

__inner_extern_class!(
    #[derive(PartialEq, Eq, Hash)]
    #[cfg(feature = "NSArray")]
    pub struct NSArray<ObjectType: ?Sized = AnyObject> {
        // SAFETY: Auto traits specified below.
        __superclass: UnsafeIgnoreAutoTraits<NSObject>,
        /// `NSArray` and `NSMutableArray` have `Retained`-like storage.
        ///
        /// This fairly trivially true for `NSMutableArray<T: IsMutable>` and
        /// `NSArray<T: IsIdCloneable>`, but let's explore the two other cases
        /// a bit:
        ///
        ///
        /// # `NSMutableArray<T: IsIdCloneable>`
        ///
        /// Must have `Arc`-like auto traits, since `NSMutableArray<T>` is
        /// `NSMutableCopying` in that case (which is a shallow clone).
        ///
        /// E.g. if we had `NSMutableArray<T: Send>: Send`, `T` would be
        /// accessible from multiple threads, therefore we require `T: Sync`:
        /// ```ignore
        /// std::thread::scope(|s| {
        ///     let obj: Retained<NSMutableArray<T>>;
        ///     let obj2 = obj.mutableCopy();
        ///     s.spawn(move || {
        ///         let obj = obj;
        ///     });
        ///     s.spawn(move || {
        ///         let obj2 = obj2;
        ///     });
        /// })
        /// ```
        ///
        /// Similarly, if we had `NSMutableArray<T: Sync>: Sync`, `T` would be
        /// dropable from other threads, therefore we require `T: Send`:
        /// ```ignore
        /// let obj: Retained<NSMutableArray<T>>;
        /// std::thread::spawn(|| {
        ///     let obj2 = obj.mutableCopy();
        ///
        ///     // Do some long-lived work, the original `obj` is dropped in
        ///     // the original thread in the meantime.
        ///
        ///     // And now, the destructor is run on the wrong thread.
        ///     drop(obj2);
        /// });
        /// // (note: example does not work due to lifetime issues, but the
        /// // idea still stands).
        /// ```
        ///
        ///
        /// # `NSArray<T: IsMutable>`
        ///
        /// This could safely be `Arc`-like, but for consistency with
        /// `NSMutableArray` (esp. since `get_mut` is available on `NSArray`
        /// and not `NSMutableArray`), we make it `Box`-like.
        ///
        /// This may seem wrong since arrays are unconditionally immutable,
        /// e.g. `ClassType::Mutability` is always
        /// `ImmutableWithMutableSubclass`.
        ///
        /// But since we already require `T: IsCloneable` on `NSCopying`, and
        /// already prevent other forms of `&NSArray<T> -> Retained<NSArray<T>>`,
        /// this is actually fine, since `Retained<NSArray<T>>: Send | Sync`
        /// requires `NSArray<T>: Send + Sync` (and hence `T: Send + Sync`).
        __inner: PhantomData<Retained<ObjectType>>,
    }

    #[cfg(feature = "NSArray")]
    unsafe impl<ObjectType: ?Sized + Message> ClassType for NSArray<ObjectType> {
        type Super = NSObject;
        type Mutability = mutability::ImmutableWithMutableSubclass<NSMutableArray<ObjectType>>;

        fn as_super(&self) -> &Self::Super {
            &self.__superclass.0
        }

        fn as_super_mut(&mut self) -> &mut Self::Super {
            &mut self.__superclass.0
        }
    }
);

__inner_extern_class!(
    #[derive(PartialEq, Eq, Hash)]
    #[cfg(feature = "NSArray")]
    pub struct NSMutableArray<ObjectType: ?Sized = AnyObject> {
        // Inherit auto traits from superclass.
        __superclass: NSArray<ObjectType>,
    }

    #[cfg(feature = "NSArray")]
    unsafe impl<ObjectType: ?Sized + Message> ClassType for NSMutableArray<ObjectType> {
        #[inherits(NSObject)]
        type Super = NSArray<ObjectType>;
        type Mutability = mutability::MutableWithImmutableSuperclass<NSArray<ObjectType>>;

        fn as_super(&self) -> &Self::Super {
            &self.__superclass
        }

        fn as_super_mut(&mut self) -> &mut Self::Super {
            &mut self.__superclass
        }
    }
);

__inner_extern_class!(
    #[derive(PartialEq, Eq, Hash)]
    #[cfg(feature = "NSDictionary")]
    pub struct NSDictionary<KeyType: ?Sized = AnyObject, ObjectType: ?Sized = AnyObject> {
        // SAFETY: Auto traits specified below.
        __superclass: UnsafeIgnoreAutoTraits<NSObject>,
        // Same as if the dictionary was implemented with:
        // `(NSArray<KeyType>, NSArray<ObjectType>)`
        __inner: PhantomData<(Retained<KeyType>, Retained<ObjectType>)>,
    }

    #[cfg(feature = "NSDictionary")]
    unsafe impl<KeyType: ?Sized + Message, ObjectType: ?Sized + Message> ClassType
        for NSDictionary<KeyType, ObjectType>
    {
        type Super = NSObject;
        type Mutability =
            mutability::ImmutableWithMutableSubclass<NSMutableDictionary<KeyType, ObjectType>>;

        fn as_super(&self) -> &Self::Super {
            &self.__superclass.0
        }

        fn as_super_mut(&mut self) -> &mut Self::Super {
            &mut self.__superclass.0
        }
    }
);

__inner_extern_class!(
    #[derive(PartialEq, Eq, Hash)]
    #[cfg(feature = "NSDictionary")]
    pub struct NSMutableDictionary<KeyType: ?Sized = AnyObject, ObjectType: ?Sized = AnyObject> {
        // Inherit auto traits from superclass.
        __superclass: NSDictionary<KeyType, ObjectType>,
    }

    #[cfg(feature = "NSDictionary")]
    unsafe impl<KeyType: ?Sized + Message, ObjectType: ?Sized + Message> ClassType
        for NSMutableDictionary<KeyType, ObjectType>
    {
        #[inherits(NSObject)]
        type Super = NSDictionary<KeyType, ObjectType>;
        type Mutability =
            mutability::MutableWithImmutableSuperclass<NSDictionary<KeyType, ObjectType>>;

        fn as_super(&self) -> &Self::Super {
            &self.__superclass
        }

        fn as_super_mut(&mut self) -> &mut Self::Super {
            &mut self.__superclass
        }
    }
);

__inner_extern_class!(
    #[derive(PartialEq, Eq, Hash)]
    #[cfg(feature = "NSSet")]
    pub struct NSSet<ObjectType: ?Sized = AnyObject> {
        // SAFETY: Auto traits specified below.
        __superclass: UnsafeIgnoreAutoTraits<NSObject>,
        // Same as if the set was implemented as `NSArray<ObjectType>`.
        __inner: PhantomData<Retained<ObjectType>>,
    }

    #[cfg(feature = "NSSet")]
    unsafe impl<ObjectType: ?Sized + Message> ClassType for NSSet<ObjectType> {
        type Super = NSObject;
        type Mutability = mutability::ImmutableWithMutableSubclass<NSMutableSet<ObjectType>>;

        fn as_super(&self) -> &Self::Super {
            &self.__superclass.0
        }

        fn as_super_mut(&mut self) -> &mut Self::Super {
            &mut self.__superclass.0
        }
    }
);

__inner_extern_class!(
    #[derive(PartialEq, Eq, Hash)]
    #[cfg(feature = "NSSet")]
    pub struct NSMutableSet<ObjectType: ?Sized = AnyObject> {
        // Inherit auto traits from superclass.
        __superclass: NSSet<ObjectType>,
    }

    #[cfg(feature = "NSSet")]
    unsafe impl<ObjectType: ?Sized + Message> ClassType for NSMutableSet<ObjectType> {
        #[inherits(NSObject)]
        type Super = NSSet<ObjectType>;
        type Mutability = mutability::MutableWithImmutableSuperclass<NSSet<ObjectType>>;

        fn as_super(&self) -> &Self::Super {
            &self.__superclass
        }

        fn as_super_mut(&mut self) -> &mut Self::Super {
            &mut self.__superclass
        }
    }
);

__inner_extern_class!(
    #[derive(PartialEq, Eq, Hash)]
    #[cfg(feature = "NSSet")]
    pub struct NSCountedSet<ObjectType: ?Sized = AnyObject> {
        // Inherit auto traits from superclass.
        __superclass: NSMutableSet<ObjectType>,
    }

    #[cfg(feature = "NSSet")]
    unsafe impl<ObjectType: ?Sized + Message> ClassType for NSCountedSet<ObjectType> {
        #[inherits(NSSet<ObjectType>, NSObject)]
        type Super = NSMutableSet<ObjectType>;
        type Mutability = mutability::Mutable;

        fn as_super(&self) -> &Self::Super {
            &self.__superclass
        }

        fn as_super_mut(&mut self) -> &mut Self::Super {
            &mut self.__superclass
        }
    }
);

__inner_extern_class!(
    #[derive(PartialEq, Eq, Hash)]
    #[cfg(feature = "NSOrderedSet")]
    pub struct NSOrderedSet<ObjectType: ?Sized = AnyObject> {
        // SAFETY: Auto traits specified below.
        __superclass: UnsafeIgnoreAutoTraits<NSObject>,
        // Same as if the set was implemented with `NSArray<ObjectType>`.
        __inner: PhantomData<Retained<ObjectType>>,
    }

    #[cfg(feature = "NSOrderedSet")]
    unsafe impl<ObjectType: ?Sized + Message> ClassType for NSOrderedSet<ObjectType> {
        type Super = NSObject;
        type Mutability = mutability::ImmutableWithMutableSubclass<NSMutableOrderedSet<ObjectType>>;

        fn as_super(&self) -> &Self::Super {
            &self.__superclass.0
        }

        fn as_super_mut(&mut self) -> &mut Self::Super {
            &mut self.__superclass.0
        }
    }
);

__inner_extern_class!(
    #[derive(PartialEq, Eq, Hash)]
    #[cfg(feature = "NSOrderedSet")]
    pub struct NSMutableOrderedSet<ObjectType: ?Sized = AnyObject> {
        // Inherit auto traits from superclass.
        __superclass: NSOrderedSet<ObjectType>,
    }

    #[cfg(feature = "NSOrderedSet")]
    unsafe impl<ObjectType: ?Sized + Message> ClassType for NSMutableOrderedSet<ObjectType> {
        #[inherits(NSObject)]
        type Super = NSOrderedSet<ObjectType>;
        type Mutability = mutability::MutableWithImmutableSuperclass<NSOrderedSet<ObjectType>>;

        fn as_super(&self) -> &Self::Super {
            &self.__superclass
        }

        fn as_super_mut(&mut self) -> &mut Self::Super {
            &mut self.__superclass
        }
    }
);

__inner_extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "NSEnumerator")]
    pub struct NSEnumerator<ObjectType: ?Sized = AnyObject> {
        // SAFETY: Auto traits specified below.
        __superclass: UnsafeIgnoreAutoTraits<NSObject>,
        // Enumerators are basically the same as if we were storing
        // `NSMutableArray<ObjectType>`, and removed an element from that on
        // each iteration.
        //
        // This is only true because everything in this file has that type of
        // ownership; if something didn't, and instead had e.g. `Arc<T>`-like
        // ownership, we'd have to modify the ownership of enumerators to
        // match with that (if we wanted to safely use enumerators returned by
        // that type).
        __inner: PhantomData<Retained<ObjectType>>,
    }

    #[cfg(feature = "NSEnumerator")]
    unsafe impl<ObjectType: ?Sized + Message> ClassType for NSEnumerator<ObjectType> {
        type Super = NSObject;
        type Mutability = mutability::Mutable;

        fn as_super(&self) -> &Self::Super {
            &self.__superclass.0
        }

        fn as_super_mut(&mut self) -> &mut Self::Super {
            &mut self.__superclass.0
        }
    }
);
