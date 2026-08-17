# gl_generator

[![Version](https://img.shields.io/crates/v/gl_generator.svg)](https://crates.io/crates/gl_generator)
[![License](https://img.shields.io/crates/l/gl_generator.svg)](https://github.com/brendanzab/gl-rs/blob/master/LICENSE)
[![Downloads](https://img.shields.io/crates/d/gl_generator.svg)](https://crates.io/crates/gl_generator)

Code generators for creating bindings to the Khronos OpenGL APIs.

## Usage

If you need a specific version of OpenGL, or you need a different API
(OpenGL ES, EGL, WGL, GLX), or if you need certain extensions, you should use
the `gl_generator` plugin instead.

See [gfx_gl](https://github.com/gfx-rs/gfx_gl) for an example of using a
custom gfx-rs loader for a project.

Add this to your `Cargo.toml`:

```toml
[build-dependencies]
gl_generator = "0.5.0"
```

Under the `[package]` section, add:

```toml
build = "build.rs"
```

Create a `build.rs` to pull your specific version/API:

```rust
extern crate gl_generator;

use gl_generator::{Registry, Api, Profile, Fallbacks, GlobalGenerator};
use std::env;
use std::fs::File;
use std::path::Path;

fn main() {
    let dest = env::var("OUT_DIR").unwrap();
    let mut file = File::create(&Path::new(&dest).join("bindings.rs")).unwrap();

    Registry::new(Api::Gl, (4, 5), Profile::Core, Fallbacks::All, [])
        .write_bindings(GlobalGenerator, &mut file)
        .unwrap();
}
```

Then use it like this:

```rust
mod gl {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

/// Simple loading example
fn main() {
    let window = ...;

    // Assuming `window` is GLFW: initialize, and make current
    gl::load_with(|s| window.get_proc_address(s) as *const _);
}
```

The `build.rs` file will generate all the OpenGL functions in a file named,
`bindings.rs` plus all enumerations, and all types in the `types` submodule.

## Generator types

### Global generator

The global generator is the one used by default by the `gl` crate. See above
for more details.

### Struct generator

The struct generator is a cleaner alternative to the global generator.

The main difference is that you must call `gl::Gl::load_with` instead of
`gl::load_with`, and this functions returns a struct of type `Gl`. The OpenGL
functions are not static functions but member functions in this `Gl` struct.
This is important when the GL functions are associated with the current
context, as is true on Windows.

The enumerations and types are still static and available in a similar way as
in the global generator.

### Static generator

The static generator generates plain old bindings. You don't need to load the
functions.

This generator should only be used only if the platform you are compiling for
is guaranteed to support the requested API. Otherwise you will get a
compilation error.
For example, you can use it for WGL and OpenGL 1.1 on Windows or GLX and
OpenGL 1.3 on Linux, because Windows and Linux are guaranteed to provide
implementations for these APIs.

You will need to manually provide the linkage. For example to use WGL or
OpenGL 1.1 on Windows, you will need to add
`#[link="OpenGL32.lib"] extern {}` somewhere in your code.

### Custom Generators

The `gl_generator` can be extended with custom generators. This is a niche
feature useful only in very rare cases. To create a custom generator, implement
the `gl_generator::Generator` trait. See the source of the
`gl_generator::generators` module for examples.

Various utility functions are provided in the `generators` module, but the api
is unstable, so it has been placed behind a feature flag. In access these
functions, you will need to add the `"unstable_generator_utils"` feature to
your `Cargo.toml`:

```toml
[build-dependencies.gl_generator]
version = "0.4.2"
features = ["unstable_generator_utils"]
```

## Extra features

The global and struct generators will attempt to use fallbacks functions when
they are available. For example, if `glGenFramebuffers` cannot be loaded it will
also attempt to load `glGenFramebuffersEXT` as a fallback.

## Changelog

### v0.5.0

- Rename `Ns` to `API`, and expose at the top level
- Remove the need for clients to depend on the `khronos_api` crate by
  determining the XML source based on the requested `API`
- Use a `(u8, u8)` instead of a string for the target version number
- Use a `Profile` enum instead of a string for the profile
- Remove unused fields from `Registry`
- Accept types satisfying `AsRef<[&str]>` for extension lists
- Separate parsing and generation stages in API
- Hide `registry::{Feature, Filter, Require, Remove, Extension}` types from the
  public API
- Move `registry::{Fallbacks, Api, Profile}` types to top level module
- Remove `GlxOpcode::type` field
- Make `ty` fields on `Enum` and `Binding` take `Cow<'static, str>`s to reduce
  allocations

### v0.4.2

- Update crate metadata

### v0.4.1

- Upgrade `khronos_api` to v1.0.0

### v0.4.0

- Upgrade `xml-rs` to v0.2.2
- Use `raw::c_void` for `GLvoid`
- Remove `registry::{Group, EnumNs, CmdNs}`
- Remove `groups` field from `registry::Registry`
- Remove `is_safe` field from `registry::Cmd`
- Remove `comment` field from `registry::{Require, Remove, GlxOpcode}`
- Downgrade `khronos_api` to be a dev-dependency
