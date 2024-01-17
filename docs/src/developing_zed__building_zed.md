# Building Zed

🚧 TODO:

- [ ] Remove ZI-specific things
- [ ] Rework any steps that currently require a ZI-specific account

How to build Zed from source for the first time.

### Prerequisites

🚧 TODO 🚧 Update for open source

- Be added to the GitHub organization
- Be added to the Vercel team
- Create a [Personal Access Token](https://github.com/settings/personal-access-tokens/new) on Github
  - 🚧 TODO 🚧 What permissions are required?
  - 🚧 TODO 🚧 What changes when repo isn't private?
  - Go to https://github.com/settings/tokens and Generate new token
  - GitHub currently provides two kinds of tokens:
    - Classic Tokens, where only `repo` (Full control of private repositories) OAuth scope has to be selected
      Unfortunately, unselecting `repo` scope and selecting every its inner scope instead does not allow the token users to read from private repositories
    - (not applicable) Fine-grained Tokens, at the moment of writing, did not allow any kind of access of non-owned private repos
  - Keep the token in the browser tab/editor for the next two steps

### Dependencies

- Install [Rust](https://www.rust-lang.org/tools/install)

- Install the [GitHub CLI](https://cli.github.com/), [Livekit](https://formulae.brew.sh/formula/livekit) & [Foreman](https://formulae.brew.sh/formula/foreman)

```bash
brew install gh
brew install livekit
brew install foreman
```

- Install [Xcode](https://apps.apple.com/us/app/xcode/id497799835?mt=12) from the macOS App Store

- Install [Xcode command line tools](https://developer.apple.com/xcode/resources/)

```bash
xcode-select --install
```

- If `xcode-select --print-path prints /Library/Developer/CommandLineTools…` run `sudo xcode-select --switch /Applications/Xcode.app/Contents/Developer.`

* Install [Postgres](https://postgresapp.com)

* Install the wasm toolchain

```bash
rustup target add wasm32-wasi
```

### Building Zed from Source

1. Clone the `zed` repo

```bash
gh repo clone zed-industries/zed
```

1. (Optional but recommended) Add your GITHUB_TOKEN to your `.zshrc` or `.bashrc` like this: `export GITHUB_TOKEN=yourGithubAPIToken`
1. (🚧 TODO 🚧 - Will this be relevant for open source?) Ensure the Zed.dev website is checked out in a sibling directory and install its dependencies:

```bash
cd ..
git clone https://github.com/zed-industries/zed.dev
cd zed.dev && npm install
pnpm install -g vercel
```

1. (🚧 TODO 🚧 - Will this be relevant for open source?) Link your zed.dev project to Vercel

- `vercel link`
- Select the `zed-industries` team. If you don't have this get someone on the team to add you to it.
- Select the `zed.dev` project

1. (🚧 TODO 🚧 - Will this be relevant for open source?) Run `vercel pull` to pull down the environment variables and project info from Vercel
1. Open Postgres.app
1. From `./path/to/zed/` run `GITHUB_TOKEN={yourGithubAPIToken} script/bootstrap`

- You don't need to include the GITHUB_TOKEN if you exported it above.
- Consider removing the token (if it's fine for you to recreate such tokens during occasional migrations) or store this token somewhere safe (like your Zed 1Password vault).

1. To run the Zed app:
   - If you are working on zed:
     - `cargo run`
   - If you are just using the latest version, but not working on zed:
     - `cargo run --release`
   - If you need to run the collaboration server locally:
     - `script/zed-local`

## Troubleshooting

**`error: failed to run custom build command for gpui v0.1.0 (/Users/path/to/zed)`**

- Try `xcode-select --switch /Applications/Xcode.app/Contents/Developer`

**`xcrun: error: unable to find utility "metal", not a developer tool or in PATH`**

### `script/bootstrap`

```bash
Error: Cannot install in Homebrew on ARM processor in Intel default prefix (/usr/local)!
Please create a new installation in /opt/homebrew using one of the
"Alternative Installs" from:
https://docs.brew.sh/Installation
```

- In that case try `/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"`

- If Homebrew is not in your PATH:
  - Replace `{username}` with your home folder name (usually your login name)
  - `echo 'eval "$(/opt/homebrew/bin/brew shellenv)"' >> /Users/{username}/.zprofile`
  - `eval "$(/opt/homebrew/bin/brew shellenv)"`

```
seeding database...
thread 'main' panicked at 'failed to deserialize github user from 'https://api.github.com/orgs/zed-industries/teams/staff/members': reqwest::Error { kind: Decode, source: Error("invalid type: map, expected a sequence", line: 1, column: 0) }', crates/collab/src/bin/seed.rs:111:10
```

Wrong permissions for `GITHUB_TOKEN` token used, the token needs to be able to read from private repos.
For Classic GitHub Tokens, that required OAuth scope `repo` (seacrh the scope name above for more details)

Same command

`sudo xcode-select --switch /Applications/Xcode.app/Contents/Developer`

### If you experience errors that mention some dependency is using unstable features

Try `cargo clean` and `cargo build`,
