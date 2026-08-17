<!-- markdownlint-disable-next-line MD041 -->
<p>
<picture>
  <img src="assets/logo.svg" alt="Taffy" height="70">
</picture>
</p>

[![GitHub CI](https://github.com/DioxusLabs/taffy/actions/workflows/ci.yml/badge.svg)](https://github.com/DioxusLabs/taffy/actions/workflows/ci.yml)
[![crates.io](https://img.shields.io/crates/v/taffy.svg)](https://crates.io/crates/taffy)
[![docs.rs](https://img.shields.io/docsrs/taffy)](https://docs.rs/taffy)
![Crates.io MSRV](https://img.shields.io/crates/msrv/taffy)

Taffy is a flexible, high-performance, cross-platform UI layout library written in [Rust](https://www.rust-lang.org).

It currently implements the CSS **Block**, **Flexbox** and **CSS Grid** layout algorithms. Support for other paradigms is planned. For more information on this and other future development plans see the [roadmap issue](https://github.com/DioxusLabs/taffy/issues/345).

This crate is a collaborative, cross-team project, and is designed to be used as a dependency for other UI and GUI libraries.
Right now, it powers:

- [Servo](https://github.com/servo/servo): an alternative web browser
- [Blitz](https://github.com/DioxusLabs/blitz): a radically modular web engine
- [Bevy](https://bevyengine.org/): an ergonomic, ECS-first Rust game engine
- [Takumi](https://github.com/kane50613/takumi): Renders your React components to images
- [iocraft](https://github.com/ccbrown/iocraft): crafting beautiful interfaces for the terminal
- [Slint](https://github.com/slint-ui/slint): a declarative GUI toolkit for building native user interfaces
- The [Lapce](https://lapce.dev/) text editor via the [Floem](https://github.com/lapce/floem) UI framework
- The [Zed](https://zed.dev/) text editor via the [GPUI](https://github.com/zed-industries/zed/tree/main/crates/gpui) UI framework

## Usage

```rust
use taffy::prelude::*;

// First create an instance of TaffyTree
let mut tree : TaffyTree<()> = TaffyTree::new();

// Create a tree of nodes using `TaffyTree.new_leaf` and `TaffyTree.new_with_children`.
// These functions both return a node id which can be used to refer to that node
// The Style struct is used to specify styling information
let header_node = tree
    .new_leaf(
        Style {
            size: Size { width: length(800.0), height: length(100.0) },
            ..Default::default()
        },
    ).unwrap();

let body_node = tree
    .new_leaf(
        Style {
            size: Size { width: length(800.0), height: auto() },
            flex_grow: 1.0,
            ..Default::default()
        },
    ).unwrap();

let root_node = tree
    .new_with_children(
        Style {
            flex_direction: FlexDirection::Column,
            size: Size { width: length(800.0), height: length(600.0) },
            ..Default::default()
        },
        &[header_node, body_node],
    )
    .unwrap();

// Call compute_layout on the root of your tree to run the layout algorithm
tree.compute_layout(root_node, Size::MAX_CONTENT).unwrap();

// Inspect the computed layout using `TaffyTree.layout`
assert_eq!(tree.layout(root_node).unwrap().size.width, 800.0);
assert_eq!(tree.layout(root_node).unwrap().size.height, 600.0);
assert_eq!(tree.layout(header_node).unwrap().size.width, 800.0);
assert_eq!(tree.layout(header_node).unwrap().size.height, 100.0);
assert_eq!(tree.layout(body_node).unwrap().size.width, 800.0);
assert_eq!(tree.layout(body_node).unwrap().size.height, 500.0); // This value was not set explicitly, but was computed by Taffy

```

## Bindings to other languages

- Python via [stretchable](https://github.com/mortencombat/stretchable)
- [WIP C bindings](https://github.com/DioxusLabs/taffy/pull/404)
- [WIP WASM bindings](https://github.com/DioxusLabs/taffy/pull/394)

## Learning Resources

Taffy implements the Flexbox and CSS Grid specifications faithfully, so documentation designed for the web should translate cleanly to Taffy's implementation. For reference documentation on individual style properties we recommend the MDN documentation (for example [this page](https://developer.mozilla.org/en-US/docs/Web/CSS/width) on the `width` property). Such pages can usually be found by searching for "MDN property-name" using a search engine.

If you are interested in guide-level documentation on CSS layout, then we recommend the following resources:

### Flexbox

- [Flexbox Froggy](https://flexboxfroggy.com/). This is an interactive tutorial/game that allows you to learn the essential parts of Flexbox in a fun engaging way.
- [A Complete Guide To Flexbox](https://css-tricks.com/snippets/css/a-guide-to-flexbox/) by CSS Tricks. This is detailed guide with illustrations and comprehensive written explanation of the different Flexbox properties and how they work.

### CSS Grid

- [CSS Grid Garden](https://cssgridgarden.com/). This is an interactive tutorial/game that allows you to learn the essential parts of CSS Grid in a fun engaging way.
- [A Complete Guide To CSS Grid](https://css-tricks.com/snippets/css/complete-guide-grid/) by CSS Tricks. This is detailed guide with illustrations and comprehensive written explanation of the different CSS Grid properties and how they work.

## Benchmarks (vs. [Yoga](https://github.com/facebook/yoga))

- Run on a 2021 MacBook Pro with M1 Pro processor using [criterion](https://github.com/bheisler/criterion.rs)
- The benchmarks measure layout computation only. They do not measure tree creation.
- Yoga benchmarks were run via the [yoga](https://github.com/bschwind/yoga-rs) crate (Rust bindings)
- Most popular websites seem to have between 3,000 and 10,000 nodes (although they also require text layout, which neither yoga nor taffy implement).

Note that the table below contains multiple different units (milliseconds vs. microseconds)

| Benchmark          | Node Count | Depth | Yoga ([ba27f9d]) | Taffy ([71027a8]) |
| ---                | ---        | ---   | ---              | ---               |
| yoga 'huge nested' | 1,000      | 3     | 364.60 µs        | 329.04 µs         |
| yoga 'huge nested' | 10,000     | 4     | 4.1988 ms        | 4.3486 ms         |
| yoga 'huge nested' | 100,000    | 5     | 45.804 ms        | 38.559 ms         |
| big trees (wide)   | 1,000      | 1     | 737.77 µs        | 505.99 µs         |
| big trees (wide)   | 10,000     | 1     | 7.1007 ms        | 8.3395 ms         |
| big trees (wide)   | 100,000    | 1     | 135.78 ms        | 247.42 ms         |
| big trees (deep)   | 4,000      | 12    | 2.2333 ms        | 1.7400 ms         |
| big trees (deep)   | 10,000     | 14    | 5.9477 ms        | 4.4445 ms         |
| big trees (deep)   | 100,000    | 17    | 76.755 ms        | 63.778 ms         |
| super deep         | 1,000      | 1,000 | 555.32 µs        | 472.85 µs         |

[ba27f9d]: https://github.com/facebook/yoga/commit/ba27f9d1ecfa7518019845b84b035d3d4a2a6658
[71027a8]: https://github.com/DioxusLabs/taffy/commit/71027a8de03b343e120852b84bb7dca9fb4651c5

## Contributions

[Contributions welcome](https://github.com/DioxusLabs/taffy/blob/main/CONTRIBUTING.md):
if you'd like to use, improve or build `taffy`, feel free to join the conversation, open an [issue](https://github.com/DioxusLabs/taffy/issues) or submit a [PR](https://github.com/DioxusLabs/taffy/pulls).
If you have questions about how to use `taffy`, open a [discussion](https://github.com/DioxusLabs/taffy/discussions) so we can answer your questions in a way that others can find.
