use gpui2::{rgb, Hsla};

pub fn mac_os_traffic_light_red() -> Hsla {
    rgb::<Hsla>(0xEC695E)
}
pub fn mac_os_traffic_light_yellow() -> Hsla {
    rgb::<Hsla>(0xF4BF4F)
}
pub fn mac_os_traffic_light_green() -> Hsla {
    rgb::<Hsla>(0x62C554)
}

pub struct UIColors {
    pub transparent: Hsla,
    pub mac_os_traffic_light_red: Hsla,
    pub mac_os_traffic_light_yellow: Hsla,
    pub mac_os_traffic_light_green: Hsla,
    pub border: Hsla,
    pub border_variant: Hsla,
    pub border_focused: Hsla,
    pub border_transparent: Hsla,
    pub elevated_surface: Hsla,
    pub surface: Hsla,
    pub background: Hsla,
    pub element: Hsla,
    pub element_hover: Hsla,
    pub element_active: Hsla,
    pub element_selected: Hsla,
    pub element_disabled: Hsla,
    pub element_placeholder: Hsla,
    pub ghost_element: Hsla,
    pub ghost_element_hover: Hsla,
    pub ghost_element_active: Hsla,
    pub ghost_element_selected: Hsla,
    pub ghost_element_disabled: Hsla,
    pub text: Hsla,
    pub text_muted: Hsla,
    pub text_placeholder: Hsla,
    pub text_disabled: Hsla,
    pub text_accent: Hsla,
    pub icon: Hsla,
    pub icon_muted: Hsla,
    pub icon_disabled: Hsla,
    pub icon_placeholder: Hsla,
    pub icon_accent: Hsla,
    pub syntax: SyntaxColor,
    pub status_bar: Hsla,
    pub title_bar: Hsla,
    pub toolbar: Hsla,
    pub tab_bar: Hsla,
    pub editor: Hsla,
    pub editor_subheader: Hsla,
    pub editor_active_line: Hsla,
    pub terminal: Hsla,
    pub created: Hsla,
    pub modified: Hsla,
    pub deleted: Hsla,
    pub conflict: Hsla,
    pub hidden: Hsla,
    pub ignored: Hsla,
    pub renamed: Hsla,
    pub error: Hsla,
    pub warning: Hsla,
    pub info: Hsla,
    pub success: Hsla,
    pub git_created: Hsla,
    pub git_modified: Hsla,
    pub git_deleted: Hsla,
    pub git_conflict: Hsla,
    pub git_ignored: Hsla,
    pub git_renamed: Hsla,
    pub player: [PlayerColor; 8],
}
