# Accounts

Signing in to Zed is not a requirement. You can use most features you'd expect in a code editor without ever doing so. We'll outline the few features that do require signing in, and how to do so, here.

## What Features Require Signing In?

1. All real-time [collaboration features](./collaboration.md).
2. [LLM-powered features](./assistant/assistant.md), if you are using Zed as the provider of your LLM models. Alternatively, you can [bring and configure your own API keys](./assistant/configuration.md) if you'd prefer, and avoid having to sign in.

## Signing In

Zed uses GitHub's OAuth flow to authenticate users, requiring only the `read:user` GitHub scope, which grants read-only access to your GitHub profile information.

1. Open Zed and click the `Sign In` button in the top-right corner of the window, or run the `client: sign in` command from the command palette (`cmd-shift-p` on macOS or `ctrl-shift-p` on Windows/Linux).
2. Your default web browser will open to the Zed sign-in page.
3. Authenticate with your GitHub account when prompted.
4. After successful authentication, your browser will display a confirmation, and you'll be automatically signed in to Zed.

**Note**: If you're behind a corporate firewall, ensure that connections to `zed.dev` and `collab.zed.dev` are allowed.

## Signing Out

To sign out of Zed, you can use either of these methods:

- Click on the profile icon in the upper right corner and select `Sign Out` from the dropdown menu.
- Open the command palette and run the `client: sign out` command.
