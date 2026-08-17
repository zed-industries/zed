# merman-render

[![Crates.io](https://img.shields.io/crates/v/merman-render.svg)](https://crates.io/crates/merman-render)
[![Documentation](https://docs.rs/merman-render/badge.svg)](https://docs.rs/merman-render)
[![Crates.io Downloads](https://img.shields.io/crates/d/merman-render.svg)](https://crates.io/crates/merman-render)
[![Made with Rust](https://img.shields.io/badge/made%20with-Rust-orange.svg)](https://www.rust-lang.org)

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![License: Apache 2.0](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)

`merman-render` is the low-level layout and SVG crate behind [merman](https://crates.io/crates/merman). It consumes `merman-core` parse results and produces layout JSON or Mermaid-like SVG.

Most applications should start with the `merman` crate and `merman::render::HeadlessRenderer`. Use `merman-render` directly when you need lower-level control over layout, text measurement, SVG options, or SVG postprocessing.

## What It Provides

- Headless layout for parsed Mermaid diagrams.
- Mermaid-parity SVG emission.
- `LayoutOptions::headless_svg_defaults()` for editor/export use cases.
- Text measurement hooks through `TextMeasurer`.
- Math rendering hooks through `MathRenderer`.
- `SvgPipeline` presets and postprocessors for readable or rasterizer-friendly SVG.

## Direct Rendering Example

```rust
use merman_core::{Engine, ParseOptions};
use merman_render::{layout_parsed_render_layout_only, LayoutOptions};
use merman_render::svg::{
    render_layout_svg_parts_for_render_model_with_config, SvgPipeline, SvgRenderOptions,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let engine = Engine::new();
    let parsed = engine
        .parse_diagram_for_render_model_sync(
            "flowchart TD\nA[API] --> B[DB]",
            ParseOptions::strict(),
        )?
        .expect("diagram detected");

    let layout_options = LayoutOptions::headless_svg_defaults();
    let layout = layout_parsed_render_layout_only(&parsed, &layout_options)?;

    let svg_options = SvgRenderOptions {
        diagram_id: Some("example-diagram".to_string()),
        ..SvgRenderOptions::default()
    };

    let svg = render_layout_svg_parts_for_render_model_with_config(
        &layout,
        &parsed.model,
        &parsed.meta.effective_config,
        parsed.meta.title.as_deref(),
        layout_options.text_measurer.as_ref(),
        &svg_options,
    )?;

    let svg = SvgPipeline::resvg_safe().process_to_string(&svg)?;
    println!("{svg}");

    Ok(())
}
```

## SVG Output Pipelines

The default SVG renderer aims for Mermaid DOM parity. Host applications can opt into an output pipeline after rendering:

- `SvgPipeline::parity()` leaves the SVG unchanged.
- `SvgPipeline::readable()` keeps fallback text for `<foreignObject>` labels.
- `SvgPipeline::resvg_safe()` prepares SVG for common `usvg` / `resvg` rasterization paths.
- `ScopedCssPostprocessor`, `CssOverridePostprocessor`, and custom `SvgPostprocessor` implementations let applications inject host-specific styling without forking the renderer.

See [`docs/rendering/SVG_OUTPUT_PIPELINE.md`](https://github.com/Latias94/merman/blob/main/docs/rendering/SVG_OUTPUT_PIPELINE.md) for the higher-level integration guide.

## Relationship To merman

`merman` re-exports the common render APIs behind its `render` feature and adds `HeadlessRenderer`, SVG id sanitization helpers, and optional raster helpers. Direct `merman-render` users get the same layout/SVG engine with less convenience wrapping and more explicit control over each phase.
