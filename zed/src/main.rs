use fs::OpenOptions;
use gpui::{
    executor,
    geometry::{rect::RectF, vector::vec2f},
    platform::{current as platform, App as _, Runner as _, WindowOptions},
    FontCache,
};
use log::LevelFilter;
use simplelog::SimpleLogger;
use std::{fs, mem, rc::Rc, sync::Arc};
use zed::{
    editor, settings,
    workspace::{self, OpenParams},
};

fn main() {
    init_logger();

    let platform = Arc::new(platform::app());

    let foreground = Rc::new(
        executor::Foreground::platform(platform.dispatcher())
            .expect("could not foreground create executor"),
    );

    let font_cache = FontCache::new();

    let (settings_tx, settings_rx) = settings::channel(&font_cache).unwrap();

    let mut app = gpui::App::new(As).unwrap();

    platform::runner()
        .on_finish_launching(move || {
            log::info!("finish launching");

            workspace::init(&mut app);
            editor::init(&mut app);

            if stdout_is_a_pty() {
                platform.activate(true);
            }

            let paths = std::env::args()
                .skip(1)
                .filter_map(|arg| match fs::canonicalize(arg) {
                    Ok(path) => Some(path),
                    Err(error) => {
                        log::error!("error parsing path argument: {}", error);
                        None
                    }
                })
                .collect::<Vec<_>>();

            if !paths.is_empty() {
                app.dispatch_global_action(
                    "workspace:open_paths",
                    OpenParams {
                        paths,
                        settings: settings_rx,
                    },
                );
                mem::forget(app); // This is here until we hold on to the app for some reason
            }

            // let window = platform
            //     .open_window(
            //         WindowOptions {
            //             bounds: RectF::new(vec2f(0., 0.), vec2f(1024., 768.)),
            //             title: Some("Zed"),
            //         },
            //         foreground,
            //     )
            //     .expect("error opening window");

            // mem::forget(window); // Leak window for now so it doesn't close
        })
        .run();
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
