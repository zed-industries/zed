use futures::executor::block_on;
use merman_core::{Engine, ParseOptions};
use merman_render::model::LayoutDiagram;
use merman_render::svg::{SvgRenderOptions, render_er_diagram_svg};
use merman_render::{LayoutOptions, layout_parsed};
use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin()
        .read_to_string(&mut input)
        .expect("read stdin");

    let engine = Engine::new();
    let parsed = block_on(engine.parse_diagram(&input, ParseOptions::default()))
        .expect("parse ok")
        .expect("diagram detected");

    let layout_options = LayoutOptions::default();
    let layouted = layout_parsed(&parsed, &layout_options).expect("layout ok");
    let LayoutDiagram::ErDiagram(layout) = &layouted.layout else {
        panic!("expected ErDiagram layout");
    };

    let svg = render_er_diagram_svg(
        layout,
        &layouted.semantic,
        &layouted.meta.effective_config,
        layouted.meta.title.as_deref(),
        layout_options.text_measurer.as_ref(),
        &SvgRenderOptions::default(),
    )
    .expect("render svg");

    print!("{svg}");
}
