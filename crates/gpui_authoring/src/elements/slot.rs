//! A reactive boundary over an [`Entity`]'s state, driven by a closure.

use crate::{
    AnyElement, App, Bounds, Context, Element, ElementId, Entity, GlobalElementId, IntoElement,
    LayoutId, Pixels, Style, StyleRefinement, Styled, Window,
    view::{paint_view, prepaint_cached_view},
};
use refineable::Refineable;

/// Renders an [`Entity`] through a closure instead of a [`Render`][crate::Render]
/// implementation.
///
/// [`Entity::cached`] gets the same reuse for types that implement
/// [`Render`][crate::Render]. This is for state that has no renderer of its own,
/// or that is rendered differently in different places.
pub trait EntitySlotExt<T: 'static> {
    /// Renders this entity's state through `builder`, reusing the subtree it
    /// produces until the entity is notified.
    ///
    /// `id` identifies the slot, and becomes the root of the element id space the
    /// subtree's internal state lives in, so an entity rendered twice in
    /// different places needs a slot id per place.
    ///
    /// A slot is laid out from its [own style][crate::Styled], not from its
    /// contents: reusing a subtree means never rendering it to measure it. Give
    /// the slot a definite size, such as `.size_full()`.
    fn slot<F, E>(&self, id: impl Into<ElementId>, builder: F) -> SlotElement<T, F>
    where
        F: Fn(&T, &mut Window, &mut Context<T>) -> E + 'static,
        E: IntoElement + 'static;
}

impl<T: 'static> EntitySlotExt<T> for Entity<T> {
    #[track_caller]
    fn slot<F, E>(&self, id: impl Into<ElementId>, builder: F) -> SlotElement<T, F>
    where
        F: Fn(&T, &mut Window, &mut Context<T>) -> E + 'static,
        E: IntoElement + 'static,
    {
        SlotElement {
            entity: self.clone(),
            id: id.into(),
            style: StyleRefinement::default(),
            builder,
            #[cfg(debug_assertions)]
            source: core::panic::Location::caller(),
        }
    }
}

/// The element [`EntitySlotExt::slot`] returns.
///
/// Re-evaluates `builder` only when the entity was notified, the bounds it is
/// drawn at change, or the text style or content mask around it change. Otherwise
/// the previous frame's subtree is replayed, hitboxes and scene records included.
pub struct SlotElement<T: 'static, F> {
    entity: Entity<T>,
    id: ElementId,
    style: StyleRefinement,
    builder: F,
    #[cfg(debug_assertions)]
    source: &'static core::panic::Location<'static>,
}

impl<T: 'static, F, E> SlotElement<T, F>
where
    F: Fn(&T, &mut Window, &mut Context<T>) -> E + 'static,
    E: IntoElement + 'static,
{
    /// Runs `builder` against the entity's current state.
    fn build(&self, window: &mut Window, cx: &mut App) -> AnyElement {
        self.entity.update(cx, |state, cx| {
            (self.builder)(state, window, cx).into_any_element()
        })
    }
}

impl<T: 'static, F, E> Styled for SlotElement<T, F>
where
    F: Fn(&T, &mut Window, &mut Context<T>) -> E + 'static,
    E: IntoElement + 'static,
{
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl<T: 'static, F, E> IntoElement for SlotElement<T, F>
where
    F: Fn(&T, &mut Window, &mut Context<T>) -> E + 'static,
    E: IntoElement + 'static,
{
    type Element = Self;

    fn into_element(self) -> Self::Element {
        self
    }
}

impl<T: 'static, F, E> Element for SlotElement<T, F>
where
    F: Fn(&T, &mut Window, &mut Context<T>) -> E + 'static,
    E: IntoElement + 'static,
{
    type RequestLayoutState = Option<AnyElement>;
    type PrepaintState = Option<AnyElement>;

    fn id(&self) -> Option<ElementId> {
        Some(self.id.clone())
    }

    fn source_location(&self) -> Option<&'static core::panic::Location<'static>> {
        #[cfg(debug_assertions)]
        return Some(self.source);

        #[cfg(not(debug_assertions))]
        return None;
    }

    fn request_layout(
        &mut self,
        _id: Option<&GlobalElementId>,
        window: &mut Window,
        cx: &mut App,
    ) -> (LayoutId, Self::RequestLayoutState) {
        let entity_id = self.entity.entity_id();
        window.with_rendered_view(entity_id, |window| {
            if window.is_inspector_picking(cx) {
                // The inspector picks through the tree it can see, so a subtree
                // that is being reused has to be built for it.
                let mut element = self.build(window, cx);
                let layout_id = element.request_layout(window, cx);
                (layout_id, Some(element))
            } else {
                let mut root_style = Style::default();
                root_style.refine(&self.style);
                let layout_id = window.request_layout(root_style, None, cx);
                (layout_id, None)
            }
        })
    }

    fn prepaint(
        &mut self,
        global_id: Option<&GlobalElementId>,
        bounds: Bounds<Pixels>,
        element: &mut Self::RequestLayoutState,
        window: &mut Window,
        cx: &mut App,
    ) -> Option<AnyElement> {
        let entity_id = self.entity.entity_id();
        window.set_view_id(entity_id);
        window.with_rendered_view(entity_id, |window| {
            if let Some(mut element) = element.take() {
                element.prepaint(window, cx);
                return Some(element);
            }

            prepaint_cached_view(entity_id, global_id, bounds, window, cx, |window, cx| {
                self.build(window, cx)
            })
        })
    }

    fn paint(
        &mut self,
        global_id: Option<&GlobalElementId>,
        _bounds: Bounds<Pixels>,
        _request_layout: &mut Self::RequestLayoutState,
        element: &mut Self::PrepaintState,
        window: &mut Window,
        cx: &mut App,
    ) {
        paint_view(
            self.entity.entity_id(),
            true,
            global_id,
            element,
            window,
            cx,
        );
    }
}

#[cfg(test)]
mod tests {
    use std::{cell::Cell, rc::Rc};

    use crate::{
        AnyWindowHandle, AppContext as _, Context, Entity, EntitySlotExt as _, IntoElement,
        ParentElement as _, Render, Styled as _, TestAppContext, Window, div, green, px,
    };

    struct Counter(usize);

    struct SlotView {
        counter: Entity<Counter>,
        builds: Rc<Cell<usize>>,
    }

    impl Render for SlotView {
        fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
            let builds = self.builds.clone();
            div().size_full().child(
                self.counter
                    .slot("counter", move |_state, _window, _cx| {
                        builds.set(builds.get() + 1);
                        div().w(px(10.)).h(px(10.)).bg(green())
                    })
                    .size_full(),
            )
        }
    }

    /// Draws the window, returning how many quads the frame painted.
    fn draw(cx: &mut TestAppContext, window: AnyWindowHandle) -> usize {
        cx.update_window(window, |_, window, cx| {
            window.draw(cx).clear(cx);
            window.frame_state.rendered_frame.scene.quads.len()
        })
        .unwrap()
    }

    #[gpui::test]
    fn a_slot_reuses_its_subtree_until_the_entity_notifies(cx: &mut TestAppContext) {
        let builds = Rc::new(Cell::new(0));
        let counter = cx.new(|_| Counter(1));
        let window: AnyWindowHandle = cx
            .add_window({
                let counter = counter.clone();
                let builds = builds.clone();
                move |_, _| SlotView { counter, builds }
            })
            .into();

        let first = draw(cx, window);
        assert_eq!(builds.get(), 1, "the first frame builds the slot");
        assert!(first > 0, "the slot's content is painted");

        let second = draw(cx, window);
        assert_eq!(builds.get(), 1, "a clean slot is not rebuilt");
        assert_eq!(
            second, first,
            "the reused subtree is still painted into the scene"
        );

        counter.update(cx, |counter, cx| {
            counter.0 += 1;
            cx.notify();
        });

        let third = draw(cx, window);
        assert!(builds.get() > 1, "notifying the entity rebuilds the slot");
        assert_eq!(third, first, "the rebuilt subtree paints the same content");
    }
}
