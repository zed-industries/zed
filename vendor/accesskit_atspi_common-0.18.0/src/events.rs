// Copyright 2022 The AccessKit Authors. All rights reserved.
// Licensed under the Apache License, Version 2.0 (found in
// the LICENSE-APACHE file) or the MIT license (found in
// the LICENSE-MIT file), at your option.

use accesskit_consumer::NodeId;
use atspi_common::{Politeness, Role, State};

use crate::{NodeIdOrRoot, Rect};

#[derive(Debug)]
pub enum Event {
    Object {
        target: NodeIdOrRoot,
        event: ObjectEvent,
    },
    Window {
        target: NodeId,
        name: String,
        event: WindowEvent,
    },
}

#[derive(Debug)]
pub enum Property {
    Name(String),
    Description(String),
    Parent(NodeIdOrRoot),
    Role(Role),
    Value(f64),
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug)]
pub enum ObjectEvent {
    ActiveDescendantChanged(NodeId),
    Announcement(String, Politeness),
    BoundsChanged(Rect),
    CaretMoved(i32),
    ChildAdded(usize, NodeId),
    ChildRemoved(NodeId),
    PropertyChanged(Property),
    SelectionChanged,
    StateChanged(State, bool),
    TextInserted {
        start_index: i32,
        length: i32,
        content: String,
    },
    TextRemoved {
        start_index: i32,
        length: i32,
        content: String,
    },
    TextSelectionChanged,
}

#[derive(Debug)]
pub enum WindowEvent {
    Activated,
    Deactivated,
}
