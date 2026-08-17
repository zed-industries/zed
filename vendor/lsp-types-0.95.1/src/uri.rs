use std::{fmt::Display, hash::Hash, ops::Deref, str::FromStr};

use serde::{de::Error, Deserialize, Serialize};

/// Newtype struct around `url::Url` with serialization implementations that properly encode brackets for LSP compatibility.
#[derive(Debug, Clone)]
pub struct Uri(url::Url);

impl Serialize for Uri {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Encode brackets for LSP compatibility
        let url_str = self.0.as_str();
        if url_str.contains('[') || url_str.contains(']') {
            let encoded: String = url_str.chars().flat_map(|c| match c {
                '[' => vec!['%', '5', 'B'],
                ']' => vec!['%', '5', 'D'], 
                _ => vec![c],
            }).collect();
            encoded.serialize(serializer)
        } else {
            url_str.serialize(serializer)
        }
    }
}

impl<'de> Deserialize<'de> for Uri {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let string = String::deserialize(deserializer)?;
        url::Url::parse(&string)
            .map(Uri)
            .map_err(|error| Error::custom(error.to_string()))
    }
}

impl Ord for Uri {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.as_str().cmp(other.as_str())
    }
}

impl PartialOrd for Uri {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl FromStr for Uri {
    type Err = url::ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        url::Url::parse(s).map(Self)
    }
}

impl Uri {
    /// Create a URI from a file path.
    /// 
    /// This is a convenience method that converts a file path to a file:// URI.
    /// On Windows, this handles drive letters and backslashes appropriately.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use lsp_types::Uri;
    /// 
    /// let uri = Uri::from_file_path("/Users/test/file.txt").unwrap();
    /// assert_eq!(uri.scheme(), "file");
    /// ```
    pub fn from_file_path<P: AsRef<std::path::Path>>(path: P) -> Result<Self, ()> {
        url::Url::from_file_path(path).map(Self)
    }
}

impl Deref for Uri {
    type Target = url::Url;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl PartialEq for Uri {
    fn eq(&self, other: &Self) -> bool {
        self.as_str() == other.as_str()
    }
}

impl Eq for Uri {}

impl Hash for Uri {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.as_str().hash(state)
    }
}

impl Display for Uri {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Display the original URL without bracket encoding
        self.0.fmt(f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;
    use std::collections::HashMap;
    use crate::{TextDocumentIdentifier, Location, Range, Position, WorkspaceEdit, TextEdit};

    #[test]
    fn test_from_str_and_deserialize_consistency() {
        let test_cases = vec![
            "file:///Users/test/[slug].tsx",
            "file:///Users/test/blog/[id]/[slug].tsx", 
            "file:///Users/test/[[...slug]].tsx",
            "file:///Users/test/normal.tsx",
            "https://example.com/[path].html",
        ];

        for uri_str in test_cases {
            // Create URI using from_str
            let from_str_uri = Uri::from_str(uri_str).expect("Should parse with from_str");

            // Create URI using deserialize
            let json_str = format!("\"{}\"", uri_str);
            let deserialized_uri: Uri = serde_json::from_str(&json_str).expect("Should deserialize");

            // They should be equal
            assert_eq!(from_str_uri, deserialized_uri, 
                "from_str and deserialize should produce the same Uri for: {}", uri_str);

            // Both should have the same underlying URL
            assert_eq!(from_str_uri.as_str(), deserialized_uri.as_str(),
                "Both should have the same underlying URL string for: {}", uri_str);
        }
    }

    #[test]
    fn test_bracket_percent_encoding_issue() {
        // This demonstrates the actual issue: brackets should be percent-encoded in URIs for LSP
        let file_path_with_brackets = "file:///Users/test/blog/[slug].tsx";

        // What the URI does with bracket encoding
        let uri = Uri::from_str(file_path_with_brackets).expect("Should parse URI");

        // When we serialize this with serde, it should percent-encode brackets
        let doc_id = TextDocumentIdentifier::new(uri.clone());
        let serialized = serde_json::to_string(&doc_id).expect("Should serialize");

        // The issue is that LSP expects percent-encoded brackets
        // [ should become %5B and ] should become %5D
        let expected_encoded = r#"{"uri":"file:///Users/test/blog/%5Bslug%5D.tsx"}"#;

        // Show that we now have correct percent-encoding
        assert_eq!(
            serialized, expected_encoded,
            "Implementation should now percent-encode brackets"
        );
    }

    #[test]
    fn test_uri_behavior_with_brackets() {
        // Test how the Uri handles different ways of creating URIs with brackets

        // Method 1: Parse a string with unencoded brackets
        let uri1 = Uri::from_str("file:///Users/test/[slug].tsx").expect("Should parse");

        // Method 2: Build URI with encoded brackets
        let uri2 = Uri::from_str("file:///Users/test/%5Bslug%5D.tsx").expect("Should parse");

        // Show what happens during serialization
        let doc1 = TextDocumentIdentifier::new(uri1);
        let doc2 = TextDocumentIdentifier::new(uri2.clone());

        let ser1 = serde_json::to_string(&doc1).expect("Should serialize");
        let ser2 = serde_json::to_string(&doc2).expect("Should serialize");

        // Both should serialize with encoded brackets
        assert!(
            ser1.contains("%5B") && ser1.contains("%5D"),
            "Serialized URI should contain encoded brackets"
        );
        assert!(
            ser2.contains("%5B") && ser2.contains("%5D"),
            "Serialized URI should contain encoded brackets"
        );
    }

    #[test]
    fn test_uri_with_brackets_serialization() {
        // Create a URI with brackets like Fresh framework uses: [slug].tsx
        let file_path = "file:///Users/test/project/routes/blog/[slug].tsx";
        let uri = Uri::from_str(file_path).expect("Should parse URI with brackets");

        // Test serialization of TextDocumentIdentifier
        let doc_id = TextDocumentIdentifier::new(uri.clone());
        let serialized = serde_json::to_string(&doc_id).expect("Should serialize");

        // Test that the serialized JSON contains properly encoded brackets
        assert!(
            serialized.contains("%5B"),
            "Serialized JSON should contain encoded opening bracket"
        );
        assert!(
            serialized.contains("%5D"),
            "Serialized JSON should contain encoded closing bracket"
        );
        assert!(
            !serialized.contains("[slug]"),
            "Serialized JSON should not contain unencoded brackets"
        );

        // Test deserialization - should handle encoded URLs correctly
        let deserialized: TextDocumentIdentifier =
            serde_json::from_str(&serialized).expect("Should deserialize");
        // The deserialized URI will have encoded brackets in the underlying URL
        let expected_encoded_uri =
            Uri::from_str("file:///Users/test/project/routes/blog/%5Bslug%5D.tsx")
                .expect("Should parse encoded URI");
        assert_eq!(
            deserialized.uri, expected_encoded_uri,
            "Deserialized URI should have encoded brackets"
        );

        // Test Location with brackets
        let location = Location::new(
            uri.clone(),
            Range::new(Position::new(0, 0), Position::new(0, 10)),
        );
        let location_serialized = serde_json::to_string(&location).expect("Should serialize Location");

        let location_deserialized: Location =
            serde_json::from_str(&location_serialized).expect("Should deserialize Location");
        // Check that Location serialization also contains encoded brackets
        assert!(
            location_serialized.contains("%5B"),
            "Location serialization should contain encoded brackets"
        );
        assert_eq!(
            location.range, location_deserialized.range,
            "Ranges should match"
        );
    }

    #[test]
    fn test_workspace_edit_with_bracket_uris() {
        // Test WorkspaceEdit which uses the custom url_map serializer
        let file_path = "file:///Users/test/project/routes/blog/[slug].tsx";
        let uri = Uri::from_str(file_path).expect("Should parse URI with brackets");

        let mut changes = HashMap::new();
        changes.insert(
            uri.clone(),
            vec![TextEdit::new(
                Range::new(Position::new(0, 0), Position::new(0, 5)),
                "Hello".to_string(),
            )],
        );

        let workspace_edit = WorkspaceEdit::new(changes);
        let serialized =
            serde_json::to_string(&workspace_edit).expect("Should serialize WorkspaceEdit");

        // Verify encoded brackets in serialized form
        assert!(
            serialized.contains("%5B"),
            "Serialized WorkspaceEdit should contain encoded brackets"
        );
        assert!(
            serialized.contains("%5D"),
            "Serialized WorkspaceEdit should contain encoded brackets"
        );

        // Test deserialization
        let deserialized: WorkspaceEdit =
            serde_json::from_str(&serialized).expect("Should deserialize WorkspaceEdit");

        // Check that the URI with brackets is preserved
        if let Some(changes) = deserialized.changes {
            // After serialization and deserialization, the URI will have encoded brackets
            let expected_encoded_uri =
                Uri::from_str("file:///Users/test/project/routes/blog/%5Bslug%5D.tsx")
                    .expect("Should parse encoded URI");
            assert!(
                changes.contains_key(&expected_encoded_uri),
                "Should contain URI with encoded brackets"
            );
            assert_eq!(changes.len(), 1, "Should have exactly one change");
        } else {
            panic!("WorkspaceEdit should have changes");
        }
    }

    #[test]
    fn test_uri_serialization_roundtrip() {
        let test_cases = vec![
            "file:///Users/test/[slug].tsx",
            "file:///Users/test/blog/[id]/[slug].tsx",
            "file:///Users/test/[[...slug]].tsx",
            "file:///Users/test/[category]/[...slug].tsx",
        ];

        for file_path in test_cases {
            let uri = Uri::from_str(file_path).expect(&format!("Should parse URI: {}", file_path));

            // Direct URI serialization
            let uri_serialized = serde_json::to_string(&uri).expect("Should serialize URI");
            let uri_deserialized: Uri =
                serde_json::from_str(&uri_serialized).expect("Should deserialize URI");

            // Test that serialized form contains encoded brackets
            if file_path.contains('[') || file_path.contains(']') {
                assert!(
                    uri_serialized.contains("%5B"),
                    "Serialized URI should contain encoded opening brackets when original has brackets"
                );
                assert!(
                    uri_serialized.contains("%5D"),
                    "Serialized URI should contain encoded closing brackets when original has brackets"
                );
            }

            // After round-trip, URIs with brackets will have encoded brackets
            // This is correct behavior for LSP compliance
            if file_path.contains('[') || file_path.contains(']') {
                // The deserialized URI should have encoded brackets
                assert!(
                    uri_deserialized.as_str().contains("%5B")
                        && uri_deserialized.as_str().contains("%5D"),
                    "Round-trip URI should have encoded brackets"
                );
            } else {
                assert_eq!(
                    uri, uri_deserialized,
                    "URI without brackets should round-trip exactly"
                );
            }
        }
    }

    #[test]
    fn test_from_file_path() {
        // Test basic file path conversion
        let uri = Uri::from_file_path("/Users/test/file.txt").expect("Should convert file path");
        assert_eq!(uri.scheme(), "file");
        assert!(uri.as_str().starts_with("file:///"));
        assert!(uri.as_str().contains("file.txt"));
    }

    #[test]
    fn test_from_file_path_with_brackets() {
        use std::path::Path;

        // Test file path with brackets
        let path = Path::new("/Users/test/[slug].tsx");
        let uri = Uri::from_file_path(path).expect("Should convert file path with brackets");
        
        // The URI should be created successfully
        assert_eq!(uri.scheme(), "file");
        
        // Test serialization - brackets should be encoded
        let serialized = serde_json::to_string(&uri).expect("Should serialize");
        assert!(
            serialized.contains("%5B") && serialized.contains("%5D"),
            "Serialized URI from file path should encode brackets"
        );
    }

    #[test]
    fn test_from_file_path_consistency_with_from_str() {
        use std::path::Path;

        let test_paths = vec![
            "/Users/test/file.txt",
            "/Users/test/[slug].tsx",
            "/Users/test/blog/[id]/[slug].tsx",
        ];

        for path_str in test_paths {
            let path = Path::new(path_str);
            
            // Create URI from file path
            let uri_from_path = Uri::from_file_path(path).expect("Should convert file path");
            
            // Create URI from the resulting file:// URL string
            let file_url = uri_from_path.as_str();
            let uri_from_str = Uri::from_str(file_url).expect("Should parse file URL");
            
            // They should be equal
            assert_eq!(uri_from_path, uri_from_str, 
                "URI from file path and from string should be equal for: {}", path_str);
                
            // Both should serialize the same way (with encoded brackets if present)
            let serialized_from_path = serde_json::to_string(&uri_from_path).expect("Should serialize");
            let serialized_from_str = serde_json::to_string(&uri_from_str).expect("Should serialize");
            assert_eq!(serialized_from_path, serialized_from_str,
                "Serialization should be identical for: {}", path_str);
        }
    }

    #[test]
    fn test_display_shows_original_url() {
        // Test that Display shows the original URL without bracket encoding
        let test_cases = vec![
            ("file:///Users/test/[slug].tsx", "[slug].tsx"),
            ("file:///Users/test/blog/[id]/[slug].tsx", "[id]/[slug].tsx"),
            ("file:///Users/test/[[...slug]].tsx", "[[...slug]].tsx"),
            ("file:///Users/test/normal.tsx", "normal.tsx"),
            ("https://example.com/[path].html", "[path].html"),
        ];

        for (uri_str, expected_part) in test_cases {
            let uri = Uri::from_str(uri_str).expect("Should parse URI");
            
            // Display should show the original URL with unencoded brackets
            let display_str = uri.to_string();
            assert!(
                display_str.contains(expected_part),
                "Display should contain unencoded brackets: {} should contain {}",
                display_str, expected_part
            );
            
            // Display should NOT contain encoded brackets
            if expected_part.contains('[') || expected_part.contains(']') {
                assert!(
                    !display_str.contains("%5B") && !display_str.contains("%5D"),
                    "Display should not contain encoded brackets: {}", display_str
                );
            }
        }
    }

    #[test] 
    fn test_display_vs_serialization() {
        // Test the difference between Display (original URL) and serialization (encoded brackets)
        let uri_str = "file:///Users/test/[slug].tsx";
        let uri = Uri::from_str(uri_str).expect("Should parse URI");

        // Display should show unencoded brackets
        let display_str = uri.to_string();
        assert!(display_str.contains("[slug]"));
        assert!(!display_str.contains("%5B"));

        // Serialization should show encoded brackets
        let serialized = serde_json::to_string(&uri).expect("Should serialize");
        assert!(serialized.contains("%5B") && serialized.contains("%5D"));
        assert!(!serialized.contains("[slug]"));

        // They should be different for URIs with brackets
        assert_ne!(format!("\"{}\"", display_str), serialized,
            "Display and serialization should differ for URIs with brackets");
    }
}
