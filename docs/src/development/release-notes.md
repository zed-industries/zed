# Release Notes

Whenever you open a pull request, the body is automatically populated based on this [pull request template](https://github.com/zed-industries/zed/blob/main/.github/pull_request_template.md).

```md
...

Release Notes:

- N/A _or_ Added/Fixed/Improved ...
```

On Wednesdays, we run a [`get-preview-channel-changes`](https://github.com/zed-industries/zed/blob/main/script/get-preview-channel-changes) script that scrapes `Release Notes` lines from pull requests landing in preview, as documented in our [Release](https://zed.dev/docs/development/releases) docs.

The script outputs everything below the `Release Notes` line, including additional data such as the pull request author (if not a Zed team member) and a link to the pull request.
If you use `N/A`, the script skips your pull request entirely.

## Guidelines for crafting your `Release Notes` line(s)

- A `Release Notes` line should only be written if the user can see or feel the difference in Zed.
- A `Release Notes` line should be written such that a Zed user can understand what the change is.
  Don't assume a user knows technical editor developer lingo; phrase your change in language they understand as a user of a text editor.
- If you want to include technical details about your pull request for other team members to see, do so above the `Release Notes` line.
- Changes to docs should be labeled as `N/A`.
- If your pull request adds/changes a setting or a keybinding, always mention that setting or keybinding.
  Don't make the user dig into docs or the pull request to find this information (although it should be included in docs as well).
- For pull requests that are reverts:
  - If the item being reverted **has already been shipped**, include a `Release Notes` line explaining why we reverted, as this is a breaking change.
  - If the item being reverted **hasn't been shipped**, edit the original PR's `Release Notes` line to be `N/A`; otherwise, it will be included and the compiler of the release notes may not know to skip it, leading to a potentially-awkward situation where we are stating we shipped something we actually didn't.
