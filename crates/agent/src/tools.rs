mod apply_code_action_tool;
mod ask_user_tool;
mod context_server_registry;
mod copy_path_tool;
mod create_directory_tool;
mod create_thread_tool;
mod delete_path_tool;
mod diagnostics_tool;
mod edit_file_tool;
mod edit_session;
#[cfg(all(test, feature = "unit-eval"))]
mod evals;
mod fetch_tool;
mod find_path_tool;
mod find_references_tool;
mod get_code_actions_tool;
mod go_to_definition_tool;
mod grep_tool;
mod list_agents_and_models_tool;
mod list_directory_tool;
mod move_path_tool;
mod read_file_tool;
mod rename_tool;
mod skill_tool;
mod spawn_agent_tool;
mod symbol_locator;
mod terminal_tool;
mod tool_permissions;
mod web_search_tool;
mod write_file_tool;

use crate::AgentTool;
use feature_flags::{
    CreateThreadToolFeatureFlag, FeatureFlagAppExt as _, LspToolFeatureFlag, RenameToolFeatureFlag,
};
use gpui::{App, Entity};
use language_model::LanguageModelRequestTool;
use project::{Project, ProjectPath, Worktree};
use prompt_store::ProjectContext;
use serde::{
    Deserialize, Deserializer,
    de::{DeserializeOwned, Error as _},
};
use std::path::{Path, PathBuf};
use util::path_list::PathList;

/// The project roots an agent session is allowed to touch.
///
/// `None` is the unscoped case: every operation passes through untouched, so
/// callers apply a scope unconditionally (`None` => skip, `Some` => filter).
#[derive(Clone, Debug, Default)]
pub struct ProjectScope(Option<PathList>);

impl ProjectScope {
    pub fn unscoped() -> Self {
        Self(None)
    }

    pub fn from_roots(roots: PathList) -> Self {
        Self(Some(roots))
    }

    pub fn roots(&self) -> Option<&PathList> {
        self.0.as_ref()
    }

    pub fn is_unscoped(&self) -> bool {
        self.0.is_none()
    }

    /// Whether an absolute path is inside the scope.
    ///
    /// A scope entry may be a whole project root or any directory within one,
    /// so containment is a component-wise prefix test against the entries.
    /// Always `true` when unscoped.
    pub fn contains(&self, abs_path: &Path) -> bool {
        match self.0.as_ref() {
            None => true,
            Some(roots) => roots.paths().iter().any(|root| abs_path.starts_with(root)),
        }
    }

    /// Whether a worktree is *relevant* to the scope, i.e. at least one scope
    /// entry lies inside it. This decides whether a worktree takes part in
    /// enumeration at all; individual results are still filtered with
    /// [`Self::contains`], since a scope entry may itself be a subdirectory.
    pub fn intersects_worktree(&self, worktree: &Entity<Worktree>, cx: &App) -> bool {
        match self.0.as_ref() {
            None => true,
            Some(roots) => {
                let abs_path = worktree.read(cx).abs_path();
                roots
                    .paths()
                    .iter()
                    .any(|root| root.starts_with(&*abs_path))
            }
        }
    }

    /// Project roots that are entirely outside this scope.
    ///
    /// Skills and rules are discovered per root and non-recursively, so a root
    /// is only dropped when no scope entry lies inside it. Empty when unscoped.
    pub fn excluded_roots(&self, project: &Project, cx: &App) -> Vec<PathBuf> {
        let Some(roots) = self.0.as_ref() else {
            return Vec::new();
        };
        project
            .visible_worktrees(cx)
            .filter(|worktree| {
                let abs_path = worktree.read(cx).abs_path();
                !roots
                    .paths()
                    .iter()
                    .any(|root| root.starts_with(&*abs_path))
            })
            .map(|worktree| worktree.read(cx).abs_path().to_path_buf())
            .collect()
    }

    /// Resolve an agent-supplied path against the project, returning `None`
    /// when the resolved path lands outside the scope. Equivalent to
    /// `Project::find_project_path` when unscoped.
    pub fn resolve_project_path(
        &self,
        project: &Project,
        path: impl AsRef<Path>,
        cx: &App,
    ) -> Option<ProjectPath> {
        if self.is_unscoped() {
            return project.find_project_path(path, cx);
        }
        let project_path = project.find_project_path(path, cx)?;
        let abs_path = project.absolute_path(&project_path, cx)?;
        self.contains(&abs_path).then_some(project_path)
    }

    /// Derive a scoped [`ProjectContext`] for the system prompt from the
    /// project-wide one: keep worktrees the scope intersects, and drop any
    /// skill whose file lives under an entirely out-of-scope root. Global
    /// (`~/.agents/skills`) and built-in (`<built-in>`) skills never match an
    /// excluded root, so they survive. Returns `None` when unscoped.
    pub fn scoped_project_context(
        &self,
        base: &ProjectContext,
        project: &Project,
        cx: &App,
    ) -> Option<ProjectContext> {
        let roots = self.0.as_ref()?;
        let excluded_roots = self.excluded_roots(project, cx);
        let worktrees = base
            .worktrees
            .iter()
            .filter(|worktree| {
                roots
                    .paths()
                    .iter()
                    .any(|root| root.starts_with(&*worktree.abs_path))
            })
            .cloned()
            .collect::<Vec<_>>();
        let skills = base
            .skills()
            .iter()
            .filter(|skill| {
                !excluded_roots
                    .iter()
                    .any(|root| Path::new(&skill.location).starts_with(root))
            })
            .cloned()
            .collect::<Vec<_>>();
        Some(ProjectContext::new(worktrees).with_skills(skills))
    }
}

/// Deserialize a value that may have been provided as a JSON-encoded string
/// instead of the structured value. Some models occasionally stringify nested
/// arguments, so we accept either form.
pub(crate) fn deserialize_maybe_stringified<'de, T, D>(deserializer: D) -> Result<T, D::Error>
where
    T: DeserializeOwned,
    D: Deserializer<'de>,
{
    fn to_custom_error<E>(e: serde_json::Error) -> E
    where
        E: serde::de::Error,
    {
        E::custom(format!("{e}"))
    }

    let raw_value = serde_json::Value::deserialize(deserializer)
        .map_err(|error| D::Error::custom(format!("invalid JSON: {error}")))?;

    match T::deserialize(&raw_value) {
        Ok(value) => Ok(value),
        Err(original_error) => {
            let Some(string) = raw_value.as_str() else {
                return Err(to_custom_error(original_error));
            };

            serde_json::from_str(string).map_err(to_custom_error)
        }
    }
}

pub use apply_code_action_tool::*;
pub use ask_user_tool::*;
pub use context_server_registry::*;
pub use copy_path_tool::*;
pub use create_directory_tool::*;
pub use create_thread_tool::*;
pub use delete_path_tool::*;
pub use diagnostics_tool::*;
pub use edit_file_tool::*;
pub use fetch_tool::*;
pub use find_path_tool::*;
pub use find_references_tool::*;
pub use get_code_actions_tool::*;
pub use go_to_definition_tool::*;
pub use grep_tool::*;
pub use list_agents_and_models_tool::*;
pub use list_directory_tool::*;
pub use move_path_tool::*;
pub use read_file_tool::*;
pub use rename_tool::*;
pub use skill_tool::*;
pub use spawn_agent_tool::*;
pub use symbol_locator::*;

pub use terminal_tool::*;
pub use tool_permissions::*;
pub use web_search_tool::*;
pub use write_file_tool::*;

macro_rules! tools {
    ($($tool:ty),* $(,)?) => {
        /// Every built-in tool name, determined at compile time.
        pub const ALL_TOOL_NAMES: &[&str] = &[
            $(<$tool>::NAME,)*
        ];

        const _: () = {
            const fn str_eq(a: &str, b: &str) -> bool {
                let a = a.as_bytes();
                let b = b.as_bytes();
                if a.len() != b.len() {
                    return false;
                }
                let mut i = 0;
                while i < a.len() {
                    if a[i] != b[i] {
                        return false;
                    }
                    i += 1;
                }
                true
            }

            const NAMES: &[&str] = ALL_TOOL_NAMES;
            let mut i = 0;
            while i < NAMES.len() {
                let mut j = i + 1;
                while j < NAMES.len() {
                    if str_eq(NAMES[i], NAMES[j]) {
                        panic!("Duplicate tool name in tools! macro");
                    }
                    j += 1;
                }
                i += 1;
            }
        };

        /// Returns whether the tool with the given name supports the given provider.
        pub fn tool_supports_provider(name: &str, provider: &language_model::LanguageModelProviderId) -> bool {
            $(
                if name == <$tool>::NAME {
                    return <$tool>::supports_provider(provider);
                }
            )*
            false
        }

        /// Returns whether the tool with the given name may be provided to an
        /// agent in a restricted workspace. Unknown tools (e.g. MCP tools) are
        /// considered allowed.
        pub fn tool_allowed_in_restricted_mode(name: &str) -> bool {
            $(
                if name == <$tool>::NAME {
                    return <$tool>::allow_in_restricted_mode();
                }
            )*
            true
        }

        /// A list of all built-in tools
        pub fn built_in_tools() -> impl Iterator<Item = LanguageModelRequestTool> {
            fn language_model_tool<T: AgentTool>() -> LanguageModelRequestTool {
                let mut input_schema = T::input_schema().to_value();
                language_model::tool_schema::normalize_tool_schema(&mut input_schema);
                LanguageModelRequestTool::function(
                    T::NAME.to_string(),
                    T::description().to_string(),
                    input_schema,
                    T::supports_input_streaming(),
                )
            }
            [
                $(
                    language_model_tool::<$tool>(),
                )*
            ]
            .into_iter()
        }
    };
}

// Adding a tool here (and constructing it in `Thread::add_default_tools`) is
// not enough to make the model actually receive it. Three further gates will
// silently drop the tool rather than fail to compile:
//
// 1. `assets/settings/default.json`: the `write` and `ask` agent profiles each
//    carry an explicit `tools` allowlist. `Thread::enabled_tools` filters out
//    any tool not present there with value `true`, so it never reaches the
//    model.
// 2. `test_all_tools_are_in_tool_info_or_excluded` in
//    `crates/settings_ui/src/pages/tool_permissions_setup.rs`: every tool must
//    be in the permission-UI `TOOLS` list (if it calls
//    `decide_permission_from_settings`) or in `EXCLUDED_TOOLS`.
// 3. `tool_feature_flag_enabled`: some tools are gated behind a feature flag and
//    are dropped unless it is active. The agent-profile UI uses the same gate so
//    it never offers a tool the agent can't actually use.
tools! {
    ApplyCodeActionTool,
    AskUserTool,
    CopyPathTool,
    CreateDirectoryTool,
    CreateThreadTool,
    DeletePathTool,
    DiagnosticsTool,
    EditFileTool,
    FetchTool,
    FindPathTool,
    FindReferencesTool,
    GetCodeActionsTool,
    GoToDefinitionTool,
    GrepTool,
    ListAgentsAndModelsTool,
    ListDirectoryTool,
    MovePathTool,
    ReadFileTool,
    RenameTool,
    SkillTool,
    SpawnAgentTool,
    TerminalTool,
    WebSearchTool,
    WriteFileTool,
}

/// Some built-in tools are gated behind a feature flag and only become usable
/// once that flag is active. Tools without a flag are always available.
///
/// This is the single source of truth for that gating: `Thread::enabled_tools`
/// uses it to decide what the model receives, and the agent-profile
/// configuration UI uses it to decide what to offer — so the UI can never list
/// a tool the agent would silently drop (see #56778).
pub fn tool_feature_flag_enabled(tool_name: &str, cx: &App) -> bool {
    match tool_name {
        RenameTool::NAME => cx.has_flag::<RenameToolFeatureFlag>(),
        FindReferencesTool::NAME
        | GetCodeActionsTool::NAME
        | ApplyCodeActionTool::NAME
        | GoToDefinitionTool::NAME => cx.has_flag::<LspToolFeatureFlag>(),
        CreateThreadTool::NAME => cx.has_flag::<CreateThreadToolFeatureFlag>(),
        _ => true,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn built_in_tool_schemas_are_normalized() {
        let tools = built_in_tools().collect::<Vec<_>>();

        assert_eq!(tools.len(), ALL_TOOL_NAMES.len());
        for tool in tools {
            let language_model::LanguageModelRequestToolInput::Function { input_schema, .. } =
                tool.input
            else {
                panic!("built-in tool `{}` should use a JSON schema", tool.name);
            };
            assert_eq!(input_schema.get("$schema"), None, "tool `{}`", tool.name);
            assert_eq!(input_schema.get("title"), None, "tool `{}`", tool.name);
            assert_eq!(
                input_schema.get("description"),
                None,
                "tool `{}`",
                tool.name
            );
            assert!(
                input_schema["properties"].is_object(),
                "tool `{}` should have object properties",
                tool.name
            );
        }
    }

    #[test]
    fn fetch_and_terminal_are_forbidden_in_restricted_mode() {
        assert!(!tool_allowed_in_restricted_mode(FetchTool::NAME));
        assert!(!tool_allowed_in_restricted_mode(TerminalTool::NAME));

        // Every other built-in tool, and unknown (e.g. MCP) tools, are allowed.
        for name in ALL_TOOL_NAMES {
            let expected = *name != FetchTool::NAME && *name != TerminalTool::NAME;
            assert_eq!(
                tool_allowed_in_restricted_mode(name),
                expected,
                "unexpected restricted-mode policy for tool `{name}`"
            );
        }
        assert!(tool_allowed_in_restricted_mode("some_mcp_tool"));
    }

    #[test]
    fn project_scope_passes_through_when_unscoped() {
        let scope = ProjectScope::unscoped();
        assert!(scope.is_unscoped());
        assert!(scope.roots().is_none());
        assert!(scope.contains(Path::new("/root/a/file.txt")));
        assert!(scope.contains(Path::new("/elsewhere/file.txt")));
    }

    #[test]
    fn project_scope_filters_to_its_roots() {
        let scope = ProjectScope::from_roots(PathList::new(&[
            PathBuf::from("/root/a"),
            PathBuf::from("/root/b"),
        ]));
        assert!(!scope.is_unscoped());
        assert!(scope.contains(Path::new("/root/a")));
        assert!(scope.contains(Path::new("/root/a/src/main.rs")));
        assert!(scope.contains(Path::new("/root/b/nested/file.txt")));
        assert!(!scope.contains(Path::new("/root/c/file.txt")));
        // `starts_with` is component-wise, so a sibling whose name merely shares
        // a prefix with an in-scope root must not match.
        assert!(!scope.contains(Path::new("/root/ab/file.txt")));
    }

    #[test]
    fn project_scope_accepts_subpath_entries() {
        let scope = ProjectScope::from_roots(PathList::new(&[PathBuf::from("/root/a/crates")]));
        assert!(scope.contains(Path::new("/root/a/crates")));
        assert!(scope.contains(Path::new("/root/a/crates/foo/Cargo.toml")));
        assert!(!scope.contains(Path::new("/root/a")));
        assert!(!scope.contains(Path::new("/root/a/other")));
        // A sibling that merely shares a name prefix must not match.
        assert!(!scope.contains(Path::new("/root/a/crates-extra/x")));
    }
}
