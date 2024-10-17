# Remote Development

Remote Development allows you to code at the speed of thought, even when your codebase is not on your local machine. You use Zed locally so the UI is immediately responsive, but offload heavy computation to the development server so that you can work effectively.

> **Note:** Remoting is still "alpha". We are still refining the relaibility and performance.

## Overview

Remote development requires two computers, your local machine that runs the Zed UI and the remote server which runs a small Zed server. The two communicate over SSH, so you will need to be able to SSH from your local machine into the remote server to use this feature.

> **Note:** The original version of remote development sent traffic via Zed's servers. As of Zed v0.157 you can no-longer configure new projects in this mode, and in Zed v0.159 support for these will be removed completely.

## Setup

1. Download and install the latest [Zed Preview](https://zed.dev/releases/preview). You ned at least Zed v0.158.
1. Open the remote projects dialogue with `cmd-shift-p remote`.
1. Click "New Server" and enter the command you use to ssh into the server. See [Supported SSH options](#supported-ssh-options) for options you can pass.
1. Your local machine will attempt to connect to the remote server using the `ssh` binary on your path. Assuming the connection is successful, it will download the latest version of the Zed server and upload it to the remote over SSH.
1. Once the Zed server is running, you will be prompted to choose a path to open on the remote server.
   > **Note:** Zed does not currently handle opening very large directories (for example, `/` or `~` that may have >100,000 files) very well. We are working on improving this, but suggest in the meantime opening only specific projects, or subfolders of very large mono-repos.

## Supported platforms

The remote machine must be able to run Zed's server. The following platforms should work, though note that we have not exhaustively tested every Linux distribution:

- macOS Catalina or later (Intel or Apple Silicon)
- Linux (x86_64 or arm64, we do not yet support 32-bit platforms). You must have `glibc` installed at version 2.29 (released in 2019) or greater and available globally.
- Windows is not yet supported.

## Settings

The remote machine will by default inherit all your settings from your local machine's Zed configuration, additionally any project configuration from `.zed/settings.json` in the root of your project will be applied. If you want different settings (for example language server configuration) on a per-server basis you can use `cmd-shift-p Open Server Settings` while connected to a remote project to create a settings file for that server.

## Supported SSH Options

Under the hood, Zed shells out to the `ssh` binary to connect to the remote server. We create one SSH control master per project, and use then use that to multiplex ssh connections for the Zed protocol itself, any terminals you open and tasks you run. We read settings from your ssh config file, but if you want to specify additional options to the ssh control master you can configure Zed to set them.

When typing in the "New Server" dialogue, you can use bash-style quoting to pass options containing a space. Once you have created a server it will be added to the `"ssh_connections": []` array in your settings file. You can edit the settings file directly to make changes to SSH connections.

Supported options:
* `-p` / `-l` - these are equivalent to passing the port and the username in the host string.
* `-L` / `-R` for port forwarding
* `-i` - to use a specific key file
* `-o` - to set custom options
* `-J` / `-w` - to proxy the SSH connection
* And also... `-4`, `-6`, `-A`, `-a`, `-C`, `-K`, `-k`, `-X`, `-x`, `-Y`, `-y`, `-B`, `-b`, `-c`, `-D`, `-I`, `-i`, `-J`, `-l`, `-m`, `-o`, `-P`, `-p`, `-w`

Note that we deliberately disallow some options (for example `-t` or `-T`) that Zed will set for you.

## Connecting & Reconnecting

When you first connect to a remote project, the Zed running on your local machine will SSH in, and upload the latest version of the Zed server. If this fails you should see an error message.

Once the remote server is uploaded, we run two copies of it:
* The first is the "proxy" process. This is attached to the SSH tty and so is killed when your SSH connection closes.
* The second is the "server" process. This process is backgrounded so it will continue running for about 10 minutes after the connection is closed. This allows us to recover quickly if your connection is lost.

Your local Zed will continually ping the remote server and expect it to reply. If it hasn't replied for a few seconds, it will start a reconnect process. You can tell this is happening because the server icon in the top left will change color.

If the reconnection is successful, and the "server" process is still running on the remote host, a new proxy process will be created and any pending edits uploaded. If reconnecting fails, Zed will show an error overlay, and give you the option to manually retry.

If you are struggling with connection issues, you should be able to see more information in the Zed log `cmd-shift-p Open Log`. If you are seeing things that are unexpected, please file a [GitHub issue](https://github.com/zed-industries/zed/issues/new) or reach out in the #remoting-feedback channel in the [Zed Discord](https://discord.gg/zed-community).

## Known Limitations

- Zed extensions are not yet supported on remotes, so languages that need them for support do not work.
- You can't open files from the remote Terminal by typing the `zed` command.
- Zed does not yet support automatic port-forwarding. You can use `-R` and `-L` in your SSH arguments for now.

## Feedback

Please join the #remoting-feedback channel in the [Zed Discord](https://discord.gg/zed-community).

## Troubleshooting

When you create a new
