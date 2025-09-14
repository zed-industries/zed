use editor::{Editor, EditorElement, EditorStyle};
use gpui::{
    App, Context, Entity, Render, Styled, TextStyle, Window, prelude::*,
};
use theme::{Theme, ThemeSettings};
use ui::prelude::*;

/// A component that renders a preview of a theme showing sample code and UI elements
pub struct ThemePreview {
    editor: Entity<Editor>,
    theme: Arc<Theme>,
}

impl ThemePreview {
    pub fn new(theme: Arc<Theme>, window: &mut Window, cx: &mut Context<Self>) -> Self {
        let editor = cx.new(|cx| {
            let mut editor = Editor::single_line(window, cx);
            
            // Set sample code content
            let sample_code = r#"fn main() {
    let message = "Hello, World!";
    println!("{}", message);
    
    // This is a comment
    let numbers = vec![1, 2, 3, 4, 5];
    
    for num in numbers {
        if num % 2 == 0 {
            println!("Even: {}", num);
        } else {
            println!("Odd: {}", num);
        }
    }
}"#;
            
            editor.set_text(sample_code, window, cx);
            editor.set_read_only(true, cx);
            editor
        });

        Self { editor, theme }
    }
}

impl Render for ThemePreview {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let settings = ThemeSettings::get_global(cx);
        let text_style = TextStyle {
            color: self.theme.colors().text,
            font_family: settings.ui_font.family.clone(),
            font_features: settings.ui_font.features.clone(),
            font_fallbacks: settings.ui_font.fallbacks.clone(),
            font_size: rems(0.6).into(),
            font_weight: settings.ui_font.weight,
            line_height: relative(1.3),
            ..Default::default()
        };

        let editor_style = EditorStyle {
            background: self.theme.colors().editor_background,
            local_player: self.theme.players().local(),
            text: text_style,
            ..Default::default()
        };

        v_flex()
            .w(rems(14.))
            .h(rems(10.))
            .bg(self.theme.colors().background)
            .border_1()
            .border_color(self.theme.colors().border)
            .rounded_md()
            .overflow_hidden()
            .child(
                // Header bar
                h_flex()
                    .w_full()
                    .h_6()
                    .bg(self.theme.colors().surface_background)
                    .border_b_1()
                    .border_color(self.theme.colors().border_variant)
                    .px_2()
                    .items_center()
                    .gap_2()
                    .child(
                        div()
                            .w_2()
                            .h_2()
                            .bg(self.theme.colors().error)
                            .rounded_full()
                    )
                    .child(
                        div()
                            .w_2()
                            .h_2()
                            .bg(self.theme.colors().warning)
                            .rounded_full()
                    )
                    .child(
                        div()
                            .w_2()
                            .h_2()
                            .bg(self.theme.colors().success)
                            .rounded_full()
                    )
                    .child(
                        Label::new("theme-preview.rs")
                            .size(LabelSize::XSmall)
                            .color(self.theme.colors().text_muted)
                    )
            )
            .child(
                // Editor content
                EditorElement::new(&self.editor, editor_style)
                    .flex_1()
                    .p_2()
            )
            .child(
                // Status bar
                h_flex()
                    .w_full()
                    .h_4()
                    .bg(self.theme.colors().surface_background)
                    .border_t_1()
                    .border_color(self.theme.colors().border_variant)
                    .px_2()
                    .items_center()
                    .justify_between()
                    .child(
                        Label::new("Rust")
                            .size(LabelSize::XSmall)
                            .color(self.theme.colors().text_muted)
                    )
                    .child(
                        Label::new("Ln 1, Col 1")
                            .size(LabelSize::XSmall)
                            .color(self.theme.colors().text_muted)
                    )
            )
    }
}