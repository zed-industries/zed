//! Context for reusing dynamic memory allocations.

use super::geometry::{Bounds, BoundsBuilder, Transform};
use super::path_builder::{PathBuilder, TransformSink};
use super::path_data::PathData;
use super::raster::HeapStorage;
use super::segment::Segment;
use super::stroke::stroke_with_storage;
use super::style::{Fill, Style};

use crate::lib::Vec;
use core::borrow::Borrow;

/// Scratch memory for reusable heap allocations.
#[derive(Default)]
pub struct Scratch {
    pub(super) inner: Inner,
    pub(super) render: HeapStorage,
}

impl Scratch {
    /// Creates a new scratch memory context.
    pub fn new() -> Self {
        Self::default()
    }

    /// Applies the style and transform to the path and emits the result to the specified sink.
    pub fn apply<'a>(
        &mut self,
        data: impl PathData,
        style: impl Into<Style<'a>>,
        transform: Option<Transform>,
        sink: &mut impl PathBuilder,
    ) -> Fill {
        self.inner.apply(data, &style.into(), transform, sink)
    }

    /// Computes the bounding box of the path.
    pub fn bounds<'a>(
        &mut self,
        data: impl PathData,
        style: impl Into<Style<'a>>,
        transform: Option<Transform>,
    ) -> Bounds {
        let style = style.into();
        let mut bounds = BoundsBuilder::new();
        self.apply(data, style, transform, &mut bounds);
        bounds.build()
    }
}

#[derive(Default)]
pub(super) struct Inner {
    pub segments: Vec<Segment>,
}

impl Inner {
    pub fn apply(
        &mut self,
        data: impl PathData,
        style: &Style,
        transform: Option<Transform>,
        sink: &mut impl PathBuilder,
    ) -> Fill {
        match style {
            Style::Fill(fill) => {
                if let Some(transform) = transform {
                    let mut transform_sink = TransformSink { sink, transform };
                    data.copy_to(&mut transform_sink);
                    *fill
                } else {
                    data.copy_to(sink);
                    *fill
                }
            }
            Style::Stroke(stroke) => {
                if let Some(transform) = transform {
                    if stroke.scale {
                        let mut transform_sink = TransformSink { sink, transform };
                        stroke_with_storage(
                            data.commands(),
                            stroke,
                            &mut transform_sink,
                            &mut self.segments,
                        );
                    } else {
                        stroke_with_storage(
                            data.commands()
                                .map(|cmd| cmd.borrow().transform(&transform)),
                            stroke,
                            sink,
                            &mut self.segments,
                        );
                    }
                } else {
                    stroke_with_storage(data.commands(), stroke, sink, &mut self.segments);
                }
                Fill::NonZero
            }
        }
    }
}
