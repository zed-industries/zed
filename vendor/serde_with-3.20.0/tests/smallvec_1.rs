//! Test Cases for `SmallVec`

mod utils;

use crate::utils::is_equal;
use expect_test::expect;
use serde::{Deserialize, Serialize};
use serde_with::{
    formats::{PreferMany, PreferOne},
    serde_as, DisplayFromStr, OneOrMany,
};
use smallvec_1::SmallVec;

#[test]
fn test_smallvec_basic() {
    #[serde_as]
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct S(#[serde_as(as = "SmallVec<[_; 4]>")] SmallVec<[u32; 4]>);

    // Normal
    is_equal(
        S(SmallVec::from_vec(vec![1, 2, 3])),
        expect![[r#"
          [
            1,
            2,
            3
          ]"#]],
    );
    is_equal(S(SmallVec::new()), expect![[r#"[]"#]]);
}

#[test]
fn test_smallvec_displayfromstr() {
    #[serde_as]
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct S(#[serde_as(as = "SmallVec<[DisplayFromStr; 4]>")] SmallVec<[u32; 4]>);

    // Normal
    is_equal(
        S(SmallVec::from_vec(vec![1, 2, 3])),
        expect![[r#"
          [
            "1",
            "2",
            "3"
          ]"#]],
    );
    is_equal(S(SmallVec::new()), expect![[r#"[]"#]]);
}

#[test]
fn test_smallvec_large() {
    #[serde_as]
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct S(#[serde_as(as = "SmallVec<[_; 2]>")] SmallVec<[u32; 2]>);

    // Test with more elements than inline capacity
    is_equal(
        S(SmallVec::from_vec(vec![1, 2, 3, 4, 5])),
        expect![[r#"
          [
            1,
            2,
            3,
            4,
            5
          ]"#]],
    );
}

#[test]
fn test_smallvec_nested() {
    #[serde_as]
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct S {
        #[serde_as(as = "SmallVec<[DisplayFromStr; 4]>")]
        values: SmallVec<[u8; 4]>,
    }

    is_equal(
        S {
            values: SmallVec::from_vec(vec![1, 2, 3]),
        },
        expect![[r#"
          {
            "values": [
              "1",
              "2",
              "3"
            ]
          }"#]],
    );
}

#[test]
fn test_smallvec_string() {
    #[serde_as]
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct S(#[serde_as(as = "SmallVec<[_; 4]>")] SmallVec<[String; 4]>);

    is_equal(
        S(SmallVec::from_vec(vec![
            "foo".to_string(),
            "bar".to_string(),
        ])),
        expect![[r#"
          [
            "foo",
            "bar"
          ]"#]],
    );
}

#[test]
fn test_smallvec_oneormany_preferone() {
    #[serde_as]
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct S(#[serde_as(as = "OneOrMany<_, PreferOne>")] SmallVec<[String; 4]>);

    is_equal(
        S(SmallVec::from_vec(vec!["foo".to_string()])),
        expect![[r#""foo""#]],
    );
    is_equal(
        S(SmallVec::from_vec(vec![
            "foo".to_string(),
            "bar".to_string(),
        ])),
        expect![[r#"
            [
              "foo",
              "bar"
            ]"#]],
    );
}

#[test]
fn test_smallvec_oneormany_prefermany() {
    #[serde_as]
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct S(#[serde_as(as = "OneOrMany<_, PreferMany>")] SmallVec<[String; 4]>);

    is_equal(
        S(SmallVec::from_vec(vec!["foo".to_string()])),
        expect![[r#"
            [
              "foo"
            ]"#]],
    );
    is_equal(
        S(SmallVec::from_vec(vec![
            "foo".to_string(),
            "bar".to_string(),
        ])),
        expect![[r#"
            [
              "foo",
              "bar"
            ]"#]],
    );
}
