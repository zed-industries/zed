// Copyright 2022 The AccessKit Authors. All rights reserved.
// Licensed under the Apache License, Version 2.0 (found in
// the LICENSE-APACHE file) or the MIT license (found in
// the LICENSE-MIT file), at your option.

// Derived from Chromium's accessibility abstraction.
// Copyright 2017 The Chromium Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE.chromium file.

use accesskit::{
    Action, ActionData, ActionRequest, Affine, Live, NodeId as LocalNodeId, Orientation, Point,
    Rect, Role, Toggled, TreeId,
};
use accesskit_consumer::{FilterResult, Node, NodeId, Tree, TreeState};
use atspi_common::{
    CoordType, Granularity, Interface, InterfaceSet, Layer, Politeness, RelationType,
    Role as AtspiRole, ScrollType, State, StateSet,
};
use std::{
    collections::HashMap,
    hash::{Hash, Hasher},
    iter::FusedIterator,
    sync::{Arc, RwLock, RwLockReadGuard, Weak},
};

use crate::{
    adapter::Adapter,
    context::{AppContext, Context},
    filters::filter,
    text_attributes::ATTRIBUTE_GETTERS,
    util::*,
    Action as AtspiAction, Error, ObjectEvent, Property, Rect as AtspiRect, Result,
};

pub(crate) struct NodeWrapper<'a>(pub(crate) &'a Node<'a>);

impl NodeWrapper<'_> {
    pub(crate) fn name(&self) -> Option<String> {
        if self.0.label_comes_from_value() {
            self.0.value()
        } else {
            self.0.label()
        }
    }

    pub(crate) fn description(&self) -> Option<String> {
        self.0.description()
    }

    pub(crate) fn parent_id(&self) -> Option<NodeId> {
        self.0.parent_id()
    }

    pub(crate) fn id(&self) -> NodeId {
        self.0.id()
    }

    fn filtered_child_ids(
        &self,
    ) -> impl DoubleEndedIterator<Item = NodeId> + FusedIterator<Item = NodeId> + '_ {
        self.0.filtered_children(&filter).map(|child| child.id())
    }

    pub(crate) fn role(&self) -> AtspiRole {
        if self.0.has_role_description() {
            return AtspiRole::Extended;
        }

        match self.0.role() {
            Role::Alert => AtspiRole::Notification,
            Role::AlertDialog => AtspiRole::Alert,
            Role::Comment | Role::Suggestion => AtspiRole::Section,
            // TODO: See how to represent ARIA role="application"
            Role::Application => AtspiRole::Embedded,
            Role::Article => AtspiRole::Article,
            Role::Audio => AtspiRole::Audio,
            Role::Banner | Role::Header => AtspiRole::Landmark,
            Role::Blockquote => AtspiRole::BlockQuote,
            Role::Caret => AtspiRole::Unknown,
            Role::Button => {
                if self.0.toggled().is_some() {
                    AtspiRole::ToggleButton
                } else {
                    AtspiRole::Button
                }
            }
            Role::DefaultButton => AtspiRole::Button,
            Role::Canvas => AtspiRole::Canvas,
            Role::Caption => AtspiRole::Caption,
            Role::Cell | Role::GridCell => AtspiRole::TableCell,
            Role::CheckBox => AtspiRole::CheckBox,
            Role::Switch => AtspiRole::ToggleButton,
            Role::ColorWell => AtspiRole::Button,
            Role::ColumnHeader => AtspiRole::ColumnHeader,
            Role::ComboBox | Role::EditableComboBox => AtspiRole::ComboBox,
            Role::Complementary => AtspiRole::Landmark,
            Role::ContentDeletion => AtspiRole::ContentDeletion,
            Role::ContentInsertion => AtspiRole::ContentInsertion,
            Role::ContentInfo | Role::Footer => AtspiRole::Landmark,
            Role::Definition => AtspiRole::DescriptionValue,
            Role::DescriptionList => AtspiRole::DescriptionList,
            Role::Details => AtspiRole::Panel,
            Role::Dialog => AtspiRole::Dialog,
            Role::DisclosureTriangle => AtspiRole::ToggleButton,
            Role::DocCover => AtspiRole::Image,
            Role::DocBackLink | Role::DocBiblioRef | Role::DocGlossRef | Role::DocNoteRef => {
                AtspiRole::Link
            }
            Role::DocBiblioEntry | Role::DocEndnote => AtspiRole::ListItem,
            Role::DocNotice | Role::DocTip => AtspiRole::Comment,
            Role::DocFootnote => AtspiRole::Footnote,
            Role::DocPageBreak => AtspiRole::Separator,
            Role::DocPageFooter => AtspiRole::Footer,
            Role::DocPageHeader => AtspiRole::Header,
            Role::DocAcknowledgements
            | Role::DocAfterword
            | Role::DocAppendix
            | Role::DocBibliography
            | Role::DocChapter
            | Role::DocConclusion
            | Role::DocCredits
            | Role::DocEndnotes
            | Role::DocEpilogue
            | Role::DocErrata
            | Role::DocForeword
            | Role::DocGlossary
            | Role::DocIndex
            | Role::DocIntroduction
            | Role::DocPageList
            | Role::DocPart
            | Role::DocPreface
            | Role::DocPrologue
            | Role::DocToc => AtspiRole::Landmark,
            Role::DocAbstract
            | Role::DocColophon
            | Role::DocCredit
            | Role::DocDedication
            | Role::DocEpigraph
            | Role::DocExample
            | Role::DocPullquote
            | Role::DocQna => AtspiRole::Section,
            Role::DocSubtitle => AtspiRole::Heading,
            Role::Document => AtspiRole::DocumentFrame,
            Role::EmbeddedObject => AtspiRole::Embedded,
            // TODO: Forms which lack an accessible name are no longer
            // exposed as forms. Forms which have accessible
            // names should be exposed as `AtspiRole::Landmark` according to Core AAM.
            Role::Form => AtspiRole::Form,
            Role::Figure | Role::Feed => AtspiRole::Panel,
            Role::GenericContainer | Role::Ruby => AtspiRole::Section,
            Role::GraphicsDocument => AtspiRole::DocumentFrame,
            Role::GraphicsObject => AtspiRole::Panel,
            Role::GraphicsSymbol => AtspiRole::Image,
            Role::Grid => AtspiRole::Table,
            Role::Group => AtspiRole::Panel,
            Role::Heading => AtspiRole::Heading,
            Role::Iframe | Role::IframePresentational => AtspiRole::InternalFrame,
            // TODO: If there are unignored children, then it should be AtspiRole::ImageMap.
            Role::Image => AtspiRole::Image,
            Role::TextRun => AtspiRole::Static,
            Role::Legend => AtspiRole::Label,
            // Layout table objects are treated the same as `Role::GenericContainer`.
            Role::LayoutTable => AtspiRole::Section,
            Role::LayoutTableCell => AtspiRole::Section,
            Role::LayoutTableRow => AtspiRole::Section,
            // TODO: Having a separate accessible object for line breaks
            // is inconsistent with other implementations.
            Role::LineBreak => AtspiRole::Static,
            Role::Link => AtspiRole::Link,
            Role::List => AtspiRole::List,
            Role::ListBox => AtspiRole::ListBox,
            // TODO: Use `AtspiRole::MenuItem' inside a combo box.
            Role::ListBoxOption => AtspiRole::ListItem,
            Role::ListGrid => AtspiRole::Table,
            Role::ListItem => AtspiRole::ListItem,
            // Regular list markers only expose their alternative text, but do not
            // expose their descendants, and the descendants should be ignored. This
            // is because the alternative text depends on the counter style and can
            // be different from the actual (visual) marker text, and hence,
            // inconsistent with the descendants. We treat a list marker as non-text
            // only if it still has non-ignored descendants, which happens only when =>
            // - The list marker itself is ignored but the descendants are not
            // - Or the list marker contains images
            Role::ListMarker => AtspiRole::Static,
            Role::Log => AtspiRole::Log,
            Role::Main => AtspiRole::Landmark,
            Role::Mark => AtspiRole::Static,
            Role::Math => AtspiRole::Math,
            Role::Marquee => AtspiRole::Marquee,
            Role::Menu | Role::MenuListPopup => AtspiRole::Menu,
            Role::MenuBar => AtspiRole::MenuBar,
            Role::MenuItem | Role::MenuListOption => AtspiRole::MenuItem,
            Role::MenuItemCheckBox => AtspiRole::CheckMenuItem,
            Role::MenuItemRadio => AtspiRole::RadioMenuItem,
            Role::Meter => AtspiRole::LevelBar,
            Role::Navigation => AtspiRole::Landmark,
            Role::Note => AtspiRole::Comment,
            Role::Pane | Role::ScrollView => AtspiRole::Panel,
            Role::Paragraph => AtspiRole::Paragraph,
            Role::PdfActionableHighlight => AtspiRole::Button,
            Role::PdfRoot => AtspiRole::DocumentFrame,
            Role::PluginObject => AtspiRole::Embedded,
            Role::ProgressIndicator => AtspiRole::ProgressBar,
            Role::RadioButton => AtspiRole::RadioButton,
            Role::RadioGroup => AtspiRole::Panel,
            Role::Region => AtspiRole::Landmark,
            Role::RootWebArea => AtspiRole::DocumentWeb,
            Role::Row => AtspiRole::TableRow,
            Role::RowGroup => AtspiRole::Panel,
            Role::RowHeader => AtspiRole::RowHeader,
            // TODO: Generally exposed as description on <ruby> (`Role::Ruby`) element, not
            // as its own object in the tree.
            // However, it's possible to make a `Role::RubyAnnotation` element show up in the
            // tree, for example by adding tabindex="0" to the source <rp> or <rt>
            // element or making the source element the target of an aria-owns.
            // Therefore, we need to gracefully handle it if it actually
            // shows up in the tree.
            Role::RubyAnnotation => AtspiRole::Static,
            Role::Section => AtspiRole::Section,
            Role::SectionFooter => AtspiRole::Footer,
            Role::SectionHeader => AtspiRole::Header,
            Role::ScrollBar => AtspiRole::ScrollBar,
            Role::Search => AtspiRole::Landmark,
            Role::Slider => AtspiRole::Slider,
            Role::SpinButton => AtspiRole::SpinButton,
            Role::Splitter => AtspiRole::Separator,
            Role::Label => AtspiRole::Label,
            Role::Status => AtspiRole::StatusBar,
            Role::SvgRoot => AtspiRole::DocumentFrame,
            Role::Tab => AtspiRole::PageTab,
            Role::Table => AtspiRole::Table,
            Role::TabList => AtspiRole::PageTabList,
            Role::TabPanel => AtspiRole::ScrollPane,
            // TODO: This mapping should also be applied to the dfn
            // element.
            Role::Term => AtspiRole::DescriptionTerm,
            Role::TitleBar => AtspiRole::TitleBar,
            Role::TextInput
            | Role::MultilineTextInput
            | Role::SearchInput
            | Role::EmailInput
            | Role::NumberInput
            | Role::PhoneNumberInput
            | Role::UrlInput => AtspiRole::Entry,
            Role::DateInput
            | Role::DateTimeInput
            | Role::WeekInput
            | Role::MonthInput
            | Role::TimeInput => AtspiRole::DateEditor,
            Role::PasswordInput => AtspiRole::PasswordText,
            Role::Abbr | Role::Code | Role::Emphasis | Role::Strong | Role::Time => {
                AtspiRole::Static
            }
            Role::Timer => AtspiRole::Timer,
            Role::Toolbar => AtspiRole::ToolBar,
            Role::Tooltip => AtspiRole::ToolTip,
            Role::Tree => AtspiRole::Tree,
            Role::TreeItem => AtspiRole::TreeItem,
            Role::TreeGrid => AtspiRole::TreeTable,
            Role::Video => AtspiRole::Video,
            // In AT-SPI, elements with `AtspiRole::Frame` are windows with titles and
            // buttons, while those with `AtspiRole::Window` are windows without those
            // elements.
            Role::Window => AtspiRole::Frame,
            Role::WebView => AtspiRole::Panel,
            Role::FigureCaption => AtspiRole::Caption,
            // TODO: Are there special cases to consider?
            Role::Unknown => AtspiRole::Unknown,
            Role::ImeCandidate | Role::Keyboard => AtspiRole::RedundantObject,
            Role::Terminal => AtspiRole::Terminal,
        }
    }

    fn is_focused(&self) -> bool {
        self.0.is_focused()
    }

    pub(crate) fn state(&self, is_window_focused: bool) -> StateSet {
        let state = self.0;
        let atspi_role = self.role();
        let mut atspi_state = StateSet::empty();
        if is_window_focused
            && ((state.parent_id().is_none() && state.role() == Role::Window)
                || (state.is_dialog()
                    && state.tree_state.active_dialog().map(|d| d.id()) == Some(state.id())))
        {
            atspi_state.insert(State::Active);
        }
        if state.is_text_input() && !state.is_read_only() {
            atspi_state.insert(State::Editable);
        }
        // TODO: Focus and selection.
        if state.is_focusable(&filter) {
            atspi_state.insert(State::Focusable);
        }
        let filter_result = filter(self.0);
        if filter_result == FilterResult::Include {
            atspi_state.insert(State::Visible | State::Showing);
        }
        if state.is_required() {
            atspi_state.insert(State::Required);
        }
        if state.is_multiselectable() {
            atspi_state.insert(State::Multiselectable);
        }
        if let Some(orientation) = state.orientation() {
            atspi_state.insert(if orientation == Orientation::Horizontal {
                State::Horizontal
            } else {
                State::Vertical
            });
        }
        if atspi_role != AtspiRole::ToggleButton && state.toggled().is_some() {
            atspi_state.insert(State::Checkable);
        }
        if state.is_modal() {
            atspi_state.insert(State::Modal);
        }
        if let Some(selected) = state.is_selected() {
            if !state.is_disabled() {
                atspi_state.insert(State::Selectable);
            }
            if selected {
                atspi_state.insert(State::Selected);
            }
        }
        if state.is_text_input() {
            atspi_state.insert(State::SelectableText);
            atspi_state.insert(match state.is_multiline() {
                true => State::MultiLine,
                false => State::SingleLine,
            });
        }

        // Special case for indeterminate progressbar.
        if state.role() == Role::ProgressIndicator && state.numeric_value().is_none() {
            atspi_state.insert(State::Indeterminate);
        }

        // Toggled state
        match state.toggled() {
            Some(Toggled::Mixed) => atspi_state.insert(State::Indeterminate),
            Some(Toggled::True) if atspi_role == AtspiRole::ToggleButton => {
                atspi_state.insert(State::Pressed)
            }
            Some(Toggled::True) => atspi_state.insert(State::Checked),
            _ => {}
        }

        if state.is_read_only_supported() && state.is_read_only_or_disabled() {
            atspi_state.insert(State::ReadOnly);
        } else {
            atspi_state.insert(State::Enabled | State::Sensitive);
        }

        if self.is_focused() {
            atspi_state.insert(State::Focused);
        }

        atspi_state
    }

    fn placeholder(&self) -> Option<&str> {
        self.0.placeholder()
    }

    fn position_in_set(&self) -> Option<String> {
        self.0.position_in_set().map(|p| (p + 1).to_string())
    }

    fn size_of_set(&self) -> Option<String> {
        self.0
            .size_of_set_from_container(&filter)
            .map(|s| s.to_string())
    }

    fn braille_label(&self) -> Option<&str> {
        self.0.braille_label()
    }

    fn braille_role_description(&self) -> Option<&str> {
        self.0.braille_role_description()
    }

    fn attributes(&self) -> HashMap<&'static str, String> {
        let mut attributes = HashMap::new();
        if let Some(placeholder) = self.placeholder() {
            attributes.insert("placeholder-text", placeholder.to_string());
        }
        if let Some(position_in_set) = self.position_in_set() {
            attributes.insert("posinset", position_in_set);
        }
        if let Some(size_of_set) = self.size_of_set() {
            attributes.insert("setsize", size_of_set);
        }
        if let Some(label) = self.braille_label() {
            attributes.insert("braillelabel", label.to_string());
        }
        if let Some(role_description) = self.braille_role_description() {
            attributes.insert("brailleroledescription", role_description.to_string());
        }

        attributes
    }

    fn is_root(&self) -> bool {
        self.0.is_root()
    }

    fn supports_action(&self) -> bool {
        self.0.is_clickable(&filter)
    }

    fn supports_component(&self) -> bool {
        self.0.raw_bounds().is_some() || self.is_root()
    }

    fn supports_hyperlink(&self) -> bool {
        self.0.supports_url()
    }

    fn supports_selection(&self) -> bool {
        self.0.is_container_with_selectable_children()
    }

    fn supports_text(&self) -> bool {
        self.0.supports_text_ranges()
    }

    fn supports_value(&self) -> bool {
        self.current_value().is_some()
    }

    pub(crate) fn interfaces(&self) -> InterfaceSet {
        let mut interfaces = InterfaceSet::new(Interface::Accessible);
        if self.supports_action() {
            interfaces.insert(Interface::Action);
        }
        if self.supports_component() {
            interfaces.insert(Interface::Component);
        }
        if self.supports_hyperlink() {
            interfaces.insert(Interface::Hyperlink);
        }
        if self.supports_selection() {
            interfaces.insert(Interface::Selection);
        }
        if self.supports_text() {
            interfaces.insert(Interface::Text);
        }
        if self.supports_value() {
            interfaces.insert(Interface::Value);
        }
        interfaces
    }

    pub(crate) fn live(&self) -> Politeness {
        let live = self.0.live();
        match live {
            Live::Off => Politeness::None,
            Live::Polite => Politeness::Polite,
            Live::Assertive => Politeness::Assertive,
        }
    }

    fn n_actions(&self) -> i32 {
        if self.0.is_clickable(&filter) {
            1
        } else {
            0
        }
    }

    fn get_action_name(&self, index: i32) -> String {
        if index != 0 {
            return String::new();
        }
        String::from(if self.0.is_clickable(&filter) {
            "click"
        } else {
            ""
        })
    }

    fn raw_bounds_and_transform(&self) -> (Option<Rect>, Affine) {
        let state = self.0;
        (state.raw_bounds(), state.direct_transform())
    }

    fn extents(&self, window_bounds: &WindowBounds, coord_type: CoordType) -> Option<Rect> {
        let mut bounds = self.0.bounding_box();
        if self.is_root() {
            let window_bounds = window_bounds.inner.with_origin(Point::ZERO);
            if !window_bounds.is_empty() {
                if let Some(bounds) = &mut bounds {
                    bounds.x0 = bounds.x0.min(window_bounds.x1);
                    bounds.y0 = bounds.y0.min(window_bounds.y1);
                    bounds.x1 = bounds.x1.min(window_bounds.x1);
                    bounds.y1 = bounds.y1.min(window_bounds.y1);
                } else {
                    bounds = Some(window_bounds);
                }
            }
        }
        bounds.map(|bounds| {
            let new_origin = window_bounds.accesskit_point_to_atspi_point(
                bounds.origin(),
                self.0.filtered_parent(&filter),
                coord_type,
            );
            bounds.with_origin(new_origin)
        })
    }

    fn current_value(&self) -> Option<f64> {
        self.0.numeric_value()
    }

    pub(crate) fn notify_changes(
        &self,
        window_bounds: &WindowBounds,
        adapter: &Adapter,
        old: &NodeWrapper<'_>,
    ) {
        self.notify_state_changes(adapter, old);
        self.notify_property_changes(adapter, old);
        self.notify_bounds_changes(window_bounds, adapter, old);
        self.notify_children_changes(adapter, old);
    }

    fn notify_state_changes(&self, adapter: &Adapter, old: &NodeWrapper<'_>) {
        let old_state = old.state(true);
        let new_state = self.state(true);
        let changed_states = old_state ^ new_state;
        for state in changed_states.iter() {
            if state == State::Focused {
                // This is handled specially in `focus_moved`.
                continue;
            }
            adapter.emit_object_event(
                self.id(),
                ObjectEvent::StateChanged(state, new_state.contains(state)),
            );
        }
    }

    fn notify_property_changes(&self, adapter: &Adapter, old: &NodeWrapper<'_>) {
        let name = self.name();
        if name != old.name() {
            let name = name.unwrap_or_default();
            adapter.emit_object_event(
                self.id(),
                ObjectEvent::PropertyChanged(Property::Name(name.clone())),
            );

            let live = self.live();
            if live != Politeness::None {
                adapter.emit_object_event(self.id(), ObjectEvent::Announcement(name, live));
            }
        }
        let description = self.description();
        if description != old.description() {
            adapter.emit_object_event(
                self.id(),
                ObjectEvent::PropertyChanged(Property::Description(
                    description.unwrap_or_default(),
                )),
            );
        }
        let parent_id = self.parent_id();
        if parent_id != old.parent_id() {
            let parent = self
                .0
                .filtered_parent(&filter)
                .map_or(NodeIdOrRoot::Root, |node| NodeIdOrRoot::Node(node.id()));
            adapter.emit_object_event(
                self.id(),
                ObjectEvent::PropertyChanged(Property::Parent(parent)),
            );
        }
        let role = self.role();
        if role != old.role() {
            adapter.emit_object_event(
                self.id(),
                ObjectEvent::PropertyChanged(Property::Role(role)),
            );
        }
        if let Some(value) = self.current_value() {
            if Some(value) != old.current_value() {
                adapter.emit_object_event(
                    self.id(),
                    ObjectEvent::PropertyChanged(Property::Value(value)),
                );
            }
        }
    }

    fn notify_bounds_changes(
        &self,
        window_bounds: &WindowBounds,
        adapter: &Adapter,
        old: &NodeWrapper<'_>,
    ) {
        if self.raw_bounds_and_transform() != old.raw_bounds_and_transform() {
            if let Some(extents) = self.extents(window_bounds, CoordType::Window) {
                adapter.emit_object_event(self.id(), ObjectEvent::BoundsChanged(extents.into()));
            }
        }
    }

    fn notify_children_changes(&self, adapter: &Adapter, old: &NodeWrapper<'_>) {
        let old_filtered_children = old.filtered_child_ids().collect::<Vec<NodeId>>();
        let new_filtered_children = self.filtered_child_ids().collect::<Vec<NodeId>>();
        for (index, child) in new_filtered_children.iter().enumerate() {
            if !old_filtered_children.contains(child) {
                adapter.emit_object_event(self.id(), ObjectEvent::ChildAdded(index, *child));
            }
        }
        for child in old_filtered_children.into_iter() {
            if !new_filtered_children.contains(&child) {
                adapter.emit_object_event(self.id(), ObjectEvent::ChildRemoved(child));
            }
        }
    }
}

#[derive(Clone)]
pub struct PlatformNode {
    context: Weak<Context>,
    adapter_id: usize,
    id: NodeId,
}

impl PlatformNode {
    pub(crate) fn new(context: &Arc<Context>, adapter_id: usize, id: NodeId) -> Self {
        Self {
            context: Arc::downgrade(context),
            adapter_id,
            id,
        }
    }

    fn from_adapter_root(adapter_id_and_context: &(usize, Arc<Context>)) -> Self {
        let (adapter_id, context) = adapter_id_and_context;
        Self::new(context, *adapter_id, context.read_tree().state().root_id())
    }

    fn upgrade_context(&self) -> Result<Arc<Context>> {
        if let Some(context) = self.context.upgrade() {
            Ok(context)
        } else {
            Err(Error::Defunct)
        }
    }

    fn with_tree_state<F, T>(&self, f: F) -> Result<T>
    where
        F: FnOnce(&TreeState) -> Result<T>,
    {
        let context = self.upgrade_context()?;
        let tree = context.read_tree();
        f(tree.state())
    }

    fn with_tree_and_context<F, T>(&self, f: F) -> Result<T>
    where
        F: FnOnce(&Tree, &Context) -> Result<T>,
    {
        let context = self.upgrade_context()?;
        let tree = context.read_tree();
        f(&tree, &context)
    }

    fn resolve_with_context<F, T>(&self, f: F) -> Result<T>
    where
        for<'a> F: FnOnce(Node<'a>, &'a Tree, &Context) -> Result<T>,
    {
        self.with_tree_and_context(|tree, context| {
            if let Some(node) = tree.state().node_by_id(self.id) {
                f(node, tree, context)
            } else {
                Err(Error::Defunct)
            }
        })
    }

    fn resolve_for_selection_with_context<F, T>(&self, f: F) -> Result<T>
    where
        for<'a> F: FnOnce(Node<'a>, &'a Tree, &Context) -> Result<T>,
    {
        self.resolve_with_context(|node, tree, context| {
            let wrapper = NodeWrapper(&node);
            if wrapper.supports_selection() {
                f(node, tree, context)
            } else {
                Err(Error::UnsupportedInterface)
            }
        })
    }

    fn resolve_for_text_with_context<F, T>(&self, f: F) -> Result<T>
    where
        for<'a> F: FnOnce(Node<'a>, &'a Tree, &Context) -> Result<T>,
    {
        self.resolve_with_context(|node, tree, context| {
            let wrapper = NodeWrapper(&node);
            if wrapper.supports_text() {
                f(node, tree, context)
            } else {
                Err(Error::UnsupportedInterface)
            }
        })
    }

    fn resolve<F, T>(&self, f: F) -> Result<T>
    where
        for<'a> F: FnOnce(Node<'a>) -> Result<T>,
    {
        self.resolve_with_context(|node, _, _| f(node))
    }

    fn resolve_for_selection<F, T>(&self, f: F) -> Result<T>
    where
        for<'a> F: FnOnce(Node<'a>) -> Result<T>,
    {
        self.resolve(|node| {
            let wrapper = NodeWrapper(&node);
            if wrapper.supports_selection() {
                f(node)
            } else {
                Err(Error::UnsupportedInterface)
            }
        })
    }

    fn resolve_for_text<F, T>(&self, f: F) -> Result<T>
    where
        for<'a> F: FnOnce(Node<'a>) -> Result<T>,
    {
        self.resolve_for_text_with_context(|node, _, _| f(node))
    }

    fn do_action_internal<F>(&self, target: NodeId, f: F) -> Result<()>
    where
        F: FnOnce(&TreeState, &Context, LocalNodeId, TreeId) -> ActionRequest,
    {
        let context = self.upgrade_context()?;
        let tree = context.read_tree();
        if let Some((target_node, target_tree)) = tree.state().locate_node(target) {
            let request = f(tree.state(), &context, target_node, target_tree);
            drop(tree);
            context.do_action(request);
            Ok(())
        } else {
            Err(Error::Defunct)
        }
    }

    pub fn name(&self) -> Result<String> {
        self.resolve(|node| {
            let wrapper = NodeWrapper(&node);
            Ok(wrapper.name().unwrap_or_default())
        })
    }

    pub fn description(&self) -> Result<String> {
        self.resolve(|node| {
            let wrapper = NodeWrapper(&node);
            Ok(wrapper.description().unwrap_or_default())
        })
    }

    pub fn relative(&self, id: NodeId) -> Self {
        Self {
            context: self.context.clone(),
            adapter_id: self.adapter_id,
            id,
        }
    }

    pub fn root(&self) -> Result<PlatformRoot> {
        let context = self.upgrade_context()?;
        Ok(PlatformRoot::new(&context.app_context))
    }

    pub fn toolkit_name(&self) -> Result<String> {
        self.with_tree_state(|state| Ok(state.toolkit_name().unwrap_or_default().to_string()))
    }

    pub fn toolkit_version(&self) -> Result<String> {
        self.with_tree_state(|state| Ok(state.toolkit_version().unwrap_or_default().to_string()))
    }

    pub fn parent(&self) -> Result<NodeIdOrRoot> {
        self.resolve(|node| {
            let parent = node
                .filtered_parent(&filter)
                .map_or(NodeIdOrRoot::Root, |node| NodeIdOrRoot::Node(node.id()));
            Ok(parent)
        })
    }

    pub fn child_count(&self) -> Result<i32> {
        self.resolve(|node| {
            i32::try_from(node.filtered_children(&filter).count())
                .map_err(|_| Error::TooManyChildren)
        })
    }

    pub fn adapter_id(&self) -> usize {
        self.adapter_id
    }

    pub fn id(&self) -> NodeId {
        self.id
    }

    pub fn accessible_id(&self) -> Result<String> {
        self.resolve(|node| {
            if let Some(author_id) = node.author_id() {
                Ok(author_id.to_string())
            } else {
                Ok(String::new())
            }
        })
    }

    pub fn child_at_index(&self, index: usize) -> Result<Option<NodeId>> {
        self.resolve(|node| {
            let child = node
                .filtered_children(&filter)
                .nth(index)
                .map(|child| child.id());
            Ok(child)
        })
    }

    pub fn map_children<T, I>(&self, f: impl Fn(NodeId) -> I) -> Result<T>
    where
        T: FromIterator<I>,
    {
        self.resolve(|node| {
            let children = node
                .filtered_children(&filter)
                .map(|child| child.id())
                .map(f)
                .collect();
            Ok(children)
        })
    }

    pub fn index_in_parent(&self) -> Result<i32> {
        self.resolve(|node| {
            i32::try_from(node.preceding_filtered_siblings(&filter).count())
                .map_err(|_| Error::IndexOutOfRange)
        })
    }

    pub fn relation_set<T>(
        &self,
        f: impl Fn(NodeId) -> T,
    ) -> Result<HashMap<RelationType, Vec<T>>> {
        self.resolve(|node| {
            let mut relations = HashMap::new();
            let controls: Vec<_> = node
                .controls()
                .filter(|controlled| filter(controlled) == FilterResult::Include)
                .map(|controlled| controlled.id())
                .map(f)
                .collect();
            if !controls.is_empty() {
                relations.insert(RelationType::ControllerFor, controls);
            }
            Ok(relations)
        })
    }

    pub fn role(&self) -> Result<AtspiRole> {
        self.resolve(|node| {
            let wrapper = NodeWrapper(&node);
            Ok(wrapper.role())
        })
    }

    pub fn localized_role_name(&self) -> Result<String> {
        self.resolve(|node| Ok(node.role_description().unwrap_or_default().to_string()))
    }

    pub fn state(&self) -> StateSet {
        self.resolve_with_context(|node, tree, _| {
            let wrapper = NodeWrapper(&node);
            Ok(wrapper.state(tree.state().focus_id().is_some()))
        })
        .unwrap_or(State::Defunct.into())
    }

    pub fn attributes(&self) -> Result<HashMap<&'static str, String>> {
        self.resolve(|node| {
            let wrapper = NodeWrapper(&node);
            Ok(wrapper.attributes())
        })
    }

    pub fn supports_action(&self) -> Result<bool> {
        self.resolve(|node| {
            let wrapper = NodeWrapper(&node);
            Ok(wrapper.supports_action())
        })
    }

    pub fn supports_component(&self) -> Result<bool> {
        self.resolve(|node| {
            let wrapper = NodeWrapper(&node);
            Ok(wrapper.supports_component())
        })
    }

    pub fn supports_hyperlink(&self) -> Result<bool> {
        self.resolve(|node| {
            let wrapper = NodeWrapper(&node);
            Ok(wrapper.supports_hyperlink())
        })
    }

    pub fn supports_selection(&self) -> Result<bool> {
        self.resolve(|node| {
            let wrapper = NodeWrapper(&node);
            Ok(wrapper.supports_selection())
        })
    }

    pub fn supports_text(&self) -> Result<bool> {
        self.resolve(|node| {
            let wrapper = NodeWrapper(&node);
            Ok(wrapper.supports_text())
        })
    }

    pub fn supports_value(&self) -> Result<bool> {
        self.resolve(|node| {
            let wrapper = NodeWrapper(&node);
            Ok(wrapper.supports_value())
        })
    }

    pub fn interfaces(&self) -> Result<InterfaceSet> {
        self.resolve(|node| {
            let wrapper = NodeWrapper(&node);
            Ok(wrapper.interfaces())
        })
    }

    pub fn n_actions(&self) -> Result<i32> {
        self.resolve(|node| {
            let wrapper = NodeWrapper(&node);
            Ok(wrapper.n_actions())
        })
    }

    pub fn action_name(&self, index: i32) -> Result<String> {
        self.resolve(|node| {
            let wrapper = NodeWrapper(&node);
            Ok(wrapper.get_action_name(index))
        })
    }

    pub fn actions(&self) -> Result<Vec<AtspiAction>> {
        self.resolve(|node| {
            let wrapper = NodeWrapper(&node);
            let n_actions = wrapper.n_actions() as usize;
            let mut actions = Vec::with_capacity(n_actions);
            for i in 0..n_actions {
                actions.push(AtspiAction {
                    localized_name: wrapper.get_action_name(i as i32),
                    description: "".into(),
                    key_binding: "".into(),
                });
            }
            Ok(actions)
        })
    }

    pub fn do_action(&self, index: i32) -> Result<bool> {
        if index != 0 {
            return Ok(false);
        }
        self.do_action_internal(self.id, |_, _, target_node, target_tree| ActionRequest {
            action: Action::Click,
            target_tree,
            target_node,
            data: None,
        })?;
        Ok(true)
    }

    pub fn contains(&self, x: i32, y: i32, coord_type: CoordType) -> Result<bool> {
        self.resolve_with_context(|node, _, context| {
            let window_bounds = context.read_root_window_bounds();
            let wrapper = NodeWrapper(&node);
            if let Some(extents) = wrapper.extents(&window_bounds, coord_type) {
                Ok(extents.contains(Point::new(x.into(), y.into())))
            } else {
                Ok(false)
            }
        })
    }

    pub fn accessible_at_point(
        &self,
        x: i32,
        y: i32,
        coord_type: CoordType,
    ) -> Result<Option<NodeId>> {
        self.resolve_with_context(|node, _, context| {
            let window_bounds = context.read_root_window_bounds();
            let point = window_bounds.atspi_point_to_accesskit_point(
                Point::new(x.into(), y.into()),
                Some(node),
                coord_type,
            );
            let point = node.transform().inverse() * point;
            Ok(node.node_at_point(point, &filter).map(|node| node.id()))
        })
    }

    pub fn extents(&self, coord_type: CoordType) -> Result<AtspiRect> {
        self.resolve_with_context(|node, _, context| {
            let window_bounds = context.read_root_window_bounds();
            let wrapper = NodeWrapper(&node);
            Ok(wrapper
                .extents(&window_bounds, coord_type)
                .map_or(AtspiRect::INVALID, AtspiRect::from))
        })
    }

    pub fn layer(&self) -> Result<Layer> {
        self.resolve(|node| {
            let wrapper = NodeWrapper(&node);
            if wrapper.role() == AtspiRole::Window {
                Ok(Layer::Window)
            } else {
                Ok(Layer::Widget)
            }
        })
    }

    pub fn grab_focus(&self) -> Result<bool> {
        self.do_action_internal(self.id, |_, _, target_node, target_tree| ActionRequest {
            action: Action::Focus,
            target_tree,
            target_node,
            data: None,
        })?;
        Ok(true)
    }

    pub fn scroll_to(&self, scroll_type: ScrollType) -> Result<bool> {
        self.do_action_internal(self.id, |_, _, target_node, target_tree| ActionRequest {
            action: Action::ScrollIntoView,
            target_tree,
            target_node,
            data: atspi_scroll_type_to_scroll_hint(scroll_type).map(ActionData::ScrollHint),
        })?;
        Ok(true)
    }

    pub fn scroll_to_point(&self, coord_type: CoordType, x: i32, y: i32) -> Result<bool> {
        self.resolve_with_context(|node, tree, context| {
            let window_bounds = context.read_root_window_bounds();
            let point = window_bounds.atspi_point_to_accesskit_point(
                Point::new(x.into(), y.into()),
                node.filtered_parent(&filter),
                coord_type,
            );
            let (target_node, target_tree) =
                tree.state().locate_node(self.id).ok_or(Error::Defunct)?;
            context.do_action(ActionRequest {
                action: Action::ScrollToPoint,
                target_tree,
                target_node,
                data: Some(ActionData::ScrollToPoint(point)),
            });
            Ok(())
        })?;
        Ok(true)
    }

    pub fn n_anchors(&self) -> Result<i32> {
        self.resolve(|node| if node.url().is_some() { Ok(1) } else { Ok(0) })
    }

    pub fn hyperlink_start_index(&self) -> Result<i32> {
        self.resolve(|_| {
            // TODO: Support rich text
            Ok(-1)
        })
    }

    pub fn hyperlink_end_index(&self) -> Result<i32> {
        self.resolve(|_| {
            // TODO: Support rich text
            Ok(-1)
        })
    }

    pub fn hyperlink_object(&self, index: i32) -> Result<Option<NodeId>> {
        self.resolve(|_| {
            if index == 0 {
                Ok(Some(self.id))
            } else {
                Ok(None)
            }
        })
    }

    pub fn uri(&self, index: i32) -> Result<String> {
        self.resolve(|node| {
            if index == 0 {
                Ok(node.url().map(|s| s.to_string()).unwrap_or_default())
            } else {
                Ok(String::new())
            }
        })
    }

    pub fn hyperlink_is_valid(&self) -> Result<bool> {
        self.resolve(|node| Ok(node.url().is_some()))
    }

    pub fn n_selected_children(&self) -> Result<i32> {
        self.resolve_for_selection(|node| {
            node.items(filter)
                .filter(|item| item.is_selected() == Some(true))
                .count()
                .try_into()
                .map_err(|_| Error::TooManyChildren)
        })
    }

    pub fn selected_child(&self, selected_child_index: usize) -> Result<Option<NodeId>> {
        self.resolve_for_selection(|node| {
            Ok(node
                .items(filter)
                .filter(|item| item.is_selected() == Some(true))
                .nth(selected_child_index)
                .map(|node| node.id()))
        })
    }

    pub fn select_child(&self, child_index: usize) -> Result<bool> {
        self.resolve_for_selection_with_context(|node, tree, context| {
            if let Some(child) = node.filtered_children(filter).nth(child_index) {
                if let Some(true) = child.is_selected() {
                    Ok(true)
                } else if child.is_selectable() && child.is_clickable(&filter) {
                    let (target_node, target_tree) =
                        tree.state().locate_node(child.id()).ok_or(Error::Defunct)?;
                    context.do_action(ActionRequest {
                        action: Action::Click,
                        target_tree,
                        target_node,
                        data: None,
                    });
                    Ok(true)
                } else {
                    Ok(false)
                }
            } else {
                Err(Error::Defunct)
            }
        })
    }

    pub fn deselect_selected_child(&self, selected_child_index: usize) -> Result<bool> {
        self.resolve_for_selection_with_context(|node, tree, context| {
            if let Some(child) = node
                .items(filter)
                .filter(|c| c.is_selected() == Some(true))
                .nth(selected_child_index)
            {
                if child.is_clickable(&filter) {
                    let (target_node, target_tree) =
                        tree.state().locate_node(child.id()).ok_or(Error::Defunct)?;
                    context.do_action(ActionRequest {
                        action: Action::Click,
                        target_tree,
                        target_node,
                        data: None,
                    });
                    Ok(true)
                } else {
                    Ok(false)
                }
            } else {
                Err(Error::Defunct)
            }
        })
    }

    pub fn is_child_selected(&self, child_index: usize) -> Result<bool> {
        self.resolve_for_selection(|node| {
            node.filtered_children(filter)
                .nth(child_index)
                .map(|child| child.is_item_like() && child.is_selected() == Some(true))
                .ok_or(Error::Defunct)
        })
    }

    pub fn select_all(&self) -> Result<bool> {
        // We don't support selecting all children at once.
        Ok(false)
    }

    pub fn clear_selection(&self) -> Result<bool> {
        // We don't support deselecting all children at once.
        Ok(false)
    }

    pub fn deselect_child(&self, child_index: usize) -> Result<bool> {
        self.resolve_for_selection_with_context(|node, tree, context| {
            if let Some(child) = node.filtered_children(filter).nth(child_index) {
                if let Some(false) = child.is_selected() {
                    Ok(true)
                } else if child.is_selectable() && child.is_clickable(&filter) {
                    let (target_node, target_tree) =
                        tree.state().locate_node(child.id()).ok_or(Error::Defunct)?;
                    context.do_action(ActionRequest {
                        action: Action::Click,
                        target_tree,
                        target_node,
                        data: None,
                    });
                    Ok(true)
                } else {
                    Ok(false)
                }
            } else {
                Err(Error::Defunct)
            }
        })
    }

    pub fn character_count(&self) -> Result<i32> {
        self.resolve_for_text(|node| {
            node.document_range()
                .end()
                .to_global_usv_index()
                .try_into()
                .map_err(|_| Error::TooManyCharacters)
        })
    }

    pub fn caret_offset(&self) -> Result<i32> {
        self.resolve_for_text(|node| {
            node.text_selection_focus().map_or(Ok(-1), |focus| {
                focus
                    .to_global_usv_index()
                    .try_into()
                    .map_err(|_| Error::TooManyCharacters)
            })
        })
    }

    pub fn string_at_offset(
        &self,
        offset: i32,
        granularity: Granularity,
    ) -> Result<(String, i32, i32)> {
        self.resolve_for_text(|node| {
            let range = text_range_from_offset(&node, offset, granularity)?;
            let text = range.text();
            let start = range
                .start()
                .to_global_usv_index()
                .try_into()
                .map_err(|_| Error::TooManyCharacters)?;
            let end = range
                .end()
                .to_global_usv_index()
                .try_into()
                .map_err(|_| Error::TooManyCharacters)?;

            Ok((text, start, end))
        })
    }

    pub fn text(&self, start_offset: i32, end_offset: i32) -> Result<String> {
        self.resolve_for_text(|node| {
            let range = text_range_from_offsets(&node, start_offset, end_offset)
                .ok_or(Error::IndexOutOfRange)?;
            Ok(range.text())
        })
    }

    pub fn set_caret_offset(&self, offset: i32) -> Result<bool> {
        self.resolve_for_text_with_context(|node, tree, context| {
            let offset = text_position_from_offset(&node, offset).ok_or(Error::IndexOutOfRange)?;
            let (target_node, target_tree) =
                tree.state().locate_node(node.id()).ok_or(Error::Defunct)?;
            context.do_action(ActionRequest {
                action: Action::SetTextSelection,
                target_tree,
                target_node,
                data: Some(ActionData::SetTextSelection(
                    offset.to_degenerate_range().to_text_selection(),
                )),
            });
            Ok(true)
        })
    }

    pub fn text_attribute_value(&self, offset: i32, attribute_name: &str) -> Result<String> {
        self.resolve_for_text(|node| {
            let pos = text_position_from_offset(&node, offset).ok_or(Error::IndexOutOfRange)?;
            Ok(ATTRIBUTE_GETTERS
                .get(attribute_name)
                .and_then(|getter| (*getter)(pos.inner_node()))
                .unwrap_or_default())
        })
    }

    pub fn text_attributes(
        &self,
        offset: i32,
    ) -> Result<(HashMap<&'static str, String>, i32, i32)> {
        self.text_attribute_run(offset, false)
    }

    pub fn default_text_attributes(&self) -> Result<HashMap<&'static str, String>> {
        self.resolve_for_text(|node| {
            let mut result = HashMap::with_capacity(ATTRIBUTE_GETTERS.len());
            for (name, getter) in ATTRIBUTE_GETTERS.entries() {
                if let Some(value) = (*getter)(&node) {
                    result.insert(*name, value);
                }
            }
            Ok(result)
        })
    }

    pub fn character_extents(&self, offset: i32, coord_type: CoordType) -> Result<AtspiRect> {
        self.resolve_for_text_with_context(|node, _, context| {
            let range = text_range_from_offset(&node, offset, Granularity::Char)?;
            if let Some(bounds) = range.bounding_boxes().first() {
                let window_bounds = context.read_root_window_bounds();
                let new_origin = window_bounds.accesskit_point_to_atspi_point(
                    bounds.origin(),
                    Some(node),
                    coord_type,
                );
                Ok(bounds.with_origin(new_origin).into())
            } else {
                Ok(AtspiRect::INVALID)
            }
        })
    }

    pub fn offset_at_point(&self, x: i32, y: i32, coord_type: CoordType) -> Result<i32> {
        self.resolve_for_text_with_context(|node, _, context| {
            let window_bounds = context.read_root_window_bounds();
            let point = window_bounds.atspi_point_to_accesskit_point(
                Point::new(x.into(), y.into()),
                Some(node),
                coord_type,
            );
            let point = node.transform().inverse() * point;
            node.text_position_at_point(point)
                .to_global_usv_index()
                .try_into()
                .map_err(|_| Error::TooManyCharacters)
        })
    }

    pub fn n_selections(&self) -> Result<i32> {
        self.resolve_for_text(|node| {
            match node.text_selection().filter(|range| !range.is_degenerate()) {
                Some(_) => Ok(1),
                None => Ok(0),
            }
        })
    }

    pub fn selection(&self, selection_num: i32) -> Result<(i32, i32)> {
        if selection_num != 0 {
            return Ok((-1, -1));
        }

        self.resolve_for_text(|node| {
            node.text_selection()
                .filter(|range| !range.is_degenerate())
                .map_or(Ok((-1, -1)), |range| {
                    let start = range
                        .start()
                        .to_global_usv_index()
                        .try_into()
                        .map_err(|_| Error::TooManyCharacters)?;
                    let end = range
                        .end()
                        .to_global_usv_index()
                        .try_into()
                        .map_err(|_| Error::TooManyCharacters)?;

                    Ok((start, end))
                })
        })
    }

    pub fn add_selection(&self, start_offset: i32, end_offset: i32) -> Result<bool> {
        // We only support one selection.
        self.set_selection(0, start_offset, end_offset)
    }

    pub fn remove_selection(&self, selection_num: i32) -> Result<bool> {
        if selection_num != 0 {
            return Ok(false);
        }

        self.resolve_for_text_with_context(|node, tree, context| {
            // Simply collapse the selection to the position of the caret if a caret is
            // visible, otherwise set the selection to 0.
            let selection_end = node
                .text_selection_focus()
                .unwrap_or_else(|| node.document_range().start());
            let (target_node, target_tree) =
                tree.state().locate_node(node.id()).ok_or(Error::Defunct)?;
            context.do_action(ActionRequest {
                action: Action::SetTextSelection,
                target_tree,
                target_node,
                data: Some(ActionData::SetTextSelection(
                    selection_end.to_degenerate_range().to_text_selection(),
                )),
            });
            Ok(true)
        })
    }

    pub fn set_selection(
        &self,
        selection_num: i32,
        start_offset: i32,
        end_offset: i32,
    ) -> Result<bool> {
        if selection_num != 0 {
            return Ok(false);
        }

        self.resolve_for_text_with_context(|node, tree, context| {
            let range = text_range_from_offsets(&node, start_offset, end_offset)
                .ok_or(Error::IndexOutOfRange)?;
            let (target_node, target_tree) =
                tree.state().locate_node(node.id()).ok_or(Error::Defunct)?;
            context.do_action(ActionRequest {
                action: Action::SetTextSelection,
                target_tree,
                target_node,
                data: Some(ActionData::SetTextSelection(range.to_text_selection())),
            });
            Ok(true)
        })
    }

    pub fn range_extents(
        &self,
        start_offset: i32,
        end_offset: i32,
        coord_type: CoordType,
    ) -> Result<AtspiRect> {
        self.resolve_for_text_with_context(|node, _, context| {
            if let Some(rect) = text_range_bounds_from_offsets(&node, start_offset, end_offset) {
                let window_bounds = context.read_root_window_bounds();
                let new_origin = window_bounds.accesskit_point_to_atspi_point(
                    rect.origin(),
                    Some(node),
                    coord_type,
                );
                Ok(rect.with_origin(new_origin).into())
            } else {
                Ok(AtspiRect::INVALID)
            }
        })
    }

    pub fn text_attribute_run(
        &self,
        offset: i32,
        include_defaults: bool,
    ) -> Result<(HashMap<&'static str, String>, i32, i32)> {
        self.resolve_for_text(|node| {
            let pos = text_position_from_offset(&node, offset).ok_or(Error::IndexOutOfRange)?;
            let mut result = HashMap::with_capacity(ATTRIBUTE_GETTERS.len());
            for (name, getter) in ATTRIBUTE_GETTERS.entries() {
                if let Some(value) = (*getter)(pos.inner_node()) {
                    if !include_defaults {
                        if let Some(default) = (*getter)(&node) {
                            if value == default {
                                continue;
                            }
                        }
                    }
                    result.insert(*name, value);
                }
            }
            let start = if pos.is_format_start() {
                pos
            } else {
                pos.backward_to_format_start()
            };
            let end = pos.forward_to_format_end();
            Ok((
                result,
                start
                    .to_global_usv_index()
                    .try_into()
                    .map_err(|_| Error::TooManyCharacters)?,
                end.to_global_usv_index()
                    .try_into()
                    .map_err(|_| Error::TooManyCharacters)?,
            ))
        })
    }

    pub fn scroll_substring_to(
        &self,
        start_offset: i32,
        end_offset: i32,
        scroll_type: ScrollType,
    ) -> Result<bool> {
        self.resolve_for_text_with_context(|node, tree, context| {
            if let Some(range) = text_range_from_offsets(&node, start_offset, end_offset) {
                let position = if matches!(
                    scroll_type,
                    ScrollType::BottomRight | ScrollType::BottomEdge | ScrollType::RightEdge
                ) {
                    range.end()
                } else {
                    range.start()
                };
                let (target_node, target_tree) = tree
                    .state()
                    .locate_node(position.inner_node().id())
                    .ok_or(Error::Defunct)?;
                context.do_action(ActionRequest {
                    action: Action::ScrollIntoView,
                    target_tree,
                    target_node,
                    data: atspi_scroll_type_to_scroll_hint(scroll_type).map(ActionData::ScrollHint),
                });
                Ok(true)
            } else {
                Ok(false)
            }
        })
    }

    pub fn scroll_substring_to_point(
        &self,
        start_offset: i32,
        end_offset: i32,
        coord_type: CoordType,
        x: i32,
        y: i32,
    ) -> Result<bool> {
        self.resolve_for_text_with_context(|node, tree, context| {
            let window_bounds = context.read_root_window_bounds();
            let target_point = window_bounds.atspi_point_to_accesskit_point(
                Point::new(x.into(), y.into()),
                Some(node),
                coord_type,
            );

            if let Some(rect) = text_range_bounds_from_offsets(&node, start_offset, end_offset) {
                let point = Point::new(target_point.x - rect.x0, target_point.y - rect.y0);
                let (target_node, target_tree) =
                    tree.state().locate_node(node.id()).ok_or(Error::Defunct)?;
                context.do_action(ActionRequest {
                    action: Action::ScrollToPoint,
                    target_tree,
                    target_node,
                    data: Some(ActionData::ScrollToPoint(point)),
                });
                return Ok(true);
            }
            Ok(false)
        })
    }

    pub fn minimum_value(&self) -> Result<f64> {
        self.resolve(|node| Ok(node.min_numeric_value().unwrap_or(f64::MIN)))
    }

    pub fn maximum_value(&self) -> Result<f64> {
        self.resolve(|node| Ok(node.max_numeric_value().unwrap_or(f64::MAX)))
    }

    pub fn minimum_increment(&self) -> Result<f64> {
        self.resolve(|node| Ok(node.numeric_value_step().unwrap_or(0.0)))
    }

    pub fn current_value(&self) -> Result<f64> {
        self.resolve(|node| {
            let wrapper = NodeWrapper(&node);
            Ok(wrapper.current_value().unwrap_or(0.0))
        })
    }

    pub fn set_current_value(&self, value: f64) -> Result<()> {
        self.do_action_internal(self.id, |_, _, target_node, target_tree| ActionRequest {
            action: Action::SetValue,
            target_tree,
            target_node,
            data: Some(ActionData::NumericValue(value)),
        })
    }
}

impl PartialEq for PlatformNode {
    fn eq(&self, other: &Self) -> bool {
        self.adapter_id == other.adapter_id && self.id == other.id
    }
}

impl Eq for PlatformNode {}

impl Hash for PlatformNode {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.adapter_id.hash(state);
        self.id.hash(state);
    }
}

#[derive(Clone)]
pub struct PlatformRoot {
    app_context: Weak<RwLock<AppContext>>,
}

impl PlatformRoot {
    pub fn new(app_context: &Arc<RwLock<AppContext>>) -> Self {
        Self {
            app_context: Arc::downgrade(app_context),
        }
    }

    fn resolve_app_context<F, T>(&self, f: F) -> Result<T>
    where
        for<'a> F: FnOnce(RwLockReadGuard<'a, AppContext>) -> Result<T>,
    {
        let app_context = match self.app_context.upgrade() {
            Some(context) => context,
            None => return Err(Error::Defunct),
        };
        let app_context = app_context.read().unwrap();
        f(app_context)
    }

    pub fn name(&self) -> Result<String> {
        self.resolve_app_context(|context| Ok(context.name.clone().unwrap_or_default()))
    }

    pub fn child_count(&self) -> Result<i32> {
        self.resolve_app_context(|context| {
            i32::try_from(context.adapters.len()).map_err(|_| Error::TooManyChildren)
        })
    }

    pub fn child_at_index(&self, index: usize) -> Result<Option<PlatformNode>> {
        self.resolve_app_context(|context| {
            let child = context
                .adapters
                .get(index)
                .map(PlatformNode::from_adapter_root);
            Ok(child)
        })
    }

    pub fn child_id_at_index(&self, index: usize) -> Result<Option<(usize, NodeId)>> {
        self.resolve_app_context(|context| {
            let child = context
                .adapters
                .get(index)
                .map(|(adapter_id, context)| (*adapter_id, context.read_tree().state().root_id()));
            Ok(child)
        })
    }

    pub fn map_children<T, I>(&self, f: impl Fn(PlatformNode) -> I) -> Result<T>
    where
        T: FromIterator<I>,
    {
        self.resolve_app_context(|context| {
            let children = context
                .adapters
                .iter()
                .map(PlatformNode::from_adapter_root)
                .map(f)
                .collect();
            Ok(children)
        })
    }

    pub fn map_child_ids<T, I>(&self, f: impl Fn((usize, NodeId)) -> I) -> Result<T>
    where
        T: FromIterator<I>,
    {
        self.resolve_app_context(|context| {
            let children = context
                .adapters
                .iter()
                .map(|(adapter_id, context)| (*adapter_id, context.read_tree().state().root_id()))
                .map(f)
                .collect();
            Ok(children)
        })
    }

    pub fn toolkit_name(&self) -> Result<String> {
        self.resolve_app_context(|context| Ok(context.toolkit_name.clone().unwrap_or_default()))
    }

    pub fn toolkit_version(&self) -> Result<String> {
        self.resolve_app_context(|context| Ok(context.toolkit_version.clone().unwrap_or_default()))
    }

    pub fn id(&self) -> Result<i32> {
        self.resolve_app_context(|context| Ok(context.id.unwrap_or(-1)))
    }

    pub fn set_id(&mut self, id: i32) -> Result<()> {
        let app_context = match self.app_context.upgrade() {
            Some(context) => context,
            None => return Err(Error::Defunct),
        };
        let mut app_context = app_context.write().unwrap();
        app_context.id = Some(id);
        Ok(())
    }
}

impl PartialEq for PlatformRoot {
    fn eq(&self, other: &Self) -> bool {
        self.app_context.ptr_eq(&other.app_context)
    }
}

impl Hash for PlatformRoot {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.app_context.as_ptr().hash(state);
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NodeIdOrRoot {
    Node(NodeId),
    Root,
}
