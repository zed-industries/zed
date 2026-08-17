# Cargo features in framework crates

Each framework crate has a set of Cargo features that control which parts of
it that is enabled. These are split into two categories; file and dependency
features.

This is quite important for compilation speed, but if you don't want to bother
with it, such as when just starting a new project and experimenting or when
running an example, use the `"all"` feature.


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
