use super::*;

pub struct TerminalLayoutData<'a> {
    pub text_style: TextStyle,
    pub selection_color: Color,
    pub terminal_theme: &'a TerminalStyle,
    pub size: TerminalDimensions,
}

impl<'a> TerminalLayoutData<'a> {
    pub fn new(settings: &'a Settings, font_cache: &FontCache, constraint: Vector2F) -> Self {
        let text_style = Self::make_text_style(font_cache, &settings);
        let selection_color = settings.theme.editor.selection.selection;
        let terminal_theme = &settings.theme.terminal;

        let line_height = font_cache.line_height(text_style.font_size);

        let cell_width = font_cache.em_advance(text_style.font_id, text_style.font_size);
        let dimensions = TerminalDimensions::new(line_height, cell_width, constraint);

        TerminalLayoutData {
            size: dimensions,
            text_style,
            selection_color,
            terminal_theme,
        }
    }

    ///Configures a text style from the current settings.
    pub fn make_text_style(font_cache: &FontCache, settings: &Settings) -> TextStyle {
        // Pull the font family from settings properly overriding
        let family_id = settings
            .terminal_overrides
            .font_family
            .as_ref()
            .or_else(|| settings.terminal_defaults.font_family.as_ref())
            .and_then(|family_name| font_cache.load_family(&[family_name]).log_err())
            .unwrap_or(settings.buffer_font_family);

        let font_size = settings
            .terminal_overrides
            .font_size
            .or(settings.terminal_defaults.font_size)
            .unwrap_or(settings.buffer_font_size);

        let font_id = font_cache
            .select_font(family_id, &Default::default())
            .unwrap();

        TextStyle {
            color: settings.theme.editor.text_color,
            font_family_id: family_id,
            font_family_name: font_cache.family_name(family_id).unwrap(),
            font_id,
            font_size,
            font_properties: Default::default(),
            underline: Default::default(),
        }
    }
}
