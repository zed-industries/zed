// todo("windows"): remove
#![cfg_attr(windows, allow(dead_code))]

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{
    AtlasTextureId, AtlasTile, Background, Bounds, ContentMask, Corners, Edges, Hsla, Pixels,
    Point, Radians, ScaledPixels, Size,
    bounds_tree::BoundsTree,
    frame_array::{FrameArray, FramePosition},
    point,
};
use std::{
    fmt::Debug,
    iter::Peekable,
    ops::{Add, Range, Sub},
    slice,
};

mod segments;
#[cfg(any(test, feature = "bench-support"))]
pub use segments::SceneSegmentStats;
use segments::{SceneRuns, SegmentFallback};
#[cfg(any(test, feature = "bench-support"))]
mod equivalence;
#[cfg(any(test, feature = "bench-support"))]
pub(crate) use equivalence::SceneEquivalence;

#[allow(non_camel_case_types, unused)]
#[expect(missing_docs)]
pub type PathVertex_ScaledPixels = PathVertex<ScaledPixels>;

#[expect(missing_docs)]
pub type DrawOrder = u32;

#[derive(Clone, Copy, Debug, Default)]
pub(crate) struct SceneIndex {
    pub operations: FramePosition<PaintOperation>,
    ordering: usize,
    primitives: [usize; 6],
    paths: usize,
    surfaces: usize,
    layer_depth: usize,
}

impl SceneIndex {
    pub(crate) fn rebase(self, old: Self, new: Self) -> Self {
        Self {
            operations: self.operations.rebase(old.operations, new.operations),
            ordering: self.ordering - old.ordering + new.ordering,
            primitives: std::array::from_fn(|index| {
                self.primitives[index] - old.primitives[index] + new.primitives[index]
            }),
            paths: self.paths - old.paths + new.paths,
            surfaces: self.surfaces - old.surfaces + new.surfaces,
            layer_depth: self.layer_depth - old.layer_depth + new.layer_depth,
        }
    }
}

#[derive(Default)]
struct ReplayArrays {
    ready: bool,
    shadows: Vec<Shadow>,
    quads: Vec<Quad>,
    underlines: Vec<Underline>,
    monochrome_sprites: Vec<MonochromeSprite>,
    subpixel_sprites: Vec<SubpixelSprite>,
    polychrome_sprites: Vec<PolychromeSprite>,
}

/// A boolean stored as a `u32` so that GPU-facing structs contain no
/// compiler-inserted padding bytes, which would be undefined behavior to
/// reinterpret as `&[u8]` when writing instance buffers. Guaranteed to be
/// `0` or `1` by construction; shaders read it as a `u32`/`uint`.
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
#[repr(transparent)]
pub struct PaddedBool32(u32);

impl From<bool> for PaddedBool32 {
    fn from(value: bool) -> Self {
        PaddedBool32(value as u32)
    }
}

#[derive(Default)]
#[expect(missing_docs)]
pub struct Scene {
    runs: Option<SceneRuns>,
    #[cfg(any(test, feature = "bench-support"))]
    pub(crate) segment_stats: SceneSegmentStats,
    pub(crate) paint_operations: FrameArray<PaintOperation>,
    primitive_bounds: BoundsTree<ScaledPixels>,
    ordering_indices: Vec<usize>,
    layer_stack: Vec<DrawOrder>,
    pub shadows: Vec<Shadow>,
    pub quads: Vec<Quad>,
    pub paths: Vec<Path<ScaledPixels>>,
    pub underlines: Vec<Underline>,
    pub monochrome_sprites: Vec<MonochromeSprite>,
    pub subpixel_sprites: Vec<SubpixelSprite>,
    pub polychrome_sprites: Vec<PolychromeSprite>,
    pub surfaces: Vec<PaintSurface>,
    sprite_sort_keys: Vec<u128>,
    sprite_sort_counts: Vec<usize>,
    primitive_sort_indices: Vec<usize>,
    primitive_sort_counts: Vec<usize>,
    path_indices: Vec<usize>,
    surface_indices: Vec<usize>,
    replay_arrays: ReplayArrays,
    #[cfg(debug_assertions)]
    consumed_segments: Vec<Range<usize>>,
}

#[expect(missing_docs)]
impl Scene {
    pub fn clear(&mut self) {
        self.runs = None;
        #[cfg(any(test, feature = "bench-support"))]
        {
            self.segment_stats = SceneSegmentStats::default();
        }
        self.replay_arrays.ready = false;
        self.paint_operations.clear();
        self.primitive_bounds.clear();
        self.ordering_indices.clear();
        self.layer_stack.clear();
        self.paths.clear();
        self.shadows.clear();
        self.quads.clear();
        self.underlines.clear();
        self.monochrome_sprites.clear();
        self.subpixel_sprites.clear();
        self.polychrome_sprites.clear();
        self.surfaces.clear();
        self.path_indices.clear();
        self.surface_indices.clear();
        #[cfg(debug_assertions)]
        self.consumed_segments.clear();
    }

    pub fn len(&self) -> usize {
        if let Some(runs) = &self.runs {
            return runs.index().operations.offset();
        }
        self.paint_operations.len()
    }

    pub(crate) fn index(&self) -> SceneIndex {
        if let Some(runs) = &self.runs {
            return runs.checkpoint();
        }
        SceneIndex {
            operations: self.paint_operations.position(),
            ordering: self.primitive_bounds.len(),
            primitives: [
                self.shadows.len(),
                self.quads.len(),
                self.underlines.len(),
                self.monochrome_sprites.len(),
                self.subpixel_sprites.len(),
                self.polychrome_sprites.len(),
            ],
            paths: self.paths.len(),
            surfaces: self.surfaces.len(),
            layer_depth: self.layer_stack.len(),
        }
    }

    fn prepare_replay_arrays(&mut self) {
        if self.replay_arrays.ready {
            return;
        }
        let arrays = &mut self.replay_arrays;
        arrays.shadows.clear();
        arrays.quads.clear();
        arrays.underlines.clear();
        arrays.monochrome_sprites.clear();
        arrays.subpixel_sprites.clear();
        arrays.polychrome_sprites.clear();
        // Presentation sorts the typed arrays. The immutable operation stream
        // retains paint order, so its lazy snapshot gives segment-contiguous data.
        for operation in &self.paint_operations {
            if let PaintOperation::Primitive(primitive) = operation {
                match primitive {
                    RecordedPrimitive::Shadow(value) => arrays.shadows.push(*value),
                    RecordedPrimitive::Quad(value) => arrays.quads.push(*value),
                    RecordedPrimitive::Underline(value) => arrays.underlines.push(*value),
                    RecordedPrimitive::MonochromeSprite(value) => {
                        arrays.monochrome_sprites.push(*value)
                    }
                    RecordedPrimitive::SubpixelSprite(value) => {
                        arrays.subpixel_sprites.push(*value)
                    }
                    RecordedPrimitive::PolychromeSprite(value) => {
                        arrays.polychrome_sprites.push(*value)
                    }
                    RecordedPrimitive::Path { .. } | RecordedPrimitive::Surface { .. } => {}
                }
            }
        }
        arrays.ready = true;
    }

    pub(crate) fn replay_segment(
        &mut self,
        range: Range<SceneIndex>,
        previous: &mut Self,
        #[cfg(any(test, feature = "bench-support"))] stats: &mut crate::ViewReuseStats,
    ) {
        if self.runs.is_some() {
            let result = if let (Some(runs), Some(previous_runs)) =
                (self.runs.as_mut(), previous.runs.as_mut())
            {
                runs.reuse(range.clone(), previous_runs)
            } else {
                Err(SegmentFallback::SourceFlat)
            };
            match result {
                Ok(count) => {
                    #[cfg(any(test, feature = "bench-support"))]
                    {
                        stats.scene_bulk_segments += 1;
                        self.segment_stats.reused_runs += count;
                    }
                    #[cfg(not(any(test, feature = "bench-support")))]
                    let _ = count;
                    return;
                }
                Err(reason) => self.record_segment_fallback(reason),
            }
            previous.flatten_runs();
            let runs = self.runs.as_mut().expect("segmented recording");
            runs.recording_scene().replay(
                range.start.operations.range_to(range.end.operations),
                previous,
            );
            runs.checkpoint();
            return;
        }
        previous.flatten_runs();
        let bulk = range.start.layer_depth == 0
            && range.end.layer_depth == 0
            && self.layer_stack.is_empty()
            && range.start.paths == range.end.paths
            && range.start.surfaces == range.end.surfaces
            && self.primitive_bounds.try_replay_range(
                &previous.primitive_bounds,
                range.start.ordering..range.end.ordering,
            );
        if !bulk {
            self.replay(
                range.start.operations.range_to(range.end.operations),
                previous,
            );
            return;
        }
        #[cfg(debug_assertions)]
        {
            debug_assert!(
                previous.consumed_segments.iter().all(|prior| {
                    range.end.operations.offset() <= prior.start
                        || prior.end <= range.start.operations.offset()
                }),
                "scene segment consumed twice"
            );
            previous
                .consumed_segments
                .push(range.start.operations.range_to(range.end.operations));
        }
        previous.prepare_replay_arrays();
        macro_rules! copy {
            ($index:expr, $field:ident) => {{
                let values = &previous.replay_arrays.$field
                    [range.start.primitives[$index]..range.end.primitives[$index]];
                self.$field.extend_from_slice(values);
            }};
        }
        copy!(0, shadows);
        copy!(1, quads);
        copy!(2, underlines);
        copy!(3, monochrome_sprites);
        copy!(4, subpixel_sprites);
        copy!(5, polychrome_sprites);
        #[cfg(any(test, feature = "bench-support"))]
        {
            stats.scene_bulk_segments += 1;
        }
        self.paint_operations.carry_copy(
            &mut previous.paint_operations,
            range.start.operations..range.end.operations,
        );
        self.ordering_indices.extend_from_slice(
            &previous.ordering_indices[range.start.operations.range_to(range.end.operations)],
        );
    }

    pub fn push_layer(&mut self, bounds: Bounds<ScaledPixels>) {
        if let Some(runs) = self.runs.as_mut() {
            runs.recording_scene().push_layer(bounds);
            return;
        }
        self.ordering_indices.push(self.primitive_bounds.len());
        let order = self.primitive_bounds.insert(bounds);
        self.layer_stack.push(order);
        self.paint_operations
            .push(PaintOperation::StartLayer(bounds));
    }

    pub fn pop_layer(&mut self) {
        if let Some(runs) = self.runs.as_mut() {
            runs.recording_scene().pop_layer();
            return;
        }
        self.ordering_indices.push(self.primitive_bounds.len());
        self.layer_stack.pop();
        self.paint_operations.push(PaintOperation::EndLayer);
    }

    #[inline(always)]
    pub(crate) fn record_primitive(&mut self, primitive: impl Into<Primitive>) {
        // Materializing the enum before this branch prevents specialization of
        // the flat path for concrete primitive types.
        if let Some(runs) = self.runs.as_mut() {
            runs.recording_scene().insert_primitive(primitive);
        } else {
            self.insert_primitive(primitive);
        }
    }

    pub fn insert_primitive(&mut self, primitive: impl Into<Primitive>) {
        debug_assert!(self.runs.is_none(), "segmented scenes use record_primitive");
        let ordering_index = self.primitive_bounds.len();
        if let Some(operation) = self.prepare_primitive(primitive.into(), None) {
            self.ordering_indices.push(ordering_index);
            self.paint_operations.push(operation);
        }
    }

    fn prepare_primitive(
        &mut self,
        primitive: Primitive,
        previous: Option<(&BoundsTree<ScaledPixels>, usize)>,
    ) -> Option<PaintOperation> {
        let clipped_bounds = primitive
            .bounds()
            .intersect(&primitive.content_mask().bounds);

        if clipped_bounds.is_empty() {
            return None;
        }

        let order = self.layer_stack.last().copied().unwrap_or_else(|| {
            if let Some((previous, index)) = previous {
                self.primitive_bounds
                    .insert_replayed(clipped_bounds, previous, index)
            } else {
                self.primitive_bounds.insert(clipped_bounds)
            }
        });
        let primitive = match primitive {
            Primitive::Shadow(mut shadow) => {
                shadow.order = order;
                self.shadows.push(shadow);
                RecordedPrimitive::Shadow(shadow)
            }
            Primitive::Quad(mut quad) => {
                quad.order = order;
                self.quads.push(quad);
                RecordedPrimitive::Quad(quad)
            }
            Primitive::Path(mut path) => {
                path.order = order;
                path.id = PathId(self.paths.len());
                let record = RecordedPrimitive::Path {
                    id: path.id,
                    bounds: path.bounds,
                    content_mask: path.content_mask,
                };
                self.paths.push(path);
                record
            }
            Primitive::Underline(mut underline) => {
                underline.order = order;
                self.underlines.push(underline);
                RecordedPrimitive::Underline(underline)
            }
            Primitive::MonochromeSprite(mut sprite) => {
                sprite.order = order;
                self.monochrome_sprites.push(sprite);
                RecordedPrimitive::MonochromeSprite(sprite)
            }
            Primitive::SubpixelSprite(mut sprite) => {
                sprite.order = order;
                self.subpixel_sprites.push(sprite);
                RecordedPrimitive::SubpixelSprite(sprite)
            }
            Primitive::PolychromeSprite(mut sprite) => {
                sprite.order = order;
                self.polychrome_sprites.push(sprite);
                RecordedPrimitive::PolychromeSprite(sprite)
            }
            Primitive::Surface(mut surface) => {
                surface.order = order;
                let record = RecordedPrimitive::Surface {
                    index: self.surfaces.len(),
                    bounds: surface.bounds,
                    content_mask: surface.content_mask,
                };
                self.surfaces.push(surface);
                record
            }
        };
        Some(PaintOperation::Primitive(primitive))
    }

    pub fn replay(&mut self, range: Range<usize>, prev_scene: &mut Scene) {
        self.flatten_runs();
        prev_scene.flatten_runs();
        #[cfg(debug_assertions)]
        if !range.is_empty() {
            debug_assert!(
                prev_scene
                    .consumed_segments
                    .iter()
                    .all(|prior| { range.end <= prior.start || prior.end <= range.start }),
                "scene segment consumed twice"
            );
            prev_scene.consumed_segments.push(range.clone());
        }
        let start = self.paint_operations.len();
        let previous_start = range.start;
        self.paint_operations
            .extend_from_slice(&prev_scene.paint_operations[range]);
        for index in start..self.paint_operations.len() {
            let previous_index = prev_scene.ordering_indices[previous_start + index - start];
            self.ordering_indices.push(self.primitive_bounds.len());
            let operation = self.paint_operations[index];
            match operation {
                PaintOperation::Primitive(primitive) => {
                    let primitive = match primitive {
                        RecordedPrimitive::Shadow(value) => Primitive::Shadow(value),
                        RecordedPrimitive::Quad(value) => Primitive::Quad(value),
                        RecordedPrimitive::Underline(value) => Primitive::Underline(value),
                        RecordedPrimitive::MonochromeSprite(value) => {
                            Primitive::MonochromeSprite(value)
                        }
                        RecordedPrimitive::SubpixelSprite(value) => {
                            Primitive::SubpixelSprite(value)
                        }
                        RecordedPrimitive::PolychromeSprite(value) => {
                            Primitive::PolychromeSprite(value)
                        }
                        RecordedPrimitive::Path {
                            id,
                            bounds,
                            content_mask,
                        } => {
                            let source_index = prev_scene.path_index(id);
                            let path = &mut prev_scene.paths[source_index];
                            debug_assert_eq!(path.bounds, bounds);
                            debug_assert_eq!(path.content_mask, content_mask);
                            Primitive::Path(Path {
                                id: path.id,
                                order: path.order,
                                bounds: path.bounds,
                                content_mask: path.content_mask,
                                vertices: std::mem::take(&mut path.vertices),
                                color: path.color,
                                start: path.start,
                                current: path.current,
                                contour_count: path.contour_count,
                            })
                        }
                        RecordedPrimitive::Surface {
                            index,
                            bounds,
                            content_mask,
                        } => {
                            let surface = prev_scene.surface(index);
                            debug_assert_eq!(surface.bounds, bounds);
                            debug_assert_eq!(surface.content_mask, content_mask);
                            Primitive::Surface(surface.clone())
                        }
                    };
                    if let Some(operation) = self.prepare_primitive(
                        primitive,
                        Some((&prev_scene.primitive_bounds, previous_index)),
                    ) {
                        self.paint_operations[index] = operation;
                    }
                }
                PaintOperation::StartLayer(bounds) => {
                    let order = self.primitive_bounds.insert_replayed(
                        bounds,
                        &prev_scene.primitive_bounds,
                        previous_index,
                    );
                    self.layer_stack.push(order);
                }
                PaintOperation::EndLayer => {
                    self.layer_stack.pop();
                }
            }
        }
    }

    fn path_index(&self, id: PathId) -> usize {
        self.path_indices.get(id.0).copied().unwrap_or(id.0)
    }

    #[cfg(any(test, feature = "bench-support"))]
    fn path(&self, id: PathId) -> &Path<ScaledPixels> {
        &self.paths[self.path_index(id)]
    }

    fn surface(&self, index: usize) -> &PaintSurface {
        &self.surfaces[self.surface_indices.get(index).copied().unwrap_or(index)]
    }

    #[cfg(any(test, feature = "bench-support"))]
    pub(crate) fn rendered_snapshot(&self) -> Vec<String> {
        let mut primitives = Vec::new();
        macro_rules! append {
            ($field:ident) => {
                primitives.extend(self.$field.iter().map(|primitive| {
                    (
                        primitive.order,
                        format!("{}:{primitive:?}", stringify!($field)),
                    )
                }));
            };
        }
        append!(shadows);
        append!(quads);
        append!(paths);
        append!(underlines);
        append!(monochrome_sprites);
        append!(subpixel_sprites);
        append!(polychrome_sprites);
        append!(surfaces);
        // GPU batches are stored by kind, not painter order. Preserve order values and
        // the relative sequence within each kind, including texture-sorted sprite ties.
        primitives.sort_by_key(|(order, _)| *order);
        primitives
            .into_iter()
            .map(|(_, primitive)| primitive)
            .collect()
    }

    #[cfg(any(test, feature = "bench-support"))]
    pub(crate) fn reuse_snapshot(&self) -> Vec<String> {
        if let Some(runs) = &self.runs {
            return runs.canonical_operations();
        }
        self.paint_operations
            .iter()
            .map(|operation| match operation {
                PaintOperation::Primitive(primitive) => match primitive {
                    RecordedPrimitive::Shadow(value) => format!("shadow:{value:?}"),
                    RecordedPrimitive::Quad(value) => format!("quad:{value:?}"),
                    RecordedPrimitive::Path { id, .. } => format!("path:{:?}", self.path(*id)),
                    RecordedPrimitive::Underline(value) => format!("underline:{value:?}"),
                    RecordedPrimitive::MonochromeSprite(value) => format!("monochrome:{value:?}"),
                    RecordedPrimitive::SubpixelSprite(value) => format!("subpixel:{value:?}"),
                    RecordedPrimitive::PolychromeSprite(value) => format!("polychrome:{value:?}"),
                    RecordedPrimitive::Surface { index, .. } => {
                        format!("surface:{:?}", self.surface(*index))
                    }
                },
                PaintOperation::StartLayer(bounds) => format!("layer:{bounds:?}"),
                PaintOperation::EndLayer => "end-layer".into(),
            })
            .collect()
    }

    pub fn finish(&mut self) {
        if let Some(mut runs) = self.runs.take() {
            #[cfg(any(test, feature = "bench-support"))]
            {
                self.segment_stats.recorded_runs += runs.recorded_count();
            }
            match runs.finish(self) {
                Ok(()) => {
                    self.runs = Some(runs);
                    #[cfg(any(test, feature = "bench-support"))]
                    {
                        self.segment_stats.segmented_frames += 1;
                    }
                    return;
                }
                Err(reason) => self.record_segment_fallback(reason),
            }
            self.runs = Some(runs);
            self.flatten_runs();
        }
        let primitive_count = self
            .shadows
            .len()
            .max(self.quads.len())
            .max(self.underlines.len());
        if self.primitive_sort_indices.capacity() > primitive_count.saturating_mul(2) {
            self.primitive_sort_indices.shrink_to(primitive_count);
        }
        if self.primitive_sort_counts.capacity() > primitive_count.saturating_mul(8).min(131_072) {
            self.primitive_sort_counts
                .shrink_to(primitive_count.saturating_mul(4).min(65_536));
        }
        sort_primitives(
            &mut self.shadows,
            &mut self.primitive_sort_indices,
            &mut self.primitive_sort_counts,
            |shadow| shadow.order,
        );
        sort_primitives(
            &mut self.quads,
            &mut self.primitive_sort_indices,
            &mut self.primitive_sort_counts,
            |quad| quad.order,
        );
        sort_if_needed(&mut self.paths, |path| path.order);
        self.path_indices.resize(self.paths.len(), 0);
        for (index, path) in self.paths.iter().enumerate() {
            self.path_indices[path.id.0] = index;
        }
        sort_primitives(
            &mut self.underlines,
            &mut self.primitive_sort_indices,
            &mut self.primitive_sort_counts,
            |underline| underline.order,
        );
        let sprite_count = self
            .monochrome_sprites
            .len()
            .max(self.subpixel_sprites.len())
            .max(self.polychrome_sprites.len());
        if self.sprite_sort_keys.capacity() > sprite_count.saturating_mul(2) {
            self.sprite_sort_keys.shrink_to(sprite_count);
        }
        if self.sprite_sort_counts.capacity() > sprite_count.saturating_mul(8) {
            self.sprite_sort_counts
                .shrink_to(sprite_count.saturating_mul(4).min(65_536));
        }
        sort_sprites(
            &mut self.monochrome_sprites,
            &mut self.sprite_sort_keys,
            &mut self.sprite_sort_counts,
            |sprite| (sprite.order, sprite.tile.texture_id.index),
        );
        sort_sprites(
            &mut self.subpixel_sprites,
            &mut self.sprite_sort_keys,
            &mut self.sprite_sort_counts,
            |sprite| (sprite.order, sprite.tile.texture_id.index),
        );
        sort_sprites(
            &mut self.polychrome_sprites,
            &mut self.sprite_sort_keys,
            &mut self.sprite_sort_counts,
            |sprite| (sprite.order, sprite.tile.texture_id.index),
        );
        if !self.surfaces.is_empty() {
            let mut surfaces: Vec<_> = self.surfaces.drain(..).enumerate().collect();
            surfaces.sort_by_key(|(_, surface)| surface.order);
            self.surface_indices
                .extend(self.surface_indices.len()..surfaces.len());
            let mut permutation = vec![0; surfaces.len()];
            for (index, (old_index, surface)) in surfaces.into_iter().enumerate() {
                permutation[old_index] = index;
                self.surfaces.push(surface);
            }
            for index in &mut self.surface_indices {
                *index = permutation[*index];
            }
        }
    }

    #[cfg_attr(
        all(
            any(target_os = "linux", target_os = "freebsd"),
            not(any(feature = "x11", feature = "wayland"))
        ),
        allow(dead_code)
    )]
    pub fn batches(&self) -> impl Iterator<Item = PrimitiveBatch> + '_ {
        BatchIterator {
            shadows_start: 0,
            shadows_iter: self.shadows.iter().peekable(),
            quads_start: 0,
            quads_iter: self.quads.iter().peekable(),
            paths_start: 0,
            paths_iter: self.paths.iter().peekable(),
            underlines_start: 0,
            underlines_iter: self.underlines.iter().peekable(),
            monochrome_sprites_start: 0,
            monochrome_sprites_iter: self.monochrome_sprites.iter().peekable(),
            subpixel_sprites_start: 0,
            subpixel_sprites_iter: self.subpixel_sprites.iter().peekable(),
            polychrome_sprites_start: 0,
            polychrome_sprites_iter: self.polychrome_sprites.iter().peekable(),
            surfaces_start: 0,
            surfaces_iter: self.surfaces.iter().peekable(),
        }
    }
}

impl Scene {
    fn record_segment_fallback(&mut self, _reason: SegmentFallback) {
        #[cfg(any(test, feature = "bench-support"))]
        self.segment_stats.fallback(_reason);
    }

    pub(crate) fn enable_segments(&mut self) {
        if self.runs.is_none() && self.paint_operations.is_empty() {
            self.runs = Some(SceneRuns::default());
        }
    }

    fn flatten_runs(&mut self) {
        let Some(runs) = self.runs.take() else {
            return;
        };
        #[cfg(debug_assertions)]
        let consumed = runs.consumed_ranges();
        #[cfg(any(test, feature = "bench-support"))]
        let stats = std::mem::take(&mut self.segment_stats);
        self.clear();
        #[cfg(any(test, feature = "bench-support"))]
        {
            self.segment_stats = stats;
        }
        for scene in runs.into_scenes() {
            for operation in &scene.paint_operations {
                match *operation {
                    PaintOperation::StartLayer(bounds) => self.push_layer(bounds),
                    PaintOperation::EndLayer => self.pop_layer(),
                    PaintOperation::Primitive(primitive) => {
                        let primitive = match primitive {
                            RecordedPrimitive::Shadow(value) => Primitive::Shadow(value),
                            RecordedPrimitive::Quad(value) => Primitive::Quad(value),
                            RecordedPrimitive::Underline(value) => Primitive::Underline(value),
                            RecordedPrimitive::MonochromeSprite(value) => {
                                Primitive::MonochromeSprite(value)
                            }
                            RecordedPrimitive::SubpixelSprite(value) => {
                                Primitive::SubpixelSprite(value)
                            }
                            RecordedPrimitive::PolychromeSprite(value) => {
                                Primitive::PolychromeSprite(value)
                            }
                            RecordedPrimitive::Path { id, .. } => {
                                Primitive::Path(scene.paths[scene.path_index(id)].clone())
                            }
                            RecordedPrimitive::Surface { index, .. } => {
                                Primitive::Surface(scene.surface(index).clone())
                            }
                        };
                        self.insert_primitive(primitive);
                    }
                }
            }
        }
        #[cfg(debug_assertions)]
        {
            self.consumed_segments = consumed;
        }
    }
}

fn sort_if_needed<T, K: Ord>(primitives: &mut [T], key: impl Fn(&T) -> K) {
    if !primitives.is_sorted_by_key(&key) {
        primitives.sort_by_key(key);
    }
}

fn sort_primitives<T: Copy>(
    primitives: &mut [T],
    indices: &mut Vec<usize>,
    counts: &mut Vec<usize>,
    key: impl Fn(&T) -> DrawOrder,
) {
    if primitives.is_sorted_by_key(&key) {
        return;
    }
    if primitives.len() < 32 {
        primitives.sort_by_key(key);
        return;
    }
    let mut minimum = u32::MAX;
    let mut maximum = 0;
    for primitive in primitives.iter() {
        let order = key(primitive);
        minimum = minimum.min(order);
        maximum = maximum.max(order);
    }
    let buckets = u64::from(maximum) - u64::from(minimum) + 1;
    if buckets > primitives.len().saturating_mul(4).min(65_536) as u64 {
        primitives.sort_by_key(key);
        return;
    }
    counts.resize(buckets as usize, 0);
    for primitive in primitives.iter() {
        counts[(key(primitive) - minimum) as usize] += 1;
    }
    let mut offset = 0;
    for count in counts.iter_mut() {
        let length = *count;
        *count = offset;
        offset += length;
    }
    indices.resize(primitives.len(), 0);
    for (index, primitive) in primitives.iter().enumerate() {
        let destination = &mut counts[(key(primitive) - minimum) as usize];
        indices[*destination] = index;
        *destination += 1;
    }
    counts.clear();
    for start in 0..indices.len() {
        if indices[start] == start {
            continue;
        }
        let primitive = primitives[start];
        let mut destination = start;
        loop {
            let source = indices[destination];
            indices[destination] = destination;
            if source == start {
                primitives[destination] = primitive;
                break;
            }
            primitives[destination] = primitives[source];
            destination = source;
        }
    }
    indices.clear();
}

fn sort_sprites<T: Copy>(
    primitives: &mut [T],
    keys: &mut Vec<u128>,
    counts: &mut Vec<usize>,
    key: impl Fn(&T) -> (u32, u32),
) {
    if primitives.is_sorted_by_key(&key) {
        return;
    }
    if primitives.len() < 256 || u32::try_from(primitives.len()).is_err() {
        primitives.sort_by_key(key);
        return;
    }

    let mut minimum = (u32::MAX, u32::MAX);
    let mut maximum = (0, 0);
    for primitive in primitives.iter() {
        let (order, tile) = key(primitive);
        minimum = (minimum.0.min(order), minimum.1.min(tile));
        maximum = (maximum.0.max(order), maximum.1.max(tile));
    }
    let orders = u64::from(maximum.0) - u64::from(minimum.0) + 1;
    let tiles = u64::from(maximum.1) - u64::from(minimum.1) + 1;
    let buckets = orders
        .checked_mul(tiles)
        .filter(|buckets| *buckets <= primitives.len().saturating_mul(4).min(65_536) as u64);
    keys.clear();
    if let Some(buckets) = buckets {
        let bucket = |primitive: &T| {
            let (order, tile) = key(primitive);
            ((u64::from(order - minimum.0) * tiles) + u64::from(tile - minimum.1)) as usize
        };
        counts.clear();
        counts.resize(buckets as usize, 0);
        for primitive in primitives.iter() {
            counts[bucket(primitive)] += 1;
        }
        let mut offset = 0;
        for count in counts.iter_mut() {
            let length = *count;
            *count = offset;
            offset += length;
        }
        keys.resize(primitives.len(), 0);
        for (index, primitive) in primitives.iter().enumerate() {
            let destination = &mut counts[bucket(primitive)];
            keys[*destination] = index as u128;
            *destination += 1;
        }
        counts.clear();
    } else {
        keys.extend(primitives.iter().enumerate().map(|(index, primitive)| {
            let (order, tile_id) = key(primitive);
            // Equal-key sprites can overlap, so the insertion index must remain
            // the final tie-breaker even though the compact keys use an unstable sort.
            ((order as u128) << 64) | ((tile_id as u128) << 32) | index as u128
        }));
        keys.sort_unstable();
    }
    for start in 0..keys.len() {
        if keys[start] as u32 as usize == start {
            continue;
        }
        let primitive = primitives[start];
        let mut destination = start;
        loop {
            let source = keys[destination] as u32 as usize;
            // The low bits double as a visited marker; the sorted high bits
            // are no longer needed once the permutation has been constructed.
            keys[destination] = destination as u128;
            if source == start {
                primitives[destination] = primitive;
                break;
            }
            primitives[destination] = primitives[source];
            destination = source;
        }
    }
    keys.clear();
}

#[cfg(test)]
mod sort_tests {
    use super::*;

    #[test]
    fn ordinary_counting_sort_matches_stable_oracle() {
        let mut indices = Vec::new();
        let mut counts = Vec::new();
        for length in [0, 1, 2, 31, 32, 33, 132, 256, 16_385] {
            let bound: u32 = (length * 4).min(65_536);
            for maximum in [0, 1, bound.saturating_sub(1), bound, u32::MAX] {
                let original: Vec<_> = (0..length)
                    .map(|index| {
                        let order = if index % 3 == 0 { maximum } else { 0 };
                        (order, index)
                    })
                    .collect();
                let mut expected = original.clone();
                expected.sort_by_key(|value| value.0);
                let mut actual = original;
                sort_primitives(&mut actual, &mut indices, &mut counts, |value| value.0);
                assert_eq!(actual, expected);
                sort_primitives(&mut actual, &mut indices, &mut counts, |value| value.0);
                assert_eq!(actual, expected);
                assert!(indices.is_empty());
                assert!(counts.is_empty());
                assert!(counts.capacity() <= 65_536);
            }
        }
    }

    #[test]
    fn ordinary_sort_releases_oversized_scratch() {
        let mut scene = Scene::default();
        scene.primitive_sort_indices.reserve(8192);
        scene.primitive_sort_counts.reserve(8192);
        scene.finish();
        assert_eq!(scene.primitive_sort_indices.capacity(), 0);
        assert_eq!(scene.primitive_sort_counts.capacity(), 0);
    }

    #[test]
    fn already_sorted_preserves_stable_order() {
        for count in [0, 1, 16, 255, 256, 8192] {
            let original: Vec<_> = (0..count).map(|index| (index % 7, index)).collect();
            let mut expected = original.clone();
            expected.sort_by_key(|primitive| primitive.0);
            let mut actual = original;
            sort_if_needed(&mut actual, |primitive| primitive.0);
            assert_eq!(actual, expected);
            sort_if_needed(&mut actual, |primitive| primitive.0);
            assert_eq!(actual, expected);
        }
    }

    #[test]
    fn compact_keys_preserve_stable_order() {
        use rand::{Rng, SeedableRng, rngs::StdRng};

        let mut keys = Vec::new();
        let mut counts = Vec::new();
        for seed in 0..32 {
            let mut random = StdRng::seed_from_u64(seed);
            for count in [0, 1, 16, 255, 256, 257, 8192] {
                let original: Vec<_> = (0..count)
                    .map(|index| (random.random_range(0..8), random.random_range(0..8), index))
                    .collect();
                let mut expected = original.clone();
                expected.sort_by_key(|primitive| (primitive.0, primitive.1));
                let mut actual = original;
                sort_sprites(&mut actual, &mut keys, &mut counts, |primitive| {
                    (primitive.0, primitive.1)
                });
                assert_eq!(actual, expected);
                sort_sprites(&mut actual, &mut keys, &mut counts, |primitive| {
                    (primitive.0, primitive.1)
                });
                assert_eq!(actual, expected);
                assert!(keys.is_empty());
            }
        }
    }

    #[test]
    fn sprite_counting_sort_handles_high_and_sparse_keys() {
        let mut keys = Vec::new();
        let mut counts = Vec::new();
        for sparse in [false, true] {
            let original = (0..4096)
                .map(|index| {
                    let order = if sparse && index % 2 == 0 {
                        0
                    } else {
                        u32::MAX - index % 8
                    };
                    let tile = if sparse && index % 3 == 0 {
                        0
                    } else {
                        u32::MAX - index % 16
                    };
                    (order, tile, index)
                })
                .collect::<Vec<_>>();
            let mut expected = original.clone();
            expected.sort_by_key(|value| (value.0, value.1));
            let mut actual = original;
            sort_sprites(&mut actual, &mut keys, &mut counts, |value| {
                (value.0, value.1)
            });
            assert_eq!(actual, expected);
        }
    }

    #[test]
    fn sprite_sort_releases_oversized_scratch() {
        let mut scene = Scene::default();
        scene.sprite_sort_keys.reserve(8192);
        scene.sprite_sort_counts.reserve(8192);
        scene.finish();
        assert_eq!(scene.sprite_sort_keys.capacity(), 0);
        assert_eq!(scene.sprite_sort_counts.capacity(), 0);
    }

    #[test]
    fn scene_finish_matches_stable_sort_and_batches() {
        use crate::{AtlasTextureKind, DevicePixels, TileId, size};
        use rand::{Rng, SeedableRng, rngs::StdRng};

        for seed in 0..16 {
            let mut random = StdRng::seed_from_u64(seed);
            for count in [0, 1, 31, 32, 33, 132, 255, 256, 257, 2048] {
                let mut scene = Scene::default();
                for index in 0..count {
                    let bounds = Bounds {
                        origin: point(
                            ScaledPixels(random.random_range(0.0..8.0)),
                            ScaledPixels(random.random_range(0.0..8.0)),
                        ),
                        size: size(ScaledPixels(32.0), ScaledPixels(32.0)),
                    };
                    let content_mask = ContentMask { bounds };
                    let color = Hsla {
                        h: index as f32 / count as f32,
                        s: 0.5,
                        l: 0.5,
                        a: 1.0,
                    };
                    if index % 16 == 0 {
                        scene.push_layer(bounds);
                    }
                    let tile = AtlasTile {
                        texture_id: AtlasTextureId {
                            index: random.random_range(0..3),
                            kind: AtlasTextureKind::Monochrome,
                        },
                        tile_id: TileId(random.random_range(0..8)),
                        padding: index,
                        bounds: Bounds {
                            origin: point(DevicePixels(0), DevicePixels(0)),
                            size: size(DevicePixels(8), DevicePixels(8)),
                        },
                    };
                    let transformation = TransformationMatrix::unit()
                        .rotate(Radians(random.random_range(-3.0..3.0)))
                        .translate(point(ScaledPixels(64.0), ScaledPixels(-64.0)));
                    scene.insert_primitive(MonochromeSprite {
                        order: 0,
                        pad: 0,
                        bounds,
                        content_mask,
                        color,
                        tile,
                        transformation,
                    });
                    scene.insert_primitive(SubpixelSprite {
                        order: 0,
                        pad: 0,
                        bounds,
                        content_mask,
                        color,
                        tile: AtlasTile {
                            texture_id: AtlasTextureId {
                                kind: AtlasTextureKind::Subpixel,
                                ..tile.texture_id
                            },
                            ..tile
                        },
                        transformation,
                    });
                    scene.insert_primitive(PolychromeSprite {
                        order: 0,
                        pad: 0,
                        grayscale: (index % 2 == 0).into(),
                        opacity: color.h,
                        bounds,
                        content_mask,
                        corner_radii: Corners::default(),
                        tile: AtlasTile {
                            texture_id: AtlasTextureId {
                                kind: AtlasTextureKind::Polychrome,
                                ..tile.texture_id
                            },
                            ..tile
                        },
                    });
                    scene.insert_primitive(Quad {
                        bounds,
                        content_mask,
                        border_color: color,
                        ..Quad::default()
                    });
                    scene.insert_primitive(Shadow {
                        order: 0,
                        blur_radius: ScaledPixels(index as f32),
                        bounds,
                        corner_radii: Corners::default(),
                        content_mask,
                        color,
                        element_bounds: bounds,
                        element_corner_radii: Corners::default(),
                        inset: index % 2,
                        pad: 0,
                    });
                    scene.insert_primitive(Underline {
                        order: 0,
                        pad: 0,
                        bounds,
                        content_mask,
                        color,
                        thickness: ScaledPixels(index as f32),
                        wavy: (index % 2 == 0).into(),
                    });
                    let mut path = Path::new(point(Pixels(0.0), Pixels(0.0))).scale(1.0);
                    path.bounds = bounds;
                    path.content_mask = content_mask;
                    scene.insert_primitive(path);
                    #[cfg(not(any(target_os = "macos", target_os = "ios")))]
                    scene.insert_primitive(PaintSurface {
                        order: 0,
                        bounds,
                        content_mask,
                    });
                    if index % 16 == 15 || index + 1 == count {
                        scene.pop_layer();
                    }
                }

                let mut expected = Scene::default();
                macro_rules! boundary_orders {
                    ($field:ident) => {
                        if seed % 4 != 0 {
                            for (index, primitive) in scene.$field.iter_mut().enumerate() {
                                primitive.order = if index % 2 == 0 {
                                    match seed % 4 {
                                        1 => u32::MAX,
                                        2 => count as u32 * 4 - 1,
                                        _ => count as u32 * 4,
                                    }
                                } else {
                                    0
                                };
                            }
                        }
                    };
                }
                boundary_orders!(shadows);
                boundary_orders!(quads);
                boundary_orders!(paths);
                boundary_orders!(underlines);
                boundary_orders!(monochrome_sprites);
                boundary_orders!(subpixel_sprites);
                boundary_orders!(polychrome_sprites);
                boundary_orders!(surfaces);
                macro_rules! stable_sort {
                    ($field:ident, $key:expr) => {
                        expected.$field = scene.$field.clone();
                        expected.$field.sort_by_key($key);
                    };
                }
                stable_sort!(shadows, |primitive| primitive.order);
                stable_sort!(quads, |primitive| primitive.order);
                stable_sort!(paths, |primitive| primitive.order);
                stable_sort!(underlines, |primitive| primitive.order);
                stable_sort!(monochrome_sprites, |primitive| (
                    primitive.order,
                    primitive.tile.texture_id.index
                ));
                stable_sort!(subpixel_sprites, |primitive| (
                    primitive.order,
                    primitive.tile.texture_id.index
                ));
                stable_sort!(polychrome_sprites, |primitive| (
                    primitive.order,
                    primitive.tile.texture_id.index
                ));
                stable_sort!(surfaces, |primitive| primitive.order);
                scene.finish();
                macro_rules! compare {
                    ($field:ident) => {
                        assert_eq!(scene.$field.len(), expected.$field.len());
                        for (actual, expected) in scene.$field.iter().zip(&expected.$field) {
                            assert_eq!(format!("{actual:?}"), format!("{expected:?}"));
                        }
                    };
                }
                compare!(shadows);
                compare!(quads);
                compare!(paths);
                compare!(underlines);
                compare!(monochrome_sprites);
                compare!(subpixel_sprites);
                compare!(polychrome_sprites);
                compare!(surfaces);
                assert_eq!(
                    format!("{:?}", scene.batches().collect::<Vec<_>>()),
                    format!("{:?}", expected.batches().collect::<Vec<_>>())
                );
            }
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Default)]
#[cfg_attr(
    all(
        any(target_os = "linux", target_os = "freebsd"),
        not(any(feature = "x11", feature = "wayland"))
    ),
    allow(dead_code)
)]
pub(crate) enum PrimitiveKind {
    Shadow,
    #[default]
    Quad,
    Path,
    Underline,
    MonochromeSprite,
    SubpixelSprite,
    PolychromeSprite,
    Surface,
}

#[derive(Clone, Copy)]
pub(crate) enum PaintOperation {
    Primitive(RecordedPrimitive),
    StartLayer(Bounds<ScaledPixels>),
    EndLayer,
}

#[derive(Clone, Copy)]
pub(crate) enum RecordedPrimitive {
    Shadow(Shadow),
    Quad(Quad),
    Path {
        id: PathId,
        bounds: Bounds<ScaledPixels>,
        content_mask: ContentMask<ScaledPixels>,
    },
    Underline(Underline),
    MonochromeSprite(MonochromeSprite),
    SubpixelSprite(SubpixelSprite),
    PolychromeSprite(PolychromeSprite),
    Surface {
        index: usize,
        bounds: Bounds<ScaledPixels>,
        content_mask: ContentMask<ScaledPixels>,
    },
}

#[derive(Clone)]
#[expect(missing_docs)]
pub enum Primitive {
    Shadow(Shadow),
    Quad(Quad),
    Path(Path<ScaledPixels>),
    Underline(Underline),
    MonochromeSprite(MonochromeSprite),
    SubpixelSprite(SubpixelSprite),
    PolychromeSprite(PolychromeSprite),
    Surface(PaintSurface),
}

#[expect(missing_docs)]
impl Primitive {
    pub fn bounds(&self) -> &Bounds<ScaledPixels> {
        match self {
            Primitive::Shadow(shadow) => &shadow.bounds,
            Primitive::Quad(quad) => &quad.bounds,
            Primitive::Path(path) => &path.bounds,
            Primitive::Underline(underline) => &underline.bounds,
            Primitive::MonochromeSprite(sprite) => &sprite.bounds,
            Primitive::SubpixelSprite(sprite) => &sprite.bounds,
            Primitive::PolychromeSprite(sprite) => &sprite.bounds,
            Primitive::Surface(surface) => &surface.bounds,
        }
    }

    pub fn content_mask(&self) -> &ContentMask<ScaledPixels> {
        match self {
            Primitive::Shadow(shadow) => &shadow.content_mask,
            Primitive::Quad(quad) => &quad.content_mask,
            Primitive::Path(path) => &path.content_mask,
            Primitive::Underline(underline) => &underline.content_mask,
            Primitive::MonochromeSprite(sprite) => &sprite.content_mask,
            Primitive::SubpixelSprite(sprite) => &sprite.content_mask,
            Primitive::PolychromeSprite(sprite) => &sprite.content_mask,
            Primitive::Surface(surface) => &surface.content_mask,
        }
    }
}

#[cfg_attr(
    all(
        any(target_os = "linux", target_os = "freebsd"),
        not(any(feature = "x11", feature = "wayland"))
    ),
    allow(dead_code)
)]
struct BatchIterator<'a> {
    shadows_start: usize,
    shadows_iter: Peekable<slice::Iter<'a, Shadow>>,
    quads_start: usize,
    quads_iter: Peekable<slice::Iter<'a, Quad>>,
    paths_start: usize,
    paths_iter: Peekable<slice::Iter<'a, Path<ScaledPixels>>>,
    underlines_start: usize,
    underlines_iter: Peekable<slice::Iter<'a, Underline>>,
    monochrome_sprites_start: usize,
    monochrome_sprites_iter: Peekable<slice::Iter<'a, MonochromeSprite>>,
    subpixel_sprites_start: usize,
    subpixel_sprites_iter: Peekable<slice::Iter<'a, SubpixelSprite>>,
    polychrome_sprites_start: usize,
    polychrome_sprites_iter: Peekable<slice::Iter<'a, PolychromeSprite>>,
    surfaces_start: usize,
    surfaces_iter: Peekable<slice::Iter<'a, PaintSurface>>,
}

impl<'a> Iterator for BatchIterator<'a> {
    type Item = PrimitiveBatch;

    fn next(&mut self) -> Option<Self::Item> {
        let mut orders_and_kinds = [
            (
                self.shadows_iter.peek().map(|s| s.order),
                PrimitiveKind::Shadow,
            ),
            (self.quads_iter.peek().map(|q| q.order), PrimitiveKind::Quad),
            (self.paths_iter.peek().map(|q| q.order), PrimitiveKind::Path),
            (
                self.underlines_iter.peek().map(|u| u.order),
                PrimitiveKind::Underline,
            ),
            (
                self.monochrome_sprites_iter.peek().map(|s| s.order),
                PrimitiveKind::MonochromeSprite,
            ),
            (
                self.subpixel_sprites_iter.peek().map(|s| s.order),
                PrimitiveKind::SubpixelSprite,
            ),
            (
                self.polychrome_sprites_iter.peek().map(|s| s.order),
                PrimitiveKind::PolychromeSprite,
            ),
            (
                self.surfaces_iter.peek().map(|s| s.order),
                PrimitiveKind::Surface,
            ),
        ];
        orders_and_kinds.sort_by_key(|(order, kind)| (order.unwrap_or(u32::MAX), *kind));

        let first = orders_and_kinds[0];
        let second = orders_and_kinds[1];
        let (batch_kind, max_order_and_kind) = if first.0.is_some() {
            (first.1, (second.0.unwrap_or(u32::MAX), second.1))
        } else {
            return None;
        };

        match batch_kind {
            PrimitiveKind::Shadow => {
                let shadows_start = self.shadows_start;
                let mut shadows_end = shadows_start + 1;
                self.shadows_iter.next();
                while self
                    .shadows_iter
                    .next_if(|shadow| (shadow.order, batch_kind) < max_order_and_kind)
                    .is_some()
                {
                    shadows_end += 1;
                }
                self.shadows_start = shadows_end;
                Some(PrimitiveBatch::Shadows(shadows_start..shadows_end))
            }
            PrimitiveKind::Quad => {
                let quads_start = self.quads_start;
                let mut quads_end = quads_start + 1;
                self.quads_iter.next();
                while self
                    .quads_iter
                    .next_if(|quad| (quad.order, batch_kind) < max_order_and_kind)
                    .is_some()
                {
                    quads_end += 1;
                }
                self.quads_start = quads_end;
                Some(PrimitiveBatch::Quads(quads_start..quads_end))
            }
            PrimitiveKind::Path => {
                let paths_start = self.paths_start;
                let mut paths_end = paths_start + 1;
                self.paths_iter.next();
                while self
                    .paths_iter
                    .next_if(|path| (path.order, batch_kind) < max_order_and_kind)
                    .is_some()
                {
                    paths_end += 1;
                }
                self.paths_start = paths_end;
                Some(PrimitiveBatch::Paths(paths_start..paths_end))
            }
            PrimitiveKind::Underline => {
                let underlines_start = self.underlines_start;
                let mut underlines_end = underlines_start + 1;
                self.underlines_iter.next();
                while self
                    .underlines_iter
                    .next_if(|underline| (underline.order, batch_kind) < max_order_and_kind)
                    .is_some()
                {
                    underlines_end += 1;
                }
                self.underlines_start = underlines_end;
                Some(PrimitiveBatch::Underlines(underlines_start..underlines_end))
            }
            PrimitiveKind::MonochromeSprite => {
                let texture_id = self.monochrome_sprites_iter.peek().unwrap().tile.texture_id;
                let sprites_start = self.monochrome_sprites_start;
                let mut sprites_end = sprites_start + 1;
                self.monochrome_sprites_iter.next();
                while self
                    .monochrome_sprites_iter
                    .next_if(|sprite| {
                        (sprite.order, batch_kind) < max_order_and_kind
                            && sprite.tile.texture_id == texture_id
                    })
                    .is_some()
                {
                    sprites_end += 1;
                }
                self.monochrome_sprites_start = sprites_end;
                Some(PrimitiveBatch::MonochromeSprites {
                    texture_id,
                    range: sprites_start..sprites_end,
                })
            }
            PrimitiveKind::SubpixelSprite => {
                let texture_id = self.subpixel_sprites_iter.peek().unwrap().tile.texture_id;
                let sprites_start = self.subpixel_sprites_start;
                let mut sprites_end = sprites_start + 1;
                self.subpixel_sprites_iter.next();
                while self
                    .subpixel_sprites_iter
                    .next_if(|sprite| {
                        (sprite.order, batch_kind) < max_order_and_kind
                            && sprite.tile.texture_id == texture_id
                    })
                    .is_some()
                {
                    sprites_end += 1;
                }
                self.subpixel_sprites_start = sprites_end;
                Some(PrimitiveBatch::SubpixelSprites {
                    texture_id,
                    range: sprites_start..sprites_end,
                })
            }
            PrimitiveKind::PolychromeSprite => {
                let texture_id = self.polychrome_sprites_iter.peek().unwrap().tile.texture_id;
                let sprites_start = self.polychrome_sprites_start;
                let mut sprites_end = sprites_start + 1;
                self.polychrome_sprites_iter.next();
                while self
                    .polychrome_sprites_iter
                    .next_if(|sprite| {
                        (sprite.order, batch_kind) < max_order_and_kind
                            && sprite.tile.texture_id == texture_id
                    })
                    .is_some()
                {
                    sprites_end += 1;
                }
                self.polychrome_sprites_start = sprites_end;
                Some(PrimitiveBatch::PolychromeSprites {
                    texture_id,
                    range: sprites_start..sprites_end,
                })
            }
            PrimitiveKind::Surface => {
                let surfaces_start = self.surfaces_start;
                let mut surfaces_end = surfaces_start + 1;
                self.surfaces_iter.next();
                while self
                    .surfaces_iter
                    .next_if(|surface| (surface.order, batch_kind) < max_order_and_kind)
                    .is_some()
                {
                    surfaces_end += 1;
                }
                self.surfaces_start = surfaces_end;
                Some(PrimitiveBatch::Surfaces(surfaces_start..surfaces_end))
            }
        }
    }
}

#[derive(Debug)]
#[cfg_attr(
    all(
        any(target_os = "linux", target_os = "freebsd"),
        not(any(feature = "x11", feature = "wayland"))
    ),
    allow(dead_code)
)]
#[allow(missing_docs)]
pub enum PrimitiveBatch {
    Shadows(Range<usize>),
    Quads(Range<usize>),
    Paths(Range<usize>),
    Underlines(Range<usize>),
    MonochromeSprites {
        texture_id: AtlasTextureId,
        range: Range<usize>,
    },
    #[cfg_attr(target_os = "macos", allow(dead_code))]
    SubpixelSprites {
        texture_id: AtlasTextureId,
        range: Range<usize>,
    },
    PolychromeSprites {
        texture_id: AtlasTextureId,
        range: Range<usize>,
    },
    Surfaces(Range<usize>),
}

impl PrimitiveBatch {
    #[expect(missing_docs)]
    pub fn label(&self) -> String {
        match self {
            Self::Shadows(range) => format!("shadows ({})", range.len()),
            Self::Quads(range) => format!("quads ({})", range.len()),
            Self::Paths(range) => format!("paths ({})", range.len()),
            Self::Underlines(range) => format!("underlines ({})", range.len()),
            Self::MonochromeSprites { texture_id, range } => {
                format!(
                    "monochrome sprites ({}) on atlas {}",
                    range.len(),
                    texture_id.index
                )
            }
            Self::SubpixelSprites { texture_id, range } => {
                format!(
                    "subpixel sprites ({}) on atlas {}",
                    range.len(),
                    texture_id.index
                )
            }
            Self::PolychromeSprites { texture_id, range } => {
                format!(
                    "polychrome sprites ({}) on atlas {}",
                    range.len(),
                    texture_id.index
                )
            }
            Self::Surfaces(range) => format!("surfaces ({})", range.len()),
        }
    }
}

#[derive(Default, Debug, Copy, Clone)]
#[repr(C)]
#[expect(missing_docs)]
pub struct Quad {
    pub order: DrawOrder,
    pub border_style: BorderStyle,
    pub bounds: Bounds<ScaledPixels>,
    pub content_mask: ContentMask<ScaledPixels>,
    pub background: Background,
    pub border_color: Hsla,
    pub corner_radii: Corners<ScaledPixels>,
    pub border_widths: Edges<ScaledPixels>,
}

impl From<Quad> for Primitive {
    fn from(quad: Quad) -> Self {
        Primitive::Quad(quad)
    }
}

#[derive(Debug, Copy, Clone)]
#[repr(C)]
#[expect(missing_docs)]
pub struct Underline {
    pub order: DrawOrder,
    pub pad: u32, // align to 8 bytes
    pub bounds: Bounds<ScaledPixels>,
    pub content_mask: ContentMask<ScaledPixels>,
    pub color: Hsla,
    pub thickness: ScaledPixels,
    pub wavy: PaddedBool32,
}

impl From<Underline> for Primitive {
    fn from(underline: Underline) -> Self {
        Primitive::Underline(underline)
    }
}

#[derive(Debug, Copy, Clone)]
#[repr(C)]
#[expect(missing_docs)]
pub struct Shadow {
    pub order: DrawOrder,
    pub blur_radius: ScaledPixels,
    pub bounds: Bounds<ScaledPixels>,
    pub corner_radii: Corners<ScaledPixels>,
    pub content_mask: ContentMask<ScaledPixels>,
    pub color: Hsla,
    pub element_bounds: Bounds<ScaledPixels>,
    pub element_corner_radii: Corners<ScaledPixels>,
    /// 0 = drop shadow (rendered outside the element), 1 = inset shadow (rendered inside).
    pub inset: u32,
    pub pad: u32, // align to 8 bytes
}

impl From<Shadow> for Primitive {
    fn from(shadow: Shadow) -> Self {
        Primitive::Shadow(shadow)
    }
}

/// The style of a border.
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
#[repr(C)]
pub enum BorderStyle {
    /// A solid border.
    #[default]
    Solid = 0,
    /// A dashed border.
    Dashed = 1,
}

/// A data type representing a 2 dimensional transformation that can be applied to an element.
#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct TransformationMatrix {
    /// 2x2 matrix containing rotation and scale,
    /// stored row-major
    pub rotation_scale: [[f32; 2]; 2],
    /// translation vector
    pub translation: [f32; 2],
}

impl Eq for TransformationMatrix {}

impl TransformationMatrix {
    /// The unit matrix, has no effect.
    pub fn unit() -> Self {
        Self {
            rotation_scale: [[1.0, 0.0], [0.0, 1.0]],
            translation: [0.0, 0.0],
        }
    }

    /// Move the origin by a given point
    pub fn translate(mut self, point: Point<ScaledPixels>) -> Self {
        self.compose(Self {
            rotation_scale: [[1.0, 0.0], [0.0, 1.0]],
            translation: [point.x.0, point.y.0],
        })
    }

    /// Clockwise rotation in radians around the origin
    pub fn rotate(self, angle: Radians) -> Self {
        self.compose(Self {
            rotation_scale: [
                [angle.0.cos(), -angle.0.sin()],
                [angle.0.sin(), angle.0.cos()],
            ],
            translation: [0.0, 0.0],
        })
    }

    /// Scale around the origin
    pub fn scale(self, size: Size<f32>) -> Self {
        self.compose(Self {
            rotation_scale: [[size.width, 0.0], [0.0, size.height]],
            translation: [0.0, 0.0],
        })
    }

    /// Perform matrix multiplication with another transformation
    /// to produce a new transformation that is the result of
    /// applying both transformations: first, `other`, then `self`.
    #[inline]
    pub fn compose(self, other: TransformationMatrix) -> TransformationMatrix {
        if other == Self::unit() {
            return self;
        }
        // Perform matrix multiplication
        TransformationMatrix {
            rotation_scale: [
                [
                    self.rotation_scale[0][0] * other.rotation_scale[0][0]
                        + self.rotation_scale[0][1] * other.rotation_scale[1][0],
                    self.rotation_scale[0][0] * other.rotation_scale[0][1]
                        + self.rotation_scale[0][1] * other.rotation_scale[1][1],
                ],
                [
                    self.rotation_scale[1][0] * other.rotation_scale[0][0]
                        + self.rotation_scale[1][1] * other.rotation_scale[1][0],
                    self.rotation_scale[1][0] * other.rotation_scale[0][1]
                        + self.rotation_scale[1][1] * other.rotation_scale[1][1],
                ],
            ],
            translation: [
                self.translation[0]
                    + self.rotation_scale[0][0] * other.translation[0]
                    + self.rotation_scale[0][1] * other.translation[1],
                self.translation[1]
                    + self.rotation_scale[1][0] * other.translation[0]
                    + self.rotation_scale[1][1] * other.translation[1],
            ],
        }
    }

    /// Apply transformation to a point, mainly useful for debugging
    pub fn apply(&self, point: Point<Pixels>) -> Point<Pixels> {
        let input = [point.x.0, point.y.0];
        let mut output = self.translation;
        for (i, output_cell) in output.iter_mut().enumerate() {
            for (k, input_cell) in input.iter().enumerate() {
                *output_cell += self.rotation_scale[i][k] * *input_cell;
            }
        }
        Point::new(output[0].into(), output[1].into())
    }
}

impl Default for TransformationMatrix {
    fn default() -> Self {
        Self::unit()
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
#[expect(missing_docs)]
pub struct MonochromeSprite {
    pub order: DrawOrder,
    pub pad: u32,
    pub tile: AtlasTile,
    pub bounds: Bounds<ScaledPixels>,
    pub content_mask: ContentMask<ScaledPixels>,
    pub color: Hsla,
    pub transformation: TransformationMatrix,
}

impl From<MonochromeSprite> for Primitive {
    fn from(sprite: MonochromeSprite) -> Self {
        Primitive::MonochromeSprite(sprite)
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
#[expect(missing_docs)]
pub struct SubpixelSprite {
    pub order: DrawOrder,
    pub pad: u32, // align to 8 bytes
    pub tile: AtlasTile,
    pub bounds: Bounds<ScaledPixels>,
    pub content_mask: ContentMask<ScaledPixels>,
    pub color: Hsla,
    pub transformation: TransformationMatrix,
}

impl From<SubpixelSprite> for Primitive {
    fn from(sprite: SubpixelSprite) -> Self {
        Primitive::SubpixelSprite(sprite)
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
#[expect(missing_docs)]
pub struct PolychromeSprite {
    pub order: DrawOrder,
    pub pad: u32,
    pub tile: AtlasTile,
    pub grayscale: PaddedBool32,
    pub opacity: f32,
    pub bounds: Bounds<ScaledPixels>,
    pub content_mask: ContentMask<ScaledPixels>,
    pub corner_radii: Corners<ScaledPixels>,
}

impl From<PolychromeSprite> for Primitive {
    fn from(sprite: PolychromeSprite) -> Self {
        Primitive::PolychromeSprite(sprite)
    }
}

#[derive(Clone, Debug)]
#[allow(missing_docs)]
pub struct PaintSurface {
    pub order: DrawOrder,
    pub bounds: Bounds<ScaledPixels>,
    pub content_mask: ContentMask<ScaledPixels>,
    #[cfg(any(target_os = "macos", target_os = "ios"))]
    pub image_buffer: objc2_core_foundation::CFRetained<objc2_core_video::CVPixelBuffer>,
}

impl From<PaintSurface> for Primitive {
    fn from(surface: PaintSurface) -> Self {
        Primitive::Surface(surface)
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[expect(missing_docs)]
pub struct PathId(pub usize);

/// A line made up of a series of vertices and control points.
#[derive(Clone, Debug)]
#[expect(missing_docs)]
pub struct Path<P: Clone + Debug + Default + PartialEq> {
    pub id: PathId,
    pub order: DrawOrder,
    pub bounds: Bounds<P>,
    pub content_mask: ContentMask<P>,
    pub vertices: Vec<PathVertex<P>>,
    pub color: Background,
    start: Point<P>,
    current: Point<P>,
    contour_count: usize,
}

impl Path<Pixels> {
    /// Create a new path with the given starting point.
    pub fn new(start: Point<Pixels>) -> Self {
        Self {
            id: PathId(0),
            order: DrawOrder::default(),
            vertices: Vec::new(),
            start,
            current: start,
            bounds: Bounds {
                origin: start,
                size: Default::default(),
            },
            content_mask: Default::default(),
            color: Default::default(),
            contour_count: 0,
        }
    }

    /// Scale this path by the given factor.
    pub fn scale(&self, factor: f32) -> Path<ScaledPixels> {
        Path {
            id: self.id,
            order: self.order,
            bounds: self.bounds.scale(factor),
            content_mask: self.content_mask.scale(factor),
            vertices: self
                .vertices
                .iter()
                .map(|vertex| vertex.scale(factor))
                .collect(),
            start: self.start.map(|start| start.scale(factor)),
            current: self.current.scale(factor),
            contour_count: self.contour_count,
            color: self.color,
        }
    }

    /// Move the start, current point to the given point.
    pub fn move_to(&mut self, to: Point<Pixels>) {
        self.contour_count += 1;
        self.start = to;
        self.current = to;
    }

    /// Draw a straight line from the current point to the given point.
    pub fn line_to(&mut self, to: Point<Pixels>) {
        self.contour_count += 1;
        if self.contour_count > 1 {
            self.push_triangle(
                (self.start, self.current, to),
                (point(0., 1.), point(0., 1.), point(0., 1.)),
            );
        }
        self.current = to;
    }

    /// Draw a curve from the current point to the given point, using the given control point.
    pub fn curve_to(&mut self, to: Point<Pixels>, ctrl: Point<Pixels>) {
        self.contour_count += 1;
        if self.contour_count > 1 {
            self.push_triangle(
                (self.start, self.current, to),
                (point(0., 1.), point(0., 1.), point(0., 1.)),
            );
        }

        self.push_triangle(
            (self.current, ctrl, to),
            (point(0., 0.), point(0.5, 0.), point(1., 1.)),
        );
        self.current = to;
    }

    /// Push a triangle to the Path.
    pub fn push_triangle(
        &mut self,
        xy: (Point<Pixels>, Point<Pixels>, Point<Pixels>),
        st: (Point<f32>, Point<f32>, Point<f32>),
    ) {
        self.bounds = self
            .bounds
            .union(&Bounds {
                origin: xy.0,
                size: Default::default(),
            })
            .union(&Bounds {
                origin: xy.1,
                size: Default::default(),
            })
            .union(&Bounds {
                origin: xy.2,
                size: Default::default(),
            });

        self.vertices.push(PathVertex {
            xy_position: xy.0,
            st_position: st.0,
            content_mask: Default::default(),
        });
        self.vertices.push(PathVertex {
            xy_position: xy.1,
            st_position: st.1,
            content_mask: Default::default(),
        });
        self.vertices.push(PathVertex {
            xy_position: xy.2,
            st_position: st.2,
            content_mask: Default::default(),
        });
    }
}

impl<T> Path<T>
where
    T: Clone + Debug + Default + PartialEq + PartialOrd + Add<T, Output = T> + Sub<Output = T>,
{
    #[allow(unused)]
    #[expect(missing_docs)]
    pub fn clipped_bounds(&self) -> Bounds<T> {
        self.bounds.intersect(&self.content_mask.bounds)
    }
}

impl From<Path<ScaledPixels>> for Primitive {
    fn from(path: Path<ScaledPixels>) -> Self {
        Primitive::Path(path)
    }
}

#[derive(Clone, Debug)]
#[repr(C)]
#[expect(missing_docs)]
pub struct PathVertex<P: Clone + Debug + Default + PartialEq> {
    pub xy_position: Point<P>,
    pub st_position: Point<f32>,
    pub content_mask: ContentMask<P>,
}

#[expect(missing_docs)]
impl PathVertex<Pixels> {
    pub fn scale(&self, factor: f32) -> PathVertex<ScaledPixels> {
        PathVertex {
            xy_position: self.xy_position.scale(factor),
            st_position: self.st_position,
            content_mask: self.content_mask.scale(factor),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{px, size};

    #[test]
    fn segment_checkpoints_rebase_inherited_layer_depth() {
        let mut source = Scene::default();
        let old = source.index();
        source.push_layer(quad().bounds);
        let descendant = source.index();
        let mut target = Scene::default();
        target.push_layer(quad().bounds);
        target.push_layer(quad().bounds);
        let new = target.index();
        let rebased = descendant.rebase(old, new);
        target.push_layer(quad().bounds);
        assert_eq!(rebased.layer_depth, target.index().layer_depth);
        assert_eq!(rebased.ordering, target.index().ordering);
        assert_eq!(
            rebased.operations.offset(),
            target.index().operations.offset()
        );
        assert_eq!(rebased.rebase(new, old).layer_depth, descendant.layer_depth);
    }

    fn quad() -> Quad {
        let bounds = Bounds::new(point(px(0.), px(0.)), size(px(20.), px(20.))).scale(1.);
        Quad {
            bounds,
            content_mask: ContentMask { bounds },
            ..Quad::default()
        }
    }

    fn path() -> Path<ScaledPixels> {
        let mut path = Path::new(point(px(0.), px(0.)));
        path.line_to(point(px(10.), px(0.)));
        path.line_to(point(px(0.), px(10.)));
        path.content_mask = ContentMask {
            bounds: Bounds::new(point(px(0.), px(0.)), size(px(20.), px(20.))),
        };
        path.scale(1.)
    }

    #[test]
    fn segmented_runs_keep_closed_layers_paths_and_surfaces() {
        fn paint(scene: &mut Scene) {
            scene.push_layer(quad().bounds);
            scene.record_primitive(path());
            scene.push_layer(quad().bounds);
            scene.record_primitive(quad());
            scene.pop_layer();
            #[cfg(not(any(target_os = "macos", target_os = "ios")))]
            scene.record_primitive(PaintSurface {
                order: 0,
                bounds: quad().bounds,
                content_mask: quad().content_mask,
            });
            scene.pop_layer();
        }
        let mut source = Scene::default();
        source.enable_segments();
        let start = source.index();
        paint(&mut source);
        let end = source.index();
        source.finish();
        assert_eq!(source.segment_stats.segmented_frames, 1);

        let mut target = Scene::default();
        target.enable_segments();
        target.record_primitive(path());
        target.replay_segment(
            start..end,
            &mut source,
            &mut crate::ViewReuseStats::default(),
        );
        target.finish();
        let mut fresh = Scene::default();
        fresh.insert_primitive(path());
        paint(&mut fresh);
        fresh.finish();
        target
            .equivalence_snapshot()
            .compare(&fresh.equivalence_snapshot())
            .expect("closed layer run matches fresh");
        assert_eq!(target.segment_stats.reused_runs, 1);
        assert_eq!(target.segment_stats.recorded_runs, 1);
        assert_eq!(target.segment_stats.segmented_frames, 1);
        assert_eq!(
            target
                .paths
                .iter()
                .map(|path| path.id.0)
                .collect::<Vec<_>>(),
            vec![0, 1]
        );
    }

    #[test]
    fn cross_layer_fallback_preserves_already_reused_runs() {
        let mut source = Scene::default();
        source.enable_segments();
        let start = source.index();
        source.record_primitive(quad());
        let prefix_end = source.index();
        source.push_layer(quad().bounds);
        let inside_start = source.index();
        source.record_primitive(path());
        let inside_end = source.index();
        source.pop_layer();
        source.finish();

        let mut target = Scene::default();
        target.enable_segments();
        let mut stats = crate::ViewReuseStats::default();
        target.replay_segment(start..prefix_end, &mut source, &mut stats);
        target.push_layer(quad().bounds);
        target.replay_segment(inside_start..inside_end, &mut source, &mut stats);
        target.pop_layer();
        target.finish();

        let mut fresh = Scene::default();
        fresh.insert_primitive(quad());
        fresh.push_layer(quad().bounds);
        fresh.insert_primitive(path());
        fresh.pop_layer();
        fresh.finish();
        target
            .equivalence_snapshot()
            .compare(&fresh.equivalence_snapshot())
            .expect("cross-layer fallback matches fresh");
        assert_eq!(target.segment_stats.reused_runs, 1);
        assert_eq!(target.segment_stats.fallback_cross_layer, 1);
        assert_eq!(target.segment_stats.segmented_frames, 1);
    }

    #[test]
    fn replacing_a_run_keeps_later_bands() {
        let mut source = Scene::default();
        source.enable_segments();
        let start = source.index();
        source.record_primitive(quad());
        let prefix_end = source.index();
        source.record_primitive(path());
        let suffix_start = source.index();
        source.record_primitive(quad());
        let end = source.index();
        source.finish();
        let suffix_order = source.quads[1].order;

        let mut target = Scene::default();
        target.enable_segments();
        let mut stats = crate::ViewReuseStats::default();
        target.replay_segment(start..prefix_end, &mut source, &mut stats);
        target.record_primitive(path());
        target.index();
        target.record_primitive(path());
        target.replay_segment(suffix_start..end, &mut source, &mut stats);
        target.finish();
        assert_eq!(target.quads[1].order, suffix_order);
        assert_eq!(target.segment_stats.reused_runs, 2);
        assert_eq!(target.segment_stats.recorded_runs, 2);
        assert_eq!(target.segment_stats.segmented_frames, 1);
        let mut fresh = Scene::default();
        fresh.insert_primitive(quad());
        fresh.insert_primitive(path());
        fresh.insert_primitive(path());
        fresh.insert_primitive(quad());
        fresh.finish();
        target
            .equivalence_snapshot()
            .compare(&fresh.equivalence_snapshot())
            .expect("replaced run matches fresh");
    }

    #[test]
    fn reordered_clean_runs_fall_back_without_losing_primitives() {
        let mut source = Scene::default();
        source.enable_segments();
        let start = source.index();
        source.record_primitive(quad());
        let middle = source.index();
        source.record_primitive(path());
        let end = source.index();
        source.finish();

        let mut target = Scene::default();
        target.enable_segments();
        let mut stats = crate::ViewReuseStats::default();
        target.replay_segment(middle..end, &mut source, &mut stats);
        target.replay_segment(start..middle, &mut source, &mut stats);
        target.finish();
        assert_eq!(target.segment_stats.fallback_bands, 1);
        let mut fresh = Scene::default();
        fresh.insert_primitive(path());
        fresh.insert_primitive(quad());
        fresh.finish();
        target
            .equivalence_snapshot()
            .compare(&fresh.equivalence_snapshot())
            .expect("reordered runs match fresh");
    }

    #[test]
    fn flat_sources_are_imported_without_disabling_segments() {
        let mut source = Scene::default();
        let start = source.index();
        source.insert_primitive(path());
        let end = source.index();
        source.finish();
        let expected = source.equivalence_snapshot();
        let mut target = Scene::default();
        target.enable_segments();
        target.replay_segment(
            start..end,
            &mut source,
            &mut crate::ViewReuseStats::default(),
        );
        target.finish();
        target
            .equivalence_snapshot()
            .compare(&expected)
            .expect("imported flat source matches fresh");
        assert_eq!(target.segment_stats.fallback_source_flat, 1);
        assert_eq!(target.segment_stats.segmented_frames, 1);
    }

    #[test]
    fn replay_moves_paths_and_recomputes_order_in_nested_layers() {
        let mut source = Scene::default();
        source.insert_primitive(quad());
        source.push_layer(quad().bounds);
        source.insert_primitive(path());
        source.push_layer(quad().bounds);
        source.insert_primitive(path());
        source.pop_layer();
        source.pop_layer();
        source.insert_primitive(path());
        source.finish();
        let old_bounds: Vec<_> = source
            .paths
            .iter()
            .map(|path| (path.id, path.bounds))
            .collect();
        let allocations: Vec<_> = source
            .paths
            .iter()
            .map(|path| path.vertices.as_ptr())
            .collect();
        let mut target = Scene::default();
        let mut fresh = Scene::default();
        for scene in [&mut target, &mut fresh] {
            scene.insert_primitive(quad());
            scene.insert_primitive(quad());
        }
        target.replay(0..source.len(), &mut source);
        fresh.insert_primitive(quad());
        fresh.push_layer(quad().bounds);
        fresh.insert_primitive(path());
        fresh.push_layer(quad().bounds);
        fresh.insert_primitive(path());
        fresh.pop_layer();
        fresh.pop_layer();
        fresh.insert_primitive(path());
        target.finish();
        fresh.finish();
        assert_eq!(target.reuse_snapshot(), fresh.reuse_snapshot());
        assert_eq!(
            target
                .paths
                .iter()
                .map(|path| path.vertices.as_ptr())
                .collect::<Vec<_>>(),
            allocations
        );
        assert!(source.paths.iter().all(|path| path.vertices.is_empty()));
        assert_eq!(
            source
                .paths
                .iter()
                .map(|path| (path.id, path.bounds))
                .collect::<Vec<_>>(),
            old_bounds
        );
        let mut next = Scene::default();
        next.replay(0..target.len(), &mut target);
        next.finish();
        assert_eq!(next.reuse_snapshot(), fresh.reuse_snapshot());
    }

    #[cfg(debug_assertions)]
    #[test]
    #[should_panic(expected = "scene segment consumed twice")]
    fn replay_rejects_double_scene_consumption() {
        let mut source = Scene::default();
        source.insert_primitive(path());
        source.finish();
        let mut target = Scene::default();
        target.replay(0..1, &mut source);
        target.replay(0..1, &mut source);
    }

    #[test]
    fn replay_resolves_path_ids_after_painter_order_sorting() {
        let mut source = Scene::default();
        source.insert_primitive(quad());
        source.insert_primitive(path());
        let mut separate = path();
        separate.bounds.origin.x = ScaledPixels(100.);
        separate.content_mask.bounds = separate.bounds;
        source.insert_primitive(separate);
        source.finish();
        source.finish();
        assert_eq!(source.paths[0].id, PathId(1));
        let expected = source.reuse_snapshot();
        let mut target = Scene::default();
        target.replay(0..source.len(), &mut source);
        target.finish();
        assert_eq!(target.reuse_snapshot(), expected);
    }

    #[test]
    fn retained_segments_match_fresh_paint_with_overlapping_layers() {
        use rand::{Rng, SeedableRng};
        for seed in 0..16 {
            let mut random = rand::rngs::StdRng::seed_from_u64(seed);
            let mut segments: Vec<Vec<Quad>> = (0..24)
                .map(|_| {
                    (0..4)
                        .map(|_| {
                            let mut quad = quad();
                            quad.bounds.origin = point(
                                ScaledPixels(random.random_range(-32.0..64.0)),
                                ScaledPixels(random.random_range(-32.0..64.0)),
                            );
                            quad.content_mask.bounds = quad.bounds;
                            quad
                        })
                        .collect()
                })
                .collect();
            let mut previous = Scene::default();
            let paint = |scene: &mut Scene, segment: &[Quad], layered: bool| {
                if layered {
                    scene.push_layer(quad().bounds);
                }
                for quad in segment {
                    scene.insert_primitive(*quad);
                }
                if layered {
                    scene.pop_layer();
                }
            };
            let mut ranges = Vec::new();
            for (index, segment) in segments.iter().enumerate() {
                let start = previous.index();
                paint(&mut previous, segment, index % 3 == 0);
                ranges.push(start..previous.index());
            }
            previous.finish();
            let mut stats = crate::ViewReuseStats::default();
            for iteration in 0..8 {
                let mut target = Scene::default();
                let mut fresh = Scene::default();
                let mut next_ranges = Vec::new();
                for (index, segment) in segments.iter_mut().enumerate() {
                    let start = target.index();
                    let layered = index % 3 == 0;
                    if random.random_range(0..4) == 0 {
                        segment[0].bounds.origin.x += ScaledPixels(8.);
                        segment[0].content_mask.bounds = segment[0].bounds;
                        paint(&mut target, segment, layered);
                    } else {
                        target.replay_segment(ranges[index].clone(), &mut previous, &mut stats);
                    }
                    paint(&mut fresh, segment, layered);
                    next_ranges.push(start..target.index());
                }
                target.finish();
                fresh.finish();
                assert_eq!(
                    target.reuse_snapshot(),
                    fresh.reuse_snapshot(),
                    "seed={seed} frame={iteration}"
                );
                previous = target;
                ranges = next_ranges;
            }
            assert!(stats.scene_bulk_segments > 0);
        }
    }
}
