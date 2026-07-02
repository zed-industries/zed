use crate::{CGRect, id};
use anyhow::Result;
use gpui::{Bounds, DisplayId, Pixels, PlatformDisplay, point, px, size};
use objc::{class, msg_send, sel, sel_impl};
use uuid::Uuid;

#[derive(Debug)]
pub(crate) struct IosDisplay {
    bounds: Bounds<Pixels>,
}

impl IosDisplay {
    /// The device's main screen, in points.
    pub(crate) fn primary() -> Self {
        let screen_bounds: CGRect = unsafe {
            let screen: id = msg_send![class!(UIScreen), mainScreen];
            msg_send![screen, bounds]
        };
        Self {
            bounds: Bounds {
                origin: point(
                    px(screen_bounds.origin.x as f32),
                    px(screen_bounds.origin.y as f32),
                ),
                size: size(
                    px(screen_bounds.size.width as f32),
                    px(screen_bounds.size.height as f32),
                ),
            },
        }
    }
}

impl PlatformDisplay for IosDisplay {
    fn id(&self) -> DisplayId {
        DisplayId::new(0)
    }

    fn uuid(&self) -> Result<Uuid> {
        // iOS exposes no stable display identifier, and there is only one
        // display, so a fixed UUID suffices.
        Ok(Uuid::from_bytes(*b"gpui-ios-display"))
    }

    fn bounds(&self) -> Bounds<Pixels> {
        self.bounds
    }
}
