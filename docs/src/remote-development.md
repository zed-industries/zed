# Remote Development

Remote Development allows you to code at the speed of thought, even when your codebase is not on your local machine. You use Zed locally so the UI is immediately responsive, but offload heavy computation to the development server so that you can work effectively.

> **Note:** Remoting is still "alpha". We have several changes we would like to make before it is fully released.

## Overview

Remote development requires running two instances of Zed. A headless instance on the remote machine, and the editor interface on your local computer. All configuration is done on your local computer.

Currently the two instances connect via Zed's servers, but we intend to build peer to peer communication before the feature is fully released.

## Setup

1. Download and install the latest [Zed Preview](https://zed.dev/releases/preview).
1. Open the remote projects dialogue with `cmd-shift-p remote`.
1. Click "New Server".
1. Choose whether to setup via SSH, or to follow the manual setup.
   > **Note:** With both options your laptop and the remote machine will communicate
   > via https://collab.zed.dev/, so you will need outbound internet access on the remote machine.
1. On your laptop you can now open folders on the remote machine.
   > **Note:** Zed does not currently handle opening very large directories (for example, `/` or `~` that may have >100,000 files) very well. We are working on improving this, but suggest in the meantime opening only specific projects, or subfolders of very large mono-repos.

## Troubleshooting

### UI is not showing up

You need to be on a relatively recent Zed (v0.145.0 or later).

### SSH connections

If you chose to connect via SSH, the command you specify will be run in a Zed terminal given you an opportunity to type any passwords/keyphrases etc. that you need.
Once a connection is established, Zed will be downloaded and installed to `~/.local/bin/zed` on the remote machine, and run.

If you don't see any output from the Zed command, it is likely that Zed is crashing
on startup. You can troubleshoot this by switching to manual mode and passing the `--foreground` flag. Please [file a bug](https://github.com/zed-industries/zed) so we can debug it together.

If you are trying to connect to a platform like GitHub Codespaces or Google Cloud, you may want to first make sure that your SSH configuration is set up correctly. Once you can `ssh X` to connect to the machine, then Zed will be able to connect.

> **Note:** In an earlier version of remoting, we supported typing in `gh cs ssh` or `gcloud compute ssh` directly. This is no longer supported. Instead you should make sure your SSH configuration is up to date with `gcloud compute ssh --config` or `gh cs ssh --config`, or use Manual setup mode if you cannot ssh directly to the machine.

### zed --dev-server-token isn't connecting

There are a few likely causes of failure:

- `zed --dev-server-token` runs but outputs nothing. This is probably because the Zed background process is crashing on startup. Try running `zed --dev-server-token XX --foreground` to see any output, and [file a bug](https://github.com/zed-industries/zed) so we can debug it together.
- `zed --dev-server-token` outputs something like "Connection refused" or "Unauthorized" and immediately exits. This is likely due to issues making outbound HTTP requests to https://collab.zed.dev from your host. You can try to debug this with `curl https://collab.zed.dev`, but we have seen cases where curl is whitelisted, but other binaries are not allowed network access.
- `zed --dev-server-token` outputs "Zed is already running". If you are editing an existing server, it is possible that clicking "Connect" a second time will work, but if not you will have to manually log into the server and kill the Zed process.

## Supported platforms

The remote machine must be able to run Zed. The following platforms should work, though note that we have not exhaustively tested every Linux distribution:

- macOS Catalina or later (Intel or Apple Silicon)
- Linux (x86_64 or arm64, we do not yet support 32-bit platforms). You must have `glibc` installed at version 2.29 (released in 2019) or greater and available globally.
- Windows is not yet supported.

## Settings and extensions

> **Note:** This may change as the alpha program continues.

<!--
TBD: Remote user settings need a name. Perhaps `zed: remote user settings`?
-->

You can edit the settings file on the remote instance. To do so, add a new project to your server in the directory `~/.config/zed`. You can create a file called `settings.json` if it does not yet exist.

Note that this is most useful for configuring language servers, as any UI related settings do not apply.

If you'd like to install language-server extensions, you can add them to the list of `auto_installed_extensions`. Again you don't need to do this to get syntax highlighting (which is handled by the local zed).

```json
{
  "auto_install_extensions": {
    "java": true
  }
}
```

## Known Limitations

- You can't use the Terminal or Tasks if you choose "Manual Connection"
- You can't run `zed` in headless mode and in GUI mode at the same time on the same machine.
- You can't open files from the remote Terminal by typing the `zed` command.

## Feedback

Please join the #remoting-feedback channel in the [Zed Discord](https://discord.gg/zed-community).

# Direct SSH Connections

The current alpha release of Zed always connects via our servers. This was to get experience building the feature on top of our existing collaboration support. We plan to move to direct SSH connections for any machine that can be SSH'd into.

We are working on a direct SSH connection feature, which you can try out if you'd like.

> **Note:** Direct SSH support does not support most features yet! You cannot use project search, language servers, or basically do anything except edit files...

To try this out you can either from the command line run:

```sh
zed ssh://user@host:port/path/to/project
```

Or you can (in your settings file) add:

```json
"ssh_connections": []
```

And then from the command palette choose `projects: Open Remote` and configure an SSH connection from there.
