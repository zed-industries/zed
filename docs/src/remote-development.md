# Remote Development

Remote Development is in the early stages of development. If you'd like to try it please email [alpha@zed.dev](mailto:alpha@zed.dev).

Remote Development allows you to code at the speed of thought, even when your codebase is not on your local machine. You use Zed locally so the UI is immediately responsive, but offload heavy computation to the development server so that you can work effectively.

## Overview

Remote development requires running two instances of Zed. A headless instance on the remote machine, and the editor interface on your local computer. All configuration is done on your local computer, except for starting the headless instance.

Currently the two instances connect via Zed's servers, but we intend to build peer to peer communication in the future.

## Setup

> NOTE: You must be in the alpha program to see this UI. The instructions will likely change as the feature gets closer to launch.

1. Open the projects dialog with `cmd-option-o` and then click "Connect…".
2. Click "Add Server"
3. Give it a name, and copy the instructions given.
4. On the remote machine, install Zed
   ```
   curl https://zed.dev/install.sh | bash
   ```
5. On the remote machine, paste the instructions from step 3.

   > NOTE: Currently you must keep this process open. We are working on making it background itself.

6. On your laptop you can now open folders on the remote machine.

   > NOTE: Zed does not currently handle opening very large directories (e.g. `/` or `~` that may have >100,000 files) very well. We are working on improving this, but suggest in the meantime opening only specific projects, or subfolders of very large mono-repos.

## Known Limitations

- The Terminal does not work remotely.
- You cannot spawn Tasks remotely.
- Extensions aren't yet supported in headless Zed.

## Feedback

- Please join the [#remoting-feedback](https://discord.com/channels/869392257814519848/1235290452270387241) Discord channel.
