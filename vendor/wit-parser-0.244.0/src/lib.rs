use crate::abi::AbiVariant;
use anyhow::{Context, Result, bail};
use id_arena::{Arena, Id};
use indexmap::IndexMap;
use semver::Version;
use std::borrow::Cow;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::path::Path;

#[cfg(feature = "decoding")]
pub mod decoding;
#[cfg(feature = "decoding")]
mod metadata;
#[cfg(feature = "decoding")]
pub use metadata::PackageMetadata;

pub mod abi;
mod ast;
pub use ast::SourceMap;
use ast::lex::Span;
pub use ast::{ParsedUsePath, parse_use_path};
mod sizealign;
pub use sizealign::*;
mod resolve;
pub use resolve::*;
mod live;
pub use live::{LiveTypes, TypeIdVisitor};

#[cfg(feature = "serde")]
use serde_derive::Serialize;
#[cfg(feature = "serde")]
mod serde_;
#[cfg(feature = "serde")]
use serde_::*;

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
    pub foreign_deps: IndexMap<PackageName, IndexMap<String, (AstItem, Vec<Stability>)>>,

    /// Doc comments for this package.
    pub docs: Docs,

    package_name_span: Span,
    unknown_type_spans: Vec<Span>,
    interface_spans: Vec<InterfaceSpan>,
    world_spans: Vec<WorldSpan>,
    type_spans: Vec<Span>,
    foreign_dep_spans: Vec<Span>,
    required_resource_types: Vec<(TypeId, Span)>,
}

/// Tracks a set of packages, all pulled from the same group of WIT source files.
#[derive(Clone)]
pub struct UnresolvedPackageGroup {
    /// The "main" package in this package group which was found at the root of
    /// the WIT files.
    ///
    /// Note that this is required to be present in all WIT files.
    pub main: UnresolvedPackage,

    /// Nested packages found while parsing `main`, if any.
    pub nested: Vec<UnresolvedPackage>,

    /// A set of processed source files from which these packages have been parsed.
    pub source_map: SourceMap,
}

#[derive(Clone)]
struct WorldSpan {
    span: Span,
    imports: Vec<Span>,
    exports: Vec<Span>,
    includes: Vec<Span>,
}

#[derive(Clone)]
struct InterfaceSpan {
    span: Span,
    funcs: Vec<Span>,
}

#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "kebab-case"))]
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

    /// Determines the "semver compatible track" for the given version.
    ///
    /// This method implements the logic from the component model where semver
    /// versions can be compatible with one another. For example versions 1.2.0
    /// and 1.2.1 would be considered both compatible with one another because
    /// they're on the same semver compatible track.
    ///
    /// This predicate is used during
    /// [`Resolve::merge_world_imports_based_on_semver`] for example to
    /// determine whether two imports can be merged together. This is
    /// additionally used when creating components to match up imports in
    /// core wasm to imports in worlds.
    pub fn version_compat_track(version: &Version) -> Version {
        let mut version = version.clone();
        version.build = semver::BuildMetadata::EMPTY;
        if !version.pre.is_empty() {
            return version;
        }
        if version.major != 0 {
            version.minor = 0;
            version.patch = 0;
            return version;
        }
        if version.minor != 0 {
            version.patch = 0;
            return version;
        }
        version
    }

    /// Returns the string corresponding to
    /// [`PackageName::version_compat_track`]. This is done to match the
    /// component model's expected naming scheme of imports and exports.
    pub fn version_compat_track_string(version: &Version) -> String {
        let version = Self::version_compat_track(version);
        if !version.pre.is_empty() {
            return version.to_string();
        }
        if version.major != 0 {
            return format!("{}", version.major);
        }
        if version.minor != 0 {
            return format!("{}.{}", version.major, version.minor);
        }
        version.to_string()
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
    highlighted: Option<String>,
}

impl Error {
    fn new(span: Span, msg: impl Into<String>) -> Error {
        Error {
            span,
            msg: msg.into(),
            highlighted: None,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.highlighted.as_ref().unwrap_or(&self.msg).fmt(f)
    }
}

impl std::error::Error for Error {}

#[derive(Debug)]
struct PackageNotFoundError {
    span: Span,
    requested: PackageName,
    known: Vec<PackageName>,
    highlighted: Option<String>,
}

impl PackageNotFoundError {
    pub fn new(span: Span, requested: PackageName, known: Vec<PackageName>) -> Self {
        Self {
            span,
            requested,
            known,
            highlighted: None,
        }
    }
}

impl fmt::Display for PackageNotFoundError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(highlighted) = &self.highlighted {
            return highlighted.fmt(f);
        }
        if self.known.is_empty() {
            write!(
                f,
                "package '{}' not found. no known packages.",
                self.requested
            )?;
        } else {
            write!(
                f,
                "package '{}' not found. known packages:\n",
                self.requested
            )?;
            for known in self.known.iter() {
                write!(f, "    {known}\n")?;
            }
        }
        Ok(())
    }
}

impl std::error::Error for PackageNotFoundError {}

impl UnresolvedPackageGroup {
    /// Parses the given string as a wit document.
    ///
    /// The `path` argument is used for error reporting. The `contents` provided
    /// are considered to be the contents of `path`. This function does not read
    /// the filesystem.
    pub fn parse(path: impl AsRef<Path>, contents: &str) -> Result<UnresolvedPackageGroup> {
        let mut map = SourceMap::default();
        map.push(path.as_ref(), contents);
        map.parse()
    }

    /// Parse a WIT package at the provided path.
    ///
    /// The path provided is inferred whether it's a file or a directory. A file
    /// is parsed with [`UnresolvedPackageGroup::parse_file`] and a directory is
    /// parsed with [`UnresolvedPackageGroup::parse_dir`].
    pub fn parse_path(path: impl AsRef<Path>) -> Result<UnresolvedPackageGroup> {
        let path = path.as_ref();
        if path.is_dir() {
            UnresolvedPackageGroup::parse_dir(path)
        } else {
            UnresolvedPackageGroup::parse_file(path)
        }
    }

    /// Parses a WIT package from the file provided.
    ///
    /// The return value represents all packages found in the WIT file which
    /// might be either one or multiple depending on the syntax used.
    pub fn parse_file(path: impl AsRef<Path>) -> Result<UnresolvedPackageGroup> {
        let path = path.as_ref();
        let contents = std::fs::read_to_string(path)
            .with_context(|| format!("failed to read file {path:?}"))?;
        Self::parse(path, &contents)
    }

    /// Parses a WIT package from the directory provided.
    ///
    /// This method will look at all files under the `path` specified. All
    /// `*.wit` files are parsed and assumed to be part of the same package
    /// grouping. This is useful when a WIT package is split across multiple
    /// files.
    pub fn parse_dir(path: impl AsRef<Path>) -> Result<UnresolvedPackageGroup> {
        let path = path.as_ref();
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
            if !filename.ends_with(".wit") {
                continue;
            }
            map.push_file(&path)?;
        }
        map.parse()
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

    /// Stability annotation for this world itself.
    #[cfg_attr(
        feature = "serde",
        serde(skip_serializing_if = "Stability::is_unknown")
    )]
    pub stability: Stability,

    /// All the included worlds from this world. Empty if this is fully resolved
    #[cfg_attr(feature = "serde", serde(skip))]
    pub includes: Vec<(Stability, WorldId)>,

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
#[derive(Debug, Clone, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize))]
#[cfg_attr(feature = "serde", serde(into = "String"))]
pub enum WorldKey {
    /// A kebab-name.
    Name(String),
    /// An interface which is assigned no kebab-name.
    Interface(InterfaceId),
}

impl Hash for WorldKey {
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        match self {
            WorldKey::Name(s) => {
                0u8.hash(hasher);
                s.as_str().hash(hasher);
            }
            WorldKey::Interface(i) => {
                1u8.hash(hasher);
                i.hash(hasher);
            }
        }
    }
}

impl PartialEq for WorldKey {
    fn eq(&self, other: &WorldKey) -> bool {
        match (self, other) {
            (WorldKey::Name(a), WorldKey::Name(b)) => a.as_str() == b.as_str(),
            (WorldKey::Name(_), _) => false,
            (WorldKey::Interface(a), WorldKey::Interface(b)) => a == b,
            (WorldKey::Interface(_), _) => false,
        }
    }
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
#[cfg_attr(feature = "serde", serde(rename_all = "kebab-case"))]
pub enum WorldItem {
    /// An interface is being imported or exported from a world, indicating that
    /// it's a namespace of functions.
    Interface {
        #[cfg_attr(feature = "serde", serde(serialize_with = "serialize_id"))]
        id: InterfaceId,
        #[cfg_attr(
            feature = "serde",
            serde(skip_serializing_if = "Stability::is_unknown")
        )]
        stability: Stability,
    },

    /// A function is being directly imported or exported from this world.
    Function(Function),

    /// A type is being exported from this world.
    ///
    /// Note that types are never imported into worlds at this time.
    #[cfg_attr(feature = "serde", serde(serialize_with = "serialize_id"))]
    Type(TypeId),
}

impl WorldItem {
    pub fn stability<'a>(&'a self, resolve: &'a Resolve) -> &'a Stability {
        match self {
            WorldItem::Interface { stability, .. } => stability,
            WorldItem::Function(f) => &f.stability,
            WorldItem::Type(id) => &resolve.types[*id].stability,
        }
    }
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

    /// Stability attribute for this interface.
    #[cfg_attr(
        feature = "serde",
        serde(skip_serializing_if = "Stability::is_unknown")
    )]
    pub stability: Stability,

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
    /// Stability attribute for this type.
    #[cfg_attr(
        feature = "serde",
        serde(skip_serializing_if = "Stability::is_unknown")
    )]
    pub stability: Stability,
}

#[derive(Debug, Clone, PartialEq, Hash, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "kebab-case"))]
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
    Map(Type, Type),
    FixedSizeList(Type, u32),
    Future(Option<Type>),
    Stream(Option<Type>),
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
            TypeDefKind::Map(_, _) => "map",
            TypeDefKind::FixedSizeList(..) => "fixed size list",
            TypeDefKind::Future(_) => "future",
            TypeDefKind::Stream(_) => "stream",
            TypeDefKind::Type(_) => "type",
            TypeDefKind::Unknown => "unknown",
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "kebab-case"))]
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
#[cfg_attr(feature = "serde", serde(rename_all = "kebab-case"))]
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
    F32,
    F64,
    Char,
    String,
    ErrorContext,
    Id(TypeId),
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Int {
    U8,
    U16,
    U32,
    U64,
}

#[derive(Debug, Clone, PartialEq, Hash, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct Record {
    pub fields: Vec<Field>,
}

#[derive(Debug, Clone, PartialEq, Hash, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct Field {
    pub name: String,
    #[cfg_attr(feature = "serde", serde(rename = "type"))]
    pub ty: Type,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Docs::is_empty"))]
    pub docs: Docs,
}

#[derive(Debug, Clone, PartialEq, Hash, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct Flags {
    pub flags: Vec<Flag>,
}

#[derive(Debug, Clone, PartialEq, Hash, Eq)]
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

#[derive(Debug, Clone, PartialEq, Hash, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct Tuple {
    pub types: Vec<Type>,
}

#[derive(Debug, Clone, PartialEq, Hash, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct Variant {
    pub cases: Vec<Case>,
}

#[derive(Debug, Clone, PartialEq, Hash, Eq)]
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
        discriminant_type(self.cases.len())
    }
}

#[derive(Debug, Clone, PartialEq, Hash, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct Enum {
    pub cases: Vec<EnumCase>,
}

#[derive(Debug, Clone, PartialEq, Hash, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct EnumCase {
    pub name: String,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Docs::is_empty"))]
    pub docs: Docs,
}

impl Enum {
    pub fn tag(&self) -> Int {
        discriminant_type(self.cases.len())
    }
}

/// This corresponds to the `discriminant_type` function in the Canonical ABI.
fn discriminant_type(num_cases: usize) -> Int {
    match num_cases.checked_sub(1) {
        None => Int::U8,
        Some(n) if n <= u8::max_value() as usize => Int::U8,
        Some(n) if n <= u16::max_value() as usize => Int::U16,
        Some(n) if n <= u32::max_value() as usize => Int::U32,
        _ => panic!("too many cases to fit in a repr"),
    }
}

#[derive(Debug, Clone, PartialEq, Hash, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct Result_ {
    pub ok: Option<Type>,
    pub err: Option<Type>,
}

#[derive(Clone, Default, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct Docs {
    pub contents: Option<String>,
}

impl Docs {
    pub fn is_empty(&self) -> bool {
        self.contents.is_none()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct Function {
    pub name: String,
    pub kind: FunctionKind,
    #[cfg_attr(feature = "serde", serde(serialize_with = "serialize_params"))]
    pub params: Vec<(String, Type)>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub result: Option<Type>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Docs::is_empty"))]
    pub docs: Docs,
    /// Stability attribute for this function.
    #[cfg_attr(
        feature = "serde",
        serde(skip_serializing_if = "Stability::is_unknown")
    )]
    pub stability: Stability,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "kebab-case"))]
pub enum FunctionKind {
    /// A freestanding function.
    ///
    /// ```wit
    /// interface foo {
    ///     the-func: func();
    /// }
    /// ```
    Freestanding,

    /// An async freestanding function.
    ///
    /// ```wit
    /// interface foo {
    ///     the-func: async func();
    /// }
    /// ```
    AsyncFreestanding,

    /// A resource method where the first parameter is implicitly
    /// `borrow<T>`.
    ///
    /// ```wit
    /// interface foo {
    ///     resource r {
    ///         the-func: func();
    ///     }
    /// }
    /// ```
    #[cfg_attr(feature = "serde", serde(serialize_with = "serialize_id"))]
    Method(TypeId),

    /// An async resource method where the first parameter is implicitly
    /// `borrow<T>`.
    ///
    /// ```wit
    /// interface foo {
    ///     resource r {
    ///         the-func: async func();
    ///     }
    /// }
    /// ```
    #[cfg_attr(feature = "serde", serde(serialize_with = "serialize_id"))]
    AsyncMethod(TypeId),

    /// A static resource method.
    ///
    /// ```wit
    /// interface foo {
    ///     resource r {
    ///         the-func: static func();
    ///     }
    /// }
    /// ```
    #[cfg_attr(feature = "serde", serde(serialize_with = "serialize_id"))]
    Static(TypeId),

    /// An async static resource method.
    ///
    /// ```wit
    /// interface foo {
    ///     resource r {
    ///         the-func: static async func();
    ///     }
    /// }
    /// ```
    #[cfg_attr(feature = "serde", serde(serialize_with = "serialize_id"))]
    AsyncStatic(TypeId),

    /// A resource constructor where the return value is implicitly `own<T>`.
    ///
    /// ```wit
    /// interface foo {
    ///     resource r {
    ///         constructor();
    ///     }
    /// }
    /// ```
    #[cfg_attr(feature = "serde", serde(serialize_with = "serialize_id"))]
    Constructor(TypeId),
}

impl FunctionKind {
    /// Returns whether this is an async function or not.
    pub fn is_async(&self) -> bool {
        match self {
            FunctionKind::Freestanding
            | FunctionKind::Method(_)
            | FunctionKind::Static(_)
            | FunctionKind::Constructor(_) => false,
            FunctionKind::AsyncFreestanding
            | FunctionKind::AsyncMethod(_)
            | FunctionKind::AsyncStatic(_) => true,
        }
    }

    /// Returns the resource, if present, that this function kind refers to.
    pub fn resource(&self) -> Option<TypeId> {
        match self {
            FunctionKind::Freestanding | FunctionKind::AsyncFreestanding => None,
            FunctionKind::Method(id)
            | FunctionKind::Static(id)
            | FunctionKind::Constructor(id)
            | FunctionKind::AsyncMethod(id)
            | FunctionKind::AsyncStatic(id) => Some(*id),
        }
    }

    /// Returns the resource, if present, that this function kind refers to.
    pub fn resource_mut(&mut self) -> Option<&mut TypeId> {
        match self {
            FunctionKind::Freestanding | FunctionKind::AsyncFreestanding => None,
            FunctionKind::Method(id)
            | FunctionKind::Static(id)
            | FunctionKind::Constructor(id)
            | FunctionKind::AsyncMethod(id)
            | FunctionKind::AsyncStatic(id) => Some(id),
        }
    }
}

/// Possible forms of name mangling that are supported by this crate.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Mangling {
    /// The "standard" component model mangling format for 32-bit linear
    /// memories. This is specified in WebAssembly/component-model#378
    Standard32,

    /// The "legacy" name mangling supported in versions 218-and-prior for this
    /// crate. This is the original support for how components were created from
    /// core wasm modules and this does not correspond to any standard. This is
    /// preserved for now while tools transition to the new scheme.
    Legacy,
}

impl std::str::FromStr for Mangling {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Mangling> {
        match s {
            "legacy" => Ok(Mangling::Legacy),
            "standard32" => Ok(Mangling::Standard32),
            _ => {
                bail!(
                    "unknown name mangling `{s}`, \
                     supported values are `legacy` or `standard32`"
                )
            }
        }
    }
}

/// Possible lift/lower ABI choices supported when mangling names.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum LiftLowerAbi {
    /// Both imports and exports will use the synchronous ABI.
    Sync,

    /// Both imports and exports will use the async ABI (with a callback for
    /// each export).
    AsyncCallback,

    /// Both imports and exports will use the async ABI (with no callbacks for
    /// exports).
    AsyncStackful,
}

impl LiftLowerAbi {
    fn import_prefix(self) -> &'static str {
        match self {
            Self::Sync => "",
            Self::AsyncCallback | Self::AsyncStackful => "[async-lower]",
        }
    }

    /// Get the import [`AbiVariant`] corresponding to this [`LiftLowerAbi`]
    pub fn import_variant(self) -> AbiVariant {
        match self {
            Self::Sync => AbiVariant::GuestImport,
            Self::AsyncCallback | Self::AsyncStackful => AbiVariant::GuestImportAsync,
        }
    }

    fn export_prefix(self) -> &'static str {
        match self {
            Self::Sync => "",
            Self::AsyncCallback => "[async-lift]",
            Self::AsyncStackful => "[async-lift-stackful]",
        }
    }

    /// Get the export [`AbiVariant`] corresponding to this [`LiftLowerAbi`]
    pub fn export_variant(self) -> AbiVariant {
        match self {
            Self::Sync => AbiVariant::GuestExport,
            Self::AsyncCallback => AbiVariant::GuestExportAsync,
            Self::AsyncStackful => AbiVariant::GuestExportAsyncStackful,
        }
    }
}

/// Combination of [`Mangling`] and [`LiftLowerAbi`].
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ManglingAndAbi {
    /// See [`Mangling::Standard32`].
    ///
    /// As of this writing, the standard name mangling only supports the
    /// synchronous ABI.
    Standard32,

    /// See [`Mangling::Legacy`] and [`LiftLowerAbi`].
    Legacy(LiftLowerAbi),
}

impl ManglingAndAbi {
    /// Get the import [`AbiVariant`] corresponding to this [`ManglingAndAbi`]
    pub fn import_variant(self) -> AbiVariant {
        match self {
            Self::Standard32 => AbiVariant::GuestImport,
            Self::Legacy(abi) => abi.import_variant(),
        }
    }

    /// Get the export [`AbiVariant`] corresponding to this [`ManglingAndAbi`]
    pub fn export_variant(self) -> AbiVariant {
        match self {
            Self::Standard32 => AbiVariant::GuestExport,
            Self::Legacy(abi) => abi.export_variant(),
        }
    }

    /// Switch the ABI to be sync if it's async.
    pub fn sync(self) -> Self {
        match self {
            Self::Standard32 | Self::Legacy(LiftLowerAbi::Sync) => self,
            Self::Legacy(LiftLowerAbi::AsyncCallback)
            | Self::Legacy(LiftLowerAbi::AsyncStackful) => Self::Legacy(LiftLowerAbi::Sync),
        }
    }

    /// Returns whether this is an async ABI
    pub fn is_async(&self) -> bool {
        match self {
            Self::Standard32 | Self::Legacy(LiftLowerAbi::Sync) => false,
            Self::Legacy(LiftLowerAbi::AsyncCallback)
            | Self::Legacy(LiftLowerAbi::AsyncStackful) => true,
        }
    }

    pub fn mangling(&self) -> Mangling {
        match self {
            Self::Standard32 => Mangling::Standard32,
            Self::Legacy(_) => Mangling::Legacy,
        }
    }
}

impl Function {
    pub fn item_name(&self) -> &str {
        match &self.kind {
            FunctionKind::Freestanding | FunctionKind::AsyncFreestanding => &self.name,
            FunctionKind::Method(_)
            | FunctionKind::Static(_)
            | FunctionKind::AsyncMethod(_)
            | FunctionKind::AsyncStatic(_) => &self.name[self.name.find('.').unwrap() + 1..],
            FunctionKind::Constructor(_) => "constructor",
        }
    }

    /// Returns an iterator over the types used in parameters and results.
    ///
    /// Note that this iterator is not transitive, it only iterates over the
    /// direct references to types that this function has.
    pub fn parameter_and_result_types(&self) -> impl Iterator<Item = Type> + '_ {
        self.params.iter().map(|(_, t)| *t).chain(self.result)
    }

    /// Gets the core export name for this function.
    pub fn standard32_core_export_name<'a>(&'a self, interface: Option<&str>) -> Cow<'a, str> {
        self.core_export_name(interface, Mangling::Standard32)
    }

    pub fn legacy_core_export_name<'a>(&'a self, interface: Option<&str>) -> Cow<'a, str> {
        self.core_export_name(interface, Mangling::Legacy)
    }
    /// Gets the core export name for this function.
    pub fn core_export_name<'a>(
        &'a self,
        interface: Option<&str>,
        mangling: Mangling,
    ) -> Cow<'a, str> {
        match interface {
            Some(interface) => match mangling {
                Mangling::Standard32 => Cow::Owned(format!("cm32p2|{interface}|{}", self.name)),
                Mangling::Legacy => Cow::Owned(format!("{interface}#{}", self.name)),
            },
            None => match mangling {
                Mangling::Standard32 => Cow::Owned(format!("cm32p2||{}", self.name)),
                Mangling::Legacy => Cow::Borrowed(&self.name),
            },
        }
    }
    /// Collect any future and stream types appearing in the signature of this
    /// function by doing a depth-first search over the parameter types and then
    /// the result types.
    ///
    /// For example, given the WIT function `foo: func(x: future<future<u32>>,
    /// y: u32) -> stream<u8>`, we would return `[future<u32>,
    /// future<future<u32>>, stream<u8>]`.
    ///
    /// This may be used by binding generators to refer to specific `future` and
    /// `stream` types when importing canonical built-ins such as `stream.new`,
    /// `future.read`, etc.  Using the example above, the import
    /// `[future-new-0]foo` would indicate a call to `future.new` for the type
    /// `future<u32>`.  Likewise, `[future-new-1]foo` would indicate a call to
    /// `future.new` for `future<future<u32>>`, and `[stream-new-2]foo` would
    /// indicate a call to `stream.new` for `stream<u8>`.
    pub fn find_futures_and_streams(&self, resolve: &Resolve) -> Vec<TypeId> {
        let mut results = Vec::new();
        for (_, ty) in self.params.iter() {
            find_futures_and_streams(resolve, *ty, &mut results);
        }
        if let Some(ty) = self.result {
            find_futures_and_streams(resolve, ty, &mut results);
        }
        results
    }

    /// Check if this function is a resource constructor in shorthand form.
    /// I.e. without an explicit return type annotation.
    pub fn is_constructor_shorthand(&self, resolve: &Resolve) -> bool {
        let FunctionKind::Constructor(containing_resource_id) = self.kind else {
            return false;
        };

        let Some(Type::Id(id)) = &self.result else {
            return false;
        };

        let TypeDefKind::Handle(Handle::Own(returned_resource_id)) = resolve.types[*id].kind else {
            return false;
        };

        return containing_resource_id == returned_resource_id;
    }

    /// Returns the `module`, `name`, and signature to use when importing this
    /// function's `task.return` intrinsic using the `mangling` specified.
    pub fn task_return_import(
        &self,
        resolve: &Resolve,
        interface: Option<&WorldKey>,
        mangling: Mangling,
    ) -> (String, String, abi::WasmSignature) {
        match mangling {
            Mangling::Standard32 => todo!(),
            Mangling::Legacy => {}
        }
        // For exported async functions, generate a `task.return` intrinsic.
        let module = match interface {
            Some(key) => format!("[export]{}", resolve.name_world_key(key)),
            None => "[export]$root".to_string(),
        };
        let name = format!("[task-return]{}", self.name);

        let mut func_tmp = self.clone();
        func_tmp.params = Vec::new();
        func_tmp.result = None;
        if let Some(ty) = self.result {
            func_tmp.params.push(("x".to_string(), ty));
        }
        let sig = resolve.wasm_signature(AbiVariant::GuestImport, &func_tmp);
        (module, name, sig)
    }

    // push_imported_future_and_stream_intrinsics(wat, resolve, "[export]", interface, func);
}

fn find_futures_and_streams(resolve: &Resolve, ty: Type, results: &mut Vec<TypeId>) {
    let Type::Id(id) = ty else {
        return;
    };

    match &resolve.types[id].kind {
        TypeDefKind::Resource
        | TypeDefKind::Handle(_)
        | TypeDefKind::Flags(_)
        | TypeDefKind::Enum(_) => {}
        TypeDefKind::Record(r) => {
            for Field { ty, .. } in &r.fields {
                find_futures_and_streams(resolve, *ty, results);
            }
        }
        TypeDefKind::Tuple(t) => {
            for ty in &t.types {
                find_futures_and_streams(resolve, *ty, results);
            }
        }
        TypeDefKind::Variant(v) => {
            for Case { ty, .. } in &v.cases {
                if let Some(ty) = ty {
                    find_futures_and_streams(resolve, *ty, results);
                }
            }
        }
        TypeDefKind::Option(ty)
        | TypeDefKind::List(ty)
        | TypeDefKind::FixedSizeList(ty, ..)
        | TypeDefKind::Type(ty) => {
            find_futures_and_streams(resolve, *ty, results);
        }
        TypeDefKind::Map(k, v) => {
            find_futures_and_streams(resolve, *k, results);
            find_futures_and_streams(resolve, *v, results);
        }
        TypeDefKind::Result(r) => {
            if let Some(ty) = r.ok {
                find_futures_and_streams(resolve, ty, results);
            }
            if let Some(ty) = r.err {
                find_futures_and_streams(resolve, ty, results);
            }
        }
        TypeDefKind::Future(ty) => {
            if let Some(ty) = ty {
                find_futures_and_streams(resolve, *ty, results);
            }
            results.push(id);
        }
        TypeDefKind::Stream(ty) => {
            if let Some(ty) = ty {
                find_futures_and_streams(resolve, *ty, results);
            }
            results.push(id);
        }
        TypeDefKind::Unknown => unreachable!(),
    }
}

/// Representation of the stability attributes associated with a world,
/// interface, function, or type.
///
/// This is added for WebAssembly/component-model#332 where @since and @unstable
/// annotations were added to WIT.
///
/// The order of the of enum values is significant since it is used with Ord and PartialOrd
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde_derive::Deserialize, Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "kebab-case"))]
pub enum Stability {
    /// This item does not have either `@since` or `@unstable`.
    Unknown,

    /// `@unstable(feature = foo)`
    ///
    /// This item is explicitly tagged `@unstable`. A feature name is listed and
    /// this item is excluded by default in `Resolve` unless explicitly enabled.
    Unstable {
        feature: String,
        #[cfg_attr(
            feature = "serde",
            serde(
                skip_serializing_if = "Option::is_none",
                default,
                serialize_with = "serialize_optional_version",
                deserialize_with = "deserialize_optional_version"
            )
        )]
        deprecated: Option<Version>,
    },

    /// `@since(version = 1.2.3)`
    ///
    /// This item is explicitly tagged with `@since` as stable since the
    /// specified version.  This may optionally have a feature listed as well.
    Stable {
        #[cfg_attr(feature = "serde", serde(serialize_with = "serialize_version"))]
        #[cfg_attr(feature = "serde", serde(deserialize_with = "deserialize_version"))]
        since: Version,
        #[cfg_attr(
            feature = "serde",
            serde(
                skip_serializing_if = "Option::is_none",
                default,
                serialize_with = "serialize_optional_version",
                deserialize_with = "deserialize_optional_version"
            )
        )]
        deprecated: Option<Version>,
    },
}

impl Stability {
    /// Returns whether this is `Stability::Unknown`.
    pub fn is_unknown(&self) -> bool {
        matches!(self, Stability::Unknown)
    }

    pub fn is_stable(&self) -> bool {
        matches!(self, Stability::Stable { .. })
    }
}

impl Default for Stability {
    fn default() -> Stability {
        Stability::Unknown
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_discriminant_type() {
        assert_eq!(discriminant_type(1), Int::U8);
        assert_eq!(discriminant_type(0x100), Int::U8);
        assert_eq!(discriminant_type(0x101), Int::U16);
        assert_eq!(discriminant_type(0x10000), Int::U16);
        assert_eq!(discriminant_type(0x10001), Int::U32);
        if let Ok(num_cases) = usize::try_from(0x100000000_u64) {
            assert_eq!(discriminant_type(num_cases), Int::U32);
        }
    }

    #[test]
    fn test_find_futures_and_streams() {
        let mut resolve = Resolve::default();
        let t0 = resolve.types.alloc(TypeDef {
            name: None,
            kind: TypeDefKind::Future(Some(Type::U32)),
            owner: TypeOwner::None,
            docs: Docs::default(),
            stability: Stability::Unknown,
        });
        let t1 = resolve.types.alloc(TypeDef {
            name: None,
            kind: TypeDefKind::Future(Some(Type::Id(t0))),
            owner: TypeOwner::None,
            docs: Docs::default(),
            stability: Stability::Unknown,
        });
        let t2 = resolve.types.alloc(TypeDef {
            name: None,
            kind: TypeDefKind::Stream(Some(Type::U32)),
            owner: TypeOwner::None,
            docs: Docs::default(),
            stability: Stability::Unknown,
        });
        let found = Function {
            name: "foo".into(),
            kind: FunctionKind::Freestanding,
            params: vec![("p1".into(), Type::Id(t1)), ("p2".into(), Type::U32)],
            result: Some(Type::Id(t2)),
            docs: Docs::default(),
            stability: Stability::Unknown,
        }
        .find_futures_and_streams(&resolve);
        assert_eq!(3, found.len());
        assert_eq!(t0, found[0]);
        assert_eq!(t1, found[1]);
        assert_eq!(t2, found[2]);
    }
}
