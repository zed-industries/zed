use gpui::{
    App, Bounds, Context, Render, Window, WindowBackgroundAppearance, WindowBounds, WindowOptions,
    div, prelude::*, px, rgb, size,
};
use gpui_platform::application;

#[derive(Clone, Copy)]
enum Palette {
    Target,
    Helper,
    Alternate,
}

impl Palette {
    fn from_name(name: &str) -> Option<Self> {
        match name {
            "target" => Some(Self::Target),
            "helper" => Some(Self::Helper),
            "alternate" => Some(Self::Alternate),
            _ => None,
        }
    }

    fn colors(self) -> [u32; 5] {
        match self {
            Self::Target => [0x172554, 0x06b6d4, 0xfacc15, 0xef4444, 0x22c55e],
            Self::Helper => [0x3b0764, 0xa855f7, 0xf0abfc, 0x7e22ce, 0xc084fc],
            Self::Alternate => [0x450a0a, 0xf97316, 0xfef08a, 0xdc2626, 0xfb923c],
        }
    }
}

struct Fixture {
    palette: Palette,
}

impl Render for Fixture {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let [background, left, upper, lower, right] = self.palette.colors();

        div()
            .size_full()
            .flex()
            .gap_8()
            .p_8()
            .bg(rgb(background))
            .child(div().w_1_4().h_full().bg(rgb(left)))
            .child(
                div()
                    .flex_1()
                    .h_full()
                    .flex()
                    .flex_col()
                    .gap_8()
                    .child(div().h_1_2().w_full().bg(rgb(upper)))
                    .child(div().flex_1().w_full().bg(rgb(lower))),
            )
            .child(div().w_1_4().h_full().bg(rgb(right)))
    }
}

fn parse_arguments() -> Result<(String, Palette), String> {
    let mut title = "GPUI X11 Workspace Redraw".to_string();
    let mut palette = Palette::Target;
    let mut arguments = std::env::args().skip(1);

    while let Some(argument) = arguments.next() {
        match argument.as_str() {
            "--title" => {
                title = arguments
                    .next()
                    .ok_or_else(|| "--title requires a value".to_string())?;
            }
            "--palette" => {
                let name = arguments
                    .next()
                    .ok_or_else(|| "--palette requires a value".to_string())?;
                palette =
                    Palette::from_name(&name).ok_or_else(|| format!("unknown palette: {name}"))?;
            }
            _ => return Err(format!("unknown argument: {argument}")),
        }
    }

    Ok((title, palette))
}

fn main() {
    env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .init();

    let (title, palette) = match parse_arguments() {
        Ok(arguments) => arguments,
        Err(error) => {
            eprintln!("error: {error}");
            std::process::exit(2);
        }
    };

    application().run(move |cx: &mut App| {
        let bounds = Bounds::centered(None, size(px(600.0), px(700.0)), cx);
        if let Err(error) = cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                titlebar: Some(gpui::TitlebarOptions {
                    title: Some(title.clone().into()),
                    ..Default::default()
                }),
                window_background: WindowBackgroundAppearance::Opaque,
                app_id: Some("gpui-x11-workspace-redraw".to_string()),
                ..Default::default()
            },
            |_, cx| cx.new(|_| Fixture { palette }),
        ) {
            eprintln!("failed to open fixture window: {error:#}");
            cx.quit();
        }
    });
}
