//! Our build and binning against lyon's tessellator, on `measure.rs`'s
//! fixtures. Needs the `lyon_tessellation` and `lyon_algorithms`
//! dev-dependencies, a `PathBuilder::into_lyon` hook and a mount; see
//! README.md.
//!
//! - Ours: `build()` then `bin` at scales 1, 1.5 and 2.
//! - Lyon, fresh: the tessellation the old `PathBuilder::build` did, with new
//!   tessellators and buffers per path, but without converting the triangles
//!   into the old `Path`.
//! - Lyon, reused: the same, reusing tessellators and buffers, as our binner
//!   reuses its scratch memory.

use crate::path_binning_measure::{fixtures, time};
use crate::{PathBuilder, PathStyle, Pixels};
use lyon_algorithms::measure::{PathMeasurements, SampleType};
use lyon_tessellation::{
    BuffersBuilder, FillOptions, FillTessellator, FillVertex, StrokeOptions, StrokeTessellator,
    StrokeVertex, VertexBuffers,
};

struct Tessellators {
    fill: FillTessellator,
    stroke: StrokeTessellator,
    buffers: VertexBuffers<lyon_path::math::Point, u32>,
}

impl Tessellators {
    fn new() -> Self {
        Self {
            fill: FillTessellator::new(),
            stroke: StrokeTessellator::new(),
            buffers: VertexBuffers::new(),
        }
    }
}

/// The old `PathBuilder::build`'s dashing.
fn dash(path: &lyon_path::Path, dash_array: &[Pixels]) -> lyon_path::Path {
    let measurements = PathMeasurements::from_path(path, 0.01);
    let mut sampler = measurements.create_sampler(path, SampleType::Normalized);
    let mut builder = lyon_path::Path::builder();
    let total_length = sampler.length();
    let mut position = 0.;
    let mut dash_index = 0;
    while position < total_length {
        let dash_length = dash_array[dash_index % dash_array.len()].0;
        let next_position = (position + dash_length).min(total_length);
        if dash_index % 2 == 0 {
            sampler.split_range(
                position / total_length..next_position / total_length,
                &mut builder,
            );
        }
        position = next_position;
        dash_index += 1;
    }
    builder.build()
}

fn tessellate(builder: PathBuilder, tessellators: &mut Tessellators) {
    let (path, style, dash_array) = builder.into_lyon();
    let Tessellators {
        fill,
        stroke,
        buffers,
    } = tessellators;
    buffers.vertices.clear();
    buffers.indices.clear();
    let result = match style {
        PathStyle::Fill(options) => fill.tessellate_path(
            &path,
            &FillOptions::default().with_fill_rule(options.fill_rule),
            &mut BuffersBuilder::new(buffers, |vertex: FillVertex| vertex.position()),
        ),
        PathStyle::Stroke(options) => {
            let dashed = dash_array.map(|dash_array| dash(&path, &dash_array));
            stroke.tessellate_path(
                dashed.as_ref().unwrap_or(&path),
                &StrokeOptions::default()
                    .with_line_width(options.line_width)
                    .with_line_cap(options.line_cap)
                    .with_line_join(options.line_join)
                    .with_miter_limit(options.miter_limit),
                &mut BuffersBuilder::new(buffers, |vertex: StrokeVertex| vertex.position()),
            )
        }
    };
    std::hint::black_box((result.is_ok(), buffers.indices.len()));
}

#[test]
#[ignore]
fn tessellator() {
    let filter = std::env::var("FIXTURES").unwrap_or_default();
    println!("fixture\tours_1x\tours_1.5x\tours_2x\tlyon_fresh\tlyon_reused");
    let mut reused = Tessellators::new();
    for (name, make) in fixtures() {
        if !filter.is_empty() && !name.contains(filter.as_str()) {
            continue;
        }
        let ours = [1.0, 1.5, 2.0].map(|scale| {
            time(|| {
                for builder in make() {
                    if let Ok(path) = builder.build() {
                        std::hint::black_box(path.bin(scale));
                    }
                }
            })
        });
        let fresh = time(|| {
            for builder in make() {
                tessellate(builder, &mut Tessellators::new());
            }
        });
        let reused = time(|| {
            for builder in make() {
                tessellate(builder, &mut reused);
            }
        });
        println!(
            "{name}\t{:.2}\t{:.2}\t{:.2}\t{fresh:.2}\t{reused:.2}",
            ours[0], ours[1], ours[2]
        );
    }
}
