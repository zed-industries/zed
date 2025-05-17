use smallvec::SmallVec;

use crate::{
    AnyElement, Context, GlobalElementId, InteractiveElement, IntoElement, ParentElement, Render,
    Styled, Window, div,
};
use std::panic;

/// A unique identifier for an element that can be debugged.
#[cfg(debug_assertions)]
#[derive(Debug, Eq, PartialEq, Hash)]
pub struct InspectorElementId {
    pub(crate) global_id: GlobalElementId,
    pub(crate) source: &'static panic::Location<'static>,
    pub(crate) instance_id: usize,
}

#[cfg(debug_assertions)]
impl Clone for InspectorElementId {
    fn clone(&self) -> Self {
        Self {
            global_id: GlobalElementId(self.global_id.0.clone()),
            source: self.source,
            instance_id: self.instance_id,
        }
    }
}

#[cfg(not(debug_assertions))]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct InspectorElementId;

pub(crate) struct Inspector {
    active_element_id: Option<InspectorElementId>,
}

impl Inspector {
    pub fn new() -> Self {
        Self {
            active_element_id: None,
        }
    }

    pub fn select(&mut self, id: Option<InspectorElementId>, cx: &mut Context<Self>) {
        self.active_element_id = id;
        cx.notify();
    }

    pub fn active_element_id(&self) -> Option<&InspectorElementId> {
        self.active_element_id.as_ref()
    }

    fn render_elements(
        &mut self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> SmallVec<[AnyElement; 1]> {
        let mut elements = SmallVec::new();
        if let Some(inspected_element_id) = self.active_element_id.take() {
            if let Some(states_by_type_id) = window
                .next_frame
                .inspector_state
                .element_states
                .remove(&inspected_element_id)
            {
                for (type_id, state) in &states_by_type_id {
                    if let Some(render_inspector) = cx.inspector_element_registry.remove(&type_id) {
                        let mut element = (render_inspector)(
                            inspected_element_id.clone(),
                            state.as_ref(),
                            window,
                            cx,
                        );
                        elements.push(element);
                        cx.inspector_element_registry
                            .insert(*type_id, render_inspector);
                    }
                }

                window
                    .next_frame
                    .inspector_state
                    .element_states
                    .insert(inspected_element_id.clone(), states_by_type_id);
            }

            self.active_element_id = Some(inspected_element_id);
        }

        elements
    }
}

impl Render for Inspector {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div().flex().flex_col().size_full().items_end().child(
            div()
                .flex()
                .flex_col()
                .occlude()
                .children(self.render_elements(window, cx)),
        )
    }
}
