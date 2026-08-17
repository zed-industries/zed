use std::pin::Pin;

use crate::prelude::*;
use crate::stream::{self, IntoStream};

impl stream::Extend<()> for () {
    fn extend<'a, S: IntoStream<Item = ()> + 'a>(
        &'a mut self,
        stream: S,
    ) -> Pin<Box<dyn Future<Output = ()> + 'a + Send>> 
    where
        <S as IntoStream>::IntoStream: Send,
    {
        let stream = stream.into_stream();

        Box::pin(async move {
            pin_utils::pin_mut!(stream);

            while let Some(_) = stream.next().await {}
        })
    }
}
