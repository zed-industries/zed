#

Documentation Gaps from December 2025 Releases

This document identifies features from the December 2025 releases (0.216.x and 0.217.x) that are missing or insufficiently documented.

## Git Features (High Priority)

| Feature                                              | Release | PR                                                                                                                     | Notes                                                                                                                                |
| ---------------------------------------------------- | ------- | ---------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------ |
| **File History view**                                | 0.216.0 | [#42441](https://github.com/zed-industries/zed/pull/42441), [#44016](https://github.com/zed-industries/zed/pull/44016) | No documentation for this major new feature that shows commit history for individual files (accessible via right-click context menu) |
| **UI for deleting Git branches**                     | 0.216.0 | [#42703](https://github.com/zed-industries/zed/pull/42703)                                                             | Not documented in `git.md`                                                                                                           |
| **Word diff highlighting**                           | 0.216.0 | [#43269](https://github.com/zed-industries/zed/pull/43269)                                                             | The `word_diff_enabled` language setting is not documented in `git.md` or settings reference                                         |
| **Git remotes support**                              | 0.217.1 | [#42819](https://github.com/zed-industries/zed/pull/42819)                                                             | Not mentioned in `git.md`                                                                                                            |
| **Git pushRemote/pushDefault configuration**         | 0.216.0 | [#41700](https://github.com/zed-industries/zed/pull/41700)                                                             | Git will check pushRemote and pushDefault before falling back to branch remote - not documented                                      |
| **Branch names on git conflict buttons**             | 0.217.1 | [#44421](https://github.com/zed-industries/zed/pull/44421)                                                             | Changed from HEAD/ORIGIN to branch names - not documented                                                                            |
| **Self-hosted git provider / Bitbucket integration** | 0.217.1 | [#42343](https://github.com/zed-industries/zed/pull/42343)                                                             | Improved support not fully documented                                                                                                |

## AI Features

| Feature                                                | Release          | PR                                                                                                                     | Notes                                                                                                  |
| ------------------------------------------------------ | ---------------- | ---------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------ |
| **Automatic file context detection when pasting code** | 0.217.1          | [#42982](https://github.com/zed-industries/zed/pull/42982)                                                             | Partially documented in `agent-panel.md`, but the collapsible badges feature isn't explained in detail |
| **Grok 4.1 Fast models (2M context, vision)**          | 0.217.1          | [#43419](https://github.com/zed-industries/zed/pull/43419)                                                             | The xAI section in `llm-providers.md` doesn't mention Grok 4.1 Fast or the 2M token context            |
| **GPT-5.2 support**                                    | 0.216.1          | [#44656](https://github.com/zed-industries/zed/pull/44656)                                                             | Not explicitly mentioned in the models documentation                                                   |
| **Extension-provided agents display names**            | 0.217.1, 0.216.1 | [#44496](https://github.com/zed-industries/zed/pull/44496), [#44660](https://github.com/zed-industries/zed/pull/44660) | Not documented                                                                                         |
| **Bedrock `allow_global` option**                      | 0.217.1          | [#44103](https://github.com/zed-industries/zed/pull/44103)                                                             | Cross-region inference global endpoints not documented                                                 |

## Editor/Actions Features

| Feature                                                          | Release | PR                                                         | Notes                                                                                     |
| ---------------------------------------------------------------- | ------- | ---------------------------------------------------------- | ----------------------------------------------------------------------------------------- |
| **Command palette history**                                      | 0.217.1 | [#44517](https://github.com/zed-industries/zed/pull/44517) | `command-palette.md` doesn't mention you can press `up` to see recently executed commands |
| **`RotateSelectionsForward`/`RotateSelectionsBackward` actions** | 0.217.1 | [#41236](https://github.com/zed-industries/zed/pull/41236) | Not documented in any editing docs                                                        |
| **`editor::InsertSnippet` action**                               | 0.217.1 | [#44428](https://github.com/zed-industries/zed/pull/44428) | Not documented in `snippets.md`                                                           |
| **`workspace::ZoomIn`/`workspace::ZoomOut` actions**             | 0.216.0 | [#44587](https://github.com/zed-industries/zed/pull/44587) | Only `ToggleZoom` is mentioned; these new actions aren't documented                       |
| **Force Touch support for go-to-definition (macOS)**             | 0.216.0 | [#40399](https://github.com/zed-industries/zed/pull/40399) | Not documented                                                                            |
| **Scroll keybindings for OutlinePanel**                          | 0.216.0 | [#42438](https://github.com/zed-industries/zed/pull/42438) | Not documented                                                                            |

## Remote Development

| Feature                                     | Release | PR                                                         | Notes                                     |
| ------------------------------------------- | ------- | ---------------------------------------------------------- | ----------------------------------------- |
| **`connection_timeout` SSH setting**        | 0.216.0 | [#44823](https://github.com/zed-industries/zed/pull/44823) | Not documented in `remote-development.md` |
| **10s connect timeout for server download** | 0.217.1 | [#44216](https://github.com/zed-industries/zed/pull/44216) | Not documented                            |
| **SSH hostname display in Recent Projects** | 0.217.1 | [#44349](https://github.com/zed-industries/zed/pull/44349) | Not documented                            |

## UI/Settings Features

| Feature                                            | Release | PR                                                         | Notes                                                                      |
| -------------------------------------------------- | ------- | ---------------------------------------------------------- | -------------------------------------------------------------------------- |
| **Overhauled preview tabs settings**               | 0.217.1 | [#43921](https://github.com/zed-industries/zed/pull/43921) | The settings reference has the settings, but the overhaul isn't called out |
| **`show_user_menu` setting**                       | 0.216.0 | [#44466](https://github.com/zed-industries/zed/pull/44466) | Documented in `all-settings.md`, but not prominently highlighted           |
| **`launchpad` value for `restore_on_startup`**     | 0.216.0 | [#44048](https://github.com/zed-industries/zed/pull/44048) | ✅ Documented in settings reference                                        |
| **`zed --wait` working with directories**          | 0.216.0 | [#44936](https://github.com/zed-industries/zed/pull/44936) | Documented for files but not explicitly for directories                    |
| **Standardized `cmd-o`/`cmd-k cmd-o` keybindings** | 0.216.0 | [#44598](https://github.com/zed-industries/zed/pull/44598) | Open file vs open folder keybindings not documented                        |

## Language-Specific Features

| Feature                                            | Release | PR                                                         | Notes                                          |
| -------------------------------------------------- | ------- | ---------------------------------------------------------- | ---------------------------------------------- |
| **Doxygen grammar support for C/C++**              | 0.216.0 | [#43581](https://github.com/zed-industries/zed/pull/43581) | Not documented in C/C++ language docs          |
| **Tailwind support for Gleam**                     | 0.216.0 | [#43968](https://github.com/zed-industries/zed/pull/43968) | Not documented in Gleam language docs          |
| **ESLint working directories configuration**       | 0.216.0 | [#43677](https://github.com/zed-industries/zed/pull/43677) | ✅ Documented in `javascript.md`               |
| **On-type formatting with newlines**               | 0.216.0 | [#44882](https://github.com/zed-industries/zed/pull/44882) | Only a setting reference exists, not explained |
| **JavaScript highlighting in GitHub Actions YAML** | 0.216.0 | [#43771](https://github.com/zed-industries/zed/pull/43771) | Not documented                                 |

## Helix Mode

| Feature                                                  | Release | PR                                                         | Notes                        |
| -------------------------------------------------------- | ------- | ---------------------------------------------------------- | ---------------------------- |
| **`space /` keybinding for global search**               | 0.216.0 | [#43363](https://github.com/zed-industries/zed/pull/43363) | Not documented in `helix.md` |
| **Custom diff/git-related actions in goto mode**         | 0.216.0 | [#45006](https://github.com/zed-industries/zed/pull/45006) | Not documented               |
| **Visual line movement changes for j/k/Up/Down**         | 0.216.0 | [#42676](https://github.com/zed-industries/zed/pull/42676) | Not documented               |
| **Search category and non-Helix binding clarifications** | 0.216.0 | [#43735](https://github.com/zed-industries/zed/pull/43735) | Not documented               |

## Priority Recommendations

The most impactful documentation gaps to address:

1. **File History view** ([#42441](https://github.com/zed-industries/zed/pull/42441), [#44016](https://github.com/zed-industries/zed/pull/44016)) - This is a significant new feature with no documentation
2. **Git branch deletion UI** ([#42703](https://github.com/zed-industries/zed/pull/42703)) - Users don't know this exists
3. **Word diff highlighting setting** ([#43269](https://github.com/zed-industries/zed/pull/43269)) - New configurable feature with no docs
4. **Command palette history** ([#44517](https://github.com/zed-industries/zed/pull/44517)) - Simple but very useful feature that should be in `command-palette.md`
5. **Grok 4.1 Fast / extended context models** ([#43419](https://github.com/zed-industries/zed/pull/43419)) - AI model docs are incomplete
