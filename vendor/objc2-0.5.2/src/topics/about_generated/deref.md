# Use of `Deref`

Framework crates uses the [`Deref`] trait in a bit special way: All objects
deref to their superclasses. For example, `NSMutableArray` derefs to
`NSArray`, which in turn derefs to `NSObject`.

Note that this is explicitly recommended against in [the
documentation][`Deref`] and [the Rust Design patterns
book][anti-pattern-deref] (see those links for details).

Due to Objective-C objects only ever being accessible behind pointers in
the first place, the problems stated there are less severe, and having the
implementation just means that everything is much nicer when you actually
want to use the objects!

All objects also implement [`AsRef`] and [`AsMut`] to their superclass,
and can be used in [`Retained::into_super`], so if you favour explicit
conversion, that is a possibility too.

[`Deref`]: std::ops::Deref
[`ClassType`]: crate::ClassType
[anti-pattern-deref]: https://rust-unofficial.github.io/patterns/anti_patterns/deref.html
[`Retained::into_super`]: crate::rc::Retained::into_super
