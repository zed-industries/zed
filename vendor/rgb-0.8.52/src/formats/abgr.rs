#[repr(C)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
/// An `Alpha + Blue + Green + Red` pixel.
///
/// # Examples
///
/// ```
/// use rgb::Abgr;
///
/// let pixel: Abgr<u8> = Abgr { a: 255, b: 0, g: 0, r: 0 };
/// ```
pub struct Abgr<T, A = T> {
    /// Alpha Component
    pub a: A,
    /// Blue Component
    pub b: T,
    /// Green Component
    pub g: T,
    /// Red Component
    pub r: T,
}
