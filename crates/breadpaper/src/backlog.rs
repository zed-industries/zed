//! The Backlog file model (spec `v6-backlog.md`): parses `backlog.md` into
//! Soon / Someday / Completed sections of checklist tasks with byte spans, and
//! produces surgical span edits — rename in place, append to a section, move
//! to Completed with a date stamp — so every rewrite preserves content the
//! model doesn't understand (prose, unknown sections, blank-line style). Pure
//! functions over strings — no GPUI.

use chrono::NaiveDate;
use std::ops::Range;

use crate::day_plan::heading_level_and_text;

pub const DEFAULT_BACKLOG: &str = r#"# Backlog

<!-- Soon = tasks for the coming days. Someday = worth keeping, no commitment.
     Checking a task off in the Backlog panel records it in today's note and
     files it here under Completed with the date. -->

## Soon

## Someday

## Completed
"#;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SectionKind {
    Soon,
    Someday,
    Completed,
}

impl SectionKind {
    pub const ALL: [SectionKind; 3] = [Self::Soon, Self::Someday, Self::Completed];

    pub fn heading(self) -> &'static str {
        match self {
            Self::Soon => "Soon",
            Self::Someday => "Someday",
            Self::Completed => "Completed",
        }
    }
}

/// A top-level checklist task inside one of the backlog's sections.
#[derive(Debug, Clone, PartialEq)]
pub struct BacklogTask {
    /// 0-based line of the task's checkbox line.
    pub line: u32,
    /// Byte span of the task line plus its indented children, including the
    /// trailing newline (when present).
    pub span: Range<usize>,
    /// Byte span of the task text on the checkbox line (after the `- [ ] `).
    pub text_span: Range<usize>,
    pub text: String,
    pub checked: bool,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Backlog {
    pub soon: Vec<BacklogTask>,
    pub someday: Vec<BacklogTask>,
    pub completed: Vec<BacklogTask>,
}

impl Backlog {
    pub fn section(&self, kind: SectionKind) -> &[BacklogTask] {
        match kind {
            SectionKind::Soon => &self.soon,
            SectionKind::Someday => &self.someday,
            SectionKind::Completed => &self.completed,
        }
    }

    /// True when the open sections hold no open (unchecked) tasks — the
    /// panel's empty state. Hand-checked tasks left in Soon/Someday don't
    /// count as open work.
    pub fn is_empty(&self) -> bool {
        let all_checked = |tasks: &[BacklogTask]| tasks.iter().all(|task| task.checked);
        all_checked(&self.soon) && all_checked(&self.someday)
    }

    /// Finds the task a panel gesture addressed: section + line + text as
    /// they were when the gesture started. The line disambiguates duplicate
    /// texts (spec §6.5: addressing is by line, not text); when lines have
    /// shifted underneath the gesture, an *unambiguous* text match still
    /// resolves. `None` means the task can't be identified safely and the
    /// caller should drop the gesture rather than guess.
    pub fn locate_task(&self, kind: SectionKind, line: u32, text: &str) -> Option<&BacklogTask> {
        let tasks = self.section(kind);
        tasks
            .iter()
            .find(|task| task.line == line && task.text == text)
            .or_else(|| {
                let mut matches = tasks.iter().filter(|task| task.text == text);
                let first = matches.next()?;
                matches.next().is_none().then_some(first)
            })
    }

    /// Whether any section (Completed included) already holds `text`, under
    /// the dedup normalization of spec §7.2.
    pub fn contains_task(&self, text: &str) -> bool {
        let wanted = normalize_task_text(text);
        SectionKind::ALL.iter().any(|&kind| {
            self.section(kind)
                .iter()
                .any(|task| normalize_task_text(&task.text) == wanted)
        })
    }
}

/// A single span replacement against the text the model was parsed from.
/// Ranges of one edit batch all address the original text; `apply_edits`
/// (and GPUI's `Buffer::edit`) resolve the offset shifts.
#[derive(Debug, Clone, PartialEq)]
pub struct Edit {
    pub range: Range<usize>,
    pub new_text: String,
}

/// Applies disjoint edits to `text`, back to front so earlier ranges stay
/// valid.
pub fn apply_edits(text: &str, mut edits: Vec<Edit>) -> String {
    edits.sort_by_key(|edit| std::cmp::Reverse(edit.range.start));
    let mut result = text.to_string();
    for edit in edits {
        debug_assert!(
            text.get(edit.range.clone()).is_some(),
            "edit range {:?} is out of bounds or off a char boundary",
            edit.range
        );
        if text.get(edit.range.clone()).is_some() {
            result.replace_range(edit.range, &edit.new_text);
        }
    }
    result
}

/// A line of the source text: its content (without the newline) and its byte
/// span (with the newline, when present).
struct Line<'a> {
    content: &'a str,
    start: usize,
    /// End of the line including its `\n` (== `content` end at EOF).
    end: usize,
}

fn line_table(text: &str) -> Vec<Line<'_>> {
    let mut lines = Vec::new();
    let mut start = 0;
    for segment in text.split_inclusive('\n') {
        lines.push(Line {
            content: segment.strip_suffix('\n').unwrap_or(segment),
            start,
            end: start + segment.len(),
        });
        start += segment.len();
    }
    lines
}

/// The lines belonging to the first heading matching `heading`
/// (case-insensitively, at any level), ending before the next heading of
/// equal or higher level. `None` when the heading doesn't exist. Unicode
/// lowercasing, matching the Day Planner's heading resolution, so the two
/// features agree on which section a configured heading names.
fn section_line_range(lines: &[Line<'_>], heading: &str) -> Option<Range<usize>> {
    let wanted = heading.to_lowercase();
    let (start, level) = lines.iter().enumerate().find_map(|(index, line)| {
        let (level, text) = heading_level_and_text(line.content)?;
        (text.to_lowercase() == wanted).then_some((index, level))
    })?;
    let end = lines[start + 1..]
        .iter()
        .position(|line| {
            heading_level_and_text(line.content).is_some_and(|(other, _)| other <= level)
        })
        .map(|offset| start + 1 + offset)
        .unwrap_or(lines.len());
    Some(start + 1..end)
}

/// Matches a **top-level** `- [ ]` / `- [x]` list item (also `*` / `+`
/// bullets, like the Day Planner). Returns the checked state and the byte
/// column where the task text starts. Indented checkboxes are children, not
/// tasks.
fn parse_top_level_task(line: &str) -> Option<(bool, usize)> {
    if line.starts_with([' ', '\t']) {
        return None;
    }
    let after_bullet = line.strip_prefix(['-', '*', '+'])?;
    let after_space = after_bullet.trim_start_matches([' ', '\t']);
    if after_space.len() == after_bullet.len() {
        return None;
    }
    let mut chars = after_space.strip_prefix('[')?.chars();
    let checked = match chars.next()? {
        ' ' => false,
        'x' | 'X' => true,
        _ => return None,
    };
    let after_checkbox = chars.as_str().strip_prefix(']')?;
    let text = after_checkbox.trim_start();
    if text.len() == after_checkbox.len() && !text.is_empty() {
        return None;
    }
    Some((checked, line.len() - text.len()))
}

/// Indented, non-blank lines beneath a task are its children and travel with
/// it (spec §5.1). Blank lines *between* children belong to the run (loose
/// lists), but trailing blank lines after the last child stay put.
fn is_child_line(line: &str) -> bool {
    line.starts_with([' ', '\t']) && !line.trim().is_empty()
}

pub fn parse_backlog(text: &str) -> Backlog {
    let lines = line_table(text);
    let mut backlog = Backlog::default();
    for kind in SectionKind::ALL {
        let Some(range) = section_line_range(&lines, kind.heading()) else {
            continue;
        };
        let tasks = match kind {
            SectionKind::Soon => &mut backlog.soon,
            SectionKind::Someday => &mut backlog.someday,
            SectionKind::Completed => &mut backlog.completed,
        };
        let mut row = range.start;
        while row < range.end {
            let Some(line) = lines.get(row) else {
                break;
            };
            let Some((checked, text_column)) = parse_top_level_task(line.content) else {
                row += 1;
                continue;
            };
            let mut last_row = row;
            let mut probe = row + 1;
            while probe < range.end {
                let Some(next) = lines.get(probe) else {
                    break;
                };
                if is_child_line(next.content) {
                    last_row = probe;
                } else if !next.content.trim().is_empty() {
                    break;
                }
                probe += 1;
            }
            tasks.push(BacklogTask {
                line: row as u32,
                span: line.start..lines[last_row].end,
                text_span: line.start + text_column..line.start + line.content.len(),
                text: line.content[text_column..].to_string(),
                checked,
            });
            row = last_row + 1;
        }
    }
    backlog
}

/// Rewrites just the task's text in place — bullet marker, checkbox state,
/// and children untouched (spec §6.2).
pub fn rename_task_edit(task: &BacklogTask, new_text: &str) -> Edit {
    Edit {
        range: task.text_span.clone(),
        new_text: new_text.to_string(),
    }
}

/// The task line + children as a newline-terminated block, verbatim.
fn task_block(text: &str, task: &BacklogTask) -> String {
    let raw = text.get(task.span.clone()).unwrap_or("");
    if raw.ends_with('\n') {
        raw.to_string()
    } else {
        format!("{raw}\n")
    }
}

/// A brand-new open task line for `append_to_section_edit`.
pub fn new_task_block(text: &str) -> String {
    format!("- [ ] {}\n", text.trim())
}

/// Moves the task (with children) out of its section and appends it to the
/// end of Completed, checked off and stamped ` ✅ <date>` (spec §6.3 step 2).
pub fn complete_task_edits(text: &str, task: &BacklogTask, date: NaiveDate) -> Vec<Edit> {
    let block = task_block(text, task);
    let (first_line, children) = block.split_once('\n').unwrap_or((block.as_str(), ""));
    let first_line = if task.checked {
        first_line.trim_end().to_string()
    } else {
        // Flip the checkbox: the marker is the first `[ ]` on the line
        // (before the task text, which starts at `text_span`).
        first_line.trim_end().replacen("[ ]", "[x]", 1)
    };
    let mut completed_block = format!("{first_line} ✅ {}\n", date.format("%Y-%m-%d"));
    completed_block.push_str(children);
    if !children.is_empty() && !children.ends_with('\n') {
        completed_block.push('\n');
    }
    vec![
        Edit {
            range: task.span.clone(),
            new_text: String::new(),
        },
        append_to_section_edit(text, SectionKind::Completed, &completed_block),
    ]
}

/// Moves the task (with children) verbatim to the end of another section.
pub fn move_task_edits(text: &str, task: &BacklogTask, to: SectionKind) -> Vec<Edit> {
    vec![
        Edit {
            range: task.span.clone(),
            new_text: String::new(),
        },
        append_to_section_edit(text, to, &task_block(text, task)),
    ]
}

/// The edit inserting `block` (newline-terminated lines) at the end of a
/// section's content: after its last non-blank line, right after the heading
/// when the section is empty, or — when the heading doesn't exist yet —
/// appending the heading at the end of the file (create-if-missing, never
/// clobber; spec §5.1).
pub fn append_to_section_edit(text: &str, kind: SectionKind, block: &str) -> Edit {
    let lines = line_table(text);
    match section_line_range(&lines, kind.heading()) {
        Some(range) => {
            let anchor = lines[range.clone()]
                .iter()
                .rposition(|line| !line.content.trim().is_empty())
                .map(|offset| range.start + offset);
            match anchor {
                Some(row) => insert_after_line(&lines, row, block.to_string()),
                // Empty section: land right under the heading, keeping one
                // blank line between them.
                None => insert_after_line(&lines, range.start - 1, format!("\n{block}")),
            }
        }
        None => {
            let prefix = if text.is_empty() {
                ""
            } else if text.ends_with('\n') {
                "\n"
            } else {
                "\n\n"
            };
            Edit {
                range: text.len()..text.len(),
                new_text: format!("{prefix}## {}\n\n{block}", kind.heading()),
            }
        }
    }
}

/// The edit appending `- [x] <task text>` at the end of a daily note's
/// planner/task section (matched like the Day Planner panel), or at the end
/// of the file when the heading is missing (spec §6.3 step 1). Never touches
/// existing content.
pub fn append_done_to_note_edit(note_text: &str, heading: &str, task_text: &str) -> Edit {
    let line = format!("- [x] {task_text}\n");
    let lines = line_table(note_text);
    let section = heading
        .trim()
        .is_empty()
        .then_some(None)
        .unwrap_or_else(|| section_line_range(&lines, heading.trim()));
    match section {
        Some(range) => {
            let anchor = lines[range.clone()]
                .iter()
                .rposition(|entry| !entry.content.trim().is_empty())
                .map(|offset| range.start + offset)
                .unwrap_or(range.start - 1);
            insert_after_line(&lines, anchor, line)
        }
        None => {
            let prefix = if note_text.is_empty() || note_text.ends_with('\n') {
                ""
            } else {
                "\n"
            };
            Edit {
                range: note_text.len()..note_text.len(),
                new_text: format!("{prefix}{line}"),
            }
        }
    }
}

/// Inserts `new_text` on its own line right after line `row`, supplying the
/// newline the last line of a file may be missing.
fn insert_after_line(lines: &[Line<'_>], row: usize, new_text: String) -> Edit {
    let line = &lines[row];
    let has_newline = line.end > line.start + line.content.len();
    Edit {
        range: line.end..line.end,
        new_text: if has_newline {
            new_text
        } else {
            format!("\n{new_text}")
        },
    }
}

/// Splits a completed task's text into its label and the ` ✅ YYYY-MM-DD`
/// completion date, when one is present.
pub fn split_completion(text: &str) -> (&str, Option<&str>) {
    let Some(index) = text.rfind('✅') else {
        return (text, None);
    };
    let after = text['✅'.len_utf8() + index..].trim();
    let looks_like_date = !after.is_empty()
        && after
            .chars()
            .all(|character| character.is_ascii_digit() || character == '-');
    if looks_like_date {
        (text[..index].trim_end(), Some(after))
    } else {
        (text, None)
    }
}

/// Normalizes task text for duplicate detection (spec §7.2):
/// whitespace-insensitive, ignoring any completion-date suffix.
pub fn normalize_task_text(text: &str) -> String {
    let (label, _) = split_completion(text);
    label.split_whitespace().collect::<Vec<_>>().join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "\
# Backlog

Some orienting prose the model must never touch.

## Soon

- [ ] Renew passport
- [ ] Fix the day-planner overlap bug
  - notes and sub-items travel with their parent
  - [ ] even indented checkboxes are children

## Someday

- [ ] Learn woodworking

## Junk drawer

- [ ] not a backlog task

## Completed

- [x] Book dentist appointment ✅ 2026-07-23
";

    fn date(y: i32, m: u32, d: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(y, m, d).unwrap()
    }

    fn task<'a>(backlog: &'a Backlog, kind: SectionKind, text: &str) -> &'a BacklogTask {
        backlog
            .section(kind)
            .iter()
            .find(|task| task.text == text)
            .unwrap_or_else(|| panic!("no task {text:?} in {kind:?}"))
    }

    #[test]
    fn parses_sections_and_children() {
        let backlog = parse_backlog(SAMPLE);
        assert_eq!(
            backlog
                .soon
                .iter()
                .map(|task| task.text.as_str())
                .collect::<Vec<_>>(),
            vec!["Renew passport", "Fix the day-planner overlap bug"]
        );
        assert_eq!(
            backlog
                .someday
                .iter()
                .map(|task| task.text.as_str())
                .collect::<Vec<_>>(),
            vec!["Learn woodworking"]
        );
        assert_eq!(backlog.completed.len(), 1);
        assert!(backlog.completed[0].checked);
        assert!(!backlog.is_empty());

        // The child lines belong to the second Soon task's span.
        let with_children = &backlog.soon[1];
        assert!(SAMPLE[with_children.span.clone()].contains("travel with their parent"));
        assert!(SAMPLE[with_children.span.clone()].contains("even indented checkboxes"));

        // Unknown sections are not modeled.
        assert!(!backlog.contains_task("not a backlog task"));
    }

    #[test]
    fn parses_headings_case_insensitively_at_any_level() {
        let backlog = parse_backlog("### sOoN\n- [ ] A\n# Someday\n- [ ] B\n");
        assert_eq!(backlog.soon.len(), 1);
        assert_eq!(backlog.someday.len(), 1);
    }

    #[test]
    fn first_matching_heading_wins() {
        let backlog = parse_backlog("## Soon\n- [ ] First\n## Break\n## Soon\n- [ ] Second\n");
        assert_eq!(backlog.soon.len(), 1);
        assert_eq!(backlog.soon[0].text, "First");
    }

    #[test]
    fn missing_sections_parse_as_empty() {
        let backlog = parse_backlog("# Notes\nJust prose.\n");
        assert_eq!(backlog, Backlog::default());
        assert!(backlog.is_empty());
    }

    #[test]
    fn indented_checkboxes_are_not_tasks() {
        let backlog = parse_backlog("## Soon\n  - [ ] indented\n- [ ] top level\n");
        assert_eq!(backlog.soon.len(), 1);
        assert_eq!(backlog.soon[0].text, "top level");
    }

    #[test]
    fn duplicate_texts_are_both_parsed() {
        let backlog = parse_backlog("## Soon\n- [ ] Twice\n- [ ] Twice\n");
        assert_eq!(backlog.soon.len(), 2);
        assert_ne!(backlog.soon[0].span, backlog.soon[1].span);
        // The line disambiguates duplicates; an ambiguous text with a stale
        // line resolves to nothing rather than guessing.
        assert_eq!(
            backlog
                .locate_task(SectionKind::Soon, 2, "Twice")
                .map(|task| task.line),
            Some(2)
        );
        assert_eq!(backlog.locate_task(SectionKind::Soon, 9, "Twice"), None);
    }

    #[test]
    fn locate_task_falls_back_to_unique_text_when_lines_shift() {
        let backlog = parse_backlog("## Soon\n\n\n- [ ] Only\n");
        // A stale line (from before blank lines were removed above the task)
        // still resolves because the text is unambiguous.
        assert_eq!(
            backlog
                .locate_task(SectionKind::Soon, 1, "Only")
                .map(|task| task.line),
            Some(3)
        );
        assert_eq!(backlog.locate_task(SectionKind::Soon, 3, "Gone"), None);
    }

    #[test]
    fn loose_children_travel_across_blank_lines() {
        let source = "## Soon\n\
                      - [ ] Task\n  - note A\n\n  - note B\n\n\
                      - [ ] Next\n\n\
                      ## Completed\n";
        let backlog = parse_backlog(source);
        assert_eq!(backlog.soon.len(), 2);
        let first = &backlog.soon[0];
        // Blank lines between children are inside the span; the trailing
        // blank after note B is not.
        assert_eq!(&source[first.span.clone()], "- [ ] Task\n  - note A\n\n  - note B\n");

        let edited = apply_edits(source, complete_task_edits(source, first, date(2026, 7, 24)));
        let reparsed = parse_backlog(&edited);
        assert_eq!(reparsed.soon.len(), 1);
        assert_eq!(reparsed.soon[0].text, "Next");
        let completed_span = &edited[reparsed.completed[0].span.clone()];
        assert!(completed_span.contains("note A"));
        assert!(completed_span.contains("note B"));
        // No stray child bullets left behind in Soon.
        assert!(!edited[..edited.find("## Completed").unwrap()].contains("note B"));
    }

    #[test]
    fn hand_checked_tasks_do_not_count_as_open_work() {
        let backlog = parse_backlog("## Soon\n- [x] Done by hand\n## Someday\n");
        assert_eq!(backlog.soon.len(), 1);
        assert!(backlog.soon[0].checked);
        assert!(backlog.is_empty());
    }

    #[test]
    fn rename_touches_only_the_task_text() {
        let backlog = parse_backlog(SAMPLE);
        let task = task(&backlog, SectionKind::Soon, "Renew passport");
        let edited = apply_edits(SAMPLE, vec![rename_task_edit(task, "Renew both passports")]);
        assert_eq!(
            edited,
            SAMPLE.replace("- [ ] Renew passport", "- [ ] Renew both passports")
        );
    }

    #[test]
    fn complete_moves_task_with_children_and_stamps_date() {
        let backlog = parse_backlog(SAMPLE);
        let task = task(&backlog, SectionKind::Soon, "Fix the day-planner overlap bug");
        let edited = apply_edits(SAMPLE, complete_task_edits(SAMPLE, task, date(2026, 7, 24)));

        let reparsed = parse_backlog(&edited);
        assert_eq!(reparsed.soon.len(), 1);
        assert_eq!(reparsed.completed.len(), 2);
        assert_eq!(
            reparsed.completed[1].text,
            "Fix the day-planner overlap bug ✅ 2026-07-24"
        );
        assert!(reparsed.completed[1].checked);
        // Children traveled along, in order, after the previous completed task.
        let completed_span = &edited[reparsed.completed[1].span.clone()];
        assert!(completed_span.contains("travel with their parent"));
        assert!(edited.find("Book dentist").unwrap() < edited.find("overlap bug").unwrap());
        // Everything the model doesn't own is untouched.
        assert!(edited.contains("Some orienting prose the model must never touch."));
        assert!(edited.contains("## Junk drawer\n\n- [ ] not a backlog task"));
    }

    #[test]
    fn complete_creates_missing_completed_section() {
        let source = "## Soon\n\n- [ ] Only task\n";
        let backlog = parse_backlog(source);
        let task = task(&backlog, SectionKind::Soon, "Only task");
        let edited = apply_edits(source, complete_task_edits(source, task, date(2026, 7, 24)));
        assert_eq!(
            edited,
            "## Soon\n\n\n## Completed\n\n- [x] Only task ✅ 2026-07-24\n"
        );
        let reparsed = parse_backlog(&edited);
        assert_eq!(reparsed.completed.len(), 1);
        assert!(reparsed.soon.is_empty());
    }

    #[test]
    fn complete_task_without_trailing_newline() {
        let source = "## Soon\n- [ ] Last line";
        let backlog = parse_backlog(source);
        let task = task(&backlog, SectionKind::Soon, "Last line");
        let edited = apply_edits(source, complete_task_edits(source, task, date(2026, 7, 24)));
        let reparsed = parse_backlog(&edited);
        assert!(reparsed.soon.is_empty());
        assert_eq!(reparsed.completed.len(), 1);
        assert_eq!(reparsed.completed[0].text, "Last line ✅ 2026-07-24");
    }

    #[test]
    fn move_between_sections_keeps_children_verbatim() {
        let backlog = parse_backlog(SAMPLE);
        let task = task(&backlog, SectionKind::Soon, "Fix the day-planner overlap bug");
        let edited = apply_edits(SAMPLE, move_task_edits(SAMPLE, task, SectionKind::Someday));
        let reparsed = parse_backlog(&edited);
        assert_eq!(reparsed.soon.len(), 1);
        assert_eq!(
            reparsed
                .someday
                .iter()
                .map(|task| task.text.as_str())
                .collect::<Vec<_>>(),
            vec!["Learn woodworking", "Fix the day-planner overlap bug"]
        );
        assert!(edited.contains("  - notes and sub-items travel with their parent"));
        assert!(!reparsed.someday[1].checked);
    }

    #[test]
    fn append_into_empty_section_lands_under_heading() {
        let edited = apply_edits(
            DEFAULT_BACKLOG,
            vec![append_to_section_edit(
                DEFAULT_BACKLOG,
                SectionKind::Soon,
                &new_task_block("  Renew passport  "),
            )],
        );
        assert!(edited.contains("## Soon\n\n- [ ] Renew passport\n\n## Someday"));
        let reparsed = parse_backlog(&edited);
        assert_eq!(reparsed.soon.len(), 1);
        assert_eq!(reparsed.soon[0].text, "Renew passport");
    }

    #[test]
    fn append_done_to_note_targets_planner_section() {
        let note = "# Monday\n\n## Day planner\n\n- [ ] 09:00 Standup\n\n## Personal\n\nprose\n";
        let edit = append_done_to_note_edit(note, "Day planner", "Renew passport");
        let edited = apply_edits(note, vec![edit]);
        assert_eq!(
            edited,
            "# Monday\n\n## Day planner\n\n- [ ] 09:00 Standup\n- [x] Renew passport\n\n\
             ## Personal\n\nprose\n"
        );
    }

    #[test]
    fn append_done_to_note_falls_back_to_end_of_file() {
        for note in ["# Monday\nno planner heading", "# Monday\nno planner heading\n", ""] {
            let edit = append_done_to_note_edit(note, "Day planner", "Task");
            let edited = apply_edits(note, vec![edit]);
            assert!(edited.ends_with("- [x] Task\n"), "got {edited:?}");
            assert!(edited.starts_with(note.trim_end_matches('\n')));
        }
    }

    #[test]
    fn append_done_to_note_with_empty_planner_section() {
        let note = "## Day planner\n\n## Personal\n";
        let edited = apply_edits(
            note,
            vec![append_done_to_note_edit(note, "Day planner", "Task")],
        );
        assert_eq!(edited, "## Day planner\n- [x] Task\n\n## Personal\n");
    }

    #[test]
    fn normalize_ignores_whitespace_and_completion_suffix() {
        assert_eq!(
            normalize_task_text("  Fix   the bug  "),
            normalize_task_text("Fix the bug ✅ 2026-07-23")
        );
        assert_ne!(
            normalize_task_text("Fix the bug"),
            normalize_task_text("fix the bug")
        );
    }

    #[test]
    fn split_completion_handles_suffix_and_plain_text() {
        assert_eq!(
            split_completion("Book dentist ✅ 2026-07-23"),
            ("Book dentist", Some("2026-07-23"))
        );
        assert_eq!(split_completion("No suffix"), ("No suffix", None));
        assert_eq!(
            split_completion("Emoji ✅ but not a date"),
            ("Emoji ✅ but not a date", None)
        );
    }

    #[test]
    fn contains_task_checks_every_section() {
        let backlog = parse_backlog(SAMPLE);
        assert!(backlog.contains_task("Renew  passport"));
        assert!(backlog.contains_task("Learn woodworking"));
        assert!(backlog.contains_task("Book dentist appointment"));
        assert!(!backlog.contains_task("Brand new"));
    }

    #[test]
    fn default_backlog_round_trips() {
        let backlog = parse_backlog(DEFAULT_BACKLOG);
        assert!(backlog.is_empty());
        assert!(backlog.completed.is_empty());
        // Appending to each section keeps the others' content identical.
        let edited = apply_edits(
            DEFAULT_BACKLOG,
            vec![append_to_section_edit(
                DEFAULT_BACKLOG,
                SectionKind::Someday,
                &new_task_block("Learn woodworking"),
            )],
        );
        assert!(edited.contains("<!-- Soon = tasks for the coming days."));
        assert!(edited.contains("## Someday\n\n- [ ] Learn woodworking\n\n## Completed"));
    }
}
