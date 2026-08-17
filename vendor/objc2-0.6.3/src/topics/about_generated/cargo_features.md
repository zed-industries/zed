# Cargo features and long compile-times in framework crates

Each framework crate has a set of Cargo features that control which parts of
it that is enabled. These are split into two categories; file and dependency
features.

**Most features are enabled by default** for ease of use for newcomers / in
small hobby projects, but if you're developing a library for others to use, it
is recommended that you disable them by adding `default-features = false`
[to your `Cargo.toml`][cargo-dep-features].

This can vastly help reduce the compile-time of the framework crates.

[cargo-dep-features]: https://doc.rust-lang.org/cargo/reference/features.html#dependency-features


## File features

Each framework C header corresponds to one Cargo feature, and everything that
was declared inside of that header is locked behind that Cargo feature.

As an example, let's use `MetalKit`. This framework has four public C headers,
`MTKDefines.h`, `MTKModel.h`, `MTKTextureLoader.h` and `MTKView.h`. This in
turn means we get four Cargo features in `objc2-metal-kit`, `MTKDefines`,
`MTKModel`, `MTKTextureLoader` and `MTKView`, that enables the functionality
exposed by each of those headers, as well as any required dependency features
(e.g. `MTKModel.h` uses `MTLDevice`, so `objc2-metal/MTLDevice` is enabled for
you).


## Dependency features

As you can see above, frameworks rarely stand alone, instead they often have
some sort of dependency on other frameworks. Some of these dependencies are
considered required, and enabled by default (often `objc2-foundation`), while
for some it makes sense to allow control of whether the dependency is enabled.

Let's keep using `MetalKit` as the example. By default, `objc2-metal-kit` will
import the dependencies `objc2`, `objc2-foundation` and `objc2-metal`, since
those are central to how the `MetalKit` works.

But it also has an optional dependency on `objc2-app-kit` and `block2`, since
those are not always required; in this case, `objc2-app-kit` is only needed if
you want to use the `MTKView` class, so it would be wasteful to enable the
dependency if you didn't need that class.

Such optional dependencies can be enabled with Cargo features of the same name
as the dependency.


### Linking and minimum version compatibility

A few select dependency features are **not** enabled by default, as these
would raise the minimum supported version of your application. An example is
the `objc2-uniform-type-identifiers` dependency of the `objc2-app-kit` crate,
which was first introduced in macOS 11.0. If you want to use capabilities from
that framework in `objc2-app-kit`, you have to enable the feature manually.

(Note that this _could_ also have been worked around by the user with the
`-weak_framework UniformTypeIdentifiers` linker flag, but that's not very
discoverable, so `objc2` chooses to default to making your application
portable across the full set of versions that Rust supports).
