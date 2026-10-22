use super::{AnyWeakView, RetainedViewCacheKey, RetainedViewState, paint_view, prepaint_view};
use crate::element::InternedHandle;
use crate::{
    AnyElement, App, EntityId, GlobalElementId, LayoutId, PaintIndex, PrepaintStateIndex,
    ReusedSubtree, Window, window::ViewExecutionScope,
};
use collections::FxHashMap;
use std::any::TypeId;
use std::ops::Range;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub(crate) struct ViewOccurrenceId {
    index: u32,
    generation: u32,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum SpliceFallback {
    Blocked,
    Transaction,
}

struct ViewLayout {
    id: LayoutId,
    input: Option<taffy::LayoutInput>,
    output: Option<taffy::LayoutOutput>,
    scope: ViewExecutionScope,
}

struct ViewRecord {
    generation: u32,
    parent: Option<ViewOccurrenceId>,
    last_drawn: u64,
    own_dirty: bool,
    descendant_dirty: bool,
    painted: bool,
    blocked: bool,
    dirty_path_blocked: bool,
}

struct RetainedView {
    entity: EntityId,
    element_path: InternedHandle,
    view: Option<AnyWeakView>,
    state: Option<Box<RetainedViewState>>,
    prepaint_scope: Option<ViewExecutionScope>,
    paint_scope: Option<ViewExecutionScope>,
    pending_paint: Option<Vec<SpliceChild>>,
    own_prepaint_entities: collections::FxHashSet<EntityId>,
    own_paint_entities: collections::FxHashSet<EntityId>,
    own_layout_entities: collections::FxHashSet<EntityId>,
    own_globals: collections::FxHashSet<TypeId>,
    layout: Option<Box<ViewLayout>>,
    prepared: Option<(LayoutId, AnyElement)>,
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
    cold: Vec<Option<RetainedView>>,
    free: Vec<u32>,
    by_occurrence: FxHashMap<(EntityId, InternedHandle), ViewOccurrenceId>,
    by_entity: FxHashMap<EntityId, Vec<ViewOccurrenceId>>,
    by_dependency: FxHashMap<EntityId, Vec<ViewOccurrenceId>>,
    by_global: FxHashMap<TypeId, Vec<ViewOccurrenceId>>,
    access_pool: Vec<collections::FxHashSet<EntityId>>,
    epoch: u64,
}

impl ViewStore {
    pub(crate) fn take_access_set(&mut self) -> collections::FxHashSet<EntityId> {
        self.access_pool.pop().unwrap_or_default()
    }

    fn recycle_access_set(&mut self, mut entities: collections::FxHashSet<EntityId>) {
        if entities.capacity() > 0 {
            entities.clear();
            self.access_pool.push(entities);
        }
    }

    pub(crate) fn element_path(&self, id: ViewOccurrenceId) -> InternedHandle {
        self.retained(id).element_path
    }

    pub(crate) fn extend_accessed(
        &self,
        id: ViewOccurrenceId,
        entities: &mut collections::FxHashSet<EntityId>,
    ) {
        let retained = self.retained(id);
        entities.insert(retained.entity);
        entities.extend(retained.own_prepaint_entities.iter().copied());
        entities.extend(retained.own_paint_entities.iter().copied());
        entities.extend(retained.own_layout_entities.iter().copied());
    }

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
            .as_ref()
            .expect("live occurrence")
    }

    fn retained_mut(&mut self, id: ViewOccurrenceId) -> &mut RetainedView {
        self.cold[id.index as usize]
            .as_mut()
            .expect("live occurrence")
    }

    fn insert(
        &mut self,
        entity: EntityId,
        element_path: InternedHandle,
        parent: Option<ViewOccurrenceId>,
        view: Option<&AnyWeakView>,
        fallback: Option<SpliceFallback>,
    ) -> ViewOccurrenceId {
        let key = (entity, element_path);
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
                    painted: false,
                    blocked: false,
                    dirty_path_blocked: false,
                });
                self.cold.push(None);
                index
            });
            let id = ViewOccurrenceId {
                index,
                generation: self.records[index as usize].generation,
            };
            self.records[index as usize].own_dirty = false;
            self.records[index as usize].descendant_dirty = false;
            self.records[index as usize].painted = false;
            self.records[index as usize].blocked = false;
            self.records[index as usize].dirty_path_blocked = false;
            self.cold[index as usize] = Some(RetainedView {
                entity,
                element_path,
                view: view.cloned(),
                state: None,
                prepaint_scope: None,
                paint_scope: None,
                pending_paint: None,
                own_prepaint_entities: Default::default(),
                own_paint_entities: Default::default(),
                own_layout_entities: Default::default(),
                own_globals: Default::default(),
                layout: None,
                prepared: None,
            });
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
            record.blocked = true;
        } else if fallback.is_some() {
            record.blocked = fallback.is_some();
        }
        if view.is_none() {
            self.invalidate_state(id);
            self.retained_mut(id).view = None;
        } else if self.retained(id).view.is_none() {
            self.retained_mut(id).view = view.cloned();
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
                for global in &retained.own_globals {
                    if let Some(occurrences) = self.by_global.get_mut(global) {
                        occurrences.retain(|id| id.index as usize != index);
                        if occurrences.is_empty() {
                            self.by_global.remove(global);
                        }
                    }
                }
                for entity in retained
                    .own_prepaint_entities
                    .union(&retained.own_paint_entities)
                    .chain(retained.own_layout_entities.iter())
                {
                    if let Some(occurrences) = self.by_dependency.get_mut(entity) {
                        occurrences.retain(|id| id.index as usize != index);
                        if occurrences.is_empty() {
                            self.by_dependency.remove(entity);
                        }
                    }
                }
                self.by_occurrence
                    .remove(&(retained.entity, retained.element_path));
                if let Some(occurrences) = self.by_entity.get_mut(&retained.entity) {
                    occurrences.retain(|id| id.index as usize != index);
                    if occurrences.is_empty() {
                        self.by_entity.remove(&retained.entity);
                    }
                }
                if let Some(generation) = record.generation.checked_add(1) {
                    record.generation = generation;
                    self.free.push(index as u32);
                }
            } else {
                record.own_dirty = false;
                record.descendant_dirty = false;
                record.dirty_path_blocked = false;
                let retained = self.cold[index].as_mut().expect("live occurrence");
                retained.prepared = None;
                // Hidden ancestors still prepaint children, but their previous
                // paint ranges no longer name records in the rendered frame.
                if !record.painted {
                    retained.paint_scope = None;
                }
                record.painted = false;
            }
        }
    }

    pub(crate) fn mark_dirty(&mut self, entity: EntityId) -> bool {
        let mut occurrences = self.by_entity.get(&entity).cloned().unwrap_or_default();
        occurrences.extend(
            self.by_dependency
                .get(&entity)
                .into_iter()
                .flatten()
                .copied(),
        );
        let retained = !occurrences.is_empty();
        for id in occurrences {
            self.record_mut(id).own_dirty = true;
            let mut parent = self.record(id).parent;
            let mut blocked = self.record(id).blocked;
            while let Some(id) = parent {
                let record = self.record_mut(id);
                record.descendant_dirty = true;
                blocked |= record.blocked;
                record.dirty_path_blocked |= blocked;
                parent = record.parent;
            }
        }
        retained
    }

    pub(crate) fn dirty_global_readers(&mut self, global: TypeId) -> bool {
        let occurrences = self.by_global.get(&global).cloned().unwrap_or_default();
        for id in &occurrences {
            self.record_mut(*id).own_dirty = true;
            let mut parent = self.record(*id).parent;
            let mut blocked = self.record(*id).blocked;
            while let Some(id) = parent {
                let record = self.record_mut(id);
                record.descendant_dirty = true;
                blocked |= record.blocked;
                record.dirty_path_blocked |= blocked;
                parent = record.parent;
            }
        }
        !occurrences.is_empty()
    }

    pub(crate) fn append_global_access(
        &mut self,
        id: ViewOccurrenceId,
        globals: collections::FxHashSet<TypeId>,
    ) {
        for global in globals {
            if self.retained_mut(id).own_globals.insert(global) {
                self.by_global.entry(global).or_default().push(id);
            }
        }
    }

    fn clear_global_access(&mut self, id: ViewOccurrenceId) {
        for global in std::mem::take(&mut self.retained_mut(id).own_globals) {
            if let Some(occurrences) = self.by_global.get_mut(&global) {
                occurrences.retain(|occurrence| *occurrence != id);
                if occurrences.is_empty() {
                    self.by_global.remove(&global);
                }
            }
        }
    }

    pub(crate) fn mark_unretained_descendant(&mut self, entity: EntityId) {
        for id in self.by_entity.get(&entity).cloned().unwrap_or_default() {
            let mut current = Some(id);
            while let Some(id) = current {
                let record = self.record_mut(id);
                record.descendant_dirty = true;
                record.dirty_path_blocked = true;
                current = record.parent;
            }
        }
    }

    pub(crate) fn rollback_views(&mut self, views: &[ViewOccurrenceId]) {
        for id in views {
            self.record_mut(*id).last_drawn = 0;
        }
    }

    pub(super) fn invalidate_state(&mut self, id: ViewOccurrenceId) {
        self.clear_global_access(id);
        let retained = self.retained_mut(id);
        let dependencies = retained
            .own_prepaint_entities
            .union(&retained.own_paint_entities)
            .chain(retained.own_layout_entities.iter())
            .copied()
            .collect::<Vec<_>>();
        retained.state = None;
        retained.prepaint_scope = None;
        retained.paint_scope = None;
        retained.pending_paint = None;
        retained.own_prepaint_entities.clear();
        retained.own_paint_entities.clear();
        retained.own_layout_entities.clear();
        retained.layout = None;
        retained.prepared = None;
        for entity in dependencies {
            if let Some(occurrences) = self.by_dependency.get_mut(&entity) {
                occurrences.retain(|occurrence| *occurrence != id);
                if occurrences.is_empty() {
                    self.by_dependency.remove(&entity);
                }
            }
        }
    }

    pub(crate) fn append_own_access(
        &mut self,
        mut parent: Option<ViewOccurrenceId>,
        mut own: collections::FxHashSet<EntityId>,
        paint: bool,
    ) {
        while let Some(id) = parent {
            if self.state(id).is_none() {
                parent = self.record(id).parent;
                continue;
            }
            let retained = self.retained_mut(id);
            own.remove(&retained.entity);
            let new_dependencies = own
                .iter()
                .filter(|entity| {
                    !retained.own_prepaint_entities.contains(entity)
                        && !retained.own_paint_entities.contains(entity)
                })
                .copied()
                .collect::<Vec<_>>();
            if paint {
                retained.own_paint_entities.extend(own);
            } else {
                retained.own_prepaint_entities.extend(own);
            }
            for entity in new_dependencies {
                self.by_dependency.entry(entity).or_default().push(id);
            }
            break;
        }
    }

    pub(super) fn set_own_access(
        &mut self,
        id: ViewOccurrenceId,
        mut own: collections::FxHashSet<EntityId>,
        paint: bool,
    ) {
        own.remove(&self.retained(id).entity);
        if own.is_empty() {
            self.recycle_access_set(own);
            own = Default::default();
        }
        let retained = self.retained_mut(id);
        if own.is_empty()
            && retained.own_prepaint_entities.is_empty()
            && retained.own_paint_entities.is_empty()
            && retained.own_layout_entities.is_empty()
        {
            return;
        }
        let old_dependencies = retained
            .own_prepaint_entities
            .union(&retained.own_paint_entities)
            .chain(retained.own_layout_entities.iter())
            .copied()
            .collect::<Vec<_>>();
        if paint {
            retained.own_paint_entities = own;
        } else {
            retained.own_prepaint_entities = own;
            retained.own_paint_entities.clear();
        }
        let dependencies = retained
            .own_prepaint_entities
            .union(&retained.own_paint_entities)
            .chain(retained.own_layout_entities.iter())
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

    fn set_layout_access(
        &mut self,
        id: ViewOccurrenceId,
        mut own: collections::FxHashSet<EntityId>,
    ) {
        own.remove(&self.retained(id).entity);
        if own.is_empty() {
            self.recycle_access_set(own);
            own = Default::default();
        }
        let retained = self.retained_mut(id);
        let previous = std::mem::replace(&mut retained.own_layout_entities, own);
        for entity in previous {
            if let Some(occurrences) = self.by_dependency.get_mut(&entity) {
                occurrences.retain(|occurrence| *occurrence != id);
                if occurrences.is_empty() {
                    self.by_dependency.remove(&entity);
                }
            }
        }
        self.set_own_access(id, Default::default(), false);
    }

    fn append_layout_access(
        &mut self,
        id: ViewOccurrenceId,
        mut own: collections::FxHashSet<EntityId>,
    ) {
        let retained = self.retained_mut(id);
        own.remove(&retained.entity);
        let mut dependencies = Vec::new();
        for entity in own {
            let new = !retained.own_prepaint_entities.contains(&entity)
                && !retained.own_paint_entities.contains(&entity);
            if retained.own_layout_entities.insert(entity) && new {
                dependencies.push(entity);
            }
        }
        for entity in dependencies {
            self.by_dependency.entry(entity).or_default().push(id);
        }
    }

    pub(crate) fn block_splicing(&mut self, id: ViewOccurrenceId) {
        let mut current = Some(id);
        while let Some(id) = current {
            let record = self.record_mut(id);
            record.blocked = true;
            current = record.parent;
        }
    }

    pub(crate) fn is_own_dirty(&self, id: ViewOccurrenceId) -> bool {
        self.record(id).own_dirty
    }

    #[cfg(any(test, feature = "bench-support"))]
    pub(crate) fn is_entity(&self, id: ViewOccurrenceId, entity: EntityId) -> bool {
        self.retained(id).entity == entity
    }

    #[cfg(any(test, feature = "bench-support"))]
    pub(crate) fn has_own_dirty_ancestor(&self, id: ViewOccurrenceId) -> bool {
        let mut parent = self.record(id).parent;
        while let Some(id) = parent {
            if self.record(id).own_dirty {
                return true;
            }
            parent = self.record(id).parent;
        }
        false
    }

    pub(super) fn begin_reconstruction(
        &mut self,
        id: ViewOccurrenceId,
        scope: &ViewExecutionScope,
    ) {
        if self.retained(id).layout.is_none() {
            self.clear_global_access(id);
            self.set_layout_access(id, Default::default());
        }
        self.record_mut(id).blocked = scope.fallback.is_some();
    }

    pub(crate) fn is_descendant_dirty(&self, id: ViewOccurrenceId) -> bool {
        self.record(id).descendant_dirty
    }

    pub(super) fn state(&self, id: ViewOccurrenceId) -> Option<&RetainedViewState> {
        self.retained(id).state.as_deref()
    }

    pub(super) fn has_paint(&self, id: ViewOccurrenceId) -> bool {
        self.retained(id).paint_scope.is_some()
    }

    pub(super) fn set_state(
        &mut self,
        id: ViewOccurrenceId,
        state: RetainedViewState,
        scope: ViewExecutionScope,
    ) {
        let retained = self.retained_mut(id);
        if let Some(previous) = retained.state.as_mut() {
            **previous = state;
        } else {
            retained.state = Some(Box::new(state));
        }
        retained.prepaint_scope = Some(scope);
        retained.paint_scope = None;
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
        self.record_mut(id).painted = true;
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
            scope.dispatch_node = scope
                .dispatch_node
                .map(|node| dispatch.refresh_node_id(node));
        }
    }

    pub(crate) fn reuse_paint(
        &mut self,
        id: ViewOccurrenceId,
        old: &PaintIndex,
        new: &PaintIndex,
        dispatch: &[ReusedSubtree],
    ) {
        self.record_mut(id).painted = true;
        let retained = self.retained_mut(id);
        if let Some(state) = retained.state.as_mut() {
            state.paint_range =
                state.paint_range.start.rebase(old, new)..state.paint_range.end.rebase(old, new);
        }
        if let Some(scope) = retained.paint_scope.as_mut() {
            scope.dispatch_node = Some(
                dispatch
                    .iter()
                    .rev()
                    .find_map(|range| {
                        scope
                            .dispatch_node
                            .and_then(|node| range.remap_node_id(node))
                    })
                    .expect("paint scope was replayed during prepaint"),
            );
        }
    }
}

impl Window {
    pub(crate) fn with_view_measure_access<R>(
        &mut self,
        occurrence: ViewOccurrenceId,
        cx: &mut App,
        measure: impl FnOnce(&mut Window, &mut App) -> R,
    ) -> R {
        let entity = self.view_store.retained(occurrence).entity;
        let parent = self.current_view_occurrence.replace(occurrence);
        let previous = self.begin_view_entity_access(cx);
        let result = self.with_rendered_view(entity, |window| measure(window, cx));
        let (own, _) = self.end_cached_entity_access(previous, cx);
        self.view_store.append_layout_access(occurrence, own);
        self.current_view_occurrence = parent;
        result
    }

    pub(crate) fn invalidate_retained_global(&mut self, global: TypeId) {
        if self.view_splicing && self.view_store.dirty_global_readers(global) {
            self.invalidator.set_dirty(true);
        }
    }

    pub(super) fn request_implicit_view_layout(
        &mut self,
        entity: EntityId,
        _global_id: &GlobalElementId,
        view: &AnyWeakView,
        allow_reuse: bool,
        cx: &mut App,
        render: &mut dyn FnMut(&mut Window, &mut App) -> AnyElement,
    ) -> (LayoutId, Option<AnyElement>) {
        let occurrence = self.view_store.insert(
            entity,
            self.element_id_stack.handle(),
            self.current_view_occurrence,
            Some(view),
            self.retained_scope_fallback,
        );
        if let Some((layout, element)) = self.view_store.retained_mut(occurrence).prepared.take() {
            self.layout_engine
                .as_mut()
                .expect("layout engine")
                .retain_subtree(layout);
            return (layout, Some(element));
        }
        let scope = self.capture_view_scope();
        let previous_parent = self.current_view_occurrence.replace(occurrence);
        #[cfg(any(test, feature = "bench-support"))]
        let force = self.force_view_reconstruction;
        #[cfg(not(any(test, feature = "bench-support")))]
        let force = false;
        let can_retain = allow_reuse
            && !self.implicit_ancestor_dirty
            && self.view_store.retained(occurrence).layout.is_some()
            && self.view_store.state(occurrence).is_some()
            && !self.view_store.is_own_dirty(occurrence)
            && !self.refreshing
            && !force
            && self.retained_scope_fallback.is_none()
            && self
                .view_store
                .retained(occurrence)
                .layout
                .as_ref()
                .is_some_and(|layout| self.view_layout_scope_matches(&layout.scope, &scope))
            && !self.a11y.is_active();
        if can_retain {
            let result = if self.view_store.is_descendant_dirty(occurrence) {
                self.prepare_implicit_layout(occurrence, cx)
            } else {
                Some(())
            };
            if result.is_some() {
                let layout = self
                    .view_store
                    .retained(occurrence)
                    .layout
                    .as_ref()
                    .expect("retained layout")
                    .id;
                self.layout_engine
                    .as_mut()
                    .expect("layout engine")
                    .retain_subtree(layout);
                self.current_view_occurrence = previous_parent;
                return (layout, None);
            }
        }
        #[cfg(any(test, feature = "bench-support"))]
        self.record_view_render(
            entity,
            false,
            true,
            self.view_store.state(occurrence).is_none(),
        );
        self.view_store.clear_global_access(occurrence);
        let previous_access = self.begin_view_entity_access(cx);
        let ancestor_dirty = self.implicit_ancestor_dirty;
        self.implicit_ancestor_dirty |= self.view_store.is_own_dirty(occurrence);
        let mut element = self.with_rendered_view(entity, |window| render(window, cx));
        let layout = self.with_rendered_view(entity, |window| element.request_layout(window, cx));
        self.implicit_ancestor_dirty = ancestor_dirty;
        let (own, _) = self.end_cached_entity_access(previous_access, cx);
        self.view_store.set_layout_access(occurrence, own);
        self.layout_engine
            .as_mut()
            .expect("layout engine")
            .register_retained_boundary(layout);
        let view_layout = ViewLayout {
            id: layout,
            input: None,
            output: None,
            scope,
        };
        let retained = self.view_store.retained_mut(occurrence);
        if let Some(previous) = retained.layout.as_mut() {
            **previous = view_layout;
        } else {
            retained.layout = Some(Box::new(view_layout));
        }
        self.current_view_occurrence = previous_parent;
        (layout, Some(element))
    }

    fn prepare_implicit_layout(&mut self, id: ViewOccurrenceId, cx: &mut App) -> Option<()> {
        if !self.can_splice_view(id) {
            return None;
        }
        for child in self.dirty_view_children(id) {
            if self.view_store.retained(child).layout.is_none() {
                continue;
            }
            if !self.view_store.is_own_dirty(child) {
                self.prepare_implicit_layout(child, cx)?;
                continue;
            }
            let retained = self.view_store.retained(child);
            let layout = retained.layout.as_ref().expect("implicit layout");
            let old_id = layout.id;
            let old_style = self
                .layout_engine
                .as_ref()
                .expect("layout engine")
                .retained_style(old_id);
            let input = layout.input?;
            let old_output = layout.output.clone();
            let scope = layout.scope.clone();
            let old_size = retained
                .state
                .as_ref()
                .expect("retained state")
                .cache_key
                .bounds
                .size;
            let view = retained.view.clone()?;
            let global_id = self.element_id_interner.global_id(retained.element_path);
            let entity = retained.entity;
            let changes = self
                .layout_engine
                .as_ref()
                .expect("layout engine")
                .layout_changes();
            let (layout, element) = self.with_view_layout_scope(scope, |window| {
                window.request_implicit_view_layout(
                    entity,
                    &global_id,
                    &view,
                    true,
                    cx,
                    &mut |window, cx| {
                        let view = view.upgrade().expect("validated retained renderer");
                        (view.render)(&view, window, cx)
                    },
                )
            });
            self.view_store.retained_mut(child).prepared = element.map(|element| (layout, element));
            let engine = self.layout_engine.as_ref().expect("layout engine");
            if engine.retained_style(layout) != old_style {
                return None;
            }
            let mut engine = self.layout_engine.take().expect("layout engine");
            engine.compute_retained_layout(layout, input, self, cx);
            let size = engine.layout_bounds(layout, self.scale_factor()).size;
            let output = engine.retained_output(layout);
            self.layout_engine = Some(engine);
            if layout != old_id || size != old_size || output != old_output {
                return None;
            }
            self.layout_engine
                .as_mut()
                .expect("layout engine")
                .accept_internal_layout_changes(changes);
            self.view_store
                .retained_mut(child)
                .layout
                .as_mut()
                .expect("implicit layout")
                .input = Some(input);
        }
        Some(())
    }

    pub(super) fn implicit_prepaint_layout(&mut self, id: ViewOccurrenceId) -> bool {
        let geometry_unchanged = self
            .view_store
            .retained(id)
            .layout
            .as_ref()
            .is_none_or(|layout| layout.input.is_none())
            || self.implicit_geometry_unchanged(id);
        let Some(layout) = self.view_store.retained_mut(id).layout.as_mut() else {
            return true;
        };
        let input = self
            .layout_engine
            .as_ref()
            .expect("layout engine")
            .retained_input(layout.id);
        let unchanged = layout.input.is_none() || (layout.input == input && geometry_unchanged);
        layout.input = input;
        layout.output = self
            .layout_engine
            .as_ref()
            .expect("layout engine")
            .retained_output(layout.id);
        unchanged
    }

    fn implicit_geometry_unchanged(&self, id: ViewOccurrenceId) -> bool {
        let Some(layout) = self.view_store.retained(id).layout.as_ref() else {
            return true;
        };
        let boundaries = self
            .view_store
            .records
            .iter()
            .enumerate()
            .filter(|(_, record)| record.parent == Some(id))
            .filter_map(|(index, _)| {
                self.view_store.cold[index]
                    .as_ref()?
                    .layout
                    .as_ref()
                    .map(|layout| layout.id)
            })
            .collect::<Vec<_>>();
        self.layout_engine
            .as_ref()
            .expect("layout engine")
            .retained_geometry_unchanged(layout.id, &boundaries)
    }

    pub(super) fn is_implicit_view(&self, id: ViewOccurrenceId) -> bool {
        self.view_store.retained(id).layout.is_some()
    }

    pub(super) fn reconstruct_implicit_view(
        &mut self,
        id: ViewOccurrenceId,
        cx: &mut App,
        render: &mut dyn FnMut(&mut Window, &mut App) -> AnyElement,
    ) -> (AnyElement, LayoutId) {
        let retained = self.view_store.retained(id);
        let layout = retained.layout.as_ref().expect("implicit layout");
        let input = layout.input.expect("computed layout input");
        let layout_id = layout.id;
        let scope = layout.scope.clone();
        let entity = retained.entity;
        let view = retained.view.clone().expect("retained renderer");
        let global_id = self.element_id_interner.global_id(retained.element_path);
        self.layout_engine
            .as_mut()
            .expect("layout engine")
            .reset_retained_subtree(layout_id);
        let (layout, element) = self.with_view_layout_scope(scope, |window| {
            window.request_implicit_view_layout(entity, &global_id, &view, false, cx, render)
        });
        let mut engine = self.layout_engine.take().expect("layout engine");
        engine.compute_retained_layout(layout, input, self, cx);
        self.layout_engine = Some(engine);
        (element.expect("fresh implicit output"), layout)
    }

    pub(super) fn take_prepared_view(&mut self, id: ViewOccurrenceId) -> Option<AnyElement> {
        self.view_store
            .retained_mut(id)
            .prepared
            .take()
            .map(|(_, element)| element)
    }

    pub(super) fn discard_implicit_view_layout(&mut self, entity: EntityId) {
        let key = (entity, self.element_id_stack.handle());
        if let Some(id) = self.view_store.by_occurrence.get(&key).copied() {
            let retained = self.view_store.retained_mut(id);
            if retained.layout.take().is_some() {
                retained.prepared = None;
                self.view_store.invalidate_state(id);
            }
        }
    }

    pub(super) fn begin_view_occurrence(
        &mut self,
        entity: EntityId,
        global_id: Option<&GlobalElementId>,
        view: Option<&AnyWeakView>,
    ) -> Option<ViewOccurrenceId> {
        if view.is_none() && self.current_view_occurrence.is_none() {
            return None;
        }
        global_id?;
        let id = self.view_store.insert(
            entity,
            self.element_id_stack.handle(),
            self.current_view_occurrence,
            view,
            self.retained_scope_fallback,
        );
        self.next_frame.prepaint_views.push(id);
        Some(id)
    }

    pub(crate) fn register_retained_access(&mut self, id: ViewOccurrenceId, _cx: &mut App) {
        if let Some(layout) = self.view_store.retained(id).layout.as_ref() {
            self.layout_engine
                .as_mut()
                .expect("layout engine")
                .retain_subtree(layout.id);
        }
        if let Some(entities) = self
            .cached_entity_access_collectors
            .last()
            .and_then(|index| self.cached_entity_access_stack[*index].as_mut())
        {
            self.view_store.extend_accessed(id, entities);
        } else {
            self.view_store
                .extend_accessed(id, &mut self.frame_accessed_entities);
        }
    }

    pub(crate) fn reuse_retained_prepaint(
        &mut self,
        range: Range<PrepaintStateIndex>,
        cx: &mut App,
    ) {
        let start = self.next_frame.prepaint_views.len();
        self.reuse_prepaint(range);
        for index in start..self.next_frame.prepaint_views.len() {
            self.register_retained_access(self.next_frame.prepaint_views[index], cx);
        }
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

    pub(super) fn can_splice_view(&self, id: ViewOccurrenceId) -> bool {
        if self.a11y.is_active() {
            return false;
        }
        #[cfg(any(feature = "inspector", debug_assertions))]
        if self.inspector_enabled() {
            return false;
        }
        let record = self.view_store.record(id);
        if !record.own_dirty && !self.implicit_geometry_unchanged(id) {
            return false;
        }
        if record.dirty_path_blocked || record.blocked {
            return false;
        }
        let retained = self.view_store.retained(id);
        let (Some(prepaint_scope), Some(paint_scope)) =
            (&retained.prepaint_scope, &retained.paint_scope)
        else {
            return false;
        };
        if !self.view_scope_is_current(prepaint_scope) || !self.view_scope_is_current(paint_scope) {
            return false;
        }
        if prepaint_scope.fallback.is_some() || paint_scope.fallback.is_some() {
            return false;
        }
        if retained
            .view
            .as_ref()
            .and_then(AnyWeakView::upgrade)
            .is_none()
        {
            return false;
        }
        self.dirty_view_children(id)
            .into_iter()
            .all(|child| self.can_splice_view(child))
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
            .dispatch_node
            .expect("prepaint dispatch node");
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
            let mut old_paint_range = state.paint_range.clone();
            old_paint_range.start.views_index -= 1;
            // Explicit cached views access a handle outside their content range.
            // Implicit views already carry their occurrence through the frame.
            if retained.layout.is_none() {
                child_range.start.accessed_element_states_index -= 1;
                old_paint_range.start = old_paint_range.start.with_cached_state_access();
            }
            let cache_key: RetainedViewCacheKey = state.cache_key.clone();
            let global_id = self.element_id_interner.global_id(retained.element_path);
            let entity = retained.entity;
            let view = retained.view.clone().expect("retained child renderer");
            let mut scope = retained.prepaint_scope.clone().expect("prepaint scope");
            let paint_scope = retained.paint_scope.clone().expect("paint scope");
            self.reuse_retained_prepaint(cursor..child_range.start, cx);
            scope.dispatch_node = Some(
                self.dispatch_replay
                    .iter()
                    .rev()
                    .find_map(|range| {
                        scope
                            .dispatch_node
                            .and_then(|node| range.remap_node_id(node))
                    })
                    .expect("child root was replayed in prefix"),
            );
            let element = self.with_view_scope(scope, |window| {
                let mut element = window.take_prepared_view(child);
                prepaint_view(
                    entity,
                    Some(&global_id),
                    cache_key.bounds,
                    &mut element,
                    Some(&view),
                    &mut None,
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
        self.reuse_retained_prepaint(cursor..ancestor_range.end, cx);
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
            let global_id = self.element_id_interner.global_id(retained.element_path);
            let entity = retained.entity;
            child.paint_scope.dispatch_node = Some(
                self.dispatch_replay
                    .iter()
                    .rev()
                    .find_map(|range| {
                        child
                            .paint_scope
                            .dispatch_node
                            .and_then(|node| range.remap_node_id(node))
                    })
                    .expect("child root was replayed during prepaint"),
            );
            self.with_view_scope(child.paint_scope, |window| {
                paint_view(
                    entity,
                    true,
                    Some(&global_id),
                    &mut child.element,
                    Some(child.occurrence),
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
        AnyWindowHandle, AppContext, Context, Entity, FocusHandle, InteractiveElement, IntoElement,
        ParentElement, Pixels, Render, StatefulInteractiveElement, StyleRefinement, Styled,
        TestAppContext, ViewReuseSnapshot, Window, div, px, util::FluentBuilder,
    };
    use std::{cell::Cell, rc::Rc, time::Duration};

    crate::actions!(retained_test, [ProbeAction]);

    struct ImplicitLeaf {
        renders: Rc<Cell<usize>>,
        width: Pixels,
        margin: Pixels,
        color: u32,
        intrinsic: bool,
    }

    impl Render for ImplicitLeaf {
        fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
            self.renders.set(self.renders.get() + 1);
            div()
                .when(!self.intrinsic, |element| element.w(self.width))
                .h(px(20.))
                .ml(self.margin)
                .bg(crate::rgb(self.color))
                .when(self.intrinsic, |element| {
                    element.child(div().w(self.width).h(px(20.)))
                })
        }
    }

    struct ImplicitBranch {
        renders: Rc<Cell<usize>>,
        children: Vec<Entity<ImplicitLeaf>>,
    }

    impl Render for ImplicitBranch {
        fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
            self.renders.set(self.renders.get() + 1);
            div().flex().size(px(100.)).children(self.children.clone())
        }
    }

    struct ImplicitRoot(Entity<ImplicitBranch>);

    impl Render for ImplicitRoot {
        fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
            div().child(self.0.clone())
        }
    }

    struct ImplicitCachedRoot(Entity<ImplicitLeaf>);

    impl Render for ImplicitCachedRoot {
        fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
            div().child(
                self.0
                    .clone()
                    .cached(StyleRefinement::default().size(px(30.))),
            )
        }
    }

    #[crate::test]
    fn ordinary_ancestor_notification_preserves_explicit_cached_children(cx: &mut TestAppContext) {
        for implicit in [false, true] {
            let renders = Rc::new(Cell::new(0));
            let leaf = cx.new(|_| ImplicitLeaf {
                renders: renders.clone(),
                width: px(30.),
                margin: px(0.),
                color: 0xff0000,
                intrinsic: false,
            });
            let window = cx.add_window(|window, _| {
                window.set_view_retention(implicit);
                ImplicitCachedRoot(leaf)
            });
            let root = window.root(cx).expect("root");
            cx.update_window(window.into(), |_, window, cx| window.draw(cx).clear(cx))
                .expect("initial draw");
            cx.run_until_parked();
            let first_renders = renders.get();
            cx.update_window(window.into(), |_, window, _| window.take_view_reuse_stats())
                .expect("reset stats");
            root.update(cx, |_, cx| cx.notify());
            cx.update_window(window.into(), |_, window, cx| window.draw(cx).clear(cx))
                .expect("notification draw");
            let stats = cx
                .update_window(window.into(), |_, window, _| window.take_view_reuse_stats())
                .expect("stats");
            assert_eq!(
                renders.get(),
                first_renders,
                "implicit={implicit}: {stats:?}"
            );
            cx.update_window(window.into(), |_, window, cx| {
                window.refresh();
                window.draw(cx).clear(cx);
            })
            .expect("refresh draw");
            assert_eq!(renders.get(), first_renders + 1);
        }
    }

    #[crate::test]
    fn implicit_layout_fallback_preserves_clean_sibling(cx: &mut TestAppContext) {
        let clean_renders = Rc::new(Cell::new(0));
        let dirty_renders = Rc::new(Cell::new(0));
        let clean = cx.new(|_| ImplicitLeaf {
            renders: clean_renders.clone(),
            width: px(20.),
            margin: px(0.),
            color: 0xff0000,
            intrinsic: false,
        });
        let dirty = cx.new(|_| ImplicitLeaf {
            renders: dirty_renders.clone(),
            width: px(20.),
            margin: px(0.),
            color: 0x00ff00,
            intrinsic: false,
        });
        let branch = cx.new(|_| ImplicitBranch {
            renders: Rc::new(Cell::new(0)),
            children: vec![clean, dirty.clone()],
        });
        let window = cx.add_window(|window, _| {
            window.set_view_retention(true);
            ImplicitRoot(branch)
        });
        if !cx
            .update_window(window.into(), |_, window, _| window.layout_cache_enabled())
            .expect("window")
        {
            return;
        }
        cx.update_window(window.into(), |_, window, cx| window.draw(cx).clear(cx))
            .expect("initial draw");
        let first_clean_renders = clean_renders.get();
        let first_dirty_renders = dirty_renders.get();
        dirty.update(cx, |leaf, cx| {
            leaf.width = px(30.);
            cx.notify();
        });
        cx.update_window(window.into(), |_, window, cx| window.draw(cx).clear(cx))
            .expect("fallback draw");
        assert_eq!(clean_renders.get(), first_clean_renders);
        assert_eq!(dirty_renders.get(), first_dirty_renders + 1);
    }

    #[crate::test]
    fn implicit_boundaries_render_once_on_layout_fallback(cx: &mut TestAppContext) {
        let leaf_renders = Rc::new(Cell::new(0));
        let branch_renders = Rc::new(Cell::new(0));
        let leaf = cx.new(|_| ImplicitLeaf {
            renders: leaf_renders.clone(),
            width: px(20.),
            margin: px(0.),
            color: 0xff0000,
            intrinsic: false,
        });
        let branch = cx.new(|_| ImplicitBranch {
            renders: branch_renders.clone(),
            children: vec![leaf.clone()],
        });
        let window = cx.add_window(|window, _| {
            window.set_view_retention(true);
            ImplicitRoot(branch)
        });
        if !cx
            .update_window(window.into(), |_, window, _| window.layout_cache_enabled())
            .expect("window")
        {
            return;
        }
        cx.update_window(window.into(), |_, window, cx| window.draw(cx).clear(cx))
            .expect("initial draw");
        let initial_leaf_renders = leaf_renders.get();
        let initial_branch_renders = branch_renders.get();
        leaf.update(cx, |leaf, cx| {
            leaf.color ^= 0xffffff;
            cx.notify();
        });
        cx.update_window(window.into(), |_, window, cx| window.draw(cx).clear(cx))
            .expect("color draw");
        assert_eq!(leaf_renders.get(), initial_leaf_renders + 1);
        assert_eq!(
            branch_renders.get(),
            initial_branch_renders,
            "unchanged layout must splice"
        );

        leaf.update(cx, |leaf, cx| {
            leaf.margin = px(3.);
            cx.notify();
        });
        cx.update_window(window.into(), |_, window, cx| window.draw(cx).clear(cx))
            .expect("margin draw");
        assert_eq!(
            leaf_renders.get(),
            initial_leaf_renders + 2,
            "speculative Render must be consumed by fallback"
        );
        assert_eq!(
            branch_renders.get(),
            initial_branch_renders + 1,
            "equal-size parent-facing style change must fall back"
        );

        leaf.update(cx, |leaf, cx| {
            leaf.width = px(30.);
            cx.notify();
        });
        cx.update_window(window.into(), |_, window, cx| window.draw(cx).clear(cx))
            .expect("size draw");
        assert_eq!(leaf_renders.get(), initial_leaf_renders + 3);
        assert_eq!(branch_renders.get(), initial_branch_renders + 2);

        leaf.update(cx, |leaf, cx| {
            leaf.intrinsic = true;
            cx.notify();
        });
        cx.update_window(window.into(), |_, window, cx| {
            window.draw(cx).clear(cx);
            window.take_view_reuse_stats();
        })
        .expect("intrinsic mount");
        let previous = leaf_renders.get();
        let previous_branch = branch_renders.get();
        leaf.update(cx, |leaf, cx| {
            leaf.width = px(40.);
            cx.notify();
        });
        cx.update_window(window.into(), |_, window, cx| {
            window.draw(cx).clear(cx);
        })
        .expect("intrinsic size draw");
        assert_eq!(leaf_renders.get(), previous + 1);
        assert!(
            branch_renders.get() > previous_branch,
            "an intrinsic size change must fall back even with an identical root style"
        );
    }

    #[crate::test]
    fn retained_root_skips_color_layout_but_resizes_and_moves_siblings(cx: &mut TestAppContext) {
        let leaf = cx.new(|_| ImplicitLeaf {
            renders: Rc::default(),
            width: px(20.),
            margin: px(0.),
            color: 0xff0000,
            intrinsic: false,
        });
        let sibling = cx.new(|_| ImplicitLeaf {
            renders: Rc::default(),
            width: px(20.),
            margin: px(0.),
            color: 0x00ff00,
            intrinsic: false,
        });
        let branch = cx.new(|_| ImplicitBranch {
            renders: Rc::default(),
            children: vec![leaf.clone(), sibling.clone()],
        });
        let window = cx.add_window(|window, _| {
            window.set_view_retention(true);
            ImplicitRoot(branch)
        });
        let draw = |cx: &mut TestAppContext| {
            cx.update_window(window.into(), |_, window, cx| {
                window.draw(cx).clear(cx);
                if !window.view_splicing {
                    return None;
                }
                let occurrence = window.view_store.by_entity[&sibling.entity_id()][0];
                let bounds = window
                    .view_store
                    .state(occurrence)
                    .expect("sibling state")
                    .cache_key
                    .bounds;
                Some((window.take_view_reuse_stats(), bounds))
            })
            .expect("draw")
        };
        let Some((_, initial_bounds)) = draw(cx) else {
            return;
        };
        leaf.update(cx, |leaf, cx| {
            leaf.color ^= 0xffffff;
            cx.notify();
        });
        let (stats, unchanged_bounds) = draw(cx).expect("retained color frame");
        assert_eq!(stats.root_layout_computations, 0);
        assert!(stats.frames > 0);
        assert_eq!(stats.root_layout_skips, stats.frames);
        assert_eq!(unchanged_bounds, initial_bounds);
        leaf.update(cx, |leaf, cx| {
            leaf.width = px(30.);
            cx.notify();
        });
        let (stats, moved_bounds) = draw(cx).expect("retained resize frame");
        assert_eq!(stats.root_layout_computations, 1);
        assert_eq!(moved_bounds.origin.x, initial_bounds.origin.x + px(10.));
        assert_eq!(moved_bounds.size, initial_bounds.size);
    }

    struct TextPanel {
        renders: Rc<Cell<usize>>,
        alternate: bool,
    }

    impl Render for TextPanel {
        fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
            self.renders.set(self.renders.get() + 1);
            div()
                .w(px(100.))
                .h(px(20.))
                .child(if self.alternate { "Panel 1" } else { "Panel 0" })
        }
    }

    struct TextPanels(Vec<Entity<TextPanel>>);

    impl Render for TextPanels {
        fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
            div().flex().children(self.0.clone())
        }
    }

    #[crate::test]
    fn retained_text_panels_keep_unchanged_bounds_without_root_layout(cx: &mut TestAppContext) {
        let renders = Rc::new(Cell::new(0));
        let panels = (0..8)
            .map(|_| {
                cx.new(|_| TextPanel {
                    renders: renders.clone(),
                    alternate: false,
                })
            })
            .collect::<Vec<_>>();
        let window = cx.add_window(|window, _| {
            window.set_view_retention(true);
            TextPanels(panels.clone())
        });
        let retained = cx
            .update_window(window.into(), |_, window, cx| {
                window.draw(cx).clear(cx);
                window.take_view_reuse_stats();
                window.view_splicing
            })
            .expect("initial text frame");
        if !retained {
            return;
        }
        let initial_renders = renders.get();
        for panel in panels.iter().take(2) {
            panel.update(cx, |panel, cx| {
                panel.alternate = true;
                cx.notify();
            });
        }
        cx.update_window(window.into(), |_, window, cx| {
            window.draw(cx).clear(cx);
            assert_eq!(window.view_reuse_stats().root_layout_computations, 0);
            assert!(window.view_reuse_stats().frames > 0);
            assert_eq!(
                window.view_reuse_stats().root_layout_skips,
                window.view_reuse_stats().frames
            );
        })
        .expect("updated text frame");
        assert_eq!(renders.get(), initial_renders + 2);
    }

    struct ProbeGlobal(u32);
    impl crate::Global for ProbeGlobal {}
    struct AbsentGlobal;
    impl crate::Global for AbsentGlobal {}

    struct GlobalRoot {
        renders: Rc<Cell<usize>>,
        observed: Rc<Cell<u32>>,
        phase: DependencyPhase,
    }

    struct GlobalBoundaryRoot {
        renders: Rc<Cell<usize>>,
        child: Entity<GlobalRoot>,
    }

    impl Render for GlobalBoundaryRoot {
        fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
            self.renders.set(self.renders.get() + 1);
            div().child(self.child.clone())
        }
    }

    impl Render for GlobalRoot {
        fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
            self.renders.set(self.renders.get() + 1);
            let color = if matches!(self.phase, DependencyPhase::Render) {
                cx.global::<ProbeGlobal>().0
            } else {
                0
            };
            std::hint::black_box(cx.try_global::<AbsentGlobal>());
            let phase = self.phase;
            let prepaint_observed = self.observed.clone();
            let paint_observed = self.observed.clone();
            div().size_full().bg(crate::rgb(color)).child(crate::canvas(
                move |_, _, cx| {
                    if matches!(phase, DependencyPhase::Prepaint) {
                        prepaint_observed.set(cx.global::<ProbeGlobal>().0);
                    }
                },
                move |_, _, _, cx| {
                    if matches!(phase, DependencyPhase::Paint) {
                        paint_observed.set(cx.global::<ProbeGlobal>().0);
                    }
                },
            ))
        }
    }

    #[crate::test]
    fn retained_root_tracks_globals_in_all_phases(cx: &mut TestAppContext) {
        for phase in [
            DependencyPhase::Render,
            DependencyPhase::Prepaint,
            DependencyPhase::Paint,
        ] {
            cx.set_global(ProbeGlobal(0xff0000));
            let renders = Rc::new(Cell::new(0));
            let observed = Rc::new(Cell::new(0));
            let window = cx.add_window(|window, _| {
                window.set_view_retention(true);
                GlobalRoot {
                    renders: renders.clone(),
                    observed: observed.clone(),
                    phase,
                }
            });
            let retained = cx
                .update_window(window.into(), |_, window, cx| {
                    window.draw(cx).clear(cx);
                    window.view_splicing
                })
                .expect("initial draw");
            if !retained {
                return;
            }
            let initial = renders.get();
            cx.update_window(window.into(), |_, window, cx| window.draw(cx).clear(cx))
                .expect("clean draw");
            assert_eq!(renders.get(), initial, "unchanged root must retain");

            cx.set_global(ProbeGlobal(0x00ff00));
            cx.update_window(window.into(), |_, window, cx| window.draw(cx).clear(cx))
                .expect("global draw");
            assert_eq!(
                renders.get(),
                initial + 1,
                "global readers must render once"
            );
            if !matches!(phase, DependencyPhase::Render) {
                assert_eq!(observed.get(), 0x00ff00);
            }

            let previous = renders.get();
            cx.update_global::<ProbeGlobal, _>(|global, _| global.0 = 0x0000ff);
            cx.update_window(window.into(), |_, window, cx| window.draw(cx).clear(cx))
                .expect("updated global draw");
            assert_eq!(renders.get(), previous + 1);
            if !matches!(phase, DependencyPhase::Render) {
                assert_eq!(observed.get(), 0x0000ff);
            }

            let previous = renders.get();
            cx.set_global(AbsentGlobal);
            cx.update_window(window.into(), |_, window, cx| window.draw(cx).clear(cx))
                .expect("absent global installed");
            assert_eq!(
                renders.get(),
                previous + 1,
                "absent try_global reads are dependencies too"
            );
            let previous = renders.get();
            cx.update_window(window.into(), |_, window, cx| {
                window.resize(crate::size(px(640.), px(480.)));
                window.bounds_changed(cx);
                window.draw(cx).clear(cx);
            })
            .expect("resize draw");
            assert_eq!(renders.get(), previous + 1, "resize must refresh the root");
            cx.update_window(window.into(), |_, window, _| window.remove_window())
                .expect("remove window");
        }
    }

    #[crate::test]
    fn global_reads_belong_to_the_nearest_boundary(cx: &mut TestAppContext) {
        cx.set_global(ProbeGlobal(0xff0000));
        let root_renders = Rc::new(Cell::new(0));
        let child_renders = Rc::new(Cell::new(0));
        let window = cx.add_window(|window, cx| {
            window.set_view_retention(true);
            GlobalBoundaryRoot {
                renders: root_renders.clone(),
                child: cx.new(|_| GlobalRoot {
                    renders: child_renders.clone(),
                    observed: Rc::default(),
                    phase: DependencyPhase::Render,
                }),
            }
        });
        let retained = cx
            .update_window(window.into(), |_, window, cx| {
                window.draw(cx).clear(cx);
                window.view_splicing
            })
            .expect("initial draw");
        if !retained {
            return;
        }
        let initial_root = root_renders.get();
        let initial_child = child_renders.get();
        cx.set_global(ProbeGlobal(0x00ff00));
        cx.update_window(window.into(), |_, window, cx| window.draw(cx).clear(cx))
            .expect("global draw");
        assert_eq!(
            root_renders.get(),
            initial_root,
            "nested reads must not dirty the ancestor's own output"
        );
        assert_eq!(child_renders.get(), initial_child + 1);
    }

    struct MeasureGlobal(usize);
    impl crate::Global for MeasureGlobal {}

    struct MeasuredView {
        renders: Rc<Cell<usize>>,
        observed: Rc<Cell<usize>>,
        entity: Option<Entity<Cell<usize>>>,
        global: bool,
    }

    impl Render for MeasuredView {
        fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
            self.renders.set(self.renders.get() + 1);
            MeasuredElement {
                observed: self.observed.clone(),
                entity: self.entity.clone(),
                global: self.global,
            }
        }
    }

    struct MeasuredElement {
        observed: Rc<Cell<usize>>,
        entity: Option<Entity<Cell<usize>>>,
        global: bool,
    }

    impl IntoElement for MeasuredElement {
        type Element = Self;
        fn into_element(self) -> Self {
            self
        }
    }

    impl crate::Element for MeasuredElement {
        type RequestLayoutState = ();
        type PrepaintState = ();
        fn id(&self) -> Option<crate::ElementId> {
            None
        }
        fn source_location(&self) -> Option<&'static core::panic::Location<'static>> {
            None
        }
        fn request_layout(
            &mut self,
            _: Option<&crate::GlobalElementId>,
            _: Option<&crate::InspectorElementId>,
            window: &mut Window,
            _: &mut crate::App,
        ) -> (crate::LayoutId, ()) {
            let observed = self.observed.clone();
            let entity = self.entity.clone();
            let global = self.global;
            (
                window.request_measured_layout(crate::Style::default(), move |_, _, _, cx| {
                    let value = if global {
                        cx.global::<MeasureGlobal>().0
                    } else {
                        entity.as_ref().map_or(0, |entity| entity.read(cx).get())
                    };
                    observed.set(value);
                    crate::size(px(20.), px(20.))
                }),
                (),
            )
        }
        fn prepaint(
            &mut self,
            _: Option<&crate::GlobalElementId>,
            _: Option<&crate::InspectorElementId>,
            _: crate::Bounds<Pixels>,
            _: &mut (),
            _: &mut Window,
            _: &mut crate::App,
        ) {
        }
        fn paint(
            &mut self,
            _: Option<&crate::GlobalElementId>,
            _: Option<&crate::InspectorElementId>,
            _: crate::Bounds<Pixels>,
            _: &mut (),
            _: &mut (),
            _: &mut Window,
            _: &mut crate::App,
        ) {
        }
    }

    struct MeasureRoot {
        renders: Rc<Cell<usize>>,
        children: Vec<Entity<MeasuredView>>,
    }

    impl Render for MeasureRoot {
        fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
            self.renders.set(self.renders.get() + 1);
            div().flex().children(self.children.clone())
        }
    }

    #[crate::test]
    fn custom_measure_dependencies_dirty_only_the_owner(cx: &mut TestAppContext) {
        for global in [false, true] {
            cx.set_global(MeasureGlobal(1));
            let source = cx.new(|_| Cell::new(1));
            let root_renders = Rc::new(Cell::new(0));
            let owner_renders = Rc::new(Cell::new(0));
            let sibling_renders = Rc::new(Cell::new(0));
            let observed = Rc::new(Cell::new(0));
            let owner = cx.new(|_| MeasuredView {
                renders: owner_renders.clone(),
                observed: observed.clone(),
                entity: (!global).then(|| source.clone()),
                global,
            });
            let sibling = cx.new(|_| MeasuredView {
                renders: sibling_renders.clone(),
                observed: Rc::default(),
                entity: None,
                global: false,
            });
            let window = cx.add_window(|window, _| {
                window.set_view_retention(true);
                MeasureRoot {
                    renders: root_renders.clone(),
                    children: vec![owner, sibling],
                }
            });
            let retained = cx
                .update_window(window.into(), |_, window, cx| {
                    window.draw(cx).clear(cx);
                    window.view_splicing
                })
                .expect("initial draw");
            if !retained {
                return;
            }
            assert_eq!(observed.get(), 1);
            let initial = (
                root_renders.get(),
                owner_renders.get(),
                sibling_renders.get(),
            );
            if global {
                cx.set_global(MeasureGlobal(2));
            } else {
                source.update(cx, |source, cx| {
                    source.set(2);
                    cx.notify();
                });
            }
            cx.update_window(window.into(), |_, window, cx| window.draw(cx).clear(cx))
                .expect("measured dependency draw");
            assert_eq!(observed.get(), 2);
            assert_eq!(owner_renders.get(), initial.1 + 1);
            assert_eq!(
                sibling_renders.get(),
                initial.2,
                "unrelated sibling must retain"
            );
            assert_eq!(
                root_renders.get(),
                initial.0,
                "measure reads must not belong to the root"
            );
            cx.update_window(window.into(), |_, window, _| window.remove_window())
                .expect("remove measured window");
        }
    }

    #[derive(Default)]
    struct Leaf {
        renders: Rc<Cell<usize>>,
        color: u32,
        focus: Option<FocusHandle>,
        focus_on_prepaint: bool,
        action_color: Rc<Cell<u32>>,
        hover_color: Rc<Cell<u32>>,
        tooltip_color: Rc<Cell<u32>>,
        tooltip: Option<Entity<crate::EmptyView>>,
        dependency: Option<Entity<Cell<usize>>>,
    }

    impl Render for Leaf {
        fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
            self.renders.set(self.renders.get() + 1);
            let color = self.color;
            let action_color = self.action_color.clone();
            let hover_color = self.hover_color.clone();
            let tooltip_color = self.tooltip_color.clone();
            let focus = self.focus.clone();
            let focus_on_prepaint = self.focus_on_prepaint;
            let dependency = self.dependency.clone();
            let content = div()
                .id("leaf")
                .size(px(20.))
                .bg(crate::rgb(color))
                .key_context("RetainedLeaf")
                .when_some(self.focus.as_ref(), |this, focus| this.track_focus(focus))
                .on_action(move |_: &ProbeAction, _, _| action_color.set(color))
                .on_hover(move |hovered, _, _| {
                    if *hovered {
                        hover_color.set(color);
                    }
                })
                .when_some(self.tooltip.clone(), |this, tooltip| {
                    this.tooltip(move |_, _| {
                        tooltip_color.set(color);
                        tooltip.clone().into()
                    })
                    .tooltip_show_delay(Duration::from_millis(1))
                })
                .child("retained");
            div()
                .child(crate::canvas(
                    move |_, window, cx| {
                        if focus_on_prepaint && let Some(focus) = focus.as_ref() {
                            window.focus(focus, cx);
                        }
                    },
                    move |_, _, _, cx| {
                        if let Some(dependency) = dependency.as_ref() {
                            std::hint::black_box(dependency.read(cx).get());
                        }
                    },
                ))
                .child(content)
        }
    }

    #[derive(Default)]
    struct Branch {
        renders: Rc<Cell<usize>>,
        children: Vec<Entity<Leaf>>,
        child_size: Option<Pixels>,
        deferred_children: bool,
        overlapping_sibling: bool,
        uncached_children: bool,
        dynamic_prepaint_order: bool,
        untracked_child_size: Option<Rc<Cell<Pixels>>>,
    }

    impl Render for Branch {
        fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
            self.renders.set(self.renders.get() + 1);
            let child_size = self
                .untracked_child_size
                .as_ref()
                .map(|size| size.get())
                .or(self.child_size)
                .unwrap_or(px(20.));
            div()
                .flex()
                .size_full()
                .p(px(2.))
                .text_color(crate::rgb(0xabcdef))
                .opacity(0.7)
                .overflow_hidden()
                .key_context("RetainedBranch")
                .when(self.dynamic_prepaint_order, |this| {
                    this.with_dynamic_prepaint_order(|_, _| smallvec::smallvec![1, 0])
                })
                .children(self.children.iter().enumerate().map(|(index, child)| {
                    let child = if self.uncached_children {
                        child.clone().into_any_element()
                    } else {
                        child
                            .clone()
                            .cached(StyleRefinement::default().size(child_size))
                            .into_any_element()
                    };
                    let child = if self.deferred_children {
                        crate::deferred(child).into_any_element()
                    } else {
                        child
                    };
                    div()
                        .id(("slot", index))
                        .size(child_size)
                        .flex_shrink_0()
                        .child(child)
                }))
                .when(self.overlapping_sibling, |this| {
                    this.child(
                        div()
                            .absolute()
                            .left(px(4.))
                            .top(px(4.))
                            .size(px(30.))
                            .bg(crate::rgb(0x0000ff)),
                    )
                })
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
        callbacks_deferred: bool,
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
            let callbacks = crate::canvas(
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
            );
            let callbacks = if self.callbacks_deferred {
                crate::deferred(callbacks).into_any_element()
            } else {
                callbacks.into_any_element()
            };
            div()
                .bg(crate::rgb(color))
                .child(
                    self.leaf
                        .clone()
                        .cached(StyleRefinement::default().size(px(20.))),
                )
                .child(callbacks)
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
        for (phase, callbacks_deferred) in [
            DependencyPhase::Render,
            DependencyPhase::Prepaint,
            DependencyPhase::Paint,
        ]
        .into_iter()
        .flat_map(|phase| [false, true].map(|deferred| (phase, deferred)))
        {
            let parent_renders = Rc::new(Cell::new(0));
            let leaf = cx.new(|_| Leaf {
                renders: Rc::new(Cell::new(0)),
                color: 0xff0000,
                ..Leaf::default()
            });
            let parent = cx.new(|_| DependentParent {
                renders: parent_renders.clone(),
                leaf: leaf.clone(),
                phase,
                callbacks_deferred,
            });
            let window = cx.add_window(|window, _| {
                window.set_view_retention(true);
                DependentRoot(parent)
            });
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

    struct Fixture {
        window: AnyWindowHandle,
        leaves: [Entity<Leaf>; 2],
        branch: Entity<Branch>,
        branch_renders: Rc<Cell<usize>>,
        outer_renders: Rc<Cell<usize>>,
        identities: Vec<(crate::EntityId, String)>,
    }

    impl Fixture {
        fn new(cx: &mut TestAppContext, forced: bool, deferred: bool, overlapping: bool) -> Self {
            let tooltip = cx.new(|_| crate::EmptyView);
            let leaves = [0xff0000, 0xffff00].map(|color| {
                cx.new(|cx| Leaf {
                    color,
                    focus: Some(cx.focus_handle()),
                    tooltip: Some(tooltip.clone()),
                    ..Leaf::default()
                })
            });
            let branch_renders = Rc::new(Cell::new(0));
            let outer_renders = Rc::new(Cell::new(0));
            let branch = cx.new(|_| Branch {
                renders: branch_renders.clone(),
                children: leaves.to_vec(),
                deferred_children: deferred,
                overlapping_sibling: overlapping,
                ..Branch::default()
            });
            let outer = cx.new(|_| Outer {
                renders: outer_renders.clone(),
                child: branch.clone(),
            });
            let window = cx.add_window(|window, _| {
                window.set_force_view_reconstruction(forced);
                window.set_view_retention(!forced);
                Root(outer.clone())
            });
            let root = window.root(cx).expect("fixture root");
            Self {
                window: window.into(),
                identities: vec![
                    (root.entity_id(), "root".into()),
                    (outer.entity_id(), "outer".into()),
                    (branch.entity_id(), "branch".into()),
                    (leaves[0].entity_id(), "leaf-0".into()),
                    (leaves[1].entity_id(), "leaf-1".into()),
                    (tooltip.entity_id(), "tooltip".into()),
                ],
                leaves,
                branch,
                branch_renders,
                outer_renders,
            }
        }

        fn capture(&self, cx: &mut TestAppContext) -> ViewReuseSnapshot {
            cx.update_window(self.window, |_, window, cx| {
                window.draw(cx).clear(cx);
                window.view_reuse_snapshot(|id| {
                    self.identities
                        .iter()
                        .find(|(entity, _)| *entity == id)
                        .expect("fixture view identity")
                        .1
                        .clone()
                })
            })
            .expect("fixture window")
        }

        fn notify_leaf(&self, index: usize, cx: &mut TestAppContext) {
            self.leaves[index].update(cx, |leaf, cx| {
                leaf.color ^= 0xffffff;
                cx.notify();
            });
        }
    }

    fn compare_pair(reused: &Fixture, fresh: &Fixture, frame: usize, cx: &mut TestAppContext) {
        reused
            .capture(cx)
            .compare(&fresh.capture(cx), frame)
            .expect("differential output");
    }

    #[crate::test]
    fn splice_output_and_overlapping_later_sibling(cx: &mut TestAppContext) {
        for overlapping in [false, true] {
            let reused = Fixture::new(cx, false, false, overlapping);
            let fresh = Fixture::new(cx, true, false, overlapping);
            let mut leaf_renders = [1; 2];
            for frame in 0..8 {
                if frame > 1 {
                    leaf_renders[frame % 2] += 1;
                    if frame == 4 {
                        leaf_renders[(frame + 1) % 2] += 1;
                    }
                    for fixture in [&reused, &fresh] {
                        fixture.notify_leaf(frame % 2, cx);
                        if frame == 4 {
                            fixture.notify_leaf((frame + 1) % 2, cx);
                        }
                    }
                }
                compare_pair(&reused, &fresh, frame, cx);
                assert_eq!(reused.branch_renders.get(), 1);
                assert_eq!(reused.outer_renders.get(), 1);
                for (leaf, renders) in reused.leaves.iter().zip(leaf_renders) {
                    assert_eq!(leaf.read_with(cx, |leaf, _| leaf.renders.get()), renders);
                }
            }
            cx.update_window(reused.window, |_, window, _| {
                let stats = window.view_reuse_stats();
                assert!(stats.splice_prepaint_calls > 0);
                assert_eq!(stats.render_ancestor_caused, 0);
            })
            .expect("splice counters");
        }
    }

    #[crate::test]
    fn changed_external_bounds_reconstruct_the_child(cx: &mut TestAppContext) {
        let reused = Fixture::new(cx, false, false, true);
        let fresh = Fixture::new(cx, true, false, true);
        compare_pair(&reused, &fresh, 0, cx);
        for fixture in [&reused, &fresh] {
            fixture.branch.update(cx, |branch, cx| {
                branch.child_size = Some(px(30.));
                cx.notify();
            });
        }
        compare_pair(&reused, &fresh, 1, cx);
        assert_eq!(reused.outer_renders.get(), 1);
        assert_eq!(reused.branch_renders.get(), 2);
        cx.update_window(reused.window, |_, window, _| {
            assert!(window.view_reuse_stats().miss_bounds > 0);
        })
        .expect("changed bounds counter");
        for fixture in [&reused, &fresh] {
            fixture.notify_leaf(0, cx);
        }
        compare_pair(&reused, &fresh, 2, cx);
        assert_eq!(reused.branch_renders.get(), 2);
    }

    #[crate::test]
    fn deferred_dirty_child_falls_back(cx: &mut TestAppContext) {
        let reused = Fixture::new(cx, false, true, false);
        let fresh = Fixture::new(cx, true, true, false);
        compare_pair(&reused, &fresh, 0, cx);
        compare_pair(&reused, &fresh, 1, cx);
        for fixture in [&reused, &fresh] {
            fixture.notify_leaf(0, cx);
        }
        compare_pair(&reused, &fresh, 2, cx);
        assert_eq!(reused.branch_renders.get(), 2);
        assert_eq!(reused.outer_renders.get(), 2);
    }

    #[crate::test]
    fn implicit_children_preserve_offsets_and_can_become_explicit(cx: &mut TestAppContext) {
        let reused = Fixture::new(cx, false, false, false);
        let fresh = Fixture::new(cx, true, false, false);
        for fixture in [&reused, &fresh] {
            fixture.branch.update(cx, |branch, cx| {
                branch.uncached_children = true;
                cx.notify();
            });
        }
        compare_pair(&reused, &fresh, 0, cx);
        let outer_renders = reused.outer_renders.get();
        for fixture in [&reused, &fresh] {
            fixture.notify_leaf(0, cx);
        }
        compare_pair(&reused, &fresh, 1, cx);
        assert_eq!(reused.outer_renders.get(), outer_renders);
        for fixture in [&reused, &fresh] {
            fixture.branch.update(cx, |branch, cx| {
                branch.uncached_children = false;
                cx.notify();
            });
        }
        compare_pair(&reused, &fresh, 2, cx);
        let outer_renders = reused.outer_renders.get();
        let branch_renders = reused.branch_renders.get();
        for fixture in [&reused, &fresh] {
            fixture.notify_leaf(0, cx);
        }
        compare_pair(&reused, &fresh, 3, cx);
        assert_eq!(reused.outer_renders.get(), outer_renders);
        assert_eq!(reused.branch_renders.get(), branch_renders);
    }

    #[crate::test]
    fn dynamic_prepaint_order_falls_back(cx: &mut TestAppContext) {
        let reused = Fixture::new(cx, false, false, false);
        let fresh = Fixture::new(cx, true, false, false);
        for fixture in [&reused, &fresh] {
            fixture.branch.update(cx, |branch, cx| {
                branch.dynamic_prepaint_order = true;
                cx.notify();
            });
        }
        compare_pair(&reused, &fresh, 0, cx);
        let outer_renders = reused.outer_renders.get();
        for fixture in [&reused, &fresh] {
            fixture.notify_leaf(0, cx);
            fixture.notify_leaf(1, cx);
        }
        compare_pair(&reused, &fresh, 1, cx);
        assert!(reused.outer_renders.get() > outer_renders);
    }

    #[crate::test]
    fn untracked_external_style_is_expected_to_diverge(cx: &mut TestAppContext) {
        for splicing in [false, true] {
            let reused = Fixture::new(cx, false, false, false);
            let fresh = Fixture::new(cx, true, false, false);
            cx.update_window(reused.window, |_, window, _| {
                window.set_view_retention(splicing)
            })
            .expect("select reuse mode");
            let sizes = [Rc::new(Cell::new(px(20.))), Rc::new(Cell::new(px(20.)))];
            for (fixture, size) in [&reused, &fresh].into_iter().zip(&sizes) {
                fixture.branch.update(cx, |branch, cx| {
                    branch.untracked_child_size = Some(size.clone());
                    cx.notify();
                });
            }
            compare_pair(&reused, &fresh, 0, cx);
            for (fixture, size) in [&reused, &fresh].into_iter().zip(&sizes) {
                // Deliberately violate the composing view's notification contract.
                size.set(px(40.));
                fixture.notify_leaf(0, cx);
            }
            let result = reused.capture(cx).compare(&fresh.capture(cx), 1);
            if splicing {
                result.expect_err("the oracle must detect untracked external-style changes");
            } else {
                result.expect("legacy ancestor reconstruction masks this contract violation");
            }
        }
    }

    #[crate::test]
    fn spliced_child_preserves_focus_keys_hover_and_tooltips(cx: &mut TestAppContext) {
        cx.update(|cx| {
            cx.bind_keys([crate::KeyBinding::new(
                "ctrl-j",
                ProbeAction,
                Some("RetainedBranch > RetainedLeaf"),
            )]);
        });
        let reused = Fixture::new(cx, false, false, false);
        let fresh = Fixture::new(cx, true, false, false);
        compare_pair(&reused, &fresh, 0, cx);
        for fixture in [&reused, &fresh] {
            fixture.leaves[0].update(cx, |leaf, cx| {
                leaf.focus_on_prepaint = true;
                leaf.color = 0x00ff00;
                cx.notify();
            });
        }
        compare_pair(&reused, &fresh, 1, cx);
        assert_eq!(reused.branch_renders.get(), 1);
        assert_eq!(reused.outer_renders.get(), 1);
        for fixture in [&reused, &fresh] {
            let focus = fixture.leaves[0]
                .read_with(cx, |leaf, _| leaf.focus.clone())
                .expect("leaf focus");
            cx.update_window(fixture.window, |_, window, cx| {
                assert!(focus.is_focused(window));
                assert!(window.is_action_available(&ProbeAction, cx));
            })
            .expect("spliced focus path");
            cx.simulate_keystrokes(fixture.window, "ctrl-j");
            assert_eq!(
                fixture.leaves[0].read_with(cx, |leaf, _| leaf.action_color.get()),
                0x00ff00
            );
        }
        for fixture in [&reused, &fresh] {
            cx.update_window(fixture.window, |_, window, cx| {
                window.simulate_mouse_move(crate::point(px(10.), px(10.)), cx);
            })
            .expect("mouse move into spliced child");
        }
        cx.executor().advance_clock(Duration::from_millis(2));
        cx.run_until_parked();
        compare_pair(&reused, &fresh, 2, cx);
        for fixture in [&reused, &fresh] {
            assert_eq!(
                fixture.leaves[0].read_with(cx, |leaf, _| leaf.hover_color.get()),
                0x00ff00
            );
            assert_eq!(
                fixture.leaves[0].read_with(cx, |leaf, _| leaf.tooltip_color.get()),
                0x00ff00
            );
            cx.update_window(fixture.window, |_, window, _| {
                assert!(
                    window
                        .rendered_frame
                        .tooltip_requests
                        .iter()
                        .any(Option::is_some)
                );
                assert!(window.tooltip_bounds.is_some());
            })
            .expect("tooltip for spliced child");
        }
    }

    #[crate::test]
    fn spliced_paint_dependency_survives_complete_ancestor_reuse(cx: &mut TestAppContext) {
        let fixture = Fixture::new(cx, false, false, false);
        let dependency = cx.new(|_| Cell::new(0));
        fixture.capture(cx);
        fixture.leaves[0].update(cx, |leaf, cx| {
            leaf.dependency = Some(dependency.clone());
            cx.notify();
        });
        fixture.capture(cx);
        let renders = fixture.leaves[0].read_with(cx, |leaf, _| leaf.renders.get());
        for _ in 0..3 {
            fixture.capture(cx);
        }
        dependency.update(cx, |dependency, cx| {
            dependency.set(1);
            cx.notify();
        });
        fixture.capture(cx);
        assert_eq!(
            fixture.leaves[0].read_with(cx, |leaf, _| leaf.renders.get()),
            renders + 1
        );
        assert_eq!(fixture.branch_renders.get(), 1);
        assert_eq!(fixture.outer_renders.get(), 1);
    }

    #[crate::test]
    fn disabled_splicing_does_not_allocate_retained_storage(cx: &mut TestAppContext) {
        let fixture = Fixture::new(cx, true, false, false);
        fixture.capture(cx);
        cx.update_window(fixture.window, |_, window, _| {
            assert_eq!(window.view_store.records.capacity(), 0);
            assert_eq!(window.view_store.cold.capacity(), 0);
            assert_eq!(window.view_store.by_occurrence.capacity(), 0);
            assert_eq!(window.dispatch_replay.capacity(), 0);
            assert_eq!(window.next_frame.prepaint_views.capacity(), 0);
            assert_eq!(window.rendered_frame.paint_views.capacity(), 0);
        })
        .expect("legacy path allocates no retention metadata");
    }

    #[crate::test]
    fn occurrence_identity_and_generation_survive_remove_reinsert(cx: &mut TestAppContext) {
        let fixture = Fixture::new(cx, false, false, false);
        fixture.branch.update(cx, |branch, cx| {
            branch.children = vec![fixture.leaves[0].clone(), fixture.leaves[0].clone()];
            cx.notify();
        });
        fixture.capture(cx);
        let old_occurrences = cx
            .update_window(fixture.window, |_, window, _| {
                window.view_store.by_entity[&fixture.leaves[0].entity_id()].clone()
            })
            .expect("two occurrences");
        assert_eq!(old_occurrences.len(), 2);
        assert_ne!(old_occurrences[0], old_occurrences[1]);
        let initial_renders = fixture.leaves[0].read_with(cx, |leaf, _| leaf.renders.get());
        fixture.notify_leaf(0, cx);
        fixture.capture(cx);
        assert_eq!(
            fixture.leaves[0].read_with(cx, |leaf, _| leaf.renders.get()),
            initial_renders + 2
        );
        assert_eq!(fixture.branch_renders.get(), 2);
        fixture.branch.update(cx, |branch, cx| {
            branch.children.clear();
            cx.notify();
        });
        fixture.capture(cx);
        cx.update_window(fixture.window, |_, window, _| {
            assert!(
                !window
                    .view_store
                    .by_entity
                    .contains_key(&fixture.leaves[0].entity_id())
            );
        })
        .expect("removed occurrences are collected");
        fixture.branch.update(cx, |branch, cx| {
            branch.children.push(fixture.leaves[0].clone());
            cx.notify();
        });
        fixture.capture(cx);
        cx.update_window(fixture.window, |_, window, _| {
            let new_occurrence = window.view_store.by_entity[&fixture.leaves[0].entity_id()][0];
            assert!(!old_occurrences.contains(&new_occurrence));
            assert!(
                old_occurrences
                    .iter()
                    .any(|old| old.index == new_occurrence.index
                        && old.generation != new_occurrence.generation)
            );
        })
        .expect("reinserted occurrence has a fresh generation");
    }
}
