/// Wrapper to unsafely define a wrapper type that can be used with `bytemuck`'s traits.
///
/// This is very useful as it allows us to use bytemuck on foreign types. Despite the
/// unsafe assertion, it means that bytemuck is handling all the actual casting,
/// so we can't screw up size or alignment handling.
///
/// Once wrapped you can use the [`bytemuck::TransparentWrapper`] methods and
/// all the free methods that come with [`bytemuck::Pod`] and [`bytemuck::Zeroable`].
///
/// # Safety
///
/// Once wrapped, the resulting type must follow all the invariants
/// of the [`bytemuck::Pod`] and [`bytemuck::Zeroable`] traits.
#[macro_export]
macro_rules! bytemuck_wrapper {
    (unsafe struct $name:ident($inner:ty)) => {
        #[derive(Copy, Clone)]
        #[repr(transparent)]
        struct $name($inner);

        unsafe impl bytemuck::Zeroable for $name {}
        unsafe impl bytemuck::Pod for $name {}
        unsafe impl bytemuck::TransparentWrapper<$inner> for $name {}
    };
}
