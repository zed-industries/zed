use crate::json::ToJson;
pub use pathfinder_color::*;
use serde_json::json;

impl ToJson for ColorU {
    fn to_json(&self) -> serde_json::Value {
        json!(format!("0x{:x}{:x}{:x}", self.r, self.g, self.b))
    }
}
