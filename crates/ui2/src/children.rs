use std::any::Any;

use gpui3::{AnyElement, ViewContext};

pub type HackyChildren<S> = fn(&mut ViewContext<S>, &dyn Any) -> Vec<AnyElement<S>>;

pub type HackyChildrenPayload = Box<dyn Any + Send + Sync>;
