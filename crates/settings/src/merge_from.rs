use std::rc::Rc;

/// Trait for recursively merging settings structures.
///
/// This trait allows settings objects to be merged from optional sources,
/// where `None` values are ignored and `Some` values override existing values.
///
/// HashMaps, structs and similar types are merged by combining their contents key-wise,
/// but all other types (including Vecs) are last-write-wins.
/// (Though see also ExtendingVec and SaturatingBool)
#[allow(unused)]
pub trait MergeFrom {
    /// Merge from an optional source of the same type.
    /// If `other` is `None`, no changes are made.
    /// If `other` is `Some(value)`, fields from `value` are merged into `self`.
    fn merge_from(&mut self, other: Option<&Self>);
}

macro_rules! merge_from_overwrites {
    ($($type:ty),+) => {
        $(
            impl MergeFrom for $type {
                fn merge_from(&mut self, other: Option<&Self>) {
                    if let Some(value) = other {
                        *self = value.clone();
                    }
                }
            }
        )+
    }
}

merge_from_overwrites!(
    u16,
    u32,
    u64,
    usize,
    i16,
    i32,
    i64,
    bool,
    f64,
    f32,
    std::num::NonZeroUsize,
    std::num::NonZeroU32,
    String,
    std::sync::Arc<str>,
    gpui::SharedString,
    std::path::PathBuf,
    gpui::Modifiers,
    gpui::FontFeatures
);

impl<T: Clone> MergeFrom for Vec<T> {
    fn merge_from(&mut self, other: Option<&Self>) {
        if let Some(other) = other {
            *self = other.clone()
        }
    }
}

// Implementations for collections that extend/merge their contents
impl<K, V> MergeFrom for collections::HashMap<K, V>
where
    K: Clone + std::hash::Hash + Eq,
    V: Clone + MergeFrom,
{
    fn merge_from(&mut self, other: Option<&Self>) {
        let Some(other) = other else { return };
        for (k, v) in other {
            if let Some(existing) = self.get_mut(k) {
                existing.merge_from(Some(v));
            } else {
                self.insert(k.clone(), v.clone());
            }
        }
    }
}

impl<K, V> MergeFrom for collections::BTreeMap<K, V>
where
    K: Clone + std::hash::Hash + Eq + Ord,
    V: Clone + MergeFrom,
{
    fn merge_from(&mut self, other: Option<&Self>) {
        let Some(other) = other else { return };
        for (k, v) in other {
            if let Some(existing) = self.get_mut(k) {
                existing.merge_from(Some(v));
            } else {
                self.insert(k.clone(), v.clone());
            }
        }
    }
}

impl<K, V> MergeFrom for collections::IndexMap<K, V>
where
    K: std::hash::Hash + Eq + Clone,
    // Q: ?Sized + std::hash::Hash + collections::Equivalent<K> + Eq,
    V: Clone + MergeFrom,
{
    fn merge_from(&mut self, other: Option<&Self>) {
        let Some(other) = other else { return };
        for (k, v) in other {
            if let Some(existing) = self.get_mut(k) {
                existing.merge_from(Some(v));
            } else {
                self.insert(k.clone(), v.clone());
            }
        }
    }
}

impl<T> MergeFrom for collections::BTreeSet<T>
where
    T: Clone + Ord,
{
    fn merge_from(&mut self, other: Option<&Self>) {
        let Some(other) = other else { return };
        for item in other {
            self.insert(item.clone());
        }
    }
}

impl<T> MergeFrom for collections::HashSet<T>
where
    T: Clone + std::hash::Hash + Eq,
{
    fn merge_from(&mut self, other: Option<&Self>) {
        let Some(other) = other else { return };
        for item in other {
            self.insert(item.clone());
        }
    }
}

impl MergeFrom for serde_json::Value {
    fn merge_from(&mut self, other: Option<&Self>) {
        let Some(other) = other else { return };
        match (self, other) {
            (serde_json::Value::Object(this), serde_json::Value::Object(other)) => {
                for (k, v) in other {
                    if let Some(existing) = this.get_mut(k) {
                        existing.merge_from(other.get(k));
                    } else {
                        this.insert(k.clone(), v.clone());
                    }
                }
            }
            (this, other) => *this = other.clone(),
        }
    }
}

impl<T: MergeFrom + Clone> MergeFrom for Rc<T> {
    fn merge_from(&mut self, other: Option<&Self>) {
        let Some(other) = other else { return };
        let mut this: T = self.as_ref().clone();
        this.merge_from(Some(other.as_ref()));
        *self = Rc::new(this)
    }
}
