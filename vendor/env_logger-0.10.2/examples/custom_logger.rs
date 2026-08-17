/*!
Using `env_logger` to drive a custom logger.

Before running this example, try setting the `MY_LOG_LEVEL` environment variable to `info`:

```no_run,shell
$ export MY_LOG_LEVEL='info'
```

If you only want to change the way logs are formatted, look at the `custom_format` example.
*/

use env_logger::filter::{Builder, Filter};

use log::{info, Log, Metadata, Record, SetLoggerError};

const FILTER_ENV: &str = "MY_LOG_LEVEL";

struct MyLogger {
    inner: Filter,
}

impl MyLogger {
    fn new() -> MyLogger {
        let mut builder = Builder::from_env(FILTER_ENV);

        MyLogger {
            inner: builder.build(),
        }
    }

    fn init() -> Result<(), SetLoggerError> {
        let logger = Self::new();

        log::set_max_level(logger.inner.filter());
        log::set_boxed_logger(Box::new(logger))
    }
}

impl Log for MyLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        self.inner.enabled(metadata)
    }

    fn log(&self, record: &Record) {
        // Check if the record is matched by the logger before logging
        if self.inner.matches(record) {
            println!("{} - {}", record.level(), record.args());
        }
    }

    fn flush(&self) {}
}

fn main() {
    MyLogger::init().unwrap();

    info!("a log from `MyLogger`");
}
