---
title: Sandboxing
description: Zed Agent tool calls can run in an OS-level sandbox to restrict certain operations.
---

# Sandboxing

You can restrict what operations the [Zed Agent](./zed-agent.md) can run in multiple ways. One way to restrict them is
[Tool Permissions](./tool-permissions.md), but these are of limited use when the agent wants to do things like run a
complicated script in a terminal.

Sandboxing runs certain tool actions in an OS-level sandbox which limits filesystem access, network access, and access to protected Git metadata. This way, even if the agent wants to run an arbitrary script, that script will only be able to write to the files and folders you have allowed it to. You can similarly restrict network and Git metadata access in sandboxed tool actions.

[Tool Permissions](./tool-permissions.md) can be used in addition to sandboxing:

- Tool permissions restrict the agent's ability to run certain tool actions in the first place
- Once a tool action is actually running, sandboxing restricts what it can do

Sandboxing applies only to Zed Agent. It does not sandbox Zed itself, language servers, extensions, tasks, your normal terminal tabs, [External Agents](./external-agents.md), or [Terminal Threads](./terminal-threads.md).

## Sandboxed Tools {#sandboxed-tools}

Zed Agent sandboxing currently applies to the `terminal` tool.

| Tool       | What sandboxing limits                                                                              |
| ---------- | --------------------------------------------------------------------------------------------------- |
| `terminal` | Filesystem writes, protected Git metadata, and outbound network access for commands the agent runs. |

Other built-in tools, including `fetch`, are still governed by [Tool Permissions](./tool-permissions.md), [Agent Profiles](./agent-profiles.md), and project trust, but they are not currently run inside this OS sandbox.

## Default Access {#default-access}

By default, sandboxed Zed Agent tool actions have these restrictions:

| Access type         | Default behavior                                                                                          |
| ------------------- | --------------------------------------------------------------------------------------------------------- |
| Filesystem reads    | Terminal commands can read the filesystem, except for protected Git metadata file contents.               |
| Project writes      | Terminal commands can write inside open project directories, except for protected Git metadata.           |
| Git metadata        | `.git` directories and linked worktree Git metadata are protected unless you approve Git metadata access. |
| Temporary files     | Terminal commands receive a writable temporary location. The exact behavior differs by platform.          |
| Other writes        | Writes outside the default writable locations are blocked unless you approve a broader sandbox request.   |
| Outbound networking | Network access is blocked unless you approve a host-specific or unrestricted network sandbox request.     |

## Approval Prompts {#approval-prompts}

When the agent needs access outside the default sandbox, Zed shows a sandbox approval prompt before the tool action runs.
Depending on what the tool requested, the prompt can ask you to allow:

- network access to specific hosts, such as `github.com` or `*.npmjs.org`
- network access to any host
- access to protected Git metadata, such as `.git` directories and linked worktree metadata
- write access to specific filesystem paths
- unrestricted filesystem writes
- running a terminal command without the sandbox

You can grant a sandbox request for:

- one tool action
- the rest of the current thread
- always

Approvals for the rest of the thread are remembered only for that thread.
Approvals granted with “always” are saved in `settings.json` under `agent.sandbox_permissions`.

## Persistent Sandbox Permissions {#persistent-sandbox-permissions}

If you want to pre-approve common sandbox requests, add persistent permissions to your settings file:

```json [settings]
{
  "agent": {
    "sandbox_permissions": {
      "network_hosts": ["github.com", "*.npmjs.org"],
      "allow_git_access": true,
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
| `allow_git_access`   | Allow sandboxed terminal commands to access protected Git metadata without prompting.                             |
| `write_paths`        | Directory subtrees that sandboxed terminal commands may write to without prompting. Paths are absolute.           |
| `allow_fs_write_all` | Allow sandboxed terminal commands to write anywhere without prompting.                                            |
| `allow_unsandboxed`  | Allow terminal commands to run outside the sandbox without prompting when the agent explicitly requests it.       |

Prefer narrow grants, such as a specific host, Git metadata access, or write path, over `allow_all_hosts`, `allow_fs_write_all`, or `allow_unsandboxed`.

## Platform Support {#platform-support}

Sandboxing uses different operating system mechanisms on each platform.
The user-facing prompts are similar, but the enforcement details vary.

### macOS {#macos}

On macOS, Zed uses Apple's Seatbelt sandbox through `sandbox-exec`.

Sandboxed terminal commands:

- can read the filesystem
- can write inside open project directories, except protected Git metadata
- can write to a per-thread temporary directory exposed through `$TMPDIR`, `$TMP`, and `$TEMP`
- cannot read or write protected Git metadata unless you approve Git metadata access
- cannot write elsewhere unless you approve additional paths or broader write access
- cannot reach the network unless you approve network access

When network access is approved on macOS, Zed uses an HTTP/HTTPS proxy so access can be limited to approved hosts.
Tools that do not honor proxy environment variables, such as SSH, FTP, and raw socket clients, may not work even after host-specific network access is approved.
For networked terminal commands, prefer HTTPS URLs over SSH URLs when possible.

### Linux {#linux}

On Linux, Zed uses Bubblewrap (`bwrap`) for sandboxing.

Zed only uses a non-setuid `bwrap` binary.
Its sandbox is built entirely on unprivileged user namespaces, so a setuid-root `bwrap` provides no extra functionality, and running one would mean executing root-privileged setup with arguments partly derived from model-influenced input.
If the only `bwrap` found on your `PATH` is setuid-root, Zed refuses to run it; install a non-setuid Bubblewrap to enable sandboxing.

Sandboxed terminal commands:

- can read the filesystem
- can write inside open project directories, except protected Git metadata
- can write to `/tmp`, which is backed by a fresh temporary filesystem and is cleared between terminal tool calls
- cannot read or write protected Git metadata unless you approve Git metadata access
- cannot write elsewhere unless you approve additional paths or broader write access
- cannot reach the network unless you approve network access

Linux network sandboxing can allow or block outbound networking as a whole, but cannot enforce a per-host allowlist.
If you approve network access for one host on Linux, the sandbox must grant unrestricted outbound network access for that tool action.
Zed still asks for the narrower request when that is what the agent asked for, but the platform enforcement is all-or-nothing.

If Bubblewrap is unavailable or cannot create a sandbox in the current environment, Zed may run the command without the OS sandbox and show a warning in the tool output.

### Windows {#windows}

On Windows, Zed Agent sandboxing is supported only when the agent action runs inside WSL.

Zed uses the Linux Bubblewrap sandbox inside WSL because WSL provides the Linux process and filesystem primitives that Bubblewrap needs.
Native Windows processes do not currently have the same sandbox integration in Zed, so a native Windows command cannot be confined by Zed Agent's OS sandbox in the same way.

When running inside WSL, the Linux sandboxing behavior applies, including the requirement that `bwrap` not be setuid-root:

- filesystem isolation is provided by Bubblewrap
- protected Git metadata requires Git metadata access approval
- `/tmp` is temporary for sandboxed terminal calls
- network access is all-or-nothing rather than host-specific

If WSL is not installed, or if you choose to run a command without the sandbox, Zed falls back to the standard terminal behavior of running in your native shell.
It selects the shell using the usual preference order: Git Bash (or scoop's bash) when one is installed, otherwise PowerShell, and finally `cmd.exe`.
Because the command then runs against native Windows paths instead of WSL's Linux filesystem, path conventions change accordingly (for example `C:\...` or `/c/...` rather than WSL's `/mnt/c/...`), so a command written for the sandboxed WSL shell may behave differently.

## Choosing What to Approve {#choosing-what-to-approve}

When reviewing a sandbox prompt, prefer the narrowest permission that lets the task proceed:

- approve a specific host instead of all hosts when the destination is known
- approve Git metadata access when the command needs to run Git operations such as `git fetch`, `git commit`, or `git status`
- approve a specific write path instead of unrestricted filesystem writes
- approve unsandboxed execution only when the command cannot work inside the sandbox
- use one-time approvals for unfamiliar commands
- use thread or always approvals only for access you expect to reuse

If a command fails because the sandbox blocked access, ask the agent why it needs that access before approving a broader request.
