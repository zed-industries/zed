/// A unique identifier for an element that can be inspected.
#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub struct InspectorElementId {
    /// Stable part of the ID.
    #[cfg(any(feature = "inspector", debug_assertions))]
    pub path: std::rc::Rc<InspectorElementPath>,
    /// Disambiguates elements that have the same path.
    #[cfg(any(feature = "inspector", debug_assertions))]
    pub instance_id: usize,
}

/// `GlobalElementId` qualified by source location of element construction.
#[cfg(any(feature = "inspector", debug_assertions))]
#[derive(Debug, Eq, PartialEq, Hash)]
pub struct InspectorElementPath {
    /// The path to the nearest ancestor element that has an `ElementId`.
    #[cfg(any(feature = "inspector", debug_assertions))]
    pub global_id: crate::GlobalElementId,
    /// Source location where this element was constructed.
    #[cfg(any(feature = "inspector", debug_assertions))]
    pub source_location: &'static std::panic::Location<'static>,
}

#[cfg(any(feature = "inspector", debug_assertions))]
impl Clone for InspectorElementPath {
    fn clone(&self) -> Self {
        Self {
            global_id: crate::GlobalElementId(self.global_id.0.clone()),
            source_location: self.source_location,
        }
    }
}

impl Into<InspectorElementId> for &InspectorElementId {
    fn into(self) -> InspectorElementId {
        self.clone()
    }
}

#[cfg(any(feature = "inspector", debug_assertions))]
impl Into<InspectorElementPath> for &InspectorElementPath {
    fn into(self) -> InspectorElementPath {
        self.clone()
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
        active_element: Option<InspectedElement>,
        pub pick_depth: Option<f32>,
    }

    struct InspectedElement {
        id: InspectorElementId,
        states: FxHashMap<TypeId, Box<dyn Any>>,
    }

    impl InspectedElement {
        fn new(id: InspectorElementId) -> Self {
            InspectedElement {
                id,
                states: FxHashMap::default(),
            }
        }
    }

    impl Inspector {
        pub fn new() -> Self {
            Self {
                active_element: None,
                pick_depth: Some(0.0),
            }
        }

        pub fn select(&mut self, id: InspectorElementId, window: &mut Window) {
            self.set_active_element_id(id, window);
            self.pick_depth = None;
        }

        pub fn hover(&mut self, id: InspectorElementId, window: &mut Window) {
            if self.is_picking() {
                let changed = self.set_active_element_id(id, window);
                if changed {
                    self.pick_depth = Some(0.0);
                }
            }
        }

        pub fn set_active_element_id(
            &mut self,
            id: InspectorElementId,
            window: &mut Window,
        ) -> bool {
            let changed = Some(&id) != self.active_element_id();
            if changed {
                self.active_element = Some(InspectedElement::new(id));
                window.refresh();
            }
            changed
        }

        pub fn active_element_id(&self) -> Option<&InspectorElementId> {
            self.active_element.as_ref().map(|e| &e.id)
        }

        pub fn with_active_element_state<T: 'static, R>(
            &mut self,
            window: &mut Window,
            f: impl FnOnce(&mut Option<T>, &mut Window) -> R,
        ) -> R {
            let Some(active_element) = &mut self.active_element else {
                return f(&mut None, window);
            };

            let type_id = TypeId::of::<T>();
            let mut inspector_state = active_element
                .states
                .remove(&type_id)
                .map(|state| *state.downcast().unwrap());

            let result = f(&mut inspector_state, window);

            if let Some(inspector_state) = inspector_state {
                active_element
                    .states
                    .insert(type_id, Box::new(inspector_state));
            }

            result
        }

        pub fn start_picking(&mut self) {
            self.pick_depth = Some(0.0);
        }

        pub fn is_picking(&self) -> bool {
            self.pick_depth.is_some()
        }

        fn render_inspector_states(
            &mut self,
            window: &mut Window,
            cx: &mut Context<Self>,
        ) -> Vec<AnyElement> {
            let mut elements = Vec::new();
            if let Some(active_element) = self.active_element.take() {
                for (type_id, state) in &active_element.states {
                    if let Some(render_inspector) = cx
                        .inspector_element_registry
                        .renderers_by_type_id
                        .remove(&type_id)
                    {
                        let mut element = (render_inspector)(
                            active_element.id.clone(),
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

                self.active_element = Some(active_element);
            }

            elements
        }
    }

    impl Render for Inspector {
        fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
            if let Some(inspector_renderer) = cx.inspector_renderer.take() {
                let rendered_inspector_states = self.render_inspector_states(window, cx);
                let result = inspector_renderer(
                    self.active_element.as_ref().map(|e| &e.id),
                    rendered_inspector_states,
                    window,
                    cx,
                );
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
