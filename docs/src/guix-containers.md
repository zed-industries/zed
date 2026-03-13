---
title: Guix Containers - Zed
description: Open projects in Guix containers with Zed. Use manifest.scm to run language servers, tasks, and terminals inside guix shell --container.
---

# Guix Containers

Guix Containers let you reopen a project inside a `guix shell --container` environment while keeping the Zed UI local.

If your project contains a `manifest.scm`, Zed can reopen that project in a Guix container so project-side commands run inside the container environment instead of on the host.

## Requirements

- `guix` must be installed and available in your `PATH`.
- Your project must contain a `manifest.scm`.
- The paths you want to open in one Guix workspace must resolve to the same `manifest.scm`.

## Using Guix Containers in Zed

### Automatic prompt

When you open a local project that contains `manifest.scm`, Zed will display a prompt asking whether to re-open the project in a container.

Choosing "Open in Container" opens the Guix container options modal for the current project. From there you can save options and reopen the project in the container.

### Manual open

If you dismiss the prompt or want to reopen the project in a container later, you can:

- run `projects: open guix container` from the command palette while the local project is open
- open the Remote Projects modal with {#kb projects::OpenRemote} and choose `Connect Guix Container`

Both paths open the same Guix container options modal used by the automatic prompt flow.

## Guix Container Options

The Guix container options modal lets you configure:

- `Allow network access (-N)`
- `Allow nested Guix invocations (--nesting)`
- extra `--expose` mounts
- extra `--share` mounts
- extra `guix shell` arguments

For `--expose` and `--share`, enter one mount per line using either:

- `source`
- `source=target`

Saving writes per-project settings for the detected `manifest.scm` and project root.

## Settings

Guix container settings are stored under `remote.guix_connections` in your settings file.

```json [settings]
{
  "remote": {
    "guix_connections": [
      {
        "manifest_path": "/home/me/code/project/manifest.scm",
        "project_root": "/home/me/code/project",
        "options": {
          "allow_network": true,
          "nesting": false,
          "expose": [
            { "source": "/var/cache/guix" }
          ],
          "share": [
            { "source": "/tmp/project-cache", "target": "/cache" }
          ],
          "extra_args": ["--pure"]
        }
      }
    ]
  }
}
```

In most cases it is simpler to edit these options through the Guix container modal rather than by hand.

## Working in a Guix Container

Once connected, Zed runs project-side commands inside the Guix container environment, including:

- language servers
- tasks
- terminals
- other remote-project command execution

Zed keeps its UI local while the project host runs inside the container.

## Known Limitations

> **Note:** This feature is still in development.

- Guix containers are opened from an existing local project. There is not a separate standalone Guix project picker.
- If opened paths resolve to different `manifest.scm` roots, Zed will not combine them into one Guix container workspace.
- If you change `manifest.scm` or Guix container options, reopen the project in the container to apply the new environment.
- Port forwarding is not currently supported by the Guix transport.

## See also

- [Remote Development](./remote-development.md) for SSH- and WSL-based remote projects.
- [Dev Containers](./dev-containers.md) for `devcontainer.json`-based container workflows.
- [Tasks](./tasks.md) for running commands in project environments.
