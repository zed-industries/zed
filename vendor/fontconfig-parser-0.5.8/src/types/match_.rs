mod edit;
mod test;

pub use self::edit::*;
pub use self::test::*;

#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Match {
    pub target: MatchTarget,
    pub tests: Vec<Test>,
    pub edits: Vec<Edit>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum MatchTarget {
    Pattern,
    Font,
    Scan,
}

parse_enum! {
    MatchTarget,
    (Pattern, "pattern"),
    (Font, "font"),
    (Scan, "scan"),
}

impl Default for MatchTarget {
    fn default() -> Self {
        MatchTarget::Pattern
    }
}
