#[cfg(feature = "unstable")]
#[test]
fn test_send() {
    use async_std::prelude::*;
    use async_std::{stream, task};

    task::block_on(async {
        fn test_send_trait<T: Send>(_: &T) {}

        let stream = stream::repeat(1u8).take(10);
        test_send_trait(&stream);

        let fut = stream.collect::<Vec<_>>();

        // This line triggers a compilation error
        test_send_trait(&fut);
    });
}
