/// Trait for recursively merging settings structures.
///
/// When Zed starts it loads settinsg from `default.json` to initialize
/// everything. These may be further refined by loading the user's settings,
/// and any settings profiles; and then further refined by loading any
/// local project settings.
///
/// The default behaviour of merging is:
/// * For objects with named keys (HashMap, structs, etc.). The values are merged deeply
///   (so if the default settings has languages.JSON.prettier.allowed = true, and the user's settings has
///    languages.JSON.tab_size = 4; the merged settings file will have both settings).
/// * For options, a None value is ignored, but Some values are merged recursively.
/// * For other types (including Vec), a merge overwrites the current value.
///
/// If you want to break the rules you can (e.g. ExtendingVec, or SaturatingBool).
#[allow(unused)]
pub trait MergeFrom {
    /// Merge from a source of the same type.
    fn merge_from(&mut self, other: &Self);

    /// Merge from an optional source of the same type.
    fn merge_from_option(&mut self, other: Option<&Self>) {
        if let Some(other) = other {
            self.merge_from(other);
        }
    }
}

macro_rules! merge_from_overwrites {
    ($($type:ty),+) => {
        $(
            impl MergeFrom for $type {
                fn merge_from(&mut self, other: &Self) {
                    *self = other.clone();
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
    char,
    std::num::NonZeroUsize,
    std::num::NonZeroU32,
    String,
    std::sync::Arc<str>,
    gpui::SharedString,
    std::path::PathBuf,
    gpui::Modifiers,
    gpui::FontFeatures,
    gpui::FontWeight
);

impl<T: Clone + MergeFrom> MergeFrom for Option<T> {
    fn merge_from(&mut self, other: &Self) {
        let Some(other) = other else {
            return;
        };
        if let Some(this) = self {
            this.merge_from(other);
        } else {
            self.replace(other.clone());
        }
    }
}

impl<T: Clone> MergeFrom for Vec<T> {
    fn merge_from(&mut self, other: &Self) {
        *self = other.clone()
    }
}

impl<T: MergeFrom> MergeFrom for Box<T> {
    fn merge_from(&mut self, other: &Self) {
        self.as_mut().merge_from(other.as_ref())
    }
}

// Implementations for collections that extend/merge their contents
impl<K, V> MergeFrom for collections::HashMap<K, V>
where
    K: Clone + std::hash::Hash + Eq,
    V: Clone + MergeFrom,
{
    fn merge_from(&mut self, other: &Self) {
        for (k, v) in other {
            if let Some(existing) = self.get_mut(k) {
                existing.merge_from(v);
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
    fn merge_from(&mut self, other: &Self) {
        for (k, v) in other {
            if let Some(existing) = self.get_mut(k) {
                existing.merge_from(v);
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
    fn merge_from(&mut self, other: &Self) {
        for (k, v) in other {
            if let Some(existing) = self.get_mut(k) {
                existing.merge_from(v);
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
    fn merge_from(&mut self, other: &Self) {
        for item in other {
            self.insert(item.clone());
        }
    }
}

impl<T> MergeFrom for collections::HashSet<T>
where
    T: Clone + std::hash::Hash + Eq,
{
    fn merge_from(&mut self, other: &Self) {
        for item in other {
            self.insert(item.clone());
        }
    }
}

impl MergeFrom for serde_json::Value {
    fn merge_from(&mut self, other: &Self) {
        match (self, other) {
            (serde_json::Value::Object(this), serde_json::Value::Object(other)) => {
                for (k, v) in other {
                    if let Some(existing) = this.get_mut(k) {
                        existing.merge_from(v);
                    } else {
                        this.insert(k.clone(), v.clone());
                    }
                }
            }
            (this, other) => *this = other.clone(),
        }
    }
}
