use gpui2::geometry::AbsoluteLength;
use gpui2::{rgb, Hsla};

#[derive(Clone, Copy)]
pub struct Token {
    pub mac_os_traffic_light_red: Hsla,
    pub mac_os_traffic_light_yellow: Hsla,
    pub mac_os_traffic_light_green: Hsla,
    pub list_indent_depth: AbsoluteLength,
}

impl Default for Token {
    fn default() -> Self {
        Self {
            mac_os_traffic_light_red: rgb::<Hsla>(0xEC695E),
            mac_os_traffic_light_yellow: rgb::<Hsla>(0xF4BF4F),
            mac_os_traffic_light_green: rgb::<Hsla>(0x62C554),
            list_indent_depth: AbsoluteLength::Rems(0.5),
        }
    }
}

pub fn token() -> Token {
    Token::default()
}
