use crate::{
    AnyEntity, AnyWeakEntity, App, Context, ElementId, Entity, EntityId, Subscription, Window,
};

type ReadFn<P> = for<'a> fn(&AnyEntity, &'a App) -> &'a P;
type WriteFn<P> = fn(&AnyEntity, &mut App, &mut dyn FnMut(&mut P));

fn read_entity<'a, P: 'static>(entity: &AnyEntity, cx: &'a App) -> &'a P {
    cx.entities.read_any(entity)
}

fn write_entity<P: 'static>(entity: &AnyEntity, cx: &mut App, update: &mut dyn FnMut(&mut P)) {
    let entity = match entity.clone().downcast::<P>() {
        Ok(entity) => entity,
        Err(_) => unreachable!("an identity projection always stores an entity of its value type"),
    };
    entity.update(cx, |state, cx| {
        update(state);
        cx.notify();
    });
}

struct ReadProjectionState<E: 'static, P: ?Sized + 'static> {
    source: Entity<E>,
    lens: for<'a> fn(&'a E) -> &'a P,
    _subscription: Subscription,
}

impl<E: 'static, P: ?Sized + 'static> ReadProjectionState<E, P> {
    fn new<T: 'static>(
        source: &Entity<E>,
        lens: for<'a> fn(&'a E) -> &'a P,
        cx: &mut Context<T>,
    ) -> Self {
        Self {
            source: source.clone(),
            lens,
            _subscription: cx.observe(source, |_, _, cx| cx.notify()),
        }
    }

    fn update_source<T: 'static>(
        &mut self,
        source: &Entity<E>,
        lens: for<'a> fn(&'a E) -> &'a P,
        cx: &mut Context<T>,
    ) {
        if self.source != *source {
            self.source = source.clone();
            self._subscription = cx.observe(source, |_, _, cx| cx.notify());
            // The projected value is now read from a different entity, so
            // views that read this projection last frame must re-render even
            // though neither the old nor the new source notified.
            cx.notify();
        }
        self.lens = lens;
    }

    fn get<'a>(&self, cx: &'a App) -> &'a P {
        (self.lens)(self.source.read(cx))
    }
}

fn read_projection<'a, E: 'static, P: ?Sized + 'static>(entity: &AnyEntity, cx: &'a App) -> &'a P {
    cx.entities
        .read_any::<ReadProjectionState<E, P>>(entity)
        .get(cx)
}

struct MutableProjectionState<E: 'static, P: ?Sized + 'static> {
    read: ReadProjectionState<E, P>,
    write: for<'a> fn(&'a mut E) -> &'a mut P,
}

impl<E: 'static, P: ?Sized + 'static> MutableProjectionState<E, P> {
    fn new<T: 'static>(
        source: &Entity<E>,
        read: for<'a> fn(&'a E) -> &'a P,
        write: for<'a> fn(&'a mut E) -> &'a mut P,
        cx: &mut Context<T>,
    ) -> Self {
        Self {
            read: ReadProjectionState::new(source, read, cx),
            write,
        }
    }

    fn update_source<T: 'static>(
        &mut self,
        source: &Entity<E>,
        read: for<'a> fn(&'a E) -> &'a P,
        write: for<'a> fn(&'a mut E) -> &'a mut P,
        cx: &mut Context<T>,
    ) {
        self.read.update_source(source, read, cx);
        self.write = write;
    }
}

fn read_mutable_projection<'a, E: 'static, P: ?Sized + 'static>(
    entity: &AnyEntity,
    cx: &'a App,
) -> &'a P {
    cx.entities
        .read_any::<MutableProjectionState<E, P>>(entity)
        .read
        .get(cx)
}

fn write_projection<E: 'static, P: ?Sized + 'static>(
    entity: &AnyEntity,
    cx: &mut App,
    update: &mut dyn FnMut(&mut P),
) {
    let (source, write) = {
        let state = cx.entities.read_any::<MutableProjectionState<E, P>>(entity);
        (state.read.source.clone(), state.write)
    };
    source.update(cx, |state, cx| {
        update(write(state));
        cx.notify();
    });
}

/// A read-only handle to a value `P` projected out of an entity.
///
/// Projections erase their source: a `Projection<String>` may be backed by an
/// `Entity<String>` or by a lens into a field of some larger entity, and the
/// holder can't tell the difference. This makes them the right parameter type
/// for components that need to *read* state without dictating how the caller
/// stores it.
///
/// Projections are created during render, via [`Window::use_projection`] and
/// friends (or by converting an [`Entity`] with `From`). There is no way to
/// construct a lens projection outside a render context: a projection's
/// identity comes from its render call site, and state that needs an identity
/// independent of any view should be a proper entity instead.
///
/// Projections are strong handles: holding one keeps the source entity alive,
/// so reads are infallible. Use [`Projection::downgrade`] where that would
/// create a cycle.
///
/// Reads are access-tracked just like direct entity reads, so a view that
/// reads a projection during render is re-rendered when the source entity
/// notifies.
///
/// Note that notifications are only as fine-grained as the source entity: a
/// projection into a frequently-notified entity re-renders its readers on
/// every notification, whether or not the projected value changed. If that
/// becomes a problem, restructure the state so the projected value lives in
/// its own entity, and project from that.
pub struct Projection<P: ?Sized + 'static> {
    entity: AnyEntity,
    read: ReadFn<P>,
}

impl<P: ?Sized + 'static> Clone for Projection<P> {
    fn clone(&self) -> Self {
        Self {
            entity: self.entity.clone(),
            read: self.read,
        }
    }
}

impl<P: ?Sized + 'static> Projection<P> {
    /// Read the projected value.
    pub fn read<'a>(&self, cx: &'a App) -> &'a P {
        (self.read)(&self.entity, cx)
    }

    /// This projection's identity: the backing entity of the `use_projection`
    /// call site that created it, or the source entity for identity conversions
    /// from [`Entity`]. Notifications for the projected value are delivered as
    /// notifications of this entity.
    pub fn entity_id(&self) -> EntityId {
        self.entity.entity_id()
    }

    /// Convert this projection into a weak variant, which does not keep its
    /// backing state alive.
    pub fn downgrade(&self) -> WeakProjection<P> {
        WeakProjection {
            entity: self.entity.downgrade(),
            read: self.read,
        }
    }
}

impl<P: ?Sized + 'static> std::fmt::Debug for Projection<P> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Projection")
            .field("entity_id", &self.entity.entity_id())
            .finish_non_exhaustive()
    }
}

/// A read-write handle to a value `P` projected out of an entity.
///
/// Like [`Projection`], but writable: updates are applied through the lens to
/// the source entity, which is then notified. See [`Window::use_projection_mut`].
pub struct ProjectionMut<P: ?Sized + 'static> {
    read: Projection<P>,
    write: WriteFn<P>,
}

impl<P: ?Sized + 'static> Clone for ProjectionMut<P> {
    fn clone(&self) -> Self {
        Self {
            read: self.read.clone(),
            write: self.write,
        }
    }
}

impl<P: ?Sized + 'static> ProjectionMut<P> {
    /// Read the projected value.
    pub fn read<'a>(&self, cx: &'a App) -> &'a P {
        self.read.read(cx)
    }

    /// This projection's identity. See [`Projection::entity_id`].
    pub fn entity_id(&self) -> EntityId {
        self.read.entity_id()
    }

    /// Update the projected value, notifying the source entity.
    ///
    /// Unlike [`Entity::update`], this always notifies: a holder of a
    /// `ProjectionMut` has no other way to signal that the state changed, so
    /// every write is treated as a change.
    ///
    /// The usual entity update rules apply: calling this while the source
    /// entity is already being updated will panic.
    pub fn update<R>(&self, cx: &mut App, f: impl FnOnce(&mut P) -> R) -> R {
        let mut f = Some(f);
        let mut result = None;
        (self.write)(&self.read.entity, cx, &mut |value| {
            if let Some(f) = f.take() {
                result = Some(f(value));
            }
        });
        result.expect("the projection's write function must invoke the callback exactly once")
    }

    /// A read-only projection of the same value.
    pub fn read_only(&self) -> Projection<P> {
        self.read.clone()
    }

    /// Convert this projection into a weak variant, which does not keep its
    /// backing state alive.
    pub fn downgrade(&self) -> WeakProjectionMut<P> {
        WeakProjectionMut {
            read: self.read.downgrade(),
            write: self.write,
        }
    }
}

impl<P: ?Sized + 'static> std::fmt::Debug for ProjectionMut<P> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ProjectionMut")
            .field("entity_id", &self.read.entity.entity_id())
            .finish_non_exhaustive()
    }
}

/// A weak variant of [`Projection`] which does not keep its backing state
/// alive. Upgrade it to read.
pub struct WeakProjection<P: ?Sized + 'static> {
    entity: AnyWeakEntity,
    read: ReadFn<P>,
}

impl<P: ?Sized + 'static> Clone for WeakProjection<P> {
    fn clone(&self) -> Self {
        Self {
            entity: self.entity.clone(),
            read: self.read,
        }
    }
}

impl<P: ?Sized + 'static> WeakProjection<P> {
    /// This projection's identity. See [`Projection::entity_id`].
    pub fn entity_id(&self) -> EntityId {
        self.entity.entity_id()
    }

    /// Upgrade to a strong projection. Returns `None` if the backing state has
    /// been released.
    pub fn upgrade(&self) -> Option<Projection<P>> {
        Some(Projection {
            entity: self.entity.upgrade()?,
            read: self.read,
        })
    }
}

impl<P: ?Sized + 'static> std::fmt::Debug for WeakProjection<P> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WeakProjection")
            .field("entity_id", &self.entity.entity_id())
            .finish_non_exhaustive()
    }
}

/// A weak variant of [`ProjectionMut`] which does not keep its backing state
/// alive. Upgrade it to read or write.
pub struct WeakProjectionMut<P: ?Sized + 'static> {
    read: WeakProjection<P>,
    write: WriteFn<P>,
}

impl<P: ?Sized + 'static> Clone for WeakProjectionMut<P> {
    fn clone(&self) -> Self {
        Self {
            read: self.read.clone(),
            write: self.write,
        }
    }
}

impl<P: ?Sized + 'static> WeakProjectionMut<P> {
    /// This projection's identity. See [`Projection::entity_id`].
    pub fn entity_id(&self) -> EntityId {
        self.read.entity_id()
    }

    /// Upgrade to a strong projection. Returns `None` if the backing state has
    /// been released.
    pub fn upgrade(&self) -> Option<ProjectionMut<P>> {
        Some(ProjectionMut {
            read: self.read.upgrade()?,
            write: self.write,
        })
    }
}

impl<P: ?Sized + 'static> std::fmt::Debug for WeakProjectionMut<P> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WeakProjectionMut")
            .field("entity_id", &self.read.entity.entity_id())
            .finish_non_exhaustive()
    }
}

impl Window {
    /// Use a read-only projection of part of an entity's state. Must be called
    /// during render.
    ///
    /// The lens must be a plain function (closures that capture nothing coerce
    /// automatically):
    ///
    /// ```ignore
    /// let name: Projection<String> = window.use_projection(cx, &person, |person| &person.name);
    /// ```
    ///
    /// The projection's backing state is memoized per call site, like
    /// [`Window::use_state`], so sibling projections of different fields of one
    /// entity don't collide. When rendering multiple projections from the same
    /// location (e.g. in a loop), use [`Window::use_keyed_projection`].
    #[track_caller]
    pub fn use_projection<E: 'static, P: ?Sized + 'static>(
        &mut self,
        cx: &mut App,
        source: &Entity<E>,
        lens: for<'a> fn(&'a E) -> &'a P,
    ) -> Projection<P> {
        self.use_keyed_projection(
            ElementId::CodeLocation(*core::panic::Location::caller()),
            cx,
            source,
            lens,
        )
    }

    /// Like [`Window::use_projection`], with an explicit key to disambiguate
    /// call sites that render multiple times (e.g. in a loop).
    pub fn use_keyed_projection<E: 'static, P: ?Sized + 'static>(
        &mut self,
        key: impl Into<ElementId>,
        cx: &mut App,
        source: &Entity<E>,
        lens: for<'a> fn(&'a E) -> &'a P,
    ) -> Projection<P> {
        let state =
            self.use_keyed_state(key, cx, |_, cx| ReadProjectionState::new(source, lens, cx));
        state.update(cx, |state, cx| state.update_source(source, lens, cx));
        Projection {
            entity: state.into_any(),
            read: read_projection::<E, P>,
        }
    }

    /// Use a read-write projection of part of an entity's state. Must be
    /// called during render. See [`Window::use_projection`].
    ///
    /// Takes two lenses because reads only have shared access to the entity
    /// while writes have exclusive access; they should address the same value.
    /// The [`crate::project!`] macro writes both from a single field path.
    #[track_caller]
    pub fn use_projection_mut<E: 'static, P: ?Sized + 'static>(
        &mut self,
        cx: &mut App,
        source: &Entity<E>,
        read: for<'a> fn(&'a E) -> &'a P,
        write: for<'a> fn(&'a mut E) -> &'a mut P,
    ) -> ProjectionMut<P> {
        self.use_keyed_projection_mut(
            ElementId::CodeLocation(*core::panic::Location::caller()),
            cx,
            source,
            read,
            write,
        )
    }

    /// Like [`Window::use_projection_mut`], with an explicit key to
    /// disambiguate call sites that render multiple times (e.g. in a loop).
    pub fn use_keyed_projection_mut<E: 'static, P: ?Sized + 'static>(
        &mut self,
        key: impl Into<ElementId>,
        cx: &mut App,
        source: &Entity<E>,
        read: for<'a> fn(&'a E) -> &'a P,
        write: for<'a> fn(&'a mut E) -> &'a mut P,
    ) -> ProjectionMut<P> {
        let state = self.use_keyed_state(key, cx, |_, cx| {
            MutableProjectionState::new(source, read, write, cx)
        });
        state.update(cx, |state, cx| state.update_source(source, read, write, cx));
        ProjectionMut {
            read: Projection {
                entity: state.into_any(),
                read: read_mutable_projection::<E, P>,
            },
            write: write_projection::<E, P>,
        }
    }
}

/// Use a read-write projection of an entity field, writing both lenses from a
/// single field path. Must be called during render.
///
/// ```ignore
/// let name: ProjectionMut<String> = project!(window, cx, &person, name);
/// let city: ProjectionMut<String> = project!(window, cx, &person, address.city);
/// ```
///
/// Expands to [`Window::use_projection_mut`] with `|state| &state.<path>` and
/// `|state| &mut state.<path>` as the lenses.
#[macro_export]
macro_rules! project {
    ($window:expr, $cx:expr, $entity:expr, $($field:ident).+) => {
        $window.use_projection_mut(
            $cx,
            $entity,
            |state| &state.$($field).+,
            |state| &mut state.$($field).+,
        )
    };
}

impl<P: 'static> From<Entity<P>> for Projection<P> {
    fn from(entity: Entity<P>) -> Self {
        Self {
            entity: entity.into_any(),
            read: read_entity::<P>,
        }
    }
}

impl<P: 'static> From<Entity<P>> for ProjectionMut<P> {
    fn from(entity: Entity<P>) -> Self {
        Self {
            read: Projection {
                entity: entity.into_any(),
                read: read_entity::<P>,
            },
            write: write_entity::<P>,
        }
    }
}

impl<P: ?Sized + 'static> From<ProjectionMut<P>> for Projection<P> {
    fn from(projection: ProjectionMut<P>) -> Self {
        projection.read
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{AppContext as _, TestAppContext};
    use std::{
        cell::{Cell, RefCell},
        rc::Rc,
    };

    struct Person {
        name: String,
        age: u32,
    }

    fn projection<E: 'static, P: ?Sized + 'static>(
        cx: &mut TestAppContext,
        source: &Entity<E>,
        lens: for<'a> fn(&'a E) -> &'a P,
    ) -> Projection<P> {
        let state = cx.update(|cx| cx.new(|cx| ReadProjectionState::new(source, lens, cx)));
        Projection {
            entity: state.into_any(),
            read: read_projection::<E, P>,
        }
    }

    fn projection_mut<E: 'static, P: ?Sized + 'static>(
        cx: &mut TestAppContext,
        source: &Entity<E>,
        read: for<'a> fn(&'a E) -> &'a P,
        write: for<'a> fn(&'a mut E) -> &'a mut P,
    ) -> ProjectionMut<P> {
        let state =
            cx.update(|cx| cx.new(|cx| MutableProjectionState::new(source, read, write, cx)));
        ProjectionMut {
            read: Projection {
                entity: state.into_any(),
                read: read_mutable_projection::<E, P>,
            },
            write: write_projection::<E, P>,
        }
    }

    #[test]
    fn projection_reads_and_writes_through_the_lens() {
        let mut cx = TestAppContext::single();
        let person = cx.update(|cx| {
            cx.new(|_| Person {
                name: "Ada".to_string(),
                age: 36,
            })
        });

        let name = projection_mut(
            &mut cx,
            &person,
            |person| &person.name,
            |person| &mut person.name,
        );

        cx.update(|cx| {
            assert_eq!(name.read(cx), "Ada");
            name.update(cx, |name| name.push_str(" Lovelace"));
            assert_eq!(name.read(cx), "Ada Lovelace");
            assert_eq!(person.read(cx).name, "Ada Lovelace");
            assert_eq!(person.read(cx).age, 36);
        });
    }

    #[test]
    fn writes_through_a_projection_notify_the_source_entity() {
        let mut cx = TestAppContext::single();
        let person = cx.update(|cx| {
            cx.new(|_| Person {
                name: "Ada".to_string(),
                age: 36,
            })
        });
        let age = projection_mut(
            &mut cx,
            &person,
            |person| &person.age,
            |person| &mut person.age,
        );

        let notified = Rc::new(Cell::new(0));
        let _subscription = cx.update(|cx| {
            cx.observe(&person, {
                let notified = notified.clone();
                move |_, _| notified.set(notified.get() + 1)
            })
        });

        cx.update(|cx| age.update(cx, |age| *age += 1));

        assert_eq!(notified.get(), 1);
        cx.update(|cx| assert_eq!(*age.read(cx), 37));
    }

    #[test]
    fn entities_convert_to_projections() {
        let mut cx = TestAppContext::single();
        let value = cx.update(|cx| cx.new(|_| "hello".to_string()));

        let read_write: ProjectionMut<String> = value.clone().into();
        let read_only: Projection<String> = value.clone().into();
        let downgraded: Projection<String> = read_write.clone().into();

        cx.update(|cx| {
            read_write.update(cx, |value| value.push_str(" world"));
            assert_eq!(read_only.read(cx), "hello world");
            assert_eq!(downgraded.read(cx), "hello world");
            assert_eq!(read_only.entity_id(), value.entity_id());
        });
    }

    #[test]
    fn clones_share_the_same_source() {
        let mut cx = TestAppContext::single();
        let person = cx.update(|cx| {
            cx.new(|_| Person {
                name: "Ada".to_string(),
                age: 36,
            })
        });

        let name = projection_mut(
            &mut cx,
            &person,
            |person| &person.name,
            |person| &mut person.name,
        );
        let name_clone = name.clone();

        cx.update(|cx| {
            name.update(cx, |name| *name = "Grace".to_string());
            assert_eq!(name_clone.read(cx), "Grace");
        });
    }

    #[test]
    fn projections_can_be_unsized() {
        let mut cx = TestAppContext::single();
        let person = cx.update(|cx| {
            cx.new(|_| Person {
                name: "Ada".to_string(),
                age: 36,
            })
        });

        let name: Projection<str> = projection(&mut cx, &person, |person| person.name.as_str());

        cx.update(|cx| assert_eq!(name.read(cx), "Ada"));
    }

    #[test]
    fn use_projection_assigns_stable_distinct_identities() {
        use crate::{AnyWindowHandle, Context, IntoElement, Render, Window, div};
        use std::cell::RefCell;

        #[derive(Default)]
        struct Recorded {
            identities: Vec<(EntityId, EntityId)>,
            names: Vec<String>,
            name_projection: Option<ProjectionMut<String>>,
        }

        struct HookView {
            person: Entity<Person>,
            recorded: Rc<RefCell<Recorded>>,
        }

        impl Render for HookView {
            fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
                let name = crate::project!(window, cx, &self.person, name);
                let age = window.use_projection_mut(
                    cx,
                    &self.person,
                    |person| &person.age,
                    |person| &mut person.age,
                );

                let mut recorded = self.recorded.borrow_mut();
                recorded
                    .identities
                    .push((name.entity_id(), age.entity_id()));
                recorded.names.push(name.read(cx).clone());
                recorded.name_projection = Some(name);
                div()
            }
        }

        let mut cx = TestAppContext::single();
        let person = cx.update(|cx| {
            cx.new(|_| Person {
                name: "Ada".to_string(),
                age: 36,
            })
        });
        let person_id = person.entity_id();
        let recorded = Rc::new(RefCell::new(Recorded::default()));

        let window: AnyWindowHandle = cx
            .add_window({
                let recorded = recorded.clone();
                move |_, _| HookView { person, recorded }
            })
            .into();

        cx.update_window(window, |_, window, cx| {
            window.draw(cx).clear();
            window.draw(cx).clear();
        })
        .unwrap();

        let name_projection = recorded
            .borrow_mut()
            .name_projection
            .take()
            .expect("render ran");
        cx.update(|cx| name_projection.update(cx, |name| name.push_str(" Lovelace")));

        cx.update_window(window, |_, window, cx| {
            window.draw(cx).clear();
        })
        .unwrap();

        let recorded = recorded.borrow();
        assert!(recorded.identities.len() >= 3);
        let first = recorded.identities[0];
        assert!(
            recorded.identities.iter().all(|frame| *frame == first),
            "identities must be stable across frames: {:?}",
            recorded.identities
        );
        let (name_id, age_id) = first;
        assert_ne!(name_id, age_id, "call sites must have distinct identities");
        assert_ne!(name_id, person_id, "identity must differ from the source");
        assert_ne!(age_id, person_id, "identity must differ from the source");
        assert_eq!(
            recorded.names.last().map(String::as_str),
            Some("Ada Lovelace"),
            "writes through the projection must be visible to later renders"
        );
    }

    #[test]
    fn keyed_projection_updates_escaped_handles() {
        use crate::{Context, IntoElement, Render, Window, div};

        struct HookView {
            source: Entity<Person>,
            projection: Rc<RefCell<Option<ProjectionMut<String>>>>,
        }

        impl Render for HookView {
            fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
                let projection = window.use_keyed_projection_mut(
                    "name",
                    cx,
                    &self.source,
                    |person| &person.name,
                    |person| &mut person.name,
                );
                *self.projection.borrow_mut() = Some(projection);
                div()
            }
        }

        let mut cx = TestAppContext::single();
        let first = cx.update(|cx| {
            cx.new(|_| Person {
                name: "Ada".to_string(),
                age: 36,
            })
        });
        let second = cx.update(|cx| {
            cx.new(|_| Person {
                name: "Grace".to_string(),
                age: 37,
            })
        });
        let projection = Rc::new(RefCell::new(None));
        let window = cx.add_window({
            let first = first.clone();
            let projection = projection.clone();
            move |_, _| HookView {
                source: first,
                projection,
            }
        });

        cx.update_window(*window, |_, window, cx| window.draw(cx).clear())
            .unwrap();
        let escaped = projection
            .borrow()
            .as_ref()
            .expect("render created a projection")
            .clone();
        let identity = escaped.entity_id();

        window
            .update(&mut cx, |view, _, cx| {
                view.source = second.clone();
                cx.notify();
            })
            .unwrap();
        cx.update_window(*window, |_, window, cx| window.draw(cx).clear())
            .unwrap();

        cx.update(|cx| {
            assert_eq!(escaped.entity_id(), identity);
            assert_eq!(escaped.read(cx), "Grace");
            escaped.update(cx, |name| name.push_str(" Hopper"));
            assert_eq!(second.read(cx).name, "Grace Hopper");
            assert_eq!(first.read(cx).name, "Ada");
        });
    }

    #[test]
    fn swapping_the_source_notifies_and_replaces_the_subscription() {
        let mut cx = TestAppContext::single();
        let first = cx.update(|cx| {
            cx.new(|_| Person {
                name: "Ada".to_string(),
                age: 36,
            })
        });
        let second = cx.update(|cx| {
            cx.new(|_| Person {
                name: "Grace".to_string(),
                age: 37,
            })
        });
        let state = cx.update(|cx| {
            cx.new(|cx| {
                MutableProjectionState::new(
                    &first,
                    |person| &person.name,
                    |person| &mut person.name,
                    cx,
                )
            })
        });
        let notifications = Rc::new(Cell::new(0));
        let _subscription = cx.update(|cx| {
            cx.observe(&state, {
                let notifications = notifications.clone();
                move |_, _| notifications.set(notifications.get() + 1)
            })
        });

        cx.update(|cx| {
            state.update(cx, |state, cx| {
                state.update_source(&first, |person| &person.name, |person| &mut person.name, cx)
            });
        });
        assert_eq!(
            notifications.get(),
            0,
            "an unchanged source should not notify"
        );

        cx.update(|cx| {
            state.update(cx, |state, cx| {
                state.update_source(
                    &second,
                    |person| &person.name,
                    |person| &mut person.name,
                    cx,
                )
            });
        });
        assert_eq!(notifications.get(), 1, "swapping the source should notify");

        cx.update(|cx| first.update(cx, |_, cx| cx.notify()));
        assert_eq!(
            notifications.get(),
            1,
            "the old source's subscription should be dropped"
        );

        cx.update(|cx| second.update(cx, |_, cx| cx.notify()));
        assert_eq!(notifications.get(), 2);
    }

    #[test]
    fn escaped_projection_keeps_its_backing_state_alive() {
        use crate::{Context, IntoElement, Render, Window, div};

        struct HookView {
            source: Entity<Person>,
            show_projection: bool,
            projection: Rc<RefCell<Option<ProjectionMut<String>>>>,
        }

        impl Render for HookView {
            fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
                if self.show_projection {
                    *self.projection.borrow_mut() =
                        Some(crate::project!(window, cx, &self.source, name));
                }
                div()
            }
        }

        let mut cx = TestAppContext::single();
        let source = cx.update(|cx| {
            cx.new(|_| Person {
                name: "Ada".to_string(),
                age: 36,
            })
        });
        let projection = Rc::new(RefCell::new(None));
        let window = cx.add_window({
            let source = source.clone();
            let projection = projection.clone();
            move |_, _| HookView {
                source,
                show_projection: true,
                projection,
            }
        });

        cx.update_window(*window, |_, window, cx| window.draw(cx).clear())
            .unwrap();
        let escaped = projection
            .borrow_mut()
            .take()
            .expect("render created a projection");

        window
            .update(&mut cx, |view, _, cx| {
                view.show_projection = false;
                cx.notify();
            })
            .unwrap();
        cx.update_window(*window, |_, window, cx| window.draw(cx).clear())
            .unwrap();
        drop(source);
        cx.update(|_| {});

        cx.update(|cx| {
            escaped.update(cx, |name| name.push_str(" Lovelace"));
            assert_eq!(escaped.read(cx), "Ada Lovelace");
        });
    }

    #[test]
    fn weak_projections_do_not_keep_the_source_alive() {
        let mut cx = TestAppContext::single();
        let person = cx.update(|cx| {
            cx.new(|_| Person {
                name: "Ada".to_string(),
                age: 36,
            })
        });

        let name = projection_mut(
            &mut cx,
            &person,
            |person| &person.name,
            |person| &mut person.name,
        );
        let weak_name = name.downgrade();
        let weak_read_only = name.read_only().downgrade();

        {
            let upgraded = weak_name.upgrade().expect("source is alive");
            cx.update(|cx| {
                upgraded.update(cx, |name| name.push_str(" Lovelace"));
                assert_eq!(upgraded.read(cx), "Ada Lovelace");
            });
        }

        drop(person);
        drop(name);
        cx.update(|_| {});

        assert!(weak_name.upgrade().is_none());
        assert!(weak_read_only.upgrade().is_none());
    }
}
