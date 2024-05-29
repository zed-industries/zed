use markup5ever_rcdom::Handle;

pub trait Visitor: Sized {
    type Error;

    fn visit_node(&mut self, node: &Handle) -> Result<(), Self::Error> {
        walk_node(self, node)
    }

    fn visit_children<'a>(
        &mut self,
        children: impl Iterator<Item = &'a Handle>,
    ) -> Result<(), Self::Error> {
        walk_children(self, children)
    }
}

pub fn walk_node<V: Visitor>(visitor: &mut V, node: &Handle) -> Result<(), V::Error> {
    visitor.visit_children(node.children.borrow().iter())
}

pub fn walk_children<'a, V: Visitor>(
    visitor: &mut V,
    children: impl Iterator<Item = &'a Handle>,
) -> Result<(), V::Error> {
    for child in children {
        visitor.visit_node(child)?;
    }

    Ok(())
}
