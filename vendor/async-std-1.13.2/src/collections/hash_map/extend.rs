use std::collections::HashMap;
use std::hash::{BuildHasher, Hash};
use std::pin::Pin;

use crate::prelude::*;
use crate::stream::{self, IntoStream};

impl<K, V, H> stream::Extend<(K, V)> for HashMap<K, V, H>
where
    K: Eq + Hash + Send,
    V: Send,
    H: BuildHasher + Default + Send,
{
    fn extend<'a, S: IntoStream<Item = (K, V)> + 'a>(
        &'a mut self,
        stream: S,
    ) -> Pin<Box<dyn Future<Output = ()> + 'a + Send>>
    where
        <S as IntoStream>::IntoStream: Send,
    {
        let stream = stream.into_stream();

        // The following is adapted from the hashbrown source code:
        // https://github.com/rust-lang/hashbrown/blob/d1ad4fc3aae2ade446738eea512e50b9e863dd0c/src/map.rs#L2470-L2491
        //
        // Keys may be already present or show multiple times in the stream. Reserve the entire
        // hint lower bound if the map is empty. Otherwise reserve half the hint (rounded up), so
        // the map will only resize twice in the worst case.

        let additional = if self.is_empty() {
            stream.size_hint().0
        } else {
            (stream.size_hint().0 + 1) / 2
        };
        self.reserve(additional);

        Box::pin(stream.for_each(move |(k, v)| {
            self.insert(k, v);
        }))
    }
}
