use anyhow::{anyhow, Result};
use indexmap::IndexMap;
use serde_json::Value;
use std::{
    cell::RefCell,
    mem,
    rc::{Rc, Weak},
};

pub fn resolve_references(value: Value) -> Result<Value> {
    let tree = Tree::from_json(value)?;
    tree.resolve()?;
    tree.to_json()
}

#[derive(Clone)]
enum Node {
    Reference {
        path: String,
        parent: Option<Weak<RefCell<Node>>>,
    },
    Object {
        base: Option<String>,
        children: IndexMap<String, Tree>,
        resolved: bool,
        parent: Option<Weak<RefCell<Node>>>,
    },
    Array {
        children: Vec<Tree>,
        resolved: bool,
        parent: Option<Weak<RefCell<Node>>>,
    },
    String(String),
    Number(serde_json::Number),
    Bool(bool),
    Null,
}

#[derive(Clone)]
struct Tree(Rc<RefCell<Node>>);

impl Tree {
    pub fn new(node: Node) -> Self {
        Self(Rc::new(RefCell::new(node)))
    }

    fn from_json(value: Value) -> Result<Self> {
        match value {
            Value::String(value) => {
                if let Some(path) = value.strip_prefix("$") {
                    Ok(Self::new(Node::Reference {
                        path: path.to_string(),
                        parent: None,
                    }))
                } else {
                    Ok(Self::new(Node::String(value)))
                }
            }
            Value::Number(value) => Ok(Self::new(Node::Number(value))),
            Value::Bool(value) => Ok(Self::new(Node::Bool(value))),
            Value::Null => Ok(Self::new(Node::Null)),
            Value::Object(object) => {
                let tree = Self::new(Node::Object {
                    base: Default::default(),
                    children: Default::default(),
                    resolved: false,
                    parent: None,
                });
                let mut children = IndexMap::new();
                let mut resolved = true;
                let mut base = None;
                for (key, value) in object.into_iter() {
                    let value = if key == "extends" {
                        if value.is_string() {
                            if let Value::String(value) = value {
                                base = value.strip_prefix("$").map(str::to_string);
                                resolved = false;
                                Self::new(Node::String(value))
                            } else {
                                unreachable!()
                            }
                        } else {
                            Tree::from_json(value)?
                        }
                    } else {
                        Tree::from_json(value)?
                    };
                    value
                        .0
                        .borrow_mut()
                        .set_parent(Some(Rc::downgrade(&tree.0)));
                    resolved &= value.is_resolved();
                    children.insert(key.clone(), value);
                }

                *tree.0.borrow_mut() = Node::Object {
                    base,
                    children,
                    resolved,
                    parent: None,
                };
                Ok(tree)
            }
            Value::Array(elements) => {
                let tree = Self::new(Node::Array {
                    children: Default::default(),
                    resolved: false,
                    parent: None,
                });

                let mut children = Vec::new();
                let mut resolved = true;
                for element in elements {
                    let child = Tree::from_json(element)?;
                    child
                        .0
                        .borrow_mut()
                        .set_parent(Some(Rc::downgrade(&tree.0)));
                    resolved &= child.is_resolved();
                    children.push(child);
                }

                *tree.0.borrow_mut() = Node::Array {
                    children,
                    resolved,
                    parent: None,
                };
                Ok(tree)
            }
        }
    }

    fn to_json(&self) -> Result<Value> {
        match &*self.0.borrow() {
            Node::Reference { .. } => Err(anyhow!("unresolved tree")),
            Node::String(value) => Ok(Value::String(value.clone())),
            Node::Number(value) => Ok(Value::Number(value.clone())),
            Node::Bool(value) => Ok(Value::Bool(*value)),
            Node::Null => Ok(Value::Null),
            Node::Object { children, .. } => {
                let mut json_children = serde_json::Map::new();
                for (key, value) in children {
                    json_children.insert(key.clone(), value.to_json()?);
                }
                Ok(Value::Object(json_children))
            }
            Node::Array { children, .. } => {
                let mut json_children = Vec::new();
                for child in children {
                    json_children.push(child.to_json()?);
                }
                Ok(Value::Array(json_children))
            }
        }
    }

    fn parent(&self) -> Option<Tree> {
        match &*self.0.borrow() {
            Node::Reference { parent, .. }
            | Node::Object { parent, .. }
            | Node::Array { parent, .. } => parent.as_ref().and_then(|p| p.upgrade()).map(Tree),
            _ => None,
        }
    }

    fn get(&self, path: &str) -> Result<Option<Tree>> {
        let mut tree = self.clone();
        for component in path.split('.') {
            let node = tree.0.borrow();
            match &*node {
                Node::Object { children, .. } => {
                    if let Some(subtree) = children.get(component).cloned() {
                        drop(node);
                        tree = subtree;
                    } else {
                        return Err(anyhow!(
                            "key \"{}\" does not exist in path \"{}\"",
                            component,
                            path
                        ));
                    }
                }
                Node::Reference { .. } => return Ok(None),
                Node::Array { .. }
                | Node::String(_)
                | Node::Number(_)
                | Node::Bool(_)
                | Node::Null => {
                    return Err(anyhow!(
                        "key \"{}\" in path \"{}\" is not an object",
                        component,
                        path
                    ))
                }
            }
        }

        Ok(Some(tree))
    }

    fn is_resolved(&self) -> bool {
        match &*self.0.borrow() {
            Node::Reference { .. } => false,
            Node::Object { resolved, .. } | Node::Array { resolved, .. } => *resolved,
            Node::String(_) | Node::Number(_) | Node::Bool(_) | Node::Null => true,
        }
    }

    fn update_resolved(&self) {
        match &mut *self.0.borrow_mut() {
            Node::Object {
                resolved, children, ..
            } => {
                *resolved = children.values().all(|c| c.is_resolved());
            }
            Node::Array {
                resolved, children, ..
            } => {
                *resolved = children.iter().all(|c| c.is_resolved());
            }
            _ => {}
        }
    }

    pub fn resolve(&self) -> Result<()> {
        let mut unresolved = vec![self.clone()];
        let mut made_progress = true;

        while made_progress && !unresolved.is_empty() {
            made_progress = false;
            for mut tree in mem::take(&mut unresolved) {
                made_progress |= tree.resolve_subtree(self, &mut unresolved)?;
                if tree.is_resolved() {
                    while let Some(parent) = tree.parent() {
                        parent.update_resolved();
                        tree = parent;
                    }
                }
            }
        }

        if unresolved.is_empty() {
            Ok(())
        } else {
            Err(anyhow!("tree contains cycles"))
        }
    }

    fn resolve_subtree(&self, root: &Tree, unresolved: &mut Vec<Tree>) -> Result<bool> {
        let mut made_progress = false;
        let borrow = self.0.borrow();
        match &*borrow {
            Node::Reference { path, parent } => {
                if let Some(subtree) = root.get(&path)? {
                    if subtree.is_resolved() {
                        let parent = parent.clone();
                        drop(borrow);
                        let mut new_node = subtree.0.borrow().clone();
                        new_node.set_parent(parent);
                        *self.0.borrow_mut() = new_node;
                        Ok(true)
                    } else {
                        unresolved.push(self.clone());
                        Ok(false)
                    }
                } else {
                    unresolved.push(self.clone());
                    Ok(false)
                }
            }
            Node::Object {
                base,
                children,
                resolved,
                ..
            } => {
                if *resolved {
                    Ok(false)
                } else {
                    let mut children_resolved = true;
                    for child in children.values() {
                        made_progress |= child.resolve_subtree(root, unresolved)?;
                        children_resolved &= child.is_resolved();
                    }

                    if children_resolved {
                        let mut has_base = false;
                        let mut resolved_base = None;
                        if let Some(base) = base {
                            has_base = true;
                            if let Some(base) = root.get(base)? {
                                if base.is_resolved() {
                                    resolved_base = Some(base);
                                }
                            }
                        }

                        drop(borrow);

                        if let Some(base) = resolved_base.as_ref() {
                            self.extend_from(&base);
                        }

                        if let Node::Object { resolved, .. } = &mut *self.0.borrow_mut() {
                            if has_base {
                                if resolved_base.is_some() {
                                    *resolved = true;
                                } else {
                                    unresolved.push(self.clone());
                                }
                            } else {
                                *resolved = true;
                            }
                        }
                    }

                    Ok(made_progress)
                }
            }
            Node::Array {
                children, resolved, ..
            } => {
                if *resolved {
                    Ok(false)
                } else {
                    let mut children_resolved = true;
                    for child in children.iter() {
                        made_progress |= child.resolve_subtree(root, unresolved)?;
                        children_resolved &= child.is_resolved();
                    }

                    if children_resolved {
                        drop(borrow);

                        if let Node::Array { resolved, .. } = &mut *self.0.borrow_mut() {
                            *resolved = true;
                        }
                    }

                    Ok(made_progress)
                }
            }
            Node::String(_) | Node::Number(_) | Node::Bool(_) | Node::Null => {
                return Ok(false);
            }
        }
    }

    fn extend_from(&self, base: &Tree) {
        if Rc::ptr_eq(&self.0, &base.0) {
            return;
        }

        if let (
            Node::Object { children, .. },
            Node::Object {
                children: base_children,
                ..
            },
        ) = (&mut *self.0.borrow_mut(), &*base.0.borrow())
        {
            for (key, base_value) in base_children {
                if let Some(value) = children.get(key) {
                    value.extend_from(base_value);
                } else {
                    let base_value = base_value.clone();
                    base_value
                        .0
                        .borrow_mut()
                        .set_parent(Some(Rc::downgrade(&self.0)));
                    children.insert(key.clone(), base_value);
                }
            }
        }
    }
}

impl Node {
    fn set_parent(&mut self, new_parent: Option<Weak<RefCell<Node>>>) {
        match self {
            Node::Reference { parent, .. }
            | Node::Object { parent, .. }
            | Node::Array { parent, .. } => *parent = new_parent,
            _ => {}
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_references() {
        let json = serde_json::json!({
            "a": {
                "x": "$b.d"
            },
            "b": {
                "c": "$a",
                "d": "$e.f"
            },
            "e": {
                "extends": "$a",
                "f": "1"
            }
        });

        assert_eq!(
            resolve_references(json).unwrap(),
            serde_json::json!({
            "e": {
              "f": "1",
              "x": "1"
            },
            "a": {
              "x": "1"
            },
            "b": {
              "c": {
                "x": "1"
              },
              "d": "1"
            }})
        )
    }

    #[test]
    fn test_cycles() {
        let json = serde_json::json!({
            "a": {
                "b": "$c.d"
            },
            "c": {
                "d": "$a.b",
            },
        });

        assert!(resolve_references(json).is_err());
    }
}
