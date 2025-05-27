use std::sync::Arc;

use gpui::{App, FontFeatures, FontWeight};
use project::project_settings::{InlineBlameSettings, ProjectSettings};
use settings::{EditableSettingControl, Settings};
use theme::{FontFamilyCache, ThemeSettings};
use ui::{
    CheckboxWithLabel, ContextMenu, DropdownMenu, NumericStepper, SettingsContainer, SettingsGroup,
    prelude::*,
};

use crate::EditorSettings;

#[derive(IntoElement)]
pub struct EditorSettingsControls {}

impl Default for EditorSettingsControls {
    fn default() -> Self {
        Self::new()
    }
}

impl EditorSettingsControls {
    pub fn new() -> Self {
        Self {}
    }
}

impl RenderOnce for EditorSettingsControls {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        SettingsContainer::new()
            .child(
                SettingsGroup::new("Font")
                    .child(
                        h_flex()
                            .gap_2()
                            .justify_between()
                            .child(BufferFontFamilyControl)
                            .child(BufferFontWeightControl),
                    )
                    .child(BufferFontSizeControl)
                    .child(BufferFontLigaturesControl),
            )
            .child(SettingsGroup::new("Editor").child(InlineGitBlameControl))
            .child(
                SettingsGroup::new("Gutter").child(
                    h_flex()
                        .gap_2()
                        .justify_between()
                        .child(LineNumbersControl)
                        .child(RelativeLineNumbersControl),
                ),
            )
    }
}

#[derive(IntoElement)]
struct BufferFontFamilyControl;

impl EditableSettingControl for BufferFontFamilyControl {
    type Value = SharedString;
    type Settings = ThemeSettings;

    fn name(&self) -> SharedString {
        "Buffer Font Family".into()
    }

    fn read(cx: &App) -> Self::Value {
        let settings = ThemeSettings::get_global(cx);
        settings.buffer_font.family.clone()
    }

    fn apply(
        settings: &mut <Self::Settings as Settings>::FileContent,
        value: Self::Value,
        _cx: &App,
    ) {
        settings.buffer_font_family = Some(value.to_string());
    }
}

impl RenderOnce for BufferFontFamilyControl {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let value = Self::read(cx);

        h_flex()
            .gap_2()
            .child(Icon::new(IconName::Font))
            .child(DropdownMenu::new(
                "buffer-font-family",
                value.clone(),
                ContextMenu::build(window, cx, |mut menu, _, cx| {
                    let font_family_cache = FontFamilyCache::global(cx);

                    for font_name in font_family_cache.list_font_families(cx) {
                        menu = menu.custom_entry(
                            {
                                let font_name = font_name.clone();
                                move |_window, _cx| Label::new(font_name.clone()).into_any_element()
                            },
                            {
                                let font_name = font_name.clone();
                                move |_window, cx| {
                                    Self::write(font_name.clone(), cx);
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
struct BufferFontSizeControl;

impl EditableSettingControl for BufferFontSizeControl {
    type Value = Pixels;
    type Settings = ThemeSettings;

    fn name(&self) -> SharedString {
        "Buffer Font Size".into()
    }

    fn read(cx: &App) -> Self::Value {
        ThemeSettings::get_global(cx).buffer_font_size(cx)
    }

    fn apply(
        settings: &mut <Self::Settings as Settings>::FileContent,
        value: Self::Value,
        _cx: &App,
    ) {
        settings.buffer_font_size = Some(value.into());
    }
}

impl RenderOnce for BufferFontSizeControl {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let value = Self::read(cx);

        h_flex()
            .gap_2()
            .child(Icon::new(IconName::FontSize))
            .child(NumericStepper::new(
                "buffer-font-size",
                value.to_string(),
                move |_, _, cx| {
                    Self::write(value - px(1.), cx);
                },
                move |_, _, cx| {
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

    fn read(cx: &App) -> Self::Value {
        let settings = ThemeSettings::get_global(cx);
        settings.buffer_font.weight
    }

    fn apply(
        settings: &mut <Self::Settings as Settings>::FileContent,
        value: Self::Value,
        _cx: &App,
    ) {
        settings.buffer_font_weight = Some(value.0);
    }
}

impl RenderOnce for BufferFontWeightControl {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let value = Self::read(cx);

        h_flex()
            .gap_2()
            .child(Icon::new(IconName::FontWeight))
            .child(DropdownMenu::new(
                "buffer-font-weight",
                value.0.to_string(),
                ContextMenu::build(window, cx, |mut menu, _window, _cx| {
                    for weight in FontWeight::ALL {
                        menu = menu.custom_entry(
                            move |_window, _cx| Label::new(weight.0.to_string()).into_any_element(),
                            {
                                move |_, cx| {
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
struct BufferFontLigaturesControl;

impl EditableSettingControl for BufferFontLigaturesControl {
    type Value = bool;
    type Settings = ThemeSettings;

    fn name(&self) -> SharedString {
        "Buffer Font Ligatures".into()
    }

    fn read(cx: &App) -> Self::Value {
        let settings = ThemeSettings::get_global(cx);
        settings
            .buffer_font
            .features
            .is_calt_enabled()
            .unwrap_or(true)
    }

    fn apply(
        settings: &mut <Self::Settings as Settings>::FileContent,
        value: Self::Value,
        _cx: &App,
    ) {
        let value = if value { 1 } else { 0 };

        let mut features = settings
            .buffer_font_features
            .as_ref()
            .map(|features| features.tag_value_list().to_vec())
            .unwrap_or_default();

        if let Some(calt_index) = features.iter().position(|(tag, _)| tag == "calt") {
            features[calt_index].1 = value;
        } else {
            features.push(("calt".into(), value));
        }

        settings.buffer_font_features = Some(FontFeatures(Arc::new(features)));
    }
}

impl RenderOnce for BufferFontLigaturesControl {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let value = Self::read(cx);

        CheckboxWithLabel::new(
            "buffer-font-ligatures",
            Label::new(self.name()),
            value.into(),
            |selection, _, cx| {
                Self::write(
                    match selection {
                        ToggleState::Selected => true,
                        ToggleState::Unselected | ToggleState::Indeterminate => false,
                    },
                    cx,
                );
            },
        )
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

    fn read(cx: &App) -> Self::Value {
        let settings = ProjectSettings::get_global(cx);
        settings.git.inline_blame_enabled()
    }

    fn apply(
        settings: &mut <Self::Settings as Settings>::FileContent,
        value: Self::Value,
        _cx: &App,
    ) {
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
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let value = Self::read(cx);

        CheckboxWithLabel::new(
            "inline-git-blame",
            Label::new(self.name()),
            value.into(),
            |selection, _, cx| {
                Self::write(
                    match selection {
                        ToggleState::Selected => true,
                        ToggleState::Unselected | ToggleState::Indeterminate => false,
                    },
                    cx,
                );
            },
        )
    }
}

#[derive(IntoElement)]
struct LineNumbersControl;

impl EditableSettingControl for LineNumbersControl {
    type Value = bool;
    type Settings = EditorSettings;

    fn name(&self) -> SharedString {
        "Line Numbers".into()
    }

    fn read(cx: &App) -> Self::Value {
        let settings = EditorSettings::get_global(cx);
        settings.gutter.line_numbers
    }

    fn apply(
        settings: &mut <Self::Settings as Settings>::FileContent,
        value: Self::Value,
        _cx: &App,
    ) {
        if let Some(gutter) = settings.gutter.as_mut() {
            gutter.line_numbers = Some(value);
        } else {
            settings.gutter = Some(crate::editor_settings::GutterContent {
                line_numbers: Some(value),
                ..Default::default()
            });
        }
    }
}

impl RenderOnce for LineNumbersControl {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let value = Self::read(cx);

        CheckboxWithLabel::new(
            "line-numbers",
            Label::new(self.name()),
            value.into(),
            |selection, _, cx| {
                Self::write(
                    match selection {
                        ToggleState::Selected => true,
                        ToggleState::Unselected | ToggleState::Indeterminate => false,
                    },
                    cx,
                );
            },
        )
    }
}

#[derive(IntoElement)]
struct RelativeLineNumbersControl;

impl EditableSettingControl for RelativeLineNumbersControl {
    type Value = bool;
    type Settings = EditorSettings;

    fn name(&self) -> SharedString {
        "Relative Line Numbers".into()
    }

    fn read(cx: &App) -> Self::Value {
        let settings = EditorSettings::get_global(cx);
        settings.relative_line_numbers
    }

    fn apply(
        settings: &mut <Self::Settings as Settings>::FileContent,
        value: Self::Value,
        _cx: &App,
    ) {
        settings.relative_line_numbers = Some(value);
    }
}

impl RenderOnce for RelativeLineNumbersControl {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let value = Self::read(cx);

        DropdownMenu::new(
            "relative-line-numbers",
            if value { "Relative" } else { "Ascending" },
            ContextMenu::build(window, cx, |menu, _window, _cx| {
                menu.custom_entry(
                    |_window, _cx| Label::new("Ascending").into_any_element(),
                    move |_, cx| Self::write(false, cx),
                )
                .custom_entry(
                    |_window, _cx| Label::new("Relative").into_any_element(),
                    move |_, cx| Self::write(true, cx),
                )
            }),
        )
    }
}
