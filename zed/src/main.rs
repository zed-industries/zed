// Allow binary to be called Zed for a nice application menu when running executable direcly
#![allow(non_snake_case)]

use fs::OpenOptions;
use log::LevelFilter;
use simplelog::SimpleLogger;
use std::{fs, path::PathBuf, sync::Arc};
use zed::{
    self, assets, editor, file_finder,
    fs::RealFs,
    language, menus, rpc, settings,
    workspace::{self, OpenParams},
    worktree::{self},
    AppState,
};
use zrpc::ForegroundRouter;

fn main() {
    init_logger();

    let app = gpui::App::new(assets::Assets).unwrap();

    let (_, settings) = settings::channel(&app.font_cache()).unwrap();
    let languages = Arc::new(language::LanguageRegistry::new());
    languages.set_theme(&settings.borrow().theme);

    let mut app_state = AppState {
        languages: languages.clone(),
        settings,
        rpc_router: Arc::new(ForegroundRouter::new()),
        rpc: rpc::Client::new(languages),
        fs: Arc::new(RealFs),
    };

    app.run(move |cx| {
        worktree::init(
            cx,
            &app_state.rpc,
            Arc::get_mut(&mut app_state.rpc_router).unwrap(),
        );
        zed::init(cx);
        workspace::init(cx);
        editor::init(cx);
        file_finder::init(cx);

        let app_state = Arc::new(app_state);
        cx.set_menus(menus::menus(&app_state.clone()));

        if stdout_is_a_pty() {
            cx.platform().activate(true);
        }

        let paths = collect_path_args();
        if !paths.is_empty() {
            cx.dispatch_global_action(
                "workspace:open_paths",
                OpenParams {
                    paths,
                    app_state: app_state.clone(),
                },
            );
        }
    });
}

fn init_logger() {
    let level = LevelFilter::Info;

    if stdout_is_a_pty() {
        SimpleLogger::init(level, Default::default()).expect("could not initialize logger");
    } else {
        let log_dir_path = dirs::home_dir()
            .expect("could not locate home directory for logging")
            .join("Library/Logs/");
        let log_file_path = log_dir_path.join("Zed.log");
        fs::create_dir_all(&log_dir_path).expect("could not create log directory");
        let log_file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(log_file_path)
            .expect("could not open logfile");
        simplelog::WriteLogger::init(level, simplelog::Config::default(), log_file)
            .expect("could not initialize logger");
    }
}

fn stdout_is_a_pty() -> bool {
    unsafe { libc::isatty(libc::STDOUT_FILENO as i32) != 0 }
}

fn collect_path_args() -> Vec<PathBuf> {
    std::env::args()
        .skip(1)
        .filter_map(|arg| match fs::canonicalize(arg) {
            Ok(path) => Some(path),
            Err(error) => {
                log::error!("error parsing path argument: {}", error);
                None
            }
        })
        .collect::<Vec<_>>()
}
