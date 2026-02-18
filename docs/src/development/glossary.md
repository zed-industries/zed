---
title: Zed Development: Glossary
description: "Guide to zed development: glossary for Zed development."
---

# Zed Development: Glossary

This page defines terms and structures used throughout the Zed codebase.

It is a best-effort list and a work in progress.

<!--
TBD: Glossary Improvement

Questions:

- Can we generate this list from doc comments throughout zed?
- We should have a section that shows the various UI parts and their names. (Can't do that in the channel.)
-->

## Naming conventions

These are common naming patterns across the codebase. `Name` is a placeholder
for any type name, such as `AnyElement` or `LspStore`.

- `AnyName`: A type-erased version of _name_. Think `Box<dyn NameTrait>`.
- `NameStore`: A wrapper type which abstracts over whether operations are running locally or on a remote.

## GPUI

### State management

- `App`: A singleton that holds full application state, including all entities. `App` is not `Send`, so it exists only on the thread that created it (usually the main/UI thread). If you see `&mut App`, you are on the UI thread.
- `Context`: A wrapper around `App` with specialized behavior for a specific `Entity`. You can think of it as `(&mut App, Entity<V>)`. For example, `App::spawn` takes `AsyncFnOnce(AsyncApp) -> Ret`, while `Context::spawn` takes `AsyncFnOnce(WeakEntity<V>, AsyncApp) -> Ret`.
- `AsyncApp`: An owned version of `App` for async contexts. It is still not `Send`, so it still runs on the main thread, and operations may fail if the `App` has already terminated.
  `AsyncApp` exists because `App` is usually accessed as `&mut App`, which is awkward to hold across async boundaries.
- `AppContext`: A trait that abstracts over `App`, `AsyncApp`, `Context`, and their test variants.
- `Task`: A future running (or scheduled to run) on a background or foreground executor. Unlike regular futures, tasks do not need `.await` to start. You still need to await them to read their result.
- `Executor`: Used to spawn tasks that run either on the foreground or background thread. Try to run the tasks on the background thread.
  - `BackgroundExecutor`: A threadpool running `Task`s.
  - `ForegroundExecutor`: The main thread running `Task`s.
- `Entity`: A strong, well-typed reference to a struct which is managed by gpui. Effectively a pointer/map key into the `App::EntityMap`.
- `WeakEntity`: A runtime checked reference to an `Entity` which may no longer exist. Similar to [`std::rc::Weak`](https://doc.rust-lang.org/std/rc/struct.Weak.html).
- `Global`: A singleton type which has only one value, that is stored in the `App`.
- `Event`: A data type that can be sent by an `Entity` to subscribers.
- `Action`: An event that represents a user's keyboard input that can be handled by listeners
  Example: `file finder: toggle`
- `Observing`: Reacting to notifications that entities have changed.
- `Subscription`: An event handler that is used to react to the changes of state in the application.
  1. Emitted event handling
  2. Observing `{new,release,on notify}` of an entity

### UI

- `View`: An `Entity` which can produce an `Element` through its implementation of `Render`.
- `Element`: A type that can be laid out and painted to the screen.
- `element expression`: An expression that builds an element tree, example:

```rust
h_flex()
    .id(text[i])
    .relative()
    .when(selected, |this| {
        this.child(
            div()
                .h_4()
                .absolute()
                etc etc
```

- `Component`: A builder which can be rendered turning it into an `Element`.
- `Dispatch tree`: TODO
- `Focus`: The place where keystrokes are handled first
- `Focus tree`: Path from the place that has the current focus to the UI Root. Example <img> TODO

## Zed UI

- `Window`: A struct representing a Zed window in your desktop environment (see image below). You can have multiple windows open. This is mostly passed around for rendering.
- `Modal`: A UI element that floats on top of the rest of the UI
- `Picker`: A struct representing a list of items floating on top of the UI (Modal). You can select an item and confirm. What happens on select or confirm is determined by the picker's delegate. (The 'Modal' in the image below is a picker.)
- `PickerDelegate`: A trait used to specialize behavior for a `Picker`. The `Picker` stores the `PickerDelegate` in the field delegate.
- `Center`: The middle of the zed window, the center is split into multiple `Pane`s. In the codebase this is a field on the `Workspace` struct. (see image below).
- `Pane`: An area in the `Center` where we can place items, such as an editor, multi-buffer or terminal (see image below).
- `Panel`: An `Entity` implementing the `Panel` trait. Panels can be placed in a `Dock`. In the image below: `ProjectPanel` is in the left dock, `DebugPanel` is in the bottom dock, and `AgentPanel` is in the right dock. `Editor` does not implement `Panel`.
- `Dock`: A UI element similar to a `Pane` that can be opened and hidden. Up to three docks can be open at once: left, right, and bottom. A dock contains one or more `Panel`s, not `Pane`s.

<img width="1921" height="1080" alt="Screenshot for the Pane and Dock features" src="https://github.com/user-attachments/assets/2cb1170e-2850-450d-89bb-73622b5d07b2" />

- `Project`: One or more `Worktree`s
- `Worktree`: Represents either local or remote files.

<img width="552" height="1118" alt="Screenshot for the Worktree feature" src="https://github.com/user-attachments/assets/da5c58e4-b02e-4038-9736-27e3509fdbfa" />

- [Multibuffer](https://zed.dev/docs/multibuffers): A list of Editors, a multi-buffer allows editing multiple files simultaneously. A multi-buffer opens when an operation in Zed returns multiple locations, examples: _search_ or _go to definition_. See project search in the image below.

<img width="800" height="886" alt="Screenshot for the MultiBuffer feature" src="https://github.com/user-attachments/assets/d59dcecd-8ab6-4172-8fb6-b1fc3c3eaf9d" />

## Editor

- `Editor`: The text editor type. Most editable surfaces in Zed are an `Editor`, including single-line inputs. Each pane in the image above contains one or more `Editor` instances.
- `Workspace`: The root of the window
- `Entry`: A file, dir, pending dir or unloaded dir.
- `Buffer`: The in-memory representation of a 'file' together with relevant data such as syntax trees, git status and diagnostics.
- `pending selection`: You have mouse down and you're dragging but you have not yet released.

## Collab

- `Collab session`: Multiple users working in a shared `Project`
- `Upstream client`: The zed client which has shared their workspace
- `Downstream client`: The zed client joining a shared workspace

## Debugger

- `DapStore`: Is an entity that manages debugger sessions
- `debugger::Session`: An entity that manages the lifecycle of a debug session and communication with DAPs.
- `BreakpointStore`: Is an entity that manages breakpoints states in local and remote instances of Zed
- `DebugSession`: Manages a debug session's UI and running state
- `RunningState`: Directly manages all the views of a debug session
- `VariableList`: The variable and watch list view of a debug session
- `Console`: TODO
- `Terminal`: TODO
- `BreakpointList`: TODO
