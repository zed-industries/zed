use crate::ast::{self, kw};
use crate::parser::{Cursor, Parse, Parser, Peek, Result};
use std::mem;

/// The value types for a wasm module.
#[allow(missing_docs)]
#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub enum ValType<'a> {
    I32,
    I64,
    F32,
    F64,
    V128,
    Ref(RefType<'a>),
    Rtt(Option<u32>, ast::Index<'a>),
}

impl<'a> Parse<'a> for ValType<'a> {
    fn parse(parser: Parser<'a>) -> Result<Self> {
        let mut l = parser.lookahead1();
        if l.peek::<kw::i32>() {
            parser.parse::<kw::i32>()?;
            Ok(ValType::I32)
        } else if l.peek::<kw::i64>() {
            parser.parse::<kw::i64>()?;
            Ok(ValType::I64)
        } else if l.peek::<kw::f32>() {
            parser.parse::<kw::f32>()?;
            Ok(ValType::F32)
        } else if l.peek::<kw::f64>() {
            parser.parse::<kw::f64>()?;
            Ok(ValType::F64)
        } else if l.peek::<kw::v128>() {
            parser.parse::<kw::v128>()?;
            Ok(ValType::V128)
        } else if l.peek::<RefType>() {
            Ok(ValType::Ref(parser.parse()?))
        } else if l.peek::<ast::LParen>() {
            parser.parens(|p| {
                let mut l = p.lookahead1();
                if l.peek::<kw::rtt>() {
                    p.parse::<kw::rtt>()?;
                    Ok(ValType::Rtt(p.parse()?, p.parse()?))
                } else {
                    Err(l.error())
                }
            })
        } else {
            Err(l.error())
        }
    }
}

impl<'a> Peek for ValType<'a> {
    fn peek(cursor: Cursor<'_>) -> bool {
        kw::i32::peek(cursor)
            || kw::i64::peek(cursor)
            || kw::f32::peek(cursor)
            || kw::f64::peek(cursor)
            || kw::v128::peek(cursor)
            || (ast::LParen::peek(cursor) && kw::rtt::peek2(cursor))
            || RefType::peek(cursor)
    }
    fn display() -> &'static str {
        "valtype"
    }
}

/// A heap type for a reference type
#[allow(missing_docs)]
#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub enum HeapType<'a> {
    /// An untyped function reference: funcref. This is part of the reference
    /// types proposal.
    Func,
    /// A reference to any host value: externref. This is part of the reference
    /// types proposal.
    Extern,
    /// A reference to any reference value: anyref. This is part of the GC
    /// proposal.
    Any,
    /// A reference that has an identity that can be compared: eqref. This is
    /// part of the GC proposal.
    Eq,
    /// A reference to a GC object. This is part of the GC proposal.
    Data,
    /// An unboxed 31-bit integer: i31ref. This may be going away if there is no common
    /// supertype of all reference types. Part of the GC proposal.
    I31,
    /// A reference to a function, struct, or array: ref T. This is part of the
    /// GC proposal.
    Index(ast::Index<'a>),
}

impl<'a> Parse<'a> for HeapType<'a> {
    fn parse(parser: Parser<'a>) -> Result<Self> {
        let mut l = parser.lookahead1();
        if l.peek::<kw::func>() {
            parser.parse::<kw::func>()?;
            Ok(HeapType::Func)
        } else if l.peek::<kw::r#extern>() {
            parser.parse::<kw::r#extern>()?;
            Ok(HeapType::Extern)
        } else if l.peek::<kw::r#any>() {
            parser.parse::<kw::r#any>()?;
            Ok(HeapType::Any)
        } else if l.peek::<kw::eq>() {
            parser.parse::<kw::eq>()?;
            Ok(HeapType::Eq)
        } else if l.peek::<kw::data>() {
            parser.parse::<kw::data>()?;
            Ok(HeapType::Data)
        } else if l.peek::<kw::i31>() {
            parser.parse::<kw::i31>()?;
            Ok(HeapType::I31)
        } else if l.peek::<ast::Index>() {
            Ok(HeapType::Index(parser.parse()?))
        } else {
            Err(l.error())
        }
    }
}

impl<'a> Peek for HeapType<'a> {
    fn peek(cursor: Cursor<'_>) -> bool {
        kw::func::peek(cursor)
            || kw::r#extern::peek(cursor)
            || kw::any::peek(cursor)
            || kw::eq::peek(cursor)
            || kw::data::peek(cursor)
            || kw::i31::peek(cursor)
            || (ast::LParen::peek(cursor) && kw::r#type::peek2(cursor))
    }
    fn display() -> &'static str {
        "heaptype"
    }
}

/// A reference type in a wasm module.
#[allow(missing_docs)]
#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub struct RefType<'a> {
    pub nullable: bool,
    pub heap: HeapType<'a>,
}

impl<'a> RefType<'a> {
    /// A `funcref` as an abbreviation for `(ref null func)`.
    pub fn func() -> Self {
        RefType {
            nullable: true,
            heap: HeapType::Func,
        }
    }

    /// An `externref` as an abbreviation for `(ref null extern)`.
    pub fn r#extern() -> Self {
        RefType {
            nullable: true,
            heap: HeapType::Extern,
        }
    }

    /// An `anyref` as an abbreviation for `(ref null any)`.
    pub fn any() -> Self {
        RefType {
            nullable: true,
            heap: HeapType::Any,
        }
    }

    /// An `eqref` as an abbreviation for `(ref null eq)`.
    pub fn eq() -> Self {
        RefType {
            nullable: true,
            heap: HeapType::Eq,
        }
    }

    /// An `dataref` as an abbreviation for `(ref null data)`.
    pub fn data() -> Self {
        RefType {
            nullable: true,
            heap: HeapType::Data,
        }
    }

    /// An `i31ref` as an abbreviation for `(ref null i31)`.
    pub fn i31() -> Self {
        RefType {
            nullable: true,
            heap: HeapType::I31,
        }
    }
}

impl<'a> Parse<'a> for RefType<'a> {
    fn parse(parser: Parser<'a>) -> Result<Self> {
        let mut l = parser.lookahead1();
        if l.peek::<kw::funcref>() {
            parser.parse::<kw::funcref>()?;
            Ok(RefType::func())
        } else if l.peek::<kw::anyfunc>() {
            parser.parse::<kw::anyfunc>()?;
            Ok(RefType::func())
        } else if l.peek::<kw::externref>() {
            parser.parse::<kw::externref>()?;
            Ok(RefType::r#extern())
        } else if l.peek::<kw::anyref>() {
            parser.parse::<kw::anyref>()?;
            Ok(RefType::any())
        } else if l.peek::<kw::eqref>() {
            parser.parse::<kw::eqref>()?;
            Ok(RefType::eq())
        } else if l.peek::<kw::dataref>() {
            parser.parse::<kw::dataref>()?;
            Ok(RefType::data())
        } else if l.peek::<kw::i31ref>() {
            parser.parse::<kw::i31ref>()?;
            Ok(RefType::i31())
        } else if l.peek::<ast::LParen>() {
            parser.parens(|p| {
                let mut l = parser.lookahead1();
                if l.peek::<kw::r#ref>() {
                    p.parse::<kw::r#ref>()?;

                    let mut nullable = false;
                    if parser.peek::<kw::null>() {
                        parser.parse::<kw::null>()?;
                        nullable = true;
                    }

                    Ok(RefType {
                        nullable,
                        heap: parser.parse()?,
                    })
                } else {
                    Err(l.error())
                }
            })
        } else {
            Err(l.error())
        }
    }
}

impl<'a> Peek for RefType<'a> {
    fn peek(cursor: Cursor<'_>) -> bool {
        kw::funcref::peek(cursor)
            || /* legacy */ kw::anyfunc::peek(cursor)
            || kw::externref::peek(cursor)
            || kw::anyref::peek(cursor)
            || kw::eqref::peek(cursor)
            || kw::dataref::peek(cursor)
            || kw::i31ref::peek(cursor)
            || (ast::LParen::peek(cursor) && kw::r#ref::peek2(cursor))
    }
    fn display() -> &'static str {
        "reftype"
    }
}

/// The types of values that may be used in a struct or array.
#[allow(missing_docs)]
#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub enum StorageType<'a> {
    I8,
    I16,
    Val(ValType<'a>),
}

impl<'a> Parse<'a> for StorageType<'a> {
    fn parse(parser: Parser<'a>) -> Result<Self> {
        let mut l = parser.lookahead1();
        if l.peek::<kw::i8>() {
            parser.parse::<kw::i8>()?;
            Ok(StorageType::I8)
        } else if l.peek::<kw::i16>() {
            parser.parse::<kw::i16>()?;
            Ok(StorageType::I16)
        } else if l.peek::<ValType>() {
            Ok(StorageType::Val(parser.parse()?))
        } else {
            Err(l.error())
        }
    }
}

/// Type for a `global` in a wasm module
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct GlobalType<'a> {
    /// The element type of this `global`
    pub ty: ValType<'a>,
    /// Whether or not the global is mutable or not.
    pub mutable: bool,
}

impl<'a> Parse<'a> for GlobalType<'a> {
    fn parse(parser: Parser<'a>) -> Result<Self> {
        if parser.peek2::<kw::r#mut>() {
            parser.parens(|p| {
                p.parse::<kw::r#mut>()?;
                Ok(GlobalType {
                    ty: parser.parse()?,
                    mutable: true,
                })
            })
        } else {
            Ok(GlobalType {
                ty: parser.parse()?,
                mutable: false,
            })
        }
    }
}

/// Min/max limits used for tables/memories.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Limits {
    /// The minimum number of units for this type.
    pub min: u32,
    /// An optional maximum number of units for this type.
    pub max: Option<u32>,
}

impl<'a> Parse<'a> for Limits {
    fn parse(parser: Parser<'a>) -> Result<Self> {
        let min = parser.parse()?;
        let max = if parser.peek::<u32>() {
            Some(parser.parse()?)
        } else {
            None
        };
        Ok(Limits { min, max })
    }
}

/// Min/max limits used for 64-bit memories
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Limits64 {
    /// The minimum number of units for this type.
    pub min: u64,
    /// An optional maximum number of units for this type.
    pub max: Option<u64>,
}

impl<'a> Parse<'a> for Limits64 {
    fn parse(parser: Parser<'a>) -> Result<Self> {
        let min = parser.parse()?;
        let max = if parser.peek::<u64>() {
            Some(parser.parse()?)
        } else {
            None
        };
        Ok(Limits64 { min, max })
    }
}

/// Configuration for a table of a wasm mdoule
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct TableType<'a> {
    /// Limits on the element sizes of this table
    pub limits: Limits,
    /// The type of element stored in this table
    pub elem: RefType<'a>,
}

impl<'a> Parse<'a> for TableType<'a> {
    fn parse(parser: Parser<'a>) -> Result<Self> {
        Ok(TableType {
            limits: parser.parse()?,
            elem: parser.parse()?,
        })
    }
}

/// Configuration for a memory of a wasm module
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum MemoryType {
    /// A 32-bit memory
    B32 {
        /// Limits on the page sizes of this memory
        limits: Limits,
        /// Whether or not this is a shared (atomic) memory type
        shared: bool,
    },
    /// A 64-bit memory
    B64 {
        /// Limits on the page sizes of this memory
        limits: Limits64,
        /// Whether or not this is a shared (atomic) memory type
        shared: bool,
    },
}

impl<'a> Parse<'a> for MemoryType {
    fn parse(parser: Parser<'a>) -> Result<Self> {
        if parser.peek::<kw::i64>() {
            parser.parse::<kw::i64>()?;
            let limits = parser.parse()?;
            let shared = parser.parse::<Option<kw::shared>>()?.is_some();
            Ok(MemoryType::B64 { limits, shared })
        } else {
            parser.parse::<Option<kw::i32>>()?;
            let limits = parser.parse()?;
            let shared = parser.parse::<Option<kw::shared>>()?.is_some();
            Ok(MemoryType::B32 { limits, shared })
        }
    }
}

/// A function type with parameters and results.
#[derive(Clone, Debug, Default)]
pub struct FunctionType<'a> {
    /// The parameters of a function, optionally each having an identifier for
    /// name resolution and a name for the custom `name` section.
    pub params: Box<
        [(
            Option<ast::Id<'a>>,
            Option<ast::NameAnnotation<'a>>,
            ValType<'a>,
        )],
    >,
    /// The results types of a function.
    pub results: Box<[ValType<'a>]>,
}

impl<'a> FunctionType<'a> {
    fn finish_parse(&mut self, allow_names: bool, parser: Parser<'a>) -> Result<()> {
        let mut params = Vec::from(mem::take(&mut self.params));
        let mut results = Vec::from(mem::take(&mut self.results));
        while parser.peek2::<kw::param>() || parser.peek2::<kw::result>() {
            parser.parens(|p| {
                let mut l = p.lookahead1();
                if l.peek::<kw::param>() {
                    if results.len() > 0 {
                        return Err(p.error(
                            "result before parameter (or unexpected token): \
                             cannot list params after results",
                        ));
                    }
                    p.parse::<kw::param>()?;
                    if p.is_empty() {
                        return Ok(());
                    }
                    let (id, name) = if allow_names {
                        (p.parse::<Option<_>>()?, p.parse::<Option<_>>()?)
                    } else {
                        (None, None)
                    };
                    let parse_more = id.is_none() && name.is_none();
                    let ty = p.parse()?;
                    params.push((id, name, ty));
                    while parse_more && !p.is_empty() {
                        params.push((None, None, p.parse()?));
                    }
                } else if l.peek::<kw::result>() {
                    p.parse::<kw::result>()?;
                    while !p.is_empty() {
                        results.push(p.parse()?);
                    }
                } else {
                    return Err(l.error());
                }
                Ok(())
            })?;
        }
        self.params = params.into();
        self.results = results.into();
        Ok(())
    }
}

impl<'a> Parse<'a> for FunctionType<'a> {
    fn parse(parser: Parser<'a>) -> Result<Self> {
        let mut ret = FunctionType {
            params: Box::new([]),
            results: Box::new([]),
        };
        ret.finish_parse(true, parser)?;
        Ok(ret)
    }
}

impl<'a> Peek for FunctionType<'a> {
    fn peek(cursor: Cursor<'_>) -> bool {
        if let Some(next) = cursor.lparen() {
            match next.keyword() {
                Some(("param", _)) | Some(("result", _)) => return true,
                _ => {}
            }
        }

        false
    }

    fn display() -> &'static str {
        "function type"
    }
}

/// A function type with parameters and results.
#[derive(Clone, Debug, Default)]
pub struct FunctionTypeNoNames<'a>(pub FunctionType<'a>);

impl<'a> Parse<'a> for FunctionTypeNoNames<'a> {
    fn parse(parser: Parser<'a>) -> Result<Self> {
        let mut ret = FunctionType {
            params: Box::new([]),
            results: Box::new([]),
        };
        ret.finish_parse(false, parser)?;
        Ok(FunctionTypeNoNames(ret))
    }
}

impl<'a> Peek for FunctionTypeNoNames<'a> {
    fn peek(cursor: Cursor<'_>) -> bool {
        FunctionType::peek(cursor)
    }

    fn display() -> &'static str {
        FunctionType::display()
    }
}

impl<'a> From<FunctionTypeNoNames<'a>> for FunctionType<'a> {
    fn from(ty: FunctionTypeNoNames<'a>) -> FunctionType<'a> {
        ty.0
    }
}

/// A struct type with fields.
#[derive(Clone, Debug)]
pub struct StructType<'a> {
    /// The fields of the struct
    pub fields: Vec<StructField<'a>>,
}

impl<'a> Parse<'a> for StructType<'a> {
    fn parse(parser: Parser<'a>) -> Result<Self> {
        let mut ret = StructType { fields: Vec::new() };
        while !parser.is_empty() {
            let field = if parser.peek2::<kw::field>() {
                parser.parens(|parser| {
                    parser.parse::<kw::field>()?;
                    StructField::parse(parser, true)
                })
            } else {
                StructField::parse(parser, false)
            };
            ret.fields.push(field?);
        }
        Ok(ret)
    }
}

/// A field of a struct type.
#[derive(Clone, Debug)]
pub struct StructField<'a> {
    /// An optional identifier for name resolution.
    pub id: Option<ast::Id<'a>>,
    /// Whether this field may be mutated or not.
    pub mutable: bool,
    /// The storage type stored in this field.
    pub ty: StorageType<'a>,
}

impl<'a> StructField<'a> {
    fn parse(parser: Parser<'a>, with_id: bool) -> Result<Self> {
        let id = if with_id { parser.parse()? } else { None };
        let (ty, mutable) = if parser.peek2::<kw::r#mut>() {
            let ty = parser.parens(|parser| {
                parser.parse::<kw::r#mut>()?;
                parser.parse()
            })?;
            (ty, true)
        } else {
            (parser.parse::<StorageType<'a>>()?, false)
        };
        Ok(StructField { id, mutable, ty })
    }
}

/// An array type with fields.
#[derive(Clone, Debug)]
pub struct ArrayType<'a> {
    /// Whether this field may be mutated or not.
    pub mutable: bool,
    /// The storage type stored in this field.
    pub ty: StorageType<'a>,
}

impl<'a> Parse<'a> for ArrayType<'a> {
    fn parse(parser: Parser<'a>) -> Result<Self> {
        let (ty, mutable) = if parser.peek2::<kw::r#mut>() {
            let ty = parser.parens(|parser| {
                parser.parse::<kw::r#mut>()?;
                parser.parse()
            })?;
            (ty, true)
        } else {
            (parser.parse::<StorageType<'a>>()?, false)
        };
        Ok(ArrayType { mutable, ty })
    }
}

/// A type for a nested module
#[derive(Clone, Debug, Default)]
pub struct ModuleType<'a> {
    /// The imports that are expected for this module type.
    pub imports: Vec<ast::Import<'a>>,
    /// The exports that this module type is expected to have.
    pub exports: Vec<ExportType<'a>>,
}

impl<'a> Parse<'a> for ModuleType<'a> {
    fn parse(parser: Parser<'a>) -> Result<Self> {
        let mut imports = Vec::new();
        while parser.peek2::<kw::import>() {
            imports.push(parser.parens(|p| p.parse())?);
        }
        let mut exports = Vec::new();
        while parser.peek2::<kw::export>() {
            parser.parens(|p| {
                exports.push(p.parse()?);
                Ok(())
            })?;
        }
        Ok(ModuleType { imports, exports })
    }
}

impl<'a> Peek for ModuleType<'a> {
    fn peek(cursor: Cursor<'_>) -> bool {
        if let Some(next) = cursor.lparen() {
            match next.keyword() {
                Some(("import", _)) | Some(("export", _)) => return true,
                _ => {}
            }
        }

        false
    }

    fn display() -> &'static str {
        "module type"
    }
}

/// A type for a nested instance
#[derive(Clone, Debug, Default)]
pub struct InstanceType<'a> {
    /// The exported types from this instance
    pub exports: Vec<ExportType<'a>>,
}

impl<'a> Parse<'a> for InstanceType<'a> {
    fn parse(parser: Parser<'a>) -> Result<Self> {
        let mut exports = Vec::new();
        while !parser.is_empty() {
            exports.push(parser.parens(|p| p.parse())?);
        }
        Ok(InstanceType { exports })
    }
}

impl<'a> Peek for InstanceType<'a> {
    fn peek(cursor: Cursor<'_>) -> bool {
        if let Some(next) = cursor.lparen() {
            match next.keyword() {
                Some(("export", _)) => return true,
                _ => {}
            }
        }

        false
    }

    fn display() -> &'static str {
        "instance type"
    }
}

/// The type of an exported item from a module or instance.
#[derive(Debug, Clone)]
pub struct ExportType<'a> {
    /// Where this export was defined.
    pub span: ast::Span,
    /// The name of this export.
    pub name: &'a str,
    /// The signature of the item that's exported.
    pub item: ast::ItemSig<'a>,
}

impl<'a> Parse<'a> for ExportType<'a> {
    fn parse(parser: Parser<'a>) -> Result<Self> {
        let span = parser.parse::<kw::export>()?.0;
        let name = parser.parse()?;
        let item = parser.parens(|p| p.parse())?;
        Ok(ExportType { span, name, item })
    }
}

/// A definition of a type.
#[derive(Debug)]
pub enum TypeDef<'a> {
    /// A function type definition.
    Func(FunctionType<'a>),
    /// A struct type definition.
    Struct(StructType<'a>),
    /// An array type definition.
    Array(ArrayType<'a>),
    /// A module type definition.
    Module(ModuleType<'a>),
    /// An instance type definition.
    Instance(InstanceType<'a>),
}

/// A type declaration in a module
#[derive(Debug)]
pub struct Type<'a> {
    /// Where this type was defined.
    pub span: ast::Span,
    /// An optional identifer to refer to this `type` by as part of name
    /// resolution.
    pub id: Option<ast::Id<'a>>,
    /// The type that we're declaring.
    pub def: TypeDef<'a>,
}

impl<'a> Parse<'a> for Type<'a> {
    fn parse(parser: Parser<'a>) -> Result<Self> {
        let span = parser.parse::<kw::r#type>()?.0;
        let id = parser.parse()?;
        let def = parser.parens(|parser| {
            let mut l = parser.lookahead1();
            if l.peek::<kw::func>() {
                parser.parse::<kw::func>()?;
                Ok(TypeDef::Func(parser.parse()?))
            } else if l.peek::<kw::r#struct>() {
                parser.parse::<kw::r#struct>()?;
                Ok(TypeDef::Struct(parser.parse()?))
            } else if l.peek::<kw::array>() {
                parser.parse::<kw::array>()?;
                Ok(TypeDef::Array(parser.parse()?))
            } else if l.peek::<kw::module>() {
                parser.parse::<kw::module>()?;
                Ok(TypeDef::Module(parser.parse()?))
            } else if l.peek::<kw::instance>() {
                parser.parse::<kw::instance>()?;
                Ok(TypeDef::Instance(parser.parse()?))
            } else {
                Err(l.error())
            }
        })?;
        Ok(Type { span, id, def })
    }
}

/// A reference to a type defined in this module.
#[derive(Clone, Debug)]
pub struct TypeUse<'a, T> {
    /// The type that we're referencing, if it was present.
    pub index: Option<ast::ItemRef<'a, kw::r#type>>,
    /// The inline type, if present.
    pub inline: Option<T>,
}

impl<'a, T> TypeUse<'a, T> {
    /// Constructs a new instance of `TypeUse` without an inline definition but
    /// with an index specified.
    pub fn new_with_index(idx: ast::Index<'a>) -> TypeUse<'a, T> {
        TypeUse {
            index: Some(ast::ItemRef::Item {
                idx,
                kind: kw::r#type::default(),
                exports: Vec::new(),
            }),
            inline: None,
        }
    }
}

impl<'a, T: Peek + Parse<'a>> Parse<'a> for TypeUse<'a, T> {
    fn parse(parser: Parser<'a>) -> Result<Self> {
        let index = if parser.peek2::<kw::r#type>() {
            Some(parser.parse()?)
        } else {
            None
        };
        let inline = parser.parse()?;

        Ok(TypeUse { index, inline })
    }
}

impl<'a> From<TypeUse<'a, FunctionTypeNoNames<'a>>> for TypeUse<'a, FunctionType<'a>> {
    fn from(src: TypeUse<'a, FunctionTypeNoNames<'a>>) -> TypeUse<'a, FunctionType<'a>> {
        TypeUse {
            index: src.index,
            inline: src.inline.map(|x| x.into()),
        }
    }
}
