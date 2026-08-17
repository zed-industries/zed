#[repr(C)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
/// A `Blue + Green + Red + Alpha` pixel.
///
/// # Examples
///
/// ```
/// use rgb::Bgra;
///
/// let pixel: Bgra<u8> = Bgra { b: 0, g: 0, r: 0, a: 255 };
/// ```
pub struct Bgra<T, A = T> {
    /// Blue Component
    pub b: T,
    /// Green Component
    pub g: T,
    /// Red Component
    pub r: T,
    /// Alpha Component
    pub a: A,
}
