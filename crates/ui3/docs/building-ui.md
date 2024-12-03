# Building UI with GPUI

## Common patterns

### Method ordering

- id
- Flex properties
- Position properties
- Size properties
- Style properties
- Handlers
- State properties

### Using the Label Component to Create UI Text

The `Label` component helps in displaying text on user interfaces. It creates an interface where specific parameters such as label color, line height style, and strikethrough can be set.

Firstly, to create a `Label` instance, use the `Label::new()` function. This function takes a string that will be displayed as text in the interface.

```rust
Label::new("Hello, world!");
```

Now let's dive a bit deeper into how to customize `Label` instances:

- **Setting Color:** To set the color of the label using various predefined color options such as `Default`, `Muted`, `Created`, `Modified`, `Deleted`, etc, the `color()` function is called on the `Label` instance:

    ```rust
    Label::new("Hello, world!").color(LabelColor::Default);
    ```

- **Setting Line Height Style:** To set the line height style, the `line_height_style()` function is utilized:

    ```rust
    Label::new("Hello, world!").line_height_style(LineHeightStyle::TextLabel);
    ```

- **Adding a Strikethrough:** To add a strikethrough in a `Label`, the  `set_strikethrough()` function is used:

    ```rust
    Label::new("Hello, world!").set_strikethrough(true);
    ```

That's it! Now you can use the `Label` component to create and customize text on your application's interface.

## Building a new component

TODO
