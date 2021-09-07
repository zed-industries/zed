// Allow binary to be called Zed for a nice application menu when running executable direcly
#![allow(non_snake_case)]

use fs::OpenOptions;
use gpui::AssetSource;
use log::LevelFilter;
use parking_lot::Mutex;
use simplelog::SimpleLogger;
use std::{fs, path::PathBuf, sync::Arc};
use zed::{
    self,
    assets::Assets,
    channel::ChannelList,
    chat_panel, editor, file_finder,
    fs::RealFs,
    language, menus, rpc, settings, theme_selector,
    user::UserStore,
    workspace::{self, OpenNew, OpenParams, OpenPaths},
    AppState,
};

fn main() {
    init_logger();

    let app = gpui::App::new(Assets).unwrap();
    let embedded_fonts = Assets
        .list("fonts")
        .into_iter()
        .map(|f| Arc::new(Assets.load(&f).unwrap().to_vec()))
        .collect::<Vec<_>>();
    app.platform().fonts().add_fonts(&embedded_fonts).unwrap();

    let themes = settings::ThemeRegistry::new(Assets, app.font_cache());
    let (settings_tx, settings) = settings::channel(&app.font_cache(), &themes).unwrap();
    let languages = Arc::new(language::LanguageRegistry::new());
    languages.set_theme(&settings.borrow().theme.syntax);

    app.run(move |cx| {
        let rpc = rpc::Client::new();
        let user_store = Arc::new(UserStore::new(rpc.clone()));
        let app_state = Arc::new(AppState {
            languages: languages.clone(),
            settings_tx: Arc::new(Mutex::new(settings_tx)),
            settings,
            themes,
            channel_list: cx.add_model(|cx| ChannelList::new(user_store, rpc.clone(), cx)),
            rpc,
            fs: Arc::new(RealFs),
        });

        zed::init(&app_state, cx);
        workspace::init(cx);
        editor::init(cx);
        file_finder::init(cx);
        chat_panel::init(cx);
        theme_selector::init(&app_state, cx);

        cx.set_menus(menus::menus(&app_state.clone()));

        if stdout_is_a_pty() {
            cx.platform().activate(true);
        }

        let paths = collect_path_args();
        if paths.is_empty() {
            cx.dispatch_global_action(OpenNew(app_state));
        } else {
            cx.dispatch_global_action(OpenPaths(OpenParams { paths, app_state }));
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
