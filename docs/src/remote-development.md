# Remote Development

Remote Development is in the early stages of development. If you'd like to try it please email [alpha@zed.dev](mailto:alpha@zed.dev).

Remote Development allows you to code at the speed of thought, even when your codebase is not on your local machine. You use Zed locally so the UI is immediately responsive, but offload heavy computation to the development server so that you can work effectively.

## Overview

Remote development requires running two instances of Zed. A headless instance on the remote machine, and the editor interface on your local computer. All configuration is done on your local computer.

Currently the two instances connect via Zed's servers, but we intend to build peer to peer communication in the future.

## Setup

> **Note:** You must be in the alpha program to see this UI. The instructions will likely change as the feature gets closer to launch.

1. Download and install the latest [Zed Preview](https://zed.dev/releases/preview).
1. Open the remote projects dialogue with `cmd-shift-p remote`.
2. Click "Add Server".
3. Choose whether to setup via SSH, or to follow the manual setup.
   > **Note:** With both options your laptop and the remote machine will communicate
     via https://collab.zed.dev/, so you will need outbound internet access on the remote machine.
6. On your laptop you can now open folders on the remote machine.
   > **Note:** Zed does not currently handle opening very large directories (for example, `/` or `~` that may have >100,000 files) very well. We are working on improving this, but suggest in the meantime opening only specific projects, or subfolders of very large mono-repos.

## Toubleshooting

### UI is not showing up

This can happen either if you were just added to the alpha, in which case you need to restart Zed. Or, if you lost connection to the Zed server, in which case you just need to click "Sign In" in the top right.

### SSH connections

If you chose to connect via SSH, the command you specify will be run in a Zed terminal given you an opportunity to type any passwords/keyphrases etc. that you need.
Once a connection is established, Zed will be downloaded and installed to `~/.local/bin/zed` on the remote machine, and run.

If you don't see any output from the Zed command, it is likely that Zed is crashing
on startup. You can troubleshoot this by switching to manual mode and passing the `--foreground` flag. Please [file a bug](https://github.com/zed-industries/zed) so we can debug it together.

### SSH-like connections

Zed intercepts `ssh` in a way that should make it possible to intercept connections made by most "ssh wrappers". For example you
can specify:

- `user@host` will assume you meant `ssh user@host`
- `ssh -J jump target` to connect via a jump-host
- `gh cs ssh -c example-codespace` to connect to a GitHub codespace
- `doctl compute ssh example-droplet` to connect to a DigitalOcean Droplet
- `gcloud compute ssh` for a Google Cloud instance

### zed --dev-server-token isn't connecting

There are a few likely causes of failure:

- `zed --dev-server-token` runs but outputs nothing. This is probably because the Zed background process is crashing on startup. Try running `zed --dev-server-token XX --foreground` to see any output, and [file a bug](https://github.com/zed-industries/zed) so we can debug it together.
- `zed --dev-server-token` outputs something like "Connection refused" or "Unauthorized" and immediately exits. This is likely due to issues making outbound HTTP requests to https://collab.zed.dev from your host. You can try to debug this with `curl https://collab.zed.dev`, but we have seen cases where curl is whitelisted, but other binaries are not allowed network access.
- `zed --dev-server-token` outputs "Zed is already running". If you are editing an existing server, it is possible that clicking "Connect" a second time will work, but if not you will have to manually log into the server and kill the Zed process.

## Supported platforms

The remote machine must be able to run Zed. The following platforms should work, though note that we have not exhaustively tested every Linux distribution:

- macOS Catalina or later (Intel or Apple Silicon)
- Linux (x86_64 only). You must have `glibc` installed at version 2.29 (released in 2019) or greater and available globally.
- Windows is not yet supported.

## Known Limitations

- The Terminal does not work remotely unless you configure the machine to use SSH.
- You cannot spawn Tasks remotely.
- Extensions aren't yet supported in headless Zed.
- You can not run `zed` in headless mode and in GUI mode at the same time on the same machine.

## Feedback

Please join the #remoting-feedback channel in the [Zed Discord](https://discord.gg/qSDQ8VWc7k).
