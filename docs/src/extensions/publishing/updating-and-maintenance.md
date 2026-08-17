---
title: Updating and Maintenance
description: "Update your published extension and see how maintenance and unmaintained extensions are handled."
---

# Updating and Maintenance {#updating-and-maintenance}

Once your extension has been published, this page covers everything that comes afterwards: how to ship updates, how maintenance is handled, and what happens when an extension is no longer maintained.

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

However, not every extension is perfect when it is first published and issues might arise, requiring maintenance effort. While we do not demand this effort from anyone, we generally expect:

- users and contributors to report issues and propose improvements in the original extension repository first, rather than publishing a competing extension
- extension owners to get back to those users within a reasonable timeframe

Now, maintaining an extension might not be everyone's cup of tea or things might have changed since publishing - that is totally fine! We fully understand that maintenance can take a lot of time and, as mentioned above, we do not require it.
At the same time, we do not want existing extensions to go stale in a bad state, as that leaves users of that extension with a poor experience. These two goals obviously clash: we cannot both leave maintenance entirely optional and just live with stale extensions.

## Unmaintained Extensions {#unmaintained-extensions}

To resolve this, in the case an extension owner no longer wants to maintain an extension or is unresponsive, we provide the following options:

- The current owner may transfer ownership of the repository to a new owner.
- A contributor may fork the extension and publish their fork as a replacement for the current extension.
- Zed staff can fork the extension into the `zed-extensions` organization and maintenance can continue from there as a joint effort of the community and Zed staff.
- Anyone may open an issue or pull request against `zed-industries/extensions` asking for the removal of the extension.

In order for any of the options to be applicable, we require either:

- written permission from the current extension owner
- proof of the reporting contributor that the upstream extension owner has been unresponsive to change requests for at least two months
