//! Implements an elasticlunr.js inverted index. Most users do not need to use this module directly.

use std::collections::BTreeMap;

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq)]
struct TermFrequency {
    #[serde(rename = "tf")]
    pub term_freq: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Default)]
struct IndexItem {
    pub docs: BTreeMap<String, TermFrequency>,
    #[serde(rename = "df")]
    pub doc_freq: i64,
    #[serde(flatten, serialize_with = "IndexItem::serialize")]
    pub children: BTreeMap<char, IndexItem>,
}

impl IndexItem {
    fn new() -> Self {
        Default::default()
    }

    fn serialize<S>(map: &BTreeMap<char, IndexItem>, ser: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        use serde::ser::SerializeMap;

        let mut ser_map = ser.serialize_map(Some(map.len()))?;
        let mut buf = [0u8; 4];
        for (key, value) in map {
            let key = key.encode_utf8(&mut buf);
            ser_map.serialize_entry(key, value)?;
        }
        ser_map.end()
    }

    fn add_token(&mut self, doc_ref: &str, token: &str, term_freq: f64) {
        let mut iter = token.chars();
        if let Some(character) = iter.next() {
            let mut item = self
                .children
                .entry(character)
                .or_insert_with(IndexItem::new);

            for character in iter {
                let tmp = item;
                item = tmp.children.entry(character).or_insert_with(IndexItem::new);
            }

            if !item.docs.contains_key(doc_ref) {
                item.doc_freq += 1;
            }
            item.docs
                .insert(doc_ref.into(), TermFrequency { term_freq });
        }
    }

    fn get_node(&self, token: &str) -> Option<&IndexItem> {
        let mut root = self;
        for ch in token.chars() {
            if let Some(item) = root.children.get(&ch) {
                root = item;
            } else {
                return None;
            }
        }

        Some(root)
    }

    fn remove_token(&mut self, doc_ref: &str, token: &str) {
        let mut iter = token.char_indices();
        if let Some((_, ch)) = iter.next() {
            if let Some(item) = self.children.get_mut(&ch) {
                if let Some((idx, _)) = iter.next() {
                    item.remove_token(doc_ref, &token[idx..]);
                } else if item.docs.contains_key(doc_ref) {
                    item.docs.remove(doc_ref);
                    item.doc_freq -= 1;
                }
            }
        }
    }
}

/// Implements an elasticlunr.js inverted index. Most users do not need to use this type directly.
#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct InvertedIndex {
    root: IndexItem,
}

impl InvertedIndex {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn add_token(&mut self, doc_ref: &str, token: &str, term_freq: f64) {
        self.root.add_token(doc_ref, token, term_freq)
    }

    pub fn has_token(&self, token: &str) -> bool {
        self.root.get_node(token).map_or(false, |_| true)
    }

    pub fn remove_token(&mut self, doc_ref: &str, token: &str) {
        self.root.remove_token(doc_ref, token)
    }

    pub fn get_docs(&self, token: &str) -> Option<BTreeMap<String, f64>> {
        self.root.get_node(token).map(|node| {
            node.docs
                .iter()
                .map(|(k, &v)| (k.clone(), v.term_freq))
                .collect()
        })
    }

    pub fn get_term_frequency(&self, doc_ref: &str, token: &str) -> f64 {
        self.root
            .get_node(token)
            .and_then(|node| node.docs.get(doc_ref))
            .map_or(0., |docs| docs.term_freq)
    }

    pub fn get_doc_frequency(&self, token: &str) -> i64 {
        self.root.get_node(token).map_or(0, |node| node.doc_freq)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adding_token() {
        let mut inverted_index = InvertedIndex::new();
        let token = "foo";

        inverted_index.add_token("123", token, 1.);
        assert_eq!(inverted_index.get_doc_frequency("foo"), 1);
        assert_eq!(inverted_index.get_term_frequency("123", "foo"), 1.);
    }

    #[test]
    fn has_token() {
        let mut inverted_index = InvertedIndex::new();
        let token = "foo";

        inverted_index.add_token("123", token, 1.);
        assert!(inverted_index.has_token(token));
        assert!(inverted_index.has_token("fo"));
        assert!(inverted_index.has_token("f"));

        assert!(!inverted_index.has_token("bar"));
        assert!(!inverted_index.has_token("foo "));
        assert!(!inverted_index.has_token("foo  "))
    }

    #[test]
    fn adding_another_document_to_the_token() {
        let mut inverted_index = InvertedIndex::new();
        let token = "foo";

        inverted_index.add_token("123", token, 1.);
        inverted_index.add_token("456", token, 1.);

        assert_eq!(inverted_index.get_term_frequency("123", "foo"), 1.);
        assert_eq!(inverted_index.get_term_frequency("456", "foo"), 1.);
        assert_eq!(inverted_index.get_doc_frequency("foo"), 2);
    }

    #[test]
    fn df_of_nonexistant_token() {
        let mut inverted_index = InvertedIndex::new();
        let token = "foo";

        inverted_index.add_token("123", token, 1.);
        inverted_index.add_token("456", token, 1.);

        assert_eq!(inverted_index.get_doc_frequency("foo"), 2);
        assert_eq!(inverted_index.get_doc_frequency("fox"), 0);
    }

    #[test]
    fn adding_existing_doc() {
        let mut inverted_index = InvertedIndex::new();
        let token = "foo";

        inverted_index.add_token("123", token, 1.);
        inverted_index.add_token("456", token, 1.);
        inverted_index.add_token("456", token, 100.);

        assert_eq!(inverted_index.get_term_frequency("456", "foo"), 100.);
        assert_eq!(inverted_index.get_doc_frequency("foo"), 2);
    }

    #[test]
    fn checking_token_exists_in() {
        let mut inverted_index = InvertedIndex::new();
        let token = "foo";

        inverted_index.add_token("123", token, 1.);

        assert!(inverted_index.has_token(token));
    }

    #[test]
    fn checking_if_a_token_does_not_exist() {
        let mut inverted_index = InvertedIndex::new();
        let token = "foo";

        inverted_index.add_token("123", token, 1.);
        assert!(!inverted_index.has_token("fooo"));
        assert!(!inverted_index.has_token("bar"));
        assert!(!inverted_index.has_token("fof"));
    }

    #[test]
    fn retrieving_items() {
        let mut inverted_index = InvertedIndex::new();
        let token = "foo";

        inverted_index.add_token("123", token, 1.);
        assert_eq!(
            inverted_index.get_docs(token).unwrap(),
            btreemap! {
                "123".into() => 1.
            }
        );

        assert_eq!(inverted_index.get_docs(""), Some(BTreeMap::new()));

        inverted_index.add_token("234", "boo", 100.);
        inverted_index.add_token("345", "too", 101.);

        assert_eq!(
            inverted_index.get_docs(token).unwrap(),
            btreemap! {
                "123".into() => 1.
            }
        );

        inverted_index.add_token("234", token, 100.);
        inverted_index.add_token("345", token, 101.);

        assert_eq!(
            inverted_index.get_docs(token).unwrap(),
            btreemap! {
                "123".into() => 1.,
                "234".into() => 100.,
                "345".into() => 101.,
            }
        );
    }

    #[test]
    fn retrieving_nonexistant_items() {
        let inverted_index = InvertedIndex::new();

        assert_eq!(inverted_index.get_docs("foo"), None);
        assert_eq!(inverted_index.get_docs("fox"), None);
    }

    #[test]
    fn df_of_items() {
        let mut inverted_index = InvertedIndex::new();

        inverted_index.add_token("123", "foo", 1.);
        inverted_index.add_token("456", "foo", 1.);
        inverted_index.add_token("789", "bar", 1.);

        assert_eq!(inverted_index.get_doc_frequency("foo"), 2);
        assert_eq!(inverted_index.get_doc_frequency("bar"), 1);
        assert_eq!(inverted_index.get_doc_frequency("baz"), 0);
        assert_eq!(inverted_index.get_doc_frequency("ba"), 0);
        assert_eq!(inverted_index.get_doc_frequency("b"), 0);
        assert_eq!(inverted_index.get_doc_frequency("fo"), 0);
        assert_eq!(inverted_index.get_doc_frequency("f"), 0);
    }

    #[test]
    fn removing_document_from_token() {
        let mut inverted_index = InvertedIndex::new();
        assert_eq!(inverted_index.get_docs("foo"), None);

        inverted_index.add_token("123", "foo", 1.);
        assert_eq!(
            inverted_index.get_docs("foo").unwrap(),
            btreemap! {
                "123".into() => 1.,
            }
        );

        inverted_index.remove_token("123", "foo");
        assert_eq!(inverted_index.get_docs("foo"), Some(BTreeMap::new()));
        assert_eq!(inverted_index.get_doc_frequency("foo"), 0);
        assert_eq!(inverted_index.has_token("foo"), true);
    }

    #[test]
    fn removing_nonexistant_document() {
        let mut inverted_index = InvertedIndex::new();

        inverted_index.add_token("123", "foo", 1.);
        inverted_index.add_token("567", "bar", 1.);
        inverted_index.remove_token("foo", "456");

        assert_eq!(
            inverted_index.get_docs("foo").unwrap(),
            btreemap! {
                "123".into() => 1.
            }
        );
        assert_eq!(inverted_index.get_doc_frequency("foo"), 1);
    }

    #[test]
    fn removing_documet_nonexistant_key() {
        let mut inverted_index = InvertedIndex::new();

        inverted_index.remove_token("123", "foo");
        assert!(!inverted_index.has_token("foo"));
        assert_eq!(inverted_index.get_doc_frequency("foo"), 0);
    }

    #[test]
    fn get_term_frequency() {
        let mut inverted_index = InvertedIndex::new();
        let token = "foo";

        inverted_index.add_token("123", token, 2.);
        inverted_index.add_token("456", token, 3.);

        assert_eq!(inverted_index.get_term_frequency("123", token), 2.);
        assert_eq!(inverted_index.get_term_frequency("456", token), 3.);
        assert_eq!(inverted_index.get_term_frequency("789", token), 0.);
    }

    #[test]
    fn get_term_frequency_nonexistant_token() {
        let mut inverted_index = InvertedIndex::new();
        let token = "foo";

        inverted_index.add_token("123", token, 2.);
        inverted_index.add_token("456", token, 3.);

        assert_eq!(inverted_index.get_term_frequency("123", "ken"), 0.);
        assert_eq!(inverted_index.get_term_frequency("456", "ken"), 0.);
    }

    #[test]
    fn get_term_frequency_nonexistant_docref() {
        let mut inverted_index = InvertedIndex::new();
        let token = "foo";

        inverted_index.add_token("123", token, 2.);
        inverted_index.add_token("456", token, 3.);

        assert_eq!(inverted_index.get_term_frequency(token, "12"), 0.);
        assert_eq!(inverted_index.get_term_frequency(token, "23"), 0.);
        assert_eq!(inverted_index.get_term_frequency(token, "45"), 0.);
    }

    #[test]
    fn get_term_frequency_nonexistant_token_and_docref() {
        let mut inverted_index = InvertedIndex::new();
        let token = "foo";

        inverted_index.add_token("123", token, 2.);
        inverted_index.add_token("456", token, 3.);

        assert_eq!(inverted_index.get_term_frequency("token", "1"), 0.);
        assert_eq!(inverted_index.get_term_frequency("abc", "2"), 0.);
        assert_eq!(inverted_index.get_term_frequency("fo", "123"), 0.);
    }
}
