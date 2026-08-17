use core::fmt;
use std::str::FromStr;

use crate::{uri, Error};
use ahash::AHashSet;
use fluent_uri::Uri;
use serde_json::Value;

/// A JSON Schema vocabulary identifier, representing standard vocabularies (Core, Applicator, etc.)
/// or custom ones via URI.
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Vocabulary {
    Core,
    Applicator,
    Unevaluated,
    Validation,
    Metadata,
    Format,
    FormatAnnotation,
    Content,
    Custom(Uri<String>),
}

impl FromStr for Vocabulary {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "https://json-schema.org/draft/2020-12/vocab/core"
            | "https://json-schema.org/draft/2019-09/vocab/core" => Ok(Vocabulary::Core),
            "https://json-schema.org/draft/2020-12/vocab/applicator"
            | "https://json-schema.org/draft/2019-09/vocab/applicator" => {
                Ok(Vocabulary::Applicator)
            }
            "https://json-schema.org/draft/2020-12/vocab/unevaluated" => {
                Ok(Vocabulary::Unevaluated)
            }
            "https://json-schema.org/draft/2020-12/vocab/validation"
            | "https://json-schema.org/draft/2019-09/vocab/validation" => {
                Ok(Vocabulary::Validation)
            }
            "https://json-schema.org/draft/2020-12/vocab/meta-data"
            | "https://json-schema.org/draft/2019-09/vocab/meta-data" => Ok(Vocabulary::Metadata),
            "https://json-schema.org/draft/2020-12/vocab/format"
            | "https://json-schema.org/draft/2019-09/vocab/format" => Ok(Vocabulary::Format),
            "https://json-schema.org/draft/2020-12/vocab/format-annotation" => {
                Ok(Vocabulary::FormatAnnotation)
            }
            "https://json-schema.org/draft/2020-12/vocab/content"
            | "https://json-schema.org/draft/2019-09/vocab/content" => Ok(Vocabulary::Content),
            _ => Ok(Vocabulary::Custom(uri::from_str(s)?)),
        }
    }
}

/// A set of enabled JSON Schema vocabularies.
#[derive(Clone, Default, PartialEq, Eq)]
pub struct VocabularySet {
    known: u8,
    custom: AHashSet<Uri<String>>,
}

impl fmt::Debug for VocabularySet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut debug_list = f.debug_list();

        // Add known vocabularies
        if self.known & (1 << 0) != 0 {
            debug_list.entry(&"core");
        }
        if self.known & (1 << 1) != 0 {
            debug_list.entry(&"applicator");
        }
        if self.known & (1 << 2) != 0 {
            debug_list.entry(&"unevaluated");
        }
        if self.known & (1 << 3) != 0 {
            debug_list.entry(&"validation");
        }
        if self.known & (1 << 4) != 0 {
            debug_list.entry(&"meta-data");
        }
        if self.known & (1 << 5) != 0 {
            debug_list.entry(&"format");
        }
        if self.known & (1 << 6) != 0 {
            debug_list.entry(&"format-annotation");
        }
        if self.known & (1 << 7) != 0 {
            debug_list.entry(&"content");
        }

        // Add custom vocabularies
        if !self.custom.is_empty() {
            let mut custom: Vec<_> = self.custom.iter().map(Uri::as_str).collect();
            custom.sort_unstable();
            for uri in custom {
                debug_list.entry(&uri);
            }
        }
        debug_list.finish()
    }
}

impl VocabularySet {
    pub(crate) fn new() -> Self {
        Self::default()
    }

    pub(crate) fn from_known(known: u8) -> Self {
        Self {
            known,
            custom: AHashSet::new(),
        }
    }

    pub(crate) fn add(&mut self, vocabulary: Vocabulary) {
        match vocabulary {
            Vocabulary::Core => self.known |= 1 << 0,
            Vocabulary::Applicator => self.known |= 1 << 1,
            Vocabulary::Unevaluated => self.known |= 1 << 2,
            Vocabulary::Validation => self.known |= 1 << 3,
            Vocabulary::Metadata => self.known |= 1 << 4,
            Vocabulary::Format => self.known |= 1 << 5,
            Vocabulary::FormatAnnotation => self.known |= 1 << 6,
            Vocabulary::Content => self.known |= 1 << 7,
            Vocabulary::Custom(uri) => {
                self.custom.insert(uri);
            }
        }
    }
    #[must_use]
    pub fn contains(&self, vocabulary: &Vocabulary) -> bool {
        match vocabulary {
            Vocabulary::Core => self.known & (1 << 0) != 0,
            Vocabulary::Applicator => self.known & (1 << 1) != 0,
            Vocabulary::Unevaluated => self.known & (1 << 2) != 0,
            Vocabulary::Validation => self.known & (1 << 3) != 0,
            Vocabulary::Metadata => self.known & (1 << 4) != 0,
            Vocabulary::Format => self.known & (1 << 5) != 0,
            Vocabulary::FormatAnnotation => self.known & (1 << 6) != 0,
            Vocabulary::Content => self.known & (1 << 7) != 0,
            Vocabulary::Custom(uri) => self.custom.contains(uri),
        }
    }
}

pub(crate) const DRAFT_2020_12_VOCABULARIES: u8 = 0b1111_1111;
pub(crate) const DRAFT_2019_09_VOCABULARIES: u8 = 0b1001_1011;

pub(crate) fn find(document: &Value) -> Result<Option<VocabularySet>, Error> {
    if let Some(schema) = document.get("$id").and_then(|s| s.as_str()) {
        match schema {
            "https://json-schema.org/schema" | "https://json-schema.org/draft/2020-12/schema" => {
                // All known vocabularies
                Ok(Some(VocabularySet::from_known(DRAFT_2020_12_VOCABULARIES)))
            }
            "https://json-schema.org/draft/2019-09/schema" => {
                // Core, Applicator, Validation, Metadata, Content
                Ok(Some(VocabularySet::from_known(DRAFT_2019_09_VOCABULARIES)))
            }
            "https://json-schema.org/draft-07/schema"
            | "https://json-schema.org/draft-06/schema"
            | "https://json-schema.org/draft-04/schema" => Ok(None),
            _ => {
                // For unknown schemas, parse the $vocabulary object
                if let Some(vocab_obj) = document.get("$vocabulary").and_then(|v| v.as_object()) {
                    let mut set = VocabularySet::new();
                    for (uri, enabled) in vocab_obj {
                        if enabled.as_bool().unwrap_or(false) {
                            set.add(Vocabulary::from_str(uri)?);
                        }
                    }
                    Ok(Some(set))
                } else {
                    Ok(None)
                }
            }
        }
    } else {
        Ok(None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(&Vocabulary::Core, 0b0000_0001, true)]
    #[test_case(&Vocabulary::Applicator, 0b0000_0010, true)]
    #[test_case(&Vocabulary::Unevaluated, 0b0000_0100, true)]
    #[test_case(&Vocabulary::Validation, 0b0000_1000, true)]
    #[test_case(&Vocabulary::Metadata, 0b0001_0000, true)]
    #[test_case(&Vocabulary::Format, 0b0010_0000, true)]
    #[test_case(&Vocabulary::FormatAnnotation, 0b0100_0000, true)]
    #[test_case(&Vocabulary::Content, 0b1000_0000, true)]
    #[test_case(&Vocabulary::Core, 0b1111_1110, false)]
    #[test_case(&Vocabulary::Applicator, 0b1111_1101, false)]
    #[test_case(&Vocabulary::Unevaluated, 0b111_11011, false)]
    #[test_case(&Vocabulary::Validation, 0b1111_0111, false)]
    #[test_case(&Vocabulary::Metadata, 0b1110_1111, false)]
    #[test_case(&Vocabulary::Format, 0b1101_1111, false)]
    #[test_case(&Vocabulary::FormatAnnotation, 0b1011_1111, false)]
    #[test_case(&Vocabulary::Content, 0b0111_1111, false)]
    fn test_vocabulary_set(vocabulary: &Vocabulary, known: u8, expected: bool) {
        let set = VocabularySet::from_known(known);
        assert_eq!(set.contains(vocabulary), expected);
    }

    #[test]
    fn test_vocabulary_set_add_and_contains() {
        let mut set = VocabularySet::new();

        set.add(Vocabulary::Core);
        set.add(Vocabulary::Applicator);
        set.add(Vocabulary::Validation);
        set.add(Vocabulary::Metadata);
        set.add(Vocabulary::Content);

        assert!(set.contains(&Vocabulary::Core));
        assert!(set.contains(&Vocabulary::Applicator));
        assert!(set.contains(&Vocabulary::Validation));
        assert!(set.contains(&Vocabulary::Metadata));
        assert!(set.contains(&Vocabulary::Content));

        assert!(!set.contains(&Vocabulary::Unevaluated));
        assert!(!set.contains(&Vocabulary::Format));
        assert!(!set.contains(&Vocabulary::FormatAnnotation));

        set.add(Vocabulary::Unevaluated);
        set.add(Vocabulary::Format);
        set.add(Vocabulary::FormatAnnotation);

        assert!(set.contains(&Vocabulary::Unevaluated));
        assert!(set.contains(&Vocabulary::Format));
        assert!(set.contains(&Vocabulary::FormatAnnotation));
    }

    #[test]
    fn test_vocabulary_set_debug() {
        let mut set = VocabularySet::from_known(0b0001_1111); // Core, Applicator, Unevaluated, Validation, Metadata
        set.add(Vocabulary::Custom(
            uri::from_str("https://example.com/custom-vocab").unwrap(),
        ));

        assert_eq!(
            format!("{set:?}"),
            "[\"core\", \"applicator\", \"unevaluated\", \"validation\", \"meta-data\", \"https://example.com/custom-vocab\"]"
        );
    }

    #[test]
    fn test_custom_vocabulary() {
        let custom_uri = uri::from_str("https://example.com/custom-vocab").expect("Invalid URI");
        let mut set = VocabularySet::new();
        set.add(Vocabulary::Custom(custom_uri.clone()));

        assert!(set.contains(&Vocabulary::Custom(custom_uri)));
        assert!(!set.contains(&Vocabulary::Custom(
            uri::from_str("https://example.com/other-vocab").expect("Invalid URI")
        )));
    }

    #[test_case(
        &serde_json::json!({"$id": "https://json-schema.org/draft/2020-12/schema"}),
        "Some([\"core\", \"applicator\", \"unevaluated\", \"validation\", \"meta-data\", \"format\", \"format-annotation\", \"content\"])"
        ; "2020-12 draft"
    )]
    #[test_case(
        &serde_json::json!({"$id": "https://json-schema.org/draft/2019-09/schema"}),
        "Some([\"core\", \"applicator\", \"validation\", \"meta-data\", \"content\"])"
        ; "2019-09 draft"
    )]
    #[test_case(
        &serde_json::json!({"$id": "https://json-schema.org/draft-07/schema"}),
        "None"
        ; "draft-07"
    )]
    #[test_case(
        &serde_json::json!({
            "$id": "https://example.com/custom-schema",
            "$vocabulary": {
                "https://example.com/custom-vocab1": true,
                "https://example.com/custom-vocab2": true,
                "https://example.com/custom-vocab3": false,
            }
        }),
        "Some([\"https://example.com/custom-vocab1\", \"https://example.com/custom-vocab2\"])"
        ; "custom schema"
    )]
    #[test_case(
        &serde_json::json!({}),
        "None"
        ; "no $id keyword"
    )]
    fn test_find(schema: &serde_json::Value, expected: &str) {
        let set = find(schema).expect("Invalid vocabulary");
        assert_eq!(format!("{set:?}"), expected);
    }
}
