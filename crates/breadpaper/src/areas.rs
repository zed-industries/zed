use anyhow::{Context as _, Result, bail};
use serde::Deserialize;
use std::fs;
use std::path::{Component, Path, PathBuf};

use crate::vault::{VAULT_MARKER_DIR, Vault, write_if_missing};

pub const TIMELINE_AREA_ID: &str = "timeline";
pub const INSTALLED_AREAS_DIR: &str = "areas";
pub const AREA_MANIFEST_FILE: &str = "manifest.toml";

const TIMELINE_MANIFEST: &str = include_str!("../assets/areas/timeline/manifest.toml");
const TIMELINE_DOC: &str = include_str!("../assets/areas/timeline/doc.md");
const TIMELINE_WEEK_REVIEW_SKILL: &str =
    include_str!("../assets/areas/timeline/skills/week-review.md");
const TIMELINE_WRAP_TODAY_SKILL: &str =
    include_str!("../assets/areas/timeline/skills/wrap-today.md");
const TIMELINE_WRAP_YESTERDAY_SKILL: &str =
    include_str!("../assets/areas/timeline/skills/wrap-yesterday.md");
const TIMELINE_DASHBOARD_HTML: &str = include_str!("../assets/areas/timeline/assets/index.html");
const TIMELINE_DASHBOARD_SEED: &str =
    include_str!("../assets/areas/timeline/assets/data.seed.js");

/// The parsed shape of an Area's `manifest.toml` (§5.2 of the V3 spec).
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct AreaManifestContent {
    schema: Option<u32>,
    id: String,
    name: String,
    version: Option<u32>,
    summary: Option<String>,
    doc: String,
    #[serde(default)]
    scaffold: Vec<ScaffoldEntryContent>,
    #[serde(default)]
    skill: Vec<AreaSkillContent>,
    #[serde(default)]
    surface: Vec<AreaSurfaceContent>,
}

impl AreaManifestContent {
    fn resolve(self) -> Result<AreaManifest> {
        Ok(AreaManifest {
            schema: self.schema.unwrap_or(1),
            id: self.id,
            name: self.name,
            version: self.version.unwrap_or(1),
            summary: self.summary.unwrap_or_default(),
            doc: self.doc,
            scaffold: self
                .scaffold
                .into_iter()
                .map(ScaffoldEntryContent::resolve)
                .collect::<Result<_>>()?,
            skills: self.skill.into_iter().map(AreaSkillContent::resolve).collect(),
            surfaces: self
                .surface
                .into_iter()
                .map(AreaSurfaceContent::resolve)
                .collect(),
        })
    }
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct ScaffoldEntryContent {
    kind: String,
    path: String,
    source: Option<String>,
}

impl ScaffoldEntryContent {
    fn resolve(self) -> Result<ScaffoldEntry> {
        match self.kind.as_str() {
            "dir" => Ok(ScaffoldEntry::Dir { path: self.path }),
            "file" => Ok(ScaffoldEntry::File {
                source: self.source.with_context(|| {
                    format!("scaffold file entry {:?} is missing a source", self.path)
                })?,
                path: self.path,
            }),
            other => bail!("unknown scaffold kind {other:?} (expected \"dir\" or \"file\")"),
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct AreaSkillContent {
    id: String,
    name: String,
    file: String,
    summary: Option<String>,
    #[serde(default)]
    reads: Vec<String>,
    #[serde(default)]
    writes: Vec<String>,
}

impl AreaSkillContent {
    fn resolve(self) -> AreaSkill {
        AreaSkill {
            id: self.id,
            name: self.name,
            file: self.file,
            summary: self.summary.unwrap_or_default(),
            reads: self.reads,
            writes: self.writes,
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct AreaSurfaceContent {
    kind: String,
    name: String,
    open: String,
}

impl AreaSurfaceContent {
    fn resolve(self) -> AreaSurface {
        AreaSurface {
            kind: self.kind,
            name: self.name,
            open: self.open,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct AreaManifest {
    pub schema: u32,
    pub id: String,
    pub name: String,
    pub version: u32,
    pub summary: String,
    /// Vault-relative path the explainer doc is materialized to.
    pub doc: String,
    pub scaffold: Vec<ScaffoldEntry>,
    pub skills: Vec<AreaSkill>,
    pub surfaces: Vec<AreaSurface>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ScaffoldEntry {
    Dir { path: String },
    /// `path` is the vault-relative destination; `source` is the asset path
    /// within the catalog package.
    File { path: String, source: String },
}

#[derive(Debug, Clone, PartialEq)]
pub struct AreaSkill {
    pub id: String,
    pub name: String,
    /// Vault-relative path the skill file is materialized to.
    pub file: String,
    pub summary: String,
    /// Declared read scope; surfaced only, not enforced in V3.
    pub reads: Vec<String>,
    /// Declared write scope; surfaced only, not enforced in V3.
    pub writes: Vec<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct AreaSurface {
    pub kind: String,
    pub name: String,
    /// Vault-relative path this surface opens.
    pub open: String,
}

fn parse_manifest(manifest_toml: &str) -> Result<AreaManifest> {
    toml::from_str::<AreaManifestContent>(manifest_toml)
        .context("parsing Area manifest")?
        .resolve()
}

/// Vault-relative root under which Claude Code discovers project skills. Its
/// files are generated from the manifest, not shipped as static assets.
pub const CLAUDE_SKILLS_DIR: &str = ".claude/skills";

/// A file an Area ships into the vault, pairing the vault-relative destination
/// with the asset path inside the catalog package it came from.
struct ShippedFile {
    destination: String,
    source: String,
}

/// A `.claude/skills/<id>/SKILL.md` bridge generated from a skill's manifest
/// entry. Claude Code only discovers skills under `.claude/skills/`, so each
/// Area skill gets a thin bridge there whose front matter Claude Code reads and
/// whose body points back to the canonical skill file — keeping one source of
/// truth for the ritual itself.
struct ClaudeBridge {
    destination: String,
    content: String,
}

fn yaml_quote(value: &str) -> String {
    let escaped = value.replace('\\', "\\\\").replace('"', "\\\"");
    format!("\"{escaped}\"")
}

fn claude_bridge_content(skill: &AreaSkill) -> String {
    let mut content = format!(
        "---\nname: {name}\ndescription: {description}\ndisable-model-invocation: true\n---\n\n\
         This is a BreadPaper Area skill. Read and follow the full instructions in\n\
         `{file}` (relative to the vault root), then carry out the ritual it describes.\n\
         It appends to your notes and never rewrites what you wrote.\n",
        name = yaml_quote(&skill.name),
        description = yaml_quote(&skill.summary),
        file = skill.file,
    );
    if !skill.reads.is_empty() {
        content.push_str(&format!("\nReads: {}\n", skill.reads.join(", ")));
    }
    if !skill.writes.is_empty() {
        content.push_str(&format!("Writes: {}\n", skill.writes.join(", ")));
    }
    content
}

fn claude_bridge_files(manifest: &AreaManifest) -> Vec<ClaudeBridge> {
    manifest
        .skills
        .iter()
        .map(|skill| ClaudeBridge {
            destination: format!("{CLAUDE_SKILLS_DIR}/{}/SKILL.md", skill.id),
            content: claude_bridge_content(skill),
        })
        .collect()
}

fn shipped_files(manifest: &AreaManifest) -> Vec<ShippedFile> {
    let mut files = vec![ShippedFile {
        destination: manifest.doc.clone(),
        source: "doc.md".to_string(),
    }];
    for skill in &manifest.skills {
        files.push(ShippedFile {
            destination: skill.file.clone(),
            source: format!("skills/{}.md", skill.id),
        });
    }
    for entry in &manifest.scaffold {
        if let ScaffoldEntry::File { path, source } = entry {
            files.push(ShippedFile {
                destination: path.clone(),
                source: source.clone(),
            });
        }
    }
    files
}

/// An Area package shipped inside the BreadPaper binary.
pub struct CatalogArea {
    pub manifest: AreaManifest,
    manifest_toml: &'static str,
    assets: &'static [(&'static str, &'static str)],
}

impl CatalogArea {
    fn asset(&self, source: &str) -> Option<&'static str> {
        self.assets
            .iter()
            .find(|(path, _)| *path == source)
            .map(|(_, contents)| *contents)
    }
}

/// The app-shipped Area catalog, in gallery order.
pub fn catalog() -> Result<Vec<CatalogArea>> {
    Ok(vec![CatalogArea {
        manifest: parse_manifest(TIMELINE_MANIFEST)
            .context("parsing the bundled Timeline Area manifest")?,
        manifest_toml: TIMELINE_MANIFEST,
        assets: &[
            ("doc.md", TIMELINE_DOC),
            ("skills/week-review.md", TIMELINE_WEEK_REVIEW_SKILL),
            ("skills/wrap-today.md", TIMELINE_WRAP_TODAY_SKILL),
            ("skills/wrap-yesterday.md", TIMELINE_WRAP_YESTERDAY_SKILL),
            ("assets/index.html", TIMELINE_DASHBOARD_HTML),
            ("assets/data.seed.js", TIMELINE_DASHBOARD_SEED),
        ],
    }])
}

pub fn catalog_area(area_id: &str) -> Result<Option<CatalogArea>> {
    Ok(catalog()?
        .into_iter()
        .find(|area| area.manifest.id == area_id))
}

/// Joins a manifest-declared vault-relative path onto the vault root,
/// rejecting absolute paths and `..` so a manifest can never reach outside
/// the vault.
pub fn vault_file_path(vault_root: &Path, relative: &str) -> Result<PathBuf> {
    let relative_path = Path::new(relative);
    let plain = !relative_path.as_os_str().is_empty()
        && relative_path
            .components()
            .all(|component| matches!(component, Component::Normal(_)));
    if !plain {
        bail!("Area path {relative:?} must be a plain vault-relative path");
    }
    Ok(vault_root.join(relative_path))
}

fn installed_area_dir(vault_root: &Path, area_id: &str) -> PathBuf {
    vault_root
        .join(VAULT_MARKER_DIR)
        .join(INSTALLED_AREAS_DIR)
        .join(area_id)
}

pub fn installed_manifest_path(vault_root: &Path, area_id: &str) -> PathBuf {
    installed_area_dir(vault_root, area_id).join(AREA_MANIFEST_FILE)
}

/// Loads the installed provenance copy of an Area's manifest, if present.
pub fn load_installed_manifest(vault_root: &Path, area_id: &str) -> Result<Option<AreaManifest>> {
    let manifest_path = installed_manifest_path(vault_root, area_id);
    let manifest_toml = match fs::read_to_string(&manifest_path) {
        Ok(manifest_toml) => manifest_toml,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(None),
        Err(error) => {
            return Err(error)
                .with_context(|| format!("reading {}", manifest_path.display()));
        }
    };
    parse_manifest(&manifest_toml)
        .with_context(|| format!("parsing {}", manifest_path.display()))
        .map(Some)
}

/// Writes an Area's editable files into the vault (create-if-missing, never
/// clobbering) and records the installed manifest copy under
/// `.breadpaper/areas/<id>/`. Idempotent; missing files are re-materialized.
/// Blocking I/O — call from a background thread.
pub fn materialize_area(vault_root: &Path, area: &CatalogArea) -> Result<()> {
    for entry in &area.manifest.scaffold {
        if let ScaffoldEntry::Dir { path } = entry {
            let dir = vault_file_path(vault_root, path)?;
            fs::create_dir_all(&dir).with_context(|| format!("creating {}", dir.display()))?;
        }
    }
    for file in shipped_files(&area.manifest) {
        let contents = area.asset(&file.source).with_context(|| {
            format!(
                "the {} Area package has no asset {:?}",
                area.manifest.id, file.source
            )
        })?;
        write_if_missing(&vault_file_path(vault_root, &file.destination)?, contents)?;
    }
    for bridge in claude_bridge_files(&area.manifest) {
        write_if_missing(
            &vault_file_path(vault_root, &bridge.destination)?,
            &bridge.content,
        )?;
    }

    // The installed manifest is provenance owned by the app, not a user file,
    // so re-installing overwrites it to record the current package.
    let manifest_path = installed_manifest_path(vault_root, &area.manifest.id);
    if let Some(parent) = manifest_path.parent() {
        fs::create_dir_all(parent).with_context(|| format!("creating {}", parent.display()))?;
    }
    fs::write(&manifest_path, area.manifest_toml)
        .with_context(|| format!("writing {}", manifest_path.display()))?;
    Ok(())
}

/// Materializes a catalog Area and registers it (enabled) in the vault
/// config. Registration happens last, so a failed install never leaves a
/// registered-but-missing Area behind. Blocking I/O — call from a background
/// thread.
pub fn install_area(vault_root: &Path, area_id: &str) -> Result<()> {
    let area = catalog_area(area_id)?
        .with_context(|| format!("no Area {area_id:?} in the catalog"))?;
    materialize_area(vault_root, &area)?;
    crate::vault::update_areas_registry(vault_root, |installed| {
        if let Some(entry) = installed.iter_mut().find(|entry| entry.id == area_id) {
            entry.enabled = true;
            entry.version = area.manifest.version;
        } else {
            installed.push(crate::vault::InstalledArea {
                id: area_id.to_string(),
                enabled: true,
                version: area.manifest.version,
            });
        }
    })
}

/// Re-materializes every enabled installed Area (create-if-missing), so a vault
/// opened after an app update gains any newly shipped Area files — new skills
/// and their Claude Code bridges — without a manual reinstall. Idempotent and
/// never clobbers user edits; the registry is left untouched. Areas that are
/// registered but absent from the catalog are skipped. Blocking I/O — call from
/// a background thread.
pub fn reconcile_enabled_areas(vault: &Vault) -> Result<()> {
    for entry in &vault.config.areas.installed {
        if !entry.enabled {
            continue;
        }
        if let Some(area) = catalog_area(&entry.id)? {
            materialize_area(&vault.root, &area)?;
        }
    }
    Ok(())
}

/// Disables an Area in the registry without touching any files.
pub fn deactivate_area(vault_root: &Path, area_id: &str) -> Result<()> {
    crate::vault::update_areas_registry(vault_root, |installed| {
        if let Some(entry) = installed.iter_mut().find(|entry| entry.id == area_id) {
            entry.enabled = false;
        }
    })
}

/// The manifests of the vault's enabled Areas, in registry order. Individual
/// load failures are logged and skipped so one broken Area can't blank the
/// whole panel section.
pub fn enabled_areas(vault: &Vault) -> Vec<AreaManifest> {
    let mut manifests = Vec::new();
    for entry in &vault.config.areas.installed {
        if !entry.enabled {
            continue;
        }
        match load_installed_manifest(&vault.root, &entry.id) {
            Ok(Some(manifest)) => manifests.push(manifest),
            Ok(None) => match catalog_area(&entry.id) {
                Ok(Some(area)) => manifests.push(area.manifest),
                Ok(None) => log::warn!(
                    "BreadPaper: Area {:?} is registered but has no installed manifest and is not in the catalog",
                    entry.id
                ),
                Err(error) => {
                    log::error!("BreadPaper: couldn't load the Areas catalog: {error:?}")
                }
            },
            Err(error) => log::warn!(
                "BreadPaper: couldn't load the installed manifest for Area {:?}: {error:?}",
                entry.id
            ),
        }
    }
    manifests
}

/// What removing an Area's files would do, computed before the user confirms.
#[derive(Debug, PartialEq)]
pub struct RemovalPlan {
    pub area_name: String,
    /// Vault-relative shipped files that still match their catalog source.
    pub delete: Vec<String>,
    /// Vault-relative shipped files modified since install; always preserved.
    pub keep_modified: Vec<String>,
}

/// Compares the Area's shipped files (from the installed manifest, falling
/// back to the catalog manifest) against their catalog sources. Files that
/// differ — edited by the user or their LLM — are preserved, never deleted.
/// Blocking I/O — call from a background thread.
pub fn plan_removal(vault_root: &Path, area_id: &str) -> Result<RemovalPlan> {
    let catalog_area = catalog_area(area_id)?;
    let manifest = match load_installed_manifest(vault_root, area_id)? {
        Some(manifest) => manifest,
        None => catalog_area
            .as_ref()
            .map(|area| area.manifest.clone())
            .with_context(|| {
                format!("Area {area_id:?} has no installed manifest and is not in the catalog")
            })?,
    };

    let mut plan = RemovalPlan {
        area_name: manifest.name.clone(),
        delete: Vec::new(),
        keep_modified: Vec::new(),
    };
    for file in shipped_files(&manifest) {
        let path = vault_file_path(vault_root, &file.destination)?;
        let current = match fs::read(&path) {
            Ok(current) => current,
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => continue,
            Err(error) => {
                return Err(error).with_context(|| format!("reading {}", path.display()));
            }
        };
        let original = catalog_area
            .as_ref()
            .and_then(|area| area.asset(&file.source));
        match original {
            Some(original) if current == original.as_bytes() => {
                plan.delete.push(file.destination);
            }
            // No catalog source to compare against (or the contents differ):
            // treat the file as user-modified and keep it.
            _ => plan.keep_modified.push(file.destination),
        }
    }
    // Claude Code bridges are generated, not shipped as assets, so compare each
    // against its freshly-generated content instead of a catalog source.
    for bridge in claude_bridge_files(&manifest) {
        let path = vault_file_path(vault_root, &bridge.destination)?;
        let current = match fs::read_to_string(&path) {
            Ok(current) => current,
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => continue,
            Err(error) => {
                return Err(error).with_context(|| format!("reading {}", path.display()));
            }
        };
        if current == bridge.content {
            plan.delete.push(bridge.destination);
        } else {
            plan.keep_modified.push(bridge.destination);
        }
    }
    Ok(plan)
}

#[derive(Debug, PartialEq)]
pub struct RemovalOutcome {
    pub deleted: Vec<String>,
    pub kept_modified: Vec<String>,
}

/// Deletes the Area's unmodified shipped files, its installed manifest, and
/// its registry entry. Recomputes the plan at deletion time so edits made
/// while the confirmation dialog was open are still preserved. Blocking I/O —
/// call from a background thread.
pub fn delete_area(vault_root: &Path, area_id: &str) -> Result<RemovalOutcome> {
    let plan = plan_removal(vault_root, area_id)?;
    let mut deleted = Vec::new();
    for destination in &plan.delete {
        let path = vault_file_path(vault_root, destination)?;
        match fs::remove_file(&path) {
            Ok(()) => {
                deleted.push(destination.clone());
                prune_empty_parents(vault_root, &path);
            }
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
            Err(error) => {
                return Err(error).with_context(|| format!("deleting {}", path.display()));
            }
        }
    }

    let installed_dir = installed_area_dir(vault_root, area_id);
    if installed_dir.exists() {
        fs::remove_dir_all(&installed_dir)
            .with_context(|| format!("deleting {}", installed_dir.display()))?;
        // `installed_dir` is now gone; prune from it as the deleted entry so
        // pruning starts at its parent (`.breadpaper/areas/`). The walk stops
        // at `.breadpaper/` because config.toml keeps it non-empty.
        prune_empty_parents(vault_root, &installed_dir);
    }

    crate::vault::update_areas_registry(vault_root, |installed| {
        installed.retain(|entry| entry.id != area_id);
    })?;

    Ok(RemovalOutcome {
        deleted,
        kept_modified: plan.keep_modified,
    })
}

/// Removes now-empty directories left behind by a deleted file, walking up
/// toward (but never past) the vault root.
fn prune_empty_parents(vault_root: &Path, deleted_file: &Path) {
    let mut current = deleted_file.parent();
    while let Some(directory) = current {
        if directory == vault_root || !directory.starts_with(vault_root) {
            break;
        }
        // remove_dir refuses to delete non-empty directories, which is
        // exactly the stop condition; any failure just means the directory
        // stays behind.
        if fs::remove_dir(directory).is_err() {
            break;
        }
        current = directory.parent();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vault::{VaultStatus, scaffold_vault};

    fn detect(root: &Path) -> Vault {
        match Vault::detect(root) {
            VaultStatus::Valid(vault) => vault,
            other => panic!("expected valid vault, got {other:?}"),
        }
    }

    #[test]
    fn catalog_parses() {
        let catalog = catalog().unwrap();
        assert_eq!(catalog.len(), 1);
        let manifest = &catalog[0].manifest;
        assert_eq!(manifest.id, TIMELINE_AREA_ID);
        assert_eq!(manifest.version, 1);
        assert_eq!(manifest.doc, "areas/Timeline.md");
        assert_eq!(manifest.skills.len(), 3);
        assert_eq!(manifest.skills[0].file, "skills/timeline/week-review.md");
        assert_eq!(manifest.skills[1].file, "skills/timeline/wrap-today.md");
        assert_eq!(manifest.skills[2].file, "skills/timeline/wrap-yesterday.md");
        assert_eq!(manifest.surfaces.len(), 1);
        assert_eq!(manifest.surfaces[0].open, "_weekly/site/index.html");
        // Every shipped file must have a bundled asset behind it.
        for file in shipped_files(manifest) {
            assert!(
                catalog[0].asset(&file.source).is_some(),
                "missing asset {:?}",
                file.source
            );
        }
    }

    #[test]
    fn scaffold_preinstalls_timeline_area() {
        let dir = tempfile::tempdir().unwrap();
        scaffold_vault(dir.path()).unwrap();
        assert!(dir.path().join("areas/Timeline.md").is_file());
        assert!(dir.path().join("skills/timeline/week-review.md").is_file());
        assert!(dir.path().join("_weekly/site/index.html").is_file());
        assert!(dir.path().join("_weekly/site/data.js").is_file());
        assert!(installed_manifest_path(dir.path(), TIMELINE_AREA_ID).is_file());
        // Claude Code bridges are generated for every skill so a `claude`
        // session opened in the vault can invoke them via `/<skill-id>`.
        for skill_id in ["week-review", "wrap-today", "wrap-yesterday"] {
            assert!(
                dir.path()
                    .join(format!(".claude/skills/{skill_id}/SKILL.md"))
                    .is_file(),
                "missing Claude bridge for {skill_id}"
            );
        }

        let vault = detect(dir.path());
        let installed = &vault.config.areas.installed;
        assert_eq!(installed.len(), 1);
        assert_eq!(installed[0].id, TIMELINE_AREA_ID);
        assert!(installed[0].enabled);
        assert_eq!(enabled_areas(&vault).len(), 1);
    }

    #[test]
    fn claude_bridges_carry_frontmatter_and_survive_edits() {
        let dir = tempfile::tempdir().unwrap();
        scaffold_vault(dir.path()).unwrap();
        let bridge = dir.path().join(".claude/skills/wrap-today/SKILL.md");
        let content = fs::read_to_string(&bridge).unwrap();
        // Diego's choice: explicit `/wrap-today` only, never model-invoked.
        assert!(content.contains("disable-model-invocation: true"));
        // Body points back to the canonical skill file — one source of truth.
        assert!(content.contains("skills/timeline/wrap-today.md"));

        // create-if-missing: a user (or their LLM) edit survives re-materialize.
        fs::write(&bridge, "edited by hand").unwrap();
        let area = catalog_area(TIMELINE_AREA_ID).unwrap().unwrap();
        materialize_area(dir.path(), &area).unwrap();
        assert_eq!(fs::read_to_string(&bridge).unwrap(), "edited by hand");

        // An edited bridge is preserved on removal, like any user-touched file.
        let plan = plan_removal(dir.path(), TIMELINE_AREA_ID).unwrap();
        assert!(
            plan.keep_modified
                .contains(&".claude/skills/wrap-today/SKILL.md".to_string())
        );
    }

    #[test]
    fn reconcile_backfills_missing_files_on_existing_vault() {
        let dir = tempfile::tempdir().unwrap();
        scaffold_vault(dir.path()).unwrap();
        // Simulate a vault scaffolded by an older app that lacked bridges and
        // the wrap skills: drop the whole `.claude` tree and one skill file.
        fs::remove_dir_all(dir.path().join(".claude")).unwrap();
        fs::remove_file(dir.path().join("skills/timeline/wrap-yesterday.md")).unwrap();

        let vault = detect(dir.path());
        reconcile_enabled_areas(&vault).unwrap();
        assert!(dir.path().join("skills/timeline/wrap-yesterday.md").is_file());
        assert!(
            dir.path()
                .join(".claude/skills/wrap-today/SKILL.md")
                .is_file()
        );
        assert!(
            dir.path()
                .join(".claude/skills/week-review/SKILL.md")
                .is_file()
        );
    }

    #[test]
    fn reconcile_skips_disabled_areas() {
        let dir = tempfile::tempdir().unwrap();
        scaffold_vault(dir.path()).unwrap();
        deactivate_area(dir.path(), TIMELINE_AREA_ID).unwrap();
        fs::remove_dir_all(dir.path().join(".claude")).unwrap();

        let vault = detect(dir.path());
        reconcile_enabled_areas(&vault).unwrap();
        assert!(!dir.path().join(".claude").exists());
    }

    #[test]
    fn materialize_never_clobbers() {
        let dir = tempfile::tempdir().unwrap();
        scaffold_vault(dir.path()).unwrap();
        let skill_path = dir.path().join("skills/timeline/week-review.md");
        fs::write(&skill_path, "my edited skill").unwrap();

        let area = catalog_area(TIMELINE_AREA_ID).unwrap().unwrap();
        materialize_area(dir.path(), &area).unwrap();
        assert_eq!(fs::read_to_string(&skill_path).unwrap(), "my edited skill");
    }

    #[test]
    fn plan_keeps_modified_files_and_delete_preserves_them() {
        let dir = tempfile::tempdir().unwrap();
        scaffold_vault(dir.path()).unwrap();
        let skill_path = dir.path().join("skills/timeline/week-review.md");
        fs::write(&skill_path, "my edited skill").unwrap();

        let plan = plan_removal(dir.path(), TIMELINE_AREA_ID).unwrap();
        assert_eq!(
            plan.keep_modified,
            vec!["skills/timeline/week-review.md".to_string()]
        );
        assert!(plan.delete.contains(&"areas/Timeline.md".to_string()));
        assert!(plan.delete.contains(&"_weekly/site/index.html".to_string()));

        let outcome = delete_area(dir.path(), TIMELINE_AREA_ID).unwrap();
        assert_eq!(
            outcome.kept_modified,
            vec!["skills/timeline/week-review.md".to_string()]
        );
        assert!(skill_path.is_file());
        assert!(!dir.path().join("areas/Timeline.md").is_file());
        assert!(!dir.path().join("_weekly/site").exists());
        // The generated bridges are unmodified, so removal deletes them and
        // prunes the now-empty `.claude` tree.
        assert!(!dir.path().join(".claude").exists());
        assert!(!installed_manifest_path(dir.path(), TIMELINE_AREA_ID).exists());
        // The now-empty installed-areas dir is pruned, but `.breadpaper/`
        // survives because config.toml keeps it non-empty.
        assert!(
            !dir.path()
                .join(VAULT_MARKER_DIR)
                .join(INSTALLED_AREAS_DIR)
                .exists()
        );
        assert!(dir.path().join(VAULT_MARKER_DIR).is_dir());

        let vault = detect(dir.path());
        assert!(vault.config.areas.installed.is_empty());
        // Core config survives the registry rewrite.
        assert_eq!(vault.config.daily.dir, "daily");
    }

    #[test]
    fn delete_never_touches_user_notes() {
        let dir = tempfile::tempdir().unwrap();
        scaffold_vault(dir.path()).unwrap();
        let daily_note = dir.path().join("daily/2026-07-20.md");
        fs::write(&daily_note, "my day").unwrap();
        let weekly_note = dir.path().join("weekly/2026-W30.md");
        fs::write(&weekly_note, "my week").unwrap();

        delete_area(dir.path(), TIMELINE_AREA_ID).unwrap();
        assert_eq!(fs::read_to_string(&daily_note).unwrap(), "my day");
        assert_eq!(fs::read_to_string(&weekly_note).unwrap(), "my week");
    }

    #[test]
    fn deactivate_then_reinstall_roundtrip() {
        let dir = tempfile::tempdir().unwrap();
        scaffold_vault(dir.path()).unwrap();

        deactivate_area(dir.path(), TIMELINE_AREA_ID).unwrap();
        let vault = detect(dir.path());
        assert!(!vault.config.areas.installed[0].enabled);
        assert!(enabled_areas(&vault).is_empty());
        // Files stay on disk.
        assert!(dir.path().join("areas/Timeline.md").is_file());

        install_area(dir.path(), TIMELINE_AREA_ID).unwrap();
        let vault = detect(dir.path());
        assert!(vault.config.areas.installed[0].enabled);
        assert_eq!(vault.config.areas.installed.len(), 1);
    }

    #[test]
    fn install_rematerializes_missing_files() {
        let dir = tempfile::tempdir().unwrap();
        scaffold_vault(dir.path()).unwrap();
        let doc_path = dir.path().join("areas/Timeline.md");
        fs::remove_file(&doc_path).unwrap();

        install_area(dir.path(), TIMELINE_AREA_ID).unwrap();
        assert!(doc_path.is_file());
    }

    #[test]
    fn vault_file_path_rejects_escapes() {
        let root = Path::new("/vault");
        assert!(vault_file_path(root, "notes/ok.md").is_ok());
        assert!(vault_file_path(root, "../outside.md").is_err());
        assert!(vault_file_path(root, "/etc/passwd").is_err());
        assert!(vault_file_path(root, "").is_err());
    }
}
