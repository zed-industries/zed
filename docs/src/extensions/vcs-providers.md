---
title: External VCS Providers
description: Experimental read-only version control providers
---

# External VCS Providers

This experimental API lets an extension supply repository discovery, changed files,
and comparison contents to Zed's native source-control UI. Git keeps its existing
backend. A provider does not emulate Git or create `.git` metadata.

The initial API is local and read-only. Stage, restore, commit, branch changes,
checkout creation/removal, agent archive, and remote transport are not provided.
Read-only capabilities constrain Zed's calls; they are not an operating-system
sandbox for the provider executable. Zed requires workspace trust before invoking
the extension callback or starting its process.

## Extension registration

An extension using development API 0.9 declares providers in its manifest:

```toml
[vcs_providers.example]
root_markers = [".example-repository"]
```

Markers are filenames, checked in the opened directory and its ancestors without
recursively scanning those ancestors. They identify candidates; the provider must
confirm repository ownership. An existing nested Git repository retains ownership
of its files. Ambiguous external ownership must be reported, not resolved by
registration order.

Implement `Extension::vcs_provider_command` to return the executable, arguments, and
environment. This callback follows the language-server command pattern, but the
returned process speaks the VCS protocol below. The native host owns its pipes,
requests, cancellation, and lifetime. `process::Command::output` is not a streaming
transport and must not run the provider loop from inside WASM.

## Wire protocol 1

Messages use JSON-RPC 2.0 with `Content-Length: <UTF-8 byte count>\r\n\r\n`
framing. Headers are bounded to 8 KiB and each body to 16 MiB. Diagnostics belong
on stderr; stdout contains protocol frames only. Clients generate integer request
IDs. Providers send responses and notifications, but cannot request client actions.

All model fields use camelCase. Repository, revision, snapshot and baseline IDs are
opaque strings; an identifier such as `r42` is valid. Paths use slash-separated
components relative to `scopeUri`, not the repository root. Absolute paths, empty
components, `.`/`..`, NUL, backslashes and colon-containing components are rejected
by the initial portable-path contract. Report an unsupported path rather than
silently omitting it. File URIs identify local paths only.

| Method | Parameters | Result |
| --- | --- | --- |
| `initialize` | `protocolVersion`, `clientInfo: {name, version}` | `protocolVersion`, `serverInfo`, `capabilities: {readOnly: true}` |
| `vcs/discover` | `scopeUri` | Array of repository descriptors |
| `vcs/snapshot` | `repositoryId`, `baselineId` | A snapshot of changes for that comparison |
| `vcs/readContent` | `repositoryId`, `snapshotId`, `baselineId`, `path` | Baseline-side content |
| `shutdown` | Empty object | `null`, after provider cleanup |

After shutdown, the client sends an `exit` notification. Dropping a request sends
`$/cancelRequest` with its `id`. Shutdown and process exit have bounded deadlines;
the host terminates and reaps its owned process group if the provider does not
finish. Providers must keep their subprocesses in that ownership boundary and
cancel their own outstanding work on shutdown or stdin EOF.

The optional `vcs/didChange` notification has `{repositoryId}` parameters. It
invalidates the visible snapshot; Zed also refreshes after relevant file changes
and supports explicit refresh. Request failure, timeout, malformed data and process
EOF are errors, not empty successful snapshots. Error `code`, `message`, and
optional `data` retain provider details. Responses received before EOF remain valid.

### Repositories and scopes

```json
{
  "id": "example:checkout-1",
  "rootUri": "file:///workspace",
  "scopeUri": "file:///workspace/app",
  "displayName": "Example VCS",
  "baselines": [
    {"id": "working", "label": "Working changes"},
    {"id": "branch", "label": "Changes since branch base"}
  ]
}
```

The repository root must contain the requested scope. A provider must return that
scope unchanged and limit status/content queries accordingly. Scope containment in
the protocol is lexical; filesystem symlinks require a separate check before Zed
opens a working file. Multiple windows may open different scopes in one repository.

### Snapshots and contents

```json
{
  "id": "snapshot:9",
  "repositoryId": "example:checkout-1",
  "baselineId": "working",
  "revisionId": "r42",
  "changes": [
    {"path": "src/main.rs", "status": "modified", "kind": "file"}
  ]
}
```

`revisionId` identifies the immutable contents of the selected baseline, which may
differ from the current checkout revision. Providers must change it whenever any
baseline-side content changes. Rename mapping changes must be reflected in the
corresponding change entry’s `oldPath`. A fresh snapshot token with the
same baseline, revision identity and change list does not invalidate already
loaded comparison contents. Old snapshot tokens must continue resolving their
pinned contents, or return an explicit expiration error.

Statuses are `added`, `modified`, `deleted`, `renamed`, or `untracked`. A rename
includes `oldPath`. Content kinds are `file`, `binary`, `symlink`, or `submodule`.
Optional `oldMode` and `newMode` are numeric Unix mode bits, e.g. decimal `33188`
for octal `100644`. A content-kind hint may be conservative; `readContent` provides
the authoritative baseline type.

`readContent.path` identifies the current-side path. For a rename, the provider
resolves its original baseline path using the requested snapshot. It also accepts
unchanged files within the scope, so Zed can compare an unsaved buffer whose disk
file does not appear in status. The result is one of:

```json
{"kind":"text","text":"baseline contents\n"}
{"kind":"absent"}
{"kind":"binary"}
{"kind":"symlink","target":"relative-target"}
```

`absent` means the file is absent from the baseline, never that a backend read
failed. Comparison contents remain pinned to the requested snapshot. The snapshot
does not promise an atomic working filesystem: Zed owns live buffer versions and
rejects stale asynchronous results when a repository or baseline changes.

## Validation

Public conformance tests use a provider without Git metadata, an index, or Git SHA
identifiers. They cover framing, typed errors, cancellation, late responses,
response-before-EOF, stderr limits and child cleanup. Product tests additionally
cover trust, multiple scopes, nested Git ownership, unsaved buffers, native UI and
capability checks at dispatch. A successful CLI exchange alone does not establish
native UI correctness.
