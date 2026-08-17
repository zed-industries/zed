//! Different variants of an `Item` in our intermediate representation.

use super::context::BindgenContext;
use super::dot::DotAttributes;
use super::function::Function;
use super::module::Module;
use super::ty::Type;
use super::var::Var;
use std::io;

/// A item we parse and translate.
#[derive(Debug)]
pub(crate) enum ItemKind {
    /// A module, created implicitly once (the root module), or via C++
    /// namespaces.
    Module(Module),

    /// A type declared in any of the multiple ways it can be declared.
    Type(Type),

    /// A function or method declaration.
    Function(Function),

    /// A variable declaration, most likely a static.
    Var(Var),
}

impl ItemKind {
    /// Get a reference to this `ItemKind`'s underlying `Module`, or `None` if it
    /// is some other kind.
    pub(crate) fn as_module(&self) -> Option<&Module> {
        match *self {
            ItemKind::Module(ref module) => Some(module),
            _ => None,
        }
    }

    /// Transform our `ItemKind` into a string.
    pub(crate) fn kind_name(&self) -> &'static str {
        match *self {
            ItemKind::Module(..) => "Module",
            ItemKind::Type(..) => "Type",
            ItemKind::Function(..) => "Function",
            ItemKind::Var(..) => "Var",
        }
    }

    /// Is this a module?
    pub(crate) fn is_module(&self) -> bool {
        self.as_module().is_some()
    }

    /// Get a reference to this `ItemKind`'s underlying `Function`, or `None` if
    /// it is some other kind.
    pub(crate) fn as_function(&self) -> Option<&Function> {
        match *self {
            ItemKind::Function(ref func) => Some(func),
            _ => None,
        }
    }

    /// Is this a function?
    pub(crate) fn is_function(&self) -> bool {
        self.as_function().is_some()
    }

    /// Get a reference to this `ItemKind`'s underlying `Function`, or panic if
    /// it is some other kind.
    pub(crate) fn expect_function(&self) -> &Function {
        self.as_function().expect("Not a function")
    }

    /// Get a reference to this `ItemKind`'s underlying `Type`, or `None` if
    /// it is some other kind.
    pub(crate) fn as_type(&self) -> Option<&Type> {
        match *self {
            ItemKind::Type(ref ty) => Some(ty),
            _ => None,
        }
    }

    /// Get a mutable reference to this `ItemKind`'s underlying `Type`, or `None`
    /// if it is some other kind.
    pub(crate) fn as_type_mut(&mut self) -> Option<&mut Type> {
        match *self {
            ItemKind::Type(ref mut ty) => Some(ty),
            _ => None,
        }
    }

    /// Is this a type?
    pub(crate) fn is_type(&self) -> bool {
        self.as_type().is_some()
    }

    /// Get a reference to this `ItemKind`'s underlying `Type`, or panic if it is
    /// some other kind.
    pub(crate) fn expect_type(&self) -> &Type {
        self.as_type().expect("Not a type")
    }

    /// Get a reference to this `ItemKind`'s underlying `Var`, or `None` if it is
    /// some other kind.
    pub(crate) fn as_var(&self) -> Option<&Var> {
        match *self {
            ItemKind::Var(ref v) => Some(v),
            _ => None,
        }
    }

    /// Is this a variable?
    pub(crate) fn is_var(&self) -> bool {
        self.as_var().is_some()
    }
}

impl DotAttributes for ItemKind {
    fn dot_attributes<W>(
        &self,
        ctx: &BindgenContext,
        out: &mut W,
    ) -> io::Result<()>
    where
        W: io::Write,
    {
        writeln!(out, "<tr><td>kind</td><td>{}</td></tr>", self.kind_name())?;

        match *self {
            ItemKind::Module(ref module) => module.dot_attributes(ctx, out),
            ItemKind::Type(ref ty) => ty.dot_attributes(ctx, out),
            ItemKind::Function(ref func) => func.dot_attributes(ctx, out),
            ItemKind::Var(ref var) => var.dot_attributes(ctx, out),
        }
    }
}
