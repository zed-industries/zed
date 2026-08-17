use crate::Property;

/// This element contains a single value which is compared with the target ('pattern', 'font', 'scan' or 'default') property "property" (substitute any of the property names seen above).
/// 'compare' can be one of "eq", "not_eq", "less", "less_eq", "more", "more_eq", "contains" or "not_contains".
/// 'qual' may either be the default, "any", in which case the match succeeds if any value associated with the property matches the test value,
/// or "all", in which case all of the values associated with the property must match the test value. 'ignore-blanks' takes a boolean value.
/// if 'ignore-blanks' is set "true", any blanks in the string will be ignored on its comparison. this takes effects only when compare="eq" or compare="not_eq".
/// When used in a <match target="font"> element, the target= attribute in the <test> element selects between matching the original pattern or the font.
/// "default" selects whichever target the outer <match> element has selected.
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Test {
    pub qual: TestQual,
    pub target: TestTarget,
    pub compare: TestCompare,
    pub value: Property,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum TestTarget {
    Default,
    Pattern,
    Font,
    Scan,
}

parse_enum! {
    TestTarget,
    (Default, "default"),
    (Pattern, "pattern"),
    (Font, "font"),
    (Scan, "scan"),
}

impl Default for TestTarget {
    fn default() -> Self {
        TestTarget::Default
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum TestCompare {
    Eq,
    NotEq,
    Less,
    LessEq,
    More,
    MoreEq,
    Contains,
    NotContains,
}

parse_enum! {
    TestCompare,
    (Eq, "eq"),
    (NotEq, "not_eq"),
    (Less, "less"),
    (LessEq, "less_eq"),
    (More, "more"),
    (MoreEq, "more_eq"),
    (Contains, "contains"),
    (NotContains, "not_contains"),
}

impl Default for TestCompare {
    fn default() -> Self {
        TestCompare::Eq
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum TestQual {
    Any,
    All,
}

parse_enum! {
    TestQual,
    (Any, "any"),
    (All, "all"),
}

impl Default for TestQual {
    fn default() -> Self {
        TestQual::Any
    }
}
