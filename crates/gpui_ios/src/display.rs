use anyhow::Result;
use gpui::{Bounds, DisplayId, Pixels, PlatformDisplay, Point, Size, px};
use objc::{class, msg_send, runtime::Object, sel, sel_impl};
use uuid::Uuid;

/// Represents the iPad's screen. On iPadOS there is exactly one display per
/// UIScene; Stage Manager allows multiple scenes but each maps to its own
/// UIWindowScene with its own coordinate space.
#[derive(Debug)]
pub(crate) struct IosDisplay {
    uuid: Uuid,
    bounds: Bounds<Pixels>,
}

impl IosDisplay {
    pub fn new(bounds: Bounds<Pixels>) -> Self {
        Self {
            uuid: Uuid::new_v4(),
            bounds,
        }
    }
}

impl PlatformDisplay for IosDisplay {
    fn id(&self) -> DisplayId {
        DisplayId::new(1)
    }

    fn uuid(&self) -> Result<Uuid> {
        Ok(self.uuid)
    }

    fn bounds(&self) -> Bounds<Pixels> {
        self.bounds
    }
}

/// Returns the main screen's logical bounds and native scale via `UIScreen.mainScreen`.
///
/// On a 13" iPad Pro the logical size is 1024×1366 pt at 2× scale. Falls back
/// to those values when called before UIScreen is available (e.g., in unit tests).
pub(crate) fn main_screen_bounds_and_scale() -> (Bounds<Pixels>, f32) {
    unsafe {
        let screen: *mut Object = msg_send![class!(UIScreen), mainScreen];
        if screen.is_null() {
            let fallback = Bounds {
                origin: Point::default(),
                size: Size {
                    width: px(1024.0),
                    height: px(1366.0),
                },
            };
            return (fallback, 2.0);
        }

        let cg_rect: CGRect = msg_send![screen, bounds];
        let scale: f32 = msg_send![screen, nativeScale];

        let bounds = Bounds {
            origin: Point {
                x: px(cg_rect.origin.x as f32),
                y: px(cg_rect.origin.y as f32),
            },
            size: Size {
                width: px(cg_rect.size.width as f32),
                height: px(cg_rect.size.height as f32),
            },
        };

        (bounds, scale)
    }
}

/// CGPoint — matches the UIKit/CoreGraphics ABI on 64-bit Apple platforms.
#[repr(C)]
#[derive(Clone, Copy, Default)]
struct CGPoint {
    x: f64,
    y: f64,
}

/// CGSize — matches the UIKit/CoreGraphics ABI on 64-bit Apple platforms.
#[repr(C)]
#[derive(Clone, Copy, Default)]
struct CGSize {
    width: f64,
    height: f64,
}

/// CGRect — matches the UIKit/CoreGraphics ABI on 64-bit Apple platforms.
#[repr(C)]
#[derive(Clone, Copy, Default)]
struct CGRect {
    origin: CGPoint,
    size: CGSize,
}
