//! This module defines the `Type` type, representing the dynamic form of a component interface type.

use crate::component::matching::InstanceType;
use crate::{Engine, ExternType, FuncType};
use alloc::sync::Arc;
use core::fmt;
use core::ops::Deref;
use wasmtime_environ::PrimaryMap;
use wasmtime_environ::component::{
    ComponentTypes, Export, InterfaceType, ResourceIndex, TypeComponentIndex,
    TypeComponentInstanceIndex, TypeDef, TypeEnumIndex, TypeFlagsIndex, TypeFuncIndex,
    TypeFutureIndex, TypeFutureTableIndex, TypeListIndex, TypeModuleIndex, TypeOptionIndex,
    TypeRecordIndex, TypeResourceTableIndex, TypeResultIndex, TypeStreamIndex,
    TypeStreamTableIndex, TypeTupleIndex, TypeVariantIndex,
};

pub use crate::component::resources::ResourceType;

/// An owned and `'static` handle for type information in a component.
///
/// The components here are:
///
/// * `index` - a `TypeFooIndex` defined in the `wasmtime_environ` crate. This
///   then points into the next field of...
///
/// * `types` - this is an allocation originally created from compilation and is
///   stored in a compiled `Component`. This contains all types necessary and
///   information about recursive structures and all other type information
///   within the component. The above `index` points into this structure.
///
/// * `resources` - this is used to "close the loop" and represent a concrete
///   instance type rather than an abstract component type. Instantiating a
///   component with different resources produces different instance types but
///   the same underlying component type, so this field serves the purpose to
///   distinguish instance types from one another. This is runtime state created
///   during instantiation and threaded through here.
#[derive(Clone)]
struct Handle<T> {
    index: T,
    types: Arc<ComponentTypes>,
    resources: Arc<PrimaryMap<ResourceIndex, ResourceType>>,
}

impl<T> Handle<T> {
    fn new(index: T, ty: &InstanceType<'_>) -> Handle<T> {
        Handle {
            index,
            types: ty.types.clone(),
            resources: ty.resources.clone(),
        }
    }

    fn instance(&self) -> InstanceType<'_> {
        InstanceType {
            types: &self.types,
            resources: &self.resources,
        }
    }

    fn equivalent<'a>(
        &'a self,
        other: &'a Self,
        type_check: fn(&TypeChecker<'a>, T, T) -> bool,
    ) -> bool
    where
        T: PartialEq + Copy,
    {
        (self.index == other.index
            && Arc::ptr_eq(&self.types, &other.types)
            && Arc::ptr_eq(&self.resources, &other.resources))
            || type_check(
                &TypeChecker {
                    a_types: &self.types,
                    b_types: &other.types,
                    a_resource: &self.resources,
                    b_resource: &other.resources,
                },
                self.index,
                other.index,
            )
    }
}

impl<T: fmt::Debug> fmt::Debug for Handle<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Handle")
            .field("index", &self.index)
            .finish()
    }
}

/// Type checker between two `Handle`s
struct TypeChecker<'a> {
    a_types: &'a ComponentTypes,
    a_resource: &'a PrimaryMap<ResourceIndex, ResourceType>,
    b_types: &'a ComponentTypes,
    b_resource: &'a PrimaryMap<ResourceIndex, ResourceType>,
}

impl TypeChecker<'_> {
    fn interface_types_equal(&self, a: InterfaceType, b: InterfaceType) -> bool {
        match (a, b) {
            (InterfaceType::Own(o1), InterfaceType::Own(o2)) => self.resources_equal(o1, o2),
            (InterfaceType::Own(_), _) => false,
            (InterfaceType::Borrow(b1), InterfaceType::Borrow(b2)) => self.resources_equal(b1, b2),
            (InterfaceType::Borrow(_), _) => false,
            (InterfaceType::List(l1), InterfaceType::List(l2)) => self.lists_equal(l1, l2),
            (InterfaceType::List(_), _) => false,
            (InterfaceType::Record(r1), InterfaceType::Record(r2)) => self.records_equal(r1, r2),
            (InterfaceType::Record(_), _) => false,
            (InterfaceType::Variant(v1), InterfaceType::Variant(v2)) => self.variants_equal(v1, v2),
            (InterfaceType::Variant(_), _) => false,
            (InterfaceType::Result(r1), InterfaceType::Result(r2)) => self.results_equal(r1, r2),
            (InterfaceType::Result(_), _) => false,
            (InterfaceType::Option(o1), InterfaceType::Option(o2)) => self.options_equal(o1, o2),
            (InterfaceType::Option(_), _) => false,
            (InterfaceType::Enum(e1), InterfaceType::Enum(e2)) => self.enums_equal(e1, e2),
            (InterfaceType::Enum(_), _) => false,
            (InterfaceType::Tuple(t1), InterfaceType::Tuple(t2)) => self.tuples_equal(t1, t2),
            (InterfaceType::Tuple(_), _) => false,
            (InterfaceType::Flags(f1), InterfaceType::Flags(f2)) => self.flags_equal(f1, f2),
            (InterfaceType::Flags(_), _) => false,
            (InterfaceType::Bool, InterfaceType::Bool) => true,
            (InterfaceType::Bool, _) => false,
            (InterfaceType::U8, InterfaceType::U8) => true,
            (InterfaceType::U8, _) => false,
            (InterfaceType::U16, InterfaceType::U16) => true,
            (InterfaceType::U16, _) => false,
            (InterfaceType::U32, InterfaceType::U32) => true,
            (InterfaceType::U32, _) => false,
            (InterfaceType::U64, InterfaceType::U64) => true,
            (InterfaceType::U64, _) => false,
            (InterfaceType::S8, InterfaceType::S8) => true,
            (InterfaceType::S8, _) => false,
            (InterfaceType::S16, InterfaceType::S16) => true,
            (InterfaceType::S16, _) => false,
            (InterfaceType::S32, InterfaceType::S32) => true,
            (InterfaceType::S32, _) => false,
            (InterfaceType::S64, InterfaceType::S64) => true,
            (InterfaceType::S64, _) => false,
            (InterfaceType::Float32, InterfaceType::Float32) => true,
            (InterfaceType::Float32, _) => false,
            (InterfaceType::Float64, InterfaceType::Float64) => true,
            (InterfaceType::Float64, _) => false,
            (InterfaceType::String, InterfaceType::String) => true,
            (InterfaceType::String, _) => false,
            (InterfaceType::Char, InterfaceType::Char) => true,
            (InterfaceType::Char, _) => false,
            (InterfaceType::Future(t1), InterfaceType::Future(t2)) => {
                self.future_table_types_equal(t1, t2)
            }
            (InterfaceType::Future(_), _) => false,
            (InterfaceType::Stream(t1), InterfaceType::Stream(t2)) => {
                self.stream_table_types_equal(t1, t2)
            }
            (InterfaceType::Stream(_), _) => false,
            (InterfaceType::ErrorContext(_), InterfaceType::ErrorContext(_)) => true,
            (InterfaceType::ErrorContext(_), _) => false,
        }
    }

    fn lists_equal(&self, l1: TypeListIndex, l2: TypeListIndex) -> bool {
        let a = &self.a_types[l1];
        let b = &self.b_types[l2];
        self.interface_types_equal(a.element, b.element)
    }

    fn resources_equal(&self, o1: TypeResourceTableIndex, o2: TypeResourceTableIndex) -> bool {
        let a = &self.a_types[o1];
        let b = &self.b_types[o2];
        self.a_resource[a.ty] == self.b_resource[b.ty]
    }

    fn records_equal(&self, r1: TypeRecordIndex, r2: TypeRecordIndex) -> bool {
        let a = &self.a_types[r1];
        let b = &self.b_types[r2];
        if a.fields.len() != b.fields.len() {
            return false;
        }
        a.fields
            .iter()
            .zip(b.fields.iter())
            .all(|(a_field, b_field)| {
                a_field.name == b_field.name && self.interface_types_equal(a_field.ty, b_field.ty)
            })
    }

    fn variants_equal(&self, v1: TypeVariantIndex, v2: TypeVariantIndex) -> bool {
        let a = &self.a_types[v1];
        let b = &self.b_types[v2];
        if a.cases.len() != b.cases.len() {
            return false;
        }
        a.cases
            .iter()
            .zip(b.cases.iter())
            .all(|((a_name, a_ty), (b_name, b_ty))| {
                if a_name != b_name {
                    return false;
                }
                match (a_ty, b_ty) {
                    (Some(a_case_ty), Some(b_case_ty)) => {
                        self.interface_types_equal(*a_case_ty, *b_case_ty)
                    }
                    (None, None) => true,
                    _ => false,
                }
            })
    }

    fn results_equal(&self, r1: TypeResultIndex, r2: TypeResultIndex) -> bool {
        let a = &self.a_types[r1];
        let b = &self.b_types[r2];
        let oks = match (a.ok, b.ok) {
            (Some(ok1), Some(ok2)) => self.interface_types_equal(ok1, ok2),
            (None, None) => true,
            _ => false,
        };
        if !oks {
            return false;
        }
        match (a.err, b.err) {
            (Some(err1), Some(err2)) => self.interface_types_equal(err1, err2),
            (None, None) => true,
            _ => false,
        }
    }

    fn options_equal(&self, o1: TypeOptionIndex, o2: TypeOptionIndex) -> bool {
        let a = &self.a_types[o1];
        let b = &self.b_types[o2];
        self.interface_types_equal(a.ty, b.ty)
    }

    fn enums_equal(&self, e1: TypeEnumIndex, e2: TypeEnumIndex) -> bool {
        let a = &self.a_types[e1];
        let b = &self.b_types[e2];
        a.names == b.names
    }

    fn tuples_equal(&self, t1: TypeTupleIndex, t2: TypeTupleIndex) -> bool {
        let a = &self.a_types[t1];
        let b = &self.b_types[t2];
        if a.types.len() != b.types.len() {
            return false;
        }
        a.types
            .iter()
            .zip(b.types.iter())
            .all(|(&a, &b)| self.interface_types_equal(a, b))
    }

    fn flags_equal(&self, f1: TypeFlagsIndex, f2: TypeFlagsIndex) -> bool {
        let a = &self.a_types[f1];
        let b = &self.b_types[f2];
        a.names == b.names
    }

    fn future_table_types_equal(&self, t1: TypeFutureTableIndex, t2: TypeFutureTableIndex) -> bool {
        self.futures_equal(self.a_types[t1].ty, self.b_types[t2].ty)
    }

    fn futures_equal(&self, t1: TypeFutureIndex, t2: TypeFutureIndex) -> bool {
        let a = &self.a_types[t1];
        let b = &self.b_types[t2];
        match (a.payload, b.payload) {
            (Some(t1), Some(t2)) => self.interface_types_equal(t1, t2),
            (None, None) => true,
            _ => false,
        }
    }

    fn stream_table_types_equal(&self, t1: TypeStreamTableIndex, t2: TypeStreamTableIndex) -> bool {
        self.streams_equal(self.a_types[t1].ty, self.b_types[t2].ty)
    }

    fn streams_equal(&self, t1: TypeStreamIndex, t2: TypeStreamIndex) -> bool {
        let a = &self.a_types[t1];
        let b = &self.b_types[t2];
        match (a.payload, b.payload) {
            (Some(t1), Some(t2)) => self.interface_types_equal(t1, t2),
            (None, None) => true,
            _ => false,
        }
    }
}

/// A `list` interface type
#[derive(Clone, Debug)]
pub struct List(Handle<TypeListIndex>);

impl PartialEq for List {
    fn eq(&self, other: &Self) -> bool {
        self.0.equivalent(&other.0, TypeChecker::lists_equal)
    }
}

impl Eq for List {}

impl List {
    pub(crate) fn from(index: TypeListIndex, ty: &InstanceType<'_>) -> Self {
        List(Handle::new(index, ty))
    }

    /// Retrieve the element type of this `list`.
    pub fn ty(&self) -> Type {
        Type::from(&self.0.types[self.0.index].element, &self.0.instance())
    }
}

/// A field declaration belonging to a `record`
#[derive(Debug)]
pub struct Field<'a> {
    /// The name of the field
    pub name: &'a str,
    /// The type of the field
    pub ty: Type,
}

/// A `record` interface type
#[derive(Clone, Debug)]
pub struct Record(Handle<TypeRecordIndex>);

impl Record {
    pub(crate) fn from(index: TypeRecordIndex, ty: &InstanceType<'_>) -> Self {
        Record(Handle::new(index, ty))
    }

    /// Retrieve the fields of this `record` in declaration order.
    pub fn fields(&self) -> impl ExactSizeIterator<Item = Field<'_>> {
        self.0.types[self.0.index].fields.iter().map(|field| Field {
            name: &field.name,
            ty: Type::from(&field.ty, &self.0.instance()),
        })
    }
}

impl PartialEq for Record {
    fn eq(&self, other: &Self) -> bool {
        self.0.equivalent(&other.0, TypeChecker::records_equal)
    }
}

impl Eq for Record {}

/// A `tuple` interface type
#[derive(Clone, Debug)]
pub struct Tuple(Handle<TypeTupleIndex>);

impl Tuple {
    pub(crate) fn from(index: TypeTupleIndex, ty: &InstanceType<'_>) -> Self {
        Tuple(Handle::new(index, ty))
    }

    /// Retrieve the types of the fields of this `tuple` in declaration order.
    pub fn types(&self) -> impl ExactSizeIterator<Item = Type> + '_ {
        self.0.types[self.0.index]
            .types
            .iter()
            .map(|ty| Type::from(ty, &self.0.instance()))
    }
}

impl PartialEq for Tuple {
    fn eq(&self, other: &Self) -> bool {
        self.0.equivalent(&other.0, TypeChecker::tuples_equal)
    }
}

impl Eq for Tuple {}

/// A case declaration belonging to a `variant`
pub struct Case<'a> {
    /// The name of the case
    pub name: &'a str,
    /// The optional payload type of the case
    pub ty: Option<Type>,
}

/// A `variant` interface type
#[derive(Clone, Debug)]
pub struct Variant(Handle<TypeVariantIndex>);

impl Variant {
    pub(crate) fn from(index: TypeVariantIndex, ty: &InstanceType<'_>) -> Self {
        Variant(Handle::new(index, ty))
    }

    /// Retrieve the cases of this `variant` in declaration order.
    pub fn cases(&self) -> impl ExactSizeIterator<Item = Case<'_>> {
        self.0.types[self.0.index]
            .cases
            .iter()
            .map(|(name, ty)| Case {
                name,
                ty: ty.as_ref().map(|ty| Type::from(ty, &self.0.instance())),
            })
    }
}

impl PartialEq for Variant {
    fn eq(&self, other: &Self) -> bool {
        self.0.equivalent(&other.0, TypeChecker::variants_equal)
    }
}

impl Eq for Variant {}

/// An `enum` interface type
#[derive(Clone, Debug)]
pub struct Enum(Handle<TypeEnumIndex>);

impl Enum {
    pub(crate) fn from(index: TypeEnumIndex, ty: &InstanceType<'_>) -> Self {
        Enum(Handle::new(index, ty))
    }

    /// Retrieve the names of the cases of this `enum` in declaration order.
    pub fn names(&self) -> impl ExactSizeIterator<Item = &str> {
        self.0.types[self.0.index]
            .names
            .iter()
            .map(|name| name.deref())
    }
}

impl PartialEq for Enum {
    fn eq(&self, other: &Self) -> bool {
        self.0.equivalent(&other.0, TypeChecker::enums_equal)
    }
}

impl Eq for Enum {}

/// An `option` interface type
#[derive(Clone, Debug)]
pub struct OptionType(Handle<TypeOptionIndex>);

impl OptionType {
    pub(crate) fn from(index: TypeOptionIndex, ty: &InstanceType<'_>) -> Self {
        OptionType(Handle::new(index, ty))
    }

    /// Retrieve the type parameter for this `option`.
    pub fn ty(&self) -> Type {
        Type::from(&self.0.types[self.0.index].ty, &self.0.instance())
    }
}

impl PartialEq for OptionType {
    fn eq(&self, other: &Self) -> bool {
        self.0.equivalent(&other.0, TypeChecker::options_equal)
    }
}

impl Eq for OptionType {}

/// A `result` interface type
#[derive(Clone, Debug)]
pub struct ResultType(Handle<TypeResultIndex>);

impl ResultType {
    pub(crate) fn from(index: TypeResultIndex, ty: &InstanceType<'_>) -> Self {
        ResultType(Handle::new(index, ty))
    }

    /// Retrieve the `ok` type parameter for this `result`.
    pub fn ok(&self) -> Option<Type> {
        Some(Type::from(
            self.0.types[self.0.index].ok.as_ref()?,
            &self.0.instance(),
        ))
    }

    /// Retrieve the `err` type parameter for this `result`.
    pub fn err(&self) -> Option<Type> {
        Some(Type::from(
            self.0.types[self.0.index].err.as_ref()?,
            &self.0.instance(),
        ))
    }
}

impl PartialEq for ResultType {
    fn eq(&self, other: &Self) -> bool {
        self.0.equivalent(&other.0, TypeChecker::results_equal)
    }
}

impl Eq for ResultType {}

/// A `flags` interface type
#[derive(Clone, Debug)]
pub struct Flags(Handle<TypeFlagsIndex>);

impl Flags {
    pub(crate) fn from(index: TypeFlagsIndex, ty: &InstanceType<'_>) -> Self {
        Flags(Handle::new(index, ty))
    }

    /// Retrieve the names of the flags of this `flags` type in declaration order.
    pub fn names(&self) -> impl ExactSizeIterator<Item = &str> {
        self.0.types[self.0.index]
            .names
            .iter()
            .map(|name| name.deref())
    }
}

impl PartialEq for Flags {
    fn eq(&self, other: &Self) -> bool {
        self.0.equivalent(&other.0, TypeChecker::flags_equal)
    }
}

impl Eq for Flags {}

/// An `future` interface type
#[derive(Clone, Debug)]
pub struct FutureType(Handle<TypeFutureIndex>);

impl FutureType {
    pub(crate) fn from(index: TypeFutureIndex, ty: &InstanceType<'_>) -> Self {
        FutureType(Handle::new(index, ty))
    }

    /// Retrieve the type parameter for this `future`.
    pub fn ty(&self) -> Option<Type> {
        Some(Type::from(
            self.0.types[self.0.index].payload.as_ref()?,
            &self.0.instance(),
        ))
    }
}

impl PartialEq for FutureType {
    fn eq(&self, other: &Self) -> bool {
        self.0.equivalent(&other.0, TypeChecker::futures_equal)
    }
}

impl Eq for FutureType {}

/// An `stream` interface type
#[derive(Clone, Debug)]
pub struct StreamType(Handle<TypeStreamIndex>);

impl StreamType {
    pub(crate) fn from(index: TypeStreamIndex, ty: &InstanceType<'_>) -> Self {
        StreamType(Handle::new(index, ty))
    }

    /// Retrieve the type parameter for this `stream`.
    pub fn ty(&self) -> Option<Type> {
        Some(Type::from(
            self.0.types[self.0.index].payload.as_ref()?,
            &self.0.instance(),
        ))
    }
}

impl PartialEq for StreamType {
    fn eq(&self, other: &Self) -> bool {
        self.0.equivalent(&other.0, TypeChecker::streams_equal)
    }
}

impl Eq for StreamType {}

/// Represents a component model interface type
#[derive(Clone, PartialEq, Eq, Debug)]
#[expect(missing_docs, reason = "self-describing variants")]
pub enum Type {
    Bool,
    S8,
    U8,
    S16,
    U16,
    S32,
    U32,
    S64,
    U64,
    Float32,
    Float64,
    Char,
    String,
    List(List),
    Record(Record),
    Tuple(Tuple),
    Variant(Variant),
    Enum(Enum),
    Option(OptionType),
    Result(ResultType),
    Flags(Flags),
    Own(ResourceType),
    Borrow(ResourceType),
    Future(FutureType),
    Stream(StreamType),
    ErrorContext,
}

impl Type {
    /// Retrieve the inner [`List`] of a [`Type::List`].
    ///
    /// # Panics
    ///
    /// This will panic if `self` is not a [`Type::List`].
    pub fn unwrap_list(&self) -> &List {
        if let Type::List(handle) = self {
            &handle
        } else {
            panic!("attempted to unwrap a {} as a list", self.desc())
        }
    }

    /// Retrieve the inner [`Record`] of a [`Type::Record`].
    ///
    /// # Panics
    ///
    /// This will panic if `self` is not a [`Type::Record`].
    pub fn unwrap_record(&self) -> &Record {
        if let Type::Record(handle) = self {
            &handle
        } else {
            panic!("attempted to unwrap a {} as a record", self.desc())
        }
    }

    /// Retrieve the inner [`Tuple`] of a [`Type::Tuple`].
    ///
    /// # Panics
    ///
    /// This will panic if `self` is not a [`Type::Tuple`].
    pub fn unwrap_tuple(&self) -> &Tuple {
        if let Type::Tuple(handle) = self {
            &handle
        } else {
            panic!("attempted to unwrap a {} as a tuple", self.desc())
        }
    }

    /// Retrieve the inner [`Variant`] of a [`Type::Variant`].
    ///
    /// # Panics
    ///
    /// This will panic if `self` is not a [`Type::Variant`].
    pub fn unwrap_variant(&self) -> &Variant {
        if let Type::Variant(handle) = self {
            &handle
        } else {
            panic!("attempted to unwrap a {} as a variant", self.desc())
        }
    }

    /// Retrieve the inner [`Enum`] of a [`Type::Enum`].
    ///
    /// # Panics
    ///
    /// This will panic if `self` is not a [`Type::Enum`].
    pub fn unwrap_enum(&self) -> &Enum {
        if let Type::Enum(handle) = self {
            &handle
        } else {
            panic!("attempted to unwrap a {} as a enum", self.desc())
        }
    }

    /// Retrieve the inner [`OptionType`] of a [`Type::Option`].
    ///
    /// # Panics
    ///
    /// This will panic if `self` is not a [`Type::Option`].
    pub fn unwrap_option(&self) -> &OptionType {
        if let Type::Option(handle) = self {
            &handle
        } else {
            panic!("attempted to unwrap a {} as a option", self.desc())
        }
    }

    /// Retrieve the inner [`ResultType`] of a [`Type::Result`].
    ///
    /// # Panics
    ///
    /// This will panic if `self` is not a [`Type::Result`].
    pub fn unwrap_result(&self) -> &ResultType {
        if let Type::Result(handle) = self {
            &handle
        } else {
            panic!("attempted to unwrap a {} as a result", self.desc())
        }
    }

    /// Retrieve the inner [`Flags`] of a [`Type::Flags`].
    ///
    /// # Panics
    ///
    /// This will panic if `self` is not a [`Type::Flags`].
    pub fn unwrap_flags(&self) -> &Flags {
        if let Type::Flags(handle) = self {
            &handle
        } else {
            panic!("attempted to unwrap a {} as a flags", self.desc())
        }
    }

    /// Retrieve the inner [`ResourceType`] of a [`Type::Own`].
    ///
    /// # Panics
    ///
    /// This will panic if `self` is not a [`Type::Own`].
    pub fn unwrap_own(&self) -> &ResourceType {
        match self {
            Type::Own(ty) => ty,
            _ => panic!("attempted to unwrap a {} as a own", self.desc()),
        }
    }

    /// Retrieve the inner [`ResourceType`] of a [`Type::Borrow`].
    ///
    /// # Panics
    ///
    /// This will panic if `self` is not a [`Type::Borrow`].
    pub fn unwrap_borrow(&self) -> &ResourceType {
        match self {
            Type::Borrow(ty) => ty,
            _ => panic!("attempted to unwrap a {} as a own", self.desc()),
        }
    }

    /// Convert the specified `InterfaceType` to a `Type`.
    pub(crate) fn from(ty: &InterfaceType, instance: &InstanceType<'_>) -> Self {
        match ty {
            InterfaceType::Bool => Type::Bool,
            InterfaceType::S8 => Type::S8,
            InterfaceType::U8 => Type::U8,
            InterfaceType::S16 => Type::S16,
            InterfaceType::U16 => Type::U16,
            InterfaceType::S32 => Type::S32,
            InterfaceType::U32 => Type::U32,
            InterfaceType::S64 => Type::S64,
            InterfaceType::U64 => Type::U64,
            InterfaceType::Float32 => Type::Float32,
            InterfaceType::Float64 => Type::Float64,
            InterfaceType::Char => Type::Char,
            InterfaceType::String => Type::String,
            InterfaceType::List(index) => Type::List(List::from(*index, instance)),
            InterfaceType::Record(index) => Type::Record(Record::from(*index, instance)),
            InterfaceType::Tuple(index) => Type::Tuple(Tuple::from(*index, instance)),
            InterfaceType::Variant(index) => Type::Variant(Variant::from(*index, instance)),
            InterfaceType::Enum(index) => Type::Enum(Enum::from(*index, instance)),
            InterfaceType::Option(index) => Type::Option(OptionType::from(*index, instance)),
            InterfaceType::Result(index) => Type::Result(ResultType::from(*index, instance)),
            InterfaceType::Flags(index) => Type::Flags(Flags::from(*index, instance)),
            InterfaceType::Own(index) => Type::Own(instance.resource_type(*index)),
            InterfaceType::Borrow(index) => Type::Borrow(instance.resource_type(*index)),
            InterfaceType::Future(index) => Type::Future(instance.future_type(*index)),
            InterfaceType::Stream(index) => Type::Stream(instance.stream_type(*index)),
            InterfaceType::ErrorContext(_) => Type::ErrorContext,
        }
    }

    fn desc(&self) -> &'static str {
        match self {
            Type::Bool => "bool",
            Type::S8 => "s8",
            Type::U8 => "u8",
            Type::S16 => "s16",
            Type::U16 => "u16",
            Type::S32 => "s32",
            Type::U32 => "u32",
            Type::S64 => "s64",
            Type::U64 => "u64",
            Type::Float32 => "float32",
            Type::Float64 => "float64",
            Type::Char => "char",
            Type::String => "string",
            Type::List(_) => "list",
            Type::Record(_) => "record",
            Type::Tuple(_) => "tuple",
            Type::Variant(_) => "variant",
            Type::Enum(_) => "enum",
            Type::Option(_) => "option",
            Type::Result(_) => "result",
            Type::Flags(_) => "flags",
            Type::Own(_) => "own",
            Type::Borrow(_) => "borrow",
            Type::Future(_) => "future",
            Type::Stream(_) => "stream",
            Type::ErrorContext => "error-context",
        }
    }
}

/// Component function type
#[derive(Clone, Debug)]
pub struct ComponentFunc(Handle<TypeFuncIndex>);

impl ComponentFunc {
    pub(crate) fn from(index: TypeFuncIndex, ty: &InstanceType<'_>) -> Self {
        Self(Handle::new(index, ty))
    }

    /// Iterates over types of function parameters and names.
    pub fn params(&self) -> impl ExactSizeIterator<Item = (&str, Type)> + '_ {
        let ty = &self.0.types[self.0.index];
        self.0.types[ty.params]
            .types
            .iter()
            .zip(&ty.param_names)
            .map(|(ty, name)| (name.as_str(), Type::from(ty, &self.0.instance())))
    }

    /// Iterates over types of function results
    pub fn results(&self) -> impl ExactSizeIterator<Item = Type> + '_ {
        let results = self.0.types[self.0.index].results;
        self.0.types[results]
            .types
            .iter()
            .map(|ty| Type::from(ty, &self.0.instance()))
    }

    #[doc(hidden)]
    pub fn typecheck<Params, Return>(&self, cx: &InstanceType) -> anyhow::Result<()>
    where
        Params: crate::component::ComponentNamedList + crate::component::Lower,
        Return: crate::component::ComponentNamedList + crate::component::Lift,
    {
        let ty = &self.0.types[self.0.index];
        Params::typecheck(&InterfaceType::Tuple(ty.params), cx)?;
        Return::typecheck(&InterfaceType::Tuple(ty.results), cx)?;
        Ok(())
    }
}

/// Core module type
#[derive(Clone, Debug)]
pub struct Module(Handle<TypeModuleIndex>);

impl Module {
    pub(crate) fn from(index: TypeModuleIndex, ty: &InstanceType<'_>) -> Self {
        Self(Handle::new(index, ty))
    }

    /// Iterates over imports of the module
    pub fn imports<'a>(
        &'a self,
        engine: &'a Engine,
    ) -> impl ExactSizeIterator<Item = ((&'a str, &'a str), ExternType)> + 'a {
        self.0.types[self.0.index]
            .imports
            .iter()
            .map(|((namespace, name), ty)| {
                (
                    (namespace.as_str(), name.as_str()),
                    ExternType::from_wasmtime(engine, self.0.types.module_types(), ty),
                )
            })
    }

    /// Iterates over exports of the module
    pub fn exports<'a>(
        &'a self,
        engine: &'a Engine,
    ) -> impl ExactSizeIterator<Item = (&'a str, ExternType)> + 'a {
        self.0.types[self.0.index].exports.iter().map(|(name, ty)| {
            (
                name.as_str(),
                ExternType::from_wasmtime(engine, self.0.types.module_types(), ty),
            )
        })
    }
}

/// Component type
#[derive(Clone, Debug)]
pub struct Component(Handle<TypeComponentIndex>);

impl Component {
    pub(crate) fn from(index: TypeComponentIndex, ty: &InstanceType<'_>) -> Self {
        Self(Handle::new(index, ty))
    }

    /// Returns import associated with `name`, if such exists in the component
    pub fn get_import(&self, engine: &Engine, name: &str) -> Option<ComponentItem> {
        self.0.types[self.0.index]
            .imports
            .get(name)
            .map(|ty| ComponentItem::from(engine, ty, &self.0.instance()))
    }

    /// Iterates over imports of the component
    pub fn imports<'a>(
        &'a self,
        engine: &'a Engine,
    ) -> impl ExactSizeIterator<Item = (&'a str, ComponentItem)> + 'a {
        self.0.types[self.0.index].imports.iter().map(|(name, ty)| {
            (
                name.as_str(),
                ComponentItem::from(engine, ty, &self.0.instance()),
            )
        })
    }

    /// Returns export associated with `name`, if such exists in the component
    pub fn get_export(&self, engine: &Engine, name: &str) -> Option<ComponentItem> {
        self.0.types[self.0.index]
            .exports
            .get(name)
            .map(|ty| ComponentItem::from(engine, ty, &self.0.instance()))
    }

    /// Iterates over exports of the component
    pub fn exports<'a>(
        &'a self,
        engine: &'a Engine,
    ) -> impl ExactSizeIterator<Item = (&'a str, ComponentItem)> + 'a {
        self.0.types[self.0.index].exports.iter().map(|(name, ty)| {
            (
                name.as_str(),
                ComponentItem::from(engine, ty, &self.0.instance()),
            )
        })
    }

    #[doc(hidden)]
    pub fn instance_type(&self) -> InstanceType<'_> {
        InstanceType {
            types: &self.0.types,
            resources: &self.0.resources,
        }
    }
}

/// Component instance type
#[derive(Clone, Debug)]
pub struct ComponentInstance(Handle<TypeComponentInstanceIndex>);

impl ComponentInstance {
    pub(crate) fn from(index: TypeComponentInstanceIndex, ty: &InstanceType<'_>) -> Self {
        Self(Handle::new(index, ty))
    }

    /// Returns export associated with `name`, if such exists in the component instance
    pub fn get_export(&self, engine: &Engine, name: &str) -> Option<ComponentItem> {
        self.0.types[self.0.index]
            .exports
            .get(name)
            .map(|ty| ComponentItem::from(engine, ty, &self.0.instance()))
    }

    /// Iterates over exports of the component instance
    pub fn exports<'a>(
        &'a self,
        engine: &'a Engine,
    ) -> impl ExactSizeIterator<Item = (&'a str, ComponentItem)> {
        self.0.types[self.0.index].exports.iter().map(|(name, ty)| {
            (
                name.as_str(),
                ComponentItem::from(engine, ty, &self.0.instance()),
            )
        })
    }
}

/// Type of an item contained within the component
#[derive(Clone, Debug)]
pub enum ComponentItem {
    /// Component function item
    ComponentFunc(ComponentFunc),
    /// Core function item
    CoreFunc(FuncType),
    /// Core module item
    Module(Module),
    /// Component item
    Component(Component),
    /// Component instance item
    ComponentInstance(ComponentInstance),
    /// Interface type item
    Type(Type),
    /// Resource item
    Resource(ResourceType),
}

impl ComponentItem {
    pub(crate) fn from(engine: &Engine, def: &TypeDef, ty: &InstanceType<'_>) -> Self {
        match def {
            TypeDef::Component(idx) => Self::Component(Component::from(*idx, ty)),
            TypeDef::ComponentInstance(idx) => {
                Self::ComponentInstance(ComponentInstance::from(*idx, ty))
            }
            TypeDef::ComponentFunc(idx) => Self::ComponentFunc(ComponentFunc::from(*idx, ty)),
            TypeDef::Interface(iface_ty) => Self::Type(Type::from(iface_ty, ty)),
            TypeDef::Module(idx) => Self::Module(Module::from(*idx, ty)),
            TypeDef::CoreFunc(idx) => {
                let subty = &ty.types[*idx];
                Self::CoreFunc(FuncType::from_wasm_func_type(
                    engine,
                    subty.is_final,
                    subty.supertype,
                    subty.unwrap_func().clone(),
                ))
            }
            TypeDef::Resource(idx) => {
                let resource_index = ty.types[*idx].ty;
                let ty = match ty.resources.get(resource_index) {
                    // This resource type was substituted by a linker for
                    // example so it's replaced here.
                    Some(ty) => *ty,

                    // This resource type was not substituted.
                    None => ResourceType::uninstantiated(&ty.types, resource_index),
                };
                Self::Resource(ty)
            }
        }
    }
    pub(crate) fn from_export(engine: &Engine, export: &Export, ty: &InstanceType<'_>) -> Self {
        match export {
            Export::Instance { ty: idx, .. } => {
                Self::ComponentInstance(ComponentInstance::from(*idx, ty))
            }
            Export::LiftedFunction { ty: idx, .. } => {
                Self::ComponentFunc(ComponentFunc::from(*idx, ty))
            }
            Export::ModuleStatic { ty: idx, .. } | Export::ModuleImport { ty: idx, .. } => {
                Self::Module(Module::from(*idx, ty))
            }
            Export::Type(idx) => Self::from(engine, idx, ty),
        }
    }
}
