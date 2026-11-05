---
title: Dev Containers - Zed
description: Open projects in dev containers with Zed. Reproducible development environments using devcontainer.json configuration.
---

# Dev Containers

Dev Containers provide a consistent, reproducible development environment by defining your project's dependencies, tools, and settings in a container configuration.

If your repository includes a `.devcontainer/devcontainer.json` file, Zed can open a project inside a development container.

## Requirements

- Docker or Podman must be installed and available in your `PATH`. If you use `podman`, you must set the `use_podman` setting in your Zed settings.json to true.
- Your project must contain a `.devcontainer/devcontainer.json` directory/file.

By default Zed builds dev container images with BuildKit when the `docker buildx` plugin is available. If your Docker-compatible engine lacks an integrated BuildKit (for example, Apple Container accessed through a Docker-API bridge), set `"dev_container_use_buildkit": false` in your settings.json to use the classic Docker builder instead.

## Using Dev Containers in Zed

### Automatic prompt

When you open a project that contains the `.devcontainer/devcontainer.json` directory/file, Zed will display a prompt asking whether to open the project inside the dev container. Choosing "Open in Container" will:

1. Build the dev container image (if needed).
2. Launch the container.
3. Reopen the project connected to the container environment.

### Manual open

If you dismiss the prompt or want to reopen the project inside a container later, you can use Zed's command palette to run the "Project: Open Remote" command and select the option to open the project in a dev container.
Alternatively, you can reach for the Remote Projects modal (through the {#kb projects::OpenRemote} binding) and choose the "Connect Dev Container" option.

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

## See also

- [Remote Development](./remote-development.md) for connecting to remote servers over SSH.
- [Tasks](./tasks.md) for running commands in the integrated terminal.
