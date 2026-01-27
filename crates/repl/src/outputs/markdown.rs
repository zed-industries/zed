use gpui::{
    AbsoluteLength, App, AppContext, BorderStyle, ClipboardItem, Context, DefiniteLength,
    EdgesRefinement, Entity, Length, StyleRefinement, TextStyleRefinement, UnderlineStyle, Window,
    div, prelude::*, px, rems,
};
use language::Buffer;
use markdown::{HeadingLevelStyles, Markdown, MarkdownElement, MarkdownStyle};
use settings::Settings;
use theme::{ActiveTheme, ThemeSettings};

use crate::outputs::OutputContent;

pub struct MarkdownView {
    raw_text: String,
    markdown: Entity<Markdown>,
}

impl MarkdownView {
    pub fn from(text: String, cx: &mut Context<Self>) -> Self {
        let markdown = cx.new(|cx| Markdown::new(text.clone().into(), None, None, cx));

        Self {
            raw_text: text,
            markdown,
        }
    }
}

impl OutputContent for MarkdownView {
    fn clipboard_content(&self, _window: &Window, _cx: &App) -> Option<ClipboardItem> {
        Some(ClipboardItem::new_string(self.raw_text.clone()))
    }

    fn has_clipboard_content(&self, _window: &Window, _cx: &App) -> bool {
        true
    }

    fn has_buffer_content(&self, _window: &Window, _cx: &App) -> bool {
        true
    }

    fn buffer_content(&mut self, _: &mut Window, cx: &mut App) -> Option<Entity<Buffer>> {
        let buffer = cx.new(|cx| {
            let mut buffer = Buffer::local(self.raw_text.clone(), cx)
                .with_language(language::PLAIN_TEXT.clone(), cx);
            buffer.set_capability(language::Capability::ReadOnly, cx);
            buffer
        });
        Some(buffer)
    }
}

impl Render for MarkdownView {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let style = markdown_style(window, cx);
        div()
            .w_full()
            .child(MarkdownElement::new(self.markdown.clone(), style))
    }
}

fn markdown_style(window: &Window, cx: &App) -> MarkdownStyle {
    let theme_settings = ThemeSettings::get_global(cx);
    let colors = cx.theme().colors();

    let buffer_font_size = theme_settings.buffer_font_size(cx);

    let mut text_style = window.text_style();
    let line_height = buffer_font_size * 1.5;

    text_style.refine(&TextStyleRefinement {
        font_family: Some(theme_settings.ui_font.family.clone()),
        font_fallbacks: theme_settings.ui_font.fallbacks.clone(),
        font_features: Some(theme_settings.ui_font.features.clone()),
        font_size: Some(theme_settings.ui_font_size(cx).into()),
        line_height: Some(line_height.into()),
        color: Some(colors.text),
        ..Default::default()
    });

    MarkdownStyle {
        base_text_style: text_style.clone(),
        syntax: cx.theme().syntax().clone(),
        selection_background_color: colors.element_selection_background,
        code_block_overflow_x_scroll: true,
        heading_level_styles: Some(HeadingLevelStyles {
            h1: Some(TextStyleRefinement {
                font_size: Some(rems(1.15).into()),
                ..Default::default()
            }),
            h2: Some(TextStyleRefinement {
                font_size: Some(rems(1.1).into()),
                ..Default::default()
            }),
            h3: Some(TextStyleRefinement {
                font_size: Some(rems(1.05).into()),
                ..Default::default()
            }),
            h4: Some(TextStyleRefinement {
                font_size: Some(rems(1.).into()),
                ..Default::default()
            }),
            h5: Some(TextStyleRefinement {
                font_size: Some(rems(0.95).into()),
                ..Default::default()
            }),
            h6: Some(TextStyleRefinement {
                font_size: Some(rems(0.875).into()),
                ..Default::default()
            }),
        }),
        code_block: StyleRefinement {
            padding: EdgesRefinement {
                top: Some(DefiniteLength::Absolute(AbsoluteLength::Pixels(px(8.)))),
                left: Some(DefiniteLength::Absolute(AbsoluteLength::Pixels(px(8.)))),
                right: Some(DefiniteLength::Absolute(AbsoluteLength::Pixels(px(8.)))),
                bottom: Some(DefiniteLength::Absolute(AbsoluteLength::Pixels(px(8.)))),
            },
            margin: EdgesRefinement {
                top: Some(Length::Definite(px(8.).into())),
                left: Some(Length::Definite(px(0.).into())),
                right: Some(Length::Definite(px(0.).into())),
                bottom: Some(Length::Definite(px(12.).into())),
            },
            border_style: Some(BorderStyle::Solid),
            border_widths: EdgesRefinement {
                top: Some(AbsoluteLength::Pixels(px(1.))),
                left: Some(AbsoluteLength::Pixels(px(1.))),
                right: Some(AbsoluteLength::Pixels(px(1.))),
                bottom: Some(AbsoluteLength::Pixels(px(1.))),
            },
            border_color: Some(colors.border_variant),
            background: Some(colors.editor_background.into()),
            text: TextStyleRefinement {
                font_family: Some(theme_settings.buffer_font.family.clone()),
                font_fallbacks: theme_settings.buffer_font.fallbacks.clone(),
                font_features: Some(theme_settings.buffer_font.features.clone()),
                font_size: Some(buffer_font_size.into()),
                ..Default::default()
            },
            ..Default::default()
        },
        inline_code: TextStyleRefinement {
            font_family: Some(theme_settings.buffer_font.family.clone()),
            font_fallbacks: theme_settings.buffer_font.fallbacks.clone(),
            font_features: Some(theme_settings.buffer_font.features.clone()),
            font_size: Some(buffer_font_size.into()),
            background_color: Some(colors.editor_foreground.opacity(0.08)),
            ..Default::default()
        },
        link: TextStyleRefinement {
            background_color: Some(colors.editor_foreground.opacity(0.025)),
            color: Some(colors.text_accent),
            underline: Some(UnderlineStyle {
                color: Some(colors.text_accent.opacity(0.5)),
                thickness: px(1.),
                ..Default::default()
            }),
            ..Default::default()
        },
        ..Default::default()
    }
}
