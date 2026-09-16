# Contributing to Zed

Thank you for helping us make Zed better! Start small, pick one thing, and see it through.

## Before you start

- **Fixing a bug or improving docs?** A pull request is the best place to start. Looking for a task? See [contribution ideas](#contribution-ideas).
- **Adding a feature? Confirm interest before investing effort.** If there isn't a GitHub issue with staff confirmation that we want it, start a [GitHub discussion](https://github.com/zed-industries/zed/discussions), not a PR or a new issue. This especially applies to changes to the Zed Extension API.
  - For larger features, read the [Zed Feature Process](./docs/src/development/feature-process.md) before writing a proposal. It covers the context, integration points, and design decisions a strong proposal needs.
- **Keep each PR about one thing.** A bugfix should not arrive with two features and a refactoring in tow.
- **No more than three open PRs per author.** Landing your first PR improves the odds for the next; a stack of open PRs goes stale against a moving `main`. This cap also keeps review manageable when contributions outpace us, including automated bursts.
- **Sign the [Contributor License Agreement](https://zed.dev/cla)** before your contribution can be merged. All activity in Zed forums is subject to our [Code of Conduct](https://zed.dev/code-of-conduct).

Read the [AI policy](#ai-policy) below, then see [sending changes](#sending-changes) for build guides and PR requirements. Working on UI? Keep the [UI/UX checklist](./docs/src/development/ui-checklist.md) handy.

### Things we will (probably) not merge

There are few hard-and-fast rules, but we typically don't merge:

- **Changes that can be provided by an extension**, such as new languages or themes. See the [extension development docs](https://zed.dev/docs/extensions/developing-extensions).
- **Extension API changes without prior discussion** involving Zed staff.
- **New file icons.** Our default icons are hand-designed to fit together; please don't submit off-the-shelf SVGs.
- **Features whose complexity outweighs their benefit**, in our judgement, for the number of people who would use them.
- **Giant refactorings.**
- **Non-trivial changes without tests.**
- **Style-only code changes that don't alter app logic.** Reducing allocations, removing `.unwrap()`s, and fixing typos are welcome; making code "more readable" alone may not be.
- **AI-generated work the author doesn't understand.** You are responsible for the output, not just the prompt.

## AI Policy

We welcome the use of LLMs for coding, but we hold a high bar for all contributions, and **we expect a human in the loop who genuinely understands the work an LLM produces** on their behalf. For that reason, we **don't accept contributions from autonomous agents**. Pull requests that appear to violate this may be closed, sometimes without notice.

**Don't rely on LLMs to write the whole thing for you when communicating with the maintainers** (meaning replies to comments, PR descriptions, and alike). The readers are humans, and we'd like to hear from you, not from a model (we have models at home). If you're a non-native English speaker using an LLM to thoroughly edit or translate your messages to the maintainers, we'd encourage you to **put the machine translation in a quote block and include the original text in your native language after it**.

If you think it's helpful/necessary to **share context from a chat with an LLM**, please put the **relevant part of it** in a quote block (e.g., using `>`), **disclose it as AI-generated**, and add your own commentary explaining **why it's relevant and what you take from it**.

This policy was adapted from [ripgrep's AI policy](https://github.com/BurntSushi/ripgrep/blob/f0cec341ab95c25c691ad3d5754d4bd9eedde21f/AI_POLICY.md).

## Sending changes

**Need a local build?** Follow the guide for [macOS](./docs/src/development/macos.md), [Linux](./docs/src/development/linux.md), [Windows](./docs/src/development/windows.md), or [FreeBSD](./docs/src/development/freebsd.md). For help finding your way around the code, see the [bird's-eye view](#birds-eye-view-of-zed).

You don't need a finished patch to start a useful conversation. If you need help fixing a bug or implementing a feature we've agreed we want, **open a PR early** so we can discuss it with code in hand.

When preparing your PR:

- **Explain the problem and why it matters**, then describe your solution.
- **Include tests.** For UI changes, consider updating [visual regression tests](./docs/src/development/macos.md#visual-regression-tests).
- **Show visible changes** with screenshots or screen recordings, and work through the [UI/UX checklist](./docs/src/development/ui-checklist.md).
- **Review your own work**, including any AI-assisted code. Follow the [pull request template](./.github/pull_request_template.md?plain=1) for testing details and the self-review checklist.

### What to expect from review

**Opening a PR does not guarantee a merge.** We may decline a change that doesn't fit Zed's direction or quality standards, even after you've put work into it. Confirming interest early and following this guide give your PR the best chance.

We value working code and synchronous conversations over long discussion threads. Respond to GitHub comments, or offer time to pair if you need more feedback.

We'll get back to you, though sometimes more slowly than we'd like. **Pinging maintainers by username or emailing them does not raise your PR's priority**; it takes time away from review.

## Contribution ideas

We spend most of our time on Zed's core priorities, but welcome community improvements we haven't thought of or had time to tackle. In particular, we love PRs that:

- **Fix or extend the docs.** Browse [docs issues](https://github.com/zed-industries/zed/issues?q=is%3Aissue%20state%3Aopen%20type%3ADocs).
- **Fix bugs.** Start with [triaged bugs with confirmed reproduction steps](https://github.com/zed-industries/zed/issues?q=is%3Aissue%20state%3Aopen%20type%3ABug%20label%3Astate%3Areproducible), or browse [area labels](https://github.com/zed-industries/zed/labels?q=area%3A*) for parts of Zed you care about. Select a label, then add `type:Bug` to the search.
- **Make existing features work for more people** through small enhancements, such as support for more platforms or modes.
- **Add small features**, like keybindings or actions you miss from other editors or extensions.
- **Join a Community Program** like [Let's Git Together](https://github.com/zed-industries/zed/issues/41541) or [The Guild](https://zed.dev/community/guild).
- **Build features we've explicitly invited contributions for.** Find them on the [community feature board](https://github.com/orgs/zed-industries/projects/78/views/4).

You can also browse tasks for [first-time contributors](https://github.com/zed-industries/zed/issues?q=is%3Aissue%20state%3Aopen%20label%3A%22.contrib%2Fgood%20first%20issue%22) and [returning contributors](https://github.com/zed-industries/zed/issues?q=state%3Aopen%20label%3A%22.contrib%2Fgood%20non-first%20issue%22).

## UI/UX checklist

For UI changes, use the [UI/UX checklist](./docs/src/development/ui-checklist.md) in the development docs.

## Bird's-eye view of Zed

We suggest you keep the [Zed glossary](docs/src/development/glossary.md) at your side when starting out. It lists and explains some of the structures and terms you will see throughout the codebase.

Zed is made up of several smaller crates - let's go over those you're most likely to interact with:

- [`gpui`](/crates/gpui) is a GPU-accelerated UI framework which provides all of the building blocks for Zed. **We recommend familiarizing yourself with the root level GPUI documentation.**
- [`editor`](/crates/editor) contains the core `Editor` type that drives both the code editor and all various input fields within Zed. It also handles a display layer for LSP features such as Inlay Hints or code completions.
- [`project`](/crates/project) manages files and navigation within the filetree. It is also Zed's side of communication with LSP.
- [`workspace`](/crates/workspace) handles local state serialization and groups projects together.
- [`vim`](/crates/vim) is a thin implementation of Vim workflow over `editor`.
- [`lsp`](/crates/lsp) handles communication with external LSP server.
- [`language`](/crates/language) drives `editor`'s understanding of language - from providing a list of symbols to the syntax map.
- [`collab`](/crates/collab) is the collaboration server itself, driving the collaboration features such as project sharing.
- [`rpc`](/crates/rpc) defines messages to be exchanged with collaboration server.
- [`theme`](/crates/theme) defines the theme system and provides a default theme.
- [`ui`](/crates/ui) is a collection of UI components and common patterns used throughout Zed.
- [`cli`](/crates/cli) is the CLI crate which invokes the Zed binary.
- [`zed`](/crates/zed) is where all things come together, and the `main` entry point for Zed.

## Packaging Zed

Check our [notes for packaging Zed](https://zed.dev/docs/development/linux#notes-for-packaging-zed).
