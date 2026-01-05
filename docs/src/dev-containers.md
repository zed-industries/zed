# Dev Containers

Dev Containers provide a consistent, reproducible development environment by defining your project's dependencies, tools, and settings in a container configuration.

If your repository includes a `.devcontainer/devcontainer.json` file, Zed can open a project inside a development container.

## Requirements

- [(Preview Only)](https://zed.dev/releases/preview): Download and install the latest Zed.
- Docker must be installed and available in your `PATH`. Zed requires the `docker` command to be present. If you use Podman, you can alias it to `docker` (e.g., `alias docker=podman`).
- Your project must contain a `.devcontainer/devcontainer.json` directory/file.

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

## Known Limitations

> **Note:** This feature is still in development.

- **Extensions:** Zed does not yet manage extensions separately for container environments. The host's extensions are used as-is.
- **Port forwarding:** Only the `appPort` field is supported. `forwardPorts` and other advanced port-forwarding features are not implemented.
- **Configuration changes:** Updates to `devcontainer.json` do not trigger automatic rebuilds or reloads; containers must be manually restarted.

## See also

- [Remote Development](./remote-development.md) for connecting to remote servers over SSH.
- [Tasks](./tasks.md) for running commands in the integrated terminal.
