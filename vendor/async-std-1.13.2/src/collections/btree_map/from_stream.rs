use std::collections::BTreeMap;
use std::pin::Pin;

use crate::prelude::*;
use crate::stream::{self, FromStream, IntoStream};

impl<K: Ord + Send, V: Send> FromStream<(K, V)> for BTreeMap<K, V> {
    #[inline]
    fn from_stream<'a, S: IntoStream<Item = (K, V)> + 'a>(
        stream: S,
    ) -> Pin<Box<dyn Future<Output = Self> + 'a + Send>> 
    where
        <S as IntoStream>::IntoStream: Send,
    {
        let stream = stream.into_stream();

        Box::pin(async move {
            let mut out = BTreeMap::new();
            stream::extend(&mut out, stream).await;
            out
        })
    }
}
