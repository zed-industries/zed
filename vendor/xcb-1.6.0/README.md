
![Crates.io Version](https://img.shields.io/crates/v/xcb)
![docs.rs](https://img.shields.io/docsrs/xcb)

# Rust XCB

Rust-XCB is a safe Rust interface to [XCB](http://xcb.freedesktop.org).
Rust-XCB uses under the hood the core XCB functions to connect and communicate
with the X server.

__Documentation__:
https://docs.rs/xcb/latest/xcb/

Rust-XCB is constituted of the following components:
 - the core library
 - the X protocol and its extensions (generated from XML)

See [CONTRIBUTING.md](https://github.com/rust-x-bindings/rust-xcb/blob/main/CONTRIBUTING.md) for contributions.

## The core library

The main component is the `Connection` class which is used to connect and
communicate to the X server. The `Connection` class wraps calls to the C XCB
functions in a safe to use interface.

In the new API (`v1.0+`), Rust-XCB takes care of all the heavy lifting of event
and error resolution, including the handling of the different kinds of event
(regular events, "GeGeneric" events, the specifics about Xkb ...) and present
them under a unified and safe to use `enum` instead of requesting the user to
perform unsafe cast as in the C library.

The core library also provides many traits that are used in the protocol
implementation. e.g. the `BaseEvent` and `BaseError` traits, the `Reply` trait...
The `Raw` trait is also provided to convert into and from C event or error types.

## The protocol implementation

The core X protocol and all the extensions present in XCB are generated from (almost exactly) the same XML
as the XCB C bindings.
The generation is done by the build scripts, entirely written in Rust.
The build script do not generate bindings to the C protocol extensions, it
generates directly a safe Rust implementation of the protocol:
 - Simple structures that have the same memory layout than C are translated in
   Rust directly (e.g. points etc.)
 - More complex structures wrap a slice of raw data and provide accessor methods
   (`Debug` will still print all the members correctly)
 - The masks use the `bitflags` crate macro
 - The enums are enums!
 - The unions are also enums that carry data
 - The Xids (handles to windows, pixmaps etc.) are type-safe implementations of the `Xid` trait.
 - The requests are structures that serialize themselves when passed to the
   `Connection`.
    - Each request has two type of cookie, for checked and unchecked requests.
      This allows type safe reply fetching and error checking
 - The protocol and each extension provide an `Event` and an `Error` enum,
   which are unified by the core library.

## API

Here are some highlights of the API.

### Modules

Directly under the `xcb` crate is the hand-written core library.
The core protocol is generated in the `x` module and each extension protocol is also generated in its own module.

### Window creation request

```rust
let window: x::Window = conn.generate_id();

conn.send_request(&x::CreateWindow {
    depth: x::COPY_FROM_PARENT as u8,
    wid: window,
    parent: screen.root(),
    x: 0,
    y: 0,
    width: 150,
    height: 150,
    border_width: 10,
    class: x::WindowClass::InputOutput,
    visual: screen.root_visual(),
    value_list: &[
        x::Cw::BackPixel(screen.white_pixel()),
        x::Cw::EventMask(x::EventMask::EXPOSURE | x::EventMask::KEY_PRESS),
    ],
});
```

### Checked void request
```rust
// specific cookie type for the checked request
// code would not compile with `conn.send_request(..)`
let cookie = conn.send_request_checked(&x::MapWindow {window});
// reports `Ok` or a resolved error enum (e.g. x::Error::Drawable(..))
conn.check_request(cookie)?;
```

### Event and error handling

Events of the core protocol and each activated extension are unified in the `xcb::Event` enum.
Therefore, all events can be handled in a single match expression.
Many functions (such as `wait_for_event`) return a `xcb::Result` which allows idiomatic error handling.

```rust
fn main() -> xcb::Result<()> {
    // ...
    loop {
        match conn.wait_for_event()? {
            xcb::Event::X(x::Event::KeyPress(key_press)) => {
                // do stuff
            }
            xcb::Event::Xkb(xkb::Event::MapNotify(ev)) => {
                // do other stuff (pass data to xkbcommon for example)
            }
            _ => {}
        }
    }
}
```

### Debugging

All types in Rust XCB implement `Debug` in a way that allows recursive debug print.
E.g. iterators will not print a useless pointer value, but will recurse down to each element.

There is in addition the optional `"debug_atom_names"` cargo feature under which each atom
will print its name for easier debugging in some situations.
For example, Xinput provide some information about input devices with atom identifiers.
This allows you to quickly look-up which atoms you need to intern and seek for.
E.g. the feature would turn:
```
Atom {
    res_id: 303,
}
```
into
```
Atom("Abs Pressure" ; 303)
```

The feature sets global variable to have access to the connection in the `Debug::fmt` call,
so it should be activated only when needed.
