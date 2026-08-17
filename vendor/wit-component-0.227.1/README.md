# `wit-component`

`wit-component` is a crate for creating and interacting with WebAssembly
components based on the [component model proposal][model].

## CLI usage

The `wit-component` crate is available through the `wasm-tools` CLI suite under
two subcommands:

```
# Create a component from the input core wasm module
$ wasm-tools component new core.wasm -o component.wasm

# Extract a `*.wit` interface from a component
$ wasm-tools component wit component.wasm
```

## Features

* Creates WebAssembly [component binaries][model] from input core WebAssembly
  modules. Input modules communicate with the [canonical ABI] to imported and
  exported interfaces described with [`*.wit` files][wit]. The wit interface is
  required to be embedded directly in the core wasm binary.

* Supports "adapters" which can be used to bridge legacy core WebAssembly
  imported functions into [component model][model] functions. Adapters are
  themselves core wasm binaries which will be embedded into the final component.
  An adapter's exports can be imported by the main core wasm binary and the
  adapter can then call component model imports.

* A [`*.wit`][wit] interface can be extracted from an existing component to
  see the interface that it exports and intends to import.

[model]: https://github.com/webassembly/component-model
[canonical ABI]: https://github.com/WebAssembly/component-model/blob/main/design/mvp/CanonicalABI.md
[wit]: https://github.com/WebAssembly/component-model/blob/main/design/mvp/WIT.md

## Usage

Note that this crate is intended to be a low-level detail of tooling for
components. Developers will not necessarily interact with this tooling
day-to-day, instead using wrappers such as
[`cargo-component`](https://github.com/bytecodealliance/cargo-component) which
will automatically execute `wit-component` to produce component binaries.

First `wit-component` supports the wasm-based encoding of a WIT package:

```sh
$ cat demo.wit
package my:demo;

interface host {
  hello: func();
}

world demo {
  import host;
}

$ wasm-tools component wit demo.wit -o demo.wasm --wasm

# The output `demo.wasm` is a valid component binary
$ wasm-tools validate --features component-model demo.wasm
$ wasm-tools print demo.wasm

# The `*.wit` file can be recovered from the `demo.wasm` as well
$ wasm-tools component wit demo.wasm
```

Toolchain authors can use `wit-component` to embed this component types section
into a core wasm binary. For a small demo here a raw `*.wat` wasm text file will
be used where the `demo.wit` argument is specified manually, however.

```sh
$ cat demo.core.wat
(module
  (import "my:demo/host" "hello" (func))
)

$ wasm-tools component embed demo.wit --world demo demo.core.wat -o demo.wasm

# See that there's a new `component-type` custom section
$ wasm-tools objdump demo.wasm

# Convert the core wasm into a component now
$ wasm-tools component new demo.wasm -o demo.component.wasm

# Like before the output `demo.wasm` is a valid component binary
$ wasm-tools validate --features component-model demo.component.wasm
$ wasm-tools print demo.component.wasm

# Additionally like before the `*.wit` interface can still be extracted
$ wasm-tools component wit demo.component.wasm
```

Here the `demo.component.wasm` can now be shipped to a component runtime or
embedded into hosts.

## Custom Section Format

Toolchains producing components typically have a workflow that looks something
like this:

1. A bindings generator, often based on [`wit-bindgen`], is used to generate
  source code.
2. A user writes source code against these bindings.
3. The language's compiler runs and produces a core WebAssembly module.
4. The core WebAssembly module is passed to this crate, producing a component.

Steps (1) and (4) both take WIT as input and the goal is to make it such that
you don't have to specify the same WIT on both ends. Instead the WIT from (1)
should be seamlessly passed through to (4) without developers having to
explicitly opt-in to it otherwise. To do this, the `wit-component` crate and
componentization process uses a custom section.

Part of the generated bindings from (1) above is source code (or similar) to
instruct the language to embed a custom section in the final binary. The custom
section's name must start with `component-type` but can have any number of
characters following it (e.g. it must match the regex `^component-type`). A
module is allowed to have any number of custom sections, including zero.

Each custom section describes a WIT `world`. The unit of bindings generation is
a `world`, and each `world` used during step (1), which may possibly be in
multiple separate libraries, will be encoded into custom sections. Custom
sections typically have a name that includes the bindings generator,
the bindings generator's version, the world, and an optional use-provided
string. The contents of the custom section is a wasm-encoded WIT world.

An example of this is that for this document:

```wit
package a:b;

world foo {
    import host: func();
    export guest: func();
}
```

this WIT world is encoded to wasm as:

```wasm
(component
  (type (;0;)
    (component
      (type (;0;)
        (component
          (type (;0;) (func))
          (import "host" (func (;0;) (type 0)))
          (export (;1;) "guest" (func (type 0)))
        )
      )
      (export (;0;) "a:b/foo" (component (type 0)))
    )
  )
  (export (;1;) "foo" (type 0))
  (@custom "package-docs" "\00{}")
  (@producers
    (processed-by "wit-component" "0.218.0")
  )
)
```

More details about the wasm encoding of WIT can [be found
upstream](https://github.com/WebAssembly/component-model/blob/main/design/mvp/WIT.md).

Note here that the WIT world is just the single `component`-type export needed
to decode a single world, no other items are encoded into this wasm.

When determining the `world` for a wasm component the componentization process
will read the input module, remove all sections that match `^component-type`,
extract the `world` that the type represents, and then "merge" all of the worlds
together. This merging process can fail if there are conflicts, but otherwise
duplicate imports are unified together.

The final componentization process then understands the WIT `world` that's being
used and assumes that the component follows the naming scheme in
[`BuildTargets.md`](https://github.com/WebAssembly/component-model/pull/378)
upstream. (note that legacy names are also accepted at this time)

## Merging Worlds

Part of what `wit-component` does when creating a component is that it will
merge WIT `world`s from a number of sources. For example each custom section
above may have a world. For tools like `wasm-component-ld` worlds may also be
supplied on the command line. The final component, however, can only have one
set of imports and exports so a single `world` needs to be created from all
these input worlds.

There are two workhorse methods for performing this merging process and both
live within the `wit-parser` crate:

* [`wit_parser::Resolve::merge_worlds`]
* [`wit_parser::Resolve::merge_world_imports_based_on_semver`]

The first is unconditionally used to merge all worlds together. Each custom
section, input argument, etc, are all merged with the
[`wit_parser::Resolve::merge_worlds`] method. This method may fail in certain
situations, primarily if there are duplicate exports specified in different
worlds (even if they're the same name they can't unify to one export as it's not
clear which export to pick). There are some other niche situations as well in
which the merging process can fail.

Once all worlds are merged together with [`wit_parser::Resolve::merge_worlds`]
there is then an optional, but enabled by default, step to "trim" the world
down based on semver versions. This method is
[`wit_parser::Resolve::merge_world_imports_based_on_semver`] and is used to
change a world such as this:

```wit
world {
    import wasi:cli/environment@0.2.0;
    import wasi:cli/environment@0.2.1;
}
```

into this

```wit
world {
    import wasi:cli/environment@0.2.1;
}
```

Notably imports are deduplicated based on their semver versions of interfaces.
The maximum version of the interface is always selected in the end. Only
interfaces which are semver compatible are deduplicated, for example this world
cannot be deduplicated further:

```wit
world {
    import wasi:cli/environment@0.2.0;
    import wasi:cli/environment@0.3.0;
}
```

The main purpose for this deduplication is to enable internally within a
component various libraries and runtimes to evolve at a different pace with
respect to interface versions. By definition in WIT it should always be possible
to use a larger versions, so older imports are automatically "upgraded" to
newer imports in these situations. This ensures that, for example, only one
`wasi:io/poll/pollable` resource will be imported into the final component. If
two were imported there would be no way to actually functionally use the
resulting component.

Note that this semver-deduplication process is enabled by default but can be
disabled through various options and flags in tooling.

[`wit_parser::Resolve::merge_worlds`]: https://docs.rs/wit-parser/latest/wit_parser/struct.Resolve.html#method.merge_worlds
[`wit_parser::Resolve::merge_world_imports_based_on_semver`]: https://docs.rs/wit-parser/latest/wit_parser/struct.Resolve.html#method.merge_world_imports_based_on_semver
