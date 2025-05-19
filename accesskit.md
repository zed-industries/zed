# Adding AccessKit to your immediate mode UI framework

AccessKit is a cool rust project that provides a cross-platform library for interfacing with accessibility APIs.

It's a great project, but when setting out to add it to GPUI, Zed's UI framework, I had trouble figuring out how to even start.

So here's the tutorial I wish I had when I started:

## Step 0: Acquiring AccessKit

AccessKit is available on [crates.io](https://crates.io/crates/accesskit)

If you're using macOS, you'll also need to bundle your app.

## Step 1: Turning it On

AccessKit works by creating an "Adapter" object that does the work of communicating
with the platform. This interface is in a seperate, platform-specific crate.
At Zed, we're going to build our own accesskit-winit style general adapter, so we'll use
it's API as a model for the rest of this document. But first, let's make the macOS adapter

First, you need to have some way of implementing `accesskit::ActionHandler`, that works for your
framework. In gpui, we have a `MacWindowState` struct behind an `Arc<Mutex>` with a callback pointing
up to the general puprose framework, so let's use `Arc::new_cyclic` to capture a pointer to that struct in
the adapter:

```rs

struct MacWindow(Arc<Mutex<MacWindowState>>);

struct MacWindowState {
    //...
    accesskit_adapter: accesskit_macos::Adapter,
    accesskit_action_handler: Option<Box<dyn FnMut(ActionRequest)>>
}

struct MacActionHandler(Weak<Mutex<MacWindowState>>);

impl accesskit::ActionHandler for MacActionHandler {
    fn do_action(&mut self, request: accesskit::ActionRequest) {
        if let Some(this) = self.0.upgrade() {
            let mut lock = this.lock();
            if let Some(mut callback) = lock.accesskit_action_handler_callback.take() {
                drop(lock);
                callback(request);
                this.lock().accesskit_action_handler_callback = Some(callback);
            };
        }
    }
}


// Later
let window = MacWindow(Arc::new_cyclic(|weak| {
    Mutex::new(MacWindowState {
        //....
        accesskit_action_handler: None,
        acceskit_adapter: accesskit_macos::Adapter::new(
            native_view as *mut _,
            focus,
            MacActionHandler(weak.clone()),
        )
    })
}))

// Later still

impl PlatformWindow for MacWindow {
    //.....
    fn on_accesskit_action(&self, callback: Box<dyn FnMut(accesskit::ActionRequest)>) {
        self.0.lock().accesskit_action_handler_callback = Some(callback);
    }
}
```

And then we can handle it in the UI framework with just a little bit of wiring:

```rs
// in gpui::Window::new(handle)
let handle: AnyWindowHandle;
let mut window = cx.platform.open_window(/*...*/);
platform_window.on_accesskit_action({
    let mut cx = cx.to_async();
    Box::new(move |action| {
        handle
            .update(&mut cx, |_, window, cx| {
                window.dispatch_accessibility_action(action, cx)
            })
            .log_err();
    })
});
```

----

QUESTIONS:
- re: `update_view_focus_state` on macOS, is this for the window focus???



-----

- Nodes: these are the accesibility units
- NodeID: Used for references (just a number)
- Generate the tree, and then publish minimal updates to the tree as you render
- use the `Node` struct to create and provide data
  - `Role` struct is important for Aria stuff
- `TreeUpdate` consists of nodes that have changed (NodeId, Node)
- `TreeUpdate.focus` -> The element that is currently focused
- Winit specific: `Adapter` The thing that we need to create to interface with accessibility
- These "adapters" are per-platform, and since we're not using winit, we need to pull in each individual
  adapter (e.g.https://crates.io/crates/accesskit_macos)
- `update_if_active` is very important, RESEARCH THIS
- Actually testing on macOS:
  - 1 use voice over to see how it reads
  - 2 use accessibility inspector to read the data structures
    - Only works if you've bundled
- Core issue: Bidirectional communication based on AcccesKit NodeIds
- Core issue: Need minimal acceskit tree updates, with stable node IDs
- for GPUI, essentially use Winit's approach to abstracting AcceskitAdapters
  - OR look at how Glazier adopted AccessKit: https://github.com/linebender/glazier
- GOAL: Get the winit simple example of accesskit working
- Issue: AccessKit panics, rather than fails silently. This causes issues for things like dangling NodeIds
  - !!!

- KitTest
  -
