use std::fmt::{self, Display};

/// A unique identifier for an element that can be debugged.
#[derive(Debug, Eq, PartialEq, Hash)]
pub struct InspectorElementId {
    #[cfg(any(feature = "inspector", debug_assertions))]
    pub(crate) global_id: crate::GlobalElementId,
    /// Source location where this element was constructed.
    #[cfg(any(feature = "inspector", debug_assertions))]
    pub source: &'static std::panic::Location<'static>,
    #[cfg(any(feature = "inspector", debug_assertions))]
    pub(crate) instance_id: usize,
}

impl Clone for InspectorElementId {
    fn clone(&self) -> Self {
        #[cfg(any(feature = "inspector", debug_assertions))]
        {
            Self {
                global_id: crate::GlobalElementId(self.global_id.0.clone()),
                source: self.source,
                instance_id: self.instance_id,
            }
        }

        #[cfg(not(any(feature = "inspector", debug_assertions)))]
        {
            Self {}
        }
    }
}

impl Into<InspectorElementId> for &InspectorElementId {
    fn into(self) -> InspectorElementId {
        self.clone()
    }
}

impl Display for InspectorElementId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        #[cfg(any(feature = "inspector", debug_assertions))]
        {
            for (i, element_id) in self.global_id.0.iter().enumerate() {
                if i > 0 {
                    write!(f, ".")?;
                }
                write!(f, "{}", element_id)?;
            }
            write!(f, ":{}[{}]", self.source, self.instance_id)?;
        }

        #[cfg(not(any(feature = "inspector", debug_assertions)))]
        {
            write!(f, "<InspectorElementId only used in debug builds>")?;
        }

        Ok(())
    }
}

#[cfg(any(feature = "inspector", debug_assertions))]
pub(crate) use conditional::*;

#[cfg(any(feature = "inspector", debug_assertions))]
mod conditional {
    use super::*;
    use crate::{AnyElement, App, Context, Empty, IntoElement, Render, Window};
    use collections::FxHashMap;
    use std::any::{Any, TypeId};

    pub struct Inspector {
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

        fn render_inspector_states(
            &mut self,
            window: &mut Window,
            cx: &mut Context<Self>,
        ) -> Vec<AnyElement> {
            let mut elements = Vec::new();
            if let Some(inspected_element_id) = self.active_element_id.take() {
                if let Some(states_by_type_id) = window
                    .next_frame
                    .inspector_state
                    .element_states
                    .remove(&inspected_element_id)
                {
                    for (type_id, state) in &states_by_type_id {
                        if let Some(render_inspector) = cx
                            .inspector_element_registry
                            .renderers_by_type_id
                            .remove(&type_id)
                        {
                            let mut element = (render_inspector)(
                                inspected_element_id.clone(),
                                state.as_ref(),
                                window,
                                cx,
                            );
                            elements.push(element);
                            cx.inspector_element_registry
                                .renderers_by_type_id
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
            if let Some(inspector_renderer) = cx.inspector_renderer.take() {
                let rendered_inspector_states = self.render_inspector_states(window, cx);
                let result = inspector_renderer(rendered_inspector_states, window, cx);
                cx.inspector_renderer = Some(inspector_renderer);
                result
            } else {
                Empty.into_any_element()
            }
        }
    }

    #[derive(Default)]
    pub struct InspectorElementRegistry {
        renderers_by_type_id: FxHashMap<
            TypeId,
            Box<dyn Fn(InspectorElementId, &dyn Any, &mut Window, &mut App) -> AnyElement>,
        >,
    }

    impl InspectorElementRegistry {
        pub fn register<T: 'static, R: IntoElement>(
            &mut self,
            f: impl 'static + Fn(InspectorElementId, &T, &mut Window, &mut App) -> R,
        ) {
            self.renderers_by_type_id.insert(
                TypeId::of::<T>(),
                Box::new(move |id, value, window, cx| {
                    let value = value.downcast_ref().unwrap();
                    f(id, value, window, cx).into_any_element()
                }),
            );
        }
    }
}
