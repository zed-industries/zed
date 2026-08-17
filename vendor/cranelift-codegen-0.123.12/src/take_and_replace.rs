//! Helper for temporarily taking values out and then putting them back in.

/// An RAII type to temporarily take a `U` out of a `T` and then put it back
/// again on drop.
///
/// This allows you to split borrows, if necessary, to satisfy the borrow
/// checker.
///
/// The `F` type parameter must project from the container type `T` to its `U`
/// that we want to temporarily take out of it.
///
/// # Example
///
/// ```
/// use cranelift_codegen::TakeAndReplace;
///
/// #[derive(Default)]
/// struct BigContextStruct {
///     items: Vec<u32>,
///     count: usize,
/// }
///
/// impl BigContextStruct {
///     fn handle_item(&mut self, item: u32) {
///         self.count += 1;
///         println!("Handled {item}!");
///     }
/// }
///
/// let mut ctx = BigContextStruct::default();
/// ctx.items.extend([42, 1337, 1312]);
///
/// {
///     // Temporarily take `self.items` out of `ctx`.
///     let mut guard = TakeAndReplace::new(&mut ctx, |ctx| &mut ctx.items);
///     let (ctx, items) = guard.get();
///
///     // Now we can both borrow/iterate/mutate `items` and call `&mut self` helper
///     // methods on `ctx`. This would not otherwise be possible if we didn't split
///     // the borrows, since Rust's borrow checker doesn't see through methods and
///     // know that `handle_item` doesn't use `self.items`.
///     for item in items.drain(..) {
///         ctx.handle_item(item);
///     }
/// }
///
/// // When `guard` is dropped, `items` is replaced in `ctx`, allowing us to
/// // reuse its capacity and avoid future allocations.  ```
/// assert!(ctx.items.capacity() >= 3);
/// ```
pub struct TakeAndReplace<'a, T, U, F>
where
    F: Fn(&mut T) -> &mut U,
    U: Default,
{
    container: &'a mut T,
    value: U,
    proj: F,
}

impl<'a, T, U, F> Drop for TakeAndReplace<'a, T, U, F>
where
    F: Fn(&mut T) -> &mut U,
    U: Default,
{
    fn drop(&mut self) {
        *(self.proj)(self.container) = std::mem::take(&mut self.value);
    }
}

impl<'a, T, U, F> TakeAndReplace<'a, T, U, F>
where
    F: Fn(&mut T) -> &mut U,
    U: Default,
{
    /// Create a new `TakeAndReplace` that temporarily takes out
    /// `proj(container)`.
    pub fn new(mut container: &'a mut T, proj: F) -> Self {
        let value = std::mem::take(proj(&mut container));
        TakeAndReplace {
            container,
            value,
            proj,
        }
    }

    /// Get the underlying container and taken-out value.
    pub fn get(&mut self) -> (&mut T, &mut U) {
        (&mut *self.container, &mut self.value)
    }
}
