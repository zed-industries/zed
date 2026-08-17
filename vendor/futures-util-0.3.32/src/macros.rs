/// Pins a value on the stack.
///
/// Can safely pin values that are not `Unpin` by taking ownership.
///
/// **Note:** Since Rust 1.68, this macro is soft-deprecated in favor of
/// [`pin!`](https://doc.rust-lang.org/std/pin/macro.pin.html) macro
/// in the standard library.
///
/// # Example
///
/// ```rust
/// # use futures_util::pin_mut;
/// # use core::pin::Pin;
/// # struct Foo {}
/// let foo = Foo { /* ... */ };
/// pin_mut!(foo);
/// let _: Pin<&mut Foo> = foo;
/// ```
#[macro_export]
macro_rules! pin_mut {
    ($($x:ident),* $(,)?) => { $(
        // Move the value to ensure that it is owned
        let mut $x = $x;
        // Shadow the original binding so that it can't be directly accessed
        // ever again.
        #[allow(unused_mut)]
        let mut $x = unsafe {
            $crate::__private::Pin::new_unchecked(&mut $x)
        };
    )* }
}
