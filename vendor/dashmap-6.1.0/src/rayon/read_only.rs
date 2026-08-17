use crate::mapref::multiple::RefMulti;
use crate::rayon::map::Iter;
use crate::ReadOnlyView;
use core::hash::{BuildHasher, Hash};
use rayon::iter::IntoParallelIterator;

impl<K, V, S> IntoParallelIterator for ReadOnlyView<K, V, S>
where
    K: Send + Eq + Hash,
    V: Send,
    S: Send + Clone + BuildHasher,
{
    type Iter = super::map::OwningIter<K, V>;
    type Item = (K, V);

    fn into_par_iter(self) -> Self::Iter {
        super::map::OwningIter {
            shards: self.map.shards,
        }
    }
}

// This impl also enables `IntoParallelRefIterator::par_iter`
impl<'a, K, V, S> IntoParallelIterator for &'a ReadOnlyView<K, V, S>
where
    K: Send + Sync + Eq + Hash,
    V: Send + Sync,
    S: Send + Sync + Clone + BuildHasher,
{
    type Iter = Iter<'a, K, V>;
    type Item = RefMulti<'a, K, V>;

    fn into_par_iter(self) -> Self::Iter {
        Iter {
            shards: &self.map.shards,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::DashMap;
    use rayon::iter::{IntoParallelIterator, IntoParallelRefIterator, ParallelIterator};

    fn construct_sample_map() -> DashMap<i32, String> {
        let map = DashMap::new();

        map.insert(1, "one".to_string());

        map.insert(10, "ten".to_string());

        map.insert(27, "twenty seven".to_string());

        map.insert(45, "forty five".to_string());

        map
    }

    #[test]
    fn test_par_iter() {
        let map = construct_sample_map();

        let view = map.clone().into_read_only();

        view.par_iter().for_each(|entry| {
            let key = *entry.key();

            assert!(view.contains_key(&key));

            let map_entry = map.get(&key).unwrap();

            assert_eq!(view.get(&key).unwrap(), map_entry.value());

            let key_value: (&i32, &String) = view.get_key_value(&key).unwrap();

            assert_eq!(key_value.0, map_entry.key());

            assert_eq!(key_value.1, map_entry.value());
        });
    }

    #[test]
    fn test_into_par_iter() {
        let map = construct_sample_map();

        let view = map.clone().into_read_only();

        view.into_par_iter().for_each(|(key, value)| {
            let map_entry = map.get(&key).unwrap();

            assert_eq!(&key, map_entry.key());

            assert_eq!(&value, map_entry.value());
        });
    }
}
