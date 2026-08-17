---
title: Publishing Guide
description: "Submit and update extensions in the Zed Extension Registry."
---

# Publishing Guide {#publishing-your-extension}

> Before initiating the publishing process, read and ensure that your extension meets all [publishing prerequisites](./prerequisites.md) and [license requirements](./license-requirements.md). Only proceed with the steps below after satisfying these requirements. The publishing may be delayed or outright rejected otherwise.

Follow each step carefully to help the publishing process go smoothly.

> To keep the review queue manageable, every PR must add or update **exactly one extension**, and you may have **at most three open PRs** at any given time. PRs that do not adhere to this will be closed without further feedback.
> Repeated violations of these limits may result in a temporary suspension or a ban from submitting to the extension repository.

## Forking and cloning the repo

1. Fork the `zed-industries/extensions` repository.

> **Note:** It is very helpful if you fork the `zed-industries/extensions` repo to a personal GitHub account instead of a GitHub organization, as this allows Zed staff to push any needed changes to your PR to expedite the publishing process.

2. Clone the repo to your local machine

```sh
# Substitute the url of your fork here:
# git clone https://github.com/zed-industries/extensions
cd extensions
git submodule init
git submodule update
```

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
submodule = "extensions/my-extension"
path = "packages/zed"
version = "0.0.1"
```

3. Run `pnpm sort-extensions` to ensure `extensions.toml` and `.gitmodules` are sorted

That's it! Once your PR is merged, the extension will be packaged and published to the Zed extension registry.

> We do our best to get back to you in a reasonable time frame. However, we are very aware that this is currently not always the case - we sincerely apologize for that. Please be informed we are continuously iterating on the process in an effort to provide every submission much more quickly with feedback and with an overall better contribution experience.
> At the same time, we do have to enforce a strict time frame for PR authors: Please be advised that submissions will be closed after **3 weeks of no response to maintainer feedback**. We do this in the interest of everybody to keep the queue in a more manageable state. After your PR was closed, you may open another fresh PR and we will take another look.

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
