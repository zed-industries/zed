To enable rendering custom strokes or backgrounds for elements, let's update the `Background` struct to allow a `Fill` method to be specified rather than only a solid color or gradient.

Today the `Background` struct looks like this:

```rust
/// A background color, which can be either a solid color or a linear gradient.
#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct Background {
    pub(crate) tag: BackgroundTag,
    pub(crate) color_space: ColorSpace,
    pub(crate) solid: Hsla,
    pub(crate) angle: f32,
    pub(crate) colors: [LinearColorStop; 2],
    /// Padding for alignment for repr(C) layout.
    pad: u32,
}
```

I'd like to rework background to use Fill, like:

```rust
/// Represents different types of fills for backgrounds
pub enum Fill {
    Solid(Hsla),
    Gradient {
        tag: GradientTag,
        /// The angle of the gradient
        angle: f32,
        /// The color stops for the gradient
        colors: [ColorStop; 2],
    },
    Pattern {
        tag: PatternTag,
        /// The color of the pattern
        color: Hsla,
        /// The background color
        background: Hsla,
    },
}
```

But I think that might be too much complexity to take on in the short term.

In the short term we could extend `Background` to have a Pattern:

```rust
#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub(crate) enum BackgroundTag {
    Solid = 0,
    LinearGradient = 1,
    Pattern = 2,
}

/// A background color, which can be either a solid color or a linear gradient.
#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct Background {
    pub(crate) tag: BackgroundTag,
    pub(crate) color_space: ColorSpace,
    pub(crate) solid: Hsla,
    pub(crate) angle: f32,
    pub(crate) colors: [LinearColorStop; 2],
    pub(crate) pattern: Pattern,
    /// Padding for alignment for repr(C) layout.
    pad: u32,
}

/// Specifies the type of pattern
#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub enum PatternTag {
    Dash = 0,
    Hash = 1,
    // Image = 2 (?)
}

/// A pattern in a single color
#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct Pattern {
    pub tag: PatternTag,
    pub color: Hsla,
    pub repeat_x: bool,
    pub repeat_y: bool,
    pub stretch: bool,
}
```



---

`scene::Quad`, `scene::Path`
