# Weak properties

Delegates in Cocoa are often stored as [`rc::Weak`] properties, to avoid reference cycles when the delegate also wants to store the object it is the delegate of.

This can be a bit confusing sometimes if you create a delegate, set it on an object, and then expect your delegate methods to be called later on (which they in reality won't since the delegate will have been deallocated).

In practice, you will have to store your delegate objects somewhere else, for example in your top-level application delegate.

See Apple's [documentation on weak references][mem-weak] for a few more details.

[`rc::Weak`]: crate::rc::Weak
[mem-weak]: https://developer.apple.com/library/archive/documentation/Cocoa/Conceptual/MemoryMgmt/Articles/mmPractical.html#//apple_ref/doc/uid/TP40004447-1000810
