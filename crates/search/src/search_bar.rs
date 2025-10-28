use editor::{Editor, EditorElement, EditorStyle};
use gpui::{Action, Entity, FocusHandle, Hsla, IntoElement, TextStyle};
use settings::Settings;
use theme::ThemeSettings;
use ui::{IconButton, IconButtonShape};
use ui::{Tooltip, prelude::*};

pub(super) enum ActionButtonState {
    Disabled,
    Toggled,
}

pub(super) fn render_action_button(
    id_prefix: &'static str,
    icon: ui::IconName,
    button_state: Option<ActionButtonState>,
    tooltip: &'static str,
    action: &'static dyn Action,
    focus_handle: FocusHandle,
) -> impl IntoElement {
    IconButton::new(
        SharedString::from(format!("{id_prefix}-{}", action.name())),
        icon,
    )
    .shape(IconButtonShape::Square)
    .on_click({
        let focus_handle = focus_handle.clone();
        move |_, window, cx| {
            if !focus_handle.is_focused(window) {
                window.focus(&focus_handle);
            }
            window.dispatch_action(action.boxed_clone(), cx)
        }
    })
    .tooltip(move |_window, cx| Tooltip::for_action_in(tooltip, action, &focus_handle, cx))
    .when_some(button_state, |this, state| match state {
        ActionButtonState::Toggled => this.toggle_state(true),
        ActionButtonState::Disabled => this.disabled(true),
    })
}

pub(crate) fn input_base_styles(border_color: Hsla, map: impl FnOnce(Div) -> Div) -> Div {
    h_flex()
        .map(map)
        .min_w_32()
        .h_8()
        .pl_2()
        .pr_1()
        .py_1()
        .border_1()
        .border_color(border_color)
        .rounded_md()
}

pub(crate) fn render_text_input(
    editor: &Entity<Editor>,
    color_override: Option<Color>,
    app: &App,
) -> impl IntoElement {
    let (color, use_syntax) = if editor.read(app).read_only(app) {
        (app.theme().colors().text_disabled, false)
    } else {
        match color_override {
            Some(color_override) => (color_override.color(app), false),
            None => (app.theme().colors().text, true),
        }
    };

    let settings = ThemeSettings::get_global(app);
    let text_style = TextStyle {
        color,
        font_family: settings.buffer_font.family.clone(),
        font_features: settings.buffer_font.features.clone(),
        font_fallbacks: settings.buffer_font.fallbacks.clone(),
        font_size: rems(0.875).into(),
        font_weight: settings.buffer_font.weight,
        line_height: relative(1.3),
        ..TextStyle::default()
    };

    let mut editor_style = EditorStyle {
        background: app.theme().colors().toolbar_background,
        local_player: app.theme().players().local(),
        text: text_style,
        ..EditorStyle::default()
    };
    if use_syntax {
        editor_style.syntax = app.theme().syntax().clone();
    }

    EditorElement::new(editor, editor_style)
}
