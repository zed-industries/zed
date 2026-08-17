use anyhow::{Context, Result};
use id_arena::{Arena, Id};
use indexmap::IndexMap;
use semver::Version;
use std::borrow::Cow;
use std::fmt;
use std::path::Path;

#[cfg(feature = "decoding")]
pub mod decoding;
#[cfg(feature = "decoding")]
mod docs;
#[cfg(feature = "decoding")]
pub use docs::PackageDocs;

pub mod abi;
mod ast;
use ast::lex::Span;
pub use ast::SourceMap;
mod sizealign;
pub use sizealign::*;
mod resolve;
pub use resolve::{Package, PackageId, Remap, Resolve};
mod live;
pub use live::LiveTypes;

#[cfg(feature = "serde")]
use serde_derive::Serialize;
#[cfg(feature = "serde")]
mod serde_;
#[cfg(feature = "serde")]
use serde_::{
    serialize_anon_result, serialize_id, serialize_id_map, serialize_none, serialize_optional_id,
    serialize_params,
};

/// Checks if the given string is a legal identifier in wit.
pub fn validate_id(s: &str) -> Result<()> {
    ast::validate_id(0, s)?;
    Ok(())
}

pub type WorldId = Id<World>;
pub type InterfaceId = Id<Interface>;
pub type TypeId = Id<TypeDef>;

/// Representation of a parsed WIT package which has not resolved external
/// dependencies yet.
///
/// This representation has performed internal resolution of the WIT package
/// itself, ensuring that all references internally are valid and the WIT was
/// syntactically valid and such.
///
/// The fields of this structure represent a flat list of arrays unioned from
/// all documents within the WIT package. This means, for example, that all
/// types from all documents are located in `self.types`. The fields of each
/// item can help splitting back out into packages/interfaces/etc as necessary.
///
/// Note that an `UnresolvedPackage` cannot be queried in general about
/// information such as size or alignment as that would require resolution of
/// foreign dependencies. Translations such as to-binary additionally are not
/// supported on an `UnresolvedPackage` due to the lack of knowledge about the
/// foreign types. This is intended to be an intermediate state which can be
/// inspected by embedders, if necessary, before quickly transforming to a
/// [`Resolve`] to fully work with a WIT package.
///
/// After an [`UnresolvedPackage`] is parsed it can be fully resolved with
/// [`Resolve::push`]. During this operation a dependency map is specified which
/// will connect the `foreign_deps` field of this structure to packages
/// previously inserted within the [`Resolve`]. Embedders are responsible for
/// performing this resolution themselves.
#[derive(Clone)]
pub struct UnresolvedPackage {
    /// The namespace, name, and version information for this package.
    pub name: PackageName,

    /// All worlds from all documents within this package.
    ///
    /// Each world lists the document that it is from.
    pub worlds: Arena<World>,

    /// All interfaces from all documents within this package.
    ///
    /// Each interface lists the document that it is from. Interfaces are listed
    /// in topological order as well so iteration through this arena will only
    /// reference prior elements already visited when working with recursive
    /// references.
    pub interfaces: Arena<Interface>,

    /// All types from all documents within this package.
    ///
    /// Each type lists the interface or world that defined it, or nothing if
    /// it's an anonymous type. Types are listed in this arena in topological
    /// order to ensure that iteration through this arena will only reference
    /// other types transitively that are already iterated over.
    pub types: Arena<TypeDef>,

    /// All foreign dependencies that this package depends on.
    ///
    /// These foreign dependencies must be resolved to convert this unresolved
    /// package into a `Resolve`. The map here is keyed by the name of the
    /// foreign package that this depends on, and the sub-map is keyed by an
    /// interface name followed by the identifier within `self.interfaces`. The
    /// fields of `self.interfaces` describes the required types that are from
    /// each foreign interface.
    pub foreign_deps: IndexMap<PackageName, IndexMap<String, AstItem>>,

    /// Doc comments for this package.
    pub docs: Docs,

    unknown_type_spans: Vec<Span>,
    world_item_spans: Vec<(Vec<Span>, Vec<Span>)>,
    interface_spans: Vec<Span>,
    world_spans: Vec<Span>,
    foreign_dep_spans: Vec<Span>,
    source_map: SourceMap,
    include_world_spans: Vec<Span>,
    required_resource_types: Vec<(TypeId, Span)>,
}

#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "lowercase"))]
pub enum AstItem {
    #[cfg_attr(feature = "serde", serde(serialize_with = "serialize_id"))]
    Interface(InterfaceId),
    #[cfg_attr(feature = "serde", serde(serialize_with = "serialize_id"))]
    World(WorldId),
}

/// A structure used to keep track of the name of a package, containing optional
/// information such as a namespace and version information.
///
/// This is directly encoded as an "ID" in the binary component representation
/// with an interfaced tacked on as well.
#[derive(Debug, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize))]
#[cfg_attr(feature = "serde", serde(into = "String"))]
pub struct PackageName {
    /// A namespace such as `wasi` in `wasi:foo/bar`
    pub namespace: String,
    /// The kebab-name of this package, which is always specified.
    pub name: String,
    /// Optional major/minor version information.
    pub version: Option<Version>,
}

impl From<PackageName> for String {
    fn from(name: PackageName) -> String {
        name.to_string()
    }
}

impl PackageName {
    /// Returns the ID that this package name would assign the `interface` name
    /// specified.
    pub fn interface_id(&self, interface: &str) -> String {
        let mut s = String::new();
        s.push_str(&format!("{}:{}/{interface}", self.namespace, self.name));
        if let Some(version) = &self.version {
            s.push_str(&format!("@{version}"));
        }
        s
    }
}

impl fmt::Display for PackageName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.namespace, self.name)?;
        if let Some(version) = &self.version {
            write!(f, "@{version}")?;
        }
        Ok(())
    }
}

#[derive(Debug)]
struct Error {
    span: Span,
    msg: String,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.msg.fmt(f)
    }
}

impl std::error::Error for Error {}

impl UnresolvedPackage {
    /// Parses the given string as a wit document.
    ///
    /// The `path` argument is used for error reporting. The `contents` provided
    /// will not be able to use `pkg` use paths to other documents.
    pub fn parse(path: &Path, contents: &str) -> Result<Self> {
        let mut map = SourceMap::default();
        map.push(path, contents);
        map.parse()
    }

    /// Parse a WIT package at the provided path.
    ///
    /// The path provided is inferred whether it's a file or a directory. A file
    /// is parsed with [`UnresolvedPackage::parse_file`] and a directory is
    /// parsed with [`UnresolvedPackage::parse_dir`].
    pub fn parse_path(path: &Path) -> Result<Self> {
        if path.is_dir() {
            UnresolvedPackage::parse_dir(path)
        } else {
            UnresolvedPackage::parse_file(path)
        }
    }

    /// Parses a WIT package from the file provided.
    ///
    /// The WIT package returned will be a single-document package and will not
    /// be able to use `pkg` paths to other documents.
    pub fn parse_file(path: &Path) -> Result<Self> {
        let contents = std::fs::read_to_string(path)
            .with_context(|| format!("failed to read file {path:?}"))?;
        Self::parse(path, &contents)
    }

    /// Parses a WIT package from the directory provided.
    ///
    /// All files with the extension `*.wit` or `*.wit.md` will be loaded from
    /// `path` into the returned package.
    pub fn parse_dir(path: &Path) -> Result<Self> {
        let mut map = SourceMap::default();
        let cx = || format!("failed to read directory {path:?}");
        for entry in path.read_dir().with_context(&cx)? {
            let entry = entry.with_context(&cx)?;
            let path = entry.path();
            let ty = entry.file_type().with_context(&cx)?;
            if ty.is_dir() {
                continue;
            }
            if ty.is_symlink() {
                if path.is_dir() {
                    continue;
                }
            }
            let filename = match path.file_name().and_then(|s| s.to_str()) {
                Some(name) => name,
                None => continue,
            };
            if !filename.ends_with(".wit") && !filename.ends_with(".wit.md") {
                continue;
            }
            map.push_file(&path)?;
        }
        map.parse()
    }

    /// Returns an iterator over the list of source files that were read when
    /// parsing this package.
    pub fn source_files(&self) -> impl Iterator<Item = &Path> {
        self.source_map.source_files()
    }
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct World {
    /// The WIT identifier name of this world.
    pub name: String,

    /// All imported items into this interface, both worlds and functions.
    pub imports: IndexMap<WorldKey, WorldItem>,

    /// All exported items from this interface, both worlds and functions.
    pub exports: IndexMap<WorldKey, WorldItem>,

    /// The package that owns this world.
    #[cfg_attr(feature = "serde", serde(serialize_with = "serialize_optional_id"))]
    pub package: Option<PackageId>,

    /// Documentation associated with this world declaration.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Docs::is_empty"))]
    pub docs: Docs,

    /// All the included worlds from this world. Empty if this is fully resolved
    #[cfg_attr(feature = "serde", serde(skip))]
    pub includes: Vec<WorldId>,

    /// All the included worlds names. Empty if this is fully resolved
    #[cfg_attr(feature = "serde", serde(skip))]
    pub include_names: Vec<Vec<IncludeName>>,
}

#[derive(Debug, Clone)]
pub struct IncludeName {
    /// The name of the item
    pub name: String,

    /// The name to be replaced with
    pub as_: String,
}

/// The key to the import/export maps of a world. Either a kebab-name or a
/// unique interface.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
#[cfg_attr(feature = "serde", serde(into = "String"))]
pub enum WorldKey {
    /// A kebab-name.
    Name(String),
    /// An interface which is assigned no kebab-name.
    Interface(InterfaceId),
}

impl From<WorldKey> for String {
    fn from(key: WorldKey) -> String {
        match key {
            WorldKey::Name(name) => name,
            WorldKey::Interface(id) => format!("interface-{}", id.index()),
        }
    }
}

impl WorldKey {
    /// Asserts that this is `WorldKey::Name` and returns the name.
    #[track_caller]
    pub fn unwrap_name(self) -> String {
        match self {
            WorldKey::Name(name) => name,
            WorldKey::Interface(_) => panic!("expected a name, found interface"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "lowercase"))]
pub enum WorldItem {
    /// An interface is being imported or exported from a world, indicating that
    /// it's a namespace of functions.
    #[cfg_attr(feature = "serde", serde(serialize_with = "serialize_id"))]
    Interface(InterfaceId),

    /// A function is being directly imported or exported from this world.
    Function(Function),

    /// A type is being exported from this world.
    ///
    /// Note that types are never imported into worlds at this time.
    #[cfg_attr(feature = "serde", serde(serialize_with = "serialize_id"))]
    Type(TypeId),
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct Interface {
    /// Optionally listed name of this interface.
    ///
    /// This is `None` for inline interfaces in worlds.
    pub name: Option<String>,

    /// Exported types from this interface.
    ///
    /// Export names are listed within the types themselves. Note that the
    /// export name here matches the name listed in the `TypeDef`.
    #[cfg_attr(feature = "serde", serde(serialize_with = "serialize_id_map"))]
    pub types: IndexMap<String, TypeId>,

    /// Exported functions from this interface.
    pub functions: IndexMap<String, Function>,

    /// Documentation associated with this interface.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Docs::is_empty"))]
    pub docs: Docs,

    /// The package that owns this interface.
    #[cfg_attr(feature = "serde", serde(serialize_with = "serialize_optional_id"))]
    pub package: Option<PackageId>,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct TypeDef {
    pub name: Option<String>,
    pub kind: TypeDefKind,
    pub owner: TypeOwner,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Docs::is_empty"))]
    pub docs: Docs,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "lowercase"))]
pub enum TypeDefKind {
    Record(Record),
    Resource,
    Handle(Handle),
    Flags(Flags),
    Tuple(Tuple),
    Variant(Variant),
    Enum(Enum),
    Option(Type),
    Result(Result_),
    List(Type),
    Future(Option<Type>),
    Stream(Stream),
    Type(Type),

    /// This represents a type of unknown structure imported from a foreign
    /// interface.
    ///
    /// This variant is only used during the creation of `UnresolvedPackage` but
    /// by the time a `Resolve` is created then this will not exist.
    Unknown,
}

impl TypeDefKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            TypeDefKind::Record(_) => "record",
            TypeDefKind::Resource => "resource",
            TypeDefKind::Handle(handle) => match handle {
                Handle::Own(_) => "own",
                Handle::Borrow(_) => "borrow",
            },
            TypeDefKind::Flags(_) => "flags",
            TypeDefKind::Tuple(_) => "tuple",
            TypeDefKind::Variant(_) => "variant",
            TypeDefKind::Enum(_) => "enum",
            TypeDefKind::Option(_) => "option",
            TypeDefKind::Result(_) => "result",
            TypeDefKind::List(_) => "list",
            TypeDefKind::Future(_) => "future",
            TypeDefKind::Stream(_) => "stream",
            TypeDefKind::Type(_) => "type",
            TypeDefKind::Unknown => "unknown",
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "lowercase"))]
pub enum TypeOwner {
    /// This type was defined within a `world` block.
    #[cfg_attr(feature = "serde", serde(serialize_with = "serialize_id"))]
    World(WorldId),
    /// This type was defined within an `interface` block.
    #[cfg_attr(feature = "serde", serde(serialize_with = "serialize_id"))]
    Interface(InterfaceId),
    /// This type wasn't inherently defined anywhere, such as a `list<T>`, which
    /// doesn't need an owner.
    #[cfg_attr(feature = "serde", serde(untagged, serialize_with = "serialize_none"))]
    None,
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "lowercase"))]
pub enum Handle {
    #[cfg_attr(feature = "serde", serde(serialize_with = "serialize_id"))]
    Own(TypeId),
    #[cfg_attr(feature = "serde", serde(serialize_with = "serialize_id"))]
    Borrow(TypeId),
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub enum Type {
    Bool,
    U8,
    U16,
    U32,
    U64,
    S8,
    S16,
    S32,
    S64,
    Float32,
    Float64,
    Char,
    String,
    Id(TypeId),
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Int {
    U8,
    U16,
    U32,
    U64,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct Record {
    pub fields: Vec<Field>,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct Field {
    pub name: String,
    #[cfg_attr(feature = "serde", serde(rename = "type"))]
    pub ty: Type,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Docs::is_empty"))]
    pub docs: Docs,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct Flags {
    pub flags: Vec<Flag>,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct Flag {
    pub name: String,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Docs::is_empty"))]
    pub docs: Docs,
}

#[derive(Debug, Clone, PartialEq)]
pub enum FlagsRepr {
    U8,
    U16,
    U32(usize),
}

impl Flags {
    pub fn repr(&self) -> FlagsRepr {
        match self.flags.len() {
            0 => FlagsRepr::U32(0),
            n if n <= 8 => FlagsRepr::U8,
            n if n <= 16 => FlagsRepr::U16,
            n => FlagsRepr::U32(sizealign::align_to(n, 32) / 32),
        }
    }
}

impl FlagsRepr {
    pub fn count(&self) -> usize {
        match self {
            FlagsRepr::U8 => 1,
            FlagsRepr::U16 => 1,
            FlagsRepr::U32(n) => *n,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct Tuple {
    pub types: Vec<Type>,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct Variant {
    pub cases: Vec<Case>,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct Case {
    pub name: String,
    #[cfg_attr(feature = "serde", serde(rename = "type"))]
    pub ty: Option<Type>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Docs::is_empty"))]
    pub docs: Docs,
}

impl Variant {
    pub fn tag(&self) -> Int {
        match self.cases.len() {
            n if n <= u8::max_value() as usize => Int::U8,
            n if n <= u16::max_value() as usize => Int::U16,
            n if n <= u32::max_value() as usize => Int::U32,
            _ => panic!("too many cases to fit in a repr"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct Enum {
    pub cases: Vec<EnumCase>,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct EnumCase {
    pub name: String,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Docs::is_empty"))]
    pub docs: Docs,
}

impl Enum {
    pub fn tag(&self) -> Int {
        match self.cases.len() {
            n if n <= u8::max_value() as usize => Int::U8,
            n if n <= u16::max_value() as usize => Int::U16,
            n if n <= u32::max_value() as usize => Int::U32,
            _ => panic!("too many cases to fit in a repr"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct Result_ {
    pub ok: Option<Type>,
    pub err: Option<Type>,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct Stream {
    pub element: Option<Type>,
    pub end: Option<Type>,
}

#[derive(Clone, Default, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct Docs {
    pub contents: Option<String>,
}

impl Docs {
    pub fn is_empty(&self) -> bool {
        self.contents.is_none()
    }
}

pub type Params = Vec<(String, Type)>;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum Results {
    #[cfg_attr(feature = "serde", serde(serialize_with = "serialize_params"))]
    Named(Params),
    #[cfg_attr(feature = "serde", serde(serialize_with = "serialize_anon_result"))]
    Anon(Type),
}

pub enum ResultsTypeIter<'a> {
    Named(std::slice::Iter<'a, (String, Type)>),
    Anon(std::iter::Once<&'a Type>),
}

impl<'a> Iterator for ResultsTypeIter<'a> {
    type Item = &'a Type;

    fn next(&mut self) -> Option<&'a Type> {
        match self {
            ResultsTypeIter::Named(ps) => ps.next().map(|p| &p.1),
            ResultsTypeIter::Anon(ty) => ty.next(),
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        match self {
            ResultsTypeIter::Named(ps) => ps.size_hint(),
            ResultsTypeIter::Anon(ty) => ty.size_hint(),
        }
    }
}

impl<'a> ExactSizeIterator for ResultsTypeIter<'a> {}

impl Results {
    // For the common case of an empty results list.
    pub fn empty() -> Results {
        Results::Named(Vec::new())
    }

    pub fn len(&self) -> usize {
        match self {
            Results::Named(params) => params.len(),
            Results::Anon(_) => 1,
        }
    }

    pub fn throws<'a>(&self, resolve: &'a Resolve) -> Option<(Option<&'a Type>, Option<&'a Type>)> {
        if self.len() != 1 {
            return None;
        }
        match self.iter_types().next().unwrap() {
            Type::Id(id) => match &resolve.types[*id].kind {
                TypeDefKind::Result(r) => Some((r.ok.as_ref(), r.err.as_ref())),
                _ => None,
            },
            _ => None,
        }
    }

    pub fn iter_types(&self) -> ResultsTypeIter {
        match self {
            Results::Named(ps) => ResultsTypeIter::Named(ps.iter()),
            Results::Anon(ty) => ResultsTypeIter::Anon(std::iter::once(ty)),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct Function {
    pub name: String,
    pub kind: FunctionKind,
    #[cfg_attr(feature = "serde", serde(serialize_with = "serialize_params"))]
    pub params: Params,
    pub results: Results,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Docs::is_empty"))]
    pub docs: Docs,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "lowercase"))]
pub enum FunctionKind {
    Freestanding,
    #[cfg_attr(feature = "serde", serde(serialize_with = "serialize_id"))]
    Method(TypeId),
    #[cfg_attr(feature = "serde", serde(serialize_with = "serialize_id"))]
    Static(TypeId),
    #[cfg_attr(feature = "serde", serde(serialize_with = "serialize_id"))]
    Constructor(TypeId),
}

impl Function {
    pub fn item_name(&self) -> &str {
        match &self.kind {
            FunctionKind::Freestanding => &self.name,
            FunctionKind::Method(_) | FunctionKind::Static(_) => {
                &self.name[self.name.find('.').unwrap() + 1..]
            }
            FunctionKind::Constructor(_) => "constructor",
        }
    }

    /// Gets the core export name for this function.
    pub fn core_export_name<'a>(&'a self, interface: Option<&str>) -> Cow<'a, str> {
        match interface {
            Some(interface) => Cow::Owned(format!("{interface}#{}", self.name)),
            None => Cow::Borrowed(&self.name),
        }
    }
}
