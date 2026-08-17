{{#title Tutorial — Rust ♡ C++}}
# Tutorial: CXX blobstore client

This example walks through a Rust application that calls into a C++ client of a
blobstore service. In fact we'll see calls going in both directions: Rust to C++
as well as C++ to Rust. For your own use case it may be that you need just one
of these directions.

All of the code involved in the example is shown on this page, but it's also
provided in runnable form in the *demo* directory of
<https://github.com/dtolnay/cxx>. To try it out directly, run `cargo run` from
that directory.

This tutorial assumes you've read briefly about **shared structs**, **opaque
types**, and **functions** in the [*Core concepts*](concepts.md) page.

## Creating the project

We'll use Cargo, which is the build system commonly used by open source Rust
projects. (CXX works with other build systems too; refer to chapter 5.)

Create a blank Cargo project: `mkdir cxx-demo`; `cd cxx-demo`; `cargo init`.

Edit the Cargo.toml to add a dependency on the `cxx` crate:

```toml,hidelines=...
# Cargo.toml
...[package]
...name = "cxx-demo"
...version = "0.1.0"
...edition = "2021"

[dependencies]
cxx = "1.0"
```

We'll revisit this Cargo.toml later when we get to compiling some C++ code.

## Defining the language boundary

CXX relies on a description of the function signatures that will be exposed from
each language to the other. You provide this description using `extern` blocks
in a Rust module annotated with the `#[cxx::bridge]` attribute macro.

We'll open with just the following at the top of src/main.rs and walk through
each item in detail.

```rust,noplayground
// src/main.rs

#[cxx::bridge]
mod ffi {

}
#
# fn main() {}
```

The contents of this module will be everything that needs to be agreed upon by
both sides of the FFI boundary.

## Calling a C++ function from Rust

Let's obtain an instance of the C++ blobstore client, a class `BlobstoreClient`
defined in C++.

We'll treat `BlobstoreClient` as an *opaque type* in CXX's classification so
that Rust does not need to assume anything about its implementation, not even
its size or alignment. In general, a C++ type might have a move-constructor
which is incompatible with Rust's move semantics, or may hold internal
references which cannot be modeled by Rust's borrowing system. Though there are
alternatives, the easiest way to not care about any such thing on an FFI
boundary is to require no knowledge about a type by treating it as opaque.

Opaque types may only be manipulated behind an indirection such as a reference
`&`, a Rust `Box`, or a `UniquePtr` (Rust binding of `std::unique_ptr`). We'll
add a function through which C++ can return a `std::unique_ptr<BlobstoreClient>`
to Rust.

```rust,noplayground
// src/main.rs

#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("cxx-demo/include/blobstore.h");

        type BlobstoreClient;

        fn new_blobstore_client() -> UniquePtr<BlobstoreClient>;
    }
}

fn main() {
    let client = ffi::new_blobstore_client();
}
```

The nature of `unsafe` extern blocks is clarified in more detail in the
[*extern "C++"*](extern-c++.md) chapter. In brief: the programmer is **not**
promising that the signatures they have typed in are accurate; that would be
unreasonable. CXX performs static assertions that the signatures exactly match
what is declared in C++. Rather, the programmer is only on the hook for things
that C++'s semantics are not precise enough to capture, i.e. things that would
only be represented at most by comments in the C++ code. In this case, it's
whether `new_blobstore_client` is safe or unsafe to call. If that function said
something like "must be called at most once or we'll stomp yer memery", Rust
would instead want to expose it as `unsafe fn new_blobstore_client`, this time
inside a safe `extern "C++"` block because the programmer is no longer on the
hook for any safety claim about the signature.

If you build this file right now with `cargo build`, it won't build because we
haven't written a C++ implementation of `new_blobstore_client` nor instructed
Cargo about how to link it into the resulting binary. You'll see an error from
the linker like this:

```console
error: linking with `cc` failed: exit code: 1
 |
 = /bin/ld: target/debug/deps/cxx-demo-7cb7fddf3d67d880.rcgu.o: in function `cxx_demo::ffi::new_blobstore_client':
   src/main.rs:1: undefined reference to `cxxbridge1$new_blobstore_client'
   collect2: error: ld returned 1 exit status
```

## Adding in the C++ code

In CXX's integration with Cargo, all #include paths begin with a crate name by
default (when not explicitly selected otherwise by a crate; see
`CFG.include_prefix` in chapter 5). That's why we see
`include!("cxx-demo/include/blobstore.h")` above &mdash; we'll be putting the
C++ header at relative path `include/blobstore.h` within the Rust crate. If your
crate is named something other than `cxx-demo` according to the `name` field in
Cargo.toml, you will need to use that name everywhere in place of `cxx-demo`
throughout this tutorial.

```cpp
// include/blobstore.h

#pragma once
#include <memory>

class BlobstoreClient {
public:
  BlobstoreClient();
};

std::unique_ptr<BlobstoreClient> new_blobstore_client();
```

```cpp
// src/blobstore.cc

#include "cxx-demo/include/blobstore.h"

BlobstoreClient::BlobstoreClient() {}

std::unique_ptr<BlobstoreClient> new_blobstore_client() {
  return std::unique_ptr<BlobstoreClient>(new BlobstoreClient());
}
```

Using `std::make_unique` would work too, as long as you pass `std("c++14")` to
the C++ compiler as described later on.

The placement in *include/* and *src/* is not significant; you can place C++
code anywhere else in the crate as long as you use the right paths throughout
the tutorial.

Be aware that *CXX does not look at any of these files.* You're free to put
arbitrary C++ code in here, #include your own libraries, etc. All we do is emit
static assertions against what you provide in the headers.

## Compiling the C++ code with Cargo

Cargo has a [build scripts] feature suitable for compiling non-Rust code.

We need to introduce a new build-time dependency on CXX's C++ code generator in
Cargo.toml:

```toml,hidelines=...
# Cargo.toml
...[package]
...name = "cxx-demo"
...version = "0.1.0"
...edition = "2021"

[dependencies]
cxx = "1.0"

[build-dependencies]
cxx-build = "1.0"
```

Then add a build.rs build script adjacent to Cargo.toml to run the cxx-build
code generator and C++ compiler. The relevant arguments are the path to the Rust
source file containing the cxx::bridge language boundary definition, and the
paths to any additional C++ source files to be compiled during the Rust crate's
build.

```rust,noplayground
// build.rs

fn main() {
    cxx_build::bridge("src/main.rs")
        .file("src/blobstore.cc")
        .compile("cxx-demo");

    println!("cargo:rerun-if-changed=src/blobstore.cc");
    println!("cargo:rerun-if-changed=include/blobstore.h");
}
```

This build.rs would also be where you set up C++ compiler flags, for example if
you'd like to have access to `std::make_unique` from C++14. See the page on
***[Cargo-based builds](build/cargo.md)*** for more details about CXX's Cargo
integration.

```rust,noplayground
# // build.rs
#
# fn main() {
    cxx_build::bridge("src/main.rs")
        .file("src/blobstore.cc")
        .std("c++14")
        .compile("cxx-demo");
# }
```

[build scripts]: https://doc.rust-lang.org/cargo/reference/build-scripts.html

The project should now build and run successfully, though not do anything useful
yet.

```console
cxx-demo$  cargo run
  Compiling cxx-demo v0.1.0
  Finished dev [unoptimized + debuginfo] target(s) in 0.34s
  Running `target/debug/cxx-demo`

cxx-demo$
```

## Calling a Rust function from C++

Our C++ blobstore supports a `put` operation for a discontiguous buffer upload.
For example we might be uploading snapshots of a circular buffer which would
tend to consist of 2 pieces, or fragments of a file spread across memory for
some other reason (like a rope data structure).

We'll express this by handing off an iterator over contiguous borrowed chunks.
This loosely resembles the API of the widely used `bytes` crate's `Buf` trait.
During a `put`, we'll make C++ call back into Rust to obtain contiguous chunks
of the upload (all with no copying or allocation on the language boundary). In
reality the C++ client might contain some sophisticated batching of chunks
and/or parallel uploading that all of this ties into.

```rust,noplayground
// src/main.rs

#[cxx::bridge]
mod ffi {
    extern "Rust" {
        type MultiBuf;

        fn next_chunk(buf: &mut MultiBuf) -> &[u8];
    }

    unsafe extern "C++" {
        include!("cxx-demo/include/blobstore.h");

        type BlobstoreClient;

        fn new_blobstore_client() -> UniquePtr<BlobstoreClient>;
        fn put(&self, parts: &mut MultiBuf) -> u64;
    }
}
#
# fn main() {
#     let client = ffi::new_blobstore_client();
# }
```

Any signature having a `self` parameter (the Rust name for C++'s `this`) is
considered a method / non-static member function. If there is only one `type` in
the surrounding extern block, it'll be a method of that type. If there is more
than one `type`, you can disambiguate which one a method belongs to by writing
`self: &BlobstoreClient` in the argument list.

As usual, now we need to provide Rust definitions of everything declared by the
`extern "Rust"` block and a C++ definition of the new signature declared by the
`extern "C++"` block.

```rust,noplayground
// src/main.rs
#
# #[cxx::bridge]
# mod ffi {
#     extern "Rust" {
#         type MultiBuf;
#
#         fn next_chunk(buf: &mut MultiBuf) -> &[u8];
#     }
#
#     unsafe extern "C++" {
#         include!("cxx-demo/include/blobstore.h");
#
#         type BlobstoreClient;
#
#         fn new_blobstore_client() -> UniquePtr<BlobstoreClient>;
#         fn put(&self, parts: &mut MultiBuf) -> u64;
#     }
# }

// An iterator over contiguous chunks of a discontiguous file object. Toy
// implementation uses a Vec<Vec<u8>> but in reality this might be iterating
// over some more complex Rust data structure like a rope, or maybe loading
// chunks lazily from somewhere.
pub struct MultiBuf {
    chunks: Vec<Vec<u8>>,
    pos: usize,
}

pub fn next_chunk(buf: &mut MultiBuf) -> &[u8] {
    let next = buf.chunks.get(buf.pos);
    buf.pos += 1;
    next.map_or(&[], Vec::as_slice)
}
#
# fn main() {
#     let client = ffi::new_blobstore_client();
# }
```

```cpp,hidelines=...
// include/blobstore.h

...#pragma once
...#include <memory>
...
struct MultiBuf;

class BlobstoreClient {
public:
  BlobstoreClient();
  uint64_t put(MultiBuf &buf) const;
};
...
...std::unique_ptr<BlobstoreClient> new_blobstore_client();
```

In blobstore.cc we're able to call the Rust `next_chunk` function, exposed to
C++ by a header `main.rs.h` generated by the CXX code generator. In CXX's Cargo
integration this generated header has a path containing the crate name, the
relative path of the Rust source file within the crate, and a `.rs.h` extension.

```cpp,hidelines=...
// src/blobstore.cc

#include "cxx-demo/include/blobstore.h"
#include "cxx-demo/src/main.rs.h"
#include <functional>
#include <string>
...
...BlobstoreClient::BlobstoreClient() {}
...
...std::unique_ptr<BlobstoreClient> new_blobstore_client() {
...  return std::make_unique<BlobstoreClient>();
...}

// Upload a new blob and return a blobid that serves as a handle to the blob.
uint64_t BlobstoreClient::put(MultiBuf &buf) const {
  // Traverse the caller's chunk iterator.
  std::string contents;
  while (true) {
    auto chunk = next_chunk(buf);
    if (chunk.size() == 0) {
      break;
    }
    contents.append(reinterpret_cast<const char *>(chunk.data()), chunk.size());
  }

  // Pretend we did something useful to persist the data.
  auto blobid = std::hash<std::string>{}(contents);
  return blobid;
}
```

This is now ready to use. :)

```rust,noplayground
// src/main.rs
#
# #[cxx::bridge]
# mod ffi {
#     extern "Rust" {
#         type MultiBuf;
#
#         fn next_chunk(buf: &mut MultiBuf) -> &[u8];
#     }
#
#     unsafe extern "C++" {
#         include!("cxx-demo/include/blobstore.h");
#
#         type BlobstoreClient;
#
#         fn new_blobstore_client() -> UniquePtr<BlobstoreClient>;
#         fn put(&self, parts: &mut MultiBuf) -> u64;
#     }
# }
#
# pub struct MultiBuf {
#     chunks: Vec<Vec<u8>>,
#     pos: usize,
# }
# pub fn next_chunk(buf: &mut MultiBuf) -> &[u8] {
#     let next = buf.chunks.get(buf.pos);
#     buf.pos += 1;
#     next.map_or(&[], Vec::as_slice)
# }

fn main() {
    let client = ffi::new_blobstore_client();

    // Upload a blob.
    let chunks = vec![b"fearless".to_vec(), b"concurrency".to_vec()];
    let mut buf = MultiBuf { chunks, pos: 0 };
    let blobid = client.put(&mut buf);
    println!("blobid = {blobid}");
}
```

```console
cxx-demo$  cargo run
  Compiling cxx-demo v0.1.0
  Finished dev [unoptimized + debuginfo] target(s) in 0.41s
  Running `target/debug/cxx-demo`

blobid = 9851996977040795552
```

## Interlude: What gets generated?

For the curious, it's easy to look behind the scenes at what CXX has done to
make these function calls work. You shouldn't need to do this during normal
usage of CXX, but for the purpose of this tutorial it can be educative.

CXX comprises *two* code generators: a Rust one (which is the cxx::bridge
attribute procedural macro) and a C++ one.

### Rust generated code

It's easiest to view the output of the procedural macro by installing
[cargo-expand]. Then run `cargo expand ::ffi` to macro-expand the `mod ffi`
module.

[cargo-expand]: https://github.com/dtolnay/cargo-expand

```console
cxx-demo$  cargo install cargo-expand
cxx-demo$  cargo expand ::ffi
```

You'll see some deeply unpleasant code involving `#[repr(C)]`, `#[link_name]`,
and `#[export_name]`.

### C++ generated code

For debugging convenience, `cxx_build` links all generated C++ code into Cargo's
target directory under *target/cxxbridge/*.

```console
cxx-demo$  exa -T target/cxxbridge/
target/cxxbridge
├── cxx-demo
│  └── src
│     ├── main.rs.cc -> ../../../debug/build/cxx-demo-11c6f678ce5c3437/out/cxxbridge/sources/cxx-demo/src/main.rs.cc
│     └── main.rs.h -> ../../../debug/build/cxx-demo-11c6f678ce5c3437/out/cxxbridge/include/cxx-demo/src/main.rs.h
└── rust
   └── cxx.h -> ~/.cargo/registry/src/github.com-1ecc6299db9ec823/cxx-1.0.0/include/cxx.h
```

In those files you'll see declarations or templates of any CXX Rust types
present in your language boundary (like `rust::Slice<T>` for `&[T]`) and `extern
"C"` signatures corresponding to your extern functions.

If it fits your workflow better, the CXX C++ code generator is also available as
a standalone executable which outputs generated code to stdout.

```console
cxx-demo$  cargo install cxxbridge-cmd
cxx-demo$  cxxbridge src/main.rs
```

## Shared data structures

So far the calls in both directions above only used **opaque types**, not
**shared structs**.

Shared structs are data structures whose complete definition is visible to both
languages, making it possible to pass them by value across the language
boundary. Shared structs translate to a C++ aggregate-initialization compatible
struct exactly matching the layout of the Rust one.

As the last step of this demo, we'll use a shared struct `BlobMetadata` to pass
metadata about blobs between our Rust application and C++ blobstore client.

```rust,noplayground
// src/main.rs

#[cxx::bridge]
mod ffi {
    struct BlobMetadata {
        size: usize,
        tags: Vec<String>,
    }

    extern "Rust" {
        // ...
#         type MultiBuf;
#
#         fn next_chunk(buf: &mut MultiBuf) -> &[u8];
    }

    unsafe extern "C++" {
        // ...
#         include!("cxx-demo/include/blobstore.h");
#
#         type BlobstoreClient;
#
#         fn new_blobstore_client() -> UniquePtr<BlobstoreClient>;
#         fn put(&self, parts: &mut MultiBuf) -> u64;
        fn tag(&self, blobid: u64, tag: &str);
        fn metadata(&self, blobid: u64) -> BlobMetadata;
    }
}
#
# pub struct MultiBuf {
#     chunks: Vec<Vec<u8>>,
#     pos: usize,
# }
# pub fn next_chunk(buf: &mut MultiBuf) -> &[u8] {
#     let next = buf.chunks.get(buf.pos);
#     buf.pos += 1;
#     next.map_or(&[], Vec::as_slice)
# }

fn main() {
    let client = ffi::new_blobstore_client();

    // Upload a blob.
    let chunks = vec![b"fearless".to_vec(), b"concurrency".to_vec()];
    let mut buf = MultiBuf { chunks, pos: 0 };
    let blobid = client.put(&mut buf);
    println!("blobid = {blobid}");

    // Add a tag.
    client.tag(blobid, "rust");

    // Read back the tags.
    let metadata = client.metadata(blobid);
    println!("tags = {:?}", metadata.tags);
}
```

```cpp,hidelines=...
// include/blobstore.h

#pragma once
#include "rust/cxx.h"
...#include <memory>

struct MultiBuf;
struct BlobMetadata;

class BlobstoreClient {
public:
  BlobstoreClient();
  uint64_t put(MultiBuf &buf) const;
  void tag(uint64_t blobid, rust::Str tag) const;
  BlobMetadata metadata(uint64_t blobid) const;

private:
  class impl;
  std::shared_ptr<impl> impl;
};
...
...std::unique_ptr<BlobstoreClient> new_blobstore_client();
```

```cpp,hidelines=...
// src/blobstore.cc

#include "cxx-demo/include/blobstore.h"
#include "cxx-demo/src/main.rs.h"
#include <algorithm>
#include <functional>
#include <set>
#include <string>
#include <unordered_map>

// Toy implementation of an in-memory blobstore.
//
// In reality the implementation of BlobstoreClient could be a large
// complex C++ library.
class BlobstoreClient::impl {
  friend BlobstoreClient;
  using Blob = struct {
    std::string data;
    std::set<std::string> tags;
  };
  std::unordered_map<uint64_t, Blob> blobs;
};

BlobstoreClient::BlobstoreClient() : impl(new class BlobstoreClient::impl) {}
...
...// Upload a new blob and return a blobid that serves as a handle to the blob.
...uint64_t BlobstoreClient::put(MultiBuf &buf) const {
...  // Traverse the caller's chunk iterator.
...  std::string contents;
...  while (true) {
...    auto chunk = next_chunk(buf);
...    if (chunk.size() == 0) {
...      break;
...    }
...    contents.append(reinterpret_cast<const char *>(chunk.data()), chunk.size());
...  }
...
...  // Insert into map and provide caller the handle.
...  auto blobid = std::hash<std::string>{}(contents);
...  impl->blobs[blobid] = {std::move(contents), {}};
...  return blobid;
...}

// Add tag to an existing blob.
void BlobstoreClient::tag(uint64_t blobid, rust::Str tag) const {
  impl->blobs[blobid].tags.emplace(tag);
}

// Retrieve metadata about a blob.
BlobMetadata BlobstoreClient::metadata(uint64_t blobid) const {
  BlobMetadata metadata{};
  auto blob = impl->blobs.find(blobid);
  if (blob != impl->blobs.end()) {
    metadata.size = blob->second.data.size();
    std::for_each(blob->second.tags.cbegin(), blob->second.tags.cend(),
                  [&](auto &t) { metadata.tags.emplace_back(t); });
  }
  return metadata;
}
...
...std::unique_ptr<BlobstoreClient> new_blobstore_client() {
...  return std::make_unique<BlobstoreClient>();
...}
```

```console
cxx-demo$  cargo run
  Running `target/debug/cxx-demo`

blobid = 9851996977040795552
tags = ["rust"]
```

*You've now seen all the code involved in the tutorial. It's available all
together in runnable form in the* demo *directory of
<https://github.com/dtolnay/cxx>. You can run it directly without stepping
through the steps above by running `cargo run` from that directory.*

<br>

# Takeaways

The key contribution of CXX is it gives you Rust&ndash;C++ interop in which
*all* of the Rust side of the code you write *really* looks like you are just
writing normal Rust, and the C++ side *really* looks like you are just writing
normal C++.

You've seen in this tutorial that none of the code involved feels like C or like
the usual perilous "FFI glue" prone to leaks or memory safety flaws.

An expressive system of opaque types, shared types, and key standard library
type bindings enables API design on the language boundary that captures the
proper ownership and borrowing contracts of the interface.

CXX plays to the strengths of the Rust type system *and* C++ type system *and*
the programmer's intuitions. An individual working on the C++ side without a
Rust background, or the Rust side without a C++ background, will be able to
apply all their usual intuitions and best practices about development in their
language to maintain a correct FFI.

<br><br>
