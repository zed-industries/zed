/// A marker trait which may only be implemented for native array types, like
/// `[T; 2]`. The library incorporates several components that are parameterized
/// by array types, but currently Rust provides no safe mechanism to express
/// that.
///
/// In order to work around the limitations, these methods only accept arrays
/// which implement the `RealArray` type. The library provides an implementation
/// of `RealArray` for arrays up to length 64, as well as for all powers of 2
/// up to 64k.
///
/// In order to let the library accept arrays of bigger sizes, `RealArray` can
/// be implemented by users via newtypes. A type as defined in the following
/// example can be passed to the library:
///
/// ```
/// use futures_intrusive::buffer::RealArray;
/// use futures_intrusive::channel::LocalChannel;
///
/// struct I32x384Array([i32; 384]);
/// unsafe impl RealArray<i32> for I32x384Array {
///     const LEN: usize = 384;
/// }
///
/// impl AsMut<[i32]> for I32x384Array {
///     fn as_mut(&mut self) -> &mut [i32] {
///         &mut self.0
///     }
/// }
///
/// impl AsRef<[i32]> for I32x384Array {
///     fn as_ref(&self) -> &[i32] {
///         &self.0
///     }
/// }
///
/// fn main() {
///     let channel = LocalChannel::<i32, I32x384Array>::new();
/// }
///
/// ```
pub unsafe trait RealArray<T> {
    /// The length of the array
    const LEN: usize;
}

macro_rules! real_array {
    ($($N:expr),+) => {
        $(
            unsafe impl<T> RealArray<T> for [T; $N] {
                const LEN: usize = $N;
            }
        )+
    }
}

real_array!(
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20,
    21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39,
    40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58,
    59, 60, 61, 62, 63, 64, 128, 256, 512, 1024, 2048, 4096, 8192, 16384,
    32768, 65536
);
