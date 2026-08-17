// Copyright 2021 The AccessKit Authors. All rights reserved.
// Licensed under the Apache License, Version 2.0 (found in
// the LICENSE-APACHE file) or the MIT license (found in
// the LICENSE-MIT file), at your option.

// Derived from Chromium's accessibility abstraction.
// Copyright 2021 The Chromium Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE.chromium file.

use accesskit::{
    Action, Affine, AriaCurrent, HasPopup, Live, Node as NodeData, NodeId as LocalNodeId,
    Orientation, Point, Rect, Role, SortDirection, TextSelection, Toggled, TreeId,
};
use alloc::{
    string::{String, ToString},
    vec::Vec,
};
use core::{fmt, iter::FusedIterator};

use crate::filters::FilterResult;
use crate::iterators::{
    ChildIds, FilteredChildren, FollowingFilteredSiblings, FollowingSiblings, LabelledBy,
    PrecedingFilteredSiblings, PrecedingSiblings,
};
use crate::tree::{State as TreeState, TreeIndex};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct NodeId(TreeIndex, LocalNodeId);

impl NodeId {
    pub(crate) fn new(local_id: LocalNodeId, tree_index: TreeIndex) -> Self {
        Self(tree_index, local_id)
    }

    pub(crate) fn with_same_tree(&self, local_id: LocalNodeId) -> Self {
        Self(self.0, local_id)
    }

    pub(crate) fn to_components(self) -> (LocalNodeId, TreeIndex) {
        (self.1, self.0)
    }
}

impl From<NodeId> for u128 {
    fn from(id: NodeId) -> Self {
        let tree_index = id.0 .0 as u128;
        let local_id = id.1 .0 as u128;
        (local_id << 64) | tree_index
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) struct ParentAndIndex(pub(crate) NodeId, pub(crate) usize);

#[derive(Clone, Debug)]
pub(crate) struct NodeState {
    pub(crate) parent_and_index: Option<ParentAndIndex>,
    pub(crate) data: NodeData,
}

#[derive(Copy, Clone, Debug)]
pub struct Node<'a> {
    pub tree_state: &'a TreeState,
    pub(crate) id: NodeId,
    pub(crate) state: &'a NodeState,
}

impl<'a> Node<'a> {
    pub fn data(&self) -> &'a NodeData {
        &self.state.data
    }

    pub fn is_focused(&self) -> bool {
        let dominated_by_active_descendant = |node_id| {
            self.tree_state
                .node_by_id(node_id)
                .and_then(|node| node.active_descendant())
                .is_some()
        };
        match self.tree_state.focus_id() {
            Some(focus_id) if focus_id == self.id() => !dominated_by_active_descendant(focus_id),
            Some(focus_id) => self
                .tree_state
                .node_by_id(focus_id)
                .and_then(|focused| focused.active_descendant())
                .is_some_and(|active_descendant| active_descendant.id() == self.id()),
            None => false,
        }
    }

    pub fn is_focused_in_tree(&self) -> bool {
        self.tree_state.focus == self.id()
    }

    pub fn is_focusable(&self, parent_filter: &impl Fn(&Node) -> FilterResult) -> bool {
        self.supports_action(Action::Focus, parent_filter) || self.is_focused_in_tree()
    }

    pub fn is_root(&self) -> bool {
        // Don't check for absence of a parent node, in case a non-root node
        // somehow gets detached from the tree.
        self.id() == self.tree_state.root_id()
    }

    /// Returns true if this node is a graft node (has a tree_id property set).
    pub fn is_graft(&self) -> bool {
        self.state.data.tree_id().is_some()
    }

    pub fn parent_id(&self) -> Option<NodeId> {
        self.state
            .parent_and_index
            .as_ref()
            .map(|ParentAndIndex(id, _)| *id)
    }

    pub fn parent(&self) -> Option<Node<'a>> {
        self.parent_id()
            .map(|id| self.tree_state.node_by_id(id).unwrap())
    }

    pub fn filtered_parent(&self, filter: &impl Fn(&Node) -> FilterResult) -> Option<Node<'a>> {
        self.parent().and_then(move |parent| {
            if filter(&parent) == FilterResult::Include {
                Some(parent)
            } else {
                parent.filtered_parent(filter)
            }
        })
    }

    pub fn parent_and_index(self) -> Option<(Node<'a>, usize)> {
        self.state
            .parent_and_index
            .as_ref()
            .map(|ParentAndIndex(parent, index)| {
                (self.tree_state.node_by_id(*parent).unwrap(), *index)
            })
    }

    /// Returns the single child of a graft node (the subtree root), if available.
    fn graft_child_id(&self) -> Option<NodeId> {
        self.state
            .data
            .tree_id()
            .and_then(|tree_id| self.tree_state.subtree_root(tree_id))
    }

    pub fn child_ids(
        &self,
    ) -> impl DoubleEndedIterator<Item = NodeId>
           + ExactSizeIterator<Item = NodeId>
           + FusedIterator<Item = NodeId>
           + 'a {
        if self.is_graft() {
            ChildIds::Graft(self.graft_child_id())
        } else {
            ChildIds::Normal {
                parent_id: self.id,
                children: self.state.data.children().iter(),
            }
        }
    }

    pub fn children(
        &self,
    ) -> impl DoubleEndedIterator<Item = Node<'a>>
           + ExactSizeIterator<Item = Node<'a>>
           + FusedIterator<Item = Node<'a>>
           + 'a {
        let state = self.tree_state;
        self.child_ids()
            .map(move |id| state.node_by_id(id).unwrap())
    }

    pub fn filtered_children(
        &self,
        filter: impl Fn(&Node) -> FilterResult + 'a,
    ) -> impl DoubleEndedIterator<Item = Node<'a>> + FusedIterator<Item = Node<'a>> + 'a {
        FilteredChildren::new(*self, filter)
    }

    pub fn following_sibling_ids(
        &self,
    ) -> impl DoubleEndedIterator<Item = NodeId>
           + ExactSizeIterator<Item = NodeId>
           + FusedIterator<Item = NodeId>
           + 'a {
        FollowingSiblings::new(*self)
    }

    pub fn following_siblings(
        &self,
    ) -> impl DoubleEndedIterator<Item = Node<'a>>
           + ExactSizeIterator<Item = Node<'a>>
           + FusedIterator<Item = Node<'a>>
           + 'a {
        let state = self.tree_state;
        self.following_sibling_ids()
            .map(move |id| state.node_by_id(id).unwrap())
    }

    pub fn following_filtered_siblings(
        &self,
        filter: impl Fn(&Node) -> FilterResult + 'a,
    ) -> impl DoubleEndedIterator<Item = Node<'a>> + FusedIterator<Item = Node<'a>> + 'a {
        FollowingFilteredSiblings::new(*self, filter)
    }

    pub fn preceding_sibling_ids(
        &self,
    ) -> impl DoubleEndedIterator<Item = NodeId>
           + ExactSizeIterator<Item = NodeId>
           + FusedIterator<Item = NodeId>
           + 'a {
        PrecedingSiblings::new(*self)
    }

    pub fn preceding_siblings(
        &self,
    ) -> impl DoubleEndedIterator<Item = Node<'a>>
           + ExactSizeIterator<Item = Node<'a>>
           + FusedIterator<Item = Node<'a>>
           + 'a {
        let state = self.tree_state;
        self.preceding_sibling_ids()
            .map(move |id| state.node_by_id(id).unwrap())
    }

    pub fn preceding_filtered_siblings(
        &self,
        filter: impl Fn(&Node) -> FilterResult + 'a,
    ) -> impl DoubleEndedIterator<Item = Node<'a>> + FusedIterator<Item = Node<'a>> + 'a {
        PrecedingFilteredSiblings::new(*self, filter)
    }

    pub fn deepest_first_child(self) -> Option<Node<'a>> {
        let mut deepest_child = self.children().next()?;
        while let Some(first_child) = deepest_child.children().next() {
            deepest_child = first_child;
        }
        Some(deepest_child)
    }

    pub fn deepest_first_filtered_child(
        &self,
        filter: &impl Fn(&Node) -> FilterResult,
    ) -> Option<Node<'a>> {
        let mut deepest_child = self.first_filtered_child(filter)?;
        while let Some(first_child) = deepest_child.first_filtered_child(filter) {
            deepest_child = first_child;
        }
        Some(deepest_child)
    }

    pub fn deepest_last_child(self) -> Option<Node<'a>> {
        let mut deepest_child = self.children().next_back()?;
        while let Some(last_child) = deepest_child.children().next_back() {
            deepest_child = last_child;
        }
        Some(deepest_child)
    }

    pub fn deepest_last_filtered_child(
        &self,
        filter: &impl Fn(&Node) -> FilterResult,
    ) -> Option<Node<'a>> {
        let mut deepest_child = self.last_filtered_child(filter)?;
        while let Some(last_child) = deepest_child.last_filtered_child(filter) {
            deepest_child = last_child;
        }
        Some(deepest_child)
    }

    pub fn is_descendant_of(&self, ancestor: &Node) -> bool {
        if self.id() == ancestor.id() {
            return true;
        }
        if let Some(parent) = self.parent() {
            return parent.is_descendant_of(ancestor);
        }
        false
    }

    /// Returns the transform defined directly on this node, or the identity
    /// transform, without taking into account transforms on ancestors.
    pub fn direct_transform(&self) -> Affine {
        self.data()
            .transform()
            .map_or(Affine::IDENTITY, |value| *value)
    }

    /// Returns the combined affine transform of this node and its ancestors,
    /// up to and including the root of this node's tree.
    pub fn transform(&self) -> Affine {
        self.parent()
            .map_or(Affine::IDENTITY, |parent| parent.transform())
            * self.direct_transform()
    }

    pub(crate) fn relative_transform(&self, stop_at: &Node) -> Affine {
        let parent_transform = if let Some(parent) = self.parent() {
            if parent.id() == stop_at.id() {
                Affine::IDENTITY
            } else {
                parent.relative_transform(stop_at)
            }
        } else {
            Affine::IDENTITY
        };
        parent_transform * self.direct_transform()
    }

    pub fn raw_bounds(&self) -> Option<Rect> {
        self.data().bounds()
    }

    pub fn has_bounds(&self) -> bool {
        self.raw_bounds().is_some()
    }

    /// Returns the node's transformed bounding box relative to the tree's
    /// container (e.g. window).
    pub fn bounding_box(&self) -> Option<Rect> {
        self.raw_bounds()
            .as_ref()
            .map(|rect| self.transform().transform_rect_bbox(*rect))
    }

    pub(crate) fn bounding_box_in_coordinate_space(&self, other: &Node) -> Option<Rect> {
        self.raw_bounds()
            .as_ref()
            .map(|rect| self.relative_transform(other).transform_rect_bbox(*rect))
    }

    pub(crate) fn hit_test(
        &self,
        point: Point,
        filter: &impl Fn(&Node) -> FilterResult,
    ) -> Option<(Node<'a>, Point)> {
        let filter_result = filter(self);

        if filter_result == FilterResult::ExcludeSubtree {
            return None;
        }

        for child in self.children().rev() {
            let point = child.direct_transform().inverse() * point;
            if let Some(result) = child.hit_test(point, filter) {
                return Some(result);
            }
        }

        if filter_result == FilterResult::Include {
            if let Some(rect) = &self.raw_bounds() {
                if rect.contains(point) {
                    return Some((*self, point));
                }
            }
        }

        None
    }

    /// Returns the deepest filtered node, either this node or a descendant,
    /// at the given point in this node's coordinate space.
    pub fn node_at_point(
        &self,
        point: Point,
        filter: &impl Fn(&Node) -> FilterResult,
    ) -> Option<Node<'a>> {
        self.hit_test(point, filter).map(|(node, _)| node)
    }

    pub fn id(&self) -> NodeId {
        self.id
    }

    pub fn locate(&self) -> (LocalNodeId, TreeId) {
        self.tree_state.locate_node(self.id).unwrap()
    }

    pub fn role(&self) -> Role {
        self.data().role()
    }

    pub fn role_description(&self) -> Option<&str> {
        self.data().role_description()
    }

    pub fn has_role_description(&self) -> bool {
        self.data().role_description().is_some()
    }

    pub fn is_live_atomic(&self) -> bool {
        self.data().is_live_atomic()
    }

    pub fn is_busy(&self) -> bool {
        self.data().is_busy()
    }

    pub fn column_index_text(&self) -> Option<&str> {
        self.data().column_index_text()
    }

    pub fn row_index_text(&self) -> Option<&str> {
        self.data().row_index_text()
    }

    pub fn braille_label(&self) -> Option<&str> {
        self.data().braille_label()
    }

    pub fn has_braille_label(&self) -> bool {
        self.data().braille_label().is_some()
    }

    pub fn braille_role_description(&self) -> Option<&str> {
        self.data().braille_role_description()
    }

    pub fn has_braille_role_description(&self) -> bool {
        self.data().braille_role_description().is_some()
    }

    pub fn aria_current(&self) -> Option<AriaCurrent> {
        self.data().aria_current()
    }

    pub fn has_popup(&self) -> Option<HasPopup> {
        self.data().has_popup()
    }

    pub fn is_hidden(&self) -> bool {
        self.fetch_inherited_flag(NodeData::is_hidden)
    }

    pub fn level(&self) -> Option<usize> {
        self.data().level()
    }

    pub fn is_disabled(&self) -> bool {
        self.data().is_disabled()
    }

    pub fn is_read_only(&self) -> bool {
        let data = self.data();
        if data.is_read_only() {
            true
        } else {
            self.should_have_read_only_state_by_default() || !self.is_read_only_supported()
        }
    }

    pub fn is_read_only_or_disabled(&self) -> bool {
        self.is_read_only() || self.is_disabled()
    }

    pub fn toggled(&self) -> Option<Toggled> {
        self.data().toggled()
    }

    pub fn numeric_value(&self) -> Option<f64> {
        self.data().numeric_value()
    }

    pub fn min_numeric_value(&self) -> Option<f64> {
        self.data().min_numeric_value()
    }

    pub fn max_numeric_value(&self) -> Option<f64> {
        self.data().max_numeric_value()
    }

    pub fn numeric_value_step(&self) -> Option<f64> {
        self.data().numeric_value_step()
    }

    pub fn numeric_value_jump(&self) -> Option<f64> {
        self.data().numeric_value_jump()
    }

    pub fn clips_children(&self) -> bool {
        self.data().clips_children()
    }

    pub fn scroll_x(&self) -> Option<f64> {
        self.data().scroll_x()
    }

    pub fn scroll_x_min(&self) -> Option<f64> {
        self.data().scroll_x_min()
    }

    pub fn scroll_x_max(&self) -> Option<f64> {
        self.data().scroll_x_max()
    }

    pub fn scroll_y(&self) -> Option<f64> {
        self.data().scroll_y()
    }

    pub fn scroll_y_min(&self) -> Option<f64> {
        self.data().scroll_y_min()
    }

    pub fn scroll_y_max(&self) -> Option<f64> {
        self.data().scroll_y_max()
    }

    pub(crate) fn fetch_inherited_property<T>(
        &self,
        getter: fn(&'a NodeData) -> Option<T>,
    ) -> Option<T> {
        let mut node = *self;
        loop {
            let value = getter(node.data());
            if value.is_some() {
                return value;
            }
            node = node.parent()?;
        }
    }

    pub(crate) fn fetch_inherited_flag(&self, getter: fn(&'a NodeData) -> bool) -> bool {
        let mut node = *self;
        loop {
            if getter(node.data()) {
                return true;
            }
            if let Some(parent) = node.parent() {
                node = parent;
            } else {
                return false;
            }
        }
    }

    pub fn is_text_input(&self) -> bool {
        matches!(
            self.role(),
            Role::TextInput
                | Role::MultilineTextInput
                | Role::SearchInput
                | Role::DateInput
                | Role::DateTimeInput
                | Role::WeekInput
                | Role::MonthInput
                | Role::TimeInput
                | Role::EmailInput
                | Role::NumberInput
                | Role::PasswordInput
                | Role::PhoneNumberInput
                | Role::UrlInput
                | Role::EditableComboBox
                | Role::SpinButton
        )
    }

    pub fn is_multiline(&self) -> bool {
        self.role() == Role::MultilineTextInput
    }

    pub fn orientation(&self) -> Option<Orientation> {
        self.data().orientation().or_else(|| {
            if self.role() == Role::ListBox {
                Some(Orientation::Vertical)
            } else if self.role() == Role::TabList {
                Some(Orientation::Horizontal)
            } else {
                None
            }
        })
    }

    pub fn is_dialog(&self) -> bool {
        matches!(self.role(), Role::AlertDialog | Role::Dialog)
    }

    pub fn is_modal(&self) -> bool {
        self.data().is_modal()
    }

    // When probing for supported actions as the next several functions do,
    // it's tempting to check the role. But it's better to not assume anything
    // beyond what the provider has explicitly told us. Rationale:
    // if the provider developer forgot to call `add_action` for an action,
    // an AT (or even AccessKit itself) can fall back to simulating
    // a mouse click. But if the provider doesn't handle an action request
    // and we assume that it will based on the role, the attempted action
    // does nothing. This stance is a departure from Chromium.

    pub fn is_clickable(&self, parent_filter: &impl Fn(&Node) -> FilterResult) -> bool {
        self.supports_action(Action::Click, parent_filter)
    }

    pub fn is_selectable(&self) -> bool {
        // It's selectable if it has the attribute, whether it's true or false.
        self.is_selected().is_some() && !self.is_disabled()
    }

    pub fn is_multiselectable(&self) -> bool {
        self.data().is_multiselectable()
    }

    pub fn size_of_set_from_container(
        &self,
        filter: &impl Fn(&Node) -> FilterResult,
    ) -> Option<usize> {
        self.selection_container(filter)
            .and_then(|c| c.size_of_set())
    }

    pub fn size_of_set(&self) -> Option<usize> {
        // TODO: compute this if it is not provided (#9).
        self.data().size_of_set()
    }

    pub fn position_in_set(&self) -> Option<usize> {
        // TODO: compute this if it is not provided (#9).
        self.data().position_in_set()
    }

    pub fn sort_direction(&self) -> Option<SortDirection> {
        self.data().sort_direction()
    }

    pub fn supports_toggle(&self) -> bool {
        self.toggled().is_some()
    }

    pub fn supports_expand_collapse(&self) -> bool {
        self.data().is_expanded().is_some()
    }

    pub fn is_invocable(&self, parent_filter: &impl Fn(&Node) -> FilterResult) -> bool {
        // A control is "invocable" if it initiates an action when activated but
        // does not maintain any state. A control that maintains state
        // when activated would be considered a toggle or expand-collapse
        // control - these controls are "clickable" but not "invocable".
        // Similarly, if the action only gives the control keyboard focus,
        // such as when clicking a text input, the control is not considered
        // "invocable", as the "invoke" action would be a redundant synonym
        // for the "set focus" action. The same logic applies to selection.
        self.is_clickable(parent_filter)
            && !self.is_text_input()
            && !matches!(self.role(), Role::Document | Role::Terminal)
            && !self.supports_toggle()
            && !self.supports_expand_collapse()
            && self.is_selected().is_none()
    }

    pub fn supports_action(
        &self,
        action: Action,
        parent_filter: &impl Fn(&Node) -> FilterResult,
    ) -> bool {
        if self.data().supports_action(action) {
            return true;
        }
        if let Some(parent) = self.filtered_parent(parent_filter) {
            return parent.data().child_supports_action(action);
        }
        false
    }

    pub fn supports_increment(&self, parent_filter: &impl Fn(&Node) -> FilterResult) -> bool {
        self.supports_action(Action::Increment, parent_filter)
    }

    pub fn supports_decrement(&self, parent_filter: &impl Fn(&Node) -> FilterResult) -> bool {
        self.supports_action(Action::Decrement, parent_filter)
    }
}

fn descendant_label_filter(node: &Node) -> FilterResult {
    match node.role() {
        Role::Label | Role::Image => FilterResult::Include,
        Role::GenericContainer => FilterResult::ExcludeNode,
        _ => FilterResult::ExcludeSubtree,
    }
}

impl<'a> Node<'a> {
    pub fn labelled_by(
        &self,
    ) -> impl DoubleEndedIterator<Item = Node<'a>> + FusedIterator<Item = Node<'a>> + 'a {
        let explicit = &self.state.data.labelled_by();
        if explicit.is_empty()
            && matches!(
                self.role(),
                Role::Button
                    | Role::CheckBox
                    | Role::DefaultButton
                    | Role::Link
                    | Role::MenuItem
                    | Role::MenuItemCheckBox
                    | Role::MenuItemRadio
                    | Role::RadioButton
            )
        {
            LabelledBy::FromDescendants(FilteredChildren::new(*self, &descendant_label_filter))
        } else {
            LabelledBy::Explicit {
                ids: explicit.iter(),
                tree_state: self.tree_state,
                node_id: self.id,
            }
        }
    }

    pub fn label_comes_from_value(&self) -> bool {
        self.role() == Role::Label
    }

    pub fn label(&self) -> Option<String> {
        let mut result = String::new();
        self.write_label(&mut result).unwrap().then_some(result)
    }

    fn write_label_direct<W: fmt::Write>(&self, mut writer: W) -> Result<bool, fmt::Error> {
        if let Some(label) = &self.data().label() {
            writer.write_str(label)?;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    pub fn write_label<W: fmt::Write>(&self, mut writer: W) -> Result<bool, fmt::Error> {
        if self.write_label_direct(&mut writer)? {
            Ok(true)
        } else {
            let mut wrote_one = false;
            for node in self.labelled_by() {
                let writer = SpacePrefixingWriter {
                    inner: &mut writer,
                    need_prefix: wrote_one,
                };
                let wrote_this_time = if node.label_comes_from_value() {
                    node.write_value(writer)
                } else {
                    node.write_label_direct(writer)
                }?;
                wrote_one = wrote_one || wrote_this_time;
            }
            Ok(wrote_one)
        }
    }

    pub fn description(&self) -> Option<String> {
        self.data()
            .description()
            .map(|description| description.to_string())
    }

    pub fn url(&self) -> Option<&str> {
        self.data().url()
    }

    pub fn supports_url(&self) -> bool {
        matches!(
            self.role(),
            Role::Link
                | Role::DocBackLink
                | Role::DocBiblioRef
                | Role::DocGlossRef
                | Role::DocNoteRef
        ) && self.url().is_some()
    }

    fn is_empty_text_input(&self) -> bool {
        let mut text_runs = self.text_runs();
        if let Some(first_text_run) = text_runs.next() {
            first_text_run
                .data()
                .value()
                .is_none_or(|value| value.is_empty())
                && text_runs.next().is_none()
        } else {
            true
        }
    }

    pub fn placeholder(&self) -> Option<&str> {
        self.data()
            .placeholder()
            .filter(|_| self.is_text_input() && self.is_empty_text_input())
    }

    pub fn value(&self) -> Option<String> {
        let mut result = String::new();
        self.write_value(&mut result).unwrap().then_some(result)
    }

    pub fn write_value<W: fmt::Write>(&self, mut writer: W) -> Result<bool, fmt::Error> {
        if let Some(value) = &self.data().value() {
            writer.write_str(value)?;
            Ok(true)
        } else if self.supports_text_ranges() && !self.is_multiline() {
            self.document_range().write_text(writer)?;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    pub fn has_value(&self) -> bool {
        self.data().value().is_some() || (self.supports_text_ranges() && !self.is_multiline())
    }

    pub fn is_read_only_supported(&self) -> bool {
        self.is_text_input()
            || matches!(
                self.role(),
                Role::CheckBox
                    | Role::ColorWell
                    | Role::ComboBox
                    | Role::Grid
                    | Role::ListBox
                    | Role::MenuItemCheckBox
                    | Role::MenuItemRadio
                    | Role::MenuListPopup
                    | Role::RadioButton
                    | Role::RadioGroup
                    | Role::Slider
                    | Role::Switch
                    | Role::TreeGrid
            )
    }

    pub fn should_have_read_only_state_by_default(&self) -> bool {
        matches!(
            self.role(),
            Role::Article
                | Role::Definition
                | Role::DescriptionList
                | Role::Document
                | Role::GraphicsDocument
                | Role::Image
                | Role::List
                | Role::ListItem
                | Role::PdfRoot
                | Role::ProgressIndicator
                | Role::RootWebArea
                | Role::Term
                | Role::Timer
                | Role::Toolbar
                | Role::Tooltip
        )
    }

    pub fn is_required(&self) -> bool {
        self.data().is_required()
    }

    pub fn live(&self) -> Live {
        self.data()
            .live()
            .unwrap_or_else(|| self.parent().map_or(Live::Off, |parent| parent.live()))
    }

    pub fn is_selected(&self) -> Option<bool> {
        self.data().is_selected()
    }

    pub fn is_item_like(&self) -> bool {
        matches!(
            self.role(),
            Role::Article
                | Role::Comment
                | Role::ListItem
                | Role::MenuItem
                | Role::MenuItemRadio
                | Role::Tab
                | Role::MenuItemCheckBox
                | Role::TreeItem
                | Role::ListBoxOption
                | Role::MenuListOption
                | Role::RadioButton
                | Role::Term
        )
    }

    pub fn is_container_with_selectable_children(&self) -> bool {
        matches!(
            self.role(),
            Role::ComboBox
                | Role::EditableComboBox
                | Role::Grid
                | Role::ListBox
                | Role::ListGrid
                | Role::Menu
                | Role::MenuBar
                | Role::MenuListPopup
                | Role::RadioGroup
                | Role::TabList
                | Role::Toolbar
                | Role::Tree
                | Role::TreeGrid
        )
    }

    pub fn controls(
        &self,
    ) -> impl DoubleEndedIterator<Item = Node<'a>> + FusedIterator<Item = Node<'a>> + 'a {
        let state = self.tree_state;
        let id = self.id;
        let data = &self.state.data;
        data.controls()
            .iter()
            .map(move |control_id| state.node_by_id(id.with_same_tree(*control_id)).unwrap())
    }

    pub fn active_descendant(&self) -> Option<Node<'a>> {
        self.state
            .data
            .active_descendant()
            .and_then(|id| self.tree_state.node_by_id(self.id.with_same_tree(id)))
    }

    pub fn raw_text_selection(&self) -> Option<&TextSelection> {
        self.data().text_selection()
    }

    pub fn raw_value(&self) -> Option<&str> {
        self.data().value()
    }

    pub fn author_id(&self) -> Option<&str> {
        self.data().author_id()
    }

    pub fn class_name(&self) -> Option<&str> {
        self.data().class_name()
    }

    pub fn index_path(&self) -> Vec<usize> {
        self.relative_index_path(self.tree_state.root_id())
    }

    pub fn relative_index_path(&self, ancestor_id: NodeId) -> Vec<usize> {
        let mut result = Vec::new();
        let mut current = *self;
        while current.id() != ancestor_id {
            let (parent, index) = current.parent_and_index().unwrap();
            result.push(index);
            current = parent;
        }
        result.reverse();
        result
    }

    pub(crate) fn first_filtered_child(
        &self,
        filter: &impl Fn(&Node) -> FilterResult,
    ) -> Option<Node<'a>> {
        for child in self.children() {
            let result = filter(&child);
            if result == FilterResult::Include {
                return Some(child);
            }
            if result == FilterResult::ExcludeNode {
                if let Some(descendant) = child.first_filtered_child(filter) {
                    return Some(descendant);
                }
            }
        }
        None
    }

    pub(crate) fn last_filtered_child(
        &self,
        filter: &impl Fn(&Node) -> FilterResult,
    ) -> Option<Node<'a>> {
        for child in self.children().rev() {
            let result = filter(&child);
            if result == FilterResult::Include {
                return Some(child);
            }
            if result == FilterResult::ExcludeNode {
                if let Some(descendant) = child.last_filtered_child(filter) {
                    return Some(descendant);
                }
            }
        }
        None
    }

    pub fn selection_container(&self, filter: &impl Fn(&Node) -> FilterResult) -> Option<Node<'a>> {
        self.filtered_parent(&|parent| match filter(parent) {
            FilterResult::Include if parent.is_container_with_selectable_children() => {
                FilterResult::Include
            }
            FilterResult::Include => FilterResult::ExcludeNode,
            filter_result => filter_result,
        })
    }

    pub fn items(
        &self,
        filter: impl Fn(&Node) -> FilterResult + 'a,
    ) -> impl DoubleEndedIterator<Item = Node<'a>> + FusedIterator<Item = Node<'a>> + 'a {
        self.filtered_children(move |child| match filter(child) {
            FilterResult::Include if child.is_item_like() => FilterResult::Include,
            FilterResult::Include => FilterResult::ExcludeNode,
            filter_result => filter_result,
        })
    }
}

struct SpacePrefixingWriter<W: fmt::Write> {
    inner: W,
    need_prefix: bool,
}

impl<W: fmt::Write> SpacePrefixingWriter<W> {
    fn write_prefix_if_needed(&mut self) -> fmt::Result {
        if self.need_prefix {
            self.inner.write_char(' ')?;
            self.need_prefix = false;
        }
        Ok(())
    }
}

impl<W: fmt::Write> fmt::Write for SpacePrefixingWriter<W> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_prefix_if_needed()?;
        self.inner.write_str(s)
    }

    fn write_char(&mut self, c: char) -> fmt::Result {
        self.write_prefix_if_needed()?;
        self.inner.write_char(c)
    }
}

#[cfg(test)]
mod tests {
    use accesskit::{
        Action, Node, NodeId, Point, Rect, Role, TextDirection, TextPosition, TextSelection, Tree,
        TreeId, TreeUpdate,
    };
    use alloc::vec;

    use crate::tests::*;

    #[test]
    fn parent_and_index() {
        let tree = test_tree();
        assert!(tree.state().root().parent_and_index().is_none());
        assert_eq!(
            Some((ROOT_ID, 0)),
            tree.state()
                .node_by_id(nid(PARAGRAPH_0_ID))
                .unwrap()
                .parent_and_index()
                .map(|(parent, index)| (parent.id().to_components().0, index))
        );
        assert_eq!(
            Some((PARAGRAPH_0_ID, 0)),
            tree.state()
                .node_by_id(nid(LABEL_0_0_IGNORED_ID))
                .unwrap()
                .parent_and_index()
                .map(|(parent, index)| (parent.id().to_components().0, index))
        );
        assert_eq!(
            Some((ROOT_ID, 1)),
            tree.state()
                .node_by_id(nid(PARAGRAPH_1_IGNORED_ID))
                .unwrap()
                .parent_and_index()
                .map(|(parent, index)| (parent.id().to_components().0, index))
        );
    }

    #[test]
    fn deepest_first_child() {
        let tree = test_tree();
        assert_eq!(
            LABEL_0_0_IGNORED_ID,
            tree.state()
                .root()
                .deepest_first_child()
                .unwrap()
                .id()
                .to_components()
                .0
        );
        assert_eq!(
            LABEL_0_0_IGNORED_ID,
            tree.state()
                .node_by_id(nid(PARAGRAPH_0_ID))
                .unwrap()
                .deepest_first_child()
                .unwrap()
                .id()
                .to_components()
                .0
        );
        assert!(tree
            .state()
            .node_by_id(nid(LABEL_0_0_IGNORED_ID))
            .unwrap()
            .deepest_first_child()
            .is_none());
    }

    #[test]
    fn filtered_parent() {
        let tree = test_tree();
        assert_eq!(
            ROOT_ID,
            tree.state()
                .node_by_id(nid(LABEL_1_1_ID))
                .unwrap()
                .filtered_parent(&test_tree_filter)
                .unwrap()
                .id()
                .to_components()
                .0
        );
        assert!(tree
            .state()
            .root()
            .filtered_parent(&test_tree_filter)
            .is_none());
    }

    #[test]
    fn deepest_first_filtered_child() {
        let tree = test_tree();
        assert_eq!(
            PARAGRAPH_0_ID,
            tree.state()
                .root()
                .deepest_first_filtered_child(&test_tree_filter)
                .unwrap()
                .id()
                .to_components()
                .0
        );
        assert!(tree
            .state()
            .node_by_id(nid(PARAGRAPH_0_ID))
            .unwrap()
            .deepest_first_filtered_child(&test_tree_filter)
            .is_none());
        assert!(tree
            .state()
            .node_by_id(nid(LABEL_0_0_IGNORED_ID))
            .unwrap()
            .deepest_first_filtered_child(&test_tree_filter)
            .is_none());
    }

    #[test]
    fn deepest_last_child() {
        let tree = test_tree();
        assert_eq!(
            EMPTY_CONTAINER_3_3_IGNORED_ID,
            tree.state()
                .root()
                .deepest_last_child()
                .unwrap()
                .id()
                .to_components()
                .0
        );
        assert_eq!(
            EMPTY_CONTAINER_3_3_IGNORED_ID,
            tree.state()
                .node_by_id(nid(PARAGRAPH_3_IGNORED_ID))
                .unwrap()
                .deepest_last_child()
                .unwrap()
                .id()
                .to_components()
                .0
        );
        assert!(tree
            .state()
            .node_by_id(nid(BUTTON_3_2_ID))
            .unwrap()
            .deepest_last_child()
            .is_none());
    }

    #[test]
    fn deepest_last_filtered_child() {
        let tree = test_tree();
        assert_eq!(
            BUTTON_3_2_ID,
            tree.state()
                .root()
                .deepest_last_filtered_child(&test_tree_filter)
                .unwrap()
                .id()
                .to_components()
                .0
        );
        assert_eq!(
            BUTTON_3_2_ID,
            tree.state()
                .node_by_id(nid(PARAGRAPH_3_IGNORED_ID))
                .unwrap()
                .deepest_last_filtered_child(&test_tree_filter)
                .unwrap()
                .id()
                .to_components()
                .0
        );
        assert!(tree
            .state()
            .node_by_id(nid(BUTTON_3_2_ID))
            .unwrap()
            .deepest_last_filtered_child(&test_tree_filter)
            .is_none());
        assert!(tree
            .state()
            .node_by_id(nid(PARAGRAPH_0_ID))
            .unwrap()
            .deepest_last_filtered_child(&test_tree_filter)
            .is_none());
    }

    #[test]
    fn is_descendant_of() {
        let tree = test_tree();
        assert!(tree
            .state()
            .node_by_id(nid(PARAGRAPH_0_ID))
            .unwrap()
            .is_descendant_of(&tree.state().root()));
        assert!(tree
            .state()
            .node_by_id(nid(LABEL_0_0_IGNORED_ID))
            .unwrap()
            .is_descendant_of(&tree.state().root()));
        assert!(tree
            .state()
            .node_by_id(nid(LABEL_0_0_IGNORED_ID))
            .unwrap()
            .is_descendant_of(&tree.state().node_by_id(nid(PARAGRAPH_0_ID)).unwrap()));
        assert!(!tree
            .state()
            .node_by_id(nid(LABEL_0_0_IGNORED_ID))
            .unwrap()
            .is_descendant_of(&tree.state().node_by_id(nid(PARAGRAPH_2_ID)).unwrap()));
        assert!(!tree
            .state()
            .node_by_id(nid(PARAGRAPH_0_ID))
            .unwrap()
            .is_descendant_of(&tree.state().node_by_id(nid(PARAGRAPH_2_ID)).unwrap()));
    }

    #[test]
    fn is_root() {
        let tree = test_tree();
        assert!(tree.state().node_by_id(nid(ROOT_ID)).unwrap().is_root());
        assert!(!tree
            .state()
            .node_by_id(nid(PARAGRAPH_0_ID))
            .unwrap()
            .is_root());
    }

    #[test]
    fn locate() {
        let tree = test_tree();
        let root = tree.state().root();
        assert_eq!((ROOT_ID, TreeId::ROOT), root.locate());
        let child = tree.state().node_by_id(nid(PARAGRAPH_0_ID)).unwrap();
        assert_eq!((PARAGRAPH_0_ID, TreeId::ROOT), child.locate());
    }

    #[test]
    fn bounding_box() {
        let tree = test_tree();
        assert!(tree
            .state()
            .node_by_id(nid(ROOT_ID))
            .unwrap()
            .bounding_box()
            .is_none());
        assert_eq!(
            Some(Rect {
                x0: 10.0,
                y0: 40.0,
                x1: 810.0,
                y1: 80.0,
            }),
            tree.state()
                .node_by_id(nid(PARAGRAPH_1_IGNORED_ID))
                .unwrap()
                .bounding_box()
        );
        assert_eq!(
            Some(Rect {
                x0: 20.0,
                y0: 50.0,
                x1: 100.0,
                y1: 70.0,
            }),
            tree.state()
                .node_by_id(nid(LABEL_1_1_ID))
                .unwrap()
                .bounding_box()
        );
    }

    #[test]
    fn node_at_point() {
        let tree = test_tree();
        assert!(tree
            .state()
            .root()
            .node_at_point(Point::new(10.0, 40.0), &test_tree_filter)
            .is_none());
        assert_eq!(
            Some(nid(LABEL_1_1_ID)),
            tree.state()
                .root()
                .node_at_point(Point::new(20.0, 50.0), &test_tree_filter)
                .map(|node| node.id())
        );
        assert_eq!(
            Some(nid(LABEL_1_1_ID)),
            tree.state()
                .root()
                .node_at_point(Point::new(50.0, 60.0), &test_tree_filter)
                .map(|node| node.id())
        );
        assert!(tree
            .state()
            .root()
            .node_at_point(Point::new(100.0, 70.0), &test_tree_filter)
            .is_none());
    }

    #[test]
    fn no_label_or_labelled_by() {
        let update = TreeUpdate {
            nodes: vec![
                (NodeId(0), {
                    let mut node = Node::new(Role::Window);
                    node.set_children(vec![NodeId(1)]);
                    node
                }),
                (NodeId(1), Node::new(Role::Button)),
            ],
            tree: Some(Tree::new(NodeId(0))),
            tree_id: TreeId::ROOT,
            focus: NodeId(0),
        };
        let tree = crate::Tree::new(update, false);
        assert_eq!(
            None,
            tree.state().node_by_id(nid(NodeId(1))).unwrap().label()
        );
    }

    #[test]
    fn label_from_labelled_by() {
        // The following mock UI probably isn't very localization-friendly,
        // but it's good for this test.
        const LABEL_1: &str = "Check email every";
        const LABEL_2: &str = "minutes";

        let update = TreeUpdate {
            nodes: vec![
                (NodeId(0), {
                    let mut node = Node::new(Role::Window);
                    node.set_children(vec![NodeId(1), NodeId(2), NodeId(3), NodeId(4)]);
                    node
                }),
                (NodeId(1), {
                    let mut node = Node::new(Role::CheckBox);
                    node.set_labelled_by(vec![NodeId(2), NodeId(4)]);
                    node
                }),
                (NodeId(2), {
                    let mut node = Node::new(Role::Label);
                    node.set_value(LABEL_1);
                    node
                }),
                (NodeId(3), {
                    let mut node = Node::new(Role::TextInput);
                    node.push_labelled_by(NodeId(4));
                    node
                }),
                (NodeId(4), {
                    let mut node = Node::new(Role::Label);
                    node.set_value(LABEL_2);
                    node
                }),
            ],
            tree: Some(Tree::new(NodeId(0))),
            tree_id: TreeId::ROOT,
            focus: NodeId(0),
        };
        let tree = crate::Tree::new(update, false);
        assert_eq!(
            Some([LABEL_1, LABEL_2].join(" ")),
            tree.state().node_by_id(nid(NodeId(1))).unwrap().label()
        );
        assert_eq!(
            Some(LABEL_2.into()),
            tree.state().node_by_id(nid(NodeId(3))).unwrap().label()
        );
    }

    #[test]
    fn label_from_descendant_label() {
        const ROOT_ID: NodeId = NodeId(0);
        const DEFAULT_BUTTON_ID: NodeId = NodeId(1);
        const DEFAULT_BUTTON_LABEL_ID: NodeId = NodeId(2);
        const LINK_ID: NodeId = NodeId(3);
        const LINK_LABEL_CONTAINER_ID: NodeId = NodeId(4);
        const LINK_LABEL_ID: NodeId = NodeId(5);
        const CHECKBOX_ID: NodeId = NodeId(6);
        const CHECKBOX_LABEL_ID: NodeId = NodeId(7);
        const RADIO_BUTTON_ID: NodeId = NodeId(8);
        const RADIO_BUTTON_LABEL_ID: NodeId = NodeId(9);
        const MENU_BUTTON_ID: NodeId = NodeId(10);
        const MENU_BUTTON_LABEL_ID: NodeId = NodeId(11);
        const MENU_ID: NodeId = NodeId(12);
        const MENU_ITEM_ID: NodeId = NodeId(13);
        const MENU_ITEM_LABEL_ID: NodeId = NodeId(14);
        const MENU_ITEM_CHECKBOX_ID: NodeId = NodeId(15);
        const MENU_ITEM_CHECKBOX_LABEL_ID: NodeId = NodeId(16);
        const MENU_ITEM_RADIO_ID: NodeId = NodeId(17);
        const MENU_ITEM_RADIO_LABEL_ID: NodeId = NodeId(18);

        const DEFAULT_BUTTON_LABEL: &str = "Play";
        const LINK_LABEL: &str = "Watch in browser";
        const CHECKBOX_LABEL: &str = "Resume from previous position";
        const RADIO_BUTTON_LABEL: &str = "Normal speed";
        const MENU_BUTTON_LABEL: &str = "More";
        const MENU_ITEM_LABEL: &str = "Share";
        const MENU_ITEM_CHECKBOX_LABEL: &str = "Apply volume processing";
        const MENU_ITEM_RADIO_LABEL: &str = "Maximize loudness for noisy environment";

        let update = TreeUpdate {
            nodes: vec![
                (ROOT_ID, {
                    let mut node = Node::new(Role::Window);
                    node.set_children(vec![
                        DEFAULT_BUTTON_ID,
                        LINK_ID,
                        CHECKBOX_ID,
                        RADIO_BUTTON_ID,
                        MENU_BUTTON_ID,
                        MENU_ID,
                    ]);
                    node
                }),
                (DEFAULT_BUTTON_ID, {
                    let mut node = Node::new(Role::DefaultButton);
                    node.push_child(DEFAULT_BUTTON_LABEL_ID);
                    node
                }),
                (DEFAULT_BUTTON_LABEL_ID, {
                    let mut node = Node::new(Role::Image);
                    node.set_label(DEFAULT_BUTTON_LABEL);
                    node
                }),
                (LINK_ID, {
                    let mut node = Node::new(Role::Link);
                    node.push_child(LINK_LABEL_CONTAINER_ID);
                    node
                }),
                (LINK_LABEL_CONTAINER_ID, {
                    let mut node = Node::new(Role::GenericContainer);
                    node.push_child(LINK_LABEL_ID);
                    node
                }),
                (LINK_LABEL_ID, {
                    let mut node = Node::new(Role::Label);
                    node.set_value(LINK_LABEL);
                    node
                }),
                (CHECKBOX_ID, {
                    let mut node = Node::new(Role::CheckBox);
                    node.push_child(CHECKBOX_LABEL_ID);
                    node
                }),
                (CHECKBOX_LABEL_ID, {
                    let mut node = Node::new(Role::Label);
                    node.set_value(CHECKBOX_LABEL);
                    node
                }),
                (RADIO_BUTTON_ID, {
                    let mut node = Node::new(Role::RadioButton);
                    node.push_child(RADIO_BUTTON_LABEL_ID);
                    node
                }),
                (RADIO_BUTTON_LABEL_ID, {
                    let mut node = Node::new(Role::Label);
                    node.set_value(RADIO_BUTTON_LABEL);
                    node
                }),
                (MENU_BUTTON_ID, {
                    let mut node = Node::new(Role::Button);
                    node.push_child(MENU_BUTTON_LABEL_ID);
                    node
                }),
                (MENU_BUTTON_LABEL_ID, {
                    let mut node = Node::new(Role::Label);
                    node.set_value(MENU_BUTTON_LABEL);
                    node
                }),
                (MENU_ID, {
                    let mut node = Node::new(Role::Menu);
                    node.set_children([MENU_ITEM_ID, MENU_ITEM_CHECKBOX_ID, MENU_ITEM_RADIO_ID]);
                    node
                }),
                (MENU_ITEM_ID, {
                    let mut node = Node::new(Role::MenuItem);
                    node.push_child(MENU_ITEM_LABEL_ID);
                    node
                }),
                (MENU_ITEM_LABEL_ID, {
                    let mut node = Node::new(Role::Label);
                    node.set_value(MENU_ITEM_LABEL);
                    node
                }),
                (MENU_ITEM_CHECKBOX_ID, {
                    let mut node = Node::new(Role::MenuItemCheckBox);
                    node.push_child(MENU_ITEM_CHECKBOX_LABEL_ID);
                    node
                }),
                (MENU_ITEM_CHECKBOX_LABEL_ID, {
                    let mut node = Node::new(Role::Label);
                    node.set_value(MENU_ITEM_CHECKBOX_LABEL);
                    node
                }),
                (MENU_ITEM_RADIO_ID, {
                    let mut node = Node::new(Role::MenuItemRadio);
                    node.push_child(MENU_ITEM_RADIO_LABEL_ID);
                    node
                }),
                (MENU_ITEM_RADIO_LABEL_ID, {
                    let mut node = Node::new(Role::Label);
                    node.set_value(MENU_ITEM_RADIO_LABEL);
                    node
                }),
            ],
            tree: Some(Tree::new(ROOT_ID)),
            tree_id: TreeId::ROOT,
            focus: ROOT_ID,
        };
        let tree = crate::Tree::new(update, false);
        assert_eq!(
            Some(DEFAULT_BUTTON_LABEL.into()),
            tree.state()
                .node_by_id(nid(DEFAULT_BUTTON_ID))
                .unwrap()
                .label()
        );
        assert_eq!(
            Some(LINK_LABEL.into()),
            tree.state().node_by_id(nid(LINK_ID)).unwrap().label()
        );
        assert_eq!(
            Some(CHECKBOX_LABEL.into()),
            tree.state().node_by_id(nid(CHECKBOX_ID)).unwrap().label()
        );
        assert_eq!(
            Some(RADIO_BUTTON_LABEL.into()),
            tree.state()
                .node_by_id(nid(RADIO_BUTTON_ID))
                .unwrap()
                .label()
        );
        assert_eq!(
            Some(MENU_BUTTON_LABEL.into()),
            tree.state()
                .node_by_id(nid(MENU_BUTTON_ID))
                .unwrap()
                .label()
        );
        assert_eq!(
            Some(MENU_ITEM_LABEL.into()),
            tree.state().node_by_id(nid(MENU_ITEM_ID)).unwrap().label()
        );
        assert_eq!(
            Some(MENU_ITEM_CHECKBOX_LABEL.into()),
            tree.state()
                .node_by_id(nid(MENU_ITEM_CHECKBOX_ID))
                .unwrap()
                .label()
        );
        assert_eq!(
            Some(MENU_ITEM_RADIO_LABEL.into()),
            tree.state()
                .node_by_id(nid(MENU_ITEM_RADIO_ID))
                .unwrap()
                .label()
        );
    }

    #[test]
    fn placeholder_should_be_exposed_on_empty_text_input() {
        const ROOT_ID: NodeId = NodeId(0);
        const TEXT_INPUT_ID: NodeId = NodeId(1);
        const TEXT_RUN_ID: NodeId = NodeId(2);

        const PLACEHOLDER: &str = "John Doe";

        let update = TreeUpdate {
            nodes: vec![
                (ROOT_ID, {
                    let mut node = Node::new(Role::Window);
                    node.set_children(vec![TEXT_INPUT_ID]);
                    node
                }),
                (TEXT_INPUT_ID, {
                    let mut node = Node::new(Role::MultilineTextInput);
                    node.set_bounds(Rect {
                        x0: 8.0,
                        y0: 8.0,
                        x1: 296.0,
                        y1: 69.5,
                    });
                    node.push_child(TEXT_RUN_ID);
                    node.set_placeholder(PLACEHOLDER);
                    node.set_text_selection(TextSelection {
                        anchor: TextPosition {
                            node: TEXT_RUN_ID,
                            character_index: 0,
                        },
                        focus: TextPosition {
                            node: TEXT_RUN_ID,
                            character_index: 0,
                        },
                    });
                    node.add_action(Action::Focus);
                    node
                }),
                (TEXT_RUN_ID, {
                    let mut node = Node::new(Role::TextRun);
                    node.set_bounds(Rect {
                        x0: 12.0,
                        y0: 10.0,
                        x1: 12.0,
                        y1: 24.0,
                    });
                    node.set_value("");
                    node.set_character_lengths([]);
                    node.set_character_positions([]);
                    node.set_character_widths([]);
                    node.set_text_direction(TextDirection::LeftToRight);
                    node
                }),
            ],
            tree: Some(Tree::new(ROOT_ID)),
            tree_id: TreeId::ROOT,
            focus: TEXT_INPUT_ID,
        };
        let tree = crate::Tree::new(update, false);
        assert_eq!(
            Some(PLACEHOLDER),
            tree.state()
                .node_by_id(nid(TEXT_INPUT_ID))
                .unwrap()
                .placeholder()
        );
    }

    #[test]
    fn placeholder_should_be_ignored_on_non_empty_text_input() {
        const ROOT_ID: NodeId = NodeId(0);
        const TEXT_INPUT_ID: NodeId = NodeId(1);
        const TEXT_RUN_ID: NodeId = NodeId(2);

        const PLACEHOLDER: &str = "John Doe";

        let update = TreeUpdate {
            nodes: vec![
                (ROOT_ID, {
                    let mut node = Node::new(Role::Window);
                    node.set_children(vec![TEXT_INPUT_ID]);
                    node
                }),
                (TEXT_INPUT_ID, {
                    let mut node = Node::new(Role::MultilineTextInput);
                    node.set_bounds(Rect {
                        x0: 8.0,
                        y0: 8.0,
                        x1: 296.0,
                        y1: 69.5,
                    });
                    node.push_child(TEXT_RUN_ID);
                    node.set_placeholder(PLACEHOLDER);
                    node.set_text_selection(TextSelection {
                        anchor: TextPosition {
                            node: TEXT_RUN_ID,
                            character_index: 1,
                        },
                        focus: TextPosition {
                            node: TEXT_RUN_ID,
                            character_index: 1,
                        },
                    });
                    node.add_action(Action::Focus);
                    node
                }),
                (TEXT_RUN_ID, {
                    let mut node = Node::new(Role::TextRun);
                    node.set_bounds(Rect {
                        x0: 12.0,
                        y0: 10.0,
                        x1: 20.0,
                        y1: 24.0,
                    });
                    node.set_value("A");
                    node.set_character_lengths([1]);
                    node.set_character_positions([0.0]);
                    node.set_character_widths([8.0]);
                    node.set_word_starts([0]);
                    node.set_text_direction(TextDirection::LeftToRight);
                    node
                }),
            ],
            tree: Some(Tree::new(ROOT_ID)),
            tree_id: TreeId::ROOT,
            focus: TEXT_INPUT_ID,
        };
        let tree = crate::Tree::new(update, false);
        assert_eq!(
            None,
            tree.state()
                .node_by_id(nid(TEXT_INPUT_ID))
                .unwrap()
                .placeholder()
        );
    }

    #[test]
    fn hidden_flag_should_be_inherited() {
        const ROOT_ID: NodeId = NodeId(0);
        const CONTAINER_ID: NodeId = NodeId(1);
        const LEAF_ID: NodeId = NodeId(2);

        let update = TreeUpdate {
            nodes: vec![
                (ROOT_ID, {
                    let mut node = Node::new(Role::Window);
                    node.set_children(vec![CONTAINER_ID]);
                    node
                }),
                (CONTAINER_ID, {
                    let mut node = Node::new(Role::GenericContainer);
                    node.set_hidden();
                    node.push_child(LEAF_ID);
                    node
                }),
                (LEAF_ID, {
                    let mut node = Node::new(Role::Button);
                    node.set_label("OK");
                    node
                }),
            ],
            tree: Some(Tree::new(ROOT_ID)),
            tree_id: TreeId::ROOT,
            focus: ROOT_ID,
        };
        let tree = crate::Tree::new(update, false);
        assert!(tree.state().node_by_id(nid(LEAF_ID)).unwrap().is_hidden());
    }

    mod node_id {
        use super::NodeId as LocalNodeId;
        use crate::node::NodeId;
        use crate::tree::TreeIndex;

        #[test]
        fn new_and_to_components_round_trip() {
            let node_id = LocalNodeId(42);
            let tree_index = TreeIndex(7);
            let id = NodeId::new(node_id, tree_index);
            let (extracted_node_id, extracted_tree_index) = id.to_components();
            assert_eq!(node_id, extracted_node_id);
            assert_eq!(tree_index, extracted_tree_index);
        }

        #[test]
        fn with_same_tree_preserves_tree_index() {
            let original_node_id = LocalNodeId(100);
            let tree_index = TreeIndex(5);
            let id = NodeId::new(original_node_id, tree_index);

            let new_node_id = LocalNodeId(200);
            let new_id = id.with_same_tree(new_node_id);

            let (extracted_node_id, extracted_tree_index) = new_id.to_components();
            assert_eq!(new_node_id, extracted_node_id);
            assert_eq!(tree_index, extracted_tree_index);
        }

        #[test]
        fn into_u128() {
            let node_id = LocalNodeId(12345);
            let tree_index = TreeIndex(67);
            let id = NodeId::new(node_id, tree_index);
            let (extracted_node_id, extracted_tree_index) = id.to_components();
            assert_eq!(node_id, extracted_node_id);
            assert_eq!(tree_index, extracted_tree_index);
        }

        #[test]
        fn equality() {
            let id1 = NodeId::new(LocalNodeId(1), TreeIndex(2));
            let id2 = NodeId::new(LocalNodeId(1), TreeIndex(2));
            let id3 = NodeId::new(LocalNodeId(1), TreeIndex(3));
            let id4 = NodeId::new(LocalNodeId(2), TreeIndex(2));

            assert_eq!(id1, id2);
            assert_ne!(id1, id3);
            assert_ne!(id1, id4);
        }
    }

    #[test]
    fn is_focused_when_node_has_focus() {
        const ROOT_ID: NodeId = NodeId(0);
        const BUTTON_ID: NodeId = NodeId(1);

        let update = TreeUpdate {
            nodes: vec![
                (ROOT_ID, {
                    let mut node = Node::new(Role::Window);
                    node.set_children(vec![BUTTON_ID]);
                    node
                }),
                (BUTTON_ID, Node::new(Role::Button)),
            ],
            tree: Some(Tree::new(ROOT_ID)),
            tree_id: TreeId::ROOT,
            focus: BUTTON_ID,
        };
        let tree = crate::Tree::new(update, true);
        assert!(tree
            .state()
            .node_by_id(nid(BUTTON_ID))
            .unwrap()
            .is_focused());
    }

    #[test]
    fn is_focused_when_node_does_not_have_focus() {
        const ROOT_ID: NodeId = NodeId(0);
        const BUTTON_ID: NodeId = NodeId(1);

        let update = TreeUpdate {
            nodes: vec![
                (ROOT_ID, {
                    let mut node = Node::new(Role::Window);
                    node.set_children(vec![BUTTON_ID]);
                    node
                }),
                (BUTTON_ID, Node::new(Role::Button)),
            ],
            tree: Some(Tree::new(ROOT_ID)),
            tree_id: TreeId::ROOT,
            focus: ROOT_ID,
        };
        let tree = crate::Tree::new(update, true);
        assert!(!tree
            .state()
            .node_by_id(nid(BUTTON_ID))
            .unwrap()
            .is_focused());
    }

    #[test]
    fn is_focused_active_descendant_is_focused() {
        const ROOT_ID: NodeId = NodeId(0);
        const LISTBOX_ID: NodeId = NodeId(1);
        const ITEM_ID: NodeId = NodeId(2);

        let update = TreeUpdate {
            nodes: vec![
                (ROOT_ID, {
                    let mut node = Node::new(Role::Window);
                    node.set_children(vec![LISTBOX_ID]);
                    node
                }),
                (LISTBOX_ID, {
                    let mut node = Node::new(Role::ListBox);
                    node.set_children(vec![ITEM_ID]);
                    node.set_active_descendant(ITEM_ID);
                    node
                }),
                (ITEM_ID, Node::new(Role::ListBoxOption)),
            ],
            tree: Some(Tree::new(ROOT_ID)),
            tree_id: TreeId::ROOT,
            focus: LISTBOX_ID,
        };
        let tree = crate::Tree::new(update, true);
        assert!(tree.state().node_by_id(nid(ITEM_ID)).unwrap().is_focused());
    }

    #[test]
    fn is_focused_node_with_active_descendant_is_not_focused() {
        const ROOT_ID: NodeId = NodeId(0);
        const LISTBOX_ID: NodeId = NodeId(1);
        const ITEM_ID: NodeId = NodeId(2);

        let update = TreeUpdate {
            nodes: vec![
                (ROOT_ID, {
                    let mut node = Node::new(Role::Window);
                    node.set_children(vec![LISTBOX_ID]);
                    node
                }),
                (LISTBOX_ID, {
                    let mut node = Node::new(Role::ListBox);
                    node.set_children(vec![ITEM_ID]);
                    node.set_active_descendant(ITEM_ID);
                    node
                }),
                (ITEM_ID, Node::new(Role::ListBoxOption)),
            ],
            tree: Some(Tree::new(ROOT_ID)),
            tree_id: TreeId::ROOT,
            focus: LISTBOX_ID,
        };
        let tree = crate::Tree::new(update, true);
        assert!(!tree
            .state()
            .node_by_id(nid(LISTBOX_ID))
            .unwrap()
            .is_focused());
    }
}
