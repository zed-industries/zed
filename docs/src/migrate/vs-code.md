# How to Migrate from VS Code to Zed

This guide is for developers who’ve spent serious time in VS Code and want to try Zed without starting from scratch.

If you’re here, you might be looking for a faster editor. Or something less cluttered. Or you’re curious about built-in collaboration. Whatever brought you here, this guide helps you move over your habits, shortcuts, and settings.

We’ll cover what to bring, what to change, and what’s different. You can ease in gradually or switch all at once. Either way, you’ll stay productive.

## Install Zed
Zed is available on macOS, Windows, and Linux.

For macOS, you can download it from zed.dev/download, or install via Homebrew:
`brew install zed-editor/zed/zed`
For most Linux users, the easiest way to install Zed is through our installation script:
`curl -f https://zed.dev/install.sh | sh`

After installation, you can launch Zed from your Applications folder (macOS) or directly from the terminal (Linux) using:
`zed .`
This opens the current directory in Zed.

## Import Settings from VS Code

During setup, you have the option to import key settings from VS Code. Zed imports the following settings:
[add]

Zed doesn’t import extensions or keybindings, but this is the fastest way to get a familiar feel while trying something new. If you skip that step during setup, you can still import settings manually later via the command palette:

`Cmd+Shift+P → Zed: Import Settings from VS Code`
