use std::cell::RefCell;

use html5ever::tendril::StrTendril;
use html5ever::{Attribute, QualName};
use markup5ever_rcdom::{Handle, NodeData};

/// A visitor for HTML nodes.
pub trait Visitor: Sized {
    /// The type of error this visitor returns.
    type Error;

    /// Visits the given HTML node.
    fn visit_node(&mut self, node: &Handle) -> Result<(), Self::Error> {
        walk_node(self, node)
    }

    /// Visits the given [`NodeData`].
    fn visit_node_data(&mut self, data: &NodeData) -> Result<(), Self::Error> {
        walk_node_data(self, data)
    }

    /// Visits the given element.
    fn visit_element(
        &mut self,
        _name: &QualName,
        _attrs: &RefCell<Vec<Attribute>>,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    /// Visits the given text.
    fn visit_text(&mut self, _contents: &RefCell<StrTendril>) -> Result<(), Self::Error> {
        Ok(())
    }

    /// Visits the children of an HTML node.
    fn visit_children<'a>(
        &mut self,
        children: impl Iterator<Item = &'a Handle>,
    ) -> Result<(), Self::Error> {
        walk_children(self, children)
    }
}

/// Walks the given HTML node.
pub fn walk_node<V: Visitor>(visitor: &mut V, node: &Handle) -> Result<(), V::Error> {
    visitor.visit_node_data(&node.data)?;
    visitor.visit_children(node.children.borrow().iter())
}

/// Walks the given [`NodeData`].
pub fn walk_node_data<V: Visitor>(visitor: &mut V, data: &NodeData) -> Result<(), V::Error> {
    match data {
        NodeData::Document
        | NodeData::Doctype { .. }
        | NodeData::ProcessingInstruction { .. }
        | NodeData::Comment { .. } => {
            // Currently left unimplemented, as we're not interested in this data
            // at this time.
        }
        NodeData::Element { name, attrs, .. } => visitor.visit_element(name, attrs)?,
        NodeData::Text { contents } => visitor.visit_text(contents)?,
    }

    Ok(())
}

/// Walks the given children.
pub fn walk_children<'a, V: Visitor>(
    visitor: &mut V,
    children: impl Iterator<Item = &'a Handle>,
) -> Result<(), V::Error> {
    for child in children {
        visitor.visit_node(child)?;
    }

    Ok(())
}
