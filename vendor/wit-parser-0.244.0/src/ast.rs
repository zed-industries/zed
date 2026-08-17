use crate::{Error, PackageNotFoundError, UnresolvedPackageGroup};
use anyhow::{Context, Result, bail};
use lex::{Span, Token, Tokenizer};
use semver::Version;
use std::borrow::Cow;
use std::fmt;
use std::mem;
use std::path::{Path, PathBuf};

pub mod lex;

pub use resolve::Resolver;
mod resolve;
pub mod toposort;

pub use lex::validate_id;

/// Representation of a single WIT `*.wit` file and nested packages.
struct PackageFile<'a> {
    /// Optional `package foo:bar;` header
    package_id: Option<PackageName<'a>>,
    /// Other AST items.
    decl_list: DeclList<'a>,
}

impl<'a> PackageFile<'a> {
    /// Parse a standalone file represented by `tokens`.
    ///
    /// This will optionally start with `package foo:bar;` and then will have a
    /// list of ast items after it.
    fn parse(tokens: &mut Tokenizer<'a>) -> Result<Self> {
        let mut package_name_tokens_peek = tokens.clone();
        let docs = parse_docs(&mut package_name_tokens_peek)?;

        // Parse `package foo:bar;` but throw it out if it's actually
        // `package foo:bar { ... }` since that's an ast item instead.
        let package_id = if package_name_tokens_peek.eat(Token::Package)? {
            let name = PackageName::parse(&mut package_name_tokens_peek, docs)?;
            if package_name_tokens_peek.eat(Token::Semicolon)? {
                *tokens = package_name_tokens_peek;
                Some(name)
            } else {
                None
            }
        } else {
            None
        };
        let decl_list = DeclList::parse_until(tokens, None)?;
        Ok(PackageFile {
            package_id,
            decl_list,
        })
    }

    /// Parse a nested package of the form `package foo:bar { ... }`
    fn parse_nested(
        tokens: &mut Tokenizer<'a>,
        docs: Docs<'a>,
        attributes: Vec<Attribute<'a>>,
    ) -> Result<Self> {
        let span = tokens.expect(Token::Package)?;
        if !attributes.is_empty() {
            bail!(Error::new(
                span,
                format!("cannot place attributes on nested packages"),
            ));
        }
        let package_id = PackageName::parse(tokens, docs)?;
        tokens.expect(Token::LeftBrace)?;
        let decl_list = DeclList::parse_until(tokens, Some(Token::RightBrace))?;
        Ok(PackageFile {
            package_id: Some(package_id),
            decl_list,
        })
    }
}

/// Stores all of the declarations in a package's scope. In AST terms, this
/// means everything except the `package` declaration that demarcates a package
/// scope. In the traditional implicit format, these are all of the declarations
/// non-`package` declarations in the file:
///
/// ```wit
/// package foo:name;
///
/// /* START DECL LIST */
/// // Some comment...
/// interface i {}
/// world w {}
/// /* END DECL LIST */
/// ```
///
/// In the nested package style, a [`DeclList`] is everything inside of each
/// `package` element's brackets:
///
/// ```wit
/// package foo:name {
///   /* START FIRST DECL LIST */
///   // Some comment...
///   interface i {}
///   world w {}
///   /* END FIRST DECL LIST */
/// }
///
/// package bar:name {
///   /* START SECOND DECL LIST */
///   // Some comment...
///   interface i {}
///   world w {}
///   /* END SECOND DECL LIST */
/// }
/// ```
#[derive(Default)]
pub struct DeclList<'a> {
    items: Vec<AstItem<'a>>,
}

impl<'a> DeclList<'a> {
    fn parse_until(tokens: &mut Tokenizer<'a>, end: Option<Token>) -> Result<DeclList<'a>> {
        let mut items = Vec::new();
        let mut docs = parse_docs(tokens)?;
        loop {
            match end {
                Some(end) => {
                    if tokens.eat(end)? {
                        break;
                    }
                }
                None => {
                    if tokens.clone().next()?.is_none() {
                        break;
                    }
                }
            }
            items.push(AstItem::parse(tokens, docs)?);
            docs = parse_docs(tokens)?;
        }
        Ok(DeclList { items })
    }

    fn for_each_path<'b>(
        &'b self,
        f: &mut dyn FnMut(
            Option<&'b Id<'a>>,
            &'b [Attribute<'a>],
            &'b UsePath<'a>,
            Option<&'b [UseName<'a>]>,
            WorldOrInterface,
        ) -> Result<()>,
    ) -> Result<()> {
        for item in self.items.iter() {
            match item {
                AstItem::World(world) => {
                    // Visit imports here first before exports to help preserve
                    // round-tripping of documents because printing a world puts
                    // imports first but textually they can be listed with
                    // exports first.
                    let mut imports = Vec::new();
                    let mut exports = Vec::new();
                    for item in world.items.iter() {
                        match item {
                            WorldItem::Use(u) => f(
                                None,
                                &u.attributes,
                                &u.from,
                                Some(&u.names),
                                WorldOrInterface::Interface,
                            )?,
                            WorldItem::Include(i) => f(
                                Some(&world.name),
                                &i.attributes,
                                &i.from,
                                None,
                                WorldOrInterface::World,
                            )?,
                            WorldItem::Type(_) => {}
                            WorldItem::Import(Import {
                                kind, attributes, ..
                            }) => imports.push((kind, attributes)),
                            WorldItem::Export(Export {
                                kind, attributes, ..
                            }) => exports.push((kind, attributes)),
                        }
                    }

                    let mut visit_kind =
                        |kind: &'b ExternKind<'a>, attrs: &'b [Attribute<'a>]| match kind {
                            ExternKind::Interface(_, items) => {
                                for item in items {
                                    match item {
                                        InterfaceItem::Use(u) => f(
                                            None,
                                            &u.attributes,
                                            &u.from,
                                            Some(&u.names),
                                            WorldOrInterface::Interface,
                                        )?,
                                        _ => {}
                                    }
                                }
                                Ok(())
                            }
                            ExternKind::Path(path) => {
                                f(None, attrs, path, None, WorldOrInterface::Interface)
                            }
                            ExternKind::Func(..) => Ok(()),
                        };

                    for (kind, attrs) in imports {
                        visit_kind(kind, attrs)?;
                    }
                    for (kind, attrs) in exports {
                        visit_kind(kind, attrs)?;
                    }
                }
                AstItem::Interface(i) => {
                    for item in i.items.iter() {
                        match item {
                            InterfaceItem::Use(u) => f(
                                Some(&i.name),
                                &u.attributes,
                                &u.from,
                                Some(&u.names),
                                WorldOrInterface::Interface,
                            )?,
                            _ => {}
                        }
                    }
                }
                AstItem::Use(u) => {
                    // At the top-level, we don't know if this is a world or an interface
                    // It is up to the resolver to decides how to handle this ambiguity.
                    f(
                        None,
                        &u.attributes,
                        &u.item,
                        None,
                        WorldOrInterface::Unknown,
                    )?;
                }

                AstItem::Package(pkg) => pkg.decl_list.for_each_path(f)?,
            }
        }
        Ok(())
    }
}

enum AstItem<'a> {
    Interface(Interface<'a>),
    World(World<'a>),
    Use(ToplevelUse<'a>),
    Package(PackageFile<'a>),
}

impl<'a> AstItem<'a> {
    fn parse(tokens: &mut Tokenizer<'a>, docs: Docs<'a>) -> Result<Self> {
        let attributes = Attribute::parse_list(tokens)?;
        match tokens.clone().next()? {
            Some((_span, Token::Interface)) => {
                Interface::parse(tokens, docs, attributes).map(Self::Interface)
            }
            Some((_span, Token::World)) => World::parse(tokens, docs, attributes).map(Self::World),
            Some((_span, Token::Use)) => ToplevelUse::parse(tokens, attributes).map(Self::Use),
            Some((_span, Token::Package)) => {
                PackageFile::parse_nested(tokens, docs, attributes).map(Self::Package)
            }
            other => Err(err_expected(tokens, "`world`, `interface` or `use`", other).into()),
        }
    }
}

#[derive(Debug, Clone)]
struct PackageName<'a> {
    docs: Docs<'a>,
    span: Span,
    namespace: Id<'a>,
    name: Id<'a>,
    version: Option<(Span, Version)>,
}

impl<'a> PackageName<'a> {
    fn parse(tokens: &mut Tokenizer<'a>, docs: Docs<'a>) -> Result<Self> {
        let namespace = parse_id(tokens)?;
        tokens.expect(Token::Colon)?;
        let name = parse_id(tokens)?;
        let version = parse_opt_version(tokens)?;
        Ok(PackageName {
            docs,
            span: Span {
                start: namespace.span.start,
                end: version
                    .as_ref()
                    .map(|(s, _)| s.end)
                    .unwrap_or(name.span.end),
            },
            namespace,
            name,
            version,
        })
    }

    fn package_name(&self) -> crate::PackageName {
        crate::PackageName {
            namespace: self.namespace.name.to_string(),
            name: self.name.name.to_string(),
            version: self.version.as_ref().map(|(_, v)| v.clone()),
        }
    }
}

struct ToplevelUse<'a> {
    span: Span,
    attributes: Vec<Attribute<'a>>,
    item: UsePath<'a>,
    as_: Option<Id<'a>>,
}

impl<'a> ToplevelUse<'a> {
    fn parse(tokens: &mut Tokenizer<'a>, attributes: Vec<Attribute<'a>>) -> Result<Self> {
        let span = tokens.expect(Token::Use)?;
        let item = UsePath::parse(tokens)?;
        let as_ = if tokens.eat(Token::As)? {
            Some(parse_id(tokens)?)
        } else {
            None
        };
        tokens.expect_semicolon()?;
        Ok(ToplevelUse {
            span,
            attributes,
            item,
            as_,
        })
    }
}

struct World<'a> {
    docs: Docs<'a>,
    attributes: Vec<Attribute<'a>>,
    name: Id<'a>,
    items: Vec<WorldItem<'a>>,
}

impl<'a> World<'a> {
    fn parse(
        tokens: &mut Tokenizer<'a>,
        docs: Docs<'a>,
        attributes: Vec<Attribute<'a>>,
    ) -> Result<Self> {
        tokens.expect(Token::World)?;
        let name = parse_id(tokens)?;
        let items = Self::parse_items(tokens)?;
        Ok(World {
            docs,
            attributes,
            name,
            items,
        })
    }

    fn parse_items(tokens: &mut Tokenizer<'a>) -> Result<Vec<WorldItem<'a>>> {
        tokens.expect(Token::LeftBrace)?;
        let mut items = Vec::new();
        loop {
            let docs = parse_docs(tokens)?;
            if tokens.eat(Token::RightBrace)? {
                break;
            }
            let attributes = Attribute::parse_list(tokens)?;
            items.push(WorldItem::parse(tokens, docs, attributes)?);
        }
        Ok(items)
    }
}

enum WorldItem<'a> {
    Import(Import<'a>),
    Export(Export<'a>),
    Use(Use<'a>),
    Type(TypeDef<'a>),
    Include(Include<'a>),
}

impl<'a> WorldItem<'a> {
    fn parse(
        tokens: &mut Tokenizer<'a>,
        docs: Docs<'a>,
        attributes: Vec<Attribute<'a>>,
    ) -> Result<WorldItem<'a>> {
        match tokens.clone().next()? {
            Some((_span, Token::Import)) => {
                Import::parse(tokens, docs, attributes).map(WorldItem::Import)
            }
            Some((_span, Token::Export)) => {
                Export::parse(tokens, docs, attributes).map(WorldItem::Export)
            }
            Some((_span, Token::Use)) => Use::parse(tokens, attributes).map(WorldItem::Use),
            Some((_span, Token::Type)) => {
                TypeDef::parse(tokens, docs, attributes).map(WorldItem::Type)
            }
            Some((_span, Token::Flags)) => {
                TypeDef::parse_flags(tokens, docs, attributes).map(WorldItem::Type)
            }
            Some((_span, Token::Resource)) => {
                TypeDef::parse_resource(tokens, docs, attributes).map(WorldItem::Type)
            }
            Some((_span, Token::Record)) => {
                TypeDef::parse_record(tokens, docs, attributes).map(WorldItem::Type)
            }
            Some((_span, Token::Variant)) => {
                TypeDef::parse_variant(tokens, docs, attributes).map(WorldItem::Type)
            }
            Some((_span, Token::Enum)) => {
                TypeDef::parse_enum(tokens, docs, attributes).map(WorldItem::Type)
            }
            Some((_span, Token::Include)) => {
                Include::parse(tokens, attributes).map(WorldItem::Include)
            }
            other => Err(err_expected(
                tokens,
                "`import`, `export`, `include`, `use`, or type definition",
                other,
            )
            .into()),
        }
    }
}

struct Import<'a> {
    docs: Docs<'a>,
    attributes: Vec<Attribute<'a>>,
    kind: ExternKind<'a>,
}

impl<'a> Import<'a> {
    fn parse(
        tokens: &mut Tokenizer<'a>,
        docs: Docs<'a>,
        attributes: Vec<Attribute<'a>>,
    ) -> Result<Import<'a>> {
        tokens.expect(Token::Import)?;
        let kind = ExternKind::parse(tokens)?;
        Ok(Import {
            docs,
            attributes,
            kind,
        })
    }
}

struct Export<'a> {
    docs: Docs<'a>,
    attributes: Vec<Attribute<'a>>,
    kind: ExternKind<'a>,
}

impl<'a> Export<'a> {
    fn parse(
        tokens: &mut Tokenizer<'a>,
        docs: Docs<'a>,
        attributes: Vec<Attribute<'a>>,
    ) -> Result<Export<'a>> {
        tokens.expect(Token::Export)?;
        let kind = ExternKind::parse(tokens)?;
        Ok(Export {
            docs,
            attributes,
            kind,
        })
    }
}

enum ExternKind<'a> {
    Interface(Id<'a>, Vec<InterfaceItem<'a>>),
    Path(UsePath<'a>),
    Func(Id<'a>, Func<'a>),
}

impl<'a> ExternKind<'a> {
    fn parse(tokens: &mut Tokenizer<'a>) -> Result<ExternKind<'a>> {
        // Create a copy of the token stream to test out if this is a function
        // or an interface import. In those situations the token stream gets
        // reset to the state of the clone and we continue down those paths.
        //
        // If neither a function nor an interface appears here though then the
        // clone is thrown away and the original token stream is parsed for an
        // interface. This will redo the original ID parse and the original
        // colon parse, but that shouldn't be too bad perf-wise.
        let mut clone = tokens.clone();
        let id = parse_id(&mut clone)?;
        if clone.eat(Token::Colon)? {
            // import foo: async? func(...)
            if clone.clone().eat(Token::Func)? || clone.clone().eat(Token::Async)? {
                *tokens = clone;
                let ret = ExternKind::Func(id, Func::parse(tokens)?);
                tokens.expect_semicolon()?;
                return Ok(ret);
            }

            // import foo: interface { ... }
            if clone.eat(Token::Interface)? {
                *tokens = clone;
                return Ok(ExternKind::Interface(id, Interface::parse_items(tokens)?));
            }
        }

        // import foo
        // import foo/bar
        // import foo:bar/baz
        let ret = ExternKind::Path(UsePath::parse(tokens)?);
        tokens.expect_semicolon()?;
        Ok(ret)
    }

    fn span(&self) -> Span {
        match self {
            ExternKind::Interface(id, _) => id.span,
            ExternKind::Path(UsePath::Id(id)) => id.span,
            ExternKind::Path(UsePath::Package { name, .. }) => name.span,
            ExternKind::Func(id, _) => id.span,
        }
    }
}

struct Interface<'a> {
    docs: Docs<'a>,
    attributes: Vec<Attribute<'a>>,
    name: Id<'a>,
    items: Vec<InterfaceItem<'a>>,
}

impl<'a> Interface<'a> {
    fn parse(
        tokens: &mut Tokenizer<'a>,
        docs: Docs<'a>,
        attributes: Vec<Attribute<'a>>,
    ) -> Result<Self> {
        tokens.expect(Token::Interface)?;
        let name = parse_id(tokens)?;
        let items = Self::parse_items(tokens)?;
        Ok(Interface {
            docs,
            attributes,
            name,
            items,
        })
    }

    pub(super) fn parse_items(tokens: &mut Tokenizer<'a>) -> Result<Vec<InterfaceItem<'a>>> {
        tokens.expect(Token::LeftBrace)?;
        let mut items = Vec::new();
        loop {
            let docs = parse_docs(tokens)?;
            if tokens.eat(Token::RightBrace)? {
                break;
            }
            let attributes = Attribute::parse_list(tokens)?;
            items.push(InterfaceItem::parse(tokens, docs, attributes)?);
        }
        Ok(items)
    }
}

#[derive(Debug)]
pub enum WorldOrInterface {
    World,
    Interface,
    Unknown,
}

enum InterfaceItem<'a> {
    TypeDef(TypeDef<'a>),
    Func(NamedFunc<'a>),
    Use(Use<'a>),
}

struct Use<'a> {
    attributes: Vec<Attribute<'a>>,
    from: UsePath<'a>,
    names: Vec<UseName<'a>>,
}

#[derive(Debug)]
enum UsePath<'a> {
    Id(Id<'a>),
    Package { id: PackageName<'a>, name: Id<'a> },
}

impl<'a> UsePath<'a> {
    fn parse(tokens: &mut Tokenizer<'a>) -> Result<Self> {
        let id = parse_id(tokens)?;
        if tokens.eat(Token::Colon)? {
            // `foo:bar/baz@1.0`
            let namespace = id;
            let pkg_name = parse_id(tokens)?;
            tokens.expect(Token::Slash)?;
            let name = parse_id(tokens)?;
            let version = parse_opt_version(tokens)?;
            Ok(UsePath::Package {
                id: PackageName {
                    docs: Default::default(),
                    span: Span {
                        start: namespace.span.start,
                        end: pkg_name.span.end,
                    },
                    namespace,
                    name: pkg_name,
                    version,
                },
                name,
            })
        } else {
            // `foo`
            Ok(UsePath::Id(id))
        }
    }

    fn name(&self) -> &Id<'a> {
        match self {
            UsePath::Id(id) => id,
            UsePath::Package { name, .. } => name,
        }
    }
}

struct UseName<'a> {
    name: Id<'a>,
    as_: Option<Id<'a>>,
}

impl<'a> Use<'a> {
    fn parse(tokens: &mut Tokenizer<'a>, attributes: Vec<Attribute<'a>>) -> Result<Self> {
        tokens.expect(Token::Use)?;
        let from = UsePath::parse(tokens)?;
        tokens.expect(Token::Period)?;
        tokens.expect(Token::LeftBrace)?;

        let mut names = Vec::new();
        while !tokens.eat(Token::RightBrace)? {
            let mut name = UseName {
                name: parse_id(tokens)?,
                as_: None,
            };
            if tokens.eat(Token::As)? {
                name.as_ = Some(parse_id(tokens)?);
            }
            names.push(name);
            if !tokens.eat(Token::Comma)? {
                tokens.expect(Token::RightBrace)?;
                break;
            }
        }
        tokens.expect_semicolon()?;
        Ok(Use {
            attributes,
            from,
            names,
        })
    }
}

struct Include<'a> {
    from: UsePath<'a>,
    attributes: Vec<Attribute<'a>>,
    names: Vec<IncludeName<'a>>,
}

struct IncludeName<'a> {
    name: Id<'a>,
    as_: Id<'a>,
}

impl<'a> Include<'a> {
    fn parse(tokens: &mut Tokenizer<'a>, attributes: Vec<Attribute<'a>>) -> Result<Self> {
        tokens.expect(Token::Include)?;
        let from = UsePath::parse(tokens)?;

        let names = if tokens.eat(Token::With)? {
            parse_list(
                tokens,
                Token::LeftBrace,
                Token::RightBrace,
                |_docs, tokens| {
                    let name = parse_id(tokens)?;
                    tokens.expect(Token::As)?;
                    let as_ = parse_id(tokens)?;
                    Ok(IncludeName { name, as_ })
                },
            )?
        } else {
            tokens.expect_semicolon()?;
            Vec::new()
        };

        Ok(Include {
            attributes,
            from,
            names,
        })
    }
}

#[derive(Debug, Clone)]
pub struct Id<'a> {
    name: &'a str,
    span: Span,
}

impl<'a> From<&'a str> for Id<'a> {
    fn from(s: &'a str) -> Id<'a> {
        Id {
            name: s.into(),
            span: Span { start: 0, end: 0 },
        }
    }
}

#[derive(Debug, Clone)]
pub struct Docs<'a> {
    docs: Vec<Cow<'a, str>>,
    span: Span,
}

impl<'a> Default for Docs<'a> {
    fn default() -> Self {
        Self {
            docs: Default::default(),
            span: Span { start: 0, end: 0 },
        }
    }
}

struct TypeDef<'a> {
    docs: Docs<'a>,
    attributes: Vec<Attribute<'a>>,
    name: Id<'a>,
    ty: Type<'a>,
}

enum Type<'a> {
    Bool(Span),
    U8(Span),
    U16(Span),
    U32(Span),
    U64(Span),
    S8(Span),
    S16(Span),
    S32(Span),
    S64(Span),
    F32(Span),
    F64(Span),
    Char(Span),
    String(Span),
    Name(Id<'a>),
    List(List<'a>),
    Map(Map<'a>),
    FixedSizeList(FixedSizeList<'a>),
    Handle(Handle<'a>),
    Resource(Resource<'a>),
    Record(Record<'a>),
    Flags(Flags<'a>),
    Variant(Variant<'a>),
    Tuple(Tuple<'a>),
    Enum(Enum<'a>),
    Option(Option_<'a>),
    Result(Result_<'a>),
    Future(Future<'a>),
    Stream(Stream<'a>),
    ErrorContext(Span),
}

enum Handle<'a> {
    Own { resource: Id<'a> },
    Borrow { resource: Id<'a> },
}

impl Handle<'_> {
    fn span(&self) -> Span {
        match self {
            Handle::Own { resource } | Handle::Borrow { resource } => resource.span,
        }
    }
}

struct Resource<'a> {
    span: Span,
    funcs: Vec<ResourceFunc<'a>>,
}

enum ResourceFunc<'a> {
    Method(NamedFunc<'a>),
    Static(NamedFunc<'a>),
    Constructor(NamedFunc<'a>),
}

impl<'a> ResourceFunc<'a> {
    fn parse(
        docs: Docs<'a>,
        attributes: Vec<Attribute<'a>>,
        tokens: &mut Tokenizer<'a>,
    ) -> Result<Self> {
        match tokens.clone().next()? {
            Some((span, Token::Constructor)) => {
                tokens.expect(Token::Constructor)?;
                tokens.expect(Token::LeftParen)?;
                let params = parse_list_trailer(tokens, Token::RightParen, |_docs, tokens| {
                    let name = parse_id(tokens)?;
                    tokens.expect(Token::Colon)?;
                    let ty = Type::parse(tokens)?;
                    Ok((name, ty))
                })?;
                let result = if tokens.eat(Token::RArrow)? {
                    let ty = Type::parse(tokens)?;
                    Some(ty)
                } else {
                    None
                };
                tokens.expect_semicolon()?;
                Ok(ResourceFunc::Constructor(NamedFunc {
                    docs,
                    attributes,
                    name: Id {
                        span,
                        name: "constructor",
                    },
                    func: Func {
                        span,
                        async_: false,
                        params,
                        result,
                    },
                }))
            }
            Some((_span, Token::Id | Token::ExplicitId)) => {
                let name = parse_id(tokens)?;
                tokens.expect(Token::Colon)?;
                let ctor = if tokens.eat(Token::Static)? {
                    ResourceFunc::Static
                } else {
                    ResourceFunc::Method
                };
                let func = Func::parse(tokens)?;
                tokens.expect_semicolon()?;
                Ok(ctor(NamedFunc {
                    docs,
                    attributes,
                    name,
                    func,
                }))
            }
            other => Err(err_expected(tokens, "`constructor` or identifier", other).into()),
        }
    }

    fn named_func(&self) -> &NamedFunc<'a> {
        use ResourceFunc::*;
        match self {
            Method(f) | Static(f) | Constructor(f) => f,
        }
    }
}

struct Record<'a> {
    span: Span,
    fields: Vec<Field<'a>>,
}

struct Field<'a> {
    docs: Docs<'a>,
    name: Id<'a>,
    ty: Type<'a>,
}

struct Flags<'a> {
    span: Span,
    flags: Vec<Flag<'a>>,
}

struct Flag<'a> {
    docs: Docs<'a>,
    name: Id<'a>,
}

struct Variant<'a> {
    span: Span,
    cases: Vec<Case<'a>>,
}

struct Case<'a> {
    docs: Docs<'a>,
    name: Id<'a>,
    ty: Option<Type<'a>>,
}

struct Enum<'a> {
    span: Span,
    cases: Vec<EnumCase<'a>>,
}

struct EnumCase<'a> {
    docs: Docs<'a>,
    name: Id<'a>,
}

struct Option_<'a> {
    span: Span,
    ty: Box<Type<'a>>,
}

struct List<'a> {
    span: Span,
    ty: Box<Type<'a>>,
}

struct Map<'a> {
    span: Span,
    key: Box<Type<'a>>,
    value: Box<Type<'a>>,
}

struct FixedSizeList<'a> {
    span: Span,
    ty: Box<Type<'a>>,
    size: u32,
}

struct Future<'a> {
    span: Span,
    ty: Option<Box<Type<'a>>>,
}

struct Tuple<'a> {
    span: Span,
    types: Vec<Type<'a>>,
}

struct Result_<'a> {
    span: Span,
    ok: Option<Box<Type<'a>>>,
    err: Option<Box<Type<'a>>>,
}

struct Stream<'a> {
    span: Span,
    ty: Option<Box<Type<'a>>>,
}

struct NamedFunc<'a> {
    docs: Docs<'a>,
    attributes: Vec<Attribute<'a>>,
    name: Id<'a>,
    func: Func<'a>,
}

type ParamList<'a> = Vec<(Id<'a>, Type<'a>)>;

struct Func<'a> {
    span: Span,
    async_: bool,
    params: ParamList<'a>,
    result: Option<Type<'a>>,
}

impl<'a> Func<'a> {
    fn parse(tokens: &mut Tokenizer<'a>) -> Result<Func<'a>> {
        fn parse_params<'a>(tokens: &mut Tokenizer<'a>, left_paren: bool) -> Result<ParamList<'a>> {
            if left_paren {
                tokens.expect(Token::LeftParen)?;
            };
            parse_list_trailer(tokens, Token::RightParen, |_docs, tokens| {
                let name = parse_id(tokens)?;
                tokens.expect(Token::Colon)?;
                let ty = Type::parse(tokens)?;
                Ok((name, ty))
            })
        }

        let async_ = tokens.eat(Token::Async)?;
        let span = tokens.expect(Token::Func)?;
        let params = parse_params(tokens, true)?;
        let result = if tokens.eat(Token::RArrow)? {
            let ty = Type::parse(tokens)?;
            Some(ty)
        } else {
            None
        };
        Ok(Func {
            span,
            async_,
            params,
            result,
        })
    }
}

impl<'a> InterfaceItem<'a> {
    fn parse(
        tokens: &mut Tokenizer<'a>,
        docs: Docs<'a>,
        attributes: Vec<Attribute<'a>>,
    ) -> Result<InterfaceItem<'a>> {
        match tokens.clone().next()? {
            Some((_span, Token::Type)) => {
                TypeDef::parse(tokens, docs, attributes).map(InterfaceItem::TypeDef)
            }
            Some((_span, Token::Flags)) => {
                TypeDef::parse_flags(tokens, docs, attributes).map(InterfaceItem::TypeDef)
            }
            Some((_span, Token::Enum)) => {
                TypeDef::parse_enum(tokens, docs, attributes).map(InterfaceItem::TypeDef)
            }
            Some((_span, Token::Variant)) => {
                TypeDef::parse_variant(tokens, docs, attributes).map(InterfaceItem::TypeDef)
            }
            Some((_span, Token::Resource)) => {
                TypeDef::parse_resource(tokens, docs, attributes).map(InterfaceItem::TypeDef)
            }
            Some((_span, Token::Record)) => {
                TypeDef::parse_record(tokens, docs, attributes).map(InterfaceItem::TypeDef)
            }
            Some((_span, Token::Id)) | Some((_span, Token::ExplicitId)) => {
                NamedFunc::parse(tokens, docs, attributes).map(InterfaceItem::Func)
            }
            Some((_span, Token::Use)) => Use::parse(tokens, attributes).map(InterfaceItem::Use),
            other => Err(err_expected(tokens, "`type`, `resource` or `func`", other).into()),
        }
    }
}

impl<'a> TypeDef<'a> {
    fn parse(
        tokens: &mut Tokenizer<'a>,
        docs: Docs<'a>,
        attributes: Vec<Attribute<'a>>,
    ) -> Result<Self> {
        tokens.expect(Token::Type)?;
        let name = parse_id(tokens)?;
        tokens.expect(Token::Equals)?;
        let ty = Type::parse(tokens)?;
        tokens.expect_semicolon()?;
        Ok(TypeDef {
            docs,
            attributes,
            name,
            ty,
        })
    }

    fn parse_flags(
        tokens: &mut Tokenizer<'a>,
        docs: Docs<'a>,
        attributes: Vec<Attribute<'a>>,
    ) -> Result<Self> {
        tokens.expect(Token::Flags)?;
        let name = parse_id(tokens)?;
        let ty = Type::Flags(Flags {
            span: name.span,
            flags: parse_list(
                tokens,
                Token::LeftBrace,
                Token::RightBrace,
                |docs, tokens| {
                    let name = parse_id(tokens)?;
                    Ok(Flag { docs, name })
                },
            )?,
        });
        Ok(TypeDef {
            docs,
            attributes,
            name,
            ty,
        })
    }

    fn parse_resource(
        tokens: &mut Tokenizer<'a>,
        docs: Docs<'a>,
        attributes: Vec<Attribute<'a>>,
    ) -> Result<Self> {
        tokens.expect(Token::Resource)?;
        let name = parse_id(tokens)?;
        let mut funcs = Vec::new();
        if tokens.eat(Token::LeftBrace)? {
            while !tokens.eat(Token::RightBrace)? {
                let docs = parse_docs(tokens)?;
                let attributes = Attribute::parse_list(tokens)?;
                funcs.push(ResourceFunc::parse(docs, attributes, tokens)?);
            }
        } else {
            tokens.expect_semicolon()?;
        }
        let ty = Type::Resource(Resource {
            span: name.span,
            funcs,
        });
        Ok(TypeDef {
            docs,
            attributes,
            name,
            ty,
        })
    }

    fn parse_record(
        tokens: &mut Tokenizer<'a>,
        docs: Docs<'a>,
        attributes: Vec<Attribute<'a>>,
    ) -> Result<Self> {
        tokens.expect(Token::Record)?;
        let name = parse_id(tokens)?;
        let ty = Type::Record(Record {
            span: name.span,
            fields: parse_list(
                tokens,
                Token::LeftBrace,
                Token::RightBrace,
                |docs, tokens| {
                    let name = parse_id(tokens)?;
                    tokens.expect(Token::Colon)?;
                    let ty = Type::parse(tokens)?;
                    Ok(Field { docs, name, ty })
                },
            )?,
        });
        Ok(TypeDef {
            docs,
            attributes,
            name,
            ty,
        })
    }

    fn parse_variant(
        tokens: &mut Tokenizer<'a>,
        docs: Docs<'a>,
        attributes: Vec<Attribute<'a>>,
    ) -> Result<Self> {
        tokens.expect(Token::Variant)?;
        let name = parse_id(tokens)?;
        let ty = Type::Variant(Variant {
            span: name.span,
            cases: parse_list(
                tokens,
                Token::LeftBrace,
                Token::RightBrace,
                |docs, tokens| {
                    let name = parse_id(tokens)?;
                    let ty = if tokens.eat(Token::LeftParen)? {
                        let ty = Type::parse(tokens)?;
                        tokens.expect(Token::RightParen)?;
                        Some(ty)
                    } else {
                        None
                    };
                    Ok(Case { docs, name, ty })
                },
            )?,
        });
        Ok(TypeDef {
            docs,
            attributes,
            name,
            ty,
        })
    }

    fn parse_enum(
        tokens: &mut Tokenizer<'a>,
        docs: Docs<'a>,
        attributes: Vec<Attribute<'a>>,
    ) -> Result<Self> {
        tokens.expect(Token::Enum)?;
        let name = parse_id(tokens)?;
        let ty = Type::Enum(Enum {
            span: name.span,
            cases: parse_list(
                tokens,
                Token::LeftBrace,
                Token::RightBrace,
                |docs, tokens| {
                    let name = parse_id(tokens)?;
                    Ok(EnumCase { docs, name })
                },
            )?,
        });
        Ok(TypeDef {
            docs,
            attributes,
            name,
            ty,
        })
    }
}

impl<'a> NamedFunc<'a> {
    fn parse(
        tokens: &mut Tokenizer<'a>,
        docs: Docs<'a>,
        attributes: Vec<Attribute<'a>>,
    ) -> Result<Self> {
        let name = parse_id(tokens)?;
        tokens.expect(Token::Colon)?;
        let func = Func::parse(tokens)?;
        tokens.expect_semicolon()?;
        Ok(NamedFunc {
            docs,
            attributes,
            name,
            func,
        })
    }
}

fn parse_id<'a>(tokens: &mut Tokenizer<'a>) -> Result<Id<'a>> {
    match tokens.next()? {
        Some((span, Token::Id)) => Ok(Id {
            name: tokens.parse_id(span)?,
            span,
        }),
        Some((span, Token::ExplicitId)) => Ok(Id {
            name: tokens.parse_explicit_id(span)?,
            span,
        }),
        other => Err(err_expected(tokens, "an identifier or string", other).into()),
    }
}

fn parse_opt_version(tokens: &mut Tokenizer<'_>) -> Result<Option<(Span, Version)>> {
    if tokens.eat(Token::At)? {
        parse_version(tokens).map(Some)
    } else {
        Ok(None)
    }
}

fn parse_version(tokens: &mut Tokenizer<'_>) -> Result<(Span, Version)> {
    let start = tokens.expect(Token::Integer)?.start;
    tokens.expect(Token::Period)?;
    tokens.expect(Token::Integer)?;
    tokens.expect(Token::Period)?;
    let end = tokens.expect(Token::Integer)?.end;
    let mut span = Span { start, end };
    eat_ids(tokens, Token::Minus, &mut span)?;
    eat_ids(tokens, Token::Plus, &mut span)?;
    let string = tokens.get_span(span);
    let version = Version::parse(string).map_err(|e| Error::new(span, e.to_string()))?;
    return Ok((span, version));

    // According to `semver.org` this is what we're parsing:
    //
    // ```ebnf
    // <pre-release> ::= <dot-separated pre-release identifiers>
    //
    // <dot-separated pre-release identifiers> ::= <pre-release identifier>
    //                                           | <pre-release identifier> "." <dot-separated pre-release identifiers>
    //
    // <build> ::= <dot-separated build identifiers>
    //
    // <dot-separated build identifiers> ::= <build identifier>
    //                                     | <build identifier> "." <dot-separated build identifiers>
    //
    // <pre-release identifier> ::= <alphanumeric identifier>
    //                            | <numeric identifier>
    //
    // <build identifier> ::= <alphanumeric identifier>
    //                      | <digits>
    //
    // <alphanumeric identifier> ::= <non-digit>
    //                             | <non-digit> <identifier characters>
    //                             | <identifier characters> <non-digit>
    //                             | <identifier characters> <non-digit> <identifier characters>
    //
    // <numeric identifier> ::= "0"
    //                        | <positive digit>
    //                        | <positive digit> <digits>
    //
    // <identifier characters> ::= <identifier character>
    //                           | <identifier character> <identifier characters>
    //
    // <identifier character> ::= <digit>
    //                          | <non-digit>
    //
    // <non-digit> ::= <letter>
    //               | "-"
    //
    // <digits> ::= <digit>
    //            | <digit> <digits>
    // ```
    //
    // This is loosely based on WIT syntax and an approximation is parsed here:
    //
    // * This function starts by parsing the optional leading `-` and `+` which
    //   indicates pre-release and build metadata.
    // * Afterwards all of $id, $integer, `-`, and `.` are chomped. The only
    //   exception here is that if `.` isn't followed by $id, $integer, or `-`
    //   then it's assumed that it's something like `use a:b@1.0.0-a.{...}`
    //   where the `.` is part of WIT syntax, not semver.
    //
    // Note that this additionally doesn't try to return any first-class errors.
    // Instead this bails out on something unrecognized for something else in
    // the system to return an error.
    fn eat_ids(tokens: &mut Tokenizer<'_>, prefix: Token, end: &mut Span) -> Result<()> {
        if !tokens.eat(prefix)? {
            return Ok(());
        }
        loop {
            let mut clone = tokens.clone();
            match clone.next()? {
                Some((span, Token::Id | Token::Integer | Token::Minus)) => {
                    end.end = span.end;
                    *tokens = clone;
                }
                Some((_span, Token::Period)) => match clone.next()? {
                    Some((span, Token::Id | Token::Integer | Token::Minus)) => {
                        end.end = span.end;
                        *tokens = clone;
                    }
                    _ => break Ok(()),
                },
                _ => break Ok(()),
            }
        }
    }
}

fn parse_docs<'a>(tokens: &mut Tokenizer<'a>) -> Result<Docs<'a>> {
    let mut docs = Docs::default();
    let mut clone = tokens.clone();
    let mut started = false;
    while let Some((span, token)) = clone.next_raw()? {
        match token {
            Token::Whitespace => {}
            Token::Comment => {
                let comment = tokens.get_span(span);
                if !started {
                    docs.span.start = span.start;
                    started = true;
                }
                let trailing_ws = comment
                    .bytes()
                    .rev()
                    .take_while(|ch| ch.is_ascii_whitespace())
                    .count();
                docs.span.end = span.end - (trailing_ws as u32);
                docs.docs.push(comment.into());
            }
            _ => break,
        };
        *tokens = clone.clone();
    }
    Ok(docs)
}

impl<'a> Type<'a> {
    fn parse(tokens: &mut Tokenizer<'a>) -> Result<Self> {
        match tokens.next()? {
            Some((span, Token::U8)) => Ok(Type::U8(span)),
            Some((span, Token::U16)) => Ok(Type::U16(span)),
            Some((span, Token::U32)) => Ok(Type::U32(span)),
            Some((span, Token::U64)) => Ok(Type::U64(span)),
            Some((span, Token::S8)) => Ok(Type::S8(span)),
            Some((span, Token::S16)) => Ok(Type::S16(span)),
            Some((span, Token::S32)) => Ok(Type::S32(span)),
            Some((span, Token::S64)) => Ok(Type::S64(span)),
            Some((span, Token::F32)) => Ok(Type::F32(span)),
            Some((span, Token::F64)) => Ok(Type::F64(span)),
            Some((span, Token::Char)) => Ok(Type::Char(span)),

            // tuple<T, U, ...>
            Some((span, Token::Tuple)) => {
                let types = parse_list(
                    tokens,
                    Token::LessThan,
                    Token::GreaterThan,
                    |_docs, tokens| Type::parse(tokens),
                )?;
                Ok(Type::Tuple(Tuple { span, types }))
            }

            Some((span, Token::Bool)) => Ok(Type::Bool(span)),
            Some((span, Token::String_)) => Ok(Type::String(span)),

            // list<T>
            // list<T, N>
            Some((span, Token::List)) => {
                tokens.expect(Token::LessThan)?;
                let ty = Type::parse(tokens)?;
                let size = if tokens.eat(Token::Comma)? {
                    let number = tokens.next()?;
                    if let Some((span, Token::Integer)) = number {
                        let size: u32 = tokens.get_span(span).parse()?;
                        Some(size)
                    } else {
                        return Err(err_expected(tokens, "fixed size", number).into());
                    }
                } else {
                    None
                };
                tokens.expect(Token::GreaterThan)?;
                if let Some(size) = size {
                    Ok(Type::FixedSizeList(FixedSizeList {
                        span,
                        ty: Box::new(ty),
                        size,
                    }))
                } else {
                    Ok(Type::List(List {
                        span,
                        ty: Box::new(ty),
                    }))
                }
            }

            // map<K, V>
            Some((span, Token::Map)) => {
                tokens.expect(Token::LessThan)?;
                let key = Type::parse(tokens)?;
                tokens.expect(Token::Comma)?;
                let value = Type::parse(tokens)?;
                tokens.expect(Token::GreaterThan)?;
                Ok(Type::Map(Map {
                    span,
                    key: Box::new(key),
                    value: Box::new(value),
                }))
            }

            // option<T>
            Some((span, Token::Option_)) => {
                tokens.expect(Token::LessThan)?;
                let ty = Type::parse(tokens)?;
                tokens.expect(Token::GreaterThan)?;
                Ok(Type::Option(Option_ {
                    span,
                    ty: Box::new(ty),
                }))
            }

            // result<T, E>
            // result<_, E>
            // result<T>
            // result
            Some((span, Token::Result_)) => {
                let mut ok = None;
                let mut err = None;

                if tokens.eat(Token::LessThan)? {
                    if tokens.eat(Token::Underscore)? {
                        tokens.expect(Token::Comma)?;
                        err = Some(Box::new(Type::parse(tokens)?));
                    } else {
                        ok = Some(Box::new(Type::parse(tokens)?));
                        if tokens.eat(Token::Comma)? {
                            err = Some(Box::new(Type::parse(tokens)?));
                        }
                    };
                    tokens.expect(Token::GreaterThan)?;
                };
                Ok(Type::Result(Result_ { span, ok, err }))
            }

            // future<T>
            // future
            Some((span, Token::Future)) => {
                let mut ty = None;

                if tokens.eat(Token::LessThan)? {
                    ty = Some(Box::new(Type::parse(tokens)?));
                    tokens.expect(Token::GreaterThan)?;
                };
                Ok(Type::Future(Future { span, ty }))
            }

            // stream<T>
            // stream
            Some((span, Token::Stream)) => {
                let mut ty = None;

                if tokens.eat(Token::LessThan)? {
                    ty = Some(Box::new(Type::parse(tokens)?));
                    tokens.expect(Token::GreaterThan)?;
                };
                Ok(Type::Stream(Stream { span, ty }))
            }

            // error-context
            Some((span, Token::ErrorContext)) => Ok(Type::ErrorContext(span)),

            // own<T>
            Some((_span, Token::Own)) => {
                tokens.expect(Token::LessThan)?;
                let resource = parse_id(tokens)?;
                tokens.expect(Token::GreaterThan)?;
                Ok(Type::Handle(Handle::Own { resource }))
            }

            // borrow<T>
            Some((_span, Token::Borrow)) => {
                tokens.expect(Token::LessThan)?;
                let resource = parse_id(tokens)?;
                tokens.expect(Token::GreaterThan)?;
                Ok(Type::Handle(Handle::Borrow { resource }))
            }

            // `foo`
            Some((span, Token::Id)) => Ok(Type::Name(Id {
                name: tokens.parse_id(span)?.into(),
                span,
            })),
            // `%foo`
            Some((span, Token::ExplicitId)) => Ok(Type::Name(Id {
                name: tokens.parse_explicit_id(span)?.into(),
                span,
            })),

            other => Err(err_expected(tokens, "a type", other).into()),
        }
    }

    fn span(&self) -> Span {
        match self {
            Type::Bool(span)
            | Type::U8(span)
            | Type::U16(span)
            | Type::U32(span)
            | Type::U64(span)
            | Type::S8(span)
            | Type::S16(span)
            | Type::S32(span)
            | Type::S64(span)
            | Type::F32(span)
            | Type::F64(span)
            | Type::Char(span)
            | Type::String(span)
            | Type::ErrorContext(span) => *span,
            Type::Name(id) => id.span,
            Type::List(l) => l.span,
            Type::Map(m) => m.span,
            Type::FixedSizeList(l) => l.span,
            Type::Handle(h) => h.span(),
            Type::Resource(r) => r.span,
            Type::Record(r) => r.span,
            Type::Flags(f) => f.span,
            Type::Variant(v) => v.span,
            Type::Tuple(t) => t.span,
            Type::Enum(e) => e.span,
            Type::Option(o) => o.span,
            Type::Result(r) => r.span,
            Type::Future(f) => f.span,
            Type::Stream(s) => s.span,
        }
    }
}

fn parse_list<'a, T>(
    tokens: &mut Tokenizer<'a>,
    start: Token,
    end: Token,
    parse: impl FnMut(Docs<'a>, &mut Tokenizer<'a>) -> Result<T>,
) -> Result<Vec<T>> {
    tokens.expect(start)?;
    parse_list_trailer(tokens, end, parse)
}

fn parse_list_trailer<'a, T>(
    tokens: &mut Tokenizer<'a>,
    end: Token,
    mut parse: impl FnMut(Docs<'a>, &mut Tokenizer<'a>) -> Result<T>,
) -> Result<Vec<T>> {
    let mut items = Vec::new();
    loop {
        // get docs before we skip them to try to eat the end token
        let docs = parse_docs(tokens)?;

        // if we found an end token then we're done
        if tokens.eat(end)? {
            break;
        }

        let item = parse(docs, tokens)?;
        items.push(item);

        // if there's no trailing comma then this is required to be the end,
        // otherwise we go through the loop to try to get another item
        if !tokens.eat(Token::Comma)? {
            tokens.expect(end)?;
            break;
        }
    }
    Ok(items)
}

fn err_expected(
    tokens: &Tokenizer<'_>,
    expected: &'static str,
    found: Option<(Span, Token)>,
) -> Error {
    match found {
        Some((span, token)) => Error::new(
            span,
            format!("expected {}, found {}", expected, token.describe()),
        ),
        None => Error::new(tokens.eof_span(), format!("expected {expected}, found eof")),
    }
}

enum Attribute<'a> {
    Since { span: Span, version: Version },
    Unstable { span: Span, feature: Id<'a> },
    Deprecated { span: Span, version: Version },
}

impl<'a> Attribute<'a> {
    fn parse_list(tokens: &mut Tokenizer<'a>) -> Result<Vec<Attribute<'a>>> {
        let mut ret = Vec::new();
        while tokens.eat(Token::At)? {
            let id = parse_id(tokens)?;
            let attr = match id.name {
                "since" => {
                    tokens.expect(Token::LeftParen)?;
                    eat_id(tokens, "version")?;
                    tokens.expect(Token::Equals)?;
                    let (_span, version) = parse_version(tokens)?;
                    tokens.expect(Token::RightParen)?;
                    Attribute::Since {
                        span: id.span,
                        version,
                    }
                }
                "unstable" => {
                    tokens.expect(Token::LeftParen)?;
                    eat_id(tokens, "feature")?;
                    tokens.expect(Token::Equals)?;
                    let feature = parse_id(tokens)?;
                    tokens.expect(Token::RightParen)?;
                    Attribute::Unstable {
                        span: id.span,
                        feature,
                    }
                }
                "deprecated" => {
                    tokens.expect(Token::LeftParen)?;
                    eat_id(tokens, "version")?;
                    tokens.expect(Token::Equals)?;
                    let (_span, version) = parse_version(tokens)?;
                    tokens.expect(Token::RightParen)?;
                    Attribute::Deprecated {
                        span: id.span,
                        version,
                    }
                }
                other => {
                    bail!(Error::new(id.span, format!("unknown attribute `{other}`"),))
                }
            };
            ret.push(attr);
        }
        Ok(ret)
    }

    fn span(&self) -> Span {
        match self {
            Attribute::Since { span, .. }
            | Attribute::Unstable { span, .. }
            | Attribute::Deprecated { span, .. } => *span,
        }
    }
}

fn eat_id(tokens: &mut Tokenizer<'_>, expected: &str) -> Result<Span> {
    let id = parse_id(tokens)?;
    if id.name != expected {
        bail!(Error::new(
            id.span,
            format!("expected `{expected}`, found `{}`", id.name),
        ));
    }
    Ok(id.span)
}

/// A listing of source files which are used to get parsed into an
/// [`UnresolvedPackage`].
///
/// [`UnresolvedPackage`]: crate::UnresolvedPackage
#[derive(Clone, Default)]
pub struct SourceMap {
    sources: Vec<Source>,
    offset: u32,
    require_f32_f64: Option<bool>,
}

#[derive(Clone)]
struct Source {
    offset: u32,
    path: PathBuf,
    contents: String,
}

impl SourceMap {
    /// Creates a new empty source map.
    pub fn new() -> SourceMap {
        SourceMap::default()
    }

    #[doc(hidden)] // NB: only here for a transitionary period
    pub fn set_require_f32_f64(&mut self, enable: bool) {
        self.require_f32_f64 = Some(enable);
    }

    /// Reads the file `path` on the filesystem and appends its contents to this
    /// [`SourceMap`].
    pub fn push_file(&mut self, path: &Path) -> Result<()> {
        let contents = std::fs::read_to_string(path)
            .with_context(|| format!("failed to read file {path:?}"))?;
        self.push(path, contents);
        Ok(())
    }

    /// Appends the given contents with the given path into this source map.
    ///
    /// The `path` provided is not read from the filesystem and is instead only
    /// used during error messages. Each file added to a [`SourceMap`] is
    /// used to create the final parsed package namely by unioning all the
    /// interfaces and worlds defined together. Note that each file has its own
    /// personal namespace, however, for top-level `use` and such.
    pub fn push(&mut self, path: &Path, contents: impl Into<String>) {
        let mut contents = contents.into();
        // Guarantee that there's at least one character in these contents by
        // appending a single newline to the end. This is excluded from
        // tokenization below so it's only here to ensure that spans which point
        // one byte beyond the end of a file (eof) point to the same original
        // file.
        contents.push('\n');
        let new_offset = self.offset + u32::try_from(contents.len()).unwrap();
        self.sources.push(Source {
            offset: self.offset,
            path: path.to_path_buf(),
            contents,
        });
        self.offset = new_offset;
    }

    /// Parses the files added to this source map into a
    /// [`UnresolvedPackageGroup`].
    pub fn parse(self) -> Result<UnresolvedPackageGroup> {
        let mut nested = Vec::new();
        let main = self.rewrite_error(|| {
            let mut resolver = Resolver::default();
            let mut srcs = self.sources.iter().collect::<Vec<_>>();
            srcs.sort_by_key(|src| &src.path);

            // Parse each source file individually. A tokenizer is created here
            // form settings and then `PackageFile` is used to parse the whole
            // stream of tokens.
            for src in srcs {
                let mut tokens = Tokenizer::new(
                    // chop off the forcibly appended `\n` character when
                    // passing through the source to get tokenized.
                    &src.contents[..src.contents.len() - 1],
                    src.offset,
                    self.require_f32_f64,
                )
                .with_context(|| format!("failed to tokenize path: {}", src.path.display()))?;
                let mut file = PackageFile::parse(&mut tokens)?;

                // Filter out any nested packages and resolve them separately.
                // Nested packages have only a single "file" so only one item
                // is pushed into a `Resolver`. Note that a nested `Resolver`
                // is used here, not the outer one.
                //
                // Note that filtering out `Package` items is required due to
                // how the implementation of disallowing nested packages in
                // nested packages currently works.
                for item in mem::take(&mut file.decl_list.items) {
                    match item {
                        AstItem::Package(nested_pkg) => {
                            let mut resolve = Resolver::default();
                            resolve.push(nested_pkg).with_context(|| {
                                format!(
                                    "failed to handle nested package in: {}",
                                    src.path.display()
                                )
                            })?;

                            nested.push(resolve.resolve()?);
                        }
                        other => file.decl_list.items.push(other),
                    }
                }

                // With nested packages handled push this file into the
                // resolver.
                resolver.push(file).with_context(|| {
                    format!("failed to start resolving path: {}", src.path.display())
                })?;
            }
            Ok(resolver.resolve()?)
        })?;
        Ok(UnresolvedPackageGroup {
            main,
            nested,
            source_map: self,
        })
    }

    pub(crate) fn rewrite_error<F, T>(&self, f: F) -> Result<T>
    where
        F: FnOnce() -> Result<T>,
    {
        let mut err = match f() {
            Ok(t) => return Ok(t),
            Err(e) => e,
        };
        if let Some(parse) = err.downcast_mut::<Error>() {
            if parse.highlighted.is_none() {
                let msg = self.highlight_err(parse.span.start, Some(parse.span.end), &parse.msg);
                parse.highlighted = Some(msg);
            }
        }
        if let Some(_) = err.downcast_mut::<Error>() {
            return Err(err);
        }
        if let Some(notfound) = err.downcast_mut::<PackageNotFoundError>() {
            if notfound.highlighted.is_none() {
                let msg = self.highlight_err(
                    notfound.span.start,
                    Some(notfound.span.end),
                    &format!("{notfound}"),
                );
                notfound.highlighted = Some(msg);
            }
        }
        if let Some(_) = err.downcast_mut::<PackageNotFoundError>() {
            return Err(err);
        }

        if let Some(lex) = err.downcast_ref::<lex::Error>() {
            let pos = match lex {
                lex::Error::Unexpected(at, _)
                | lex::Error::UnterminatedComment(at)
                | lex::Error::Wanted { at, .. }
                | lex::Error::InvalidCharInId(at, _)
                | lex::Error::IdPartEmpty(at)
                | lex::Error::InvalidEscape(at, _) => *at,
            };
            let msg = self.highlight_err(pos, None, lex);
            bail!("{msg}")
        }

        if let Some(sort) = err.downcast_mut::<toposort::Error>() {
            if sort.highlighted().is_none() {
                let span = match sort {
                    toposort::Error::NonexistentDep { span, .. }
                    | toposort::Error::Cycle { span, .. } => *span,
                };
                let highlighted = self.highlight_err(span.start, Some(span.end), &sort);
                sort.set_highlighted(highlighted);
            }
        }

        Err(err)
    }

    fn highlight_err(&self, start: u32, end: Option<u32>, err: impl fmt::Display) -> String {
        let src = self.source_for_offset(start);
        let start = src.to_relative_offset(start);
        let end = end.map(|end| src.to_relative_offset(end));
        let (line, col) = src.linecol(start);
        let snippet = src.contents.lines().nth(line).unwrap_or("");
        let mut msg = format!(
            "\
{err}
     --> {file}:{line}:{col}
      |
 {line:4} | {snippet}
      | {marker:>0$}",
            col + 1,
            file = src.path.display(),
            line = line + 1,
            col = col + 1,
            marker = "^",
        );
        if let Some(end) = end {
            if let Some(s) = src.contents.get(start..end) {
                for _ in s.chars().skip(1) {
                    msg.push('-');
                }
            }
        }
        return msg;
    }

    pub(crate) fn render_location(&self, span: Span) -> String {
        let src = self.source_for_offset(span.start);
        let start = src.to_relative_offset(span.start);
        let (line, col) = src.linecol(start);
        format!(
            "{file}:{line}:{col}",
            file = src.path.display(),
            line = line + 1,
            col = col + 1,
        )
    }

    fn source_for_offset(&self, start: u32) -> &Source {
        let i = match self.sources.binary_search_by_key(&start, |src| src.offset) {
            Ok(i) => i,
            Err(i) => i - 1,
        };
        &self.sources[i]
    }

    /// Returns an iterator over all filenames added to this source map.
    pub fn source_files(&self) -> impl Iterator<Item = &Path> {
        self.sources.iter().map(|src| src.path.as_path())
    }
}

impl Source {
    fn to_relative_offset(&self, offset: u32) -> usize {
        usize::try_from(offset - self.offset).unwrap()
    }

    fn linecol(&self, relative_offset: usize) -> (usize, usize) {
        let mut cur = 0;
        // Use split_terminator instead of lines so that if there is a `\r`,
        // it is included in the offset calculation. The `+1` values below
        // account for the `\n`.
        for (i, line) in self.contents.split_terminator('\n').enumerate() {
            if cur + line.len() + 1 > relative_offset {
                return (i, relative_offset - cur);
            }
            cur += line.len() + 1;
        }
        (self.contents.lines().count(), 0)
    }
}

pub enum ParsedUsePath {
    Name(String),
    Package(crate::PackageName, String),
}

pub fn parse_use_path(s: &str) -> Result<ParsedUsePath> {
    let mut tokens = Tokenizer::new(s, 0, None)?;
    let path = UsePath::parse(&mut tokens)?;
    if tokens.next()?.is_some() {
        bail!("trailing tokens in path specifier");
    }
    Ok(match path {
        UsePath::Id(id) => ParsedUsePath::Name(id.name.to_string()),
        UsePath::Package { id, name } => {
            ParsedUsePath::Package(id.package_name(), name.name.to_string())
        }
    })
}
