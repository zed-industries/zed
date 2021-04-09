pub use serde_json::*;

pub trait ToJson {
    fn to_json(&self) -> Value;
}

impl<T: ToJson> ToJson for Option<T> {
    fn to_json(&self) -> Value {
        if let Some(value) = self.as_ref() {
            value.to_json()
        } else {
            json!(null)
        }
    }
}
