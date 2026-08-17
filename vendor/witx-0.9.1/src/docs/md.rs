use std::{
    any::Any,
    cell::{self, RefCell},
    fmt,
    rc::{Rc, Weak},
};

/// Helper trait which simplifies generation of the Markdown document represented
/// as a tree of `MdNodeRef`s.
pub(super) trait ToMarkdown {
    /// Drives the generation of the `MdNodeRef` tree by either mutating
    /// the outer (parent) `MdNodeRef`, shared reference to the `MdNode` `node`,
    /// or spawning new child `MdNodeRef` references to nodes.
    fn generate(&self, node: MdNodeRef);
}

/// Interface required for any "content" that is expected to be generated into a
/// Markdown valid format, hence the constraint of `fmt::Display`.
///
/// In essence, any AST element that is meant to be rendered into Markdown, should
/// define a type implementing this trait.
pub(super) trait MdElement: fmt::Display + fmt::Debug + 'static {
    /// Returns `Some(id)` of this `MdElement`. Here `id` is synonym for a Markdown actionable
    /// link.
    fn id(&self) -> Option<&str>;

    /// Returns `Some(docs)`, the "docs" of this `MdElement`.
    fn docs(&self) -> Option<&str>;

    /// Sets `docs`, the "docs" of this `MdElement`.
    fn set_docs(&mut self, docs: &str);

    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

/// A Markdown node containing:
/// * the Markdown renderable `content`,
/// * a weak reference to the `parent` `MdNode` (if any), and
/// * children `MdNodeRef`s
///
/// `content` is expected to implement the `MdElement` trait.
#[derive(Debug)]
pub(super) struct MdNode {
    content: Box<dyn MdElement>,
    parent: Option<Weak<RefCell<MdNode>>>,
    children: Vec<MdNodeRef>,
}

/// Helper function for walking the tree up from some starting `MdNode`, all the way up
/// to the root of the tree.
fn walk_ancestors(parent: Option<&Weak<RefCell<MdNode>>>, cb: &mut impl FnMut(MdNodeRef)) {
    if let Some(parent) = parent.and_then(|x| x.upgrade()) {
        cb(parent.clone().into());
        walk_ancestors(parent.borrow().parent.as_ref(), cb)
    }
}

impl MdNode {
    fn new<T: MdElement + 'static>(item: T) -> Self {
        Self {
            content: Box::new(item),
            parent: None,
            children: vec![],
        }
    }

    /// Returns all ancestors of this `MdNode` all the way to the tree's root.
    pub fn ancestors(&self) -> Vec<MdNodeRef> {
        let mut ancestors = Vec::new();
        walk_ancestors(self.parent.as_ref(), &mut |parent| ancestors.push(parent));
        ancestors
    }

    /// Returns all children of this `MdNode` in a BFS order.
    pub fn children(&self) -> Vec<MdNodeRef> {
        let mut children = self.children.clone();
        for child in &self.children {
            children.append(&mut child.borrow().children());
        }
        children
    }
}

impl fmt::Display for MdNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.content.fmt(f)?;

        for child in &self.children {
            child.fmt(f)?;
        }

        Ok(())
    }
}

/// Helper struct for storing a shared mutable reference to `MdNode`.
#[derive(Debug)]
pub(super) struct MdNodeRef(Rc<RefCell<MdNode>>);

impl MdNodeRef {
    pub fn new<T: MdElement + 'static>(item: T) -> Self {
        Self(Rc::new(RefCell::new(MdNode::new(item))))
    }

    /// Spawns new `MdNode` child node, automatically wrapping it in a
    /// `MdNodeRef` and creating a weak link from child to itself.
    pub fn new_child<T: MdElement + 'static>(&self, item: T) -> Self {
        let mut child_node = MdNode::new(item);
        child_node.parent = Some(Rc::downgrade(&self.0));
        let child_ref = Self(Rc::new(RefCell::new(child_node)));
        self.borrow_mut().children.push(child_ref.clone());
        child_ref
    }

    pub fn borrow(&self) -> cell::Ref<MdNode> {
        self.0.borrow()
    }

    pub fn borrow_mut(&self) -> cell::RefMut<MdNode> {
        self.0.borrow_mut()
    }

    /// Returns an immutable reference to `MdNode`'s `content` as-is, that
    /// is as some type implementing the `MdElement` trait.
    pub fn any_ref(&self) -> cell::Ref<Box<dyn MdElement>> {
        cell::Ref::map(self.borrow(), |b| &b.content)
    }

    /// Returns a mutable reference to `MdNode`'s `content` as-is, that
    /// is as some type implementing the `MdElement` trait.
    pub fn any_ref_mut(&self) -> cell::RefMut<Box<dyn MdElement>> {
        cell::RefMut::map(self.borrow_mut(), |b| &mut b.content)
    }

    /// Returns a mutable reference to `MdNode`'s `content` cast to some type
    /// `T` which implements `MdElement` trait.
    ///
    /// Panics if `content` cannot be downcast to `T`.
    pub fn content_ref_mut<T: MdElement + 'static>(&self) -> cell::RefMut<T> {
        cell::RefMut::map(self.borrow_mut(), |b| {
            let r = b.content.as_any_mut();
            r.downcast_mut::<T>().expect("reference is not T type")
        })
    }
}

impl Clone for MdNodeRef {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl From<Rc<RefCell<MdNode>>> for MdNodeRef {
    fn from(node: Rc<RefCell<MdNode>>) -> Self {
        Self(node)
    }
}

impl fmt::Display for MdNodeRef {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.borrow().fmt(f)
    }
}

/// Record representing the Markdown tree's root.
///
/// Doesn't render to anything.
#[derive(Debug, Default)]
pub(super) struct MdRoot;

impl MdElement for MdRoot {
    fn id(&self) -> Option<&str> {
        None
    }

    fn docs(&self) -> Option<&str> {
        None
    }

    fn set_docs(&mut self, _: &str) {}

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

impl fmt::Display for MdRoot {
    fn fmt(&self, _f: &mut fmt::Formatter) -> fmt::Result {
        Ok(())
    }
}

/// Helper enum representing either a Markdown header "#" nested at some
/// `level` down the tree, or a bullet "-" in a list which is idempotent
/// to changing the nesting level.
#[derive(Debug, Clone, Copy)]
pub(super) enum MdHeading {
    Header { level: usize },
    Bullet,
}

impl MdHeading {
    /// Creates new instance of `MdHeading::Header` variant nested at some
    /// `level` down the Markdown tree.
    pub fn new_header(level: usize) -> Self {
        MdHeading::Header { level }
    }

    /// Creates new instance of `MdHeading::Bullet` variant.
    pub fn new_bullet() -> Self {
        MdHeading::Bullet
    }

    /// Copies `MdHeading` and if `MdHeading::Header`, pushes it down one
    /// level in the Markdown tree by incrementing `level`.
    pub fn new_level_down(&self) -> Self {
        let mut copy = *self;
        if let Self::Header { ref mut level } = &mut copy {
            *level += 1;
        }
        copy
    }
}

impl fmt::Display for MdHeading {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let as_string = match self {
            Self::Header { level } => "#".repeat(*level),
            Self::Bullet => "-".to_owned(),
        };
        f.write_str(&as_string)
    }
}

/// Record representing a Markdown section without any `docs`, consisting
/// of only a `header` (e.g., "###"), maybe some referencable `id` (i.e.,
/// a Markdown link), and some `title`.
///
/// Example rendering:
///
/// ### Typenames
///
#[derive(Debug)]
pub(super) struct MdSection {
    pub heading: MdHeading,
    pub id: Option<String>,
    pub title: String,
}

impl MdSection {
    pub fn new<S: AsRef<str>>(heading: MdHeading, title: S) -> Self {
        Self {
            heading,
            id: None,
            title: title.as_ref().to_owned(),
        }
    }
}

impl MdElement for MdSection {
    fn id(&self) -> Option<&str> {
        self.id.as_ref().map(|s| s.as_str())
    }

    fn docs(&self) -> Option<&str> {
        None
    }

    fn set_docs(&mut self, _: &str) {}

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

fn gen_link<S: AsRef<str>>(id: S) -> String {
    format!("<a href=\"#{id}\" name=\"{id}\"></a>", id = id.as_ref())
}

impl fmt::Display for MdSection {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_fmt(format_args!("{} ", self.heading))?;

        if let Some(id) = &self.id {
            f.write_fmt(format_args!("{} ", gen_link(id)))?;
        }

        writeln!(f, "{}", self.title)
    }
}

/// Record representing a Markdown section representing any `NamedType` element
/// of the AST.
/// Consists of:
/// * `header`, e.g., "###", or "-" for Enum variants, etc.,
/// * referencable `id`,
/// * some `name`, e.g., `errno`,
/// * `docs` paragraph, and
/// * maybe `MdType`.
///
/// Example rendering (recursive):
///
/// ### <a href="#errno" name="errno"></a> `errno`: Enum(`u16`)
/// Error codes returned by...
///
/// #### Variants
/// - `success` No error occurred...
/// - `2big` Argument list too long...
///
#[derive(Debug)]
pub(super) struct MdNamedType {
    pub heading: MdHeading,
    pub id: String,
    pub name: String,
    pub docs: String,
    pub ty: Option<String>,
}

impl MdNamedType {
    pub fn new<S: AsRef<str>>(heading: MdHeading, id: S, name: S, docs: S) -> Self {
        Self {
            heading,
            id: id.as_ref().to_owned(),
            name: name.as_ref().to_owned(),
            docs: docs.as_ref().to_owned(),
            ty: None,
        }
    }
}

impl MdElement for MdNamedType {
    fn id(&self) -> Option<&str> {
        Some(&self.id)
    }

    fn docs(&self) -> Option<&str> {
        Some(&self.docs)
    }

    fn set_docs(&mut self, docs: &str) {
        self.docs = docs.to_owned();
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

impl fmt::Display for MdNamedType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_fmt(format_args!(
            "{heading} {link} `{name}`",
            heading = self.heading,
            link = gen_link(&self.id),
            name = self.name,
        ))?;

        if let Some(tt) = &self.ty {
            f.write_fmt(format_args!(": {}", tt))?;
        }

        writeln!(f, "\n{}", self.docs)
    }
}

/// Record representing a Markdown section representing any `InterfaceFunc` element
/// of the AST.
/// Consists of:
/// * `header`, e.g., "###",
/// * referencable `id`,
/// * some `name`, e.g., `path_open`,
/// * function `inputs`, i.e., arguments,
/// * function `outputs`, i.e., results, and
/// * `docs` paragraph.
///
/// Example rendering:
///
/// ### <a href="#args_get" name="args_get"></a> Fn args_get(argv: `Pointer<Pointer<u8>>`, ...) -> `errno`
/// Read command-line...
///
/// #### Params
/// - `argv`: `Pointer<Pointer<u8>>` Some docs...
/// - ...
///
/// #### Results
/// - `error`: `errno` Error code...
///
#[derive(Debug)]
pub(super) struct MdFunc {
    pub heading: MdHeading,
    pub id: String,
    pub name: String,
    pub inputs: Vec<(String, String)>,
    pub outputs: Vec<String>,
    pub docs: String,
}

impl MdFunc {
    pub fn new<S: AsRef<str>>(heading: MdHeading, id: S, name: S, docs: S) -> Self {
        Self {
            heading,
            id: id.as_ref().to_owned(),
            name: name.as_ref().to_owned(),
            inputs: vec![],
            outputs: vec![],
            docs: docs.as_ref().to_owned(),
        }
    }
}

impl MdElement for MdFunc {
    fn id(&self) -> Option<&str> {
        Some(&self.id)
    }

    fn docs(&self) -> Option<&str> {
        Some(&self.docs)
    }

    fn set_docs(&mut self, docs: &str) {
        self.docs = docs.to_owned();
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

impl fmt::Display for MdFunc {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Expand inputs
        let inputs = self
            .inputs
            .iter()
            .map(|(name, r#type)| format!("{}: {}", name, r#type))
            .collect::<Vec<_>>()
            .join(", ");
        // Expand outputs
        let outputs: Vec<_> = self
            .outputs
            .iter()
            .map(|r#type| format!("{}", r#type))
            .collect();
        let outputs = match outputs.len() {
            0 => "".to_owned(),
            1 => format!(" -> {}", outputs[0]),
            _ => format!(" -> ({})", outputs.join(", ")),
        };
        // Format
        writeln!(f, "\n---\n")?;

        f.write_fmt(format_args!(
            "{heading} {link} `{name}({inputs}){outputs}`",
            heading = self.heading,
            link = gen_link(&self.id),
            name = self.name,
            inputs = inputs,
            outputs = outputs,
        ))?;

        writeln!(f, "\n{}", self.docs)
    }
}
