use std::path::Path;

use crate::{source_location::SourceLocation, training::teacher::TeacherModel};

#[derive(Debug, Clone, Default, clap::ValueEnum)]
pub enum ContextType {
    #[default]
    CurrentFile,
}

const MAX_CONTEXT_SIZE: usize = 32768;

pub fn collect_context(
    context_type: &ContextType,
    worktree_dir: &Path,
    cursor: SourceLocation,
) -> String {
    let context = match context_type {
        ContextType::CurrentFile => {
            let file_path = worktree_dir.join(cursor.path.as_std_path());
            let context = std::fs::read_to_string(&file_path).unwrap_or_default();

            let context = add_special_tags(&context, worktree_dir, cursor);
            context
        }
    };

    let region_end_offset = context.find(TeacherModel::REGION_END);

    if context.len() <= MAX_CONTEXT_SIZE {
        return context;
    }

    if let Some(region_end_offset) = region_end_offset
        && region_end_offset + TeacherModel::REGION_END.len() > MAX_CONTEXT_SIZE
    {
        let to_truncate = context.len() - MAX_CONTEXT_SIZE;
        format!(
            "[...{} bytes truncated]\n{}\n",
            to_truncate,
            &context[to_truncate..]
        )
    } else {
        format!(
            "{}\n[...{} bytes truncated]\n",
            &context[..MAX_CONTEXT_SIZE],
            context.len() - MAX_CONTEXT_SIZE
        )
    }
}

/// Add <|editable_region_start/end|> tags
fn add_special_tags(context: &str, worktree_dir: &Path, cursor: SourceLocation) -> String {
    let path = worktree_dir.join(cursor.path.as_std_path());
    let file = std::fs::read_to_string(&path).unwrap_or_default();
    let lines = file.lines().collect::<Vec<_>>();
    let cursor_row = cursor.point.row as usize;
    let start_line = cursor_row.saturating_sub(TeacherModel::LEFT_CONTEXT_SIZE);
    let end_line = (cursor_row + TeacherModel::RIGHT_CONTEXT_SIZE).min(lines.len());

    let snippet = lines[start_line..end_line].join("\n");

    if context.contains(&snippet) {
        let mut cursor_line = lines[cursor_row].to_string();
        cursor_line.insert_str(cursor.point.column as usize, TeacherModel::USER_CURSOR);

        let mut snippet_with_tags_lines = vec![];
        snippet_with_tags_lines.push(TeacherModel::REGION_START);
        snippet_with_tags_lines.extend(&lines[start_line..cursor_row]);
        snippet_with_tags_lines.push(&cursor_line);
        snippet_with_tags_lines.extend(&lines[cursor_row + 1..end_line]);
        snippet_with_tags_lines.push(TeacherModel::REGION_END);
        let snippet_with_tags = snippet_with_tags_lines.join("\n");

        context.replace(&snippet, &snippet_with_tags)
    } else {
        log::warn!(
            "Can't find area around the cursor in the context; proceeding without special tags"
        );
        context.to_string()
    }
}

pub fn strip_special_tags(context: &str) -> String {
    context
        .replace(TeacherModel::REGION_START, "")
        .replace(TeacherModel::REGION_END, "")
        .replace(TeacherModel::USER_CURSOR, "")
}
