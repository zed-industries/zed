use anyhow::{Context as _, Result};
use chrono::{Datelike, Days, NaiveDate, NaiveTime};
use std::fs;
use std::io::Write as _;
use std::path::{Path, PathBuf};

use crate::vault::Vault;

/// The kinds of periodic notes a vault holds, each with its own directory,
/// filename format, and template in `config.toml`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NoteKind {
    Daily,
    Weekly,
}

/// An entry in the Timeline panel (and its matching `breadpaper:` command).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TimelineEntry {
    Today,
    Yesterday,
    Tomorrow,
    ThisWeek,
    LastWeek,
}

impl TimelineEntry {
    /// Resolves the entry to the kind of note it opens and the date that
    /// identifies the note. Weekly notes are identified by the Monday starting
    /// their ISO week.
    pub fn resolve(self, today: NaiveDate) -> Option<(NoteKind, NaiveDate)> {
        match self {
            Self::Today => Some((NoteKind::Daily, today)),
            Self::Yesterday => today.pred_opt().map(|date| (NoteKind::Daily, date)),
            Self::Tomorrow => today.succ_opt().map(|date| (NoteKind::Daily, date)),
            Self::ThisWeek => week_start(today).map(|date| (NoteKind::Weekly, date)),
            Self::LastWeek => week_start(today)
                .and_then(|monday| monday.checked_sub_days(Days::new(7)))
                .map(|date| (NoteKind::Weekly, date)),
        }
    }
}

fn week_start(date: NaiveDate) -> Option<NaiveDate> {
    date.checked_sub_days(Days::new(date.weekday().num_days_from_monday() as u64))
}

/// Formats a date using the moment.js-style token vocabulary shared by
/// filename formats and `{{date:...}}` template tokens.
///
/// Supported tokens: `YYYY`, `YY`, `MMMM`, `MMM`, `MM`, `M`, `DD`, `D`,
/// `dddd`, `ddd`, `dd`, `d`, `WW`, `W` (ISO week), `GGGG`, `GG` (ISO week
/// year). Text inside `[brackets]` is emitted literally; all other characters
/// pass through unchanged.
pub fn format_date(date: NaiveDate, format: &str) -> String {
    let mut output = String::with_capacity(format.len() + 8);
    let characters: Vec<char> = format.chars().collect();
    let mut index = 0;
    while index < characters.len() {
        let character = characters[index];
        match character {
            '[' => {
                index += 1;
                while index < characters.len() && characters[index] != ']' {
                    output.push(characters[index]);
                    index += 1;
                }
                if index < characters.len() {
                    index += 1;
                }
            }
            'Y' | 'M' | 'D' | 'd' | 'W' | 'G' => {
                let mut run = 1;
                while index + run < characters.len() && characters[index + run] == character {
                    run += 1;
                }
                emit_date_token(&mut output, date, character, run);
                index += run;
            }
            _ => {
                output.push(character);
                index += 1;
            }
        }
    }
    output
}

fn emit_date_token(output: &mut String, date: NaiveDate, token: char, run: usize) {
    let expansion = match (token, run) {
        ('Y', 2) => format!("{:02}", date.year() % 100),
        ('Y', _) => format!("{:04}", date.year()),
        ('M', 1) => date.month().to_string(),
        ('M', 2) => format!("{:02}", date.month()),
        ('M', 3) => date.format("%b").to_string(),
        ('M', _) => date.format("%B").to_string(),
        ('D', 1) => date.day().to_string(),
        ('D', _) => format!("{:02}", date.day()),
        ('d', 1) => date.weekday().num_days_from_sunday().to_string(),
        ('d', 2) => {
            let mut name = date.format("%A").to_string();
            name.truncate(2);
            name
        }
        ('d', 3) => date.format("%a").to_string(),
        ('d', _) => date.format("%A").to_string(),
        ('W', 1) => date.iso_week().week().to_string(),
        ('W', _) => format!("{:02}", date.iso_week().week()),
        ('G', 2) => format!("{:02}", date.iso_week().year() % 100),
        ('G', _) => format!("{:04}", date.iso_week().year()),
        _ => date.format("%A").to_string(),
    };
    output.push_str(&expansion);
}

/// Expands Obsidian-style template tokens: `{{date}}`, `{{date:FORMAT}}`,
/// `{{time}}`, and `{{title}}`. Unrecognized tokens are left as-is.
pub fn expand_template(
    template: &str,
    date: NaiveDate,
    time: NaiveTime,
    title: &str,
) -> String {
    let mut output = String::with_capacity(template.len());
    let mut rest = template;
    while let Some(start) = rest.find("{{") {
        let Some(end_offset) = rest[start + 2..].find("}}") else {
            break;
        };
        let token = &rest[start + 2..start + 2 + end_offset];
        output.push_str(&rest[..start]);
        match token {
            "date" => output.push_str(&date.format("%Y-%m-%d").to_string()),
            "time" => output.push_str(&time.format("%H:%M").to_string()),
            "title" => output.push_str(title),
            _ => {
                if let Some(format) = token.strip_prefix("date:") {
                    output.push_str(&format_date(date, format));
                } else {
                    output.push_str(&rest[start..start + 2 + end_offset + 2]);
                }
            }
        }
        rest = &rest[start + 2 + end_offset + 2..];
    }
    output.push_str(rest);
    output
}

/// The outcome of ensuring a note exists.
#[derive(Debug, PartialEq)]
pub enum EnsureNoteOutcome {
    AlreadyExisted,
    Created,
    /// The note was created empty because the configured template is missing.
    CreatedWithoutTemplate,
}

/// Ensures the note of `kind` for `date` exists, creating it from the vault's
/// template if missing, and returns its path. Existing notes are never
/// touched. Blocking I/O — call from a background thread.
pub fn ensure_note(
    vault: &Vault,
    kind: NoteKind,
    date: NaiveDate,
    time: NaiveTime,
) -> Result<(PathBuf, EnsureNoteOutcome)> {
    let path = vault.note_path(kind, date);
    if path.exists() {
        return Ok((path, EnsureNoteOutcome::AlreadyExisted));
    }

    let template_path = vault.template_path(kind);
    let (template, outcome) = match fs::read_to_string(&template_path) {
        Ok(template) => (template, EnsureNoteOutcome::Created),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            (String::new(), EnsureNoteOutcome::CreatedWithoutTemplate)
        }
        Err(error) => {
            return Err(error)
                .with_context(|| format!("reading template {}", template_path.display()));
        }
    };

    let title = path
        .file_stem()
        .map(|stem| stem.to_string_lossy().into_owned())
        .unwrap_or_default();
    let contents = expand_template(&template, date, time, &title);

    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).with_context(|| format!("creating {}", parent.display()))?;
    }
    write_new_file(&path, &contents)?;
    Ok((path, outcome))
}

/// Creates `path` with `contents`, failing if it already exists, and removing
/// the file again if the write fails partway so no partial note is left behind.
fn write_new_file(path: &Path, contents: &str) -> Result<()> {
    let mut file = match fs::OpenOptions::new().write(true).create_new(true).open(path) {
        Ok(file) => file,
        Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => return Ok(()),
        Err(error) => {
            return Err(error).with_context(|| format!("creating {}", path.display()));
        }
    };
    if let Err(error) = file.write_all(contents.as_bytes()) {
        drop(file);
        if let Err(cleanup_error) = fs::remove_file(path) {
            log::error!(
                "failed to clean up partially written note {}: {cleanup_error}",
                path.display()
            );
        }
        return Err(error).with_context(|| format!("writing {}", path.display()));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vault::{VaultStatus, scaffold_vault};

    fn date(y: i32, m: u32, d: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(y, m, d).unwrap()
    }

    #[test]
    fn format_date_tokens() {
        let d = date(2026, 7, 20); // a Monday
        assert_eq!(format_date(d, "YYYY-MM-DD"), "2026-07-20");
        assert_eq!(format_date(d, "YY M D"), "26 7 20");
        assert_eq!(format_date(d, "dddd, MMMM D, YYYY"), "Monday, July 20, 2026");
        assert_eq!(format_date(d, "ddd MMM DD"), "Mon Jul 20");
        assert_eq!(format_date(d, "dd"), "Mo");
        assert_eq!(format_date(d, "d"), "1");
        assert_eq!(format_date(date(2026, 7, 5), "D MMM"), "5 Jul");
    }

    #[test]
    fn format_date_week_tokens() {
        for d in [
            date(2026, 7, 20),
            date(2026, 1, 1),
            date(2025, 12, 29), // ISO week year differs from calendar year
            date(2027, 1, 3),
        ] {
            assert_eq!(
                format_date(d, "GGGG-[W]WW"),
                format!("{:04}-W{:02}", d.iso_week().year(), d.iso_week().week()),
                "for {d}"
            );
        }
        assert_eq!(format_date(date(2026, 2, 2), "[Week] W"), "Week 6");
    }

    #[test]
    fn format_date_literals_pass_through() {
        let d = date(2026, 7, 20);
        assert_eq!(format_date(d, "[Day] D [of] MMMM"), "Day 20 of July");
        assert_eq!(format_date(d, "YYYY/MM/DD daily"), "2026/07/20 1aily");
    }

    #[test]
    fn timeline_entries_resolve() {
        let tuesday = date(2026, 7, 21);
        assert_eq!(
            TimelineEntry::Today.resolve(tuesday),
            Some((NoteKind::Daily, tuesday))
        );
        assert_eq!(
            TimelineEntry::Yesterday.resolve(tuesday),
            Some((NoteKind::Daily, date(2026, 7, 20)))
        );
        assert_eq!(
            TimelineEntry::Tomorrow.resolve(tuesday),
            Some((NoteKind::Daily, date(2026, 7, 22)))
        );
        assert_eq!(
            TimelineEntry::ThisWeek.resolve(tuesday),
            Some((NoteKind::Weekly, date(2026, 7, 20)))
        );
        assert_eq!(
            TimelineEntry::LastWeek.resolve(tuesday),
            Some((NoteKind::Weekly, date(2026, 7, 13)))
        );

        // A Monday's ThisWeek is itself; a Sunday's is the previous Monday.
        assert_eq!(
            TimelineEntry::ThisWeek.resolve(date(2026, 7, 20)),
            Some((NoteKind::Weekly, date(2026, 7, 20)))
        );
        assert_eq!(
            TimelineEntry::ThisWeek.resolve(date(2026, 7, 26)),
            Some((NoteKind::Weekly, date(2026, 7, 20)))
        );
    }

    #[test]
    fn expand_template_tokens() {
        let d = date(2026, 7, 19);
        let t = NaiveTime::from_hms_opt(14, 31, 0).unwrap();
        assert_eq!(
            expand_template(
                "# {{date:dddd, MMMM D, YYYY}}\n{{date}} {{time}} {{title}}",
                d,
                t,
                "2026-07-19",
            ),
            "# Sunday, July 19, 2026\n2026-07-19 14:31 2026-07-19"
        );
    }

    #[test]
    fn expand_template_leaves_unknown_tokens() {
        let d = date(2026, 7, 19);
        let t = NaiveTime::from_hms_opt(0, 0, 0).unwrap();
        assert_eq!(
            expand_template("{{weather}} and {{unclosed", d, t, "x"),
            "{{weather}} and {{unclosed"
        );
    }

    #[test]
    fn ensure_note_creates_daily_from_template() {
        let dir = tempfile::tempdir().unwrap();
        scaffold_vault(dir.path()).unwrap();
        let VaultStatus::Valid(vault) = crate::vault::Vault::detect(dir.path()) else {
            panic!("expected valid vault");
        };
        let d = date(2026, 7, 20);
        let t = NaiveTime::from_hms_opt(9, 0, 0).unwrap();

        let (path, outcome) = ensure_note(&vault, NoteKind::Daily, d, t).unwrap();
        assert_eq!(outcome, EnsureNoteOutcome::Created);
        assert_eq!(path, dir.path().join("daily/2026-07-20.md"));
        let contents = fs::read_to_string(&path).unwrap();
        assert!(contents.starts_with("# Monday, July 20, 2026\n"));

        // A second call must not touch the file.
        fs::write(&path, "user edits").unwrap();
        let (_, outcome) = ensure_note(&vault, NoteKind::Daily, d, t).unwrap();
        assert_eq!(outcome, EnsureNoteOutcome::AlreadyExisted);
        assert_eq!(fs::read_to_string(&path).unwrap(), "user edits");
    }

    #[test]
    fn ensure_note_creates_weekly_from_template() {
        let dir = tempfile::tempdir().unwrap();
        scaffold_vault(dir.path()).unwrap();
        let VaultStatus::Valid(vault) = crate::vault::Vault::detect(dir.path()) else {
            panic!("expected valid vault");
        };
        let monday = date(2026, 7, 20);
        let t = NaiveTime::from_hms_opt(9, 0, 0).unwrap();

        let (path, outcome) = ensure_note(&vault, NoteKind::Weekly, monday, t).unwrap();
        assert_eq!(outcome, EnsureNoteOutcome::Created);
        assert_eq!(path, dir.path().join("weekly/2026-W30.md"));
        let contents = fs::read_to_string(&path).unwrap();
        assert!(contents.starts_with("# Week 30, 2026\n"), "got: {contents}");
    }

    #[test]
    fn ensure_note_missing_template_creates_empty() {
        let dir = tempfile::tempdir().unwrap();
        scaffold_vault(dir.path()).unwrap();
        fs::remove_file(dir.path().join("templates/daily.md")).unwrap();
        let VaultStatus::Valid(vault) = crate::vault::Vault::detect(dir.path()) else {
            panic!("expected valid vault");
        };
        let (path, outcome) = ensure_note(
            &vault,
            NoteKind::Daily,
            date(2026, 7, 20),
            NaiveTime::from_hms_opt(9, 0, 0).unwrap(),
        )
        .unwrap();
        assert_eq!(outcome, EnsureNoteOutcome::CreatedWithoutTemplate);
        assert_eq!(fs::read_to_string(&path).unwrap(), "");
    }
}
