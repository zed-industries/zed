use crate::{AnyEntity, AnyWeakEntity, App, Entity, EntityId};

/// A lens is a plain function pointer rather than a closure so that
/// projections are allocation-free: non-capturing closures coerce to function
/// pointers automatically, and capturing closures are rejected at compile
/// time. Projections are intentionally restricted to pure structural access —
/// if locating the value requires runtime data, restructure the state so it
/// doesn't (for example, by giving the value its own entity).
///
/// The erased pointer's true type is known only to the trampoline function
/// stored alongside it, which is monomorphized together with it and is the
/// only code that transmutes it back.
#[derive(Copy, Clone)]
struct ErasedLens(fn());

type ReadFn<P> = for<'a> fn(ErasedLens, &AnyEntity, &'a App) -> &'a P;
type WriteFn<P> = fn(ErasedLens, &AnyEntity, &mut App, &mut dyn FnMut(&mut P));

fn read_trampoline<'a, E: 'static, P: ?Sized + 'static>(
    lens: ErasedLens,
    entity: &AnyEntity,
    cx: &'a App,
) -> &'a P {
    // SAFETY: `lens` was erased from exactly this function pointer type in
    // `Entity::<E>::project`/`project_mut`, which pairs it with this
    // monomorphization of the trampoline. All function pointers have the same
    // size and layout.
    let lens = unsafe { std::mem::transmute::<fn(), for<'b> fn(&'b E) -> &'b P>(lens.0) };
    lens(cx.entities.read_any::<E>(entity))
}

fn write_trampoline<E: 'static, P: ?Sized + 'static>(
    lens: ErasedLens,
    entity: &AnyEntity,
    cx: &mut App,
    f: &mut dyn FnMut(&mut P),
) {
    // SAFETY: `lens` was erased from exactly this function pointer type in
    // `Entity::<E>::project_mut`, which pairs it with this monomorphization of
    // the trampoline. All function pointers have the same size and layout.
    let lens = unsafe { std::mem::transmute::<fn(), for<'b> fn(&'b mut E) -> &'b mut P>(lens.0) };
    let entity = match entity.downcast_ref::<E>() {
        Some(entity) => entity,
        None => unreachable!("a projection always stores the handle its lens was created for"),
    };
    entity.update(cx, |state, cx| {
        f(lens(state));
        cx.notify();
    });
}

/// A read-only handle to a value `P` projected out of an entity.
///
/// Projections erase their source: a `Projection<String>` may be backed by an
/// `Entity<String>` or by a lens into a field of some larger entity, and the
/// holder can't tell the difference. This makes them the right parameter type
/// for components that need to *read* state without dictating how the caller
/// stores it. `P` may be unsized, so display-only components can accept e.g.
/// `Projection<str>` and be backed by any string-shaped state.
///
/// Projections are strong handles: holding one keeps the source entity alive,
/// so reads are infallible. Use [`Projection::downgrade`] where that would
/// create a cycle. They are also allocation-free — the cost of constructing
/// one is cloning the entity handle.
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
    lens: ErasedLens,
    read: ReadFn<P>,
}

impl<P: ?Sized + 'static> Clone for Projection<P> {
    fn clone(&self) -> Self {
        Self {
            entity: self.entity.clone(),
            lens: self.lens,
            read: self.read,
        }
    }
}

impl<P: ?Sized + 'static> Projection<P> {
    /// Read the projected value.
    pub fn read<'a>(&self, cx: &'a App) -> &'a P {
        (self.read)(self.lens, &self.entity, cx)
    }

    /// The id of the entity this projection reads from. Notifications for the
    /// projected value are delivered as notifications of this entity.
    pub fn entity_id(&self) -> EntityId {
        self.entity.entity_id()
    }

    /// Convert this projection into a weak variant, which does not keep the
    /// source entity alive.
    pub fn downgrade(&self) -> WeakProjection<P> {
        WeakProjection {
            entity: self.entity.downgrade(),
            lens: self.lens,
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
/// the source entity, which is then notified. See [`Entity::project_mut`].
pub struct ProjectionMut<P: ?Sized + 'static> {
    read: Projection<P>,
    write_lens: ErasedLens,
    write: WriteFn<P>,
}

impl<P: ?Sized + 'static> Clone for ProjectionMut<P> {
    fn clone(&self) -> Self {
        Self {
            read: self.read.clone(),
            write_lens: self.write_lens,
            write: self.write,
        }
    }
}

impl<P: ?Sized + 'static> ProjectionMut<P> {
    /// Read the projected value.
    pub fn read<'a>(&self, cx: &'a App) -> &'a P {
        self.read.read(cx)
    }

    /// The id of the entity this projection reads from and writes to.
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
        (self.write)(self.write_lens, &self.read.entity, cx, &mut |value| {
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

    /// Convert this projection into a weak variant, which does not keep the
    /// source entity alive.
    pub fn downgrade(&self) -> WeakProjectionMut<P> {
        WeakProjectionMut {
            read: self.read.downgrade(),
            write_lens: self.write_lens,
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

/// A weak variant of [`Projection`] which does not keep the source entity
/// alive. Upgrade it to read.
pub struct WeakProjection<P: ?Sized + 'static> {
    entity: AnyWeakEntity,
    lens: ErasedLens,
    read: ReadFn<P>,
}

impl<P: ?Sized + 'static> Clone for WeakProjection<P> {
    fn clone(&self) -> Self {
        Self {
            entity: self.entity.clone(),
            lens: self.lens,
            read: self.read,
        }
    }
}

impl<P: ?Sized + 'static> WeakProjection<P> {
    /// The id of the entity this projection reads from.
    pub fn entity_id(&self) -> EntityId {
        self.entity.entity_id()
    }

    /// Upgrade to a strong projection. Returns `None` if the source entity has
    /// been released.
    pub fn upgrade(&self) -> Option<Projection<P>> {
        Some(Projection {
            entity: self.entity.upgrade()?,
            lens: self.lens,
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

/// A weak variant of [`ProjectionMut`] which does not keep the source entity
/// alive. Upgrade it to read or write.
pub struct WeakProjectionMut<P: ?Sized + 'static> {
    read: WeakProjection<P>,
    write_lens: ErasedLens,
    write: WriteFn<P>,
}

impl<P: ?Sized + 'static> Clone for WeakProjectionMut<P> {
    fn clone(&self) -> Self {
        Self {
            read: self.read.clone(),
            write_lens: self.write_lens,
            write: self.write,
        }
    }
}

impl<P: ?Sized + 'static> WeakProjectionMut<P> {
    /// The id of the entity this projection reads from and writes to.
    pub fn entity_id(&self) -> EntityId {
        self.read.entity_id()
    }

    /// Upgrade to a strong projection. Returns `None` if the source entity has
    /// been released.
    pub fn upgrade(&self) -> Option<ProjectionMut<P>> {
        Some(ProjectionMut {
            read: self.read.upgrade()?,
            write_lens: self.write_lens,
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

impl<E: 'static> Entity<E> {
    /// Project a read-only view of part of this entity's state.
    ///
    /// The lens maps the entity's state to the projected value. It must be a
    /// plain function (closures that capture nothing coerce automatically):
    ///
    /// ```ignore
    /// let name: Projection<String> = person.project(|person| &person.name);
    /// let name_str: Projection<str> = person.project(|person| person.name.as_str());
    /// ```
    pub fn project<P: ?Sized + 'static>(&self, lens: for<'a> fn(&'a E) -> &'a P) -> Projection<P> {
        Projection {
            entity: self.clone().into_any(),
            // SAFETY: erasing a function pointer's type; `read_trampoline::<E, P>`
            // stored alongside it transmutes it back to exactly this type.
            lens: ErasedLens(unsafe {
                std::mem::transmute::<for<'a> fn(&'a E) -> &'a P, fn()>(lens)
            }),
            read: read_trampoline::<E, P>,
        }
    }

    /// Project a read-write view of part of this entity's state.
    ///
    /// Takes two lenses because reads only have shared access to the entity
    /// while writes have exclusive access; they should address the same value.
    /// Like [`Entity::project`], the lenses must be plain functions:
    ///
    /// ```ignore
    /// let name = person.project_mut(|person| &person.name, |person| &mut person.name);
    /// ```
    pub fn project_mut<P: ?Sized + 'static>(
        &self,
        read: for<'a> fn(&'a E) -> &'a P,
        write: for<'a> fn(&'a mut E) -> &'a mut P,
    ) -> ProjectionMut<P> {
        ProjectionMut {
            read: self.project(read),
            // SAFETY: erasing a function pointer's type; `write_trampoline::<E, P>`
            // stored alongside it transmutes it back to exactly this type.
            write_lens: ErasedLens(unsafe {
                std::mem::transmute::<for<'a> fn(&'a mut E) -> &'a mut P, fn()>(write)
            }),
            write: write_trampoline::<E, P>,
        }
    }
}

impl<P: 'static> From<Entity<P>> for Projection<P> {
    fn from(entity: Entity<P>) -> Self {
        entity.project(|value| value)
    }
}

impl<P: 'static> From<Entity<P>> for ProjectionMut<P> {
    fn from(entity: Entity<P>) -> Self {
        entity.project_mut(|value| value, |value| value)
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
    use std::{cell::Cell, rc::Rc};

    struct Person {
        name: String,
        age: u32,
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

        let name = person.project_mut(|person| &person.name, |person| &mut person.name);

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
        let age = person.project_mut(|person| &person.age, |person| &mut person.age);

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
    fn entities_convert_to_projections_with_an_identity_lens() {
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

        let name = person.project_mut(|person| &person.name, |person| &mut person.name);
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

        let name: Projection<str> = person.project(|person| person.name.as_str());

        cx.update(|cx| assert_eq!(name.read(cx), "Ada"));
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

        let name = person.project_mut(|person| &person.name, |person| &mut person.name);
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
