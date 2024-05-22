# Remote Development

Remote Development is in the early stages of development. If you'd like to try it please email [alpha@zed.dev](mailto:alpha@zed.dev).

Remote Development allows you to code at the speed of thought, even when your codebase is not on your local machine. You use Zed locally so the UI is immediately responsive, but offload heavy computation to the development server so that you can work effectively.

## Overview

Remote development requires running two instances of Zed. A headless instance on the remote machine, and the editor interface on your local computer. All configuration is done on your local computer, except for starting the headless instance.

Currently the two instances connect via Zed's servers, but we intend to build peer to peer communication in the future.

## Setup

> NOTE: You must be in the alpha program to see this UI. The instructions will likely change as the feature gets closer to launch.

1. Open the remote projects dialogue with `cmd-shift-p remote`
2. Click "Add Server"
3. Choose whether to setup via SSH, or to follow the manual setup.
   > NOTE: With both options your laptop and the remote machine will communicate
     via https://collab.zed.dev/, so you will need outbound internet access on the remote.
4. On the remote machine, install Zed
   ```
   curl https://zed.dev/install.sh | bash
   ```
5. On the remote machine, paste the instructions from step 3. You should see `connected!`.
   > NOTE: If this command runs but doesn't output anything, try running `zed --foreground --dev-server-token YY.XXX`. It is possible that the zed background process is crashing on startup.
6. On your laptop you can now open folders on the remote machine.
   > NOTE: Zed does not currently handle opening very large directories (e.g. `/` or `~` that may have >100,000 files) very well. We are working on improving this, but suggest in the meantime opening only specific projects, or subfolders of very large mono-repos.

## Toubleshooting

### SSH connections

If you chose to connect via SSH, the command you specify will be run in a zed terminal. Once a connection is established (you may need to type your password, or
your key passphrase) we will download and install zed to `~/.local/bin/zed` on the
remote machine and boot it in headless mode.

If you don't see any output from the zed command, it is likely that zed is crashing
on startup. You can troubleshoot this switching to manual mode. Please file a bug for
any issues you encouter.

### Tested ssh formats

Because of the way we control the ssh session, we are able to support most "ssh wrappers". We also pull in your existing ssh config if you have options set there then they will apply to the zed connection too. For example:

* `user@host` will assume you meant `ssh user@host`
* `gh cs ssh -c example-codespace` to connect to a github codespace
* `ssh -J` to connect via a jump-host
* `doctl compute ssh example-droplet` to connect to

### In either mode:

There are a few likely causes of failure

* `zed --dev-server-token` runs but outputs nothing. This is probably because the zed background process is crashing on startup. Try running `zed --dev-server-token XX --foreground` to see any output, and [file a bug](https://github.com/zed-industries/zed) so we can debug it together.
* `zed --dev-server-token` outputs something like "Connection refused" or "Unauthorized" and immediately exits. This is likely due to issues making outbound HTTP requests from the box.
* `zed --dev-server-token` outputs "Zed is already running". If you are editing an existing server, it is possible that clicking "Connect" a second time will work, but if not you will have to manually log into the server and kill the zed process.


## Supported platforms

The remote machine must be able to run Zed. The following platforms should work, though note that we have not exhaustively tested every linux distribution:

* macOS Catalina or later (Intel or Apple Silicon))
* Linux (x86_64 only). You must have `glibc` installed at version 2.29 (released in 2019) or greater and available globally.
* Windows is not yet supported.

## Known Limitations

- The Terminal does not work remotely.
- You cannot spawn Tasks remotely.
- Extensions aren't yet supported in headless Zed.
- You can not run `zed` in headless mode and in GUI mode at the same time on the same machine.

## Feedback

- Please join the #remoting-feedback in the [Zed Discord](https://discord.gg/qSDQ8VWc7k).
