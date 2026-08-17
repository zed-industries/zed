use crate::formats::gray_a::GrayA;
use core::ops::{Deref, DerefMut};

#[repr(C)]
#[cfg_attr(feature = "unstable-experimental", deprecated(note = "renamed to GrayA"))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
/// A pixel for grayscale value + alpha components (rgb crate v0.8)
///
/// Through a `Deref` hack it renames the fields from `.0` and `.1`
/// to `.v` (value) and `.a` (alpha)
#[allow(non_camel_case_types)]
pub struct GrayAlpha_v08<T, A = T>(
    /// Grayscale Component
    ///
    /// This field has been renamed to `.v`
    #[deprecated(note = "Please use the .v field instaed (it's available through the magic of Deref to GrayA type)")]
    pub T,
    /// Alpha Component. This field has been renamed to `.a`.
    #[deprecated(note = "Please use the .a field instead (it's available through the magic of Deref to GrayA type)")]
    pub A,
);

impl<T: Copy> GrayAlpha_v08<T> {
    /// Reads the `.v` field
    ///
    /// Please use the `.v` field directly whenever possible. This function isn't necessary, and exists only to ease migration between major versions of the RGB crate.
    #[allow(deprecated)]
    pub fn value(self) -> T {
        self.0
    }

    /// Exposes the `.v` field for writing
    ///
    /// Please use the `.v` field directly whenever possible.  This function isn't necessary, and exists only to ease migration between major versions of the RGB crate.
    #[allow(deprecated)]
    pub fn value_mut(&mut self) -> &mut T {
        &mut self.0
    }
}

impl<T, A> Deref for GrayAlpha_v08<T, A> {
    type Target = GrayA<T, A>;

    /// A trick that allows using `.v` and `.a` on the old `GrayAlpha` type.
    fn deref(&self) -> &GrayA<T, A> {
        unsafe {
            &*(self as *const Self).cast::<GrayA::<T, A>>()
        }
    }
}

impl<T, A> DerefMut for GrayAlpha_v08<T, A> {
    /// A trick that allows using `.v` and `.a` on the old `GrayAlpha` type.
    fn deref_mut(&mut self) -> &mut GrayA<T, A> {
        unsafe {
            &mut *(self as *mut Self).cast::<GrayA::<T, A>>()
        }
    }
}

#[test]
#[allow(deprecated)]
fn swizzle() {
    let mut g = GrayAlpha_v08(10u8, 20u8);
    assert_eq!(10, g.v);
    assert_eq!(20, g.a);
    g.a = 7;
    assert_eq!(10, g.v);
    assert_eq!(7, g.1);
}
