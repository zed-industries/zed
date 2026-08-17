/// Logs a message at the info level.
///
/// Passed data uses a colorize_string formatter from a `paris` crate, so it can
/// contains special tags for controlling ANSI colors and styles
/// More info: <https://docs.rs/paris/1.5.7/paris/formatter/fn.colorize_string.html>
///
/// # Examples
///
/// ```edition2018
/// use log::info;
///
/// # fn main() {
/// # struct Connection { port: u32, speed: f32 }
/// let conn_info = Connection { port: 40, speed: 3.20 };
///
/// info!("Connected to port {} at {} Mb/s", conn_info.port, conn_info.speed);
/// info!(target: "connection_events", "Successfull connection, port: {}, speed: {}",
///       conn_info.port, conn_info.speed);
/// # }
/// ```
#[macro_export]
macro_rules! info {
    ($($args:tt)+) => {
        $crate::__private::log::info!("{}", $crate::__private::paris::formatter::colorize_string(format!($($args)*)));
    };
}

/// Logs a message at the debug level.
///
/// Passed data uses a colorize_string formatter from a `paris` crate, so it can
/// contains special tags for controlling ANSI colors and styles
/// More info: <https://docs.rs/paris/1.5.7/paris/formatter/fn.colorize_string.html>
///
/// # Examples
///
/// ```edition2018
/// use log::debug;
///
/// # fn main() {
/// # struct Position { x: f32, y: f32 }
/// let pos = Position { x: 3.234, y: -1.223 };
///
/// debug!("New position: x: {}, y: {}", pos.x, pos.y);
/// debug!(target: "app_events", "New position: x: {}, y: {}", pos.x, pos.y);
/// # }
/// ```
#[macro_export]
macro_rules! debug {
    ($($args:tt)+) => {
        $crate::__private::log::debug!("{}", $crate::__private::paris::formatter::colorize_string(format!($($args)*)));
    };
}

/// Logs a message at the trace level.
///
/// Passed data uses a colorize_string formatter from a `paris` crate, so it can
/// contains special tags for controlling ANSI colors and styles
/// More info: <https://docs.rs/paris/1.5.7/paris/formatter/fn.colorize_string.html>
///
/// # Examples
///
/// ```edition2018
/// use log::trace;
///
/// # fn main() {
/// # struct Position { x: f32, y: f32 }
/// let pos = Position { x: 3.234, y: -1.223 };
///
/// trace!("Position is: x: {}, y: {}", pos.x, pos.y);
/// trace!(target: "app_events", "x is {} and y is {}",
///        if pos.x >= 0.0 { "positive" } else { "negative" },
///        if pos.y >= 0.0 { "positive" } else { "negative" });
/// # }
/// ```
#[macro_export]
macro_rules! trace {
    ($($args:tt)+) => {
        $crate::__private::log::trace!("{}", $crate::__private::paris::formatter::colorize_string(format!($($args)*)));
    };
}

/// Logs a message at the warn level.
///
/// Passed data uses a colorize_string formatter from a `paris` crate, so it can
/// contains special tags for controlling ANSI colors and styles
/// More info: <https://docs.rs/paris/1.5.7/paris/formatter/fn.colorize_string.html>
///
/// # Examples
///
/// ```edition2018
/// use log::warn;
///
/// # fn main() {
/// let warn_description = "Invalid Input";
///
/// warn!("Warning! {}!", warn_description);
/// warn!(target: "input_events", "App received warning: {}", warn_description);
/// # }
/// ```
#[macro_export]
macro_rules! warn {
    ($($args:tt)+) => {
        $crate::__private::log::warn!("{}", $crate::__private::paris::formatter::colorize_string(format!($($args)*)));
    };
}

/// Logs a message at the error level.
///
/// Passed data uses a colorize_string formatter from a `paris` crate, so it can
/// contains special tags for controlling ANSI colors and styles
/// More info: <https://docs.rs/paris/1.5.7/paris/formatter/fn.colorize_string.html>
///
/// # Examples
///
/// ```edition2018
/// use log::error;
///
/// # fn main() {
/// let (err_info, port) = ("No connection", 22);
///
/// error!("Error: {} on port {}", err_info, port);
/// error!(target: "app_events", "App Error: {}, Port: {}", err_info, 22);
/// # }
/// ```
#[macro_export]
macro_rules! error {
    ($($args:tt)+) => {
        $crate::__private::log::error!("{}", $crate::__private::paris::formatter::colorize_string(format!($($args)*)));
    };
}
