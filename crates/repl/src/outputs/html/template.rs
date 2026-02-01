use gpui::{App, Window};
use markdown::{MarkdownFont, MarkdownStyle};
use theme::GlobalTheme;

/// Wraps user HTML content with a themed template that matches the current Zed theme.
///
/// Uses MarkdownStyle::themed to ensure consistent styling with markdown outputs.
pub fn wrap_html_with_theme(html: &str, window: &Window, cx: &App) -> String {
    let theme = GlobalTheme::theme(cx);
    let colors = theme.colors();
    let markdown_style = MarkdownStyle::themed(MarkdownFont::Editor, window, cx);

    // Get base text style from markdown
    let text_style = &markdown_style.base_text_style;
    let font_family = &text_style.font_family;
    let font_size = text_style.font_size.to_pixels(window.rem_size());

    // Convert GPUI Hsla colors to CSS hsla() format
    let bg_color = format!("{}", colors.background);
    let text_color = format!("{}", text_style.color);
    let border_color = format!("{}", colors.border);

    // Get link styling from markdown
    let link_color = markdown_style
        .link
        .color
        .map(|c| format!("{}", c))
        .unwrap_or_else(|| format!("{}", colors.text_accent));

    // Use theme color for code background (Background fields are private)
    let code_bg = format!("{}", colors.editor_background);

    format!(
        r#"<!DOCTYPE html>
<html>
<head>
    <meta charset="UTF-8">
    <style>
        :root {{
            --bg-color: {};
            --text-color: {};
            --border-color: {};
            --link-color: {};
            --code-bg: {};
        }}

        body {{
            margin: 0;
            padding: 8px;
            overflow: auto;
            background-color: var(--bg-color);
            color: var(--text-color);
            font-family: "{}", -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Helvetica, Arial, sans-serif;
            font-size: {}px;
        }}

        a {{
            color: var(--link-color);
        }}

        code, pre {{
            background-color: var(--code-bg);
            padding: 2px 4px;
            border-radius: 3px;
        }}

        pre {{
            padding: 8px;
        }}

        hr {{
            border-color: var(--border-color);
        }}

        /* Modern table styling matching markdown tables */
        table {{
            border-collapse: collapse;
            border-spacing: 0;
            margin: 8px 0;
        }}

        /* Remove pandas default borders */
        table[border] {{
            border: none !important;
        }}

        th, td {{
            padding: 4px 8px;
            text-align: left;
        }}

        /* Border between columns (not on first column) */
        th:not(:first-child),
        td:not(:first-child) {{
            border-left: 1px solid var(--border-color);
        }}

        /* Border between rows (not on first row) */
        tbody tr:not(:first-child) th,
        tbody tr:not(:first-child) td {{
            border-top: 1px solid var(--border-color);
        }}

        /* Header styling */
        thead th {{
            font-weight: 600;
        }}

        /* Border after header */
        thead tr {{
            border-bottom: 1px solid var(--border-color);
        }}
    </style>
</head>
<body>
    {}
    <script>
        function measureContent() {{
            const width = Math.max(
                document.documentElement.scrollWidth,
                document.body.scrollWidth
            );
            const height = Math.max(
                document.documentElement.scrollHeight,
                document.body.scrollHeight
            );
            window.ipc.postMessage(JSON.stringify({{ width: width, height: height }}));
        }}

        if (document.readyState === 'loading') {{
            document.addEventListener('DOMContentLoaded', measureContent);
        }} else {{
            measureContent();
        }}

        window.addEventListener('resize', measureContent);
    </script>
</body>
</html>"#,
        bg_color, text_color, border_color, link_color, code_bg, font_family, font_size, html
    )
}
