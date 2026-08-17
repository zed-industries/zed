/// Alias elements provide a shorthand notation for the set of common match operations needed to substitute one font family for another. They contain a <family> element followed by optional <prefer>, <accept> and <default> elements. Fonts matching the <family> element are edited to prepend the list of <prefer>ed families before the matching <family>, append the <accept>able families after the matching <family> and append the <default> families to the end of the family list.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Alias {
    /// alias name
    pub alias: String,

    /// `<prefer>`
    pub prefer: Vec<String>,
    /// `<accept>`
    pub accept: Vec<String>,
    /// `<default>`
    pub default: Vec<String>,
}
