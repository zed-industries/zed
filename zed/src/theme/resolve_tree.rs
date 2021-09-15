use anyhow::{anyhow, Result};
use serde_json::Value;
use std::{
    cell::RefCell,
    collections::HashMap,
    mem,
    rc::{Rc, Weak},
};

pub fn resolve(value: Value) -> Result<Value> {
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
        children: HashMap<String, Tree>,
        resolved: bool,
        parent: Option<Weak<RefCell<Node>>>,
    },
    String {
        value: String,
        parent: Option<Weak<RefCell<Node>>>,
    },
}

#[derive(Clone)]
struct Tree(Rc<RefCell<Node>>);

impl Tree {
    pub fn new(node: Node) -> Self {
        Self(Rc::new(RefCell::new(node)))
    }

    fn from_json(value: Value) -> Result<Self> {
        match value {
            Value::String(s) => {
                if let Some(path) = s.strip_prefix("$") {
                    Ok(Self::new(Node::Reference {
                        path: path.to_string(),
                        parent: None,
                    }))
                } else {
                    Ok(Self::new(Node::String {
                        value: s,
                        parent: None,
                    }))
                }
            }
            Value::Object(object) => {
                let mut tree = Self::new(Node::Object {
                    base: Default::default(),
                    children: Default::default(),
                    resolved: false,
                    parent: None,
                });
                let mut children = HashMap::new();
                let mut resolved = true;
                let mut base = None;
                for (key, value) in object.into_iter() {
                    if key == "extends" {
                        if let Value::String(s) = value {
                            base = Some(s);
                            resolved = false;
                        }
                    } else {
                        let value = Tree::from_json(value)?;
                        value
                            .0
                            .borrow_mut()
                            .set_parent(Some(Rc::downgrade(&tree.0)));
                        resolved &= value.is_resolved();
                        children.insert(key.clone(), value);
                    }
                }

                *tree.0.borrow_mut() = Node::Object {
                    base,
                    children,
                    resolved,
                    parent: None,
                };
                Ok(tree)
            }
            _ => return Err(anyhow!("unsupported json type")),
        }
    }

    fn to_json(&self) -> Result<Value> {
        match &*self.0.borrow() {
            Node::Reference { .. } => Err(anyhow!("unresolved tree")),
            Node::String { value, .. } => Ok(Value::String(value.clone())),
            Node::Object { children, .. } => {
                let mut json_children = serde_json::Map::new();
                for (key, value) in children {
                    json_children.insert(key.clone(), value.to_json()?);
                }
                Ok(Value::Object(json_children))
            }
            _ => unimplemented!(),
        }
    }

    fn parent(&self) -> Option<Tree> {
        match &*self.0.borrow() {
            Node::Reference { parent, .. }
            | Node::Object { parent, .. }
            | Node::String { parent, .. } => parent.as_ref().and_then(|p| p.upgrade()).map(Tree),
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
                        return Err(anyhow!("key does not exist"));
                    }
                }
                Node::Reference { .. } => return Ok(None),
                Node::String { .. } => return Err(anyhow!("component is not an object")),
            }
        }

        Ok(Some(tree))
    }

    fn is_resolved(&self) -> bool {
        match &*self.0.borrow() {
            Node::Reference { .. } => false,
            Node::Object { resolved, .. } => *resolved,
            Node::String { .. } => true,
        }
    }

    fn update_resolved(&self) {
        match &mut *self.0.borrow_mut() {
            Node::Object {
                resolved, children, ..
            } => {
                *resolved = children.values().all(|c| c.is_resolved());
            }
            _ => {}
        }
    }

    pub fn resolve(&self) -> Result<()> {
        let mut unresolved = vec![self.clone()];
        let mut made_progress = true;
        while made_progress && !unresolved.is_empty() {
            made_progress = false;
            dbg!("===========");
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
            Err(anyhow!("could not resolve tree"))
        }
    }

    fn resolve_subtree(&self, root: &Tree, unresolved: &mut Vec<Tree>) -> Result<bool> {
        let mut made_progress = false;
        let borrow = self.0.borrow();
        match &*borrow {
            Node::Reference { path, parent } => {
                print!("entering reference ${}: ", path);
                if let Some(subtree) = root.get(&path)? {
                    if subtree.is_resolved() {
                        println!("resolved");
                        let parent = parent.clone();
                        drop(borrow);
                        let mut new_node = subtree.0.borrow().clone();
                        new_node.set_parent(parent);
                        *self.0.borrow_mut() = new_node;
                        Ok(true)
                    } else {
                        println!("unresolved (but existing)");
                        unresolved.push(self.clone());
                        Ok(false)
                    }
                } else {
                    println!("unresolved (referant does not exist)");
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
                    println!("already resolved");
                    Ok(false)
                } else {
                    let mut children_resolved = true;
                    for (key, child) in children.iter() {
                        println!("resolving subtree {}", key);
                        made_progress |= child.resolve_subtree(root, unresolved)?;
                        children_resolved &= child.is_resolved();
                    }

                    if children_resolved {
                        drop(borrow);
                        if let Node::Object { resolved, .. } = &mut *self.0.borrow_mut() {
                            *resolved = true;
                        }
                    }

                    Ok(made_progress)
                }
            }
            Node::String { value, .. } => {
                println!("terminating at string: {}", value);
                return Ok(false);
            }
        }
    }
}

impl Node {
    fn set_parent(&mut self, new_parent: Option<Weak<RefCell<Node>>>) {
        match self {
            Node::Reference { parent, .. }
            | Node::Object { parent, .. }
            | Node::String { parent, .. } => *parent = new_parent,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_basic() {
        let json = serde_json::json!({
            "a": {
                "x": "$b.d"
            },
            "b": {
                "c": "$a",
                "d": "$e.f"
            },
            "e": {
                "f": "1"
            }
        });

        dbg!(resolve(json).unwrap());
    }
}
