# Staged Docs Releases Plan

## Background & Current State

| Component | Status | Location |
|---|---|---|
| Docs build (CI check) | **xtask-generated** (in `run_tests.rs`) | `check_docs()` at L514–560 |
| Docs deploy workflow | **Hand-written YAML** | `.github/workflows/deploy_cloudflare.yml` |
| Composite build action | **Hand-written YAML** | `.github/actions/build_docs/action.yml` |
| Docs-proxy worker | **Runtime JS** | `.cloudflare/docs-proxy/src/worker.js` |
| book.toml | **Config** | `docs/book.toml` (site-url = `/docs/`) |

There's already a TODO in the codebase acknowledging the duplication:

```
// todo(ci): un-inline build_docs/action.yml here
```
(`tooling/xtask/src/tasks/workflows/run_tests.rs` L549)

---

## Step 1: Move Docs Build & Deploy into xtask Workflow Generation

### 1.1 Create `deploy_docs.rs` with shared build helpers and deploy workflow

Create `tooling/xtask/src/tasks/workflows/deploy_docs.rs`. This module owns all docs build and deploy logic and exports shared helpers used by both the new deploy workflow and `check_docs()` in `run_tests.rs`.

**Shared helpers** — export as `pub(crate)`:

```rust
pub(crate) fn lychee_link_check(dir: &str) -> Step<Use> { ... }
pub(crate) fn install_mdbook() -> Step<Use> { ... }
pub(crate) fn build_docs_book() -> Step<Run> { ... }
```

Copy each implementation verbatim from the inline private functions currently inside `check_docs()` in `run_tests.rs`.

**Build job** — mirror `.github/actions/build_docs/action.yml` step-for-step, using the patterns from `check_docs()` in `run_tests.rs` as the reference for how shared steps are expressed in xtask. Steps in order:

1. `steps::checkout_repo()`
2. A step that runs `cp ./.cargo/collab-config.toml ./.cargo/config.toml` — the deploy workflow uses `collab-config.toml` (not `ci-config.toml`), so this is distinct from `steps::setup_cargo_config`
3. `steps::cache_rust_dependencies_namespace()`
4. `steps::install_linux_dependencies` (covers `./script/linux`, `./script/install-mold`, `./script/download-wasi-sdk`)
5. `steps::script("./script/generate-action-metadata")`
6. `lychee_link_check("./docs/src/**/*")` — check Markdown links
7. `install_mdbook()`
8. `build_docs_book()`
9. `lychee_link_check("target/deploy/docs")` — check links in generated HTML

Set `DOCS_AMPLITUDE_API_KEY` on the build job (currently passed via `env:` to the composite action in `deploy_cloudflare.yml`).

### 1.1.1
**Deploy job** — copy each step verbatim from the `deploy-docs` job in `deploy_cloudflare.yml`. Steps in order:

1. Wrangler: `pages deploy target/deploy --project-name=docs`
2. Wrangler: `r2 object put -f script/install.sh zed-open-source-website-assets/install.sh`
3. Wrangler: `deploy .cloudflare/docs-proxy/src/worker.js`
4. Upload Wrangler logs artifact (`always()`, path `~/.config/.wrangler/logs/`, name `wrangler_logs`)

Note: steps "Deploy Docs Workers" and "Deploy Install Workers" in `deploy_cloudflare.yml` run the identical `wrangler deploy .cloudflare/docs-proxy/src/worker.js` command — this is a copy-paste bug. Include it only once (step 3 above).

Each Wrangler step uses `cloudflare/wrangler-action@da0e0dfe58b7a431659754fdf3f186c529afbe65` with `apiToken` and `accountId` from `vars::CLOUDFLARE_API_TOKEN` and `vars::CLOUDFLARE_ACCOUNT_ID`.

**Update `check_docs()`** — once the module exists, replace the inline private `lychee_link_check`, `install_mdbook`, and `build_docs` functions in `run_tests.rs` with calls to `deploy_docs::lychee_link_check`, `deploy_docs::install_mdbook`, and `deploy_docs::build_docs_book`. This resolves the TODO at L549.

### 1.2 Register the new workflow

In `tooling/xtask/src/tasks/workflows.rs`:
- Add `mod deploy_docs;`
- Add `WorkflowFile::zed(deploy_docs::deploy_docs)` to the workflows array

### 1.3 Add Cloudflare secrets to `vars.rs`

Add typed secret references:

```rust
secret!(CLOUDFLARE_API_TOKEN);
secret!(CLOUDFLARE_ACCOUNT_ID);
secret!(DOCS_AMPLITUDE_API_KEY);
```

### 1.4 Verify generated output matches the hand-written workflow

- Run `cargo xtask workflows` to generate `.github/workflows/deploy_docs.yml`
- Diff the generated file against `.github/workflows/deploy_cloudflare.yml` and `.github/actions/build_docs/action.yml` to confirm all steps, env vars, secrets, triggers, and job dependencies are equivalent
- Pay attention to: step ordering, `if:` conditions, artifact names, the Wrangler action hash, and that the duplicate worker deploy step is now deduplicated

### 1.5 Ask the user to approve the generated files

**Stop and show the generated `.github/workflows/deploy_docs.yml` to the user.** Ask them to confirm the output looks correct before proceeding to delete the hand-written files. Do not continue to step 1.6 without explicit approval.

### 1.6 Delete hand-written files

- Delete `.github/workflows/deploy_cloudflare.yml` (replaced by the generated workflow)
- Delete `.github/actions/build_docs/action.yml` (steps are now inlined in xtask)

### 1.7 Final check

The CI self-check (`check_xtask_workflows`) will enforce that the generated file stays in sync going forward.

---

## Step 2: Split Deployments into Nightly, Preview, and Stable

### 2.1 Introduce `DocsChannel` and parameterize the build

Add a `DocsChannel` enum to `deploy_docs.rs`:

```rust
pub(crate) enum DocsChannel {
    Nightly,  // site-url = "/docs/nightly/", project = "docs-nightly"
    Preview,  // site-url = "/docs/preview/", project = "docs-preview"
    Stable,   // site-url = "/docs/",          project = "docs"
}
```

mdBook supports overriding `book.toml` values at build time via `MDBOOK_`-prefixed environment variables, using `__` for TOML key nesting. The `site-url` field lives under `[book]`, so setting `MDBOOK_BOOK__SITE_URL=/docs/nightly/` before `mdbook build` overrides it without touching the file. The `book.toml` in the repository remains unchanged at `site-url = "/docs/"`.

The `build_docs_book()` step stays as-is; the channel env var is applied to the build job:

```rust
fn build_job(channel: DocsChannel, deps: &[&NamedJob]) -> NamedJob {
    // ...
    .add_env(("MDBOOK_BOOK__SITE_URL", channel.site_url()))
}
```

The deploy step uses `--project-name` matching the channel:

```rust
fn pages_deploy_step(channel: &DocsChannel) -> Step<Use> {
    // wrangler: pages deploy target/deploy --project-name=<channel.project_name()>
}
```

Export `pub(crate) fn deploy_docs_job(channel: DocsChannel, deps: &[&NamedJob]) -> NamedJob` for use in `release.rs`.

### 2.2 Create `deploy_docs_nightly.rs`

Create `tooling/xtask/src/tasks/workflows/deploy_docs_nightly.rs`. This is the standalone workflow for nightly docs, triggered on push to `main`. It calls `deploy_docs::deploy_docs_job(DocsChannel::Nightly, &[])`. The deploy job includes the `install.sh` R2 upload and docs-proxy worker deploy (these are `main`-push operations, currently bundled in `deploy_cloudflare.yml`).

Register it in `workflows.rs`: `WorkflowFile::zed(deploy_docs_nightly::deploy_docs_nightly)`.

### 2.3 Add preview and stable deploy jobs to `release.rs`

In `release.rs`, call `deploy_docs::deploy_docs_job` twice after `validate_release_assets` completes:

```rust
let deploy_docs_preview = deploy_docs::deploy_docs_job(
    DocsChannel::Preview,
    &[&validate_release_assets],
);
let deploy_docs_stable = deploy_docs::deploy_docs_job(
    DocsChannel::Stable,
    &[&validate_release_assets],
);
```

Apply an `if:` condition to each job:
- Preview: `startsWith(github.ref, 'refs/tags/v') && contains(github.ref, '-pre')`
- Stable: `startsWith(github.ref, 'refs/tags/v') && !contains(github.ref, '-pre')`

Add both jobs to the release workflow with `.add_job(...)`.

### 2.4 Update the docs-proxy worker

Modify `.cloudflare/docs-proxy/src/worker.js` to route all three channels:

```javascript
export default {
  async fetch(request, _env, _ctx) {
    const url = new URL(request.url);

    let hostname;
    if (url.pathname.startsWith("/docs/nightly")) {
      hostname = "docs-nightly.pages.dev";
    } else if (url.pathname.startsWith("/docs/preview")) {
      hostname = "docs-preview.pages.dev";
    } else {
      hostname = "docs-anw.pages.dev";
    }

    url.hostname = hostname;
    let res = await fetch(url, request);

    if (res.status === 404) {
      res = await fetch("https://zed.dev/404");
    }

    return res;
  },
};
```

The `docs-nightly` and `docs-preview` Pages project hostnames will be auto-assigned by Cloudflare on first deploy — verify the actual `*.pages.dev` hostnames and update accordingly.

**Important:** Confirm no existing stable docs pages have a path starting with `nightly` or `preview` — grep `docs/src/SUMMARY.md` to verify.

### 2.5 Wire install.sh and worker deploys

- **install.sh R2 upload** → stays in the nightly workflow (runs on push to `main`, matching current behavior)
- **docs-proxy worker deploy** → stays in the nightly workflow (worker routes all three channels, deploying once on `main` push is sufficient)

---

### 2.6 Add `noindex` meta tag for nightly and preview

Also set a `DOCS_CHANNEL` env var (`nightly`, `preview`, or `stable`) on the build job alongside `MDBOOK_BOOK__SITE_URL` in step 2.1. Add a `channel_name() -> &'static str` method to `DocsChannel` returning the appropriate string.

In `docs/theme/index.hbs`, add a `#noindex#` placeholder in `<head>` directly after the existing `{{#if is_print}}` noindex block:

```html
#noindex#
```

In `crates/docs_preprocessor/src/main.rs`'s `handle_postprocessing()`, read `DOCS_CHANNEL` and replace the placeholder — following the same pattern as the existing `#amplitude_key#` and `#description#` replacements:

- `nightly` or `preview`: replace `#noindex#` with `<meta name="robots" content="noindex, nofollow">`
- anything else: replace `#noindex#` with an empty string
