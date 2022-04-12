# Zed

[![CI](https://github.com/zed-industries/zed/actions/workflows/ci.yml/badge.svg)](https://github.com/zed-industries/zed/actions/workflows/ci.yml)

Welcome to Zed, a lightning-fast, collaborative code editor that makes your dreams come true.

## Development tips

### Testing against locally-running servers

Make sure you have `zed.dev` cloned as a sibling to this repo.

```
cd ..
git clone https://github.com/zed-industries/zed.dev
```

Make sure your local database is created, migrated, and seeded with initial data. Install [Postgres](https://postgresapp.com), then from the `zed` repository root, run:

```
script/sqlx database create
script/sqlx migrate run
script/seed-db
```

Run the web frontend and the collaboration server.

```
brew install foreman
foreman start
```

If you want to run Zed pointed at the local servers, you can run:

```
script/zed_with_local_servers
# or...
script/zed_with_local_servers --release
```

### Dump element JSON

If you trigger `cmd-alt-i`, Zed will copy a JSON representation of the current window contents to the clipboard. You can paste this in a tool like [DJSON](https://chrome.google.com/webstore/detail/djson-json-viewer-formatt/chaeijjekipecdajnijdldjjipaegdjc?hl=en) to navigate the state of on-screen elements in a structured way.

## Roadmap

We will organize our efforts around the following major milestones. We'll create tracking issues for each of these milestones to detail the individual tasks that comprise them.

### Minimal text editor

[Tracking issue](https://github.com/zed-industries/zed/issues/2)

Ship a minimal text editor to investors and other insiders. It should be extremely fast and stable, but all it can do is open, edit, and save text files, making it potentially useful for basic editing but not for real coding.

Establish basic infrastructure for building the app bundle and uploading an artifact. Once this is released, we should regularly distribute updates as features land.

### Collaborative code editor for internal use

[Tracking issue](https://github.com/zed-industries/zed/issues/6)

Turn the minimal text editor into a collaborative _code_ editor. This will include the minimal features that the Zed team needs to collaborate in Zed to build Zed without net loss in developer productivity. This includes productivity-critical features such as:

- Syntax highlighting and syntax-aware editing and navigation
- The ability to see and edit non-local working copies of a repository
- Language server support for Rust code navigation, refactoring, diagnostics, etc.
- Project browsing and project-wide search and replace

We want to tackle collaboration fairly early so that the rest of the design of the product can flow around that assumption. We could probably produce a single-player code editor more quickly, but at the risk of having collaboration feel more "bolted on" when we eventually add it.

### Private alpha for Rust teams on macOS

The "minimal" milestones were about getting Zed to a point where the Zed team could use Zed productively to build Zed. What features are required for someone outside the company to use Zed to productively work on another project that is also written in Rust?

This includes infrastructure like auto-updates, error reporting, and metrics collection. It also includes some amount of polish to make the tool more discoverable for someone that didn't write it, such as a UI for updating settings and key bindings. We may also need to enhance the server to support user authentication and related concerns.

The initial target audience is like us. A small team working in Rust that's potentially interested in collaborating. As the alpha proceeds, we can work with teams of different sizes.

### Private beta for Rust teams on macOS

Once we're getting sufficiently positive feedback from our initial alpha users, we widen the audience by letting people share invites. Now may be a good time to get Zed running on the web, so that it's extremely easy for a Zed user to share a link and be collaborating in seconds. Once someone is using Zed on the Web, we'll let them register for the private beta and download the native binary if they're on macOS.

### Expand to other languages

Depending on how the Rust beta is going, focus hard on dominating another niche language such as Elixr or getting a foothold within a niche of a larger language, such as React/Typescript. Alternatively, go wide at this point and add decent support several widely-used languages such as Python, Ruby, Typescript, etc. This would entail taking 1-2 weeks per language and making sure we ship a solid experience based on a publicly-available language server. Each language has slightly different development practices, so we need to make sure Zed's UX meshes well with those practices.

### Future directions

Each of these sections could probably broken into multiple milestones, but this part of the roadmap is too far in the future to go into that level of detail at this point.

#### Expand to other platforms

Support Linux and Windows. We'll probably want to hire at least one person that prefers to work on each respective platform and have them spearhead the effort to port Zed to that platform. Once they've done so, they can join the general development effort while ensuring the user experience stays good on that platform.

#### Expand on collaboration

To start with, we'll focus on synchronous collaboration because that's where we're most differentiated, but there's no reason we have to limit ourselves to that. How can our tool facilitate collaboration generally, whether it's sync or async? What would it take for a team to go 100% Zed and collaborate fully within the tool? If we haven't added it already, basic Git support would be nice.
