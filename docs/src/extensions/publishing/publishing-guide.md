---
title: Publishing Guide
description: "Submit and update extensions in the Zed Extension Registry."
---

# Publishing Guide {#publishing-your-extension}

> Before initiating the publishing process, read and ensure that your extension meets all [publishing prerequisites](./prerequisites.md) and [license requirements](./license-requirements.md). Only proceed with the steps below after satisfying these requirements. The publishing may be delayed or outright rejected otherwise.

Follow each step carefully to help the publishing process go smoothly.

## Pull request rules {#pull-request-rules}

To keep the review queue manageable, the following rules apply to every PR against the `zed-industries/extensions` repository:

- Every PR must add or update **exactly one extension**.
- You may have **at most three open PRs** at any given time.
- Respond to maintainer feedback within **3 weeks**, otherwise your PR will be closed.

PRs that do not adhere to these rules will be closed without further feedback. Repeated violations of these rules may result in a temporary suspension or a ban from submitting to the extension repository.

## Forking and cloning the repo

1. Fork the `zed-industries/extensions` repository.

> **Note:** It is very helpful if you fork the `zed-industries/extensions` repo to a personal GitHub account instead of a GitHub organization, as this allows Zed staff to push any needed changes to your PR to expedite the publishing process.

2. Clone the repo to your local machine.

```sh
# Substitute the URL of your fork here:
git clone https://github.com/your-username/extensions
cd extensions
git submodule init
git submodule update
```

## Submitting your extension {#submitting-your-extension}

To publish an extension, open a PR to [the `zed-industries/extensions` repo](https://github.com/zed-industries/extensions).

In your PR, do the following:

1. Add your extension as a Git submodule within the `extensions/` directory under the `extensions/{extension-id}` path.
   - The submodule must use an HTTPS URL and not an SSH URL (`git@github.com`).
   - Your extension repository must be publicly available.
   - The checked out submodule commit must be present on a branch and thus not be a detached commit.

```sh
git submodule add https://github.com/your-username/foobar-zed.git extensions/my-extension
git add extensions/my-extension
```

2. Add a new entry to the top-level `extensions.toml` file containing your extension:
   - Make sure the `version` matches the one set in `extension.toml` at the particular commit.

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

3. Run `pnpm sort-extensions` to ensure `extensions.toml` and `.gitmodules` are sorted.

That's it! Once your PR is accepted and merged, the extension will be packaged and published to the Zed extension registry.

## Review process {#review-process}

Once your PR is open, a maintainer will review it and may request changes. Keep the [pull request rules](#pull-request-rules) in mind: submissions are closed after **3 weeks of no response to maintainer feedback**.

For everything beyond that - how long reviews take, why we enforce the timeframe, or what to do after a PR was closed - see the [FAQ](./faq.md).
