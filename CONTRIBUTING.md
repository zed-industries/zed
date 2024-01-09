# CONTRIBUTING

Thanks for your interest in contributing to Zed, the collaborative platform that is also a code editor!

We want to ensure that no one ends up spending time on a pull request that may not be accepted, so we ask that you discuss your ideas with the team and community before starting on a contribution.

All activity in Zed communities is subject to our [Code of Conduct](https://docs.zed.dev/community/code-of-conduct). Contributors to Zed must sign our [Contributor License Agreement TODO](LINK) before their contributions can be merged.

## Contribution ideas

If you already have an idea of what you'd like to contribute, you can skip this section, otherwise, here are a few resources to help you find something to work on:

- Our [public roadmap TODO](LINK) details what features we plan to add to Zed.
- Our [Top-Ranking Issues issue](https://github.com/zed-industries/community/issues/52) shows the most popular feature requests and issues, as voted on by the community.

At the moment, we are generally not looking to extend Zed's language or theme support by directly adding these features to Zed - we really want to build a plugin system to handle making the editor extensible going forward. This isn't to say that we won't accept contributions that add support for a new language or theme, but more to emphasize that we want to discuss these types of contributions first.

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

## Zed channels

Once you have an idea of what you'd like to contribute, you'll want to communicate this to the team. If you're new to Zed's channels, here's a guide [link to up-to-date docs TODO](LINK) to help bring you up to speed.

[Since ~February 2022, the Zed Industries team has been exclusively using Zed to build Zed](https://x.com/nathansobo/status/1497958891509932035). We've built these tools to specifically address our own issues and frustrations with the current state of collaborative coding. These are not features we've built to simply look flashy, we work in channels every day of the workweek, aggressively dogfooding everything.

![Staff usage of channels](./assets/screenshots/staff_usage_of_channels.png)

*Channel metrics were not collected prior to August 2023*

While we still have improvements to make, we believe we've sanded down a lot of the sharp edges and that the experience is both smooth and enjoyable - one that gets you as close to hypothetically sitting next to your teammates as possible, even if you're potentially on different sides of the globe. We want to continue working this way amongst ourselves and we are extremely excited to work with *you* in this way. We invite you to contribute to Zed *through* Zed.

We plan to organize office hours on a weekly basis - they will take place in forelinked Zed channel.

### Proposal & Discussion

Before starting on a contribution, we ask that you look to see if there is any existing PRs, or in-Zed discussions about the thing you want to implement. If there is no existing work, find a [public channel TODO](LINK) that is relevant to your contribution, check the channel notes to see which Zed team members typically work in that channel, and post a message in the chat. If you're not sure which channel is best, you can post in the <whatever-general> channel.

*Please wait to begin working on your contribution until you've received feedback from the team. Turning down a contribution that was not discussed beforehand is a bummer for everyone.*

## Implementation & Help

Once approved, feel free to begin working on your contribution. If you have any questions, you can post in the channel you originally proposed your contribution in, or you can post in the <TODO-whatever-general> channel. If you need help, reach out to a Zed teammate - we're happy to pair with you to help you learn the codebase and get your contribution merged.

**Zed makes heavy use of unit and integration testing, we encourage you to write tests for your contribution.**

Reviewing code in a pull request, after the fact, is hard and tedious - the team generally likes to build trust and review code through pair programming. We'd prefer have conversations about the code, through Zed, while it is being written, so decisions can be made in real-time and less time is spent on fixing things after the fact. Ideally, GitHub is only used to merge code that has already been discussed and reviewed in Zed.
