/// Casting the struct to slices of its components
pub trait ComponentSlice<T> {
    /// The components interpreted as an array, e.g. one `RGB` expands to 3 elements.
    ///
    /// It's implemented for individual pixels as well as slices of pixels.
    fn as_slice(&self) -> &[T];

    /// The components interpreted as a mutable array, e.g. one `RGB` expands to 3 elements.
    ///
    /// It's implemented for individual pixels as well as slices of pixels.
    ///
    /// If you get an error when calling this on an array, add `[..]`
    ///
    /// > use of unstable library feature 'array_methods'
    ///
    /// ```rust,ignore
    /// arr[..].as_mut_slice()
    /// ```
    fn as_mut_slice(&mut self) -> &mut [T];
}

/// Use [`::bytemuck::cast_slice()`] instead.
///
/// Casting a slice of `RGB/A` values to a slice of `u8`
///
/// If instead of `RGB8` you use `RGB<MyCustomType>`, and you want to cast from/to that custom type,
/// implement the `Plain` trait for it:
///
/// ```rust
/// # #[derive(Copy, Clone)]
/// # struct MyCustomType;
/// unsafe impl rgb::Pod for MyCustomType {}
/// unsafe impl rgb::Zeroable for MyCustomType {}
/// ```
///
/// Plain types are not allowed to contain struct padding, booleans, chars, enums, references or pointers.
#[cfg(feature = "as-bytes")]
pub trait ComponentBytes<T: crate::Pod> where Self: ComponentSlice<T> {
    /// The components interpreted as raw bytes, in machine's native endian. In `RGB` bytes of the red component are first.
    #[inline]
    fn as_bytes(&self) -> &[u8] {
        assert_ne!(0, core::mem::size_of::<T>());
        let slice = self.as_slice();
        unsafe {
            core::slice::from_raw_parts(slice.as_ptr().cast(), core::mem::size_of_val(slice))
        }
    }

    /// The components interpreted as raw bytes, in machine's native endian. In `RGB` bytes of the red component are first.
    #[inline]
    fn as_bytes_mut(&mut self) -> &mut [u8] {
        assert_ne!(0, core::mem::size_of::<T>());
        let slice = self.as_mut_slice();
        unsafe {
            core::slice::from_raw_parts_mut(slice.as_mut_ptr().cast(), core::mem::size_of_val(slice))
        }
    }
}

/// Applying operation to every component
///
/// ```rust
/// use rgb::prelude::*;
/// # let pixel = rgb::RGB::new(0u8,0,0);
/// let inverted = pixel.map(|c| 255 - c);
///
/// // For simple math there are Add/Sub/Mul implementations:
/// let halved = pixel.map(|c| c / 2);
/// let doubled = pixel * 2;
/// ```
pub trait ComponentMap<DestPixel, SrcComponent, DestComponent> {
    /// Convenience function (equivalent of `self.iter().map().collect()`) for applying the same formula to every component.
    ///
    /// Note that it returns the pixel directly, not an Interator.
    fn map<Callback>(&self, f: Callback) -> DestPixel
        where Callback: FnMut(SrcComponent) -> DestComponent;
}

/// Same as `ComponentMap`, but doesn't change the alpha channel (if there's any alpha).
///
/// Import via `use rgb::prelude::*;` instead of directly.
pub trait ColorComponentMap<DestPixel, SrcComponent, DestComponent> {
    /// Convenience function for applying the same formula to every rgb/gray component, but skipping the alpha component.
    ///
    /// Note that it returns the pixel directly, not an Interator.
    #[doc(alias = "map_colors_same")]
    fn map_colors<Callback>(&self, f: Callback) -> DestPixel
        where Callback: FnMut(SrcComponent) -> DestComponent {
            #[allow(deprecated)]
            self.map_c(f)
        }

    /// Alias of `map_colors`
    #[deprecated(note = "renamed to map_colors")]
    fn map_c<Callback>(&self, f: Callback) -> DestPixel
        where Callback: FnMut(SrcComponent) -> DestComponent {
            self.map_colors(f)
    }
}
