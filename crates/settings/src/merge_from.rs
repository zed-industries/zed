use gpui::SharedString;
use std::sync::Arc;

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
    String,
    Arc<str>,
    SharedString,
    gpui::Modifiers,
);

/// Merge for Vec takes all the settings from other
impl<T: Clone> MergeFrom for Vec<T> {
    fn merge_from(&mut self, other: Option<&Self>) {
        if let Some(other) = other {
            *self = other.clone()
        }
    }
}

pub type ExtendedVec<T>(pub Vec<T>);
/// The default merge
impl<T: Clone> MergeFrom for Vec<T> {
    fn merge_from(&mut self, other: Option<&Self>) {
        let Some(other) = other else {return};
        self.extend(other.clone());
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
            if let Some(existing) = self.get_mut(&k) {
                existing.merge_from(Some(v));
            } else {
                self.insert(k.clone(), v.clone());
            }
        }
    }
}

impl<K, Q, V> MergeFrom for collections::IndexMap<K, V>
where
    K: std::hash::Hash + Eq + Clone,
    Q: ?Sized + std::hash::Hash + collections::Equivalent<K> + Eq,
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
