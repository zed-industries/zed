use crate::Abi;
use std::collections::{HashMap, HashSet};
use std::rc::{Rc, Weak};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Id(String);

impl Id {
    pub fn new<S: AsRef<str>>(s: S) -> Self {
        Id(s.as_ref().to_string())
    }
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl AsRef<str> for Id {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

impl PartialEq<&str> for Id {
    fn eq(&self, rhs: &&str) -> bool {
        PartialEq::eq(self.as_ref(), *rhs)
    }
}

impl PartialEq<Id> for &str {
    fn eq(&self, rhs: &Id) -> bool {
        PartialEq::eq(*self, rhs.as_ref())
    }
}

impl From<&str> for Id {
    fn from(s: &str) -> Self {
        Self::new(s)
    }
}

#[derive(Debug, Clone)]
pub struct Document {
    definitions: Vec<Definition>,
    entries: HashMap<Id, Entry>,
}

impl Document {
    pub(crate) fn new(definitions: Vec<Definition>, entries: HashMap<Id, Entry>) -> Self {
        Document {
            definitions,
            entries,
        }
    }
    pub fn typename(&self, name: &Id) -> Option<Rc<NamedType>> {
        self.entries.get(name).and_then(|e| match e {
            Entry::Typename(nt) => Some(nt.upgrade().expect("always possible to upgrade entry")),
            _ => None,
        })
    }
    pub fn typenames<'a>(&'a self) -> impl Iterator<Item = Rc<NamedType>> + 'a {
        self.definitions.iter().filter_map(|d| match d {
            Definition::Typename(nt) => Some(nt.clone()),
            _ => None,
        })
    }
    /// All of the (unique) types used as "err" variant of results returned from
    /// functions.
    pub fn error_types<'a>(&'a self) -> impl Iterator<Item = TypeRef> + 'a {
        let errors: HashSet<TypeRef> = self
            .modules()
            .flat_map(|m| {
                m.funcs()
                    .filter_map(|f| {
                        if f.results.len() == 1 {
                            Some(f.results[0].tref.type_().clone())
                        } else {
                            None
                        }
                    })
                    .filter_map(|t| match &*t {
                        Type::Variant(v) => {
                            let (_ok, err) = v.as_expected()?;
                            Some(err?.clone())
                        }
                        _ => None,
                    })
                    .collect::<HashSet<TypeRef>>()
            })
            .collect();
        errors.into_iter()
    }
    pub fn module(&self, name: &Id) -> Option<Rc<Module>> {
        self.entries.get(&name).and_then(|e| match e {
            Entry::Module(m) => Some(m.upgrade().expect("always possible to upgrade entry")),
            _ => None,
        })
    }
    pub fn modules<'a>(&'a self) -> impl Iterator<Item = Rc<Module>> + 'a {
        self.definitions.iter().filter_map(|d| match d {
            Definition::Module(m) => Some(m.clone()),
            _ => None,
        })
    }

    pub fn constants<'a>(&'a self) -> impl Iterator<Item = &'a Constant> + 'a {
        self.definitions.iter().filter_map(|d| match d {
            Definition::Constant(c) => Some(c),
            _ => None,
        })
    }
}

impl PartialEq for Document {
    fn eq(&self, rhs: &Document) -> bool {
        // For equality, we don't care about the ordering of definitions,
        // so we only need to check that the entries map is equal
        self.entries == rhs.entries
    }
}
impl Eq for Document {}

impl std::hash::Hash for Document {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        std::hash::Hash::hash(&self.definitions, state);
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Definition {
    Typename(Rc<NamedType>),
    Module(Rc<Module>),
    Constant(Constant),
}

#[derive(Debug, Clone)]
pub enum Entry {
    Typename(Weak<NamedType>),
    Module(Weak<Module>),
}

impl Entry {
    pub fn kind(&self) -> &'static str {
        match self {
            Entry::Typename { .. } => "typename",
            Entry::Module { .. } => "module",
        }
    }
}

impl PartialEq for Entry {
    fn eq(&self, rhs: &Entry) -> bool {
        match (self, rhs) {
            (Entry::Typename(t), Entry::Typename(t_rhs)) => {
                t.upgrade()
                    .expect("possible to upgrade entry when part of document")
                    == t_rhs
                        .upgrade()
                        .expect("possible to upgrade entry when part of document")
            }
            (Entry::Module(m), Entry::Module(m_rhs)) => {
                m.upgrade()
                    .expect("possible to upgrade entry when part of document")
                    == m_rhs
                        .upgrade()
                        .expect("possible to upgrade entry when part of document")
            }
            _ => false,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TypeRef {
    Name(Rc<NamedType>),
    Value(Rc<Type>),
}

impl TypeRef {
    pub fn type_(&self) -> &Rc<Type> {
        match self {
            TypeRef::Name(named) => named.type_(),
            TypeRef::Value(v) => v,
        }
    }

    pub fn named(&self) -> bool {
        match self {
            TypeRef::Name(_) => true,
            TypeRef::Value(_) => false,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NamedType {
    pub name: Id,
    pub tref: TypeRef,
    pub docs: String,
}

impl NamedType {
    pub fn type_(&self) -> &Rc<Type> {
        self.tref.type_()
    }
}

/// Structure of all possible interface types.
///
/// Note that this is intended to match the interface types proposal itself.
/// Currently this is relatively close to that with just a few `*.witx`
/// extensions for now.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Type {
    /// A structure with named field.
    Record(RecordDatatype),
    /// An enumeration where a value is one of a number of variants.
    Variant(Variant),
    /// A "handle" which is an un-forgeable reference. Today this is an `i32`
    /// where a module can't forge and use integers it was not already given
    /// access to.
    Handle(HandleDatatype),
    /// A list of a type, stored in linear memory.
    ///
    /// Note that lists of `char` are specialized to indicate strings.
    List(TypeRef),
    /// A `witx`-specific type representing a raw mutable pointer into linear
    /// memory
    Pointer(TypeRef),
    /// A `witx`-specific type representing a raw const pointer into linear
    /// memory
    ConstPointer(TypeRef),
    /// A builtin base-case type.
    Builtin(BuiltinType),
}

impl Type {
    /// Returns a human-readable string to describe this type.
    pub fn kind(&self) -> &'static str {
        use Type::*;
        match self {
            Record(_) => "record",
            Variant(_) => "variant",
            Handle(_) => "handle",
            List(_) => "list",
            Pointer(_) => "pointer",
            ConstPointer(_) => "constpointer",
            Builtin(_) => "builtin",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BuiltinType {
    /// This is a 32-bit unicode scalar value, not a code point.
    ///
    /// Same as the Rust language's `char` type.
    Char,
    /// An 8-bit unsigned integer.
    U8 {
        /// Indicates whether this type is intended to represent the `char`
        /// type in the C language. The C `char` type is often unsigned, but
        /// it's language-specific. At an interface-types level this is an
        /// unsigned byte but binding generators may wish to bind this as the
        /// language-specific representation for a C character instead.
        ///
        /// This is also currently used exclusively in conjunction with `@witx
        /// pointer` to hint that it's pointing to unicode string data as well.
        lang_c_char: bool,
    },
    /// A 16-bit unsigned integer.
    U16,
    /// A 32-bit unsigned integer.
    U32 {
        /// Indicates that this 32-bit value should actually be considered a
        /// pointer-like value in language bindings. At the interface types
        /// layer this is always a 32-bit unsigned value, but binding
        /// generators may wish to instead bind this as the equivalent of C's
        /// `size_t` for convenience with other APIs.
        ///
        /// This allows witx authors to communicate the intent that the
        /// argument or return-value is pointer-like.
        lang_ptr_size: bool,
    },
    /// A 64-bit unsigned integer.
    U64,
    /// An 8-bit signed integer
    S8,
    /// A 16-bit signed integer
    S16,
    /// A 32-bit signed integer
    S32,
    /// A 64-bit signed integer
    S64,
    /// A 32-bit floating point value.
    F32,
    /// A 64-bit floating point value.
    F64,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum IntRepr {
    U8,
    U16,
    U32,
    U64,
}

impl IntRepr {
    pub fn to_builtin(&self) -> BuiltinType {
        match self {
            IntRepr::U8 => BuiltinType::U8 { lang_c_char: false },
            IntRepr::U16 => BuiltinType::U16,
            IntRepr::U32 => BuiltinType::U32 {
                lang_ptr_size: false,
            },
            IntRepr::U64 => BuiltinType::U64,
        }
    }
}

/// A struct-like value with named fields.
///
/// Records map to `struct`s in most languages where this is a type with a
/// number of named fields that all have their own particular type. Field order
/// dictates layout in memory.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RecordDatatype {
    /// A hint as to what this record might be.
    ///
    /// Note that in the future this will only be a hint, not a control of the
    /// actual representation itself. At this time though the record layout of
    /// bitflags is different from other types.
    pub kind: RecordKind,

    /// A list of named fields for this record.
    pub members: Vec<RecordMember>,
}

/// Different kinds of records used for hinting various language-specific types.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RecordKind {
    /// A tuple where the name of all fields are consecutive integers starting
    /// at "0".
    Tuple,
    /// A record where all fields are `bool`s. Currently represented as an
    /// integer with bits set or not set.
    Bitflags(IntRepr),
    /// All other structures.
    Other,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RecordMember {
    pub name: Id,
    pub tref: TypeRef,
    pub docs: String,
}

impl RecordDatatype {
    pub fn is_tuple(&self) -> bool {
        match self.kind {
            RecordKind::Tuple => true,
            _ => false,
        }
    }

    pub fn bitflags_repr(&self) -> Option<IntRepr> {
        match self.kind {
            RecordKind::Bitflags(i) => Some(i),
            _ => None,
        }
    }
}

/// A type which represents how values can be one of a set of possible cases.
///
/// This type maps to an `enum` in languages like Rust, but doesn't have an
/// equivalent in languages like JS or C. The closest analog in C is a tagged
/// union, but a `Variant` is always consistent whereas a tagged union in C
/// could be mis-tagged or such.
///
/// Variants are used to represent one of a possible set of types. For example
/// an enum-like variant, a result that is either success or failure, or even a
/// simple `bool`. Variants are primarily used heavily with various kinds of
/// shorthands in the `*.witx` format to represent idioms in languages.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Variant {
    /// The bit representation of the width of this variant's tag when the
    /// variant is stored in memory.
    pub tag_repr: IntRepr,
    /// The possible cases that values of this variant type can take.
    pub cases: Vec<Case>,
}

impl Variant {
    /// If this variant looks like an `expected` shorthand, return the ok/err
    /// types associated with this result.
    ///
    /// Only matches variants fo the form:
    ///
    /// ```text
    /// (variant
    ///     (case "ok" ok?)
    ///     (case "err" err?))
    /// ```
    pub fn as_expected(&self) -> Option<(Option<&TypeRef>, Option<&TypeRef>)> {
        if self.cases.len() != 2 {
            return None;
        }
        if self.cases[0].name != "ok" {
            return None;
        }
        if self.cases[1].name != "err" {
            return None;
        }
        Some((self.cases[0].tref.as_ref(), self.cases[1].tref.as_ref()))
    }

    /// Returns whether this variant type is "bool-like" meaning that it matches
    /// this type:
    ///
    /// ```text
    /// (variant
    ///     (case "false")
    ///     (case "true"))
    /// ```
    pub fn is_bool(&self) -> bool {
        self.cases.len() == 2
            && self.cases[0].name == "false"
            && self.cases[1].name == "true"
            && self.cases[0].tref.is_none()
            && self.cases[1].tref.is_none()
    }

    /// Returns whether this variant type is "enum-like" meaning that all of its
    /// cases have no payload associated with them.
    pub fn is_enum(&self) -> bool {
        self.cases.iter().all(|c| c.tref.is_none())
    }
}

/// One of a number of possible types that a `Variant` can take.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Case {
    /// The name of this case and how to identify it.
    pub name: Id,
    /// An optional payload type for this case and data that can be associated
    /// with it.
    pub tref: Option<TypeRef>,
    /// Documentation for this case.
    pub docs: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct HandleDatatype {}

#[derive(Debug, Clone)]
pub struct Module {
    pub name: Id,
    definitions: Vec<ModuleDefinition>,
    entries: HashMap<Id, ModuleEntry>,
    pub docs: String,
}

impl Module {
    pub(crate) fn new(
        name: Id,
        definitions: Vec<ModuleDefinition>,
        entries: HashMap<Id, ModuleEntry>,
        docs: String,
    ) -> Self {
        Module {
            name,
            definitions,
            entries,
            docs,
        }
    }
    pub fn import(&self, name: &Id) -> Option<Rc<ModuleImport>> {
        self.entries.get(name).and_then(|e| match e {
            ModuleEntry::Import(d) => Some(d.upgrade().expect("always possible to upgrade entry")),
            _ => None,
        })
    }
    pub fn imports<'a>(&'a self) -> impl Iterator<Item = Rc<ModuleImport>> + 'a {
        self.definitions.iter().filter_map(|d| match d {
            ModuleDefinition::Import(d) => Some(d.clone()),
            _ => None,
        })
    }
    pub fn func(&self, name: &Id) -> Option<Rc<InterfaceFunc>> {
        self.entries.get(name).and_then(|e| match e {
            ModuleEntry::Func(d) => Some(d.upgrade().expect("always possible to upgrade entry")),
            _ => None,
        })
    }
    pub fn funcs<'a>(&'a self) -> impl Iterator<Item = Rc<InterfaceFunc>> + 'a {
        self.definitions.iter().filter_map(|d| match d {
            ModuleDefinition::Func(d) => Some(d.clone()),
            _ => None,
        })
    }
}

impl PartialEq for Module {
    fn eq(&self, rhs: &Module) -> bool {
        // For equality, we don't care about the ordering of definitions,
        // so we only need to check that the entries map is equal
        self.name == rhs.name && self.entries == rhs.entries && self.docs == rhs.docs
    }
}
impl Eq for Module {}

impl std::hash::Hash for Module {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        std::hash::Hash::hash(&self.name, state);
        std::hash::Hash::hash(&self.definitions, state);
        std::hash::Hash::hash(&self.docs, state);
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ModuleDefinition {
    Import(Rc<ModuleImport>),
    Func(Rc<InterfaceFunc>),
}

#[derive(Debug, Clone)]
pub enum ModuleEntry {
    Import(Weak<ModuleImport>),
    Func(Weak<InterfaceFunc>),
}

impl PartialEq for ModuleEntry {
    fn eq(&self, rhs: &ModuleEntry) -> bool {
        match (self, rhs) {
            (ModuleEntry::Import(i), ModuleEntry::Import(i_rhs)) => {
                i.upgrade()
                    .expect("always possible to upgrade moduleentry when part of module")
                    == i_rhs
                        .upgrade()
                        .expect("always possible to upgrade moduleentry when part of module")
            }
            (ModuleEntry::Func(i), ModuleEntry::Func(i_rhs)) => {
                i.upgrade()
                    .expect("always possible to upgrade moduleentry when part of module")
                    == i_rhs
                        .upgrade()
                        .expect("always possible to upgrade moduleentry when part of module")
            }
            _ => false,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ModuleImport {
    pub name: Id,
    pub variant: ModuleImportVariant,
    pub docs: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ModuleImportVariant {
    Memory,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct InterfaceFunc {
    pub abi: Abi,
    pub name: Id,
    pub params: Vec<InterfaceFuncParam>,
    pub results: Vec<InterfaceFuncParam>,
    pub noreturn: bool,
    pub docs: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct InterfaceFuncParam {
    pub name: Id,
    pub tref: TypeRef,
    pub docs: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Constant {
    pub ty: Id,
    pub name: Id,
    pub value: u64,
    pub docs: String,
}
