# Small Patterns

A collection of small, useful patterns.

## Splitting streams

`async-std` doesn't provide a `split()` method on `io` handles. Instead, splitting a stream into a read and write half can be done like this:

```rust,edition2018
# extern crate async_std;
use async_std::{io, net::TcpStream};
async fn echo(stream: TcpStream) {
    let (reader, writer) = &mut (&stream, &stream);
    io::copy(reader, writer).await;
}
```
