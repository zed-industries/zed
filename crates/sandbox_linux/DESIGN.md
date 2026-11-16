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

### Future improvements

#### Synthetic IPv6 addresses for domains

The sandbox's DNS only hands out synthetic IPv4 addresses (from `198.18.0.0/15`). A connection to a domain has two legs, joined by the supervisor's relay, which copies bytes rather than translating packets:

1. app to relay, inside the sandbox, using the synthetic address;
2. supervisor to the real server, on the host network, using whatever the name resolves to there (IPv6 included).

So leg 1 is always IPv4, while the real server can still be IPv6-only. Queries for IPv6 addresses (`AAAA`) get an empty answer (`NOERROR`, no records) rather than `NXDOMAIN`, so resolvers fall back to the IPv4 answer cleanly.

Raw IPv6 addresses (IPv6 grants, `curl http://[2606:4700::1]`) still work: the relay listens at the exact address the app connects to, with `IP_FREEBIND` set on the socket. Without it, IPv6 `bind()` only accepts addresses assigned to an interface and ignores the namespace's local routes, unlike IPv4.

What this gives up, all uncommon:
- explicitly IPv6-only invocations (`curl -6`, `ssh -6`) get no address for a domain;
- tools in IPv6-only environments that query only `AAAA` records;
- agents debugging IPv6 itself get a confusing result.

If one of these matters, add a synthetic IPv6 range (a ULA `/64`) alongside the IPv4 one, using the same `IP_FREEBIND` relay path.

#### Revoking network grants

Narrowing the policy currently leaves existing TCP connections and UDP flows alive, just as already-open files keep working. If revoking a grant should cut access off, `update_policy` could report the connections and flows the new policy wouldn't allow (like `still_open` does for files) and offer to close them.

#### Relaying DNS to real servers

Port 53 of every address in the sandbox is answered by the supervisor's DNS. That's how queries reach it whatever server `resolv.conf` names, but it also means `dig @8.8.8.8` gets our answers: it's indistinguishable from the system resolver asking its configured server. Relaying instead has costs: real answers make later connections raw-IP requests, so domain rules no longer apply, and an allowed resolver is a channel for exfiltrating data in queries. A middle ground: relay port 53 only when an `Ip` rule names that exact address and port, and answer everything else ourselves (including under `Any` or `LocalNetwork` rules).

#### Async supervisor

Every relayed TCP connection takes two threads, and every UDP flow and DNS query one, so a busy `npm install` can mean hundreds of threads. An event loop in the supervisor (`smol`, which Zed already uses: `async-io` plus a small executor) would make these tasks instead. Seccomp notifications would stay on the worker pool, since prompts block. Measure first.

#### Exact errors for non-blocking connects

A `connect` on a non-blocking socket succeeds as soon as it's allowed, and the host connection is made in the background: holding the syscall instead would freeze the app's event loop for a round trip per connection. A host that refuses or can't be reached then shows up as a reset after a successful connect (`curl: (56) Recv failure: Connection reset by peer` instead of `Connection refused`). Most clients use non-blocking sockets (curl, Go, Node, Python sockets with a timeout), so this is the common case. A middle ground: wait briefly for the host connect (a few milliseconds, enough for local and fast failures) before returning early.

#### Sandbox differences skill

#### Datagram unix sockets

`socket` and `socketpair` refuse `SOCK_DGRAM` unix sockets: one could send to any socket by path with `sendmsg`, whose address is behind a pointer, and trapping every `sendmsg` would slow all UDP. This breaks `sd_notify`, syslog clients and datagram pairs used for IPC. If those matter, trap `sendmsg`/`sendmmsg` (continuing for IP sockets) and measure the cost.

#### Unix socket peer credentials

The supervisor makes brokered `connect` calls on its copy of the app's socket, so a server's `SO_PEERCRED` reports the supervisor's pid (the uid is the same). Servers that authorize by pid would see the wrong process.

[^1]: Technically, the supervisor doesn't "allow or deny" the syscall, since this introduces a race condition, where a malicious program makes a syscall, then, in between approval and the kernel executing it, changes the contents of one of the arguments which is behind a pointer (e.g. file paths). Instead, the syscall arguments (including any data behind a pointer) are copied into the supervisor, and the supervisor *makes the syscall on behalf of the sandboxed program*, and returns the result.