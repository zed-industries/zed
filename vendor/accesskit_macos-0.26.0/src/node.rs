// Copyright 2022 The AccessKit Authors. All rights reserved.
// Licensed under the Apache License, Version 2.0 (found in
// the LICENSE-APACHE file) or the MIT license (found in
// the LICENSE-MIT file), at your option.

// Derived from Chromium's accessibility abstraction.
// Copyright 2018 The Chromium Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE.chromium file.

#![allow(non_upper_case_globals)]

use accesskit::{
    Action, ActionData, ActionRequest, Orientation, Role, TextAlign, TextSelection, Toggled,
};
use accesskit_consumer::{FilterResult, Node, NodeId, Tree};
use objc2::{
    declare_class, msg_send_id,
    mutability::InteriorMutable,
    rc::Id,
    runtime::{AnyObject, Sel},
    sel, ClassType, DeclaredClass,
};
use objc2_app_kit::*;
use objc2_foundation::{
    ns_string, NSArray, NSAttributedString, NSCopying, NSInteger, NSMutableAttributedString,
    NSMutableDictionary, NSNumber, NSObject, NSObjectProtocol, NSPoint, NSRange, NSRect, NSString,
    NSURL,
};
use std::rc::{Rc, Weak};

use crate::{context::Context, filters::filter, util::*};

const SCROLL_TO_VISIBLE_ACTION: &str = "AXScrollToVisible";

fn ns_role(node: &Node) -> &'static NSAccessibilityRole {
    let role = node.role();
    // TODO: Handle special cases.
    unsafe {
        match role {
            Role::Unknown => NSAccessibilityUnknownRole,
            Role::TextRun => NSAccessibilityUnknownRole,
            Role::Cell | Role::GridCell => NSAccessibilityCellRole,
            Role::Label => NSAccessibilityStaticTextRole,
            Role::Image => NSAccessibilityImageRole,
            Role::Link => NSAccessibilityLinkRole,
            Role::Row => NSAccessibilityRowRole,
            Role::ListItem => NSAccessibilityGroupRole,
            Role::ListMarker => ns_string!("AXListMarker"),
            Role::TreeItem => NSAccessibilityRowRole,
            Role::ListBoxOption => NSAccessibilityStaticTextRole,
            Role::MenuItem => NSAccessibilityMenuItemRole,
            Role::MenuListOption => NSAccessibilityMenuItemRole,
            Role::Paragraph => NSAccessibilityGroupRole,
            Role::GenericContainer => NSAccessibilityUnknownRole,
            Role::CheckBox => NSAccessibilityCheckBoxRole,
            Role::RadioButton => NSAccessibilityRadioButtonRole,
            Role::TextInput
            | Role::SearchInput
            | Role::EmailInput
            | Role::NumberInput
            | Role::PasswordInput
            | Role::PhoneNumberInput
            | Role::UrlInput => NSAccessibilityTextFieldRole,
            Role::Button => {
                if node.toggled().is_some() {
                    NSAccessibilityCheckBoxRole
                } else {
                    NSAccessibilityButtonRole
                }
            }
            Role::DefaultButton => NSAccessibilityButtonRole,
            Role::Pane => NSAccessibilityUnknownRole,
            Role::RowHeader => NSAccessibilityCellRole,
            Role::ColumnHeader => NSAccessibilityCellRole,
            Role::RowGroup => NSAccessibilityGroupRole,
            Role::List => NSAccessibilityListRole,
            Role::Table => NSAccessibilityTableRole,
            Role::LayoutTableCell => NSAccessibilityGroupRole,
            Role::LayoutTableRow => NSAccessibilityGroupRole,
            Role::LayoutTable => NSAccessibilityGroupRole,
            Role::Switch => NSAccessibilityCheckBoxRole,
            Role::Menu => NSAccessibilityMenuRole,
            Role::MultilineTextInput => NSAccessibilityTextAreaRole,
            Role::DateInput | Role::DateTimeInput | Role::WeekInput | Role::MonthInput => {
                ns_string!("AXDateField")
            }
            Role::TimeInput => ns_string!("AXTimeField"),
            Role::Abbr => NSAccessibilityGroupRole,
            Role::Alert => NSAccessibilityGroupRole,
            Role::AlertDialog => NSAccessibilityWindowRole,
            Role::Application => NSAccessibilityGroupRole,
            Role::Article => NSAccessibilityGroupRole,
            Role::Audio => NSAccessibilityGroupRole,
            Role::Banner => NSAccessibilityGroupRole,
            Role::Blockquote => NSAccessibilityGroupRole,
            Role::Canvas => NSAccessibilityImageRole,
            Role::Caption => NSAccessibilityGroupRole,
            Role::Caret => NSAccessibilityUnknownRole,
            Role::Code => NSAccessibilityGroupRole,
            Role::ColorWell => NSAccessibilityColorWellRole,
            Role::ComboBox => NSAccessibilityPopUpButtonRole,
            Role::EditableComboBox => NSAccessibilityComboBoxRole,
            Role::Complementary => NSAccessibilityGroupRole,
            Role::Comment => NSAccessibilityGroupRole,
            Role::ContentDeletion => NSAccessibilityGroupRole,
            Role::ContentInsertion => NSAccessibilityGroupRole,
            Role::ContentInfo => NSAccessibilityGroupRole,
            Role::Definition => NSAccessibilityGroupRole,
            Role::DescriptionList => NSAccessibilityListRole,
            Role::Details => NSAccessibilityGroupRole,
            Role::Dialog => NSAccessibilityWindowRole,
            Role::DisclosureTriangle => NSAccessibilityButtonRole,
            Role::Document => NSAccessibilityGroupRole,
            Role::EmbeddedObject => NSAccessibilityGroupRole,
            Role::Emphasis => NSAccessibilityGroupRole,
            Role::Feed => NSAccessibilityUnknownRole,
            Role::FigureCaption => NSAccessibilityGroupRole,
            Role::Figure => NSAccessibilityGroupRole,
            Role::Footer => NSAccessibilityGroupRole,
            Role::Form => NSAccessibilityGroupRole,
            Role::Grid => NSAccessibilityTableRole,
            Role::Group => NSAccessibilityGroupRole,
            Role::Header => NSAccessibilityGroupRole,
            Role::Heading => ns_string!("Heading"),
            Role::Iframe => NSAccessibilityGroupRole,
            Role::IframePresentational => NSAccessibilityGroupRole,
            Role::ImeCandidate => NSAccessibilityUnknownRole,
            Role::Keyboard => NSAccessibilityUnknownRole,
            Role::Legend => NSAccessibilityGroupRole,
            Role::LineBreak => NSAccessibilityGroupRole,
            Role::ListBox => NSAccessibilityListRole,
            Role::Log => NSAccessibilityGroupRole,
            Role::Main => NSAccessibilityGroupRole,
            Role::Mark => NSAccessibilityGroupRole,
            Role::Marquee => NSAccessibilityGroupRole,
            Role::Math => NSAccessibilityGroupRole,
            Role::MenuBar => NSAccessibilityMenuBarRole,
            Role::MenuItemCheckBox => NSAccessibilityMenuItemRole,
            Role::MenuItemRadio => NSAccessibilityMenuItemRole,
            Role::MenuListPopup => NSAccessibilityMenuRole,
            Role::Meter => NSAccessibilityLevelIndicatorRole,
            Role::Navigation => NSAccessibilityGroupRole,
            Role::Note => NSAccessibilityGroupRole,
            Role::PluginObject => NSAccessibilityGroupRole,
            Role::ProgressIndicator => NSAccessibilityProgressIndicatorRole,
            Role::RadioGroup => NSAccessibilityRadioGroupRole,
            Role::Region => NSAccessibilityGroupRole,
            Role::RootWebArea => ns_string!("AXWebArea"),
            Role::Ruby => NSAccessibilityGroupRole,
            Role::RubyAnnotation => NSAccessibilityUnknownRole,
            Role::ScrollBar => NSAccessibilityScrollBarRole,
            Role::ScrollView => NSAccessibilityUnknownRole,
            Role::Search => NSAccessibilityGroupRole,
            Role::Section => NSAccessibilityGroupRole,
            Role::SectionFooter => NSAccessibilityGroupRole,
            Role::SectionHeader => NSAccessibilityGroupRole,
            Role::Slider => NSAccessibilitySliderRole,
            Role::SpinButton => NSAccessibilityIncrementorRole,
            Role::Splitter => NSAccessibilitySplitterRole,
            Role::Status => NSAccessibilityGroupRole,
            Role::Strong => NSAccessibilityGroupRole,
            Role::Suggestion => NSAccessibilityGroupRole,
            Role::SvgRoot => NSAccessibilityGroupRole,
            Role::Tab => NSAccessibilityRadioButtonRole,
            Role::TabList => NSAccessibilityTabGroupRole,
            Role::TabPanel => NSAccessibilityGroupRole,
            Role::Term => NSAccessibilityGroupRole,
            Role::Time => NSAccessibilityGroupRole,
            Role::Timer => NSAccessibilityGroupRole,
            Role::TitleBar => NSAccessibilityStaticTextRole,
            Role::Toolbar => NSAccessibilityToolbarRole,
            Role::Tooltip => NSAccessibilityGroupRole,
            Role::Tree => NSAccessibilityOutlineRole,
            Role::TreeGrid => NSAccessibilityTableRole,
            Role::Video => NSAccessibilityGroupRole,
            Role::WebView => NSAccessibilityUnknownRole,
            // Use the group role for Role::Window, since the NSWindow
            // provides the top-level accessibility object for the window.
            Role::Window => NSAccessibilityGroupRole,
            Role::PdfActionableHighlight => NSAccessibilityButtonRole,
            Role::PdfRoot => NSAccessibilityGroupRole,
            Role::GraphicsDocument => NSAccessibilityGroupRole,
            Role::GraphicsObject => NSAccessibilityGroupRole,
            Role::GraphicsSymbol => NSAccessibilityImageRole,
            Role::DocAbstract => NSAccessibilityGroupRole,
            Role::DocAcknowledgements => NSAccessibilityGroupRole,
            Role::DocAfterword => NSAccessibilityGroupRole,
            Role::DocAppendix => NSAccessibilityGroupRole,
            Role::DocBackLink => NSAccessibilityLinkRole,
            Role::DocBiblioEntry => NSAccessibilityGroupRole,
            Role::DocBibliography => NSAccessibilityGroupRole,
            Role::DocBiblioRef => NSAccessibilityGroupRole,
            Role::DocChapter => NSAccessibilityGroupRole,
            Role::DocColophon => NSAccessibilityGroupRole,
            Role::DocConclusion => NSAccessibilityGroupRole,
            Role::DocCover => NSAccessibilityImageRole,
            Role::DocCredit => NSAccessibilityGroupRole,
            Role::DocCredits => NSAccessibilityGroupRole,
            Role::DocDedication => NSAccessibilityGroupRole,
            Role::DocEndnote => NSAccessibilityGroupRole,
            Role::DocEndnotes => NSAccessibilityGroupRole,
            Role::DocEpigraph => NSAccessibilityGroupRole,
            Role::DocEpilogue => NSAccessibilityGroupRole,
            Role::DocErrata => NSAccessibilityGroupRole,
            Role::DocExample => NSAccessibilityGroupRole,
            Role::DocFootnote => NSAccessibilityGroupRole,
            Role::DocForeword => NSAccessibilityGroupRole,
            Role::DocGlossary => NSAccessibilityGroupRole,
            Role::DocGlossRef => NSAccessibilityLinkRole,
            Role::DocIndex => NSAccessibilityGroupRole,
            Role::DocIntroduction => NSAccessibilityGroupRole,
            Role::DocNoteRef => NSAccessibilityLinkRole,
            Role::DocNotice => NSAccessibilityGroupRole,
            Role::DocPageBreak => NSAccessibilitySplitterRole,
            Role::DocPageFooter => NSAccessibilityGroupRole,
            Role::DocPageHeader => NSAccessibilityGroupRole,
            Role::DocPageList => NSAccessibilityGroupRole,
            Role::DocPart => NSAccessibilityGroupRole,
            Role::DocPreface => NSAccessibilityGroupRole,
            Role::DocPrologue => NSAccessibilityGroupRole,
            Role::DocPullquote => NSAccessibilityGroupRole,
            Role::DocQna => NSAccessibilityGroupRole,
            Role::DocSubtitle => ns_string!("AXHeading"),
            Role::DocTip => NSAccessibilityGroupRole,
            Role::DocToc => NSAccessibilityGroupRole,
            Role::ListGrid => NSAccessibilityUnknownRole,
            Role::Terminal => NSAccessibilityTextAreaRole,
        }
    }
}

fn ns_sub_role(node: &Node) -> &'static NSAccessibilitySubrole {
    let role = node.role();

    unsafe {
        match role {
            Role::Alert => ns_string!("AXApplicationAlert"),
            Role::AlertDialog => NSAccessibilityDialogSubrole,
            Role::Article => ns_string!("AXDocumentArticle"),
            Role::Banner => ns_string!("AXLandmarkBanner"),
            Role::Button if node.toggled().is_some() => NSAccessibilityToggleSubrole,
            Role::Code => ns_string!("AXCodeStyleGroup"),
            Role::Complementary => ns_string!("AXLandmarkComplementary"),
            Role::ContentDeletion => ns_string!("AXDeleteStyleGroup"),
            Role::ContentInsertion => ns_string!("AXInsertStyleGroup"),
            Role::ContentInfo => ns_string!("AXLandmarkContentInfo"),
            Role::Definition => ns_string!("AXDefinition"),
            Role::Dialog => NSAccessibilityDialogSubrole,
            Role::Document => ns_string!("AXDocument"),
            Role::Emphasis => ns_string!("AXEmphasisStyleGroup"),
            Role::Feed => ns_string!("AXApplicationGroup"),
            Role::Footer => ns_string!("AXLandmarkContentInfo"),
            Role::Form => ns_string!("AXLandmarkForm"),
            Role::GraphicsDocument => ns_string!("AXDocument"),
            Role::Group => ns_string!("AXApplicationGroup"),
            Role::Header => ns_string!("AXLandmarkBanner"),
            Role::LayoutTableCell => NSAccessibilityGroupRole,
            Role::LayoutTableRow => NSAccessibilityTableRowSubrole,
            Role::Log => ns_string!("AXApplicationLog"),
            Role::Main => ns_string!("AXLandmarkMain"),
            Role::Marquee => ns_string!("AXApplicationMarquee"),
            Role::Math => ns_string!("AXDocumentMath"),
            Role::Meter => ns_string!("AXMeter"),
            Role::Navigation => ns_string!("AXLandmarkNavigation"),
            Role::Note => ns_string!("AXDocumentNote"),
            Role::PasswordInput => NSAccessibilitySecureTextFieldSubrole,
            Role::Region => ns_string!("AXLandmarkRegion"),
            Role::Search => ns_string!("AXLandmarkSearch"),
            Role::SearchInput => NSAccessibilitySearchFieldSubrole,
            Role::SectionFooter => ns_string!("AXSectionFooter"),
            Role::SectionHeader => ns_string!("AXSectionHeader"),
            Role::Status => ns_string!("AXApplicationStatus"),
            Role::Strong => ns_string!("AXStrongStyleGroup"),
            Role::Switch => NSAccessibilitySwitchSubrole,
            Role::Tab => NSAccessibilityTabButtonSubrole,
            Role::TabPanel => ns_string!("AXTabPanel"),
            Role::Term => ns_string!("AXTerm"),
            Role::Time => ns_string!("AXTimeGroup"),
            Role::Timer => ns_string!("AXApplicationTimer"),
            Role::TreeItem => NSAccessibilityOutlineRowSubrole,
            Role::Tooltip => ns_string!("AXUserInterfaceTooltip"),
            _ => NSAccessibilityUnknownSubrole,
        }
    }
}

pub(crate) fn can_be_focused(node: &Node) -> bool {
    filter(node) == FilterResult::Include && node.role() != Role::Window
}

#[derive(PartialEq)]
pub(crate) enum Value {
    Bool(bool),
    Number(f64),
    String(String),
}

pub(crate) struct NodeWrapper<'a>(pub(crate) &'a Node<'a>);

impl NodeWrapper<'_> {
    fn is_root(&self) -> bool {
        self.0.is_root()
    }

    pub(crate) fn title(&self) -> Option<String> {
        if self.is_root() && self.0.role() == Role::Window {
            // If the group element that we expose for the top-level window
            // includes a title, VoiceOver behavior is broken.
            return None;
        }
        self.0.label()
    }

    pub(crate) fn description(&self) -> Option<String> {
        self.0.description()
    }

    pub(crate) fn placeholder(&self) -> Option<&str> {
        self.0.placeholder()
    }

    pub(crate) fn value(&self) -> Option<Value> {
        if let Some(toggled) = self.0.toggled() {
            return Some(Value::Bool(toggled != Toggled::False));
        }
        if self.0.role() == Role::Tab {
            // On Mac, tabs are exposed as radio buttons, and are treated as checkable.
            // Also, `Node::is_selected` is mapped to checked via `accessibilityValue`.
            return Some(Value::Bool(self.0.is_selected().unwrap_or(false)));
        }
        if let Some(value) = self.0.value() {
            return Some(Value::String(value));
        }
        if let Some(value) = self.0.numeric_value() {
            return Some(Value::Number(value));
        }
        None
    }

    pub(crate) fn supports_text_ranges(&self) -> bool {
        self.0.supports_text_ranges()
    }

    pub(crate) fn raw_text_selection(&self) -> Option<&TextSelection> {
        self.0.raw_text_selection()
    }

    fn is_container_with_selectable_children(&self) -> bool {
        self.0.is_container_with_selectable_children() && self.0.role() != Role::TabList
    }

    pub(crate) fn is_item_like(&self) -> bool {
        self.0.is_item_like() && self.0.role() != Role::Tab
    }
}

// derived from objc2 0.6 `AnyObject::downcast_ref`
// TODO: can be removed after updating objc2 to 0.6 which has `AnyObject::downcast_ref`
fn downcast_ref<T: ClassType>(obj: &NSObject) -> Option<&T> {
    obj.is_kind_of::<T>()
        .then(|| unsafe { &*(obj as *const NSObject).cast::<T>() })
}

pub(crate) struct PlatformNodeIvars {
    context: Weak<Context>,
    node_id: NodeId,
}

declare_class!(
    #[derive(Debug)]
    pub(crate) struct PlatformNode;

    unsafe impl ClassType for PlatformNode {
        #[inherits(NSObject)]
        type Super = NSAccessibilityElement;
        type Mutability = InteriorMutable;
        const NAME: &'static str = "AccessKitNode";
    }

    impl DeclaredClass for PlatformNode {
        type Ivars = PlatformNodeIvars;
    }

    unsafe impl PlatformNode {
        #[method_id(accessibilityParent)]
        fn parent(&self) -> Option<Id<AnyObject>> {
            self.resolve_with_context(|node, _, context| {
                if let Some(parent) = node.filtered_parent(&filter) {
                    Some(Id::into_super(Id::into_super(Id::into_super(context.get_or_create_platform_node(parent.id())))))
                } else {
                    context
                        .view
                        .load()
                        .and_then(|view| unsafe { NSAccessibility::accessibilityParent(&*view) })
                }
            })
            .flatten()
        }

        #[method_id(accessibilityWindow)]
        fn window(&self) -> Option<Id<AnyObject>> {
            self.resolve_with_context(|_, _, context| {
                context
                    .view
                    .load()
                    .and_then(|view| unsafe { NSAccessibility::accessibilityParent(&*view) })
            })
            .flatten()
        }

        #[method_id(accessibilityTopLevelUIElement)]
        fn top_level(&self) -> Option<Id<AnyObject>> {
            self.resolve_with_context(|_, _, context| {
                context
                    .view
                    .load()
                    .and_then(|view| unsafe { NSAccessibility::accessibilityParent(&*view) })
            })
            .flatten()
        }

        #[method_id(accessibilityChildren)]
        fn children(&self) -> Option<Id<NSArray<PlatformNode>>> {
            self.children_internal()
        }

        #[method_id(accessibilityChildrenInNavigationOrder)]
        fn children_in_navigation_order(&self) -> Option<Id<NSArray<PlatformNode>>> {
            // For now, we assume the children are in navigation order.
            self.children_internal()
        }

        #[method_id(accessibilitySelectedChildren)]
        fn selected_children(&self) -> Option<Id<NSArray<PlatformNode>>> {
            self.resolve_with_context(|node, _, context| {
                let wrapper = NodeWrapper(node);
                if !wrapper.is_container_with_selectable_children() {
                    return None;
                }
                let platform_nodes = node
                    .items(filter)
                    .filter(|item| item.is_selected() == Some(true))
                    .map(|child| context.get_or_create_platform_node(child.id()))
                    .collect::<Vec<Id<PlatformNode>>>();
                Some(NSArray::from_vec(platform_nodes))
            })
            .flatten()
        }

        #[method(accessibilityFrame)]
        fn frame(&self) -> NSRect {
            self.resolve_with_context(|node, _, context| {
                let view = match context.view.load() {
                    Some(view) => view,
                    None => {
                        return NSRect::ZERO;
                    }
                };

                node.bounding_box().map_or_else(
                    || {
                        if node.is_root() {
                            unsafe { NSAccessibility::accessibilityFrame(&*view) }
                        } else {
                            NSRect::ZERO
                        }
                    },
                    |rect| to_ns_rect(&view, rect),
                )
            })
            .unwrap_or(NSRect::ZERO)
        }

        #[method_id(accessibilityRole)]
        fn role(&self) -> Id<NSAccessibilityRole> {
            self.resolve(ns_role)
                .unwrap_or(unsafe { NSAccessibilityUnknownRole })
                .copy()
        }

        #[method_id(accessibilitySubrole)]
        fn sub_role(&self) -> Id<NSAccessibilitySubrole> {
            self.resolve(ns_sub_role)
                .unwrap_or(unsafe { NSAccessibilityUnknownSubrole })
                .copy()
        }

        #[method_id(accessibilityRoleDescription)]
        fn role_description(&self) -> Option<Id<NSString>> {
            self.resolve(|node| {
                if let Some(role_description) = node.role_description() {
                    Some(NSString::from_str(role_description))
                } else {
                    unsafe { msg_send_id![super(self), accessibilityRoleDescription] }
                }
            })
            .flatten()
        }

        #[method_id(accessibilityIdentifier)]
        fn identifier(&self) -> Option<Id<NSString>> {
            self.resolve(|node| {
                node.author_id().map(NSString::from_str)
            })
            .flatten()
        }

        #[method_id(accessibilityTitle)]
        fn title(&self) -> Option<Id<NSString>> {
            self.resolve(|node| {
                let wrapper = NodeWrapper(node);
                wrapper.title().map(|title| NSString::from_str(&title))
            })
            .flatten()
        }

        #[method_id(accessibilityHelp)]
        fn description(&self) -> Option<Id<NSString>> {
            self.resolve(|node| {
                let wrapper = NodeWrapper(node);
                wrapper.description().map(|description| NSString::from_str(&description))
            })
            .flatten()
        }

        #[method_id(accessibilityPlaceholderValue)]
        fn placeholder(&self) -> Option<Id<NSString>> {
            self.resolve(|node| {
                let wrapper = NodeWrapper(node);
                wrapper.placeholder().map(NSString::from_str)
            })
            .flatten()
        }

        #[method_id(accessibilityValue)]
        fn value(&self) -> Option<Id<NSObject>> {
            self.resolve(|node| {
                let wrapper = NodeWrapper(node);
                wrapper.value().map(|value| match value {
                    Value::Bool(value) => {
                        Id::into_super(Id::into_super(NSNumber::new_bool(value)))
                    }
                    Value::Number(value) => {
                        Id::into_super(Id::into_super(NSNumber::new_f64(value)))
                    }
                    Value::String(value) => {
                        Id::into_super(NSString::from_str(&value))
                    }
                })
            })
            .flatten()
        }

        #[method(setAccessibilityValue:)]
        fn set_value(&self, value: &NSObject) {
            if let Some(string) = downcast_ref::<NSString>(value) {
                self.resolve_with_context(|node, tree, context| {
                    if let Some((target_node, target_tree)) = tree.state().locate_node(node.id()) {
                        context.do_action(ActionRequest {
                            action: Action::SetValue,
                            target_tree,
                            target_node,
                            data: Some(ActionData::Value(string.to_string().into())),
                        });
                    }
                });
            } else if let Some(number) = downcast_ref::<NSNumber>(value) {
                self.resolve_with_context(|node, tree, context| {
                    if let Some((target_node, target_tree)) = tree.state().locate_node(node.id()) {
                        context.do_action(ActionRequest {
                            action: Action::SetValue,
                            target_tree,
                            target_node,
                            data: Some(ActionData::NumericValue(number.doubleValue())),
                        });
                    }
                });
            }
        }

        #[method_id(accessibilityMinValue)]
        fn min_value(&self) -> Option<Id<NSNumber>> {
            self.resolve(|node| {
                node.min_numeric_value().map(NSNumber::new_f64)
            })
            .flatten()
        }

        #[method_id(accessibilityMaxValue)]
        fn max_value(&self) -> Option<Id<NSNumber>> {
            self.resolve(|node| {
                node.max_numeric_value().map(NSNumber::new_f64)
            })
            .flatten()
        }

        #[method_id(accessibilityURL)]
        fn url(&self) -> Option<Id<NSURL>> {
            self.resolve(|node| {
                node.supports_url().then(|| node.url()).flatten().and_then(|url| {
                    let ns_string = NSString::from_str(url);
                    unsafe { NSURL::URLWithString(&ns_string) }
                })
            })
            .flatten()
        }

        #[method(accessibilityOrientation)]
        fn orientation(&self) -> NSAccessibilityOrientation {
            self.resolve(|node| {
                match node.orientation() {
                    Some(Orientation::Horizontal) => NSAccessibilityOrientation::Horizontal,
                    Some(Orientation::Vertical) => NSAccessibilityOrientation::Vertical,
                    None => NSAccessibilityOrientation::Unknown,
                }
            })
            .unwrap_or(NSAccessibilityOrientation::Unknown)
        }

        #[method(isAccessibilityElement)]
        fn is_accessibility_element(&self) -> bool {
            self.resolve(|node| filter(node) == FilterResult::Include)
                .unwrap_or(false)
        }

        #[method(isAccessibilityFocused)]
        fn is_focused(&self) -> bool {
            self.resolve(|node| node.is_focused() && can_be_focused(node))
                .unwrap_or(false)
        }

        #[method(isAccessibilityEnabled)]
        fn is_enabled(&self) -> bool {
            self.resolve(|node| !node.is_disabled()).unwrap_or(false)
        }

        #[method(setAccessibilityFocused:)]
        fn set_focused(&self, focused: bool) {
            self.resolve_with_context(|node, tree, context| {
                if focused {
                    if node.is_focusable(&filter) {
                        if let Some((target_node, target_tree)) = tree.state().locate_node(node.id()) {
                            context.do_action(ActionRequest {
                                action: Action::Focus,
                                target_tree,
                                target_node,
                                data: None,
                            });
                        }
                    }
                } else {
                    let root = tree.state().root();
                    if root.is_focusable(&filter) {
                        if let Some((target_node, target_tree)) = tree.state().locate_node(root.id()) {
                            context.do_action(ActionRequest {
                                action: Action::Focus,
                                target_tree,
                                target_node,
                                data: None,
                            });
                        }
                    }
                }
            });
        }

        #[method(accessibilityPerformPress)]
        fn press(&self) -> bool {
            self.resolve_with_context(|node, tree, context| {
                let clickable = node.is_clickable(&filter);
                if clickable {
                    if let Some((target_node, target_tree)) = tree.state().locate_node(node.id()) {
                        context.do_action(ActionRequest {
                            action: Action::Click,
                            target_tree,
                            target_node,
                            data: None,
                        });
                    }
                }
                clickable
            })
            .unwrap_or(false)
        }

        #[method(accessibilityPerformIncrement)]
        fn increment(&self) -> bool {
            self.resolve_with_context(|node, tree, context| {
                let supports_increment = node.supports_increment(&filter);
                if supports_increment {
                    if let Some((target_node, target_tree)) = tree.state().locate_node(node.id()) {
                        context.do_action(ActionRequest {
                            action: Action::Increment,
                            target_tree,
                            target_node,
                            data: None,
                        });
                    }
                }
                supports_increment
            })
            .unwrap_or(false)
        }

        #[method(accessibilityPerformDecrement)]
        fn decrement(&self) -> bool {
            self.resolve_with_context(|node, tree, context| {
                let supports_decrement = node.supports_decrement(&filter);
                if supports_decrement {
                    if let Some((target_node, target_tree)) = tree.state().locate_node(node.id()) {
                        context.do_action(ActionRequest {
                            action: Action::Decrement,
                            target_tree,
                            target_node,
                            data: None,
                        });
                    }
                }
                supports_decrement
            })
            .unwrap_or(false)
        }

        #[method(accessibilityNotifiesWhenDestroyed)]
        fn notifies_when_destroyed(&self) -> bool {
            true
        }

        #[method(accessibilityNumberOfCharacters)]
        fn number_of_characters(&self) -> NSInteger {
            self.resolve(|node| {
                if node.supports_text_ranges() {
                    node.document_range().end().to_global_utf16_index() as _
                } else {
                    0
                }
            })
            .unwrap_or(0)
        }

        #[method_id(accessibilitySelectedText)]
        fn selected_text(&self) -> Option<Id<NSString>> {
            self.resolve(|node| {
                if node.supports_text_ranges() {
                    if let Some(range) = node.text_selection() {
                        let text = range.text();
                        return Some(NSString::from_str(&text));
                    }
                }
                None
            })
            .flatten()
        }

        #[method(accessibilitySelectedTextRange)]
        fn selected_text_range(&self) -> NSRange {
            self.resolve(|node| {
                if node.supports_text_ranges() {
                    if let Some(range) = node.text_selection() {
                        return to_ns_range(&range);
                    }
                }
                NSRange::new(0, 0)
            })
            .unwrap_or_else(|| NSRange::new(0, 0))
        }

        #[method(accessibilityInsertionPointLineNumber)]
        fn insertion_point_line_number(&self) -> NSInteger {
            self.resolve(|node| {
                if node.supports_text_ranges() {
                    if let Some(pos) = node.text_selection_focus() {
                        return pos.to_line_index() as _;
                    }
                }
                0
            })
            .unwrap_or(0)
        }

        #[method(accessibilityRangeForLine:)]
        fn range_for_line(&self, line_index: NSInteger) -> NSRange {
            self.resolve(|node| {
                if node.supports_text_ranges() && line_index >= 0 {
                    if let Some(range) = node.line_range_from_index(line_index as _) {
                        return to_ns_range(&range);
                    }
                }
                NSRange::new(0, 0)
            })
            .unwrap_or_else(|| NSRange::new(0, 0))
        }

        #[method(accessibilityRangeForPosition:)]
        fn range_for_position(&self, point: NSPoint) -> NSRange {
            self.resolve_with_context(|node, _, context| {
                let view = match context.view.load() {
                    Some(view) => view,
                    None => {
                        return NSRange::new(0, 0);
                    }
                };

                if node.supports_text_ranges() {
                    let point = from_ns_point(&view, node, point);
                    let pos = node.text_position_at_point(point);
                    return to_ns_range_for_character(&pos);
                }
                NSRange::new(0, 0)
            })
            .unwrap_or_else(|| NSRange::new(0, 0))
        }

        #[method_id(accessibilityStringForRange:)]
        fn string_for_range(&self, range: NSRange) -> Option<Id<NSString>> {
            self.resolve(|node| {
                if node.supports_text_ranges() {
                    if let Some(range) = from_ns_range(node, range) {
                        let text = range.text();
                        return Some(NSString::from_str(&text));
                    }
                }
                None
            })
            .flatten()
        }

        #[method_id(accessibilityAttributedStringForRange:)]
        fn attributed_string_for_range(&self, range: NSRange) -> Option<Id<NSAttributedString>> {
            self.resolve(|node| {
                if node.supports_text_ranges() {
                    if let Some(range) = from_ns_range(node, range) {
                        let mut result = NSMutableAttributedString::new();
                        unsafe { result.beginEditing() };
                        range.traverse_text::<_, ()>(|node, text| {
                            let ns_text = NSString::from_str(text);
                            let mut attrs = NSMutableDictionary::new();
                            if let Some(color) = node.background_color() {
                                attrs.insert_id(
                                    unsafe { NSAccessibilityBackgroundColorTextAttribute },
                                    to_color_attribute(color)
                                );
                            }
                            if let Some(color) = node.foreground_color() {
                                attrs.insert_id(
                                    unsafe { NSAccessibilityForegroundColorTextAttribute },
                                    to_color_attribute(color)
                                );
                            }
                            let mut font_attrs = NSMutableDictionary::<NSAccessibilityFontAttributeKey, AnyObject>::new();
                            if let Some(family) = node.font_family() {
                                font_attrs.insert_id(
                                    unsafe { NSAccessibilityFontFamilyKey },
                                    Id::into_super(Id::into_super(NSString::from_str(family)))
                                );
                            }
                            if let Some(size) = node.font_size() {
                                font_attrs.insert_id(
                                    unsafe { NSAccessibilityFontSizeKey },
                                    Id::into_super(Id::into_super(Id::into_super(NSNumber::new_f32(size))))
                                );
                            }
                            if let Some(weight) = node.font_weight() {
                                if weight >= 700.0 {
                                    font_attrs.insert_id(
                                        ns_string!("AXFontBold"),
                                        Id::into_super(Id::into_super(Id::into_super(NSNumber::new_bool(true))))
                                    );
                                }
                            }
                            if node.is_italic() {
                                font_attrs.insert_id(
                                    ns_string!("AXFontItalic"),
                                    Id::into_super(Id::into_super(Id::into_super(NSNumber::new_bool(true))))
                                );
                            }
                            if !font_attrs.is_empty() {
                                attrs.insert_id(
                                    unsafe { NSAccessibilityFontTextAttribute },
                                    Id::into_super(Id::into_super(Id::into_super(font_attrs)))
                                );
                            }
                            if let Some(deco) = node.underline() {
                                attrs.insert_id(
                                    unsafe { NSAccessibilityUnderlineTextAttribute },
                                    Id::into_super(Id::into_super(Id::into_super(NSNumber::new_bool(true))))
                                );
                                attrs.insert_id(
                                    unsafe { NSAccessibilityUnderlineColorTextAttribute },
                                    to_color_attribute(deco.color)
                                );
                            }
                            if let Some(deco) = node.strikethrough() {
                                attrs.insert_id(
                                    unsafe { NSAccessibilityStrikethroughTextAttribute },
                                    Id::into_super(Id::into_super(Id::into_super(NSNumber::new_bool(true))))
                                );
                                attrs.insert_id(
                                    unsafe { NSAccessibilityStrikethroughColorTextAttribute },
                                    to_color_attribute(deco.color)
                                );
                            }
                            if let Some(language) = node.language() {
                                attrs.insert_id(
                                    unsafe { NSAccessibilityLanguageTextAttribute },
                                    Id::into_super(Id::into_super(NSString::from_str(language)))
                                );
                            }
                            if let Some(align) = node.text_align() {
                                let ns_align = match align {
                                    TextAlign::Left => NSTextAlignment::Left,
                                    TextAlign::Center => NSTextAlignment::Center,
                                    TextAlign::Right => NSTextAlignment::Right,
                                    TextAlign::Justify => NSTextAlignment::Justified,
                                };
                                attrs.insert_id(
                                    unsafe { NSAccessibilityTextAlignmentAttribute },
                                    Id::into_super(Id::into_super(Id::into_super(NSNumber::new_isize(ns_align.0))))
                                );
                            }
                            let part = unsafe { NSAttributedString::new_with_attributes(&ns_text, &attrs) };
                            unsafe { result.appendAttributedString(&part) };
                            None
                        });
                        unsafe { result.endEditing() };
                        return Some(Id::into_super(result));
                    }
                }
                None
            })
            .flatten()
        }

        #[method(accessibilityFrameForRange:)]
        fn frame_for_range(&self, range: NSRange) -> NSRect {
            self.resolve_with_context(|node, _, context| {
                let view = match context.view.load() {
                    Some(view) => view,
                    None => {
                        return NSRect::ZERO;
                    }
                };

                if node.supports_text_ranges() {
                    if let Some(range) = from_ns_range(node, range) {
                        let rects = range.bounding_boxes();
                        if let Some(rect) =
                            rects.into_iter().reduce(|rect1, rect2| rect1.union(rect2))
                        {
                            return to_ns_rect(&view, rect);
                        }
                    }
                }
                NSRect::ZERO
            })
            .unwrap_or(NSRect::ZERO)
        }

        #[method(accessibilityLineForIndex:)]
        fn line_for_index(&self, index: NSInteger) -> NSInteger {
            self.resolve(|node| {
                if node.supports_text_ranges() && index >= 0 {
                    if let Some(pos) = node.text_position_from_global_utf16_index(index as _) {
                        return pos.to_line_index() as _;
                    }
                }
                0
            })
            .unwrap_or(0)
        }

        #[method(accessibilityRangeForIndex:)]
        fn range_for_index(&self, index: NSInteger) -> NSRange {
            self.resolve(|node| {
                if node.supports_text_ranges() && index >= 0 {
                    if let Some(pos) = node.text_position_from_global_utf16_index(index as _) {
                        return to_ns_range_for_character(&pos);
                    }
                }
                NSRange::new(0, 0)
            })
            .unwrap_or_else(|| NSRange::new(0, 0))
        }

        #[method(accessibilityStyleRangeForIndex:)]
        fn style_range_for_index(&self, index: NSInteger) -> NSRange {
            self.resolve(|node| {
                if node.supports_text_ranges() && index >= 0 {
                    if let Some(pos) = node.text_position_from_global_utf16_index(index as _) {
                        let start = if pos.is_format_start() {
                            pos
                        } else {
                            pos.backward_to_format_start()
                        };
                        let mut range = start.to_degenerate_range();
                        range.set_end(pos.forward_to_format_end());
                        return to_ns_range(&range);
                    }
                }
                NSRange::new(0, 0)
            })
            .unwrap_or_else(|| NSRange::new(0, 0))
        }

        #[method(setAccessibilitySelectedTextRange:)]
        fn set_selected_text_range(&self, range: NSRange) {
            self.resolve_with_context(|node, tree, context| {
                if node.supports_text_ranges() {
                    if let Some(range) = from_ns_range(node, range) {
                        if let Some((target_node, target_tree)) = tree.state().locate_node(node.id()) {
                            context.do_action(ActionRequest {
                                action: Action::SetTextSelection,
                                target_tree,
                                target_node,
                                data: Some(ActionData::SetTextSelection(range.to_text_selection())),
                            });
                        }
                    }
                }
            });
        }

        #[method(isAccessibilityRequired)]
        fn is_required(&self) -> bool {
            self.resolve(|node| node.is_required())
                .unwrap_or(false)
        }

        #[method(isAccessibilitySelected)]
        fn is_selected(&self) -> bool {
            self.resolve(|node| {
                let wrapper = NodeWrapper(node);
                wrapper.is_item_like()
                    && node.is_selectable()
                    && node.is_selected().unwrap_or(false)
            })
            .unwrap_or(false)
        }

        #[method(setAccessibilitySelected:)]
        fn set_selected(&self, selected: bool) {
            self.resolve_with_context(|node, tree, context| {
                let wrapper = NodeWrapper(node);
                if !node.is_clickable(&filter)
                    || !wrapper.is_item_like()
                    || !node.is_selectable()
                {
                    return;
                }
                if node.is_selected() == Some(selected) {
                    return;
                }
                if let Some((target_node, target_tree)) = tree.state().locate_node(node.id()) {
                    context.do_action(ActionRequest {
                        action: Action::Click,
                        target_tree,
                        target_node,
                        data: None,
                    });
                }
            });
        }

        #[method_id(accessibilityAttributeValue:)]
        fn accessibility_attribute_value(&self, attr: &NSString) -> Option<Id<NSString>> {
            self.resolve(|node| {
                if attr == ns_string!("AXBrailleLabel") && node.has_braille_label() {
                    return Some(NSString::from_str(node.braille_label().unwrap()))
                } else if attr == ns_string!("AXBrailleRoleDescription") && node.has_braille_role_description() {
                    return Some(NSString::from_str(node.braille_role_description().unwrap()))
                }

                None
            })
            .flatten()
        }

        #[method_id(accessibilityRows)]
        fn rows(&self) -> Option<Id<NSArray<PlatformNode>>> {
            self.resolve_with_context(|node, _, context| {
                let wrapper = NodeWrapper(node);
                if !wrapper.is_container_with_selectable_children() {
                    return None;
                }
                let platform_nodes = node
                    .items(filter)
                    .map(|child| context.get_or_create_platform_node(child.id()))
                    .collect::<Vec<Id<PlatformNode>>>();
                Some(NSArray::from_vec(platform_nodes))
            })
            .flatten()
        }

        #[method_id(accessibilitySelectedRows)]
        fn selected_rows(&self) -> Option<Id<NSArray<PlatformNode>>> {
            self.resolve_with_context(|node, _, context| {
                let wrapper = NodeWrapper(node);
                if !wrapper.is_container_with_selectable_children() {
                    return None;
                }
                let platform_nodes = node
                    .items(filter)
                    .filter(|item| item.is_selected() == Some(true))
                    .map(|child| context.get_or_create_platform_node(child.id()))
                    .collect::<Vec<Id<PlatformNode>>>();
                Some(NSArray::from_vec(platform_nodes))
            })
            .flatten()
        }

        #[method(accessibilityPerformPick)]
        fn pick(&self) -> bool {
            self.resolve_with_context(|node, tree, context| {
                let wrapper = NodeWrapper(node);
                let selectable = node.is_clickable(&filter)
                    && wrapper.is_item_like()
                    && node.is_selectable();
                if selectable {
                    if let Some((target_node, target_tree)) = tree.state().locate_node(node.id()) {
                        context.do_action(ActionRequest {
                            action: Action::Click,
                            target_tree,
                            target_node,
                            data: None,
                        });
                    }
                }
                selectable
            })
            .unwrap_or(false)
        }

        #[method_id(accessibilityLinkedUIElements)]
        fn linked_ui_elements(&self) -> Option<Id<NSArray<PlatformNode>>> {
            self.resolve_with_context(|node, _, context| {
                let platform_nodes: Vec<Id<PlatformNode>> = node
                    .controls()
                    .filter(|controlled| filter(controlled) == FilterResult::Include)
                    .map(|controlled| context.get_or_create_platform_node(controlled.id()))
                    .collect();
                if platform_nodes.is_empty() {
                    None
                } else {
                    Some(NSArray::from_vec(platform_nodes))
                }
            })
            .flatten()
        }

        #[method_id(accessibilityTabs)]
        fn tabs(&self) -> Option<Id<NSArray<PlatformNode>>> {
            self.resolve_with_context(|node, _, context| {
                if node.role() != Role::TabList {
                    return None;
                }
                let platform_nodes = node
                    .filtered_children(filter)
                    .filter(|child| child.role() == Role::Tab)
                    .map(|tab| context.get_or_create_platform_node(tab.id()))
                    .collect::<Vec<Id<PlatformNode>>>();
                Some(NSArray::from_vec(platform_nodes))
            })
            .flatten()
        }

        #[method(isAccessibilityModal)]
        fn is_modal(&self) -> bool {
            self.resolve(|node| node.is_modal())
                .unwrap_or(false)
        }

        // We discovered through experimentation that when mixing the newer
        // NSAccessibility protocols with the older informal protocol,
        // the platform uses both protocols to discover which actions are
        // available and then perform actions. That means our implementation
        // of the legacy methods below only needs to cover actions not already
        // handled by the newer methods.

        #[method_id(accessibilityActionNames)]
        fn action_names(&self) -> Id<NSArray<NSString>> {
            let mut result = vec![];
            self.resolve(|node| {
                if node.supports_action(Action::ScrollIntoView, &filter) {
                    result.push(ns_string!(SCROLL_TO_VISIBLE_ACTION).copy());
                }
            });
            NSArray::from_vec(result)
        }

        #[method(accessibilityPerformAction:)]
        fn perform_action(&self, action: &NSString) {
            self.resolve_with_context(|node, tree, context| {
                if action == ns_string!(SCROLL_TO_VISIBLE_ACTION) {
                    if let Some((target_node, target_tree)) = tree.state().locate_node(node.id()) {
                        context.do_action(ActionRequest {
                            action: Action::ScrollIntoView,
                            target_tree,
                            target_node,
                            data: None,
                        });
                    }
                }
            });
        }

        #[method(isAccessibilitySelectorAllowed:)]
        fn is_selector_allowed(&self, selector: Sel) -> bool {
            self.resolve(|node| {
                if selector == sel!(setAccessibilityFocused:) {
                    return node.is_focusable(&filter);
                }
                if selector == sel!(accessibilityPerformPress) {
                    return node.is_clickable(&filter);
                }
                if selector == sel!(accessibilityPerformIncrement) {
                    return node.supports_increment(&filter);
                }
                if selector == sel!(accessibilityPerformDecrement) {
                    return node.supports_decrement(&filter);
                }
                if selector == sel!(accessibilityNumberOfCharacters)
                    || selector == sel!(accessibilitySelectedText)
                    || selector == sel!(accessibilitySelectedTextRange)
                    || selector == sel!(accessibilityInsertionPointLineNumber)
                    || selector == sel!(accessibilityRangeForLine:)
                    || selector == sel!(accessibilityRangeForPosition:)
                    || selector == sel!(accessibilityStringForRange:)
                    || selector == sel!(accessibilityAttributedStringForRange:)
                    || selector == sel!(accessibilityFrameForRange:)
                    || selector == sel!(accessibilityLineForIndex:)
                    || selector == sel!(accessibilityRangeForIndex:)
                    || selector == sel!(accessibilityStyleRangeForIndex:)
                    || selector == sel!(setAccessibilitySelectedTextRange:)
                {
                    return node.supports_text_ranges();
                }
                if selector == sel!(setAccessibilityValue:) {
                    return (node.supports_text_ranges() && !node.is_read_only()) || node.supports_action(Action::SetValue, &filter);
                }
                if selector == sel!(isAccessibilitySelected) {
                    let wrapper = NodeWrapper(node);
                    return wrapper.is_item_like();
                }
                if selector == sel!(accessibilityRows)
                    || selector == sel!(accessibilitySelectedRows)
                {
                    let wrapper = NodeWrapper(node);
                    return wrapper.is_container_with_selectable_children()
                }
                if selector == sel!(setAccessibilitySelected:)
                    || selector == sel!(accessibilityPerformPick)
                {
                    let wrapper = NodeWrapper(node);
                    return node.is_clickable(&filter)
                        && wrapper.is_item_like()
                        && node.is_selectable();
                }
                if selector == sel!(accessibilityTabs) {
                    return node.role() == Role::TabList;
                }
                if selector == sel!(isAccessibilityModal) {
                    return node.is_dialog();
                }
                if selector == sel!(accessibilityAttributeValue:) {
                    return node.has_braille_label() || node.has_braille_role_description()
                }
                if selector == sel!(accessibilityURL) {
                    return node.supports_url();
                }
                selector == sel!(accessibilityParent)
                    || selector == sel!(accessibilityChildren)
                    || selector == sel!(accessibilityChildrenInNavigationOrder)
                    || selector == sel!(accessibilitySelectedChildren)
                    || selector == sel!(accessibilityFrame)
                    || selector == sel!(accessibilityRole)
                    || selector == sel!(accessibilitySubrole)
                    || selector == sel!(isAccessibilityEnabled)
                    || selector == sel!(accessibilityWindow)
                    || selector == sel!(accessibilityTopLevelUIElement)
                    || selector == sel!(accessibilityLinkedUIElements)
                    || selector == sel!(accessibilityRoleDescription)
                    || selector == sel!(accessibilityIdentifier)
                    || selector == sel!(accessibilityTitle)
                    || selector == sel!(accessibilityHelp)
                    || selector == sel!(accessibilityPlaceholderValue)
                    || selector == sel!(accessibilityValue)
                    || selector == sel!(accessibilityMinValue)
                    || selector == sel!(accessibilityMaxValue)
                    || selector == sel!(isAccessibilityRequired)
                    || selector == sel!(accessibilityOrientation)
                    || selector == sel!(isAccessibilityElement)
                    || selector == sel!(isAccessibilityFocused)
                    || selector == sel!(accessibilityNotifiesWhenDestroyed)
                    || selector == sel!(isAccessibilitySelectorAllowed:)
                    || selector == sel!(accessibilityActionNames)
                    || selector == sel!(accessibilityPerformAction:)
            })
            .unwrap_or(false)
        }
    }
);

impl PlatformNode {
    pub(crate) fn new(context: Weak<Context>, node_id: NodeId) -> Id<Self> {
        let this = Self::alloc().set_ivars(PlatformNodeIvars { context, node_id });

        unsafe { msg_send_id![super(this), init] }
    }

    fn resolve_with_context<F, T>(&self, f: F) -> Option<T>
    where
        F: FnOnce(&Node, &Tree, &Rc<Context>) -> T,
    {
        let context = self.ivars().context.upgrade()?;
        let tree = context.tree.borrow();
        let state = tree.state();
        let node = state.node_by_id(self.ivars().node_id)?;
        Some(f(&node, &tree, &context))
    }

    fn resolve<F, T>(&self, f: F) -> Option<T>
    where
        F: FnOnce(&Node) -> T,
    {
        self.resolve_with_context(|node, _, _| f(node))
    }

    fn children_internal(&self) -> Option<Id<NSArray<PlatformNode>>> {
        self.resolve_with_context(|node, _, context| {
            let platform_nodes = node
                .filtered_children(filter)
                .map(|child| context.get_or_create_platform_node(child.id()))
                .collect::<Vec<Id<PlatformNode>>>();
            NSArray::from_vec(platform_nodes)
        })
    }
}
