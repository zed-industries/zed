A conflict-free replicated database.

Our goal is for this database to contain all the text inserted in Zed.

The database is divided into *contexts*, with each context containing a collection of *documents*.

These contexts and the documents are really just namespaces in a global table of document *fragments*. Each fragment is a sequence of one or more characters, which may or may not be visible in a given branch.

There are two different kinds of contexts:

Worktrees: A worktree is assumed to contain many documents with paths relative to some shared root. Worktrees can be used to collaborate on source code, but also documentation in a variety of formats, such as markdown, or other page generation formats.

Channels: Worktrees are associated with one or more channels, and channels are associated with zero or more worktrees. Channels are named by paths in a hierarchical namespace.

The channel namespace is also the top-level namespace for zed.dev URLs.

https://zed.dev/zed -> The #zed channel.
https://zed.dev/zed/insiders -> The #zed/insiders channel.
