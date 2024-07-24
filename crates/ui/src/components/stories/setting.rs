use gpui::View;

use crate::prelude::*;

use crate::{
    LegacySettingsGroup, LegacySettingsMenu, SecondarySettingType, SettingLayout, SettingType,
    SettingsItem, ToggleType,
};

pub struct SettingStory {
    menus: Vec<(SharedString, View<LegacySettingsMenu>)>,
}

impl SettingStory {
    pub fn new() -> Self {
        Self { menus: Vec::new() }
    }

    pub fn init(cx: &mut ViewContext<Self>) -> Self {
        let mut story = Self::new();
        story.empty_menu(cx);
        story.editor_example(cx);
        story.menu_single_group(cx);
        story
    }
}

impl SettingStory {
    pub fn empty_menu(&mut self, cx: &mut ViewContext<Self>) {
        let menu = cx.new_view(|_cx| LegacySettingsMenu::new("Empty Menu"));

        self.menus.push(("Empty Menu".into(), menu));
    }

    pub fn menu_single_group(&mut self, cx: &mut ViewContext<Self>) {
        let theme_setting = SettingsItem::new(
            "theme-setting",
            "Theme".into(),
            SettingType::Dropdown,
            Some(cx.theme().name.clone().into()),
        )
        .layout(SettingLayout::Stacked);
        let high_contrast_setting = SettingsItem::new(
            "theme-contrast",
            "Use high contrast theme".into(),
            SettingType::Toggle(ToggleType::Checkbox),
            None,
        )
        .toggled(false);
        let appearance_setting = SettingsItem::new(
            "switch-appearance",
            "Match system appearance".into(),
            SettingType::ToggleAnd(SecondarySettingType::Dropdown),
            Some("When Dark".to_string().into()),
        )
        .layout(SettingLayout::FullLineJustified);

        let group = LegacySettingsGroup::new("Appearance")
            .add_setting(theme_setting)
            .add_setting(appearance_setting)
            .add_setting(high_contrast_setting);

        let menu = cx.new_view(|_cx| LegacySettingsMenu::new("Appearance").add_group(group));

        self.menus.push(("Single Group".into(), menu));
    }

    pub fn editor_example(&mut self, cx: &mut ViewContext<Self>) {
        let font_group = LegacySettingsGroup::new("Font")
            .add_setting(
                SettingsItem::new(
                    "font-family",
                    "Font".into(),
                    SettingType::Dropdown,
                    Some("Berkeley Mono".to_string().into()),
                )
                .icon(IconName::Font)
                .layout(SettingLayout::AutoWidth),
            )
            .add_setting(
                SettingsItem::new(
                    "font-weight",
                    "Font Weight".into(),
                    SettingType::Dropdown,
                    Some("400".to_string().into()),
                )
                .icon(IconName::FontWeight)
                .layout(SettingLayout::AutoWidth),
            )
            .add_setting(
                SettingsItem::new(
                    "font-size",
                    "Font Size".into(),
                    SettingType::Dropdown,
                    Some("14".to_string().into()),
                )
                .icon(IconName::FontSize)
                .layout(SettingLayout::AutoWidth),
            )
            .add_setting(
                SettingsItem::new(
                    "line-height",
                    "Line Height".into(),
                    SettingType::Dropdown,
                    Some("1.35".to_string().into()),
                )
                .icon(IconName::LineHeight)
                .layout(SettingLayout::AutoWidth),
            )
            .add_setting(
                SettingsItem::new(
                    "enable-ligatures",
                    "Enable Ligatures".into(),
                    SettingType::Toggle(ToggleType::Checkbox),
                    None,
                )
                .toggled(true),
            );

        let editor_group = LegacySettingsGroup::new("Editor")
            .add_setting(
                SettingsItem::new(
                    "show-indent-guides",
                    "Indent Guides".into(),
                    SettingType::Toggle(ToggleType::Checkbox),
                    None,
                )
                .toggled(true),
            )
            .add_setting(
                SettingsItem::new(
                    "show-git-blame",
                    "Git Blame".into(),
                    SettingType::Toggle(ToggleType::Checkbox),
                    None,
                )
                .toggled(false),
            );

        let gutter_group = LegacySettingsGroup::new("Gutter")
            .add_setting(
                SettingsItem::new(
                    "enable-git-hunks",
                    "Show Git Hunks".into(),
                    SettingType::Toggle(ToggleType::Checkbox),
                    None,
                )
                .toggled(true),
            )
            .add_setting(
                SettingsItem::new(
                    "show-line-numbers",
                    "Line Numbers".into(),
                    SettingType::ToggleAnd(SecondarySettingType::Dropdown),
                    Some("Ascending".to_string().into()),
                )
                .toggled(true)
                .layout(SettingLayout::FullLineJustified),
            );

        let scrollbar_group = LegacySettingsGroup::new("Scrollbar")
            .add_setting(
                SettingsItem::new(
                    "scrollbar-visibility",
                    "Show scrollbar when:".into(),
                    SettingType::Dropdown,
                    Some("Always Visible".to_string().into()),
                )
                .layout(SettingLayout::AutoWidth)
                .icon(IconName::Visible),
            )
            .add_setting(
                SettingsItem::new(
                    "show-diagnostic-markers",
                    "Diagnostic Markers".into(),
                    SettingType::Toggle(ToggleType::Checkbox),
                    None,
                )
                .toggled(true),
            )
            .add_setting(
                SettingsItem::new(
                    "show-git-markers",
                    "Git Status Markers".into(),
                    SettingType::Toggle(ToggleType::Checkbox),
                    None,
                )
                .toggled(false),
            )
            .add_setting(
                SettingsItem::new(
                    "show-selection-markers",
                    "Selection & Match Markers".into(),
                    SettingType::Toggle(ToggleType::Checkbox),
                    None,
                )
                .toggled(true),
            );

        let menu = cx.new_view(|_cx| {
            LegacySettingsMenu::new("Editor")
                .add_group(font_group)
                .add_group(editor_group)
                .add_group(gutter_group)
                .add_group(scrollbar_group)
        });

        self.menus.push(("Editor Example".into(), menu));
    }
}

impl Render for SettingStory {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .bg(cx.theme().colors().background)
            .text_color(cx.theme().colors().text)
            .children(self.menus.iter().map(|(name, menu)| {
                v_flex()
                    .p_2()
                    .gap_2()
                    .child(Headline::new(name.clone()).size(HeadlineSize::Medium))
                    .child(menu.clone())
            }))
    }
}
