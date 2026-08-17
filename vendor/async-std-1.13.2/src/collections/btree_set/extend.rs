use std::collections::BTreeSet;
use std::pin::Pin;

use crate::prelude::*;
use crate::stream::{self, IntoStream};

impl<T: Ord + Send> stream::Extend<T> for BTreeSet<T> {
    fn extend<'a, S: IntoStream<Item = T> + 'a>(
        &'a mut self,
        stream: S,
    ) -> Pin<Box<dyn Future<Output = ()> + 'a + Send>>
    where
        <S as IntoStream>::IntoStream: Send,
    {
        Box::pin(stream.into_stream().for_each(move |item| {
            self.insert(item);
        }))
    }
}
