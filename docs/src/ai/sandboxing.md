---
title: Sandboxing
description: Zed Agent tool calls can run in an OS-level sandbox to restrict certain operations.
---

# Sandboxing

You can restrict what operations the [Zed Agent](./zed-agent.md) can run in multiple ways. One way to restrict them is
[Tool Permissions](./tool-permissions.md), but these are of limited use when the agent wants to do things like run a
complicated script in a terminal.

Sandboxing restricts what certain tool actions can do after they start. Terminal commands run in an OS-level sandbox which limits filesystem access, network access, and access to protected Git metadata. The `fetch` tool uses the same network approval model to restrict which hosts it can reach. This way, even if the agent wants to run an arbitrary script or fetch a URL, it can only access what you have allowed.

[Tool Permissions](./tool-permissions.md) can be used in addition to sandboxing:

- Tool permissions restrict the agent's ability to run certain tool actions in the first place
- Once a tool action is actually running, sandboxing restricts what it can do

Sandboxing applies only to Zed Agent threads. It does not sandbox Zed itself, language servers, extensions, tasks, your normal terminal tabs, [External Agents](./external-agents.md), or [Terminal Threads](./terminal-threads.md).

## Sandboxed Tools {#sandboxed-tools}

Sandboxing currently applies to `terminal` and `fetch`.

When the agent runs a terminal command, the OS sandbox can restrict filesystem writes, protected Git metadata, and outbound network access. When the agent fetches a URL, sandboxing restricts which network hosts the fetch can reach.

Other built-in tools are still governed by [Tool Permissions](./tool-permissions.md), [Agent Profiles](./agent-profiles.md), and project trust; they are not run inside the OS-level sandbox.

## Default Access {#default-access}

By default, sandboxed terminal tool actions have these restrictions:

| Access type         | Default behavior                                                                                          |
| ------------------- | --------------------------------------------------------------------------------------------------------- |
| Filesystem reads    | Terminal commands can read the filesystem, except for protected Git metadata file contents.               |
| Project writes      | Terminal commands can write inside open project directories, except for protected Git metadata.           |
| Git metadata        | `.git` directories and linked worktree Git metadata are protected unless you approve Git metadata access. |
| Temporary files     | Terminal commands receive a writable temporary location. The exact behavior differs by platform.          |
| Other writes        | Writes outside the default writable locations are blocked unless you approve a broader sandbox request.   |
| Outbound networking | Network access is blocked unless you approve a host-specific or unrestricted network sandbox request.     |

By default, `fetch` cannot reach the network unless you approve access to the URL's host. If the response redirects to another host, Zed asks for access to that host before following the redirect. Fetch sandboxing does not allow `localhost` or IP-literal URLs.

## Approval Prompts {#approval-prompts}

When the agent needs access outside the default sandbox, Zed shows a sandbox approval prompt before the command or fetch runs. The prompt explains what the tool is asking for, such as:

- network access to specific hosts, such as `github.com` or `*.npmjs.org`
- network access to any host
- access to protected Git metadata, such as `.git` directories and linked worktree metadata
- write access to specific filesystem paths
- unrestricted filesystem writes
- running a command without the sandbox

For terminal commands, you can approve a request once, for the rest of the current thread, or always. Thread approvals are remembered only for that thread. “Always” approvals are saved in `settings.json` under `agent.sandbox_permissions`.

For `fetch`, sandbox prompts are one-time approvals. Persistent network grants in `agent.sandbox_permissions` can still pre-approve common hosts.

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

| Setting              | What it allows                                                                                                                                                 |
| -------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `network_hosts`      | Network access to the listed hosts without prompting. This applies to terminal commands and `fetch`. Entries can be exact hostnames or leading-`*.` wildcards. |
| `allow_all_hosts`    | Network access to any host without prompting. This applies to terminal commands and `fetch`.                                                                   |
| `allow_git_access`   | Access to protected Git metadata without prompting.                                                                                                            |
| `write_paths`        | Writes to the listed directory subtrees without prompting. Paths are absolute.                                                                                 |
| `allow_fs_write_all` | Writes anywhere on the filesystem without prompting.                                                                                                           |
| `allow_unsandboxed`  | Turns off sandboxing for terminal commands and disables `fetch` network sandbox prompts.                                                                       |

Prefer narrow grants, such as a specific host, Git metadata access, or write path, over `allow_all_hosts`, `allow_fs_write_all`, or `allow_unsandboxed`. Avoid `allow_all_hosts` for `fetch` unless you want the agent to fetch from any supported non-localhost hostname without prompting. Avoid `allow_unsandboxed` unless you want to turn off sandboxing for both terminal commands and fetch network access.

## Platform Support {#platform-support}

Sandboxing uses different operating system mechanisms on each platform. The prompts are similar, but the enforcement details vary for terminal commands. Fetch network approval works the same way across platforms.

### macOS {#macos}

On macOS, Zed uses Apple's Seatbelt sandbox through `sandbox-exec`.

A sandboxed terminal command can write inside open project directories and to a per-thread temporary directory exposed through `$TMPDIR`, `$TMP`, and `$TEMP`. It cannot write elsewhere, read or write protected Git metadata, or reach the network unless you approve that access.

When you approve host-specific network access, Zed uses an HTTP/HTTPS proxy so access can be limited to approved hosts. Tools that do not honor proxy environment variables, such as SSH, FTP, and raw socket clients, may not work even after host-specific network access is approved. For networked terminal commands, prefer HTTPS URLs over SSH URLs when possible.

When you approve Git metadata access, Zed may also expose the inherited `SSH_AUTH_SOCK` Unix socket. This lets workflows such as SSH commit signing work without granting outbound network access.

### Linux {#linux}

On Linux, Zed uses Bubblewrap (`bwrap`) for sandboxing.

Zed only uses a non-setuid `bwrap` binary. The sandbox is built on unprivileged user namespaces, so a setuid-root `bwrap` provides no extra functionality, and running one would mean executing root-privileged setup with arguments partly derived from model-influenced input. If the only `bwrap` found on your `PATH` is setuid-root, Zed refuses to run it; install a non-setuid Bubblewrap to enable sandboxing.

A sandboxed terminal command can write inside open project directories, except for protected Git metadata. It can also write to `/tmp`, which is backed by a fresh temporary filesystem and cleared between terminal tool calls. It cannot write elsewhere, write protected Git metadata, or reach the network unless you approve that access. On Linux, existing protected Git metadata is read-only by default.

Host-specific network access works through an HTTP/HTTPS proxy, like on macOS. Tools that do not honor proxy environment variables, such as SSH, FTP, and raw socket clients, may not work even after host-specific network access is approved. If a command needs unrestricted network access, approve all-host network access instead.

If Bubblewrap is unavailable or cannot create a sandbox in the current environment, Zed asks whether to retry, run the command without the sandbox, or deny the command.

### Windows {#windows}

On Windows, Zed Agent sandboxing is supported only when the agent action runs inside WSL.

Zed uses the Linux Bubblewrap sandbox inside WSL because WSL provides the Linux process and filesystem primitives that Bubblewrap needs. When a command runs inside WSL, the filesystem and Git metadata behavior is similar to Linux, including the requirement that `bwrap` not be setuid-root. Terminal network access is all-or-nothing on Windows: Zed cannot restrict sandboxed terminal commands to specific hosts there.

Native Windows processes do not currently have the same sandbox integration in Zed. If WSL is not installed, or if you choose to run a command without the sandbox, Zed falls back to the standard terminal behavior of running in your native shell. It selects the shell using the usual preference order: Git Bash (or scoop's bash) when one is installed, otherwise PowerShell, and finally `cmd.exe`.

Because the command then runs against native Windows paths instead of WSL's Linux filesystem, path conventions change accordingly. For example, a command may need `C:\...` or `/c/...` rather than WSL's `/mnt/c/...`, so a command written for the sandboxed WSL shell may behave differently.

## Choosing What to Approve {#choosing-what-to-approve}

When reviewing a sandbox prompt, prefer the narrowest permission that lets the task proceed:

- approve a specific host instead of all hosts when the destination is known
- approve fetch access only for hosts you expect the agent to retrieve
- approve Git metadata access when the command needs to run Git operations such as `git fetch`, `git commit`, or `git status`
- approve a specific write path instead of unrestricted filesystem writes
- approve unsandboxed execution only when the command cannot work inside the sandbox
- use one-time approvals for unfamiliar commands
- use thread or always approvals only for access you expect to reuse

If a command fails because the sandbox blocked access, ask the agent why it needs that access before approving a broader request.
