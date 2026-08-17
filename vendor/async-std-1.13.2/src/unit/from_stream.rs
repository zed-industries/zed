use std::pin::Pin;

use crate::prelude::*;
use crate::stream::{FromStream, IntoStream};

impl FromStream<()> for () {
    #[inline]
    fn from_stream<'a, S: IntoStream<Item = ()> + 'a>(
        stream: S,
    ) -> Pin<Box<dyn Future<Output = Self> + 'a + Send>>
    where
        <S as IntoStream>::IntoStream: Send,
    {
        Box::pin(stream.into_stream().for_each(drop))
    }
}
