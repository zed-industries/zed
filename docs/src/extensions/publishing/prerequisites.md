---
title: Extension Publishing Prerequisites
description: "Review the requirements for publishing an extension to the Zed Extension Registry."
---

# Extension Publishing Prerequisites {#extension-publishing-prerequisites}

Before submitting your extension for publishing, make sure it meets the following requirements.

Note that maintainters will raise non-compliance during the publishing process. Should you chose to not follow these requirements, publishing will be delayed or may outright be rejected.

## General Requirements

- Your extension must have been tested locally at the submodule commit you are submitting it at.
- Publish functionality that is not already available in the extension marketplace.
  - If you face issues with an existing extension, first try contributing to the existing extension. See [the section below](#requirements-post-publishing) for more details on this reqirement.
- Use an appropriate ID for the extension. The ID must be
  - unique
  - camel-cased
  - not include the words `zed` nor `extension`
  - be a good indicator for what your extension provides (more on this below)
- Do not misuse existing APIs to workaround limitations of our current API interface.
- Include only the resources the extension needs to function within your extension.
- License your extension under one of the [allowed licenses](./license-requirements.md).
- All user-facing text must be in English.
- Do not read or modify anything outside the environment Zed designates for the extension.
  - Use the [Zed Rust Extension API](https://docs.rs/zed_extension_api/latest/zed_extension_api/) to read and modify the environment.
  - Use Rust standard library methods to read and modify the work directory provided to the extension.
  - Ask the user to make any other required changes themselves.

## Language Extensions

- Your extension may only provide support for that language as well as any dialects that are directly associated with that language.
- Your extension may also provide language servers and snippets for that language.
- Your extension ID and name should be similar or equal to the name of the primary langauge you intend to add.
- Your extension must not include any Rust-code should it not also provide language servers.

## Language Server Extensions

- Should your extension only provide a language server, make sure your extension ID reflects that (e.g., by suffixing it with `-language-server` or `-lsp`).
- Do not bundle a language server with the extension. Download it or check for it in the user's environment through the [Zed Rust Extension API](https://docs.rs/zed_extension_api/latest/zed_extension_api/).

## Debugger Extensions

- Should your extension only provide a debugger, make sure your extension ID reflects that (e.g., by suffixing it with `-debugger`).
- Do not bundle the debug adapter with the extension. Download it or check for it in the user's environment through the [Zed Rust Extension API](https://docs.rs/zed_extension_api/latest/zed_extension_api/).

## Theme Extensions

- Your extension must only provide themes and nothing else.
- Your extension ID must include something to indicate it is a theme, e.g. be suffixed with `-theme`.

## Icon Theme Extensions

- Your extension must only provide an icon theme and nothing else.
- Your extension ID must include something to indicate it is an icon theme, e.g. be suffixed with `-icon-theme` or `-icons`.

## Snippet Extensions

- Should your extension only provide snippets, make sure your extension ID reflects that (e.g., by suffixing it with `-snippets`).
- Your snippets may only be scoped to the global scope if appropriate. Language-specific snippets must be scoped to the given languages

## MCP Server Extensions

> MCP server extensions will be deprecated in favor of the MCP registry in the future, progress for this is tracked in [#59351](https://github.com/zed-industries/zed/issues/59351). Please make sure to also publish your server against the registry to be sure it can be used with future versions of Zed.

- Your extension must only provide one MCP server and nothing else.
- Your extension ID must include something to indicate it is an MCP server, e.g. be prefixed with `mcp-server-` or suffixed with `-mcp-server`.
- Do not bundle the MCP server with the extension. Download it or check for it in the user's environment through the [Zed Rust Extension API](https://docs.rs/zed_extension_api/latest/zed_extension_api/).

## Agent Server and Slash Command Extensions

Agent server and slash command extensions have been deprecated and submissions will no longer be accepted. If you intend on making an agent server available within Zed, publish it to the [ACP Registry](https://agentclientprotocol.com/registry) instead.

Passing everything on this list? Let's [get your extension published!](./publishing-guide.md)

# Requirements Post Publishing

After you extension has been published, there are no further requirements from our side. Thank you for enriching the Zed extension collection, we really appreciate it!

However, not every extension is perfect when it is first published and issues might arise, requiring maintenance effort. Generally, we expect

- users to report issues in the original extension repository as well as
- extension owners to get back to those users within a reasonable timeframe

Now, maintaining an extension might not be everyones cup of tea or things might have changed since publishing - that is totally fine! We fully understand that maintenance can take a lot of time and as mentioned above, do not require it.
Yet, at the same time, we do not want existing extensions to go stale in a bad state in an effort to provide a good experience to users of that extension, which obviously clashes with the aforementioned. Due to this, we intentionally do not just want to live with stale extensions.

Thus, in the case an extension owner no longer wants to maintain an extension or is unresponsive, we provide the following options:

- The current owner may transfer ownership of the repository to a new owner.
- A contributor may fork the extension and publish their fork as a replacement for the current extension.
- Zed staff can fork the extension into the `zed-extensions` organization and maintenance can continue from there as a joined effort of the community and Zed staff.
- Open an issue or pull request against `zed-industries/extensions` asking for the removal of the extension.

In order for any of the options to be applicable, we require either

- written permission from the current extension owner or
- proof of the reporting contributor that the upstream extension owner has been unresponsive to change requests for at least two months
