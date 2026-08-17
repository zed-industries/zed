use alloc::{boxed::Box, vec, vec::Vec};

use super::{Error, Result};
use crate::front::wgsl::parse::ast;
use crate::{FastHashMap, Handle, Span};

/// A `GlobalDecl` list in which each definition occurs before all its uses.
pub struct Index<'a> {
    dependency_order: Vec<Handle<ast::GlobalDecl<'a>>>,
}

impl<'a> Index<'a> {
    /// Generate an `Index` for the given translation unit.
    ///
    /// Perform a topological sort on `tu`'s global declarations, placing
    /// referents before the definitions that refer to them.
    ///
    /// Return an error if the graph of references between declarations contains
    /// any cycles.
    pub fn generate(tu: &ast::TranslationUnit<'a>) -> Result<'a, Self> {
        // Produce a map from global definitions' names to their `Handle<GlobalDecl>`s.
        // While doing so, reject conflicting definitions.
        let mut globals = FastHashMap::with_capacity_and_hasher(tu.decls.len(), Default::default());
        for (handle, decl) in tu.decls.iter() {
            if let Some(ident) = decl_ident(decl) {
                let name = ident.name;
                if let Some(old) = globals.insert(name, handle) {
                    return Err(Box::new(Error::Redefinition {
                        previous: decl_ident(&tu.decls[old])
                            .expect("decl should have ident for redefinition")
                            .span,
                        current: ident.span,
                    }));
                }
            }
        }

        let len = tu.decls.len();
        let solver = DependencySolver {
            globals: &globals,
            module: tu,
            visited: vec![false; len],
            temp_visited: vec![false; len],
            path: Vec::new(),
            out: Vec::with_capacity(len),
        };
        let dependency_order = solver.solve()?;

        Ok(Self { dependency_order })
    }

    /// Iterate over `GlobalDecl`s, visiting each definition before all its uses.
    ///
    /// Produce handles for all of the `GlobalDecl`s of the `TranslationUnit`
    /// passed to `Index::generate`, ordered so that a given declaration is
    /// produced before any other declaration that uses it.
    pub fn visit_ordered(&self) -> impl Iterator<Item = Handle<ast::GlobalDecl<'a>>> + '_ {
        self.dependency_order.iter().copied()
    }
}

/// An edge from a reference to its referent in the current depth-first
/// traversal.
///
/// This is like `ast::Dependency`, except that we've determined which
/// `GlobalDecl` it refers to.
struct ResolvedDependency<'a> {
    /// The referent of some identifier used in the current declaration.
    decl: Handle<ast::GlobalDecl<'a>>,

    /// Where that use occurs within the current declaration.
    usage: Span,
}

/// Local state for ordering a `TranslationUnit`'s module-scope declarations.
///
/// Values of this type are used temporarily by `Index::generate`
/// to perform a depth-first sort on the declarations.
/// Technically, what we want is a topological sort, but a depth-first sort
/// has one key benefit - it's much more efficient in storing
/// the path of each node for error generation.
struct DependencySolver<'source, 'temp> {
    /// A map from module-scope definitions' names to their handles.
    globals: &'temp FastHashMap<&'source str, Handle<ast::GlobalDecl<'source>>>,

    /// The translation unit whose declarations we're ordering.
    module: &'temp ast::TranslationUnit<'source>,

    /// For each handle, whether we have pushed it onto `out` yet.
    visited: Vec<bool>,

    /// For each handle, whether it is an predecessor in the current depth-first
    /// traversal. This is used to detect cycles in the reference graph.
    temp_visited: Vec<bool>,

    /// The current path in our depth-first traversal. Used for generating
    /// error messages for non-trivial reference cycles.
    path: Vec<ResolvedDependency<'source>>,

    /// The list of declaration handles, with declarations before uses.
    out: Vec<Handle<ast::GlobalDecl<'source>>>,
}

impl<'a> DependencySolver<'a, '_> {
    /// Produce the sorted list of declaration handles, and check for cycles.
    fn solve(mut self) -> Result<'a, Vec<Handle<ast::GlobalDecl<'a>>>> {
        for (id, _) in self.module.decls.iter() {
            if self.visited[id.index()] {
                continue;
            }

            self.dfs(id)?;
        }

        Ok(self.out)
    }

    /// Ensure that all declarations used by `id` have been added to the
    /// ordering, and then append `id` itself.
    fn dfs(&mut self, id: Handle<ast::GlobalDecl<'a>>) -> Result<'a, ()> {
        let decl = &self.module.decls[id];
        let id_usize = id.index();

        self.temp_visited[id_usize] = true;
        for dep in decl.dependencies.iter() {
            if let Some(&dep_id) = self.globals.get(dep.ident) {
                self.path.push(ResolvedDependency {
                    decl: dep_id,
                    usage: dep.usage,
                });
                let dep_id_usize = dep_id.index();

                if self.temp_visited[dep_id_usize] {
                    // Found a cycle.
                    return if dep_id == id {
                        // A declaration refers to itself directly.
                        Err(Box::new(Error::RecursiveDeclaration {
                            ident: decl_ident(decl).expect("decl should have ident").span,
                            usage: dep.usage,
                        }))
                    } else {
                        // A declaration refers to itself indirectly, through
                        // one or more other definitions. Report the entire path
                        // of references.
                        let start_at = self
                            .path
                            .iter()
                            .rev()
                            .enumerate()
                            .find_map(|(i, dep)| (dep.decl == dep_id).then_some(i))
                            .unwrap_or(0);

                        Err(Box::new(Error::CyclicDeclaration {
                            ident: decl_ident(&self.module.decls[dep_id])
                                .expect("decl should have ident")
                                .span,
                            path: self.path[start_at..]
                                .iter()
                                .map(|curr_dep| {
                                    let curr_id = curr_dep.decl;
                                    let curr_decl = &self.module.decls[curr_id];

                                    (
                                        decl_ident(curr_decl).expect("decl should have ident").span,
                                        curr_dep.usage,
                                    )
                                })
                                .collect(),
                        }))
                    };
                } else if !self.visited[dep_id_usize] {
                    self.dfs(dep_id)?;
                }

                // Remove this edge from the current path.
                self.path.pop();
            }

            // Ignore unresolved identifiers; they may be predeclared objects.
        }

        // Remove this node from the current path.
        self.temp_visited[id_usize] = false;

        // Now everything this declaration uses has been visited, and is already
        // present in `out`. That means we can append this one to the
        // ordering, and mark it as visited.
        self.out.push(id);
        self.visited[id_usize] = true;

        Ok(())
    }
}

const fn decl_ident<'a>(decl: &ast::GlobalDecl<'a>) -> Option<ast::Ident<'a>> {
    match decl.kind {
        ast::GlobalDeclKind::Fn(ref f) => Some(f.name),
        ast::GlobalDeclKind::Var(ref v) => Some(v.name),
        ast::GlobalDeclKind::Const(ref c) => Some(c.name),
        ast::GlobalDeclKind::Override(ref o) => Some(o.name),
        ast::GlobalDeclKind::Struct(ref s) => Some(s.name),
        ast::GlobalDeclKind::Type(ref t) => Some(t.name),
        ast::GlobalDeclKind::ConstAssert(_) => None,
    }
}
