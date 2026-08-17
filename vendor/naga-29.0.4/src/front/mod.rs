/*!
Frontend parsers that consume binary and text shaders and load them into [`Module`](super::Module)s.
*/

mod interpolator;
mod type_gen;

#[cfg(feature = "spv-in")]
pub mod atomic_upgrade;
#[cfg(feature = "glsl-in")]
pub mod glsl;
#[cfg(feature = "spv-in")]
pub mod spv;
#[cfg(feature = "wgsl-in")]
pub mod wgsl;

use alloc::{boxed::Box, vec, vec::Vec};
use core::ops;

use crate::{
    arena::{Arena, Handle, HandleVec, UniqueArena},
    proc::{ResolveContext, ResolveError, TypeResolution},
    FastHashMap,
};

/// A table of types for an `Arena<Expression>`.
///
/// A front end can use a `Typifier` to get types for an arena's expressions
/// while it is still contributing expressions to it. At any point, you can call
/// [`typifier.grow(expr, arena, ctx)`], where `expr` is a `Handle<Expression>`
/// referring to something in `arena`, and the `Typifier` will resolve the types
/// of all the expressions up to and including `expr`. Then you can write
/// `typifier[handle]` to get the type of any handle at or before `expr`.
///
/// Note that `Typifier` does *not* build an `Arena<Type>` as a part of its
/// usual operation. Ideally, a module's type arena should only contain types
/// actually needed by `Handle<Type>`s elsewhere in the module — functions,
/// variables, [`Compose`] expressions, other types, and so on — so we don't
/// want every little thing that occurs as the type of some intermediate
/// expression to show up there.
///
/// Instead, `Typifier` accumulates a [`TypeResolution`] for each expression,
/// which refers to the `Arena<Type>` in the [`ResolveContext`] passed to `grow`
/// as needed. [`TypeResolution`] is a lightweight representation for
/// intermediate types like this; see its documentation for details.
///
/// If you do need to register a `Typifier`'s conclusion in an `Arena<Type>`
/// (say, for a [`LocalVariable`] whose type you've inferred), you can use
/// [`register_type`] to do so.
///
/// [`typifier.grow(expr, arena)`]: Typifier::grow
/// [`register_type`]: Typifier::register_type
/// [`Compose`]: crate::Expression::Compose
/// [`LocalVariable`]: crate::LocalVariable
#[derive(Debug, Default)]
pub struct Typifier {
    resolutions: HandleVec<crate::Expression, TypeResolution>,
}

impl Typifier {
    pub const fn new() -> Self {
        Typifier {
            resolutions: HandleVec::new(),
        }
    }

    pub fn reset(&mut self) {
        self.resolutions.clear()
    }

    pub fn get<'a>(
        &'a self,
        expr_handle: Handle<crate::Expression>,
        types: &'a UniqueArena<crate::Type>,
    ) -> &'a crate::TypeInner {
        self.resolutions[expr_handle].inner_with(types)
    }

    /// Add an expression's type to an `Arena<Type>`.
    ///
    /// Add the type of `expr_handle` to `types`, and return a `Handle<Type>`
    /// referring to it.
    ///
    /// # Note
    ///
    /// If you just need a [`TypeInner`] for `expr_handle`'s type, consider
    /// using `typifier[expression].inner_with(types)` instead. Calling
    /// [`TypeResolution::inner_with`] often lets us avoid adding anything to
    /// the arena, which can significantly reduce the number of types that end
    /// up in the final module.
    ///
    /// [`TypeInner`]: crate::TypeInner
    pub fn register_type(
        &self,
        expr_handle: Handle<crate::Expression>,
        types: &mut UniqueArena<crate::Type>,
    ) -> Handle<crate::Type> {
        match self[expr_handle].clone() {
            TypeResolution::Handle(handle) => handle,
            TypeResolution::Value(inner) => {
                types.insert(crate::Type { name: None, inner }, crate::Span::UNDEFINED)
            }
        }
    }

    /// Grow this typifier until it contains a type for `expr_handle`.
    pub fn grow(
        &mut self,
        expr_handle: Handle<crate::Expression>,
        expressions: &Arena<crate::Expression>,
        ctx: &ResolveContext,
    ) -> Result<(), ResolveError> {
        if self.resolutions.len() <= expr_handle.index() {
            for (eh, expr) in expressions.iter().skip(self.resolutions.len()) {
                //Note: the closure can't `Err` by construction
                let resolution = ctx.resolve(expr, |h| Ok(&self.resolutions[h]))?;
                log::debug!("Resolving {eh:?} = {expr:?} : {resolution:?}");
                self.resolutions.insert(eh, resolution);
            }
        }
        Ok(())
    }

    /// Recompute the type resolution for `expr_handle`.
    ///
    /// If the type of `expr_handle` hasn't yet been calculated, call
    /// [`grow`](Self::grow) to ensure it is covered.
    ///
    /// In either case, when this returns, `self[expr_handle]` should be an
    /// updated type resolution for `expr_handle`.
    pub fn invalidate(
        &mut self,
        expr_handle: Handle<crate::Expression>,
        expressions: &Arena<crate::Expression>,
        ctx: &ResolveContext,
    ) -> Result<(), ResolveError> {
        if self.resolutions.len() <= expr_handle.index() {
            self.grow(expr_handle, expressions, ctx)
        } else {
            let expr = &expressions[expr_handle];
            //Note: the closure can't `Err` by construction
            let resolution = ctx.resolve(expr, |h| Ok(&self.resolutions[h]))?;
            self.resolutions[expr_handle] = resolution;
            Ok(())
        }
    }
}

impl ops::Index<Handle<crate::Expression>> for Typifier {
    type Output = TypeResolution;
    fn index(&self, handle: Handle<crate::Expression>) -> &Self::Output {
        &self.resolutions[handle]
    }
}

/// Type representing a lexical scope, associating a name to a single variable
///
/// The scope is generic over the variable representation and name representation
/// in order to allow larger flexibility on the frontends on how they might
/// represent them.
type Scope<Name, Var> = FastHashMap<Name, Var>;

/// Structure responsible for managing variable lookups and keeping track of
/// lexical scopes
///
/// The symbol table is generic over the variable representation and its name
/// to allow larger flexibility on the frontends on how they might represent them.
///
/// ```
/// use naga::front::SymbolTable;
///
/// // Create a new symbol table with `u32`s representing the variable
/// let mut symbol_table: SymbolTable<&str, u32> = SymbolTable::default();
///
/// // Add two variables named `var1` and `var2` with 0 and 2 respectively
/// symbol_table.add("var1", 0);
/// symbol_table.add("var2", 2);
///
/// // Check that `var1` exists and is `0`
/// assert_eq!(symbol_table.lookup("var1"), Some(&0));
///
/// // Push a new scope and add a variable to it named `var1` shadowing the
/// // variable of our previous scope
/// symbol_table.push_scope();
/// symbol_table.add("var1", 1);
///
/// // Check that `var1` now points to the new value of `1` and `var2` still
/// // exists with its value of `2`
/// assert_eq!(symbol_table.lookup("var1"), Some(&1));
/// assert_eq!(symbol_table.lookup("var2"), Some(&2));
///
/// // Pop the scope
/// symbol_table.pop_scope();
///
/// // Check that `var1` now refers to our initial variable with value `0`
/// assert_eq!(symbol_table.lookup("var1"), Some(&0));
/// ```
///
/// Scopes are ordered as a LIFO stack so a variable defined in a later scope
/// with the same name as another variable defined in a earlier scope will take
/// precedence in the lookup. Scopes can be added with [`push_scope`] and
/// removed with [`pop_scope`].
///
/// A root scope is added when the symbol table is created and must always be
/// present. Trying to pop it will result in a panic.
///
/// Variables can be added with [`add`] and looked up with [`lookup`]. Adding a
/// variable will do so in the currently active scope and as mentioned
/// previously a lookup will search from the current scope to the root scope.
///
/// [`push_scope`]: Self::push_scope
/// [`pop_scope`]: Self::push_scope
/// [`add`]: Self::add
/// [`lookup`]: Self::lookup
pub struct SymbolTable<Name, Var> {
    /// Stack of lexical scopes. Not all scopes are active; see [`cursor`].
    ///
    /// [`cursor`]: Self::cursor
    scopes: Vec<Scope<Name, Var>>,
    /// Limit of the [`scopes`] stack (exclusive). By using a separate value for
    /// the stack length instead of `Vec`'s own internal length, the scopes can
    /// be reused to cache memory allocations.
    ///
    /// [`scopes`]: Self::scopes
    cursor: usize,
    lookup_cursor_is_one_behind: bool,
}

impl<Name, Var> SymbolTable<Name, Var> {
    /// Adds a new lexical scope.
    ///
    /// All variables declared after this point will be added to this scope
    /// until another scope is pushed or [`pop_scope`] is called, causing this
    /// scope to be removed along with all variables added to it.
    ///
    /// # PANICS
    /// - If the current lookup scope doesn't match the current scope
    ///
    /// [`pop_scope`]: Self::pop_scope
    pub fn push_scope(&mut self) {
        self.check_lookup_scope_matches_current_scope();

        // If the cursor is equal to the scope's stack length then we need to
        // push another empty scope. Otherwise we can reuse the already existing
        // scope.
        if self.scopes.len() == self.cursor {
            self.scopes.push(FastHashMap::default())
        } else {
            self.scopes[self.cursor].clear();
        }

        self.cursor += 1;
    }

    /// Removes the current lexical scope and all its variables
    ///
    /// # PANICS
    /// - If the current lexical scope is the root scope
    /// - If the current lookup scope doesn't match the current scope
    pub fn pop_scope(&mut self) {
        // Despite the method title, the variables are only deleted when the
        // scope is reused. This is because while a clear is inevitable if the
        // scope needs to be reused, there are cases where the scope might be
        // popped and not reused, i.e. if another scope with the same nesting
        // level is never pushed again.
        assert!(self.cursor != 1, "Tried to pop the root scope");
        self.check_lookup_scope_matches_current_scope();

        self.cursor -= 1;
    }

    /// Reduces the lookup scope by one level.
    ///
    /// # PANICS
    /// - If the current lookup scope doesn't match the current scope
    pub fn reduce_lookup_scope(&mut self) {
        self.check_lookup_scope_matches_current_scope();
        self.lookup_cursor_is_one_behind = true;
    }

    /// Resets the lookup scope to the current scope.
    ///
    /// # PANICS
    /// - If the current lookup scope already matches the current scope
    pub fn reset_lookup_scope(&mut self) {
        assert!(
            self.lookup_cursor_is_one_behind,
            "current lookup scope already matches the current scope"
        );
        self.lookup_cursor_is_one_behind = false;
    }

    fn check_lookup_scope_matches_current_scope(&self) {
        assert!(
            !self.lookup_cursor_is_one_behind,
            "current lookup scope doesn't match the current scope"
        );
    }
}

impl<Name, Var> SymbolTable<Name, Var>
where
    Name: core::hash::Hash + Eq,
{
    /// Perform a lookup for a variable named `name`.
    ///
    /// As stated in the struct level documentation the lookup will proceed from
    /// the current scope to the root scope, returning `Some` when a variable is
    /// found or `None` if there doesn't exist a variable with `name` in any
    /// scope.
    pub fn lookup<Q>(&self, name: &Q) -> Option<&Var>
    where
        Name: core::borrow::Borrow<Q>,
        Q: core::hash::Hash + Eq + ?Sized,
    {
        let cursor = self
            .cursor
            .saturating_sub(self.lookup_cursor_is_one_behind.into());
        // Iterate backwards through the scopes and try to find the variable
        for scope in self.scopes[..cursor].iter().rev() {
            if let Some(var) = scope.get(name) {
                return Some(var);
            }
        }

        None
    }

    /// Adds a new variable to the current scope.
    ///
    /// Returns the previous variable with the same name in this scope if it
    /// exists, so that the frontend might handle it in case variable shadowing
    /// is disallowed.
    pub fn add(&mut self, name: Name, var: Var) -> Option<Var> {
        self.scopes[self.cursor - 1].insert(name, var)
    }

    /// Adds a new variable to the root scope.
    ///
    /// This is used in GLSL for builtins which aren't known in advance and only
    /// when used for the first time, so there must be a way to add those
    /// declarations to the root unconditionally from the current scope.
    ///
    /// Returns the previous variable with the same name in the root scope if it
    /// exists, so that the frontend might handle it in case variable shadowing
    /// is disallowed.
    pub fn add_root(&mut self, name: Name, var: Var) -> Option<Var> {
        self.scopes[0].insert(name, var)
    }
}

impl<Name, Var> Default for SymbolTable<Name, Var> {
    /// Constructs a new symbol table with a root scope
    fn default() -> Self {
        Self {
            scopes: vec![FastHashMap::default()],
            cursor: 1,
            lookup_cursor_is_one_behind: false,
        }
    }
}

use core::fmt;

impl<Name: fmt::Debug, Var: fmt::Debug> fmt::Debug for SymbolTable<Name, Var> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("SymbolTable ")?;
        f.debug_list()
            .entries(self.scopes[..self.cursor].iter())
            .finish()
    }
}

impl crate::Module {
    pub fn get_or_insert_default_doc_comments(&mut self) -> &mut Box<crate::DocComments> {
        self.doc_comments
            .get_or_insert_with(|| Box::new(crate::DocComments::default()))
    }
}
