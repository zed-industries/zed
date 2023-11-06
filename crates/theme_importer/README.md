# Zed Theme Importer

---

## Usage

- `cargo run -p theme_importer` - Import the context of `assets/themes/src`

---

## Troubleshooting

As the importer generates rust files, you may need to manually do some cleanup in `registry.rs` and `themes/mod.rs` if you remove themes or delete the `themes` folder in the theme crate.

---

## Required Structure

To import a theme or series of themes 3 things are required:

- `family.json`: A JSON file containing the theme family metadata and list of theme variants
- `{theme_name}.json`: One theme json for each theme variant
- `LICENSE`: A license file for the theme family

### `family.json`

#### `name`

The name of the theme family. Avoid special characters.

This will be used for the theme family directory name (lowercased) and the theme family name in the Zed UI.

Good:

- `Rose Pine`
- `Synthwave 84`
- `Monokai Solarized`

Bad:

- `Rosé Pine`
- `Synthwave '84`
- `Monokai (Solarized)`

#### `author`

The author of the theme family. This can be a name or a username.

This will be used for the theme family author in the Zed UI.

#### `themes`

A list of theme variants.

`appearance` can be either `light` or `dark`. This will impact which default fallback colors are used, and where the theme shows up in the Zed UI.

### `{theme_name}.json`

Each theme added to the family must have a corresponding JSON file. This JSON file can be obtained from the VSCode extensions folder (once you have installed it.) This is usually located at `~/.vscode/extensions` (on macOS).

You can use `open ~/.vscode/extensions` to open the folder in Finder directly.

Copy that json file into the theme family directory and tidy up the filenames as needed.

### `LICENSE`

A LICENSE file is required to import a theme family. Failing to provide a complete text license will cause it to be skipped when the import is run.

If the theme only provices a license code (e.g. MIT, Apache 2.0, etc.) then put that code into the LICENSE file.

If no license is provided, either contact the theme creator or don't add the theme.

---

### Complete Example:

An example family with multiple variants:

```json
{
  "name": "Ayu",
  // When both name and username are available
  // prefer the `username (name)` format
  "author": "dempfi (Ike Ku)",
  "themes": [
    {
      "name": "Ayu Light",
      "file_name": "ayu-light.json",
      "appearance": "light"
    },
    {
      "name": "Ayu Mirage",
      "file_name": "ayu-mirage.json",
      "appearance": "dark"
    },
    {
      "name": "Ayu Dark",
      "file_name": "ayu-dark.json",
      "appearance": "dark"
    }
  ]
}
```

An example single variant family:

```json
{
  "name": "Andromeda",
  "author": "Eliver Lara (EliverLara)",
  "themes": [
    {
      "name": "Andromeda",
      "file_name": "andromeda.json",
      "appearance": "dark"
    },
    {
      "name": "Andromeda Bordered",
      "file_name": "andromeda-bordered.json",
      "appearance": "dark"
    }
  ]
}
```
