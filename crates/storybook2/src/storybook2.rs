#![allow(dead_code, unused_variables)]

mod assets;
mod collab_panel;
mod stories;
mod story;
mod story_selector;
mod themes;
mod workspace;

use std::sync::Arc;

use clap::Parser;
use gpui3::{
    div, px, size, view, Bounds, Context, Element, ViewContext, WindowBounds, WindowOptions,
};
use log::LevelFilter;
use simplelog::SimpleLogger;
use ui::prelude::*;
use ui::themed;

use crate::assets::Assets;
use crate::story_selector::StorySelector;
use crate::themes::rose_pine_dawn;
use crate::workspace::workspace;

// gpui2::actions! {
//     storybook,
//     [ToggleInspector]
// }

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(value_enum)]
    story: Option<StorySelector>,

    /// The name of the theme to use in the storybook.
    ///
    /// If not provided, the default theme will be used.
    #[arg(long)]
    theme: Option<String>,
}

fn main() {
    // unsafe { backtrace_on_stack_overflow::enable() };

    SimpleLogger::init(LevelFilter::Info, Default::default()).expect("could not initialize logger");

    let args = Args::parse();

    let story_selector = args.story.clone();

    let asset_source = Arc::new(Assets);
    gpui3::App::production(asset_source).run(move |cx| {
        match story_selector {
            Some(selector) => {
                let window = cx.open_window(
                    WindowOptions {
                        bounds: WindowBounds::Fixed(Bounds {
                            origin: Default::default(),
                            size: size(px(1700.), px(980.)).into(),
                        }),
                        ..Default::default()
                    },
                    move |cx| {
                        view(
                            cx.entity(|cx| StoryWrapper::new(selector)),
                            StoryWrapper::render,
                        )
                    },
                );
            }
            None => {
                let window = cx.open_window(
                    WindowOptions {
                        bounds: WindowBounds::Fixed(Bounds {
                            origin: Default::default(),
                            size: size(px(800.), px(600.)).into(),
                        }),
                        ..Default::default()
                    },
                    |cx| workspace(cx),
                );
            }
        };

        cx.activate(true);
    });
}

#[derive(Clone)]
pub struct StoryWrapper {
    selector: StorySelector,
}

impl StoryWrapper {
    pub(crate) fn new(selector: StorySelector) -> Self {
        Self { selector }
    }

    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl Element<State = Self> {
        themed(rose_pine_dawn(), cx, |cx| {
            div()
                .flex()
                .flex_col()
                .size_full()
                .child_any(self.selector.story())
        })
    }
}

// fn load_embedded_fonts(platform: &dyn gpui2::Platform) {
//     let font_paths = Assets.list("fonts");
//     let mut embedded_fonts = Vec::new();
//     for font_path in &font_paths {
//         if font_path.ends_with(".ttf") {
//             let font_path = &*font_path;
//             let font_bytes = Assets.load(font_path).unwrap().to_vec();
//             embedded_fonts.push(Arc::from(font_bytes));
//         }
//     }
//     platform.fonts().add_fonts(&embedded_fonts).unwrap();
// }
