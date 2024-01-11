mod support;
use picker::*;
use support::*;

// Today, I want to walk through building a tiny subset of Zed's interface in the latest version of our UI framework, [GPUI](https://gpui.rs).
// I'll start with a simple version of the titlebar at the top of the window.
//
// To start, let's think about modeling the above scenario in Rust's type system.
// First, we'll import everything in GPUI so it's in scope for the rest of the post, along with types from a few other Zed crates.

use gpui::{prelude::*, *};
use theme::*;
use ui::*;

// Next we'll define a `Titlebar` struct, which contains a `ProjectMenuButton`.
// The project menu button displays the name of the current project, and a menu of recent projects when clicked.
#[derive(IntoElement)]
pub struct Titlebar {
    project_menu_button: ProjectMenuButton,
}

// Note the use of the IntoElement, derive macro above.
// This trait allows our struct to be converted into an element for display in a window.
// To derive it, your type must implement the `RenderOnce` trait, which we'll discuss shortly.

// The project menu button has a current project and a vector of recent projects, where every project has a name and an id.
// Deeper in the app, we have another definition of `Project`, but this is the only data we need to render the interface.
// Structuring the view this way will allow us to iterate rapidly on what our app looks like without pulling in too many expensive dependencies.

#[derive(IntoElement)]
pub struct ProjectMenuButton {
    current: Project,
    recent: Vec<Project>,
}

pub struct Project {
    name: SharedString,
    id: ProjectId,
}

#[derive(Clone)]
pub struct ProjectId(u64);

// As I mentioned earlier, in order to derive `IntoElement` on a type, that type must implement `RenderOnce`.
// Lets do that now for titlebar. Here's the scaffold.
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
// We could have adopted a fancy macro language, but we wanted to keep things simple and stick to pure Rust.
// It's nice to express visual structure in the same language as data, and I find the approach no less readable than something like HTML, assuming you know a bit of Rust syntax.
// Individual elements are constructed via method chaining, and we've adopted Tailwind CSS naming conventions for a set of chained helper methods.
// When you apply a "classes" to an element like `Div` by chaining methods such as `flex` or `w_full`, we update its style properties.
// These styling properties are then used to perform a web-compatible layout on the tree of elements, with the help of the excellent [taffy](https://github.com/DioxusLabs/taffy) crate.

impl RenderOnce for Titlebar {
    fn render(self, cx: &mut gpui::WindowContext) -> impl IntoElement {
        div()
            .id("titlebar")
            .flex()
            .flex_row()
            .items_center()
            .w_full()
            .h(rems(1.75))
            .min_h(px(32.))
            .bg(cx.theme().colors().title_bar_background)
            .map(|this| {
                if matches!(cx.window_bounds(), WindowBounds::Fullscreen) {
                    this.pl_2()
                } else {
                    this.pl(px(80.))
                }
            })
            .on_click(|event, cx| {
                if event.up.click_count == 2 {
                    cx.zoom_window();
                }
            })
            .child(self.project_menu_button)
    }
}

// In the implementation above, we make titlebar a horizontal flex row and center its items vertically.
// We make it full width, and give it a scalable height based on *rems*, which is short for "root ems", another concept we've borrowed from the web.
// One *rem* essentially the font size of the root element, so if the root font size of the window is 16px, then 1rem = 16px.
// We define almost everything in terms of rems, allowing the UI to scale by tweaking a single parameter.
// A more principled approach to our units of measure is one of the big benefits we gained by rewriting GPUI, which is why the UI font is now scalable in preview.
//
// Interestingly, the titlebar is one place where we use a non-scalable pixel-based value in our UI, in this case for the minimum height.
// This is because the titlebar must coexist with the only UI element in the window we don't control: the macOS traffic lights.
// The traffic lights have a fixed size, and we want to ensure the titlebar is tall enough to fully contain them, regardless of the window's root font size.
// After assigning the background color, we use conditional logic within the call to `map` to apply fixed padding to the left side of the titlebar whenever the traffic lights are visible.
//
// We also arrange to zoom the window when the titlebar is double-clicked.
// Finally, we give our titlebar a single child, the project menu button.
// Note that because `RenderOnce::render` moves self, we're free to move the project menu button rather than cloning it.

// Now let's implement `RenderOnce` for `ProjectMenuButton`

impl RenderOnce for ProjectMenuButton {
    fn render(self, cx: &mut WindowContext) -> impl IntoElement {
        PopoverMenu::new("project-menu")
            .trigger(
                Button::new("project-menu-button", self.current.name)
                    .style(ButtonStyle::Subtle)
                    .label_size(LabelSize::Small)
                    .tooltip(move |cx| Tooltip::text("Recent Projects", cx)),
            )
            .menu(move |cx| {
                Some(cx.new_view(|cx| {
                    Picker::fuzzy(
                        self.recent
                            .iter()
                            .map(|project| FuzzyPickerItem {
                                id: project.id.clone(),
                                name: project.name.clone(),
                            })
                            .collect(),
                        cx,
                        |_, _, _| div(),
                    )
                }))
            })
        // TODO: Build an easier picker we can just supply with data.
    }
}

// ------------------------------------

actions!(
    zed,
    [OpenSettings, OpenTheme, SignIn, SignOut, ShareFeedback]
);

#[derive(IntoElement)]
pub struct UserMenuButton {
    avatar_url: Option<SharedUrl>,
}

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
                                .child(Icon::new(IconName::ChevronDown).color(Color::Muted)),
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
                                .child(Icon::new(IconName::ChevronDown).color(Color::Muted)),
                        )
                        .style(ButtonStyle::Subtle)
                        .tooltip(move |cx| Tooltip::text("Toggle User Menu", cx)),
                )
        }
    }
}
