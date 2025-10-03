use std::sync::Arc;

use editor::{EditorSettings, ShowMinimap};
use fs::Fs;
use fuzzy::{StringMatch, StringMatchCandidate};
use gpui::{
    Action, AnyElement, App, Context, FontFeatures, IntoElement, Pixels, SharedString, Task, Window,
};
use language::language_settings::{AllLanguageSettings, FormatOnSave};
use picker::{Picker, PickerDelegate};
use project::project_settings::ProjectSettings;
use settings::{Settings as _, update_settings_file};
use theme::{FontFamilyCache, FontFamilyName, ThemeSettings};
use ui::{
    ButtonLike, ListItem, ListItemSpacing, NumericStepper, PopoverMenu, SwitchField,
    ToggleButtonGroup, ToggleButtonGroupStyle, ToggleButtonSimple, ToggleState, Tooltip,
    prelude::*,
};

use crate::{ImportCursorSettings, ImportVsCodeSettings, SettingsImportState};

fn read_show_mini_map(cx: &App) -> ShowMinimap {
    editor::EditorSettings::get_global(cx).minimap.show
}

fn write_show_mini_map(show: ShowMinimap, cx: &mut App) {
    let fs = <dyn Fs>::global(cx);

    // This is used to speed up the UI
    // the UI reads the current values to get what toggle state to show on buttons
    // there's a slight delay if we just call update_settings_file so we manually set
    // the value here then call update_settings file to get around the delay
    let mut curr_settings = EditorSettings::get_global(cx).clone();
    curr_settings.minimap.show = show;
    EditorSettings::override_global(curr_settings, cx);

    update_settings_file(fs, cx, move |settings, _| {
        telemetry::event!(
            "Welcome Minimap Clicked",
            from = settings.editor.minimap.clone().unwrap_or_default(),
            to = show
        );
        settings.editor.minimap.get_or_insert_default().show = Some(show);
    });
}

fn read_inlay_hints(cx: &App) -> bool {
    AllLanguageSettings::get_global(cx)
        .defaults
        .inlay_hints
        .enabled
}

fn write_inlay_hints(enabled: bool, cx: &mut App) {
    let fs = <dyn Fs>::global(cx);

    let mut curr_settings = AllLanguageSettings::get_global(cx).clone();
    curr_settings.defaults.inlay_hints.enabled = enabled;
    AllLanguageSettings::override_global(curr_settings, cx);

    update_settings_file(fs, cx, move |settings, _cx| {
        settings
            .project
            .all_languages
            .defaults
            .inlay_hints
            .get_or_insert_default()
            .enabled = Some(enabled);
    });
}

fn read_git_blame(cx: &App) -> bool {
    ProjectSettings::get_global(cx).git.inline_blame.enabled
}

fn write_git_blame(enabled: bool, cx: &mut App) {
    let fs = <dyn Fs>::global(cx);

    let mut curr_settings = ProjectSettings::get_global(cx).clone();
    curr_settings.git.inline_blame.enabled = enabled;
    ProjectSettings::override_global(curr_settings, cx);

    update_settings_file(fs, cx, move |settings, _| {
        settings
            .git
            .get_or_insert_default()
            .inline_blame
            .get_or_insert_default()
            .enabled = Some(enabled);
    });
}

fn write_ui_font_family(font: SharedString, cx: &mut App) {
    let fs = <dyn Fs>::global(cx);

    update_settings_file(fs, cx, move |settings, _| {
        telemetry::event!(
            "Welcome Font Changed",
            type = "ui font",
            old = settings.theme.ui_font_family,
            new = font
        );
        settings.theme.ui_font_family = Some(FontFamilyName(font.into()));
    });
}

fn write_ui_font_size(size: Pixels, cx: &mut App) {
    let fs = <dyn Fs>::global(cx);

    update_settings_file(fs, cx, move |settings, _| {
        settings.theme.ui_font_size = Some(size.into());
    });
}

fn write_buffer_font_size(size: Pixels, cx: &mut App) {
    let fs = <dyn Fs>::global(cx);

    update_settings_file(fs, cx, move |settings, _| {
        settings.theme.buffer_font_size = Some(size.into());
    });
}

fn write_buffer_font_family(font_family: SharedString, cx: &mut App) {
    let fs = <dyn Fs>::global(cx);

    update_settings_file(fs, cx, move |settings, _| {
        telemetry::event!(
            "Welcome Font Changed",
            type = "editor font",
            old = settings.theme.buffer_font_family,
            new = font_family
        );

        settings.theme.buffer_font_family = Some(FontFamilyName(font_family.into()));
    });
}

fn read_font_ligatures(cx: &App) -> bool {
    ThemeSettings::get_global(cx)
        .buffer_font
        .features
        .is_calt_enabled()
        .unwrap_or(true)
}

fn write_font_ligatures(enabled: bool, cx: &mut App) {
    let fs = <dyn Fs>::global(cx);
    let bit = if enabled { 1 } else { 0 };

    update_settings_file(fs, cx, move |settings, _| {
        let mut features = settings
            .theme
            .buffer_font_features
            .as_mut()
            .map(|features| features.tag_value_list().to_vec())
            .unwrap_or_default();

        if let Some(calt_index) = features.iter().position(|(tag, _)| tag == "calt") {
            features[calt_index].1 = bit;
        } else {
            features.push(("calt".into(), bit));
        }

        settings.theme.buffer_font_features = Some(FontFeatures(Arc::new(features)));
    });
}

fn read_format_on_save(cx: &App) -> bool {
    match AllLanguageSettings::get_global(cx).defaults.format_on_save {
        FormatOnSave::On => true,
        FormatOnSave::Off => false,
    }
}

fn write_format_on_save(format_on_save: bool, cx: &mut App) {
    let fs = <dyn Fs>::global(cx);

    update_settings_file(fs, cx, move |settings, _| {
        settings.project.all_languages.defaults.format_on_save = Some(match format_on_save {
            true => FormatOnSave::On,
            false => FormatOnSave::Off,
        });
    });
}

fn render_setting_import_button(
    tab_index: isize,
    label: SharedString,
    icon_name: IconName,
    action: &dyn Action,
    imported: bool,
) -> impl IntoElement {
    let action = action.boxed_clone();
    h_flex().w_full().child(
        ButtonLike::new(label.clone())
            .full_width()
            .style(ButtonStyle::Outlined)
            .size(ButtonSize::Large)
            .tab_index(tab_index)
            .child(
                h_flex()
                    .w_full()
                    .justify_between()
                    .child(
                        h_flex()
                            .gap_1p5()
                            .px_1()
                            .child(
                                Icon::new(icon_name)
                                    .color(Color::Muted)
                                    .size(IconSize::XSmall),
                            )
                            .child(Label::new(label.clone())),
                    )
                    .when(imported, |this| {
                        this.child(
                            h_flex()
                                .gap_1p5()
                                .child(
                                    Icon::new(IconName::Check)
                                        .color(Color::Success)
                                        .size(IconSize::XSmall),
                                )
                                .child(Label::new("Imported").size(LabelSize::Small)),
                        )
                    }),
            )
            .on_click(move |_, window, cx| {
                telemetry::event!("Welcome Import Settings", import_source = label,);
                window.dispatch_action(action.boxed_clone(), cx);
            }),
    )
}

fn render_import_settings_section(tab_index: &mut isize, cx: &App) -> impl IntoElement {
    let import_state = SettingsImportState::global(cx);
    let imports: [(SharedString, IconName, &dyn Action, bool); 2] = [
        (
            "VS Code".into(),
            IconName::EditorVsCode,
            &ImportVsCodeSettings { skip_prompt: false },
            import_state.vscode,
        ),
        (
            "Cursor".into(),
            IconName::EditorCursor,
            &ImportCursorSettings { skip_prompt: false },
            import_state.cursor,
        ),
    ];

    let [vscode, cursor] = imports.map(|(label, icon_name, action, imported)| {
        *tab_index += 1;
        render_setting_import_button(*tab_index - 1, label, icon_name, action, imported)
    });

    v_flex()
        .gap_4()
        .child(
            v_flex()
                .child(Label::new("Import Settings").size(LabelSize::Large))
                .child(
                    Label::new("Automatically pull your settings from other editors.")
                        .color(Color::Muted),
                ),
        )
        .child(h_flex().w_full().gap_4().child(vscode).child(cursor))
}

fn render_font_customization_section(
    tab_index: &mut isize,
    window: &mut Window,
    cx: &mut App,
) -> impl IntoElement {
    let theme_settings = ThemeSettings::get_global(cx);
    let ui_font_size = theme_settings.ui_font_size(cx);
    let ui_font_family = theme_settings.ui_font.family.clone();
    let buffer_font_family = theme_settings.buffer_font.family.clone();
    let buffer_font_size = theme_settings.buffer_font_size(cx);

    let ui_font_picker =
        cx.new(|cx| font_picker(ui_font_family.clone(), write_ui_font_family, window, cx));

    let buffer_font_picker = cx.new(|cx| {
        font_picker(
            buffer_font_family.clone(),
            write_buffer_font_family,
            window,
            cx,
        )
    });

    let ui_font_handle = ui::PopoverMenuHandle::default();
    let buffer_font_handle = ui::PopoverMenuHandle::default();

    h_flex()
        .w_full()
        .gap_4()
        .child(
            v_flex()
                .w_full()
                .gap_1()
                .child(Label::new("UI Font"))
                .child(
                    h_flex()
                        .w_full()
                        .justify_between()
                        .gap_2()
                        .child(
                            PopoverMenu::new("ui-font-picker")
                                .menu({
                                    let ui_font_picker = ui_font_picker;
                                    move |_window, _cx| Some(ui_font_picker.clone())
                                })
                                .trigger(
                                    ButtonLike::new("ui-font-family-button")
                                        .style(ButtonStyle::Outlined)
                                        .size(ButtonSize::Medium)
                                        .full_width()
                                        .tab_index({
                                            *tab_index += 1;
                                            *tab_index - 1
                                        })
                                        .child(
                                            h_flex()
                                                .w_full()
                                                .justify_between()
                                                .child(Label::new(ui_font_family))
                                                .child(
                                                    Icon::new(IconName::ChevronUpDown)
                                                        .color(Color::Muted)
                                                        .size(IconSize::XSmall),
                                                ),
                                        ),
                                )
                                .full_width(true)
                                .anchor(gpui::Corner::TopLeft)
                                .offset(gpui::Point {
                                    x: px(0.0),
                                    y: px(4.0),
                                })
                                .with_handle(ui_font_handle),
                        )
                        .child(
                            NumericStepper::new(
                                "ui-font-size",
                                ui_font_size.to_string(),
                                move |_, _, cx| {
                                    write_ui_font_size(ui_font_size - px(1.), cx);
                                },
                                move |_, _, cx| {
                                    write_ui_font_size(ui_font_size + px(1.), cx);
                                },
                            )
                            .style(ui::NumericStepperStyle::Outlined)
                            .tab_index({
                                *tab_index += 2;
                                *tab_index - 2
                            }),
                        ),
                ),
        )
        .child(
            v_flex()
                .w_full()
                .gap_1()
                .child(Label::new("Editor Font"))
                .child(
                    h_flex()
                        .w_full()
                        .justify_between()
                        .gap_2()
                        .child(
                            PopoverMenu::new("buffer-font-picker")
                                .menu({
                                    let buffer_font_picker = buffer_font_picker;
                                    move |_window, _cx| Some(buffer_font_picker.clone())
                                })
                                .trigger(
                                    ButtonLike::new("buffer-font-family-button")
                                        .style(ButtonStyle::Outlined)
                                        .size(ButtonSize::Medium)
                                        .full_width()
                                        .tab_index({
                                            *tab_index += 1;
                                            *tab_index - 1
                                        })
                                        .child(
                                            h_flex()
                                                .w_full()
                                                .justify_between()
                                                .child(Label::new(buffer_font_family))
                                                .child(
                                                    Icon::new(IconName::ChevronUpDown)
                                                        .color(Color::Muted)
                                                        .size(IconSize::XSmall),
                                                ),
                                        ),
                                )
                                .full_width(true)
                                .anchor(gpui::Corner::TopLeft)
                                .offset(gpui::Point {
                                    x: px(0.0),
                                    y: px(4.0),
                                })
                                .with_handle(buffer_font_handle),
                        )
                        .child(
                            NumericStepper::new(
                                "buffer-font-size",
                                buffer_font_size.to_string(),
                                move |_, _, cx| {
                                    write_buffer_font_size(buffer_font_size - px(1.), cx);
                                },
                                move |_, _, cx| {
                                    write_buffer_font_size(buffer_font_size + px(1.), cx);
                                },
                            )
                            .style(ui::NumericStepperStyle::Outlined)
                            .tab_index({
                                *tab_index += 2;
                                *tab_index - 2
                            }),
                        ),
                ),
        )
}

type FontPicker = Picker<FontPickerDelegate>;

pub struct FontPickerDelegate {
    fonts: Vec<SharedString>,
    filtered_fonts: Vec<StringMatch>,
    selected_index: usize,
    current_font: SharedString,
    on_font_changed: Arc<dyn Fn(SharedString, &mut App) + 'static>,
}

impl FontPickerDelegate {
    fn new(
        current_font: SharedString,
        on_font_changed: impl Fn(SharedString, &mut App) + 'static,
        cx: &mut Context<FontPicker>,
    ) -> Self {
        let font_family_cache = FontFamilyCache::global(cx);

        let fonts = font_family_cache
            .try_list_font_families()
            .unwrap_or_else(|| vec![current_font.clone()]);
        let selected_index = fonts
            .iter()
            .position(|font| *font == current_font)
            .unwrap_or(0);

        let filtered_fonts = fonts
            .iter()
            .enumerate()
            .map(|(index, font)| StringMatch {
                candidate_id: index,
                string: font.to_string(),
                positions: Vec::new(),
                score: 0.0,
            })
            .collect();

        Self {
            fonts,
            filtered_fonts,
            selected_index,
            current_font,
            on_font_changed: Arc::new(on_font_changed),
        }
    }
}

impl PickerDelegate for FontPickerDelegate {
    type ListItem = AnyElement;

    fn match_count(&self) -> usize {
        self.filtered_fonts.len()
    }

    fn selected_index(&self) -> usize {
        self.selected_index
    }

    fn set_selected_index(&mut self, ix: usize, _: &mut Window, cx: &mut Context<FontPicker>) {
        self.selected_index = ix.min(self.filtered_fonts.len().saturating_sub(1));
        cx.notify();
    }

    fn placeholder_text(&self, _window: &mut Window, _cx: &mut App) -> Arc<str> {
        "Search fonts…".into()
    }

    fn update_matches(
        &mut self,
        query: String,
        _window: &mut Window,
        cx: &mut Context<FontPicker>,
    ) -> Task<()> {
        let fonts = self.fonts.clone();
        let current_font = self.current_font.clone();

        let matches: Vec<StringMatch> = if query.is_empty() {
            fonts
                .iter()
                .enumerate()
                .map(|(index, font)| StringMatch {
                    candidate_id: index,
                    string: font.to_string(),
                    positions: Vec::new(),
                    score: 0.0,
                })
                .collect()
        } else {
            let _candidates: Vec<StringMatchCandidate> = fonts
                .iter()
                .enumerate()
                .map(|(id, font)| StringMatchCandidate::new(id, font.as_ref()))
                .collect();

            fonts
                .iter()
                .enumerate()
                .filter(|(_, font)| font.to_lowercase().contains(&query.to_lowercase()))
                .map(|(index, font)| StringMatch {
                    candidate_id: index,
                    string: font.to_string(),
                    positions: Vec::new(),
                    score: 0.0,
                })
                .collect()
        };

        let selected_index = if query.is_empty() {
            fonts
                .iter()
                .position(|font| *font == current_font)
                .unwrap_or(0)
        } else {
            matches
                .iter()
                .position(|m| fonts[m.candidate_id] == current_font)
                .unwrap_or(0)
        };

        self.filtered_fonts = matches;
        self.selected_index = selected_index;
        cx.notify();

        Task::ready(())
    }

    fn confirm(&mut self, _secondary: bool, _window: &mut Window, cx: &mut Context<FontPicker>) {
        if let Some(font_match) = self.filtered_fonts.get(self.selected_index) {
            let font = font_match.string.clone();
            (self.on_font_changed)(font.into(), cx);
        }
    }

    fn dismissed(&mut self, _window: &mut Window, _cx: &mut Context<FontPicker>) {}

    fn render_match(
        &self,
        ix: usize,
        selected: bool,
        _window: &mut Window,
        _cx: &mut Context<FontPicker>,
    ) -> Option<Self::ListItem> {
        let font_match = self.filtered_fonts.get(ix)?;

        Some(
            ListItem::new(ix)
                .inset(true)
                .spacing(ListItemSpacing::Sparse)
                .toggle_state(selected)
                .child(Label::new(font_match.string.clone()))
                .into_any_element(),
        )
    }
}

fn font_picker(
    current_font: SharedString,
    on_font_changed: impl Fn(SharedString, &mut App) + 'static,
    window: &mut Window,
    cx: &mut Context<FontPicker>,
) -> FontPicker {
    let delegate = FontPickerDelegate::new(current_font, on_font_changed, cx);

    Picker::uniform_list(delegate, window, cx)
        .show_scrollbar(true)
        .width(rems_from_px(210.))
        .max_height(Some(rems(20.).into()))
}

fn render_popular_settings_section(
    tab_index: &mut isize,
    window: &mut Window,
    cx: &mut App,
) -> impl IntoElement {
    const LIGATURE_TOOLTIP: &str =
        "Font ligatures combine two characters into one. For example, turning != into ≠.";

    v_flex()
        .pt_6()
        .gap_4()
        .border_t_1()
        .border_color(cx.theme().colors().border_variant.opacity(0.5))
        .child(Label::new("Popular Settings").size(LabelSize::Large))
        .child(render_font_customization_section(tab_index, window, cx))
        .child(
            SwitchField::new(
                "onboarding-font-ligatures",
                "Font Ligatures",
                Some("Combine text characters into their associated symbols.".into()),
                if read_font_ligatures(cx) {
                    ui::ToggleState::Selected
                } else {
                    ui::ToggleState::Unselected
                },
                |toggle_state, _, cx| {
                    let enabled = toggle_state == &ToggleState::Selected;
                    telemetry::event!(
                        "Welcome Font Ligature",
                        options = if enabled { "on" } else { "off" },
                    );

                    write_font_ligatures(enabled, cx);
                },
            )
            .tab_index({
                *tab_index += 1;
                *tab_index - 1
            })
            .tooltip(Tooltip::text(LIGATURE_TOOLTIP)),
        )
        .child(
            SwitchField::new(
                "onboarding-format-on-save",
                "Format on Save",
                Some("Format code automatically when saving.".into()),
                if read_format_on_save(cx) {
                    ui::ToggleState::Selected
                } else {
                    ui::ToggleState::Unselected
                },
                |toggle_state, _, cx| {
                    let enabled = toggle_state == &ToggleState::Selected;
                    telemetry::event!(
                        "Welcome Format On Save Changed",
                        options = if enabled { "on" } else { "off" },
                    );

                    write_format_on_save(enabled, cx);
                },
            )
            .tab_index({
                *tab_index += 1;
                *tab_index - 1
            }),
        )
        .child(
            SwitchField::new(
                "onboarding-enable-inlay-hints",
                "Inlay Hints",
                Some("See parameter names for function and method calls inline.".into()),
                if read_inlay_hints(cx) {
                    ui::ToggleState::Selected
                } else {
                    ui::ToggleState::Unselected
                },
                |toggle_state, _, cx| {
                    let enabled = toggle_state == &ToggleState::Selected;
                    telemetry::event!(
                        "Welcome Inlay Hints Changed",
                        options = if enabled { "on" } else { "off" },
                    );

                    write_inlay_hints(enabled, cx);
                },
            )
            .tab_index({
                *tab_index += 1;
                *tab_index - 1
            }),
        )
        .child(
            SwitchField::new(
                "onboarding-git-blame-switch",
                "Inline Git Blame",
                Some("See who committed each line on a given file.".into()),
                if read_git_blame(cx) {
                    ui::ToggleState::Selected
                } else {
                    ui::ToggleState::Unselected
                },
                |toggle_state, _, cx| {
                    let enabled = toggle_state == &ToggleState::Selected;
                    telemetry::event!(
                        "Welcome Git Blame Changed",
                        options = if enabled { "on" } else { "off" },
                    );

                    write_git_blame(enabled, cx);
                },
            )
            .tab_index({
                *tab_index += 1;
                *tab_index - 1
            }),
        )
        .child(
            h_flex()
                .items_start()
                .justify_between()
                .child(
                    v_flex().child(Label::new("Minimap")).child(
                        Label::new("See a high-level overview of your source code.")
                            .color(Color::Muted),
                    ),
                )
                .child(
                    ToggleButtonGroup::single_row(
                        "onboarding-show-mini-map",
                        [
                            ToggleButtonSimple::new("Auto", |_, _, cx| {
                                write_show_mini_map(ShowMinimap::Auto, cx);
                            })
                            .tooltip(Tooltip::text(
                                "Show the minimap if the editor's scrollbar is visible.",
                            )),
                            ToggleButtonSimple::new("Always", |_, _, cx| {
                                write_show_mini_map(ShowMinimap::Always, cx);
                            }),
                            ToggleButtonSimple::new("Never", |_, _, cx| {
                                write_show_mini_map(ShowMinimap::Never, cx);
                            }),
                        ],
                    )
                    .selected_index(match read_show_mini_map(cx) {
                        ShowMinimap::Auto => 0,
                        ShowMinimap::Always => 1,
                        ShowMinimap::Never => 2,
                    })
                    .tab_index(tab_index)
                    .style(ToggleButtonGroupStyle::Outlined)
                    .width(ui::rems_from_px(3. * 64.)),
                ),
        )
}

pub(crate) fn render_editing_page(window: &mut Window, cx: &mut App) -> impl IntoElement {
    let mut tab_index = 0;
    v_flex()
        .gap_6()
        .child(render_import_settings_section(&mut tab_index, cx))
        .child(render_popular_settings_section(&mut tab_index, window, cx))
}
