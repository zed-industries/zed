# Developing Zed

See the platform-specific instructions for building Zed from source:

- [macOS](./development/macos.md)
- [Linux](./development/linux.md)
- [Windows](./development/windows.md)

If you'd like to develop collaboration features, additionally see:

- [Local Collaboration](./development/local-collaboration.md)

## Authentication

When developing Zed you will typically want to sign in to the production collab
instance, unless you are specifically working on features that require running
collab locally.

In order to bypass the keychain prompts that pop up when trying to sign in each
time you run a development build of Zed, you can use the development auth
provider.

This will store your Zed access token in a local file on disk that can be read
in development, bypassing the need to retrieve the credential from the system
keychain.

To enable the development auth provider, set this in your shell:

```
ZED_DEVELOPMENT_AUTH=1
```

You may want to add this to your shell profile so you don't need to remember to enable it each time.

> Note: This only works for development builds. It is a no-op in all non-development release channels.

## Contributor links

- [CONTRIBUTING.md](https://github.com/zed-industries/zed/blob/main/CONTRIBUTING.md)
- [Releases](./development/releases.md)
- [Debugging Crashes](./development/debugging-crashes.md)
- [Code of Conduct](https://zed.dev/code-of-conduct)
- [Zed Contributor License](https://zed.dev/cla)
