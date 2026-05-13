//! One-time migration from non-Default Rules (stored in the user's
//! `PromptStore` LMDB database) to global Agent Skills under
//! `~/.agents/skills/`.
//!
//! This is gated by:
//!
//! * the `skills` feature flag — users without it never have their Rules
//!   touched in any way;
//! * a global "migration already ran" flag persisted in
//!   [`GlobalKeyValueStore`] — keyed by [`MIGRATION_DONE_KEY`], so a
//!   `~/.agents/skills/` tree shared across release channels only gets
//!   populated once per machine.
//!
//! The migration is intentionally non-destructive: rule rows in the LMDB
//! database are left in place after their Skill counterparts are written.
//! That way users can still see and edit their Rules via the existing UI,
//! and a user who downgrades to a Zed build without skills support won't
//! lose anything.
//!
//! Migrated Skills are written with `disable-model-invocation: true` —
//! the model never auto-invokes them based on the description. This
//! preserves the original behavior of non-Default Rules, which were also
//! only included when the user explicitly invoked them by name. The
//! description defaults to a placeholder since it's never seen by the
//! model anyway.

use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

use agent_skills::{SKILL_FILE_NAME, global_skills_dir, slugify_skill_name};
use anyhow::{Context as _, Result};
use db::kvp::GlobalKeyValueStore;
use feature_flags::{FeatureFlagAppExt as _, SkillsFeatureFlag};
use fs::Fs;
use gpui::{App, TaskExt as _};
use util::ResultExt as _;

use crate::{PromptId, PromptStore};

/// Global KVP flag: set to `"1"` once the migration has been considered
/// for this machine, regardless of whether any rules were actually
/// migrated. Used to short-circuit the migration on every subsequent
/// launch.
pub const MIGRATION_DONE_KEY: &str = "rules_to_skills_migration_done";

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

        // Snapshot the (id, title) pairs for every non-Default user rule.
        // BuiltIn prompts (e.g. the commit-message prompt) are intentionally
        // excluded — they're not user-facing "Rules" in the agent sense.
        let user_rules: Vec<(PromptId, String)> = prompt_store.read_with(cx, |store, _| {
            store
                .all_prompt_metadata()
                .into_iter()
                .filter(|metadata| !metadata.default)
                .filter_map(|metadata| {
                    let _ = metadata.id.as_user()?;
                    let title = metadata.title.as_ref()?.to_string();
                    Some((metadata.id, title))
                })
                .collect()
        });

        if user_rules.is_empty() {
            mark_migration_done().await;
            return anyhow::Ok(());
        }

        let skills_dir = global_skills_dir();
        for (id, title) in user_rules {
            let body = prompt_store
                .update(cx, |store, cx| store.load(id, cx))
                .await;
            let body = match body {
                Ok(body) => body,
                Err(err) => {
                    log::warn!("Skipping rule {title:?}: failed to load body: {err:#}");
                    continue;
                }
            };

            let Some(slug) = slugify_skill_name(&title) else {
                log::warn!(
                    "Skipping rule {title:?}: title contains no characters \
                     valid for a skill name"
                );
                continue;
            };

            if let Err(err) = write_migrated_skill(fs.as_ref(), &skills_dir, &slug, &body).await {
                log::warn!("Failed to write skill for rule {title:?}: {err:#}");
            }
        }

        mark_migration_done().await;
        anyhow::Ok(())
    })
    .detach_and_log_err(cx);
}

async fn mark_migration_done() {
    GlobalKeyValueStore::global()
        .write_kvp(MIGRATION_DONE_KEY.into(), "1".into())
        .await
        .log_err();
}

/// Write a single migrated rule to disk as `<skills_dir>/<name>/SKILL.md`.
///
/// If `<skills_dir>/<slug>/` is already taken, picks the first free
/// `<slug>-2`, `<slug>-3`, … so two rules whose titles slugify to the
/// same value don't clobber each other (and an existing skill the user
/// already created by hand isn't overwritten either).
async fn write_migrated_skill(
    fs: &dyn Fs,
    skills_dir: &Path,
    slug: &str,
    body: &str,
) -> Result<()> {
    let (name, dir) = pick_available_skill_dir(fs, skills_dir, slug).await?;
    fs.create_dir(&dir).await?;
    let content = format_skill_file(&name, body);
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
}
