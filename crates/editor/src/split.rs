use crate::EditorEvent;

use super::{Editor, EditorElement, EditorStyle};
use gpui::{App, Context, Entity, Render, Window};
use ui::{Element, IntoElement};
use workspace::ItemHandle;

// stage 1: render two side-by-side editors with the correct scrolling behavior
// stage 2: add alignment map to insert blank lines

/// An editor that can be rendered with a split diff layout.
///
/// When [secondary] is `None`, it is rendered with an inline diff style.
pub struct SplittableEditor {
    primary: Entity<Editor>,
    secondary: Option<Entity<Editor>>,
}

impl SplittableEditor {
    fn subscribe(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        cx.subscribe_in(
            &self.primary,
            window,
            |this, editor, event: &EditorEvent, window, cx| {},
        )
        .detach();
    }

    fn sync_state(&mut self, cx: &mut App) {}
}

impl SplittableEditor {}

struct SplitEditorElement {
    primary: Entity<Editor>,
    secondary: Entity<Editor>,
    style: EditorStyle,
}

struct SplitEditorElementLayout {}

impl Element for SplitEditorElement {
    type RequestLayoutState = ();

    type PrepaintState = SplitEditorElementLayout;

    fn id(&self) -> Option<ui::ElementId> {
        todo!()
    }

    fn source_location(&self) -> Option<&'static std::panic::Location<'static>> {
        todo!()
    }

    fn request_layout(
        &mut self,
        id: Option<&gpui::GlobalElementId>,
        inspector_id: Option<&gpui::InspectorElementId>,
        window: &mut ui::Window,
        cx: &mut ui::App,
    ) -> (gpui::LayoutId, Self::RequestLayoutState) {
    }

    fn prepaint(
        &mut self,
        id: Option<&gpui::GlobalElementId>,
        inspector_id: Option<&gpui::InspectorElementId>,
        bounds: gpui::Bounds<ui::Pixels>,
        request_layout: &mut Self::RequestLayoutState,
        window: &mut ui::Window,
        cx: &mut ui::App,
    ) -> Self::PrepaintState {
        todo!()
    }

    fn paint(
        &mut self,
        id: Option<&gpui::GlobalElementId>,
        inspector_id: Option<&gpui::InspectorElementId>,
        bounds: gpui::Bounds<ui::Pixels>,
        request_layout: &mut Self::RequestLayoutState,
        prepaint: &mut Self::PrepaintState,
        window: &mut ui::Window,
        cx: &mut ui::App,
    ) {
        todo!()
    }
}

impl Render for SplittableEditor {
    fn render(
        &mut self,
        window: &mut ui::Window,
        cx: &mut ui::Context<Self>,
    ) -> impl ui::IntoElement {
        enum SplittableEditorElement {
            Single(EditorElement),
            Split(SplitEditorElement),
        }

        impl Element for SplittableEditorElement {}
        impl IntoElement for SplittableEditorElement {
            type Element = Self;

            fn into_element(self) -> Self::Element {
                self
            }
        }

        let style;

        if let Some(secondary) = self.secondary.clone() {
            SplittableEditorElement::Split(SplitEditorElement {
                primary: self.primary.clone(),
                secondary,
                style,
            })
        } else {
            SplittableEditorElement::Single(EditorElement::new(&self.primary.clone(), style))
        }
    }
}

impl IntoElement for SplitEditorElement {
    type Element = Self;

    fn into_element(self) -> Self::Element {
        self
    }
}
