---
title: Extension License Requirements
description: "Understand the license requirements for publishing a Zed extension."
---

# Extension License Requirements {#extension-license-requirements}

As of October 1st, 2025, extension repositories must include a license.
The following licenses are accepted:

- [Apache 2.0](https://www.apache.org/licenses/LICENSE-2.0)
- [BSD 2-Clause](https://opensource.org/license/bsd-2-clause)
- [BSD 3-Clause](https://opensource.org/license/bsd-3-clause)
- [CC BY 4.0](https://creativecommons.org/licenses/by/4.0)
- [GNU GPLv3](https://www.gnu.org/licenses/gpl-3.0.en.html)
- [GNU LGPLv3](https://www.gnu.org/licenses/lgpl-3.0.en.html)
- [MIT](https://opensource.org/license/mit)
- [Unlicense](https://unlicense.org)
- [zlib](https://opensource.org/license/zlib)

This allows us to distribute the resulting binary produced from your extension code to our users.
Without a valid license, the pull request to add or update your extension will fail CI.

Your license file should be at the root of your extension, though not necessarily at the root of the repository. If your extension is in a subdirectory within its repository, the license must reside within that subdirectory; a license at the repository root will not work. You may symlink an existing license into the extension directory or choose another accepted license for the extension code.

Any filename that has `LICENCE` or `LICENSE` as a prefix (case insensitive) will be inspected to ensure it matches one of the accepted licenses. See the [license validation source code](https://github.com/zed-industries/extensions/blob/main/src/lib/license.js).

Please note that:

- This license requirement applies only to your extension code itself (the code that gets compiled into the extension binary).
- It does not apply to any tools your extension may download or interact with, such as language servers or other external dependencies.
- If your repository contains both extension code and other projects (like a language server), you are not required to relicense those other projects; only the extension code needs to be one of the aforementioned accepted licenses.
