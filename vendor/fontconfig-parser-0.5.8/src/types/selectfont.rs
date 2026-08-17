use crate::Property;

#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SelectFont {
    pub rejects: Vec<FontMatch>,
    pub accepts: Vec<FontMatch>,
}

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum FontMatch {
    Glob(String),
    Pattern(Vec<Property>),
}
