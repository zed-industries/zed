// Copyright 2014-2017 The html5ever Project Developers. See the
// COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#[macro_use]
extern crate html5ever;

use std::borrow::Cow;
use std::cell::{Cell, Ref, RefCell};
use std::collections::HashMap;
use std::io;

use html5ever::parse_document;
use html5ever::tendril::*;
use html5ever::tree_builder::{
    AppendNode, AppendText, ElementFlags, NodeOrText, QuirksMode, TreeSink,
};
use html5ever::{Attribute, QualName};

struct Sink {
    next_id: Cell<usize>,
    names: RefCell<HashMap<usize, &'static QualName>>,
}

impl Sink {
    fn get_id(&self) -> usize {
        let id = self.next_id.get();
        self.next_id.set(id + 2);
        id
    }
}

impl TreeSink for Sink {
    type Handle = usize;
    type Output = Self;
    type ElemName<'a> = Ref<'a, QualName>;
    fn finish(self) -> Self {
        self
    }

    fn parse_error(&self, msg: Cow<'static, str>) {
        println!("Parse error: {msg}");
    }

    fn get_document(&self) -> usize {
        0
    }

    fn get_template_contents(&self, target: &usize) -> usize {
        if let Some(expanded_name!(html "template")) =
            self.names.borrow().get(target).map(|n| n.expanded())
        {
            target + 1
        } else {
            panic!("not a template element")
        }
    }

    fn set_quirks_mode(&self, mode: QuirksMode) {
        println!("Set quirks mode to {mode:?}");
    }

    fn same_node(&self, x: &usize, y: &usize) -> bool {
        x == y
    }

    fn elem_name(&self, target: &usize) -> Self::ElemName<'_> {
        Ref::map(self.names.borrow(), |map| {
            *map.get(target).expect("not an element")
        })
    }

    fn create_element(&self, name: QualName, _: Vec<Attribute>, _: ElementFlags) -> usize {
        let id = self.get_id();
        println!("Created {name:?} as {id}");
        // N.B. We intentionally leak memory here to minimize the implementation complexity
        //      of this example code. A real implementation would either want to use a real
        //      real DOM tree implentation, or else use an arena as the backing store for
        //      memory used by the parser.
        self.names
            .borrow_mut()
            .insert(id, Box::leak(Box::new(name)));
        id
    }

    fn create_comment(&self, text: StrTendril) -> usize {
        let id = self.get_id();
        println!("Created comment \"{}\" as {}", text.escape_default(), id);
        id
    }

    #[allow(unused_variables)]
    fn create_pi(&self, target: StrTendril, value: StrTendril) -> usize {
        unimplemented!()
    }

    fn append(&self, parent: &usize, child: NodeOrText<usize>) {
        match child {
            AppendNode(n) => println!("Append node {n} to {parent}"),
            AppendText(t) => println!("Append text to {}: \"{}\"", parent, t.escape_default()),
        }
    }

    fn append_before_sibling(&self, sibling: &usize, new_node: NodeOrText<usize>) {
        match new_node {
            AppendNode(n) => println!("Append node {n} before {sibling}"),
            AppendText(t) => println!("Append text before {}: \"{}\"", sibling, t.escape_default()),
        }
    }

    fn append_based_on_parent_node(
        &self,
        element: &Self::Handle,
        _prev_element: &Self::Handle,
        child: NodeOrText<Self::Handle>,
    ) {
        self.append_before_sibling(element, child);
    }

    fn append_doctype_to_document(
        &self,
        name: StrTendril,
        public_id: StrTendril,
        system_id: StrTendril,
    ) {
        println!("Append doctype: {name} {public_id} {system_id}");
    }

    fn add_attrs_if_missing(&self, target: &usize, attrs: Vec<Attribute>) {
        assert!(self.names.borrow().contains_key(target), "not an element");
        println!("Add missing attributes to {target}:");
        for attr in attrs.into_iter() {
            println!("    {:?} = {}", attr.name, attr.value);
        }
    }

    fn associate_with_form(
        &self,
        _target: &usize,
        _form: &usize,
        _nodes: (&usize, Option<&usize>),
    ) {
        // No form owner support.
    }

    fn remove_from_parent(&self, target: &usize) {
        println!("Remove {target} from parent");
    }

    fn reparent_children(&self, node: &usize, new_parent: &usize) {
        println!("Move children from {node} to {new_parent}");
    }

    fn mark_script_already_started(&self, node: &usize) {
        println!("Mark script {node} as already started");
    }

    fn set_current_line(&self, line_number: u64) {
        println!("Set current line to {line_number}");
    }

    fn pop(&self, elem: &usize) {
        println!("Popped element {elem}");
    }
}

/// Same example as the "noop-tree-builder", but this time every function implemented in our
/// Sink object prints a log, so it's easier to get an understating of when each function is
/// called.
fn main() {
    let sink = Sink {
        next_id: Cell::new(1),
        names: RefCell::new(HashMap::new()),
    };
    let stdin = io::stdin();
    parse_document(sink, Default::default())
        .from_utf8()
        .read_from(&mut stdin.lock())
        .unwrap();
}
