#[repr(C)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
/// A `Grayscale` pixel (rgb crate v0.8)
#[allow(non_camel_case_types)]
pub struct Gray_v08<T>(
    /// Grayscale Component. This field will be renamed to `v`.
    #[deprecated(note = "Please use .value() or .value_mut() instead. This field will be renamed to .v in the next major version")]
    pub T,
);

impl<T: Copy> Gray_v08<T> {
    /// Reads the `.0` field
    ///
    /// This function isn't necessary, but it is forwards-compatible with the next major version of the RGB crate.
    #[allow(deprecated)]
    pub fn value(self) -> T {
        self.0
    }

    /// Exposes the `.0` field for writing
    ///
    /// This function isn't necessary, but it is forwards-compatible with the next major version of the RGB crate.
    #[allow(deprecated)]
    pub fn value_mut(&mut self) -> &mut T {
        &mut self.0
    }

    /// Add alpha component to this pixel
    #[allow(deprecated)]
    pub fn with_alpha(self, add_alpha_value: T) -> crate::formats::gray_alpha::GrayAlpha_v08<T> {
        crate::formats::gray_alpha::GrayAlpha_v08(self.0, add_alpha_value)
    }
}

#[cfg(feature = "unstable-experimental")]
/// A `Grayscale` pixel (rgb crate v0.9)
///
/// This is the new gray pixel type as opposed to the legacy gray type
/// (`rgb::Gray`) which is kept for backwards-compatibility.
///
/// # Examples
///
/// ```
/// use rgb::Gray;
///
/// let pixel: Gray<u8> = Gray { v: 0 };
/// ```
#[allow(non_camel_case_types)]
#[repr(C)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[doc(alias = "Luma")]
pub struct Gray_v09<T> {
    /// Grayscale Component
    pub v: T,
}

#[cfg(feature = "unstable-experimental")]
impl<T> core::ops::Deref for Gray_v08<T> {
    type Target = Gray_v09<T>;

    fn deref(&self) -> &Gray_v09<T> {
        unsafe {
            &*(self as *const Self as *const Gray_v09::<T>)
        }
    }
}

#[cfg(feature = "unstable-experimental")]
impl<T: Copy> Gray_v09<T> {
    /// Reads the `.v` field
    ///
    /// This function isn't necessary, but it is forwards-compatible with the next major version of the RGB crate.
    pub fn value(self) -> T {
        self.v
    }

    /// Exposes the `.v` field for writing
    ///
    /// This function isn't necessary, but it is forwards-compatible with the next major version of the RGB crate.
    pub fn value_mut(&mut self) -> &mut T {
        &mut self.v
    }

    /// Add alpha component to this pixel
    pub fn with_alpha(self, add_alpha_value: T) -> crate::formats::gray_a::GrayA<T> {
        crate::formats::gray_a::GrayA { v: self.v, a: add_alpha_value }
    }
}

#[test]
#[cfg(feature = "unstable-experimental")]
fn swizzle() {
    let g = Gray_v08(10u8);
    assert_eq!(10, g.v);
    assert_eq!(10, g.0);
}
