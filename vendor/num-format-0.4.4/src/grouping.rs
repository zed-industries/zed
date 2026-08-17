/// Type for specifying how digits are grouped together (e.g. 1,000,000 vs. 10,00,000 vs. 1000000).
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "with-serde", derive(Serialize, Deserialize))]
pub enum Grouping {
    /// Digits are separated into groups of three (e.g. 10,000,000)
    Standard,
    /// The first three digits are grouped together and all digits after that are
    /// separated into groups of two (e.g. 1,00,00,000)
    Indian,
    /// No grouping (e.g. 10000000)
    Posix,
}
