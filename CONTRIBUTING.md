## Contributing to Zed
Thanks for your interest in contributing to Zed, the collaborative platform that is also a code editor!

Read on if you're looking for an outline of your first contribution - from finding your way around the codebase and asking questions, through modifying and testing the changes, finishing off with submitting your changes for review and interacting with Zed core team and Zed community as a whole.

### Getting in touch
We believe that journeys are best when shared - hence there are multiple outlets for Zed users and developers to share their success stories and hurdles.

If you have questions, ask them away on our [Discord](https://discord.gg/XTtXmZYEpN) or in a dedicated [Zed channel](https://zed.dev/preview/channel/open-source-81). We also plan to organise office hours on a weekly basis - they will take place in forelinked Zed channel.

All activity in Zed communities is subject to our [Code of Conduct](https://docs.zed.dev/community/code-of-conduct).

### Bird-eye's view of Zed
Zed is made up of several smaller crates - let's go over those you're most likely to interact with:
- [gpui](/crates/gpui) is a GPU-accelerated UI framework which provides all of the building blocks for Zed.
- [editor](/crates/editor) contains the core `Editor` type that drives both the code editor and all various input fields within Zed. It also handles a display layer for LSP features such as Inlay Hints or code completions.
- [project](/crates/project) manages files and navigation within the filetree. It is also Zed's side of communication with LSP.
- [workspace](/crates/workspace) handles local state serialization and groups projects together.
- [vim](/crates/vim) is a thin implementation of Vim workflow over `editor`.
- [lsp](/crates/lsp) handles communication with external LSP server.
- [language](/crates/language) drives `editor`'s understanding of language - from providing a list of symbols to the syntax map.
- [collab](/crates/collab) is the collaboration server itself, driving the collaboration features such as project sharing.
- [rpc](/crates/rpc) defines messages to be exchanged with collaboration server.


### Upstreaming your changes
Here be dragons :)
