# Contributing to Zed

Thanks for your interest in contributing to Zed, the collaborative platform that is also a code editor!

We want to avoid anyone spending time on a pull request that may not be accepted, so we suggest you discuss your ideas with the team and community before starting on major changes. Bug fixes, however, are almost always welcome.

All activity in Zed forums is subject to our [Code of Conduct](https://docs.zed.dev/community/code-of-conduct). Additionally, contributors must sign our [Contributor License Agreement](https://zed.dev/cla) before their contributions can be merged.

## Contribution ideas

If you're looking for ideas about what to work on, check out:

- Our public roadmap (link coming soon!) contains a rough outline of our near-term priorities for Zed.
- Our [top-ranking issues](https://github.com/zed-industries/community/issues/52) based on votes by the community.

Outside of a handful of extremely popular languages and themes, we are generally not looking to extend Zed's language or theme support by directly building them into Zed. We really want to build a plugin system to handle making the editor extensible going forward. If you are passionate about shipping new languages or themes we suggest contributing to the extension system to help us get there faster.

## Proposing changes

The best way to propose a change is to [start a discussion on our GitHub repository](https://github.com/zed-industries/zed/discussions).

First, write a short **problem statement**, which *clearly* and *briefly* describes the problem you want to solve independently from any specific solution. It doesn't need to be long or formal, but it's difficult to consider a solution in absence of a clear understanding of the problem.

Next, write a short **solution proposal**. How can the problem (or set of problems) you have stated above be addressed? What are the pros and cons of your approach? Again, keep it brief and informal. This isn't a specification, but rather a starting point for a conversation.

By effectively engaging with the Zed team and community early in your process, we're better positioned to give you feedback and understand your pull request once you open it. If the first thing we see from you is a big changeset, we're much less likely to respond to it in a timely manner.

## Pair programming

We plan to set aside time each week to pair program with contributors on promising pull requests in Zed. This will be an experiment. We tend to prefer pairing over async code review on our team, and we'd like to see how well it works in an open source setting. If we're finding it difficult to get on the same page with async review, we may ask you to pair with us if you're open to it. The closer a contribution is to the goals outlined in our roadmap, the more likely we'll be to spend time pairing on it.

## Tips to improve the chances of your PR getting reviewed and merged

- Discuss your plans ahead of time with the team
- Small, focused, incremental pull requests are much easier to review
- Spend time explaining your changes in the pull request body
- Add test coverage and documentation
- Choose tasks that align with our roadmap
- Pair with us and watch us code to learn the codebase

## Bird-eye's view of Zed

Zed is made up of several smaller crates - let's go over those you're most likely to interact with:

- [gpui](/crates/gpui) is a GPU-accelerated UI framework which provides all of the building blocks for Zed. **We recommend familiarizing yourself with the root level GPUI documentation**
- [editor](/crates/editor) contains the core `Editor` type that drives both the code editor and all various input fields within Zed. It also handles a display layer for LSP features such as Inlay Hints or code completions.
- [project](/crates/project) manages files and navigation within the filetree. It is also Zed's side of communication with LSP.
- [workspace](/crates/workspace) handles local state serialization and groups projects together.
- [vim](/crates/vim) is a thin implementation of Vim workflow over `editor`.
- [lsp](/crates/lsp) handles communication with external LSP server.
- [language](/crates/language) drives `editor`'s understanding of language - from providing a list of symbols to the syntax map.
- [collab](/crates/collab) is the collaboration server itself, driving the collaboration features such as project sharing.
- [rpc](/crates/rpc) defines messages to be exchanged with collaboration server.
- [theme](/crates/theme) defines the theme system and provides a default theme.
- [ui](/crates/ui) is a collection of UI components and common patterns used throughout Zed.
