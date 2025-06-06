//! The main prompt panel for the CodeOrbit extension.
//! 
//! This module implements the UI for the CodeOrbit prompt panel where users
//! can interact with the AI assistant.

use gpui::*;
use crate::core::Result;

/// The main prompt panel component.
pub struct PromptPanel {
    input: String,
    output: String,
    is_visible: bool,
}

impl PromptPanel {
    /// Creates a new instance of the prompt panel.
    pub fn new() -> Self {
        Self {
            input: String::new(),
            output: String::new(),
            is_visible: false,
        }
    }

    /// Shows the prompt panel.
    pub fn show(&mut self) {
        self.is_visible = true;
    }

    /// Hides the prompt panel.
    pub fn hide(&mut self) {
        self.is_visible = false;
    }

    /// Toggles the visibility of the prompt panel.
    pub fn toggle_visibility(&mut self) {
        self.is_visible = !self.is_visible;
    }

    /// Handles user input.
    pub fn handle_input(&mut self, input: &str) -> Result<()> {
        self.input = input.to_string();
        // TODO: Process the input through the orchestrator
        Ok(())
    }

    /// Updates the output display.
    pub fn update_output(&mut self, output: &str) {
        self.output = output.to_string();
    }

    /// Renders the prompt panel.
    pub fn render(&self, cx: &mut WindowContext) -> impl IntoElement {
        if !self.is_visible {
            return div();
        }

        div()
            .id("codeorbit-prompt-panel")
            .fixed()
            .bottom_0()
            .right_0()
            .w_96()
            .h_96()
            .bg(rgba(0.1, 0.1, 0.1, 0.95))
            .rounded_lg()
            .shadow_lg()
            .p_4()
            .flex()
            .flex_col()
            .child(
                div()
                    .flex_1()
                    .overflow_auto()
                    .child(div().text(&self.output))
            )
            .child(
                div()
                    .mt_4()
                    .child(
                        textarea()
                            .w_full()
                            .p_2()
                            .bg(rgba(0.2, 0.2, 0.2, 1.0))
                            .text_color(rgba(1.0, 1.0, 1.0, 1.0))
                            .rounded()
                            .placeholder("Ask CodeOrbit...")
                            .value(&self.input)
                            .on_input(move |_, input, cx| {
                                // TODO: Handle input
                                cx.notify();
                            })
                    )
            )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::TestAppContext;

    #[test]
    fn test_prompt_panel_visibility() {
        let mut panel = PromptPanel::new();
        assert!(!panel.is_visible);
        
        panel.show();
        assert!(panel.is_visible);
        
        panel.hide();
        assert!(!panel.is_visible);
        
        panel.toggle_visibility();
        assert!(panel.is_visible);
        
        panel.toggle_visibility();
        assert!(!panel.is_visible);
    }
    
    #[test]
    fn test_handle_input() {
        let mut panel = PromptPanel::new();
        let input = "Test input";
        
        assert_eq!(panel.input, "");
        panel.handle_input(input).unwrap();
        assert_eq!(panel.input, input);
    }
    
    #[test]
    fn test_update_output() {
        let mut panel = PromptPanel::new();
        let output = "Test output";
        
        assert_eq!(panel.output, "");
        panel.update_output(output);
        assert_eq!(panel.output, output);
    }
}
