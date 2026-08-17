//! These types are not used for generating `Index`es. They are provided to help with
//! creating compatible JSON structures for configuring the JavaScript search
//! function.
//!
//! *Reference:*
//! <http://elasticlunr.com/docs/configuration.js.html>

use std::collections::BTreeMap;

/// Used to set the search configuration for a specific field.
/// When `expand` or `bool` is `None`, elasticlunr.js will use the value from
/// the global configuration. The `boost` field, if present,
/// increases the importance of this field when ordering search results.
#[derive(Serialize, Deserialize, Default, Debug, Copy, Clone, Eq, PartialEq)]
pub struct SearchOptionsField {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boost: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bool: Option<SearchBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<bool>,
}

/// Sets which boolean model is used for searching with
/// multiple terms. Defaults to `Or`.
///
/// - *AND* requires every search term to be present in results
/// - *OR* accepts results which have at least one term
///
#[derive(Serialize, Deserialize, Debug, Copy, Clone, Eq, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SearchBool {
    Or,
    And,
}

impl Default for SearchBool {
    fn default() -> Self {
        SearchBool::Or
    }
}

/// The search configuration map which is passed to the
/// elasticlunr.js `Index.search()` function.
///
/// |Key     |Default|
/// |--------|-------|
/// |`bool`  |`OR`   |
/// |`expand`|`false`|
#[derive(Serialize, Deserialize, Default, Debug, Clone, Eq, PartialEq)]
pub struct SearchOptions {
    pub bool: SearchBool,
    pub expand: bool,
    pub fields: BTreeMap<String, SearchOptionsField>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_normal_config() {
        let options = SearchOptions {
            fields: btreemap![
                "title".into() => SearchOptionsField {
                    boost: Some(5),
                    ..Default::default()
                },
                "body".into() => SearchOptionsField {
                    boost: Some(1),
                    ..Default::default()
                },
            ],
            ..Default::default()
        };
        let stringed = serde_json::to_string(&options).unwrap();

        assert_eq!(
            stringed,
            r#"{"bool":"OR","expand":false,"fields":{"body":{"boost":1},"title":{"boost":5}}}"#
        );
    }

    #[test]
    fn test_complex_config() {
        let options = SearchOptions {
            fields: btreemap! {
                "title".into() => SearchOptionsField {
                    expand: Some(true),
                    ..Default::default()
                },
                "body".into() => SearchOptionsField {
                    bool: Some(SearchBool::Or),
                    ..Default::default()
                },
                "breadcrumbs".into() => SearchOptionsField {
                    bool: Some(SearchBool::default()),
                    boost: Some(200),
                    ..Default::default()
                },
            },
            expand: false,
            bool: SearchBool::And,
        };
        let stringed = serde_json::to_string_pretty(&options).unwrap();

        assert_eq!(
            stringed,
            r#"{
  "bool": "AND",
  "expand": false,
  "fields": {
    "body": {
      "bool": "OR"
    },
    "breadcrumbs": {
      "boost": 200,
      "bool": "OR"
    },
    "title": {
      "expand": true
    }
  }
}"#
        );
    }
}
