# Developing CodeOrbit

See the platform-specific instructions for building CodeOrbit from source:

- [macOS](./development/macos.md)
- [Linux](./development/linux.md)
- [Windows](./development/windows.md)

If you'd like to develop collaboration features, additionally see:

- [Local Collaboration](./development/local-collaboration.md)

## Keychain access

CodeOrbit stores secrets in the system keychain.

However, when running a development build of CodeOrbit on macOS (and perhaps other
platforms) trying to access the keychain results in a lot of keychain prompts
that require entering your password over and over.

On macOS this is caused by the development build not having a stable identity.
Even if you choose the "Always Allow" option, the OS will still prompt you for
your password again the next time something changes in the binary.

This quickly becomes annoying and impedes development speed.

That is why, by default, when running a development build of CodeOrbit an alternative
credential provider is used in order to bypass the system keychain.

> Note: This is **only** the case for development builds. For all non-development
> release channels the system keychain is always used.

If you need to test something out using the real system keychain in a
development build, run CodeOrbit with the following environment variable set:

```
codeorbit_DEVELOPMENT_USE_KEYCHAIN=1
```

## Contributor links

- [CONTRIBUTING.md](https://github.com/CodeOrbit-industries/CodeOrbit/blob/main/CONTRIBUTING.md)
- [Releases](./development/releases.md)
- [Debugging Crashes](./development/debugging-crashes.md)
- [Code of Conduct](https://CodeOrbit.dev/code-of-conduct)
- [CodeOrbit Contributor License](https://CodeOrbit.dev/cla)
