# Rust bindings for EGL

<table><tr>
  <td><a href="https://docs.rs/khronos-egl">Documentation</a></td>
  <td><a href="https://crates.io/crates/khronos-egl">Crate informations</a></td>
  <td><a href="https://github.com/timothee-haudebourg/khronos-egl">Repository</a></td>
</tr></table>

This crate provides a binding for the Khronos EGL 1.5 API.
It was originally a fork of the [egl](https://crates.io/crates/egl) crate,
which is left unmaintained.

## Usage

You can access the EGL API using an [`Instance`](https://docs.rs/khronos-egl/latest/khronos-egl/struct.Instance.html)
object defined by either statically linking with `libEGL.so.1` at compile time,
or dynamically loading the EGL library at runtime.

### Static linking

You must enable static linking using the `static` feature in your `Cargo.toml`:
```toml
khronos-egl = { version = ..., features = ["static"] }
```

This will add a dependency to the [`pkg-config`](https://crates.io/crates/pkg-config) crate,
necessary to find the EGL library at compile time.

If you wish to disable linking EGL in this crate, and provide linking in
your crate instead, enable the `no-pkg-config` feature.
```toml
khronos-egl = {version = ..., features = ["static", "no-pkg-config"]}
```

Here is a simple example showing how to use this library to create an EGL context when static linking is enabled.

```rust
extern crate khronos_egl as egl;

fn main() -> Result<(), egl::Error> {
	// Create an EGL API instance.
	// The `egl::Static` API implementation is only available when the `static` feature is enabled.
	let egl = egl::Instance::new(egl::Static);

	let wayland_display = wayland_client::Display::connect_to_env().expect("unable to connect to the wayland server");
	let display = egl.get_display(wayland_display.get_display_ptr() as *mut std::ffi::c_void).unwrap();
	egl.initialize(display)?;

	let attributes = [
		egl::RED_SIZE, 8,
		egl::GREEN_SIZE, 8,
		egl::BLUE_SIZE, 8,
		egl::NONE
	];

	let config = egl.choose_first_config(display, &attributes)?.expect("unable to find an appropriate ELG configuration");

	let context_attributes = [
		egl::CONTEXT_MAJOR_VERSION, 4,
		egl::CONTEXT_MINOR_VERSION, 0,
		egl::CONTEXT_OPENGL_PROFILE_MASK, egl::CONTEXT_OPENGL_CORE_PROFILE_BIT,
		egl::NONE
	];

	egl.create_context(display, config, None, &context_attributes);

	Ok(())
}
```

The creation of a `Display` instance is not detailed here since it depends on your display server.
It is created using the `get_display` function with a pointer to the display server connection handle.
For instance, if you are using the [wayland-client](https://crates.io/crates/wayland-client) crate,
you can get this pointer using the `Display::get_display_ptr` method.

#### Static API Instance

It may be bothering in some applications to pass the `Instance` to every fonction that needs to call the EGL API.
One workaround would be to define a static `Instance`,
which should be possible to define at compile time using static linking.
However this is not yet supported by the stable `rustc` compiler.
With the nightly compiler,
you can combine the `nightly` and `static` features so that this crate
can provide a static `Instance`, called `API` that can then be accessed everywhere.

```rust
use egl::API as egl;
```

### Dynamic Linking

Dynamic linking allows your application to accept multiple versions of EGL and be more flexible.
You must enable dynamic linking using the `dynamic` feature in your `Cargo.toml`:
```toml
khronos-egl = { version = ..., features = ["dynamic"] }
```

This will add a dependency to the [`libloading`](https://crates.io/crates/libloading) crate,
necessary to find the EGL library at runtime.
You can then load the EGL API into a `Instance<Dynamic<libloading::Library>>` as follows:

```rust
let lib = libloading::Library::new("libEGL.so.1").expect("unable to find libEGL.so.1");
let egl = unsafe { egl::DynamicInstance::<egl::EGL1_4>::load_required_from(lib).expect("unable to load libEGL.so.1") };
```

Here, `egl::EGL1_4` is used to specify what is the minimum required version of EGL that must be provided by `libEGL.so.1`.
This will return a `DynamicInstance<egl::EGL1_4>`, however in that case where `libEGL.so.1` provides a more recent version of EGL,
you can still upcast ths instance to provide version specific features:
```rust
match egl.upcast::<egl::EGL1_5>() {
	Some(egl1_5) => {
		// do something with EGL 1.5
	}
	None => {
		// do something with EGL 1.4 instead.
	}
};
```

### NixOS

A `shell.nix` file is present for nix users to build the crate easily.
Just enter a new nix shell using the given configuration file,
and `cargo build` should work.
If you want to run the tests and examples you will need to use `shell-wayland.nix` instead
that will also load wayland since most of them depend on it.

## Testing

Most test and examples most be compiled with the `static` feature.

## Troubleshooting

### Static Linking with OpenGL ES

When using OpenGL ES with `khronos-egl` with the `static` feature,
it is necessary to place a dummy extern at the top of your application which links libEGL first, then GLESv1/2.
This is because libEGL provides symbols required by GLESv1/2.
Here's how to work around this:

```rust
#[link(name = "EGL")]
#[link(name = "GLESv2")]
extern {}
```

## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

If the original `egl` crate was licensed only under the Apache 2.0 license,
I believe I have made enough breaking changes so that no relevant code from the
original code remains and the rest can be relicensed.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
