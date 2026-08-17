# Merman Rust Examples

Run these commands from the repository root.

Examples `01` through `08` and `11` accept Mermaid source on stdin. If stdin is an interactive
terminal, they do not wait for input: they print a short note to stderr and render a built-in
example. To render custom Mermaid, pipe source into the command or redirect a `.mmd` file.

## Built-In Input

```bash
cargo run -p merman --features render --example example_01_svg_basic > out.svg
cargo run -p merman --example example_02_semantic_json
cargo run -p merman --features render --example example_03_layout_json
cargo run -p merman --features ascii --example example_04_ascii_output
cargo run -p merman --features raster --example example_05_raster_output -- target/example.png
cargo run -p merman --features render --example example_06_svg_pipeline > pipeline.svg
cargo run -p merman --features render --example example_07_theme_css > themed.svg
cargo run -p merman --example example_08_deterministic_gantt
cargo run -p merman --features render --example example_09_multiple_diagrams
cargo run -p merman --features egui-example --example example_10_integration_egui
cargo run -p merman --features render --example example_11_custom_output_environment > host-preview.svg
```

## Custom Input

Pipe a Mermaid string:

```bash
printf "flowchart LR\nA --> B\n" | \
  cargo run -p merman --features render --example example_01_svg_basic > out.svg
```

Redirect a Mermaid file:

```bash
cargo run -p merman --features render --example example_06_svg_pipeline \
  < fixtures/flowchart/basic.mmd > pipeline.svg
```

Render custom PNG output:

```bash
printf "flowchart LR\nA --> B\n" | \
  cargo run -p merman --features raster --example example_05_raster_output -- target/example.png
```

## Output Paths

- `example_01`, `example_06`, and `example_07` write SVG to stdout.
- `example_11` writes host-controlled resvg-safe SVG to stdout.
- `example_02`, `example_03`, and `example_08` write JSON to stdout.
- `example_04` writes terminal text to stdout. Pass `-- --ascii` for ASCII-only output.
- `example_05` writes PNG to `target/merman-raster-example.png` by default, or to the path passed
  after `--`.
- `example_09` writes SVG files to `target/merman-multiple-diagrams/`.
- `example_10` opens an egui desktop window.
