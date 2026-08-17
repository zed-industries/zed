use std::pin::Pin;

use crate::prelude::*;
use crate::stream::{Stream, Sum};
use std::convert::identity;

impl<T, U> Sum<Option<U>> for Option<T>
where
    T: Sum<U>,
{
    #[doc = r#"
        Takes each element in the `Iterator`: if it is a `None`, no further
        elements are taken, and the `None` is returned. Should no `None` occur,
        the sum of all elements is returned.

        # Examples

        This sums up the position of the character 'a' in a vector of strings,
        if a word did not have the character 'a' the operation returns `None`:

        ```
        # fn main() { async_std::task::block_on(async {
        #
        use async_std::prelude::*;
        use async_std::stream;

        let words = stream::from_iter(vec!["have", "a", "great", "day"]);
        let total: Option<usize> = words.map(|w| w.find('a')).sum().await;
        assert_eq!(total, Some(5));
        #
        # }) }
        ```
    "#]
    fn sum<'a, S>(stream: S) -> Pin<Box<dyn Future<Output = Option<T>> + 'a>>
    where
        S: Stream<Item = Option<U>> + 'a,
    {
        Box::pin(async move {
            // Using `take_while` here because it is able to stop the stream early
            // if a failure occurs
            let mut found_none = false;
            let out = <T as Sum<U>>::sum(
                stream
                    .take_while(|elem| {
                        elem.is_some() || {
                            found_none = true;
                            // Stop processing the stream on `None`
                            false
                        }
                    })
                    .filter_map(identity),
            )
            .await;

            if found_none { None } else { Some(out) }
        })
    }
}
