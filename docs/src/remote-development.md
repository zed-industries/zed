# Remote Development

Remote Development allows you to code at the speed of thought, even when your codebase is not on your local machine. You use Zed locally so the UI is immediately responsive, but offload heavy computation to the development server so that you can work effectively.

> **Note:** Remoting is still "beta". We are still refining the reliability and performance.

## Overview

Remote development requires two computers, your local machine that runs the Zed UI and the remote server which runs a Zed headless server. The two communicate over SSH, so you will need to be able to SSH from your local machine into the remote server to use this feature.

> **Note:** The original version of remote development sent traffic via Zed's servers. As of Zed v0.157 you can no-longer use this mode.

## Setup

1. Download and install the latest [Zed Preview](https://zed.dev/releases/preview). You need at least Zed v0.159.
1. Open the remote projects dialogue with <kbd>cmd-shift-p remote</kbd> or <kbd>cmd-control-o</kbd>.
1. Click "Connect New Server" and enter the command you use to SSH into the server. See [Supported SSH options](#supported-ssh-options) for options you can pass.
1. Your local machine will attempt to connect to the remote server using the `ssh` binary on your path. Assuming the connection is successful, it will download the latest version of the Zed server and upload it to the remote over SSH.
1. Once the Zed server is running, you will be prompted to choose a path to open on the remote server.
   > **Note:** Zed does not currently handle opening very large directories (for example, `/` or `~` that may have >100,000 files) very well. We are working on improving this, but suggest in the meantime opening only specific projects, or subfolders of very large mono-repos.

For simple cases where you don't need any SSH arguments, you can run `zed ssh://[<user>@]<host>[:<port>]/<path>` to open a remote folder/file directly.

## Supported platforms

The remote machine must be able to run Zed's server. The following platforms should work, though note that we have not exhaustively tested every Linux distribution:

- macOS Catalina or later (Intel or Apple Silicon)
- Linux (x86_64 or arm64, we do not yet support 32-bit platforms)
- Windows is not yet supported.

## Settings

When opening a remote project there are three relevant settings locations:

- The local Zed settings (in `~/.zed/settings.json` on macOS or `~/.config/zed/settings.json` on Linux) on your local machine.
- The server Zed settings (in the same place) on the remote server.
- The project settings (in `.zed/settings.json` or `.editorconfig` of your project)

Both the local Zed and the server Zed read the project settings, but they are not aware of the other's main `settings.json`.

Depending on the kind of setting you want to make, which settings file you should use:

- Project settings should be used for things that affect the project: indentation settings, which formatter / language server to use, etc.
- Server settings should be used for things that affect the server: paths to language servers, etc.
- Local settings should be used for things that affect the UI: font size, etc.

## Initializing the remote server

Once you provide the SSH options, Zed shells out to `ssh` on your local machine to create a ControlMaster connection with the options you provide.

Any prompts that SSH needs will be shown in the UI, so you can verify host keys, type key passwords, etc.

Once the master connection is established, Zed will check to see if the remote server binary is present in `~/.zed_server` on the remote, and that its version matches the current version of Zed that you're using.

If it is not there or the version mismatches, Zed will try to download the latest version. By default, it will download from `https://zed.dev` directly, but if you set: `{"remote_server": {"download":false}}` in your local settings, it will download the binary to your local machine and then upload it to the remote server.

## Maintaining the SSH connection

Once the server is initialized. Zed will create new SSH connections (reusing the existing ControlMaster) to run the remote development server.

Each connection tries to run the development server in proxy mode. This mode will start the daemon if it is not running, and reconnect to it if it is. This way when your connection drops and is restarted, you can continue to work without interruption.

In the case that reconnecting fails, the daemon will not be re-used. That said, unsaved changes are by default persisted locally, so that you do not lose work. You can always reconnect to the project at a later date and Zed will restore unsaved changes.

If you are struggling with connection issues, you should be able to see more information in the Zed log `cmd-shift-p Open Log`. If you are seeing things that are unexpected, please file a [GitHub issue](https://github.com/zed-industries/zed/issues/new) or reach out in the #remoting-feedback channel in the [Zed Discord](https://discord.gg/zed-community).

## Supported SSH Options

Under the hood, Zed shells out to the `ssh` binary to connect to the remote server. We create one SSH control master per project, and use then use that to multiplex SSH connections for the Zed protocol itself, any terminals you open and tasks you run. We read settings from your SSH config file, but if you want to specify additional options to the SSH control master you can configure Zed to set them.

When typing in the "Connect New Server" dialogue, you can use bash-style quoting to pass options containing a space. Once you have created a server it will be added to the `"ssh_connections": []` array in your settings file. You can edit the settings file directly to make changes to SSH connections.

Supported options:

- `-p` / `-l` - these are equivalent to passing the port and the username in the host string.
- `-L` / `-R` for port forwarding
- `-i` - to use a specific key file
- `-o` - to set custom options
- `-J` / `-w` - to proxy the SSH connection
- And also... `-4`, `-6`, `-A`, `-a`, `-C`, `-K`, `-k`, `-X`, `-x`, `-Y`, `-y`, `-B`, `-b`, `-c`, `-D`, `-I`, `-i`, `-J`, `-l`, `-m`, `-o`, `-P`, `-p`, `-w`

Note that we deliberately disallow some options (for example `-t` or `-T`) that Zed will set for you.

## Known Limitations

- Zed extensions are not yet supported on remotes, so languages that need them for support do not work.
- You can't open files from the remote Terminal by typing the `zed` command.
- Zed does not yet support automatic port-forwarding. You can use `-R` and `-L` in your SSH arguments for now.

## Feedback

Please join the #remoting-feedback channel in the [Zed Discord](https://discord.gg/zed-community).
