# Zed Extensions

This directory contains extensions for Zed that are largely maintained by the Zed team. They currently live in the Zed repository for ease of maintenance.

If you are looking for the Zed extension registry, see the [`zed-industries/extensions`](https://github.com/zed-industries/extensions) repo.

## Structure

Currently, Zed includes support for a number of languages without requiring installing an extension. Those languages can be found under [`crates/languages/src`](https://github.com/zed-industries/zed/tree/main/crates/languages/src).

Support for all other languages is done via extensions. This directory ([extensions/](https://github.com/zed-industries/zed/tree/main/extensions/)) contains a number of officially maintained extensions. These extensions use the same [zed_extension_api](https://docs.rs/zed_extension_api/latest/zed_extension_api/) available to all [Zed Extensions](https://zed.dev/extensions) for providing [language servers](https://zed.dev/docs/extensions/languages#language-servers), [tree-sitter grammars](https://zed.dev/docs/extensions/languages#grammar) and [tree-sitter queries](https://zed.dev/docs/extensions/languages#tree-sitter-queries).

## Dev Extensions

See the docs for [Developing an Extension Locally](https://zed.dev/docs/extensions/developing-extensions#developing-an-extension-locally) for how to work with one of these extensions.

## Updating

> [!NOTE]
> This update process is usually handled by Zed staff.
> Community contributors should just submit a PR (step 1) and we'll take it from there.

The process for updating an extension in this directory has three parts.

1. Create a PR with your changes. (Merge it)
2. Bump the extension version in:

   - extensions/{language_name}/extension.toml
   - extensions/{language_name}/Cargo.toml
   - Cargo.lock

   You can do this manually, or with a script:

   ```sh
   # Output the current version for a given language
   ./script/language-extension-version <langname>

   # Update the version in `extension.toml` and `Cargo.toml` and trigger a `cargo check`
   ./script/language-extension-version <langname> <new_version>
   ```

   Commit your changes to a branch, push a PR and merge it.

3. Open a PR to [`zed-industries/extensions`](https://github.com/zed-industries/extensions) repo that updates the extension in question

Edit [`extensions.toml`](https://github.com/zed-industries/extensions/blob/main/extensions.toml) in the extensions repo to reflect the new version you set above and update the submodule latest Zed commit.

```sh
# Go into your clone of the extensions repo
cd ../extensions

# Update
git checkout main
git pull
just init-submodule extensions/zed

# Update the Zed submodule
cd extensions/zed
git checkout main
git pull
cd -
git add extensions.toml extensions/zed
```
