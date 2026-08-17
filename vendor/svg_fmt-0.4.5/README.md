# svg_fmt

A set of simple types using `Display` formatters `{}` to easily write in the SVG format.
This can be useful to dump information in a visual way when debugging.

The crate is very small (and has no dependency).

## Example

```rust
use svg_fmt::*;

println!("{}", BeginSvg { w: 800.0, h: 600.0 });
println!("    {}",
    rectangle(20.0, 50.0, 200.0, 100.0)
        .fill(Fill::Color(red()))
        .stroke(Stroke::Color(black(), 3.0))
        .border_radius(5.0)
);
println!("    {}",
    text(25.0, 100.0, "Hi!")
        .size(42.0)
        .color(white())
);
println!("{}", EndSvg);

```

