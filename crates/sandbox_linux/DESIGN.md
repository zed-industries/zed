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

### Implementation

I will refer to three processes:
- the owner process - the process that sets up the sandbox (i.e. Zed)
- the sandbox process - the process that the untrusted command runs in
  - technically this is actually two processes, an in-sandbox helper that then spawns the untrusted command
- the supervisor process - the process that is registered with the kernel to validate syscalls made by the sandbox process

There is a `SandboxPolicy` object which contains a list of rules. For example, for filesystem sandboxing a rule looks like:
```rust
struct FsRule {
  path: PathPattern,
  read: Option<Access>,    // read a file at this path
  write: Option<Access>,   // write a file at this path
  connect: Option<Access>, // connect to a socket at this path
}

enum Access {
  Allow,
  Deny,
  Ask,
}
```

The owner process sets the policy, and a more cache-friendly representation gets sent to the supervisor. When the sandbox process tries to make a syscall that is not explicitly allowed to run untrapped, the arguments get sent to the supervisor.

The supervisor makes the call on behalf of the sandboxed program, and copies arguments into its own memory to close a TOCTOU where the sandboxed process can pass a pointer to the supervisor, get the data validated, then change the data behind that pointer before the syscall is actually executed.

### Network-specific considerations

The same principle applies to network sandboxing, with some extra machinery.

In particular, the kernel deals only with IP addresses, but we would like to be able to grant permission to access domain names such as `google.com`. We do this by (loosely):
- configuring the namespace such that DNS queries go to a resolver running in the supervisor process
- when the sandboxed process makes a DNS request for `google.com`, we return a synthetic IP address
- the supervisor records the mapping between `google.com` and the synthetic IP address
- when the sandboxed process tries to connect to that synthetic IP address, the supervisor can convert it back to `google.com`, and look at the policy to understand how to react (allow, deny, ask)
  - the real DNS lookup also goes through a rebinding check, so I can't grant access to `attacker_controlled_domain.com` and have that grant access to `127.0.0.1` for example
  - a rule for a host the user knows is on their network (`home.lan`) can opt out of the check for local-network addresses with `NetRule::reaching_local_network`; loopback addresses still need a rule of their own

There is then a relay, that maps these synthetic IP addresses outside the sandbox. This lives in the supervisor. It creates real connections to the actual IP addresses for the domains (using the system DNS resolver). So when it succeeds, there are two legs:
- sandbox <-> supervisor
- supervisor <-> `google.com`

The relay just plainly copies bytes from one to the other.

### `Ask` mode

When a permission is in `Ask` mode, it blocks the syscall indefinitely(ish) until the owner process responds. The `Handler` type in a `Ask` variant is created by providing a callback that takes a `Responder`. This type is how the custom user code can indicate to the supervisor what the result was. It *must* do this using the dedicated methods. Dropping it is considered an error, which:
- panics in debug mode
- fails closed and warns in release mode

When it responds, by default it responds *only for the current syscall*. Unlike the old sandbox implementation, there's no obvious way to say "allow this syscall again in the future", because it often refers to ephemeral state (e.g. file descriptors). For example, if you allow writes to `/foo/bar/baz.rs`, and you want that approval to persist for the rest of the sandbox, should that apply to:
- only `/foo/bar/baz.rs`
- `/foo/bar`
- `/foo`
- `/`

The `linux_sandbox` crate doesn't address this issue, but provides the `Decision` type, which is what a `Responder` actually responds with. It contains a decision for this syscall, but also a list of rules to remember for the rest of this session. It is up zed to determine what options to show to the user.

It's almost always wrong to not supply any extra rules to remember.

## Tradeoffs

The one major tradeoff is performance: this design imposes a round trip to the supervisor on every syscall that may be blocked (opening files and sockets, writing, connecting). On a 9950x3d, a trapped `open` costs ~10 microseconds compared to ~0.4 microseconds untrapped. Relaying a 1 gigabit stream cost about 2.5% of a CPU core.. `stat` and friends, directory listing, and `execve` are not restricted, so don't get the overhead.

## Implementation ideas (ai generated)

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

Every relayed TCP connection takes two threads, and every UDP flow and DNS query one, so a busy `npm install` can mean hundreds of threads. An event loop in the supervisor (`smol`, which Zed already uses: `async-io` plus a small executor) would make these tasks instead; seccomp notifications would stay with their leader/follower receivers, since prompts block.

Measured (9950X3D; benchmark harness in the benchmarking thread): at 1 Gbit/s every relay model costs 2.5–3% of a core, so throughput alone doesn't motivate this. For bursts of short connections, async halves the relay's CPU per exchange (21 vs 48 µs) and replaces 626 threads at 1000 concurrent connections with 3. Not urgent; revisit if supervisor resource usage becomes a problem.

#### Exact errors for non-blocking connects

A `connect` on a non-blocking socket succeeds as soon as it's allowed, and the host connection is made in the background: holding the syscall instead would freeze the app's event loop for a round trip per connection. A host that refuses or can't be reached then shows up as a reset after a successful connect (`curl: (56) Recv failure: Connection reset by peer` instead of `Connection refused`). Most clients use non-blocking sockets (curl, Go, Node, Python sockets with a timeout), so this is the common case. A middle ground: wait briefly for the host connect (a few milliseconds, enough for local and fast failures) before returning early.

#### Sandbox differences skill

In-sandbox behaviour is subtly different from unsandboxed Linux (exact error codes on denied operations, resets instead of refusals after non-blocking connects, `getsockname` paths, `SO_PEERCRED` pids). Collect the differences into a documented "skill" that agents can be given, so tools don't misread sandbox artefacts as environment failures.

#### Datagram unix sockets

`socket` and `socketpair` refuse `SOCK_DGRAM` unix sockets: one could send to any socket by path with `sendmsg`, whose address is behind a pointer, and seccomp can't read the address to notice. The same trap would also let unconnected UDP `sendto` reach ports no rule names: today those datagrams go nowhere, because a relay socket only exists for ports the policy names. This breaks `sd_notify`, syslog clients and datagram pairs used for IPC.

Measured (9950X3D; benchmark harness in the benchmarking thread): trapping every `send*` costs +10.8 µs of CPU per send with the current notification path — the supervisor saturates a core at 1 Gbit/s of QUIC-sized datagrams. An optimized path (answer on the receiving thread, `SYNC_WAKE_UP`, cached pidfds) brings it to ~+2.2 µs per send: +28% of a core at 1 Gbit/s, and +1% or less with GSO or `sendmmsg` batches of 16. Whether that budget is acceptable is a product call.

#### Unix socket peer credentials

The supervisor makes brokered `connect` calls on its copy of the app's socket, so a server's `SO_PEERCRED` reports the supervisor's pid (the uid is the same). Servers that authorize by pid would see the wrong process.

[^1]: Technically, the supervisor doesn't "allow or deny" the syscall, since this introduces a race condition, where a malicious program makes a syscall, then, in between approval and the kernel executing it, changes the contents of one of the arguments which is behind a pointer (e.g. file paths). Instead, the syscall arguments (including any data behind a pointer) are copied into the supervisor, and the supervisor *makes the syscall on behalf of the sandboxed program*, and returns the result.