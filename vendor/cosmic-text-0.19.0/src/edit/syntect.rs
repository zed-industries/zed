#[cfg(not(feature = "std"))]
use alloc::{string::String, vec::Vec};
#[cfg(feature = "std")]
use std::{fs, io, path::Path};
use syntect::highlighting::{
    FontStyle, HighlightState, Highlighter, RangedHighlightIterator, ThemeSet,
};
use syntect::parsing::{ParseState, ScopeStack, SyntaxReference, SyntaxSet};

use crate::{
    Action, AttrsList, BorrowedWithFontSystem, BufferRef, Change, Color, Cursor, Edit, Editor,
    FontSystem, Renderer, Selection, Shaping, Style, UnderlineStyle, Weight,
};

pub use syntect::highlighting::Theme as SyntaxTheme;

#[derive(Debug)]
pub struct SyntaxSystem {
    pub syntax_set: SyntaxSet,
    pub theme_set: ThemeSet,
}

impl SyntaxSystem {
    /// Create a new [`SyntaxSystem`]
    pub fn new() -> Self {
        Self {
            //TODO: store newlines in buffer
            syntax_set: SyntaxSet::load_defaults_nonewlines(),
            theme_set: ThemeSet::load_defaults(),
        }
    }
}

/// A wrapper of [`Editor`] with syntax highlighting provided by [`SyntaxSystem`]
#[derive(Debug)]
pub struct SyntaxEditor<'syntax_system, 'buffer> {
    editor: Editor<'buffer>,
    syntax_system: &'syntax_system SyntaxSystem,
    syntax: &'syntax_system SyntaxReference,
    theme: &'syntax_system SyntaxTheme,
    highlighter: Highlighter<'syntax_system>,
    syntax_cache: Vec<(ParseState, ScopeStack)>,
}

impl<'syntax_system, 'buffer> SyntaxEditor<'syntax_system, 'buffer> {
    /// Create a new [`SyntaxEditor`] with the provided [`Buffer`], [`SyntaxSystem`], and theme name.
    ///
    /// A good default theme name is "base16-eighties.dark".
    ///
    /// Returns None if theme not found
    pub fn new(
        buffer: impl Into<BufferRef<'buffer>>,
        syntax_system: &'syntax_system SyntaxSystem,
        theme_name: &str,
    ) -> Option<Self> {
        let editor = Editor::new(buffer);
        let syntax = syntax_system.syntax_set.find_syntax_plain_text();
        let theme = syntax_system.theme_set.themes.get(theme_name)?;
        let highlighter = Highlighter::new(theme);

        Some(Self {
            editor,
            syntax_system,
            syntax,
            theme,
            highlighter,
            syntax_cache: Vec::new(),
        })
    }

    /// Modifies the theme of the [`SyntaxEditor`], returning false if the theme is missing
    pub fn update_theme(&mut self, theme_name: &str) -> bool {
        if let Some(theme) = self.syntax_system.theme_set.themes.get(theme_name) {
            if self.theme != theme {
                self.theme = theme;
                self.highlighter = Highlighter::new(theme);
                self.syntax_cache.clear();

                // Reset attrs to match default foreground and no highlighting
                self.with_buffer_mut(|buffer| {
                    for line in buffer.lines.iter_mut() {
                        let mut attrs = line.attrs_list().defaults();
                        if let Some(foreground) = self.theme.settings.foreground {
                            attrs = attrs.color(Color::rgba(
                                foreground.r,
                                foreground.g,
                                foreground.b,
                                foreground.a,
                            ));
                        }
                        line.set_attrs_list(AttrsList::new(&attrs));
                    }
                });
            }

            true
        } else {
            false
        }
    }

    /// Load text from a file, and also set syntax to the best option
    ///
    /// ## Errors
    ///
    /// Returns an [`io::Error`] if reading the file fails
    #[cfg(feature = "std")]
    pub fn load_text<P: AsRef<Path>>(
        &mut self,
        _font_system: &mut FontSystem,
        path: P,
        mut attrs: crate::Attrs,
    ) -> io::Result<()> {
        let path = path.as_ref();

        // Set attrs to match default foreground
        if let Some(foreground) = self.theme.settings.foreground {
            attrs = attrs.color(Color::rgba(
                foreground.r,
                foreground.g,
                foreground.b,
                foreground.a,
            ));
        }

        // Clear buffer first (allows sane handling of non-existant files)
        self.editor.with_buffer_mut(|buffer| {
            buffer.set_text("", &attrs, Shaping::Advanced, None);
        });

        // Update syntax based on file name
        self.syntax = match self.syntax_system.syntax_set.find_syntax_for_file(path) {
            Ok(Some(some)) => some,
            Ok(None) => {
                log::warn!("no syntax found for {path:?}");
                self.syntax_system.syntax_set.find_syntax_plain_text()
            }
            Err(err) => {
                log::warn!("failed to determine syntax for {path:?}: {err:?}");
                self.syntax_system.syntax_set.find_syntax_plain_text()
            }
        };

        // Clear syntax cache
        self.syntax_cache.clear();

        // Set text
        let text = fs::read_to_string(path)?;
        self.editor.with_buffer_mut(|buffer| {
            buffer.set_text(&text, &attrs, Shaping::Advanced, None);
        });

        Ok(())
    }

    /// Set syntax highlighting by file extension
    pub fn syntax_by_extension(&mut self, extension: &str) {
        self.syntax = match self
            .syntax_system
            .syntax_set
            .find_syntax_by_extension(extension)
        {
            Some(some) => some,
            None => {
                log::warn!("no syntax found for {extension:?}");
                self.syntax_system.syntax_set.find_syntax_plain_text()
            }
        };

        self.syntax_cache.clear();
    }

    /// Get the default background color
    pub fn background_color(&self) -> Color {
        if let Some(background) = self.theme.settings.background {
            Color::rgba(background.r, background.g, background.b, background.a)
        } else {
            Color::rgb(0, 0, 0)
        }
    }

    /// Get the default foreground (text) color
    pub fn foreground_color(&self) -> Color {
        if let Some(foreground) = self.theme.settings.foreground {
            Color::rgba(foreground.r, foreground.g, foreground.b, foreground.a)
        } else {
            Color::rgb(0xFF, 0xFF, 0xFF)
        }
    }

    /// Get the default cursor color
    pub fn cursor_color(&self) -> Color {
        if let Some(some) = self.theme.settings.caret {
            Color::rgba(some.r, some.g, some.b, some.a)
        } else {
            self.foreground_color()
        }
    }

    /// Get the default selection color
    pub fn selection_color(&self) -> Color {
        if let Some(some) = self.theme.settings.selection {
            Color::rgba(some.r, some.g, some.b, some.a)
        } else {
            let foreground_color = self.foreground_color();
            Color::rgba(
                foreground_color.r(),
                foreground_color.g(),
                foreground_color.b(),
                0x33,
            )
        }
    }

    /// Get the current syntect theme
    pub fn theme(&self) -> &SyntaxTheme {
        self.theme
    }

    /// Draw the editor
    ///
    /// Automatically resolves any pending dirty state before drawing.
    #[cfg(feature = "swash")]
    pub fn draw<F>(
        &mut self,
        font_system: &mut FontSystem,
        cache: &mut crate::SwashCache,
        callback: F,
    ) where
        F: FnMut(i32, i32, u32, u32, Color),
    {
        self.editor.draw(
            font_system,
            cache,
            self.foreground_color(),
            self.cursor_color(),
            self.selection_color(),
            self.foreground_color(),
            callback,
        );
    }

    /// Render the editor using the provided renderer.
    ///
    /// The caller is responsible for calling [`Edit::shape_as_needed`] first
    /// to ensure layout is up to date.
    pub fn render<R: Renderer>(&self, renderer: &mut R) {
        let size = self.with_buffer(|buffer| buffer.size());
        if let Some(width) = size.0 {
            if let Some(height) = size.1 {
                renderer.rectangle(0, 0, width as u32, height as u32, self.background_color());
            }
        }
        self.editor.render(
            renderer,
            self.foreground_color(),
            self.cursor_color(),
            self.selection_color(),
            self.foreground_color(),
        );
    }
}

impl<'buffer> Edit<'buffer> for SyntaxEditor<'_, 'buffer> {
    fn buffer_ref(&self) -> &BufferRef<'buffer> {
        self.editor.buffer_ref()
    }

    fn buffer_ref_mut(&mut self) -> &mut BufferRef<'buffer> {
        self.editor.buffer_ref_mut()
    }

    fn cursor(&self) -> Cursor {
        self.editor.cursor()
    }

    fn set_cursor(&mut self, cursor: Cursor) {
        self.editor.set_cursor(cursor);
    }

    fn selection(&self) -> Selection {
        self.editor.selection()
    }

    fn set_selection(&mut self, selection: Selection) {
        self.editor.set_selection(selection);
    }

    fn auto_indent(&self) -> bool {
        self.editor.auto_indent()
    }

    fn set_auto_indent(&mut self, auto_indent: bool) {
        self.editor.set_auto_indent(auto_indent);
    }

    fn tab_width(&self) -> u16 {
        self.editor.tab_width()
    }

    fn set_tab_width(&mut self, tab_width: u16) {
        self.editor.set_tab_width(tab_width);
    }

    fn shape_as_needed(&mut self, font_system: &mut FontSystem, prune: bool) {
        #[cfg(feature = "std")]
        let now = std::time::Instant::now();

        let cursor = self.cursor();
        self.editor.with_buffer_mut(|buffer| {
            let metrics = buffer.metrics();
            let scroll = buffer.scroll();
            let scroll_end = scroll.vertical + buffer.size().1.unwrap_or(f32::INFINITY);
            let mut total_height = 0.0;
            let mut highlighted = 0;
            for line_i in 0..buffer.lines.len() {
                // Break out if we have reached the end of scroll and are past the cursor
                if total_height > scroll_end && line_i > cursor.line {
                    break;
                }

                let line = &mut buffer.lines[line_i];
                if line.metadata().is_some() && line_i < self.syntax_cache.len() {
                    //TODO: duplicated code!
                    if line_i >= scroll.line && total_height < scroll_end {
                        // Perform shaping and layout of this line in order to count if we have reached scroll
                        match buffer.line_layout(font_system, line_i) {
                            Some(layout_lines) => {
                                for layout_line in layout_lines.iter() {
                                    total_height +=
                                        layout_line.line_height_opt.unwrap_or(metrics.line_height);
                                }
                            }
                            None => {
                                //TODO: should this be possible?
                            }
                        }
                    }
                    continue;
                }
                highlighted += 1;

                let (mut parse_state, scope_stack) =
                    if line_i > 0 && line_i <= self.syntax_cache.len() {
                        self.syntax_cache[line_i - 1].clone()
                    } else {
                        (ParseState::new(self.syntax), ScopeStack::new())
                    };
                let mut highlight_state = HighlightState::new(&self.highlighter, scope_stack);
                let ops = parse_state
                    .parse_line(line.text(), &self.syntax_system.syntax_set)
                    .expect("failed to parse syntax");
                let ranges = RangedHighlightIterator::new(
                    &mut highlight_state,
                    &ops,
                    line.text(),
                    &self.highlighter,
                );

                let attrs = line.attrs_list().defaults();
                let mut attrs_list = AttrsList::new(&attrs);
                let original_attrs = attrs.clone(); // Store a clone for comparison
                for (style, _, range) in ranges {
                    let span_attrs = attrs
                        .clone() // Clone attrs for modification
                        .color(Color::rgba(
                            style.foreground.r,
                            style.foreground.g,
                            style.foreground.b,
                            style.foreground.a,
                        ))
                        //TODO: background
                        .style(if style.font_style.contains(FontStyle::ITALIC) {
                            Style::Italic
                        } else {
                            Style::Normal
                        })
                        .weight(if style.font_style.contains(FontStyle::BOLD) {
                            Weight::BOLD
                        } else {
                            Weight::NORMAL
                        })
                        .underline(if style.font_style.contains(FontStyle::UNDERLINE) {
                            UnderlineStyle::Single
                        } else {
                            UnderlineStyle::None
                        });
                    if span_attrs != original_attrs {
                        attrs_list.add_span(range, &span_attrs);
                    }
                }

                // Update line attributes. This operation only resets if the line changes
                line.set_attrs_list(attrs_list);

                // Perform shaping and layout of this line in order to count if we have reached scroll
                if line_i >= scroll.line && total_height < scroll_end {
                    match buffer.line_layout(font_system, line_i) {
                        Some(layout_lines) => {
                            for layout_line in layout_lines.iter() {
                                total_height +=
                                    layout_line.line_height_opt.unwrap_or(metrics.line_height);
                            }
                        }
                        None => {
                            //TODO: should this be possible?
                        }
                    }
                }

                let cache_item = (parse_state.clone(), highlight_state.path.clone());
                if line_i < self.syntax_cache.len() {
                    if self.syntax_cache[line_i] != cache_item {
                        self.syntax_cache[line_i] = cache_item;
                        if line_i + 1 < buffer.lines.len() {
                            buffer.lines[line_i + 1].reset();
                        }
                    }
                } else {
                    buffer.lines[line_i].set_metadata(self.syntax_cache.len());
                    self.syntax_cache.push(cache_item);
                }
            }

            if highlighted > 0 {
                buffer.set_redraw(true);
                #[cfg(feature = "std")]
                log::debug!(
                    "Syntax highlighted {} lines in {:?}",
                    highlighted,
                    now.elapsed()
                );
            }
        });

        self.editor.shape_as_needed(font_system, prune);
    }

    fn delete_range(&mut self, start: Cursor, end: Cursor) {
        self.editor.delete_range(start, end);
    }

    fn insert_at(&mut self, cursor: Cursor, data: &str, attrs_list: Option<AttrsList>) -> Cursor {
        self.editor.insert_at(cursor, data, attrs_list)
    }

    fn copy_selection(&self) -> Option<String> {
        self.editor.copy_selection()
    }

    fn delete_selection(&mut self) -> bool {
        self.editor.delete_selection()
    }

    fn apply_change(&mut self, change: &Change) -> bool {
        self.editor.apply_change(change)
    }

    fn start_change(&mut self) {
        self.editor.start_change();
    }

    fn finish_change(&mut self) -> Option<Change> {
        self.editor.finish_change()
    }

    fn action(&mut self, font_system: &mut FontSystem, action: Action) {
        self.editor.action(font_system, action);
    }

    fn cursor_position(&self) -> Option<(i32, i32)> {
        self.editor.cursor_position()
    }
}

impl BorrowedWithFontSystem<'_, SyntaxEditor<'_, '_>> {
    /// Load text from a file, and also set syntax to the best option
    ///
    /// ## Errors
    ///
    /// Returns an [`io::Error`] if reading the file fails
    #[cfg(feature = "std")]
    pub fn load_text<P: AsRef<Path>>(&mut self, path: P, attrs: crate::Attrs) -> io::Result<()> {
        self.inner.load_text(self.font_system, path, attrs)
    }

    #[cfg(feature = "swash")]
    pub fn draw<F>(&mut self, cache: &mut crate::SwashCache, f: F)
    where
        F: FnMut(i32, i32, u32, u32, Color),
    {
        self.inner.draw(self.font_system, cache, f);
    }
}
