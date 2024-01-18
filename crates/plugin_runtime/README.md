# Zed's Plugin Runner
This is a short guide that aims to answer the following questions:

- How do plugins work in Zed?
- How can I create a new plugin?
- How can I integrate plugins into a part of Zed?

### Nomenclature

- Host-side: The native Rust runtime managing plugins, e.g. Zed.
- Guest-side: The wasm-based runtime that plugins use.

## How plugins work
Zed's plugins are WebAssembly (Wasm) based, and have access to the WebAssembly System Interface (WASI), which allows for permissions-based access to subsets of system resources, like the filesystem.

To execute plugins, Zed's plugin system uses the sandboxed [`wasmtime`](https://wasmtime.dev/) runtime, which is Open Source and developed by the [Bytecode Alliance](https://bytecodealliance.org/). Wasmtime uses the [Cranelift](https://docs.rs/cranelift/latest/cranelift/) codegen library to compile plugins to native code.

Zed has three `plugin` crates that implement different things:

1. `plugin_runtime` is a host-side library that loads and runs compiled `Wasm` plugins, in addition to setting up system bindings. This crate should be used host-side

2. `plugin` contains a prelude for guest-side plugins to depend on. It re-exports some required crates (e.g. `serde`, `bincode`) and provides some necessary macros for generating bindings that `plugin_runtime` can hook into.

3. `plugin_macros` implements the proc macros required by `plugin`, like the `#[import]` and `#[export]` attribute macros, and should also be used guest-side.

### ABI
The interface between the host Rust runtime ('Runtime') and plugins implemented in Wasm ('Plugin') is pretty simple.

When calling a guest-side function, all arguments are serialized to bytes and passed through `Buffer`s. We currently use `serde` + [`bincode`](https://docs.rs/bincode/latest/bincode/) to do this serialization. This means that any type that can be serialized using serde can be passed across the ABI boundary. For types that represent resources that cannot pass the ABI boundary (e.g. `Rope`), we are working on an opaque callback-based system.

> **Note**: It's important to note that there is a draft ABI standard for Wasm called WebAssembly Interface Types (often abbreviated `WITX`). This standard is currently not stable and only experimentally supported in some runtimes. Once this proposal becomes stable, it would be a good idea to transition towards using WITX as the ABI, rather than the rather rudimentary `bincode` ABI we have now.

All `Buffer`s are stored in Wasm linear memory (Wasm memory). A `Buffer` is a pointer, length pair to a byte array somewhere in Wasm memory. A `Buffer` itself is represented as a pair of two 4-byte (`u32`) fields:

```rust
struct Buffer {
    ptr: u32,
    len: u32,
}
```

Which we encode as a single `u64` when crossing the ABI boundary:

```
+-------+-------+
| ptr   | len   |
+-------+-------+
        |
~ ~ ~ ~ | ~ ~ ~ ~ spOoky ABI boundary O.o
        V
+---------------+
| u64           |
+---------------+
```

All functions that a plugin exports or imports have the following properties:

- A function signature of `fn(u64) -> u64`, where both the argument (input) and return type (output) are a `Buffer`:

    - The input `Buffer` will contain the input arguments serialized to `bincode`.
    - The output `Buffer` will contain the output arguments serialized to `bincode`.

- Have a name starting with two underscores.

Luckily for us, we don't have to worry about mangling names or writing serialization code. The `plugin::prelude::*` defines a couple of macros—aptly named `#[import]` and `#[export]`—that generate all serialization code and perform all mangling of names requisite for crossing the ABI boundary.

There are also a couple important things every plugin must have:

- `__alloc_buffer` function that, given a `u32` length, returns a `u32` pointer to a buffer of that length.
- `__free_buffer` function that, given a buffer encoded as a `u64`, frees the buffer at the given location, and does not return anything.

Luckily enough for us yet again, the `plugin` prelude defines two ready-made versions of these functions, so you don't have to worry about implementing them yourselves.

So, what does importing and exporting functions from a plugin look like in practice? I'm glad you asked...

## Creating new plugins
Since Zed's plugin system uses Wasm + WASI, in theory any language that compiles to Wasm can be used to write plugins. In practice, and out of practicality, however, we currently only really support plugins written in Rust.

A plugin is just a rust crate like any other. All plugins embedded in Zed are located in the `plugins` folder in the root. These plugins will automatically be compiled, optimized, and recompiled on change, so it's recommended that when creating a new plugin you create it there.

As plugins are compiled to Wasm + WASI, you need to have the `wasm32-wasi` toolchain installed on your system. If you don't have it already, a little rustup magick will do the trick:

```bash
rustup target add wasm32-wasi
```

### Configuring a plugin
After you've created a new plugin in `plugins` using `cargo new --lib`, edit your `Cargo.toml` to ensure that it looks something like this:

```toml
[package]
name = "my_very_cool_incredible_plugin_with_a_short_name_of_course"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["cdylib"]

[dependencies]
plugin = { path = "../../crates/plugin" }

[profile.release]
opt-level = "z"
lto = true
```

Here's a quick explainer of what we're doing:

- `crate-type = ["cdylib"]` is used because a plugin essentially acts a *library*, exposing functions with specific signatures that perform certain tasks. This key ensures that the library is generated in a reproducible manner with a layout `plugin_runtime` knows how to hook into.

- `plugin = { path = "../../crates/plugin" }` is used so we have access to the prelude, which has a few useful functions and can automatically generate serialization glue code for us.

- `[profile.release]` these options wholistically optimize for size, which will become increasingly important as we add more plugins.

### Importing and Exporting functions
To import or export a function, all you need are two things:

1. Make sure that you've imported `plugin::prelude::*`
2. Annotate your function or signature with `#[export]` or `#[import]` respectively.

Here's an example plugin that doubles the value of every float in a `Vec<f64>` passed into it:

```rust
use plugin::prelude::*;

#[export]
pub fn double(mut x: Vec<f64>) -> Vec<f64> {
    x.into_iter().map(|x| x * 2.0).collect()
}
```

All the serialization code is automatically generated by `#[export]`.

You can specify functions that must be defined host-side by using the `#[import]` attribute. This attribute must be attached to a function signature:

```rust
use plugin::prelude::*;

#[import]
fn run(command: String) -> Vec<u8>;
```

The `#[import]` macro will generate a function body that performs the proper serialization/deserialization needed to call out to the host rust runtime. Note that the same `serde` + `bincode` + `Buffer` ABI is used for both `#[import]` and `#[export]`.

> **Note**: If you'd like to see an example of importing and exporting functions, check out the `test_plugin`, which can be found in the `plugins` directory.

## Integrating plugins into Zed
Currently, plugins are used to add support for language servers to Zed. Plugins should be fairly simple to integrate for library-like applications. Here's a quick overview of how plugins work:

### Normal vs Precompiled plugins
Plugins in the `plugins` directory are automatically recompiled and serialized to disk when compiling Zed. The resulting artifacts can be found in the `plugins/bin` directory. For each `plugin`, you should see two files:

- `plugin.wasm` is the plugin compiled to Wasm. As a baseline, this should be about 4MB for debug builds and 2MB for release builds, but it depends on the specific plugin being built.

- `plugin.wasm.pre` is the plugin compiled to Wasm *and additionally* precompiled to host-platform-specific native code, determined by the `TARGET` cargo exposes at compile-time. This should be about 700KB for debug builds and 500KB in release builds. Each plugin takes about 1 or 2 seconds to compile to native code using cranelift, so precompiling plugins drastically reduces the startup time required to begin to run a plugin.

For all intents and purposes, it is *highly recommended* that you use precompiled plugins where possible, as they are much more lightweight and take much less time to instantiate.

### Instantiating a plugin
So you have something you'd like to add a plugin for. What now? The general pattern for adding support for plugins is as follows:

#### 1. Create a struct to hold the plugin
To call the functions that a plugin exports host-side, you need to have 'handles' to those functions. Each handle is typed and stored in `WasiFn<A, R>` where `A: Serialize` and `R: DeserializeOwned`.

For example, let's suppose we're creating a plugin that:

1. formats a message
2. processes a list of numbers somehow

We could create a struct for this plugin as follows:

```rust
use plugin_runtime::{WasiFn, Plugin};

pub struct CoolPlugin {
    format:  WasiFn<String, String>,
    process: WasiFn<Vec<f64>, f64>,
    runtime: Plugin,
}
```

Note that this plugin also holds an owned reference to the runtime, which is stored in the `Plugin` type. In asynchronous or multithreaded contexts, it may be required to put `Plugin` behind an `Arc<Mutex<Plugin>>`. Although plugins expose an asynchronous interface, the underlying Wasm engine can only execute a single function at a time.

> **Note**: This is a limitation of the WebAssembly standard itself. In the future, to work around this, we've been considering starting a pool of plugins, or instantiating a new plugin per call (this isn't as bad as it sounds, as instantiating a new plugin only takes about 30µs).

In the following steps, we're going to build a plugin and extract handles to fill this struct we've created.

#### 2. Bind all imported functions
While a plugin can export functions, it can also import them. We'll refer to the host-side functions that a plugin imports as 'native' functions. Native functions are represented using callbacks, and both synchronous and asynchronous callbacks are supported.

To bind imported functions, the first thing we need to do is create a new plugin using `PluginBuilder`. `PluginBuilder` uses the builder pattern to configure a new plugin, after which calling the `init` method will instantiate the `Plugin`.

You can create a new plugin builder as follows:

```rust
let builder = PluginBuilder::new_with_default_ctx();
```

This creates a plugin with a sensible default set of WASI permissions, namely the ability to write to `stdout` and `stderr` (note that, by default, plugins do not have access to `stdin`). For more control, you can use `PluginBuilder::new` and pass in a `WasiCtx` manually.

##### Synchronous Functions
To add a sync native function to a plugin, use the `.host_function` method:

```rust
let builder = builder.host_function(
    "add_f64",
    |(a, b): (f64, f64)| a + b,
).unwrap();
```

The `.host_function` method takes two arguments: the name of the function, and a sync callback that implements it. Note that this name must match the name of the function declared in the plugin exactly. For example, to use the `add_f64` from a plugin, you must include the following `#[import]` signature:

```rust
use plugin::prelude::*;

#[import]
fn add_f64(a: f64, b: f64) -> f64;
```

Note that the specific names of the arguments do not matter, as long as they are unique. Once a function has been imported, it may be used in the plugin as any other Rust function.

##### Asynchronous Functions
To add an async native function to a plugin, use the `.host_function_async` method:

```rust
let builder = builder.host_function_async(
    "half",
    |n: f64| async move { n / 2.0 },
).unwrap();
```

This method works exactly the same as the `.host_function` method, but requires a callback that returns an async future. On the plugin side, there is no distinction made between sync and async functions (as Wasm has no built-in notion of sync vs. async), so the required import signature should *not* use the `async` keyword:

```rust
use plugin::prelude::*;

#[import]
fn half(n: f64) -> f64;
```

All functions declared by the builder must be imported by the Wasm plugin, otherwise an error will be raised.

#### 3. Get the compiled plugin
Once all imports are marked, we can instantiate the plugin. To instantiate the plugin, simply call the `.init` method on a `PluginBuilder`:

```rust
let plugin = builder
    .init(
        PluginBinary::Precompiled(bytes),
    )
    .await
    .unwrap();
```

The `.init` method takes a single argument containing the plugin binary.

1. If not precompiled, use `PluginBinary::Wasm(bytes)`. This supports both the WebAssembly Textual format (`.wat`) and the WebAssembly Binary format (`.wasm`).

2. If precompiled, use `PluginBinary::Precompiled(bytes)`. This supports precompiled plugins ending in `.wasm.pre`. You need to be extra-careful when using precompiled plugins to ensure that the plugin target matches the target of the binary you are compiling.

The `.init` method is asynchronous, and must be `.await`ed upon. If the plugin is malformed or doesn't import the right functions, an error will be raised.

#### 4. Get handles to all exported functions
Once the plugin has been compiled, it's time to start filling in the plugin struct defined earlier. In the case of `CoolPlugin` from earlier, this can be done as follows:

```rust
let mut cool_plugin = CoolPlugin {
    format:  plugin.function("format").unwrap(),
    process: plugin.function("process").unwrap(),
    runtime: plugin,
};
```

Because the struct definition defines the types of functions we're grabbing handles to, it's not required to specify the types of the functions here.

Note that, yet again, the names of guest-side functions we import must match exactly. Here's an example of what that implementation might look like:

```rust
use plugin::prelude::*;

#[export]
pub fn format(message: String) -> String {
    format!("Cool Plugin says... '{}!'", message)
}

#[export]
pub fn process(numbers: Vec<f64>) -> f64 {
    // Process by calculating the average
    let mut total = 0.0;
    for number in numbers.into_iter() {
        total += number;
    }
    total / numbers.len()
}
```

That's it! Now you have a struct that holds an instance of a plugin. The last thing you need to know is how to call out the plugin you've defined...

### Using a plugin
To call a plugin function, use the async `.call` method on `Plugin`:

```rust
let average = cool_plugin.runtime
    .call(
        &cool_plugin.process,
        vec![1.0, 2.0, 3.0],
    )
    .await
    .unwrap();
```

The `.call` method takes two arguments:

1. A reference to the handle of the function we want to call.
2. The input argument to this function.

This method is async, and must be `.await`ed. If something goes wrong (e.g. the plugin panics, or there is a type mismatch between the plugin and `WasiFn`), then this method will return an error.

## Last Notes
This has been a brief overview of how the plugin system currently works in Zed. We hope to implement higher-level affordances as time goes on, to make writing plugins easier, and providing tooling so that users of Zed may also write plugins to extend their own editors.
