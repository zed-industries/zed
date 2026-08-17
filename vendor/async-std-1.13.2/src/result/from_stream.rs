use std::pin::Pin;

use crate::prelude::*;
use crate::stream::{FromStream, IntoStream};

impl<T, E, V> FromStream<Result<T, E>> for Result<V, E>
where
    T: Send,
    E: Send,
    V: FromStream<T>,
{
    /// Takes each element in the stream: if it is an `Err`, no further
    /// elements are taken, and the `Err` is returned. Should no `Err`
    /// occur, a container with the values of each `Result` is returned.
    ///
    /// # Examples
    ///
    /// ```
    /// # fn main() { async_std::task::block_on(async {
    /// #
    /// use async_std::prelude::*;
    /// use async_std::stream;
    ///
    /// let v = stream::from_iter(vec![1, 2]);
    /// let res: Result<Vec<u32>, &'static str> = v.map(|x: u32|
    ///     x.checked_add(1).ok_or("Overflow!")
    /// ).collect().await;
    /// assert_eq!(res, Ok(vec![2, 3]));
    /// #
    /// # }) }
    /// ```
    #[inline]
    fn from_stream<'a, S: IntoStream<Item = Result<T, E>> + 'a>(
        stream: S,
    ) -> Pin<Box<dyn Future<Output = Self> + 'a + Send>> 
    where
        <S as IntoStream>::IntoStream: Send,
    {
        let stream = stream.into_stream();

        Box::pin(async move {
            // Using `take_while` here because it is able to stop the stream early
            // if a failure occurs
            let mut is_error = false;
            let mut found_error = None;
            let out: V = stream
                .take_while(|elem| {
                    // Stop processing the stream on `Err`
                    !is_error
                        && (elem.is_ok() || {
                            is_error = true;
                            // Capture first `Err`
                            true
                        })
                })
                .filter_map(|elem| match elem {
                    Ok(value) => Some(value),
                    Err(err) => {
                        found_error = Some(err);
                        None
                    }
                })
                .collect()
                .await;

            if is_error {
                Err(found_error.unwrap())
            } else {
                Ok(out)
            }
        })
    }
}
