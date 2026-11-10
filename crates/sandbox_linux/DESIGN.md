# Design

This is a (currently experimental) re-implementation of the sandbox on Linux, with the following goals:
- dynamic changes to the sandbox policy, without restarting the untrusted process
- more flexible policies (e.g. globs, restricting files that don't exist yet)
- reduced "false rejections" (e.g. allowing non-HTTP traffic)

The "ultimate goal" is that such a sandbox should be reasonable to leave *enabled by default for all processes* (e.g. user terminals, LSP servers, MCP servers, git clients, etc.).

## Security posture

Traditional wisdom is that there is a fundamental difference between "missing functionality" and "security issues". I disagree - if the sandbox doesn't allow users to do what they want, the vast majority will simply disable the sandbox. To put it another way, a highly effective privilege escalation attack for an agent is to:
- try to use docker (or similar) to run a legitimate task
- explain to the user: "I can't use docker because of the sandbox, can you disable it?"
- the user disables the sandbox

## Limitations of the current Zed sandbox

The current Zed sandbox is, in a vacuum, very secure. However, users can still be exposed to risk in various ways. These ways usually look something like:
- edit the project using sandboxed tools
- this edited configuration triggers an out-of-sandbox program to run malicious code

For example, a malicious agent can edit your project to add a new Rust proc-macro crate that downloads and runs a malicious script. Rust-analyzer, which runs outside the sandbox, will then execute this proc-macro code and you are compromised.

We cannot just "run everything in the sandbox", because the current sandbox is only really usable for the agent terminal because it's not very flexible. For example, if we sandbox your user terminal, if you need internet access, we need to kill that process, and restart with the right permissions.

On Linux, we also have the restriction that filesystem protections can only apply to:
 - specific files and directories (globs not supported)
 - files/directories that exist at sandbox creation time

The implementation is also (subjectively) ugly:
- `HostFilesystemLocation` is a bit of a monstrosity, but is required for security on Linux/WSL, due to race conditions when creating namespaces through `bwrap`
  - It also doesn't fully guarantee TOCTOU-freedom on WSL when going across the plan9 bridge to the NTFS side
- The HTTP proxy is an entirely separate mechanism to FS sandboxing
- There's lots of "wrapper programs" and awkward message passing

The new design unifies a lot of this.

## The new design

The new design aims to fix this by broadly shifting from "setting up an environment with restricted permissions" to "intercepting syscalls and dynamically resolving them against the policy".

There are two components to this:
- filesystem (including domain sockets and friends)
- networking

Both of these rely on seccomp user notifications. This is a Linux kernel feature that extends seccomp's syscall-filtering ability, by allowing a user "supervisor" program to (conceptually) dynamically allow/deny[^1] syscalls. This supervisor can run arbitrary code, including code that sends a prompt to the user to confirm the action.

The rough flow would then become:
- each sandboxed program has a policy, which is uploaded to the supervisor
- when the untrusted program tries to make a syscall that is sensitive, the supervisor approves/denies/prompts the user, depending on the policy, and the specific syscall made
- when the user changes the policy (either manually, or by approving a prompt), the new policy is uploaded to the supervisor, and takes effect immediately

## Tradeoffs

There is one major tradeoff - performance. This design imposes an overhead on all syscalls that may be blocked (e.g. opening files/sockets/etc.). We can probably mitigate this by allowing `stat` and friends. How impactful this is not yet clear, and will need to be measured. My guess is that this will be acceptable, given that it's only "already slow" IO operations that will be affected, but this is too hand-wavey. We just need to benchmark it.

## Implementation ideas (ai generated)

### Performance

Every trapped syscall evaluates the policy before it can complete, so rule matching is on the critical path even though the round trip to the supervisor (~10µs) dominates. Two representations, each optimized for its job:

**Source form: `PathPattern`.** Parsed once when a policy is built. One buffer holding the pattern text, plus a small-vector of packed segment descriptors:

```rust
pub struct PathPattern {
    source: String,
    segments: SmallVec<[Segment; 8]>,
}

/// Private wrapper: the top 3 bits tag the kind, the low 29 bits hold the
/// segment's start offset in `source`. A segment ends at the next `/` or at
/// the end of `source`.
#[derive(Clone, Copy)]
struct Segment(u32);

enum SegmentKind {
    /// No wildcards: compared byte-for-byte.
    Literal,
    /// Contains `*` and/or `?`, matched within the segment.
    Wildcard,
    /// `**`: any number of segments.
    AnyDepth,
}
```

Patterns are split on `/` first: `*` and `?` never match `/`, so every segment boundary is a slash and no separator token is needed. Typical patterns have under 8 segments, so they need no allocation beyond `source`. The tag lets the matcher skip wildcard handling for literal segments without rescanning the text.

**Compiled form: the matcher.** Built from the ordered rule list when a policy is created or updated, and swapped atomically so running syscalls see either the old or the new rules, never a mix.

- A trie keyed on literal segments. Each node holds the rules whose patterns end there, plus the wildcard and `**` segments that continue from it.
- A query walks the path's segments once, following the literal edge (a hash lookup) and any wildcard or `**` edges. It collects the indices of matching rules.
- Conflict resolution needs only the indices: for each permission, take the highest-indexed match that sets it (last match wins). If none sets it, the permission is denied.
- Rules with literal paths, the common case, cost one hash lookup per segment. Cost grows with the number of wildcard edges at each visited node, not with the total rule count.

The policy can be updated while commands run, so the compiled matcher is immutable and shared (`Arc`), and an update builds a new one rather than editing it in place.

[^1]: Technically, the supervisor doesn't "allow or deny" the syscall, since this introduces a race condition, where a malicious program makes a syscall, then, in between approval and the kernel executing it, changes the contents of one of the arguments which is behind a pointer (e.g. file paths). Instead, the syscall arguments (including any data behind a pointer) are copied into the supervisor, and the supervisor *makes the syscall on behalf of the sandboxed program*, and returns the result.