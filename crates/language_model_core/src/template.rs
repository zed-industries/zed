//! Per-request expansion of template variables used in user-configured
//! settings such as LLM provider `custom_headers`.
//!
//! Templates use the syntax `${namespace:value}` and are expanded fresh on
//! every request, so values like the current git branch reflect live state.
//!
//! Only a fixed, audited set of namespaces is supported. In particular there
//! is intentionally no arbitrary shell execution: `git:*` helpers run a small
//! set of hardcoded git subcommands (the user never supplies the command), and
//! `env:*` only reads process environment variables. This keeps the feature
//! safe to evaluate even when settings originate from an untrusted source.
//!
//! Supported namespaces:
//! - `${git:branch}` - current branch name (empty when detached/not a repo)
//! - `${git:remote}` - URL(s) of all remotes, comma-separated
//! - `${git:sha}`    - short HEAD commit SHA
//! - `${env:VAR}`    - value of environment variable `VAR`
//! - `${env:VAR:-default}` - value of `VAR`, or `default` when unset/empty
//!
//! Adding a new namespace is a matter of extending [`expand_variable`].
//!
//! Expansion is async because `git:*` helpers shell out to `git`. Callers should
//! run it on a background executor so the request path is never blocked, and
//! should bound the total time (see `language_models`'s wrapper, which applies a
//! timeout using the GPUI executor's timer).

use http_client::CustomHeaders;
use http_client::http::HeaderValue;
use std::path::{Path, PathBuf};

/// Context available to template expansion for a single request.
#[derive(Clone, Debug, Default)]
pub struct TemplateContext {
    /// Working directory used to resolve `git:*` helpers. Typically the active
    /// project's worktree root. When `None`, git helpers expand to an empty
    /// string rather than running against an arbitrary directory.
    pub project_root: Option<PathBuf>,
}

impl TemplateContext {
    pub fn new(project_root: Option<PathBuf>) -> Self {
        Self { project_root }
    }
}

/// Expand every template variable found in the values of `headers`, returning a
/// new [`CustomHeaders`] suitable for appending to an outgoing request.
///
/// Header names are never templated. When no header value contains a template,
/// the original `headers` is cloned cheaply (it is `Arc`-backed) and returned
/// unchanged. Values whose expansion is no longer a valid HTTP header value are
/// dropped with a warning.
pub async fn expand_custom_headers(
    headers: &CustomHeaders,
    context: &TemplateContext,
) -> CustomHeaders {
    let needs_expansion = headers
        .iter()
        .any(|(_, value)| value.to_str().map(contains_template).unwrap_or(false));
    if !needs_expansion {
        return headers.clone();
    }

    let mut expanded = Vec::with_capacity(headers.iter().len());
    for (name, value) in headers.iter() {
        let Ok(value_str) = value.to_str() else {
            // Non-UTF-8 values cannot contain templates; keep as-is.
            expanded.push((name.clone(), value.clone()));
            continue;
        };
        if !contains_template(value_str) {
            expanded.push((name.clone(), value.clone()));
            continue;
        }
        let value_string = expand_template(value_str, context).await;
        match HeaderValue::from_str(&value_string) {
            Ok(header_value) => expanded.push((name.clone(), header_value)),
            Err(err) => {
                log::warn!(
                    "dropping custom header `{name}`: expanded value is not a valid header value ({err})"
                );
            }
        }
    }
    CustomHeaders::new(expanded)
}

/// Returns `true` if `input` contains at least one `${...}` template variable.
///
/// Cheap pre-check so callers can avoid all per-request work when there is
/// nothing to expand.
pub fn contains_template(input: &str) -> bool {
    input.contains("${")
}

/// Expand all `${namespace:value}` variables in `input` using `context`.
///
/// Unknown namespaces, malformed templates, and failed helpers expand to an
/// empty string (a warning is logged) and the surrounding literal text is
/// preserved. Expansion is single-pass: the output of one variable is never
/// re-scanned for further templates.
pub async fn expand_template(input: &str, context: &TemplateContext) -> String {
    if !contains_template(input) {
        return input.to_string();
    }

    let mut output = String::with_capacity(input.len());
    let mut cursor = 0;

    while cursor < input.len() {
        // Look for the next `${` starting at `cursor`.
        let Some(start) = input[cursor..].find("${") else {
            output.push_str(&input[cursor..]);
            break;
        };
        let start = cursor + start;
        // Emit the literal text preceding the template.
        output.push_str(&input[cursor..start]);

        // Find the matching closing brace. We do not support nesting.
        let after_open = start + 2;
        let Some(end_offset) = input[after_open..].find('}') else {
            // Malformed template (`${` with no closing `}`): leave the rest of
            // the string as a literal and stop.
            log::warn!("unterminated template variable in `{input}`; leaving as literal");
            output.push_str(&input[start..]);
            break;
        };
        let end = after_open + end_offset;
        let expression = &input[after_open..end];

        output.push_str(&expand_variable(expression, context).await);
        cursor = end + 1;
    }

    output
}

/// Expand a single template expression (the text between `${` and `}`).
///
/// To add a new namespace, add a match arm here.
async fn expand_variable(expression: &str, context: &TemplateContext) -> String {
    let Some((namespace, value)) = expression.split_once(':') else {
        log::warn!(
            "ignoring template variable `${{{expression}}}`: missing `namespace:value` form"
        );
        return String::new();
    };

    match namespace {
        "git" => expand_git(value, context).await,
        "env" => expand_env(value),
        _ => {
            log::warn!(
                "ignoring template variable `${{{expression}}}`: unknown namespace `{namespace}`"
            );
            String::new()
        }
    }
}

async fn expand_git(helper: &str, context: &TemplateContext) -> String {
    let Some(working_dir) = context.project_root.as_deref() else {
        return String::new();
    };

    match helper {
        "branch" => run_git(working_dir, &["branch", "--show-current"]).await,
        "sha" => run_git(working_dir, &["rev-parse", "--short", "HEAD"]).await,
        "remote" => git_remotes(working_dir).await,
        _ => {
            log::warn!("ignoring template variable `${{git:{helper}}}`: unknown git helper");
            String::new()
        }
    }
}

/// Return the URLs of all configured remotes, comma-separated and de-duplicated
/// while preserving order. Empty when there are no remotes.
async fn git_remotes(working_dir: &Path) -> String {
    let remotes = run_git(working_dir, &["remote"]).await;
    if remotes.is_empty() {
        return String::new();
    }

    let mut urls: Vec<String> = Vec::new();
    for remote in remotes.lines() {
        let remote = remote.trim();
        if remote.is_empty() {
            continue;
        }
        let url = sanitize_remote_url(&run_git(working_dir, &["remote", "get-url", remote]).await);
        if !url.is_empty() && !urls.iter().any(|existing| existing == &url) {
            urls.push(url);
        }
    }
    urls.join(",")
}

/// Remove any embedded credentials from a remote URL before it leaves the
/// machine in a request header.
///
/// `git remote get-url` echoes back exactly what is configured, which for HTTP
/// remotes can include userinfo such as `https://user:token@host/...` or
/// `https://token@host/...`. We strip the `userinfo@` portion from URLs that use
/// a `scheme://` form so tokens are never sent to a gateway. SCP-style SSH
/// addresses (`git@github.com:org/repo.git`) carry only a username (not a
/// secret) and have no `://`, so they are left untouched.
fn sanitize_remote_url(url: &str) -> String {
    let Some(scheme_end) = url.find("://") else {
        return url.to_string();
    };
    let after_scheme_start = scheme_end + 3;
    let (scheme, rest) = url.split_at(after_scheme_start);
    // Userinfo, if present, ends at the first `@` before the path/query/fragment
    // begins. Only treat an `@` as a userinfo separator when it precedes the
    // authority's terminators.
    let authority_end = rest.find(['/', '?', '#']).unwrap_or(rest.len());
    let authority = &rest[..authority_end];
    let Some(at) = authority.rfind('@') else {
        return url.to_string();
    };
    format!("{scheme}{}", &rest[at + 1..])
}

fn expand_env(spec: &str) -> String {
    // Support `VAR` and `VAR:-default` (the latter mirrors POSIX shell
    // parameter expansion, applying the default when the variable is unset or
    // empty).
    if let Some((var_name, default)) = spec.split_once(":-") {
        match std::env::var(var_name) {
            Ok(value) if !value.is_empty() => value,
            _ => default.to_string(),
        }
    } else {
        std::env::var(spec).unwrap_or_default()
    }
}

/// Run a fixed git subcommand in `working_dir`, returning trimmed stdout.
///
/// `args` is always a hardcoded command; user-supplied text never reaches the
/// command line, so there is no shell-injection surface. Returns an empty
/// string on any failure (non-zero exit, spawn error, timeout) after logging.
async fn run_git(working_dir: &Path, args: &[&str]) -> String {
    let output = smol::process::Command::new("git")
        .args(args)
        .current_dir(working_dir)
        .output()
        .await;

    match output {
        Ok(output) if output.status.success() => {
            String::from_utf8_lossy(&output.stdout).trim().to_string()
        }
        Ok(output) => {
            log::warn!(
                "template helper `git {}` exited with {}; expanding to empty string",
                args.join(" "),
                output.status
            );
            String::new()
        }
        Err(err) => {
            log::warn!(
                "template helper `git {}` failed to run ({err}); expanding to empty string",
                args.join(" ")
            );
            String::new()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Mutex, MutexGuard};

    /// Serializes tests that mutate process-global environment variables.
    /// Rust runs tests in parallel within one process, so concurrent
    /// `set_var`/`remove_var` calls would race; every env-touching test must
    /// hold this guard for its whole body.
    static ENV_GUARD: Mutex<()> = Mutex::new(());

    fn env_guard() -> MutexGuard<'static, ()> {
        ENV_GUARD
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
    }

    fn ctx() -> TemplateContext {
        TemplateContext::default()
    }

    #[test]
    fn passes_through_text_without_templates() {
        assert!(!contains_template("plain text"));
        assert_eq!(
            smol::block_on(expand_template("plain text", &ctx())),
            "plain text"
        );
    }

    #[test]
    fn expands_env_var() {
        let _guard = env_guard();
        // SAFETY: env mutation is serialized by `_guard` for the whole test.
        unsafe { std::env::set_var("ZED_TEMPLATE_TEST_VAR", "test_value") };
        assert_eq!(
            smol::block_on(expand_template("${env:ZED_TEMPLATE_TEST_VAR}", &ctx())),
            "test_value"
        );
        unsafe { std::env::remove_var("ZED_TEMPLATE_TEST_VAR") };
    }

    #[test]
    fn unset_env_var_expands_to_empty() {
        let _guard = env_guard();
        // SAFETY: env mutation is serialized by `_guard` for the whole test.
        unsafe { std::env::remove_var("ZED_TEMPLATE_DEFINITELY_UNSET") };
        assert_eq!(
            smol::block_on(expand_template(
                "x:${env:ZED_TEMPLATE_DEFINITELY_UNSET}",
                &ctx()
            )),
            "x:"
        );
    }

    #[test]
    fn env_var_default_used_when_unset() {
        let _guard = env_guard();
        // SAFETY: env mutation is serialized by `_guard` for the whole test.
        unsafe { std::env::remove_var("ZED_TEMPLATE_DEFINITELY_UNSET") };
        assert_eq!(
            smol::block_on(expand_template(
                "${env:ZED_TEMPLATE_DEFINITELY_UNSET:-fallback}",
                &ctx()
            )),
            "fallback"
        );
    }

    #[test]
    fn env_var_default_ignored_when_set() {
        let _guard = env_guard();
        // SAFETY: env mutation is serialized by `_guard` for the whole test.
        unsafe { std::env::set_var("ZED_TEMPLATE_SET_VAR", "actual") };
        assert_eq!(
            smol::block_on(expand_template(
                "${env:ZED_TEMPLATE_SET_VAR:-fallback}",
                &ctx()
            )),
            "actual"
        );
        unsafe { std::env::remove_var("ZED_TEMPLATE_SET_VAR") };
    }

    #[test]
    fn mixed_literal_and_template() {
        let _guard = env_guard();
        // SAFETY: env mutation is serialized by `_guard` for the whole test.
        unsafe { std::env::set_var("ZED_TEMPLATE_USER", "alice") };
        assert_eq!(
            smol::block_on(expand_template(
                "user:${env:ZED_TEMPLATE_USER},done",
                &ctx()
            )),
            "user:alice,done"
        );
        unsafe { std::env::remove_var("ZED_TEMPLATE_USER") };
    }

    #[test]
    fn unknown_namespace_expands_to_empty() {
        assert_eq!(
            smol::block_on(expand_template("[${bogus:value}]", &ctx())),
            "[]"
        );
    }

    #[test]
    fn missing_colon_expands_to_empty() {
        assert_eq!(
            smol::block_on(expand_template("[${noseparator}]", &ctx())),
            "[]"
        );
    }

    #[test]
    fn unterminated_template_left_as_literal() {
        assert_eq!(
            smol::block_on(expand_template("prefix ${env:FOO", &ctx())),
            "prefix ${env:FOO"
        );
    }

    #[test]
    fn git_helpers_empty_without_project_root() {
        // No project root -> git helpers must not run and must yield empty.
        assert_eq!(
            smol::block_on(expand_template("b:${git:branch}", &ctx())),
            "b:"
        );
        assert_eq!(
            smol::block_on(expand_template("r:${git:remote}", &ctx())),
            "r:"
        );
        assert_eq!(
            smol::block_on(expand_template("s:${git:sha}", &ctx())),
            "s:"
        );
    }

    #[test]
    fn git_branch_resolves_in_repo() {
        // Run against this repository's checkout. The branch may be empty in a
        // detached-HEAD CI checkout, so we only assert it does not panic and
        // produces a single trimmed line when present.
        let Ok(cwd) = std::env::current_dir() else {
            return;
        };
        let context = TemplateContext::new(Some(cwd));
        let branch = smol::block_on(expand_template("${git:branch}", &context));
        assert!(!branch.contains('\n'));
    }

    #[test]
    fn sanitize_remote_url_strips_credentials() {
        // user:password and bare-token userinfo are removed.
        assert_eq!(
            sanitize_remote_url("https://user:ghp_secret@github.com/org/repo.git"),
            "https://github.com/org/repo.git"
        );
        assert_eq!(
            sanitize_remote_url("https://ghp_secret@github.com/org/repo.git"),
            "https://github.com/org/repo.git"
        );
        // Other schemes are handled too.
        assert_eq!(
            sanitize_remote_url("ssh://git:secret@example.com:22/org/repo.git"),
            "ssh://example.com:22/org/repo.git"
        );

        // No credentials -> unchanged.
        assert_eq!(
            sanitize_remote_url("https://github.com/org/repo.git"),
            "https://github.com/org/repo.git"
        );
        // SCP-style SSH has no `://`; the `git@` username is not a secret and is
        // left intact.
        assert_eq!(
            sanitize_remote_url("git@github.com:org/repo.git"),
            "git@github.com:org/repo.git"
        );
        // An `@` that appears only in the path must not be treated as userinfo.
        assert_eq!(
            sanitize_remote_url("https://github.com/org/repo@v1.git"),
            "https://github.com/org/repo@v1.git"
        );
    }
}
