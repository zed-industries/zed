# Environment Variables

Multiple features in Zed are affected by environment variables:

- Tasks
- Built-in terminal
- Language servers and debug adapters

In order to make the best use of these features, it's helpful to understand where Zed gets its environment variables from and how they're used.

## Where does Zed get its environment variables from?

### The inherited environment

When Zed is launched via the macOS Dock, or a GNOME or KDE icon on Linux, or an application launcher like Alfred or Raycast, it is launched as a child of the Window manager. The process inherits the environment set by your window manager. This environment typically does not contain any customization from your shell configuration files.

In rare cases (for example when developing Zed, or using `zed --foreground`), the environment is inherited from a shell instead of the window manager.

### The CLI environment

If Zed is opened via the CLI (`zed`), it will use the environment variables from the surrounding shell session for any projects
that are opened in this way.

That means if you do

```
$ export MY_ENV_VAR=hello
$ zed .
```

the environment variable `MY_ENV_VAR` is now available to processes run by Zed for the project in the current directory. For example, in the built-in terminal.

### The loaded environment

If you have a project open in Zed that does not have an associated CLI environment, then Zed will spawn a shell, cd into the root directory of that project, and then run `printenv`. This will load an environment that will (ideally) match the CLI environment. This allows us to provide the expected environment variables to tools managed by things like `direnv`, `asdf` or `mise`, and to ensure that when we look up binaries, we are looking in the correct `PATH`.

### The HOME environment

For some tools (like `node` for language server management) Zed runs them outside of the context of a project. In this case we load the
environment from your `HOME` directory as described above.

## Where and how are environment variables used?

### Tasks

Tasks are spawned with an combined environment. In order of precedence (low to high, with the last overwriting the first):

- the Zed inherited environment
- if the project was opened from the CLI: the CLI environment
- optional, explicitly configured environment in settings

### Built-in terminal

Built-in terminals, like tasks, are spawned with an combined environment. In order of precedence (low to high):

- the Zed inherited environment
- optional, explicitly configured environment in settings

### Look-up of language servers and debug adapters

For some languages the language server adapters lookup the binary in the user's `$PATH`. Examples:

- Go
- Zig
- Rust (if [configured to do so](./languages/rust.md#binary))
- C
- TypeScript

For this look-up, Zed uses the following the environment:

- the Zed inherited environment
- if the project was opened from the CLI: the CLI environment
- if the project was not opened from the CLI: the loaded environment

### Language servers and debug adapters

After looking up a language server, Zed starts them.

These language server processes always inherit Zed's inherited environment. But, depending on the language server look-up, additional environment variables might be set or overwrite the inherited environment.

- If the language server was found in the project environment's `$PATH`, then the project environment's is passed along to the language server process. Where the project environment comes from depends on how the project was opened, via CLI or not. See previous point on look-up of language servers.
- If the language servers was not found in the project environment, Zed tries to install it globally and start it globally. In that case, the process will inherit Zed's inherited environment, and — if the project was opened via ClI — from the CLI.
