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
use std::cell::{Cell, RefCell};
use std::collections::HashMap;
use std::io;

use html5ever::parse_document;
use html5ever::tendril::*;
use html5ever::tree_builder::{ElementFlags, NodeOrText, QuirksMode, TreeSink};
use html5ever::{Attribute, ExpandedName, QualName};

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

/// By implementing the TreeSink trait we determine how the data from the tree building step
/// is processed. In this case the DOM elements are written into the "names" hashmap.
///
/// For deeper understating of each function go to the TreeSink declaration.
impl TreeSink for Sink {
    type Handle = usize;
    type Output = Self;
    type ElemName<'a> = ExpandedName<'a>;
    fn finish(self) -> Self {
        self
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

    fn same_node(&self, x: &usize, y: &usize) -> bool {
        x == y
    }

    fn elem_name(&self, target: &usize) -> ExpandedName {
        self.names
            .borrow()
            .get(target)
            .expect("not an element")
            .expanded()
    }

    fn create_element(&self, name: QualName, _: Vec<Attribute>, _: ElementFlags) -> usize {
        let id = self.get_id();
        // N.B. We intentionally leak memory here to minimize the implementation complexity
        //      of this example code. A real implementation would either want to use a real
        //      real DOM tree implentation, or else use an arena as the backing store for
        //      memory used by the parser.
        self.names
            .borrow_mut()
            .insert(id, Box::leak(Box::new(name)));
        id
    }

    fn create_comment(&self, _text: StrTendril) -> usize {
        self.get_id()
    }

    #[allow(unused_variables)]
    fn create_pi(&self, target: StrTendril, value: StrTendril) -> usize {
        unimplemented!()
    }

    fn append_before_sibling(&self, _sibling: &usize, _new_node: NodeOrText<usize>) {}

    fn append_based_on_parent_node(
        &self,
        _element: &usize,
        _prev_element: &usize,
        _new_node: NodeOrText<usize>,
    ) {
    }

    fn parse_error(&self, _msg: Cow<'static, str>) {}
    fn set_quirks_mode(&self, _mode: QuirksMode) {}
    fn append(&self, _parent: &usize, _child: NodeOrText<usize>) {}

    fn append_doctype_to_document(&self, _: StrTendril, _: StrTendril, _: StrTendril) {}
    fn add_attrs_if_missing(&self, target: &usize, _attrs: Vec<Attribute>) {
        assert!(self.names.borrow().contains_key(target), "not an element");
    }
    fn remove_from_parent(&self, _target: &usize) {}
    fn reparent_children(&self, _node: &usize, _new_parent: &usize) {}
    fn mark_script_already_started(&self, _node: &usize) {}
}

/// In this example we implement the TreeSink trait which takes each parsed elements and insert
/// it to a hashmap, while each element is given a numeric id.
fn main() {
    let sink = Sink {
        next_id: Cell::new(1),
        names: RefCell::new(HashMap::new()),
    };

    // Read HTML from the standard input and parse it
    let stdin = io::stdin();
    parse_document(sink, Default::default())
        .from_utf8()
        .read_from(&mut stdin.lock())
        .unwrap();
}
