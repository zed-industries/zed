# The Model-View-Controller design pattern

The [Model-View-Controller (MVC) design pattern][mvc-doc] is quite prominent in Cocoa application development, and you may end up needing to incorporate parts of it in your application. You can use the following as a rough guideline for how to do so.

**Model**: Use plain Rust structures inside [`Cell<T>`] or [`RefCell<T>`] so that you can mutate them behind shared references (or possibly `Rc<RefCell<T>>`, depending on if you need to share data between multiple controllers).

**View**: Use the built-in views (`NSView` or `UIView`). If you need to register a delegate on these, use the controller as the delegate.

**Controller**: Use the [`define_class!`] macro to create a new object. Use [`MainThreadOnly`] as the [`ClassType::ThreadKind`], so that you can implement view delegate protocols.

[mvc-doc]: https://developer.apple.com/library/archive/documentation/General/Conceptual/CocoaEncyclopedia/Model-View-Controller/Model-View-Controller.html
[`Cell<T>`]: core::cell::Cell
[`RefCell<T>`]: core::cell::RefCell
[`define_class!`]: crate::define_class
[`MainThreadOnly`]: crate::MainThreadOnly
[`ClassType::ThreadKind`]: crate::ClassType::ThreadKind
