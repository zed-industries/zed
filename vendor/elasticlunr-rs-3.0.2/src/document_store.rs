//! Implements an elasticlunr.js document store. Most users do not need to use this module directly.

use std::collections::BTreeMap;

/// The document store saves the complete text of each item saved to the index, if enabled.
/// Most users do not need to use this type directly.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DocumentStore {
    pub save: bool,
    pub docs: BTreeMap<String, BTreeMap<String, String>>,
    pub doc_info: BTreeMap<String, BTreeMap<String, usize>>,
    // Redundant with docs.len(), but needed for serialization
    pub length: usize,
}

impl DocumentStore {
    pub fn new(save: bool) -> Self {
        DocumentStore {
            save,
            docs: BTreeMap::new(),
            doc_info: BTreeMap::new(),
            length: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.docs.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn is_stored(&self) -> bool {
        self.save
    }

    pub fn has_doc(&self, doc_ref: &str) -> bool {
        self.docs.contains_key(doc_ref)
    }

    pub fn add_doc(&mut self, doc_ref: &str, doc: BTreeMap<String, String>) {
        if !self.has_doc(doc_ref) {
            self.length += 1;
        }

        self.docs.insert(
            doc_ref.into(),
            if self.save { doc } else { BTreeMap::new() },
        );
    }

    pub fn get_doc(&self, doc_ref: &str) -> Option<BTreeMap<String, String>> {
        self.docs.get(doc_ref).cloned()
    }

    pub fn remove_doc(&mut self, doc_ref: &str) {
        if self.has_doc(doc_ref) {
            self.length -= 1;
        }

        self.docs.remove(doc_ref);
    }

    pub fn add_field_length(&mut self, doc_ref: &str, field: &str, length: usize) {
        self.doc_info
            .entry(doc_ref.into())
            .or_insert_with(BTreeMap::new)
            .insert(field.into(), length);
    }

    pub fn get_field_length(&self, doc_ref: &str, field: &str) -> usize {
        if self.has_doc(doc_ref) {
            self.doc_info
                .get(doc_ref)
                .and_then(|e| e.get(field))
                .cloned()
                .unwrap_or(0)
        } else {
            0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_doc_tokens() {
        let mut store = DocumentStore::new(true);
        let doc = btreemap! { "title".into() => "eggs bread".into() };

        store.add_doc("1", doc.clone());
        assert_eq!(store.get_doc("1").unwrap(), doc);
    }

    #[test]
    fn create_doc_no_store() {
        let mut store = DocumentStore::new(false);
        let doc = btreemap! { "title".into() => "eggs bread".into() };

        store.add_doc("1", doc);
        assert_eq!(store.len(), 1);
        assert_eq!(store.is_stored(), false);
        assert_eq!(store.has_doc("1"), true);
    }

    #[test]
    fn add_doc_no_store() {
        let mut store = DocumentStore::new(false);
        let doc1 = btreemap! { "title".into() => "eggs bread".into() };
        let doc2 = btreemap! { "title".into() => "hello world".into() };

        store.add_doc("1", doc1);
        store.add_doc("2", doc2);
        assert_eq!(store.len(), 2);
        assert_eq!(store.is_stored(), false);
        assert_eq!(store.has_doc("1"), true);
        assert_eq!(store.has_doc("2"), true);
    }

    #[test]
    fn is_stored_true() {
        let store = DocumentStore::new(true);
        assert_eq!(store.is_stored(), true);
    }

    #[test]
    fn is_stored_false() {
        let store = DocumentStore::new(false);
        assert_eq!(store.is_stored(), false);
    }

    #[test]
    fn get_doc_no_store() {
        let mut store = DocumentStore::new(false);
        let doc1 = btreemap! { "title".into() => "eggs bread".into() };
        let doc2 = btreemap! { "title".into() => "hello world".into() };

        store.add_doc("1", doc1);
        store.add_doc("2", doc2);
        assert_eq!(store.len(), 2);
        assert_eq!(store.is_stored(), false);
        assert_eq!(store.get_doc("1").unwrap(), BTreeMap::new());
        assert_eq!(store.get_doc("2").unwrap(), BTreeMap::new());
    }

    #[test]
    fn get_nonexistant_doc_no_store() {
        let mut store = DocumentStore::new(false);
        let doc1 = btreemap! { "title".into() => "eggs bread".into() };
        let doc2 = btreemap! { "title".into() => "hello world".into() };

        store.add_doc("1", doc1);
        store.add_doc("2", doc2);
        assert_eq!(store.len(), 2);
        assert_eq!(store.is_stored(), false);
        assert_eq!(store.get_doc("6"), None);
        assert_eq!(store.get_doc("2").unwrap(), BTreeMap::new());
    }

    #[test]
    fn remove_doc_no_store() {
        let mut store = DocumentStore::new(false);
        let doc1 = btreemap! { "title".into() => "eggs bread".into() };
        let doc2 = btreemap! { "title".into() => "hello world".into() };

        store.add_doc("1", doc1);
        store.add_doc("2", doc2);
        store.remove_doc("1");
        assert_eq!(store.len(), 1);
        assert_eq!(store.is_stored(), false);
        assert_eq!(store.get_doc("2").unwrap(), BTreeMap::new());
        assert_eq!(store.get_doc("1"), None);
    }

    #[test]
    fn remove_nonexistant_doc() {
        let mut store = DocumentStore::new(false);
        let doc1 = btreemap! { "title".into() => "eggs bread".into() };
        let doc2 = btreemap! { "title".into() => "hello world".into() };

        store.add_doc("1", doc1);
        store.add_doc("2", doc2);
        store.remove_doc("8");
        assert_eq!(store.len(), 2);
        assert_eq!(store.is_stored(), false);
        assert_eq!(store.get_doc("2").unwrap(), BTreeMap::new());
        assert_eq!(store.get_doc("1").unwrap(), BTreeMap::new());
    }

    #[test]
    fn get_num_docs() {
        let mut store = DocumentStore::new(true);

        assert_eq!(store.len(), 0);
        store.add_doc("1", btreemap! { "title".into() => "eggs bread".into() });
        assert_eq!(store.len(), 1);
    }

    #[test]
    fn get_doc() {
        let mut store = DocumentStore::new(true);

        assert_eq!(store.len(), 0);
        store.add_doc("1", btreemap! { "title".into() => "eggs bread".into() });
        assert_eq!(
            store.get_doc("1").unwrap(),
            btreemap! { "title".into() => "eggs bread".into() }
        );
    }

    #[test]
    fn get_doc_many_fields() {
        let mut store = DocumentStore::new(true);

        assert_eq!(store.len(), 0);
        store.add_doc(
            "1",
            btreemap! {
                "title".into() => "eggs bread".into()
            },
        );
        store.add_doc(
            "2",
            btreemap! {
                "title".into() => "boo bar".into()
            },
        );
        store.add_doc(
            "3",
            btreemap! {
                "title".into() => "oracle".into(),
                "body".into() => "Oracle is demonspawn".into()
            },
        );
        assert_eq!(
            store.get_doc("3").unwrap(),
            btreemap! {
                "title".into() => "oracle".into(),
                "body".into() => "Oracle is demonspawn".into()
            }
        );
        assert_eq!(store.len(), 3);
    }

    #[test]
    fn get_nonexistant_doc() {
        let mut store = DocumentStore::new(true);

        assert_eq!(store.len(), 0);
        store.add_doc(
            "1",
            btreemap! {
                "title".into() => "eggs bread".into()
            },
        );
        store.add_doc(
            "2",
            btreemap! {
                "title".into() => "boo bar".into()
            },
        );
        store.add_doc(
            "3",
            btreemap! {
                "title".into() => "oracle".into(),
                "body".into() => "Oracle is demonspawn".into()
            },
        );
        assert_eq!(store.get_doc("4"), None);
        assert_eq!(store.get_doc("0"), None);
        assert_eq!(store.len(), 3);
    }

    #[test]
    fn check_store_has_key() {
        let mut store = DocumentStore::new(true);

        assert!(!store.has_doc("foo"));
        store.add_doc("foo", btreemap! { "title".into() => "eggs bread".into() });
        assert!(store.has_doc("foo"));
    }

    #[test]
    fn remove_doc() {
        let mut store = DocumentStore::new(true);

        store.add_doc("foo", btreemap! { "title".into() => "eggs bread".into() });
        assert!(store.has_doc("foo"));
        assert_eq!(store.len(), 1);
        store.remove_doc("foo");
        assert!(!store.has_doc("foo"));
        assert_eq!(store.len(), 0);
    }

    #[test]
    fn remove_nonexistant_store() {
        let mut store = DocumentStore::new(true);

        store.add_doc("foo", btreemap! { "title".into() => "eggs bread".into() });
        assert!(store.has_doc("foo"));
        assert_eq!(store.len(), 1);
        store.remove_doc("bar");
        assert!(store.has_doc("foo"));
        assert_eq!(store.len(), 1);
    }

    #[test]
    fn add_field_len() {
        let mut store = DocumentStore::new(true);

        store.add_doc("foo", btreemap! { "title".into() => "eggs bread".into() });
        store.add_field_length("foo", "title", 2);
        assert_eq!(store.get_field_length("foo", "title"), 2);
    }

    #[test]
    fn add_field_length_multiple() {
        let mut store = DocumentStore::new(true);

        store.add_doc("foo", btreemap! { "title".into() => "eggs bread".into() });
        store.add_field_length("foo", "title", 2);
        store.add_field_length("foo", "body", 10);
        assert_eq!(store.get_field_length("foo", "title"), 2);
        assert_eq!(store.get_field_length("foo", "body"), 10);
    }
}
