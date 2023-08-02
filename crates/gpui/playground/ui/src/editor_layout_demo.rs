use gpui::{AnyElement, Element, LayoutContext, View, ViewContext};

#[derive(Element, Clone, Default)]
pub struct Playground<V: View>(PhantomData<V>);

// example layout design here: https://www.figma.com/file/5QLTmxjO0xQpDD3CD4hR6T/Untitled?type=design&node-id=0%3A1&mode=design&t=SoJieVVIvDDDKagv-1

impl<V: View> Playground<V> {
    pub fn render(&mut self, _: &mut V, _: &mut gpui::ViewContext<V>) -> impl Element<V> {
        col() // fullscreen container with header and main in it
            .width(flex(1.))
            .height(flex(1.))
            .fill(colors(gray.900))
            .children([
                row() // header container
                    .fill(colors(gray.900))
                    .width(flex(1.))
                    .children([
                        row() // tab bar
                            .width(flex(1.))
                            .gap(spacing(2))
                            .padding(spacing(3))
                            .overflow_x(scroll())
                            .chidren([
                                row() // tab
                                    .padding_x(spacing(3))
                                    .padding_y(spacing(2))
                                    .corner_radius(6.)
                                    .gap(spacing(3))
                                    .align(center())
                                    .fill(colors(gray.800))
                                    .children([text("Tab title 1"), svg("icon_name")]),
                                row() // tab
                                    .padding_x(spacing(3))
                                    .padding_y(spacing(2))
                                    .corner_radius(6.)
                                    .gap(spacing(3))
                                    .align(center())
                                    .fill(colors(gray.800))
                                    .children([text("Tab title 2"), svg("icon_name")]),
                                row() // tab
                                    .padding_x(spacing(3))
                                    .padding_y(spacing(2))
                                    .corner_radius(6.)
                                    .gap(spacing(3))
                                    .align(center())
                                    .fill(colors(gray.800))
                                    .children([text("Tab title 3"), svg("icon_name")]),
                            ]),
                        row() // tab bar actions
                            .border_left(colors(gray.700))
                            .gap(spacing(2))
                            .padding(spacing(3))
                            .chidren([
                                row()
                                    .width(spacing(8))
                                    .height(spacing(8))
                                    .corner_radius(6.)
                                    .justify(center())
                                    .align(center())
                                    .fill(colors(gray.800))
                                    .child(svg(icon_name)),
                                row()
                                    .width(spacing(8))
                                    .height(spacing(8))
                                    .corner_radius(6.)
                                    .justify(center())
                                    .align(center())
                                    .fill(colors(gray.800))
                                    .child(svg(icon_name)),
                                row()
                                    .width(spacing(8))
                                    .height(spacing(8))
                                    .corner_radius(6.)
                                    .justify(center())
                                    .align(center())
                                    .fill(colors(gray.800))
                                    .child(svg(icon_name)),
                            ]),
                    ]),
                row() // main container
                    .width(flex(1.))
                    .height(flex(1.))
                    .children([
                        col() // left sidebar
                            .fill(colors(gray.800))
                            .border_right(colors(gray.700))
                            .height(flex(1.))
                            .width(260.)
                            .children([
                                col() // containter to hold list items and notification alert box
                                    .justify(between())
                                    .padding_x(spacing(6))
                                    .padding_bottom(3)
                                    .padding_top(spacing(6))
                                    .children([
                                        col().gap(spacing(3)).children([ // sidebar list
                                            text("Item"),
                                            text("Item"),
                                            text("Item"),
                                            text("Item"),
                                            text("Item"),
                                            text("Item"),
                                            text("Item"),
                                            text("Item"),
                                        ]),
                                        col().align(center()).gap(spacing(1)).children([ // notification alert box
                                            text("Title text").size("lg"),
                                            text("Description text goes here")
                                                .text_color(colors(rose.200)),
                                        ]),
                                    ]),
                                row()
                                    .padding_x(spacing(3))
                                    .padding_y(spacing(2))
                                    .border_top(1., colors(gray.700))
                                    .align(center())
                                    .gap(spacing(2))
                                    .fill(colors(gray.900))
                                    .children([
                                        row() // avatar container
                                            .width(spacing(8))
                                            .height(spacing(8))
                                            .corner_radius(spacing(8))
                                            .justify(center())
                                            .align(center())
                                            .child(image(image_url)),
                                        text("FirstName Lastname"), // user name
                                    ]),
                            ]),
                        col() // primary content container
                            .align(center())
                            .justify(center())
                            .child(
                                col().justify(center()).gap(spacing(8)).children([ // detail container wrapper for center positioning
                                    col() // blue rectangle
                                        .width(rem(30.))
                                        .height(rem(20.))
                                        .corner_radius(16.)
                                        .fill(colors(blue.200)),
                                    col().gap(spacing(1)).children([ // center content text items
                                        text("This is a title").size("lg"),
                                        text("This is a description").text_color(colors(gray.500)),
                                    ]),
                                ]),
                            ),
                        col(), // right sidebar
                    ]),
            ])
    }
}

// row(
//     padding(),
//     width(),
//     fill(),
// )

// .width(flex(1.))
// .height(flex(1.))
// .justify(end())
// .align(start()) // default
// .fill(green)
// .child(other_tab_bar())
// .child(profile_menu())
