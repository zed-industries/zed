/// Trait for recursively merging settings structures.
///
/// This trait allows settings objects to be merged from optional sources,
/// where `None` values are ignored and `Some` values override existing values.
pub trait MergeFrom {
    /// Merge from an optional source of the same type.
    /// If `other` is `None`, no changes are made.
    /// If `other` is `Some(value)`, fields from `value` are merged into `self`.
    fn merge_from(&mut self, other: Option<&Self>);
}
// Implementations for basic types that simply overwrite if Some value is provided
impl MergeFrom for String {
    fn merge_from(&mut self, other: Option<&Self>) {
        if let Some(value) = other {
            *self = value.clone();
        }
    }
}

impl MergeFrom for i32 {
    fn merge_from(&mut self, other: Option<&Self>) {
        if let Some(value) = other {
            *self = *value;
        }
    }
}

impl MergeFrom for i64 {
    fn merge_from(&mut self, other: Option<&Self>) {
        if let Some(value) = other {
            *self = *value;
        }
    }
}

impl MergeFrom for u32 {
    fn merge_from(&mut self, other: Option<&Self>) {
        if let Some(value) = other {
            *self = *value;
        }
    }
}

impl MergeFrom for u64 {
    fn merge_from(&mut self, other: Option<&Self>) {
        if let Some(value) = other {
            *self = *value;
        }
    }
}

impl MergeFrom for bool {
    fn merge_from(&mut self, other: Option<&Self>) {
        if let Some(value) = other {
            *self = *value;
        }
    }
}

impl MergeFrom for f64 {
    fn merge_from(&mut self, other: Option<&Self>) {
        if let Some(value) = other {
            *self = *value;
        }
    }
}

impl MergeFrom for f32 {
    fn merge_from(&mut self, other: Option<&Self>) {
        if let Some(value) = other {
            *self = *value;
        }
    }
}
// Implementations for collections that extend/merge their contents
impl<K, V> MergeFrom for collections::HashMap<K, V>
where
    K: Clone + std::hash::Hash + Eq,
    V: Clone,
{
    fn merge_from(&mut self, other: Option<&Self>) {
        if let Some(other_map) = other {
            self.extend(other_map.clone());
        }
    }
}

impl<K, V> MergeFrom for collections::IndexMap<K, V>
where
    K: Clone + std::hash::Hash + Eq,
    V: Clone,
{
    fn merge_from(&mut self, other: Option<&Self>) {
        if let Some(other_map) = other {
            self.extend(other_map.clone());
        }
    }
}
