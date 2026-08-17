---
title: Extension Publishing Prerequisites
description: "Review the requirements for publishing an extension to the Zed Extension Registry."
---

# Extension Publishing Prerequisites {#extension-publishing-prerequisites}

Before submitting your extension for publishing, make sure it meets the following requirements.

Note that maintainters will raise non-compliance during the publishing process. Should you chose to not follow these requirements, publishing will be delayed or may outright be rejected.

## General Requirements

- Test your extension locally at the submodule commit you are submitting it at.
- Publish functionality that is not already available in the extension marketplace.
  - If you face issues with an existing extension, first try contributing to the existing extension. See [the section below](#requirements-post-publishing) for more details on this requirement.
- Use an appropriate ID for your extension. The ID must:
  - be unique
  - be camel-cased
  - not include the words `zed` or `extension`
  - be a good indicator for what your extension provides (more on this below)
- Do not misuse existing APIs to work around limitations of our current API interface.
- Include only the resources your extension needs to function.
- License your extension under one of the [allowed licenses](./license-requirements.md).
- Write all user-facing text in English.
- Do not read or modify anything outside the environment Zed designates for your extension.
  - Use the [Zed Rust Extension API](https://docs.rs/zed_extension_api/latest/zed_extension_api/) to read and modify the environment.
  - Use Rust standard library methods to read and modify the work directory provided to the extension.
  - Ask the user to make any other required changes themselves.

## Language Extensions

- Only provide support for the language your extension targets, as well as any dialects that are directly associated with that language.
- You may also provide language servers and snippets for that language.
- Choose an extension ID and name similar or equal to the name of the primary language you intend to add.
- Do not include any Rust code should your extension not also provide language servers.

## Language Server Extensions

- Should your extension only provide a language server, make sure your extension ID reflects that (e.g., by suffixing it with `-language-server` or `-lsp`).
- Do not bundle a language server with the extension. Download it or check for it in the user's environment through the [Zed Rust Extension API](https://docs.rs/zed_extension_api/latest/zed_extension_api/).

## Debugger Extensions

- Should your extension only provide a debugger, make sure your extension ID reflects that (e.g., by suffixing it with `-debugger`).
- Do not bundle the debug adapter with the extension. Download it or check for it in the user's environment through the [Zed Rust Extension API](https://docs.rs/zed_extension_api/latest/zed_extension_api/).

## Theme Extensions

- Only provide themes and nothing else.
- Make sure your extension ID indicates it is a theme (e.g., by suffixing it with `-theme`).

## Icon Theme Extensions

- Only provide an icon theme and nothing else.
- Make sure your extension ID indicates it is an icon theme (e.g., by suffixing it with `-icon-theme` or `-icons`).

## Snippet Extensions

- Should your extension only provide snippets, make sure your extension ID reflects that (e.g., by suffixing it with `-snippets`).
- Only scope your snippets to the global scope if appropriate. Scope language-specific snippets to the given languages.

## MCP Server Extensions

> MCP server extensions will be deprecated in favor of the MCP registry in the future, progress for this is tracked in [#59351](https://github.com/zed-industries/zed/issues/59351). Please make sure to also publish your server against the registry to be sure it can be used with future versions of Zed.

- Only provide one MCP server and nothing else.
- Make sure your extension ID indicates it is an MCP server (e.g., by prefixing it with `mcp-server-` or suffixing it with `-mcp-server`).
- Do not bundle the MCP server with the extension. Download it or check for it in the user's environment through the [Zed Rust Extension API](https://docs.rs/zed_extension_api/latest/zed_extension_api/).

## Agent Server and Slash Command Extensions

Agent server and slash command extensions have been deprecated and submissions will no longer be accepted. If you intend on making an agent server available within Zed, publish it to the [ACP Registry](https://agentclientprotocol.com/registry) instead.

Passing everything on this list? Let's [get your extension published!](./publishing-guide.md)

# Requirements Post Publishing

After your extension has been published, there are no further requirements from our side. Thank you for enriching the Zed extension collection, we really appreciate it!

However, not every extension is perfect when it is first published and issues might arise, requiring maintenance effort. While we do not demand this effort from anyone, we generally expect:

- users and contributors to report issues and propose improvements in the original extension repository first, rather than publishing a competing extension
- extension owners to get back to those users within a reasonable timeframe

Now, maintaining an extension might not be everyone's cup of tea or things might have changed since publishing - that is totally fine! We fully understand that maintenance can take a lot of time and, as mentioned above, we do not require it.
At the same time, we do not want existing extensions to go stale in a bad state, as that leaves users of that extension with a poor experience. These two goals obviously clash: we cannot both leave maintenance entirely optional and just live with stale extensions.

To resolve this, in the case an extension owner no longer wants to maintain an extension or is unresponsive, we provide the following options:

- The current owner may transfer ownership of the repository to a new owner.
- A contributor may fork the extension and publish their fork as a replacement for the current extension.
- Zed staff can fork the extension into the `zed-extensions` organization and maintenance can continue from there as a joint effort of the community and Zed staff.
- The current owner may open an issue or pull request against `zed-industries/extensions` asking for the removal of the extension.

In order for any of the options to be applicable, we require either:

- written permission from the current extension owner
- proof of the reporting contributor that the upstream extension owner has been unresponsive to change requests for at least two months
