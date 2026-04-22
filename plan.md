# Property test plan for git graph commit data loading

## Goal

Add randomized state-machine tests around `git_store` commit-data loading so we can validate handler lifecycle, pending request bookkeeping, and remote/host consistency.

## Test style

Use randomized state-machine / operation-sequence tests instead of generating arbitrary maps directly.

That keeps the tested states reachable and lets us assert invariants after every step.

## Operations to randomize

Start with a small operation set:

- Fetch commit data without a waiter
- Fetch commit data with a waiter
- Successfully enqueue a request
- Fail to enqueue a request
- Deliver a commit-data result
- Close the handler
- Reopen the handler
- For remote cases, deliver host-side loaded data to the remote client

## Core invariants

### Open-handler invariants

When the handler is `Open`:

- For all `sha` where `commit_data[sha] == Loading(_)`, `pending_requests.contains(sha)` must be true.
- For all `sha` where `commit_data[sha] == Loading(Some(_))`, `completers.contains_key(sha)` must be true.
- For all `sha` in `pending_requests`, `commit_data[sha]` must exist and be `Loading(_)`.
- For all `sha` in `completers`, `commit_data[sha]` must exist and be `Loading(Some(_))`.
- `completers.keys()` must be a subset of `pending_requests`.
- For all `sha` where `commit_data[sha] == Loading(None)`, `completers.contains_key(sha)` must be false.
- For all `sha` where `commit_data[sha] == Loaded(_)`, `pending_requests.contains(sha)` must be false.
- For all `sha` where `commit_data[sha] == Loaded(_)`, `completers.contains_key(sha)` must be false.

### Closed-handler invariants

When the handler is `Closed`:

- `commit_data` must contain no `Loading(_)` entries.
- No pending request bookkeeping should survive the close transition.

## Transition / postcondition checks

### Result delivery

If a result is delivered for `sha` while the handler is `Open`, afterwards:

- `commit_data[sha] == Loaded(_)`
- `pending_requests.contains(sha)` is false
- `completers.contains_key(sha)` is false

### Successful enqueue

After a successful enqueue of `sha`:

- `commit_data[sha]` exists and is `Loading(_)`
- `pending_requests.contains(sha)` is true
- if the request was waiter-backed, `commit_data[sha] == Loading(Some(_))`
- if the request was waiter-backed, `completers.contains_key(sha)` is true

### Handler close

Right after a handler close:

- any `sha` that was still pending has been removed from `commit_data`
- no `Loading(_)` entries remain in `commit_data`

## Remote / host consistency property

For all loaded commit-data entries in a remote client, the host must also have those same entries as loaded.

More concretely:

- if the remote side has `commit_data[sha] == Loaded(data)`
- then the host side must also have `commit_data[sha] == Loaded(host_data)`
- and the loaded host entry must correspond to the same `sha`

If we want to strengthen this later, we can also assert that the loaded payload fields match exactly, not just that both sides are loaded for the same `sha`.

## Possible future property

Once enqueue-failure semantics are finalized, add a property around waiter-backed requests:

- calling `fetch_commit_data(..., needs_waiter = true, ...)` should never leave the system in a state where that `sha` is `Loading(None)`

This one depends on the final failure / retry policy, so it can wait until that behavior is settled.
