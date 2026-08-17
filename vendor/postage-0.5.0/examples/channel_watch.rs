use postage::{prelude::Stream, sink::Sink, watch};

#[tokio::main]
async fn main() {
    // Postage provides a watch channel, similar to the tokio watch channel.
    // Watch channels store a single value, and receivers subscribe to updates.
    // When receivers are initially created, they observe the stored value.

    // There are a few ways of constructing watch channels:

    // This constructs a channel with T::default() - in this case 0
    let (_tx, mut rx) = watch::channel::<usize>();
    assert_eq!(Some(0), rx.recv().await);

    // You can provide an initial value
    let (_tx, mut rx) = watch::channel_with(42);
    assert_eq!(Some(42), rx.recv().await);

    // You can also construct a channel with Option<T>, which implements Default.
    // Let's keep track of all the cookie boxes we sent to Alice in a Vec!
    let (mut tx, mut rx) = watch::channel_with_option::<Vec<usize>>();

    // since rx observes the initial value, we will get a None message
    assert_eq!(Some(None), rx.recv().await);

    tx.send(Some(vec![4])).await.ok();
    assert_eq!(Some(Some(vec![4])), rx.recv().await);

    // You can borrow the contained value in the sender and receiver:
    // This blocks the channel, so keep it short!
    {
        let mut value = tx.borrow_mut();
        if let Some(vec) = value.as_mut() {
            vec.push(12);
        }
    }

    // Receivers will get a message with the update
    assert_eq!(Some(Some(vec![4, 12])), rx.recv().await);

    // Receivers can also can borrow the current value:
    let cookie_data = rx.borrow();
    if let Some(value) = cookie_data.as_ref() {
        let total: usize = value.iter().copied().sum();
        println!("Alice has this many cookies: {}", total)
    }
}
