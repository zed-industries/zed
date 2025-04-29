//! Example usage of the StatefulComponent trait for a checkbox component

use component::{Component, ComponentScope};
use component_preview::stateful_component::StatefulComponent;
use gpui::{AnyElement, App, Entity, IntoElement, ToggleButton, Window, div, prelude::*};

/// Define the component as usual
pub struct Checkbox;

impl Component for Checkbox {
    fn scope() -> ComponentScope {
        ComponentScope::Input
    }
    
    fn name() -> &'static str {
        "ui::Checkbox"
    }
    
    fn description() -> Option<&'static str> {
        Some("A checkbox component for toggling boolean values")
    }
    
    // Instead of implementing preview directly, we'll implement StatefulComponent
    // which will automatically provide the preview implementation
}

/// Define the data needed for our preview
pub struct CheckboxPreviewData {
    pub checked: bool,
    pub label: String,
}

/// Implement the StatefulComponent trait to provide stateful previews
impl StatefulComponent for Checkbox {
    type PreviewData = CheckboxPreviewData;
    
    // Create the initial preview data
    fn create_preview_data(window: &mut Window, cx: &mut App) -> Entity<Self::PreviewData> {
        cx.new(|_| CheckboxPreviewData {
            checked: false,
            label: "Toggle me".to_string(),
        })
    }
    
    // Render the component using the preview data
    fn render_stateful_preview(
        preview_data: &Entity<Self::PreviewData>,
        window: &mut Window, 
        cx: &mut App
    ) -> Option<AnyElement> {
        // We can safely read our strongly-typed preview data
        let is_checked = preview_data.read(cx).checked;
        let label = preview_data.read(cx).label.clone();
        
        // Create the example component
        let checkbox = ToggleButton::new("checkbox-example", is_checked)
            .on_toggle(move |_toggled, window, cx| {
                // Update the preview data when toggled
                preview_data.update(cx, |data, cx| {
                    data.checked = !data.checked;
                    cx.notify();
                });
            })
            .label(label);
            
        // Return our example wrapped in a container
        Some(
            div()
                .p_4()
                .child(checkbox)
                .into_any_element()
        )
    }
}

// Now when this component is previewed, it will automatically maintain state across renders
// No need to do any special thread-local or global state management!