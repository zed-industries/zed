use std::pin::Pin;

use crate::prelude::*;
use crate::stream::{Product, Stream};
use std::convert::identity;

impl<T, U> Product<Option<U>> for Option<T>
where
    T: Product<U>,
{
    #[doc = r#"
        Takes each element in the `Stream`: if it is a `None`, no further
        elements are taken, and the `None` is returned. Should no `None` occur,
        the product of all elements is returned.

        # Examples

        This multiplies every integer in a vector, rejecting the product if a negative element is
        encountered:

        ```
        # fn main() { async_std::task::block_on(async {
        #
        use async_std::prelude::*;
        use async_std::stream;

        let v = stream::from_iter(vec![1, 2, 4]);
        let prod: Option<i32> = v.map(|x|
            if x < 0 {
                None
            } else {
                Some(x)
            }).product().await;
        assert_eq!(prod, Some(8));
        #
        # }) }
        ```
    "#]
    fn product<'a, S>(stream: S) -> Pin<Box<dyn Future<Output = Option<T>> + 'a>>
    where
        S: Stream<Item = Option<U>> + 'a,
    {
        Box::pin(async move {
            // Using `take_while` here because it is able to stop the stream early
            // if a failure occurs
            let mut found_none = false;
            let out = <T as Product<U>>::product(
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
