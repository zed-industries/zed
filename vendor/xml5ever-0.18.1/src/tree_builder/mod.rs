// Copyright 2014-2017 The html5ever Project Developers. See the
// COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

mod types;

use log::{debug, warn};
use mac::unwrap_or_return;
use markup5ever::{local_name, namespace_prefix, namespace_url, ns};
use std::borrow::Cow;
use std::borrow::Cow::Borrowed;
use std::collections::btree_map::Iter;
use std::collections::{BTreeMap, HashSet, VecDeque};
use std::fmt::{Debug, Error, Formatter};
use std::mem;

pub use self::interface::{NextParserState, NodeOrText, Tracer, TreeSink};
use self::types::*;
use crate::interface::{self, create_element, AppendNode, Attribute, QualName};
use crate::interface::{AppendText, ExpandedName};
use crate::tokenizer::states::Quiescent;
use crate::tokenizer::{self, EndTag, StartTag, Tag, TokenSink};
use crate::tokenizer::{Doctype, EmptyTag, Pi, ShortTag};
use crate::{LocalName, Namespace, Prefix};

use crate::tendril::{StrTendril, Tendril};

static XML_URI: &str = "http://www.w3.org/XML/1998/namespace";
static XMLNS_URI: &str = "http://www.w3.org/2000/xmlns/";

type InsResult = Result<(), Cow<'static, str>>;

#[derive(Debug)]
struct NamespaceMapStack(Vec<NamespaceMap>);

impl NamespaceMapStack {
    fn new() -> NamespaceMapStack {
        NamespaceMapStack(vec![NamespaceMap::default()])
    }

    fn push(&mut self, map: NamespaceMap) {
        self.0.push(map);
    }

    #[doc(hidden)]
    pub fn pop(&mut self) {
        self.0.pop();
    }
}

#[doc(hidden)]
pub struct NamespaceMap {
    // Map that maps prefixes to URI.
    //
    // Key denotes namespace prefix, and value denotes
    // URI it maps to.
    //
    // If value of value is None, that means the namespace
    // denoted by key has been undeclared.
    scope: BTreeMap<Option<Prefix>, Option<Namespace>>,
}

impl Debug for NamespaceMap {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "\nNamespaceMap[")?;
        for (key, value) in &self.scope {
            writeln!(f, "   {:?} : {:?}", key, value)?;
        }
        write!(f, "]")
    }
}

impl NamespaceMap {
    // Returns an empty namespace.
    #[doc(hidden)]
    pub fn empty() -> NamespaceMap {
        NamespaceMap {
            scope: BTreeMap::new(),
        }
    }

    fn default() -> NamespaceMap {
        NamespaceMap {
            scope: {
                let mut map = BTreeMap::new();
                map.insert(None, None);
                map.insert(Some(namespace_prefix!("xml")), Some(ns!(xml)));
                map.insert(Some(namespace_prefix!("xmlns")), Some(ns!(xmlns)));
                map
            },
        }
    }

    #[doc(hidden)]
    pub fn get(&self, prefix: &Option<Prefix>) -> Option<&Option<Namespace>> {
        self.scope.get(prefix)
    }

    #[doc(hidden)]
    pub fn get_scope_iter(&self) -> Iter<Option<Prefix>, Option<Namespace>> {
        self.scope.iter()
    }

    #[doc(hidden)]
    pub fn insert(&mut self, name: &QualName) {
        let prefix = name.prefix.as_ref().cloned();
        let namespace = Some(Namespace::from(&*name.ns));
        self.scope.insert(prefix, namespace);
    }

    fn insert_ns(&mut self, attr: &Attribute) -> InsResult {
        if &*attr.value == XMLNS_URI {
            return Err(Borrowed("Can't declare XMLNS URI"));
        };

        let opt_uri = if attr.value.is_empty() {
            None
        } else {
            Some(Namespace::from(&*attr.value))
        };

        let result = match (&attr.name.prefix, &*attr.name.local) {
            (&Some(namespace_prefix!("xmlns")), "xml") => {
                if &*attr.value != XML_URI {
                    Err(Borrowed("XML namespace can't be redeclared"))
                } else {
                    Ok(())
                }
            },

            (&Some(namespace_prefix!("xmlns")), "xmlns") => {
                Err(Borrowed("XMLNS namespaces can't be changed"))
            },

            (&Some(namespace_prefix!("xmlns")), _) | (&None, "xmlns") => {
                // We can have two cases of properly defined xmlns
                // First with default namespace e.g.
                //
                //     <a xmlns = "www.uri.org" />
                let ns_prefix = if &*attr.name.local == "xmlns" {
                    None

                // Second is with named namespace e.g.
                //
                //     <a xmlns:a = "www.uri.org" />
                } else {
                    Some(Prefix::from(&*attr.name.local))
                };

                if opt_uri.is_some() && self.scope.contains_key(&ns_prefix) {
                    Err(Borrowed("Namespace already defined"))
                } else {
                    self.scope.insert(ns_prefix, opt_uri);
                    Ok(())
                }
            },

            (_, _) => Err(Borrowed("Invalid namespace declaration.")),
        };
        result
    }
}

/// Tree builder options, with an impl for Default.
#[derive(Copy, Clone, Default)]
pub struct XmlTreeBuilderOpts {}

/// The XML tree builder.
pub struct XmlTreeBuilder<Handle, Sink> {
    /// Configuration options for XmlTreeBuilder
    _opts: XmlTreeBuilderOpts,

    /// Consumer of tree modifications.
    pub sink: Sink,

    /// The document node, which is created by the sink.
    doc_handle: Handle,

    /// Next state change for the tokenizer, if any.
    next_tokenizer_state: Option<tokenizer::states::XmlState>,

    /// Stack of open elements, most recently added at end.
    open_elems: Vec<Handle>,

    /// Current element pointer.
    curr_elem: Option<Handle>,

    /// Stack of namespace identifiers and namespaces.
    namespace_stack: NamespaceMapStack,

    /// Current namespace identifier
    current_namespace: NamespaceMap,

    /// Current tree builder phase.
    phase: XmlPhase,
}
impl<Handle, Sink> XmlTreeBuilder<Handle, Sink>
where
    Handle: Clone,
    Sink: TreeSink<Handle = Handle>,
{
    /// Create a new tree builder which sends tree modifications to a particular `TreeSink`.
    ///
    /// The tree builder is also a `TokenSink`.
    pub fn new(mut sink: Sink, opts: XmlTreeBuilderOpts) -> XmlTreeBuilder<Handle, Sink> {
        let doc_handle = sink.get_document();
        XmlTreeBuilder {
            _opts: opts,
            sink,
            doc_handle,
            next_tokenizer_state: None,
            open_elems: vec![],
            curr_elem: None,
            namespace_stack: NamespaceMapStack::new(),
            current_namespace: NamespaceMap::empty(),
            phase: Start,
        }
    }

    /// Call the `Tracer`'s `trace_handle` method on every `Handle` in the tree builder's
    /// internal state.  This is intended to support garbage-collected DOMs.
    pub fn trace_handles(&self, tracer: &dyn Tracer<Handle = Handle>) {
        tracer.trace_handle(&self.doc_handle);
        for e in self.open_elems.iter() {
            tracer.trace_handle(e);
        }
        if let Some(h) = self.curr_elem.as_ref() {
            tracer.trace_handle(h);
        }
    }

    // Debug helper
    #[cfg(not(for_c))]
    #[allow(dead_code)]
    fn dump_state(&self, label: String) {
        debug!("dump_state on {}", label);
        debug!("    open_elems:");
        for node in self.open_elems.iter() {
            debug!(" {:?}", self.sink.elem_name(node));
        }
        debug!("");
    }

    #[cfg(for_c)]
    fn debug_step(&self, _mode: XmlPhase, _token: &Token) {}

    #[cfg(not(for_c))]
    fn debug_step(&self, mode: XmlPhase, token: &Token) {
        debug!(
            "processing {:?} in insertion mode {:?}",
            format!("{:?}", token),
            mode
        );
    }

    fn declare_ns(&mut self, attr: &mut Attribute) {
        if let Err(msg) = self.current_namespace.insert_ns(attr) {
            self.sink.parse_error(msg);
        } else {
            attr.name.ns = ns!(xmlns);
        }
    }

    fn find_uri(&self, prefix: &Option<Prefix>) -> Result<Option<Namespace>, Cow<'static, str>> {
        let mut uri = Err(Borrowed("No appropriate namespace found"));

        for ns in self
            .namespace_stack
            .0
            .iter()
            .chain(Some(&self.current_namespace))
            .rev()
        {
            if let Some(el) = ns.get(prefix) {
                uri = Ok(el.clone());
                break;
            }
        }
        uri
    }

    fn bind_qname(&mut self, name: &mut QualName) {
        match self.find_uri(&name.prefix) {
            Ok(uri) => {
                let ns_uri = match uri {
                    Some(e) => e,
                    None => ns!(),
                };
                name.ns = ns_uri;
            },
            Err(msg) => {
                self.sink.parse_error(msg);
            },
        }
    }

    // This method takes in name qualified name and binds it to the
    // existing namespace context.
    //
    // Returns false if the attribute is a duplicate, returns true otherwise.
    fn bind_attr_qname(
        &mut self,
        present_attrs: &mut HashSet<(Namespace, LocalName)>,
        name: &mut QualName,
    ) -> bool {
        // Attributes don't have default namespace
        let mut not_duplicate = true;

        if name.prefix.is_some() {
            self.bind_qname(name);
            not_duplicate = Self::check_duplicate_attr(present_attrs, name);
        }
        not_duplicate
    }

    fn check_duplicate_attr(
        present_attrs: &mut HashSet<(Namespace, LocalName)>,
        name: &QualName,
    ) -> bool {
        let pair = (name.ns.clone(), name.local.clone());

        if present_attrs.contains(&pair) {
            return false;
        }
        present_attrs.insert(pair);
        true
    }

    fn process_namespaces(&mut self, tag: &mut Tag) {
        // List of already present namespace local name attribute pairs.
        let mut present_attrs: HashSet<(Namespace, LocalName)> = Default::default();

        let mut new_attr = vec![];
        // First we extract all namespace declarations
        for attr in tag.attrs.iter_mut().filter(|attr| {
            attr.name.prefix == Some(namespace_prefix!("xmlns"))
                || attr.name.local == local_name!("xmlns")
        }) {
            self.declare_ns(attr);
        }

        // Then we bind those namespace declarations to attributes
        for attr in tag.attrs.iter_mut().filter(|attr| {
            attr.name.prefix != Some(namespace_prefix!("xmlns"))
                && attr.name.local != local_name!("xmlns")
        }) {
            if self.bind_attr_qname(&mut present_attrs, &mut attr.name) {
                new_attr.push(attr.clone());
            }
        }
        tag.attrs = new_attr;

        // Then we bind the tags namespace.
        self.bind_qname(&mut tag.name);

        // Finally, we dump current namespace if its unneeded.
        let x = mem::replace(&mut self.current_namespace, NamespaceMap::empty());

        // Only start tag doesn't dump current namespace. However, <script /> is treated
        // differently than every other empty tag, so it needs to retain the current
        // namespace as well.
        if tag.kind == StartTag || (tag.kind == EmptyTag && tag.name.local == local_name!("script"))
        {
            self.namespace_stack.push(x);
        }
    }

    fn process_to_completion(&mut self, mut token: Token) {
        // Queue of additional tokens yet to be processed.
        // This stays empty in the common case where we don't split whitespace.
        let mut more_tokens = VecDeque::new();

        loop {
            let phase = self.phase;

            #[allow(clippy::unused_unit)]
            match self.step(phase, token) {
                Done => {
                    token = unwrap_or_return!(more_tokens.pop_front(), ());
                },
                Reprocess(m, t) => {
                    self.phase = m;
                    token = t;
                },
            }
        }
    }
}

impl<Handle, Sink> TokenSink for XmlTreeBuilder<Handle, Sink>
where
    Handle: Clone,
    Sink: TreeSink<Handle = Handle>,
{
    fn process_token(&mut self, token: tokenizer::Token) {
        // Handle `ParseError` and `DoctypeToken`; convert everything else to the local `Token` type.
        let token = match token {
            tokenizer::ParseError(e) => {
                self.sink.parse_error(e);
                return;
            },

            tokenizer::DoctypeToken(d) => Doctype(d),
            tokenizer::PIToken(x) => Pi(x),
            tokenizer::TagToken(x) => Tag(x),
            tokenizer::CommentToken(x) => Comment(x),
            tokenizer::NullCharacterToken => NullCharacter,
            tokenizer::EOFToken => Eof,
            tokenizer::CharacterTokens(x) => Characters(x),
        };

        self.process_to_completion(token);
    }

    fn end(&mut self) {
        for node in self.open_elems.drain(..).rev() {
            self.sink.pop(&node);
        }
    }

    fn query_state_change(&mut self) -> Option<tokenizer::states::XmlState> {
        self.next_tokenizer_state.take()
    }
}

fn current_node<Handle>(open_elems: &[Handle]) -> &Handle {
    open_elems.last().expect("no current element")
}

#[doc(hidden)]
impl<Handle, Sink> XmlTreeBuilder<Handle, Sink>
where
    Handle: Clone,
    Sink: TreeSink<Handle = Handle>,
{
    fn current_node(&self) -> &Handle {
        self.open_elems.last().expect("no current element")
    }

    fn insert_appropriately(&mut self, child: NodeOrText<Handle>) {
        let target = current_node(&self.open_elems);
        self.sink.append(target, child);
    }

    fn insert_tag(&mut self, tag: Tag) -> XmlProcessResult {
        let child = create_element(&mut self.sink, tag.name, tag.attrs);
        self.insert_appropriately(AppendNode(child.clone()));
        self.add_to_open_elems(child)
    }

    fn append_tag(&mut self, tag: Tag) -> XmlProcessResult {
        let child = create_element(&mut self.sink, tag.name, tag.attrs);
        self.insert_appropriately(AppendNode(child.clone()));
        self.sink.pop(&child);
        Done
    }

    fn append_tag_to_doc(&mut self, tag: Tag) -> Handle {
        let child = create_element(&mut self.sink, tag.name, tag.attrs);

        self.sink
            .append(&self.doc_handle, AppendNode(child.clone()));
        child
    }

    fn add_to_open_elems(&mut self, el: Handle) -> XmlProcessResult {
        self.open_elems.push(el);

        Done
    }

    fn append_comment_to_doc(&mut self, text: StrTendril) -> XmlProcessResult {
        let comment = self.sink.create_comment(text);
        self.sink.append(&self.doc_handle, AppendNode(comment));
        Done
    }

    fn append_comment_to_tag(&mut self, text: StrTendril) -> XmlProcessResult {
        let target = current_node(&self.open_elems);
        let comment = self.sink.create_comment(text);
        self.sink.append(target, AppendNode(comment));
        Done
    }

    fn append_doctype_to_doc(&mut self, doctype: Doctype) -> XmlProcessResult {
        fn get_tendril(opt: Option<StrTendril>) -> StrTendril {
            match opt {
                Some(expr) => expr,
                None => Tendril::new(),
            }
        }
        self.sink.append_doctype_to_document(
            get_tendril(doctype.name),
            get_tendril(doctype.public_id),
            get_tendril(doctype.system_id),
        );
        Done
    }

    fn append_pi_to_doc(&mut self, pi: Pi) -> XmlProcessResult {
        let pi = self.sink.create_pi(pi.target, pi.data);
        self.sink.append(&self.doc_handle, AppendNode(pi));
        Done
    }

    fn append_pi_to_tag(&mut self, pi: Pi) -> XmlProcessResult {
        let target = current_node(&self.open_elems);
        let pi = self.sink.create_pi(pi.target, pi.data);
        self.sink.append(target, AppendNode(pi));
        Done
    }

    fn append_text(&mut self, chars: StrTendril) -> XmlProcessResult {
        self.insert_appropriately(AppendText(chars));
        Done
    }

    fn tag_in_open_elems(&self, tag: &Tag) -> bool {
        self.open_elems
            .iter()
            .any(|a| self.sink.elem_name(a) == tag.name.expanded())
    }

    // Pop elements until an element from the set has been popped.  Returns the
    // number of elements popped.
    fn pop_until<P>(&mut self, pred: P)
    where
        P: Fn(ExpandedName) -> bool,
    {
        loop {
            if self.current_node_in(&pred) {
                break;
            }
            self.pop();
        }
    }

    fn current_node_in<TagSet>(&self, set: TagSet) -> bool
    where
        TagSet: Fn(ExpandedName) -> bool,
    {
        // FIXME: take namespace into consideration:
        set(self.sink.elem_name(self.current_node()))
    }

    fn close_tag(&mut self, tag: Tag) -> XmlProcessResult {
        debug!(
            "Close tag: current_node.name {:?} \n Current tag {:?}",
            self.sink.elem_name(self.current_node()),
            &tag.name
        );

        if *self.sink.elem_name(self.current_node()).local != tag.name.local {
            self.sink
                .parse_error(Borrowed("Current node doesn't match tag"));
        }

        let is_closed = self.tag_in_open_elems(&tag);

        if is_closed {
            self.pop_until(|p| p == tag.name.expanded());
            self.pop();
        }

        Done
    }

    fn no_open_elems(&self) -> bool {
        self.open_elems.is_empty()
    }

    fn pop(&mut self) -> Handle {
        self.namespace_stack.pop();
        let node = self.open_elems.pop().expect("no current element");
        self.sink.pop(&node);
        node
    }

    fn stop_parsing(&mut self) -> XmlProcessResult {
        warn!("stop_parsing for XML5 not implemented, full speed ahead!");
        Done
    }

    fn complete_script(&mut self) {
        let current = current_node(&self.open_elems);
        if self.sink.complete_script(current) == NextParserState::Suspend {
            self.next_tokenizer_state = Some(Quiescent);
        }
    }
}

fn any_not_whitespace(x: &StrTendril) -> bool {
    !x.bytes()
        .all(|b| matches!(b, b'\t' | b'\r' | b'\n' | b'\x0C' | b' '))
}

#[doc(hidden)]
impl<Handle, Sink> XmlTreeBuilder<Handle, Sink>
where
    Handle: Clone,
    Sink: TreeSink<Handle = Handle>,
{
    fn step(&mut self, mode: XmlPhase, token: Token) -> XmlProcessResult {
        self.debug_step(mode, &token);

        match mode {
            Start => match token {
                Tag(Tag {
                    kind: StartTag,
                    name,
                    attrs,
                }) => {
                    let tag = {
                        let mut tag = Tag {
                            kind: StartTag,
                            name,
                            attrs,
                        };
                        self.process_namespaces(&mut tag);
                        tag
                    };
                    self.phase = Main;
                    let handle = self.append_tag_to_doc(tag);
                    self.add_to_open_elems(handle)
                },
                Tag(Tag {
                    kind: EmptyTag,
                    name,
                    attrs,
                }) => {
                    let tag = {
                        let mut tag = Tag {
                            kind: EmptyTag,
                            name,
                            attrs,
                        };
                        self.process_namespaces(&mut tag);
                        tag
                    };
                    self.phase = End;
                    let handle = self.append_tag_to_doc(tag);
                    self.sink.pop(&handle);
                    Done
                },
                Comment(comment) => self.append_comment_to_doc(comment),
                Pi(pi) => self.append_pi_to_doc(pi),
                Characters(ref chars) if !any_not_whitespace(chars) => Done,
                Eof => {
                    self.sink
                        .parse_error(Borrowed("Unexpected EOF in start phase"));
                    Reprocess(End, Eof)
                },
                Doctype(d) => {
                    self.append_doctype_to_doc(d);
                    Done
                },
                _ => {
                    self.sink
                        .parse_error(Borrowed("Unexpected element in start phase"));
                    Done
                },
            },
            Main => match token {
                Characters(chs) => self.append_text(chs),
                Tag(Tag {
                    kind: StartTag,
                    name,
                    attrs,
                }) => {
                    let tag = {
                        let mut tag = Tag {
                            kind: StartTag,
                            name,
                            attrs,
                        };
                        self.process_namespaces(&mut tag);
                        tag
                    };
                    self.insert_tag(tag)
                },
                Tag(Tag {
                    kind: EmptyTag,
                    name,
                    attrs,
                }) => {
                    let tag = {
                        let mut tag = Tag {
                            kind: EmptyTag,
                            name,
                            attrs,
                        };
                        self.process_namespaces(&mut tag);
                        tag
                    };
                    if tag.name.local == local_name!("script") {
                        self.insert_tag(tag.clone());
                        self.complete_script();
                        self.close_tag(tag)
                    } else {
                        self.append_tag(tag)
                    }
                },
                Tag(Tag {
                    kind: EndTag,
                    name,
                    attrs,
                }) => {
                    let tag = {
                        let mut tag = Tag {
                            kind: EndTag,
                            name,
                            attrs,
                        };
                        self.process_namespaces(&mut tag);
                        tag
                    };
                    if tag.name.local == local_name!("script") {
                        self.complete_script();
                    }
                    let retval = self.close_tag(tag);
                    if self.no_open_elems() {
                        self.phase = End;
                    }
                    retval
                },
                Tag(Tag { kind: ShortTag, .. }) => {
                    self.pop();
                    if self.no_open_elems() {
                        self.phase = End;
                    }
                    Done
                },
                Comment(comment) => self.append_comment_to_tag(comment),
                Pi(pi) => self.append_pi_to_tag(pi),
                Eof | NullCharacter => Reprocess(End, Eof),
                Doctype(_) => {
                    self.sink
                        .parse_error(Borrowed("Unexpected element in main phase"));
                    Done
                },
            },
            End => match token {
                Comment(comment) => self.append_comment_to_doc(comment),
                Pi(pi) => self.append_pi_to_doc(pi),
                Characters(ref chars) if !any_not_whitespace(chars) => Done,
                Eof => self.stop_parsing(),
                _ => {
                    self.sink
                        .parse_error(Borrowed("Unexpected element in end phase"));
                    Done
                },
            },
        }
    }
}
