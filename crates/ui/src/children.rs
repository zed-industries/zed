use std::any::Any;

use gpui2::{AnyElement, ViewContext};

pub type HackyChildren<V> = fn(&mut ViewContext<V>, &dyn Any) -> Vec<AnyElement<V>>;

pub type HackyChildrenPayload = Box<dyn Any>;
