//! Test expansion of enum variants which have no associated data.

use darling::FromMeta;

#[derive(Debug, FromMeta)]
#[darling(rename_all = "snake_case")]
enum Pattern {
    Owned,
    Immutable,
    Mutable,
}

#[test]
fn expansion() {}
