# Themes

The `themes` directory in an extension should contain one or more theme files.

Each theme file should adhere to the JSON schema specified at [`https://zed.dev/schema/themes/v0.2.0.json`](https://zed.dev/schema/themes/v0.2.0.json).

See [this blog post](https://zed.dev/blog/user-themes-now-in-preview) for more details about creating themes.

## Theme JSON Structure

The structure of a Zed theme is defined in the [Zed Theme JSON Schema](https://zed.dev/schema/themes/v0.2.0.json).

A Zed theme consists of a Theme Family object including:

- `name`: The name for the theme family
- `author`: The name of the author of the theme family
- `themes`: An array of Themes belonging to the theme family

The core components a Theme object include:

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

We recommend looking at our [existing themes](https://github.com/zed-industries/zed/tree/main/assets/themes) to get a more comprehensive idea of what can be styled.
