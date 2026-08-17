# arrayref

[![Build Status](https://travis-ci.org/droundy/arrayref.svg?branch=master)](https://travis-ci.org/droundy/arrayref)
[![Coverage Status](https://coveralls.io/repos/droundy/arrayref/badge.svg?branch=master&service=github)](https://coveralls.io/github/droundy/arrayref?branch=master)

[Documentation](https://docs.rs/arrayref)

This is a very small rust module, which contains just four macros, for
the taking of array references to slices of... sliceable things.
These macros (which are awkwardly named) should be perfectly safe, and
have seen just a tad of code review.

## Why would I want this?

The goal of arrayref is to enable the effective use of APIs that
involve array references rather than slices, for situations where
parameters must have a given size.  As an example, consider the
`byteorder` crate.  This is a very nice crate with a simple API
containing functions that look like:

```rust
fn read_u16(buf: &[u8]) -> u16;
fn write_u16(buf: &mut [u8], n: u16);
```

Looking at this, you might wonder why they accept a slice reference as
input.  After all, they always want just two bytes.  These functions
must panic if given a slice that is too small, which means that unless
they are inlined, then a runtime bounds-check is forced, even if it
may be statically known that the input is the right size.

Wouldn't it be nicer if we had functions more like

```rust
fn read_u16_array(buf: &[u8; 2]) -> u16;
fn write_u16_array(buf: &mut [u8; 2], n: u16);
```

The type signature would tell users precisely what size of input is
required, and the compiler could check at compile time that the input
is of the appropriate size: this sounds like the zero-cost
abstractions rust is famous for!  However, there is a catch, which
arises when you try to *use* these nicer functions, which is that
usually you are looking at two bytes in a stream.  So, e.g. consider
that we are working with a hypothetical (and simplified) ipv6 address.

Doing this with our array version (which looks so beautiful in terms
of accurately describing what we want!) looks terrible:

```rust
let addr: &[u8; 16] = ...;
let mut segments = [0u16; 8];
// array-based API
for i in 0 .. 8 {
    let mut two_bytes = [addr[2*i], addr[2*i+1]];
    segments[i] = read_u16_array(&two_bytes);
}
// slice-based API
for i in 0 .. 8 {
    segments[i] = read_u16(&addr[2*i..]);
}
```

The array-based approach looks way worse.  We need to create a fresh
copy of the bytes, just so it will be in an array of the proper size!
Thus the whole "zero-cost abstraction" argument for using array
references fails.  The trouble is that there is no (safe) way (until
[RFC 495][1] lands) to obtain an array reference to a portion of a
larger array or slice.  Doing so is the equivalent of taking a slice
with a size known at compile time, and ought to be built into the
language.

[1]: https://github.com/rust-lang/rfcs/blob/master/text/0495-array-pattern-changes.md

The arrayref crate allows you to do this kind of slicing.  So the
above (very contrived) example can be implemented with array
references as:

```rust
let addr: &[u8; 16] = ...;
let mut segments = [0u16; 8];
// array-based API with arrayref
for i in 0 .. 8 {
    segments[i] = read_u16_array(array_ref![addr,2*i,2]);
}
```

Here the `array_ref![addr,2*i,2]` macro allows us to take an array
reference to a slice consisting of two bytes starting at `2*i`.  Apart
from the syntax (less nice than slicing), it is essentially the same
as the slice approach.  However, this code makes explicit the
need for precisely *two* bytes both in the caller, and in the function
signature.

This module provides three other macros providing related
functionality, which you can read about in the
[documentation](https://droundy.github.io/arrayref).

For an example of how these macros can be used in an actual program,
see [my rust translation of tweetnacl][2], which uses `arrayref`
to almost exclusively accept array references in functions, with the
only exception being those which truly expect data of arbitrary
length.  In my opinion, the result is code that is far more legible
than the original C code, since the size of each argument is
explicit.  Moreover (although I have not tested this), the use of
array references rather than slices *should* result in far fewer
bounds checks, since almost all sizes are known at compile time.

[2]: https://github.com/droundy/onionsalt/blob/master/src/crypto/mod.rs
