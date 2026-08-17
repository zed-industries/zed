#[repr(C)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
/// An `Alpha + Red + Green + Blue` pixel.
///
/// # Examples
///
/// ```
/// use rgb::Argb;
///
/// let pixel: Argb<u8> = Argb { a: 255, r: 0, g: 0, b: 0 };
/// ```
pub struct Argb<T, A = T> {
    /// Alpha Component
    pub a: A,
    /// Red Component
    pub r: T,
    /// Green Component
    pub g: T,
    /// Blue Component
    pub b: T,
}
