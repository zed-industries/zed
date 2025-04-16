# Key Dispatch

GPUI is designed for keyboard-first interactivity.

To expose functionality to the mouse, you render a button with a click handler.

To expose functionality to the keyboard, you bind an _action_ in a _key context_.

Actions are similar to framework-level events like `MouseDown`, `KeyDown`, etc, but you can define them yourself:

```rust
mod menu {
    #[gpui::action]
    struct MoveUp;

    #[gpui::action]
    struct MoveDown;
}
```

Actions are frequently unit structs, for which we have a macro. The above could also be written:

```rust
mod menu {
    actions!(gpui, [MoveUp, MoveDown]);
}
```

Actions can also be more complex types:

```rust
mod menu {
    #[gpui::action]
    struct Move {
        direction: Direction,
        select: bool,
    }
}
```

To bind actions, chain `on_action` on to your element:

```rust
impl Render for Menu {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .on_action(|this: &mut Menu, move: &MoveUp, window: &mut Window, cx: &mut Context<Menu>| {
                // ...
            })
            .on_action(|this, move: &MoveDown, cx| {
                // ...
            })
            .children(unimplemented!())
    }
}
```

In order to bind keys to actions, you need to declare a _key context_ for part of the element tree by calling `key_context`.

```rust
impl Render for Menu {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .key_context("menu")
            .on_action(|this: &mut Menu, move: &MoveUp, window: &mut Window, cx: &mut Context<Menu>| {
                // ...
            })
            .on_action(|this, move: &MoveDown, cx| {
                // ...
            })
            .children(unimplemented!())
    }
}
```

Now you can target your context in the keymap. Note how actions are identified in the keymap by their fully-qualified type name.

```json
{
  "context": "menu",
  "bindings": {
    "up": "menu::MoveUp",
    "down": "menu::MoveDown"
  }
}
```

If you had opted for the more complex type definition, you'd provide the serialized representation of the action alongside the name:

```json
{
  "context": "menu",
  "bindings": {
    "up": ["menu::Move", {direction: "up", select: false}]
    "down": ["menu::Move", {direction: "down", select: false}]
    "shift-up": ["menu::Move", {direction: "up", select: true}]
    "shift-down": ["menu::Move", {direction: "down", select: true}]
  }
}
```
