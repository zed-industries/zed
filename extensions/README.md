# Zed Official Extensions

This directory contains some official Zed extensions.

See also [zed-industries/extensions](https://github.com/zed-industries/extensions) repo.

## Structure

Currently Zed includes support for a number of languages without requiring installing an extension. Those languages can be found under [crates/languages/src](https://github.com/zed-industries/zed/tree/main/crates/languages/src).

Support for all other languages is done via Extensions.  This directory ([extensions/](https://github.com/zed-industries/zed/tree/main/extensions/)) contains a number of officially maintained extensions. These extensions use the same [zed_extension_api](https://docs.rs/zed_extension_api/latest/zed_extension_api/) available to all [Zed Extensions](https://zed.dev/extensions) for providing [language servers](https://zed.dev/docs/extensions/languages#language-servers), [tree-sitter grammars](https://zed.dev/docs/extensions/languages#grammar) and [tree-sitter queries](https://zed.dev/docs/extensions/languages#tree-sitter-queries).

## Dev-Extensions

If you would like to modify one of these extensions:

1. [Install Rust](https://www.rust-lang.org/tools/install)
2. Checkout the zed repo: `git clone https://github.com/zed-industries/zed.git`
3. Open Zed and got the Extensions page (`cmd-shift-x` or `ctrl-shift-x`)
4. Click "Install Dev Extension"
5. Select the folder of the extension you want to modify, e.g. `extensions/lua` for the Lua extension.

Make your changes and then click "Rebuild" next to your installed Dev Extension. Repeat.

## Updating

> [!NOTE]
> This update process is usually handled by Zed Employees.
> Community contributors should just submit a PR (step 1) and we'll take it from there.

The process for updating an extension in this directory has three parts.

1. Create a PR with your changes. (Merge it)

2. Bumps the extension version in:
- extensions/{language_name}/extension.toml
- extensions/{language_name}/Cargo.toml
- Cargo.lock

You can do this manually, or with a script:
```sh
# outputs the current version for a given language
./script/language-extension-version <langname>

# update extension.toml/Cargo.toml and trigger cargo update
./script/language-extension-version <langname> <new_version>
```
Commit your changes to a branch, push a PR and merge it.

3. Copy the commit id from your squashed PR merge and create a PR against the [zed-industries/extensions](https://github.com/zed-industries/extensions) repo which updates the extension in question.

Edit [extensions.toml](https://github.com/zed-industries/extensions/blob/main/extensions.toml) in the extensions repo to reflect the new version you set above and update the submodule to the zed repo commit id above.

```sh
# Go into your clone of the extensions repo
cd ../extensions

# update
git checkout main
git pull
git submodule update

# update the zed submodule
cd extensions/zed
git fetch
git checkout <commit-id>
cd ..
git checkout -b bump_etc_etc
git add extensions.toml extensions/zed
```

When looking at your extensions repo PR, see which files in the zed submodule have been changed, specifically other extensions which may have had changes that have not yet been bundled into new versions for release.
