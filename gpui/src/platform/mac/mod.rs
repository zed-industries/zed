use super::*;

mod app;
mod event;

pub fn app() -> impl App {
    app::App::default()
}
