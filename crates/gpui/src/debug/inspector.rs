use crate::{
    AnyElement, App, Bounds, Context, ElementId, GlobalElementId, InteractiveElement, IntoElement,
    ParentElement, Pixels, Render, SharedString, Style, StyleRefinement, Styled, Window, div, px,
    rgb, util::FluentBuilder,
};
use std::collections::HashMap;

/// Metadata about an element for inspection purposes
#[derive(Default, Clone)]
pub struct ElementMetadata {
    pub bounds: Option<Bounds<Pixels>>,
    pub style: Option<Style>,
    pub children: Vec<ElementId>,
    pub parent: Option<GlobalElementId>,
}

pub(crate) struct Inspector {
    selected_element: Option<GlobalElementId>,
    element_hover: Option<GlobalElementId>,
    expanded_elements: HashMap<GlobalElementId, bool>,
}

impl Default for Inspector {
    fn default() -> Self {
        Self {
            selected_element: None,
            element_hover: None,
            expanded_elements: HashMap::new(),
        }
    }
}

impl Render for Inspector {
    fn render(
        &mut self,
        window: &mut crate::Window,
        cx: &mut crate::Context<Self>,
    ) -> impl IntoElement {
        let mut has_info = false;
        let selected_element_info = if let Some(id) = &self.selected_element {
            has_info = true;
            self.get_element(window, id)
        } else {
            has_info = false;
            None
        };

        div()
            .id("GPUI_TOOLS_INSPECTOR")
            .flex()
            .flex_col()
            .size_full()
            .bg(rgb(0xf0f0f0))
            .p_4()
            .gap_4()
            .child(
                // Header
                div()
                    .flex()
                    .w_full()
                    .justify_between()
                    .pb_2()
                    .border_b_1()
                    .border_color(rgb(0xdddddd))
                    .child("GPUI Element Inspector"),
            )
            .child(
                // Element info section
                div()
                    .flex()
                    .flex_col()
                    .gap_2()
                    .when_some(selected_element_info.clone(), |this, info| {
                        this.child(
                            div()
                                .flex()
                                .flex_col()
                                .p_2()
                                .bg(rgb(0xffffff))
                                .border_1()
                                .border_color(rgb(0xdddddd))
                                .rounded_md()
                                .child(div().child(format!(
                                    "Element: {:?}",
                                    self.selected_element.as_ref().unwrap()
                                )))
                                .when_some(info.bounds, |this, bounds| {
                                    this.child(format!("Bounds: {:?}", bounds))
                                }),
                        )
                    })
                    .when(has_info, |this| {
                        this.child(
                            div()
                                .p_2()
                                .child("No element selected. Use the mouse to select an element."),
                        )
                    }),
            )
            .child(
                // Element style section
                div().flex().flex_col().gap_2().when_some(
                    selected_element_info,
                    |this, element| {
                        this.when_some(element.style.as_ref(), |this, style| {
                            this.child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .p_2()
                                    .bg(rgb(0xffffff))
                                    .border_1()
                                    .border_color(rgb(0xdddddd))
                                    .rounded_md()
                                    .child(div().child("Style Properties:"))
                                    .child(self.render_style_properties(style)),
                            )
                        })
                    },
                ),
            )
    }
}

impl Inspector {
    fn property_div(
        &self,
        name: impl Into<SharedString>,
        value: impl Into<Option<SharedString>>,
    ) -> Option<impl IntoElement> {
        if let Some(value) = value.into() {
            let property_string: SharedString = format!("{:?}", value.into()).into();

            Some(
                div()
                    .flex()
                    .gap_2()
                    .child(div().text_xs().text_color(rgb(0x666666)).child(name.into()))
                    .child(div().text_xs().child(property_string)),
            )
        } else {
            None
        }
    }

    /// Render the style properties of an element
    fn render_style_properties(&self, style: &Style) -> impl IntoElement {
        let width: SharedString = format!("{:?}", style.size.width).into();
        let height: SharedString = format!("{:?}", style.size.height).into();

        div()
            .flex()
            .flex_col()
            .gap_1()
            .px_2()
            .children(self.property_div("width", width))
            .children(self.property_div("height", height))
        // .children(self.property_div("background", style.background))
        // .children(self.property_div("color", style.text_color))
        // .children(self.property_div("font_size", style.font_size))
        // .children(self.property_div("font_weight", style.font_weight))
        // .children(self.property_div("padding", style.padding))
        // .children(self.property_div("margin", style.margin))
        // .children(self.property_div("border", style.border))
        // .children(self.property_div("border_color", style.border_color))
        // .children(self.property_div("border_radius", style.border_radius))
    }

    /// Get element metadata by GlobalElementId
    pub fn get_element(
        &self,
        window: &mut Window,
        id: &GlobalElementId,
    ) -> Option<ElementMetadata> {
        let mut result = None;
        window.with_element_state(id, |state: Option<&ElementMetadata>, _window| {
            result = state.cloned();
            ((), &result.unwrap_or_default())
        });
        result
    }

    /// Select an element for inspection
    pub fn select_element(&mut self, id: GlobalElementId) {
        self.selected_element = Some(id);
    }

    /// Set hover state for an element
    pub fn hover_element(&mut self, id: Option<GlobalElementId>) {
        self.element_hover = id;
    }

    /// Toggle expanded state of an element in the tree view
    pub fn toggle_expanded(&mut self, id: &GlobalElementId) {
        let is_expanded = self.expanded_elements.get(id).copied().unwrap_or(false);
        self.expanded_elements.insert(id.clone(), !is_expanded);
    }

    /// Check if an element is expanded in the tree view
    pub fn is_expanded(&self, id: &GlobalElementId) -> bool {
        self.expanded_elements.get(id).copied().unwrap_or(false)
    }

    /// Register an element for inspection
    pub fn register_element(
        window: &mut Window,
        id: &GlobalElementId,
        bounds: Bounds<Pixels>,
        style: Style,
        parent: Option<GlobalElementId>,
    ) {
        window.with_element_state(id, |existing: Option<ElementMetadata>, _window| {
            let mut metadata = existing.unwrap_or_default();
            metadata.bounds = Some(bounds);
            metadata.style = Some(style);
            metadata.parent = parent;
            ((), metadata)
        });
    }

    /// Add a child to a parent element's metadata
    pub fn register_child(window: &mut Window, parent_id: &GlobalElementId, child_id: ElementId) {
        window.with_element_state(parent_id, |existing: Option<ElementMetadata>, _window| {
            let mut metadata = existing.unwrap_or_default();
            if !metadata.children.contains(&child_id) {
                metadata.children.push(child_id);
            }
            ((), metadata)
        });
    }
}
