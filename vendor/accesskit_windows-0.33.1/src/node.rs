// Copyright 2022 The AccessKit Authors. All rights reserved.
// Licensed under the Apache License, Version 2.0 (found in
// the LICENSE-APACHE file) or the MIT license (found in
// the LICENSE-MIT file), at your option.

// Derived from Chromium's accessibility abstraction.
// Copyright 2021 The Chromium Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE.chromium file.

#![allow(non_upper_case_globals)]

use accesskit::{
    Action, ActionData, ActionRequest, AriaCurrent, HasPopup, Live, NodeId as LocalNodeId,
    Orientation, Point, Role, SortDirection, Toggled, TreeId,
};
use accesskit_consumer::{FilterResult, Node, NodeId, Tree, TreeState};
use std::{
    fmt::Write,
    sync::{Arc, Weak, atomic::Ordering},
};
use windows::{
    Win32::{
        Foundation::*,
        System::{Com::*, Variant::*},
        UI::Accessibility::*,
    },
    core::*,
};

use crate::{
    context::Context,
    filters::{filter, filter_with_root_exception},
    text::PlatformRange as PlatformTextRange,
    util::*,
};

const RUNTIME_ID_SIZE: usize = 5;

fn runtime_id_from_node_id(id: NodeId) -> [i32; RUNTIME_ID_SIZE] {
    static_assertions::assert_eq_size!(NodeId, u128);
    let id: u128 = id.into();
    [
        UiaAppendRuntimeId as _,
        ((id >> 96) & 0xFFFFFFFF) as _,
        ((id >> 64) & 0xFFFFFFFF) as _,
        ((id >> 32) & 0xFFFFFFFF) as _,
        (id & 0xFFFFFFFF) as _,
    ]
}

pub(crate) struct NodeWrapper<'a>(pub(crate) &'a Node<'a>);

impl NodeWrapper<'_> {
    fn control_type(&self) -> UIA_CONTROLTYPE_ID {
        let role = self.0.role();
        // TODO: Handle special cases. (#14)
        match role {
            Role::Unknown => UIA_CustomControlTypeId,
            Role::TextRun => UIA_CustomControlTypeId,
            Role::Cell | Role::GridCell => UIA_DataItemControlTypeId,
            Role::Label => UIA_TextControlTypeId,
            Role::Image => UIA_ImageControlTypeId,
            Role::Link => UIA_HyperlinkControlTypeId,
            Role::Row => UIA_DataItemControlTypeId,
            Role::ListItem => UIA_ListItemControlTypeId,
            Role::ListMarker => UIA_GroupControlTypeId,
            Role::TreeItem => UIA_TreeItemControlTypeId,
            Role::ListBoxOption => UIA_ListItemControlTypeId,
            Role::MenuItem => UIA_MenuItemControlTypeId,
            Role::MenuListOption => UIA_ListItemControlTypeId,
            Role::Paragraph => UIA_GroupControlTypeId,
            Role::GenericContainer => UIA_GroupControlTypeId,
            Role::CheckBox => UIA_CheckBoxControlTypeId,
            Role::RadioButton => UIA_RadioButtonControlTypeId,
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
            | Role::UrlInput => UIA_EditControlTypeId,
            Role::Button | Role::DefaultButton => UIA_ButtonControlTypeId,
            Role::Pane => UIA_PaneControlTypeId,
            Role::RowHeader => UIA_DataItemControlTypeId,
            Role::ColumnHeader => UIA_DataItemControlTypeId,
            Role::RowGroup => UIA_GroupControlTypeId,
            Role::List => UIA_ListControlTypeId,
            Role::Table => UIA_TableControlTypeId,
            Role::LayoutTableCell => UIA_DataItemControlTypeId,
            Role::LayoutTableRow => UIA_DataItemControlTypeId,
            Role::LayoutTable => UIA_TableControlTypeId,
            Role::Switch => UIA_ButtonControlTypeId,
            Role::Menu => UIA_MenuControlTypeId,
            Role::Abbr => UIA_TextControlTypeId,
            Role::Alert => UIA_TextControlTypeId,
            Role::AlertDialog => {
                // Documentation suggests the use of UIA_PaneControlTypeId,
                // but Chromium's implementation uses UIA_WindowControlTypeId
                // instead.
                UIA_WindowControlTypeId
            }
            Role::Application => UIA_PaneControlTypeId,
            Role::Article => UIA_GroupControlTypeId,
            Role::Audio => UIA_GroupControlTypeId,
            Role::Banner => UIA_GroupControlTypeId,
            Role::Blockquote => UIA_GroupControlTypeId,
            Role::Canvas => UIA_ImageControlTypeId,
            Role::Caption => UIA_TextControlTypeId,
            Role::Caret => UIA_GroupControlTypeId,
            Role::Code => UIA_TextControlTypeId,
            Role::ColorWell => UIA_ButtonControlTypeId,
            Role::ComboBox | Role::EditableComboBox => UIA_ComboBoxControlTypeId,
            Role::Complementary => UIA_GroupControlTypeId,
            Role::Comment => UIA_GroupControlTypeId,
            Role::ContentDeletion => UIA_GroupControlTypeId,
            Role::ContentInsertion => UIA_GroupControlTypeId,
            Role::ContentInfo => UIA_GroupControlTypeId,
            Role::Definition => UIA_GroupControlTypeId,
            Role::DescriptionList => UIA_ListControlTypeId,
            Role::Details => UIA_GroupControlTypeId,
            Role::Dialog => {
                // Documentation suggests the use of UIA_PaneControlTypeId,
                // but Chromium's implementation uses UIA_WindowControlTypeId
                // instead.
                UIA_WindowControlTypeId
            }
            Role::DisclosureTriangle => UIA_ButtonControlTypeId,
            Role::Document | Role::Terminal => UIA_DocumentControlTypeId,
            Role::EmbeddedObject => UIA_PaneControlTypeId,
            Role::Emphasis => UIA_TextControlTypeId,
            Role::Feed => UIA_GroupControlTypeId,
            Role::FigureCaption => UIA_TextControlTypeId,
            Role::Figure => UIA_GroupControlTypeId,
            Role::Footer => UIA_GroupControlTypeId,
            Role::Form => UIA_GroupControlTypeId,
            Role::Grid => UIA_DataGridControlTypeId,
            Role::Group => UIA_GroupControlTypeId,
            Role::Header => UIA_GroupControlTypeId,
            Role::Heading => UIA_TextControlTypeId,
            Role::Iframe => UIA_DocumentControlTypeId,
            Role::IframePresentational => UIA_GroupControlTypeId,
            Role::ImeCandidate => UIA_PaneControlTypeId,
            Role::Keyboard => UIA_PaneControlTypeId,
            Role::Legend => UIA_TextControlTypeId,
            Role::LineBreak => UIA_TextControlTypeId,
            Role::ListBox => UIA_ListControlTypeId,
            Role::Log => UIA_GroupControlTypeId,
            Role::Main => UIA_GroupControlTypeId,
            Role::Mark => UIA_TextControlTypeId,
            Role::Marquee => UIA_TextControlTypeId,
            Role::Math => UIA_GroupControlTypeId,
            Role::MenuBar => UIA_MenuBarControlTypeId,
            Role::MenuItemCheckBox => UIA_CheckBoxControlTypeId,
            Role::MenuItemRadio => UIA_RadioButtonControlTypeId,
            Role::MenuListPopup => UIA_ListControlTypeId,
            Role::Meter => UIA_ProgressBarControlTypeId,
            Role::Navigation => UIA_GroupControlTypeId,
            Role::Note => UIA_GroupControlTypeId,
            Role::PluginObject => UIA_GroupControlTypeId,
            Role::ProgressIndicator => UIA_ProgressBarControlTypeId,
            Role::RadioGroup => UIA_GroupControlTypeId,
            Role::Region => UIA_GroupControlTypeId,
            Role::RootWebArea => UIA_DocumentControlTypeId,
            Role::Ruby => UIA_GroupControlTypeId,
            Role::RubyAnnotation => {
                // Generally exposed as description on <ruby> (Role::Ruby)
                // element, not as its own object in the tree.
                // However, it's possible to make a RubyAnnotation element
                // show up in the AX tree, for example by adding tabindex="0"
                // to the source <rp> or <rt> element or making the source element
                // the target of an aria-owns. Therefore, browser side needs to
                // gracefully handle it if it actually shows up in the tree.
                UIA_TextControlTypeId
            }
            Role::ScrollBar => UIA_ScrollBarControlTypeId,
            Role::ScrollView => UIA_PaneControlTypeId,
            Role::Search => UIA_GroupControlTypeId,
            Role::Section => UIA_GroupControlTypeId,
            Role::SectionFooter => UIA_GroupControlTypeId,
            Role::SectionHeader => UIA_GroupControlTypeId,
            Role::Slider => UIA_SliderControlTypeId,
            Role::SpinButton => UIA_SpinnerControlTypeId,
            Role::Splitter => UIA_SeparatorControlTypeId,
            Role::Status => UIA_StatusBarControlTypeId,
            Role::Strong => UIA_TextControlTypeId,
            Role::Suggestion => UIA_GroupControlTypeId,
            Role::SvgRoot => UIA_ImageControlTypeId,
            Role::Tab => UIA_TabItemControlTypeId,
            Role::TabList => UIA_TabControlTypeId,
            Role::TabPanel => UIA_PaneControlTypeId,
            Role::Term => UIA_ListItemControlTypeId,
            Role::Time => UIA_TextControlTypeId,
            Role::Timer => UIA_PaneControlTypeId,
            Role::TitleBar => UIA_PaneControlTypeId,
            Role::Toolbar => UIA_ToolBarControlTypeId,
            Role::Tooltip => UIA_ToolTipControlTypeId,
            Role::Tree => UIA_TreeControlTypeId,
            Role::TreeGrid => UIA_DataGridControlTypeId,
            Role::Video => UIA_GroupControlTypeId,
            Role::WebView => UIA_DocumentControlTypeId,
            Role::Window => {
                // TODO: determine whether to use Window or Pane.
                // It may be good to use Pane for nested windows,
                // as Chromium does. (#14)
                UIA_WindowControlTypeId
            }
            Role::PdfActionableHighlight => UIA_CustomControlTypeId,
            Role::PdfRoot => UIA_DocumentControlTypeId,
            Role::GraphicsDocument => UIA_DocumentControlTypeId,
            Role::GraphicsObject => UIA_PaneControlTypeId,
            Role::GraphicsSymbol => UIA_ImageControlTypeId,
            Role::DocAbstract => UIA_GroupControlTypeId,
            Role::DocAcknowledgements => UIA_GroupControlTypeId,
            Role::DocAfterword => UIA_GroupControlTypeId,
            Role::DocAppendix => UIA_GroupControlTypeId,
            Role::DocBackLink => UIA_HyperlinkControlTypeId,
            Role::DocBiblioEntry => UIA_ListItemControlTypeId,
            Role::DocBibliography => UIA_GroupControlTypeId,
            Role::DocBiblioRef => UIA_HyperlinkControlTypeId,
            Role::DocChapter => UIA_GroupControlTypeId,
            Role::DocColophon => UIA_GroupControlTypeId,
            Role::DocConclusion => UIA_GroupControlTypeId,
            Role::DocCover => UIA_ImageControlTypeId,
            Role::DocCredit => UIA_GroupControlTypeId,
            Role::DocCredits => UIA_GroupControlTypeId,
            Role::DocDedication => UIA_GroupControlTypeId,
            Role::DocEndnote => UIA_ListItemControlTypeId,
            Role::DocEndnotes => UIA_GroupControlTypeId,
            Role::DocEpigraph => UIA_GroupControlTypeId,
            Role::DocEpilogue => UIA_GroupControlTypeId,
            Role::DocErrata => UIA_GroupControlTypeId,
            Role::DocExample => UIA_GroupControlTypeId,
            Role::DocFootnote => UIA_ListItemControlTypeId,
            Role::DocForeword => UIA_GroupControlTypeId,
            Role::DocGlossary => UIA_GroupControlTypeId,
            Role::DocGlossRef => UIA_HyperlinkControlTypeId,
            Role::DocIndex => UIA_GroupControlTypeId,
            Role::DocIntroduction => UIA_GroupControlTypeId,
            Role::DocNoteRef => UIA_HyperlinkControlTypeId,
            Role::DocNotice => UIA_GroupControlTypeId,
            Role::DocPageBreak => UIA_SeparatorControlTypeId,
            Role::DocPageFooter => UIA_GroupControlTypeId,
            Role::DocPageHeader => UIA_GroupControlTypeId,
            Role::DocPageList => UIA_GroupControlTypeId,
            Role::DocPart => UIA_GroupControlTypeId,
            Role::DocPreface => UIA_GroupControlTypeId,
            Role::DocPrologue => UIA_GroupControlTypeId,
            Role::DocPullquote => UIA_GroupControlTypeId,
            Role::DocQna => UIA_GroupControlTypeId,
            Role::DocSubtitle => UIA_GroupControlTypeId,
            Role::DocTip => UIA_GroupControlTypeId,
            Role::DocToc => UIA_GroupControlTypeId,
            Role::ListGrid => UIA_DataGridControlTypeId,
        }
    }

    fn localized_control_type(&self) -> Option<&str> {
        self.0.role_description()
    }

    fn aria_role(&self) -> Option<&str> {
        match self.0.role() {
            Role::Alert => Some("alert"),
            Role::AlertDialog => Some("alertdialog"),
            Role::Application => Some("application"),
            Role::Article => Some("article"),
            Role::Banner | Role::Header => Some("banner"),
            Role::Button | Role::DefaultButton => Some("button"),
            Role::Blockquote => Some("blockquote"),
            Role::Caption | Role::FigureCaption => Some("caption"),
            Role::Cell => Some("cell"),
            Role::CheckBox => Some("checkbox"),
            Role::Code => Some("code"),
            Role::ColumnHeader => Some("columnheader"),
            Role::ComboBox | Role::EditableComboBox => Some("combobox"),
            Role::Comment => Some("comment"),
            Role::Complementary => Some("complementary"),
            Role::ContentInfo | Role::Footer => Some("contentinfo"),
            Role::Definition => Some("definition"),
            Role::ContentDeletion => Some("deletion"),
            Role::Dialog => Some("dialog"),
            Role::Document
            | Role::Iframe
            | Role::WebView
            | Role::RootWebArea
            | Role::Terminal
            | Role::PdfRoot => Some("document"),
            Role::Emphasis => Some("emphasis"),
            Role::Feed => Some("feed"),
            Role::Figure => Some("figure"),
            Role::Form => Some("form"),
            Role::GenericContainer => Some("generic"),
            Role::GraphicsDocument => Some("graphics-document"),
            Role::GraphicsObject => Some("graphics-object"),
            Role::GraphicsSymbol => Some("graphics-symbol"),
            Role::Grid | Role::ListGrid => Some("grid"),
            Role::GridCell => Some("gridcell"),
            Role::Group
            | Role::Details
            | Role::IframePresentational
            | Role::TitleBar
            | Role::LayoutTable
            | Role::LayoutTableCell
            | Role::LayoutTableRow
            | Role::Audio
            | Role::Video
            | Role::ListMarker
            | Role::EmbeddedObject
            | Role::ImeCandidate => Some("group"),
            Role::Heading => Some("heading"),
            Role::Image | Role::Canvas => Some("img"),
            Role::ContentInsertion => Some("insertion"),
            Role::Link => Some("link"),
            Role::List | Role::DescriptionList | Role::MenuListPopup => Some("list"),
            Role::ListBox => Some("listbox"),
            Role::ListItem => Some("listitem"),
            Role::Log => Some("log"),
            Role::Main => Some("main"),
            Role::Mark => Some("marker"),
            Role::Marquee => Some("marquee"),
            Role::Math => Some("math"),
            Role::Menu => Some("menu"),
            Role::MenuBar => Some("menubar"),
            Role::MenuItem => Some("menuitem"),
            Role::MenuItemCheckBox => Some("menuitemcheckbox"),
            Role::MenuItemRadio => Some("menuitemradio"),
            Role::Meter => Some("meter"),
            Role::Navigation => Some("navigation"),
            Role::Note => Some("note"),
            Role::ListBoxOption | Role::MenuListOption => Some("option"),
            Role::Paragraph => Some("paragraph"),
            Role::ProgressIndicator => Some("progressbar"),
            Role::RadioButton => Some("radio"),
            Role::RadioGroup => Some("radiogroup"),
            Role::Region
            | Role::Pane
            | Role::Window
            | Role::Keyboard
            | Role::Unknown
            | Role::ScrollView
            | Role::Caret => Some("region"),
            Role::Row => Some("row"),
            Role::RowGroup => Some("rowgroup"),
            Role::RowHeader => Some("rowheader"),
            Role::ScrollBar => Some("scrollbar"),
            Role::Search => Some("search"),
            Role::SearchInput => Some("searchbox"),
            Role::SectionFooter => Some("sectionfooter"),
            Role::SectionHeader => Some("sectionheader"),
            Role::Splitter => Some("separator"),
            Role::Slider => Some("slider"),
            Role::SpinButton => Some("spinbutton"),
            Role::Status => Some("status"),
            Role::Strong => Some("strong"),
            // subscript
            Role::Suggestion => Some("suggestion"),
            // superscript
            Role::Switch => Some("switch"),
            Role::Tab => Some("tab"),
            Role::Table => Some("table"),
            Role::TabList => Some("tablist"),
            Role::TabPanel => Some("tabpanel"),
            Role::Term => Some("term"),
            Role::TextInput
            | Role::MultilineTextInput
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
            | Role::ColorWell => Some("textbox"),
            Role::Time => Some("time"),
            Role::Timer => Some("timer"),
            Role::Toolbar => Some("toolbar"),
            Role::Tooltip => Some("tooltip"),
            Role::Tree => Some("tree"),
            Role::TreeGrid => Some("treegrid"),
            Role::TreeItem => Some("treeitem"),
            _ => {
                // TODO: Expose more ARIA roles.
                None
            }
        }
    }

    pub(crate) fn name(&self) -> Option<WideString> {
        let mut result = WideString::default();
        if self.0.label_comes_from_value() {
            self.0.write_value(&mut result)
        } else {
            self.0.write_label(&mut result)
        }
        .unwrap()
        .then_some(result)
    }

    fn description(&self) -> Option<String> {
        self.0.description()
    }

    fn culture(&self) -> Option<LocaleName<'_>> {
        self.0.language().map(LocaleName)
    }

    fn placeholder(&self) -> Option<&str> {
        self.0.placeholder()
    }

    fn is_content_element(&self) -> bool {
        filter(self.0) == FilterResult::Include
    }

    fn aria_properties(&self) -> Option<WideString> {
        let mut result = WideString::default();
        let mut properties = AriaProperties::new(&mut result);

        // TODO: Atomic, and busy flags should include false when explicitly set to that
        if self.0.is_live_atomic() {
            properties.write_bool_property("atomic", true).unwrap();
        }

        if let Some(label) = self.0.braille_label() {
            properties.write_property("braillelabel", label).unwrap();
        }

        if let Some(description) = self.0.braille_role_description() {
            properties
                .write_property("brailleroledescription", description)
                .unwrap();
        }

        if self.0.is_busy() {
            properties.write_bool_property("busy", true).unwrap();
        }

        if let Some(colindextext) = self.0.column_index_text() {
            properties
                .write_property("colindextext", colindextext)
                .unwrap();
        }

        if let Some(current) = self.0.aria_current() {
            if current != AriaCurrent::False {
                properties
                    .write_property(
                        "current",
                        match current {
                            AriaCurrent::True => "true",
                            AriaCurrent::Page => "page",
                            AriaCurrent::Step => "step",
                            AriaCurrent::Location => "location",
                            AriaCurrent::Date => "date",
                            AriaCurrent::Time => "time",
                            AriaCurrent::False => unreachable!(),
                        },
                    )
                    .unwrap();
            }
        }

        if let Some(has_popup) = self.0.has_popup() {
            properties
                .write_property(
                    "haspopup",
                    match has_popup {
                        HasPopup::Menu => "menu",
                        HasPopup::Listbox => "listbox",
                        HasPopup::Tree => "tree",
                        HasPopup::Grid => "grid",
                        HasPopup::Dialog => "dialog",
                    },
                )
                .unwrap();
        }

        if let Some(level) = self.0.level() {
            properties.write_usize_property("level", level).unwrap();
        }

        if self.0.is_multiline() {
            properties.write_bool_property("multiline", true).unwrap();
        }

        if let Some(rowindextext) = self.0.row_index_text() {
            properties
                .write_property("rowindextext", rowindextext)
                .unwrap();
        }

        if let Some(sort_direction) = self.0.sort_direction() {
            properties
                .write_property(
                    "sort",
                    match sort_direction {
                        SortDirection::Ascending => "ascending",
                        SortDirection::Descending => "descending",
                        SortDirection::Other => "other",
                    },
                )
                .unwrap();
        }

        if properties.has_properties() {
            Some(result)
        } else {
            None
        }
    }

    fn is_enabled(&self) -> bool {
        !self.0.is_disabled()
    }

    fn is_focusable(&self) -> bool {
        self.0.is_focusable(&filter)
    }

    fn is_focused(&self) -> bool {
        self.0.is_focused()
    }

    fn live_setting(&self) -> LiveSetting {
        let live = self.0.live();
        match live {
            Live::Off => Off,
            Live::Polite => Polite,
            Live::Assertive => Assertive,
        }
    }

    fn automation_id(&self) -> Option<&str> {
        self.0.author_id()
    }

    fn class_name(&self) -> Option<&str> {
        self.0.class_name()
    }

    fn orientation(&self) -> OrientationType {
        match self.0.orientation() {
            Some(Orientation::Horizontal) => OrientationType_Horizontal,
            Some(Orientation::Vertical) => OrientationType_Vertical,
            None => OrientationType_None,
        }
    }

    fn is_toggle_pattern_supported(&self) -> bool {
        self.0.toggled().is_some() && !self.is_selection_item_pattern_supported()
    }

    fn toggle_state(&self) -> ToggleState {
        match self.0.toggled().unwrap() {
            Toggled::False => ToggleState_Off,
            Toggled::True => ToggleState_On,
            Toggled::Mixed => ToggleState_Indeterminate,
        }
    }

    fn is_invoke_pattern_supported(&self) -> bool {
        self.0.is_invocable(&filter)
    }

    fn is_value_pattern_supported(&self) -> bool {
        if self.0.supports_url() {
            return true;
        }
        self.0.has_value() && !self.0.label_comes_from_value()
    }

    fn is_range_value_pattern_supported(&self) -> bool {
        self.0.numeric_value().is_some()
    }

    fn value(&self) -> WideString {
        if let Some(url) = self.0.supports_url().then(|| self.0.url()).flatten() {
            let mut result = WideString::default();
            result.write_str(url).unwrap();
            return result;
        }
        let mut result = WideString::default();
        self.0.write_value(&mut result).unwrap();
        result
    }

    fn is_read_only(&self) -> bool {
        self.0.is_read_only()
    }

    fn numeric_value(&self) -> f64 {
        self.0.numeric_value().unwrap()
    }

    fn min_numeric_value(&self) -> f64 {
        self.0.min_numeric_value().unwrap_or(0.0)
    }

    fn max_numeric_value(&self) -> f64 {
        self.0.max_numeric_value().unwrap_or(0.0)
    }

    fn numeric_value_step(&self) -> f64 {
        self.0.numeric_value_step().unwrap_or(0.0)
    }

    fn numeric_value_jump(&self) -> f64 {
        self.0
            .numeric_value_jump()
            .unwrap_or_else(|| self.numeric_value_step())
    }

    fn is_required(&self) -> bool {
        self.0.is_required()
    }

    fn is_scroll_item_pattern_supported(&self) -> bool {
        self.0.supports_action(Action::ScrollIntoView, &filter)
    }

    pub(crate) fn is_selection_item_pattern_supported(&self) -> bool {
        match self.0.role() {
            // TODO: tables (#29)
            // https://www.w3.org/TR/core-aam-1.1/#mapping_state-property_table
            // SelectionItem.IsSelected is exposed when aria-checked is True or
            // False, for 'radio' and 'menuitemradio' roles.
            Role::RadioButton | Role::MenuItemRadio => {
                matches!(self.0.toggled(), Some(Toggled::True | Toggled::False))
            }
            // https://www.w3.org/TR/wai-aria-1.1/#aria-selected
            // SelectionItem.IsSelected is exposed when aria-select is True or False.
            Role::ListBoxOption
            | Role::ListItem
            | Role::MenuListOption
            | Role::Tab
            | Role::TreeItem => self.0.is_selected().is_some(),
            Role::GridCell => true,
            _ => false,
        }
    }

    pub(crate) fn is_selected(&self) -> bool {
        match self.0.role() {
            // https://www.w3.org/TR/core-aam-1.1/#mapping_state-property_table
            // SelectionItem.IsSelected is set according to the True or False
            // value of aria-checked for 'radio' and 'menuitemradio' roles.
            Role::RadioButton | Role::MenuItemRadio => self.0.toggled() == Some(Toggled::True),
            // https://www.w3.org/TR/wai-aria-1.1/#aria-selected
            // SelectionItem.IsSelected is set according to the True or False
            // value of aria-selected.
            _ => self.0.is_selected().unwrap_or(false),
        }
    }

    fn position_in_set(&self) -> Option<i32> {
        self.0
            .position_in_set()
            .and_then(|p| p.try_into().ok())
            .map(|p: i32| p + 1)
    }

    fn size_of_set(&self) -> Option<i32> {
        self.0
            .size_of_set_from_container(&filter)
            .and_then(|s| s.try_into().ok())
    }

    fn level(&self) -> Option<i32> {
        self.0
            .level()
            .and_then(|level| level.checked_add(1))
            .and_then(|level| level.try_into().ok())
    }

    fn is_selection_pattern_supported(&self) -> bool {
        self.0.is_container_with_selectable_children()
    }

    fn is_multiselectable(&self) -> bool {
        self.0.is_multiselectable()
    }

    fn is_text_pattern_supported(&self) -> bool {
        self.0.supports_text_ranges()
    }

    fn is_expand_collapse_pattern_supported(&self) -> bool {
        self.0.supports_expand_collapse()
    }

    fn expand_collapse_state(&self) -> ExpandCollapseState {
        match self.0.data().is_expanded() {
            Some(true) => ExpandCollapseState_Expanded,
            Some(false) => ExpandCollapseState_Collapsed,
            // TODO: Handle the menu button case. (#27)
            None => ExpandCollapseState_LeafNode,
        }
    }

    fn is_password(&self) -> bool {
        self.0.role() == Role::PasswordInput
    }

    fn is_dialog(&self) -> bool {
        self.0.is_dialog()
    }

    fn is_window_pattern_supported(&self) -> bool {
        self.0.is_dialog()
    }

    fn is_modal(&self) -> bool {
        self.0.is_modal()
    }

    pub(crate) fn enqueue_property_changes(
        &self,
        queue: &mut Vec<QueuedEvent>,
        platform_node: &PlatformNode,
        element: &IRawElementProviderSimple,
        old: &NodeWrapper,
    ) {
        self.enqueue_simple_property_changes(queue, platform_node, element, old);
        self.enqueue_pattern_property_changes(queue, element, old);
        self.enqueue_property_implied_events(queue, element, old);
    }

    fn enqueue_property_implied_events(
        &self,
        queue: &mut Vec<QueuedEvent>,
        element: &IRawElementProviderSimple,
        old: &NodeWrapper,
    ) {
        if self.is_text_pattern_supported()
            && old.is_text_pattern_supported()
            && self.0.raw_text_selection() != old.0.raw_text_selection()
        {
            queue.push(QueuedEvent::Simple {
                element: element.clone(),
                event_id: UIA_Text_TextSelectionChangedEventId,
            });
        }
    }

    fn enqueue_property_change(
        &self,
        queue: &mut Vec<QueuedEvent>,
        element: &IRawElementProviderSimple,
        property_id: UIA_PROPERTY_ID,
        old_value: Variant,
        new_value: Variant,
    ) {
        let old_value: VARIANT = old_value.into();
        let new_value: VARIANT = new_value.into();
        queue.push(QueuedEvent::PropertyChanged {
            element: element.clone(),
            property_id,
            old_value,
            new_value,
        });
    }
}

#[implement(
    IRawElementProviderSimple,
    IRawElementProviderFragment,
    IRawElementProviderFragmentRoot,
    IToggleProvider,
    IInvokeProvider,
    IValueProvider,
    IRangeValueProvider,
    IScrollItemProvider,
    ISelectionItemProvider,
    ISelectionProvider,
    ITextProvider,
    IExpandCollapseProvider,
    IWindowProvider
)]
pub(crate) struct PlatformNode {
    pub(crate) context: Weak<Context>,
    pub(crate) node_id: Option<NodeId>,
}

impl PlatformNode {
    pub(crate) fn new(context: &Arc<Context>, node_id: NodeId) -> Self {
        Self {
            context: Arc::downgrade(context),
            node_id: Some(node_id),
        }
    }

    pub(crate) fn unspecified_root(context: &Arc<Context>) -> Self {
        Self {
            context: Arc::downgrade(context),
            node_id: None,
        }
    }

    fn upgrade_context(&self) -> Result<Arc<Context>> {
        upgrade(&self.context)
    }

    fn with_tree_state_and_context<F, T>(&self, f: F) -> Result<T>
    where
        F: FnOnce(&TreeState, &Context) -> Result<T>,
    {
        self.with_tree_and_context(|tree, context| f(tree.state(), context))
    }

    fn with_tree_and_context<F, T>(&self, f: F) -> Result<T>
    where
        F: FnOnce(&Tree, &Context) -> Result<T>,
    {
        let context = self.upgrade_context()?;
        let tree = context.read_tree();
        f(&tree, &context)
    }

    fn with_tree_state<F, T>(&self, f: F) -> Result<T>
    where
        F: FnOnce(&TreeState) -> Result<T>,
    {
        self.with_tree_state_and_context(|state, _| f(state))
    }

    fn node<'a>(&self, tree: &'a Tree) -> Result<Node<'a>> {
        let state = tree.state();
        if let Some(id) = self.node_id {
            if let Some(node) = state.node_by_id(id) {
                Ok(node)
            } else {
                Err(element_not_available())
            }
        } else {
            Ok(state.root())
        }
    }

    fn node_with_location<'a>(&self, tree: &'a Tree) -> Result<(Node<'a>, LocalNodeId, TreeId)> {
        let node = self.node(tree)?;
        let (local_id, tree_id) = tree
            .state()
            .locate_node(node.id())
            .ok_or_else(element_not_available)?;
        Ok((node, local_id, tree_id))
    }

    fn resolve_with_context<F, T>(&self, f: F) -> Result<T>
    where
        for<'a> F: FnOnce(Node<'a>, &Context) -> Result<T>,
    {
        self.with_tree_and_context(|tree, context| {
            let node = self.node(tree)?;
            f(node, context)
        })
    }

    fn resolve_with_tree_state_and_context<F, T>(&self, f: F) -> Result<T>
    where
        for<'a> F: FnOnce(Node<'a>, &TreeState, &Context) -> Result<T>,
    {
        self.with_tree_and_context(|tree, context| {
            let node = self.node(tree)?;
            f(node, tree.state(), context)
        })
    }

    fn resolve<F, T>(&self, f: F) -> Result<T>
    where
        for<'a> F: FnOnce(Node<'a>) -> Result<T>,
    {
        self.resolve_with_context(|node, _| f(node))
    }

    fn resolve_with_context_for_text_pattern<F, T>(&self, f: F) -> Result<T>
    where
        for<'a> F: FnOnce(Node<'a>, &Context) -> Result<T>,
    {
        self.with_tree_and_context(|tree, context| {
            let node = self.node(tree)?;
            if node.supports_text_ranges() {
                f(node, context)
            } else {
                Err(element_not_available())
            }
        })
    }

    fn resolve_for_text_pattern<F, T>(&self, f: F) -> Result<T>
    where
        for<'a> F: FnOnce(Node<'a>) -> Result<T>,
    {
        self.resolve_with_context_for_text_pattern(|node, _| f(node))
    }

    fn do_complex_action<F>(&self, f: F) -> Result<()>
    where
        for<'a> F: FnOnce(Node<'a>, LocalNodeId, TreeId) -> Result<Option<ActionRequest>>,
    {
        let context = self.upgrade_context()?;
        if context.is_placeholder.load(Ordering::SeqCst) {
            return Err(element_not_enabled());
        }
        let tree = context.read_tree();
        let (node, target_node, target_tree) = self.node_with_location(&tree)?;
        if let Some(request) = f(node, target_node, target_tree)? {
            drop(tree);
            context.do_action(request);
        }
        Ok(())
    }

    fn do_action<F>(&self, f: F) -> Result<()>
    where
        F: FnOnce() -> (Action, Option<ActionData>),
    {
        self.do_complex_action(|node, target_node, target_tree| {
            if node.is_disabled() {
                return Err(element_not_enabled());
            }
            let (action, data) = f();
            Ok(Some(ActionRequest {
                action,
                target_tree,
                target_node,
                data,
            }))
        })
    }

    fn click(&self) -> Result<()> {
        self.do_action(|| (Action::Click, None))
    }

    fn set_expanded(&self, expanded: bool) -> Result<()> {
        self.do_complex_action(|node, target_node, target_tree| {
            if node.is_disabled() {
                return Err(element_not_enabled());
            }
            let Some(current) = node.data().is_expanded() else {
                return Err(invalid_operation());
            };
            if current == expanded {
                return Err(invalid_operation());
            }
            Ok(Some(ActionRequest {
                action: if expanded {
                    Action::Expand
                } else {
                    Action::Collapse
                },
                target_tree,
                target_node,
                data: None,
            }))
        })
    }

    fn set_selected(&self, selected: bool) -> Result<()> {
        self.do_complex_action(|node, target_node, target_tree| {
            if node.is_disabled() {
                return Err(element_not_enabled());
            }
            let wrapper = NodeWrapper(&node);
            if selected == wrapper.is_selected() {
                return Ok(None);
            }
            Ok(Some(ActionRequest {
                action: Action::Click,
                target_tree,
                target_node,
                data: None,
            }))
        })
    }

    fn relative(&self, node_id: NodeId) -> Self {
        Self {
            context: self.context.clone(),
            node_id: Some(node_id),
        }
    }

    fn is_root(&self, state: &TreeState) -> bool {
        self.node_id.is_some_and(|id| id == state.root_id())
    }
}

#[allow(non_snake_case)]
impl IRawElementProviderSimple_Impl for PlatformNode_Impl {
    fn ProviderOptions(&self) -> Result<ProviderOptions> {
        Ok(ProviderOptions_ServerSideProvider)
    }

    fn GetPatternProvider(&self, pattern_id: UIA_PATTERN_ID) -> Result<IUnknown> {
        self.pattern_provider(pattern_id)
    }

    fn GetPropertyValue(&self, property_id: UIA_PROPERTY_ID) -> Result<VARIANT> {
        self.resolve_with_tree_state_and_context(|node, state, context| {
            let wrapper = NodeWrapper(&node);
            let mut result = wrapper.get_property_value(property_id);
            if result.is_empty() {
                if node.is_root() {
                    match property_id {
                        UIA_NamePropertyId => {
                            result = window_title(context.hwnd).into();
                        }
                        UIA_NativeWindowHandlePropertyId => {
                            result = (context.hwnd.0.0 as i32).into();
                        }
                        _ => (),
                    }
                }
                match property_id {
                    UIA_FrameworkIdPropertyId => result = state.toolkit_name().into(),
                    UIA_ProviderDescriptionPropertyId => result = toolkit_description(state).into(),
                    UIA_ControllerForPropertyId => {
                        let controlled: Vec<IUnknown> = node
                            .controls()
                            .filter(|controlled| filter(controlled) == FilterResult::Include)
                            .map(|controlled| self.relative(controlled.id()))
                            .map(IRawElementProviderSimple::from)
                            .filter_map(|controlled| controlled.cast::<IUnknown>().ok())
                            .collect();
                        result = controlled.into();
                    }
                    _ => (),
                }
            }
            Ok(result.into())
        })
    }

    fn HostRawElementProvider(&self) -> Result<IRawElementProviderSimple> {
        self.with_tree_state_and_context(|state, context| {
            if self.is_root(state) {
                unsafe { UiaHostProviderFromHwnd(context.hwnd.0) }
            } else {
                Err(Error::empty())
            }
        })
    }
}

#[allow(non_snake_case)]
impl IRawElementProviderFragment_Impl for PlatformNode_Impl {
    fn Navigate(&self, direction: NavigateDirection) -> Result<IRawElementProviderFragment> {
        self.resolve(|node| {
            let result = match direction {
                NavigateDirection_Parent => node.filtered_parent(&filter_with_root_exception),
                NavigateDirection_NextSibling => node.following_filtered_siblings(&filter).next(),
                NavigateDirection_PreviousSibling => {
                    node.preceding_filtered_siblings(&filter).next()
                }
                NavigateDirection_FirstChild => node.filtered_children(&filter).next(),
                NavigateDirection_LastChild => node.filtered_children(&filter).next_back(),
                _ => None,
            };
            match result {
                Some(result) => Ok(self.relative(result.id()).into()),
                None => Err(Error::empty()),
            }
        })
    }

    fn GetRuntimeId(&self) -> Result<*mut SAFEARRAY> {
        let node_id = if let Some(id) = self.node_id {
            id
        } else {
            // Since this `PlatformNode` isn't associated with a specific
            // node ID, but always uses whatever node is currently the root,
            // we shouldn't return a UIA runtime ID calculated from an
            // AccessKit node ID, as we normally do. Fortunately,
            // UIA doesn't seem to actually call `GetRuntimeId` on the root.
            return Err(not_implemented());
        };
        let runtime_id = runtime_id_from_node_id(node_id);
        Ok(safe_array_from_i32_slice(&runtime_id))
    }

    fn BoundingRectangle(&self) -> Result<UiaRect> {
        self.resolve_with_context(|node, context| {
            let rect = node.bounding_box().map_or(UiaRect::default(), |rect| {
                let client_top_left = context.client_top_left();
                UiaRect {
                    left: rect.x0 + client_top_left.x,
                    top: rect.y0 + client_top_left.y,
                    width: rect.width(),
                    height: rect.height(),
                }
            });
            Ok(rect)
        })
    }

    fn GetEmbeddedFragmentRoots(&self) -> Result<*mut SAFEARRAY> {
        Ok(std::ptr::null_mut())
    }

    fn SetFocus(&self) -> Result<()> {
        self.do_action(|| (Action::Focus, None))
    }

    fn FragmentRoot(&self) -> Result<IRawElementProviderFragmentRoot> {
        self.with_tree_state(|state| {
            if self.is_root(state) {
                Ok(self.to_interface())
            } else {
                let root_id = state.root_id();
                Ok(self.relative(root_id).into())
            }
        })
    }
}

#[allow(non_snake_case)]
impl IRawElementProviderFragmentRoot_Impl for PlatformNode_Impl {
    fn ElementProviderFromPoint(&self, x: f64, y: f64) -> Result<IRawElementProviderFragment> {
        self.resolve_with_context(|node, context| {
            let client_top_left = context.client_top_left();
            let point = Point::new(x - client_top_left.x, y - client_top_left.y);
            let point = node.transform().inverse() * point;
            node.node_at_point(point, &filter).map_or_else(
                || Err(Error::empty()),
                |node| Ok(self.relative(node.id()).into()),
            )
        })
    }

    fn GetFocus(&self) -> Result<IRawElementProviderFragment> {
        self.with_tree_state(|state| {
            if let Some(node) = state.focus() {
                let self_id = if let Some(id) = self.node_id {
                    id
                } else {
                    state.root_id()
                };
                if node.id() != self_id {
                    return Ok(self.relative(node.id()).into());
                }
            }
            Err(Error::empty())
        })
    }
}

macro_rules! properties {
    ($(($id:ident, $m:ident)),+) => {
        impl NodeWrapper<'_> {
            fn get_property_value(&self, property_id: UIA_PROPERTY_ID) -> Variant {
                match property_id {
                    $($id => {
                        self.$m().into()
                    })*
                    _ => Variant::empty()
                }
            }
            fn enqueue_simple_property_changes(
                &self,
                queue: &mut Vec<QueuedEvent>,
                platform_node: &PlatformNode,
                element: &IRawElementProviderSimple,
                old: &NodeWrapper,
            ) {
                $({
                    let old_value = old.$m();
                    let new_value = self.$m();
                    if old_value != new_value {
                        self.enqueue_property_change(
                            queue,
                            element,
                            $id,
                            old_value.into(),
                            new_value.into(),
                        );
                    }
                })*

                let mut old_controls = old.0.controls().filter(|controlled| filter(controlled) == FilterResult::Include);
                let mut new_controls = self.0.controls().filter(|controlled| filter(controlled) == FilterResult::Include);
                let mut are_equal = true;
                let mut controls: Vec<IUnknown> = Vec::new();
                loop {
                    let old_controlled = old_controls.next();
                    let new_controlled = new_controls.next();
                    match (old_controlled, new_controlled) {
                        (Some(a), Some(b)) => {
                            are_equal = are_equal && a.id() == b.id();
                            controls.push(platform_node.relative(b.id()).into());
                        }
                        (None, None) => break,
                        _ => are_equal = false,
                    }
                }
                if !are_equal {
                    self.enqueue_property_change(
                        queue,
                        &element,
                        UIA_ControllerForPropertyId,
                        Variant::empty(),
                        controls.into(),
                    );
                }
            }
        }
    };
}

macro_rules! patterns {
    ($(($pattern_id:ident, $provider_interface:ident, $provider_interface_impl:ident, $is_supported:ident, (
        $(($property_id:ident, $com_getter:ident, $getter:ident, $com_type:ident)),*
    ), (
        $($extra_trait_method:item),*
    ))),+) => {
        impl PlatformNode_Impl {
            fn pattern_provider(&self, pattern_id: UIA_PATTERN_ID) -> Result<IUnknown> {
                self.resolve(|node| {
                    let wrapper = NodeWrapper(&node);
                    match pattern_id {
                        $($pattern_id if wrapper.$is_supported() => {
                            let intermediate: $provider_interface = self.to_interface();
                            return intermediate.cast();
                        })*
                        _ => (),
                    }
                    Err(Error::empty())
                })
            }
        }
        impl NodeWrapper<'_> {
            fn enqueue_pattern_property_changes(
                &self,
                queue: &mut Vec<QueuedEvent>,
                element: &IRawElementProviderSimple,
                old: &NodeWrapper,
            ) {
                $(if self.$is_supported() && old.$is_supported() {
                    $({
                        let old_value = old.$getter();
                        let new_value = self.$getter();
                        if old_value != new_value {
                            self.enqueue_property_change(
                                queue,
                                element,
                                $property_id,
                                old_value.into(),
                                new_value.into(),
                            );
                        }
                    })*
                })*
            }
        }
        $(#[allow(non_snake_case)]
        impl $provider_interface_impl for PlatformNode_Impl {
            $(fn $com_getter(&self) -> Result<$com_type> {
                self.resolve(|node| {
                    let wrapper = NodeWrapper(&node);
                    Ok(wrapper.$getter().into())
                })
            })*
            $($extra_trait_method)*
        })*
    };
}

properties! {
    (UIA_ControlTypePropertyId, control_type),
    (UIA_LocalizedControlTypePropertyId, localized_control_type),
    (UIA_AriaRolePropertyId, aria_role),
    (UIA_NamePropertyId, name),
    (UIA_FullDescriptionPropertyId, description),
    (UIA_CulturePropertyId, culture),
    (UIA_HelpTextPropertyId, placeholder),
    (UIA_IsContentElementPropertyId, is_content_element),
    (UIA_IsControlElementPropertyId, is_content_element),
    (UIA_IsEnabledPropertyId, is_enabled),
    (UIA_IsKeyboardFocusablePropertyId, is_focusable),
    (UIA_HasKeyboardFocusPropertyId, is_focused),
    (UIA_LiveSettingPropertyId, live_setting),
    (UIA_AutomationIdPropertyId, automation_id),
    (UIA_ClassNamePropertyId, class_name),
    (UIA_OrientationPropertyId, orientation),
    (UIA_IsRequiredForFormPropertyId, is_required),
    (UIA_IsPasswordPropertyId, is_password),
    (UIA_LevelPropertyId, level),
    (UIA_PositionInSetPropertyId, position_in_set),
    (UIA_SizeOfSetPropertyId, size_of_set),
    (UIA_AriaPropertiesPropertyId, aria_properties),
    (UIA_IsDialogPropertyId, is_dialog)
}

patterns! {
    (UIA_TogglePatternId, IToggleProvider, IToggleProvider_Impl, is_toggle_pattern_supported, (
        (UIA_ToggleToggleStatePropertyId, ToggleState, toggle_state, ToggleState)
    ), (
        fn Toggle(&self) -> Result<()> {
            self.click()
        }
    )),
    (UIA_InvokePatternId, IInvokeProvider, IInvokeProvider_Impl, is_invoke_pattern_supported, (), (
        fn Invoke(&self) -> Result<()> {
            self.click()
        }
    )),
    (UIA_ValuePatternId, IValueProvider, IValueProvider_Impl, is_value_pattern_supported, (
        (UIA_ValueValuePropertyId, Value, value, BSTR),
        (UIA_ValueIsReadOnlyPropertyId, IsReadOnly, is_read_only, BOOL)
    ), (
        fn SetValue(&self, value: &PCWSTR) -> Result<()> {
            self.do_action(|| {
                let value = unsafe { value.to_string() }.unwrap();
                (Action::SetValue, Some(ActionData::Value(value.into())))
            })
        }
    )),
    (UIA_RangeValuePatternId, IRangeValueProvider, IRangeValueProvider_Impl, is_range_value_pattern_supported, (
        (UIA_RangeValueValuePropertyId, Value, numeric_value, f64),
        (UIA_RangeValueIsReadOnlyPropertyId, IsReadOnly, is_read_only, BOOL),
        (UIA_RangeValueMinimumPropertyId, Minimum, min_numeric_value, f64),
        (UIA_RangeValueMaximumPropertyId, Maximum, max_numeric_value, f64),
        (UIA_RangeValueSmallChangePropertyId, SmallChange, numeric_value_step, f64),
        (UIA_RangeValueLargeChangePropertyId, LargeChange, numeric_value_jump, f64)
    ), (
        fn SetValue(&self, value: f64) -> Result<()> {
            self.do_action(|| {
                (Action::SetValue, Some(ActionData::NumericValue(value)))
            })
        }
    )),
    (UIA_ScrollItemPatternId, IScrollItemProvider, IScrollItemProvider_Impl, is_scroll_item_pattern_supported, (), (
        fn ScrollIntoView(&self) -> Result<()> {
            self.do_complex_action(|_node, target_node, target_tree| {
                Ok(Some(ActionRequest {
                    action: Action::ScrollIntoView,
                    target_tree,
                    target_node,
                    data: None,
                }))
            })
        }
    )),
    (UIA_SelectionItemPatternId, ISelectionItemProvider, ISelectionItemProvider_Impl, is_selection_item_pattern_supported, (), (
        fn IsSelected(&self) -> Result<BOOL> {
            self.resolve(|node| {
                let wrapper = NodeWrapper(&node);
                Ok(wrapper.is_selected().into())
            })
        },

        fn Select(&self) -> Result<()> {
            self.set_selected(true)
        },

        fn AddToSelection(&self) -> Result<()> {
            self.set_selected(true)
        },

        fn RemoveFromSelection(&self) -> Result<()> {
            self.set_selected(false)
        },

        fn SelectionContainer(&self) -> Result<IRawElementProviderSimple> {
            self.resolve(|node| {
                if let Some(container) = node.selection_container(&filter) {
                    Ok(self.relative(container.id()).into())
                } else {
                    Err(E_FAIL.into())
                }
            })
        }
    )),
    (UIA_SelectionPatternId, ISelectionProvider, ISelectionProvider_Impl, is_selection_pattern_supported, (
        (UIA_SelectionCanSelectMultiplePropertyId, CanSelectMultiple, is_multiselectable, BOOL),
        (UIA_SelectionIsSelectionRequiredPropertyId, IsSelectionRequired, is_required, BOOL)
    ), (
        fn GetSelection(&self) -> Result<*mut SAFEARRAY> {
            self.resolve(|node| {
                let selection: Vec<_> = node
                    .items(&filter)
                    .filter(|item| item.is_selected() == Some(true))
                    .map(|item| self.relative(item.id()))
                    .map(IRawElementProviderSimple::from)
                    .filter_map(|item| item.cast::<IUnknown>().ok())
                    .collect();
                Ok(safe_array_from_com_slice(&selection))
            })
        }
    )),
    (UIA_TextPatternId, ITextProvider, ITextProvider_Impl, is_text_pattern_supported, (), (
        fn GetSelection(&self) -> Result<*mut SAFEARRAY> {
            self.resolve_for_text_pattern(|node| {
                if let Some(range) = node.text_selection() {
                    let platform_range: ITextRangeProvider = PlatformTextRange::new(&self.context, range).into();
                    let iunknown: IUnknown = platform_range.cast()?;
                    Ok(safe_array_from_com_slice(&[iunknown]))
                } else {
                    Ok(std::ptr::null_mut())
                }
            })
        },

        fn GetVisibleRanges(&self) -> Result<*mut SAFEARRAY> {
            // TBD: Do we need this? The Quorum GUI toolkit, which is our
            // current point of comparison for text functionality,
            // doesn't implement it.
            Ok(std::ptr::null_mut())
        },

        fn RangeFromChild(&self, _child: Ref<IRawElementProviderSimple>) -> Result<ITextRangeProvider> {
            // We don't support embedded objects in text.
            Err(not_implemented())
        },

        fn RangeFromPoint(&self, point: &UiaPoint) -> Result<ITextRangeProvider> {
            self.resolve_with_context_for_text_pattern(|node, context| {
                let client_top_left = context.client_top_left();
                let point = Point::new(point.x - client_top_left.x, point.y - client_top_left.y);
                let point = node.transform().inverse() * point;
                let pos = node.text_position_at_point(point);
                let range = pos.to_degenerate_range();
                Ok(PlatformTextRange::new(&self.context, range).into())
            })
        },

        fn DocumentRange(&self) -> Result<ITextRangeProvider> {
            self.resolve_for_text_pattern(|node| {
                let range = node.document_range();
                Ok(PlatformTextRange::new(&self.context, range).into())
            })
        },

        fn SupportedTextSelection(&self) -> Result<SupportedTextSelection> {
            self.resolve_for_text_pattern(|node| {
                if node.has_text_selection() {
                    Ok(SupportedTextSelection_Single)
                } else {
                    Ok(SupportedTextSelection_None)
                }
            })
        }
    )),
    (UIA_ExpandCollapsePatternId, IExpandCollapseProvider, IExpandCollapseProvider_Impl, is_expand_collapse_pattern_supported, (
        (UIA_ExpandCollapseExpandCollapseStatePropertyId, ExpandCollapseState, expand_collapse_state, ExpandCollapseState)
    ), (
        fn Expand(&self) -> Result<()> {
            self.set_expanded(true)
        },

        fn Collapse(&self) -> Result<()> {
            self.set_expanded(false)
        }
    )),
    (UIA_WindowPatternId, IWindowProvider, IWindowProvider_Impl, is_window_pattern_supported, (
        (UIA_WindowIsModalPropertyId, IsModal, is_modal, BOOL)
    ), (
        fn SetVisualState(&self, _: WindowVisualState) -> Result<()> {
            Err(invalid_operation())
        },

        fn Close(&self) -> Result<()> {
            Err(not_supported())
        },

        fn WaitForInputIdle(&self, _: i32) -> Result<BOOL> {
            Err(not_supported())
        },

        fn CanMaximize(&self) -> Result<BOOL> {
            Err(not_supported())
        },

        fn CanMinimize(&self) -> Result<BOOL> {
            Err(not_supported())
        },

        fn WindowVisualState(&self) -> Result<WindowVisualState> {
            Err(not_supported())
        },

        fn WindowInteractionState(&self) -> Result<WindowInteractionState> {
            Ok(WindowInteractionState_ReadyForUserInteraction)
        },

        fn IsTopmost(&self) -> Result<BOOL> {
            Err(not_supported())
        }
    ))
}

// Ensures that `PlatformNode` is actually safe to use in the free-threaded
// manner that we advertise via `ProviderOptions`.
#[test]
fn platform_node_impl_send_sync() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<PlatformNode>();
}
