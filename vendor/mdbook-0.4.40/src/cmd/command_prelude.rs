//! Helpers for building the command-line arguments for commands.

pub use clap::{arg, Arg, ArgMatches, Command};
use std::path::PathBuf;

pub trait CommandExt: Sized {
    fn _arg(self, arg: Arg) -> Self;

    fn arg_dest_dir(self) -> Self {
        self._arg(
            Arg::new("dest-dir")
                .short('d')
                .long("dest-dir")
                .value_name("dest-dir")
                .value_parser(clap::value_parser!(PathBuf))
                .help(
                    "Output directory for the book\n\
                    Relative paths are interpreted relative to the book's root directory.\n\
                    If omitted, mdBook uses build.build-dir from book.toml \
                    or defaults to `./book`.",
                ),
        )
    }

    fn arg_root_dir(self) -> Self {
        self._arg(
            Arg::new("dir")
                .help(
                    "Root directory for the book\n\
                    (Defaults to the current directory when omitted)",
                )
                .value_parser(clap::value_parser!(PathBuf)),
        )
    }

    fn arg_open(self) -> Self {
        self._arg(arg!(-o --open "Opens the compiled book in a web browser"))
    }

    #[cfg(any(feature = "watch", feature = "serve"))]
    fn arg_watcher(self) -> Self {
        #[cfg(feature = "watch")]
        return self._arg(
            Arg::new("watcher")
                .long("watcher")
                .value_parser(["poll", "native"])
                .default_value("poll")
                .help("The filesystem watching technique"),
        );
        #[cfg(not(feature = "watch"))]
        return self;
    }
}

impl CommandExt for Command {
    fn _arg(self, arg: Arg) -> Self {
        self.arg(arg)
    }
}
