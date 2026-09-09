#![cfg(target_os = "macos")]
//! macOS platform implementation for GPUI.
//!
//! macOS screens have a y axis that goes up from the bottom of the screen and
//! an origin at the bottom left of the main display.

mod dispatcher;
mod display;
mod display_link;
mod events;
mod keyboard;
mod pasteboard;
mod system_notifications;

#[cfg(feature = "screen-capture")]
mod screen_capture;

use gpui_apple::metal_renderer as renderer;

pub mod metal_renderer {
    pub use gpui_apple::metal_renderer::{PathRasterizationVertex, PathSprite, SurfaceBounds};

    #[cfg(any(test, feature = "bench-support", feature = "test-support"))]
    pub use gpui_apple::metal_renderer::MetalHeadlessRenderer;
}

#[cfg(feature = "font-kit")]
mod open_type;

#[cfg(feature = "font-kit")]
mod text_system;

mod platform;
mod window;
mod window_appearance;

use cocoa::{
    base::{id, nil},
    foundation::{NSAutoreleasePool, NSNotFound, NSString, NSUInteger},
};

use objc::runtime::{BOOL, NO, YES};
use std::{
    ffi::{CStr, c_char},
    ops::Range,
};

pub(crate) use dispatcher::*;
pub(crate) use display::*;
pub(crate) use display_link::*;
pub(crate) use keyboard::*;
pub(crate) use platform::*;
pub(crate) use window::*;

#[cfg(feature = "font-kit")]
pub(crate) use text_system::*;

pub use platform::MacPlatform;

trait BoolExt {
    fn to_objc(self) -> BOOL;
}

impl BoolExt for bool {
    fn to_objc(self) -> BOOL {
        if self { YES } else { NO }
    }
}

trait NSStringExt {
    unsafe fn to_str(&self) -> &str;
}

impl NSStringExt for id {
    unsafe fn to_str(&self) -> &str {
        unsafe {
            let cstr = self.UTF8String();
            if cstr.is_null() {
                ""
            } else {
                CStr::from_ptr(cstr as *mut c_char).to_str().unwrap()
            }
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
struct NSRange {
    pub location: NSUInteger,
    pub length: NSUInteger,
}

impl NSRange {
    fn invalid() -> Self {
        Self {
            location: NSNotFound as NSUInteger,
            length: 0,
        }
    }

    fn is_valid(&self) -> bool {
        self.location != NSNotFound as NSUInteger
    }

    fn to_range(self) -> Option<Range<usize>> {
        if self.is_valid() {
            let start = self.location as usize;
            let end = start + self.length as usize;
            Some(start..end)
        } else {
            None
        }
    }
}

impl From<Range<usize>> for NSRange {
    fn from(range: Range<usize>) -> Self {
        NSRange {
            location: range.start as NSUInteger,
            length: range.len() as NSUInteger,
        }
    }
}

unsafe impl objc::Encode for NSRange {
    fn encode() -> objc::Encoding {
        let encoding = format!(
            "{{NSRange={}{}}}",
            NSUInteger::encode().as_str(),
            NSUInteger::encode().as_str()
        );
        unsafe { objc::Encoding::from_str(&encoding) }
    }
}

/// Allow NSString::alloc use here because it sets autorelease
#[allow(clippy::disallowed_methods)]
unsafe fn ns_string(string: &str) -> id {
    unsafe { NSString::alloc(nil).init_str(string).autorelease() }
}

#[cfg(all(test, feature = "test-support", feature = "font-kit"))]
mod view_tree_pixel_tests {
    #[test]
    #[ignore = "requires a Metal device"]
    fn view_tree_scene_matches_full_refresh_pixels() {
        use gpui::{
            AppContext as _, Context, Entity, HeadlessAppContext, IntoElement, ParentElement,
            Render, Styled, Window, div, px, rgb, size,
        };

        struct Tile(u32);
        impl Render for Tile {
            fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
                div()
                    .w(px(140.))
                    .h(px(90.))
                    .rounded_lg()
                    .border_2()
                    .border_color(rgb(0xffffff))
                    .bg(rgb(self.0))
                    .opacity(0.8)
                    .text_color(rgb(0xffffff))
                    .child("Memoized λ")
                    .child(
                        gpui::canvas(
                            |_, _, _| (),
                            |bounds, _, window, _| {
                                let mut path = gpui::PathBuilder::fill();
                                path.move_to(bounds.origin);
                                path.line_to(bounds.bottom_right());
                                path.line_to(bounds.bottom_left());
                                path.close();
                                window.paint_path(path.build().expect("triangle"), rgb(0xffbb22));
                            },
                        )
                        .w(px(80.))
                        .h(px(30.)),
                    )
            }
        }
        struct Root(Vec<Entity<Tile>>);
        impl Render for Root {
            fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
                div()
                    .size_full()
                    .bg(rgb(0x182030))
                    .flex()
                    .gap_3()
                    .overflow_hidden()
                    .children(self.0.iter().cloned())
            }
        }
        let mut cx = HeadlessAppContext::with_platform(
            std::sync::Arc::new(crate::MacTextSystem::new()),
            std::sync::Arc::new(()),
            || Some(Box::new(crate::metal_renderer::MetalHeadlessRenderer::new())),
        );
        let first = cx.new(|_| Tile(0x883344));
        let second = cx.new(|_| Tile(0x338844));
        let window = cx
            .open_window(size(px(400.), px(200.)), |_, cx| {
                cx.new(|_| Root(vec![first.clone(), second]))
            })
            .expect("offscreen window");
        cx.run_until_parked();
        let mut reused = 0;
        for step in 0..8 {
            first.update(&mut cx, |tile, cx| {
                tile.0 += 0x030201;
                cx.notify();
            });
            window
                .update(&mut cx, |root, window, cx| {
                    if step == 3 {
                        root.0.reverse();
                        cx.notify();
                    }
                    if step == 5 {
                        window.resize(size(px(220.), px(140.)));
                    }
                    if step == 7 {
                        root.0.retain(|tile| tile != &first);
                        cx.notify();
                    }
                })
                .expect("update tiles");
            cx.run_until_parked();
            cx.update_window(window.into(), |_, window, cx| {
                window.draw(cx).clear(cx);
                reused += window.view_tree_stats().reused_subtrees;
            })
            .expect("draw incremental scene");
            let incremental = cx
                .capture_screenshot(window.into())
                .expect("Metal readback");
            assert!(
                incremental
                    .pixels()
                    .any(|pixel| pixel != incremental.get_pixel(0, 0)),
                "nontrivial GPU output"
            );
            cx.update_window(window.into(), |_, window, cx| {
                window.refresh();
                window.draw(cx).clear(cx);
            })
            .expect("draw reference scene");
            let reference = cx
                .capture_screenshot(window.into())
                .expect("reference Metal readback");
            assert!(incremental == reference, "GPU pixels differ at step {step}");
        }
        assert!(reused > 0, "pixel oracle must exercise node reuse");
    }
}
