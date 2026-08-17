#![allow(unused)]

macro_rules! log {
    (target: $target:expr, $lvl:expr, $($arg:tt)+) => {{
        let _ = $target;
        let _ = log!($lvl, $($arg)+);
    }};
    ($lvl:expr, $($arg:tt)+) => {{
        let _ = $lvl;
        let _ = format_args!($($arg)+);
    }};
}
macro_rules! error {
    (target: $target:expr, $($arg:tt)+) => { log!(target: $target, "", $($arg)+) };
    ($($arg:tt)+) => { log!("", $($arg)+) };
}
macro_rules! warn {
    (target: $target:expr, $($arg:tt)*) => { log!(target: $target, "", $($arg)*) };
    ($($arg:tt)*) => { log!("", $($arg)*) };
}
macro_rules! info {
    (target: $target:expr, $($arg:tt)+) => { log!(target: $target, "", $($arg)+) };
    ($($arg:tt)+) => { log!("", $($arg)+) };
}
macro_rules! debug {
    (target: $target:expr, $($arg:tt)+) => { log!(target: $target, "", $($arg)+) };
    ($($arg:tt)+) => { log!("", $($arg)+) };
}
macro_rules! trace {
    (target: $target:expr, $($arg:tt)+) => { log!(target: $target, "", $($arg)+) };
    ($($arg:tt)+) => { log!("", $($arg)+) };
}
