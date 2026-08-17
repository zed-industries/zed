#[repr(C)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
/// A `Red + Green + Blue + Alpha` pixel.
///
/// # Examples
///
/// ```
/// use rgb::Rgba;
///
/// let pixel: Rgba<u8> = Rgba { r: 0, g: 0, b: 0, a: 255 };
/// ```
pub struct Rgba<T, A = T> {
    /// Red Component
    pub r: T,
    /// Green Component
    pub g: T,
    /// Blue Component
    pub b: T,
    /// Alpha Component
    pub a: A,
}
