# Pending Changes

This is a list of pending changes to the Zed extension API that require a breaking change.

This list should be updated as we notice things that should be changed so that we can batch them up in a single release.

## vNext

### New Features

- Add notification API to allow extensions to show notifications to users
  - Supports info, warning, and error notifications
  - Allows setting notification title
  - Supports links to documentation or additional resources

### Slash Commands

- Rename `SlashCommand.tooltip_text` to `SlashCommand.menu_text`
  - We may even want to remove it entirely, as right now this is only used for featured slash commands, and slash commands defined by extensions aren't currently able to be featured.
