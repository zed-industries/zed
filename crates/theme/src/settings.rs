use crate::{
    Appearance, DEFAULT_ICON_THEME_NAME, SyntaxTheme, Theme, status_colors_refinement,
    syntax_overrides, theme_colors_refinement,
};
use collections::HashMap;
use derive_more::{Deref, DerefMut};
use gpui::{
    App, Context, Font, FontFallbacks, FontStyle, FontWeight, Global, Pixels, Subscription, Window,
    px,
};
use refineable::Refineable;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
pub use settings::{FontFamilyName, IconThemeName, ThemeMode, ThemeName};
use settings::{Settings, SettingsContent};
use std::sync::Arc;

const MIN_FONT_SIZE: Pixels = px(6.0);
const MAX_FONT_SIZE: Pixels = px(100.0);
const MIN_LINE_HEIGHT: f32 = 1.0;

#[derive(
    Debug,
    Default,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Clone,
    Copy,
    Serialize,
    Deserialize,
    JsonSchema,
)]

/// Specifies the density of the UI.
/// Note: This setting is still experimental. See [this tracking issue](https://github.com/zed-industries/zed/issues/18078)
#[serde(rename_all = "snake_case")]
pub enum UiDensity {
    /// A denser UI with tighter spacing and smaller elements.
    #[serde(alias = "compact")]
    Compact,
    #[default]
    #[serde(alias = "default")]
    /// The default UI density.
    Default,
    #[serde(alias = "comfortable")]
    /// A looser UI with more spacing and larger elements.
    Comfortable,
}

impl UiDensity {
    /// The spacing ratio of a given density.
    /// TODO: Standardize usage throughout the app or remove
    pub fn spacing_ratio(self) -> f32 {
        match self {
            UiDensity::Compact => 0.75,
            UiDensity::Default => 1.0,
            UiDensity::Comfortable => 1.25,
        }
    }
}

impl From<String> for UiDensity {
    fn from(s: String) -> Self {
        match s.as_str() {
            "compact" => Self::Compact,
            "default" => Self::Default,
            "comfortable" => Self::Comfortable,
            _ => Self::default(),
        }
    }
}

impl From<UiDensity> for String {
    fn from(val: UiDensity) -> Self {
        match val {
            UiDensity::Compact => "compact".to_string(),
            UiDensity::Default => "default".to_string(),
            UiDensity::Comfortable => "comfortable".to_string(),
        }
    }
}

impl From<settings::UiDensity> for UiDensity {
    fn from(val: settings::UiDensity) -> Self {
        match val {
            settings::UiDensity::Compact => Self::Compact,
            settings::UiDensity::Default => Self::Default,
            settings::UiDensity::Comfortable => Self::Comfortable,
        }
    }
}

/// Customizable settings for the UI and theme system.
#[derive(Clone, PartialEq)]
pub struct ThemeSettings {
    /// The UI font size. Determines the size of text in the UI,
    /// as well as the size of a [gpui::Rems] unit.
    ///
    /// Changing this will impact the size of all UI elements.
    ui_font_size: Pixels,
    /// The font used for UI elements.
    pub ui_font: Font,
    /// The font size used for buffers, and the terminal.
    ///
    /// The terminal font size can be overridden using it's own setting.
    buffer_font_size: Pixels,
    /// The font used for buffers, and the terminal.
    ///
    /// The terminal font family can be overridden using it's own setting.
    pub buffer_font: Font,
    /// The agent font size. Determines the size of text in the agent panel. Falls back to the UI font size if unset.
    agent_ui_font_size: Option<Pixels>,
    /// The agent buffer font size. Determines the size of user messages in the agent panel.
    agent_buffer_font_size: Option<Pixels>,
    /// The line height for buffers, and the terminal.
    ///
    /// Changing this may affect the spacing of some UI elements.
    ///
    /// The terminal font family can be overridden using it's own setting.
    pub buffer_line_height: BufferLineHeight,
    /// The current theme selection.
    pub theme: ThemeSelection,
    /// Manual overrides for the active theme.
    ///
    /// Note: This setting is still experimental. See [this tracking issue](https://github.com/zed-industries/zed/issues/18078)
    pub experimental_theme_overrides: Option<settings::ThemeStyleContent>,
    /// Manual overrides per theme
    pub theme_overrides: HashMap<String, settings::ThemeStyleContent>,
    /// The current icon theme selection.
    pub icon_theme: IconThemeSelection,
    /// The density of the UI.
    /// Note: This setting is still experimental. See [this tracking issue](
    pub ui_density: UiDensity,
    /// The amount of fading applied to unnecessary code.
    pub unnecessary_code_fade: f32,
}

pub(crate) const DEFAULT_LIGHT_THEME: &'static str = "One Light";
pub(crate) const DEFAULT_DARK_THEME: &'static str = "One Dark";

/// Returns the name of the default theme for the given [`Appearance`].
pub fn default_theme(appearance: Appearance) -> &'static str {
    match appearance {
        Appearance::Light => DEFAULT_LIGHT_THEME,
        Appearance::Dark => DEFAULT_DARK_THEME,
    }
}

/// The appearance of the system.
#[derive(Debug, Clone, Copy, Deref)]
pub struct SystemAppearance(pub Appearance);

impl Default for SystemAppearance {
    fn default() -> Self {
        Self(Appearance::Dark)
    }
}

#[derive(Deref, DerefMut, Default)]
struct GlobalSystemAppearance(SystemAppearance);

impl Global for GlobalSystemAppearance {}

impl SystemAppearance {
    /// Initializes the [`SystemAppearance`] for the application.
    pub fn init(cx: &mut App) {
        *cx.default_global::<GlobalSystemAppearance>() =
            GlobalSystemAppearance(SystemAppearance(cx.window_appearance().into()));
    }

    /// Returns the global [`SystemAppearance`].
    pub fn global(cx: &App) -> Self {
        cx.global::<GlobalSystemAppearance>().0
    }

    /// Returns a mutable reference to the global [`SystemAppearance`].
    pub fn global_mut(cx: &mut App) -> &mut Self {
        cx.global_mut::<GlobalSystemAppearance>()
    }
}

#[derive(Default)]
struct BufferFontSize(Pixels);

impl Global for BufferFontSize {}

#[derive(Default)]
pub(crate) struct UiFontSize(Pixels);

impl Global for UiFontSize {}

/// In-memory override for the font size in the agent panel.
#[derive(Default)]
pub struct AgentFontSize(Pixels);

impl Global for AgentFontSize {}

/// Represents the selection of a theme, which can be either static or dynamic.
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(untagged)]
pub enum ThemeSelection {
    /// A static theme selection, represented by a single theme name.
    Static(ThemeName),
    /// A dynamic theme selection, which can change based the [ThemeMode].
    Dynamic {
        /// The mode used to determine which theme to use.
        #[serde(default)]
        mode: ThemeMode,
        /// The theme to use for light mode.
        light: ThemeName,
        /// The theme to use for dark mode.
        dark: ThemeName,
    },
}

impl From<settings::ThemeSelection> for ThemeSelection {
    fn from(selection: settings::ThemeSelection) -> Self {
        match selection {
            settings::ThemeSelection::Static(theme) => ThemeSelection::Static(theme),
            settings::ThemeSelection::Dynamic { mode, light, dark } => {
                ThemeSelection::Dynamic { mode, light, dark }
            }
        }
    }
}

impl ThemeSelection {
    /// Returns the theme name for the selected [ThemeMode].
    pub fn name(&self, system_appearance: Appearance) -> ThemeName {
        match self {
            Self::Static(theme) => theme.clone(),
            Self::Dynamic { mode, light, dark } => match mode {
                ThemeMode::Light => light.clone(),
                ThemeMode::Dark => dark.clone(),
                ThemeMode::System => match system_appearance {
                    Appearance::Light => light.clone(),
                    Appearance::Dark => dark.clone(),
                },
            },
        }
    }

    /// Returns the [ThemeMode] for the [ThemeSelection].
    pub fn mode(&self) -> Option<ThemeMode> {
        match self {
            ThemeSelection::Static(_) => None,
            ThemeSelection::Dynamic { mode, .. } => Some(*mode),
        }
    }
}

/// Represents the selection of an icon theme, which can be either static or dynamic.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum IconThemeSelection {
    /// A static icon theme selection, represented by a single icon theme name.
    Static(IconThemeName),
    /// A dynamic icon theme selection, which can change based on the [`ThemeMode`].
    Dynamic {
        /// The mode used to determine which theme to use.
        mode: ThemeMode,
        /// The icon theme to use for light mode.
        light: IconThemeName,
        /// The icon theme to use for dark mode.
        dark: IconThemeName,
    },
}

impl From<settings::IconThemeSelection> for IconThemeSelection {
    fn from(selection: settings::IconThemeSelection) -> Self {
        match selection {
            settings::IconThemeSelection::Static(theme) => IconThemeSelection::Static(theme),
            settings::IconThemeSelection::Dynamic { mode, light, dark } => {
                IconThemeSelection::Dynamic { mode, light, dark }
            }
        }
    }
}

impl IconThemeSelection {
    /// Returns the icon theme name based on the given [`Appearance`].
    pub fn name(&self, system_appearance: Appearance) -> IconThemeName {
        match self {
            Self::Static(theme) => theme.clone(),
            Self::Dynamic { mode, light, dark } => match mode {
                ThemeMode::Light => light.clone(),
                ThemeMode::Dark => dark.clone(),
                ThemeMode::System => match system_appearance {
                    Appearance::Light => light.clone(),
                    Appearance::Dark => dark.clone(),
                },
            },
        }
    }

    /// Returns the [`ThemeMode`] for the [`IconThemeSelection`].
    pub fn mode(&self) -> Option<ThemeMode> {
        match self {
            IconThemeSelection::Static(_) => None,
            IconThemeSelection::Dynamic { mode, .. } => Some(*mode),
        }
    }
}

// impl ThemeSettingsContent {
/// Sets the theme for the given appearance to the theme with the specified name.
pub fn set_theme(
    current: &mut SettingsContent,
    theme_name: impl Into<Arc<str>>,
    appearance: Appearance,
) {
    if let Some(selection) = current.theme.theme.as_mut() {
        let theme_to_update = match selection {
            settings::ThemeSelection::Static(theme) => theme,
            settings::ThemeSelection::Dynamic { mode, light, dark } => match mode {
                ThemeMode::Light => light,
                ThemeMode::Dark => dark,
                ThemeMode::System => match appearance {
                    Appearance::Light => light,
                    Appearance::Dark => dark,
                },
            },
        };

        *theme_to_update = ThemeName(theme_name.into());
    } else {
        current.theme.theme = Some(settings::ThemeSelection::Static(ThemeName(
            theme_name.into(),
        )));
    }
}

/// Sets the icon theme for the given appearance to the icon theme with the specified name.
pub fn set_icon_theme(
    current: &mut SettingsContent,
    icon_theme_name: IconThemeName,
    appearance: Appearance,
) {
    if let Some(selection) = current.theme.icon_theme.as_mut() {
        let icon_theme_to_update = match selection {
            settings::IconThemeSelection::Static(theme) => theme,
            settings::IconThemeSelection::Dynamic { mode, light, dark } => match mode {
                ThemeMode::Light => light,
                ThemeMode::Dark => dark,
                ThemeMode::System => match appearance {
                    Appearance::Light => light,
                    Appearance::Dark => dark,
                },
            },
        };

        *icon_theme_to_update = icon_theme_name;
    } else {
        current.theme.icon_theme = Some(settings::IconThemeSelection::Static(icon_theme_name));
    }
}

/// Sets the mode for the theme.
pub fn set_mode(content: &mut SettingsContent, mode: ThemeMode) {
    let theme = content.theme.as_mut();

    if let Some(selection) = theme.theme.as_mut() {
        match selection {
            settings::ThemeSelection::Static(theme) => {
                // If the theme was previously set to a single static theme,
                // we don't know whether it was a light or dark theme, so we
                // just use it for both.
                *selection = settings::ThemeSelection::Dynamic {
                    mode,
                    light: theme.clone(),
                    dark: theme.clone(),
                };
            }
            settings::ThemeSelection::Dynamic {
                mode: mode_to_update,
                ..
            } => *mode_to_update = mode,
        }
    } else {
        theme.theme = Some(settings::ThemeSelection::Dynamic {
            mode,
            light: ThemeName(DEFAULT_LIGHT_THEME.into()),
            dark: ThemeName(DEFAULT_DARK_THEME.into()),
        });
    }

    if let Some(selection) = theme.icon_theme.as_mut() {
        match selection {
            settings::IconThemeSelection::Static(icon_theme) => {
                // If the icon theme was previously set to a single static
                // theme, we don't know whether it was a light or dark
                // theme, so we just use it for both.
                *selection = settings::IconThemeSelection::Dynamic {
                    mode,
                    light: icon_theme.clone(),
                    dark: icon_theme.clone(),
                };
            }
            settings::IconThemeSelection::Dynamic {
                mode: mode_to_update,
                ..
            } => *mode_to_update = mode,
        }
    } else {
        theme.icon_theme = Some(settings::IconThemeSelection::Static(IconThemeName(
            DEFAULT_ICON_THEME_NAME.into(),
        )));
    }
}
// }

/// The buffer's line height.
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum BufferLineHeight {
    /// A less dense line height.
    #[default]
    Comfortable,
    /// The default line height.
    Standard,
    /// A custom line height, where 1.0 is the font's height. Must be at least 1.0.
    Custom(f32),
}

impl From<settings::BufferLineHeight> for BufferLineHeight {
    fn from(value: settings::BufferLineHeight) -> Self {
        match value {
            settings::BufferLineHeight::Comfortable => BufferLineHeight::Comfortable,
            settings::BufferLineHeight::Standard => BufferLineHeight::Standard,
            settings::BufferLineHeight::Custom(line_height) => {
                BufferLineHeight::Custom(line_height)
            }
        }
    }
}

impl BufferLineHeight {
    /// Returns the value of the line height.
    pub fn value(&self) -> f32 {
        match self {
            BufferLineHeight::Comfortable => 1.618,
            BufferLineHeight::Standard => 1.3,
            BufferLineHeight::Custom(line_height) => *line_height,
        }
    }
}

impl ThemeSettings {
    /// Returns the buffer font size.
    pub fn buffer_font_size(&self, cx: &App) -> Pixels {
        let font_size = cx
            .try_global::<BufferFontSize>()
            .map(|size| size.0)
            .unwrap_or(self.buffer_font_size);
        clamp_font_size(font_size)
    }

    /// Returns the UI font size.
    pub fn ui_font_size(&self, cx: &App) -> Pixels {
        let font_size = cx
            .try_global::<UiFontSize>()
            .map(|size| size.0)
            .unwrap_or(self.ui_font_size);
        clamp_font_size(font_size)
    }

    /// Returns the agent panel font size. Falls back to the UI font size if unset.
    pub fn agent_ui_font_size(&self, cx: &App) -> Pixels {
        cx.try_global::<AgentFontSize>()
            .map(|size| size.0)
            .or(self.agent_ui_font_size)
            .map(clamp_font_size)
            .unwrap_or_else(|| self.ui_font_size(cx))
    }

    /// Returns the agent panel buffer font size.
    pub fn agent_buffer_font_size(&self, cx: &App) -> Pixels {
        cx.try_global::<AgentFontSize>()
            .map(|size| size.0)
            .or(self.agent_buffer_font_size)
            .map(clamp_font_size)
            .unwrap_or_else(|| self.buffer_font_size(cx))
    }

    /// Returns the buffer font size, read from the settings.
    ///
    /// The real buffer font size is stored in-memory, to support temporary font size changes.
    /// Use [`Self::buffer_font_size`] to get the real font size.
    pub fn buffer_font_size_settings(&self) -> Pixels {
        self.buffer_font_size
    }

    /// Returns the UI font size, read from the settings.
    ///
    /// The real UI font size is stored in-memory, to support temporary font size changes.
    /// Use [`Self::ui_font_size`] to get the real font size.
    pub fn ui_font_size_settings(&self) -> Pixels {
        self.ui_font_size
    }

    /// Returns the agent font size, read from the settings.
    ///
    /// The real agent font size is stored in-memory, to support temporary font size changes.
    /// Use [`Self::agent_ui_font_size`] to get the real font size.
    pub fn agent_ui_font_size_settings(&self) -> Option<Pixels> {
        self.agent_ui_font_size
    }

    /// Returns the agent buffer font size, read from the settings.
    ///
    /// The real agent buffer font size is stored in-memory, to support temporary font size changes.
    /// Use [`Self::agent_buffer_font_size`] to get the real font size.
    pub fn agent_buffer_font_size_settings(&self) -> Option<Pixels> {
        self.agent_buffer_font_size
    }

    // TODO: Rename: `line_height` -> `buffer_line_height`
    /// Returns the buffer's line height.
    pub fn line_height(&self) -> f32 {
        f32::max(self.buffer_line_height.value(), MIN_LINE_HEIGHT)
    }

    /// Applies the theme overrides, if there are any, to the current theme.
    pub fn apply_theme_overrides(&self, mut arc_theme: Arc<Theme>) -> Arc<Theme> {
        // Apply the old overrides setting first, so that the new setting can override those.
        if let Some(experimental_theme_overrides) = &self.experimental_theme_overrides {
            let mut theme = (*arc_theme).clone();
            ThemeSettings::modify_theme(&mut theme, experimental_theme_overrides);
            arc_theme = Arc::new(theme);
        }

        if let Some(theme_overrides) = self.theme_overrides.get(arc_theme.name.as_ref()) {
            let mut theme = (*arc_theme).clone();
            ThemeSettings::modify_theme(&mut theme, theme_overrides);
            arc_theme = Arc::new(theme);
        }

        arc_theme
    }

    fn modify_theme(base_theme: &mut Theme, theme_overrides: &settings::ThemeStyleContent) {
        if let Some(window_background_appearance) = theme_overrides.window_background_appearance {
            base_theme.styles.window_background_appearance = window_background_appearance.into();
        }
        let status_color_refinement = status_colors_refinement(&theme_overrides.status);

        base_theme.styles.colors.refine(&theme_colors_refinement(
            &theme_overrides.colors,
            &status_color_refinement,
        ));
        base_theme.styles.status.refine(&status_color_refinement);
        base_theme.styles.player.merge(&theme_overrides.players);
        base_theme.styles.accents.merge(&theme_overrides.accents);
        base_theme.styles.syntax = SyntaxTheme::merge(
            base_theme.styles.syntax.clone(),
            syntax_overrides(&theme_overrides),
        );
    }
}

/// Observe changes to the adjusted buffer font size.
pub fn observe_buffer_font_size_adjustment<V: 'static>(
    cx: &mut Context<V>,
    f: impl 'static + Fn(&mut V, &mut Context<V>),
) -> Subscription {
    cx.observe_global::<BufferFontSize>(f)
}

/// Gets the font size, adjusted by the difference between the current buffer font size and the one set in the settings.
pub fn adjusted_font_size(size: Pixels, cx: &App) -> Pixels {
    let adjusted_font_size =
        if let Some(BufferFontSize(adjusted_size)) = cx.try_global::<BufferFontSize>() {
            let buffer_font_size = ThemeSettings::get_global(cx).buffer_font_size;
            let delta = *adjusted_size - buffer_font_size;
            size + delta
        } else {
            size
        };
    clamp_font_size(adjusted_font_size)
}

/// Adjusts the buffer font size.
pub fn adjust_buffer_font_size(cx: &mut App, f: impl FnOnce(Pixels) -> Pixels) {
    let buffer_font_size = ThemeSettings::get_global(cx).buffer_font_size;
    let adjusted_size = cx
        .try_global::<BufferFontSize>()
        .map_or(buffer_font_size, |adjusted_size| adjusted_size.0);
    cx.set_global(BufferFontSize(clamp_font_size(f(adjusted_size))));
    cx.refresh_windows();
}

/// Resets the buffer font size to the default value.
pub fn reset_buffer_font_size(cx: &mut App) {
    if cx.has_global::<BufferFontSize>() {
        cx.remove_global::<BufferFontSize>();
        cx.refresh_windows();
    }
}

// TODO: Make private, change usages to use `get_ui_font_size` instead.
#[allow(missing_docs)]
pub fn setup_ui_font(window: &mut Window, cx: &mut App) -> gpui::Font {
    let (ui_font, ui_font_size) = {
        let theme_settings = ThemeSettings::get_global(cx);
        let font = theme_settings.ui_font.clone();
        (font, theme_settings.ui_font_size(cx))
    };

    window.set_rem_size(ui_font_size);
    ui_font
}

/// Sets the adjusted UI font size.
pub fn adjust_ui_font_size(cx: &mut App, f: impl FnOnce(Pixels) -> Pixels) {
    let ui_font_size = ThemeSettings::get_global(cx).ui_font_size(cx);
    let adjusted_size = cx
        .try_global::<UiFontSize>()
        .map_or(ui_font_size, |adjusted_size| adjusted_size.0);
    cx.set_global(UiFontSize(clamp_font_size(f(adjusted_size))));
    cx.refresh_windows();
}

/// Resets the UI font size to the default value.
pub fn reset_ui_font_size(cx: &mut App) {
    if cx.has_global::<UiFontSize>() {
        cx.remove_global::<UiFontSize>();
        cx.refresh_windows();
    }
}

/// Sets the adjusted font size of agent responses in the agent panel.
pub fn adjust_agent_ui_font_size(cx: &mut App, f: impl FnOnce(Pixels) -> Pixels) {
    let agent_ui_font_size = ThemeSettings::get_global(cx).agent_ui_font_size(cx);
    let adjusted_size = cx
        .try_global::<AgentFontSize>()
        .map_or(agent_ui_font_size, |adjusted_size| adjusted_size.0);
    cx.set_global(AgentFontSize(clamp_font_size(f(adjusted_size))));
    cx.refresh_windows();
}

/// Resets the agent response font size in the agent panel to the default value.
pub fn reset_agent_ui_font_size(cx: &mut App) {
    if cx.has_global::<AgentFontSize>() {
        cx.remove_global::<AgentFontSize>();
        cx.refresh_windows();
    }
}

/// Sets the adjusted font size of user messages in the agent panel.
pub fn adjust_agent_buffer_font_size(cx: &mut App, f: impl FnOnce(Pixels) -> Pixels) {
    let agent_buffer_font_size = ThemeSettings::get_global(cx).agent_buffer_font_size(cx);
    let adjusted_size = cx
        .try_global::<AgentFontSize>()
        .map_or(agent_buffer_font_size, |adjusted_size| adjusted_size.0);
    cx.set_global(AgentFontSize(clamp_font_size(f(adjusted_size))));
    cx.refresh_windows();
}

/// Resets the user message font size in the agent panel to the default value.
pub fn reset_agent_buffer_font_size(cx: &mut App) {
    if cx.has_global::<AgentFontSize>() {
        cx.remove_global::<AgentFontSize>();
        cx.refresh_windows();
    }
}

/// Ensures font size is within the valid range.
pub fn clamp_font_size(size: Pixels) -> Pixels {
    size.clamp(MIN_FONT_SIZE, MAX_FONT_SIZE)
}

fn clamp_font_weight(weight: f32) -> FontWeight {
    FontWeight(weight.clamp(100., 950.))
}

/// font fallback from settings
pub fn font_fallbacks_from_settings(
    fallbacks: Option<Vec<settings::FontFamilyName>>,
) -> Option<FontFallbacks> {
    fallbacks.map(|fallbacks| {
        FontFallbacks::from_fonts(
            fallbacks
                .into_iter()
                .map(|font_family| font_family.0.to_string())
                .collect(),
        )
    })
}

impl settings::Settings for ThemeSettings {
    fn from_settings(content: &settings::SettingsContent) -> Self {
        let content = &content.theme;
        let theme_selection: ThemeSelection = content.theme.clone().unwrap().into();
        let icon_theme_selection: IconThemeSelection = content.icon_theme.clone().unwrap().into();
        Self {
            ui_font_size: clamp_font_size(content.ui_font_size.unwrap().into()),
            ui_font: Font {
                family: content.ui_font_family.as_ref().unwrap().0.clone().into(),
                features: content.ui_font_features.clone().unwrap(),
                fallbacks: font_fallbacks_from_settings(content.ui_font_fallbacks.clone()),
                weight: clamp_font_weight(content.ui_font_weight.unwrap().0),
                style: Default::default(),
            },
            buffer_font: Font {
                family: content
                    .buffer_font_family
                    .as_ref()
                    .unwrap()
                    .0
                    .clone()
                    .into(),
                features: content.buffer_font_features.clone().unwrap(),
                fallbacks: font_fallbacks_from_settings(content.buffer_font_fallbacks.clone()),
                weight: clamp_font_weight(content.buffer_font_weight.unwrap().0),
                style: FontStyle::default(),
            },
            buffer_font_size: clamp_font_size(content.buffer_font_size.unwrap().into()),
            buffer_line_height: content.buffer_line_height.unwrap().into(),
            agent_ui_font_size: content.agent_ui_font_size.map(Into::into),
            agent_buffer_font_size: content.agent_buffer_font_size.map(Into::into),
            theme: theme_selection,
            experimental_theme_overrides: content.experimental_theme_overrides.clone(),
            theme_overrides: content.theme_overrides.clone(),
            icon_theme: icon_theme_selection,
            ui_density: content.ui_density.unwrap_or_default().into(),
            unnecessary_code_fade: content.unnecessary_code_fade.unwrap().0.clamp(0.0, 0.9),
        }
    }

    fn import_from_vscode(vscode: &settings::VsCodeSettings, current: &mut SettingsContent) {
        vscode.from_f32_setting("editor.fontWeight", &mut current.theme.buffer_font_weight);
        vscode.from_f32_setting("editor.fontSize", &mut current.theme.buffer_font_size);
        if let Some(font) = vscode.read_string("editor.font") {
            current.theme.buffer_font_family = Some(FontFamilyName(font.into()));
        }
        // TODO: possibly map editor.fontLigatures to buffer_font_features?
    }
}
