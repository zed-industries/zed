---
title: Running and Testing Code - Zed
description: Run, test, and debug your code without leaving Zed. Tasks, REPL, and debugger integration.
---

# Running & Testing

This section covers how to run, test, and debug your code without leaving Zed.

## What's here

- **[Terminal](./terminal.md)**: Zed's built-in terminal emulator. Open multiple terminals, customize your shell, and integrate with the editor. Tasks and commands run here.

- **[Tasks](./tasks.md)**: Define and run shell commands with access to editor context like the current file, selection, or symbol. Use tasks to build, lint, run scripts, or execute any repeatable workflow.

- **[Debugger](./debugger.md)**: Set breakpoints, step through code, and inspect variables using Zed's built-in debugger. Works with C, C++, Go, JavaScript, Python, Rust, TypeScript, and more through the Debug Adapter Protocol.

- **[REPL](./repl.md)**: Run code interactively using Jupyter kernels. Execute selections or cells and see results inline—useful for Python, TypeScript (Deno), R, Julia, and other supported languages.

## Quick start {#quick-start}

**Open a terminal**: Press {#kb terminal_panel::ToggleFocus} to toggle the terminal panel.

**Run a task**: Press {#kb task::Spawn} to open the task picker, then type any shell command.

**Start debugging**: Press {#kb debugger::Start} to open the debug panel and select a configuration.

**Run code interactively**: In a Python or TypeScript file, select some code and press {#kb repl::Run} to execute it in a REPL session.
