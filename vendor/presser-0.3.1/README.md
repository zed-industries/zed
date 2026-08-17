<!-- Allow this file to not have a first line heading -->
<!-- markdownlint-disable-file MD041 no-emphasis-as-heading -->

<!-- inline html -->
<!-- markdownlint-disable-file MD033 -->

<div align="center">

# `🗜 presser`

**Utilities to help make copying data around into raw, possibly-uninitialized buffers easier and safer.**

[![Embark](https://img.shields.io/badge/embark-open%20source-blueviolet.svg)](https://embark.dev)
[![Embark](https://img.shields.io/badge/discord-ark-%237289da.svg?logo=discord)](https://discord.gg/dAuKfZS)
[![Crates.io](https://img.shields.io/crates/v/presser.svg)](https://crates.io/crates/presser)
[![Published Docs](https://docs.rs/presser/badge.svg)](https://docs.rs/presser)
[![Git Docs](https://img.shields.io/badge/git%20main%20docs-published-blue?style=plastic)](https://embarkstudios.github.io/presser/presser/index.html)
[![dependency status](https://deps.rs/repo/github/EmbarkStudios/presser/status.svg)](https://deps.rs/repo/github/EmbarkStudios/presser)
</div>

`presser` can help you when copying data into raw buffers. One primary use-case is copying data into
graphics-api-allocated buffers which will then be accessed by the GPU. Common methods for doing this
right now in Rust can often invoke UB in subtle and hard-to-see ways. For example, viewing an allocated
but uninitialized buffer as an `&mut [u8]` **is instantly undefined behavior**\*, and `transmute`ing even a
`T: Copy` type which has *any padding bytes in its layout* as a `&[u8]` to be the source of a copy is
**also instantly undefined behavior**, in both cases because it is *invalid* to create a reference to an invalid
value (and uninitialized memory is an invalid `u8`), *even if* your code never actually accesses that memory.
This immediately makes what seems like the most straightforward way to copy data into buffers unsound 😬

\* *If you're currently thinking to yourself "bah! what's the issue? surely an uninit u8 is just any random bit pattern
and that's fine we don't care," [check out this blog post](https://www.ralfj.de/blog/2019/07/14/uninit.html) by
@RalfJung, one of the people leading the effort to better define Rust's memory and execution model. As is explored
in that blog post, an*uninit*piece of memory is not simply*an arbitrary bit pattern*, it is a wholly separate
state about a piece of memory, outside of its value, which lets the compiler perform optimizations that reorder,
delete, and otherwise change the actual execution flow of your program in ways that cannot be described simply
by "the value could have*some*possible bit pattern". LLVM and Clang are changing themselves to require special
`noundef` attribute to perform many important optimizations that are otherwise unsound. For a concrete example
of the sorts of problems this can cause, [see this issue @scottmcm hit](https://github.com/rust-lang/rust/pull/98919#issuecomment-1186106387).*

## A bad example

```rust
#[derive(Clone, Copy)]
#[repr(C)]
struct MyDataStruct {
    a: u8,
    b: u32,
}

let my_data = MyDataStruct { a: 0, b: 42 };

// 🚨 MyDataStruct contains 3 padding bytes after `a`, which are
// uninit, therefore getting a slice that includes them is UB!
let my_data_bytes: &[u8] = transmute(&my_data);

// allocate an uninit buffer of some size
let my_buffer: MyBufferType = some_api.alloc_buffer_size(2048);

// 🚨 this is UB for the same reason, these bytes are uninit!
let buffer_as_bytes: &mut [u8] =
    slice::from_raw_parts(my_buffer.ptr(), my_buffer.size());

// 🚨 this is UB because not only are both slices invalid,
// this is not ensuring proper alignment!
buffer_as_bytes.copy_from_slice(my_data_bytes);
```

`presser` helps with this by allowing you to view raw allocated memory of some size as a "`Slab`" of memory and then
provides *safe, valid* ways to copy data into that memory:

### A good example

```rust
#[derive(Clone, Copy)]
#[repr(C)]
struct MyDataStruct {
    a: u8,
    b: u32,
}

let my_data = MyDataStruct { a: 0, b: 42 };

// allocate an uninit buffer of some size
let my_buffer: MyBufferType = some_api.alloc_buffer_size(2048);

// use `RawAllocation` helper to allow access to a presser `Slab`.
// alternatively, you could implement the `Slab` on your buffer type directly if that
// type is owned by your code!
let raw_allocation = presser::RawAllocation::from_raw_parts(my_buffer.ptr(), my_buffer.size());

// here we assert that we have exclusive access to the data in the buffer, and get the actual
// `Slab` to use to copy into.
let slab = unsafe { raw_allocation.borrow_as_slab(); }

// now we may safely copy `my_data` into `my_buffer`, starting at a minimum offset of 0 into the buffer
let copy_record = presser::copy_to_offset(&my_data, &mut slab, 0)?;

// note that due to alignment requirements of `my_data`, the *actual* start of the bytes of
// `my_data` may be placed at a different offset than requested. so, we check the returned
// `CopyRecord` to check the actual start offset of the copied data.
let actual_start_offset = copy_record.copy_start_offset;
```

Note that actually accessing the copied data is a completely separate issue which `presser` does not
(as of now) concern itself with. BE CAREFUL!

See more in [the git `main` docs](https://embarkstudios.github.io/presser/presser/index.html)
or [the released version docs](https://docs.rs/presser).

## Contribution

[![Contributor Covenant](https://img.shields.io/badge/contributor%20covenant-v1.4-ff69b4.svg)](CODE_OF_CONDUCT.md)

We welcome community contributions to this project.

Please read our [Contributor Guide](CONTRIBUTING.md) for more information on how to get started.
Please also read our [Contributor Terms](CONTRIBUTING.md#contributor-terms) before you make any contributions.

Any contribution intentionally submitted for inclusion in an Embark Studios project, shall comply with the Rust standard licensing model (MIT OR Apache 2.0) and therefore be dual licensed as described below, without any additional terms or conditions:

### License

This contribution is dual licensed under EITHER OF

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

For clarity, "your" refers to Embark or any other licensee/user of the contribution.
