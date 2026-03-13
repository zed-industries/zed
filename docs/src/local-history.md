# Local History

Local history records snapshots of files in your workspace when you save. It
lets you review changes and restore earlier versions from the Local History
panel.

## Getting Started {#getting-started}

1. Open the Local History panel from the status bar or the command bar.
2. Save a file.
3. Select an entry to review changes or restore it.

## Local History Panel {#local-history-panel}

The panel shows entries for the active editor file. Each entry includes a
timestamp and actions to open a diff or restore that snapshot.

Local history reads from all configured storage endpoints. New snapshots are
written to the active endpoint.

## Storage and Retention {#storage-and-retention}

Local history is stored on your machine. By default, it uses Zed's data
directory, but you can add additional storage endpoints and switch the active
one (for example, to an external drive).

Snapshots are excluded by default for common, easily regenerated paths and
file types, such as `node_modules` and `*.min.*`. Excluded paths are not
recorded and do not count toward size limits.

Retention defaults:

- Per-worktree cap: 0.12% of available free space in the active endpoint, or
  300 MiB (whichever is larger)
- Minimum age before deletion: 100 days
- Pruning policy: `both` (entries are removed only when the cap is exceeded
  and the entry is older than the minimum age)

## Settings {#settings}

Use the Settings Editor to configure the Local History panel button, dock, and
default width.

Or add this to your `settings.json`:

```json [settings]
{
  "local_history": {
    "enabled": true,
    "capture_on_save": true,
    "storage_paths": ["/Volumes/External/ZedHistory"],
    "active_storage_path": "/Volumes/External/ZedHistory",
    "min_age_days": 100,
    "cap_free_space_percent": 0.12,
    "cap_min_bytes": 314572800,
    "prune_policy": "both",
    "exclude_globs": [
      "**/.git/**",
      "**/.hg/**",
      "**/.svn/**",
      "**/.jj/**",
      "**/node_modules/**",
      "**/target/**",
      "**/dist/**",
      "**/build/**",
      "**/out/**",
      "**/.gradle/**",
      "**/.idea/**",
      "**/.zed/**",
      "**/*.min.*",
      "**/*.map"
    ]
  },
  "local_history_panel": {
    "button": true,
    "dock": "right",
    "default_width": 300,
    "show_relative_path": false
  }
}
```

Valid `prune_policy` values are `both`, `size_only`, `age_only`, and `any`.
