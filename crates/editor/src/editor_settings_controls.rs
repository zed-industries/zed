use gpui::{AppContext, FontWeight};
use project::project_settings::{InlineBlameSettings, ProjectSettings};
use settings::{EditableSettingControl, Settings};
use theme::ThemeSettings;
use ui::{
    prelude::*, CheckboxWithLabel, ContextMenu, DropdownMenu, NumericStepper, SettingsContainer,
    SettingsGroup,
};

#[derive(IntoElement)]
pub struct EditorSettingsControls {}

impl EditorSettingsControls {
    pub fn new() -> Self {
        Self {}
    }
}

impl RenderOnce for EditorSettingsControls {
    fn render(self, _cx: &mut WindowContext) -> impl IntoElement {
        SettingsContainer::new()
            .child(
                SettingsGroup::new("Font")
                    .child(BufferFontSizeControl)
                    .child(BufferFontWeightControl),
            )
            .child(SettingsGroup::new("Editor").child(InlineGitBlameControl))
    }
}

#[derive(IntoElement)]
struct BufferFontSizeControl;

impl EditableSettingControl for BufferFontSizeControl {
    type Value = Pixels;
    type Settings = ThemeSettings;

    fn name(&self) -> SharedString {
        "Buffer Font Size".into()
    }

    fn read(cx: &AppContext) -> Self::Value {
        let settings = ThemeSettings::get_global(cx);
        settings.buffer_font_size
    }

    fn apply(settings: &mut <Self::Settings as Settings>::FileContent, value: Self::Value) {
        settings.buffer_font_size = Some(value.into());
    }
}

impl RenderOnce for BufferFontSizeControl {
    fn render(self, cx: &mut WindowContext) -> impl IntoElement {
        let value = Self::read(cx);

        h_flex()
            .gap_2()
            .child(Icon::new(IconName::FontSize))
            .child(NumericStepper::new(
                value.to_string(),
                move |_, cx| {
                    Self::write(value - px(1.), cx);
                },
                move |_, cx| {
                    Self::write(value + px(1.), cx);
                },
            ))
    }
}

#[derive(IntoElement)]
struct BufferFontWeightControl;

impl EditableSettingControl for BufferFontWeightControl {
    type Value = FontWeight;
    type Settings = ThemeSettings;

    fn name(&self) -> SharedString {
        "Buffer Font Weight".into()
    }

    fn read(cx: &AppContext) -> Self::Value {
        let settings = ThemeSettings::get_global(cx);
        settings.buffer_font.weight
    }

    fn apply(settings: &mut <Self::Settings as Settings>::FileContent, value: Self::Value) {
        settings.buffer_font_weight = Some(value.0);
    }
}

impl RenderOnce for BufferFontWeightControl {
    fn render(self, cx: &mut WindowContext) -> impl IntoElement {
        let value = Self::read(cx);

        h_flex()
            .gap_2()
            .child(Icon::new(IconName::FontWeight))
            .child(DropdownMenu::new(
                "buffer-font-weight",
                value.0.to_string(),
                ContextMenu::build(cx, |mut menu, _cx| {
                    for weight in FontWeight::ALL {
                        menu = menu.custom_entry(
                            move |_cx| Label::new(weight.0.to_string()).into_any_element(),
                            {
                                move |cx| {
                                    Self::write(weight, cx);
                                }
                            },
                        )
                    }

                    menu
                }),
            ))
    }
}

#[derive(IntoElement)]
struct InlineGitBlameControl;

impl EditableSettingControl for InlineGitBlameControl {
    type Value = bool;
    type Settings = ProjectSettings;

    fn name(&self) -> SharedString {
        "Inline Git Blame".into()
    }

    fn read(cx: &AppContext) -> Self::Value {
        let settings = ProjectSettings::get_global(cx);
        settings.git.inline_blame_enabled()
    }

    fn apply(settings: &mut <Self::Settings as Settings>::FileContent, value: Self::Value) {
        if let Some(inline_blame) = settings.git.inline_blame.as_mut() {
            inline_blame.enabled = value;
        } else {
            settings.git.inline_blame = Some(InlineBlameSettings {
                enabled: false,
                ..Default::default()
            });
        }
    }
}

impl RenderOnce for InlineGitBlameControl {
    fn render(self, cx: &mut WindowContext) -> impl IntoElement {
        let value = Self::read(cx);

        CheckboxWithLabel::new(
            "inline-git-blame",
            Label::new(self.name()),
            value.into(),
            |selection, cx| {
                Self::write(
                    match selection {
                        Selection::Selected => true,
                        Selection::Unselected | Selection::Indeterminate => false,
                    },
                    cx,
                );
            },
        )
    }
}
