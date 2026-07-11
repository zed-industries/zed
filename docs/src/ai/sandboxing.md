---
title: Sandboxing
description: Zed Agent tool calls can run in an OS-level sandbox to restrict certain operations.
---

# Sandboxing

You can restrict what operations the [Zed Agent](./zed-agent.md) can run in multiple ways. One way to restrict them is
[Tool Permissions](./tool-permissions.md), but these are of limited use when the agent wants to do things like run a
complicated script in a terminal.

Sandboxing instead uses OS features to forcibly restrict which resources a tool
call has access to. This does _not_ rely on an agent following a particular set
of instructions. If the agent attempts to access a resource that is restricted
by the sandbox, the OS will block it. See [How much can I trust the
sandbox?](#trust) for more details.

[Tool Permissions](./tool-permissions.md) can be used in addition to sandboxing:

- Tool permissions restrict the agent's ability to run certain tool actions in the first place
- Once a tool action is actually running, sandboxing restricts what it can do

Sandboxing applies only to Zed Agent. It does not sandbox Zed itself, language servers, extensions, tasks, your normal
terminal tabs, [External Agents](./external-agents.md), or [Terminal Threads](./terminal-threads.md).

## Sandboxed Tools {#sandboxed-tools}

Zed Agent sandboxing currently applies to the `terminal` and `fetch` tools.

| Tool       | What sandboxing limits                                                                                |
| ---------- | ----------------------------------------------------------------------------------------------------- |
| `terminal` | Filesystem writes and outbound network access for commands the agent runs; Git metadata is protected. |
| `fetch`    | Hosts which can be accessed.                                                                          |

Tools are still governed by [Tool Permissions](./tool-permissions.md), [Agent
Profiles](./agent-profiles.md), and project trust, but they are not currently
run inside this OS sandbox.

## Requirements {#requirements}

Sandboxing is supported, in some form, on all platforms. In order to sandbox a
`terminal` tool call, the following is required:

- On Linux, a runnable, non-setuid `bwrap` binary must be on the `$PATH`. See [Installing Bubblewrap](#installing-bubblewrap).
- On Windows, WSL must be available.

There are no extra requirements on MacOS.

The `fetch` tool has no extra requirements on any platform.

## Default Access {#default-access}

By default, sandboxed Zed Agent tool actions have these restrictions:

| Access type         | Default behavior                                                                                                                                                                       |
| ------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Filesystem reads    | Terminal commands can read most of the filesystem, including protected Git metadata.                                                                                                   |
| Project writes      | Terminal commands can write inside open project directories, except for protected Git metadata.                                                                                        |
| Git metadata        | `.git` directories and linked worktree Git metadata remain readable but are not writable while sandboxed.                                                                              |
| Temporary files     | Terminal commands receive a writable temporary location. The exact behavior differs by platform.                                                                                       |
| Other writes        | Writes outside the default writable locations are blocked unless you approve a broader sandbox request.                                                                                |
| Outbound networking | Network access is blocked unless you approve a host-specific or unrestricted network sandbox request. Host-specific enforcement is not available on every platform.                    |
| Local IPC sockets   | Sandboxed commands cannot open Unix-domain sockets (for example, to the desktop session bus or a container daemon), which could otherwise be used to run commands outside the sandbox. |

## How much can I trust the sandbox? {#trust}

Enabling sandboxing dramatically reduces the risk of various kinds of attacks.
However, it does not fully eliminate them.

Firstly, sandboxing relies on OS-level features, which may contain bugs.
Operating systems have historically had bugs in security features. And while we
have tested thoroughly, there may also be bugs in Zed's implementation. These
could allow privilege escalation - for example, allow an agent to write to a
file that it should only have read access to.

Sandboxing also only applies the restrictions that the user requested. If an
agent requests write access to your home directory, sandboxing will (and
should!) do nothing to prevent an agent adding a malicious key to `$HOME/.ssh`.

Be careful with what you grant the agent. At any point in time, you can view the
state of the sandbox by hovering the padlock icon in the top right of the
thread. If an agent is requesting an overly broad permission, deny it, and ask
it to use a smaller grant. When requesting elevated privileges, agents must
provide a `reason`, which is displayed in the prompt. Read it, and decide
whether it makes sense before approving.

Also, sandboxing restricts **only** what the `terminal` and `fetch` tools in the
Zed agent can do. It has **no effect** on other parts of Zed, including:

- Language servers
- The built-in git client
- The regular terminal
- And more...

Even when sandboxing is enabled, you should remain vigilant. A malicious or
unaligned agent may use these side channels to escalate privileges. For example:

- An agent may add a malicious Rust procedural macro to your codebase, which
  will be automatically executed by `rust-analyzer` **outside the sandbox**.
- An agent may modify a `Makefile` to inject a malicious script, which is
  executed **outside the sandbox** when you next run `make` in the built-in
  terminal.
- The agent cannot write to your repository's protected `.git` directory, but it
  can create a submodule under your project whose Git metadata (including config
  such as `core.fsmonitor`) it fully controls. That metadata may then be executed
  **outside the sandbox** when you subsequently run Git commands in a regular
  terminal. Your shell prompt may even execute Git commands every time it
  renders!

There are steps you can take to mitigate these issues. For example:

- disable language servers that execute user-defined code from the project (such
  as Rust procedural macros).
- use a shell prompt that reports Git status without executing
  repository-defined programs.
- review the diff before running `git commit`

But none of this changes the fundamental principle: **a sandbox is not a
substitute for good security practices**. It is one layer in a defense-in-depth
strategy.

Zed's default profile aims to strike a balance between security and convenience,
but we encourage you to tune your settings based on your own security
requirements and risk profile. A disabled sandbox is not a very effective
sandbox.

## Approval Prompts {#approval-prompts}

When the agent needs access outside the default sandbox, Zed shows a sandbox approval prompt before the tool action runs.
Depending on what the tool requested, the prompt can ask you to allow:

- network access to specific hosts, such as `github.com` or `*.npmjs.org`
- network access to any host
- write access to specific filesystem paths
- unrestricted filesystem writes, except protected Git metadata
- running a terminal command without the sandbox

You can grant a sandbox request for:

- one tool action
- the rest of the current thread
- always

Approvals for the rest of the thread are remembered only for that thread. Approvals granted with “always” are saved in
`settings.json` under `agent.sandbox_permissions`.

## Persistent Sandbox Permissions {#persistent-sandbox-permissions}

If you want to pre-approve common sandbox requests, add persistent permissions to your settings file:

```json [settings]
{
  "agent": {
    "sandbox_permissions": {
      "network_hosts": ["github.com", "*.npmjs.org"],
      "write_paths": ["/Users/you/.cache/my-tool"]
    }
  }
}
```

The available options are:

| Setting              | Description                                                                                                       |
| -------------------- | ----------------------------------------------------------------------------------------------------------------- |
| `network_hosts`      | Hosts that sandboxed tools may reach without prompting. Entries can be exact hostnames or leading-`*.` wildcards. |
| `allow_all_hosts`    | Allow sandboxed tools to reach any host without prompting.                                                        |
| `write_paths`        | Directory subtrees that sandboxed terminal commands may write to without prompting. Paths are absolute.           |
| `allow_fs_write_all` | Allow sandboxed terminal commands to write anywhere except protected Git metadata without prompting.              |
| `allow_unsandboxed`  | Turn sandboxing off entirely for Zed Agent terminal commands. The fetch tool will have no restrictions.           |

Prefer narrow grants, such as a specific host or write path, over `allow_all_hosts`, `allow_fs_write_all`, or
`allow_unsandboxed`.

## Git Metadata {#git-metadata}

Git metadata writes are not grantable while a terminal command is sandboxed. This includes writes to `.git` directories,
linked worktree metadata, refs, the index, hooks, local Git config, and other Git-controlled metadata files. Approving a
specific writable path or `allow_fs_write_all` does not make Git metadata writable.

## Platform Support {#platform-support}

Sandboxing uses different operating system mechanisms on each platform. The user-facing prompts are similar, but the
enforcement details vary.

### macOS {#macos}

On macOS, Zed uses Apple's Seatbelt sandbox through `sandbox-exec`.

Sandboxed terminal commands:

- can read the filesystem
- can write inside open project directories, except protected Git metadata
- can write to a per-thread temporary directory exposed through `$TMPDIR`, `$TMP`, and `$TEMP`
- can read protected Git metadata
- cannot write protected Git metadata, even if you approve broader write access
- cannot write elsewhere unless you approve additional paths or broader write access
- cannot reach the network unless you approve network access
- can reach only an allowlist of macOS system (Mach) services that developer tooling needs; services that could be abused to escape the sandbox (LaunchServices and launchd, which can launch processes outside it), read the clipboard (the pasteboard), or capture audio are not reachable

When network access is approved on macOS, Zed uses an HTTP/HTTPS proxy so access can be limited to approved hosts.
Tools that do not honor proxy environment variables, such as SSH, FTP, and raw socket clients, may not work even after host-specific network access is approved.
For networked terminal commands, prefer HTTPS URLs over SSH URLs when possible.

### Linux {#linux}

On Linux, Zed uses [Bubblewrap][bubblewrap] (`bwrap`) for sandboxing.

Zed only uses a non-setuid `bwrap` binary. Its sandbox is built entirely on unprivileged user namespaces, so a setuid-root
`bwrap` provides no extra functionality, and running one would mean executing root-privileged setup with arguments partly
derived from model-influenced input. If the only `bwrap` found on your `PATH` is setuid-root, Zed refuses to run it;
install a non-setuid Bubblewrap to enable sandboxing.

Sandboxed terminal commands:

- can read the filesystem, including protected Git metadata contents
- can write inside open project directories, except protected Git metadata
- can write to `/tmp`, which is backed by a fresh temporary filesystem and is cleared between terminal tool calls (when you approve unrestricted filesystem writes, `/tmp` is instead your real host `/tmp` rather than a fresh temporary filesystem)
- cannot write protected Git metadata
- cannot write elsewhere unless you approve additional paths or broader write access
- cannot reach the network unless you approve network access

When host-specific network access is approved on Linux, Zed uses an HTTP/HTTPS proxy so access can be limited to approved
hosts. Tools that do not honor proxy environment variables, such as SSH, FTP, and raw socket clients, may not work even
after host-specific network access is approved.

If Bubblewrap is unavailable or cannot create a sandbox in the current environment, Zed may run the command without the OS
sandbox and show a warning in the tool output.

#### Installing Bubblewrap {#installing-bubblewrap}

Zed needs a runnable, non-setuid `bwrap` binary on your `$PATH`. Installing
`bubblewrap` from your distribution's package manager is usually all you need.

You can test whether it's working with:

```sh
bwrap --ro-bind / / -- echo "working"
```

"Non-setuid" here refers to the [setuid bit][setuid bit]. Historically,
bubblewrap has shipped both a setuid and non-setuid binary. The setuid binary is
being phased out for security concerns, and so Zed's sandbox _explicitly rejects
setuid `bwrap` binaries_.

##### Ubuntu-specific requirements {#installing-bubblewrap-ubuntu}

> **Note:** The following does not affect Ubuntu on WSL.

Bubblewrap relies on a Linux kernel feature known as "namespaces". Unprivileged
users on many systems can create namespaces, but historically, this feature has
been used for a variety of attacks.

In response to this, in Ubuntu 23.10, Canonical [added a security
measure][ubuntu blog] that restricts unprivileged user namespaces. These
restrictions are enforced by AppArmor.

Because of this, you may also need to install an AppArmor profile for bubblewrap
after you install it. This is a configuration file that gives bubblewrap the
ability to create namespaces without needing `sudo`.

```sh
sudo apt install bubblewrap

# On Ubuntu 25.04 and later, `apparmor` ships with a profile for bubblewrap by default.
# Make sure you're up-to-date
sudo apt install --only-upgrade apparmor

# On older versions, manually install the profile
sudo apt update
sudo apt install apparmor-profiles apparmor-utils
sudo install -m 0644 \
  /usr/share/apparmor/extra-profiles/bwrap-userns-restrict \
  /etc/apparmor.d/bwrap-userns-restrict
sudo apparmor_parser -r /etc/apparmor.d/bwrap-userns-restrict
```

### Windows {#windows}

On Windows, Zed Agent sandboxing is supported only when the agent action runs inside WSL.

Zed uses the Linux Bubblewrap sandbox inside WSL because WSL provides the Linux process and filesystem primitives that
Bubblewrap needs. Native Windows processes do not currently have the same sandbox integration in Zed, so a native Windows
command cannot be confined by Zed Agent's OS sandbox in the same way.

When running inside WSL, the Linux sandboxing behavior applies, including the requirement that `bwrap` not be setuid-root:

- filesystem isolation is provided by Bubblewrap
- protected Git metadata contents remain readable, but writes are blocked
- `/tmp` is temporary for sandboxed terminal calls
- network access is all-or-nothing rather than host-specific, so host-specific network requests are rejected and the agent must request unrestricted network access when network access is needed

If WSL is not installed, or if you choose to run a command without the sandbox, Zed falls back to the standard terminal
behavior of running in your native shell. It selects the shell using the usual preference order: a bash (scoop's bash or
Git Bash) when one is installed, otherwise PowerShell, and finally `cmd.exe`. Because the command then runs against native
Windows paths instead of WSL's Linux filesystem, path conventions change accordingly (for example `C:\...` or `/c/...`
rather than WSL's `/mnt/c/...`), so a command written for the sandboxed WSL shell may behave differently.

## Choosing What to Approve {#choosing-what-to-approve}

When reviewing a sandbox prompt, prefer the narrowest permission that lets the task proceed:

- approve a specific host instead of all hosts when the destination is known
- approve a specific write path instead of unrestricted filesystem writes
- approve unsandboxed execution only when the command cannot work inside the sandbox
- use one-time approvals for unfamiliar commands
- use thread or always approvals only for access you expect to reuse

If a command fails because the sandbox blocked access, ask the agent why it needs that access before approving a broader
request.

[bubblewrap]: https://github.com/containers/bubblewrap
[setuid bit]: https://en.wikipedia.org/wiki/Setuid
[ubuntu blog]: https://ubuntu.com/blog/ubuntu-23-10-restricted-unprivileged-user-namespaces
