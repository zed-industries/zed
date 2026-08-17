# merman-core

[![Crates.io](https://img.shields.io/crates/v/merman-core.svg)](https://crates.io/crates/merman-core)
[![Documentation](https://docs.rs/merman-core/badge.svg)](https://docs.rs/merman-core)
[![Crates.io Downloads](https://img.shields.io/crates/d/merman-core.svg)](https://crates.io/crates/merman-core)
[![Made with Rust](https://img.shields.io/badge/made%20with-Rust-orange.svg)](https://www.rust-lang.org)

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![License: Apache 2.0](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)

`merman-core` is the parser and semantic-model crate behind [merman](https://crates.io/crates/merman). Use it when you need Mermaid detection, metadata, semantic JSON, or typed render models without pulling in layout, SVG, or raster dependencies.

Most application code that wants rendered output should use the `merman` crate with the `render` feature instead.

## What It Provides

- Mermaid diagram detection and preprocessing, including front matter and directives.
- Strict and lenient parsing through `ParseOptions`.
- Semantic JSON via `Engine::parse_diagram_sync`.
- Typed render models via `Engine::parse_diagram_for_render_model_sync`.
- Metadata-only parsing for integrations that only need the diagram type, title, and effective config.
- Runtime-agnostic async APIs plus synchronous helpers for editor and CLI integrations.

## Parse To Semantic JSON

```rust
use merman_core::{Engine, ParseOptions};

fn main() -> Result<(), merman_core::Error> {
    let engine = Engine::new();
    let parsed = engine
        .parse_diagram_sync("flowchart TD; A[API] --> B[DB];", ParseOptions::strict())?
        .expect("diagram detected");

    assert_eq!(parsed.meta.diagram_type, "flowchart-v2");
    println!("{}", parsed.model);

    Ok(())
}
```

## Skip Detection When The Type Is Known

Markdown renderers often know the diagram type from the fence info string. Use the `*_with_type_sync` APIs to skip the detection pass.

```rust
use merman_core::{Engine, ParseOptions};

fn main() -> Result<(), merman_core::Error> {
    let engine = Engine::new();
    let parsed = engine
        .parse_diagram_with_type_sync(
            "sequence",
            "sequenceDiagram\nAlice->>Bob: Hello",
            ParseOptions::strict(),
        )?
        .expect("diagram detected");

    assert_eq!(parsed.meta.diagram_type, "sequence");
    Ok(())
}
```

Common internal ids include `flowchart-v2`, `sequence`, `classDiagram`, `stateDiagram`, `architecture`, `mindmap`, and `gantt`.

## Rendering Handoff

If the next step is layout or SVG rendering, prefer `Engine::parse_diagram_for_render_model_sync`. It returns a render-optimized typed model and avoids building a large public semantic JSON tree for diagrams with typed render support.

```rust
use merman_core::{Engine, ParseOptions};

fn main() -> Result<(), merman_core::Error> {
    let engine = Engine::new();
    let parsed = engine
        .parse_diagram_for_render_model_sync("flowchart TD; A --> B", ParseOptions::strict())?
        .expect("diagram detected");

    println!("{} -> {}", parsed.meta.diagram_type, parsed.model.kind());
    Ok(())
}
```

## Compatibility

`merman-core` tracks the pinned Mermaid baseline documented in the project README and treats upstream Mermaid as the compatibility target. The semantic JSON API is the stable parser-facing shape; typed render models are optimized for the renderer and may expose a different internal structure.
