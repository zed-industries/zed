use std::pin::Pin;

use crate::prelude::*;
use crate::stream::{FromStream, IntoStream};
use std::convert::identity;

impl<T: Send, V> FromStream<Option<T>> for Option<V>
where
    V: FromStream<T>,
{
    /// Takes each element in the stream: if it is `None`, no further
    /// elements are taken, and `None` is returned. Should no `None`
    /// occur, a container with the values of each `Option` is returned.
    #[inline]
    fn from_stream<'a, S: IntoStream<Item = Option<T>> + 'a>(
        stream: S,
    ) -> Pin<Box<dyn Future<Output = Self> + 'a + Send>> 
    where
        <S as IntoStream>::IntoStream: Send,
    {
        let stream = stream.into_stream();

        Box::pin(async move {
            // Using `take_while` here because it is able to stop the stream early
            // if a failure occurs
            let mut found_none = false;
            let out: V = stream
                .take_while(|elem| {
                    elem.is_some() || {
                        found_none = true;
                        // Stop processing the stream on `None`
                        false
                    }
                })
                .filter_map(identity)
                .collect()
                .await;

            if found_none { None } else { Some(out) }
        })
    }
}
