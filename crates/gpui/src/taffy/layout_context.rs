use crate::taffy::inline_layout::{InlineMeasureContext, compute_inline_layout_impl};
use crate::taffy::tree::node_id_to_default_key;
use crate::taffy::{AvailableSpace, GpuiNodeContext, NodeMeasureCtx, TaffyLayoutEngine};
use crate::{App, Pixels, Size, Window, size};
use stacksafe::StackSafe;
use std::sync::Arc;
use taffy::{
    CacheTree, LayoutBlockContainer, LayoutFlexboxContainer, LayoutGridContainer,
    LayoutPartialTree, RoundTree, TraversePartialTree, TraverseTree, compute_block_layout,
    compute_cached_layout, compute_flexbox_layout, compute_grid_layout, compute_leaf_layout,
    geometry::Size as TaffySize,
    style::AvailableSpace as TaffyAvailableSpace,
    tree::{Layout, LayoutInput, LayoutOutput, NodeId, RunMode},
};

pub struct LayoutContext<'a> {
    pub(crate) engine: &'a mut TaffyLayoutEngine,
    pub(crate) window: &'a mut Window,
    pub(crate) app: &'a mut App,
    pub(crate) scale_factor: f32,
}

/// Conversion into Taffy available-space values with the correct scale applied.
///
/// Note: different input types carry different semantics (see impl docs).
pub(crate) trait IntoTaffyAvailableSpace {
    fn into_taffy_available_space(self, scale_factor: f32) -> TaffySize<TaffyAvailableSpace>;
}

/// Preserves the semantic distinction between MinContent and MaxContent.
impl IntoTaffyAvailableSpace for Size<AvailableSpace> {
    fn into_taffy_available_space(self, scale_factor: f32) -> TaffySize<TaffyAvailableSpace> {
        let transform = |axis: AvailableSpace| match axis {
            AvailableSpace::Definite(pixels) => {
                TaffyAvailableSpace::Definite(pixels.0 * scale_factor)
            }
            AvailableSpace::MinContent => TaffyAvailableSpace::MinContent,
            AvailableSpace::MaxContent => TaffyAvailableSpace::MaxContent,
        };
        TaffySize {
            width: transform(self.width),
            height: transform(self.height),
        }
    }
}

/// Treats finite values as Definite and non-finite as MaxContent.
///
/// This cannot represent MinContent; use Size<AvailableSpace> when MinContent is required.
impl IntoTaffyAvailableSpace for TaffySize<f32> {
    fn into_taffy_available_space(self, scale_factor: f32) -> TaffySize<TaffyAvailableSpace> {
        let transform = |axis: f32| {
            if axis.is_finite() {
                TaffyAvailableSpace::Definite(axis * scale_factor)
            } else {
                TaffyAvailableSpace::MaxContent
            }
        };
        TaffySize {
            width: transform(self.width),
            height: transform(self.height),
        }
    }
}

impl<'a> LayoutContext<'a> {
    pub(crate) fn text_system(&self) -> Arc<crate::WindowTextSystem> {
        self.window.text_system().clone()
    }

    pub(crate) fn get_rem_size(&self) -> Pixels {
        self.window.rem_size()
    }

    pub(crate) fn get_scale_factor(&self) -> f32 {
        self.scale_factor
    }

    pub(crate) fn get_style(&self, node_id: NodeId) -> taffy::Style {
        self.engine.tree.nodes[node_id_to_default_key(node_id)]
            .style
            .clone()
    }

    pub(crate) fn to_taffy_available_space<S: IntoTaffyAvailableSpace>(
        &self,
        space: S,
    ) -> TaffySize<TaffyAvailableSpace> {
        space.into_taffy_available_space(self.scale_factor)
    }

    pub(crate) fn from_taffy_available_space(
        &self,
        space: taffy::geometry::Size<TaffyAvailableSpace>,
    ) -> Size<AvailableSpace> {
        let inverse = |axis: TaffyAvailableSpace| match axis {
            TaffyAvailableSpace::Definite(value) => {
                AvailableSpace::Definite(Pixels(value / self.scale_factor))
            }
            TaffyAvailableSpace::MinContent => AvailableSpace::MinContent,
            TaffyAvailableSpace::MaxContent => AvailableSpace::MaxContent,
        };
        size(inverse(space.width), inverse(space.height))
    }

    // Standard recursive layout dispatch when no measure context is present
    fn compute_child_layout_internal(
        &mut self,
        node_id: NodeId,
        inputs: LayoutInput,
    ) -> LayoutOutput {
        let style = self.engine.tree.nodes[node_id_to_default_key(node_id)]
            .style
            .clone();
        let has_context = self.engine.tree.nodes[node_id_to_default_key(node_id)].has_context;
        let has_children = !self.engine.tree.children[node_id_to_default_key(node_id)].is_empty();

        if has_context {
            self.compute_measured_leaf_layout(node_id, inputs)
        } else if !has_children {
            compute_leaf_layout(
                inputs,
                &style,
                |_, _| 0.0,
                |_, _| taffy::geometry::Size {
                    width: 0.0,
                    height: 0.0,
                },
            )
        } else {
            match style.display {
                taffy::Display::Block => compute_block_layout(self, node_id, inputs),
                taffy::Display::Flex => compute_flexbox_layout(self, node_id, inputs),
                taffy::Display::Grid => compute_grid_layout(self, node_id, inputs),
                taffy::Display::None => LayoutOutput::HIDDEN,
            }
        }
    }

    fn compute_measured_leaf_layout(
        &mut self,
        node_id: NodeId,
        inputs: LayoutInput,
    ) -> LayoutOutput {
        let scale_factor = self.scale_factor;
        let style = self.engine.tree.nodes[node_id_to_default_key(node_id)]
            .style
            .clone();

        compute_leaf_layout(
            inputs,
            &style,
            |_val, _basis| 0.0,
            |known, avail| {
                let known_dimensions = Size {
                    width: known.width.map(|w| Pixels(w / scale_factor)),
                    height: known.height.map(|h| Pixels(h / scale_factor)),
                };
                let available_space = self.from_taffy_available_space(avail);

                let mut context = self
                    .engine
                    .tree
                    .node_context_data
                    .get_mut(node_id_to_default_key(node_id))
                    .expect("Measured node should always have context");

                let measured = if let GpuiNodeContext::Measure(measure) = &mut context {
                    (measure)(known_dimensions, available_space, self.window, self.app)
                } else {
                    panic!("Measured node should have a Measure context")
                };

                taffy::geometry::Size {
                    width: measured.width.0 * scale_factor,
                    height: measured.height.0 * scale_factor,
                }
            },
        )
    }
}

// ============================================================================
// Taffy Trait Implementations
// ============================================================================

impl<'a> LayoutPartialTree for LayoutContext<'a> {
    type CoreContainerStyle<'b>
        = &'b taffy::Style
    where
        Self: 'b;
    type CustomIdent = String;

    fn get_core_container_style(&self, node_id: NodeId) -> Self::CoreContainerStyle<'_> {
        &self.engine.tree.nodes[node_id_to_default_key(node_id)].style
    }

    fn set_unrounded_layout(&mut self, node_id: NodeId, layout: &Layout) {
        self.engine.tree.nodes[node_id_to_default_key(node_id)].unrounded_layout = *layout;
    }

    fn compute_child_layout(&mut self, node_id: NodeId, inputs: LayoutInput) -> LayoutOutput {
        compute_cached_layout(self, node_id, inputs, |tree, node_id, inputs| {
            // 1. Take the measure context from the node (ownership transfer)
            let context = tree
                .engine
                .tree
                .node_context_data
                .get_mut(node_id_to_default_key(node_id));
            let mut measure_ctx = if let Some(GpuiNodeContext::MeasureContext(_)) = context {
                let GpuiNodeContext::MeasureContext(measure_ctx) = std::mem::replace(
                    context.unwrap(),
                    GpuiNodeContext::Measure(StackSafe::new(Box::new(|_, _, _, _| {
                        Size::default()
                    }))),
                ) else {
                    unreachable!()
                };
                Some(measure_ctx)
            } else {
                None
            };

            let output = if let Some(mut s) = measure_ctx.take() {
                // 2. Delegate to the context's layout logic
                let style = tree.engine.tree.nodes[node_id_to_default_key(node_id)]
                    .style
                    .clone();
                let output = s.layout(&style, inputs, tree);
                // 3. Restore the context for later calls
                tree.engine.tree.node_context_data.insert(
                    node_id_to_default_key(node_id),
                    GpuiNodeContext::MeasureContext(s),
                );
                output
            } else {
                // 4. Standard Fallback
                tree.compute_child_layout_internal(node_id, inputs)
            };

            output
        })
    }
}

// Boilerplate trait implementations required for Taffy algorithms
impl<'a> TraversePartialTree for LayoutContext<'a> {
    type ChildIter<'b>
        = std::iter::Cloned<std::slice::Iter<'b, NodeId>>
    where
        Self: 'b;
    fn child_ids(&self, id: NodeId) -> Self::ChildIter<'_> {
        let key = node_id_to_default_key(id);
        self.engine.tree.children[key].iter().cloned()
    }

    fn child_count(&self, id: NodeId) -> usize {
        let key = node_id_to_default_key(id);
        self.engine.tree.children[key].len()
    }
    fn get_child_id(&self, id: NodeId, index: usize) -> NodeId {
        self.engine.tree.children[node_id_to_default_key(id)][index]
    }
}
impl<'a> TraverseTree for LayoutContext<'a> {}
impl<'a> RoundTree for LayoutContext<'a> {
    fn get_unrounded_layout(&self, id: NodeId) -> Layout {
        self.engine.tree.nodes[node_id_to_default_key(id)].unrounded_layout
    }
    fn set_final_layout(&mut self, id: NodeId, l: &Layout) {
        let node = &mut self.engine.tree.nodes[node_id_to_default_key(id)];
        node.final_layout = *l;
        node.layout_generation = self.engine.layout_generation;
    }
}
impl<'a> CacheTree for LayoutContext<'a> {
    fn cache_get(
        &self,
        id: NodeId,
        k: taffy::geometry::Size<Option<f32>>,
        a: taffy::geometry::Size<TaffyAvailableSpace>,
        m: RunMode,
    ) -> Option<LayoutOutput> {
        self.engine.tree.nodes[node_id_to_default_key(id)]
            .cache
            .get(k, a, m)
    }

    fn cache_store(
        &mut self,
        id: NodeId,
        k: taffy::geometry::Size<Option<f32>>,
        a: taffy::geometry::Size<TaffyAvailableSpace>,
        m: RunMode,
        o: LayoutOutput,
    ) {
        self.engine.tree.nodes[node_id_to_default_key(id)]
            .cache
            .store(k, a, m, o);
    }

    fn cache_clear(&mut self, id: NodeId) {
        self.engine.tree.nodes[node_id_to_default_key(id)]
            .cache
            .clear();
    }
}
impl<'a> LayoutBlockContainer for LayoutContext<'a> {
    type BlockContainerStyle<'b>
        = &'b taffy::Style
    where
        Self: 'b;
    type BlockItemStyle<'b>
        = &'b taffy::Style
    where
        Self: 'b;
    fn get_block_container_style(&self, id: NodeId) -> Self::BlockContainerStyle<'_> {
        &self.engine.tree.nodes[node_id_to_default_key(id)].style
    }
    fn get_block_child_style(&self, id: NodeId) -> Self::BlockItemStyle<'_> {
        &self.engine.tree.nodes[node_id_to_default_key(id)].style
    }
}
impl<'a> LayoutFlexboxContainer for LayoutContext<'a> {
    type FlexboxContainerStyle<'b>
        = &'b taffy::Style
    where
        Self: 'b;
    type FlexboxItemStyle<'b>
        = &'b taffy::Style
    where
        Self: 'b;
    fn get_flexbox_container_style(&self, id: NodeId) -> Self::FlexboxContainerStyle<'_> {
        &self.engine.tree.nodes[node_id_to_default_key(id)].style
    }
    fn get_flexbox_child_style(&self, id: NodeId) -> Self::FlexboxItemStyle<'_> {
        &self.engine.tree.nodes[node_id_to_default_key(id)].style
    }
}
impl<'a> LayoutGridContainer for LayoutContext<'a> {
    type GridContainerStyle<'b>
        = &'b taffy::Style
    where
        Self: 'b;
    type GridItemStyle<'b>
        = &'b taffy::Style
    where
        Self: 'b;
    fn get_grid_container_style(&self, id: NodeId) -> Self::GridContainerStyle<'_> {
        &self.engine.tree.nodes[node_id_to_default_key(id)].style
    }
    fn get_grid_child_style(&self, id: NodeId) -> Self::GridItemStyle<'_> {
        &self.engine.tree.nodes[node_id_to_default_key(id)].style
    }
}

impl NodeMeasureCtx for InlineMeasureContext {
    fn layout(
        &mut self,
        style: &taffy::Style,
        inputs: LayoutInput,
        context: &mut LayoutContext,
    ) -> LayoutOutput {
        let (output, layout) = compute_inline_layout_impl(self, style, inputs, context);
        if let Some(layout) = layout {
            *self.result.lock() = Some(layout);
        }
        output
    }
}
