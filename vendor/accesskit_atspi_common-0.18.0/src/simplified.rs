// Copyright 2024 The AccessKit Authors. All rights reserved.
// Licensed under the Apache License, Version 2.0 (found in
// the LICENSE-APACHE file) or the MIT license (found in
// the LICENSE-MIT file), at your option.

//! API that corresponds more closely to the libatspi client API,
//! intended to be used by bindings to languages with less rich
//! type systems.

use std::collections::HashMap;

use crate::{
    Adapter, Event as EventEnum, NodeIdOrRoot, ObjectEvent, PlatformNode, PlatformRoot, Property,
    WindowEvent,
};

pub use crate::{
    CoordType, Error, Granularity, Layer, Rect, RelationType, Result, Role, ScrollType, StateSet,
};

#[derive(Clone, Hash, PartialEq)]
pub enum Accessible {
    Node(PlatformNode),
    Root(PlatformRoot),
}

impl Accessible {
    pub fn role(&self) -> Result<Role> {
        match self {
            Self::Node(node) => node.role(),
            Self::Root(_) => Ok(Role::Application),
        }
    }

    pub fn localized_role_name(&self) -> Result<String> {
        match self {
            Self::Node(node) => node.localized_role_name(),
            Self::Root(_) => Ok("".into()),
        }
    }

    pub fn name(&self) -> Result<String> {
        match self {
            Self::Node(node) => node.name(),
            Self::Root(root) => root.name(),
        }
    }

    pub fn description(&self) -> Result<String> {
        match self {
            Self::Node(node) => node.description(),
            Self::Root(_) => Ok("".into()),
        }
    }

    pub fn state(&self) -> StateSet {
        match self {
            Self::Node(node) => node.state(),
            Self::Root(_) => StateSet::empty(),
        }
    }

    pub fn attributes(&self) -> Result<HashMap<&'static str, String>> {
        match self {
            Self::Node(node) => node.attributes(),
            Self::Root(_) => Ok(HashMap::new()),
        }
    }

    pub fn parent(&self) -> Result<Option<Self>> {
        match self {
            Self::Node(node) => match node.parent()? {
                NodeIdOrRoot::Node(id) => Ok(Some(Self::Node(node.relative(id)))),
                NodeIdOrRoot::Root => node.root().map(|root| Some(Self::Root(root))),
            },
            Self::Root(_) => Ok(None),
        }
    }

    pub fn index_in_parent(&self) -> Result<i32> {
        match self {
            Self::Node(node) => node.index_in_parent(),
            Self::Root(_) => Ok(-1),
        }
    }

    pub fn child_count(&self) -> Result<i32> {
        match self {
            Self::Node(node) => node.child_count(),
            Self::Root(root) => root.child_count(),
        }
    }

    pub fn accessible_id(&self) -> Result<String> {
        match self {
            Self::Node(node) => node.accessible_id(),
            Self::Root(_) => Ok(String::new()),
        }
    }

    pub fn child_at_index(&self, index: usize) -> Result<Option<Self>> {
        match self {
            Self::Node(node) => node
                .child_at_index(index)
                .map(|id| id.map(|id| Self::Node(node.relative(id)))),
            Self::Root(root) => root
                .child_at_index(index)
                .map(|child| child.map(Self::Node)),
        }
    }

    pub fn map_children<T, I>(&self, f: impl Fn(Self) -> I) -> Result<T>
    where
        T: FromIterator<I>,
    {
        match self {
            Self::Node(node) => node.map_children(|id| f(Self::Node(node.relative(id)))),
            Self::Root(root) => root.map_children(|node| f(Self::Node(node))),
        }
    }

    pub fn relation_set(&self) -> Result<HashMap<RelationType, Vec<Self>>> {
        match self {
            Self::Node(node) => node.relation_set(|id| Self::Node(node.relative(id))),
            Self::Root(_) => Ok(HashMap::new()),
        }
    }

    pub fn application(&self) -> Result<Self> {
        match self {
            Self::Node(node) => node.root().map(Self::Root),
            Self::Root(root) => Ok(Self::Root(root.clone())),
        }
    }

    pub fn toolkit_name(&self) -> Result<String> {
        match self {
            Self::Node(node) => node.toolkit_name(),
            Self::Root(root) => root.toolkit_name(),
        }
    }

    pub fn toolkit_version(&self) -> Result<String> {
        match self {
            Self::Node(node) => node.toolkit_version(),
            Self::Root(root) => root.toolkit_version(),
        }
    }

    pub fn supports_action(&self) -> Result<bool> {
        match self {
            Self::Node(node) => node.supports_action(),
            Self::Root(_) => Ok(false),
        }
    }

    pub fn n_actions(&self) -> Result<i32> {
        match self {
            Self::Node(node) => node.n_actions(),
            Self::Root(_) => Err(Error::UnsupportedInterface),
        }
    }

    pub fn action_name(&self, index: i32) -> Result<String> {
        match self {
            Self::Node(node) => node.action_name(index),
            Self::Root(_) => Err(Error::UnsupportedInterface),
        }
    }

    pub fn do_action(&self, index: i32) -> Result<bool> {
        match self {
            Self::Node(node) => node.do_action(index),
            Self::Root(_) => Err(Error::UnsupportedInterface),
        }
    }

    pub fn supports_component(&self) -> Result<bool> {
        match self {
            Self::Node(node) => node.supports_component(),
            Self::Root(_) => Ok(false),
        }
    }

    pub fn contains(&self, x: i32, y: i32, coord_type: CoordType) -> Result<bool> {
        match self {
            Self::Node(node) => node.contains(x, y, coord_type),
            Self::Root(_) => Err(Error::UnsupportedInterface),
        }
    }

    pub fn accessible_at_point(
        &self,
        x: i32,
        y: i32,
        coord_type: CoordType,
    ) -> Result<Option<Self>> {
        match self {
            Self::Node(node) => node
                .accessible_at_point(x, y, coord_type)
                .map(|id| id.map(|id| Self::Node(node.relative(id)))),
            Self::Root(_) => Err(Error::UnsupportedInterface),
        }
    }

    pub fn extents(&self, coord_type: CoordType) -> Result<Rect> {
        match self {
            Self::Node(node) => node.extents(coord_type),
            Self::Root(_) => Err(Error::UnsupportedInterface),
        }
    }

    pub fn layer(&self) -> Result<Layer> {
        match self {
            Self::Node(node) => node.layer(),
            Self::Root(_) => Err(Error::UnsupportedInterface),
        }
    }

    pub fn grab_focus(&self) -> Result<bool> {
        match self {
            Self::Node(node) => node.grab_focus(),
            Self::Root(_) => Err(Error::UnsupportedInterface),
        }
    }

    pub fn scroll_to(&self, scroll_type: ScrollType) -> Result<bool> {
        match self {
            Self::Node(node) => node.scroll_to(scroll_type),
            Self::Root(_) => Err(Error::UnsupportedInterface),
        }
    }

    pub fn scroll_to_point(&self, coord_type: CoordType, x: i32, y: i32) -> Result<bool> {
        match self {
            Self::Node(node) => node.scroll_to_point(coord_type, x, y),
            Self::Root(_) => Err(Error::UnsupportedInterface),
        }
    }

    pub fn supports_hyperlink(&self) -> Result<bool> {
        match self {
            Self::Node(node) => node.supports_hyperlink(),
            Self::Root(_) => Ok(false),
        }
    }

    pub fn n_anchors(&self) -> Result<i32> {
        match self {
            Self::Node(node) => node.n_anchors(),
            Self::Root(_) => Err(Error::UnsupportedInterface),
        }
    }

    pub fn hyperlink_start_index(&self) -> Result<i32> {
        match self {
            Self::Node(node) => node.hyperlink_start_index(),
            Self::Root(_) => Err(Error::UnsupportedInterface),
        }
    }

    pub fn hyperlink_end_index(&self) -> Result<i32> {
        match self {
            Self::Node(node) => node.hyperlink_end_index(),
            Self::Root(_) => Err(Error::UnsupportedInterface),
        }
    }

    pub fn hyperlink_object(&self, index: i32) -> Result<Option<Self>> {
        match self {
            Self::Node(node) => node
                .hyperlink_object(index)
                .map(|id| id.map(|id| Self::Node(node.relative(id)))),
            Self::Root(_) => Err(Error::UnsupportedInterface),
        }
    }

    pub fn uri(&self, index: i32) -> Result<String> {
        match self {
            Self::Node(node) => node.uri(index),
            Self::Root(_) => Err(Error::UnsupportedInterface),
        }
    }

    pub fn hyperlink_is_valid(&self) -> Result<bool> {
        match self {
            Self::Node(node) => node.hyperlink_is_valid(),
            Self::Root(_) => Err(Error::UnsupportedInterface),
        }
    }

    pub fn supports_selection(&self) -> Result<bool> {
        match self {
            Self::Node(node) => node.supports_selection(),
            Self::Root(_) => Ok(false),
        }
    }

    pub fn n_selected_children(&self) -> Result<i32> {
        match self {
            Self::Node(node) => node.n_selected_children(),
            Self::Root(_) => Err(Error::UnsupportedInterface),
        }
    }

    pub fn selected_child(&self, selected_child_index: usize) -> Result<Option<Self>> {
        match self {
            Self::Node(node) => node
                .selected_child(selected_child_index)
                .map(|id| id.map(|id| Self::Node(node.relative(id)))),
            Self::Root(_) => Err(Error::UnsupportedInterface),
        }
    }

    pub fn select_child(&self, child_index: usize) -> Result<bool> {
        match self {
            Self::Node(node) => node.select_child(child_index),
            Self::Root(_) => Err(Error::UnsupportedInterface),
        }
    }

    pub fn deselect_selected_child(&self, selected_child_index: usize) -> Result<bool> {
        match self {
            Self::Node(node) => node.deselect_selected_child(selected_child_index),
            Self::Root(_) => Err(Error::UnsupportedInterface),
        }
    }

    pub fn is_child_selected(&self, child_index: usize) -> Result<bool> {
        match self {
            Self::Node(node) => node.is_child_selected(child_index),
            Self::Root(_) => Err(Error::UnsupportedInterface),
        }
    }

    pub fn select_all(&self) -> Result<bool> {
        match self {
            Self::Node(node) => node.select_all(),
            Self::Root(_) => Err(Error::UnsupportedInterface),
        }
    }

    pub fn clear_selection(&self) -> Result<bool> {
        match self {
            Self::Node(node) => node.clear_selection(),
            Self::Root(_) => Err(Error::UnsupportedInterface),
        }
    }

    pub fn deselect_child(&self, child_index: usize) -> Result<bool> {
        match self {
            Self::Node(node) => node.deselect_child(child_index),
            Self::Root(_) => Err(Error::UnsupportedInterface),
        }
    }

    pub fn supports_text(&self) -> Result<bool> {
        match self {
            Self::Node(node) => node.supports_text(),
            Self::Root(_) => Ok(false),
        }
    }

    pub fn character_count(&self) -> Result<i32> {
        match self {
            Self::Node(node) => node.character_count(),
            Self::Root(_) => Err(Error::UnsupportedInterface),
        }
    }

    pub fn caret_offset(&self) -> Result<i32> {
        match self {
            Self::Node(node) => node.caret_offset(),
            Self::Root(_) => Err(Error::UnsupportedInterface),
        }
    }

    pub fn string_at_offset(
        &self,
        offset: i32,
        granularity: Granularity,
    ) -> Result<(String, i32, i32)> {
        match self {
            Self::Node(node) => node.string_at_offset(offset, granularity),
            Self::Root(_) => Err(Error::UnsupportedInterface),
        }
    }

    pub fn text(&self, start_offset: i32, end_offset: i32) -> Result<String> {
        match self {
            Self::Node(node) => node.text(start_offset, end_offset),
            Self::Root(_) => Err(Error::UnsupportedInterface),
        }
    }

    pub fn set_caret_offset(&self, offset: i32) -> Result<bool> {
        match self {
            Self::Node(node) => node.set_caret_offset(offset),
            Self::Root(_) => Err(Error::UnsupportedInterface),
        }
    }

    pub fn text_attribute_value(&self, offset: i32, attribute_name: &str) -> Result<String> {
        match self {
            Self::Node(node) => node.text_attribute_value(offset, attribute_name),
            Self::Root(_) => Err(Error::UnsupportedInterface),
        }
    }

    pub fn text_attributes(
        &self,
        offset: i32,
    ) -> Result<(HashMap<&'static str, String>, i32, i32)> {
        match self {
            Self::Node(node) => node.text_attributes(offset),
            Self::Root(_) => Err(Error::UnsupportedInterface),
        }
    }

    pub fn default_text_attributes(&self) -> Result<HashMap<&'static str, String>> {
        match self {
            Self::Node(node) => node.default_text_attributes(),
            Self::Root(_) => Err(Error::UnsupportedInterface),
        }
    }

    pub fn character_extents(&self, offset: i32, coord_type: CoordType) -> Result<Rect> {
        match self {
            Self::Node(node) => node.character_extents(offset, coord_type),
            Self::Root(_) => Err(Error::UnsupportedInterface),
        }
    }

    pub fn offset_at_point(&self, x: i32, y: i32, coord_type: CoordType) -> Result<i32> {
        match self {
            Self::Node(node) => node.offset_at_point(x, y, coord_type),
            Self::Root(_) => Err(Error::UnsupportedInterface),
        }
    }

    pub fn n_selections(&self) -> Result<i32> {
        match self {
            Self::Node(node) => node.n_selections(),
            Self::Root(_) => Err(Error::UnsupportedInterface),
        }
    }

    pub fn selection(&self, selection_num: i32) -> Result<(i32, i32)> {
        match self {
            Self::Node(node) => node.selection(selection_num),
            Self::Root(_) => Err(Error::UnsupportedInterface),
        }
    }

    pub fn add_selection(&self, start_offset: i32, end_offset: i32) -> Result<bool> {
        match self {
            Self::Node(node) => node.add_selection(start_offset, end_offset),
            Self::Root(_) => Err(Error::UnsupportedInterface),
        }
    }

    pub fn remove_selection(&self, selection_num: i32) -> Result<bool> {
        match self {
            Self::Node(node) => node.remove_selection(selection_num),
            Self::Root(_) => Err(Error::UnsupportedInterface),
        }
    }

    pub fn set_selection(
        &self,
        selection_num: i32,
        start_offset: i32,
        end_offset: i32,
    ) -> Result<bool> {
        match self {
            Self::Node(node) => node.set_selection(selection_num, start_offset, end_offset),
            Self::Root(_) => Err(Error::UnsupportedInterface),
        }
    }

    pub fn range_extents(
        &self,
        start_offset: i32,
        end_offset: i32,
        coord_type: CoordType,
    ) -> Result<Rect> {
        match self {
            Self::Node(node) => node.range_extents(start_offset, end_offset, coord_type),
            Self::Root(_) => Err(Error::UnsupportedInterface),
        }
    }

    pub fn text_attribute_run(
        &self,
        offset: i32,
        include_defaults: bool,
    ) -> Result<(HashMap<&'static str, String>, i32, i32)> {
        match self {
            Self::Node(node) => node.text_attribute_run(offset, include_defaults),
            Self::Root(_) => Err(Error::UnsupportedInterface),
        }
    }

    pub fn scroll_substring_to(
        &self,
        start_offset: i32,
        end_offset: i32,
        scroll_type: ScrollType,
    ) -> Result<bool> {
        match self {
            Self::Node(node) => node.scroll_substring_to(start_offset, end_offset, scroll_type),
            Self::Root(_) => Err(Error::UnsupportedInterface),
        }
    }

    pub fn scroll_substring_to_point(
        &self,
        start_offset: i32,
        end_offset: i32,
        coord_type: CoordType,
        x: i32,
        y: i32,
    ) -> Result<bool> {
        match self {
            Self::Node(node) => {
                node.scroll_substring_to_point(start_offset, end_offset, coord_type, x, y)
            }
            Self::Root(_) => Err(Error::UnsupportedInterface),
        }
    }

    pub fn supports_value(&self) -> Result<bool> {
        match self {
            Self::Node(node) => node.supports_value(),
            Self::Root(_) => Ok(false),
        }
    }

    pub fn minimum_value(&self) -> Result<f64> {
        match self {
            Self::Node(node) => node.minimum_value(),
            Self::Root(_) => Err(Error::UnsupportedInterface),
        }
    }

    pub fn maximum_value(&self) -> Result<f64> {
        match self {
            Self::Node(node) => node.maximum_value(),
            Self::Root(_) => Err(Error::UnsupportedInterface),
        }
    }

    pub fn minimum_increment(&self) -> Result<f64> {
        match self {
            Self::Node(node) => node.minimum_increment(),
            Self::Root(_) => Err(Error::UnsupportedInterface),
        }
    }

    pub fn current_value(&self) -> Result<f64> {
        match self {
            Self::Node(node) => node.current_value(),
            Self::Root(_) => Err(Error::UnsupportedInterface),
        }
    }

    pub fn set_current_value(&self, value: f64) -> Result<()> {
        match self {
            Self::Node(node) => node.set_current_value(value),
            Self::Root(_) => Err(Error::UnsupportedInterface),
        }
    }
}

#[derive(PartialEq)]
pub enum EventData {
    U32(u32),
    F64(f64),
    String(String),
    Rect(Rect),
    Accessible(Accessible),
}

#[derive(PartialEq)]
pub struct Event {
    pub kind: String,
    pub source: Accessible,
    pub detail1: i32,
    pub detail2: i32,
    pub data: Option<EventData>,
}

impl Event {
    pub fn new(adapter: &Adapter, event: EventEnum) -> Self {
        match event {
            EventEnum::Object { target, event } => {
                let source = match target {
                    NodeIdOrRoot::Node(target) => Accessible::Node(adapter.platform_node(target)),
                    NodeIdOrRoot::Root => Accessible::Root(adapter.platform_root()),
                };
                match event {
                    ObjectEvent::ActiveDescendantChanged(child) => {
                        let child = Accessible::Node(adapter.platform_node(child));
                        Self {
                            kind: "object:active-descendant-changed".into(),
                            source,
                            detail1: 0,
                            detail2: 0,
                            data: Some(EventData::Accessible(child)),
                        }
                    }
                    ObjectEvent::Announcement(message, politeness) => Self {
                        kind: "object:announcement".into(),
                        source,
                        detail1: politeness as i32,
                        detail2: 0,
                        data: Some(EventData::String(message)),
                    },
                    ObjectEvent::BoundsChanged(bounds) => Self {
                        kind: "object:bounds-changed".into(),
                        source,
                        detail1: 0,
                        detail2: 0,
                        data: Some(EventData::Rect(bounds)),
                    },
                    ObjectEvent::CaretMoved(offset) => Self {
                        kind: "object:text-caret-moved".into(),
                        source,
                        detail1: offset,
                        detail2: 0,
                        data: None,
                    },
                    ObjectEvent::ChildAdded(index, child) => {
                        let child = Accessible::Node(adapter.platform_node(child));
                        Self {
                            kind: "object:children-changed:add".into(),
                            source,
                            detail1: index as i32,
                            detail2: 0,
                            data: Some(EventData::Accessible(child)),
                        }
                    }
                    ObjectEvent::ChildRemoved(child) => {
                        let child = Accessible::Node(adapter.platform_node(child));
                        Self {
                            kind: "object:children-changed:remove".into(),
                            source,
                            detail1: -1,
                            detail2: 0,
                            data: Some(EventData::Accessible(child)),
                        }
                    }
                    ObjectEvent::PropertyChanged(property) => Self {
                        kind: match property {
                            Property::Name(_) => "object:property-change:accessible-name",
                            Property::Description(_) => {
                                "object:property-change:accessible-description"
                            }
                            Property::Parent(_) => "object:property-change:accessible-parent",
                            Property::Role(_) => "object:property-change:accessible-role",
                            Property::Value(_) => "object:property-change:accessible-value",
                        }
                        .into(),
                        source,
                        detail1: 0,
                        detail2: 0,
                        data: Some(match property {
                            Property::Name(value) => EventData::String(value),
                            Property::Description(value) => EventData::String(value),
                            Property::Parent(parent) => {
                                let parent = match parent {
                                    NodeIdOrRoot::Node(parent) => {
                                        Accessible::Node(adapter.platform_node(parent))
                                    }
                                    NodeIdOrRoot::Root => Accessible::Root(adapter.platform_root()),
                                };
                                EventData::Accessible(parent)
                            }
                            Property::Role(value) => EventData::U32(value as u32),
                            Property::Value(value) => EventData::F64(value),
                        }),
                    },
                    ObjectEvent::SelectionChanged => Self {
                        kind: "object:selection-changed".into(),
                        source,
                        detail1: 0,
                        detail2: 0,
                        data: None,
                    },
                    ObjectEvent::StateChanged(state, value) => Self {
                        kind: format!("object:state-changed:{}", state.to_static_str()),
                        source,
                        detail1: value as i32,
                        detail2: 0,
                        data: None,
                    },
                    ObjectEvent::TextInserted {
                        start_index,
                        length,
                        content,
                    } => Self {
                        kind: "object:text-changed:insert".into(),
                        source,
                        detail1: start_index,
                        detail2: length,
                        data: Some(EventData::String(content)),
                    },
                    ObjectEvent::TextRemoved {
                        start_index,
                        length,
                        content,
                    } => Self {
                        kind: "object:text-changed:delete".into(),
                        source,
                        detail1: start_index,
                        detail2: length,
                        data: Some(EventData::String(content)),
                    },
                    ObjectEvent::TextSelectionChanged => Self {
                        kind: "object:text-selection-changed".into(),
                        source,
                        detail1: 0,
                        detail2: 0,
                        data: None,
                    },
                }
            }
            EventEnum::Window {
                target,
                name,
                event,
            } => {
                let kind = match event {
                    WindowEvent::Activated => "window:activate",
                    WindowEvent::Deactivated => "window:deactivate",
                };
                let source = Accessible::Node(adapter.platform_node(target));
                Self {
                    kind: kind.into(),
                    source,
                    detail1: 0,
                    detail2: 0,
                    data: Some(EventData::String(name)),
                }
            }
        }
    }
}
