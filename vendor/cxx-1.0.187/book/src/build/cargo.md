{{#title Cargo-based setup — Rust ♡ C++}}
# Cargo-based builds

As one aspect of delivering a good Rust&ndash;C++ interop experience, CXX turns
Cargo into a quite usable build system for C++ projects published as a
collection of crates.io packages, including a consistent and frictionless
experience `#include`-ing C++ headers across dependencies.

## Canonical setup

CXX's integration with Cargo is handled through the [cxx-build] crate.

[cxx-build]: https://docs.rs/cxx-build

```toml,hidelines=...
# Cargo.toml
...[package]
...name = "..."
...version = "..."
...edition = "2021"

[dependencies]
cxx = "1.0"

[build-dependencies]
cxx-build = "1.0"
```

The canonical build script is as follows. The indicated line returns a
[`cc::Build`] instance (from the usual widely used `cc` crate) on which you can
set up any additional source files and compiler flags as normal.

[`cc::Build`]: https://docs.rs/cc/1.0/cc/struct.Build.html

```rust,noplayground
// build.rs

fn main() {
    cxx_build::bridge("src/main.rs")  // returns a cc::Build
        .file("src/demo.cc")
        .std("c++11")
        .compile("cxxbridge-demo");

    println!("cargo:rerun-if-changed=src/demo.cc");
    println!("cargo:rerun-if-changed=include/demo.h");
}
```

The `rerun-if-changed` lines are optional but make it so that Cargo does not
spend time recompiling your C++ code when only non-C++ code has changed since
the previous Cargo build. By default without any `rerun-if-changed`, Cargo will
re-execute the build script after *any* file changed in the project.

If stuck, try comparing what you have against the *demo/* directory of the CXX
GitHub repo, which maintains a working Cargo-based setup for the blobstore
tutorial (chapter 3).

## Header include paths

With cxx-build, by default your include paths always start with the crate name.
This applies to both `#include` within your C++ code, and `include!` in the
`extern "C++"` section of your Rust cxx::bridge.

Your crate name is determined by the `name` entry in Cargo.toml.

For example if your crate is named `yourcratename` and contains a C++ header
file `path/to/header.h` relative to Cargo.toml, that file will be includable as:

```cpp
#include "yourcratename/path/to/header.h"
```

A crate can choose a prefix for its headers that is different from the crate
name by modifying **[`CFG.include_prefix`][CFG]** from build.rs:

[CFG]: https://docs.rs/cxx-build/*/cxx_build/static.CFG.html

```rust,noplayground
// build.rs

use cxx_build::CFG;

fn main() {
    CFG.include_prefix = "my/project";

    cxx_build::bridge(...)...
}
```

Subsequently the header located at `path/to/header.h` would now be includable
as:

```cpp
#include "my/project/path/to/header.h"
```

The empty string `""` is a valid include prefix and will make it possible to
have `#include "path/to/header.h"`. However, if your crate is a library, be
considerate of possible name collisions that may occur in downstream crates. If
using an empty include prefix, you'll want to make sure your headers' local path
within the crate is sufficiently namespaced or unique.

## Including generated code

If your `#[cxx::bridge]` module contains an `extern "Rust"` block i.e. types or
functions exposed from Rust to C++, or any shared data structures, the
CXX-generated C++ header declaring those things is available using a `.rs.h`
extension on the Rust source file's name.

```cpp
// the header generated from path/to/lib.rs
#include "yourcratename/path/to/lib.rs.h"
```

For giggles, it's also available using just a plain `.rs` extension as if you
were including the Rust file directly. Use whichever you find more palatable.

```cpp
#include "yourcratename/path/to/lib.rs"
```

## Including headers from dependencies

You get to include headers from your dependencies, both handwritten ones
contained as `.h` files in their Cargo package, as well as CXX-generated ones.

It works the same as an include of a local header: use the crate name (or their
include\_prefix if their crate changed it) followed by the relative path of the
header within the crate.

```cpp
#include "dependencycratename/path/to/their/header.h`
```

Note that cross-crate imports are only made available between **direct
dependencies**. You must directly depend on the other crate in order to #include
its headers; a transitive dependency is not sufficient.

Additionally, headers from a direct dependency are only importable if the
dependency's Cargo.toml manifest contains a `links` key. If not, its headers
will not be importable from outside of the same crate. See *[the `links`
manifest key][links]* in the Cargo reference.

[links]: https://doc.rust-lang.org/cargo/reference/build-scripts.html#the-links-manifest-key

<br><br><br>

# Advanced features

The following CFG settings are only relevant to you if you are writing a library
that needs to support downstream crates `#include`-ing its C++ public headers.

## Publicly exporting header directories

**[`CFG.exported_header_dirs`][CFG]** (vector of absolute paths) defines a set
of additional directories from which the current crate, directly dependent
crates, and further crates to which this crate's headers are exported (more
below) will be able to `#include` headers.

Adding a directory to `exported_header_dirs` is similar to adding it to the
current build via the `cc` crate's [`Build::include`], but *also* makes the
directory available to downstream crates that want to `#include` one of the
headers from your crate. If the dir were added only using `Build::include`, the
downstream crate including your header would need to manually add the same
directory to their own build as well.

[`Build::include`]: https://docs.rs/cc/1/cc/struct.Build.html#method.include

When using `exported_header_dirs`, your crate must also set a `links` key for
itself in Cargo.toml. See [*the `links` manifest key*][links]. The reason is
that Cargo imposes no ordering on the execution of build scripts without a
`links` key, which means the downstream crate's build script might otherwise
execute before yours decides what to put into `exported_header_dirs`.

### Example

One of your crate's headers wants to include a system library, such as `#include
"Python.h"`.

```rust,noplayground
// build.rs

use cxx_build::CFG;
use std::path::PathBuf;

fn main() {
    let python3 = pkg_config::probe_library("python3").unwrap();
    let python_include_paths = python3.include_paths.iter().map(PathBuf::as_path);
    CFG.exported_header_dirs.extend(python_include_paths);

    cxx_build::bridge("src/bridge.rs").compile("demo");
}
```

### Example

Your crate wants to rearrange the headers that it exports vs how they're laid
out locally inside the crate's source directory.

Suppose the crate as published contains a file at `./include/myheader.h` but
wants it available to downstream crates as `#include "foo/v1/public.h"`.

```rust,noplayground
// build.rs

use cxx_build::CFG;
use std::path::Path;
use std::{env, fs};

fn main() {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let headers = Path::new(&out_dir).join("headers");
    CFG.exported_header_dirs.push(&headers);

    // We contain `include/myheader.h` locally, but
    // downstream will use `#include "foo/v1/public.h"`
    let foo = headers.join("foo").join("v1");
    fs::create_dir_all(&foo).unwrap();
    fs::copy("include/myheader.h", foo.join("public.h")).unwrap();

    cxx_build::bridge("src/bridge.rs").compile("demo");
}
```

## Publicly exporting dependencies

**[`CFG.exported_header_prefixes`][CFG]** (vector of strings) each refer to the
`include_prefix` of one of your direct dependencies, or a prefix thereof. They
describe which of your dependencies participate in your crate's C++ public API,
as opposed to private use by your crate's implementation.

As a general rule, if one of your headers `#include`s something from one of your
dependencies, you need to put that dependency's `include_prefix` into
`CFG.exported_header_prefixes` (*or* their `links` key into
`CFG.exported_header_links`; see below). On the other hand if only your C++
implementation files and *not* your headers are importing from the dependency,
you do not export that dependency.

The significance of exported headers is that if downstream code (crate **𝒜**)
contains an `#include` of a header from your crate (**ℬ**) and your header
contains an `#include` of something from your dependency (**𝒞**), the exported
dependency **𝒞** becomes available during the downstream crate **𝒜**'s build.
Otherwise the downstream crate **𝒜** doesn't know about **𝒞** and wouldn't be
able to find what header your header is referring to, and would fail to build.

When using `exported_header_prefixes`, your crate must also set a `links` key
for itself in Cargo.toml.

### Example

Suppose you have a crate with 5 direct dependencies and the `include_prefix` for
each one are:

- "crate0"
- "group/api/crate1"
- "group/api/crate2"
- "group/api/contrib/crate3"
- "detail/crate4"

Your header involves types from the first four so we re-export those as part of
your public API, while crate4 is only used internally by your cc file not your
header, so we do not export:

```rust,noplayground
// build.rs

use cxx_build::CFG;

fn main() {
    CFG.exported_header_prefixes = vec!["crate0", "group/api"];

    cxx_build::bridge("src/bridge.rs")
        .file("src/impl.cc")
        .compile("demo");
}
```

<br>

For more fine grained control, there is **[`CFG.exported_header_links`][CFG]**
(vector of strings) which each refer to the `links` attribute ([*the `links`
manifest key*][links]) of one of your crate's direct dependencies.

This achieves an equivalent result to `CFG.exported_header_prefixes` by
re-exporting a C++ dependency as part of your crate's public API, except with
finer control for cases when multiple crates might be sharing the same
`include_prefix` and you'd like to export some but not others. Links attributes
are guaranteed to be unique identifiers by Cargo.

When using `exported_header_links`, your crate must also set a `links` key for
itself in Cargo.toml.

### Example

```rust,noplayground
// build.rs

use cxx_build::CFG;

fn main() {
    CFG.exported_header_links.push("git2");

    cxx_build::bridge("src/bridge.rs").compile("demo");
}
```
