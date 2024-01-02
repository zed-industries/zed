use assets::Assets;
use clap::Parser;
use gpui::{
    px, App, Bounds, Element, IntoElement, Render, Size, VisualContext, WindowBounds, WindowOptions,
};
use log::LevelFilter;
use settings::Settings;
use simplelog::SimpleLogger;
use std::sync::Arc;
use theme::{LoadThemes, ThemeRegistry, ThemeSettings};
use workspace2_ui::{Branches, PeerId, ProjectHost, Projects, Titlebar};

#[derive(Parser)]
struct Args {
    theme: Option<String>,
}

fn main() {
    SimpleLogger::init(LevelFilter::Info, Default::default()).unwrap();

    let args = Args::parse();
    let theme_name = args.theme.unwrap_or("One Light".to_string());

    let assets = Arc::new(Assets);
    App::production(assets.clone()).run(move |cx| {
        assets.load_embedded_fonts(cx);
        settings::init(cx);
        theme::init(LoadThemes::All, cx);

        ThemeSettings::override_global_with(cx, |settings, cx| {
            settings.active_theme = ThemeRegistry::global(cx).get(&theme_name).unwrap()
        });

        cx.open_window(
            WindowOptions {
                bounds: WindowBounds::Fixed(Bounds {
                    origin: Default::default(),
                    size: Size {
                        width: px(1500.),
                        height: px(780.),
                    }
                    .into(),
                }),
                ..Default::default()
            },
            move |cx| {
                let ui_font_size = ThemeSettings::get_global(cx).ui_font_size;
                cx.set_rem_size(ui_font_size);
                cx.new_view(|_cx| TitlebarExample)
            },
        );

        cx.activate(true);
    })
}

struct TitlebarExample;

impl Render for TitlebarExample {
    fn render(&mut self, cx: &mut ui::prelude::ViewContext<Self>) -> impl Element {
        let delegate = cx.new_view(|_| ());

        Titlebar {
            delegate: delegate.clone(),
            full_screen: false,
            project_host: Some(ProjectHost {
                delegate: delegate.clone(),
                id: PeerId(1),
                login: "nathansobo".into(),
                peer_index: 0,
            }),
            projects: Projects {
                delegate: delegate.clone(),
                current: "zed".into(),
                recent: vec![],
            },
            branches: Some(Branches {
                current: "main".into(),
            }),
            collaborators: vec![],
        }
        .into_element()
    }
}
