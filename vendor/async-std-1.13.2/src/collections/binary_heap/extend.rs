use std::collections::BinaryHeap;
use std::pin::Pin;

use crate::prelude::*;
use crate::stream::{self, IntoStream};

impl<T: Ord + Send> stream::Extend<T> for BinaryHeap<T> {
    fn extend<'a, S: IntoStream<Item = T> + 'a>(
        &'a mut self,
        stream: S,
    ) -> Pin<Box<dyn Future<Output = ()> + 'a + Send>>
    where
        <S as IntoStream>::IntoStream: Send,
    {
        let stream = stream.into_stream();

        self.reserve(stream.size_hint().0);

        Box::pin(stream.for_each(move |item| self.push(item)))
    }
}
