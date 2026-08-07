# GPUI Usage Summary

GPUI is a hybrid immediate and retained mode, GPU-accelerated UI framework for Rust, designed to support a wide variety of applications.

## Overview

GPUI provides three main registers depending on your needs:

1. **State management and communication** with `Entity`'s - for storing application state that communicates between different parts of your application
2. **High-level, declarative UI** with views - all UI starts with a view that implements the `Render` trait
3. **Low-level, imperative UI** with Elements - building blocks that provide maximum flexibility and control

Each register has corresponding contexts that serve as your main interface to GPUI.

## Key Concepts at a Glance

- **Entities**: Smart pointers to state owned by the `App`, accessed only through contexts
- **Views**: `Entity<T>` where `T` implements `Render` - the root of UI rendering
- **Elements**: The building blocks of UI, styled with Tailwind CSS-inspired methods
- **Contexts**: `App`, `Context<T>`, `AsyncApp`, and `VisualContext` for interacting with GPUI
- **Actions**: User-defined structs for keyboard shortcuts and other user interactions

## Getting Started

### Basic Application Structure

Create an `Application` and run it with a callback:

```rust
use gpui::{App, Application, Bounds, Context, Window, WindowBounds, WindowOptions, div, prelude::*, px, size};

struct HelloWorld {
    text: SharedString,
}

impl Render for HelloWorld {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .bg(rgb(0x505050))
            .size(px(500.0))
            .child(format!("Hello, {}!", &self.text))
    }
}

fn main() {
    Application::new().run(|cx: &mut App| {
        // Create centered window bounds
        let bounds = Bounds::centered(None, size(px(500.), px(500.0)), cx);

        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |_, cx| {
                cx.new(|_| HelloWorld {
                    text: "World".into(),
                })
            },
        )
        .unwrap();
        cx.activate(true);
    });
}
```

### Dependencies

**macOS**:
- Install Xcode and command line tools
- Metal is used for rendering

**Linux**:
- Vulkan is used for rendering
- See system documentation for specific dependencies

## Core Concepts

### 1. Entities and Ownership

All models and views in GPUI are owned by a single top-level `App` object. Entities are created with handles and can only access their state through an `App` or `Context` reference:

```rust
struct Counter {
    count: usize,
}

Application::new().run(|cx: &mut App| {
    let counter: Entity<Counter> = cx.new(|_cx| Counter { count: 0 });
    // ...
});
```

The `Entity<T>` handle is like an `Rc` - it maintains a reference count but only provides access to state when an `App` reference is available.

### 2. Context Types

**`App`**: Root context providing access to global state and application-level services like opening windows, presenting dialogs, etc.

**`Context<T>`**: Wrapper around `App` tied to a specific entity, providing entity-level services like `notify()` and `emit()`. Dereferences into `App`.

**`AsyncApp` and `AsyncWindowContext`**: Provided by `cx.spawn` for async operations that can be held across await points.

**`VisualContext`**: Extends `AppContext` with window-specific operations for contexts that have a window.

### 3. Element State Management

When rendering elements, you often need state that persists across consecutive frames. GPUI provides two methods on `Window` for this purpose:

**`window.use_state(cx, init)`**: Creates or retrieves state that exists as long as the element is being rendered in consecutive frames. It automatically generates a key based on the caller's code location.

```rust
impl Render for MyView {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        // State persists as long as this render method is called in consecutive frames
        let markdown = window.use_state(cx, |_, cx| Markdown::new("".into(), None, None, cx));
        
        div().child(markdown.clone())
    }
}
```

**`window.use_keyed_state(key, cx, init)`**: Similar to `use_state`, but allows you to specify a custom key. This is essential when rendering lists or other dynamic content where elements may change positions, as the key ensures state persists even when the element's location in the render tree changes.

```rust
impl Render for MyView {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        // Each list item uses its ID as the key, ensuring state persists correctly
        self.items.iter().map(|item| {
            let item_state = window.use_keyed_state(item.id, cx, |_, cx| ItemState::new(cx));
            div().child(format!("Item: {}", item.name))
        })
    }
}
```

These methods are particularly useful for:
- UI components that need to track hover state, selection, or expansion
- Transient editors created within render methods
- State that should be tied to an element's lifecycle rather than the entire view
- Components that need to avoid recreating state on every render

When using these methods, the state entity automatically notifies the current view when it changes, triggering a re-render.

### 4. Accessing Entity State

```rust
// Updating
counter.update(cx, |counter: &mut Counter, cx: &mut Context<Counter>| {
    counter.count += 1;
    cx.notify(); // Notify observers
});

// Reading
let count = counter.read(cx).count;
```

### 5. Entity Handles

With `entity: Entity<T>`:

- `entity.entity_id()` - returns the unique entity ID
- `entity.downgrade()` - returns a `WeakEntity<T>` for avoiding memory leaks
- `entity.read(cx)` - returns `&T`
- `entity.read_with(cx, |entity, cx| ...)` - returns the closure's value
- `entity.update(cx, |entity, cx| ...)` - allows mutation, returns closure's value
- `entity.update_in(cx, |entity, window, cx| ...)` - also provides `Window` access

## Entity Communication

### Observe/Notify Pattern

For general state changes, use `observe` and `notify`:

```rust
let first_counter: Entity<Counter> = cx.new(|_cx| Counter { count: 0 });

let second_counter = cx.new(|cx: &mut Context<Counter>| {
    // Observe first_counter
    cx.observe(
        &first_counter,
        |second: &mut Counter, first: Entity<Counter>, cx| {
            second.count = first.read(cx).count * 2;
        },
    )
    .detach(); // Detach to keep subscription active

    Counter { count: 0 }
});

// When first_counter changes, second_counter updates
first_counter.update(cx, |counter, cx| {
    counter.count += 1;
    cx.notify();
});
```

The `observe` method returns a `Subscription` which deregisters the callback when dropped. Use `.detach()` to keep it active.

### Subscribe/Emit Pattern

For typed events, implement `EventEmitter` and use `subscribe`/`emit`:

```rust
struct CounterChangeEvent {
    increment: usize,
}

impl EventEmitter<CounterChangeEvent> for Counter {}

let second_counter = cx.new(|cx: &mut Context<Counter>| {
    cx.subscribe(&first_counter, |second: &mut Counter, _first: Entity<Counter>, event, _cx| {
        second.count += event.increment * 2;
    })
    .detach();

    Counter {
        count: first_counter.read(cx).count * 2,
    }
});

first_counter.update(cx, |first, cx| {
    first.count += 2;
    cx.emit(CounterChangeEvent { increment: 2 });
    cx.notify();
});
```

## Views and Rendering

### Creating a View

A view is an `Entity` that implements the `Render` trait:

```rust
struct MyView {
    text: SharedString,
}

impl Render for MyView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .gap_3()
            .bg(rgb(0x505050))
            .size(px(500.0))
            .justify_center()
            .items_center()
            .child(self.text.clone())
    }
}
```

### RenderOnce for Transient Components

Components that are constructed just to be rendered can implement `RenderOnce` instead:

```rust
#[derive(IntoElement)]
struct TransientComponent {
    value: usize,
}

impl RenderOnce for TransientComponent {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        div().child(format!("Value: {}", self.value))
    }
}
```

### Element Styling

GPUI uses a Tailwind CSS-inspired API for styling elements:

**Layout:**
- `.flex()`, `.flex_col()`, `.flex_row()`
- `.grid()` - enable CSS grid layout
- `.grid_cols(N)` - number of columns in grid
- `.grid_rows(N)` - number of rows in grid
- `.col_span(N)` - element spans N columns
- `.row_span(N)` - element spans N rows
- `.col_span_full()` - element spans all columns
- `.absolute()` - position element absolutely relative to parent
- `.top_8()`, `.left_8()`, `.right_8()`, `.bottom_8()` - absolute positioning helpers
- `.gap_3()`, `.gap_4()`, `.gap_6()`
- `.justify_center()`, `.justify_between()`, `.justify_end()`
- `.items_center()`, `.items_start()`, `.items_end()`
- `.w(px(100.0))`, `.h(px(200.0))`, `.size(px(500.0))`

**Colors:**
- `.bg(rgb(0x505050))`, `.bg(gpui::red())`
- `.text_color(rgb(0xffffff))`
- `.border_color(rgb(0x0000ff))`

**Borders:**
- `.border_1()`, `.border_2()`, `.border_dashed()`, `.border_none()`
- `.rounded_md()`, `.rounded_lg()`, `.rounded_none()`

**Shadows:**
- `.shadow_lg()`, `.shadow_none()`
- Custom shadows with `BoxShadow`:
  ```rust
  .shadow(vec![BoxShadow {
      color: hsla(0.0, 0.0, 0.0, 0.5),
      blur_radius: px(1.0),
      spread_radius: px(5.0),
      offset: point(px(10.0), px(10.0)),
  }])
  ```

**Text:**
- `.text_xl()`, `.text_sm()`, `.text_xs()`
- `.font_weight(FontWeight::BOLD)`
- `.line_height(px(1.5))`
- `.font_family()`, `.font_style()`
- `.letter_spacing(px(0.5))`
- `.text_ellipsis()`, `.line_clamp(3)`
- `.text_decoration_2()` - text underline/strikethrough decoration
- `.text_decoration_wavy()` - wavy decoration style
- `.text_decoration_color(gpui::red())` - decoration color

### Text Styling with TextStyle

Use `TextStyle` for more control over text appearance:

```rust
use gpui::{TextStyle, RelativeLength, DefiniteLength, AbsoluteLength};

div().text_style(TextStyle {
    font_family: Some("SF Pro".into()),
    font_features: None,
    font_size: px(16.0).into(),
    font_weight: None,
    line_height: relative(1.3).into(),
    letter_spacing: px(0.0).into(),
    color: Some(rgb(0x000000)),
    background_color: None,
    underline: None,
    strikethrough: None,
})
```

**Other:**
- `.opacity(0.5)`
- `.overflow_hidden()`, `.overflow_auto()`, `.overflow_scroll()`, `.overflow_x_scroll()`, `.overflow_y_scroll()`
- `.cursor_pointer()`, `.cursor_move()`, `.cursor_default()`
- `.id("element-id")` - for element identification

### Conditional Rendering

```rust
div()
    .when(condition, |this| {
        this.child("Only shown when true")
    })
    .when_some(option, |this, value| {
        this.child(format!("Value: {}", value))
    })
```

## Concurrency and Async

### Foreground Tasks

```rust
cx.spawn(async move |this: WeakEntity<T>, cx: &mut AsyncApp| {
    // Runs on foreground thread
    // Can update entities and UI
});
```

### Background Tasks

Use `cx.background_spawn` for work on other threads:

```rust
cx.background_spawn(async move {
    // Runs on background thread
    // Can't directly update UI
    heavy_computation().await
});
```

### Task Management

Tasks return a `Task<R>` which can be:
- Awaited in other async contexts
- Detached with `task.detach()` or `task.detach_and_log_err(cx)` to run indefinitely
- Stored in fields to cancel when dropped

```rust
let task = cx.spawn(async move |_, cx| {
    // async work
});

// Option 1: Detach
task.detach();

// Option 2: Store in struct field
self.task = Some(task);

// Option 3: Await
task.await;
```

### Timers in Tests

Prefer GPUI executor timers over `smol::Timer::after` for test timeouts:

```rust
cx.background_executor().timer(duration).await
// or in TestAppContext:
cx.background_executor.timer(duration).await
```

## Windows

### Opening a Window

```rust
cx.open_window(
    WindowOptions {
        window_bounds: Some(WindowBounds::Windowed(bounds)),
        ..Default::default()
    },
    |window, cx| {
        cx.new(|_| HelloWorld {
            text: "World".into(),
        })
    },
)
```

### Window Kinds

GPUI supports different window types:

```rust
WindowOptions {
    kind: WindowKind::Normal,    // Default regular window
    kind: WindowKind::Dialog,     // Modal dialog
    kind: WindowKind::PopUp,      // Popup window
    kind: WindowKind::Floating,   // Floating utility window
    // Linux Wayland only:
    kind: WindowKind::LayerShell(layer_shell::LayerShellOptions { /* ... */ }), // For overlays, docks, wallpapers
    ..Default::default()
}
```

### Window Options

Common window configuration options:

```rust
WindowOptions {
    titlebar: None,              // Remove default titlebar for custom titlebars
    show: false,                 // Create window without showing it
    is_movable: false,           // Prevent window from being moved
    is_resizable: false,         // Prevent window from being resized
    is_minimizable: false,       // Remove minimize button
    window_bounds: Some(bounds), // Set initial position and size
    ..Default::default()
}
```

### Window Operations

```rust
// Close a window
window.remove_window();

// Resize a window
window.resize(new_size);

// Hide/show the application
cx.hide();
cx.activate(true);  // Bring to foreground

// Observe window bounds changes
cx.observe_window_bounds(window, |_, window, _| {
    println!("Window bounds: {:?}", window.bounds());
})
.detach();
```

### Window Access

The `Window` parameter in render and event callbacks provides access to:
- Focus management
- Input state
- Action dispatching
- Direct drawing

## Focus Management

GPUI provides comprehensive focus management for keyboard navigation:

### Focus Handles

Create and manage focus handles:

```rust
struct MyView {
    focus_handle: FocusHandle,
}

impl MyView {
    fn new(cx: &mut Context<Self>) -> Self {
        let focus_handle = cx.focus_handle();
        Self { focus_handle }
    }
}

impl Focusable for MyView {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}
```

### Setting Focus

```rust
// Focus a specific entity
window.focus(&focus_handle, cx);

// Focus next/previous element
window.focus_next(cx);
window.focus_prev(cx);
```

### Focus Tracking

Track focus state on elements:

```rust
div()
    .track_focus(&self.focus_handle)
    .focus(|style| {
        style.border_2().border_color(gpui::blue())
    })
    .focus_visible(|style| {
        // Only shows with keyboard navigation (Tab), not mouse clicks
        style.border_2().border_color(gpui::green())
    })
    .child("Content")
```

### Tab Stops and Tab Index

Configure keyboard navigation order:

```rust
// Create focusable elements with tab order
let handle1 = cx.focus_handle().tab_index(1).tab_stop(true);
let handle2 = cx.focus_handle().tab_index(2).tab_stop(true);

div()
    .track_focus(&handle1)
    .tab_index(1)
    .tab_stop(true)
    .child("First item")
```

### Tab Groups

Group elements that cycle tab focus within themselves:

```rust
div()
    .id("group-1")
    .tab_index(6)
    .tab_group()  // Elements inside share tab stops [6, 1], [6, 2], etc.
    .tab_stop(false)  // The container itself is not focusable
    .child(
        div()
            .tab_index(1)
            .child("First in group")
    )
    .child(
        div()
            .tab_index(2)
            .child("Second in group")
    )
```

## Scrollable Content

Make elements scrollable:

```rust
div()
    .overflow_scroll()        // Both horizontal and vertical
    .overflow_x_scroll()      // Horizontal only
    .overflow_y_scroll()      // Vertical only
    .child("Long content...")
```

## Deferred Rendering

Use `deferred()` to render elements in floating layers (popovers, tooltips, etc.):

```rust
div()
    .child(
        button("Open Popover")
            .on_click(cx.listener(|this, _, _, cx| {
                this.open = true;
                cx.notify();
            }))
    )
    .when(self.open, |this| {
        this.child(
            deferred(
                anchored()
                    .anchor(Corner::TopLeft)
                    .snap_to_window_with_margin(px(8.))
                    .child(
                        popover()
                            .w_96()
                            .child("Popover content")
                            .on_mouse_down_out(cx.listener(|this, _, _, cx| {
                                this.open = false;
                                cx.notify();
                            }))
                    ),
            )
            .with_priority(0),  // Z-index for layer ordering
        )
    })
```

## Drag and Drop

Implement drag and drop functionality:

```rust
// Draggable items
div()
    .id(("item", ix))
    .cursor_move()
    .child(format!("Item {}", ix))
    .on_drag(drag_info, |info: &DragInfo, position, _, cx| {
        // Create dragged element
        cx.new(|_| info.position(position))
    })

// Drop target
div()
    .id("drop-target")
    .on_drop(cx.listener(|this, info: &DragInfo, _, _| {
        this.drop_on = Some(*info);
    }))
    .child("Drop items here")
```

## Prompts

Show modal prompts to users:

```rust
let answer = window.prompt(
    PromptLevel::Info,  // or PromptLevel::Warning, PromptLevel::Critical
    "Are you sure?",
    None,  // Optional detail text
    &["Ok", "Cancel"],  // Button labels
    cx,
);

// Handle the result asynchronously
cx.spawn(async move |_| {
    if answer.await.unwrap() == 0 {
        println!("User clicked Ok");
    }
})
.detach();
```

Use custom button labels with `PromptButton`:

```rust
let answer = window.prompt(
    PromptLevel::Info,
    "Are you sure?",
    None,
    &[
        PromptButton::ok("确定"),   // Ok button with custom text
        PromptButton::cancel("取消"),  // Cancel button with custom text
    ],
    cx,
);
```

## Menus

Set up application menus:

```rust
struct AppState {
    view_mode: ViewMode,
}

impl Global for AppState {}

fn set_app_menus(cx: &mut App) {
    let app_state = cx.global::<AppState>();
    cx.set_menus(vec![Menu {
        name: "my_app".into(),
        items: vec![
            MenuItem::separator(),
            MenuItem::action("List", ToggleCheck)
                .checked(app_state.view_mode == ViewMode::List),
            MenuItem::action("Grid", ToggleCheck)
                .checked(app_state.view_mode == ViewMode::Grid),
            MenuItem::separator(),
            MenuItem::action("Quit", Quit),
        ],
    }]);
}

fn main() {
    Application::new().run(|cx: &mut App| {
        cx.set_global(AppState::new());
        cx.on_action(quit);
        cx.on_action(toggle_check);
        set_app_menus(cx);
        cx.open_window(WindowOptions::default(), |_, cx| {
            cx.new(|_| MyView {})
        })
        .unwrap();
    });
}
```

## Animations

Animate elements with transitions:

```rust
svg()
    .with_animation(
        "rotate_animation",  // Unique animation name
        Animation::new(Duration::from_secs(2))
            .repeat()
            .with_easing(bounce(ease_in_out)),
        |svg, delta| {  // delta: 0.0 to 1.0
            svg.with_transformation(Transformation::rotate(
                percentage(delta),
            ))
        },
    )
```

### Images

Load and display images:

```rust
div()
    .child(img("image/app-icon.png").size_8())
    .child(img("image/black-cat-typing.gif").size_12())
```

### SVGs

Load and render SVG files:

```rust
svg()
    .path("image/arrow_circle.svg")
    .text_color(gpui::black())
    .size_20()
    .overflow_hidden()
```

### Gradients

Create linear gradients:

```rust
use gpui::{linear_gradient, linear_color_stop};

div()
    .size(px(200.), px(100.))
    .bg(linear_gradient(
        0.0,  // angle in degrees
        vec![
            linear_color_stop(gpui::red(), 0.0),
            linear_color_stop(gpui::blue(), 1.0),
        ],
    ))
```

### Manual Animation with Request Frame

For custom animations without the animation system:

```rust
struct MyView {
    opacity: f32,
    animating: bool,
}

impl Render for MyView {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        if self.animating {
            self.opacity += 0.005;
            if self.opacity >= 1.0 {
                self.animating = false;
                self.opacity = 1.0;
            } else {
                window.request_animation_frame();  // Request next frame
            }
        }

        div()
            .opacity(self.opacity)
            .child("Content")
    }
}
```

## Assets

Load custom assets (images, SVGs, etc.):

```rust
struct Assets {}

impl AssetSource for Assets {
    fn load(&self, path: &str) -> Result<Option<Cow<'static, [u8]>>> {
        std::fs::read(path)
            .map(Into::into)
            .map_err(Into::into)
            .map(Some)
    }

    fn list(&self, path: &str) -> Result<Vec<SharedString>> {
        Ok(std::fs::read_dir(path)?
            .filter_map(|entry| {
                Some(SharedString::from(
                    entry.ok()?.path().to_string_lossy().into_owned(),
                ))
            })
            .collect())
    }
}

Application::new()
    .with_assets(Assets {})
    .run(|cx: &mut App| {
        // ...
    });
```

## Input Events and Actions

### Event Handlers

Register event handlers on elements:

```rust
div()
    .on_click(cx.listener(|this: &mut T, event, window, cx| {
        // Handle click
    }))
    .on_mouse_move(cx.listener(|this, event, window, cx| {
        // Handle mouse move
    }))
```

### Actions

Define actions:

```rust
actions!(app, [Quit, Save]);

// Or with data
#[derive(PartialEq, Clone)]
struct Increment {
    amount: usize,
}
impl Action for Increment {}
```

Dispatch actions:

```rust
window.dispatch_action(Increment { amount: 1 }.boxed_clone(), cx);
// or
focus_handle.dispatch_action(&Quit, window, cx);
```

Handle actions:

```rust
div()
    .on_action(cx.listener(|this: &mut T, action: &Quit, window, cx| {
        // Handle quit action
    }))
```

## Global State

Set and manage global state:

```rust
// Set a global
cx.set_global(MyGlobal { value: 42 });

// Update a global
cx.update_global(|global: &mut MyGlobal, cx| {
    global.value += 1;
});

// Read a global
cx.read_global(|global: &MyGlobal, cx| {
    global.value
});

// Update with default if not exists
cx.update_default_global(|global: &mut MyGlobal, cx| {
    global.value = 100;
});
```

### Global State with Arc

For shared global state, use Arc-wrapped globals:

```rust
#[derive(Clone, Debug)]
pub struct TextContext {
    font_size: f32,
    line_height: f32,
}

#[derive(Clone, Debug)]
pub struct GlobalTextContext(pub Arc<TextContext>);

impl Deref for GlobalTextContext {
    type Target = Arc<TextContext>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Global for GlobalTextContext {}

// Set the global
cx.set_global(GlobalTextContext(Arc::new(TextContext {
    font_size: 16.0,
    line_height: 1.3,
})));

// Access the global
let text_context = &cx.global::<GlobalTextContext>();
let font_size = text_context.font_size;
```

## Best Practices

1. **Error Handling**: Always use `?` for error propagation, avoid `unwrap()`
   ```rust
   // Good
   let result = fallible_operation()?;

   // Bad
   let result = fallible_operation().unwrap();
   ```

2. **Notifications**: Call `cx.notify()` after state changes to trigger re-renders
   ```rust
   counter.update(cx, |counter, cx| {
       counter.count += 1;
       cx.notify();  // Important! Triggers re-render
   });
   ```

3. **Subscriptions**: Store subscriptions with leading underscore to indicate they're not actively used, preventing premature dropping
   ```rust
   struct MyView {
       _subscription: Subscription,
       _subscriptions: Vec<Subscription>,
   }
   ```

4. **Entity Access**: In tests, use `read_with()` instead of `read()` since `TestAppContext` doesn't support `read(cx)`:
   ```rust
   // In tests
   let count = counter.read_with(cx, |counter, _| counter.count);

   // In regular code
   let count = counter.read(cx).count;
   ```

5. **Shadowing**: Use variable shadowing to scope clones in async contexts:
   ```rust
   executor.spawn({
       let task_ran = task_ran.clone();
       async move {
           *task_ran.borrow_mut() = true;
       }
   });
   ```

6. **Task Management**: Always handle tasks appropriately:
   ```rust
   // Detach tasks that should run indefinitely
   task.detach();
   task.detach_and_log_err(cx);

   // Store tasks that should be cancelled when dropped
   self.background_task = Some(task);

   // Await tasks that you need the result from
   let result = task.await;
   ```

7. **Focus Management**: Always implement `Focusable` trait for views that can receive focus:
   ```rust
   struct MyView {
       focus_handle: FocusHandle,
   }

   impl Focusable for MyView {
       fn focus_handle(&self, cx: &App) -> FocusHandle {
           self.focus_handle.clone()
       }
   }
   ```

8. **Element IDs**: Add IDs to elements for testing and identification:
   ```rust
   div()
       .id("my-button")
       .child("Click me")
   ```

9. **Async from Window**: Use `window.spawn()` for async operations that need to update UI:
   ```rust
   window.spawn(cx, async move |cx| {
       let result = heavy_computation().await;
       cx.update(|_, cx| {
           // Update UI state
       })
   })
   .detach();
   ```

10. **Conditional Rendering**: Use `.when()` and `.when_some()` for clean conditional logic:
    ```rust
    div()
        .when(self.is_visible, |this| {
            this.child("Visible content")
        })
        .when_some(self.maybe_value, |this, value| {
            this.child(format!("Value: {}", value))
        })
    ```

11. **Window Operations**: Use proper window observation and lifecycle management:
    ```rust
    cx.observe_window_bounds(window, |_, window, _| {
        println!("Bounds changed: {:?}", window.bounds());
    })
    .detach();
    ```

12. **Never create mod.rs files**: Prefer `src/some_module.rs` instead of `src/some_module/mod.rs`

13. **Library roots**: Specify library root path in `Cargo.toml` with `[lib] path = "..."` instead of default `lib.rs`

14. **Full variable names**: Use complete words, not abbreviations (e.g., `queue` not `q`)

15. **Avoid indexing**: Be careful with indexing operations that may panic

16. **Silent error discarding**: Never use `let _ =` on fallible operations without explicit handling

17. **Test Parking**: In async tests, use `allow_parking()` when awaiting external futures:
    ```rust
    #[gpui::test]
    async fn test_with_external_io(cx: &mut TestAppContext) {
        cx.executor().allow_parking();
        let result = external_io_operation().await;
    }
    ```

## Importing the Prelude

Import the prelude to get common traits and types:

```rust
use gpui::prelude::*;
```

This brings in:
- `AppContext`, `Context`, `VisualContext`
- `Element`, `InteractiveElement`, `ParentElement`
- `IntoElement`, `Render`, `RenderOnce`
- `Styled`, `StyledImage`
- `StatefulInteractiveElement`, `Refineable`
- `FluentBuilder`

## Testing

GPUI provides the `#[gpui::test]` macro for testing with `TestAppContext`:

```rust
#[gpui::test]
fn basic_testing(cx: &mut TestAppContext) {
    let counter = cx.new(|cx| Counter::new(cx));

    counter.update(cx, |counter, _| {
        counter.count = 42;
    });

    // Note that TestAppContext doesn't support `read(cx)`
    let updated = counter.read_with(cx, |counter, _| counter.count);
    assert_eq!(updated, 42);
}
```

### Testing with Windows

Tests involving windows require `VisualTestContext`:

```rust
#[gpui::test]
fn test_counter_in_window(cx: &mut TestAppContext) {
    let window = cx.update(|cx| {
        cx.open_window(Default::default(), |_, cx| cx.new(|cx| Counter::new(cx)))
            .unwrap()
    });

    let mut cx = VisualTestContext::from_window(window.into(), cx);
    let counter = window.root(&mut cx).unwrap();

    // Action dispatch works via focus handle
    let focus_handle = counter.read_with(&cx, |counter, _| counter.focus_handle.clone());
    cx.update(|window, cx| {
        focus_handle.dispatch_action(&Increment, window, cx);
    });

    let count_after = counter.read_with(&cx, |counter, _| counter.count);
    assert_eq!(count_after, 1);
}
```

### Async Operations in Tests

```rust
#[gpui::test]
async fn test_async_operations(cx: &mut TestAppContext) {
    let counter = cx.new(|cx| Counter::new(cx));

    // Tasks can be awaited directly
    counter.update(cx, |counter, cx| counter.load(cx)).await;

    // Side effects don't run until you yield control
    counter.update(cx, |counter, cx| counter.reload(cx));

    // Run all pending tasks
    cx.run_until_parked();

    let count = counter.read_with(cx, |counter, _| counter.count);
    assert_eq!(count, 150);
}
```

### External Async Operations

The test executor panics if you await futures outside GPUI's control (file I/O, network). Use `allow_parking()` to disable this check:

```rust
#[gpui::test]
async fn test_allow_parking(cx: &mut TestAppContext) {
    cx.executor().allow_parking();

    // Simulate external system
    let (tx, rx) = futures::channel::oneshot::channel();
    std::thread::spawn(move || {
        std::thread::sleep(std::time::Duration::from_millis(5));
        tx.send(42).ok();
    });

    let result = rx.await.unwrap();
    assert_eq!(result, 42);
}
```

## Canvas Painting

For custom drawing with paths and shapes:

```rust
use gpui::{Path, PathBuilder, Bounds, Point, px, canvas, quad};

struct PaintingViewer {
    paths: Vec<(Path<Pixels>, Background)>,
}

impl PaintingViewer {
    fn new() -> Self {
        let mut paths = vec![];

        // Build a filled path
        let mut builder = PathBuilder::fill();
        builder.move_to(point(px(50.), px(50.)));
        builder.line_to(point(px(130.), px(50.)));
        builder.line_to(point(px(130.), px(130.)));
        builder.line_to(point(px(50.), px(130.)));
        builder.close();
        let path = builder.build().unwrap();

        // Set color with alpha
        let mut red = rgb(0xFF0000);
        red.a = 0.5;
        paths.push((path, red.into()));

        Self { paths }
    }
}

impl Render for PaintingViewer {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let paths = self.paths.clone();
        div()
            .size_full()
            .child(
                canvas(
                    move |_, _, _| {}, // prepaint closure - can prepare data
                    move |bounds, _, window, _| { // paint closure - actual drawing
                        // Draw filled paths
                        for (path, background) in &paths {
                            window.paint_path(path.clone(), *background);
                        }
                    }
                )
                .size_full()
            )
    }
}
```

### Path Styles

```rust
// Stroke options for lines
let stroke_options = StrokeOptions {
    line_width: px(2.0),
    line_cap: LineCap::Round,
    line_join: LineJoin::Round,
    miter_limit: 4.0,
};

// Use with PathBuilder for stroked paths
let mut builder = PathBuilder::stroke(stroke_options);
builder.move_to(point(px(0.), px(0.)));
builder.line_to(point(px(100.), px(100.)));
```

## Color Manipulation

### Color with Alpha

Modify color opacity:

```rust
let mut red = rgb(0xFF0000);
red.a = 0.5;  // 50% opacity
```

### HSLA Colors

Use HSLA color space:

```rust
use gpui::hsla;

let color = hsla(0.0, 0.0, 0.0, 0.5);  // hue, saturation, lightness, alpha
```

### Color Blending

Apply color operations:

```rust
let transparent_blue = gpui::blue().opacity(0.5);
let semi_transparent = color.opacity(0.25);
```

## Additional Resources

- **Ownership and data flow**: See `_ownership_and_data_flow.rs` for detailed explanations
- **Examples**: Check `crates/gpui/examples/` for practical examples
- **Zed source code**: The best way to learn advanced patterns

GPUI is still in active development with breaking changes between versions. Use the latest stable Rust version on macOS or Linux.
