# Remote Development

Remote Development allows you to code at the speed of thought, even when your codebase is not on your local machine. You use CodeOrbit locally so the UI is immediately responsive, but offload heavy computation to the development server so that you can work effectively.

## Overview

Remote development requires two computers, your local machine that runs the CodeOrbit UI and the remote server which runs a CodeOrbit headless server. The two communicate over SSH, so you will need to be able to SSH from your local machine into the remote server to use this feature.

![Architectural overview of CodeOrbit Remote Development](https://CodeOrbit.dev/img/remote-development/diagram.png)

On your local machine, CodeOrbit runs its UI, talks to language models, uses Tree-sitter to parse and syntax-highlight code, and store unsaved changes and recent projects. The source code, language servers, tasks, and the terminal all run on the remote server.

> **Note:** The original version of remote development sent traffic via CodeOrbit's servers. As of CodeOrbit v0.157 you can no-longer use that mode.

## Setup

1. Download and install the latest [CodeOrbit](https://CodeOrbit.dev/releases). You need at least CodeOrbit v0.159.
1. Use {#kb projects::OpenRemote} to open the "Remote Projects" dialog.
1. Click "Connect New Server" and enter the command you use to SSH into the server. See [Supported SSH options](#supported-ssh-options) for options you can pass.
1. Your local machine will attempt to connect to the remote server using the `ssh` binary on your path. Assuming the connection is successful, CodeOrbit will download the server on the remote host and start it.
1. Once the CodeOrbit server is running, you will be prompted to choose a path to open on the remote server.
   > **Note:** CodeOrbit does not currently handle opening very large directories (for example, `/` or `~` that may have >100,000 files) very well. We are working on improving this, but suggest in the meantime opening only specific projects, or subfolders of very large mono-repos.

For simple cases where you don't need any SSH arguments, you can run `CodeOrbit ssh://[<user>@]<host>[:<port>]/<path>` to open a remote folder/file directly. If you'd like to hotlink into an SSH project, use a link of the format: `CodeOrbit://ssh/[<user>@]<host>[:<port>]/<path>`.

## Supported platforms

The remote machine must be able to run CodeOrbit's server. The following platforms should work, though note that we have not exhaustively tested every Linux distribution:

- macOS Catalina or later (Intel or Apple Silicon)
- Linux (x86_64 or arm64, we do not yet support 32-bit platforms)
- Windows is not yet supported.

## Configuration

The list of remote servers is stored in your settings file {#kb CodeOrbit::OpenSettings}. You can edit this list using the Remote Projects dialog {#kb projects::OpenRemote}, which provides some robustness - for example it checks that the connection can be established before writing it to the settings file.

```json
{
  "ssh_connections": [
    {
      "host": "192.168.1.10",
      "projects": [{ "paths": ["~/code/CodeOrbit/CodeOrbit"] }]
    }
  ]
}
```

CodeOrbit shells out to the `ssh` on your path, and so it will inherit any configuration you have in `~/.ssh/config` for the given host. That said, if you need to override anything you can configure the following additional options on each connection:

```json
{
  "ssh_connections": [
    {
      "host": "192.168.1.10",
      "projects": [{ "paths": ["~/code/CodeOrbit/CodeOrbit"] }],
      // any argument to pass to the ssh master process
      "args": ["-i", "~/.ssh/work_id_file"],
      "port": 22, // defaults to 22
      // defaults to your username on your local machine
      "username": "me"
    }
  ]
}
```

There are two additional CodeOrbit-specific options per connection, `upload_binary_over_ssh` and `nickname`:

```json
{
  "ssh_connections": [
    {
      "host": "192.168.1.10",
      "projects": [{ "paths": ["~/code/CodeOrbit/CodeOrbit"] }],
      // by default CodeOrbit will download the server binary from the internet on the remote.
      // When this is true, it'll be downloaded to your laptop and uploaded over SSH.
      // This is useful when your remote server has restricted internet access.
      "upload_binary_over_ssh": true,
      // Shown in the CodeOrbit UI to help distinguish multiple hosts.
      "nickname": "lil-linux"
    }
  ]
}
```

If you use the command line to open a connection to a host by doing `CodeOrbit ssh://192.168.1.10/~/.vimrc`, then extra options are read from your settings file by finding the first connection that matches the host/username/port of the URL on the command line.

Additionally it's worth noting that while you can pass a password on the command line `CodeOrbit ssh://user:password@host/~`, we do not support writing a password to your settings file. If you're connecting repeatedly to the same host, you should configure key-based authentication.

## Port forwarding

If you'd like to be able to connect to ports on your remote server from your local machine, you can configure port forwarding in your settings file. This is particularly useful for developing websites so you can load the site in your browser while working.

```json
{
  "ssh_connections": [
    {
      "host": "192.168.1.10",
      "port_forwards": [{ "local_port": 8080, "remote_port": 80 }]
    }
  ]
}
```

This will cause requests from your local machine to `localhost:8080` to be forwarded to the remote machine's port 80. Under the hood this uses the `-L` argument to ssh.

By default these ports are bound to localhost, so other computers in the same network as your development machine cannot access them. You can set the local_host to bind to a different interface, for example, 0.0.0.0 will bind to all local interfaces.

```json
{
  "ssh_connections": [
    {
      "host": "192.168.1.10",
      "port_forwards": [
        {
          "local_port": 8080,
          "remote_port": 80,
          "local_host": "0.0.0.0"
        }
      ]
    }
  ]
}
```

These ports also default to the `localhost` interface on the remote host. If you need to change this, you can also set the remote host:

```json
{
  "ssh_connections": [
    {
      "host": "192.168.1.10",
      "port_forwards": [
        {
          "local_port": 8080,
          "remote_port": 80,
          "remote_host": "docker-host"
        }
      ]
    }
  ]
}
```

## CodeOrbit settings

When opening a remote project there are three relevant settings locations:

- The local CodeOrbit settings (in `~/.CodeOrbit/settings.json` on macOS or `~/.config/CodeOrbit/settings.json` on Linux) on your local machine.
- The server CodeOrbit settings (in the same place) on the remote server.
- The project settings (in `.CodeOrbit/settings.json` or `.editorconfig` of your project)

Both the local CodeOrbit and the server CodeOrbit read the project settings, but they are not aware of the other's main `settings.json`.

Depending on the kind of setting you want to make, which settings file you should use:

- Project settings should be used for things that affect the project: indentation settings, which formatter / language server to use, etc.
- Server settings should be used for things that affect the server: paths to language servers, etc.
- Local settings should be used for things that affect the UI: font size, etc.

In addition any extensions you have installed locally will be propagated to the remote server. This means that language servers, etc. will run correctly.

## Initializing the remote server

Once you provide the SSH options, CodeOrbit shells out to `ssh` on your local machine to create a ControlMaster connection with the options you provide.

Any prompts that SSH needs will be shown in the UI, so you can verify host keys, type key passwords, etc.

Once the master connection is established, CodeOrbit will check to see if the remote server binary is present in `~/.codeorbit_server` on the remote, and that its version matches the current version of CodeOrbit that you're using.

If it is not there or the version mismatches, CodeOrbit will try to download the latest version. By default, it will download from `https://CodeOrbit.dev` directly, but if you set: `{"upload_binary_over_ssh":true}` in your settings for that server, it will download the binary to your local machine and then upload it to the remote server.

If you'd like to maintain the server binary yourself you can. You can either download our prebuilt versions from [GitHub](https://github.com/CodeOrbit-industries/CodeOrbit/releases), or [build your own](https://CodeOrbit.dev/docs/development) with `cargo build -p remote_server --release`. If you do this, you must upload it to `~/.codeorbit_server/CodeOrbit-remote-server-{RELEASE_CHANNEL}-{VERSION}` on the server, for example `~/.codeorbit_server/CodeOrbit-remote-server-stable-0.181.6`. The version must exactly match the version of CodeOrbit itself you are using.

## Maintaining the SSH connection

Once the server is initialiCodeOrbit. CodeOrbit will create new SSH connections (reusing the existing ControlMaster) to run the remote development server.

Each connection tries to run the development server in proxy mode. This mode will start the daemon if it is not running, and reconnect to it if it is. This way when your connection drops and is restarted, you can continue to work without interruption.

In the case that reconnecting fails, the daemon will not be re-used. That said, unsaved changes are by default persisted locally, so that you do not lose work. You can always reconnect to the project at a later date and CodeOrbit will restore unsaved changes.

If you are struggling with connection issues, you should be able to see more information in the CodeOrbit log `cmd-shift-p Open Log`. If you are seeing things that are unexpected, please file a [GitHub issue](https://github.com/CodeOrbit-industries/CodeOrbit/issues/new) or reach out in the #remoting-feedback channel in the [CodeOrbit Discord](https://CodeOrbit.dev/community-links).

## Supported SSH Options

Under the hood, CodeOrbit shells out to the `ssh` binary to connect to the remote server. We create one SSH control master per project, and use then use that to multiplex SSH connections for the CodeOrbit protocol itself, any terminals you open and tasks you run. We read settings from your SSH config file, but if you want to specify additional options to the SSH control master you can configure CodeOrbit to set them.

When typing in the "Connect New Server" dialog, you can use bash-style quoting to pass options containing a space. Once you have created a server it will be added to the `"ssh_connections": []` array in your settings file. You can edit the settings file directly to make changes to SSH connections.

Supported options:

- `-p` / `-l` - these are equivalent to passing the port and the username in the host string.
- `-L` / `-R` for port forwarding
- `-i` - to use a specific key file
- `-o` - to set custom options
- `-J` / `-w` - to proxy the SSH connection
- `-F` for specifying an `ssh_config`
- And also... `-4`, `-6`, `-A`, `-B`, `-C`, `-D`, `-I`, `-K`, `-P`, `-X`, `-Y`, `-a`, `-b`, `-c`, `-i`, `-k`, `-l`, `-m`, `-o`, `-p`, `-w`, `-x`, `-y`

Note that we deliberately disallow some options (for example `-t` or `-T`) that CodeOrbit will set for you.

## Known Limitations

- You can't open files from the remote Terminal by typing the `CodeOrbit` command.

## Feedback

Please join the #remoting-feedback channel in the [CodeOrbit Discord](https://CodeOrbit.dev/community-links).
