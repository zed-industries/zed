# Contributing to Zed

Thank you for helping us make Zed better!

## Before you start

> **At most three open PRs per author.** Start with one and see it through.
> Landing your first PR improves the odds for the next. A stack of open PRs gets stale as `main` changes and can overwhelm review, especially when submissions are automated.

- **Fixing a bug or improving docs?** A pull request is the best place to start.
- **Adding a feature? Talk to us before you start.** If there isn't a GitHub issue with staff confirmation that we want it, start a [GitHub discussion](https://github.com/zed-industries/zed/discussions), not a PR or a new issue. This especially applies to changes to the Zed Extension API.
  - For larger features, read the [Zed Feature Process](./docs/src/development/feature-process.md) before writing a proposal. It covers the context, integration points, and design decisions a strong proposal needs.
- **Keep each PR about one thing.** If you're fixing a bug, save unrelated features and refactoring for another PR.
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

**Build and run Zed locally before opening a PR.** Follow the guide for [macOS](./docs/src/development/macos.md), [Linux](./docs/src/development/linux.md), [Windows](./docs/src/development/windows.md), or [FreeBSD](./docs/src/development/freebsd.md), and try your changes in your local build.

For help finding your way around the code, see the [codebase overview](https://zed.dev/docs/development#birds-eye-view-of-zed).

If you need help fixing a bug or implementing a feature we've agreed we want, **open a PR early**. You don't need a finished patch for us to work through it together.

When preparing your PR:

- **Explain the problem and why it matters**, then describe your solution.
- **Include tests.** For UI changes, consider updating [visual regression tests](./docs/src/development/macos.md#visual-regression-tests).
- **Show visible changes** with screenshots or screen recordings, and work through the [UI/UX checklist](./docs/src/development/ui-checklist.md).
- **Review your own diff**, including any AI-assisted code, for quality, security, reliability, and performance.

### What to expect from review

**Opening a PR does not guarantee a merge.** We may decline a change that doesn't fit Zed's direction or quality standards, even after you've put work into it. Confirming interest early and following this guide give your PR the best chance.

We value working code and synchronous conversations over long discussion threads. Respond to GitHub comments, or offer time to pair if you need more feedback.

We'll get back to you, though sometimes more slowly than we'd like. **Pinging maintainers by username or emailing them does not raise your PR's priority**; it takes time away from review.

## Contribution ideas

**Looking for a place to start?** Browse issues considered suitable for [first-time contributors](https://github.com/zed-industries/zed/issues?q=is%3Aissue%20state%3Aopen%20label%3A%22.contrib%2Fgood%20first%20issue%22) or [returning contributors](https://github.com/zed-industries/zed/issues?q=state%3Aopen%20label%3A%22.contrib%2Fgood%20non-first%20issue%22).

We spend most of our time on Zed's core priorities, but welcome community improvements we haven't thought of or had time to tackle. In particular, we love PRs that:

- **Fix or extend the docs.** Browse [docs issues](https://github.com/zed-industries/zed/issues?q=is%3Aissue%20state%3Aopen%20type%3ADocs).
- **Fix bugs.** Start with [triaged bugs with confirmed reproduction steps](https://github.com/zed-industries/zed/issues?q=is%3Aissue%20state%3Aopen%20type%3ABug%20label%3Astate%3Areproducible), or browse [area labels](https://github.com/zed-industries/zed/labels?q=area%3A*) for parts of Zed you care about. Select a label, then add `type:Bug` to the search.
- **Make existing features work for more people** through small enhancements, such as support for more platforms or modes.
- **Add small features**, like keybindings or actions you miss from other editors or extensions.
- **Join a Community Program** like [Let's Git Together](https://github.com/zed-industries/zed/issues/41541) or [The Guild](https://zed.dev/community/guild).
- **Build features we've explicitly invited contributions for.** Find them on the [community feature board](https://github.com/orgs/zed-industries/projects/78/views/4).

## UI/UX checklist

For UI changes, use the [UI/UX checklist](./docs/src/development/ui-checklist.md) in the development docs.

## Bird's-eye view of Zed

For a tour of the main crates, see the [codebase overview](https://zed.dev/docs/development#birds-eye-view-of-zed) in the development docs.

## Packaging Zed

Check our [notes for packaging Zed](https://zed.dev/docs/development/linux#notes-for-packaging-zed).
