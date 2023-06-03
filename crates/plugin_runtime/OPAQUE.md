# Opaque handles to resources

Currently, Zed's plugin system only supports moving *data* (e.g. things you can serialize) across the boundary between guest-side plugin and host-side runtime. Resources, things you can't just copy, have been set aside for now. Given how important this is to Zed, I think it's about time we address this.

Managing resources is very important to Zed, because a lot of what Zed does is exactly that—managing resources. Each open buffer you're editing is a resource, as is the language server you're querying, or the collaboration session you're currently in. Therefore, writing a plugin system with deep integration with Zed requires some mechanism to manage resources.

The reason resources are problematic is because, unlike data, we can't pass resources across the ABI boundary. Wasm can't take references to host memory (and even if it could, that doesn't mean that it's a good idea). To add support for resources to plugins, we'd need three things:

1. Some sort of way for the host-side runtime to hang onto **references** to a resource. If the plugin requests to modify a resource, but we don't even know where that resource is, that's kinda bad, isn't it?

2. Some sort of way for the guest-side runtime to hang onto **handles** to a resource. We can't reference the resource directly from a plugin, but if a resource *has* been registered with the runtime, we can at least take a runtime-provided handle to that resource so that we may request that the runtime modify it in the future.

3. Some sort of way to **modify the resources** we're holding onto. This requires two things: some way for a plugin to request a modification, and some for the runtime to apply that modification. Here I'm using 'modification' in the most general sense, which includes, e.g. reading or writing to the resource, i.e. calling a method on it.

Luckily for us, managing resources across boundaries is a problem that languages have had to deal with for eons. File descriptors referencing resources managed by the kernel quintessentially defines of resource management, but this pattern is oft repeated in games, scripting languages, or surprise surprise, when writing plugins.

To see what managing resources in plugins could look like in Rust, we need look no further than Rhai. Rhai is a scripting language powered by a tree-walk interpreter written in Rust. It's pretty neat, but what we care about is not the language itself, but how it interfaces with Rust types.

In its [guide](https://rhai.rs/book/rust/custom-types.html), Rhai claims the following:

> Rhai works seamlessly with any Rust type, as long as it implements `Clone` as this allows the `Engine` to pass by value.

This doesn't mean that the underlying resources themselves need to be copied:

> \[Because Rhai works with types implementing `Clone`\] it is extremely easy to use Rhai with data types such as `Rc<...>`, `Arc<...>`, `Rc<RefCell<...>>`, `Arc<Mutex<...>>` etc.

Given that we have to register a resource with our plugin runtime before we use it, requiring the resource to be behind a shared reference makes sense, so I think the `Clone` bound is reasonable. So how does `Rhai` represent types under the hood?

> A custom type is stored in Rhai as a Rust trait object (specifically, a `dyn rhai::Variant`), with no restrictions other than being `Clone` (plus `Send + Sync` under the `sync` feature).

I'd be interested to know how Rhai disambiguates between different types if everything's a trait object under the hood.

Rhai actually exposes a pretty nice interface for working with native Rust types. We can register a type using `Engine::register_type::<T: Variant + Clone>()`. Internally, this just grabs the string name of the type for future reference.

> **Note**: Rhai uses strings, but I wonder if you could get away with something more compact using `TypeIds`. Maybe not, given that `TypeId`s are not deterministic across builds, and we'd need matching IDs both host-side and guest side.

In Rhai, we can alternatively use the method `Engine::register_type_with_name::<T: Variant + Clone>(name: &str)` if we have a different type name host-side (in Rust) and guest-side (in Rhai). 

With respect to Wasm plugins, I think an interface like this is fairly important, because we don't know whether the original plugin was written in Rust. (This may not be true now, because we write all the plugins Zed uses, but once we allow packaging and shipping plugins, it's important to maintain a consistent interface, because even Rust changes over time.)

Once we've registered a type, we can begin using this type in functions. We can add new function using the standard `Engine::register_fn` function, which has the following signature:

```rust
pub fn register_fn<N, A, F>(&mut self, name: N, func: F) -> &mut Self
where
    N: AsRef<str> + Into<Identifier>,
    F: RegisterNativeFunction<A, ()>,
```

This is quite complex, but under the hood it's fairly similar to our own `PluginBuilder::host_function` async method. Looking at `RegisterNativeFunction`, it seems as though this trait essentially provides methods that expose the `TypeID`s and type/param names of the arguments and return types of the function.

So once we register a function, what happens when we call it? Well, let me introduce you to my friend `Engine::call_native_fn`, whose type signature is too complex to list here.

> **Note**: Finding this function took like 7 levels of indirection from `eval`. It's surprising how much shuffling of data Rhai does under the hood, I bet you could probably make it a lot faster.

This takes and returns, like everything else in Rhai, an object of type `Dynamic`. We know that we can use native Rust types, so how does Rhai perform the conversion to and from `Dynamic`?

The secret lies in `Dynamic::try_cast::<T: Any>(self) -> Option<T>`. Like most dynamic scripting languages, Rhai uses a tagged `Union` to represent types. Remember `dyn Variant` from earlier? Rhai's `Union` has a variant, `Variant`, to hold the dynamic native types:

```rust
/// Any type as a trait object.
#[allow(clippy::redundant_allocation)]
Variant(Box<Box<dyn Variant>>, Tag, AccessMode),
```

Redundant allocations aside, To `try_cast` a `Dynamic` type to `T: Any`thing, we pattern match on `Union`. In the case of variant, we:

```rust
Union::Variant(v, ..) => (*v).as_boxed_any().downcast().ok().map(|x| *x),
```

Now Rhai can do this because it's implemented in Rust. In other words, unlike Wasm, Rhai scripts can, indirectly, hold references to places in host memory. For us to implement something like this for Wasm plugins, we'd have to keep track of a "`ResourcePool`"—alive for the duration of each function call—that we can check rust types into and out of.

 I think I've got a handle on how Rhai works now, so let's stop talking about Rhai and discuss what this opaque object system would look like if we implemented it in Rust.
 
 # Design Sketch
 
First things first, we'd have to generalize the arguments we can pass to and return from functions host-side. Currently, we support anything that's `serde`able. We'd have to create a new trait, say `Value`, that has blanket implementations for both `serde` and `Clone` (or something like this; if a type is both `serde` and `clone`, we'd have to figure out a way to disambiguate).
 
 We'd also create a `ResourcePool` struct that essentially is a `Vec` of `Box<dyn Any>`. When calling a function, all `Value` arguments that are resources (e.g. `Clone` instead of `serde`) would be typecasted to `dyn Any` and stored in the `ResourcePool`. 
 
 We'd probably also need a `Resource` trait that defines an associated handle for a resource. Something like this:
 
 ```rust
 pub trait Resource {
    type Handle: Serialize + DeserializeOwned;
    fn handle(index: u32) -> Self;
    fn index(handle: Self) -> u32;
 }
 ```
 
 Where a handle is just a dead-simple wrapper around a `u32`:
 
 ```rust 
 #[derive(Serialize, Deserialize)]
 pub struct CoolHandle(u32);
 ```
 
 It's important that this handle be accessible *both* host-side and plugin side. I don't know if this means that we have another crate, like `plugin_handles`, that contains a bunch of u32 wrappers, or something else. Because a `Resource::Handle` is just a u32, it's trivially `serde`, and can cross the ABI boundary.
 
 So when we add each `T: Resource` to the `ResourcePool`, the resource pool typecasts it to `Any`, appends it to the `Vec`, and returns the associated `Resource::Handle`. This handle is what we pass through to Wasm. 
 
 ```rust
 // Implementations and attributes omitted
 pub struct Rope { ... };
 pub struct RopeHandle(u32);
 impl Resource for Arc<RwLock<Rope>> { ... }
 
 let builder: PluginBuilder = ...;
 let builder = builder
    .host_fn_async(
        "append",
        |(rope, string): (Arc<RwLock<Rope>>, &str)| async move {
            rope.write().await.append(Rope::from(string))
        }
    )
    // ...
```

He're we're providing a host function, `append` that can be called from Wasm. To import this function into a plugin, we'd do something like the following:

```rust
use plugin::prelude::*;
use plugin_handles::RopeHandle;

#[import]
pub fn append(rope: RopeHandle, string: &str);
```

This allows us to perform an operation on a `Rope`, but how do we get a `RopeHandle` into a plugin? Well, as plugins, we can only acquire resources to handles we're given, so we'd need to expose a function that takes a handle. 

To illustrate that point, here's an example. First, we'd define a plugin-side function as follows:

```rust
// same file as above ...

#[export]
pub fn append_newline(rope: RopeHandle){
    append(rope, "\n");
}
```

Host-side, we'd treat this function like any other:

```rust
pub struct NewlineAppenderPlugin {
    append_newline: WasiFn<Arc<RwLock<Rope>>, ()>,
    runtime: Arc<Mutex<Plugin>>,
}
```

To call this function, we'd do the following:

```rust
let plugin: NewlineAppenderPlugin = ...;
let rope = Arc::new(RwLock::new(Rope::from("Hello World")));

plugin.lock().await.call(
    &plugin.append_newline,
    rope.clone(),
).await?;

// `rope` is now "Hello World\n"
```

So here's what calling `append_newline` would do, from the top:

1. First, we'd create a new `ResourcePool`, and insert the `Arc<RwLock<Rope>>`, creating a `RopeHandle` in the process. (We could also reuse a resource pool across calls, but the idea is that the pool only keeps track of resources for the duration of the call).

2. Then, we'd call the Wasm plugin function `append_newline`, passing in the `RopeHandle` we created, which easily crosses the ABI boundary.

3. Next, in Wasm, we call the native imported function `append`. This sends the `RopeHandle` back over the boundary, to Rust.

4. Looking in the `Plugin`'s `ResourcePool`, we'd convert the handle into an index, grab and downcast the `dyn Any` back into the type we need, and then call the async Rust callback with an `Arc<RwLock<Rope>>`.

5. The Rust async callback actually acquires a lock and appends the newline.

6. And from here on out we return up the callstack, through Wasm, to Rust all the way back to where we started. Right before we return, we clear out the `ResourcePool`, so that we're no longer holding onto the underlying resource.

Throughout this entire chain of calls, the resource remain host-side. By temporarily checking it into a `ResourcePool`, we're able to keep a reference to the resource that we can use, while avoiding copying the uncopyable resource.

## Final Notes

Using this approach, it should be possible to add fairly good support for resources to Wasm. I've only done a little rough prototyping, so we're bound to run into some issues along the way, but I think this should be a good first approximation.

This next week, I'll try to get a production-ready version of this working, using the `Language` resource required by some Language Server Adapters.

Hope this guide made sense!