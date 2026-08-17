#![allow(dead_code)]

#[cfg(any(feature = "debug", feature = "profile"))]
use core::fmt::{Debug, Display, Write};
#[cfg(any(feature = "debug", feature = "profile"))]
use std::sync::Mutex;

#[doc(hidden)]
#[cfg(any(feature = "debug", feature = "profile"))]
pub struct DebugLogger {
    stack: Mutex<Vec<String>>,
}

#[cfg(any(feature = "debug", feature = "profile"))]
static EMPTY_STRING: String = String::new();
#[cfg(any(feature = "debug", feature = "profile"))]
impl DebugLogger {
    pub const fn new() -> Self {
        Self { stack: Mutex::new(Vec::new()) }
    }

    pub fn push_node(&self, new_key: crate::NodeId) {
        let mut stack = self.stack.lock().unwrap();
        let mut key_string = String::new();
        write!(&mut key_string, "{:?}", new_key).unwrap();
        stack.push(key_string);
    }

    pub fn pop_node(&self) {
        let mut stack = self.stack.lock().unwrap();
        stack.pop();
    }

    pub fn log(&self, message: impl Display) {
        let stack = self.stack.lock().unwrap();
        let key = stack.last().unwrap_or(&EMPTY_STRING);
        let level = stack.len() * 4;
        let space = " ";
        println!("{space:level$}{key}: {message}");
    }

    pub fn labelled_log(&self, label: &str, message: impl Display) {
        let stack = self.stack.lock().unwrap();
        let key = stack.last().unwrap_or(&EMPTY_STRING);
        let level = stack.len() * 4;
        let space = " ";
        println!("{space:level$}{key}: {label} {message}");
    }

    pub fn debug_log(&self, message: impl Debug) {
        let stack = self.stack.lock().unwrap();
        let key = stack.last().unwrap_or(&EMPTY_STRING);
        let level = stack.len() * 4;
        let space = " ";
        println!("{space:level$}{key}: {message:?}");
    }

    pub fn labelled_debug_log(&self, label: &str, message: impl Debug) {
        let stack = self.stack.lock().unwrap();
        let key = stack.last().unwrap_or(&EMPTY_STRING);
        let level = stack.len() * 4;
        let space = " ";
        println!("{space:level$}{key}: {label} {message:?}");
    }
}

#[cfg(any(feature = "debug", feature = "profile"))]
pub(crate) static NODE_LOGGER: DebugLogger = DebugLogger::new();

macro_rules! debug_log {
    // String literal label with debug printing
    ($label:literal, dbg:$item:expr) => {{
        #[cfg(feature = "debug")]
        $crate::util::debug::NODE_LOGGER.labelled_debug_log($label, $item);
    }};
    // String literal label with display printing
    ($label:literal, $item:expr) => {{
        #[cfg(feature = "debug")]
        $crate::util::debug::NODE_LOGGER.labelled_log($label, $item);
    }};
    // Debug printing
    (dbg:$item:expr) => {{
        #[cfg(feature = "debug")]
        $crate::util::debug::NODE_LOGGER.debug_log($item);
    }};
    // Display printing
    ($item:expr) => {{
        #[cfg(feature = "debug")]
        $crate::util::debug::NODE_LOGGER.log($item);
    }};
    // Blank newline
    () => {{
        #[cfg(feature = "debug")]
        println!();
    }};
}

macro_rules! debug_log_node {
    ($inputs: expr) => {
        debug_log!(dbg:$inputs.run_mode);
        debug_log!("sizing_mode", dbg:$inputs.sizing_mode);
        debug_log!("known_dimensions", dbg:$inputs.known_dimensions);
        debug_log!("parent_size", dbg:$inputs.parent_size);
        debug_log!("available_space", dbg:$inputs.available_space);
    };
}

macro_rules! debug_push_node {
    ($node_id:expr) => {
        #[cfg(any(feature = "debug", feature = "profile"))]
        $crate::util::debug::NODE_LOGGER.push_node($node_id);
        debug_log!("");
    };
}

macro_rules! debug_pop_node {
    () => {
        #[cfg(any(feature = "debug", feature = "profile"))]
        $crate::util::debug::NODE_LOGGER.pop_node();
    };
}

#[cfg(feature = "profile")]
#[allow(unused_macros)]
macro_rules! time {
    ($label:expr, $($code:tt)*) => {
        let start = ::std::time::Instant::now();
        $($code)*
        let duration = ::std::time::Instant::now().duration_since(start);
        crate::util::debug::NODE_LOGGER.log(format_args!("Performed {} in {}ms", $label, duration.as_millis()));
    };
}

#[cfg(not(feature = "profile"))]
#[allow(unused_macros)]
macro_rules! time {
    ($label:expr, $($code:tt)*) => {
        $($code)*
    };
}

#[allow(unused_imports)]
pub(crate) use {debug_log, debug_log_node, debug_pop_node, debug_push_node, time};
