use log::{Level, Log, Metadata, Record};

struct ConsoleLogger;

impl Log for ConsoleLogger {
    fn enabled(&self, _metadata: &Metadata) -> bool {
        true
    }

    fn log(&self, record: &Record) {
        if !self.enabled(record.metadata()) {
            return;
        }

        let message = format!(
            "[{}] {}: {}",
            record.level(),
            record.target(),
            record.args()
        );
        let js_string = wasm_bindgen::JsValue::from_str(&message);

        match record.level() {
            Level::Error => web_sys::console::error_1(&js_string),
            Level::Warn => web_sys::console::warn_1(&js_string),
            Level::Info => web_sys::console::info_1(&js_string),
            Level::Debug | Level::Trace => web_sys::console::log_1(&js_string),
        }
    }

    fn flush(&self) {}
}

pub fn init_logging() {
    log::set_logger(&ConsoleLogger).ok();
    log::set_max_level(log::LevelFilter::Info);
}
