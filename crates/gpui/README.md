# Welcome to GPUI!

GPUI is a hybrid immediate and retained mode, GPU accelerated, UI framework
for Rust, designed to support a wide variety of applications.

## Getting Started

GPUI is still in active development as we work on the Zed code editor and isn't yet on crates.io. You'll also need to use the latest version of stable rust and be on macOS. Add the following to your Cargo.toml:

```toml
gpui = { git = "https://github.com/zed-industries/zed" }
```

Everything in GPUI starts with an `App`. You can create one with `App::new()`, and kick off your application by passing a callback to `App::run()`. Inside this callback, you can create a new window with `AppContext::open_window()`, and register your first root view. See [gpui.rs](https://www.gpui.rs/) for a complete example.

## The Big Picture

GPUI offers three different [registers](https://en.wikipedia.org/wiki/Register_(sociolinguistics)) depending on your needs:

- State management and communication with Models. Whenever you need to store application state that communicates between different parts of your application, you'll want to use GPUI's models. Models are owned by GPUI and are only accessible through an owned smart pointer similar to an `Rc`. See the `app::model_context` module for more information.

- High level, declarative UI with Views. All UI in GPUI starts with a View. A view is simply a model that can be rendered, via the `Render` trait. At the start of each frame, GPUI will call this render method on the root view of a given window. Views build a tree of `elements`, lay them out and style them with a tailwind-style API, and then give them to GPUI to turn into pixels. See the `div` element for an all purpose swiss-army knife of rendering.

- Low level, imperative UI with Elements. Elements are the building blocks of UI in GPUI, and they provide a nice wrapper around an imperative API that provides as much flexibility and control as you need. Elements have total control over how they and their child elements are rendered and can be used for making efficient views into large lists, implement custom layouting for a code editor, and anything else you can think of. See the `element` module for more information.

Each of these registers has one or more corresponding contexts that can be accessed from all GPUI services. This context is your main interface to GPUI, and is used extensively throughout the framework.

## Other Resources

In addition to the systems above, GPUI provides a range of smaller services that are useful for building complex applications:

- Actions are user-defined structs that are used for converting keystrokes into logical operations in your UI. Use this for implementing keyboard shortcuts, such as cmd-q. See the `action` module for more information.

- Platform services, such as `quit the app` or `open a URL` are available as methods on the `app::AppContext`.

- An async executor that is integrated with the platform's event loop. See the `executor` module for more information.,

- The `[gpui::test]` macro provides a convenient way to write tests for your GPUI applications. Tests also have their own kind of context, a `TestAppContext` which provides ways of simulating common platform input. See `app::test_context` and `test` modules for more details.

Currently, the best way to learn about these APIs is to read the Zed source code, ask us about it at a fireside hack, or drop a question in the [Zed Discord](https://discord.gg/zed-community). We're working on improving the documentation, creating more examples, and will be publishing more guides to GPUI on our [blog](https://zed.dev/blog).
