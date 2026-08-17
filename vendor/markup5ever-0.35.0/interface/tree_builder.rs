// Copyright 2014-2017 The html5ever Project Developers. See the
// COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! This module contains functionality for managing the DOM, including adding/removing nodes.
//!
//! It can be used by a parser to create the DOM graph structure in memory.

use crate::interface::{Attribute, ExpandedName, QualName};
use std::borrow::Cow;
use std::fmt::Debug;
use tendril::StrTendril;
use web_atoms::{LocalName, Namespace};

pub use self::NodeOrText::{AppendNode, AppendText};
pub use self::QuirksMode::{LimitedQuirks, NoQuirks, Quirks};

/// Something which can be inserted into the DOM.
///
/// Adjacent sibling text nodes are merged into a single node, so
/// the sink may not want to allocate a `Handle` for each.
pub enum NodeOrText<Handle> {
    AppendNode(Handle),
    AppendText(StrTendril),
}

/// A document's quirks mode, for compatibility with old browsers. See [quirks mode on wikipedia]
/// for more information.
///
/// [quirks mode on wikipedia]: https://en.wikipedia.org/wiki/Quirks_mode
#[derive(PartialEq, Eq, Copy, Clone, Hash, Debug)]
pub enum QuirksMode {
    /// Full quirks mode
    Quirks,
    /// Almost standards mode
    LimitedQuirks,
    /// Standards mode
    NoQuirks,
}

/// Special properties of an element, useful for tagging elements with this information.
#[derive(Default)]
#[non_exhaustive]
pub struct ElementFlags {
    /// A document fragment should be created, associated with the element,
    /// and returned in TreeSink::get_template_contents.
    ///
    /// See [template-contents in the whatwg spec][whatwg template-contents].
    ///
    /// [whatwg template-contents]: https://html.spec.whatwg.org/multipage/#template-contents
    pub template: bool,

    /// This boolean should be recorded with the element and returned
    /// in TreeSink::is_mathml_annotation_xml_integration_point
    ///
    /// See [html-integration-point in the whatwg spec][whatwg integration-point].
    ///
    /// [whatwg integration-point]: https://html.spec.whatwg.org/multipage/#html-integration-point
    pub mathml_annotation_xml_integration_point: bool,
}

/// A constructor for an element.
///
/// # Examples
///
/// Create an element like `<div class="test-class-name"></div>`:
pub fn create_element<Sink>(sink: &Sink, name: QualName, attrs: Vec<Attribute>) -> Sink::Handle
where
    Sink: TreeSink,
{
    let mut flags = ElementFlags::default();
    match name.expanded() {
        expanded_name!(html "template") => flags.template = true,
        expanded_name!(mathml "annotation-xml") => {
            flags.mathml_annotation_xml_integration_point = attrs.iter().any(|attr| {
                attr.name.expanded() == expanded_name!("", "encoding")
                    && (attr.value.eq_ignore_ascii_case("text/html")
                        || attr.value.eq_ignore_ascii_case("application/xhtml+xml"))
            })
        },
        _ => {},
    }
    sink.create_element(name, attrs, flags)
}

/// An abstraction over any type that can represent an element's local name and namespace.
pub trait ElemName: Debug {
    fn ns(&self) -> &Namespace;
    fn local_name(&self) -> &LocalName;

    #[inline(always)]
    fn expanded(&self) -> ExpandedName<'_> {
        ExpandedName {
            ns: self.ns(),
            local: self.local_name(),
        }
    }
}

/// Methods a parser can use to create the DOM. The DOM provider implements this trait.
///
/// Having this as a trait potentially allows multiple implementations of the DOM to be used with
/// the same parser.
pub trait TreeSink {
    /// `Handle` is a reference to a DOM node.  The tree builder requires
    /// that a `Handle` implements `Clone` to get another reference to
    /// the same node.
    type Handle: Clone;

    /// The overall result of parsing.
    ///
    /// This should default to Self, but default associated types are not stable yet.
    /// [rust-lang/rust#29661](https://github.com/rust-lang/rust/issues/29661)
    type Output;

    //
    type ElemName<'a>: ElemName
    where
        Self: 'a;

    /// Consume this sink and return the overall result of parsing.
    ///
    /// TODO:This should default to `fn finish(self) -> Self::Output { self }`,
    /// but default associated types are not stable yet.
    /// [rust-lang/rust#29661](https://github.com/rust-lang/rust/issues/29661)
    fn finish(self) -> Self::Output;

    /// Signal a parse error.
    fn parse_error(&self, msg: Cow<'static, str>);

    /// Get a handle to the `Document` node.
    fn get_document(&self) -> Self::Handle;

    /// What is the name of this element?
    ///
    /// Should never be called on a non-element node;
    /// feel free to `panic!`.
    fn elem_name<'a>(&'a self, target: &'a Self::Handle) -> Self::ElemName<'a>;

    /// Create an element.
    ///
    /// When creating a template element (`name.ns.expanded() == expanded_name!(html "template")`),
    /// an associated document fragment called the "template contents" should
    /// also be created. Later calls to self.get_template_contents() with that
    /// given element return it.
    /// See [the template element in the whatwg spec][whatwg template].
    ///
    /// [whatwg template]: https://html.spec.whatwg.org/multipage/#the-template-element
    fn create_element(
        &self,
        name: QualName,
        attrs: Vec<Attribute>,
        flags: ElementFlags,
    ) -> Self::Handle;

    /// Create a comment node.
    fn create_comment(&self, text: StrTendril) -> Self::Handle;

    /// Create a Processing Instruction node.
    fn create_pi(&self, target: StrTendril, data: StrTendril) -> Self::Handle;

    /// Append a node as the last child of the given node.  If this would
    /// produce adjacent sibling text nodes, it should concatenate the text
    /// instead.
    ///
    /// The child node will not already have a parent.
    fn append(&self, parent: &Self::Handle, child: NodeOrText<Self::Handle>);

    /// When the insertion point is decided by the existence of a parent node of the
    /// element, we consider both possibilities and send the element which will be used
    /// if a parent node exists, along with the element to be used if there isn't one.
    fn append_based_on_parent_node(
        &self,
        element: &Self::Handle,
        prev_element: &Self::Handle,
        child: NodeOrText<Self::Handle>,
    );

    /// Append a `DOCTYPE` element to the `Document` node.
    fn append_doctype_to_document(
        &self,
        name: StrTendril,
        public_id: StrTendril,
        system_id: StrTendril,
    );

    /// Mark a HTML `<script>` as "already started".
    fn mark_script_already_started(&self, _node: &Self::Handle) {}

    /// Indicate that a node was popped off the stack of open elements.
    fn pop(&self, _node: &Self::Handle) {}

    /// Get a handle to a template's template contents. The tree builder
    /// promises this will never be called with something else than
    /// a template element.
    fn get_template_contents(&self, target: &Self::Handle) -> Self::Handle;

    /// Do two handles refer to the same node?
    fn same_node(&self, x: &Self::Handle, y: &Self::Handle) -> bool;

    /// Set the document's quirks mode.
    fn set_quirks_mode(&self, mode: QuirksMode);

    /// Append a node as the sibling immediately before the given node.
    ///
    /// The tree builder promises that `sibling` is not a text node.  However its
    /// old previous sibling, which would become the new node's previous sibling,
    /// could be a text node.  If the new node is also a text node, the two should
    /// be merged, as in the behavior of `append`.
    ///
    /// NB: `new_node` may have an old parent, from which it should be removed.
    fn append_before_sibling(&self, sibling: &Self::Handle, new_node: NodeOrText<Self::Handle>);

    /// Add each attribute to the given element, if no attribute with that name
    /// already exists. The tree builder promises this will never be called
    /// with something else than an element.
    fn add_attrs_if_missing(&self, target: &Self::Handle, attrs: Vec<Attribute>);

    /// Associate the given form-associatable element with the form element
    fn associate_with_form(
        &self,
        _target: &Self::Handle,
        _form: &Self::Handle,
        _nodes: (&Self::Handle, Option<&Self::Handle>),
    ) {
    }

    /// Detach the given node from its parent.
    fn remove_from_parent(&self, target: &Self::Handle);

    /// Remove all the children from node and append them to new_parent.
    fn reparent_children(&self, node: &Self::Handle, new_parent: &Self::Handle);

    /// Returns true if the adjusted current node is an HTML integration point
    /// and the token is a start tag.
    fn is_mathml_annotation_xml_integration_point(&self, _handle: &Self::Handle) -> bool {
        false
    }

    /// Called whenever the line number changes.
    fn set_current_line(&self, _line_number: u64) {}

    fn allow_declarative_shadow_roots(&self, _intended_parent: &Self::Handle) -> bool {
        true
    }

    /// Attempt to attach a declarative shadow root at the given location.
    ///
    /// Returns a boolean indicating whether the operation succeeded or not.
    fn attach_declarative_shadow(
        &self,
        _location: &Self::Handle,
        _template: &Self::Handle,
        _attrs: &[Attribute],
    ) -> bool {
        false
    }
}

/// Trace hooks for a garbage-collected DOM.
pub trait Tracer {
    type Handle;

    /// Upon a call to `trace_handles`, the tree builder will call this method
    /// for each handle in its internal state.
    fn trace_handle(&self, node: &Self::Handle);
}
