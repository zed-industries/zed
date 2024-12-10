//! # Component Preview
//!
//! This crate provides traits and and slices for defining and registering components and their previews.

use gpui::{AnyElement, WindowContext};
use linkme::distributed_slice;
use once_cell::sync::Lazy;
use std::sync::Mutex;

pub trait Component {
    fn scope() -> &'static str;
    fn name() -> &'static str {
        std::any::type_name::<Self>()
    }
    fn description() -> Option<&'static str> {
        None
    }
    fn preview(_cx: &WindowContext) -> Option<AnyElement> {
        None
    }
}

pub trait ComponentPreview: Component {
    fn preview(_cx: &WindowContext) -> AnyElement;
}

#[distributed_slice]
pub static __ALL_COMPONENTS: [fn()] = [..];

#[distributed_slice]
pub static __ALL_PREVIEWS: [fn()] = [..];

pub static COMPONENTS: Lazy<Mutex<Vec<(&'static str, &'static str, Option<&'static str>)>>> =
    Lazy::new(|| Mutex::new(Vec::new()));
pub static PREVIEWS: Lazy<Mutex<Vec<&'static str>>> = Lazy::new(|| Mutex::new(Vec::new()));

pub fn init() {
    for f in __ALL_COMPONENTS {
        f();
    }
    for f in __ALL_PREVIEWS {
        f();
    }
}

pub fn get_all_components() -> Vec<(&'static str, &'static str, Option<&'static str>)> {
    COMPONENTS.lock().unwrap().clone()
}

pub fn get_all_component_previews() -> Vec<&'static str> {
    PREVIEWS.lock().unwrap().clone()
}
