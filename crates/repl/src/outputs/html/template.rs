use gpui::{App, Window};
use settings::Settings as _;
use theme::{GlobalTheme, ThemeSettings};

/// Wraps user HTML content with a themed template that matches the current Zed theme.
///
/// - Extracts colors from the current theme
/// - Converts GPUI colors to CSS format
/// - Injects CSS custom properties for theming
/// - Adds size measurement script for dynamic layout
pub fn wrap_html_with_theme(html: &str, _window: &Window, cx: &App) -> String {
    let theme = GlobalTheme::theme(cx);
    let colors = theme.colors();
    let theme_settings = ThemeSettings::get_global(cx);

    let font_family = &theme_settings.ui_font.family;
    let font_size = theme_settings.ui_font_size(cx);

    // NOTE: relies on Display formatting as `hsla(h, s, l, a)`
    let bg_color = format!("{}", colors.background);
    let text_color = format!("{}", colors.text);
    let text_muted = format!("{}", colors.text_muted);
    let border_color = format!("{}", colors.border);
    let border_variant = format!("{}", colors.border_variant);
    let link_color = format!("{}", colors.text_accent);

    let font_size: f32 = font_size.into();

    format!(
        r#"<!DOCTYPE html>
<html>
<head>
    <meta charset="UTF-8">
    <style>
        :root {{
            --bg-color: {};
            --text-color: {};
            --text-muted: {};
            --border-color: {};
            --border-variant: {};
            --link-color: {};
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
            background-color: var(--border-variant);
        }}

        hr {{
            border-color: var(--border-color);
        }}

        table {{
            border-color: var(--border-color);
        }}

        th, td {{
            border-color: var(--border-variant);
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
        bg_color,
        text_color,
        text_muted,
        border_color,
        border_variant,
        link_color,
        font_family,
        font_size,
        html
    )
}
