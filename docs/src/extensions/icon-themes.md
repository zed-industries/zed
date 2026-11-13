---
title: Icon Themes
description: "Icon Themes for Zed extensions."
---

# Icon Themes

Extensions may provide icon themes to change the icons Zed uses for folders and files.

## Example extension

The [Material Icon Theme](https://github.com/zed-extensions/material-icon-theme) serves as an example for the structure of an extension containing an icon theme.

## Directory structure

There are two important directories for an icon theme extension:

- `icon_themes`: This directory will contain one or more JSON files containing the icon theme definitions.
- `icons`: This directory contains the icon assets distributed with the extension. You can create subdirectories in this directory as needed.

Each icon theme file should adhere to the JSON schema specified at [`https://zed.dev/schema/icon_themes/v0.3.0.json`](https://zed.dev/schema/icon_themes/v0.3.0.json).

Here is an example icon theme structure:

```json [icon-theme]
{
  "$schema": "https://zed.dev/schema/icon_themes/v0.3.0.json",
  "name": "My Icon Theme",
  "author": "Your Name",
  "themes": [
    {
      "name": "My Icon Theme",
      "appearance": "dark",
      "directory_icons": {
        "collapsed": "./icons/folder.svg",
        "expanded": "./icons/folder-open.svg"
      },
      "named_directory_icons": {
        "stylesheets": {
          "collapsed": "./icons/folder-stylesheets.svg",
          "expanded": "./icons/folder-stylesheets-open.svg"
        }
      },
      "chevron_icons": {
        "collapsed": "./icons/chevron-right.svg",
        "expanded": "./icons/chevron-down.svg"
      },
      "file_patterns": {
        "app/models/**/*.rb": "rails_model",
        "**/*_controller.rb": "rails_controller"
      },
      "file_stems": {
        "Makefile": "make"
      },
      "file_suffixes": {
        "mp3": "audio",
        "rs": "rust"
      },
      "file_icons": {
        "audio": { "path": "./icons/audio.svg" },
        "default": { "path": "./icons/file.svg" },
        "make": { "path": "./icons/make.svg" },
        "rails_controller": { "path": "./icons/rails-controller.svg" },
        "rails_model": { "path": "./icons/rails-model.svg" },
        "rust": { "path": "./icons/rust.svg" }
        // ...
      }
    }
  ]
}
```

Each icon path is resolved relative to the root of the extension directory.

## Matching files by path

`file_stems` and `file_suffixes` match a file's name. `file_patterns` matches its path instead, which lets a theme tell apart files that share a name or an extension, such as the models and controllers of a Rails project. Each key is a glob pattern and each value is a key in `file_icons`.

- Patterns are checked before `file_stems` and `file_suffixes`. When several patterns match, the first one in the file wins, so list the more specific ones first.
- A pattern that does not start with `**` matches at any depth, as if it started with `**/`: `app/models/**/*.rb` matches `app/models/user.rb` as well as `engines/billing/app/models/invoice.rb`.
- `*` does not match `/`; use `**` to cross directories. Matching is case-sensitive.
- If the icon key of a matching pattern has no icon, Zed tries the next matching pattern, and then the file's name.
- A few places only know a file's name, such as agent mentions; there, patterns that mention a directory do not match.

In this example, the extension would have this structure:

```
extension.toml
icon_themes/
  my-icon-theme.json
icons/
  audio.svg
  chevron-down.svg
  chevron-right.svg
  file.svg
  folder-open.svg
  folder.svg
  rails-controller.svg
  rails-model.svg
  rust.svg
```
