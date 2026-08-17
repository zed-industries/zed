use crate::ast::*;
use crate::Error;

mod aliases;
mod deinline_import_export;
mod gensym;
mod names;
mod types;

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
pub enum Ns {
    Func,
    Table,
    Global,
    Memory,
    Module,
    Instance,
    Event,
    Type,
}

impl Ns {
    fn from_export(kind: &ExportKind) -> Ns {
        match kind {
            ExportKind::Func => Ns::Func,
            ExportKind::Table => Ns::Table,
            ExportKind::Global => Ns::Global,
            ExportKind::Memory => Ns::Memory,
            ExportKind::Instance => Ns::Instance,
            ExportKind::Module => Ns::Module,
            ExportKind::Event => Ns::Event,
            ExportKind::Type => Ns::Type,
        }
    }
}

pub fn resolve<'a>(module: &mut Module<'a>) -> Result<Names<'a>, Error> {
    let fields = match &mut module.kind {
        ModuleKind::Text(fields) => fields,
        _ => return Ok(Default::default()),
    };

    // Ensure that each resolution of a module is deterministic in the names
    // that it generates by resetting our thread-local symbol generator.
    gensym::reset();

    // First up, de-inline import/export annotations.
    //
    // This ensures we only have to deal with inline definitions and to
    // calculate exports we only have to look for a particular kind of module
    // field.
    deinline_import_export::run(fields);

    aliases::run(fields);

    // With a canonical form of imports make sure that imports are all listed
    // first.
    for i in 1..fields.len() {
        let span = match &fields[i] {
            ModuleField::Import(i) => i.span,
            _ => continue,
        };
        let name = match &fields[i - 1] {
            ModuleField::Memory(_) => "memory",
            ModuleField::Func(_) => "function",
            ModuleField::Table(_) => "table",
            ModuleField::Global(_) => "global",
            _ => continue,
        };
        return Err(Error::new(span, format!("import after {}", name)));
    }

    // Expand all `TypeUse` annotations so all necessary `type` nodes are
    // present in the AST.
    types::expand(fields);

    // Perform name resolution over all `Index` items to resolve them all to
    // indices instead of symbolic names.
    let resolver = names::resolve(module.id, fields)?;
    Ok(Names { resolver })
}

/// Representation of the results of name resolution for a module.
///
/// This structure is returned from the
/// [`Module::resolve`](crate::Module::resolve) function and can be used to
/// resolve your own name arguments if you have any.
#[derive(Default)]
pub struct Names<'a> {
    resolver: names::Resolver<'a>,
}

impl<'a> Names<'a> {
    /// Resolves `idx` within the function namespace.
    ///
    /// If `idx` is a `Num`, it is ignored, but if it's an `Id` then it will be
    /// looked up in the function namespace and converted to a `Num`. If the
    /// `Id` is not defined then an error will be returned.
    pub fn resolve_func(&self, idx: &mut Index<'a>) -> Result<(), Error> {
        self.resolver.resolve(idx, Ns::Func)?;
        Ok(())
    }

    /// Resolves `idx` within the memory namespace.
    ///
    /// If `idx` is a `Num`, it is ignored, but if it's an `Id` then it will be
    /// looked up in the memory namespace and converted to a `Num`. If the
    /// `Id` is not defined then an error will be returned.
    pub fn resolve_memory(&self, idx: &mut Index<'a>) -> Result<(), Error> {
        self.resolver.resolve(idx, Ns::Memory)?;
        Ok(())
    }

    /// Resolves `idx` within the table namespace.
    ///
    /// If `idx` is a `Num`, it is ignored, but if it's an `Id` then it will be
    /// looked up in the table namespace and converted to a `Num`. If the
    /// `Id` is not defined then an error will be returned.
    pub fn resolve_table(&self, idx: &mut Index<'a>) -> Result<(), Error> {
        self.resolver.resolve(idx, Ns::Table)?;
        Ok(())
    }

    /// Resolves `idx` within the global namespace.
    ///
    /// If `idx` is a `Num`, it is ignored, but if it's an `Id` then it will be
    /// looked up in the global namespace and converted to a `Num`. If the
    /// `Id` is not defined then an error will be returned.
    pub fn resolve_global(&self, idx: &mut Index<'a>) -> Result<(), Error> {
        self.resolver.resolve(idx, Ns::Global)?;
        Ok(())
    }
}
