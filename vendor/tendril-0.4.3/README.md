# tendril

**Warning**: This library is at a very early stage of development, and it
contains a substantial amount of `unsafe` code. Use at your own risk!

[![Build Status](https://github.com/servo/tendril/workflows/CI/badge.svg)](https://github.com/servo/tendril/actions)

[API Documentation](https://doc.servo.org/tendril/index.html)

## Introduction

`Tendril` is a compact string/buffer type, optimized for zero-copy parsing.
Tendrils have the semantics of owned strings, but are sometimes views into
shared buffers. When you mutate a tendril, an owned copy is made if necessary.
Further mutations occur in-place until the string becomes shared, e.g. with
`clone()` or `subtendril()`.

Buffer sharing is accomplished through thread-local (non-atomic) reference
counting, which has very low overhead. The Rust type system will prevent
you at compile time from sending a tendril between threads. (See below
for thoughts on relaxing this restriction.)

Whereas `String` allocates in the heap for any non-empty string, `Tendril` can
store small strings (up to 8 bytes) in-line, without a heap allocation.
`Tendril` is also smaller than `String` on 64-bit platforms — 16 bytes versus
24. `Option<Tendril>` is the same size as `Tendril`, thanks to
[`NonZero`][NonZero].

The maximum length of a tendril is 4 GB. The library will panic if you attempt
to go over the limit.

## Formats and encoding

`Tendril` uses
[phantom types](https://doc.rust-lang.org/stable/rust-by-example/generics/phantom.html)
to track a buffer's format. This determines at compile time which
operations are available on a given tendril. For example, `Tendril<UTF8>` and
`Tendril<Bytes>` can be borrowed as `&str` and `&[u8]` respectively.

`Tendril` also integrates with
[rust-encoding](https://github.com/lifthrasiir/rust-encoding) and has
preliminary support for [WTF-8][] buffers.

## Plans for the future

### Ropes

[html5ever][] will use `Tendril` as a zero-copy text representation. It would
be good to preserve this all the way through to Servo's DOM. This would reduce
memory consumption, and possibly speed up text shaping and painting. However,
DOM text may conceivably be larger than 4 GB, and will anyway not be contiguous
in memory around e.g. a character entity reference.

*Solution:* Build a **[rope][] on top of these strings** and use that as
Servo's representation of DOM text. We can perhaps do text shaping and/or
painting in parallel for different chunks of a rope. html5ever can additionally
use this rope type as a replacement for `BufferQueue`.

Because the underlying buffers are reference-counted, the bulk of this rope
is already a [persistent data structure][]. Consider what happens when
appending two ropes to get a "new" rope. A vector-backed rope would copy a
vector of small structs, one for each chunk, and would bump the corresponding
refcounts. But it would not copy any of the string data.

If we want more sharing, then a [2-3 finger tree][] could be a good choice.
We would probably stick with `VecDeque` for ropes under a certain size.

### UTF-16 compatibility

SpiderMonkey expects text to be in UCS-2 format for the most part. The
semantics of JavaScript strings are difficult to implement on UTF-8. This also
applies to HTML parsing via `document.write`. Also, passing SpiderMonkey a
string that isn't contiguous in memory will incur additional overhead and
complexity, if not a full copy.

*Solution:* Use **WTF-8 in parsing** and in the DOM. Servo will **convert to
contiguous UTF-16 when necessary**.  The conversion can easily be parallelized,
if we find a practical need to convert huge chunks of text all at once.

### Source span information

Some html5ever API consumers want to know the originating location in the HTML
source file(s) of each token or parse error. An example application would be a
command-line HTML validator with diagnostic output similar to `rustc`'s.

*Solution:* Accept **some metadata along with each input string**. The type of
metadata is chosen by the API consumer; it defaults to `()`, which has size
zero. For any non-inline string, we can provide the associated metadata as well
as a byte offset.

[NonZero]: https://doc.rust-lang.org/core/nonzero/struct.NonZero.html
[html5ever]: https://github.com/servo/html5ever
[WTF-8]: https://simonsapin.github.io/wtf-8/
[rope]: https://en.wikipedia.org/wiki/Rope_%28data_structure%29
[persistent data structure]: https://en.wikipedia.org/wiki/Persistent_data_structure
[2-3 finger tree]: https://www.staff.city.ac.uk/~ross/papers/FingerTree.html
