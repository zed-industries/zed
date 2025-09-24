# Contributing to Zed

Thank you for helping us make Zed better!

All activity in Zed forums is subject to our [Code of
Conduct](https://zed.dev/code-of-conduct). Additionally, contributors must sign
our [Contributor License Agreement](https://zed.dev/cla) before their
contributions can be merged.

## Contribution ideas

Zed is a large project with a number of priorities. We spend most of
our time working on what we believe the product needs, but we also love working
with the community to improve the product in ways we haven't thought of (or had time to get to yet!)

In particular we love PRs that are:

- Fixes to existing bugs and issues.
- Small enhancements to existing features, particularly to make them work for more people.
- Small extra features, like keybindings or actions you miss from other editors or extensions.
- Work towards shipping larger features on our roadmap.

If you're looking for concrete ideas:

- Our [top-ranking issues](https://github.com/zed-industries/zed/issues/5393) based on votes by the community.
- Our [public roadmap](https://zed.dev/roadmap) contains a rough outline of our near-term priorities for Zed.

## Sending changes

The Zed culture values working code and synchronous conversations over long
discussion threads.

The best way to get us to take a look at a proposed change is to send a pull
request. We will get back to you (though this sometimes takes longer than we'd
like, sorry).

Although we will take a look, we tend to only merge about half the PRs that are
submitted. If you'd like your PR to have the best chance of being merged:

- Include a clear description of what you're solving, and why it's important to you.
- Include tests.
- If it changes the UI, attach screenshots or screen recordings.

The internal advice for reviewers is as follows:

- If the fix/feature is obviously great, and the code is great. Hit merge.
- If the fix/feature is obviously great, and the code is nearly great. Send PR comments, or offer to pair to get things perfect.
- If the fix/feature is not obviously great, or the code needs rewriting from scratch. Close the PR with a thank you and some explanation.

If you need more feedback from us: the best way is to be responsive to
Github comments, or to offer up time to pair with us.

If you are making a larger change, or need advice on how to finish the change
you're making, please open the PR early. We would love to help you get
things right, and it's often easier to see how to solve a problem before the
diff gets too big.

## Things we will (probably) not merge

Although there are few hard and fast rules, typically we don't merge:

- Anything that can be provided by an extension. For example a new language, or theme. For adding themes or support for a new language to Zed, check out our [docs on developing extensions](https://zed.dev/docs/extensions/developing-extensions).
- New file icons. Zed's default icon theme consists of icons that are hand-designed to fit together in a cohesive manner, please don't submit PRs with off-the-shelf SVGs.
- Giant refactorings.
- Non-trivial changes with no tests.
- Features where (in our subjective opinion) the extra complexity isn't worth it for the number of people who will benefit.
- Anything that seems completely AI generated.

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
