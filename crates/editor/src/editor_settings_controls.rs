use gpui::AppContext;
use project::project_settings::{InlineBlameSettings, ProjectSettings};
use settings::{EditableSettingControl, Settings};
use theme::ThemeSettings;
use ui::{prelude::*, CheckboxWithLabel, NumericStepper};

#[derive(IntoElement)]
pub struct EditorSettingsControls {}

impl EditorSettingsControls {
    pub fn new() -> Self {
        Self {}
    }
}

impl RenderOnce for EditorSettingsControls {
    fn render(self, cx: &mut WindowContext) -> impl IntoElement {
        v_flex()
            .gap_1()
            .child(BufferFontSizeControl::new(cx))
            .child(InlineGitBlameControl::new(cx))
    }
}

#[derive(IntoElement)]
struct BufferFontSizeControl(Pixels);

impl EditableSettingControl for BufferFontSizeControl {
    type Value = Pixels;
    type Settings = ThemeSettings;

    fn name(&self) -> SharedString {
        "Buffer Font Size".into()
    }

    fn new(cx: &AppContext) -> Self {
        let settings = ThemeSettings::get_global(cx);

        Self(settings.buffer_font_size)
    }

    fn apply(settings: &mut <Self::Settings as Settings>::FileContent, value: Self::Value) {
        settings.buffer_font_size = Some(value.into());
    }
}

impl RenderOnce for BufferFontSizeControl {
    fn render(self, _cx: &mut WindowContext) -> impl IntoElement {
        let value = self.0;

        h_flex()
            .gap_2()
            .child(Icon::new(IconName::FontSize))
            .child(NumericStepper::new(
                self.0.to_string(),
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
struct InlineGitBlameControl(bool);

impl EditableSettingControl for InlineGitBlameControl {
    type Value = bool;
    type Settings = ProjectSettings;

    fn name(&self) -> SharedString {
        "Inline Git Blame".into()
    }

    fn new(cx: &AppContext) -> Self {
        let settings = ProjectSettings::get_global(cx);
        Self(settings.git.inline_blame_enabled())
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
    fn render(self, _cx: &mut WindowContext) -> impl IntoElement {
        let value = self.0;

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
