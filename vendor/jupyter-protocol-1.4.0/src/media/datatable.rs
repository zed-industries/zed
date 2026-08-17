use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// See <https://specs.frictionlessdata.io/tabular-data-resource/>
#[derive(Default, Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct TabularDataResource {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<PathOrPaths>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<serde_json::Value>>,
    pub schema: TableSchema,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub homepage: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sources: Option<Vec<Source>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub licenses: Option<Vec<License>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dialect: Option<Dialect>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mediatype: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encoding: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bytes: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hash: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(untagged)]
pub enum PathOrPaths {
    Single(String),
    Multiple(Vec<String>),
}

#[derive(Default, Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct TableSchema {
    pub fields: Vec<TableSchemaField>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_key: Option<PrimaryKey>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub foreign_keys: Option<Vec<ForeignKey>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub missing_values: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase", untagged)]
pub enum PrimaryKey {
    Single(String),
    Multiple(Vec<String>),
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ForeignKey {
    pub fields: ForeignKeyFields,
    pub reference: ForeignKeyReference,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(untagged)]
pub enum ForeignKeyFields {
    Single(String),
    Multiple(Vec<String>),
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ForeignKeyReference {
    pub resource: String,
    pub fields: ForeignKeyFields,
}

#[derive(Default, Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct TableSchemaField {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub example: Option<String>,
    #[serde(rename = "type")]
    pub field_type: FieldType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<FieldFormat>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub constraints: Option<HashMap<String, serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rdf_type: Option<String>,
}

#[derive(Default, Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum FieldType {
    String,
    Number,
    Integer,
    Date,
    Time,
    Datetime,
    Year,
    Yearmonth,
    Boolean,
    Object,
    Geopoint,
    Geojson,
    Array,
    Duration,
    #[default]
    Any,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum FieldFormat {
    Default,
    Email,
    Uri,
    Binary,
    Uuid,
    Any,
    Array,
    Object,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Source {
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct License {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Dialect {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delimiter: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub double_quote: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_terminator: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub null_sequence: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote_char: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub escape_char: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip_initial_space: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub header: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment_char: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub case_sensitive_header: Option<bool>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_minimal_datatable() {
        // source: https://specs.frictionlessdata.io/tabular-data-resource/#examples
        let raw = r#"{
          "profile": "tabular-data-resource",
          "name": "resource-name",
          "data": [
            {
              "id": 1,
              "first_name": "Louise"
            },
            {
              "id": 2,
              "first_name": "Julia"
            }
          ],
          "schema": {
            "fields": [
              {
                "name": "id",
                "type": "integer"
              },
              {
                "name": "first_name",
                "type": "string"
              }
            ],
            "primaryKey": "id"
          }
        }"#;

        let table: TabularDataResource = serde_json::from_str(raw).unwrap();
        assert_eq!(table.name, Some("resource-name".to_string()));
        assert_eq!(table.profile, Some("tabular-data-resource".to_string()));
        assert_eq!(table.title, None);
        assert_eq!(table.data.as_ref().unwrap().len(), 2);
        assert_eq!(table.schema.fields.len(), 2);
        assert_eq!(
            table.schema.primary_key,
            Some(PrimaryKey::Single("id".to_string()))
        );
    }

    #[test]
    fn test_deserialize_comprensive_datatable() {
        // source: https://specs.frictionlessdata.io/tabular-data-resource/#examples
        let raw = r#"{
              "profile": "tabular-data-resource",
              "name": "solar-system",
              "path": "http://example.com/solar-system.csv",
              "title": "The Solar System",
              "description": "My favourite data about the solar system.",
              "format": "csv",
              "mediatype": "text/csv",
              "encoding": "utf-8",
              "bytes": 1,
              "hash": "",
              "schema": {
                "fields": [
                  {
                    "name": "id",
                    "type": "integer"
                  },
                  {
                    "name": "name",
                    "type": "string"
                  },
                  {
                    "name": "description",
                    "type": "string"
                  }
                ],
                "primaryKey": "id"
              },
              "dialect": {
                "delimiter": ";",
                "doubleQuote": true
              },
              "sources": [{
                  "title": "The Solar System - 2001",
                  "path": "http://example.com/solar-system-2001.json",
                  "email": ""
                }],
                "licenses": [{
                  "name": "CC-BY-4.0",
                  "title": "Creative Commons Attribution 4.0",
                  "path": "https://creativecommons.org/licenses/by/4.0/"
                }]
            }"#;
        let table: TabularDataResource = serde_json::from_str(raw).unwrap();
        assert_eq!(table.name, Some("solar-system".to_string()));
        assert_eq!(table.profile, Some("tabular-data-resource".to_string()));
        assert_eq!(table.title.as_ref().unwrap(), "The Solar System");
        assert_eq!(table.data, None);
        assert_eq!(table.schema.fields.len(), 3);
        assert_eq!(
            table.schema.primary_key,
            Some(PrimaryKey::Single("id".to_string()))
        );
        assert_eq!(
            table.dialect.as_ref().unwrap().delimiter.as_ref().unwrap(),
            ";"
        );
        assert!(table.dialect.as_ref().unwrap().double_quote.unwrap());
        assert_eq!(table.sources.as_ref().unwrap().len(), 1);
        assert_eq!(table.licenses.as_ref().unwrap().len(), 1);
    }
}
