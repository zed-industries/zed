// Allow binary to be called Zed for a nice application menu when running executable direcly
#![allow(non_snake_case)]

use client::{self, http, ChannelList, UserStore};
use fs::OpenOptions;
use gpui::{App, AssetSource};
use log::LevelFilter;
use parking_lot::Mutex;
use simplelog::SimpleLogger;
use std::{fs, path::PathBuf, sync::Arc};
use theme::{ThemeRegistry, DEFAULT_THEME_NAME};
use workspace::{self, settings, AppState, OpenNew, OpenParams, OpenPaths, Settings};
use zed::{
    self, assets::Assets, build_window_options, build_workspace, fs::RealFs, language, menus,
};

fn main() {
    init_logger();

    let app = gpui::App::new(Assets).unwrap();
    load_embedded_fonts(&app);

    let themes = ThemeRegistry::new(Assets, app.font_cache());
    let theme = themes.get(DEFAULT_THEME_NAME).unwrap();
    let settings = Settings::new("Zed Mono", &app.font_cache(), theme)
        .unwrap()
        .with_overrides(
            language::PLAIN_TEXT.name(),
            settings::Override {
                soft_wrap: Some(settings::SoftWrap::PreferredLineLength),
                ..Default::default()
            },
        )
        .with_overrides(
            "Markdown",
            settings::Override {
                soft_wrap: Some(settings::SoftWrap::PreferredLineLength),
                ..Default::default()
            },
        );
    let (settings_tx, settings) = postage::watch::channel_with(settings);
    let languages = Arc::new(language::build_language_registry());
    languages.set_theme(&settings.borrow().theme.editor.syntax);

    app.run(move |cx| {
        let http = http::client();
        let client = client::Client::new(http.clone());
        let user_store = cx.add_model(|cx| UserStore::new(client.clone(), http.clone(), cx));
        let mut path_openers = Vec::new();

        project::Project::init(&client);
        client::Channel::init(&client);
        client::init(client.clone(), cx);
        workspace::init(cx);
        editor::init(cx, &mut path_openers);
        go_to_line::init(cx);
        file_finder::init(cx);
        chat_panel::init(cx);
        outline::init(cx);
        project_panel::init(cx);
        diagnostics::init(cx);
        find::init(cx);
        cx.spawn({
            let client = client.clone();
            |cx| async move {
                if client.has_keychain_credentials(&cx) {
                    client.authenticate_and_connect(&cx).await?;
                }
                Ok::<_, anyhow::Error>(())
            }
        })
        .detach_and_log_err(cx);

        let app_state = Arc::new(AppState {
            languages: languages.clone(),
            settings_tx: Arc::new(Mutex::new(settings_tx)),
            settings,
            themes,
            channel_list: cx
                .add_model(|cx| ChannelList::new(user_store.clone(), client.clone(), cx)),
            client,
            user_store,
            fs: Arc::new(RealFs),
            path_openers: Arc::from(path_openers),
            build_window_options: &build_window_options,
            build_workspace: &build_workspace,
        });
        journal::init(app_state.clone(), cx);
        zed::init(&app_state, cx);
        theme_selector::init(app_state.as_ref().into(), cx);

        cx.set_menus(menus::menus(&app_state.clone()));

        if stdout_is_a_pty() {
            cx.platform().activate(true);
        }

        let paths = collect_path_args();
        if paths.is_empty() {
            cx.dispatch_global_action(OpenNew(app_state.clone()));
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
        log_panics::init();
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

fn load_embedded_fonts(app: &App) {
    let font_paths = Assets.list("fonts");
    let embedded_fonts = Mutex::new(Vec::new());
    smol::block_on(app.background().scoped(|scope| {
        for font_path in &font_paths {
            scope.spawn(async {
                let font_path = &*font_path;
                let font_bytes = Assets.load(&font_path).unwrap().to_vec();
                embedded_fonts.lock().push(Arc::from(font_bytes));
            });
        }
    }));
    app.platform()
        .fonts()
        .add_fonts(&embedded_fonts.into_inner())
        .unwrap();
}
