#![cfg(feature = "all")] // TODO: More precise
use core::panic::{RefUnwindSafe, UnwindSafe};

use static_assertions::{assert_impl_all, assert_not_impl_any};

use crate::Foundation::*;
use objc2::mutability::{Immutable, Mutable};
use objc2::rc::Retained;
use objc2::runtime::AnyObject;
use objc2::{declare_class, ClassType, DeclaredClass};

// We expect most Foundation types to be UnwindSafe and RefUnwindSafe,
// since they follow Rust's usual mutability rules (&T = immutable).
//
// A _lot_ of Objective-C code out there would be subtly broken if e.g.
// `NSString` wasn't exception safe!
// As an example: -[NSArray objectAtIndex:] can throw, but it is still
// perfectly valid to access the array after that!
//
// Note that e.g. `&mut NSMutableString` is still not exception safe, but
// that is the entire idea of `UnwindSafe` (that if the object could have
// been mutated, it is not exception safe).
//
// Also note that this is still only a speed bump, not actually part of any
// unsafe contract; we can't really protect against it if something is not
// exception safe, since `UnwindSafe` is a safe trait.
fn assert_unwindsafe<T: UnwindSafe + RefUnwindSafe>() {}

fn assert_auto_traits<T: Send + Sync + UnwindSafe + RefUnwindSafe>() {
    assert_unwindsafe::<T>();
}

macro_rules! helper {
    ($name:ident, $mutability:ty) => {
        declare_class!(
            struct $name;

            unsafe impl ClassType for $name {
                type Super = NSObject;
                type Mutability = $mutability;
                const NAME: &'static str = concat!(stringify!($name), "Test");
            }

            impl DeclaredClass for $name {}
        );
    };
}

helper!(ImmutableObject, Immutable);
helper!(ImmutableSendObject, Immutable);
unsafe impl Send for ImmutableSendObject {}
helper!(ImmutableSyncObject, Immutable);
unsafe impl Sync for ImmutableSyncObject {}
helper!(ImmutableSendSyncObject, Immutable);
unsafe impl Send for ImmutableSendSyncObject {}
unsafe impl Sync for ImmutableSendSyncObject {}

helper!(MutableObject, Mutable);
helper!(MutableSendObject, Mutable);
unsafe impl Send for MutableSendObject {}
helper!(MutableSyncObject, Mutable);
unsafe impl Sync for MutableSyncObject {}
helper!(MutableSendSyncObject, Mutable);
unsafe impl Send for MutableSendSyncObject {}
unsafe impl Sync for MutableSendSyncObject {}

#[test]
fn test_generic_auto_traits() {
    assert_auto_traits::<NSArray<NSString>>();
    assert_auto_traits::<Retained<NSArray<NSString>>>();
    assert_auto_traits::<NSMutableArray<NSString>>();
    assert_auto_traits::<Retained<NSMutableArray<NSString>>>();
    assert_auto_traits::<NSDictionary<NSString, NSString>>();
    assert_auto_traits::<Retained<NSDictionary<NSString, NSString>>>();

    macro_rules! assert_id_like {
        ($wrapper:ident<T>) => {
            assert_not_impl_any!($wrapper<ImmutableObject>: Send, Sync);
            assert_not_impl_any!($wrapper<ImmutableSendObject>: Send, Sync);
            assert_not_impl_any!($wrapper<ImmutableSyncObject>: Send, Sync);
            assert_impl_all!($wrapper<ImmutableSendSyncObject>: Send, Sync);

            assert_not_impl_any!($wrapper<MutableObject>: Send, Sync);
            assert_not_impl_any!($wrapper<MutableSendObject>: Sync);
            assert_impl_all!($wrapper<MutableSendObject>: Send);
            assert_not_impl_any!($wrapper<MutableSyncObject>: Send);
            assert_impl_all!($wrapper<MutableSyncObject>: Sync);
            assert_impl_all!($wrapper<MutableSendSyncObject>: Send, Sync);
        };
    }

    macro_rules! assert_arc_like {
        ($wrapper:ident<T>) => {
            assert_not_impl_any!($wrapper<ImmutableObject>: Send, Sync);
            assert_not_impl_any!($wrapper<ImmutableSendObject>: Send, Sync);
            assert_not_impl_any!($wrapper<ImmutableSyncObject>: Send, Sync);
            assert_impl_all!($wrapper<ImmutableSendSyncObject>: Send, Sync);

            assert_not_impl_any!($wrapper<MutableObject>: Send, Sync);
            assert_not_impl_any!($wrapper<MutableSendObject>: Send, Sync);
            assert_not_impl_any!($wrapper<MutableSyncObject>: Send, Sync);
            assert_impl_all!($wrapper<MutableSendSyncObject>: Send, Sync);
        };
    }

    // TODO
    assert_not_impl_any!(NSArray<AnyObject>: Unpin);
    assert_not_impl_any!(NSMutableArray<AnyObject>: Unpin);
    assert_not_impl_any!(NSDictionary<AnyObject, AnyObject>: Unpin);

    assert_id_like!(NSArray<T>);
    #[allow(dead_code)]
    type NSArrayId<T> = Retained<NSArray<T>>;
    assert_arc_like!(NSArrayId<T>);

    assert_id_like!(NSMutableArray<T>);
    #[allow(dead_code)]
    type NSMutableArrayId<T> = Retained<NSMutableArray<T>>;
    assert_id_like!(NSMutableArrayId<T>);

    #[allow(dead_code)]
    type NSDictionarySingle<T> = NSDictionary<T, T>;
    assert_id_like!(NSDictionarySingle<T>);
    #[allow(dead_code)]
    type NSDictionarySingleId<T> = Retained<NSDictionary<T, T>>;
    assert_arc_like!(NSDictionarySingleId<T>);
}

#[test]
fn send_sync_unwindsafe() {
    assert_auto_traits::<NSAttributedString>();
    assert_auto_traits::<NSComparisonResult>();
    assert_auto_traits::<NSData>();
    assert_auto_traits::<NSDictionary<NSString, NSString>>();
    assert_auto_traits::<NSSet<NSString>>();
    assert_auto_traits::<Retained<NSSet<NSString>>>();
    // TODO: Figure out if Send + Sync is safe?
    // assert_auto_traits::<NSEnumerator2<NSString>>();
    // assert_auto_traits::<NSFastEnumerator2<NSArray<NSString>>>();
    assert_auto_traits::<NSError>();
    assert_auto_traits::<NSException>();
    assert_auto_traits::<CGFloat>();
    assert_auto_traits::<NSPoint>();
    assert_auto_traits::<NSRect>();
    assert_auto_traits::<NSSize>();
    assert_auto_traits::<NSMutableArray<NSString>>();
    assert_auto_traits::<NSMutableAttributedString>();
    assert_auto_traits::<NSMutableData>();
    assert_auto_traits::<NSMutableDictionary<NSString, NSString>>();
    assert_auto_traits::<NSMutableSet<NSString>>();
    assert_auto_traits::<NSMutableString>();
    assert_auto_traits::<NSNumber>();
    // assert_auto_traits::<NSObject>(); // Intentional
    assert_auto_traits::<NSProcessInfo>();
    assert_auto_traits::<NSRange>();
    assert_auto_traits::<NSString>();
    assert_unwindsafe::<MainThreadMarker>(); // Intentional
    assert_auto_traits::<NSThread>();
    assert_auto_traits::<NSUUID>();
    // assert_auto_traits::<NSValue>(); // Intentional
    assert_unwindsafe::<NSZone>(); // Intentional
}
