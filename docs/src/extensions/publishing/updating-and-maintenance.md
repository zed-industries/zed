---
title: Updating and Maintenance
description: "Update your published extension and see how maintenance and unmaintained extensions are handled."
---

# Updating and Maintenance {#updating-and-maintenance}

## Updating an Extension {#updating-an-extension}

To update an extension, open a PR to [the `zed-industries/extensions` repo](https://github.com/zed-industries/extensions).

> Update PRs are subject to the same [pull request rules](./publishing-guide.md#pull-request-rules) as new submissions.

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

## Maintenance {#maintenance}

After your extension has been published, there are no further requirements from our side. Thank you for enriching the Zed extension collection, we really appreciate it!

That said, not every extension is perfect on day one — bugs may surface and improvement requests may come in. While we do not want to demand maintenance from anyone, we generally expect:

- users and contributors to report issues and propose improvements in the original extension repository first, rather than publishing a competing extension
- extension owners to respond to those reports within a reasonable timeframe

Priorities change, and stepping back from an extension is completely fine. At the same time, we don't want published extensions to go stale, since that leaves their users with a poor experience. How we balance the two is described in [Unmaintained Extensions](#unmaintained-extensions) below.

## Unmaintained Extensions {#unmaintained-extensions}

When an extension owner no longer wants to maintain an extension, or has become unresponsive, there are the following options:

- The current owner may transfer ownership of the repository to a new owner.
- A contributor may fork the extension and propose their fork as a replacement for the current extension.
- Zed staff can fork the extension into the `zed-extensions` organization and maintenance can continue from there as a joint effort of the community and Zed staff.
- The current owner may open an issue or pull request against `zed-industries/extensions` asking for the removal of their extension.

Switching to a fork does not have to be permanent: should the original extension owner become responsive again, the extension can be switched back to the original repository.

Please note: In order for **any** of these options to be applicable, we require either:

- written permission from the current extension owner, or
- proof of the reporting contributor that the upstream extension owner has been unresponsive to change requests for at least two months
