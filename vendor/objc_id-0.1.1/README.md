Rust smart pointers for Objective-C reference counting.

To ensure that Objective-C objects are retained and released
at the proper times, we can use the Id struct.

To enforce aliasing rules, an `Id` can be either owned or shared; if it is
owned, meaning the `Id` is the only reference to the object, it can be mutably
dereferenced. An owned `Id` can be downgraded to a ShareId
which can be cloned to allow multiple references.

Weak references may be created using the WeakId struct.

``` rust
use objc::runtime::{Class, Object};
use objc_id::{Id, WeakId};

let cls = Class::get("NSObject").unwrap();
let obj: Id<Object> = unsafe {
    Id::from_retained_ptr(msg_send![cls, new])
};
// obj will be released when it goes out of scope

// share the object so we can clone it
let obj = obj.share();
let another_ref = obj.clone();
// dropping our other reference will decrement the retain count
drop(another_ref);

let weak = WeakId::new(&obj);
assert!(weak.load().is_some());
// After the object is deallocated, our weak pointer returns none
drop(obj);
assert!(weak.load().is_none());
```
