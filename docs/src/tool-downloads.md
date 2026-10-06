---
title: Tool downloads in Zed
description: "Control when Zed downloads and installs language servers, debug adapters, and other tools."
---

# Tool downloads

Zed can download and install tools it needs: built-in language servers, built-in debug adapters, the Zed-managed Node.js, Prettier, GitHub Copilot, registry agents, and `ipykernel` for the REPL.
By default, Zed does not download any of them until you allow it.
You can change this during onboarding or in the Settings Editor under "Security".

Zed does not interrupt you with notifications about blocked downloads.
Instead, it shows a "Downloads Blocked" button in the title bar, and the affected features show a download icon:

- the language server button in the status bar, for language servers and Node.js
- the GitHub Copilot button in the status bar
- the agent panel, while an agent is loading
- the activity indicator, while Prettier is waiting
- the debug panel and the debug console, while a debug session is waiting

Click any of them, or run the `workspace::ToggleBinaryDownloads` action, to review the pending downloads.
Installing an agent from the agent registry or onboarding allows its download.

For each tool you can:

- **Allow** it: Zed downloads it now and remembers the decision between restarts.
- **Deny** it: features that need the tool fail with an error until Zed restarts.

Already downloaded language servers, debug adapters, Copilot, and npm-based agents keep working while their updates wait for approval in the same list.
An allowed update is downloaded the next time the tool starts, as with any update Zed installs.
GitHub Copilot restarts right away when you allow its update; restart a language server from the language server menu to update it immediately.
Registry agents distributed as archives have no older version to fall back to, so an unapproved agent stops working when the registry publishes a new version, until you allow it.
Zed uses tools found on your `PATH` first, where the language server supports it, and never asks about those.

Approvals are remembered per tool and per host, so SSH and WSL remote hosts each have their own decisions.
Run the `workspace::ClearAllowedBinaryDownloads` action to forget all approvals.
Already downloaded tools keep working, and Zed asks again before their next download.
Connected remote hosts keep the cleared approvals until their remote server restarts.

## Allowing all downloads

To let Zed download tools without asking, use the "Always Allow Downloads" button in the review dialog, or set ([how to edit](./configuring-zed.md#settings-files)):

```json [settings]
"allow_binary_downloads": true
```

## What is not covered

Extensions download and run tools through their own permissions, configured with [`granted_extension_capabilities`](./extensions/capabilities.md).
