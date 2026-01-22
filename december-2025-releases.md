# Zed December 2025 Releases

## 0.217.3

**Dec 18, 2025**

- Agent: Allowed pasting code without formatting via `ctrl/cmd+shift+v`. ([#45254](https://github.com/zed-industries/zed/pull/45254))
- Agent: Fixed an issue where pasting a single line of code would always insert an @mention. ([#45254](https://github.com/zed-industries/zed/pull/45254))
- Fixed certain rust-analyzer snippets not
 shown. ([#45229](https://github.com/zed-industries/zed/pull/45229))
- Fixed bracket colorization not applied on initial file open. ([#45190](https://github.com/zed-industries/zed/pull/45190))

---

## 0.217.2

**Dec 17, 2025**

- Added support for Gemini 3 Flash model. ([#45142](https://github.com/zed-industries/zed/pull/45142))
- Added support for Zed to download beta versions of `ty`, when configured as an enabled language server ([#45080](https://github.com/zed-industries/zed/pull/45080))

---

## 0.217.1

**Dec 16, 2025**

This week's release includes command palette history, git remotes support, branch names on git conflict buttons, automatic file context detection when pasting code into the agent panel, Grok 4.1 Fast models with 2M token context windows and vision support, and new `RotateSelectionsForward`/`RotateSelectionsBackward` actions for rotating content across multiple cursors.

### Features

#### AI

- Agent: Added automatic file context detection when pasting code into the AI agent panel. Pasted code now displays as collapsible badges showing the file path and line numbers (e.g., "app/layout.tsx (18-25)"). ([#42982](https://github.com/zed-industries/zed/pull/42982); thanks ddoemonn)
- Agent: Added support for Grok 4.1 Fast (reasoning and non-reasoning) models in the xAI provider, with 2M token context windows and full vision capabilities. ([#43419](https://github.com/zed-industries/zed/pull/43419); thanks mInrOz)
- Agent: Extended 2M token context to existing Grok 4 Fast variants (from 128K) for consistency with xAI updates. ([#43419](https://github.com/zed-industries/zed/pull/43419); thanks mInrOz)
- Agent: Enabled image/vision support for all Grok 4 family models. ([#43419](https://github.com/zed-industries/zed/pull/43419); thanks mInrOz)
- Agent: Added support for displaying the human-readable names of extension-provided agents in the agent menu. ([#44496](https://github.com/zed-industries/zed/pull/44496))
- Agent: Added fallback to locally installed Codex version if update fails. ([#44419](https://github.com/zed-industries/zed/pull/44419))
- Agent: UI now shows the checked state of a list item. ([#43567](https://github.com/zed-industries/zed/pull/43567); thanks RemcoSmitsDev)
- Edit Prediction: Improved cursor movement performance when edit predictions are enabled. ([#44161](https://github.com/zed-industries/zed/pull/44161))
- Bedrock: Added opt-in `allow_global` which enables global endpoints. ([#44103](https://github.com/zed-industries/zed/pull/44103); thanks 5herlocked)
- Bedrock: Updated cross-region-inference endpoint and model list. ([#44103](https://github.com/zed-industries/zed/pull/44103); thanks 5herlocked)

#### Git

- Changed git conflict buttons to use branch names instead of HEAD and ORIGIN. ([#44421](https://github.com/zed-industries/zed/pull/44421))
- Changed project diff to now focus on merge conflicts for files that have them. ([#44263](https://github.com/zed-industries/zed/pull/44263))
- ⭐Added support for git remotes. ([#42819](https://github.com/zed-industries/zed/pull/42819); thanks bnjjj)
- ⭐Improved self-hosted git provider support and Bitbucket integration. ([#42343](https://github.com/zed-industries/zed/pull/42343); thanks amtoaer)
- ⭐Improved commit view to not show breakpoints on hover. ([#44484](https://github.com/zed-industries/zed/pull/44484); thanks cppcoffee)
- ⭐Fixed an issue where the context menu in the Git Blame view would be frequently overlapped by the commit information tooltip. ([#42764](https://github.com/zed-industries/zed/pull/42764); thanks amustaque97)
- ⭐Fixed editor blame hover not working when inline git blame is disabled. ([#42992](https://github.com/zed-industries/zed/pull/42992); thanks errmayank)
- Commit View: Fixed layout shift that occurred while loading commit. ([#44548
](https://github.com/zed-industries/zed/pull/44548))
- Fixed font feature application for inline git blame, inline diagnostics, markdown popovers and diagnostics popovers. ([#44219](https://github.com/zed-industries/zed/pull/44219))
- Fixed git ignored directories appearing as empty when their content changes on Windows. ([#44143](https://github.com/zed-industries/zed/pull/44143))
- Fixed commit diff multibuffers to now open real project files whenever possible, restoring navigation and annotations inside those excerpts. ([#42558](https://github.com/zed-industries/zed/pull/42558); thanks xipeng-jin)
- Fixed a bug where not all branches were being shown in the branch selector when the search field was empty. ([#44742](https://github.com/zed-industries/zed/pull/44742))

#### Languages

- Markdown Preview: Changed markdown tables to scale based on their content size. ([#43555](https://github.com/zed-industries/zed/pull/43555); thanks RemcoSmitsDev)
- Python: Improved sorting order of toolchains in monorepos with multiple local virtual environments. ([#44141](https://github.com/zed-industries/zed/pull/44141))
- Improved JSONC trailing comma handling. ([#44250](https://github.com/zed-industries/zed/pull/44250))
- Greatly improved the quality of comment-directed language injections in Go. ([#43775](https://github.com/zed-industries/zed/pull/43775); thanks jeffbrennan)
- Improved Proto support to work better out of the box. ([#44440](https://github.com/zed-industries/zed/pull/44440))

#### Other

- Added history to the command palette (`up` will now show recently executed commands). ([#44517](https://github.com/zed-industries/zed/pull/44517))
- Added `RotateSelectionsForward` and `RotateSelectionsBackward` actions that rotate content in a circular fashion across multiple cursors. ([#41236](https://github.com/zed-industries/zed/pull/41236); thanks scorphus)
- Added `editor::InsertSnippet` action. ([#44428](https://github.com/zed-industries/zed/pull/44428))
- Improved Recent Projects picker to now display SSH hostname with remotes. ([#44349](https://github.com/zed-industries/zed/pull/44349); thanks wln)
- Improved visibility of the currently active match when browsing results in buffer or project search. ([#44098](https://github.com/zed-industries/zed/pull/44098))
- Remote Dev: Added 10s connect timeout for server download. ([#44216](https://github.com/zed-industries/zed/pull/44216))
- Remote Dev: Improved resiliency when initialization scripts output text. ([#44165](https://github.com/zed-industries/zed/pull/44165))
- Overhauled preview tabs settings. ([#43921](https://github.com/zed-industries/zed/pull/43921))
- Suppressed warning for trailing commas in builtin JSON files (`settings.json`, `keymap.json`, etc.). ([#43854](https://github.com/zed-industries/zed/pull/43854); thanks ian-h-chamberlain)
- Improved GPU initialization error reporting to be more reliable. ([#44487](https://github.com/zed-industries/zed/pull/44487))

---

## 0.216.1

**Dec 11, 2025**

- Added support for OpenAI's GPT-5.2. ([#44656](https://github.com/zed-industries/zed/pull/44656))
- Render agent display names from extension in menu ([#44660](https://github.com/zed-industries/zed/pull/44660))
- Fixed the default windows keybindings for adding a cursor above and below the current line, from ctrl-shift-alt-up and ctrl-shift-alt-down to ctrl-alt-up and ctrl-alt-down respectively

---

## 0.216.0

**Dec 10, 2025**

This week's release includes a `File History` view for viewing the commit history of individual files, word-based diff highlighting, UI for deleting Git branches, improved pyright/basedpyright completion sorting for Python, and many performance improvements! 🙌

### Features

#### AI

- Agent: Improved delete thread action in the history view by preventing it from also triggering the thread activation
. ([#43796](https://github.com/zed-industries/zed/pull/43796); thanks aeroxy)
- Agent: Made the thread loading state clearer in the agent panel. ([#43765](https://github.com/zed-industries/zed/pull/43765))
- Agent: Added support for deleting your entire thread history. ([#43370](https://github.com/zed-industries/zed/pull/43370); thanks RemcoSmitsDev)
- Agent: Revised tool call description for read file tool to explain outlining behavior. ([#43929](https://github.com/zed-industries/zed/pull/43929))
- Agent: Clarified grep tool description to improve agent precision when using it with the `include_pattern` parameter. ([#41225](https://github.com/zed-industries/zed/pull/41225); thanks procr1337)
- Agent: Removed timeout for agent initialization. ([#44066](https://github.com/zed-industries/zed/pull/44066))
- ACP: Added support for using @mentions after typing slash command. ([#43681](https://github.com/zed-industries/zed/pull/43681))

#### Git

- Added word diff highlighting in expanded diff hunks (less than 5 lines), configurable via the `word_diff_enabled` language setting (defaults to true). ([#43269](https://github.com/zed-industries/zed/pull/43269))
- ⭐Added a `File history` view accessible via right-click context menu on files in the editor, project panel, or git panel. Shows commit history for the selected file with author, timestamp, and commit message. Clicking a commit opens a diff view filtered to show only changes for that specific file. ([#42441](https://github.com/zed-industries/zed/pull/42441), [#44016](https://github.com/zed-industries/zed/pull/44016); thanks ddoemonn)
- ⭐Added UI for deleting Git branches. ([#42703](https://github.com/zed-industries/zed/pull/42703); thanks errmayank)
- Improved performance of multibuffers by spawning git blame processes on the background threads. ([#43918](https://github.com/zed-industries/zed/pull/43918))
- Improved git project diff responsiveness. ([#43706](https://github.com/zed-industries/zed/pull/43706))
- Improved overall git experience when loading buffers with massive git history where they would block other git jobs from running (such as staging/unstaging/committing). Now, git-blame runs separately from the git job queue on the side and the buffer with blame hints when finished thus unblocking other git operations. ([#43565](https://github.com/zed-industries/zed/pull/43565))
- ⭐Git will now check pushRemote and pushDefault configurations before falling back to branch remote. ([#41700](https://github.com/zed-industries/zed/pull/41700); thanks errmayank)
- ⭐Increased the askpass timeout for Git operations from 17 to 300 seconds and improved the error message. ([#42946](https://github.com/zed-industries/zed/pull/42946); thanks 11happy)
- ⭐Fixed a bug where hover tooltips in git commit and blame popovers were not consistently using the UI font. ([#43975](https://github.com/zed-industries/zed/pull/43975); thanks GoldStrikeArch)
- Fixed git features not working when a Windows host collaborates with a Unix guest. ([#43515](https://github.com/zed-industries/zed/pull/43515))
- Fixed git ignored directories appearing as empty when their content changes on Windows. ([#44143](https://github.com/zed-industries/zed/pull/44143))

#### Debugger

- Improved "debug test" experience in Rust with ignored tests. ([#43110](https://github.com/zed-industries/zed/pull/43110); thanks mikeHag)

#### Languages

- Rust: Changed completion tab stops to display inline rather than as a raw LSP snippet expression. ([#43891](https://github.com/zed-industries/zed/pull/43891))
- Python: Improved sorting order of pyright/basedpyright code completions. ([#44050](https://github.com/zed-industries/zed/pull/44050); thanks alkasadist)
- Added JavaScript highlighting via YAML `injections.scm` for script blocks of `actions/github-script`. ([#43771](https://github.com/zed-industries/zed/pull/43771); thanks novusnota)
- Added support for Tailwind suggestions and validations for the Gleam programming language. ([#43968](https://github.com/zed-industries/zed/pull/43968); thanks arjunbajaj)
- Added way to configure ESLint's working directories in settings. ([#43677](https://github.com/zed-industries/zed/pull/43677))
- Added Doxygen grammar support for C/C++ files. ([#43581](https://github.com/zed-industries/zed/pull/43581); thanks Clement-Lap)
- Improved grammar for "Shell Script". ([#44009](https://github.com/zed-industries/zed/pull/44009))
- Changed C preprocessor directives to use `keyword.directive` for syntax highlighting, matching C++ behavior. ([#44043](https://github.com/zed-industries/zed/pull/44043); thanks lipcut)

#### Vim / Helix

- Added Helix's `space /` keybinding to open a global search menu to Zed's Helix mode. ([#43363](https://github.com/zed-industries/zed/pull/43363); thanks godalming123)
- Changed Helix keybinds to use visual line movement for `j`, `Down`, `k`, and `Up`, and textual line movement for `g j`, `g Down`, `g k`, and `g Up`. ([#42676](https://github.com/zed-industries/zed/pull/42676); thanks probablykasper)
- Added custom mappings for Zed-specific diff and git-related actions to Helix's goto mode. ([#45006](https://github.com/zed-industries/zed/pull/45006))
- Improved Helix mode keymaps by adding a Search category and clarifying non-Helix bindings. ([#43735](https://github.com/zed-industries/zed/pull/43735); thanks atahrijouti)

#### Other

- Added ability to open a project in a DevContainer, provided a `.devcontainer/devcontainer.json` is present. ([#44442](https://github.com/zed-industries/zed/pull/44442); thanks KyleBarton)
- Introduced worktree trust mechanism, can be turned off with `"session": { "session": { "trust_all_worktrees": true }}`.
- Improved link parsing for cases when a link is embedded in parentheses, e.g. Markdown. ([#44733](https://github.com/zed-industries/zed/pull/44733); thanks KyleBarton)
- Improved path and rendering performance. ([#44655](https://github.com/zed-industries/zed/pull/44655); thanks marcocondrache)
- Settings UI: Added an "Open Keymap Editor" item under the Keymap section. ([#44914](https://github.com/zed-industries/zed/pull/44914))
- Settings UI: Added a section for configuring edit prediction providers under AI > Edit Predictions, including Codestral and GitHub Copilot. ([#44505](https://github.com/zed-industries/zed/pull/44505))
- Added support for on-type formatting with newlines. ([#44882](https://github.com/zed-industries/zed/pull/44882))
- Implemented the `zed --wait` flag so that it works when opening a directory. The command will block until the window is closed. ([#44936](https://github.com/zed-industries/zed/pull/44936))
- Added scroll keybindings for the OutlinePanel. ([#42438](https://github.com/zed-industries/zed/pull/42438); thanks 0x2CA)
- Added the actions: `workspace::ZoomIn` and `workspace::ZoomOut` that complement the existing `workspace::ToggleZoom` action. ([#44587](https://github.com/zed-industries/zed/pull/44587); thanks pedroni)
- Added a `connection_timeout` setting to specify the SSH connection timeout. ([#44823](https://github.com/zed-industries/zed/pull/44823); thanks marcocondrache)
- Standardized `cmd-o` = open file, `cmd-k, cmd-o` = open folder across operating systems. ([#44598](https://github.com/zed-industries/zed/pull/44598); thanks Zachiah)
- Added a `show_user_menu` setting (defaulting to true) which shows or hides the user menu (the one with the user avatar) in the title bar. ([#44466](https://github.com/zed-industries/zed/pull/44466); thanks notnotjake)
- macOS: Added Force Touch support for go-to-definition. ([#40399](https://github.com/zed-industries/zed/pull/40399); thanks aarol)
- Improved the UI for keymap error messages. ([#42037](https://github.com/zed-industries/zed/pull/42037); thanks johnklucinec)
- Improved LSP notification messages by adding Markdown rendering with clickable URLs, inline code, etc. ([#44215](https://github.com/zed-industries/zed/pull/44215); thanks errmayank)
- Improved new windows spawned from maximized or fullscreen windows by preserving maximized and fullscreen state. ([#44605](https://github.com/zed-industries/zed/pull/44605))
- Improved minimap performance when using custom fonts. ([#46024](https://github.com/zed-industries/zed/pull/46024))
- Added a new value to the `restore_on_startup` setting called `launchpad`. ([#44048](https://github.com/zed-industries/zed/pull/44048); thanks simonpham)
