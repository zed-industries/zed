use super::*;

impl Editor {
    pub fn style(&mut self, cx: &App) -> &EditorStyle {
        match self.style {
            Some(ref style) => style,
            None => {
                let style = self.create_style(cx);
                self.style.insert(style)
            }
        }
    }

    pub fn set_soft_wrap_mode(
        &mut self,
        mode: language_settings::SoftWrap,
        cx: &mut Context<Self>,
    ) {
        self.soft_wrap_mode_override = Some(mode);
        cx.notify();
    }

    pub fn set_hard_wrap(&mut self, hard_wrap: Option<usize>, cx: &mut Context<Self>) {
        self.hard_wrap = hard_wrap;
        cx.notify();
    }

    pub fn set_text_style_refinement(&mut self, style: TextStyleRefinement) {
        self.text_style_refinement = Some(style);
    }

    /// called by the Element so we know what style we were most recently rendered with.
    pub fn set_style(&mut self, style: EditorStyle, window: &mut Window, cx: &mut Context<Self>) {
        // We intentionally do not inform the display map about the minimap style
        // so that wrapping is not recalculated and stays consistent for the editor
        // and its linked minimap.
        if !self.mode.is_minimap() {
            let font = style.text.font();
            let font_size = style.text.font_size.to_pixels(window.rem_size());
            let display_map = self
                .placeholder_display_map
                .as_ref()
                .filter(|_| self.is_empty(cx))
                .unwrap_or(&self.display_map);

            display_map.update(cx, |map, cx| map.set_font(font, font_size, cx));
        }
        self.style = Some(style);
    }

    pub fn set_soft_wrap(&mut self) {
        self.soft_wrap_mode_override = Some(language_settings::SoftWrap::EditorWidth)
    }

    pub fn set_show_wrap_guides(&mut self, show_wrap_guides: bool, cx: &mut Context<Self>) {
        self.show_wrap_guides = Some(show_wrap_guides);
        cx.notify();
    }

    pub fn set_show_indent_guides(&mut self, show_indent_guides: bool, cx: &mut Context<Self>) {
        self.show_indent_guides = Some(show_indent_guides);
        cx.notify();
    }

    pub fn disable_indent_guides_for_buffer(
        &mut self,
        buffer_id: BufferId,
        cx: &mut Context<Self>,
    ) {
        self.buffers_with_disabled_indent_guides.insert(buffer_id);
        cx.notify();
    }

    pub fn toggle_line_numbers(
        &mut self,
        _: &ToggleLineNumbers,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let mut editor_settings = EditorSettings::get_global(cx).clone();
        editor_settings.gutter.line_numbers = !editor_settings.gutter.line_numbers;
        EditorSettings::override_global(editor_settings, cx);
    }

    pub fn line_numbers_enabled(&self, cx: &App) -> bool {
        if let Some(show_line_numbers) = self.show_line_numbers {
            return show_line_numbers;
        }
        EditorSettings::get_global(cx).gutter.line_numbers
    }

    pub fn relative_line_numbers(&self, cx: &App) -> RelativeLineNumbers {
        match (
            self.use_relative_line_numbers,
            EditorSettings::get_global(cx).relative_line_numbers,
        ) {
            (None, setting) => setting,
            (Some(false), _) => RelativeLineNumbers::Disabled,
            (Some(true), RelativeLineNumbers::Wrapped) => RelativeLineNumbers::Wrapped,
            (Some(true), _) => RelativeLineNumbers::Enabled,
        }
    }

    pub fn set_relative_line_number(&mut self, is_relative: Option<bool>, cx: &mut Context<Self>) {
        self.use_relative_line_numbers = is_relative;
        cx.notify();
    }

    pub fn set_show_gutter(&mut self, show_gutter: bool, cx: &mut Context<Self>) {
        self.show_gutter = show_gutter;
        cx.notify();
    }

    pub fn set_show_vertical_scrollbar(&mut self, show: bool, cx: &mut Context<Self>) {
        self.show_scrollbars.vertical = show;
        cx.notify();
    }

    pub fn set_show_horizontal_scrollbar(&mut self, show: bool, cx: &mut Context<Self>) {
        self.show_scrollbars.horizontal = show;
        cx.notify();
    }

    pub fn set_minimap_visibility(
        &mut self,
        minimap_visibility: MinimapVisibility,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.minimap_visibility != minimap_visibility {
            if minimap_visibility.visible() && self.minimap.is_none() {
                let minimap_settings = EditorSettings::get_global(cx).minimap;
                self.minimap =
                    self.create_minimap(minimap_settings.with_show_override(), window, cx);
            }
            self.minimap_visibility = minimap_visibility;
            cx.notify();
        }
    }

    pub fn disable_scrollbars_and_minimap(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.set_show_scrollbars(false, cx);
        self.set_minimap_visibility(MinimapVisibility::Disabled, window, cx);
    }

    pub fn hide_minimap_by_default(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.set_minimap_visibility(self.minimap_visibility.hidden(), window, cx);
    }

    /// Normally the text in full mode and auto height editors is padded on the
    /// left side by roughly half a character width for improved hit testing.
    ///
    /// Use this method to disable this for cases where this is not wanted (e.g.
    /// if you want to align the editor text with some other text above or below)
    /// or if you want to add this padding to single-line editors.
    pub fn set_offset_content(&mut self, offset_content: bool, cx: &mut Context<Self>) {
        self.offset_content = offset_content;
        cx.notify();
    }

    pub fn set_show_line_numbers(&mut self, show_line_numbers: bool, cx: &mut Context<Self>) {
        self.show_line_numbers = Some(show_line_numbers);
        cx.notify();
    }

    pub fn disable_expand_excerpt_buttons(&mut self, cx: &mut Context<Self>) {
        self.disable_expand_excerpt_buttons = true;
        cx.notify();
    }

    pub fn set_show_git_diff_gutter(&mut self, show_git_diff_gutter: bool, cx: &mut Context<Self>) {
        self.show_git_diff_gutter = Some(show_git_diff_gutter);
        cx.notify();
    }

    pub fn set_show_code_actions(&mut self, show_code_actions: bool, cx: &mut Context<Self>) {
        self.show_code_actions = Some(show_code_actions);
        cx.notify();
    }

    pub fn set_show_runnables(&mut self, show_runnables: bool, cx: &mut Context<Self>) {
        self.show_runnables = Some(show_runnables);
        cx.notify();
    }

    pub fn set_show_breakpoints(&mut self, show_breakpoints: bool, cx: &mut Context<Self>) {
        self.show_breakpoints = Some(show_breakpoints);
        cx.notify();
    }

    pub fn set_show_diff_review_button(&mut self, show: bool, cx: &mut Context<Self>) {
        self.show_diff_review_button = show;
        cx.notify();
    }

    fn set_show_scrollbars(&mut self, show: bool, cx: &mut Context<Self>) {
        self.show_scrollbars = ScrollbarAxes {
            horizontal: show,
            vertical: show,
        };
        cx.notify();
    }

    pub(super) fn wrap_guides(&self, cx: &App) -> SmallVec<[(usize, bool); 2]> {
        let mut wrap_guides = smallvec![];

        if self.show_wrap_guides == Some(false) {
            return wrap_guides;
        }

        let settings = self.buffer.read(cx).language_settings(cx);
        if settings.show_wrap_guides {
            match self.soft_wrap_mode(cx) {
                SoftWrap::Bounded(soft_wrap) => {
                    wrap_guides.push((soft_wrap as usize, true));
                }
                SoftWrap::GitDiff | SoftWrap::None | SoftWrap::EditorWidth => {}
            }
            wrap_guides.extend(settings.wrap_guides.iter().map(|guide| (*guide, false)))
        }

        wrap_guides
    }

    pub(super) fn soft_wrap_mode(&self, cx: &App) -> SoftWrap {
        let settings = self.buffer.read(cx).language_settings(cx);
        let mode = self.soft_wrap_mode_override.unwrap_or(settings.soft_wrap);
        match mode {
            language_settings::SoftWrap::PreferLine | language_settings::SoftWrap::None => {
                SoftWrap::None
            }
            language_settings::SoftWrap::EditorWidth => SoftWrap::EditorWidth,
            language_settings::SoftWrap::Bounded => {
                SoftWrap::Bounded(settings.preferred_line_length)
            }
        }
    }

    // Called by the element. This method is not designed to be called outside of the editor
    // element's layout code because it does not notify when rewrapping is computed synchronously.
    pub(super) fn set_wrap_width(&self, width: Option<Pixels>, cx: &mut App) -> bool {
        if self.is_empty(cx) {
            self.placeholder_display_map
                .as_ref()
                .map_or(false, |display_map| {
                    display_map.update(cx, |map, cx| map.set_wrap_width(width, cx))
                })
        } else {
            self.display_map
                .update(cx, |map, cx| map.set_wrap_width(width, cx))
        }
    }

    pub(super) fn toggle_soft_wrap(
        &mut self,
        _: &ToggleSoftWrap,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.soft_wrap_mode_override.is_some() {
            self.soft_wrap_mode_override.take();
        } else {
            let soft_wrap = match self.soft_wrap_mode(cx) {
                SoftWrap::GitDiff => return,
                SoftWrap::None => language_settings::SoftWrap::EditorWidth,
                SoftWrap::EditorWidth | SoftWrap::Bounded(_) => language_settings::SoftWrap::None,
            };
            self.soft_wrap_mode_override = Some(soft_wrap);
        }
        cx.notify();
    }

    pub(super) fn toggle_tab_bar(
        &mut self,
        _: &ToggleTabBar,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(workspace) = self.workspace() else {
            return;
        };
        let fs = workspace.read(cx).app_state().fs.clone();
        let current_show = TabBarSettings::get_global(cx).show;
        update_settings_file(fs, cx, move |setting, _| {
            setting.tab_bar.get_or_insert_default().show = Some(!current_show);
        });
    }

    pub(super) fn toggle_indent_guides(
        &mut self,
        _: &ToggleIndentGuides,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let currently_enabled = self.should_show_indent_guides().unwrap_or_else(|| {
            self.buffer
                .read(cx)
                .language_settings(cx)
                .indent_guides
                .enabled
        });
        self.show_indent_guides = Some(!currently_enabled);
        cx.notify();
    }

    pub(super) fn should_show_indent_guides(&self) -> Option<bool> {
        self.show_indent_guides
    }

    pub(super) fn has_indent_guides_disabled_for_buffer(&self, buffer_id: BufferId) -> bool {
        self.buffers_with_disabled_indent_guides
            .contains(&buffer_id)
    }

    pub(super) fn toggle_relative_line_numbers(
        &mut self,
        _: &ToggleRelativeLineNumbers,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let is_relative = self.relative_line_numbers(cx);
        self.set_relative_line_number(Some(!is_relative.enabled()), cx)
    }

    pub(super) fn set_number_deleted_lines(&mut self, number: bool, cx: &mut Context<Self>) {
        self.number_deleted_lines = number;
        cx.notify();
    }

    pub fn set_delegate_open_excerpts(&mut self, delegate: bool) {
        self.delegate_open_excerpts = delegate;
    }

    pub(super) fn set_delegate_expand_excerpts(&mut self, delegate: bool) {
        self.delegate_expand_excerpts = delegate;
    }

    pub(super) fn set_delegate_stage_and_restore(&mut self, delegate: bool) {
        self.delegate_stage_and_restore = delegate;
    }

    pub(super) fn set_on_local_selections_changed(
        &mut self,
        callback: Option<Box<dyn Fn(Point, &mut Window, &mut Context<Self>) + 'static>>,
    ) {
        self.on_local_selections_changed = callback;
    }

    pub(super) fn set_suppress_selection_callback(&mut self, suppress: bool) {
        self.suppress_selection_callback = suppress;
    }
}
