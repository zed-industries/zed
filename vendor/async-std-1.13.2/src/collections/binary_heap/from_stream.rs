use std::collections::BinaryHeap;
use std::pin::Pin;

use crate::prelude::*;
use crate::stream::{self, FromStream, IntoStream};

impl<T: Ord + Send> FromStream<T> for BinaryHeap<T> {
    #[inline]
    fn from_stream<'a, S: IntoStream<Item = T> + 'a>(
        stream: S,
    ) -> Pin<Box<dyn Future<Output = Self> + 'a + Send>>
    where
        <S as IntoStream>::IntoStream: Send,
    {
        let stream = stream.into_stream();

        Box::pin(async move {
            let mut out = BinaryHeap::new();
            stream::extend(&mut out, stream).await;
            out
        })
    }
}
