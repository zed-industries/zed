//! Resolution of LSP snippet variables as described in the LSP snippet syntax specification:
//! <https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/#snippet_syntax>

use std::ops::Range;

use gpui::App;
use multi_buffer::{
    MultiBufferOffset, MultiBufferRow, MultiBufferSnapshot, ToOffset as _, ToPoint as _,
};
use text::Point;
use time::{Month, OffsetDateTime, Weekday};

use crate::Editor;

pub(crate) struct SnippetVariableContext {
    clipboard: Option<String>,
    filename: Option<String>,
    filename_base: Option<String>,
    directory: Option<String>,
    filepath: Option<String>,
    relative_filepath: Option<String>,
    workspace_name: Option<String>,
    workspace_folder: Option<String>,
    now: OffsetDateTime,
}

impl SnippetVariableContext {
    pub(crate) fn new(editor: &Editor, cx: &App) -> Self {
        let clipboard = cx
            .read_from_clipboard()
            .and_then(|item| item.text())
            .filter(|text| !text.is_empty());

        let mut context = Self {
            clipboard,
            filename: None,
            filename_base: None,
            directory: None,
            filepath: None,
            relative_filepath: None,
            workspace_name: None,
            workspace_folder: None,
            now: OffsetDateTime::now_local().unwrap_or_else(|_| OffsetDateTime::now_utc()),
        };

        if let Some(buffer) = editor.buffer().read(cx).as_singleton() {
            let buffer = buffer.read(cx);

            if let Some(file) = buffer.file() {
                context.filename = Some(file.file_name(cx).to_string());
                context.filename_base = std::path::Path::new(file.file_name(cx))
                    .file_stem()
                    .map(|stem| stem.to_string_lossy().into_owned());
                context.relative_filepath =
                    Some(file.path().display(file.path_style(cx)).into_owned());

                if let Some(local_file) = file.as_local() {
                    let abs_path = local_file.abs_path(cx);
                    context.directory = abs_path
                        .parent()
                        .map(|parent| parent.to_string_lossy().into_owned());
                    context.filepath = Some(abs_path.to_string_lossy().into_owned());
                }

                if let Some(project) = editor.project.as_ref() {
                    let worktree_id = file.worktree_id(cx);
                    if let Some(worktree) = project.read(cx).worktree_for_id(worktree_id, cx) {
                        let worktree = worktree.read(cx);
                        context.workspace_folder =
                            Some(worktree.abs_path().to_string_lossy().into_owned());
                        context.workspace_name = Some(worktree.root_name_str().to_string());
                    }
                }
            }
        }

        context
    }

    pub(crate) fn resolve(
        &self,
        name: &str,
        selected_text: &str,
        snapshot: &MultiBufferSnapshot,
        range: &Range<MultiBufferOffset>,
        cursor_index: usize,
    ) -> Option<String> {
        match name {
            "TM_SELECTED_TEXT" => Some(selected_text.to_string()),
            "TM_CURRENT_LINE" => Some(self.current_line(snapshot, range)),
            "TM_CURRENT_WORD" => Some(self.current_word(snapshot, range)),
            "TM_LINE_INDEX" => Some(self.line_index(snapshot, range).to_string()),
            "TM_LINE_NUMBER" => Some((self.line_index(snapshot, range) + 1).to_string()),
            "TM_FILENAME" => self.filename.clone(),
            "TM_FILENAME_BASE" => self.filename_base.clone(),
            "TM_DIRECTORY" => self.directory.clone(),
            "TM_FILEPATH" => self.filepath.clone(),
            "RELATIVE_FILEPATH" => self.relative_filepath.clone(),
            "CLIPBOARD" => self.clipboard.clone(),
            "WORKSPACE_NAME" => self.workspace_name.clone(),
            "WORKSPACE_FOLDER" => self.workspace_folder.clone(),
            "CURSOR_INDEX" => Some(cursor_index.to_string()),
            "CURSOR_NUMBER" => Some((cursor_index + 1).to_string()),
            "LINE_COMMENT" => snapshot
                .language_scope_at(range.start)
                .and_then(|scope| scope.line_comment_prefixes().first().cloned())
                .map(|prefix| prefix.trim_end().to_string()),
            "BLOCK_COMMENT_START" => snapshot.language_scope_at(range.start).and_then(|scope| {
                scope
                    .block_comment()
                    .map(|comment| comment.start.trim().to_string())
            }),
            "BLOCK_COMMENT_END" => snapshot.language_scope_at(range.start).and_then(|scope| {
                scope
                    .block_comment()
                    .map(|comment| comment.end.trim().to_string())
            }),
            "CURRENT_YEAR" => Some(format!("{:04}", self.now.year())),
            "CURRENT_YEAR_SHORT" => Some(format!("{:02}", self.now.year() % 100)),
            "CURRENT_MONTH" => Some(format!("{:02}", self.now.month() as u8)),
            "CURRENT_MONTH_NAME" => Some(month_name(self.now.month()).to_string()),
            "CURRENT_MONTH_NAME_SHORT" => Some(month_name(self.now.month())[..3].to_string()),
            "CURRENT_DATE" => Some(format!("{:02}", self.now.day())),
            "CURRENT_DAY_NAME" => Some(weekday_name(self.now.weekday()).to_string()),
            "CURRENT_DAY_NAME_SHORT" => Some(weekday_name(self.now.weekday())[..3].to_string()),
            "CURRENT_HOUR" => Some(format!("{:02}", self.now.hour())),
            "CURRENT_MINUTE" => Some(format!("{:02}", self.now.minute())),
            "CURRENT_SECOND" => Some(format!("{:02}", self.now.second())),
            "CURRENT_SECONDS_UNIX" => Some(self.now.unix_timestamp().to_string()),
            "CURRENT_TIMEZONE_OFFSET" => Some(timezone_offset(&self.now)),
            "RANDOM" => Some(format!("{:06}", rand::random_range(0..1_000_000u32))),
            "RANDOM_HEX" => Some(format!("{:06x}", rand::random_range(0..0x100_0000u32))),
            "UUID" => Some(uuid::Uuid::new_v4().to_string()),
            _ => None,
        }
    }

    fn line_index(&self, snapshot: &MultiBufferSnapshot, range: &Range<MultiBufferOffset>) -> u32 {
        range.start.to_point(snapshot).row
    }

    fn current_line(
        &self,
        snapshot: &MultiBufferSnapshot,
        range: &Range<MultiBufferOffset>,
    ) -> String {
        let row = range.start.to_point(snapshot).row;
        let line_start = Point::new(row, 0).to_offset(snapshot);
        let line_end = Point::new(row, snapshot.line_len(MultiBufferRow(row))).to_offset(snapshot);
        snapshot.text_for_range(line_start..line_end).collect()
    }

    fn current_word(
        &self,
        snapshot: &MultiBufferSnapshot,
        range: &Range<MultiBufferOffset>,
    ) -> String {
        let (word_range, _) = snapshot.surrounding_word(range.start, None);
        snapshot.text_for_range(word_range).collect()
    }
}

fn month_name(month: Month) -> &'static str {
    match month {
        Month::January => "January",
        Month::February => "February",
        Month::March => "March",
        Month::April => "April",
        Month::May => "May",
        Month::June => "June",
        Month::July => "July",
        Month::August => "August",
        Month::September => "September",
        Month::October => "October",
        Month::November => "November",
        Month::December => "December",
    }
}

fn weekday_name(weekday: Weekday) -> &'static str {
    match weekday {
        Weekday::Monday => "Monday",
        Weekday::Tuesday => "Tuesday",
        Weekday::Wednesday => "Wednesday",
        Weekday::Thursday => "Thursday",
        Weekday::Friday => "Friday",
        Weekday::Saturday => "Saturday",
        Weekday::Sunday => "Sunday",
    }
}

fn timezone_offset(now: &OffsetDateTime) -> String {
    let offset = now.offset();
    let sign = if offset.whole_seconds() < 0 { '-' } else { '+' };
    let (hours, minutes, _) = offset.as_hms();
    format!(
        "{sign}{:02}:{:02}",
        hours.unsigned_abs(),
        minutes.unsigned_abs()
    )
}
