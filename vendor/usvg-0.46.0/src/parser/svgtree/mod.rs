// Copyright 2021 the Resvg Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

use std::collections::HashMap;
use std::num::NonZeroU32;
use std::str::FromStr;

#[rustfmt::skip] mod names;
mod parse;
mod text;

use tiny_skia_path::Transform;

use crate::{
    BlendMode, ImageRendering, Opacity, ShapeRendering, SpreadMethod, TextRendering, Units,
    Visibility,
};
pub use names::{AId, EId};

/// An SVG tree container.
///
/// Contains only element and text nodes.
/// Text nodes are present only inside the `text` element.
pub struct Document<'input> {
    nodes: Vec<NodeData>,
    attrs: Vec<Attribute<'input>>,
    links: HashMap<String, NodeId>,
}

impl<'input> Document<'input> {
    /// Returns the root node.
    #[inline]
    pub fn root<'a>(&'a self) -> SvgNode<'a, 'input> {
        SvgNode {
            id: NodeId::new(0),
            d: &self.nodes[0],
            doc: self,
        }
    }

    /// Returns the root element.
    #[inline]
    pub fn root_element<'a>(&'a self) -> SvgNode<'a, 'input> {
        // `unwrap` is safe, because `Document` is guarantee to have at least one element.
        self.root().first_element_child().unwrap()
    }

    /// Returns an iterator over document's descendant nodes.
    ///
    /// Shorthand for `doc.root().descendants()`.
    #[inline]
    pub fn descendants<'a>(&'a self) -> Descendants<'a, 'input> {
        self.root().descendants()
    }

    /// Returns an element by ID.
    ///
    /// Unlike the [`Descendants`] iterator, this is just a HashMap lookup.
    /// Meaning it's way faster.
    #[inline]
    pub fn element_by_id<'a>(&'a self, id: &str) -> Option<SvgNode<'a, 'input>> {
        let node_id = self.links.get(id)?;
        Some(self.get(*node_id))
    }

    #[inline]
    fn get<'a>(&'a self, id: NodeId) -> SvgNode<'a, 'input> {
        SvgNode {
            id,
            d: &self.nodes[id.get_usize()],
            doc: self,
        }
    }
}

impl std::fmt::Debug for Document<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        if !self.root().has_children() {
            return write!(f, "Document []");
        }

        macro_rules! writeln_indented {
            ($depth:expr, $f:expr, $fmt:expr) => {
                for _ in 0..$depth { write!($f, "    ")?; }
                writeln!($f, $fmt)?;
            };
            ($depth:expr, $f:expr, $fmt:expr, $($arg:tt)*) => {
                for _ in 0..$depth { write!($f, "    ")?; }
                writeln!($f, $fmt, $($arg)*)?;
            };
        }

        fn print_children(
            parent: SvgNode,
            depth: usize,
            f: &mut std::fmt::Formatter,
        ) -> Result<(), std::fmt::Error> {
            for child in parent.children() {
                if child.is_element() {
                    writeln_indented!(depth, f, "Element {{");
                    writeln_indented!(depth, f, "    tag_name: {:?}", child.tag_name());

                    if !child.attributes().is_empty() {
                        writeln_indented!(depth + 1, f, "attributes: [");
                        for attr in child.attributes() {
                            writeln_indented!(depth + 2, f, "{:?}", attr);
                        }
                        writeln_indented!(depth + 1, f, "]");
                    }

                    if child.has_children() {
                        writeln_indented!(depth, f, "    children: [");
                        print_children(child, depth + 2, f)?;
                        writeln_indented!(depth, f, "    ]");
                    }

                    writeln_indented!(depth, f, "}}");
                } else {
                    writeln_indented!(depth, f, "{:?}", child);
                }
            }

            Ok(())
        }

        writeln!(f, "Document [")?;
        print_children(self.root(), 1, f)?;
        writeln!(f, "]")?;

        Ok(())
    }
}

#[derive(Clone, Copy, Debug)]
pub(crate) struct ShortRange {
    start: u32,
    end: u32,
}

impl ShortRange {
    #[inline]
    fn new(start: u32, end: u32) -> Self {
        ShortRange { start, end }
    }

    #[inline]
    fn to_urange(self) -> std::ops::Range<usize> {
        self.start as usize..self.end as usize
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub(crate) struct NodeId(NonZeroU32);

impl NodeId {
    #[inline]
    fn new(id: u32) -> Self {
        debug_assert!(id < u32::MAX);

        // We are using `NonZeroU32` to reduce overhead of `Option<NodeId>`.
        NodeId(NonZeroU32::new(id + 1).unwrap())
    }

    #[inline]
    fn get(self) -> u32 {
        self.0.get() - 1
    }

    #[inline]
    fn get_usize(self) -> usize {
        self.get() as usize
    }
}

impl From<usize> for NodeId {
    #[inline]
    fn from(id: usize) -> Self {
        // We already checked that `id` is limited by u32::MAX.
        debug_assert!(id <= u32::MAX as usize);
        NodeId::new(id as u32)
    }
}

pub(crate) enum NodeKind {
    Root,
    Element {
        tag_name: EId,
        attributes: ShortRange,
    },
    Text(String),
}

struct NodeData {
    parent: Option<NodeId>,
    next_sibling: Option<NodeId>,
    children: Option<(NodeId, NodeId)>,
    kind: NodeKind,
}

/// An attribute.
#[derive(Clone)]
pub struct Attribute<'input> {
    /// Attribute's name.
    pub name: AId,
    /// Attribute's value.
    pub value: roxmltree::StringStorage<'input>,
    /// Attribute's importance
    pub important: bool,
}

impl std::fmt::Debug for Attribute<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "Attribute {{ name: {:?}, value: {}, important: {} }}",
            self.name, self.value, self.important
        )
    }
}

/// An SVG node.
#[derive(Clone, Copy)]
pub struct SvgNode<'a, 'input: 'a> {
    id: NodeId,
    doc: &'a Document<'input>,
    d: &'a NodeData,
}

impl Eq for SvgNode<'_, '_> {}

impl PartialEq for SvgNode<'_, '_> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && std::ptr::eq(self.doc, other.doc) && std::ptr::eq(self.d, other.d)
    }
}

impl<'a, 'input: 'a> SvgNode<'a, 'input> {
    #[inline]
    fn id(&self) -> NodeId {
        self.id
    }

    /// Checks if the current node is an element.
    #[inline]
    pub fn is_element(&self) -> bool {
        matches!(self.d.kind, NodeKind::Element { .. })
    }

    /// Checks if the current node is a text.
    #[inline]
    pub fn is_text(&self) -> bool {
        matches!(self.d.kind, NodeKind::Text(_))
    }

    /// Returns node's document.
    #[inline]
    pub fn document(&self) -> &'a Document<'input> {
        self.doc
    }

    /// Returns element's tag name, unless the current node is text.
    #[inline]
    pub fn tag_name(&self) -> Option<EId> {
        match self.d.kind {
            NodeKind::Element { tag_name, .. } => Some(tag_name),
            _ => None,
        }
    }
    /// Returns element's `id` attribute value.
    ///
    /// Returns an empty string otherwise.
    #[inline]
    pub fn element_id(&self) -> &'a str {
        self.attribute(AId::Id).unwrap_or("")
    }

    /// Returns an attribute value.
    pub fn attribute<T: FromValue<'a, 'input>>(&self, aid: AId) -> Option<T> {
        let value = self
            .attributes()
            .iter()
            .find(|a| a.name == aid)
            .map(|a| a.value.as_str())?;
        // These AId have an initial value of none
        let is_possible_none = matches!(
            aid,
            AId::Mask
                | AId::MarkerStart
                | AId::MarkerMid
                | AId::MarkerEnd
                | AId::ClipPath
                | AId::Filter
                | AId::FontSizeAdjust
                | AId::TextDecoration
                | AId::Stroke
                | AId::StrokeDasharray
        );
        if is_possible_none && value == "none" {
            return None;
        }
        match T::parse(*self, aid, value) {
            Some(v) => Some(v),
            None => {
                // TODO: show position in XML
                log::warn!("Failed to parse {} value: '{}'.", aid, value);
                None
            }
        }
    }

    /// Returns an attribute value.
    ///
    /// Same as `SvgNode::attribute`, but doesn't show a warning.
    pub fn try_attribute<T: FromValue<'a, 'input>>(&self, aid: AId) -> Option<T> {
        let value = self
            .attributes()
            .iter()
            .find(|a| a.name == aid)
            .map(|a| a.value.as_str())?;
        T::parse(*self, aid, value)
    }

    #[inline]
    fn node_attribute(&self, aid: AId) -> Option<SvgNode<'a, 'input>> {
        let value = self.attribute(aid)?;
        let id = if aid == AId::Href {
            svgtypes::IRI::from_str(value).ok().map(|v| v.0)
        } else {
            svgtypes::FuncIRI::from_str(value).ok().map(|v| v.0)
        }?;

        self.document().element_by_id(id)
    }

    /// Checks if an attribute is present.
    #[inline]
    pub fn has_attribute(&self, aid: AId) -> bool {
        self.attributes().iter().any(|a| a.name == aid)
    }

    /// Returns a list of all element's attributes.
    #[inline]
    pub fn attributes(&self) -> &'a [Attribute<'input>] {
        match self.d.kind {
            NodeKind::Element { ref attributes, .. } => &self.doc.attrs[attributes.to_urange()],
            _ => &[],
        }
    }

    #[inline]
    fn attribute_id(&self, aid: AId) -> Option<usize> {
        match self.d.kind {
            NodeKind::Element { ref attributes, .. } => {
                let idx = self.attributes().iter().position(|attr| attr.name == aid)?;
                Some(attributes.start as usize + idx)
            }
            _ => None,
        }
    }

    /// Finds a [`Node`] that contains the required attribute.
    ///
    /// For inheritable attributes walks over ancestors until a node with
    /// the specified attribute is found.
    ///
    /// For non-inheritable attributes checks only the current node and the parent one.
    /// As per SVG spec.
    pub fn find_attribute<T: FromValue<'a, 'input>>(&self, aid: AId) -> Option<T> {
        self.find_attribute_impl(aid)?.attribute(aid)
    }

    fn find_attribute_impl(&self, aid: AId) -> Option<SvgNode<'a, 'input>> {
        if aid.is_inheritable() {
            for n in self.ancestors() {
                if n.has_attribute(aid) {
                    return Some(n);
                }
            }

            None
        } else {
            if self.has_attribute(aid) {
                Some(*self)
            } else {
                // Non-inheritable attributes can inherit a value only from a direct parent.
                let n = self.parent_element()?;
                if n.has_attribute(aid) { Some(n) } else { None }
            }
        }
    }

    /// Returns node's text data.
    ///
    /// For text nodes returns its content. For elements returns the first child node text.
    #[inline]
    pub fn text(&self) -> &'a str {
        match self.d.kind {
            NodeKind::Element { .. } => match self.first_child() {
                Some(child) if child.is_text() => match self.doc.nodes[child.id.get_usize()].kind {
                    NodeKind::Text(ref text) => text,
                    _ => "",
                },
                _ => "",
            },
            NodeKind::Text(ref text) => text,
            _ => "",
        }
    }

    /// Returns a parent node.
    #[inline]
    pub fn parent(&self) -> Option<Self> {
        self.d.parent.map(|id| self.doc.get(id))
    }

    /// Returns the parent element.
    #[inline]
    pub fn parent_element(&self) -> Option<Self> {
        self.ancestors().skip(1).find(|n| n.is_element())
    }

    /// Returns the next sibling.
    #[inline]
    pub fn next_sibling(&self) -> Option<Self> {
        self.d.next_sibling.map(|id| self.doc.get(id))
    }

    /// Returns the first child.
    #[inline]
    pub fn first_child(&self) -> Option<Self> {
        self.d.children.map(|(id, _)| self.doc.get(id))
    }

    /// Returns the first child element.
    #[inline]
    pub fn first_element_child(&self) -> Option<Self> {
        self.children().find(|n| n.is_element())
    }

    /// Returns the last child.
    #[inline]
    pub fn last_child(&self) -> Option<Self> {
        self.d.children.map(|(_, id)| self.doc.get(id))
    }

    /// Checks if the node has child nodes.
    #[inline]
    pub fn has_children(&self) -> bool {
        self.d.children.is_some()
    }

    /// Returns an iterator over ancestor nodes starting at this node.
    #[inline]
    pub fn ancestors(&self) -> Ancestors<'a, 'input> {
        Ancestors(Some(*self))
    }

    /// Returns an iterator over children nodes.
    #[inline]
    pub fn children(&self) -> Children<'a, 'input> {
        Children {
            front: self.first_child(),
            back: self.last_child(),
        }
    }

    /// Returns an iterator which traverses the subtree starting at this node.
    #[inline]
    fn traverse(&self) -> Traverse<'a, 'input> {
        Traverse {
            root: *self,
            edge: None,
        }
    }

    /// Returns an iterator over this node and its descendants.
    #[inline]
    pub fn descendants(&self) -> Descendants<'a, 'input> {
        Descendants(self.traverse())
    }

    /// Returns an iterator over elements linked via `xlink:href`.
    #[inline]
    pub fn href_iter(&self) -> HrefIter<'a, 'input> {
        HrefIter {
            doc: self.document(),
            origin: self.id(),
            curr: self.id(),
            is_first: true,
            is_finished: false,
        }
    }
}

impl std::fmt::Debug for SvgNode<'_, '_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        match self.d.kind {
            NodeKind::Root => write!(f, "Root"),
            NodeKind::Element { .. } => {
                write!(
                    f,
                    "Element {{ tag_name: {:?}, attributes: {:?} }}",
                    self.tag_name(),
                    self.attributes()
                )
            }
            NodeKind::Text(ref text) => write!(f, "Text({:?})", text),
        }
    }
}

/// An iterator over ancestor nodes.
#[derive(Clone, Debug)]
pub struct Ancestors<'a, 'input: 'a>(Option<SvgNode<'a, 'input>>);

impl<'a, 'input: 'a> Iterator for Ancestors<'a, 'input> {
    type Item = SvgNode<'a, 'input>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let node = self.0.take();
        self.0 = node.as_ref().and_then(SvgNode::parent);
        node
    }
}

/// An iterator over children nodes.
#[derive(Clone, Debug)]
pub struct Children<'a, 'input: 'a> {
    front: Option<SvgNode<'a, 'input>>,
    back: Option<SvgNode<'a, 'input>>,
}

impl<'a, 'input: 'a> Iterator for Children<'a, 'input> {
    type Item = SvgNode<'a, 'input>;

    fn next(&mut self) -> Option<Self::Item> {
        let node = self.front.take();
        if self.front == self.back {
            self.back = None;
        } else {
            self.front = node.as_ref().and_then(SvgNode::next_sibling);
        }
        node
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
enum Edge<'a, 'input: 'a> {
    Open(SvgNode<'a, 'input>),
    Close(SvgNode<'a, 'input>),
}

#[derive(Clone, Debug)]
struct Traverse<'a, 'input: 'a> {
    root: SvgNode<'a, 'input>,
    edge: Option<Edge<'a, 'input>>,
}

impl<'a, 'input: 'a> Iterator for Traverse<'a, 'input> {
    type Item = Edge<'a, 'input>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.edge {
            Some(Edge::Open(node)) => {
                self.edge = Some(match node.first_child() {
                    Some(first_child) => Edge::Open(first_child),
                    None => Edge::Close(node),
                });
            }
            Some(Edge::Close(node)) => {
                if node == self.root {
                    self.edge = None;
                } else if let Some(next_sibling) = node.next_sibling() {
                    self.edge = Some(Edge::Open(next_sibling));
                } else {
                    self.edge = node.parent().map(Edge::Close);
                }
            }
            None => {
                self.edge = Some(Edge::Open(self.root));
            }
        }

        self.edge
    }
}

/// A descendants iterator.
#[derive(Clone, Debug)]
pub struct Descendants<'a, 'input: 'a>(Traverse<'a, 'input>);

impl<'a, 'input: 'a> Iterator for Descendants<'a, 'input> {
    type Item = SvgNode<'a, 'input>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        for edge in &mut self.0 {
            if let Edge::Open(node) = edge {
                return Some(node);
            }
        }

        None
    }
}

/// An iterator over `xlink:href` references.
#[derive(Clone, Debug)]
pub struct HrefIter<'a, 'input: 'a> {
    doc: &'a Document<'input>,
    origin: NodeId,
    curr: NodeId,
    is_first: bool,
    is_finished: bool,
}

impl<'a, 'input: 'a> Iterator for HrefIter<'a, 'input> {
    type Item = SvgNode<'a, 'input>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.is_finished {
            return None;
        }

        if self.is_first {
            self.is_first = false;
            return Some(self.doc.get(self.curr));
        }

        if let Some(link) = self.doc.get(self.curr).node_attribute(AId::Href) {
            if link.id() == self.curr || link.id() == self.origin {
                log::warn!(
                    "Element '#{}' cannot reference itself via 'xlink:href'.",
                    self.doc.get(self.origin).element_id()
                );
                self.is_finished = true;
                return None;
            }

            self.curr = link.id();
            Some(self.doc.get(self.curr))
        } else {
            None
        }
    }
}

impl EId {
    /// Checks if this is a
    /// [graphics element](https://www.w3.org/TR/SVG11/intro.html#TermGraphicsElement).
    pub fn is_graphic(&self) -> bool {
        matches!(
            self,
            EId::Circle
                | EId::Ellipse
                | EId::Image
                | EId::Line
                | EId::Path
                | EId::Polygon
                | EId::Polyline
                | EId::Rect
                | EId::Text
                | EId::Use
        )
    }

    /// Checks if this is a
    /// [gradient element](https://www.w3.org/TR/SVG11/intro.html#TermGradientElement).
    pub fn is_gradient(&self) -> bool {
        matches!(self, EId::LinearGradient | EId::RadialGradient)
    }

    /// Checks if this is a
    /// [paint server element](https://www.w3.org/TR/SVG11/intro.html#TermPaint).
    pub fn is_paint_server(&self) -> bool {
        matches!(
            self,
            EId::LinearGradient | EId::RadialGradient | EId::Pattern
        )
    }
}

impl AId {
    fn is_presentation(&self) -> bool {
        matches!(
            self,
            AId::AlignmentBaseline
                | AId::BaselineShift
                | AId::BackgroundColor // non-standard SVG attribute
                | AId::ClipPath
                | AId::ClipRule
                | AId::Color
                | AId::ColorInterpolation
                | AId::ColorInterpolationFilters
                | AId::ColorRendering
                | AId::Direction
                | AId::Display
                | AId::DominantBaseline
                | AId::Fill
                | AId::FillOpacity
                | AId::FillRule
                | AId::Filter
                | AId::FloodColor
                | AId::FloodOpacity
                | AId::FontFamily
                | AId::FontKerning // technically not presentation
                | AId::FontSize
                | AId::FontSizeAdjust
                | AId::FontStretch
                | AId::FontStyle
                | AId::FontVariant
                | AId::FontWeight
                | AId::GlyphOrientationHorizontal
                | AId::GlyphOrientationVertical
                | AId::ImageRendering
                | AId::Isolation // technically not presentation
                | AId::LetterSpacing
                | AId::LightingColor
                | AId::MarkerEnd
                | AId::MarkerMid
                | AId::MarkerStart
                | AId::Mask
                | AId::MaskType
                | AId::MixBlendMode // technically not presentation
                | AId::Opacity
                | AId::Overflow
                | AId::PaintOrder
                | AId::ShapeRendering
                | AId::StopColor
                | AId::StopOpacity
                | AId::Stroke
                | AId::StrokeDasharray
                | AId::StrokeDashoffset
                | AId::StrokeLinecap
                | AId::StrokeLinejoin
                | AId::StrokeMiterlimit
                | AId::StrokeOpacity
                | AId::StrokeWidth
                | AId::TextAnchor
                | AId::TextDecoration
                | AId::TextOverflow
                | AId::TextRendering
                | AId::Transform
                | AId::TransformOrigin
                | AId::UnicodeBidi
                | AId::VectorEffect
                | AId::Visibility
                | AId::WhiteSpace
                | AId::WordSpacing
                | AId::WritingMode
        )
    }

    /// Checks if the current attribute is inheritable.
    fn is_inheritable(&self) -> bool {
        if self.is_presentation() {
            !is_non_inheritable(*self)
        } else {
            false
        }
    }

    fn allows_inherit_value(&self) -> bool {
        matches!(
            self,
            AId::AlignmentBaseline
                | AId::BaselineShift
                | AId::ClipPath
                | AId::ClipRule
                | AId::Color
                | AId::ColorInterpolationFilters
                | AId::Direction
                | AId::Display
                | AId::DominantBaseline
                | AId::Fill
                | AId::FillOpacity
                | AId::FillRule
                | AId::Filter
                | AId::FloodColor
                | AId::FloodOpacity
                | AId::FontFamily
                | AId::FontKerning
                | AId::FontSize
                | AId::FontStretch
                | AId::FontStyle
                | AId::FontVariant
                | AId::FontWeight
                | AId::ImageRendering
                | AId::Kerning
                | AId::LetterSpacing
                | AId::MarkerEnd
                | AId::MarkerMid
                | AId::MarkerStart
                | AId::Mask
                | AId::Opacity
                | AId::Overflow
                | AId::ShapeRendering
                | AId::StopColor
                | AId::StopOpacity
                | AId::Stroke
                | AId::StrokeDasharray
                | AId::StrokeDashoffset
                | AId::StrokeLinecap
                | AId::StrokeLinejoin
                | AId::StrokeMiterlimit
                | AId::StrokeOpacity
                | AId::StrokeWidth
                | AId::TextAnchor
                | AId::TextDecoration
                | AId::TextRendering
                | AId::Visibility
                | AId::WordSpacing
                | AId::WritingMode
        )
    }
}

fn is_non_inheritable(id: AId) -> bool {
    matches!(
        id,
        AId::AlignmentBaseline
            | AId::BaselineShift
            | AId::ClipPath
            | AId::Display
            | AId::DominantBaseline
            | AId::Filter
            | AId::FloodColor
            | AId::FloodOpacity
            | AId::Mask
            | AId::Opacity
            | AId::Overflow
            | AId::LightingColor
            | AId::StopColor
            | AId::StopOpacity
            | AId::TextDecoration
            | AId::Transform
            | AId::TransformOrigin
    )
}

// TODO: is there a way yo make it less ugly? Too many lifetimes.
/// A trait for parsing attribute values.
pub trait FromValue<'a, 'input: 'a>: Sized {
    /// Parses an attribute value.
    ///
    /// When `None` is returned, the attribute value will be logged as a parsing failure.
    fn parse(node: SvgNode<'a, 'input>, aid: AId, value: &'a str) -> Option<Self>;
}

impl<'a, 'input: 'a> FromValue<'a, 'input> for &'a str {
    fn parse(_: SvgNode<'a, 'input>, _: AId, value: &'a str) -> Option<Self> {
        Some(value)
    }
}

impl<'a, 'input: 'a> FromValue<'a, 'input> for f32 {
    fn parse(_: SvgNode, _: AId, value: &str) -> Option<Self> {
        svgtypes::Number::from_str(value).ok().map(|v| v.0 as f32)
    }
}

impl<'a, 'input: 'a> FromValue<'a, 'input> for svgtypes::Length {
    fn parse(_: SvgNode, _: AId, value: &str) -> Option<Self> {
        svgtypes::Length::from_str(value).ok()
    }
}

// TODO: to svgtypes?
impl<'a, 'input: 'a> FromValue<'a, 'input> for Opacity {
    fn parse(_: SvgNode, _: AId, value: &str) -> Option<Self> {
        let length = svgtypes::Length::from_str(value).ok()?;
        if length.unit == svgtypes::LengthUnit::Percent {
            Some(Opacity::new_clamped(length.number as f32 / 100.0))
        } else if length.unit == svgtypes::LengthUnit::None {
            Some(Opacity::new_clamped(length.number as f32))
        } else {
            None
        }
    }
}

impl<'a, 'input: 'a> FromValue<'a, 'input> for Transform {
    fn parse(_: SvgNode, _: AId, value: &str) -> Option<Self> {
        let ts = match svgtypes::Transform::from_str(value) {
            Ok(v) => v,
            Err(_) => return None,
        };

        let ts = Transform::from_row(
            ts.a as f32,
            ts.b as f32,
            ts.c as f32,
            ts.d as f32,
            ts.e as f32,
            ts.f as f32,
        );

        if ts.is_valid() {
            Some(ts)
        } else {
            Some(Transform::default())
        }
    }
}

impl<'a, 'input: 'a> FromValue<'a, 'input> for svgtypes::TransformOrigin {
    fn parse(_: SvgNode, _: AId, value: &str) -> Option<Self> {
        Self::from_str(value).ok()
    }
}

impl<'a, 'input: 'a> FromValue<'a, 'input> for svgtypes::ViewBox {
    fn parse(_: SvgNode, _: AId, value: &str) -> Option<Self> {
        Self::from_str(value).ok()
    }
}

impl<'a, 'input: 'a> FromValue<'a, 'input> for Units {
    fn parse(_: SvgNode, _: AId, value: &str) -> Option<Self> {
        match value {
            "userSpaceOnUse" => Some(Units::UserSpaceOnUse),
            "objectBoundingBox" => Some(Units::ObjectBoundingBox),
            _ => None,
        }
    }
}

impl<'a, 'input: 'a> FromValue<'a, 'input> for svgtypes::AspectRatio {
    fn parse(_: SvgNode, _: AId, value: &str) -> Option<Self> {
        Self::from_str(value).ok()
    }
}

impl<'a, 'input: 'a> FromValue<'a, 'input> for svgtypes::PaintOrder {
    fn parse(_: SvgNode, _: AId, value: &str) -> Option<Self> {
        Self::from_str(value).ok()
    }
}

impl<'a, 'input: 'a> FromValue<'a, 'input> for svgtypes::Color {
    fn parse(_: SvgNode, _: AId, value: &str) -> Option<Self> {
        Self::from_str(value).ok()
    }
}

impl<'a, 'input: 'a> FromValue<'a, 'input> for svgtypes::Angle {
    fn parse(_: SvgNode, _: AId, value: &str) -> Option<Self> {
        Self::from_str(value).ok()
    }
}

impl<'a, 'input: 'a> FromValue<'a, 'input> for svgtypes::EnableBackground {
    fn parse(_: SvgNode, _: AId, value: &str) -> Option<Self> {
        Self::from_str(value).ok()
    }
}

impl<'a, 'input: 'a> FromValue<'a, 'input> for svgtypes::Paint<'a> {
    fn parse(_: SvgNode, _: AId, value: &'a str) -> Option<Self> {
        Self::from_str(value).ok()
    }
}

impl<'a, 'input: 'a> FromValue<'a, 'input> for Vec<f32> {
    fn parse(_: SvgNode, _: AId, value: &str) -> Option<Self> {
        let mut list = Vec::new();
        for n in svgtypes::NumberListParser::from(value) {
            list.push(n.ok()? as f32);
        }

        Some(list)
    }
}

impl<'a, 'input: 'a> FromValue<'a, 'input> for Vec<svgtypes::Length> {
    fn parse(_: SvgNode, _: AId, value: &str) -> Option<Self> {
        let mut list = Vec::new();
        for n in svgtypes::LengthListParser::from(value) {
            list.push(n.ok()?);
        }

        Some(list)
    }
}

impl<'a, 'input: 'a> FromValue<'a, 'input> for Visibility {
    fn parse(_: SvgNode, _: AId, value: &str) -> Option<Self> {
        match value {
            "visible" => Some(Visibility::Visible),
            "hidden" => Some(Visibility::Hidden),
            "collapse" => Some(Visibility::Collapse),
            _ => None,
        }
    }
}

impl<'a, 'input: 'a> FromValue<'a, 'input> for SpreadMethod {
    fn parse(_: SvgNode, _: AId, value: &str) -> Option<Self> {
        match value {
            "pad" => Some(SpreadMethod::Pad),
            "reflect" => Some(SpreadMethod::Reflect),
            "repeat" => Some(SpreadMethod::Repeat),
            _ => None,
        }
    }
}

impl<'a, 'input: 'a> FromValue<'a, 'input> for ShapeRendering {
    fn parse(_: SvgNode, _: AId, value: &str) -> Option<Self> {
        match value {
            "optimizeSpeed" => Some(ShapeRendering::OptimizeSpeed),
            "crispEdges" => Some(ShapeRendering::CrispEdges),
            "auto" | "geometricPrecision" => Some(ShapeRendering::GeometricPrecision),
            _ => None,
        }
    }
}

impl<'a, 'input: 'a> FromValue<'a, 'input> for TextRendering {
    fn parse(_: SvgNode, _: AId, value: &str) -> Option<Self> {
        match value {
            "optimizeSpeed" => Some(TextRendering::OptimizeSpeed),
            "auto" | "optimizeLegibility" => Some(TextRendering::OptimizeLegibility),
            "geometricPrecision" => Some(TextRendering::GeometricPrecision),
            _ => None,
        }
    }
}

impl<'a, 'input: 'a> FromValue<'a, 'input> for ImageRendering {
    fn parse(_: SvgNode, _: AId, value: &str) -> Option<Self> {
        match value {
            "auto" | "optimizeQuality" => Some(ImageRendering::OptimizeQuality),
            "optimizeSpeed" => Some(ImageRendering::OptimizeSpeed),
            "smooth" => Some(ImageRendering::Smooth),
            "high-quality" => Some(ImageRendering::HighQuality),
            "crisp-edges" => Some(ImageRendering::CrispEdges),
            "pixelated" => Some(ImageRendering::Pixelated),
            _ => None,
        }
    }
}

impl<'a, 'input: 'a> FromValue<'a, 'input> for BlendMode {
    fn parse(_: SvgNode, _: AId, value: &str) -> Option<Self> {
        match value {
            "normal" => Some(BlendMode::Normal),
            "multiply" => Some(BlendMode::Multiply),
            "screen" => Some(BlendMode::Screen),
            "overlay" => Some(BlendMode::Overlay),
            "darken" => Some(BlendMode::Darken),
            "lighten" => Some(BlendMode::Lighten),
            "color-dodge" => Some(BlendMode::ColorDodge),
            "color-burn" => Some(BlendMode::ColorBurn),
            "hard-light" => Some(BlendMode::HardLight),
            "soft-light" => Some(BlendMode::SoftLight),
            "difference" => Some(BlendMode::Difference),
            "exclusion" => Some(BlendMode::Exclusion),
            "hue" => Some(BlendMode::Hue),
            "saturation" => Some(BlendMode::Saturation),
            "color" => Some(BlendMode::Color),
            "luminosity" => Some(BlendMode::Luminosity),
            _ => None,
        }
    }
}

impl<'a, 'input: 'a> FromValue<'a, 'input> for SvgNode<'a, 'input> {
    fn parse(node: SvgNode<'a, 'input>, aid: AId, value: &str) -> Option<Self> {
        let id = if aid == AId::Href {
            svgtypes::IRI::from_str(value).ok().map(|v| v.0)
        } else {
            svgtypes::FuncIRI::from_str(value).ok().map(|v| v.0)
        }?;

        node.document().element_by_id(id)
    }
}
