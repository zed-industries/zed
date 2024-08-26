use editor::{Editor, EditorElement, EditorStyle};
use gpui::{Render, Subscription, TextStyle, View, ViewContext};
use settings::Settings;
use theme::ThemeSettings;
use ui::prelude::*;

pub struct Console {
    console: View<Editor>,
    query_bar: View<Editor>,
    _subscriptions: Vec<Subscription>,
}

impl Console {
    pub fn new(cx: &mut ViewContext<Self>) -> Self {
        let console = cx.new_view(|cx| {
            let mut editor = Editor::multi_line(cx);
            editor.move_to_end(&editor::actions::MoveToEnd, cx);
            editor.set_read_only(true);
            editor.set_show_gutter(false, cx);
            editor.set_show_inline_completions(false);
            editor
        });

        let query_bar = cx.new_view(|cx| Editor::single_line(cx));

        let _subscriptions = vec![];

        Self {
            console,
            query_bar,
            _subscriptions,
        }
    }

    pub fn add_message(&mut self, message: &str, cx: &mut ViewContext<Self>) {
        self.console.update(cx, |console, cx| {
            console.set_read_only(false);
            console.move_to_end(&editor::actions::MoveToEnd, cx);
            console.insert(format!("{}\n", message).as_str(), cx);
            console.set_read_only(true);
        });

        cx.notify();
    }

    fn render_console(&self, cx: &ViewContext<Self>) -> impl IntoElement {
        let settings = ThemeSettings::get_global(cx);
        let text_style = TextStyle {
            color: if self.console.read(cx).read_only(cx) {
                cx.theme().colors().text_disabled
            } else {
                cx.theme().colors().text
            },
            font_family: settings.buffer_font.family.clone(),
            font_features: settings.buffer_font.features.clone(),
            font_size: rems(0.875).into(),
            font_weight: settings.buffer_font.weight,
            line_height: relative(1.3),
            ..Default::default()
        };

        EditorElement::new(
            &self.console,
            EditorStyle {
                background: cx.theme().colors().editor_background,
                local_player: cx.theme().players().local(),
                text: text_style,
                ..Default::default()
            },
        )
    }

    fn render_query_bar(&self, cx: &ViewContext<Self>) -> impl IntoElement {
        let settings = ThemeSettings::get_global(cx);
        let text_style = TextStyle {
            color: if self.console.read(cx).read_only(cx) {
                cx.theme().colors().text_disabled
            } else {
                cx.theme().colors().text
            },
            font_family: settings.buffer_font.family.clone(),
            font_features: settings.buffer_font.features.clone(),
            font_size: rems(0.875).into(),
            font_weight: settings.buffer_font.weight,
            line_height: relative(1.3),
            ..Default::default()
        };

        EditorElement::new(
            &self.query_bar,
            EditorStyle {
                background: cx.theme().colors().editor_background,
                local_player: cx.theme().players().local(),
                text: text_style,
                ..Default::default()
            },
        )
    }
}

impl Render for Console {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        v_flex()
            .size_full()
            .id("Debugger Console")
            .child(self.render_console(cx))
            .child(
                div()
                    .child(self.render_query_bar(cx))
                    .pt(Spacing::XSmall.rems(cx)),
            )
            .border_2()
    }
}
