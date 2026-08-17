#![allow(dead_code, unused_imports)]
use core::panic::{RefUnwindSafe, UnwindSafe};

use static_assertions::assert_not_impl_any;

use objc2::define_class;
use objc2::rc::Retained;
use objc2::runtime::{AnyObject, NSObject};

// We expect most Foundation types to be UnwindSafe and RefUnwindSafe,
// since they follow Rust's usual mutability rules (&T = immutable).
//
// A _lot_ of Objective-C code out there would be subtly broken if e.g.
// `NSString` wasn't exception safe! As an example: `-[NSArray objectAtIndex:]`
// can throw, but it is still perfectly valid to access the array after that!
//
// I'm pretty sure that even mutable classes like `NSMutableString` is still
// exception safe, since preconditions are checked before the mutation is
// executed, and more "internal" errors like Out-Of-Memory either crash the
// application, or return / throw an error value.
//
// Also note that this is still only a speed bump, not actually part of any
// unsafe contract; we can't really protect against it if something is not
// exception safe, since `UnwindSafe` is a safe trait.
fn assert_unwindsafe<T: UnwindSafe + RefUnwindSafe>() {}

fn assert_auto_traits<T: Send + Sync + UnwindSafe + RefUnwindSafe>() {
    assert_unwindsafe::<T>();
}

define_class!(
    #[unsafe(super(NSObject))]
    struct SendSyncObject;
);

#[test]
fn test_generic_auto_traits() {
    // assert_unwindsafe::<NSArray<NSProcessInfo>>();
    // assert_unwindsafe::<Retained<NSArray<NSProcessInfo>>>();
    // assert_unwindsafe::<NSMutableArray<NSProcessInfo>>();
    // assert_unwindsafe::<Retained<NSMutableArray<NSProcessInfo>>>();
    // assert_unwindsafe::<NSDictionary<NSProcessInfo, NSProcessInfo>>();
    // assert_unwindsafe::<Retained<NSDictionary<NSProcessInfo, NSProcessInfo>>>();

    // TODO: Unpin?
    #[cfg(feature = "NSArray")]
    assert_not_impl_any!(crate::NSArray<AnyObject>: Unpin);
    #[cfg(feature = "NSArray")]
    assert_not_impl_any!(crate::NSMutableArray<AnyObject>: Unpin);
    #[cfg(feature = "NSDictionary")]
    assert_not_impl_any!(crate::NSDictionary<AnyObject, AnyObject>: Unpin);

    // Collections are not Send + Sync, since they are interior mutable, i.e.
    // mutable from `&self`.
    #[cfg(feature = "NSArray")]
    assert_not_impl_any!(crate::NSArray<SendSyncObject>: Send, Sync);
    #[cfg(feature = "NSArray")]
    assert_not_impl_any!(crate::NSMutableArray<SendSyncObject>: Send, Sync);
    #[cfg(feature = "NSDictionary")]
    assert_not_impl_any!(crate::NSDictionary<SendSyncObject, SendSyncObject>: Send, Sync);

    // TODO: Make these UnwindSafe?
    #[cfg(feature = "NSProcessInfo")]
    {
        use crate::NSProcessInfo;

        #[cfg(feature = "NSDictionary")]
        assert_not_impl_any!(crate::NSDictionary<NSProcessInfo, NSProcessInfo>: UnwindSafe, RefUnwindSafe);
        #[cfg(feature = "NSSet")]
        assert_not_impl_any!(crate::NSSet<NSProcessInfo>: UnwindSafe, RefUnwindSafe);
        #[cfg(feature = "NSSet")]
        assert_not_impl_any!(Retained<crate::NSSet<NSProcessInfo>>: UnwindSafe, RefUnwindSafe);
        #[cfg(feature = "NSArray")]
        assert_not_impl_any!(crate::NSMutableArray<NSProcessInfo>: UnwindSafe, RefUnwindSafe);
        #[cfg(feature = "NSDictionary")]
        assert_not_impl_any!(crate::NSMutableDictionary<NSProcessInfo, NSProcessInfo>: UnwindSafe, RefUnwindSafe);
        #[cfg(feature = "NSSet")]
        assert_not_impl_any!(crate::NSMutableSet<NSProcessInfo>: UnwindSafe, RefUnwindSafe);
    }
}

#[test]
fn send_sync_unwindsafe() {
    #[cfg(feature = "NSAttributedString")]
    assert_unwindsafe::<crate::NSAttributedString>();
    #[cfg(feature = "NSObjCRuntime")]
    assert_auto_traits::<crate::NSComparisonResult>();
    #[cfg(feature = "NSData")]
    assert_unwindsafe::<crate::NSData>();
    // TODO: Figure out if Send + Sync is safe?
    // assert_auto_traits::<crate::NSEnumerator2<crate::NSProcessInfo>>();
    // assert_auto_traits::<crate::NSFastEnumerator2<crate::NSArray<crate::NSProcessInfo>>>();
    #[cfg(feature = "NSError")]
    assert_auto_traits::<crate::NSError>();
    #[cfg(feature = "NSException")]
    assert_auto_traits::<crate::NSException>();
    #[cfg(all(feature = "NSGeometry", feature = "objc2-core-foundation"))]
    assert_auto_traits::<crate::NSPoint>();
    #[cfg(all(feature = "NSGeometry", feature = "objc2-core-foundation"))]
    assert_auto_traits::<crate::NSRect>();
    #[cfg(all(feature = "NSGeometry", feature = "objc2-core-foundation"))]
    assert_auto_traits::<crate::NSSize>();
    #[cfg(feature = "NSAttributedString")]
    assert_unwindsafe::<crate::NSMutableAttributedString>();
    #[cfg(feature = "NSData")]
    assert_unwindsafe::<crate::NSMutableData>();
    #[cfg(feature = "NSString")]
    assert_unwindsafe::<crate::NSMutableString>();
    #[cfg(feature = "NSValue")]
    assert_auto_traits::<crate::NSNumber>();
    // assert_auto_traits::<crate::NSObject>(); // Intentional
    #[cfg(feature = "NSProcessInfo")]
    assert_auto_traits::<crate::NSProcessInfo>();
    #[cfg(feature = "NSRange")]
    assert_auto_traits::<crate::NSRange>();
    #[cfg(feature = "NSString")]
    assert_unwindsafe::<crate::NSString>();
    #[cfg(feature = "NSThread")]
    assert_auto_traits::<crate::NSThread>();
    #[cfg(feature = "NSUUID")]
    assert_auto_traits::<crate::NSUUID>();
    // assert_auto_traits::<crate::NSValue>(); // Intentional
    #[cfg(feature = "NSZone")]
    assert_unwindsafe::<crate::NSZone>(); // Intentional
}
