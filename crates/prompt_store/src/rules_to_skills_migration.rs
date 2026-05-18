//! One-time migration from Rules (stored in the user's `PromptStore`
//! LMDB database) to two destinations:
//!
//! * **Non-Default Rules → global Agent Skills** under
//!   `~/.agents/skills/<slug>/SKILL.md`. Non-Default Rules were only ever
//!   included in a conversation when the user explicitly invoked them by
//!   name, which maps cleanly onto Skills with
//!   `disable-model-invocation: true` (slash-only, never auto-suggested
//!   to the model). See [`migrate_non_default_rules_to_skills`].
//!
//! * **Default Rules → global AGENTS.md** at the platform-appropriate
//!   path (see [`paths::agents_file`]). Default Rules were auto-included
//!   in every conversation; the global AGENTS.md is loaded into the
//!   system prompt of every conversation, so the migration target
//!   preserves the behavior. Each rule is appended under an `## H2`
//!   heading containing the rule's title. See
//!   [`migrate_default_rules_to_agents_md`].
//!
//!   **Customized built-in prompts** (currently just
//!   [`BuiltInPrompt::CommitMessage`]) are treated the same as Default
//!   user Rules — if the user has edited the body away from the
//!   built-in's `default_content()`, the edited body is appended to
//!   AGENTS.md ahead of any user Default Rules. Uncustomized built-ins
//!   (still using Zed's shipped default content) are skipped so we don't
//!   pollute AGENTS.md with text the user never wrote.
//!
//! Both migrations are gated by:
//!
//! * the `skills` feature flag — users without it never have their Rules
//!   touched in any way;
//! * a single global "migration already ran" flag persisted in
//!   [`GlobalKeyValueStore`] — keyed by [`MIGRATION_DONE_KEY`], so a
//!   shared home directory only gets populated once per machine even
//!   across release channels.
//!
//! The migration is intentionally non-destructive: rule rows in the LMDB
//! database are left in place after the migration. That way users can
//! still see and edit their Rules via the existing UI, and a user who
//! downgrades to a Zed build without skills support won't lose anything.

use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

use agent_skills::{SKILL_FILE_NAME, global_skills_dir, slugify_skill_name};
use anyhow::{Context as _, Result};
use db::kvp::GlobalKeyValueStore;
use feature_flags::{FeatureFlagAppExt as _, SkillsFeatureFlag};
use fs::Fs;
use gpui::{App, AsyncApp, Entity, TaskExt as _};
use serde::{Deserialize, Serialize};
use util::ResultExt as _;

use crate::{BuiltInPrompt, PromptId, PromptStore};
use strum::IntoEnumIterator as _;

/// Global KVP flag: set to `"1"` once the migration has been considered
/// for this machine, regardless of whether any rules were actually
/// migrated. Used to short-circuit the migration on every subsequent
/// launch.
pub const MIGRATION_DONE_KEY: &str = "rules_to_skills_migration_done";

/// Global KVP key for the JSON-serialized [`MigrationResult`] produced by
/// the most recent migration run — the lists of source-Rule titles that
/// were migrated to each destination. The title-bar banner and its
/// explainer modal read this to decide what (if anything) to tell the
/// user about what changed.
pub const MIGRATION_RESULT_KEY: &str = "rules_to_skills_migration_result";

/// A persistent record of what the rules-to-skills migration actually
/// migrated. Persisted in [`GlobalKeyValueStore`] under
/// [`MIGRATION_RESULT_KEY`] and read back by the announcement UI so the
/// modal can list specific rule names instead of vaguely gesturing.
///
/// All three lists hold the *original* user-facing Rule titles, not the
/// derived skill slug or any other transformed identifier — those are
/// what users would recognize.
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct MigrationResult {
    /// Non-Default Rules that were turned into global Skills under
    /// `~/.agents/skills/`.
    #[serde(default)]
    pub skill_names: Vec<String>,
    /// Default Rules that were appended to the global AGENTS.md.
    #[serde(default)]
    pub agents_md_names: Vec<String>,
    /// Customized built-in prompts whose edited bodies were appended to
    /// the top of the global AGENTS.md.
    #[serde(default)]
    pub customized_builtins: Vec<String>,
}

impl MigrationResult {
    /// `true` if the migration didn't actually move any Rule anywhere —
    /// i.e. the user had no Rules of any kind to migrate. The
    /// announcement banner/modal uses this to switch between the
    /// "Introducing: Skills" generic intro and the "Skills have replaced
    /// Rules" migration summary.
    pub fn is_empty(&self) -> bool {
        self.skill_names.is_empty()
            && self.agents_md_names.is_empty()
            && self.customized_builtins.is_empty()
    }
}

/// Read the most recently persisted [`MigrationResult`], if any. Returns
/// `None` when the migration hasn't run on this machine yet or the
/// persisted blob couldn't be parsed.
pub fn migration_result() -> Option<MigrationResult> {
    let json = GlobalKeyValueStore::global()
        .read_kvp(MIGRATION_RESULT_KEY)
        .log_err()
        .flatten()?;
    serde_json::from_str(&json).log_err()
}

/// Placeholder description written into the YAML frontmatter of migrated
/// skills. Migrated skills are model-disabled, so the model never sees
/// this string — it exists only because the SKILL.md schema requires a
/// non-empty `description`.
const PLACEHOLDER_DESCRIPTION: &str = "(no description)";

/// Returns `true` if a previous launch has already completed the
/// rules-to-skills migration check.
pub fn migration_done() -> bool {
    GlobalKeyValueStore::global()
        .read_kvp(MIGRATION_DONE_KEY)
        .log_err()
        .flatten()
        .is_some()
}

/// Process-lifetime guard ensuring the migration task is spawned at most
/// once per process. The KVP-backed [`migration_done`] flag handles the
/// across-launch idempotency, but it isn't enough on its own: this
/// function is wired to `cx.on_flags_ready`, which is implemented via
/// `observe_global::<FeatureFlagStore>` and therefore fires every time
/// the flag store mutates. At startup that can happen several times in
/// rapid succession (window construction, settings observers touching
/// globals, etc.). Without this guard, each of those firings would see
/// `migration_done() == false` (because the first in-flight spawn hasn't
/// written the KVP yet), spawn its own task, and the tasks would race —
/// each one calling `pick_available_skill_dir` and dutifully picking the
/// next free `-N` suffix because its sibling task already created the
/// previous one. The visible result is N duplicate `<rule>-2`,
/// `<rule>-3`, … directories per rule, where N is the number of times
/// the callback fired before the first spawn finished writing
/// `MIGRATION_DONE_KEY`.
static MIGRATION_TASK_SPAWNED: AtomicBool = AtomicBool::new(false);

/// Migrate non-Default user rules to global Skills, if not already done.
///
/// Safe to call on every startup — short-circuits immediately when the
/// migration has already run, when another invocation in this process
/// has already started it, or when the user doesn't have the `skills`
/// feature flag enabled.
pub fn migrate_rules_to_skills_if_needed(fs: Arc<dyn Fs>, cx: &mut App) {
    if !cx.has_flag::<SkillsFeatureFlag>() {
        return;
    }
    if migration_done() {
        return;
    }
    // Atomically claim the right to spawn the migration task. If another
    // invocation has already claimed it, we bail without spawning a
    // second one — see the doc comment on `MIGRATION_TASK_SPAWNED` for
    // why the KVP-backed check above isn't sufficient on its own.
    if MIGRATION_TASK_SPAWNED
        .compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire)
        .is_err()
    {
        return;
    }

    let prompt_store = PromptStore::global(cx);
    cx.spawn(async move |cx| {
        let prompt_store = prompt_store.await.context("loading prompt store")?;

        // Snapshot the (id, title) pairs for every user rule, split by
        // whether it's a Default rule or not. BuiltIn prompts (e.g. the
        // commit-message prompt) are excluded — they're not user-facing
        // "Rules" in the agent sense.
        let (default_rules, non_default_rules) = prompt_store.read_with(cx, |store, _| {
            let mut default = Vec::new();
            let mut non_default = Vec::new();
            for metadata in store.all_prompt_metadata() {
                if metadata.id.as_user().is_none() {
                    continue;
                }
                let Some(title) = metadata.title.as_ref().map(|t| t.to_string()) else {
                    continue;
                };
                if metadata.default {
                    default.push((metadata.id, title));
                } else {
                    non_default.push((metadata.id, title));
                }
            }
            (default, non_default)
        });

        let mut result = MigrationResult::default();

        result.skill_names =
            migrate_non_default_rules_to_skills(fs.as_ref(), &prompt_store, cx, non_default_rules)
                .await;

        let (agents_md_names, customized_builtins) = migrate_default_rules_to_agents_md(
            fs.as_ref(),
            paths::agents_file(),
            &prompt_store,
            cx,
            default_rules,
        )
        .await;
        result.agents_md_names = agents_md_names;
        result.customized_builtins = customized_builtins;

        // Persist the result BEFORE the done flag: if we crash between
        // these two writes the next launch will see `done == false` and
        // re-run, picking up the same (deterministic) result — worst
        // case is the AGENTS.md append happens twice, which is a
        // pre-existing limitation of the AGENTS.md migration.
        write_migration_result(&result).await;
        mark_migration_done().await;
        anyhow::Ok(())
    })
    .detach_and_log_err(cx);
}

/// Returns `true` if `body` (the result of `PromptStore::load` for the
/// given built-in) differs from the built-in's shipped `default_content`.
/// Customization detection is done by trimmed-string comparison so that
/// whitespace-only differences (e.g. trailing newlines) don't count as a
/// customization.
fn is_customized_builtin_body(builtin: BuiltInPrompt, body: &str) -> bool {
    body.trim() != builtin.default_content().trim()
}

/// Convert every non-Default user rule into a global Agent Skill on disk.
/// Returns the titles of rules that were successfully migrated (i.e. the
/// ones the user will recognize when the announcement modal lists
/// "these Rules have been migrated to Skills").
async fn migrate_non_default_rules_to_skills(
    fs: &dyn Fs,
    prompt_store: &Entity<PromptStore>,
    cx: &mut AsyncApp,
    rules: Vec<(PromptId, String)>,
) -> Vec<String> {
    if rules.is_empty() {
        return Vec::new();
    }
    let skills_dir = global_skills_dir();
    let mut migrated = Vec::with_capacity(rules.len());
    for (id, title) in rules {
        let body = match load_rule_body(prompt_store, cx, id, &title).await {
            Some(body) => body,
            None => continue,
        };
        let Some(slug) = slugify_skill_name(&title) else {
            log::warn!(
                "Skipping rule {title:?}: title contains no characters \
                 valid for a skill name"
            );
            continue;
        };
        match write_migrated_skill(fs, &skills_dir, &slug, &body).await {
            Ok(()) => migrated.push(title),
            Err(err) => {
                log::warn!("Failed to write skill for rule {title:?}: {err:#}");
            }
        }
    }
    migrated
}

/// Append all auto-included Rules to the global `AGENTS.md`, creating it
/// if necessary. Each rule lands under an `## H2` heading containing its
/// title, with the rule body underneath.
///
/// The appended block contains, in order:
///
/// 1. Each [`BuiltInPrompt`] the user has customized (uncustomized
///    built-ins are skipped so we don't write Zed's shipped default text
///    into the user's personal AGENTS.md).
/// 2. Each user Default Rule, in the order given.
///
/// Returns `(default_user_rule_titles, customized_builtin_titles)` of
/// what actually got appended, for the announcement modal to surface.
async fn migrate_default_rules_to_agents_md(
    fs: &dyn Fs,
    agents_md_path: &Path,
    prompt_store: &Entity<PromptStore>,
    cx: &mut AsyncApp,
    default_user_rules: Vec<(PromptId, String)>,
) -> (Vec<String>, Vec<String>) {
    let mut entries: Vec<(String, String)> = Vec::new();
    let mut customized_builtin_titles: Vec<String> = Vec::new();
    let mut default_user_titles: Vec<String> = Vec::new();

    // Customized built-ins come first.
    for builtin in BuiltInPrompt::iter() {
        let id = PromptId::BuiltIn(builtin);
        let title = builtin.title().to_string();
        let Some(body) = load_rule_body(prompt_store, cx, id, &title).await else {
            continue;
        };
        if !is_customized_builtin_body(builtin, &body) {
            continue;
        }
        customized_builtin_titles.push(title.clone());
        entries.push((title, body));
    }

    // Then user Default Rules.
    for (id, title) in default_user_rules {
        let Some(body) = load_rule_body(prompt_store, cx, id, &title).await else {
            continue;
        };
        default_user_titles.push(title.clone());
        entries.push((title, body));
    }

    if entries.is_empty() {
        return (default_user_titles, customized_builtin_titles);
    }
    if let Err(err) = append_default_rules_to_agents_md(fs, agents_md_path, &entries).await {
        log::warn!("Failed to append default rules to AGENTS.md: {err:#}");
        // Treat a write failure as "nothing was actually migrated" so the
        // announcement modal doesn't lie about what's in AGENTS.md.
        return (Vec::new(), Vec::new());
    }
    (default_user_titles, customized_builtin_titles)
}

async fn load_rule_body(
    prompt_store: &Entity<PromptStore>,
    cx: &mut AsyncApp,
    id: PromptId,
    title: &str,
) -> Option<String> {
    let task = prompt_store.update(cx, |store, cx| store.load(id, cx));
    match task.await {
        Ok(body) => Some(body),
        Err(err) => {
            log::warn!("Skipping rule {title:?}: failed to load body: {err:#}");
            None
        }
    }
}

/// Build the markdown text to append for the given (title, body) rules
/// and write it to `agents_md_path`, preserving any existing AGENTS.md
/// content above the appended block.
async fn append_default_rules_to_agents_md(
    fs: &dyn Fs,
    agents_md_path: &Path,
    rules: &[(String, String)],
) -> Result<()> {
    if rules.is_empty() {
        return Ok(());
    }
    let appended = format_default_rules_section(rules);

    // `fs.load` errors when the file is missing OR unreadable; treat both
    // as "no existing content" so the file gets (re-)created from the
    // migrated text.
    let existing_trimmed = fs
        .load(agents_md_path)
        .await
        .ok()
        .map(|s| s.trim().to_string());

    let final_contents = match existing_trimmed.as_deref() {
        Some(existing) if !existing.is_empty() => format!("{existing}\n\n{appended}\n"),
        _ => format!("{appended}\n"),
    };

    fs.write(agents_md_path, final_contents.as_bytes()).await?;
    Ok(())
}

/// Build the markdown text representing the migrated Default Rules block.
/// Each rule contributes an `## H2` heading followed by its body, with
/// rules separated by blank lines.
fn format_default_rules_section(rules: &[(String, String)]) -> String {
    let mut out = String::new();
    for (title, body) in rules {
        if !out.is_empty() {
            out.push_str("\n\n");
        }
        out.push_str("## ");
        out.push_str(title);
        out.push_str("\n\n");
        out.push_str(body.trim());
    }
    out
}

async fn mark_migration_done() {
    GlobalKeyValueStore::global()
        .write_kvp(MIGRATION_DONE_KEY.into(), "1".into())
        .await
        .log_err();
}

async fn write_migration_result(result: &MigrationResult) {
    let json = match serde_json::to_string(result) {
        Ok(json) => json,
        Err(err) => {
            log::warn!("Failed to serialize rules-to-skills migration result: {err:#}");
            return;
        }
    };
    GlobalKeyValueStore::global()
        .write_kvp(MIGRATION_RESULT_KEY.into(), json)
        .await
        .log_err();
}

/// Write a single migrated rule to disk as `<skills_dir>/<name>/SKILL.md`.
///
/// Three cases:
///
/// 1. `<skills_dir>/<slug>/SKILL.md` already exists with byte-identical
///    content to what we'd write — likely because the migration ran
///    successfully on a previous launch and is now being asked to
///    re-migrate the same source rule. Skip silently; don't create a
///    `<slug>-2` duplicate of the same content.
/// 2. `<skills_dir>/<slug>/` doesn't exist — happy path. Create it and
///    write the SKILL.md there.
/// 3. `<skills_dir>/<slug>/` exists with *different* content (a real
///    name collision or a hand-edited skill we shouldn't touch). Pick
///    the first free `<slug>-2`, `<slug>-3`, … and write there with the
///    suffixed name baked into the SKILL.md frontmatter.
async fn write_migrated_skill(
    fs: &dyn Fs,
    skills_dir: &Path,
    slug: &str,
    body: &str,
) -> Result<()> {
    let primary_dir = skills_dir.join(slug);
    let primary_file = primary_dir.join(SKILL_FILE_NAME);
    let primary_content = format_skill_file(slug, body);

    // Case 1: primary exists with identical content — nothing to do.
    // Compare trimmed so a stray leading/trailing newline difference
    // (which is meaningless inside a SKILL.md) doesn't trick us into
    // generating a `<slug>-N` duplicate.
    if fs.is_file(&primary_file).await
        && fs
            .load(&primary_file)
            .await
            .ok()
            .is_some_and(|existing| existing.trim() == primary_content.trim())
    {
        return Ok(());
    }

    // Cases 2 and 3: find a free directory (the primary if free,
    // otherwise a `-N` suffix) and write the SKILL.md there.
    let (name, dir) = pick_available_skill_dir(fs, skills_dir, slug).await?;
    fs.create_dir(&dir).await?;
    let content = if name == slug {
        primary_content
    } else {
        format_skill_file(&name, body)
    };
    let skill_file_path = dir.join(SKILL_FILE_NAME);
    fs.write(&skill_file_path, content.as_bytes()).await?;
    Ok(())
}

/// Build the SKILL.md file contents for a migrated rule.
fn format_skill_file(name: &str, body: &str) -> String {
    let mut output = format!(
        "---\nname: {name}\ndescription: {PLACEHOLDER_DESCRIPTION}\n\
         disable-model-invocation: true\n---\n"
    );
    let trimmed_body = body.trim();
    if !trimmed_body.is_empty() {
        output.push('\n');
        output.push_str(trimmed_body);
        output.push('\n');
    }
    output
}

/// Cap on how many suffixed variants we'll try before giving up. In
/// practice nobody has more than a handful of rules with the same slug;
/// the cap exists purely to bound the worst case if a user has somehow
/// filled `~/.agents/skills/` with thousands of `name-N` directories.
const MAX_SLUG_SUFFIX: usize = 1000;

async fn pick_available_skill_dir(
    fs: &dyn Fs,
    skills_dir: &Path,
    slug: &str,
) -> Result<(String, PathBuf)> {
    let primary = skills_dir.join(slug);
    if !fs.is_dir(&primary).await {
        return Ok((slug.to_string(), primary));
    }
    for i in 2..=MAX_SLUG_SUFFIX {
        let candidate_name = format!("{slug}-{i}");
        let candidate_dir = skills_dir.join(&candidate_name);
        if !fs.is_dir(&candidate_dir).await {
            return Ok((candidate_name, candidate_dir));
        }
    }
    anyhow::bail!(
        "no free skill directory found under {} for slug {slug:?} \
         after {MAX_SLUG_SUFFIX} attempts",
        skills_dir.display()
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use agent_skills::{SkillSource, parse_skill_frontmatter};
    use fs::FakeFs;
    use gpui::TestAppContext;

    #[test]
    fn format_skill_file_includes_disable_model_invocation() {
        let content = format_skill_file("my-rule", "Body text.");
        assert!(content.contains("\nname: my-rule\n"));
        assert!(content.contains(&format!("\ndescription: {PLACEHOLDER_DESCRIPTION}\n")));
        assert!(content.contains("\ndisable-model-invocation: true\n"));
        assert!(content.ends_with("Body text.\n"));
    }

    #[test]
    fn format_skill_file_handles_empty_body() {
        let content = format_skill_file("my-rule", "   \n  ");
        // Even for an empty body, the closing `---` must be present.
        assert!(content.contains("\n---\n"));
        assert!(content.contains("disable-model-invocation: true"));
    }

    #[test]
    fn format_skill_file_round_trips_through_parser() {
        let content = format_skill_file("my-rule", "Hello world.");
        let skill = parse_skill_frontmatter(
            Path::new("/skills/my-rule/SKILL.md"),
            &content,
            SkillSource::Global,
        )
        .expect("migrated SKILL.md should parse");
        assert_eq!(skill.name, "my-rule");
        assert_eq!(skill.description, PLACEHOLDER_DESCRIPTION);
        assert!(skill.disable_model_invocation);
    }

    #[gpui::test]
    async fn pick_available_skill_dir_returns_primary_when_unused(cx: &mut TestAppContext) {
        let fs = FakeFs::new(cx.executor());
        let skills_dir = PathBuf::from("/skills");
        fs.create_dir(&skills_dir).await.unwrap();

        let (name, dir) = pick_available_skill_dir(fs.as_ref(), &skills_dir, "my-rule")
            .await
            .unwrap();
        assert_eq!(name, "my-rule");
        assert_eq!(dir, skills_dir.join("my-rule"));
    }

    #[gpui::test]
    async fn pick_available_skill_dir_appends_suffix_on_collision(cx: &mut TestAppContext) {
        let fs = FakeFs::new(cx.executor());
        let skills_dir = PathBuf::from("/skills");
        fs.create_dir(&skills_dir.join("my-rule")).await.unwrap();
        fs.create_dir(&skills_dir.join("my-rule-2")).await.unwrap();

        let (name, dir) = pick_available_skill_dir(fs.as_ref(), &skills_dir, "my-rule")
            .await
            .unwrap();
        assert_eq!(name, "my-rule-3");
        assert_eq!(dir, skills_dir.join("my-rule-3"));
    }

    #[gpui::test]
    async fn write_migrated_skill_creates_directory_and_file(cx: &mut TestAppContext) {
        let fs = FakeFs::new(cx.executor());
        let skills_dir = PathBuf::from("/skills");
        fs.create_dir(&skills_dir).await.unwrap();

        write_migrated_skill(fs.as_ref(), &skills_dir, "my-rule", "Body.")
            .await
            .unwrap();

        let written = fs
            .load(&skills_dir.join("my-rule").join(SKILL_FILE_NAME))
            .await
            .expect("SKILL.md should exist");
        let skill = parse_skill_frontmatter(
            &skills_dir.join("my-rule").join(SKILL_FILE_NAME),
            &written,
            SkillSource::Global,
        )
        .expect("written SKILL.md should parse");
        assert_eq!(skill.name, "my-rule");
        assert!(skill.disable_model_invocation);
    }

    #[gpui::test]
    async fn write_migrated_skill_skips_when_primary_content_is_identical(cx: &mut TestAppContext) {
        let fs = FakeFs::new(cx.executor());
        let skills_dir = PathBuf::from("/skills");
        fs.create_dir(&skills_dir.join("my-rule")).await.unwrap();
        // Seed the primary location with byte-identical content to what the
        // migration would write.
        let identical = format_skill_file("my-rule", "Body.");
        fs.insert_file(
            &skills_dir.join("my-rule").join(SKILL_FILE_NAME),
            identical.as_bytes().to_vec(),
        )
        .await;

        write_migrated_skill(fs.as_ref(), &skills_dir, "my-rule", "Body.")
            .await
            .unwrap();

        // No `-2` duplicate should have been produced.
        assert!(!fs.is_dir(&skills_dir.join("my-rule-2")).await);
        // Primary still has the same content.
        let primary = fs
            .load(&skills_dir.join("my-rule").join(SKILL_FILE_NAME))
            .await
            .unwrap();
        assert_eq!(primary, identical);
    }

    #[gpui::test]
    async fn write_migrated_skill_skips_when_primary_differs_only_in_whitespace(
        cx: &mut TestAppContext,
    ) {
        let fs = FakeFs::new(cx.executor());
        let skills_dir = PathBuf::from("/skills");
        fs.create_dir(&skills_dir.join("my-rule")).await.unwrap();
        // Same logical content but with extra leading/trailing whitespace
        // (which is meaningless inside a SKILL.md).
        let mut padded = String::from("\n\n");
        padded.push_str(format_skill_file("my-rule", "Body.").trim());
        padded.push_str("\n\n");
        fs.insert_file(
            &skills_dir.join("my-rule").join(SKILL_FILE_NAME),
            padded.as_bytes().to_vec(),
        )
        .await;

        write_migrated_skill(fs.as_ref(), &skills_dir, "my-rule", "Body.")
            .await
            .unwrap();

        // No `-2` duplicate.
        assert!(!fs.is_dir(&skills_dir.join("my-rule-2")).await);
        // Primary content was NOT overwritten — the user's whitespace is
        // preserved verbatim.
        let primary = fs
            .load(&skills_dir.join("my-rule").join(SKILL_FILE_NAME))
            .await
            .unwrap();
        assert_eq!(primary, padded);
    }

    #[gpui::test]
    async fn write_migrated_skill_does_not_clobber_existing_skill(cx: &mut TestAppContext) {
        let fs = FakeFs::new(cx.executor());
        let skills_dir = PathBuf::from("/skills");
        fs.create_dir(&skills_dir.join("my-rule")).await.unwrap();
        fs.insert_file(
            &skills_dir.join("my-rule").join(SKILL_FILE_NAME),
            b"---\nname: my-rule\ndescription: pre-existing\n---\n\nDo not touch.\n".to_vec(),
        )
        .await;

        write_migrated_skill(fs.as_ref(), &skills_dir, "my-rule", "Migrated body.")
            .await
            .unwrap();

        let pre_existing = fs
            .load(&skills_dir.join("my-rule").join(SKILL_FILE_NAME))
            .await
            .unwrap();
        assert!(pre_existing.contains("Do not touch."));

        let migrated = fs
            .load(&skills_dir.join("my-rule-2").join(SKILL_FILE_NAME))
            .await
            .expect("migrated SKILL.md should have landed at the suffixed path");
        assert!(migrated.contains("Migrated body."));
        assert!(migrated.contains("disable-model-invocation: true"));
    }

    #[test]
    fn format_default_rules_section_renders_headings_and_bodies() {
        let rules = vec![
            (
                "My First Rule".to_string(),
                "Body of first rule.".to_string(),
            ),
            (
                "Second Rule".to_string(),
                "Body of second rule.".to_string(),
            ),
        ];
        let section = format_default_rules_section(&rules);
        let expected = "## My First Rule\n\nBody of first rule.\n\n\
                        ## Second Rule\n\nBody of second rule.";
        assert_eq!(section, expected);
    }

    #[test]
    fn format_default_rules_section_trims_individual_bodies() {
        // Leading and trailing whitespace on each body is trimmed, so we
        // don't end up with weird gaps between sections.
        let rules = vec![(
            "Whitespace Rule".to_string(),
            "\n\n  Body with surrounding whitespace.  \n\n".to_string(),
        )];
        let section = format_default_rules_section(&rules);
        assert_eq!(
            section,
            "## Whitespace Rule\n\nBody with surrounding whitespace."
        );
    }

    #[test]
    fn format_default_rules_section_handles_empty_input() {
        assert_eq!(format_default_rules_section(&[]), "");
    }

    #[test]
    fn is_customized_builtin_body_returns_false_for_exact_default() {
        let default = BuiltInPrompt::CommitMessage.default_content();
        assert!(!is_customized_builtin_body(
            BuiltInPrompt::CommitMessage,
            default,
        ));
    }

    #[test]
    fn is_customized_builtin_body_ignores_surrounding_whitespace() {
        // Trailing/leading whitespace doesn't count as a real edit.
        let default = BuiltInPrompt::CommitMessage.default_content();
        let padded = format!("\n\n  {}  \n\n", default.trim());
        assert!(!is_customized_builtin_body(
            BuiltInPrompt::CommitMessage,
            &padded,
        ));
    }

    #[test]
    fn is_customized_builtin_body_returns_true_for_real_edit() {
        let mut edited = BuiltInPrompt::CommitMessage.default_content().to_string();
        edited.push_str("\n\nAlways mention the ticket number.");
        assert!(is_customized_builtin_body(
            BuiltInPrompt::CommitMessage,
            &edited,
        ));
    }

    #[test]
    fn is_customized_builtin_body_returns_true_for_completely_different_body() {
        assert!(is_customized_builtin_body(
            BuiltInPrompt::CommitMessage,
            "Use emoji and rhyming couplets.",
        ));
    }

    #[gpui::test]
    async fn append_default_rules_creates_agents_md_when_missing(cx: &mut TestAppContext) {
        let fs = FakeFs::new(cx.executor());
        let agents_md = PathBuf::from("/config/AGENTS.md");
        // Don't pre-create the file or its parent dir; `fs.write` should
        // create both.
        let rules = vec![("Rule One".to_string(), "Body one.".to_string())];

        append_default_rules_to_agents_md(fs.as_ref(), &agents_md, &rules)
            .await
            .unwrap();

        let contents = fs.load(&agents_md).await.unwrap();
        assert_eq!(contents, "## Rule One\n\nBody one.\n");
    }

    #[gpui::test]
    async fn append_default_rules_appends_to_existing_agents_md(cx: &mut TestAppContext) {
        let fs = FakeFs::new(cx.executor());
        let agents_md = PathBuf::from("/config/AGENTS.md");
        fs.create_dir(agents_md.parent().unwrap()).await.unwrap();
        fs.insert_file(
            &agents_md,
            b"# Top-level Agents Doc\n\nPre-existing user content.\n".to_vec(),
        )
        .await;

        let rules = vec![
            ("Rule One".to_string(), "Body one.".to_string()),
            ("Rule Two".to_string(), "Body two.".to_string()),
        ];
        append_default_rules_to_agents_md(fs.as_ref(), &agents_md, &rules)
            .await
            .unwrap();

        let contents = fs.load(&agents_md).await.unwrap();
        // Existing content is preserved (verbatim, just trimmed of
        // trailing whitespace), followed by a blank-line separator and
        // the appended migrated section.
        assert!(contents.starts_with("# Top-level Agents Doc\n\nPre-existing user content."));
        assert!(contents.contains("\n\n## Rule One\n\nBody one."));
        assert!(contents.contains("\n\n## Rule Two\n\nBody two.\n"));
    }

    #[gpui::test]
    async fn append_default_rules_treats_whitespace_only_file_as_empty(cx: &mut TestAppContext) {
        let fs = FakeFs::new(cx.executor());
        let agents_md = PathBuf::from("/config/AGENTS.md");
        fs.create_dir(agents_md.parent().unwrap()).await.unwrap();
        fs.insert_file(&agents_md, b"   \n\n  \n".to_vec()).await;

        let rules = vec![("Rule One".to_string(), "Body one.".to_string())];
        append_default_rules_to_agents_md(fs.as_ref(), &agents_md, &rules)
            .await
            .unwrap();

        // Existing whitespace is discarded; the result is just the
        // migrated section as if the file had been missing.
        let contents = fs.load(&agents_md).await.unwrap();
        assert_eq!(contents, "## Rule One\n\nBody one.\n");
    }

    #[gpui::test]
    async fn append_default_rules_no_op_for_empty_rules(cx: &mut TestAppContext) {
        let fs = FakeFs::new(cx.executor());
        let agents_md = PathBuf::from("/config/AGENTS.md");

        append_default_rules_to_agents_md(fs.as_ref(), &agents_md, &[])
            .await
            .unwrap();

        // The file should not have been created.
        assert!(!fs.is_file(&agents_md).await);
    }
}
