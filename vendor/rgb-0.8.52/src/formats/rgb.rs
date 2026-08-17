#[repr(C)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
/// A `Red + Green + Blue` pixel.
///
/// # Examples
///
/// ```
/// use rgb::Rgb;
///
/// let pixel: Rgb<u8> = Rgb { r: 0, g: 0, b: 0 };
/// ```
pub struct Rgb<T> {
    /// Red Component
    pub r: T,
    /// Green Component
    pub g: T,
    /// Blue Component
    pub b: T,
}
