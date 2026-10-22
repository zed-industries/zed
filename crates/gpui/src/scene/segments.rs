use super::*;
use std::{cell::Cell, rc::Rc};

const BAND_WIDTH: u32 = 1 << 16;
// BatchIterator uses u32::MAX as the empty-stream sentinel.
const BAND_COUNT: u32 = u32::MAX / BAND_WIDTH;

pub(super) enum SegmentFallback {
    SourceFlat,
    CrossLayer,
    Range,
    Bands,
    OrderOverflow,
}

#[cfg(any(test, feature = "bench-support"))]
#[derive(Clone, Debug, Default)]
#[expect(missing_docs)]
pub struct SceneSegmentStats {
    pub recorded_runs: usize,
    pub reused_runs: usize,
    pub segmented_frames: usize,
    pub fallback_source_flat: usize,
    pub fallback_cross_layer: usize,
    pub fallback_range: usize,
    pub fallback_bands: usize,
    pub fallback_order_overflow: usize,
}

#[cfg(any(test, feature = "bench-support"))]
impl SceneSegmentStats {
    pub(crate) fn accumulate(&mut self, frame: &Self) {
        macro_rules! add {
            ($($field:ident),*) => { $(self.$field += frame.$field;)* };
        }
        add!(
            recorded_runs,
            reused_runs,
            segmented_frames,
            fallback_source_flat,
            fallback_cross_layer,
            fallback_range,
            fallback_bands,
            fallback_order_overflow
        );
    }

    pub(super) fn fallback(&mut self, reason: SegmentFallback) {
        let count = match reason {
            SegmentFallback::SourceFlat => &mut self.fallback_source_flat,
            SegmentFallback::CrossLayer => &mut self.fallback_cross_layer,
            SegmentFallback::Range => &mut self.fallback_range,
            SegmentFallback::Bands => &mut self.fallback_bands,
            SegmentFallback::OrderOverflow => &mut self.fallback_order_overflow,
        };
        *count += 1;
    }
}

struct Run {
    range: Range<usize>,
    scene: RunScene,
    band: Option<u32>,
    consumed: bool,
}

pub(super) enum RunScene {
    Recording(Box<Scene>),
    Kept(Rc<Scene>),
}

impl std::ops::Deref for RunScene {
    type Target = Scene;

    fn deref(&self) -> &Self::Target {
        match self {
            Self::Recording(scene) => scene,
            Self::Kept(scene) => scene,
        }
    }
}

impl RunScene {
    fn finish(self) -> Self {
        match self {
            Self::Recording(mut scene) => {
                scene.finish();
                Self::Kept(Rc::from(scene))
            }
            Self::Kept(scene) => Self::Kept(scene),
        }
    }
}

#[derive(Default)]
pub(super) struct SceneRuns {
    runs: Vec<Run>,
    index: SceneIndex,
    sealed: Cell<bool>,
}

impl SceneRuns {
    pub(super) fn index(&self) -> SceneIndex {
        match self.runs.last().map(|run| &run.scene) {
            Some(RunScene::Recording(scene)) => {
                scene.index().rebase(SceneIndex::default(), self.index)
            }
            _ => self.index,
        }
    }

    pub(super) fn checkpoint(&self) -> SceneIndex {
        // A layer reserves one order for all of its contents. Splitting it would
        // change interleaving between primitive kinds at that order.
        let index = self.index();
        if index.layer_depth == 0 {
            self.sealed.set(true);
        }
        index
    }

    pub(super) fn recording_scene(&mut self) -> &mut Scene {
        let sealed = std::mem::take(self.sealed.get_mut());
        if self.runs.is_empty() || sealed {
            self.index = self.index();
            let offset = self.index.operations.offset();
            self.runs.push(Run {
                range: offset..offset,
                scene: RunScene::Recording(Box::default()),
                band: None,
                consumed: false,
            });
        }
        let run = self.runs.last_mut().expect("recording run");
        match &mut run.scene {
            RunScene::Recording(scene) => scene,
            RunScene::Kept(_) => unreachable!("kept runs are sealed"),
        }
    }

    pub(super) fn reuse(
        &mut self,
        range: Range<SceneIndex>,
        previous: &mut Self,
    ) -> Result<usize, SegmentFallback> {
        let current = self.index();
        if range.start.layer_depth != 0 || range.end.layer_depth != 0 || current.layer_depth != 0 {
            return Err(SegmentFallback::CrossLayer);
        }
        let operations = range.start.operations.range_to(range.end.operations);
        if operations.is_empty() {
            return Ok(0);
        }
        let start = previous
            .runs
            .partition_point(|run| run.range.end <= operations.start);
        let end = previous
            .runs
            .partition_point(|run| run.range.start < operations.end);
        let Some(runs) = previous.runs.get_mut(start..end) else {
            return Err(SegmentFallback::Range);
        };
        if runs
            .first()
            .is_none_or(|run| run.range.start != operations.start)
            || runs
                .last()
                .is_none_or(|run| run.range.end != operations.end)
            || runs
                .iter()
                .any(|run| matches!(run.scene, RunScene::Recording(_)))
        {
            return Err(SegmentFallback::Range);
        }
        debug_assert!(
            !runs.iter().any(|run| run.consumed),
            "scene segment consumed twice"
        );
        let count = runs.len();
        self.index = current;
        for run in runs {
            let offset = self.index.operations.offset();
            let count = run.range.len();
            self.runs.push(Run {
                range: offset..offset + count,
                // Keep the old operation stream available if a later splice
                // crosses a layer and has to replay part of this frame.
                scene: match &run.scene {
                    RunScene::Kept(scene) => RunScene::Kept(scene.clone()),
                    RunScene::Recording(_) => unreachable!("source runs were checked"),
                },
                band: run.band,
                consumed: false,
            });
            run.consumed = true;
            self.index.operations = FramePosition::from_offset(offset + count);
        }
        self.index.ordering += range.end.ordering - range.start.ordering;
        for index in 0..self.index.primitives.len() {
            self.index.primitives[index] +=
                range.end.primitives[index] - range.start.primitives[index];
        }
        self.index.paths += range.end.paths - range.start.paths;
        self.index.surfaces += range.end.surfaces - range.start.surfaces;
        self.sealed.set(true);
        Ok(count)
    }

    fn assign_bands(&mut self) -> bool {
        // Leave gaps for new runs without renumbering the orders of clean runs.
        let mut start = 0;
        let mut minimum = 0;
        while start < self.runs.len() {
            let end = start
                + self.runs[start..]
                    .iter()
                    .position(|run| run.band.is_some())
                    .unwrap_or(self.runs.len() - start);
            let maximum = self
                .runs
                .get(end)
                .and_then(|run| run.band)
                .unwrap_or(BAND_COUNT);
            if maximum < minimum || maximum - minimum < (end - start) as u32 {
                return false;
            }
            let count = end - start;
            for (index, run) in self.runs[start..end].iter_mut().enumerate() {
                run.band = Some(
                    minimum
                        + (u64::from(maximum - minimum) * (index + 1) as u64 / (count + 1) as u64)
                            as u32,
                );
            }
            if end == self.runs.len() {
                break;
            }
            minimum = maximum + 1;
            start = end + 1;
        }
        true
    }

    pub(super) fn finish(&mut self, output: &mut Scene) -> Result<(), SegmentFallback> {
        self.index = self.index();
        for run in &mut self.runs {
            run.range.end = run.range.start + run.scene.len();
        }
        self.runs.retain(|run| !run.range.is_empty());
        if self.index.layer_depth != 0 {
            return Err(SegmentFallback::CrossLayer);
        }
        if !self.assign_bands() {
            return Err(SegmentFallback::Bands);
        }
        self.runs = std::mem::take(&mut self.runs)
            .into_iter()
            .map(|mut run| {
                run.scene = run.scene.finish();
                run
            })
            .collect();
        for run in &self.runs {
            let scene = &run.scene;
            macro_rules! check {
                ($($field:ident),*) => {
                    $(if scene.$field.last().is_some_and(|value| value.order >= BAND_WIDTH) {
                        return Err(SegmentFallback::OrderOverflow);
                    })*
                };
            }
            check!(
                shadows,
                quads,
                paths,
                underlines,
                monochrome_sprites,
                subpixel_sprites,
                polychrome_sprites,
                surfaces
            );
        }
        for run in &self.runs {
            let scene = &run.scene;
            let base = run.band.expect("assigned band") * BAND_WIDTH;
            macro_rules! append {
                ($($field:ident),*) => {
                    $(output.$field.extend(scene.$field.iter().map(|value| {
                        let mut value = value.clone();
                        value.order += base;
                        value
                    }));)*
                };
            }
            append!(
                shadows,
                quads,
                underlines,
                monochrome_sprites,
                subpixel_sprites,
                polychrome_sprites,
                surfaces
            );
            let path_offset = output.paths.len();
            output.paths.extend(scene.paths.iter().map(|path| {
                let mut path = path.clone();
                path.order += base;
                path.id.0 += path_offset;
                path
            }));
        }
        output.path_indices.resize(output.paths.len(), 0);
        for (index, path) in output.paths.iter().enumerate() {
            output.path_indices[path.id.0] = index;
        }
        self.sealed.set(true);
        Ok(())
    }

    pub(super) fn into_scenes(self) -> impl Iterator<Item = RunScene> {
        self.runs.into_iter().map(|run| run.scene)
    }

    #[cfg(any(test, feature = "bench-support"))]
    pub(super) fn recorded_count(&self) -> usize {
        self.runs
            .iter()
            .filter(|run| matches!(run.scene, RunScene::Recording(_)))
            .count()
    }

    #[cfg(debug_assertions)]
    pub(super) fn consumed_ranges(&self) -> Vec<Range<usize>> {
        self.runs
            .iter()
            .filter(|run| run.consumed)
            .map(|run| run.range.clone())
            .collect()
    }

    #[cfg(any(test, feature = "bench-support"))]
    pub(super) fn canonical_operations(&self) -> Vec<String> {
        let mut operations = Vec::new();
        let mut path_offset = 0;
        for run in &self.runs {
            operations.extend(run.scene.canonical_operations(path_offset));
            path_offset += run.scene.paths.len();
        }
        operations
    }
}
