---
title: Publishing Guide
description: "Submit and update extensions in the Zed Extension Registry."
---

# Publishing Guide {#publishing-your-extension}

In order for the publishing to go smoothly, please make sure to follow the steps below in great detail.

## Forking and cloning the repo

1. Fork the `zed-indutries/extensions` repository.

> **Note:** It is very helpful if you fork the `zed-industries/extensions` repo to a personal GitHub account instead of a GitHub organization, as this allows Zed staff to push any needed changes to your PR to expedite the publishing process.

2. Clone the repo to your local machine

```sh
# Substitute the url of your fork here:
# git clone https://github.com/zed-industries/extensions
cd extensions
git submodule init
git submodule update
```

> Prior to publishing your extension, you should have installed as well as tested it locally thoroughly. Furthermore, you should have read the [publishing prerequisites](./prerequisites.md). Note that untested extension submissions where the extension is not functioning at all will be closed eagerly without further feedback.

To publish an extension, open a PR to [the `zed-industries/extensions` repo](https://github.com/zed-industries/extensions).

In your PR, do the following:

1. Add your extension as a Git submodule within the `extensions/` directory under the `extensions/{extension-id}` path

```sh
git submodule add https://github.com/your-username/foobar-zed.git extensions/my-extension
git add extensions/my-extension
```

> All extension submodules must use HTTPS URLs and not SSH URLS (`git@github.com`). Furthermore, your extension repository must be publicly available and the checked out submodule commit must be on a branch and thus not be a detached commit.

2. Add a new entry to the top-level `extensions.toml` file containing your extension:

```toml
[my-extension]
submodule = "extensions/my-extension"
version = "0.0.1"
```

If your extension is in a subdirectory within the submodule, you can use the `path` field to point to where the extension resides:

```toml
[my-extension]
submodule = "extensions-my-extension"
path = "packages/zed"
version = "0.0.1"
```

> Note that the [required extension license](./license-requirements.md) must reside at the specified path, a license at the root of the repository will not work. However, you are free to symlink an existing license within the repository or choose an alternative license from the list of accepted licenses for the extension code.

3. Run `pnpm sort-extensions` to ensure `extensions.toml` and `.gitmodules` are sorted

Once your PR is merged, the extension will be packaged and published to the Zed extension registry.

## Updating an extension {#updating-an-extension}

To update an extension, open a PR to [the `zed-industries/extensions` repo](https://github.com/zed-industries/extensions).

In your PR do the following:

1. Update the extension's submodule to the commit of the new version. For this, you can run

```sh
# From the root of the repository:
git submodule update --remote extensions/your-extension-name
```

to update your extension to the latest commit available in your remote repository.

2. Update the `version` field for the extension in `extensions.toml`
   - Make sure the `version` matches the one set in `extension.toml` at the particular commit.

If you'd like to automate this process, there is a [community GitHub Action](https://github.com/huacnlee/zed-extension-action) you can use.
