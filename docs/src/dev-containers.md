---
title: Dev Containers - Zed
description: Open projects in dev containers with Zed. Reproducible development environments using devcontainer.json configuration.
---

# Dev Containers

Dev Containers provide a consistent, reproducible development environment by defining your project's dependencies, tools, and settings in a container configuration.

If your repository includes a `.devcontainer/devcontainer.json` file, Zed can open a project inside a development container.

## Requirements

- Docker or Podman must be installed and available in your `PATH`.
- Your project must contain a `.devcontainer/devcontainer.json` directory/file.

By default Zed tries `docker`, then falls back to `podman`. To change the order, or to use only one engine, set `"container_engines"` in your settings.json, for example `"container_engines": ["podman"]`. Zed probes each engine on the machine that will build the container and uses the first one that is present. (The older `use_podman: true` setting still works if `container_engines` is unset, but is deprecated.)

On macOS, add `"container"` to `container_engines` to build with Apple's [`container`](https://github.com/apple/container) CLI instead of Docker or Podman, for example `"container_engines": ["container", "docker"]`. `container` is never tried by default — installing it must not silently change how existing projects build. Two limitations: it only exists on macOS, so it's skipped automatically for a project opened over SSH or WSL; and it has no `docker compose` equivalent, so a devcontainer.json with a `dockerComposeFile` never selects it, falling through to the next engine in the list instead.

By default Zed builds dev container images with BuildKit when the `docker buildx` plugin is available. If your Docker-compatible engine lacks an integrated BuildKit, set `"dev_container_use_buildkit": false` in your settings.json to use the classic Docker builder instead. Apple's `container` always uses the classic builder; this setting has no effect on it.

## Using Dev Containers in Zed

### Automatic prompt

When you open a project that contains the `.devcontainer/devcontainer.json` directory/file, Zed will display a prompt asking whether to open the project inside the dev container. Choosing "Open in Container" will:

1. Build the dev container image (if needed).
2. Launch the container.
3. Reopen the project connected to the container environment.

### Manual open

If you dismiss the prompt or want to reopen the project inside a container later, you can use Zed's command palette to run the "Project: Open Remote" command and select the option to open the project in a dev container.
Alternatively, you can reach for the Remote Projects modal (through the {#kb projects::OpenRemote} binding) and choose the "Connect Dev Container" option.

## Dev containers on a remote host

A dev container can also run on a machine you reach over SSH, rather than on the machine running Zed. The project files stay on that machine, and its container engine builds and runs the container.

Open the project on the remote server first, as described in [Remote Development](./remote-development.md), and then open it in a container the same way you would locally — through the prompt, or the "Connect Dev Container" option in the Remote Projects modal ({#kb projects::OpenRemote}). Zed uses the connection it already has, so you are not asked to authenticate a second time.

In this mode:

- Docker or Podman must be installed on the **remote host**. You do not need a container engine on your own machine.
- Every path in `devcontainer.json` — mounts, workspace folders, `${localWorkspaceFolder}` — describes the remote host's filesystem.
- `initializeCommand` runs on the remote host, not on your machine. The dev container specification defines it as running on the host machine, and under this model that host is the remote server. If you rely on it for side effects on your own machine, they will not happen.
- `${localEnv:...}` resolves against the remote host's shell environment, for the same reason.
- The container is shown together with the host it runs on, in the connection modal and in your recent projects, so it is distinguishable from a local container of the same name.

You cannot open a dev container from a project shared with you by another user over collaboration, because the container would have to be built on their machine.

## Editing the dev container configuration

If you modify `.devcontainer/devcontainer.json`, Zed does not currently rebuild or reload the container automatically. After changing configuration:

- Stop or kill the existing container manually (e.g., via `docker kill <container>`).
- Reopen the project in the container.

## Working in a Dev Container

Once connected, Zed operates inside the container environment for tasks, terminals, and language servers.
Files are linked from your workspace into the container according to the dev container specification.

## Extensions

You can specify extensions in `.devcontainer/devcontainer.json` under the "customizations" field like so:

```json
{
  ...
  "customizations": {
    "zed": {
      "extensions": ["vue", "ruby"],
    },
    "vscode": {
      ...
    },
    "codespaces": {
      ...
    },
  }
}
```

Note that extensions load for the Zed session, so these extensions will exist on your local Zed instances as well.

## Known Limitations

> **Note:** This feature is still in development.

- **Configuration changes:** Updates to `devcontainer.json` do not trigger automatic rebuilds or reloads; containers must be manually restarted.

For containers running on a remote host, additionally:

- **Port forwarding** is routed through the remote host, which requires it to be able to reach container addresses directly. This is the case for Linux bridge networking, but not for Docker Desktop's virtual machine. Forwarding a port from a container on your own machine is not supported at all.
- **Windows clients** skip the `updateRemoteUserUID` step entirely, even when the host is Linux, so the container user's ID is not matched to the owner of your files on the host and the workspace may be read-only inside the container.

## See also

- [Remote Development](./remote-development.md) for connecting to remote servers over SSH.
- [Tasks](./tasks.md) for running commands in the integrated terminal.
