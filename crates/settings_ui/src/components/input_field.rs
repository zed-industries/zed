use std::rc::Rc;

use editor::Editor;
use gpui::{AnyElement, ElementId, Focusable, TextStyleRefinement};
use settings::Settings as _;
use theme::ThemeSettings;
use ui::{Tooltip, prelude::*, rems};

#[derive(IntoElement)]
pub struct SettingsInputField {
    id: Option<ElementId>,
    initial_text: Option<String>,
    placeholder: Option<&'static str>,
    confirm: Option<Rc<dyn Fn(Option<String>, &mut Window, &mut App)>>,
    tab_index: Option<isize>,
    use_buffer_font: bool,
    display_confirm_button: bool,
    display_clear_button: bool,
    clear_on_confirm: bool,
    action_slot: Option<AnyElement>,
    color: Option<Color>,
}

impl SettingsInputField {
    pub fn new() -> Self {
        Self {
            id: None,
            initial_text: None,
            placeholder: None,
            confirm: None,
            tab_index: None,
            use_buffer_font: false,
            display_confirm_button: false,
            display_clear_button: false,
            clear_on_confirm: false,
            action_slot: None,
            color: None,
        }
    }

    pub fn with_id(mut self, id: impl Into<ElementId>) -> Self {
        self.id = Some(id.into());
        self
    }

    pub fn with_initial_text(mut self, initial_text: String) -> Self {
        self.initial_text = Some(initial_text);
        self
    }

    pub fn with_placeholder(mut self, placeholder: &'static str) -> Self {
        self.placeholder = Some(placeholder);
        self
    }

    pub fn on_confirm(
        mut self,
        confirm: impl Fn(Option<String>, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.confirm = Some(Rc::new(confirm));
        self
    }

    pub fn display_confirm_button(mut self) -> Self {
        self.display_confirm_button = true;
        self
    }

    pub fn display_clear_button(mut self) -> Self {
        self.display_clear_button = true;
        self
    }

    pub fn clear_on_confirm(mut self) -> Self {
        self.clear_on_confirm = true;
        self
    }

    pub fn action_slot(mut self, action: impl IntoElement) -> Self {
        self.action_slot = Some(action.into_any_element());
        self
    }

    pub(crate) fn tab_index(mut self, arg: isize) -> Self {
        self.tab_index = Some(arg);
        self
    }

    pub fn with_buffer_font(mut self) -> Self {
        self.use_buffer_font = true;
        self
    }

    pub fn color(mut self, color: Color) -> Self {
        self.color = Some(color);
        self
    }
}

impl RenderOnce for SettingsInputField {
    fn render(self, window: &mut Window, cx: &mut App) -> impl ui::IntoElement {
        let settings = ThemeSettings::get_global(cx);
        let use_buffer_font = self.use_buffer_font;
        let color = self.color.map(|c| c.color(cx));
        let styles = TextStyleRefinement {
            font_family: use_buffer_font.then(|| settings.buffer_font.family.clone()),
            font_size: use_buffer_font.then(|| rems(0.75).into()),
            color,
            ..Default::default()
        };

        let editor = if let Some(id) = self.id {
            window.use_keyed_state(id, cx, {
                let initial_text = self.initial_text.clone();
                let placeholder = self.placeholder;
                move |window, cx| {
                    let mut editor = Editor::single_line(window, cx);
                    if let Some(text) = initial_text {
                        editor.set_text(text, window, cx);
                    }

                    if let Some(placeholder) = placeholder {
                        editor.set_placeholder_text(placeholder, window, cx);
                    }
                    editor.set_text_style_refinement(styles);
                    editor
                }
            })
        } else {
            window.use_state(cx, {
                let initial_text = self.initial_text.clone();
                let placeholder = self.placeholder;
                move |window, cx| {
                    let mut editor = Editor::single_line(window, cx);
                    if let Some(text) = initial_text {
                        editor.set_text(text, window, cx);
                    }

                    if let Some(placeholder) = placeholder {
                        editor.set_placeholder_text(placeholder, window, cx);
                    }
                    editor.set_text_style_refinement(styles);
                    editor
                }
            })
        };

        // When settings change externally (e.g. editing settings.json), the page
        // re-renders but use_keyed_state returns the cached editor with stale text.
        // Reconcile with the expected initial_text when the editor is not focused,
        // so we don't clobber what the user is actively typing.
        if let Some(initial_text) = &self.initial_text {
            let current_text = editor.read(cx).text(cx);
            if current_text != *initial_text && !editor.read(cx).is_focused(window) {
                editor.update(cx, |editor, cx| {
                    editor.set_text(initial_text.clone(), window, cx);
                });
            }
        }

        let weak_editor = editor.downgrade();
        let weak_editor_for_button = editor.downgrade();
        let weak_editor_for_clear = editor.downgrade();

        let clear_on_confirm = self.clear_on_confirm;
        let clear_on_confirm_for_button = self.clear_on_confirm;

        let theme_colors = cx.theme().colors();

        let display_confirm_button = self.display_confirm_button;
        let display_clear_button = self.display_clear_button;
        let confirm_for_button = self.confirm.clone();
        let is_editor_empty = editor.read(cx).text(cx).trim().is_empty();
        let is_editor_focused = editor.read(cx).is_focused(window);

        h_flex()
            .group("settings-input-field-editor")
            .relative()
            .py_1()
            .px_2()
            .h_8()
            .min_w_64()
            .rounded_md()
            .border_1()
            .border_color(theme_colors.border)
            .bg(theme_colors.editor_background)
            .when_some(self.tab_index, |this, tab_index| {
                let focus_handle = editor.focus_handle(cx).tab_index(tab_index).tab_stop(true);
                this.track_focus(&focus_handle)
                    .focus(|s| s.border_color(theme_colors.border_focused))
            })
            .child(editor)
            .child(
                h_flex()
                    .absolute()
                    .top_1()
                    .right_1()
                    .invisible()
                    .when(is_editor_focused, |this| this.visible())
                    .group_hover("settings-input-field-editor", |this| this.visible())
                    .when(
                        display_clear_button && !is_editor_empty && is_editor_focused,
                        |this| {
                            this.child(
                                IconButton::new("clear-button", IconName::Close)
                                    .icon_size(IconSize::Small)
                                    .icon_color(Color::Muted)
                                    .tooltip(Tooltip::text("Clear"))
                                    .on_click(move |_, window, cx| {
                                        let Some(editor) = weak_editor_for_clear.upgrade() else {
                                            return;
                                        };
                                        editor.update(cx, |editor, cx| {
                                            editor.set_text("", window, cx);
                                        });
                                    }),
                            )
                        },
                    )
                    .when(
                        display_confirm_button && !is_editor_empty && is_editor_focused,
                        |this| {
                            this.child(
                                IconButton::new("confirm-button", IconName::Check)
                                    .icon_size(IconSize::Small)
                                    .icon_color(Color::Success)
                                    .tooltip(Tooltip::text("Enter to Confirm"))
                                    .on_click(move |_, window, cx| {
                                        let Some(confirm) = confirm_for_button.as_ref() else {
                                            return;
                                        };
                                        let Some(editor) = weak_editor_for_button.upgrade() else {
                                            return;
                                        };
                                        let new_value =
                                            editor.read_with(cx, |editor, cx| editor.text(cx));
                                        let new_value =
                                            (!new_value.is_empty()).then_some(new_value);
                                        confirm(new_value, window, cx);
                                        if clear_on_confirm_for_button {
                                            editor.update(cx, |editor, cx| {
                                                editor.set_text("", window, cx);
                                            });
                                        }
                                    }),
                            )
                        },
                    )
                    .when_some(self.action_slot, |this, action| this.child(action)),
            )
            .when_some(self.confirm, |this, confirm| {
                this.on_action::<menu::Confirm>({
                    move |_, window, cx| {
                        let Some(editor) = weak_editor.upgrade() else {
                            return;
                        };
                        let new_value = editor.read_with(cx, |editor, cx| editor.text(cx));
                        let new_value = (!new_value.is_empty()).then_some(new_value);
                        confirm(new_value, window, cx);
                        if clear_on_confirm {
                            editor.update(cx, |editor, cx| {
                                editor.set_text("", window, cx);
                            });
                        }
                    }
                })
            })
    }
}
