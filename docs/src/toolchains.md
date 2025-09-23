# Toolchains

Zed projects offer a dedicated UI for toolchain selection, which lets you pick a set of tools for working with a given language in a current project.

Imagine you're working with Python project, which has virtual environments that encapsulate a set of dependencies of your project along with a suitable interpreter to run it with. The language server has to know which virtual environment you're working with, as it uses it to understand your project's code.
With toolchain selector, you don't need to spend time configuring your language server to point it at the right virtual environment directory—you can just select the right virtual environment (toolchain) from a dropdown.

You can even select different toolchains for different subprojects within your Zed project. A definition of a subproject is language-specific.
In collaborative scenarios, only the project owner can see and modify an active toolchain.

In [remote projects](./remote-development.md), you can use the toolchain selector to control the active toolchain on the SSH host. When [sharing your project](./collaboration.md), the toolchain selector is not available to guests.

## Why do we need toolchains?

The active toolchain is relevant for launching language servers, which may need it to function properly—it may not be able to resolve dependencies, which in turn may make functionalities like "Go to definition" or "Code completions" unavailable.

The active toolchain is also relevant when launching a shell in the terminal panel: some toolchains provide "activation scripts" for shells, which make those toolchains available in the shell environment for your convenience. Zed will run these activation scripts automatically when you create a new terminal.

This also applies to [tasks](./tasks.md)—Zed tasks behave "as if" you opened a new terminal tab and ran a given task invocation yourself, which in turn means that Zed task execution is affected by the active toolchain and its activation script.

## Selecting toolchains

The active toolchain (if there is one) is displayed in the status bar (on the right hand side). Click on it to access the toolchain selector—you can also use an action from a command palette ({#action toolchain::Select}).

Zed will automatically infer a set of toolchains to choose from based on the project you're working with. A default will also be selected on your behalf on a best-effort basis when you open a project for the first time.

The toolchain selection applies to a current subproject, which—depending on the structure of your Zed project—might be your whole project or just a subset of it. For example, if you have a monorepo with multiple subprojects, you might want to select a different toolchain for each subproject.

## Adding toolchains manually

If automatic detection does not suffice for you, you can add toolchains manually. To do that, click on the "Add toolchain" button in the toolchain selector. From there you can provide a path to a toolchain and set a name of your liking for it.
