# GPUI Examples

Examples can be run from the Zed repository root:

```sh
cargo run -p gpui --example hello_world
```

## Where to start

- `hello_world` shows the basic shape of a GPUI application: create an
  `Application`, open a window, create a root view, and render a `div`.
- `input` demonstrates text input, focus, selections, clipboard actions, and
  keyboard bindings.
- `uniform_list` shows how to render a simple virtualized list.
- `testing` demonstrates `#[gpui::test]`, `TestAppContext`, actions, focus, and
  window-based tests.

## Layout and styling

- `grid_layout` demonstrates CSS-grid-style layout.
- `opacity` demonstrates opacity styling.
- `pattern` shows patterned backgrounds.
- `shadow` demonstrates box shadows.
- `text` shows styled text rendering.
- `text_layout` demonstrates text alignment, decoration, weights, and wrapping.
- `text_wrapper` shows wrapping text content.

## Interaction

- `anchor` demonstrates anchored positioning.
- `data_table` combines virtualized list rendering with table-style rows and a
  custom scrollbar.
- `drag_drop` shows draggable elements and drop targets.
- `focus_visible` demonstrates keyboard-visible focus styling.
- `mouse_pressure` demonstrates pressure-sensitive pointer input where supported.
- `popover` shows floating layers with `deferred` and `anchored`.
- `scrollable` demonstrates scrollable content.
- `tab_stop` shows keyboard tab navigation.

## Images, drawing, and animation

- `animation` demonstrates GPUI animations and animated SVG transforms.
- `gif_viewer` shows GIF rendering.
- `gradient` demonstrates linear gradients and color spaces.
- `image` shows local and remote image loading, image sizing, and asset setup.
- `image_gallery` demonstrates image caching and loading remote images.
- `image_loading` shows image loading states and asset loading.
- `painting` demonstrates custom drawing with paths and canvas.
- `svg` shows SVG rendering.

## Windows and application behavior

- `move_entity_between_windows` shows moving an entity between windows.
- `on_window_close_quit` demonstrates quitting when a window closes.
- `set_menus` shows application menu setup.
- `system_notifications` demonstrates posting, replacing, dismissing, and responding to operating-system notifications.
- `window` demonstrates creating normal, dialog, popup, and floating windows.
- `window_positioning` demonstrates window bounds and placement.
- `window_shadow` demonstrates window shadow styling.

## Specialized examples

These examples are useful when working on GPUI itself, but they may not be the
best starting point for new applications:

- `active_state_bug` is a focused active-state reproduction.
- `layer_shell` demonstrates Linux layer-shell windows.
- `list_example` demonstrates bottom-aligned list state and scrollbar behavior.
- `ownership_post` supports the ownership and data-flow documentation.
- `paths_bench` is a path rendering benchmark.
- `tree` renders a deep tree of nested elements.

## Node engine boundary lab

```sh
GPUI_EXPERIMENTAL_NODE_ENGINE=0 cargo run -p gpui --example node_engine_boundaries
```

This desktop example runs with either engine. A and B are keyed `Component`
values with `use_keyed_state`. The lower panels are ordinary entity views;
retained rendering caches them automatically. Terminal traces report element
layout, prepaint, and paint calls. The window displays retained scope and layout
counts from the preceding frame. Instrumentation does not schedule frames.

| Exercise | Expected behavior |
| --- | --- |
| Increment A, then B | Each component has its own count and mount identity. |
| Swap A / B | Counts, mount identities, and focus follow the keys. |
| Remove A, then reinsert | A receives a new mount identity and count zero. |
| Change parent input, then print captured input | Displayed props and callback captures update. |
| Increment to focus, then press Enter | Only the focused card increments. |
| Hover a strip, then leave | Its color changes and returns. |
| Toggle A width | B moves when it follows A; hit targets follow geometry. |
| Toggle inherited text size | All views receive the new text style. |
| Notify one entity | Its count updates; inspect clean sibling reuse. |
| Notify shared dependency | Both lower views display the new value. |
| Toggle deferred overlay | It paints above the red strip; closing removes its hit targets. |
| Shrink and scroll the window | Clipping and hit targets follow visible content. |

Launch a second instance with `GPUI_EXPERIMENTAL_NODE_ENGINE=1` for comparison.
The engine is experimental. Differential tests compare scenes for selected
mutations and separately exercise callbacks, state lifetime, and dependency
replacement. This manual lab does not cover IME or accessibility. Deferred
painting, accessibility, inspection, and explicit window refresh conservatively
rebuild scopes. Custom measured-layout callbacks also rebuild; only GPUI's text
measurement currently opts into retaining its callback. GPU presentation still
submits the full scene; scope reuse reduces CPU work, not submitted damage regions.
