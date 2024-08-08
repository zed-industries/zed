## Creating New Themes

To create and publish a new theme to Zed, you need to follow a few steps:

1. Create a theme. You can find the json schema file describing our current structure. See [this blog post](https://zed.dev/blog/user-themes-now-in-preview) for instructions on creating and importing a local theme.
2. Set up an extension, as described below.
3. Create a folder, `themes`, in your extension's repository and place the theme you created in that directory.
4. [Open a PR in our extension repository](https://github.com/zed-industries/extensions/blob/main/AUTHORING_EXTENSIONS.md#publishing-your-extension) and submit your new theme!

### Theme JSON Structure

The structure of a Zed theme is defined in the [Zed Theme JSON Schema](https://zed.dev/schema/themes/v0.1.0.json).

A Zed theme contains a JSON object including:
- `name`: The name for the theme family
- `author`: The name of the author of this theme family
- `themes`: an array of theme objects

The core components of each of these theme objects include:

1. Theme Metadata:
   - `name`: The name of the theme
   - `appearance`: Either "light" or "dark"

2. Style Properties under the `style`, such as:
   - `background`: The main background color
   - `foreground`: The main text color
   - `accent`: The accent color used for highlighting and emphasis

3. Syntax Highlighting:
   - `syntax`: An object containing color definitions for various syntax elements (e.g., keywords, strings, comments)

4. UI Elements:
   - Colors for various UI components such as:
     - `element.background`: Background color for UI elements
     - `border`: Border colors for different states (normal, focused, selected)
     - `text`: Text colors for different states (normal, muted, accent)

5. Editor-specific Colors:
   - Colors for editor-related elements such as:
     - `editor.background`: Editor background color
     - `editor.gutter`: Gutter colors
     - `editor.line_number`: Line number colors

6. Terminal Colors:
   - ANSI color definitions for the integrated terminal

We recommend looking at our [existing themes](https://github.com/zed-industries/zed/tree/main/assets/themes) to get a better idea of what can be styled.

### Extensions

A Zed extension is a Git repository that contains a`extension.toml`:

```toml
id = "my-cool-theme"
name = "My Cool Theme"
version = "0.0.1"
schema_version = 1
authors = ["Your Name <you@example.com>"]
description = "My cool theme extension"
repository = "https://github.com/your-name/my-zed-theme"
```

And the json files for your theme, in a `themes` directory.

### Testing

To test that your extension is setup correctly:

1. use the `zed: extensions` command to open ourextensioninstaller.
2. Click `Install Dev Extension`
3. Select the directory with your theme extension.

If everything works properly, your new theme should show upinour theme selector. Use `theme selector: toggle` to check.

## Submitting

Once your theme and extension are finished, check [our extension repo](https://github.com/zed-industries/extensions/blob/main/AUTHORING_EXTENSIONS.md#publishing-your-extension) for the current documentation on publishing extensions.
