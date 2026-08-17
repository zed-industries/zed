use std::collections::HashSet;
use std::hash::{BuildHasher, Hash};
use std::pin::Pin;

use crate::prelude::*;
use crate::stream::{self, FromStream, IntoStream};

impl<T, H> FromStream<T> for HashSet<T, H>
where
    T: Eq + Hash + Send,
    H: BuildHasher + Default + Send,
{
    #[inline]
    fn from_stream<'a, S: IntoStream<Item = T> + 'a>(
        stream: S,
    ) -> Pin<Box<dyn Future<Output = Self> + 'a + Send>> 
    where
        <S as IntoStream>::IntoStream: Send,
    {
        let stream = stream.into_stream();

        Box::pin(async move {
            let mut out = HashSet::with_hasher(Default::default());
            stream::extend(&mut out, stream).await;
            out
        })
    }
}
