use anyhow::Result;
use gpui::Hsla;
use palette::FromColor;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::ThemeColorsRefinement;

/// The content of a serialized theme.
#[derive(Debug, Clone, Default, Serialize, Deserialize, JsonSchema)]
#[serde(default)]
pub struct ThemeContent {
    /// Border color. Used for most borders, is usually a high contrast color.
    #[serde(rename = "border")]
    pub border: Option<String>,

    /// Border color. Used for deemphasized borders, like a visual divider between two sections
    #[serde(rename = "border.variant")]
    pub border_variant: Option<String>,

    /// Border color. Used for focused elements, like keyboard focused list item.
    #[serde(rename = "border.focused")]
    pub border_focused: Option<String>,

    /// Border color. Used for selected elements, like an active search filter or selected checkbox.
    #[serde(rename = "border.selected")]
    pub border_selected: Option<String>,

    /// Border color. Used for transparent borders. Used for placeholder borders when an element gains a border on state change.
    #[serde(rename = "border.transparent")]
    pub border_transparent: Option<String>,

    /// Border color. Used for disabled elements, like a disabled input or button.
    #[serde(rename = "border.disabled")]
    pub border_disabled: Option<String>,

    /// Border color. Used for elevated surfaces, like a context menu, popup, or dialog.
    #[serde(rename = "elevated_surface.background")]
    pub elevated_surface_background: Option<String>,

    /// Background Color. Used for grounded surfaces like a panel or tab.
    #[serde(rename = "surface.background")]
    pub surface_background: Option<String>,

    /// Background Color. Used for the app background and blank panels or windows.
    #[serde(rename = "background")]
    pub background: Option<String>,

    /// Background Color. Used for the background of an element that should have a different background than the surface it's on.
    ///
    /// Elements might include: Buttons, Inputs, Checkboxes, Radio Buttons...
    ///
    /// For an element that should have the same background as the surface it's on, use `ghost_element_background`.
    #[serde(rename = "element.background")]
    pub element_background: Option<String>,

    /// Background Color. Used for the hover state of an element that should have a different background than the surface it's on.
    ///
    /// Hover states are triggered by the mouse entering an element, or a finger touching an element on a touch screen.
    #[serde(rename = "element.hover")]
    pub element_hover: Option<String>,

    /// Background Color. Used for the active state of an element that should have a different background than the surface it's on.
    ///
    /// Active states are triggered by the mouse button being pressed down on an element, or the Return button or other activator being pressd.
    #[serde(rename = "element.active")]
    pub element_active: Option<String>,

    /// Background Color. Used for the selected state of an element that should have a different background than the surface it's on.
    ///
    /// Selected states are triggered by the element being selected (or "activated") by the user.
    ///
    /// This could include a selected checkbox, a toggleable button that is toggled on, etc.
    #[serde(rename = "element.selected")]
    pub element_selected: Option<String>,

    /// Background Color. Used for the disabled state of an element that should have a different background than the surface it's on.
    ///
    /// Disabled states are shown when a user cannot interact with an element, like a disabled button or input.
    #[serde(rename = "element.disabled")]
    pub element_disabled: Option<String>,

    /// Background Color. Used for the area that shows where a dragged element will be dropped.
    #[serde(rename = "drop_target.background")]
    pub drop_target_background: Option<String>,

    /// Used for the background of a ghost element that should have the same background as the surface it's on.
    ///
    /// Elements might include: Buttons, Inputs, Checkboxes, Radio Buttons...
    ///
    /// For an element that should have a different background than the surface it's on, use `element_background`.
    #[serde(rename = "ghost_element.background")]
    pub ghost_element_background: Option<String>,

    /// Background Color. Used for the hover state of a ghost element that should have the same background as the surface it's on.
    ///
    /// Hover states are triggered by the mouse entering an element, or a finger touching an element on a touch screen.
    #[serde(rename = "ghost_element.hover")]
    pub ghost_element_hover: Option<String>,

    /// Background Color. Used for the active state of a ghost element that should have the same background as the surface it's on.
    ///
    /// Active states are triggered by the mouse button being pressed down on an element, or the Return button or other activator being pressd.
    #[serde(rename = "ghost_element.active")]
    pub ghost_element_active: Option<String>,

    /// Background Color. Used for the selected state of a ghost element that should have the same background as the surface it's on.
    ///
    /// Selected states are triggered by the element being selected (or "activated") by the user.
    ///
    /// This could include a selected checkbox, a toggleable button that is toggled on, etc.
    #[serde(rename = "ghost_element.selected")]
    pub ghost_element_selected: Option<String>,

    /// Background Color. Used for the disabled state of a ghost element that should have the same background as the surface it's on.
    ///
    /// Disabled states are shown when a user cannot interact with an element, like a disabled button or input.
    #[serde(rename = "ghost_element.disabled")]
    pub ghost_element_disabled: Option<String>,

    /// Text Color. Default text color used for most text.
    #[serde(rename = "text")]
    pub text: Option<String>,

    /// Text Color. Color of muted or deemphasized text. It is a subdued version of the standard text color.
    #[serde(rename = "text.muted")]
    pub text_muted: Option<String>,

    /// Text Color. Color of the placeholder text typically shown in input fields to guide the user to enter valid data.
    #[serde(rename = "text.placeholder")]
    pub text_placeholder: Option<String>,

    /// Text Color. Color used for text denoting disabled elements. Typically, the color is faded or grayed out to emphasize the disabled state.
    #[serde(rename = "text.disabled")]
    pub text_disabled: Option<String>,

    /// Text Color. Color used for emphasis or highlighting certain text, like an active filter or a matched character in a search.
    #[serde(rename = "text.accent")]
    pub text_accent: Option<String>,

    /// Fill Color. Used for the default fill color of an icon.
    #[serde(rename = "icon")]
    pub icon: Option<String>,

    /// Fill Color. Used for the muted or deemphasized fill color of an icon.
    ///
    /// This might be used to show an icon in an inactive pane, or to demphasize a series of icons to give them less visual weight.
    #[serde(rename = "icon.muted")]
    pub icon_muted: Option<String>,

    /// Fill Color. Used for the disabled fill color of an icon.
    ///
    /// Disabled states are shown when a user cannot interact with an element, like a icon button.
    #[serde(rename = "icon.disabled")]
    pub icon_disabled: Option<String>,

    /// Fill Color. Used for the placeholder fill color of an icon.
    ///
    /// This might be used to show an icon in an input that disappears when the user enters text.
    #[serde(rename = "icon.placeholder")]
    pub icon_placeholder: Option<String>,

    /// Fill Color. Used for the accent fill color of an icon.
    ///
    /// This might be used to show when a toggleable icon button is selected.
    #[serde(rename = "con.accent")]
    pub icon_accent: Option<String>,

    #[serde(rename = "status_bar.background")]
    pub status_bar_background: Option<String>,

    #[serde(rename = "title_bar.background")]
    pub title_bar_background: Option<String>,

    #[serde(rename = "toolbar.background")]
    pub toolbar_background: Option<String>,

    #[serde(rename = "tab_bar.background")]
    pub tab_bar_background: Option<String>,

    #[serde(rename = "tab.inactive_background")]
    pub tab_inactive_background: Option<String>,

    #[serde(rename = "tab.active_background")]
    pub tab_active_background: Option<String>,

    #[serde(rename = "search.match_background")]
    pub search_match_background: Option<String>,

    #[serde(rename = "panel.background")]
    pub panel_background: Option<String>,

    #[serde(rename = "panel.focused_border")]
    pub panel_focused_border: Option<String>,

    #[serde(rename = "pane.focused_border")]
    pub pane_focused_border: Option<String>,

    /// The color of the scrollbar thumb.
    #[serde(rename = "scrollbar_thumb.background")]
    pub scrollbar_thumb_background: Option<String>,

    /// The color of the scrollbar thumb when hovered over.
    #[serde(rename = "scrollbar.thumb.hover_background")]
    pub scrollbar_thumb_hover_background: Option<String>,

    /// The border color of the scrollbar thumb.
    #[serde(rename = "scrollbar.thumb.border")]
    pub scrollbar_thumb_border: Option<String>,

    /// The background color of the scrollbar track.
    #[serde(rename = "scrollbar.track.background")]
    pub scrollbar_track_background: Option<String>,

    /// The border color of the scrollbar track.
    #[serde(rename = "scrollbar.track.border")]
    pub scrollbar_track_border: Option<String>,

    #[serde(rename = "editor.foreground")]
    pub editor_foreground: Option<String>,

    #[serde(rename = "editor.background")]
    pub editor_background: Option<String>,

    #[serde(rename = "editor.gutter.background")]
    pub editor_gutter_background: Option<String>,

    #[serde(rename = "editor.subheader.background")]
    pub editor_subheader_background: Option<String>,

    #[serde(rename = "editor.active_line.background")]
    pub editor_active_line_background: Option<String>,

    #[serde(rename = "editor.highlighted_line.background")]
    pub editor_highlighted_line_background: Option<String>,

    /// Text Color. Used for the text of the line number in the editor gutter.
    #[serde(rename = "editor.line_number")]
    pub editor_line_number: Option<String>,

    /// Text Color. Used for the text of the line number in the editor gutter when the line is highlighted.
    #[serde(rename = "editor.active_line_number")]
    pub editor_active_line_number: Option<String>,

    /// Text Color. Used to mark invisible characters in the editor.
    ///
    /// Example: spaces, tabs, carriage returns, etc.
    #[serde(rename = "editor.invisible")]
    pub editor_invisible: Option<String>,

    #[serde(rename = "editor.wrap_guide")]
    pub editor_wrap_guide: Option<String>,

    #[serde(rename = "editor.active_wrap_guide")]
    pub editor_active_wrap_guide: Option<String>,

    /// Read-access of a symbol, like reading a variable.
    ///
    /// A document highlight is a range inside a text document which deserves
    /// special attention. Usually a document highlight is visualized by changing
    /// the background color of its range.
    #[serde(rename = "editor.document_highlight.read_background")]
    pub editor_document_highlight_read_background: Option<String>,

    /// Read-access of a symbol, like reading a variable.
    ///
    /// A document highlight is a range inside a text document which deserves
    /// special attention. Usually a document highlight is visualized by changing
    /// the background color of its range.
    #[serde(rename = "editor.document_highlight.write_background")]
    pub editor_document_highlight_write_background: Option<String>,

    /// Terminal background color.
    #[serde(rename = "terminal.background")]
    pub terminal_background: Option<String>,

    /// Terminal foreground color.
    #[serde(rename = "terminal.foreground")]
    pub terminal_foreground: Option<String>,

    /// Bright terminal foreground color.
    #[serde(rename = "terminal.bright_foreground")]
    pub terminal_bright_foreground: Option<String>,

    /// Dim terminal foreground color.
    #[serde(rename = "terminal.dim_foreground")]
    pub terminal_dim_foreground: Option<String>,

    /// Black ANSI terminal color.
    #[serde(rename = "terminal.ansi.black")]
    pub terminal_ansi_black: Option<String>,

    /// Bright black ANSI terminal color.
    #[serde(rename = "terminal.ansi.bright_black")]
    pub terminal_ansi_bright_black: Option<String>,

    /// Dim black ANSI terminal color.
    #[serde(rename = "terminal.ansi.dim_black")]
    pub terminal_ansi_dim_black: Option<String>,

    /// Red ANSI terminal color.
    #[serde(rename = "terminal.ansi.red")]
    pub terminal_ansi_red: Option<String>,

    /// Bright red ANSI terminal color.
    #[serde(rename = "terminal.ansi.bright_red")]
    pub terminal_ansi_bright_red: Option<String>,

    /// Dim red ANSI terminal color.
    #[serde(rename = "terminal.ansi.dim_red")]
    pub terminal_ansi_dim_red: Option<String>,

    /// Green ANSI terminal color.
    #[serde(rename = "terminal.ansi.green")]
    pub terminal_ansi_green: Option<String>,

    /// Bright green ANSI terminal color.
    #[serde(rename = "terminal.ansi.bright_green")]
    pub terminal_ansi_bright_green: Option<String>,

    /// Dim green ANSI terminal color.
    #[serde(rename = "terminal.ansi.dim_green")]
    pub terminal_ansi_dim_green: Option<String>,

    /// Yellow ANSI terminal color.
    #[serde(rename = "terminal.ansi.yellow")]
    pub terminal_ansi_yellow: Option<String>,

    /// Bright yellow ANSI terminal color.
    #[serde(rename = "terminal.ansi.bright_yellow")]
    pub terminal_ansi_bright_yellow: Option<String>,

    /// Dim yellow ANSI terminal color.
    #[serde(rename = "terminal.ansi.dim_yellow")]
    pub terminal_ansi_dim_yellow: Option<String>,

    /// Blue ANSI terminal color.
    #[serde(rename = "terminal.ansi.blue")]
    pub terminal_ansi_blue: Option<String>,

    /// Bright blue ANSI terminal color.
    #[serde(rename = "terminal.ansi.bright_blue")]
    pub terminal_ansi_bright_blue: Option<String>,

    /// Dim blue ANSI terminal color.
    #[serde(rename = "terminal.ansi.dim_blue")]
    pub terminal_ansi_dim_blue: Option<String>,

    /// Magenta ANSI terminal color.
    #[serde(rename = "terminal.ansi.magenta")]
    pub terminal_ansi_magenta: Option<String>,

    /// Bright magenta ANSI terminal color.
    #[serde(rename = "terminal.ansi.bright_magenta")]
    pub terminal_ansi_bright_magenta: Option<String>,

    /// Dim magenta ANSI terminal color.
    #[serde(rename = "terminal.ansi.dim_magenta")]
    pub terminal_ansi_dim_magenta: Option<String>,

    /// Cyan ANSI terminal color.
    #[serde(rename = "terminal.ansi.cyan")]
    pub terminal_ansi_cyan: Option<String>,

    /// Bright cyan ANSI terminal color.
    #[serde(rename = "terminal.ansi.bright_cyan")]
    pub terminal_ansi_bright_cyan: Option<String>,

    /// Dim cyan ANSI terminal color.
    #[serde(rename = "terminal.ansi.dim_cyan")]
    pub terminal_ansi_dim_cyan: Option<String>,

    /// White ANSI terminal color.
    #[serde(rename = "terminal.ansi.white")]
    pub terminal_ansi_white: Option<String>,

    /// Bright white ANSI terminal color.
    #[serde(rename = "terminal.ansi.bright_white")]
    pub terminal_ansi_bright_white: Option<String>,

    /// Dim white ANSI terminal color.
    #[serde(rename = "terminal.ansi.dim_white")]
    pub terminal_ansi_dim_white: Option<String>,

    #[serde(rename = "link_text.hover")]
    pub link_text_hover: Option<String>,
}

pub fn try_parse_color(color: &str) -> Result<Hsla> {
    let rgba = gpui::Rgba::try_from(color)?;
    let rgba = palette::rgb::Srgba::from_components((rgba.r, rgba.g, rgba.b, rgba.a));
    let hsla = palette::Hsla::from_color(rgba);

    let hsla = gpui::hsla(
        hsla.hue.into_positive_degrees() / 360.,
        hsla.saturation,
        hsla.lightness,
        hsla.alpha,
    );

    Ok(hsla)
}

impl ThemeContent {
    /// Returns a [`ThemeColorsRefinement`] based on the colors in the [`ThemeContent`].
    pub fn theme_colors_refinement(&self) -> ThemeColorsRefinement {
        ThemeColorsRefinement {
            border: self
                .border
                .as_ref()
                .and_then(|color| try_parse_color(&color).ok()),
            border_variant: self
                .border_variant
                .as_ref()
                .and_then(|color| try_parse_color(&color).ok()),
            border_focused: self
                .border_focused
                .as_ref()
                .and_then(|color| try_parse_color(&color).ok()),
            border_selected: self
                .border_selected
                .as_ref()
                .and_then(|color| try_parse_color(&color).ok()),
            border_transparent: self
                .border_transparent
                .as_ref()
                .and_then(|color| try_parse_color(&color).ok()),
            border_disabled: self
                .border_disabled
                .as_ref()
                .and_then(|color| try_parse_color(&color).ok()),
            elevated_surface_background: self
                .elevated_surface_background
                .as_ref()
                .and_then(|color| try_parse_color(&color).ok()),
            surface_background: self
                .surface_background
                .as_ref()
                .and_then(|color| try_parse_color(&color).ok()),
            background: self
                .background
                .as_ref()
                .and_then(|color| try_parse_color(&color).ok()),
            element_background: self
                .element_background
                .as_ref()
                .and_then(|color| try_parse_color(&color).ok()),
            element_hover: self
                .element_hover
                .as_ref()
                .and_then(|color| try_parse_color(&color).ok()),
            element_active: self
                .element_active
                .as_ref()
                .and_then(|color| try_parse_color(&color).ok()),
            element_selected: self
                .element_selected
                .as_ref()
                .and_then(|color| try_parse_color(&color).ok()),
            element_disabled: self
                .element_disabled
                .as_ref()
                .and_then(|color| try_parse_color(&color).ok()),
            drop_target_background: self
                .drop_target_background
                .as_ref()
                .and_then(|color| try_parse_color(&color).ok()),
            ghost_element_background: self
                .ghost_element_background
                .as_ref()
                .and_then(|color| try_parse_color(&color).ok()),
            ghost_element_hover: self
                .ghost_element_hover
                .as_ref()
                .and_then(|color| try_parse_color(&color).ok()),
            ghost_element_active: self
                .ghost_element_active
                .as_ref()
                .and_then(|color| try_parse_color(&color).ok()),
            ghost_element_selected: self
                .ghost_element_selected
                .as_ref()
                .and_then(|color| try_parse_color(&color).ok()),
            ghost_element_disabled: self
                .ghost_element_disabled
                .as_ref()
                .and_then(|color| try_parse_color(&color).ok()),
            text: self
                .text
                .as_ref()
                .and_then(|color| try_parse_color(&color).ok()),
            text_muted: self
                .text_muted
                .as_ref()
                .and_then(|color| try_parse_color(&color).ok()),
            text_placeholder: self
                .text_placeholder
                .as_ref()
                .and_then(|color| try_parse_color(&color).ok()),
            text_disabled: self
                .text_disabled
                .as_ref()
                .and_then(|color| try_parse_color(&color).ok()),
            text_accent: self
                .text_accent
                .as_ref()
                .and_then(|color| try_parse_color(&color).ok()),
            icon: self
                .icon
                .as_ref()
                .and_then(|color| try_parse_color(&color).ok()),
            icon_muted: self
                .icon_muted
                .as_ref()
                .and_then(|color| try_parse_color(&color).ok()),
            icon_disabled: self
                .icon_disabled
                .as_ref()
                .and_then(|color| try_parse_color(&color).ok()),
            icon_placeholder: self
                .icon_placeholder
                .as_ref()
                .and_then(|color| try_parse_color(&color).ok()),
            icon_accent: self
                .icon_accent
                .as_ref()
                .and_then(|color| try_parse_color(&color).ok()),
            status_bar_background: self
                .status_bar_background
                .as_ref()
                .and_then(|color| try_parse_color(&color).ok()),
            title_bar_background: self
                .title_bar_background
                .as_ref()
                .and_then(|color| try_parse_color(&color).ok()),
            toolbar_background: self
                .toolbar_background
                .as_ref()
                .and_then(|color| try_parse_color(&color).ok()),
            tab_bar_background: self
                .tab_bar_background
                .as_ref()
                .and_then(|color| try_parse_color(&color).ok()),
            tab_inactive_background: self
                .tab_inactive_background
                .as_ref()
                .and_then(|color| try_parse_color(&color).ok()),
            tab_active_background: self
                .tab_active_background
                .as_ref()
                .and_then(|color| try_parse_color(&color).ok()),
            search_match_background: self
                .search_match_background
                .as_ref()
                .and_then(|color| try_parse_color(&color).ok()),
            panel_background: self
                .panel_background
                .as_ref()
                .and_then(|color| try_parse_color(&color).ok()),
            panel_focused_border: self
                .panel_focused_border
                .as_ref()
                .and_then(|color| try_parse_color(&color).ok()),
            pane_focused_border: self
                .pane_focused_border
                .as_ref()
                .and_then(|color| try_parse_color(&color).ok()),
            scrollbar_thumb_background: self
                .scrollbar_thumb_background
                .as_ref()
                .and_then(|color| try_parse_color(&color).ok()),
            scrollbar_thumb_hover_background: self
                .scrollbar_thumb_hover_background
                .as_ref()
                .and_then(|color| try_parse_color(&color).ok()),
            scrollbar_thumb_border: self
                .scrollbar_thumb_border
                .as_ref()
                .and_then(|color| try_parse_color(&color).ok()),
            scrollbar_track_background: self
                .scrollbar_track_background
                .as_ref()
                .and_then(|color| try_parse_color(&color).ok()),
            scrollbar_track_border: self
                .scrollbar_track_border
                .as_ref()
                .and_then(|color| try_parse_color(&color).ok()),
            editor_foreground: self
                .editor_foreground
                .as_ref()
                .and_then(|color| try_parse_color(&color).ok()),
            editor_background: self
                .editor_background
                .as_ref()
                .and_then(|color| try_parse_color(&color).ok()),
            editor_gutter_background: self
                .editor_gutter_background
                .as_ref()
                .and_then(|color| try_parse_color(&color).ok()),
            editor_subheader_background: self
                .editor_subheader_background
                .as_ref()
                .and_then(|color| try_parse_color(&color).ok()),
            editor_active_line_background: self
                .editor_active_line_background
                .as_ref()
                .and_then(|color| try_parse_color(&color).ok()),
            editor_highlighted_line_background: self
                .editor_highlighted_line_background
                .as_ref()
                .and_then(|color| try_parse_color(&color).ok()),
            editor_line_number: self
                .editor_line_number
                .as_ref()
                .and_then(|color| try_parse_color(&color).ok()),
            editor_active_line_number: self
                .editor_active_line_number
                .as_ref()
                .and_then(|color| try_parse_color(&color).ok()),
            editor_invisible: self
                .editor_invisible
                .as_ref()
                .and_then(|color| try_parse_color(&color).ok()),
            editor_wrap_guide: self
                .editor_wrap_guide
                .as_ref()
                .and_then(|color| try_parse_color(&color).ok()),
            editor_active_wrap_guide: self
                .editor_active_wrap_guide
                .as_ref()
                .and_then(|color| try_parse_color(&color).ok()),
            editor_document_highlight_read_background: self
                .editor_document_highlight_read_background
                .as_ref()
                .and_then(|color| try_parse_color(&color).ok()),
            editor_document_highlight_write_background: self
                .editor_document_highlight_write_background
                .as_ref()
                .and_then(|color| try_parse_color(&color).ok()),
            terminal_background: self
                .terminal_background
                .as_ref()
                .and_then(|color| try_parse_color(&color).ok()),
            terminal_foreground: self
                .terminal_foreground
                .as_ref()
                .and_then(|color| try_parse_color(&color).ok()),
            terminal_bright_foreground: self
                .terminal_bright_foreground
                .as_ref()
                .and_then(|color| try_parse_color(&color).ok()),
            terminal_dim_foreground: self
                .terminal_dim_foreground
                .as_ref()
                .and_then(|color| try_parse_color(&color).ok()),
            terminal_ansi_black: self
                .terminal_ansi_black
                .as_ref()
                .and_then(|color| try_parse_color(&color).ok()),
            terminal_ansi_bright_black: self
                .terminal_ansi_bright_black
                .as_ref()
                .and_then(|color| try_parse_color(&color).ok()),
            terminal_ansi_dim_black: self
                .terminal_ansi_dim_black
                .as_ref()
                .and_then(|color| try_parse_color(&color).ok()),
            terminal_ansi_red: self
                .terminal_ansi_red
                .as_ref()
                .and_then(|color| try_parse_color(&color).ok()),
            terminal_ansi_bright_red: self
                .terminal_ansi_bright_red
                .as_ref()
                .and_then(|color| try_parse_color(&color).ok()),
            terminal_ansi_dim_red: self
                .terminal_ansi_dim_red
                .as_ref()
                .and_then(|color| try_parse_color(&color).ok()),
            terminal_ansi_green: self
                .terminal_ansi_green
                .as_ref()
                .and_then(|color| try_parse_color(&color).ok()),
            terminal_ansi_bright_green: self
                .terminal_ansi_bright_green
                .as_ref()
                .and_then(|color| try_parse_color(&color).ok()),
            terminal_ansi_dim_green: self
                .terminal_ansi_dim_green
                .as_ref()
                .and_then(|color| try_parse_color(&color).ok()),
            terminal_ansi_yellow: self
                .terminal_ansi_yellow
                .as_ref()
                .and_then(|color| try_parse_color(&color).ok()),
            terminal_ansi_bright_yellow: self
                .terminal_ansi_bright_yellow
                .as_ref()
                .and_then(|color| try_parse_color(&color).ok()),
            terminal_ansi_dim_yellow: self
                .terminal_ansi_dim_yellow
                .as_ref()
                .and_then(|color| try_parse_color(&color).ok()),
            terminal_ansi_blue: self
                .terminal_ansi_blue
                .as_ref()
                .and_then(|color| try_parse_color(&color).ok()),
            terminal_ansi_bright_blue: self
                .terminal_ansi_bright_blue
                .as_ref()
                .and_then(|color| try_parse_color(&color).ok()),
            terminal_ansi_dim_blue: self
                .terminal_ansi_dim_blue
                .as_ref()
                .and_then(|color| try_parse_color(&color).ok()),
            terminal_ansi_magenta: self
                .terminal_ansi_magenta
                .as_ref()
                .and_then(|color| try_parse_color(&color).ok()),
            terminal_ansi_bright_magenta: self
                .terminal_ansi_bright_magenta
                .as_ref()
                .and_then(|color| try_parse_color(&color).ok()),
            terminal_ansi_dim_magenta: self
                .terminal_ansi_dim_magenta
                .as_ref()
                .and_then(|color| try_parse_color(&color).ok()),
            terminal_ansi_cyan: self
                .terminal_ansi_cyan
                .as_ref()
                .and_then(|color| try_parse_color(&color).ok()),
            terminal_ansi_bright_cyan: self
                .terminal_ansi_bright_cyan
                .as_ref()
                .and_then(|color| try_parse_color(&color).ok()),
            terminal_ansi_dim_cyan: self
                .terminal_ansi_dim_cyan
                .as_ref()
                .and_then(|color| try_parse_color(&color).ok()),
            terminal_ansi_white: self
                .terminal_ansi_white
                .as_ref()
                .and_then(|color| try_parse_color(&color).ok()),
            terminal_ansi_bright_white: self
                .terminal_ansi_bright_white
                .as_ref()
                .and_then(|color| try_parse_color(&color).ok()),
            terminal_ansi_dim_white: self
                .terminal_ansi_dim_white
                .as_ref()
                .and_then(|color| try_parse_color(&color).ok()),
            link_text_hover: self
                .link_text_hover
                .as_ref()
                .and_then(|color| try_parse_color(&color).ok()),
        }
    }
}
