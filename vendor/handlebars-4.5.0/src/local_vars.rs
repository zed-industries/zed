use std::collections::BTreeMap;

use serde_json::value::Value as Json;

#[derive(Default, Debug, Clone)]
pub(crate) struct LocalVars {
    first: Option<Json>,
    last: Option<Json>,
    index: Option<Json>,
    key: Option<Json>,

    extra: BTreeMap<String, Json>,
}

impl LocalVars {
    pub fn put(&mut self, key: &str, value: Json) {
        match key {
            "first" => self.first = Some(value),
            "last" => self.last = Some(value),
            "index" => self.index = Some(value),
            "key" => self.key = Some(value),
            _ => {
                self.extra.insert(key.to_owned(), value);
            }
        }
    }

    pub fn get(&self, key: &str) -> Option<&Json> {
        match key {
            "first" => self.first.as_ref(),
            "last" => self.last.as_ref(),
            "index" => self.index.as_ref(),
            "key" => self.key.as_ref(),
            _ => self.extra.get(key),
        }
    }
}
