use chrono::Offset;
use env_logger::Builder;
use log::LevelFilter;
use simplelog::ConfigBuilder;
use std::fs::{self, File, OpenOptions};
use std::io::{self, Write};
use time::UtcOffset;

pub fn init_logger() {
    let level = LevelFilter::Info;

    // Prevent log file from becoming too large.
    const KIB: u64 = 1024;
    const MIB: u64 = 1024 * KIB;
    const MAX_LOG_BYTES: u64 = MIB;
    if std::fs::metadata(paths::log_file()).map_or(false, |metadata| metadata.len() > MAX_LOG_BYTES)
    {
        let _ = std::fs::rename(paths::log_file(), paths::old_log_file());
    }

    match LogWriter::new(MAX_LOG_BYTES) {
        Ok(writer) => {
            let mut config_builder = ConfigBuilder::new();

            config_builder.set_time_format_rfc3339();
            let local_offset = chrono::Local::now().offset().fix().local_minus_utc();
            if let Ok(offset) = UtcOffset::from_whole_seconds(local_offset) {
                config_builder.set_time_offset(offset);
            }

            #[cfg(any(target_os = "linux", target_os = "freebsd"))]
            {
                config_builder.add_filter_ignore_str("zbus");
                config_builder.add_filter_ignore_str("blade_graphics::hal::resource");
                config_builder.add_filter_ignore_str("naga::back::spv::writer");
            }

            let config = config_builder.build();
            simplelog::WriteLogger::init(level, config, writer)
                .expect("could not initialize logger");
        }
        Err(err) => {
            init_stdout_logger();
            log::error!(
                "could not open log file, defaulting to stdout logging: {}",
                err
            );
        }
    }
}

pub fn init_stdout_logger() {
    Builder::new()
        .parse_default_env()
        .format(|buf, record| {
            use env_logger::fmt::style::{AnsiColor, Style};

            let subtle = Style::new().fg_color(Some(AnsiColor::BrightBlack.into()));
            write!(buf, "{subtle}[{subtle:#}")?;
            write!(
                buf,
                "{} ",
                chrono::Local::now().format("%Y-%m-%dT%H:%M:%S%:z")
            )?;
            let level_style = buf.default_level_style(record.level());
            write!(buf, "{level_style}{:<5}{level_style:#}", record.level())?;
            if let Some(path) = record.module_path() {
                write!(buf, " {path}")?;
            }
            write!(buf, "{subtle}]{subtle:#}")?;
            writeln!(buf, " {}", record.args())
        })
        .init();
}

struct LogWriter {
    file: File,
    max_size: u64,
    current_size: u64,
}

impl LogWriter {
    fn new(max_size: u64) -> io::Result<Self> {
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(paths::log_file())?;
        let current_size = file.metadata()?.len();

        Ok(LogWriter {
            file,
            max_size,
            current_size,
        })
    }

    fn replace(&mut self) -> io::Result<()> {
        self.file.sync_all()?;
        fs::rename(paths::log_file(), paths::old_log_file())?;
        self.file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(paths::log_file())?;
        self.current_size = 0;
        Ok(())
    }
}

impl Write for LogWriter {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        if self.current_size + buf.len() as u64 > self.max_size {
            self.replace()?;
        }
        let bytes = self.file.write(buf)?;
        self.current_size += bytes as u64;
        Ok(bytes)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.file.flush()
    }
}
