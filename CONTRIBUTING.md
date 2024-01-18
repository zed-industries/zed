# CONTRIBUTING

Thanks for your interest in contributing to Zed, the collaborative platform that is also a code editor!

We want to ensure that no one ends up spending time on a pull request that may not be accepted, so we ask that you discuss your ideas with the team and community before starting on a contribution.

All activity in Zed communities is subject to our [Code of Conduct](https://docs.zed.dev/community/code-of-conduct). Contributors to Zed must sign our Contributor License Agreement (link coming soon) before their contributions can be merged.

## Contribution ideas

If you already have an idea of what you'd like to contribute, you can skip this section, otherwise, here are a few resources to help you find something to work on:

- Our public roadmap (link coming soon!) details what features we plan to add to Zed.
- Our [Top-Ranking Issues issue](https://github.com/zed-industries/community/issues/52) shows the most popular feature requests and issues, as voted on by the community.

At the moment, we are generally not looking to extend Zed's language or theme support by directly adding these features to Zed - we really want to build a plugin system to handle making the editor extensible going forward.

If you are passionate about shipping new languages or themes we suggest contributing to the extension system to help us get there faster.

## Resources

### Bird-eye's view of Zed

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

### Proposal & Discussion

Before starting on a contribution, we ask that you look to see if there is any existing PRs, or in-Zed discussions about the thing you want to implement. If there is no existing work, find a public channel that is relevant to your contribution, check the channel notes to see which Zed team members typically work in that channel, and post a message in the chat. If you're not sure which channel is best, you can start a discussion, ask a team member or another contributor.

*Please remember contributions not discussed with the team ahead of time likely have a lower chance of being merged or looked at in a timely manner.*

## Implementation & Help

When you start working on your contribution if you find you are struggling with something specific feel free to reach out to the team for help.

Remember the team is more likely to be available to help if you have already discussed your contribution or are working on something that is higher priority, like something on the roadmap or a top-ranking issue.

We're happy to pair with you to help you learn the codebase and get your contribution merged.

**Zed makes heavy use of unit and integration testing, it is highly likely that contributions without any unit tests will be rejected**

Reviewing code in a pull request, after the fact, is hard and tedious - the team generally likes to build trust and review code through pair programming.
We'd prefer have conversations about the code, through Zed, while it is being written, so decisions can be made in real-time and less time is spent on fixing things after the fact. Ideally, GitHub is only used to merge code that has already been discussed and reviewed in Zed.

Remember that smaller, incremental PRs are easier to review and merge than large PRs.
