// Copyright 2024 The AccessKit Authors. All rights reserved.
// Licensed under the Apache License, Version 2.0 (found in
// the LICENSE-APACHE file) or the MIT license (found in
// the LICENSE-MIT file), at your option.

use accesskit_atspi_common::{PlatformNode, Rect};
use atspi::{CoordType, Granularity, ScrollType};
use std::collections::HashMap;
use zbus::{fdo, interface};

pub(crate) struct TextInterface {
    node: PlatformNode,
}

impl TextInterface {
    pub fn new(node: PlatformNode) -> Self {
        Self { node }
    }

    fn map_error(&self) -> impl '_ + FnOnce(accesskit_atspi_common::Error) -> fdo::Error {
        |error| crate::util::map_error_from_node(&self.node, error)
    }
}

#[interface(name = "org.a11y.atspi.Text")]
impl TextInterface {
    #[zbus(property)]
    fn character_count(&self) -> fdo::Result<i32> {
        self.node.character_count().map_err(self.map_error())
    }

    #[zbus(property)]
    fn caret_offset(&self) -> fdo::Result<i32> {
        self.node.caret_offset().map_err(self.map_error())
    }

    fn get_string_at_offset(
        &self,
        offset: i32,
        granularity: Granularity,
    ) -> fdo::Result<(String, i32, i32)> {
        self.node
            .string_at_offset(offset, granularity)
            .map_err(self.map_error())
    }

    fn get_text(&self, start_offset: i32, end_offset: i32) -> fdo::Result<String> {
        self.node
            .text(start_offset, end_offset)
            .map_err(self.map_error())
    }

    fn set_caret_offset(&self, offset: i32) -> fdo::Result<bool> {
        self.node.set_caret_offset(offset).map_err(self.map_error())
    }

    fn get_attribute_value(&self, offset: i32, attribute_name: &str) -> fdo::Result<String> {
        self.node
            .text_attribute_value(offset, attribute_name)
            .map_err(self.map_error())
    }

    fn get_attributes(
        &self,
        offset: i32,
    ) -> fdo::Result<(HashMap<&'static str, String>, i32, i32)> {
        self.node.text_attributes(offset).map_err(self.map_error())
    }

    fn get_default_attributes(&self) -> fdo::Result<HashMap<&'static str, String>> {
        self.node
            .default_text_attributes()
            .map_err(self.map_error())
    }

    fn get_character_extents(&self, offset: i32, coord_type: CoordType) -> fdo::Result<Rect> {
        self.node
            .character_extents(offset, coord_type)
            .map_err(self.map_error())
    }

    fn get_offset_at_point(&self, x: i32, y: i32, coord_type: CoordType) -> fdo::Result<i32> {
        self.node
            .offset_at_point(x, y, coord_type)
            .map_err(self.map_error())
    }

    fn get_n_selections(&self) -> fdo::Result<i32> {
        self.node.n_selections().map_err(self.map_error())
    }

    fn get_selection(&self, selection_num: i32) -> fdo::Result<(i32, i32)> {
        self.node.selection(selection_num).map_err(self.map_error())
    }

    fn add_selection(&self, start_offset: i32, end_offset: i32) -> fdo::Result<bool> {
        self.node
            .add_selection(start_offset, end_offset)
            .map_err(self.map_error())
    }

    fn remove_selection(&self, selection_num: i32) -> fdo::Result<bool> {
        self.node
            .remove_selection(selection_num)
            .map_err(self.map_error())
    }

    fn set_selection(
        &self,
        selection_num: i32,
        start_offset: i32,
        end_offset: i32,
    ) -> fdo::Result<bool> {
        self.node
            .set_selection(selection_num, start_offset, end_offset)
            .map_err(self.map_error())
    }

    fn get_range_extents(
        &self,
        start_offset: i32,
        end_offset: i32,
        coord_type: CoordType,
    ) -> fdo::Result<Rect> {
        self.node
            .range_extents(start_offset, end_offset, coord_type)
            .map_err(self.map_error())
    }

    fn get_attribute_run(
        &self,
        offset: i32,
        include_defaults: bool,
    ) -> fdo::Result<(HashMap<&'static str, String>, i32, i32)> {
        self.node
            .text_attribute_run(offset, include_defaults)
            .map_err(self.map_error())
    }

    fn scroll_substring_to(
        &self,
        start_offset: i32,
        end_offset: i32,
        scroll_type: ScrollType,
    ) -> fdo::Result<bool> {
        self.node
            .scroll_substring_to(start_offset, end_offset, scroll_type)
            .map_err(self.map_error())
    }

    fn scroll_substring_to_point(
        &self,
        start_offset: i32,
        end_offset: i32,
        coord_type: CoordType,
        x: i32,
        y: i32,
    ) -> fdo::Result<bool> {
        self.node
            .scroll_substring_to_point(start_offset, end_offset, coord_type, x, y)
            .map_err(self.map_error())
    }
}
