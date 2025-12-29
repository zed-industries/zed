//! Data types that track the change state for syntax nodes.

use crate::{
    hash::DftHashMap,
    parse::syntax::{Syntax, SyntaxId},
};

#[derive(PartialEq, Eq, Clone, Copy)]
pub(crate) enum ChangeKind<'a> {
    /// This node is shallowly unchanged. For lists, this means that
    /// the delimiters match, but there may still be some differences
    /// in the children between LHS and RHS.
    Unchanged(&'a Syntax<'a>),
    ReplacedComment(&'a Syntax<'a>, &'a Syntax<'a>),
    ReplacedString(&'a Syntax<'a>, &'a Syntax<'a>),
    Novel,
}

#[derive(Debug, Default)]
pub(crate) struct ChangeMap<'a> {
    changes: DftHashMap<SyntaxId, ChangeKind<'a>>,
}

impl<'a> ChangeMap<'a> {
    pub(crate) fn insert(&mut self, node: &'a Syntax<'a>, ck: ChangeKind<'a>) {
        self.changes.insert(node.id(), ck);
    }

    pub(crate) fn get(&self, node: &Syntax<'a>) -> Option<ChangeKind<'a>> {
        self.changes.get(&node.id()).copied()
    }
}

pub(crate) fn insert_deep_unchanged<'a>(
    node: &'a Syntax<'a>,
    opposite_node: &'a Syntax<'a>,
    change_map: &mut ChangeMap<'a>,
) {
    change_map.insert(node, ChangeKind::Unchanged(opposite_node));

    match (node, opposite_node) {
        (
            Syntax::List {
                children: node_children,
                ..
            },
            Syntax::List {
                children: opposite_children,
                ..
            },
        ) => {
            for (child, opposite_child) in node_children.iter().zip(opposite_children) {
                insert_deep_unchanged(child, opposite_child, change_map);
            }
        }
        (Syntax::Atom { .. }, Syntax::Atom { .. }) => {}
        _ => unreachable!("Unchanged nodes should be both lists, or both atoms"),
    }
}

pub(crate) fn insert_deep_novel<'a>(node: &'a Syntax<'a>, change_map: &mut ChangeMap<'a>) {
    change_map.insert(node, ChangeKind::Novel);

    if let Syntax::List { children, .. } = node {
        for child in children.iter() {
            insert_deep_novel(child, change_map);
        }
    }
}
