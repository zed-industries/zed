[![Workflow Status](https://github.com/enarx/ciborium/workflows/test/badge.svg)](https://github.com/enarx/ciborium/actions?query=workflow%3A%22test%22)
[![Average time to resolve an issue](https://isitmaintained.com/badge/resolution/enarx/ciborium.svg)](https://isitmaintained.com/project/enarx/ciborium "Average time to resolve an issue")
[![Percentage of issues still open](https://isitmaintained.com/badge/open/enarx/ciborium.svg)](https://isitmaintained.com/project/enarx/ciborium "Percentage of issues still open")
![Maintenance](https://img.shields.io/badge/maintenance-activly--developed-brightgreen.svg)

# ciborium-ll

Low level CBOR parsing tools

This crate contains low-level types for encoding and decoding items in
CBOR. This crate is usable in both `no_std` and `no_alloc` environments.
To understand how this crate works, first we will look at the structure
of a CBOR item on the wire.

## Anatomy of a CBOR Item

This is a brief anatomy of a CBOR item on the wire.

```
+------------+-----------+
|            |           |
|   Major    |   Minor   |
|  (3bits)   |  (5bits)  |
|            |           |
+------------+-----------+
^                        ^
|                        |
+-----+            +-----+
      |            |
      |            |
      +----------------------------+--------------+
      |            |               |              |
      |   Prefix   |     Affix     |    Suffix    |
      |  (1 byte)  |  (0-8 bytes)  |  (0+ bytes)  |
      |            |               |              |
      +------------+---------------+--------------+

      |                            |              |
      +------------+---------------+--------------+
                   |                       |
                   v                       v

                 Header                   Body
```

The `ciborium` crate works by providing the `Decoder` and `Encoder` types
which provide input and output for a CBOR header (see: `Header`). From
there, you can either handle the body yourself or use the provided utility
functions.

For more information on the CBOR format, see
[RFC 7049](https://tools.ietf.org/html/rfc7049).

## Decoding

In order to decode CBOR, you will create a `Decoder` from a reader. The
decoder instance will allow you to `Decoder::pull()` `Header` instances
from the input.

Most CBOR items are fully contained in their headers and therefore have no
body. These items can be evaluated directly from the `Header` instance.

Bytes and text items have a body but do not contain child items. Since
both bytes and text values may be segmented, parsing them can be a bit
tricky. Therefore, we provide helper functions to parse these types. See
`Decoder::bytes()` and `Decoder::text()` for more details.

Array and map items have a body which contains child items. These can be
parsed by simply doing `Decoder::pull()` to parse the child items.

### Example

```rust
use ciborium_ll::{Decoder, Header};
use ciborium_io::Read as _;

let input = b"\x6dHello, World!";
let mut decoder = Decoder::from(&input[..]);
let mut chunks = 0;

match decoder.pull().unwrap() {
    Header::Text(len) => {
        let mut segments = decoder.text(len);
        while let Some(mut segment) = segments.pull().unwrap() {
            let mut buffer = [0u8; 7];
            while let Some(chunk) = segment.pull(&mut buffer[..]).unwrap() {
                 match chunk {
                     "Hello, " if chunks == 0 => chunks = 1,
                     "World!" if chunks == 1 => chunks = 2,
                     _ => panic!("received unexpected chunk"),
                 }
            }
        }
    }

    _ => panic!("received unexpected value"),
}

assert_eq!(chunks, 2);
```

## Encoding

To encode values to CBOR, create an `Encoder` from a writer. The encoder
instance provides the `Encoder::push()` method to write a `Header` value
to the wire. CBOR item bodies can be written directly.

For bytes and text, there are the `Encoder::bytes()` and `Encoder::text()`
utility functions, respectively, which will properly segment the output
on the wire for you.

### Example

```rust
use ciborium_ll::{Encoder, Header};
use ciborium_io::Write as _;

let mut buffer = [0u8; 19];
let mut encoder = Encoder::from(&mut buffer[..]);

// Write the structure
encoder.push(Header::Map(Some(1))).unwrap();
encoder.push(Header::Positive(7)).unwrap();
encoder.text("Hello, World!", 7).unwrap();

// Validate our output
encoder.flush().unwrap();
assert_eq!(b"\xa1\x07\x7f\x67Hello, \x66World!\xff", &buffer[..]);
```

License: Apache-2.0
