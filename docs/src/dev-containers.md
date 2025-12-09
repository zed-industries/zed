# Dev Containers

Zed can open a project inside a development container when the repository includes a `.devcontainer/devcontainer.json` file. This allows Zed to build and connect to a containerized environment automatically.

## Prerequisites

- Docker (or a compatible CLI such as Podman) must be installed and available in your `PATH`.
- Your project must contain a `.devcontainer/devcontainer.json` file.

## Using Dev Containers in Zed

### Automatic prompt

When you open a folder that contains a `.devcontainer/devcontainer.json`, Zed will display a prompt asking whether to open the project inside the dev container. Choosing "Open in Container" will:

1. Build the dev container image (if needed).
1. Launch the container.
1. Reopen the project connected to the container environment.

### Manual open

If you dismiss the prompt or want to reopen the project inside a container later, you can use Zed's command palette to select the option to open the project in a dev container.

## Editing the dev container configuration

If you modify `.devcontainer/devcontainer.json`, Zed does not currently rebuild or reload the container automatically. After changing configuration:

1. Stop or kill the existing container manually (e.g., via `docker kill <container>`).
1. Reopen the project in the container.

## Working in a dev container

Once connected, Zed operates inside the container environment for tasks, terminals, and language servers. Files are linked from your workspace into the container according to the devcontainer specification.

## Known Limitations

> **Note:** The current implementation is an MVP and has several limitations.

- **Extensions:** Zed does not yet manage extensions separately for container environments. The host's extensions are used as-is.
- **Port forwarding:** Only the `appPort` field is supported. `forwardPorts` and other advanced port-forwarding features are not implemented.
- **Configuration changes:** Updates to `devcontainer.json` do not trigger automatic rebuilds or reloads; containers must be manually restarted.
- **Dependency on Docker:** The feature requires `docker` (or a compatible CLI) to be available in the system `PATH`.
- **Other devcontainer features:** Some parts of the devcontainer specification are not yet implemented and may be added in future releases.
