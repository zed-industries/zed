use ahash::AHashMap;
use serde_json::{from_str, Value};
use std::sync::LazyLock;

pub(crate) type ContentMediaTypeCheckType = fn(&str) -> bool;

pub(crate) fn is_json(instance_string: &str) -> bool {
    from_str::<Value>(instance_string).is_ok()
}

pub(crate) static DEFAULT_CONTENT_MEDIA_TYPE_CHECKS: LazyLock<
    AHashMap<&'static str, ContentMediaTypeCheckType>,
> = LazyLock::new(|| {
    let mut map: AHashMap<&'static str, ContentMediaTypeCheckType> = AHashMap::with_capacity(1);
    map.insert("application/json", is_json);
    map
});
