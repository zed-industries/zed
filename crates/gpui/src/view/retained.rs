use super::{AnyWeakView, ViewElementCacheKey, ViewElementState, paint_view, prepaint_view};
use crate::{
    AnyElement, App, EntityId, GlobalElementId, PaintIndex, PrepaintStateIndex, ReusedSubtree,
    Window, window::ViewExecutionScope,
};
use collections::FxHashMap;
use std::ops::Range;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub(crate) struct ViewOccurrenceId {
    index: u32,
    generation: u32,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum SpliceFallback {
    UncachedView,
    DeferredDraw,
    Transaction,
    Autoscroll,
    Accessibility,
    Inspector,
    ExecutionScope,
    MissingView,
}

struct ViewRecord {
    generation: u32,
    parent: Option<ViewOccurrenceId>,
    last_drawn: u64,
    own_dirty: bool,
    descendant_dirty: bool,
    blocked: Option<SpliceFallback>,
}

struct RetainedView {
    entity: EntityId,
    global_id: GlobalElementId,
    view: Option<AnyWeakView>,
    state: Option<ViewElementState>,
    prepaint_scope: Option<ViewExecutionScope>,
    paint_scope: Option<ViewExecutionScope>,
    pending_paint: Option<Vec<SpliceChild>>,
    own_prepaint_entities: collections::FxHashSet<EntityId>,
    own_paint_entities: collections::FxHashSet<EntityId>,
}

struct SpliceChild {
    occurrence: ViewOccurrenceId,
    element: Option<AnyElement>,
    old_paint_range: Range<PaintIndex>,
    paint_scope: ViewExecutionScope,
}

#[derive(Default)]
pub(crate) struct ViewStore {
    records: Vec<ViewRecord>,
    cold: Vec<Option<Box<RetainedView>>>,
    free: Vec<u32>,
    by_occurrence: FxHashMap<(EntityId, GlobalElementId), ViewOccurrenceId>,
    by_entity: FxHashMap<EntityId, Vec<ViewOccurrenceId>>,
    by_dependency: FxHashMap<EntityId, Vec<ViewOccurrenceId>>,
    epoch: u64,
}

impl ViewStore {
    fn record(&self, id: ViewOccurrenceId) -> &ViewRecord {
        let record = &self.records[id.index as usize];
        debug_assert_eq!(record.generation, id.generation);
        record
    }

    fn record_mut(&mut self, id: ViewOccurrenceId) -> &mut ViewRecord {
        let record = &mut self.records[id.index as usize];
        debug_assert_eq!(record.generation, id.generation);
        record
    }

    fn retained(&self, id: ViewOccurrenceId) -> &RetainedView {
        self.cold[id.index as usize]
            .as_deref()
            .expect("live occurrence")
    }

    fn retained_mut(&mut self, id: ViewOccurrenceId) -> &mut RetainedView {
        self.cold[id.index as usize]
            .as_deref_mut()
            .expect("live occurrence")
    }

    fn insert(
        &mut self,
        entity: EntityId,
        global_id: &GlobalElementId,
        parent: Option<ViewOccurrenceId>,
        view: Option<&AnyWeakView>,
        fallback: Option<SpliceFallback>,
    ) -> ViewOccurrenceId {
        let key = (entity, global_id.clone());
        let id = if let Some(id) = self.by_occurrence.get(&key) {
            *id
        } else {
            let index = self.free.pop().unwrap_or_else(|| {
                let index = self.records.len() as u32;
                self.records.push(ViewRecord {
                    generation: 0,
                    parent: None,
                    last_drawn: 0,
                    own_dirty: false,
                    descendant_dirty: false,
                    blocked: None,
                });
                self.cold.push(None);
                index
            });
            let id = ViewOccurrenceId {
                index,
                generation: self.records[index as usize].generation,
            };
            self.cold[index as usize] = Some(Box::new(RetainedView {
                entity,
                global_id: global_id.clone(),
                view: view.cloned(),
                state: None,
                prepaint_scope: None,
                paint_scope: None,
                pending_paint: None,
                own_prepaint_entities: Default::default(),
                own_paint_entities: Default::default(),
            }));
            self.by_occurrence.insert(key, id);
            self.by_entity.entry(entity).or_default().push(id);
            id
        };
        let epoch = self.epoch;
        let record = self.record_mut(id);
        record.parent = parent;
        record.last_drawn = epoch;
        // An uncached occurrence must not become a relayout boundary merely because
        // another embedding of this entity is cached.
        if view.is_none() {
            record.blocked = Some(SpliceFallback::UncachedView);
        } else if fallback.is_some() {
            record.blocked = fallback;
        }
        id
    }

    pub(crate) fn begin_frame(&mut self) {
        self.epoch += 1;
    }

    pub(crate) fn finish_frame(&mut self) {
        for (index, record) in self.records.iter_mut().enumerate() {
            if self.cold[index].is_none() {
                continue;
            }
            if record.last_drawn != self.epoch {
                let retained = self.cold[index].take().expect("live occurrence");
                for entity in retained
                    .own_prepaint_entities
                    .union(&retained.own_paint_entities)
                {
                    if let Some(occurrences) = self.by_dependency.get_mut(entity) {
                        occurrences.retain(|id| id.index as usize != index);
                        if occurrences.is_empty() {
                            self.by_dependency.remove(entity);
                        }
                    }
                }
                self.by_occurrence
                    .remove(&(retained.entity, retained.global_id));
                if let Some(occurrences) = self.by_entity.get_mut(&retained.entity) {
                    occurrences.retain(|id| id.index as usize != index);
                    if occurrences.is_empty() {
                        self.by_entity.remove(&retained.entity);
                    }
                }
                record.generation = record.generation.wrapping_add(1);
                self.free.push(index as u32);
            } else {
                record.own_dirty = false;
                record.descendant_dirty = false;
            }
        }
    }

    pub(crate) fn mark_dirty(&mut self, entity: EntityId) {
        let mut occurrences = self.by_entity.get(&entity).cloned().unwrap_or_default();
        occurrences.extend(
            self.by_dependency
                .get(&entity)
                .into_iter()
                .flatten()
                .copied(),
        );
        for id in occurrences {
            self.record_mut(id).own_dirty = true;
            let mut parent = self.record(id).parent;
            let mut blocked = self.record(id).blocked;
            while let Some(id) = parent {
                let record = self.record_mut(id);
                record.descendant_dirty = true;
                blocked = blocked.or(record.blocked);
                record.blocked = blocked;
                parent = record.parent;
            }
        }
    }

    pub(crate) fn rollback_views(&mut self, views: &[ViewOccurrenceId]) {
        for id in views {
            self.record_mut(*id).last_drawn = 0;
        }
    }

    pub(super) fn set_own_access(
        &mut self,
        id: ViewOccurrenceId,
        own: collections::FxHashSet<EntityId>,
        all: collections::FxHashSet<EntityId>,
        paint: bool,
    ) {
        let retained = self.retained_mut(id);
        let old_dependencies = retained
            .own_prepaint_entities
            .union(&retained.own_paint_entities)
            .copied()
            .collect::<Vec<_>>();
        if paint {
            retained.own_paint_entities = own;
            retained
                .state
                .as_mut()
                .expect("cached state")
                .accessed_entities
                .extend(all);
        } else {
            retained.own_prepaint_entities = own;
            retained.own_paint_entities.clear();
            retained
                .state
                .as_mut()
                .expect("cached state")
                .accessed_entities = all;
        }
        let dependencies = retained
            .own_prepaint_entities
            .union(&retained.own_paint_entities)
            .copied()
            .collect::<Vec<_>>();
        for entity in old_dependencies {
            if let Some(occurrences) = self.by_dependency.get_mut(&entity) {
                occurrences.retain(|occurrence| *occurrence != id);
                if occurrences.is_empty() {
                    self.by_dependency.remove(&entity);
                }
            }
        }
        for entity in dependencies {
            self.by_dependency.entry(entity).or_default().push(id);
        }
    }

    pub(crate) fn block_splicing(&mut self, id: ViewOccurrenceId, reason: SpliceFallback) {
        let mut current = Some(id);
        while let Some(id) = current {
            let record = self.record_mut(id);
            record.blocked = Some(reason);
            current = record.parent;
        }
    }

    pub(crate) fn is_own_dirty(&self, id: ViewOccurrenceId) -> bool {
        self.record(id).own_dirty
    }

    pub(crate) fn is_descendant_dirty(&self, id: ViewOccurrenceId) -> bool {
        self.record(id).descendant_dirty
    }

    pub(super) fn state(&self, id: ViewOccurrenceId) -> Option<&ViewElementState> {
        self.retained(id).state.as_ref()
    }

    pub(super) fn set_state(
        &mut self,
        id: ViewOccurrenceId,
        state: ViewElementState,
        scope: ViewExecutionScope,
    ) {
        let retained = self.retained_mut(id);
        retained.state = Some(state);
        retained.prepaint_scope = Some(scope);
        retained.pending_paint = None;
    }

    pub(super) fn reused_view_prepaint(
        &mut self,
        id: ViewOccurrenceId,
        range: Range<PrepaintStateIndex>,
        scope: ViewExecutionScope,
    ) {
        let retained = self.retained_mut(id);
        retained
            .state
            .as_mut()
            .expect("cached state")
            .prepaint_range = range;
        retained.prepaint_scope = Some(scope);
    }

    pub(super) fn painted_view(
        &mut self,
        id: ViewOccurrenceId,
        range: Range<PaintIndex>,
        scope: ViewExecutionScope,
    ) {
        let retained = self.retained_mut(id);
        retained.state.as_mut().expect("cached state").paint_range = range;
        retained.paint_scope = Some(scope);
    }

    pub(crate) fn reuse_prepaint(
        &mut self,
        id: ViewOccurrenceId,
        old: &PrepaintStateIndex,
        new: &PrepaintStateIndex,
        dispatch: &ReusedSubtree,
    ) {
        self.record_mut(id).last_drawn = self.epoch;
        let retained = self.retained_mut(id);
        if let Some(state) = retained.state.as_mut() {
            state.prepaint_range = state.prepaint_range.start.rebase(old, new)
                ..state.prepaint_range.end.rebase(old, new);
        }
        if let Some(scope) = retained.prepaint_scope.as_mut() {
            scope.dispatch_node = dispatch.refresh_node_id(scope.dispatch_node);
        }
    }

    pub(crate) fn reuse_paint(
        &mut self,
        id: ViewOccurrenceId,
        old: &PaintIndex,
        new: &PaintIndex,
        dispatch: &[ReusedSubtree],
    ) {
        let retained = self.retained_mut(id);
        if let Some(state) = retained.state.as_mut() {
            state.paint_range =
                state.paint_range.start.rebase(old, new)..state.paint_range.end.rebase(old, new);
        }
        if let Some(scope) = retained.paint_scope.as_mut() {
            scope.dispatch_node = dispatch
                .iter()
                .rev()
                .find_map(|range| range.remap_node_id(scope.dispatch_node))
                .expect("paint scope was replayed during prepaint");
        }
    }
}

impl Window {
    pub(super) fn begin_view_occurrence(
        &mut self,
        entity: EntityId,
        global_id: Option<&GlobalElementId>,
        view: Option<&AnyWeakView>,
    ) -> Option<ViewOccurrenceId> {
        if view.is_none() && self.current_view_occurrence.is_none() {
            return None;
        }
        let id = self.view_store.insert(
            entity,
            global_id?,
            self.current_view_occurrence,
            view,
            self.retained_scope_fallback,
        );
        self.next_frame.prepaint_views.push(id);
        Some(id)
    }

    fn dirty_view_children(&self, id: ViewOccurrenceId) -> Vec<ViewOccurrenceId> {
        let state = self.view_store.state(id).expect("cached state");
        self.rendered_frame.prepaint_views
            [state.prepaint_range.start.views_index..state.prepaint_range.end.views_index]
            .iter()
            .copied()
            .filter(|child| {
                let record = self.view_store.record(*child);
                record.parent == Some(id) && (record.own_dirty || record.descendant_dirty)
            })
            .collect()
    }

    pub(super) fn prepare_view_splice(
        &mut self,
        id: ViewOccurrenceId,
    ) -> Result<(), SpliceFallback> {
        let result = self.validate_view_splice(id);
        #[cfg(any(test, feature = "bench-support"))]
        if let Err(reason) = result {
            self.view_reuse_stats.splice_fallbacks[reason as usize] += 1;
        }
        result
    }

    fn validate_view_splice(&self, id: ViewOccurrenceId) -> Result<(), SpliceFallback> {
        if self.a11y.is_active() {
            return Err(SpliceFallback::Accessibility);
        }
        #[cfg(any(feature = "inspector", debug_assertions))]
        if self.inspector_enabled() {
            return Err(SpliceFallback::Inspector);
        }
        if let Some(reason) = self.view_store.record(id).blocked {
            return Err(reason);
        }
        let retained = self.view_store.retained(id);
        let prepaint_scope = retained
            .prepaint_scope
            .as_ref()
            .ok_or(SpliceFallback::ExecutionScope)?;
        let paint_scope = retained
            .paint_scope
            .as_ref()
            .ok_or(SpliceFallback::ExecutionScope)?;
        if !self.view_scope_is_current(prepaint_scope) || !self.view_scope_is_current(paint_scope) {
            return Err(SpliceFallback::ExecutionScope);
        }
        if let Some(reason) = prepaint_scope.fallback.or(paint_scope.fallback) {
            return Err(reason);
        }
        if retained
            .view
            .as_ref()
            .and_then(AnyWeakView::upgrade)
            .is_none()
        {
            return Err(SpliceFallback::MissingView);
        }
        for child in self.dirty_view_children(id) {
            self.validate_view_splice(child)?;
        }
        Ok(())
    }

    pub(super) fn splice_view_prepaint(&mut self, id: ViewOccurrenceId, cx: &mut App) {
        let children = self.dirty_view_children(id);
        #[cfg(any(test, feature = "bench-support"))]
        {
            self.view_reuse_stats.splice_prepaint_calls += 1;
            self.view_reuse_stats.spliced_children += children.len();
        }
        let retained = self.view_store.retained(id);
        let ancestor_range = retained
            .state
            .as_ref()
            .expect("cached state")
            .prepaint_range
            .clone();
        let old_root = retained
            .prepaint_scope
            .as_ref()
            .expect("prepaint scope")
            .dispatch_node;
        let new_root = self
            .next_frame
            .dispatch_tree
            .active_node_id()
            .expect("view node");
        self.dispatch_replay
            .push(ReusedSubtree::node_mapping(old_root, new_root));
        let mut cursor = ancestor_range.start;
        let mut pending_paint = Vec::with_capacity(children.len());
        for child in children {
            let retained = self.view_store.retained(child);
            let state = retained.state.as_ref().expect("cached child state");
            let mut child_range = state.prepaint_range.clone();
            child_range.start.views_index -= 1;
            child_range.start.accessed_element_states_index -= 1;
            let mut old_paint_range = state.paint_range.clone();
            old_paint_range.start.views_index -= 1;
            // The cached-state access is outside the content range in both phases.
            old_paint_range.start = old_paint_range.start.with_cached_state_access();
            let cache_key: ViewElementCacheKey = state.cache_key.clone();
            let global_id = retained.global_id.clone();
            let entity = retained.entity;
            let view = retained.view.clone().expect("retained child renderer");
            let mut scope = retained.prepaint_scope.clone().expect("prepaint scope");
            let paint_scope = retained.paint_scope.clone().expect("paint scope");
            self.reuse_prepaint(cursor..child_range.start);
            scope.dispatch_node = self
                .dispatch_replay
                .iter()
                .rev()
                .find_map(|range| range.remap_node_id(scope.dispatch_node))
                .expect("child root was replayed in prefix");
            let element = self.with_view_scope(scope, |window| {
                prepaint_view(
                    entity,
                    Some(&global_id),
                    cache_key.bounds,
                    &mut None,
                    Some(&view),
                    window,
                    cx,
                    &mut |window, cx| {
                        let view = view.upgrade().expect("validated retained renderer");
                        (view.render)(&view, window, cx)
                    },
                )
            });
            pending_paint.push(SpliceChild {
                occurrence: child,
                element,
                old_paint_range,
                paint_scope,
            });
            cursor = child_range.end;
        }
        self.reuse_prepaint(cursor..ancestor_range.end);
        self.view_store.retained_mut(id).pending_paint = Some(pending_paint);
    }

    pub(super) fn splice_view_paint(&mut self, id: ViewOccurrenceId, cx: &mut App) {
        let retained = self.view_store.retained_mut(id);
        let ancestor_range = retained
            .state
            .as_ref()
            .expect("cached state")
            .paint_range
            .clone();
        let Some(children) = retained.pending_paint.take() else {
            self.reuse_paint(ancestor_range);
            return;
        };
        #[cfg(any(test, feature = "bench-support"))]
        {
            self.view_reuse_stats.splice_paint_calls += 1;
        }
        let mut cursor = ancestor_range.start;
        for mut child in children {
            self.reuse_paint(cursor..child.old_paint_range.start);
            let retained = self.view_store.retained(child.occurrence);
            let global_id = retained.global_id.clone();
            let entity = retained.entity;
            child.paint_scope.dispatch_node = self
                .dispatch_replay
                .iter()
                .rev()
                .find_map(|range| range.remap_node_id(child.paint_scope.dispatch_node))
                .expect("child root was replayed during prepaint");
            self.with_view_scope(child.paint_scope, |window| {
                paint_view(
                    entity,
                    true,
                    Some(&global_id),
                    &mut child.element,
                    window,
                    cx,
                );
            });
            cursor = child.old_paint_range.end;
        }
        self.reuse_paint(cursor..ancestor_range.end);
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        AppContext, Context, Entity, IntoElement, ParentElement, Render, StyleRefinement, Styled,
        TestAppContext, Window, div, px,
    };
    use std::{cell::Cell, rc::Rc};

    struct Leaf {
        renders: Rc<Cell<usize>>,
        color: u32,
    }

    impl Render for Leaf {
        fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
            self.renders.set(self.renders.get() + 1);
            div().size(px(20.)).bg(crate::rgb(self.color))
        }
    }

    struct Branch {
        renders: Rc<Cell<usize>>,
        children: Vec<Entity<Leaf>>,
    }

    impl Render for Branch {
        fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
            self.renders.set(self.renders.get() + 1);
            div().flex().children(self.children.iter().map(|child| {
                child
                    .clone()
                    .cached(StyleRefinement::default().size(px(20.)))
            }))
        }
    }

    struct Outer {
        renders: Rc<Cell<usize>>,
        child: Entity<Branch>,
    }

    impl Render for Outer {
        fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
            self.renders.set(self.renders.get() + 1);
            div().child(
                self.child
                    .clone()
                    .cached(StyleRefinement::default().size(px(80.))),
            )
        }
    }

    struct Root(Entity<Outer>);

    impl Render for Root {
        fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
            self.0
                .clone()
                .cached(StyleRefinement::default().size(px(100.)))
        }
    }

    #[crate::test]
    fn splice_nested_cached_views(cx: &mut TestAppContext) {
        let leaf_renders = [Rc::new(Cell::new(0)), Rc::new(Cell::new(0))];
        let branch_renders = Rc::new(Cell::new(0));
        let outer_renders = Rc::new(Cell::new(0));
        let leaves = leaf_renders.each_ref().map(|renders| {
            cx.new(|_| Leaf {
                renders: renders.clone(),
                color: 0xff0000,
            })
        });
        let branch = cx.new(|_| Branch {
            renders: branch_renders.clone(),
            children: leaves.to_vec(),
        });
        let outer = cx.new(|_| Outer {
            renders: outer_renders.clone(),
            child: branch,
        });
        let window = cx.add_window(|_, _| Root(outer));
        cx.update_window(window.into(), |_, window, cx| window.draw(cx).clear(cx))
            .expect("initial draw");
        assert_eq!(branch_renders.get(), 1);
        assert_eq!(outer_renders.get(), 1);
        for _ in 0..2 {
            cx.update_window(window.into(), |_, window, cx| window.draw(cx).clear(cx))
                .expect("unchanged draw");
        }
        for iteration in 0..4 {
            leaves[iteration % 2].update(cx, |leaf, cx| {
                leaf.color ^= 0xffffff;
                cx.notify();
            });
            cx.update_window(window.into(), |_, window, cx| window.draw(cx).clear(cx))
                .expect("leaf draw");
            assert_eq!(
                branch_renders.get(),
                1,
                "branch should splice instead of render"
            );
            assert_eq!(
                outer_renders.get(),
                1,
                "outer should splice instead of render"
            );
        }
        assert_eq!(leaf_renders[0].get(), 3);
        assert_eq!(leaf_renders[1].get(), 3);
        for leaf in leaves {
            leaf.update(cx, |leaf, cx| {
                leaf.color ^= 0xffffff;
                cx.notify();
            });
        }
        cx.update_window(window.into(), |_, window, cx| window.draw(cx).clear(cx))
            .expect("two dirty siblings");
        assert_eq!(branch_renders.get(), 1);
        assert_eq!(outer_renders.get(), 1);
        assert_eq!(leaf_renders[0].get(), 4);
        assert_eq!(leaf_renders[1].get(), 4);
    }

    #[derive(Clone, Copy)]
    enum DependencyPhase {
        Render,
        Prepaint,
        Paint,
    }

    struct DependentParent {
        renders: Rc<Cell<usize>>,
        leaf: Entity<Leaf>,
        phase: DependencyPhase,
    }

    impl Render for DependentParent {
        fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
            self.renders.set(self.renders.get() + 1);
            let phase = self.phase;
            let color = if matches!(phase, DependencyPhase::Render) {
                self.leaf.read(cx).color
            } else {
                0xffffff
            };
            let prepaint_leaf = self.leaf.clone();
            let paint_leaf = self.leaf.clone();
            div()
                .bg(crate::rgb(color))
                .child(
                    self.leaf
                        .clone()
                        .cached(StyleRefinement::default().size(px(20.))),
                )
                .child(crate::canvas(
                    move |_, _, cx| {
                        if matches!(phase, DependencyPhase::Prepaint) {
                            std::hint::black_box(prepaint_leaf.read(cx).color);
                        }
                    },
                    move |_, _, _, cx| {
                        if matches!(phase, DependencyPhase::Paint) {
                            std::hint::black_box(paint_leaf.read(cx).color);
                        }
                    },
                ))
        }
    }

    struct DependentRoot(Entity<DependentParent>);

    impl Render for DependentRoot {
        fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
            self.0
                .clone()
                .cached(StyleRefinement::default().size(px(100.)))
        }
    }

    #[crate::test]
    fn cached_own_entity_dependencies(cx: &mut TestAppContext) {
        for phase in [
            DependencyPhase::Render,
            DependencyPhase::Prepaint,
            DependencyPhase::Paint,
        ] {
            let parent_renders = Rc::new(Cell::new(0));
            let leaf = cx.new(|_| Leaf {
                renders: Rc::new(Cell::new(0)),
                color: 0xff0000,
            });
            let parent = cx.new(|_| DependentParent {
                renders: parent_renders.clone(),
                leaf: leaf.clone(),
                phase,
            });
            let window = cx.add_window(|_, _| DependentRoot(parent));
            cx.update_window(window.into(), |_, window, cx| window.draw(cx).clear(cx))
                .expect("initial draw");
            assert_eq!(parent_renders.get(), 1);
            for _ in 0..2 {
                cx.update_window(window.into(), |_, window, cx| window.draw(cx).clear(cx))
                    .expect("unchanged draw");
            }
            leaf.update(cx, |leaf, cx| {
                leaf.color = 0x00ff00;
                cx.notify();
            });
            cx.update_window(window.into(), |_, window, cx| window.draw(cx).clear(cx))
                .expect("dependent parent draw");
            assert_eq!(
                parent_renders.get(),
                2,
                "a parent dependency requires its own render"
            );
        }
    }
}
