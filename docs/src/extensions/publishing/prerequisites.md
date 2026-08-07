---
title: Extension Publishing Prerequisites
description: "Review the requirements for publishing an extension to the Zed Extension Registry."
---

# Extension Publishing Prerequisites {#extension-publishing-prerequisites}

Before publishing your extension, make sure that you have chosen a unique extension ID for your extension in the [extension manifest](../developing-extensions.md#directory-structure-of-a-zed-extension).
This will be the primary identifier for your extension and cannot be changed after your extension has been published.
Also, ensure that you have filled out all the required fields in the manifest.

Furthermore, please make sure that your extension fulfills the following preconditions before you move on to publishing your extension:

- Extension IDs and names must not contain the words `zed`, `Zed` or `extension`, since they are all Zed extensions.
- Your extension ID should provide some information on what your extension tries to accomplish. E.g. for themes, it should be suffixed with `-theme`, snippet extensions should be suffixed with `-snippets` and so on. An exception to that rule is an extension that provides support for languages or popular tooling that people would expect to find under that ID. You can take a look at the list of [existing extensions](https://github.com/zed-industries/extensions/blob/main/extensions.toml) to get a grasp on how this usually is enforced.
- Your extension must only include the resources it requires to function and nothing else.
  - See the [directory structure of a Zed extension](../developing-extensions.md#directory-structure-of-a-zed-extension) and the [Rust and WebAssembly](../developing-extensions.md#rust-and-webassembly) sections for more information.
- Extensions must in no way attempt to read nor modify the environment outside of the environment designated to them by Zed. Should they need to read the environment, they should use methods as provided by the [Zed Rust Extension API](https://docs.rs/zed_extension_api/latest/zed_extension_api/) and may fall back to appropriate methods from the Rust standard library. Should they need changes to the environment, they must instead ask the user to perform these for them using an appropriate method within the context (e.g. provide information for doing so using the `ContextServerConfiguration` for context servers).
  - Please make sure to have read the [Rust and WebAssembly section](../developing-extensions.md#rust-and-webassembly) for more information and help regarding this topic.
- Extensions should provide something that is not yet available in the marketplace as opposed to fixing something that could be resolved within an existing extension. For example, if you find that an existing extension's support for a language server is not functioning properly, first try contributing a fix to the existing extension as opposed to submitting a new extension immediately.
  - If you receive no response or reaction within the upstream repository within a reasonable amount of time, feel free to submit a pull request that aims to fix said issue. Please ensure that you provide your previous efforts within the pull request to the extensions repository for adding your extension. Zed maintainers will then decide on how to proceed on a case by case basis.
- Extensions that intend to provide a language, debugger or MCP server must not ship the language server as part of the extension. Instead, the extension should either download the language server or check for the availability of the language server in the user's environment using the APIs as provided by the [Zed Rust Extension API](https://docs.rs/zed_extension_api/latest/zed_extension_api/).
- Themes and icon themes should not be published as part of extensions that provide other features, e.g. language support. Instead, they should be published as a distinct extension. This also applies to themes and icon themes living in the same repository.

Furthermore, before publishing your extension, [install it locally as a dev extension](../developing-extensions.md#developing-an-extension-locally) and test it thoroughly. Submissions for extensions that have not been tested and do not function at all may be closed without further feedback.

Non-compliance with these rules will be raised during the publishing process by reviewers. If you fail to comply with the laid out guidelines, the publishing of your extension will either be delayed or rejected.
