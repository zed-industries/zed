// Copyright 2021 The AccessKit Authors. All rights reserved.
// Licensed under the Apache License, Version 2.0 (found in
// the LICENSE-APACHE file) or the MIT license (found in
// the LICENSE-MIT file), at your option.

// Derived from Chromium's accessibility abstraction.
// Copyright 2018 The Chromium Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE.chromium file.

#![cfg_attr(not(any(feature = "pyo3", feature = "schemars")), no_std)]

extern crate alloc;

#[cfg(feature = "schemars")]
use alloc::borrow::Cow;
use alloc::{boxed::Box, string::String, vec::Vec};
use core::fmt;
#[cfg(feature = "pyo3")]
use pyo3::pyclass;
#[cfg(feature = "schemars")]
use schemars::{JsonSchema, Schema, SchemaGenerator, json_schema};
#[cfg(feature = "serde")]
use serde::{
    Deserialize, Serialize,
    de::{Deserializer, IgnoredAny, MapAccess, Visitor},
    ser::{SerializeMap, Serializer},
};
#[cfg(feature = "schemars")]
use serde_json::{Map as SchemaMap, Value as SchemaValue};

pub use uuid::Uuid;

mod geometry;
pub use geometry::{Affine, Point, Rect, Size, Vec2};

/// The type of an accessibility node.
///
/// The majority of these roles come from the ARIA specification. Reference
/// the latest draft for proper usage.
///
/// Like the AccessKit schema as a whole, this list is largely taken
/// from Chromium. However, unlike Chromium's alphabetized list, this list
/// is ordered roughly by expected usage frequency (with the notable exception
/// of [`Role::Unknown`]). This is more efficient in serialization formats
/// where integers use a variable-length encoding.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "enumn", derive(enumn::N))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[cfg_attr(
    feature = "pyo3",
    pyclass(
        module = "accesskit",
        rename_all = "SCREAMING_SNAKE_CASE",
        eq,
        from_py_object
    )
)]
#[repr(u8)]
pub enum Role {
    #[default]
    Unknown,
    TextRun,
    Cell,
    Label,
    Image,
    Link,
    Row,
    ListItem,

    /// Contains the bullet, number, or other marker for a list item.
    ListMarker,

    TreeItem,
    ListBoxOption,
    MenuItem,
    MenuListOption,
    Paragraph,

    /// A generic container that should be ignored by assistive technologies
    /// and filtered out of platform accessibility trees. Equivalent to the ARIA
    /// `none` or `presentation` role, or to an HTML `div` with no role.
    GenericContainer,

    CheckBox,
    RadioButton,
    TextInput,
    Button,
    DefaultButton,
    Pane,
    RowHeader,
    ColumnHeader,
    RowGroup,
    List,
    Table,
    LayoutTableCell,
    LayoutTableRow,
    LayoutTable,
    Switch,
    Menu,

    MultilineTextInput,
    SearchInput,
    DateInput,
    DateTimeInput,
    WeekInput,
    MonthInput,
    TimeInput,
    EmailInput,
    NumberInput,
    PasswordInput,
    PhoneNumberInput,
    UrlInput,

    Abbr,
    Alert,
    AlertDialog,
    Application,
    Article,
    Audio,
    Banner,
    Blockquote,
    Canvas,
    Caption,
    Caret,
    Code,
    ColorWell,
    ComboBox,
    EditableComboBox,
    Complementary,
    Comment,
    ContentDeletion,
    ContentInsertion,
    ContentInfo,
    Definition,
    DescriptionList,
    Details,
    Dialog,
    DisclosureTriangle,
    Document,
    EmbeddedObject,
    Emphasis,
    Feed,
    FigureCaption,
    Figure,
    Footer,
    Form,
    Grid,
    GridCell,
    Group,
    Header,
    Heading,
    Iframe,
    IframePresentational,
    ImeCandidate,
    Keyboard,
    Legend,
    LineBreak,
    ListBox,
    Log,
    Main,
    Mark,
    Marquee,
    Math,
    MenuBar,
    MenuItemCheckBox,
    MenuItemRadio,
    MenuListPopup,
    Meter,
    Navigation,
    Note,
    PluginObject,
    ProgressIndicator,
    RadioGroup,
    Region,
    RootWebArea,
    Ruby,
    RubyAnnotation,
    ScrollBar,
    ScrollView,
    Search,
    Section,
    SectionFooter,
    SectionHeader,
    Slider,
    SpinButton,
    Splitter,
    Status,
    Strong,
    Suggestion,
    SvgRoot,
    Tab,
    TabList,
    TabPanel,
    Term,
    Time,
    Timer,
    TitleBar,
    Toolbar,
    Tooltip,
    Tree,
    TreeGrid,
    Video,
    WebView,
    Window,

    PdfActionableHighlight,
    PdfRoot,

    // ARIA Graphics module roles:
    // https://rawgit.com/w3c/graphics-aam/master/#mapping_role_table
    GraphicsDocument,
    GraphicsObject,
    GraphicsSymbol,

    // DPub Roles:
    // https://www.w3.org/TR/dpub-aam-1.0/#mapping_role_table
    DocAbstract,
    DocAcknowledgements,
    DocAfterword,
    DocAppendix,
    DocBackLink,
    DocBiblioEntry,
    DocBibliography,
    DocBiblioRef,
    DocChapter,
    DocColophon,
    DocConclusion,
    DocCover,
    DocCredit,
    DocCredits,
    DocDedication,
    DocEndnote,
    DocEndnotes,
    DocEpigraph,
    DocEpilogue,
    DocErrata,
    DocExample,
    DocFootnote,
    DocForeword,
    DocGlossary,
    DocGlossRef,
    DocIndex,
    DocIntroduction,
    DocNoteRef,
    DocNotice,
    DocPageBreak,
    DocPageFooter,
    DocPageHeader,
    DocPageList,
    DocPart,
    DocPreface,
    DocPrologue,
    DocPullquote,
    DocQna,
    DocSubtitle,
    DocTip,
    DocToc,

    /// Behaves similar to an ARIA grid but is primarily used by Chromium's
    /// `TableView` and its subclasses, so they can be exposed correctly
    /// on certain platforms.
    ListGrid,

    /// This is just like a multi-line document, but signals that assistive
    /// technologies should implement behavior specific to a VT-100-style
    /// terminal.
    Terminal,
}

/// An action to be taken on an accessibility node.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "enumn", derive(enumn::N))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[cfg_attr(
    feature = "pyo3",
    pyclass(
        module = "accesskit",
        rename_all = "SCREAMING_SNAKE_CASE",
        eq,
        from_py_object
    )
)]
#[repr(u8)]
pub enum Action {
    /// Do the equivalent of a single click or tap.
    Click,

    Focus,
    Blur,

    Collapse,
    Expand,

    /// Requires [`ActionRequest::data`] to be set to [`ActionData::CustomAction`].
    CustomAction,

    /// Decrement a numeric value by one step.
    Decrement,
    /// Increment a numeric value by one step.
    Increment,

    HideTooltip,
    ShowTooltip,

    /// Delete any selected text in the control's text value and
    /// insert the specified value in its place, like when typing or pasting.
    /// Requires [`ActionRequest::data`] to be set to [`ActionData::Value`].
    ReplaceSelectedText,

    /// Scroll down by the specified unit.
    ScrollDown,
    /// Scroll left by the specified unit.
    ScrollLeft,
    /// Scroll right by the specified unit.
    ScrollRight,
    /// Scroll up by the specified unit.
    ScrollUp,

    /// Scroll any scrollable containers to make the target node visible.
    /// Optionally set [`ActionRequest::data`] to [`ActionData::ScrollHint`].
    ScrollIntoView,

    /// Scroll the given object to a specified point in the tree's container
    /// (e.g. window). Requires [`ActionRequest::data`] to be set to
    /// [`ActionData::ScrollToPoint`].
    ScrollToPoint,

    /// Requires [`ActionRequest::data`] to be set to [`ActionData::SetScrollOffset`].
    SetScrollOffset,

    /// Requires [`ActionRequest::data`] to be set to [`ActionData::SetTextSelection`].
    SetTextSelection,

    /// Don't focus this node, but set it as the sequential focus navigation
    /// starting point, so that pressing Tab moves to the next element
    /// following this one, for example.
    SetSequentialFocusNavigationStartingPoint,

    /// Replace the value of the control with the specified value and
    /// reset the selection, if applicable. Requires [`ActionRequest::data`]
    /// to be set to [`ActionData::Value`] or [`ActionData::NumericValue`].
    SetValue,

    ShowContextMenu,
}

impl Action {
    fn mask(self) -> u32 {
        1 << (self as u8)
    }

    #[cfg(not(feature = "enumn"))]
    fn n(value: u8) -> Option<Self> {
        // Manually implement something similar to the enumn crate. We don't
        // want to bring this crate by default though and we can't use a
        // macro as it would break C bindings header file generation.
        match value {
            0 => Some(Action::Click),
            1 => Some(Action::Focus),
            2 => Some(Action::Blur),
            3 => Some(Action::Collapse),
            4 => Some(Action::Expand),
            5 => Some(Action::CustomAction),
            6 => Some(Action::Decrement),
            7 => Some(Action::Increment),
            8 => Some(Action::HideTooltip),
            9 => Some(Action::ShowTooltip),
            10 => Some(Action::ReplaceSelectedText),
            11 => Some(Action::ScrollDown),
            12 => Some(Action::ScrollLeft),
            13 => Some(Action::ScrollRight),
            14 => Some(Action::ScrollUp),
            15 => Some(Action::ScrollIntoView),
            16 => Some(Action::ScrollToPoint),
            17 => Some(Action::SetScrollOffset),
            18 => Some(Action::SetTextSelection),
            19 => Some(Action::SetSequentialFocusNavigationStartingPoint),
            20 => Some(Action::SetValue),
            21 => Some(Action::ShowContextMenu),
            _ => None,
        }
    }
}

fn action_mask_to_action_vec(mask: u32) -> Vec<Action> {
    let mut actions = Vec::new();
    let mut i = 0;
    while let Some(variant) = Action::n(i) {
        if mask & variant.mask() != 0 {
            actions.push(variant);
        }
        i += 1;
    }
    actions
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "enumn", derive(enumn::N))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[cfg_attr(
    feature = "pyo3",
    pyclass(
        module = "accesskit",
        rename_all = "SCREAMING_SNAKE_CASE",
        eq,
        from_py_object
    )
)]
#[repr(u8)]
pub enum Orientation {
    /// E.g. most toolbars and separators.
    Horizontal,
    /// E.g. menu or combo box.
    Vertical,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "enumn", derive(enumn::N))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[cfg_attr(
    feature = "pyo3",
    pyclass(
        module = "accesskit",
        rename_all = "SCREAMING_SNAKE_CASE",
        eq,
        from_py_object
    )
)]
#[repr(u8)]
pub enum TextDirection {
    LeftToRight,
    RightToLeft,
    TopToBottom,
    BottomToTop,
}

/// Indicates if a form control has invalid input or if a web DOM element has an
/// [`aria-invalid`] attribute.
///
/// [`aria-invalid`]: https://www.w3.org/TR/wai-aria-1.1/#aria-invalid
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "enumn", derive(enumn::N))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[cfg_attr(
    feature = "pyo3",
    pyclass(
        module = "accesskit",
        rename_all = "SCREAMING_SNAKE_CASE",
        eq,
        from_py_object
    )
)]
#[repr(u8)]
pub enum Invalid {
    True,
    Grammar,
    Spelling,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "enumn", derive(enumn::N))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[cfg_attr(
    feature = "pyo3",
    pyclass(
        module = "accesskit",
        rename_all = "SCREAMING_SNAKE_CASE",
        eq,
        from_py_object
    )
)]
#[repr(u8)]
pub enum Toggled {
    False,
    True,
    Mixed,
}

impl From<bool> for Toggled {
    #[inline]
    fn from(b: bool) -> Self {
        match b {
            false => Self::False,
            true => Self::True,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "enumn", derive(enumn::N))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[cfg_attr(
    feature = "pyo3",
    pyclass(
        module = "accesskit",
        rename_all = "SCREAMING_SNAKE_CASE",
        eq,
        from_py_object
    )
)]
#[repr(u8)]
pub enum SortDirection {
    Ascending,
    Descending,
    Other,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "enumn", derive(enumn::N))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[cfg_attr(
    feature = "pyo3",
    pyclass(
        module = "accesskit",
        rename_all = "SCREAMING_SNAKE_CASE",
        eq,
        from_py_object
    )
)]
#[repr(u8)]
pub enum AriaCurrent {
    False,
    True,
    Page,
    Step,
    Location,
    Date,
    Time,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "enumn", derive(enumn::N))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[cfg_attr(
    feature = "pyo3",
    pyclass(
        module = "accesskit",
        rename_all = "SCREAMING_SNAKE_CASE",
        eq,
        from_py_object
    )
)]
#[repr(u8)]
pub enum AutoComplete {
    Inline,
    List,
    Both,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "enumn", derive(enumn::N))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[cfg_attr(
    feature = "pyo3",
    pyclass(
        module = "accesskit",
        rename_all = "SCREAMING_SNAKE_CASE",
        eq,
        from_py_object
    )
)]
#[repr(u8)]
pub enum Live {
    Off,
    Polite,
    Assertive,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "enumn", derive(enumn::N))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[cfg_attr(
    feature = "pyo3",
    pyclass(
        module = "accesskit",
        rename_all = "SCREAMING_SNAKE_CASE",
        eq,
        from_py_object
    )
)]
#[repr(u8)]
pub enum HasPopup {
    Menu,
    Listbox,
    Tree,
    Grid,
    Dialog,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "enumn", derive(enumn::N))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[cfg_attr(
    feature = "pyo3",
    pyclass(
        module = "accesskit",
        rename_all = "SCREAMING_SNAKE_CASE",
        eq,
        from_py_object
    )
)]
#[repr(u8)]
pub enum ListStyle {
    Circle,
    Disc,
    Image,
    Numeric,
    Square,
    /// Language specific ordering (alpha, roman, cjk-ideographic, etc...)
    Other,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "enumn", derive(enumn::N))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[cfg_attr(
    feature = "pyo3",
    pyclass(
        module = "accesskit",
        rename_all = "SCREAMING_SNAKE_CASE",
        eq,
        from_py_object
    )
)]
#[repr(u8)]
pub enum TextAlign {
    Left,
    Right,
    Center,
    Justify,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "enumn", derive(enumn::N))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[cfg_attr(
    feature = "pyo3",
    pyclass(
        module = "accesskit",
        rename_all = "SCREAMING_SNAKE_CASE",
        eq,
        from_py_object
    )
)]
#[repr(u8)]
pub enum VerticalOffset {
    Subscript,
    Superscript,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "enumn", derive(enumn::N))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[cfg_attr(
    feature = "pyo3",
    pyclass(
        module = "accesskit",
        rename_all = "SCREAMING_SNAKE_CASE",
        eq,
        from_py_object
    )
)]
#[repr(u8)]
pub enum TextDecorationStyle {
    Solid,
    Dotted,
    Dashed,
    Double,
    Wavy,
}

pub type NodeIdContent = u64;

/// The stable identity of a [`Node`], unique within the node's tree.
///
/// Each tree (root or subtree) has its own independent ID space. The same
/// `NodeId` value can exist in different trees without conflict. When working
/// with multiple trees, the combination of `NodeId` and [`TreeId`] uniquely
/// identifies a node.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[repr(transparent)]
pub struct NodeId(pub NodeIdContent);

impl From<NodeIdContent> for NodeId {
    #[inline]
    fn from(inner: NodeIdContent) -> Self {
        Self(inner)
    }
}

impl From<NodeId> for NodeIdContent {
    #[inline]
    fn from(outer: NodeId) -> Self {
        outer.0
    }
}

impl fmt::Debug for NodeId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "#{}", self.0)
    }
}

/// The stable identity of a [`Tree`].
///
/// Use [`TreeId::ROOT`] for the main/root tree. For subtrees, use a random
/// UUID (version 4) to avoid collisions between independently created trees.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[repr(transparent)]
pub struct TreeId(pub Uuid);

impl TreeId {
    /// A reserved tree ID for the root tree. This uses a nil UUID.
    pub const ROOT: Self = Self(Uuid::nil());
}

/// Defines a custom action for a UI element.
///
/// For example, a list UI can allow a user to reorder items in the list by dragging the
/// items.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct CustomAction {
    pub id: i32,
    pub description: Box<str>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct TextPosition {
    /// The node's role must be [`Role::TextRun`].
    pub node: NodeId,
    /// The index of an item in [`Node::character_lengths`], or the length
    /// of that slice if the position is at the end of the line.
    pub character_index: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct TextSelection {
    /// The position where the selection started, and which does not change
    /// as the selection is expanded or contracted. If there is no selection
    /// but only a caret, this must be equal to the value of [`TextSelection::focus`].
    /// This is also known as a degenerate selection.
    pub anchor: TextPosition,
    /// The active end of the selection, which changes as the selection
    /// is expanded or contracted, or the position of the caret if there is
    /// no selection.
    pub focus: TextPosition,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize, enumn::N))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[repr(u8)]
enum Flag {
    Hidden,
    Multiselectable,
    Required,
    Visited,
    Busy,
    LiveAtomic,
    Modal,
    TouchTransparent,
    ReadOnly,
    Disabled,
    Italic,
    ClipsChildren,
    IsLineBreakingObject,
    IsPageBreakingObject,
    IsSpellingError,
    IsGrammarError,
    IsSearchMatch,
    IsSuggestion,
}

impl Flag {
    fn mask(self) -> u32 {
        1 << (self as u8)
    }
}

/// A color represented in 8-bit sRGB plus alpha.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
#[repr(C)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub alpha: u8,
}

/// The style and color for a type of text decoration.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
#[repr(C)]
pub struct TextDecoration {
    pub style: TextDecorationStyle,
    pub color: Color,
}

// The following is based on the technique described here:
// https://viruta.org/reducing-memory-consumption-in-librsvg-2.html

#[derive(Clone, Debug, PartialEq)]
enum PropertyValue {
    None,
    NodeIdVec(Vec<NodeId>),
    NodeId(NodeId),
    String(Box<str>),
    F64(f64),
    F32(f32),
    Usize(usize),
    Color(Color),
    TextDecoration(TextDecoration),
    LengthSlice(Box<[u8]>),
    CoordSlice(Box<[f32]>),
    Bool(bool),
    Invalid(Invalid),
    Toggled(Toggled),
    Live(Live),
    TextDirection(TextDirection),
    Orientation(Orientation),
    SortDirection(SortDirection),
    AriaCurrent(AriaCurrent),
    AutoComplete(AutoComplete),
    HasPopup(HasPopup),
    ListStyle(ListStyle),
    TextAlign(TextAlign),
    VerticalOffset(VerticalOffset),
    Affine(Box<Affine>),
    Rect(Rect),
    TextSelection(Box<TextSelection>),
    CustomActionVec(Vec<CustomAction>),
    TreeId(TreeId),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize, enumn::N))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[repr(u8)]
enum PropertyId {
    // NodeIdVec
    Children,
    Controls,
    Details,
    DescribedBy,
    FlowTo,
    LabelledBy,
    Owns,
    RadioGroup,

    // NodeId
    ActiveDescendant,
    ErrorMessage,
    InPageLinkTarget,
    MemberOf,
    NextOnLine,
    PreviousOnLine,
    PopupFor,

    // String
    Label,
    Description,
    Value,
    AccessKey,
    AuthorId,
    ClassName,
    FontFamily,
    HtmlTag,
    InnerHtml,
    KeyboardShortcut,
    Language,
    Placeholder,
    RoleDescription,
    StateDescription,
    Tooltip,
    Url,
    RowIndexText,
    ColumnIndexText,
    BrailleLabel,
    BrailleRoleDescription,

    // f64
    ScrollX,
    ScrollXMin,
    ScrollXMax,
    ScrollY,
    ScrollYMin,
    ScrollYMax,
    NumericValue,
    MinNumericValue,
    MaxNumericValue,
    NumericValueStep,
    NumericValueJump,

    // f32
    FontSize,
    FontWeight,

    // usize
    RowCount,
    ColumnCount,
    RowIndex,
    ColumnIndex,
    RowSpan,
    ColumnSpan,
    Level,
    SizeOfSet,
    PositionInSet,

    // Color
    ColorValue,
    BackgroundColor,
    ForegroundColor,

    // TextDecoration
    Overline,
    Strikethrough,
    Underline,

    // LengthSlice
    CharacterLengths,
    WordStarts,

    // CoordSlice
    CharacterPositions,
    CharacterWidths,

    // bool
    Expanded,
    Selected,

    // Unique enums
    Invalid,
    Toggled,
    Live,
    TextDirection,
    Orientation,
    SortDirection,
    AriaCurrent,
    AutoComplete,
    HasPopup,
    ListStyle,
    TextAlign,
    VerticalOffset,

    // Other
    Transform,
    Bounds,
    TextSelection,
    CustomActions,
    TreeId,

    // This MUST be last.
    Unset,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(transparent)]
struct PropertyIndices([u8; PropertyId::Unset as usize]);

impl Default for PropertyIndices {
    fn default() -> Self {
        Self([PropertyId::Unset as u8; PropertyId::Unset as usize])
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
struct Properties {
    indices: PropertyIndices,
    values: Vec<PropertyValue>,
}

/// A single accessible object. A complete UI is represented as a tree of these.
///
/// For brevity, and to make more of the documentation usable in bindings
/// to other languages, documentation of getter methods is written as if
/// documenting fields in a struct, and such methods are referred to
/// as properties.
#[derive(Clone, Default, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct Node {
    role: Role,
    actions: u32,
    child_actions: u32,
    flags: u32,
    properties: Properties,
}

impl PropertyIndices {
    fn get<'a>(&self, values: &'a [PropertyValue], id: PropertyId) -> &'a PropertyValue {
        let index = self.0[id as usize];
        if index == PropertyId::Unset as u8 {
            &PropertyValue::None
        } else {
            &values[index as usize]
        }
    }
}

impl Properties {
    fn get_mut(&mut self, id: PropertyId, default: PropertyValue) -> &mut PropertyValue {
        let index = self.indices.0[id as usize] as usize;
        if index == PropertyId::Unset as usize {
            self.values.push(default);
            let index = self.values.len() - 1;
            self.indices.0[id as usize] = index as u8;
            &mut self.values[index]
        } else {
            &mut self.values[index]
        }
    }

    fn set(&mut self, id: PropertyId, value: PropertyValue) {
        let index = self.indices.0[id as usize];
        if index == PropertyId::Unset as u8 {
            self.values.push(value);
            self.indices.0[id as usize] = (self.values.len() - 1) as u8;
        } else {
            self.values[index as usize] = value;
        }
    }

    fn clear(&mut self, id: PropertyId) {
        let index = self.indices.0[id as usize];
        if index != PropertyId::Unset as u8 {
            self.values[index as usize] = PropertyValue::None;
        }
    }
}

macro_rules! flag_methods {
    ($($(#[$doc:meta])* ($id:ident, $getter:ident, $setter:ident, $clearer:ident)),+) => {
        impl Node {
            $($(#[$doc])*
            #[inline]
            pub fn $getter(&self) -> bool {
                (self.flags & (Flag::$id).mask()) != 0
            }
            #[inline]
            pub fn $setter(&mut self) {
                self.flags |= (Flag::$id).mask();
            }
            #[inline]
            pub fn $clearer(&mut self) {
                self.flags &= !((Flag::$id).mask());
            })*
            fn debug_flag_properties(&self, fmt: &mut fmt::DebugStruct) {
                $(
                    if self.$getter() {
                        fmt.field(stringify!($getter), &true);
                    }
                )*
            }
        }
        $(#[cfg(test)]
        mod $getter {
            use super::{Node, Role};

            #[test]
            fn getter_should_return_default_value() {
                let node = Node::new(Role::Unknown);
                assert!(!node.$getter());
            }

            #[test]
            fn setter_should_update_the_property() {
                let mut node = Node::new(Role::Unknown);
                node.$setter();
                assert!(node.$getter());
            }

            #[test]
            fn clearer_should_reset_the_property() {
                let mut node = Node::new(Role::Unknown);
                node.$setter();
                node.$clearer();
                assert!(!node.$getter());
            }
        })*
    }
}

macro_rules! option_ref_type_getters {
    ($(($method:ident, $type:ty, $variant:ident)),+) => {
        impl PropertyIndices {
            $(fn $method<'a>(&self, values: &'a [PropertyValue], id: PropertyId) -> Option<&'a $type> {
                match self.get(values, id) {
                    PropertyValue::$variant(value) => Some(value),
                    _ => None,
                }
            })*
        }
    }
}

macro_rules! slice_type_getters {
    ($(($method:ident, $type:ty, $variant:ident)),+) => {
        impl PropertyIndices {
            $(fn $method<'a>(&self, values: &'a [PropertyValue], id: PropertyId) -> &'a [$type] {
                match self.get(values, id) {
                    PropertyValue::$variant(value) => value,
                    _ => &[],
                }
            })*
        }
    }
}

macro_rules! copy_type_getters {
    ($(($method:ident, $type:ty, $variant:ident)),+) => {
        impl PropertyIndices {
            $(fn $method(&self, values: &[PropertyValue], id: PropertyId) -> Option<$type> {
                match self.get(values, id) {
                    PropertyValue::$variant(value) => Some(*value),
                    _ => None,
                }
            })*
        }
    }
}

macro_rules! box_type_setters {
    ($(($method:ident, $type:ty, $variant:ident)),+) => {
        impl Node {
            $(fn $method(&mut self, id: PropertyId, value: impl Into<Box<$type>>) {
                self.properties.set(id, PropertyValue::$variant(value.into()));
            })*
        }
    }
}

macro_rules! copy_type_setters {
    ($(($method:ident, $type:ty, $variant:ident)),+) => {
        impl Node {
            $(fn $method(&mut self, id: PropertyId, value: $type) {
                self.properties.set(id, PropertyValue::$variant(value));
            })*
        }
    }
}

macro_rules! vec_type_methods {
    ($(($type:ty, $variant:ident, $getter:ident, $setter:ident, $pusher:ident)),+) => {
        $(slice_type_getters! {
            ($getter, $type, $variant)
        })*
        impl Node {
            $(fn $setter(&mut self, id: PropertyId, value: impl Into<Vec<$type>>) {
                self.properties.set(id, PropertyValue::$variant(value.into()));
            }
            fn $pusher(&mut self, id: PropertyId, item: $type) {
                if let PropertyValue::$variant(v) = self.properties.get_mut(id, PropertyValue::$variant(Vec::new())) {
                    v.push(item);
                }
            })*
        }
    }
}

macro_rules! property_methods {
    ($($(#[$doc:meta])* ($id:ident, $getter:ident, $type_getter:ident, $getter_result:ty, $setter:ident, $type_setter:ident, $setter_param:ty, $clearer:ident)),+) => {
        impl Node {
            $($(#[$doc])*
            #[inline]
            pub fn $getter(&self) -> $getter_result {
                self.properties.indices.$type_getter(&self.properties.values, PropertyId::$id)
            }
            #[inline]
            pub fn $setter(&mut self, value: $setter_param) {
                self.$type_setter(PropertyId::$id, value);
            }
            #[inline]
            pub fn $clearer(&mut self) {
                self.properties.clear(PropertyId::$id);
            })*
        }
    }
}

macro_rules! vec_property_methods {
    ($($(#[$doc:meta])* ($id:ident, $item_type:ty, $getter:ident, $type_getter:ident, $setter:ident, $type_setter:ident, $pusher:ident, $type_pusher:ident, $clearer:ident)),+) => {
        $(property_methods! {
            $(#[$doc])*
            ($id, $getter, $type_getter, &[$item_type], $setter, $type_setter, impl Into<Vec<$item_type>>, $clearer)
        }
        impl Node {
            #[inline]
            pub fn $pusher(&mut self, item: $item_type) {
                self.$type_pusher(PropertyId::$id, item);
            }
        })*
    }
}

macro_rules! slice_properties_debug_method {
    ($name:ident, [$($getter:ident,)*]) => {
        fn $name(&self, fmt: &mut fmt::DebugStruct) {
            $(
                let value = self.$getter();
                if !value.is_empty() {
                    fmt.field(stringify!($getter), &value);
                }
            )*
        }
    }
}

macro_rules! node_id_vec_property_methods {
    ($($(#[$doc:meta])* ($id:ident, $getter:ident, $setter:ident, $pusher:ident, $clearer:ident)),+) => {
        $(vec_property_methods! {
            $(#[$doc])*
            ($id, NodeId, $getter, get_node_id_vec, $setter, set_node_id_vec, $pusher, push_to_node_id_vec, $clearer)
        })*
        impl Node {
            slice_properties_debug_method! { debug_node_id_vec_properties, [$($getter,)*] }
        }
        $(#[cfg(test)]
        mod $getter {
            use super::{Node, NodeId, Role};

            #[test]
            fn getter_should_return_default_value() {
                let node = Node::new(Role::Unknown);
                assert!(node.$getter().is_empty());
            }
            #[test]
            fn setter_should_update_the_property() {
                let mut node = Node::new(Role::Unknown);
                node.$setter([]);
                assert!(node.$getter().is_empty());
                node.$setter([NodeId(0), NodeId(1)]);
                assert_eq!(node.$getter(), &[NodeId(0), NodeId(1)]);
            }
            #[test]
            fn pusher_should_update_the_property() {
                let mut node = Node::new(Role::Unknown);
                node.$pusher(NodeId(0));
                assert_eq!(node.$getter(), &[NodeId(0)]);
                node.$pusher(NodeId(1));
                assert_eq!(node.$getter(), &[NodeId(0), NodeId(1)]);
            }
            #[test]
            fn clearer_should_reset_the_property() {
                let mut node = Node::new(Role::Unknown);
                node.$setter([NodeId(0)]);
                node.$clearer();
                assert!(node.$getter().is_empty());
            }
        })*
    }
}

macro_rules! option_properties_debug_method {
    ($name:ident, [$($getter:ident,)*]) => {
        fn $name(&self, fmt: &mut fmt::DebugStruct) {
            $(
                if let Some(value) = self.$getter() {
                    fmt.field(stringify!($getter), &value);
                }
            )*
        }
    }
}

macro_rules! node_id_property_methods {
    ($($(#[$doc:meta])* ($id:ident, $getter:ident, $setter:ident, $clearer:ident)),+) => {
        $(property_methods! {
            $(#[$doc])*
            ($id, $getter, get_node_id_property, Option<NodeId>, $setter, set_node_id_property, NodeId, $clearer)
        })*
        impl Node {
            option_properties_debug_method! { debug_node_id_properties, [$($getter,)*] }
        }
        $(#[cfg(test)]
        mod $getter {
            use super::{Node, NodeId, Role};

            #[test]
            fn getter_should_return_default_value() {
                let node = Node::new(Role::Unknown);
                assert!(node.$getter().is_none());
            }
            #[test]
            fn setter_should_update_the_property() {
                let mut node = Node::new(Role::Unknown);
                node.$setter(NodeId(1));
                assert_eq!(node.$getter(), Some(NodeId(1)));
            }
            #[test]
            fn clearer_should_reset_the_property() {
                let mut node = Node::new(Role::Unknown);
                node.$setter(NodeId(1));
                node.$clearer();
                assert!(node.$getter().is_none());
            }
        })*
    }
}

macro_rules! string_property_methods {
    ($($(#[$doc:meta])* ($id:ident, $getter:ident, $setter:ident, $clearer:ident)),+) => {
        $(property_methods! {
            $(#[$doc])*
            ($id, $getter, get_string_property, Option<&str>, $setter, set_string_property, impl Into<Box<str>>, $clearer)
        })*
        impl Node {
            option_properties_debug_method! { debug_string_properties, [$($getter,)*] }
        }
        $(#[cfg(test)]
        mod $getter {
            use super::{Node, Role};

            #[test]
            fn getter_should_return_default_value() {
                let node = Node::new(Role::Unknown);
                assert!(node.$getter().is_none());
            }
            #[test]
            fn setter_should_update_the_property() {
                let mut node = Node::new(Role::Unknown);
                node.$setter("test");
                assert_eq!(node.$getter(), Some("test"));
            }
            #[test]
            fn clearer_should_reset_the_property() {
                let mut node = Node::new(Role::Unknown);
                node.$setter("test");
                node.$clearer();
                assert!(node.$getter().is_none());
            }
        })*
    }
}

macro_rules! f64_property_methods {
    ($($(#[$doc:meta])* ($id:ident, $getter:ident, $setter:ident, $clearer:ident)),+) => {
        $(property_methods! {
            $(#[$doc])*
            ($id, $getter, get_f64_property, Option<f64>, $setter, set_f64_property, f64, $clearer)
        })*
        impl Node {
            option_properties_debug_method! { debug_f64_properties, [$($getter,)*] }
        }
        $(#[cfg(test)]
        mod $getter {
            use super::{Node, Role};

            #[test]
            fn getter_should_return_default_value() {
                let node = Node::new(Role::Unknown);
                assert!(node.$getter().is_none());
            }
            #[test]
            fn setter_should_update_the_property() {
                let mut node = Node::new(Role::Unknown);
                node.$setter(1.0);
                assert_eq!(node.$getter(), Some(1.0));
            }
            #[test]
            fn clearer_should_reset_the_property() {
                let mut node = Node::new(Role::Unknown);
                node.$setter(1.0);
                node.$clearer();
                assert!(node.$getter().is_none());
            }
        })*
    }
}

macro_rules! f32_property_methods {
    ($($(#[$doc:meta])* ($id:ident, $getter:ident, $setter:ident, $clearer:ident)),+) => {
        $(property_methods! {
            $(#[$doc])*
            ($id, $getter, get_f32_property, Option<f32>, $setter, set_f32_property, f32, $clearer)
        })*
        impl Node {
            option_properties_debug_method! { debug_f32_properties, [$($getter,)*] }
        }
        $(#[cfg(test)]
        mod $getter {
            use super::{Node, Role};

            #[test]
            fn getter_should_return_default_value() {
                let node = Node::new(Role::Unknown);
                assert!(node.$getter().is_none());
            }
            #[test]
            fn setter_should_update_the_property() {
                let mut node = Node::new(Role::Unknown);
                node.$setter(1.0);
                assert_eq!(node.$getter(), Some(1.0));
            }
            #[test]
            fn clearer_should_reset_the_property() {
                let mut node = Node::new(Role::Unknown);
                node.$setter(1.0);
                node.$clearer();
                assert!(node.$getter().is_none());
            }
        })*
    }
}

macro_rules! usize_property_methods {
    ($($(#[$doc:meta])* ($id:ident, $getter:ident, $setter:ident, $clearer:ident)),+) => {
        $(property_methods! {
            $(#[$doc])*
            ($id, $getter, get_usize_property, Option<usize>, $setter, set_usize_property, usize, $clearer)
        })*
        impl Node {
            option_properties_debug_method! { debug_usize_properties, [$($getter,)*] }
        }
        $(#[cfg(test)]
        mod $getter {
            use super::{Node, Role};

            #[test]
            fn getter_should_return_default_value() {
                let node = Node::new(Role::Unknown);
                assert!(node.$getter().is_none());
            }
            #[test]
            fn setter_should_update_the_property() {
                let mut node = Node::new(Role::Unknown);
                node.$setter(1);
                assert_eq!(node.$getter(), Some(1));
            }
            #[test]
            fn clearer_should_reset_the_property() {
                let mut node = Node::new(Role::Unknown);
                node.$setter(1);
                node.$clearer();
                assert!(node.$getter().is_none());
            }
        })*
    }
}

macro_rules! color_property_methods {
    ($($(#[$doc:meta])* ($id:ident, $getter:ident, $setter:ident, $clearer:ident)),+) => {
        $(property_methods! {
            $(#[$doc])*
            ($id, $getter, get_color_property, Option<Color>, $setter, set_color_property, Color, $clearer)
        })*
        impl Node {
            option_properties_debug_method! { debug_color_properties, [$($getter,)*] }
        }
        $(#[cfg(test)]
        mod $getter {
            use super::{Color, Node, Role};

            #[test]
            fn getter_should_return_default_value() {
                let node = Node::new(Role::Unknown);
                assert!(node.$getter().is_none());
            }
            #[test]
            fn setter_should_update_the_property() {
                let mut node = Node::new(Role::Unknown);
                node.$setter(Color { red: 255, green: 255, blue: 255, alpha: 255 });
                assert_eq!(node.$getter(), Some(Color { red: 255, green: 255, blue: 255, alpha: 255 }));
            }
            #[test]
            fn clearer_should_reset_the_property() {
                let mut node = Node::new(Role::Unknown);
                node.$setter(Color { red: 255, green: 255, blue: 255, alpha: 255 });
                node.$clearer();
                assert!(node.$getter().is_none());
            }
        })*
    }
}

macro_rules! text_decoration_property_methods {
    ($($(#[$doc:meta])* ($id:ident, $getter:ident, $setter:ident, $clearer:ident)),+) => {
        $(property_methods! {
            $(#[$doc])*
            ($id, $getter, get_text_decoration_property, Option<TextDecoration>, $setter, set_text_decoration_property, TextDecoration, $clearer)
        })*
        impl Node {
            option_properties_debug_method! { debug_text_decoration_properties, [$($getter,)*] }
        }
        $(#[cfg(test)]
        mod $getter {
            use super::{Color, Node, Role, TextDecoration, TextDecorationStyle};

            const TEST_TEXT_DECORATION: TextDecoration = TextDecoration {
                style: TextDecorationStyle::Dotted,
                color: Color {
                    red: 0,
                    green: 0,
                    blue: 0,
                    alpha: 255,
                },
            };

            #[test]
            fn getter_should_return_default_value() {
                let node = Node::new(Role::Unknown);
                assert!(node.$getter().is_none());
            }
            #[test]
            fn setter_should_update_the_property() {
                let mut node = Node::new(Role::Unknown);
                node.$setter(TEST_TEXT_DECORATION);
                assert_eq!(node.$getter(), Some(TEST_TEXT_DECORATION));
            }
            #[test]
            fn clearer_should_reset_the_property() {
                let mut node = Node::new(Role::Unknown);
                node.$setter(TEST_TEXT_DECORATION);
                node.$clearer();
                assert!(node.$getter().is_none());
            }
        })*
    }
}

macro_rules! length_slice_property_methods {
    ($($(#[$doc:meta])* ($id:ident, $getter:ident, $setter:ident, $clearer:ident)),+) => {
        $(property_methods! {
            $(#[$doc])*
            ($id, $getter, get_length_slice_property, &[u8], $setter, set_length_slice_property, impl Into<Box<[u8]>>, $clearer)
        })*
        impl Node {
            slice_properties_debug_method! { debug_length_slice_properties, [$($getter,)*] }
        }
        $(#[cfg(test)]
        mod $getter {
            use super::{Node, Role};

            #[test]
            fn getter_should_return_default_value() {
                let node = Node::new(Role::Unknown);
                assert!(node.$getter().is_empty());
            }
            #[test]
            fn setter_should_update_the_property() {
                let mut node = Node::new(Role::Unknown);
                node.$setter([]);
                assert!(node.$getter().is_empty());
                node.$setter([1, 2]);
                assert_eq!(node.$getter(), &[1, 2]);
            }
            #[test]
            fn clearer_should_reset_the_property() {
                let mut node = Node::new(Role::Unknown);
                node.$setter([1, 2]);
                node.$clearer();
                assert!(node.$getter().is_empty());
            }
        })*
    }
}

macro_rules! coord_slice_property_methods {
    ($($(#[$doc:meta])* ($id:ident, $getter:ident, $setter:ident, $clearer:ident)),+) => {
        $(property_methods! {
            $(#[$doc])*
            ($id, $getter, get_coord_slice_property, Option<&[f32]>, $setter, set_coord_slice_property, impl Into<Box<[f32]>>, $clearer)
        })*
        impl Node {
            option_properties_debug_method! { debug_coord_slice_properties, [$($getter,)*] }
        }
        $(#[cfg(test)]
        mod $getter {
            use super::{Node, Role};

            #[test]
            fn getter_should_return_default_value() {
                let node = Node::new(Role::Unknown);
                assert!(node.$getter().is_none());
            }
            #[test]
            fn setter_should_update_the_property() {
                let mut node = Node::new(Role::Unknown);
                node.$setter([]);
                let expected: Option<&[f32]> = Some(&[]);
                assert_eq!(node.$getter(), expected);
                node.$setter([1.0, 2.0]);
                let expected: Option<&[f32]> = Some(&[1.0, 2.0]);
                assert_eq!(node.$getter(), expected);
            }
            #[test]
            fn clearer_should_reset_the_property() {
                let mut node = Node::new(Role::Unknown);
                node.$setter([1.0, 2.0]);
                node.$clearer();
                assert!(node.$getter().is_none());
            }
        })*
    }
}

macro_rules! bool_property_methods {
    ($($(#[$doc:meta])* ($id:ident, $getter:ident, $setter:ident, $clearer:ident)),+) => {
        $(property_methods! {
            $(#[$doc])*
            ($id, $getter, get_bool_property, Option<bool>, $setter, set_bool_property, bool, $clearer)
        })*
        impl Node {
            option_properties_debug_method! { debug_bool_properties, [$($getter,)*] }
        }
        $(#[cfg(test)]
        mod $getter {
            use super::{Node, Role};

            #[test]
            fn getter_should_return_default_value() {
                let node = Node::new(Role::Unknown);
                assert!(node.$getter().is_none());
            }
            #[test]
            fn setter_should_update_the_property() {
                let mut node = Node::new(Role::Unknown);
                node.$setter(true);
                assert_eq!(node.$getter(), Some(true));
            }
            #[test]
            fn clearer_should_reset_the_property() {
                let mut node = Node::new(Role::Unknown);
                node.$setter(true);
                node.$clearer();
                assert!(node.$getter().is_none());
            }
        })*
    }
}

macro_rules! unique_enum_property_methods {
    ($($(#[$doc:meta])* ($id:ident, $getter:ident, $setter:ident, $clearer:ident, $variant:ident)),+) => {
        impl Node {
            $($(#[$doc])*
            #[inline]
            pub fn $getter(&self) -> Option<$id> {
                match self.properties.indices.get(&self.properties.values, PropertyId::$id) {
                    PropertyValue::$id(value) => Some(*value),
                    _ => None,
                }
            }
            #[inline]
            pub fn $setter(&mut self, value: $id) {
                self.properties.set(PropertyId::$id, PropertyValue::$id(value));
            }
            #[inline]
            pub fn $clearer(&mut self) {
                self.properties.clear(PropertyId::$id);
            })*
            option_properties_debug_method! { debug_unique_enum_properties, [$($getter,)*] }
        }
        $(#[cfg(test)]
        mod $getter {
            use super::{Node, Role};

            #[test]
            fn getter_should_return_default_value() {
                let node = Node::new(Role::Unknown);
                assert!(node.$getter().is_none());
            }
            #[test]
            fn setter_should_update_the_property() {
                let mut node = Node::new(Role::Unknown);
                let variant = super::$id::$variant;
                node.$setter(variant);
                assert_eq!(node.$getter(), Some(variant));
            }
            #[test]
            fn clearer_should_reset_the_property() {
                let mut node = Node::new(Role::Unknown);
                node.$setter(super::$id::$variant);
                node.$clearer();
                assert!(node.$getter().is_none());
            }
        })*
    }
}

impl Node {
    #[inline]
    pub fn new(role: Role) -> Self {
        Self {
            role,
            ..Default::default()
        }
    }
}

impl Node {
    #[inline]
    pub fn role(&self) -> Role {
        self.role
    }
    #[inline]
    pub fn set_role(&mut self, value: Role) {
        self.role = value;
    }

    #[inline]
    pub fn supports_action(&self, action: Action) -> bool {
        (self.actions & action.mask()) != 0
    }
    #[inline]
    pub fn add_action(&mut self, action: Action) {
        self.actions |= action.mask();
    }
    #[inline]
    pub fn remove_action(&mut self, action: Action) {
        self.actions &= !(action.mask());
    }
    #[inline]
    pub fn clear_actions(&mut self) {
        self.actions = 0;
    }

    /// Return whether the specified action is in the set supported on this node's
    /// direct children in the filtered tree.
    #[inline]
    pub fn child_supports_action(&self, action: Action) -> bool {
        (self.child_actions & action.mask()) != 0
    }
    /// Add the specified action to the set supported on this node's direct
    /// children in the filtered tree.
    #[inline]
    pub fn add_child_action(&mut self, action: Action) {
        self.child_actions |= action.mask();
    }
    /// Remove the specified action from the set supported on this node's direct
    /// children in the filtered tree.
    #[inline]
    pub fn remove_child_action(&mut self, action: Action) {
        self.child_actions &= !(action.mask());
    }
    /// Clear the set of actions supported on this node's direct children in the
    /// filtered tree.
    #[inline]
    pub fn clear_child_actions(&mut self) {
        self.child_actions = 0;
    }
}

flag_methods! {
    /// Exclude this node and its descendants from the tree presented to
    /// assistive technologies, and from hit testing.
    (Hidden, is_hidden, set_hidden, clear_hidden),
    (Multiselectable, is_multiselectable, set_multiselectable, clear_multiselectable),
    (Required, is_required, set_required, clear_required),
    (Visited, is_visited, set_visited, clear_visited),
    (Busy, is_busy, set_busy, clear_busy),
    (LiveAtomic, is_live_atomic, set_live_atomic, clear_live_atomic),
    /// If a dialog box is marked as explicitly modal.
    (Modal, is_modal, set_modal, clear_modal),
    /// This element allows touches to be passed through when a screen reader
    /// is in touch exploration mode, e.g. a virtual keyboard normally
    /// behaves this way.
    (TouchTransparent, is_touch_transparent, set_touch_transparent, clear_touch_transparent),
    /// Use for a text widget that allows focus/selection but not input.
    (ReadOnly, is_read_only, set_read_only, clear_read_only),
    /// Use for a control or group of controls that disallows input.
    (Disabled, is_disabled, set_disabled, clear_disabled),
    (Italic, is_italic, set_italic, clear_italic),
    /// Indicates that this node clips its children, i.e. may have
    /// `overflow: hidden` or clip children by default.
    (ClipsChildren, clips_children, set_clips_children, clear_clips_children),
    /// Indicates whether this node causes a hard line-break
    /// (e.g. block level elements, or `<br>`).
    (IsLineBreakingObject, is_line_breaking_object, set_is_line_breaking_object, clear_is_line_breaking_object),
    /// Indicates whether this node causes a page break.
    (IsPageBreakingObject, is_page_breaking_object, set_is_page_breaking_object, clear_is_page_breaking_object),
    (IsSpellingError, is_spelling_error, set_is_spelling_error, clear_is_spelling_error),
    (IsGrammarError, is_grammar_error, set_is_grammar_error, clear_is_grammar_error),
    (IsSearchMatch, is_search_match, set_is_search_match, clear_is_search_match),
    (IsSuggestion, is_suggestion, set_is_suggestion, clear_is_suggestion)
}

option_ref_type_getters! {
    (get_affine_property, Affine, Affine),
    (get_string_property, str, String),
    (get_coord_slice_property, [f32], CoordSlice),
    (get_text_selection_property, TextSelection, TextSelection)
}

slice_type_getters! {
    (get_length_slice_property, u8, LengthSlice)
}

copy_type_getters! {
    (get_rect_property, Rect, Rect),
    (get_node_id_property, NodeId, NodeId),
    (get_f64_property, f64, F64),
    (get_f32_property, f32, F32),
    (get_usize_property, usize, Usize),
    (get_color_property, Color, Color),
    (get_text_decoration_property, TextDecoration, TextDecoration),
    (get_bool_property, bool, Bool),
    (get_tree_id_property, TreeId, TreeId)
}

box_type_setters! {
    (set_affine_property, Affine, Affine),
    (set_string_property, str, String),
    (set_length_slice_property, [u8], LengthSlice),
    (set_coord_slice_property, [f32], CoordSlice),
    (set_text_selection_property, TextSelection, TextSelection)
}

copy_type_setters! {
    (set_rect_property, Rect, Rect),
    (set_node_id_property, NodeId, NodeId),
    (set_f64_property, f64, F64),
    (set_f32_property, f32, F32),
    (set_usize_property, usize, Usize),
    (set_color_property, Color, Color),
    (set_text_decoration_property, TextDecoration, TextDecoration),
    (set_bool_property, bool, Bool),
    (set_tree_id_property, TreeId, TreeId)
}

vec_type_methods! {
    (NodeId, NodeIdVec, get_node_id_vec, set_node_id_vec, push_to_node_id_vec),
    (CustomAction, CustomActionVec, get_custom_action_vec, set_custom_action_vec, push_to_custom_action_vec)
}

node_id_vec_property_methods! {
    (Children, children, set_children, push_child, clear_children),
    (Controls, controls, set_controls, push_controlled, clear_controls),
    (Details, details, set_details, push_detail, clear_details),
    (DescribedBy, described_by, set_described_by, push_described_by, clear_described_by),
    (FlowTo, flow_to, set_flow_to, push_flow_to, clear_flow_to),
    (LabelledBy, labelled_by, set_labelled_by, push_labelled_by, clear_labelled_by),
    /// As with the `aria-owns` property in ARIA, this property should be set
    /// only if the nodes referenced in the property are not descendants
    /// of the owning node in the AccessKit tree. In the common case, where the
    /// owned nodes are direct children or indirect descendants, this property
    /// is unnecessary.
    (Owns, owns, set_owns, push_owned, clear_owns),
    /// On radio buttons this should be set to a list of all of the buttons
    /// in the same group as this one, including this radio button itself.
    (RadioGroup, radio_group, set_radio_group, push_to_radio_group, clear_radio_group)
}

node_id_property_methods! {
    /// For a composite widget such as a listbox, tree, or grid, identifies
    /// the currently active descendant. Used when focus remains on the container
    /// while the active item changes.
    (ActiveDescendant, active_descendant, set_active_descendant, clear_active_descendant),
    (ErrorMessage, error_message, set_error_message, clear_error_message),
    (InPageLinkTarget, in_page_link_target, set_in_page_link_target, clear_in_page_link_target),
    (MemberOf, member_of, set_member_of, clear_member_of),
    (NextOnLine, next_on_line, set_next_on_line, clear_next_on_line),
    (PreviousOnLine, previous_on_line, set_previous_on_line, clear_previous_on_line),
    (PopupFor, popup_for, set_popup_for, clear_popup_for)
}

string_property_methods! {
    /// The label of a control that can have a label. If the label is specified
    /// via the [`Node::labelled_by`] relation, this doesn't need to be set.
    /// Note that the text content of a node with the [`Role::Label`] role
    /// should be provided via [`Node::value`], not this property.
    (Label, label, set_label, clear_label),
    (Description, description, set_description, clear_description),
    (Value, value, set_value, clear_value),
    /// A single character, usually part of this node's name, that can be pressed,
    /// possibly along with a platform-specific modifier, to perform
    /// this node's default action. For menu items, the access key is only active
    /// while the menu is active, in contrast with [`keyboard_shortcut`];
    /// a single menu item may in fact have both properties.
    ///
    /// [`keyboard_shortcut`]: Node::keyboard_shortcut
    (AccessKey, access_key, set_access_key, clear_access_key),
    /// A way for application authors to identify this node for automated
    /// testing purpose. The value must be unique among this node's siblings.
    (AuthorId, author_id, set_author_id, clear_author_id),
    (ClassName, class_name, set_class_name, clear_class_name),
    /// Only present when different from parent.
    (FontFamily, font_family, set_font_family, clear_font_family),
    (HtmlTag, html_tag, set_html_tag, clear_html_tag),
    /// Inner HTML of an element. Only used for a top-level math element,
    /// to support third-party math accessibility products that parse MathML.
    (InnerHtml, inner_html, set_inner_html, clear_inner_html),
    /// A keystroke or sequence of keystrokes, complete with any required
    /// modifiers(s), that will perform this node's default action.
    /// The value of this property should be in a human-friendly format.
    (KeyboardShortcut, keyboard_shortcut, set_keyboard_shortcut, clear_keyboard_shortcut),
    /// An [IETF language tag](https://www.rfc-editor.org/info/bcp47).
    /// Only present when different from parent.
    (Language, language, set_language, clear_language),
    /// If a text input has placeholder text, it should be exposed
    /// through this property rather than [`label`].
    ///
    /// [`label`]: Node::label
    (Placeholder, placeholder, set_placeholder, clear_placeholder),
    /// An optional string that may override an assistive technology's
    /// description of the node's role. Only provide this for custom control types.
    /// The value of this property should be in a human-friendly, localized format.
    (RoleDescription, role_description, set_role_description, clear_role_description),
    /// An optional string that may override an assistive technology's
    /// description of the node's state, replacing default strings such as
    /// "checked" or "selected". Note that most platform accessibility APIs
    /// and assistive technologies do not support this feature.
    (StateDescription, state_description, set_state_description, clear_state_description),
    /// If a node's only accessible name comes from a tooltip, it should be
    /// exposed through this property rather than [`label`].
    ///
    /// [`label`]: Node::label
    (Tooltip, tooltip, set_tooltip, clear_tooltip),
    (Url, url, set_url, clear_url),
    (RowIndexText, row_index_text, set_row_index_text, clear_row_index_text),
    (ColumnIndexText, column_index_text, set_column_index_text, clear_column_index_text),
    (BrailleLabel, braille_label, set_braille_label, clear_braille_label),
    (BrailleRoleDescription, braille_role_description, set_braille_role_description, clear_braille_role_description)
}

f64_property_methods! {
    (ScrollX, scroll_x, set_scroll_x, clear_scroll_x),
    (ScrollXMin, scroll_x_min, set_scroll_x_min, clear_scroll_x_min),
    (ScrollXMax, scroll_x_max, set_scroll_x_max, clear_scroll_x_max),
    (ScrollY, scroll_y, set_scroll_y, clear_scroll_y),
    (ScrollYMin, scroll_y_min, set_scroll_y_min, clear_scroll_y_min),
    (ScrollYMax, scroll_y_max, set_scroll_y_max, clear_scroll_y_max),
    (NumericValue, numeric_value, set_numeric_value, clear_numeric_value),
    (MinNumericValue, min_numeric_value, set_min_numeric_value, clear_min_numeric_value),
    (MaxNumericValue, max_numeric_value, set_max_numeric_value, clear_max_numeric_value),
    (NumericValueStep, numeric_value_step, set_numeric_value_step, clear_numeric_value_step),
    (NumericValueJump, numeric_value_jump, set_numeric_value_jump, clear_numeric_value_jump)
}

f32_property_methods! {
    /// Font size is in pixels.
    (FontSize, font_size, set_font_size, clear_font_size),
    /// Font weight can take on any arbitrary numeric value. Increments of 100 in
    /// range `[0, 900]` represent keywords such as light, normal, bold, etc.
    (FontWeight, font_weight, set_font_weight, clear_font_weight)
}

usize_property_methods! {
    (RowCount, row_count, set_row_count, clear_row_count),
    (ColumnCount, column_count, set_column_count, clear_column_count),
    (RowIndex, row_index, set_row_index, clear_row_index),
    (ColumnIndex, column_index, set_column_index, clear_column_index),
    (RowSpan, row_span, set_row_span, clear_row_span),
    (ColumnSpan, column_span, set_column_span, clear_column_span),
    (Level, level, set_level, clear_level),
    /// For containers like [`Role::ListBox`], specifies the total number of items.
    (SizeOfSet, size_of_set, set_size_of_set, clear_size_of_set),
    /// For items like [`Role::ListBoxOption`], specifies their index in the item list.
    /// This may not exceed the value of [`size_of_set`] as set on the container.
    ///
    /// [`size_of_set`]: Node::size_of_set
    (PositionInSet, position_in_set, set_position_in_set, clear_position_in_set)
}

color_property_methods! {
    /// For [`Role::ColorWell`], specifies the selected color.
    (ColorValue, color_value, set_color_value, clear_color_value),
    /// Background color.
    (BackgroundColor, background_color, set_background_color, clear_background_color),
    /// Foreground color.
    (ForegroundColor, foreground_color, set_foreground_color, clear_foreground_color)
}

text_decoration_property_methods! {
    (Overline, overline, set_overline, clear_overline),
    (Strikethrough, strikethrough, set_strikethrough, clear_strikethrough),
    (Underline, underline, set_underline, clear_underline)
}

length_slice_property_methods! {
    /// For text runs, the length (non-inclusive) of each character
    /// in UTF-8 code units (bytes). The sum of these lengths must equal
    /// the length of [`value`], also in bytes.
    ///
    /// A character is defined as the smallest unit of text that
    /// can be selected. This isn't necessarily a single Unicode
    /// scalar value (code point). This is why AccessKit can't compute
    /// the lengths of the characters from the text itself; this information
    /// must be provided by the text editing implementation.
    ///
    /// If this node is the last text run in a line that ends with a hard
    /// line break, that line break should be included at the end of this
    /// node's value as either a CRLF or LF; in both cases, the line break
    /// should be counted as a single character for the sake of this slice.
    /// When the caret is at the end of such a line, the focus of the text
    /// selection should be on the line break, not after it.
    ///
    /// [`value`]: Node::value
    (CharacterLengths, character_lengths, set_character_lengths, clear_character_lengths),

    /// For text runs, the start index of each word in characters, as defined
    /// in [`character_lengths`]. This list must be sorted.
    ///
    /// If this text run doesn't contain the start of any words, but only
    /// the middle or end of a word, this list must be empty.
    ///
    /// If this text run is the first in the document or the first in a paragraph
    /// (that is, the previous run ends with a newline character), then the first
    /// character of the run is implicitly the start of a word. In this case,
    /// beginning this list with `0` is permitted but not necessary.
    ///
    /// The end of each word is the beginning of the next word; there are no
    /// characters that are not considered part of a word. Trailing whitespace
    /// is typically considered part of the word that precedes it, while
    /// a line's leading whitespace is considered its own word. Whether
    /// punctuation is considered a separate word or part of the preceding
    /// word depends on the particular text editing implementation.
    /// Some editors may have their own definition of a word; for example,
    /// in an IDE, words may correspond to programming language tokens.
    ///
    /// Not all assistive technologies require information about word
    /// boundaries, and not all platform accessibility APIs even expose
    /// this information, but for assistive technologies that do use
    /// this information, users will get unpredictable results if the word
    /// boundaries exposed by the accessibility tree don't match
    /// the editor's behavior. This is why AccessKit does not determine
    /// word boundaries itself.
    ///
    /// [`character_lengths`]: Node::character_lengths
    (WordStarts, word_starts, set_word_starts, clear_word_starts)
}

coord_slice_property_methods! {
    /// For text runs, this is the position of each character within
    /// the node's bounding box, in the direction given by
    /// [`text_direction`], in the coordinate space of this node.
    ///
    /// When present, the length of this slice should be the same as the length
    /// of [`character_lengths`], including for lines that end
    /// with a hard line break. The position of such a line break should
    /// be the position where an end-of-paragraph marker would be rendered.
    ///
    /// This property is optional. Without it, AccessKit can't support some
    /// use cases, such as screen magnifiers that track the caret position
    /// or screen readers that display a highlight cursor. However,
    /// most text functionality still works without this information.
    ///
    /// [`text_direction`]: Node::text_direction
    /// [`character_lengths`]: Node::character_lengths
    (CharacterPositions, character_positions, set_character_positions, clear_character_positions),

    /// For text runs, this is the advance width of each character,
    /// in the direction given by [`text_direction`], in the coordinate
    /// space of this node.
    ///
    /// When present, the length of this slice should be the same as the length
    /// of [`character_lengths`], including for lines that end
    /// with a hard line break. The width of such a line break should
    /// be non-zero if selecting the line break by itself results in
    /// a visible highlight (as in Microsoft Word), or zero if not
    /// (as in Windows Notepad).
    ///
    /// This property is optional. Without it, AccessKit can't support some
    /// use cases, such as screen magnifiers that track the caret position
    /// or screen readers that display a highlight cursor. However,
    /// most text functionality still works without this information.
    ///
    /// [`text_direction`]: Node::text_direction
    /// [`character_lengths`]: Node::character_lengths
    (CharacterWidths, character_widths, set_character_widths, clear_character_widths)
}

bool_property_methods! {
    /// Whether this node is expanded, collapsed, or neither.
    ///
    /// Setting this to `false` means the node is collapsed; omitting it means this state
    /// isn't applicable.
    (Expanded, is_expanded, set_expanded, clear_expanded),

    /// Indicates whether this node is selected or unselected.
    ///
    /// The absence of this flag (as opposed to a `false` setting)
    /// means that the concept of "selected" doesn't apply.
    /// When deciding whether to set the flag to false or omit it,
    /// consider whether it would be appropriate for a screen reader
    /// to announce "not selected". The ambiguity of this flag
    /// in platform accessibility APIs has made extraneous
    /// "not selected" announcements a common annoyance.
    (Selected, is_selected, set_selected, clear_selected)
}

unique_enum_property_methods! {
    (Invalid, invalid, set_invalid, clear_invalid, Grammar),
    (Toggled, toggled, set_toggled, clear_toggled, True),
    (Live, live, set_live, clear_live, Polite),
    (TextDirection, text_direction, set_text_direction, clear_text_direction, RightToLeft),
    (Orientation, orientation, set_orientation, clear_orientation, Vertical),
    (SortDirection, sort_direction, set_sort_direction, clear_sort_direction, Descending),
    (AriaCurrent, aria_current, set_aria_current, clear_aria_current, True),
    (AutoComplete, auto_complete, set_auto_complete, clear_auto_complete, List),
    (HasPopup, has_popup, set_has_popup, clear_has_popup, Menu),
    /// The list style type. Only available on list items.
    (ListStyle, list_style, set_list_style, clear_list_style, Disc),
    (TextAlign, text_align, set_text_align, clear_text_align, Right),
    (VerticalOffset, vertical_offset, set_vertical_offset, clear_vertical_offset, Superscript)
}

property_methods! {
    /// An affine transform to apply to any coordinates within this node
    /// and its descendants, including the [`bounds`] property of this node.
    /// The combined transforms of this node and its ancestors define
    /// the coordinate space of this node. /// This should be `None` if
    /// it would be set to the identity transform, which should be the case
    /// for most nodes.
    ///
    /// AccessKit expects the final transformed coordinates to be relative
    /// to the origin of the tree's container (e.g. window), in physical
    /// pixels, with the y coordinate being top-down.
    ///
    /// [`bounds`]: Node::bounds
    (Transform, transform, get_affine_property, Option<&Affine>, set_transform, set_affine_property, impl Into<Box<Affine>>, clear_transform),

    /// The bounding box of this node, in the node's coordinate space.
    /// This property does not affect the coordinate space of either this node
    /// or its descendants; only the [`transform`] property affects that.
    /// This, along with the recommendation that most nodes should have
    /// a [`transform`] of `None`, implies that the `bounds` property
    /// of most nodes should be in the coordinate space of the nearest ancestor
    /// with a non-`None` [`transform`], or if there is no such ancestor,
    /// the tree's container (e.g. window).
    ///
    /// [`transform`]: Node::transform
    (Bounds, bounds, get_rect_property, Option<Rect>, set_bounds, set_rect_property, Rect, clear_bounds),

    (TextSelection, text_selection, get_text_selection_property, Option<&TextSelection>, set_text_selection, set_text_selection_property, impl Into<Box<TextSelection>>, clear_text_selection),

    /// The tree that this node grafts. When set, this node acts as a graft
    /// point, and its child is the root of the specified subtree.
    ///
    /// A graft node must be created before its subtree is pushed.
    ///
    /// Removing a graft node or clearing this property removes its subtree,
    /// unless a new graft node is provided in the same update.
    (TreeId, tree_id, get_tree_id_property, Option<TreeId>, set_tree_id, set_tree_id_property, TreeId, clear_tree_id)
}

impl Node {
    option_properties_debug_method! { debug_option_properties, [transform, bounds, text_selection, tree_id,] }
}

#[cfg(test)]
mod transform {
    use super::{Affine, Node, Role};

    #[test]
    fn getter_should_return_default_value() {
        let node = Node::new(Role::Unknown);
        assert!(node.transform().is_none());
    }
    #[test]
    fn setter_should_update_the_property() {
        let mut node = Node::new(Role::Unknown);
        node.set_transform(Affine::IDENTITY);
        assert_eq!(node.transform(), Some(&Affine::IDENTITY));
    }
    #[test]
    fn clearer_should_reset_the_property() {
        let mut node = Node::new(Role::Unknown);
        node.set_transform(Affine::IDENTITY);
        node.clear_transform();
        assert!(node.transform().is_none());
    }
}

#[cfg(test)]
mod bounds {
    use super::{Node, Rect, Role};

    #[test]
    fn getter_should_return_default_value() {
        let node = Node::new(Role::Unknown);
        assert!(node.bounds().is_none());
    }
    #[test]
    fn setter_should_update_the_property() {
        let mut node = Node::new(Role::Unknown);
        let value = Rect {
            x0: 0.0,
            y0: 1.0,
            x1: 2.0,
            y1: 3.0,
        };
        node.set_bounds(value);
        assert_eq!(node.bounds(), Some(value));
    }
    #[test]
    fn clearer_should_reset_the_property() {
        let mut node = Node::new(Role::Unknown);
        node.set_bounds(Rect {
            x0: 0.0,
            y0: 1.0,
            x1: 2.0,
            y1: 3.0,
        });
        node.clear_bounds();
        assert!(node.bounds().is_none());
    }
}

#[cfg(test)]
mod text_selection {
    use super::{Node, NodeId, Role, TextPosition, TextSelection};

    #[test]
    fn getter_should_return_default_value() {
        let node = Node::new(Role::Unknown);
        assert!(node.text_selection().is_none());
    }
    #[test]
    fn setter_should_update_the_property() {
        let mut node = Node::new(Role::Unknown);
        let value = TextSelection {
            anchor: TextPosition {
                node: NodeId(0),
                character_index: 0,
            },
            focus: TextPosition {
                node: NodeId(0),
                character_index: 2,
            },
        };
        node.set_text_selection(value);
        assert_eq!(node.text_selection(), Some(&value));
    }
    #[test]
    fn clearer_should_reset_the_property() {
        let mut node = Node::new(Role::Unknown);
        node.set_text_selection(TextSelection {
            anchor: TextPosition {
                node: NodeId(0),
                character_index: 0,
            },
            focus: TextPosition {
                node: NodeId(0),
                character_index: 2,
            },
        });
        node.clear_text_selection();
        assert!(node.text_selection().is_none());
    }
}

#[cfg(test)]
mod tree_id {
    use super::{Node, Role, TreeId, Uuid};

    #[test]
    fn getter_should_return_default_value() {
        let node = Node::new(Role::GenericContainer);
        assert!(node.tree_id().is_none());
    }
    #[test]
    fn setter_should_update_the_property() {
        let mut node = Node::new(Role::GenericContainer);
        let value = TreeId(Uuid::nil());
        node.set_tree_id(value);
        assert_eq!(node.tree_id(), Some(value));
    }
    #[test]
    fn clearer_should_reset_the_property() {
        let mut node = Node::new(Role::GenericContainer);
        node.set_tree_id(TreeId(Uuid::nil()));
        node.clear_tree_id();
        assert!(node.tree_id().is_none());
    }
}

vec_property_methods! {
    (CustomActions, CustomAction, custom_actions, get_custom_action_vec, set_custom_actions, set_custom_action_vec, push_custom_action, push_to_custom_action_vec, clear_custom_actions)
}

#[cfg(test)]
mod custom_actions {
    use super::{CustomAction, Node, Role};
    use core::slice;

    #[test]
    fn getter_should_return_default_value() {
        let node = Node::new(Role::Unknown);
        assert!(node.custom_actions().is_empty());
    }
    #[test]
    fn setter_should_update_the_property() {
        let mut node = Node::new(Role::Unknown);
        let value = alloc::vec![
            CustomAction {
                id: 0,
                description: "first test action".into(),
            },
            CustomAction {
                id: 1,
                description: "second test action".into(),
            },
        ];
        node.set_custom_actions(value.clone());
        assert_eq!(node.custom_actions(), value);
    }
    #[test]
    fn pusher_should_update_the_property() {
        let mut node = Node::new(Role::Unknown);
        let first_action = CustomAction {
            id: 0,
            description: "first test action".into(),
        };
        let second_action = CustomAction {
            id: 1,
            description: "second test action".into(),
        };
        node.push_custom_action(first_action.clone());
        assert_eq!(node.custom_actions(), slice::from_ref(&first_action));
        node.push_custom_action(second_action.clone());
        assert_eq!(node.custom_actions(), &[first_action, second_action]);
    }
    #[test]
    fn clearer_should_reset_the_property() {
        let mut node = Node::new(Role::Unknown);
        node.set_custom_actions([CustomAction {
            id: 0,
            description: "test action".into(),
        }]);
        node.clear_custom_actions();
        assert!(node.custom_actions().is_empty());
    }
}

impl fmt::Debug for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut fmt = f.debug_struct("Node");

        fmt.field("role", &self.role());

        let supported_actions = action_mask_to_action_vec(self.actions);
        if !supported_actions.is_empty() {
            fmt.field("actions", &supported_actions);
        }

        let child_supported_actions = action_mask_to_action_vec(self.child_actions);
        if !child_supported_actions.is_empty() {
            fmt.field("child_actions", &child_supported_actions);
        }

        self.debug_flag_properties(&mut fmt);
        self.debug_node_id_vec_properties(&mut fmt);
        self.debug_node_id_properties(&mut fmt);
        self.debug_string_properties(&mut fmt);
        self.debug_f64_properties(&mut fmt);
        self.debug_f32_properties(&mut fmt);
        self.debug_usize_properties(&mut fmt);
        self.debug_color_properties(&mut fmt);
        self.debug_text_decoration_properties(&mut fmt);
        self.debug_length_slice_properties(&mut fmt);
        self.debug_coord_slice_properties(&mut fmt);
        self.debug_bool_properties(&mut fmt);
        self.debug_unique_enum_properties(&mut fmt);
        self.debug_option_properties(&mut fmt);

        let custom_actions = self.custom_actions();
        if !custom_actions.is_empty() {
            fmt.field("custom_actions", &custom_actions);
        }

        fmt.finish()
    }
}

#[cfg(feature = "serde")]
macro_rules! serialize_property {
    ($self:ident, $map:ident, $index:ident, $id:ident, { $($variant:ident),+ }) => {
        match &$self.values[$index as usize] {
            PropertyValue::None => (),
            $(PropertyValue::$variant(value) => {
                $map.serialize_entry(&$id, &value)?;
            })*
        }
    }
}

#[cfg(feature = "serde")]
macro_rules! deserialize_property {
    ($props:ident, $map:ident, $key:ident, { $($type:ident { $($id:ident),+ }),+ }) => {
        match $key {
            $($(PropertyId::$id => {
                let value = $map.next_value()?;
                $props.set(PropertyId::$id, PropertyValue::$type(value));
            })*)*
            PropertyId::Unset => {
                let _ = $map.next_value::<IgnoredAny>()?;
            }
        }
    }
}

#[cfg(feature = "serde")]
impl Serialize for Properties {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut len = 0;
        for value in &*self.values {
            if !matches!(*value, PropertyValue::None) {
                len += 1;
            }
        }
        let mut map = serializer.serialize_map(Some(len))?;
        for (id, index) in self.indices.0.iter().copied().enumerate() {
            if index == PropertyId::Unset as u8 {
                continue;
            }
            let id = PropertyId::n(id as _).unwrap();
            serialize_property!(self, map, index, id, {
                NodeIdVec,
                NodeId,
                String,
                F64,
                F32,
                Usize,
                Color,
                TextDecoration,
                LengthSlice,
                CoordSlice,
                Bool,
                Invalid,
                Toggled,
                Live,
                TextDirection,
                Orientation,
                SortDirection,
                AriaCurrent,
                AutoComplete,
                HasPopup,
                ListStyle,
                TextAlign,
                VerticalOffset,
                Affine,
                Rect,
                TextSelection,
                CustomActionVec,
                TreeId
            });
        }
        map.end()
    }
}

#[cfg(feature = "serde")]
struct PropertiesVisitor;

#[cfg(feature = "serde")]
impl<'de> Visitor<'de> for PropertiesVisitor {
    type Value = Properties;

    #[inline]
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("property map")
    }

    fn visit_map<V>(self, mut map: V) -> Result<Self::Value, V::Error>
    where
        V: MapAccess<'de>,
    {
        let mut props = Properties::default();
        while let Some(id) = map.next_key()? {
            deserialize_property!(props, map, id, {
                NodeIdVec {
                    Children,
                    Controls,
                    Details,
                    DescribedBy,
                    FlowTo,
                    LabelledBy,
                    Owns,
                    RadioGroup
                },
                NodeId {
                    ActiveDescendant,
                    ErrorMessage,
                    InPageLinkTarget,
                    MemberOf,
                    NextOnLine,
                    PreviousOnLine,
                    PopupFor
                },
                String {
                    Label,
                    Description,
                    Value,
                    AccessKey,
                    AuthorId,
                    ClassName,
                    FontFamily,
                    HtmlTag,
                    InnerHtml,
                    KeyboardShortcut,
                    Language,
                    Placeholder,
                    RoleDescription,
                    StateDescription,
                    Tooltip,
                    Url,
                    RowIndexText,
                    ColumnIndexText,
                    BrailleLabel,
                    BrailleRoleDescription
                },
                F64 {
                    ScrollX,
                    ScrollXMin,
                    ScrollXMax,
                    ScrollY,
                    ScrollYMin,
                    ScrollYMax,
                    NumericValue,
                    MinNumericValue,
                    MaxNumericValue,
                    NumericValueStep,
                    NumericValueJump
                },
                F32 {
                    FontSize,
                    FontWeight
                },
                Usize {
                    RowCount,
                    ColumnCount,
                    RowIndex,
                    ColumnIndex,
                    RowSpan,
                    ColumnSpan,
                    Level,
                    SizeOfSet,
                    PositionInSet
                },
                Color {
                    ColorValue,
                    BackgroundColor,
                    ForegroundColor
                },
                TextDecoration {
                    Overline,
                    Strikethrough,
                    Underline
                },
                LengthSlice {
                    CharacterLengths,
                    WordStarts
                },
                CoordSlice {
                    CharacterPositions,
                    CharacterWidths
                },
                Bool {
                    Expanded,
                    Selected
                },
                Invalid { Invalid },
                Toggled { Toggled },
                Live { Live },
                TextDirection { TextDirection },
                Orientation { Orientation },
                SortDirection { SortDirection },
                AriaCurrent { AriaCurrent },
                AutoComplete { AutoComplete },
                HasPopup { HasPopup },
                ListStyle { ListStyle },
                TextAlign { TextAlign },
                VerticalOffset { VerticalOffset },
                Affine { Transform },
                Rect { Bounds },
                TextSelection { TextSelection },
                CustomActionVec { CustomActions },
                TreeId { TreeId }
            });
        }

        Ok(props)
    }
}

#[cfg(feature = "serde")]
impl<'de> Deserialize<'de> for Properties {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_map(PropertiesVisitor)
    }
}

#[cfg(feature = "schemars")]
macro_rules! add_schema_property {
    ($gen:ident, $properties:ident, $enum_value:expr, $type:ty) => {{
        let name = format!("{:?}", $enum_value);
        let name = name[..1].to_ascii_lowercase() + &name[1..];
        let subschema = $gen.subschema_for::<$type>();
        $properties.insert(name, SchemaValue::from(subschema));
    }};
}

#[cfg(feature = "schemars")]
macro_rules! add_properties_to_schema {
    ($gen:ident, $properties:ident, { $($type:ty { $($id:ident),+ }),+ }) => {
        $($(add_schema_property!($gen, $properties, PropertyId::$id, $type);)*)*
    }
}

#[cfg(feature = "schemars")]
impl JsonSchema for Properties {
    #[inline]
    fn schema_name() -> Cow<'static, str> {
        "Properties".into()
    }

    fn json_schema(generator: &mut SchemaGenerator) -> Schema {
        let mut properties = SchemaMap::<String, SchemaValue>::new();
        add_properties_to_schema!(generator, properties, {
            Vec<NodeId> {
                Children,
                Controls,
                Details,
                DescribedBy,
                FlowTo,
                LabelledBy,
                Owns,
                RadioGroup
            },
            NodeId {
                ActiveDescendant,
                ErrorMessage,
                InPageLinkTarget,
                MemberOf,
                NextOnLine,
                PreviousOnLine,
                PopupFor
            },
            Box<str> {
                Label,
                Description,
                Value,
                AccessKey,
                AuthorId,
                ClassName,
                FontFamily,
                HtmlTag,
                InnerHtml,
                KeyboardShortcut,
                Language,
                Placeholder,
                RoleDescription,
                StateDescription,
                Tooltip,
                Url,
                RowIndexText,
                ColumnIndexText,
                BrailleLabel,
                BrailleRoleDescription
            },
            f64 {
                ScrollX,
                ScrollXMin,
                ScrollXMax,
                ScrollY,
                ScrollYMin,
                ScrollYMax,
                NumericValue,
                MinNumericValue,
                MaxNumericValue,
                NumericValueStep,
                NumericValueJump
            },
            f32 {
                FontSize,
                FontWeight
            },
            usize {
                RowCount,
                ColumnCount,
                RowIndex,
                ColumnIndex,
                RowSpan,
                ColumnSpan,
                Level,
                SizeOfSet,
                PositionInSet
            },
            Color {
                ColorValue,
                BackgroundColor,
                ForegroundColor
            },
            TextDecoration {
                Overline,
                Strikethrough,
                Underline
            },
            Box<[u8]> {
                CharacterLengths,
                WordStarts
            },
            Box<[f32]> {
                CharacterPositions,
                CharacterWidths
            },
            bool {
                Expanded,
                Selected
            },
            Invalid { Invalid },
            Toggled { Toggled },
            Live { Live },
            TextDirection { TextDirection },
            Orientation { Orientation },
            SortDirection { SortDirection },
            AriaCurrent { AriaCurrent },
            AutoComplete { AutoComplete },
            HasPopup { HasPopup },
            ListStyle { ListStyle },
            TextAlign { TextAlign },
            VerticalOffset { VerticalOffset },
            Affine { Transform },
            Rect { Bounds },
            TextSelection { TextSelection },
            Vec<CustomAction> { CustomActions }
        });
        json_schema!({
            "type": "object",
            "properties": properties
        })
    }
}

/// The data associated with an accessibility tree that's global to the
/// tree and not associated with any particular node.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct Tree {
    /// The identifier of the tree's root node.
    pub root: NodeId,
    /// The name of the UI toolkit in use.
    pub toolkit_name: Option<String>,
    /// The version of the UI toolkit.
    pub toolkit_version: Option<String>,
}

impl Tree {
    #[inline]
    pub fn new(root: NodeId) -> Tree {
        Tree {
            root,
            toolkit_name: None,
            toolkit_version: None,
        }
    }
}

/// A serializable representation of an atomic change to a [`Tree`].
///
/// The sender and receiver must be in sync; the update is only meant
/// to bring the tree from a specific previous state into its next state.
/// Trying to apply it to the wrong tree should immediately panic.
///
/// Note that for performance, an update should only include nodes that are
/// new or changed. AccessKit platform adapters will avoid raising extraneous
/// events for nodes that have not changed since the previous update,
/// but there is still a cost in processing these nodes and replacing
/// the previous instances.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct TreeUpdate {
    /// Zero or more new or updated nodes. Order doesn't matter.
    ///
    /// Each node in this list will overwrite any existing node with the same ID.
    /// This means that when updating a node, fields that are unchanged
    /// from the previous version must still be set to the same values
    /// as before.
    ///
    /// It is an error for any node in this list to not be either the root
    /// or a child of another node. For nodes other than the root, the parent
    /// must be either an unchanged node already in the tree, or another node
    /// in this list.
    ///
    /// To add a child to the tree, the list must include both the child
    /// and an updated version of the parent with the child's ID added to
    /// [`Node::children`].
    ///
    /// To remove a child and all of its descendants, this list must include
    /// an updated version of the parent node with the child's ID removed
    /// from [`Node::children`]. Neither the child nor any of its descendants
    /// may be included in this list.
    pub nodes: Vec<(NodeId, Node)>,

    /// Rarely updated information about the tree as a whole. This may be omitted
    /// if it has not changed since the previous update, but providing the same
    /// information again is also allowed. This is required when initializing
    /// a tree.
    pub tree: Option<Tree>,

    /// The identifier of the tree that this update applies to.
    ///
    /// Use [`TreeId::ROOT`] for the main/root tree. For subtrees, use a unique
    /// [`TreeId`] that identifies the subtree.
    ///
    /// When updating a subtree (non-ROOT tree_id):
    /// - A graft node with [`Node::tree_id`] set to this tree's ID must already
    ///   exist in the parent tree before the first subtree update.
    /// - The first update for a subtree must include [`tree`](Self::tree) data.
    pub tree_id: TreeId,

    /// The node within this tree that has keyboard focus when the native
    /// host (e.g. window) has focus. If no specific node within the tree
    /// has keyboard focus, this must be set to the root. The latest focus state
    /// must be provided with every tree update, even if the focus state
    /// didn't change in a given update.
    ///
    /// For subtrees, this specifies which node has focus when the subtree
    /// itself is focused (i.e., when focus is on the graft node in the parent
    /// tree).
    pub focus: NodeId,
}

/// The amount by which to scroll in the direction specified by one of the
/// `Scroll` actions.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[cfg_attr(
    feature = "pyo3",
    pyclass(module = "accesskit", rename_all = "SCREAMING_SNAKE_CASE", eq)
)]
#[repr(u8)]
pub enum ScrollUnit {
    /// A single item of a list, line of text (for vertical scrolling),
    /// character (for horizontal scrolling), or an approximation of
    /// one of these.
    Item,
    /// The amount of content that fits in the viewport.
    Page,
}

/// A suggestion about where the node being scrolled into view should be
/// positioned relative to the edges of the scrollable container.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[cfg_attr(
    feature = "pyo3",
    pyclass(module = "accesskit", rename_all = "SCREAMING_SNAKE_CASE", eq)
)]
#[repr(u8)]
pub enum ScrollHint {
    TopLeft,
    BottomRight,
    TopEdge,
    BottomEdge,
    LeftEdge,
    RightEdge,
}

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[repr(C)]
pub enum ActionData {
    CustomAction(i32),
    Value(Box<str>),
    NumericValue(f64),
    ScrollUnit(ScrollUnit),
    /// Optional suggestion for [`Action::ScrollIntoView`], specifying
    /// the preferred position of the target node relative to the scrollable
    /// container's viewport.
    ScrollHint(ScrollHint),
    /// Target for [`Action::ScrollToPoint`], in platform-native coordinates
    /// relative to the origin of the tree's container (e.g. window).
    ScrollToPoint(Point),
    /// Target for [`Action::SetScrollOffset`], in the coordinate space
    /// of the action's target node.
    SetScrollOffset(Point),
    SetTextSelection(TextSelection),
}

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct ActionRequest {
    pub action: Action,
    pub target_tree: TreeId,
    pub target_node: NodeId,
    pub data: Option<ActionData>,
}

/// Handles activation of the application's accessibility implementation.
pub trait ActivationHandler {
    /// Requests a [`TreeUpdate`] with a full tree. If the application
    /// can generate the tree synchronously within this method call,
    /// it should do so and return the [`TreeUpdate`]. Otherwise,
    /// it must send the update to the platform adapter asynchronously,
    /// no later than the next display refresh, even if a frame would not
    /// normally be rendered due to user input or other activity.
    /// The application should not return or send a placeholder [`TreeUpdate`];
    /// the platform adapter will provide one if necessary until the real
    /// tree is sent.
    ///
    /// The primary purpose of this method is to allow the application
    /// to lazily initialize its accessibility implementation. However,
    /// this method may be called consecutively without any call to
    /// [`DeactivationHandler::deactivate_accessibility`]; this typically happens
    /// if the platform adapter merely forwards tree updates to assistive
    /// technologies without maintaining any state. A call to this method
    /// must always generate a [`TreeUpdate`] with a full tree, even if
    /// the application normally sends incremental updates.
    ///
    /// The thread on which this method is called is platform-dependent.
    /// Refer to the platform adapter documentation for more details.
    fn request_initial_tree(&mut self) -> Option<TreeUpdate>;
}

/// Handles requests from assistive technologies or other clients.
pub trait ActionHandler {
    /// Perform the requested action. If the requested action is not supported,
    /// this method must do nothing.
    ///
    /// The thread on which this method is called is platform-dependent.
    /// Refer to the platform adapter documentation for more details.
    ///
    /// This method may queue the request and handle it asynchronously.
    /// This behavior is preferred over blocking, e.g. when dispatching
    /// the request to another thread.
    fn do_action(&mut self, request: ActionRequest);
}

/// Handles deactivation of the application's accessibility implementation.
pub trait DeactivationHandler {
    /// Deactivate the application's accessibility implementation and drop any
    /// associated data that can be reconstructed later. After this method
    /// is called, if an accessibility tree is needed again, the platform
    /// adapter will call [`ActivationHandler::request_initial_tree`] again.
    ///
    /// The thread on which this method is called is platform-dependent.
    /// Refer to the platform adapter documentation for more details.
    fn deactivate_accessibility(&mut self);
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::format;

    #[test]
    fn u64_should_be_convertible_to_node_id() {
        assert_eq!(NodeId::from(0u64), NodeId(0));
        assert_eq!(NodeId::from(1u64), NodeId(1));
    }

    #[test]
    fn node_id_should_be_convertible_to_u64() {
        assert_eq!(u64::from(NodeId(0)), 0u64);
        assert_eq!(u64::from(NodeId(1)), 1u64);
    }

    #[test]
    fn node_id_should_have_debug_repr() {
        assert_eq!(&format!("{:?}", NodeId(0)), "#0");
        assert_eq!(&format!("{:?}", NodeId(1)), "#1");
    }

    #[test]
    fn action_n_should_return_the_corresponding_variant() {
        assert_eq!(Action::n(0), Some(Action::Click));
        assert_eq!(Action::n(1), Some(Action::Focus));
        assert_eq!(Action::n(2), Some(Action::Blur));
        assert_eq!(Action::n(3), Some(Action::Collapse));
        assert_eq!(Action::n(4), Some(Action::Expand));
        assert_eq!(Action::n(5), Some(Action::CustomAction));
        assert_eq!(Action::n(6), Some(Action::Decrement));
        assert_eq!(Action::n(7), Some(Action::Increment));
        assert_eq!(Action::n(8), Some(Action::HideTooltip));
        assert_eq!(Action::n(9), Some(Action::ShowTooltip));
        assert_eq!(Action::n(10), Some(Action::ReplaceSelectedText));
        assert_eq!(Action::n(11), Some(Action::ScrollDown));
        assert_eq!(Action::n(12), Some(Action::ScrollLeft));
        assert_eq!(Action::n(13), Some(Action::ScrollRight));
        assert_eq!(Action::n(14), Some(Action::ScrollUp));
        assert_eq!(Action::n(15), Some(Action::ScrollIntoView));
        assert_eq!(Action::n(16), Some(Action::ScrollToPoint));
        assert_eq!(Action::n(17), Some(Action::SetScrollOffset));
        assert_eq!(Action::n(18), Some(Action::SetTextSelection));
        assert_eq!(
            Action::n(19),
            Some(Action::SetSequentialFocusNavigationStartingPoint)
        );
        assert_eq!(Action::n(20), Some(Action::SetValue));
        assert_eq!(Action::n(21), Some(Action::ShowContextMenu));
        assert_eq!(Action::n(22), None);
    }

    #[test]
    fn empty_action_mask_should_be_converted_to_empty_vec() {
        assert_eq!(
            Vec::<Action>::new(),
            action_mask_to_action_vec(Node::new(Role::Unknown).actions)
        );
    }

    #[test]
    fn action_mask_should_be_convertible_to_vec() {
        let mut node = Node::new(Role::Unknown);
        node.add_action(Action::Click);
        assert_eq!(
            &[Action::Click],
            action_mask_to_action_vec(node.actions).as_slice()
        );

        let mut node = Node::new(Role::Unknown);
        node.add_action(Action::ShowContextMenu);
        assert_eq!(
            &[Action::ShowContextMenu],
            action_mask_to_action_vec(node.actions).as_slice()
        );

        let mut node = Node::new(Role::Unknown);
        node.add_action(Action::Click);
        node.add_action(Action::ShowContextMenu);
        assert_eq!(
            &[Action::Click, Action::ShowContextMenu],
            action_mask_to_action_vec(node.actions).as_slice()
        );

        let mut node = Node::new(Role::Unknown);
        node.add_action(Action::Focus);
        node.add_action(Action::Blur);
        node.add_action(Action::Collapse);
        assert_eq!(
            &[Action::Focus, Action::Blur, Action::Collapse],
            action_mask_to_action_vec(node.actions).as_slice()
        );
    }

    #[test]
    fn new_node_should_have_user_provided_role() {
        let node = Node::new(Role::Button);
        assert_eq!(node.role(), Role::Button);
    }

    #[test]
    fn node_role_setter_should_update_the_role() {
        let mut node = Node::new(Role::Button);
        node.set_role(Role::CheckBox);
        assert_eq!(node.role(), Role::CheckBox);
    }

    macro_rules! assert_absent_action {
        ($node:ident, $action:ident) => {
            assert!(!$node.supports_action(Action::$action));
            assert!(!$node.child_supports_action(Action::$action));
        };
    }

    #[test]
    fn new_node_should_not_support_anyaction() {
        let node = Node::new(Role::Unknown);
        assert_absent_action!(node, Click);
        assert_absent_action!(node, Focus);
        assert_absent_action!(node, Blur);
        assert_absent_action!(node, Collapse);
        assert_absent_action!(node, Expand);
        assert_absent_action!(node, CustomAction);
        assert_absent_action!(node, Decrement);
        assert_absent_action!(node, Increment);
        assert_absent_action!(node, HideTooltip);
        assert_absent_action!(node, ShowTooltip);
        assert_absent_action!(node, ReplaceSelectedText);
        assert_absent_action!(node, ScrollDown);
        assert_absent_action!(node, ScrollLeft);
        assert_absent_action!(node, ScrollRight);
        assert_absent_action!(node, ScrollUp);
        assert_absent_action!(node, ScrollIntoView);
        assert_absent_action!(node, ScrollToPoint);
        assert_absent_action!(node, SetScrollOffset);
        assert_absent_action!(node, SetTextSelection);
        assert_absent_action!(node, SetSequentialFocusNavigationStartingPoint);
        assert_absent_action!(node, SetValue);
        assert_absent_action!(node, ShowContextMenu);
    }

    #[test]
    fn node_add_action_should_add_the_action() {
        let mut node = Node::new(Role::Unknown);
        node.add_action(Action::Focus);
        assert!(node.supports_action(Action::Focus));
        node.add_action(Action::Blur);
        assert!(node.supports_action(Action::Blur));
    }

    #[test]
    fn node_add_child_action_should_add_the_action() {
        let mut node = Node::new(Role::Unknown);
        node.add_child_action(Action::Focus);
        assert!(node.child_supports_action(Action::Focus));
        node.add_child_action(Action::Blur);
        assert!(node.child_supports_action(Action::Blur));
    }

    #[test]
    fn node_add_action_should_do_nothing_if_the_action_is_already_supported() {
        let mut node = Node::new(Role::Unknown);
        node.add_action(Action::Focus);
        node.add_action(Action::Focus);
        assert!(node.supports_action(Action::Focus));
    }

    #[test]
    fn node_add_child_action_should_do_nothing_if_the_action_is_already_supported() {
        let mut node = Node::new(Role::Unknown);
        node.add_child_action(Action::Focus);
        node.add_child_action(Action::Focus);
        assert!(node.child_supports_action(Action::Focus));
    }

    #[test]
    fn node_remove_action_should_remove_the_action() {
        let mut node = Node::new(Role::Unknown);
        node.add_action(Action::Blur);
        node.remove_action(Action::Blur);
        assert!(!node.supports_action(Action::Blur));
    }

    #[test]
    fn node_remove_child_action_should_remove_the_action() {
        let mut node = Node::new(Role::Unknown);
        node.add_child_action(Action::Blur);
        node.remove_child_action(Action::Blur);
        assert!(!node.child_supports_action(Action::Blur));
    }

    #[test]
    fn node_clear_actions_should_remove_all_actions() {
        let mut node = Node::new(Role::Unknown);
        node.add_action(Action::Focus);
        node.add_action(Action::Blur);
        node.clear_actions();
        assert!(!node.supports_action(Action::Focus));
        assert!(!node.supports_action(Action::Blur));
    }

    #[test]
    fn node_clear_child_actions_should_remove_all_actions() {
        let mut node = Node::new(Role::Unknown);
        node.add_child_action(Action::Focus);
        node.add_child_action(Action::Blur);
        node.clear_child_actions();
        assert!(!node.child_supports_action(Action::Focus));
        assert!(!node.child_supports_action(Action::Blur));
    }

    #[test]
    fn node_should_have_debug_repr() {
        let mut node = Node::new(Role::Unknown);
        node.add_action(Action::Click);
        node.add_action(Action::Focus);
        node.add_child_action(Action::ScrollIntoView);
        node.set_hidden();
        node.set_multiselectable();
        node.set_children([NodeId(0), NodeId(1)]);
        node.set_active_descendant(NodeId(2));
        node.push_custom_action(CustomAction {
            id: 0,
            description: "test action".into(),
        });

        assert_eq!(
            &format!("{node:?}"),
            r#"Node { role: Unknown, actions: [Click, Focus], child_actions: [ScrollIntoView], is_hidden: true, is_multiselectable: true, children: [#0, #1], active_descendant: #2, custom_actions: [CustomAction { id: 0, description: "test action" }] }"#
        );
    }

    #[test]
    fn new_tree_should_have_root_id() {
        let tree = Tree::new(NodeId(1));
        assert_eq!(tree.root, NodeId(1));
        assert_eq!(tree.toolkit_name, None);
        assert_eq!(tree.toolkit_version, None);
    }
}
