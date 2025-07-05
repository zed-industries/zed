use crate::{
    DisplayRow, EditorSnapshot, EditorStyle, RowRangeExt,
    display_map::ToDisplayPoint,
    element::{LineWithInvisibles, layout_line},
};
use gpui::{App, FontFeatures, Subscription, TextStyleRefinement, Window};
use settings::Settings;
use std::ops::Range;
use std::sync::{Arc, Mutex};
use theme::ThemeSettings;

#[derive(PartialEq, Debug, Clone)]
struct FontFeatureConfig {
    disable_ligatures_on_cursor_lines: bool,
    ligatures_globally_enabled: bool,
}

impl FontFeatureConfig {
    fn from_settings(settings: &ThemeSettings) -> Self {
        let ligatures_globally_enabled = !settings
            .buffer_font
            .features
            .tag_value_list()
            .iter()
            .any(|(tag, val)| tag == "calt" && *val == 0);

        Self {
            disable_ligatures_on_cursor_lines: settings.buffer_ligature_disable_on_cursor_lines,
            ligatures_globally_enabled,
        }
    }

    fn should_track_cursors(&self) -> bool {
        self.disable_ligatures_on_cursor_lines && self.ligatures_globally_enabled
    }
}

#[derive(Clone)]
pub struct DynamicFontFeatureProvider {
    config: FontFeatureConfig,
    cursor_positions: Arc<Mutex<Vec<(u32, u32)>>>,
    cached_ligatures_disabled_features: Option<FontFeatures>,
}

impl DynamicFontFeatureProvider {
    pub fn create(app_cx: &mut App) -> Self {
        let settings = ThemeSettings::get_global(app_cx);
        let config = FontFeatureConfig::from_settings(&settings);

        let cached_ligatures_disabled_features = if config.should_track_cursors() {
            Some(Self::create_ligatures_disabled_features(&settings))
        } else {
            None
        };

        Self {
            config,
            cursor_positions: Arc::new(Mutex::new(Vec::new())),
            cached_ligatures_disabled_features,
        }
    }

    pub fn update_config(&mut self, cx: &mut gpui::Context<crate::Editor>) {
        let settings = ThemeSettings::get_global(cx);
        let new_config = FontFeatureConfig::from_settings(&settings);

        if new_config == self.config {
            return;
        }

        self.config = new_config;
        self.cached_ligatures_disabled_features = self
            .config
            .should_track_cursors()
            .then(|| Self::create_ligatures_disabled_features(&settings));
    }

    fn create_ligatures_disabled_features(settings: &ThemeSettings) -> FontFeatures {
        let base_features = &settings.buffer_font.features;
        let mut modified_features_list: Vec<_> = base_features
            .tag_value_list()
            .iter()
            .filter(|(tag, _)| tag != "calt")
            .cloned()
            .collect();
        modified_features_list.push(("calt".into(), 0));
        FontFeatures(Arc::new(modified_features_list))
    }

    pub fn setup_subscriptions(
        &self,
        editor: &gpui::Entity<crate::Editor>,
        cx: &mut gpui::Context<crate::Editor>,
    ) -> Vec<Subscription> {
        let cursor_positions = self.cursor_positions.clone();

        let subscription = cx.subscribe(editor, move |editor: &mut crate::Editor, _, event, cx| {
            if let crate::EditorEvent::SelectionsChanged { .. } = event {
                let config = &editor.dynamic_font_features.config;

                if config.should_track_cursors() {
                    let display_snapshot =
                        editor.display_map.update(cx, |map, cx| map.snapshot(cx));

                    let positions: Vec<(u32, u32)> = editor
                        .selections
                        .disjoint_anchors()
                        .iter()
                        .map(|selection| {
                            let point = selection.head().to_display_point(&display_snapshot);
                            (point.row().0, point.column())
                        })
                        .collect();

                    if let Ok(mut cursors) = cursor_positions.lock() {
                        *cursors = positions;
                    }

                    cx.notify();
                }
            }
        });
        vec![subscription]
    }

    fn line_has_cursor(&self, line: u32) -> bool {
        self.cursor_positions
            .lock()
            .ok()
            .map(|positions| {
                positions
                    .iter()
                    .any(|(cursor_line, _)| *cursor_line == line)
            })
            .unwrap_or(false)
    }

    /// Returns a text style refinement for the given line if it needs special font features.
    pub fn get_style_for_line(&self, line: u32) -> Option<TextStyleRefinement> {
        if !self.config.should_track_cursors() || !self.line_has_cursor(line) {
            return None;
        }

        let Some(ligatures_disabled_features) = &self.cached_ligatures_disabled_features else {
            return None;
        };

        let mut refinement = TextStyleRefinement::default();
        refinement.font_features = Some(ligatures_disabled_features.clone());
        Some(refinement)
    }

    fn map_buffer_line_ranges(&self, first_line: u32, last_line: u32) -> Vec<(Range<u32>, bool)> {
        let cursor_lines: std::collections::BTreeSet<u32> = self
            .cursor_positions
            .lock()
            .ok()
            .map(|positions| {
                positions
                    .iter()
                    .map(|(line, _)| *line)
                    .filter(|line| *line >= first_line && *line < last_line)
                    .collect()
            })
            .unwrap_or_default();

        if cursor_lines.is_empty() {
            return vec![(first_line..last_line, false)];
        }

        let mut ranges = Vec::new();
        let mut current_line = first_line;

        let mut cursor_iter = cursor_lines.iter().peekable();

        while let Some(&cursor_line) = cursor_iter.next() {
            if current_line < cursor_line {
                ranges.push((current_line..cursor_line, false));
            }

            let group_start = cursor_line;
            let mut group_end = cursor_line + 1;

            while let Some(&&next_cursor) = cursor_iter.peek() {
                if next_cursor == group_end {
                    group_end += 1;
                    cursor_iter.next();
                } else {
                    break;
                }
            }

            ranges.push((group_start..group_end, true));
            current_line = group_end;
        }

        if current_line < last_line {
            ranges.push((current_line..last_line, false));
        }

        ranges
    }

    pub(crate) fn layout_lines_with_features(
        &self,
        rows: Range<DisplayRow>,
        snapshot: &EditorSnapshot,
        style: &EditorStyle,
        editor_width: gpui::Pixels,
        is_row_soft_wrapped: impl Copy + Fn(usize) -> bool,
        window: &mut Window,
        cx: &mut App,
        max_line_len: usize,
    ) -> Vec<LineWithInvisibles> {
        let first_line_number = rows.start.0;
        let last_line_number = rows.end.0;
        let line_ranges = self.map_buffer_line_ranges(first_line_number, last_line_number);

        if !self.config.should_track_cursors()
            || line_ranges.iter().all(|(_, has_cursors)| !has_cursors)
        {
            let chunks = snapshot.highlighted_chunks(rows.clone(), true, style);
            return LineWithInvisibles::from_chunks(
                chunks,
                &style,
                max_line_len,
                rows.len(),
                &snapshot.mode,
                editor_width,
                is_row_soft_wrapped,
                window,
                cx,
            );
        }

        let mut styled_lines: Vec<LineWithInvisibles> = Vec::new();

        for (range, has_cursors) in line_ranges {
            if has_cursors {
                for line_number in range.start..range.end {
                    styled_lines.push(layout_line(
                        DisplayRow(line_number),
                        snapshot,
                        style,
                        editor_width,
                        is_row_soft_wrapped,
                        window,
                        cx,
                        true,
                    ));
                }
            } else {
                let chunks = snapshot.highlighted_chunks(
                    DisplayRow(range.start)..DisplayRow(range.end),
                    true,
                    style,
                );

                styled_lines.extend(LineWithInvisibles::from_chunks(
                    chunks,
                    &style,
                    max_line_len,
                    (range.end - range.start) as usize,
                    &snapshot.mode,
                    editor_width,
                    is_row_soft_wrapped,
                    window,
                    cx,
                ));
            }
        }

        styled_lines
    }
}
