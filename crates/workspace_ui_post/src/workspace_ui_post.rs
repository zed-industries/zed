mod support;

// Today, I want to walk through building a tiny subset of Zed's UI in the latest version of our UI framework, GPUI.
// We're building a titlebar, so let's start with a `Titlebar` struct.
// If the current user is signed in, we render the avatar on the far right. Otherwise, we render a sign in button.
// If the avatar or chevron are clicked, we deploy a popover menu with a few actions.
//
// ![A screenshot of the deployed menu](https://share.cleanshot.com/Q8qzQhpF)
//
// To start, let's think about modeling the above scenario in Rust's type system.
// First, we'll import everything in GPUI so it's in scope for the rest of the post.

use gpui::{prelude::*, *};
use support::h_flex;
use theme::ActiveTheme;
use ui::{
    h_stack, popover_menu, Avatar, ButtonCommon, ButtonLike, ButtonStyle, Color, ContextMenu, Icon,
    IconPath, PopoverMenu, Tooltip,
};

// Next we'll define a titlebar struct, which for now has a single field representing the user menu button.
// In practice, we have a lot more going on in our titlebar, but let's keep it simple for now.
#[derive(IntoElement)]
pub struct Titlebar {
    user_menu_button: UserMenuButton,
}

// Note the use of the IntoElement, derive macro above.
// This trait allows our struct to be converted into an element for display
// For now, the user button just has an optional avatar URL, indicating whether the current user is signed in, and also derives this trait.
#[derive(IntoElement)]
pub struct UserMenuButton {
    avatar_url: Option<SharedUrl>,
}

// However, in order to derive `gpui::IntoElement`, your struct must implement `gpui::RenderOnce`.
// So lets do that now.
//
// ```rs
// impl RenderOnce for Titlebar {
//     fn render(self, cx: &mut gpui::WindowContext) -> impl IntoElement {
//         todo!()
//     }
// }
// ```
//
// This method moves self and returns something that can be turned into an element.
// It also takes advantage of a feature new with Rust 1.75, [*return position impl trait in trait*](https://rustc-dev-guide.rust-lang.org/return-position-impl-trait-in-trait.html).
//
// To implement render, we express a tree of elements as a method-chained Rust expression.
// We could have adopted a fancy macro language, but we wanted to keep things simple and pure Rust.
// There's something nice about using one language, rather than trying to intermingle two, and we didn't want too many macros.
//
// Individual elements are constructed via method chaining, and we've adopted Tailwind CSS naming conventions for a set of chained helper methods that style elements.
// A huge thanks to the authors of [Taffy](https://github.com/DioxusLabs/taffy), which computes a layout by interpreting these properties according to web standards.
//
// In the implementation below, we make titlebar a horizontal flex row, *center* its items vertically, and evenly distribute space *between* inflexible children.
// We define the height in terms of *rems*, which stands for "root ems", another concept we've borrowed from the web.
// It's essentially the font size of the root element, and can be used as the basis of scaling the UI.
// If we define everything relative to this root font size, then the UI can scale off this single paramater.
// A more principled approach to our units of measure is one of the big benefits we gained by rewriting GPUI, which is why the UI font is now scalable in preview.
// Interestingly, for the titlebar we also need to specify a non-scalable minimum height to ensure the titlebar is tall enough to contain the fixed-size macOS traffic lights, the only UI elements we don't render ourselves.
// We continue with a `map` statement with conditional logic to position content around the traffic lights if they are visible (when not full screen).
// Finally, we set the background color and assign an action to zoom the window on double-click.

impl RenderOnce for Titlebar {
    fn render(self, cx: &mut gpui::WindowContext) -> impl IntoElement {
        let titlebar = div()
            .id("titlebar")
            .flex() //         In practice, these three chained calls are typically
            .flex_row() //     combined in a call to `.hflex()`
            .items_center() // I'm inlining them here for exposition / clarity.
            .justify_between()
            .w_full()
            .h(rems(1.75))
            .min_h(px(32.))
            .map(|this| {
                if matches!(cx.window_bounds(), WindowBounds::Fullscreen) {
                    this.pl_2()
                } else {
                    // Use pixels here instead of a rem-based size because the macOS traffic
                    // lights are a static size, and don't scale with the rest of the UI.
                    this.pl(px(80.))
                }
            })
            .bg(cx.theme().colors().title_bar_background) // TODO: I want to come back to this and explore an idea I have for surface-based themes if there's time.
            .on_click(|event, cx| {
                if event.up.click_count == 2 {
                    cx.zoom_window();
                }
            });

        titlebar
            .child(div().id("todo"))
            .child(self.user_menu_button)
    }
}

// Are we taking method chaining too far? Maybe. It's nice to just have code be markup.

actions!(
    zed,
    [OpenSettings, OpenTheme, SignIn, SignOut, ShareFeedback]
);

impl RenderOnce for UserMenuButton {
    fn render(self, cx: &mut gpui::WindowContext) -> impl IntoElement {
        if let Some(avatar_url) = self.avatar_url {
            PopoverMenu::new("user-menu")
                .menu(|cx| {
                    ContextMenu::build(cx, |menu, _| {
                        menu.action("Settings", OpenSettings.boxed_clone())
                            .action("Theme", OpenTheme.boxed_clone())
                            .separator()
                            .action("Share Feedback", ShareFeedback.boxed_clone())
                            .action("Sign Out", SignOut.boxed_clone())
                    })
                    .into()
                })
                .trigger(
                    ButtonLike::new("user-menu")
                        .child(
                            h_flex()
                                .gap_0p5()
                                .child(Avatar::new(avatar_url.clone()))
                                .child(Icon::new(IconPath::ChevronDown).color(Color::Muted)),
                        )
                        .style(ButtonStyle::Subtle)
                        .tooltip(move |cx| Tooltip::text("Toggle User Menu", cx)),
                )
                .anchor(gpui::AnchorCorner::TopRight)
        } else {
            popover_menu("user-menu")
                .menu(|cx| {
                    ContextMenu::build(cx, |menu, _| {
                        menu.action("Settings", OpenSettings.boxed_clone())
                            .action("Theme", OpenTheme.boxed_clone())
                            .separator()
                            .action("Share Feedback", ShareFeedback.boxed_clone())
                    })
                    .into()
                })
                .trigger(
                    ButtonLike::new("user-menu")
                        .child(
                            h_stack()
                                .gap_0p5()
                                .child(Icon::new(IconPath::ChevronDown).color(Color::Muted)),
                        )
                        .style(ButtonStyle::Subtle)
                        .tooltip(move |cx| Tooltip::text("Toggle User Menu", cx)),
                )
        }
    }
}
