#![allow(missing_docs)]

use gpui::{Hsla, hsla};

#[derive(Clone, PartialEq)]
pub struct SystemColors {
    pub transparent: Hsla,
    pub mac_os_traffic_light_red: Hsla,
    pub mac_os_traffic_light_yellow: Hsla,
    pub mac_os_traffic_light_green: Hsla,
}

impl Default for SystemColors {
    fn default() -> Self {
        Self {
            transparent: hsla(0.0, 0.0, 0.0, 0.0),
            mac_os_traffic_light_red: hsla(0.0139, 0.79, 0.65, 1.0),
            mac_os_traffic_light_yellow: hsla(0.114, 0.88, 0.63, 1.0),
            mac_os_traffic_light_green: hsla(0.313, 0.49, 0.55, 1.0),
        }
    }
}
