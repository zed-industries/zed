# Dev Containers

Dev containers provide a consistent, reproducible development environment by defining your project's dependencies, tools, and settings in a container configuration.

If your repository includes a `.devcontainer/devcontainer.json` file, Zed can open a project inside a development container.

## Requirements

- Docker must be installed and available in your `PATH`. Zed requires the `docker` command to be present. If you use Podman, you can alias it to `docker` (e.g., `alias docker=podman`).
- Your project must contain a `.devcontainer/devcontainer.json` file.

## Using Dev Containers in Zed

### Automatic prompt

When you open a folder that contains a `.devcontainer/devcontainer.json`, Zed will display a prompt asking whether to open the project inside the dev container. Choosing "Open in Container" will:

1. Build the dev container image (if needed).
2. Launch the container.
3. Reopen the project connected to the container environment.

### Manual open

If you dismiss the prompt or want to reopen the project inside a container later, you can use Zed's command palette to select the option to open the project in a dev container.

## Editing the dev container configuration

If you modify `.devcontainer/devcontainer.json`, Zed does not currently rebuild or reload the container automatically. After changing configuration:

- Stop or kill the existing container manually (e.g., via `docker kill <container>`).
- Reopen the project in the container.

## Working in a dev container

Once connected, Zed operates inside the container environment for tasks, terminals, and language servers. Files are linked from your workspace into the container according to the devcontainer specification.

## Known Limitations

> **Note:** The current implementation is an MVP and has several limitations.

- **Extensions:** Zed does not yet manage extensions separately for container environments. The host's extensions are used as-is.
- **Port forwarding:** Only the `appPort` field is supported. `forwardPorts` and other advanced port-forwarding features are not implemented.
- **Configuration changes:** Updates to `devcontainer.json` do not trigger automatic rebuilds or reloads; containers must be manually restarted.
- **Dependency on Docker:** The feature requires `docker` (or a compatible CLI) to be available in the system `PATH`.
- **Other devcontainer features:** Some parts of the devcontainer specification are not yet implemented and may be added in future releases.

## See also

- [Remote Development](./remote-development.md) for connecting to remote servers over SSH.
- [Tasks](./tasks.md) for running commands in the integrated terminal.
