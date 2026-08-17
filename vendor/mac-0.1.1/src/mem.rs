//! Macros for low-level memory manipulation.

/// Make a tuple of the addresses of some of a struct's fields.
///
/// This is useful when you are transmuting between struct types
/// and would like an additional dynamic check that the layouts
/// match. It's difficult to make such an assertion statically
/// in Rust at present.
///
/// # Example
///
/// ```
/// # #[macro_use] extern crate mac;
/// use std::mem;
///
/// # fn main() {
/// struct Foo { x: i32, y: i32 }
/// struct Bar { x: u32, y: u32 }
///
/// let foo = Foo { x: 3, y: 4 };
/// let old_addrs = addrs_of!(foo => x, y);
///
/// let bar = unsafe {
///     mem::transmute::<&Foo, &Bar>(&foo)
/// };
/// let new_addrs = addrs_of!(bar => x, y);
/// assert_eq!(old_addrs, new_addrs);
///
/// assert_eq!(bar.x, 3);
/// assert_eq!(bar.y, 4);
/// # }
/// ```
#[macro_export]
macro_rules! addrs_of {
    ($obj:expr => $($field:ident),+) => {
        ( // make a tuple
            $(
                unsafe {
                    ::std::mem::transmute::<_, usize>(&$obj.$field)
                }
            ),+
        )
    }
}
