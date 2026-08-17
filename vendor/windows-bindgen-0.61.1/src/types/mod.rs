use super::*;

mod class;
mod cpp_const;
mod cpp_delegate;
mod cpp_enum;
mod cpp_fn;
mod cpp_interface;
mod cpp_method;
mod cpp_struct;
mod delegate;
mod r#enum;
mod interface;
mod method;
mod r#struct;

pub use class::*;
pub use cpp_const::*;
pub use cpp_delegate::*;
pub use cpp_enum::*;
pub use cpp_fn::*;
pub use cpp_interface::*;
pub use cpp_method::*;
pub use cpp_struct::*;
pub use delegate::*;
pub use interface::*;
pub use method::*;
pub use r#enum::*;
pub use r#struct::*;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Type {
    CppFn(CppFn),
    Class(Class),
    Interface(Interface),
    CppInterface(CppInterface),
    Delegate(Delegate),
    CppDelegate(CppDelegate),
    Enum(Enum),
    CppEnum(CppEnum),
    Struct(Struct),
    CppStruct(CppStruct),
    CppConst(CppConst),

    Generic(GenericParam),
    PtrMut(Box<Self>, usize),
    PtrConst(Box<Self>, usize),
    ArrayFixed(Box<Self>, usize),
    Array(Box<Self>),
    ArrayRef(Box<Self>),
    ConstRef(Box<Self>),
    PrimitiveOrEnum(Box<Self>, Box<Self>),

    Void,
    Bool,
    Char,
    I8,
    U8,
    I16,
    U16,
    I32,
    U32,
    I64,
    U64,
    F32,
    F64,
    ISize,
    USize,
    String,
    Object,
    Type,

    PSTR,
    PCSTR,
    PWSTR,
    PCWSTR,
    GUID,
    HRESULT,
    IUnknown,
    BSTR,
    BOOL,
}

impl Ord for Type {
    fn cmp(&self, other: &Self) -> Ordering {
        self.sort_key().cmp(&(other.sort_key()))
    }
}

impl PartialOrd for Type {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(PartialEq)]
pub enum Remap {
    Type(Type),
    Name(TypeName),
    None,
}

impl Type {
    fn sort_key(&self) -> (bool, TypeName, i32, i32) {
        // This sorts types as follows:
        // 1. functions are placed first
        // 2. type name
        // 3. type namespace
        // 4. architecture
        // 5. overloaded types

        let kind = match self {
            Self::CppFn(..) => 0,
            Self::Class(..) => 1,
            Self::Interface(..) => 2,
            Self::CppInterface(..) => 3,
            Self::Delegate(..) => 4,
            Self::CppDelegate(..) => 5,
            Self::Enum(..) => 6,
            Self::CppEnum(..) => 7,
            Self::Struct(..) => 8,
            Self::CppStruct(..) => 9,
            Self::CppConst(..) => 10,
            _ => -1,
        };

        let arches = match self {
            Self::CppFn(ty) => ty.method.arches(),
            Self::CppStruct(ty) => ty.def.arches(),
            Self::CppDelegate(ty) => ty.def.arches(),
            _ => 0,
        };

        (kind != 0, self.type_name(), arches, kind)
    }

    fn is_intrinsic(&self) -> bool {
        matches!(
            self,
            Self::Generic(..)
                | Self::Void
                | Self::Bool
                | Self::Char
                | Self::I8
                | Self::U8
                | Self::I16
                | Self::U16
                | Self::I32
                | Self::U32
                | Self::I64
                | Self::U64
                | Self::F32
                | Self::F64
                | Self::ISize
                | Self::USize
        )
    }

    pub fn remap(type_name: TypeName) -> Remap {
        match type_name {
            TypeName("System", "Guid") => Remap::Type(Self::GUID),
            TypeName("Windows.Win32.Foundation", "PSTR") => Remap::Type(Self::PSTR),
            TypeName("Windows.Win32.Foundation", "PWSTR") => Remap::Type(Self::PWSTR),
            TypeName("Windows.Win32.System.WinRT", "HSTRING") => Remap::Type(Self::String),
            TypeName("Windows.Win32.Foundation", "BSTR") => Remap::Type(Self::BSTR),
            TypeName("Windows.Win32.System.WinRT", "IInspectable") => Remap::Type(Self::Object),
            TypeName("Windows.Win32.Foundation", "CHAR") => Remap::Type(Self::I8),
            TypeName("Windows.Win32.Foundation", "BOOLEAN") => Remap::Type(Self::Bool),
            TypeName("Windows.Win32.Foundation", "BOOL") => Remap::Type(Self::BOOL),
            TypeName("Windows.Win32.System.Com", "IUnknown") => Remap::Type(Self::IUnknown),
            TypeName("System", "Type") => Remap::Type(Self::Type),

            TypeName("Windows.Foundation", "HResult")
            | TypeName("Windows.Win32.Foundation", "HRESULT") => Remap::Type(Self::HRESULT),

            TypeName("Windows.Foundation", "EventRegistrationToken")
            | TypeName("Windows.Win32.System.WinRT", "EventRegistrationToken") => {
                Remap::Type(Type::I64)
            }

            TypeName("Windows.Win32.Graphics.Direct2D.Common", "D2D_MATRIX_3X2_F") => {
                Remap::Name(TypeName("Windows.Foundation.Numerics", "Matrix3x2"))
            }

            TypeName("Windows.Win32.Graphics.Direct3D", "D3DMATRIX")
            | TypeName("Windows.Win32.Graphics.Direct2D.Common", "D2D_MATRIX_4X4_F") => {
                Remap::Name(TypeName("Windows.Foundation.Numerics", "Matrix4x4"))
            }

            TypeName("Windows.Win32.Graphics.Direct2D.Common", "D2D_POINT_2F")
            | TypeName("Windows.Win32.Graphics.Direct2D.Common", "D2D_VECTOR_2F") => {
                Remap::Name(TypeName("Windows.Foundation.Numerics", "Vector2"))
            }

            TypeName("Windows.Win32.Graphics.Direct2D.Common", "D2D_VECTOR_4F") => {
                Remap::Name(TypeName("Windows.Foundation.Numerics", "Vector4"))
            }

            _ => Remap::None,
        }
    }

    pub fn from_element_type(code: usize) -> Option<Self> {
        match code as u8 {
            ELEMENT_TYPE_VOID => Some(Self::Void),
            ELEMENT_TYPE_BOOLEAN => Some(Self::Bool),
            ELEMENT_TYPE_CHAR => Some(Self::Char),
            ELEMENT_TYPE_I1 => Some(Self::I8),
            ELEMENT_TYPE_U1 => Some(Self::U8),
            ELEMENT_TYPE_I2 => Some(Self::I16),
            ELEMENT_TYPE_U2 => Some(Self::U16),
            ELEMENT_TYPE_I4 => Some(Self::I32),
            ELEMENT_TYPE_U4 => Some(Self::U32),
            ELEMENT_TYPE_I8 => Some(Self::I64),
            ELEMENT_TYPE_U8 => Some(Self::U64),
            ELEMENT_TYPE_R4 => Some(Self::F32),
            ELEMENT_TYPE_R8 => Some(Self::F64),
            ELEMENT_TYPE_I => Some(Self::ISize),
            ELEMENT_TYPE_U => Some(Self::USize),
            ELEMENT_TYPE_STRING => Some(Self::String),
            ELEMENT_TYPE_OBJECT => Some(Self::Object),
            _ => None,
        }
    }

    #[track_caller]
    pub fn from_ref(code: TypeDefOrRef, enclosing: Option<&CppStruct>, generics: &[Self]) -> Self {
        if let TypeDefOrRef::TypeSpec(def) = code {
            let mut blob = def.blob(0);
            return Self::from_blob_impl(&mut blob, None, generics);
        }

        let mut code_name = code.type_name();

        match Self::remap(code_name) {
            Remap::Type(ty) => return ty,
            Remap::Name(type_name) => {
                code_name = type_name;
            }
            Remap::None => {}
        }

        if let Some(outer) = enclosing {
            if code_name.namespace().is_empty() {
                return Type::CppStruct(outer.nested[code_name.name()].clone());
            }
        }

        code.reader()
            .unwrap_full_name(code_name.namespace(), code_name.name())
    }

    #[track_caller]
    pub fn from_blob(blob: &mut Blob, enclosing: Option<&CppStruct>, generics: &[Self]) -> Self {
        // Used by WinRT to indicate that a struct input parameter is passed by reference rather than by value on the ABI.
        let is_const = blob.read_modifiers().iter().any(|def| {
            let type_name = def.type_name();
            type_name == TypeName::IsConst
        });

        // Used by WinRT to indicate an output parameter, but there are other ways to determine this direction so here
        // it is only used to distinguish between slices and heap-allocated arrays.
        let is_ref = blob.try_read(ELEMENT_TYPE_BYREF as usize);

        if blob.try_read(ELEMENT_TYPE_VOID as usize) {
            return Self::Void;
        }

        let is_array = blob.try_read(ELEMENT_TYPE_SZARRAY as usize); // Used by WinRT to indicate an array

        let mut pointers = 0;

        while blob.try_read(ELEMENT_TYPE_PTR as usize) {
            pointers += 1;
        }

        let kind = Self::from_blob_impl(blob, enclosing, generics);

        if pointers > 0 {
            Self::PtrMut(Box::new(kind), pointers)
        } else if is_const {
            Self::ConstRef(Box::new(kind))
        } else if is_array {
            if is_ref {
                Self::ArrayRef(Box::new(kind))
            } else {
                Self::Array(Box::new(kind))
            }
        } else {
            kind
        }
    }

    #[track_caller]
    fn from_blob_impl(blob: &mut Blob, enclosing: Option<&CppStruct>, generics: &[Self]) -> Self {
        let code = blob.read_u8();

        if let Some(code) = Self::from_element_type(code as usize) {
            return code;
        }

        match code {
            ELEMENT_TYPE_VALUETYPE | ELEMENT_TYPE_CLASS => {
                Self::from_ref(blob.decode(), enclosing, generics)
            }
            ELEMENT_TYPE_VAR => generics
                .get(blob.read_usize())
                .unwrap_or(&Self::Void)
                .clone(),
            ELEMENT_TYPE_ARRAY => {
                let kind = Self::from_blob(blob, enclosing, generics);
                let _rank = blob.read_usize();
                let _count = blob.read_usize();
                let bounds = blob.read_usize();
                Self::ArrayFixed(Box::new(kind), bounds)
            }
            ELEMENT_TYPE_GENERICINST => {
                let type_code = blob.read_u8();

                debug_assert!(matches!(
                    type_code,
                    ELEMENT_TYPE_VALUETYPE | ELEMENT_TYPE_CLASS
                ));

                let code = blob.decode::<TypeDefOrRef>();
                let code_name = code.type_name();

                let mut ty = blob
                    .reader()
                    .unwrap_full_name(code_name.namespace(), code_name.name());

                let mut item_generics = vec![];

                for _ in 0..blob.read_usize() {
                    item_generics.push(Self::from_blob_impl(blob, enclosing, generics));
                }

                ty.set_generics(item_generics);
                ty
            }
            rest => panic!("{rest:?}"),
        }
    }

    pub fn to_const_type(&self) -> Self {
        match self {
            Self::PtrMut(ty, pointers) => Self::PtrMut(Box::new(ty.to_const_type()), *pointers),
            Self::PtrConst(ty, pointers) => Self::PtrConst(Box::new(ty.to_const_type()), *pointers),
            Self::PSTR => Self::PCSTR,
            Self::PWSTR => Self::PCWSTR,
            _ => self.clone(),
        }
    }

    pub fn to_const_ptr(&self) -> Self {
        match self {
            Self::PtrMut(ty, pointers) => Self::PtrConst(ty.clone(), *pointers),
            _ => self.clone(),
        }
    }

    pub fn deref(&self) -> Self {
        match self {
            Self::PtrConst(ty, 1) | Self::PtrMut(ty, 1) => {
                if **ty == Self::Void {
                    Self::U8
                } else {
                    *ty.clone()
                }
            }
            Self::PtrConst(ty, pointers) => Self::PtrConst(ty.clone(), pointers - 1),
            Self::PtrMut(ty, pointers) => Self::PtrMut(ty.clone(), pointers - 1),
            Self::PSTR | Self::PCSTR => Self::U8,
            Self::PWSTR | Self::PCWSTR => Self::U16,
            _ => self.clone(),
        }
    }

    pub fn is_interface(&self) -> bool {
        matches!(
            self,
            Self::Class(_)
                | Self::Interface(_)
                | Self::Delegate(_)
                | Self::CppInterface(_)
                | Self::IUnknown
                | Self::Object
        )
    }

    pub fn write_name(&self, config: &Config<'_>) -> TokenStream {
        if config.sys && self.is_interface() {
            return quote! { *mut core::ffi::c_void };
        }

        match self {
            Self::Void => quote! { core::ffi::c_void },
            Self::Bool => quote! { bool },
            Self::Char => quote! { u16 },
            Self::I8 => quote! { i8 },
            Self::U8 => quote! { u8 },
            Self::I16 => quote! { i16 },
            Self::U16 => quote! { u16 },
            Self::I32 => quote! { i32 },
            Self::U32 => quote! { u32 },
            Self::I64 => quote! { i64 },
            Self::U64 => quote! { u64 },
            Self::F32 => quote! { f32 },
            Self::F64 => quote! { f64 },
            Self::ISize => quote! { isize },
            Self::USize => quote! { usize },
            Self::BSTR => {
                let name = config.write_core();
                quote! { #name BSTR }
            }
            Self::IUnknown => {
                let name = config.write_core();
                quote! { #name IUnknown }
            }
            Self::GUID => {
                let name = config.write_core();
                quote! { #name GUID }
            }
            Self::HRESULT => {
                let name = config.write_core();
                quote! { #name HRESULT }
            }
            Self::BOOL => {
                let name = config.write_core();
                quote! { #name BOOL }
            }
            Self::String => {
                let name = config.write_core();
                quote! { #name HSTRING }
            }
            Self::Object => {
                let name = config.write_core();
                quote! { #name IInspectable }
            }
            Self::PSTR => {
                let name = config.write_core();
                quote! { #name PSTR }
            }
            Self::PCSTR => {
                let name = config.write_core();
                quote! { #name PCSTR }
            }
            Self::PWSTR => {
                let name = config.write_core();
                quote! { #name PWSTR }
            }
            Self::PCWSTR => {
                let name = config.write_core();
                quote! { #name PCWSTR }
            }
            Self::CppInterface(ty) => ty.write_name(config),
            Self::Struct(ty) => ty.write_name(config),
            Self::Enum(ty) => ty.write_name(config),
            Self::Interface(ty) => ty.write_name(config),
            Self::CppStruct(ty) => ty.write_name(config),
            Self::CppEnum(ty) => ty.write_name(config),
            Self::CppFn(ty) => ty.write_name(config),
            Self::CppConst(ty) => ty.write_name(config),
            Self::CppDelegate(ty) => ty.write_name(config),
            Self::Delegate(ty) => ty.write_name(config),
            Self::Class(ty) => ty.write_name(config),
            Self::Generic(param) => to_ident(param.name()),
            Self::PtrMut(ty, pointers) => {
                let pointers = write_ptr_mut(*pointers);
                let ty = ty.write_default(config);
                quote! { #pointers #ty }
            }
            Self::PtrConst(ty, pointers) => {
                let pointers = write_ptr_const(*pointers);
                let ty = ty.write_default(config);
                quote! { #pointers #ty }
            }
            Self::ArrayFixed(ty, len) => {
                let name = ty.write_default(config);
                let len = Literal::usize_unsuffixed(*len);
                quote! { [#name; #len] }
            }
            Self::Array(ty) => ty.write_name(config),
            Self::ArrayRef(ty) => ty.write_name(config),
            Self::ConstRef(ty) => ty.write_name(config),
            Self::PrimitiveOrEnum(primitive, ty) => {
                if config.sys {
                    primitive.write_name(config)
                } else {
                    ty.write_name(config)
                }
            }
            rest => panic!("{rest:?}"),
        }
    }

    pub fn write_default(&self, config: &Config<'_>) -> TokenStream {
        if config.sys {
            return self.write_name(config);
        }

        if let Self::Array(ty) = self {
            ty.write_default(config)
        } else {
            let tokens = self.write_name(config);

            if matches!(self, Self::Generic(_)) {
                quote! { <#tokens as windows_core::Type<#tokens>>::Default }
            } else if self.is_interface() {
                quote! { Option<#tokens> }
            } else {
                tokens
            }
        }
    }

    pub fn write_impl_name(&self, config: &Config<'_>) -> TokenStream {
        match self {
            Self::IUnknown | Self::Object => {
                let name = config.write_core();
                quote! { #name IUnknownImpl }
            }
            Self::CppInterface(ty) => ty.write_impl_name(config),
            Self::Interface(ty) => ty.write_impl_name(config),
            rest => panic!("{rest:?}"),
        }
    }

    pub fn write_abi(&self, config: &Config<'_>) -> TokenStream {
        if config.sys {
            return self.write_name(config);
        }

        match self {
            Self::IUnknown
            | Self::Object
            | Self::Delegate(_)
            | Self::Class(_)
            | Self::CppInterface(_)
            | Self::Interface(_)
            | Self::String
            | Self::BSTR => quote! { *mut core::ffi::c_void },
            Self::ArrayFixed(ty, len) => {
                let name = ty.write_abi(config);
                let len = Literal::usize_unsuffixed(*len);
                quote! { [#name; #len] }
            }
            Self::Generic(name) => {
                let name = to_ident(name.name());
                quote! { windows_core::AbiType<#name> }
            }
            Self::Struct(ty) => {
                let name = self.write_name(config);
                if ty.is_copyable() {
                    name
                } else {
                    quote! { core::mem::MaybeUninit<#name> }
                }
            }
            Self::PtrMut(ty, pointers) => {
                let ty = ty.write_abi(config);
                let pointers = write_ptr_mut(*pointers);
                quote! { #pointers #ty }
            }
            Self::PtrConst(ty, pointers) => {
                let ty = ty.write_abi(config);
                let pointers = write_ptr_const(*pointers);
                quote! { #pointers #ty }
            }
            Self::PrimitiveOrEnum(ty, _) => ty.write_name(config),
            ty => ty.write_name(config),
        }
    }

    pub fn runtime_signature(&self) -> String {
        match self {
            Self::Bool => "b1".to_string(),
            Self::Char => "c2".to_string(),
            Self::I8 => "i1".to_string(),
            Self::U8 => "u1".to_string(),
            Self::I16 => "i2".to_string(),
            Self::U16 => "u2".to_string(),
            Self::I32 => "i4".to_string(),
            Self::U32 => "u4".to_string(),
            Self::I64 => "i8".to_string(),
            Self::U64 => "u8".to_string(),
            Self::F32 => "f4".to_string(),
            Self::F64 => "f8".to_string(),
            Self::ISize => "is".to_string(),
            Self::USize => "us".to_string(),
            Self::String => "string".to_string(),
            Self::Object => "cinterface(IInspectable)".to_string(),
            Self::GUID => "g16".to_string(),
            Self::HRESULT => "struct(Windows.Foundation.HResult;i4)".to_string(),
            Self::Class(ty) => ty.runtime_signature(),
            Self::Delegate(ty) => ty.runtime_signature(),
            Self::Enum(ty) => ty.runtime_signature(),
            Self::Interface(ty) => ty.runtime_signature(),
            Self::Struct(ty) => ty.runtime_signature(),
            rest => panic!("{rest:?}"),
        }
    }

    pub fn split_generic(&self) -> (Type, Vec<Type>) {
        match self {
            Self::Interface(ty) if !ty.generics.is_empty() => {
                let base = ty
                    .def
                    .reader()
                    .unwrap_full_name(ty.def.namespace(), ty.def.name());
                (base, ty.generics.clone())
            }
            Self::Delegate(ty) if !ty.generics.is_empty() => {
                let base = ty
                    .def
                    .reader()
                    .unwrap_full_name(ty.def.namespace(), ty.def.name());
                (base, ty.generics.clone())
            }
            _ => (self.clone(), vec![]),
        }
    }

    pub fn decay(&self) -> &Self {
        match self {
            Type::PtrMut(ty, _) => ty,
            Type::PtrConst(ty, _) => ty,
            Type::ArrayFixed(ty, _) => ty.decay(),
            Type::Array(ty) => ty,
            Type::ArrayRef(ty) => ty,
            Type::ConstRef(ty) => ty,
            Type::PrimitiveOrEnum(_, ty) => ty,
            _ => self,
        }
    }

    pub fn is_exclusive(&self) -> bool {
        match self {
            Self::Interface(ty) => ty.def.has_attribute("ExclusiveToAttribute"),
            _ => false,
        }
    }

    pub fn is_winrt_array(&self) -> bool {
        matches!(self, Self::Array(_))
    }

    pub fn is_winrt_array_ref(&self) -> bool {
        matches!(self, Self::ArrayRef(_))
    }

    pub fn is_async(&self) -> bool {
        match self {
            Self::Interface(ty) => ty.def.is_async(),
            _ => false,
        }
    }

    pub fn is_copyable(&self) -> bool {
        match self {
            Self::Struct(ty) => ty.is_copyable(),
            Self::CppStruct(ty) => ty.is_copyable(),
            Self::Enum(_) => true,
            Self::CppEnum(_) => true,
            Self::CppDelegate(_) => true,

            Self::Interface(..) | Self::CppInterface(..) | Self::Class(..) | Self::Delegate(..) => {
                false
            }

            Self::String | Self::BSTR | Self::Object | Self::IUnknown | Self::Generic(_) => false,
            Self::ArrayFixed(ty, _) => ty.is_copyable(),
            Self::Array(ty) => ty.is_copyable(),
            _ => true,
        }
    }

    pub fn is_dropped(&self) -> bool {
        match self {
            Self::Struct(ty) => !ty.is_copyable(),
            Self::CppInterface(..) => true,
            Self::String | Self::BSTR | Self::Object | Self::IUnknown => true,
            Self::ArrayFixed(ty, _) => ty.is_dropped(),
            _ => false,
        }
    }

    pub fn is_convertible(&self) -> bool {
        matches!(
            self,
            Self::Delegate(..)
                | Self::Interface(..)
                | Self::Class(..)
                | Self::CppInterface(..)
                | Self::PCSTR
                | Self::PCWSTR
                | Self::Object
                | Self::IUnknown
                | Self::Generic(_)
        )
    }

    pub fn is_const_ref(&self) -> bool {
        matches!(self, Self::ConstRef(_))
    }

    pub fn is_primitive(&self) -> bool {
        match self {
            Self::Enum(_) | Self::CppEnum(_) | Self::CppDelegate(_) => true,
            Self::CppStruct(ty) => ty.is_handle(),
            Self::Bool
            | Self::Char
            | Self::I8
            | Self::U8
            | Self::I16
            | Self::U16
            | Self::I32
            | Self::U32
            | Self::I64
            | Self::U64
            | Self::F32
            | Self::F64
            | Self::ISize
            | Self::USize
            | Self::HRESULT
            | Self::BOOL
            | Self::PtrConst(_, _)
            | Self::PtrMut(_, _) => true,
            _ => false,
        }
    }

    pub fn is_unsigned(&self) -> bool {
        matches!(
            self,
            Self::U8 | Self::U16 | Self::U32 | Self::U64 | Self::USize
        )
    }

    pub fn is_pointer(&self) -> bool {
        matches!(self, Self::PtrConst(_, _) | Self::PtrMut(_, _))
    }

    pub fn is_byte_size(&self) -> bool {
        match self {
            Self::PtrConst(ty, _) | Self::PtrMut(ty, _) => ty.is_byte_size(),
            Self::I8 | Self::U8 | Self::PSTR | Self::PCSTR => true,
            _ => false,
        }
    }

    pub fn is_void(&self) -> bool {
        match self {
            Type::PtrConst(ty, _) | Type::PtrMut(ty, _) => ty.is_void(),
            Type::Void => true,
            _ => false,
        }
    }

    pub fn size(&self) -> usize {
        match self {
            Type::I8 | Type::U8 => 1,
            Type::I16 | Type::U16 => 2,
            Type::I64 | Type::U64 | Type::F64 => 8,
            Type::GUID => 16,
            Type::ArrayFixed(ty, len) => ty.size() * len,
            Type::PrimitiveOrEnum(ty, _) => ty.size(),
            Self::CppStruct(ty) => ty.size(),
            Self::Struct(ty) => ty.size(),
            Self::CppEnum(ty) => ty.size(),
            _ => 4,
        }
    }

    pub fn align(&self) -> usize {
        match self {
            Type::I8 | Type::U8 => 1,
            Type::I16 | Type::U16 => 2,
            Type::I64 | Type::U64 | Type::F64 => 8,
            Type::ArrayFixed(ty, len) => ty.align() * len,
            Self::CppStruct(ty) => ty.align(),
            Self::Struct(ty) => ty.align(),
            Self::CppEnum(ty) => ty.align(),
            _ => 4,
        }
    }

    pub fn underlying_type(&self) -> Self {
        match self {
            Self::Struct(ty) => ty.def.underlying_type(),
            Self::CppEnum(ty) => ty.def.underlying_type(),
            Self::Enum(ty) => ty.def.underlying_type(),
            Self::CppStruct(ty) => ty.def.underlying_type(),
            Self::HRESULT => Type::I32,
            Self::BOOL => Type::I32,
            _ => self.clone(),
        }
    }

    fn write_no_deps(&self, config: &Config<'_>) -> TokenStream {
        if !config.no_deps || !config.sys {
            return quote! {};
        }

        match self {
            Self::HRESULT => quote! { pub type HRESULT = i32; },
            Self::BOOL => quote! { pub type BOOL = i32; },

            Self::PWSTR => quote! { pub type PWSTR = *mut u16; },
            Self::PCSTR => quote! { pub type PCSTR = *const u8; },
            Self::PSTR => quote! { pub type PSTR = *mut u8; },
            Self::PCWSTR => quote! { pub type PCWSTR = *const u16; },
            Self::BSTR => quote! { pub type BSTR = *const u16; },
            Self::String => {
                quote! { pub type HSTRING = *mut core::ffi::c_void; }
            }
            Self::GUID => quote! {
                #[repr(C)]
                #[derive(Clone, Copy)]
                pub struct GUID {
                    pub data1: u32,
                    pub data2: u16,
                    pub data3: u16,
                    pub data4: [u8; 8],
                }
                impl GUID {
                    pub const fn from_u128(uuid: u128) -> Self {
                        Self { data1: (uuid >> 96) as u32, data2: (uuid >> 80 & 0xffff) as u16, data3: (uuid >> 64 & 0xffff) as u16, data4: (uuid as u64).to_be_bytes() }
                    }
                }
            },
            Self::IUnknown => quote! {
                pub const IID_IUnknown: GUID = GUID::from_u128(0x00000000_0000_0000_c000_000000000046);
                #[repr(C)]
                pub struct IUnknown_Vtbl {
                    pub QueryInterface: unsafe extern "system" fn(this: *mut core::ffi::c_void, iid: *const GUID, interface: *mut *mut core::ffi::c_void) -> HRESULT,
                    pub AddRef: unsafe extern "system" fn(this: *mut core::ffi::c_void) -> u32,
                    pub Release: unsafe extern "system" fn(this: *mut core::ffi::c_void) -> u32,
                }
            },
            Self::Object => quote! {
                pub const IID_IInspectable: GUID = GUID::from_u128(0xaf86e2e0_b12d_4c6a_9c5a_d7aa65101e90);
                #[repr(C)]
                pub struct IInspectable_Vtbl {
                    pub base: IUnknown_Vtbl,
                    pub GetIids: unsafe extern "system" fn(this: *mut core::ffi::c_void, count: *mut u32, values: *mut *mut GUID) -> HRESULT,
                    pub GetRuntimeClassName: unsafe extern "system" fn(this: *mut core::ffi::c_void, value: *mut *mut core::ffi::c_void) -> HRESULT,
                    pub GetTrustLevel: unsafe extern "system" fn(this: *mut core::ffi::c_void, value: *mut i32) -> HRESULT,
                }
            },

            _ => quote! {},
        }
    }

    pub fn write(&self, config: &Config<'_>) -> TokenStream {
        match self {
            Self::Struct(ty) => ty.write(config),
            Self::Enum(ty) => ty.write(config),
            Self::Interface(ty) => ty.write(config),
            Self::CppStruct(ty) => ty.write(config),
            Self::CppEnum(ty) => ty.write(config),
            Self::CppFn(ty) => ty.write(config),
            Self::CppConst(ty) => ty.write(config),
            Self::CppDelegate(ty) => ty.write(config),
            Self::Delegate(ty) => ty.write(config),
            Self::Class(ty) => ty.write(config),
            Self::CppInterface(ty) => ty.write(config),

            _ => self.write_no_deps(config),
        }
    }

    pub fn set_generics(&mut self, generics: Vec<Type>) {
        match self {
            Self::Interface(ty) => ty.generics = generics,
            Self::Delegate(ty) => ty.generics = generics,
            rest => panic!("{rest:?}"),
        }
    }

    pub fn type_name(&self) -> TypeName {
        match self {
            Self::Class(ty) => ty.type_name(),
            Self::Delegate(ty) => ty.type_name(),
            Self::Enum(ty) => ty.type_name(),
            Self::Interface(ty) => ty.type_name(),
            Self::Struct(ty) => ty.type_name(),
            Self::CppDelegate(ty) => ty.type_name(),
            Self::CppEnum(ty) => ty.type_name(),
            Self::CppInterface(ty) => ty.type_name(),
            Self::CppStruct(ty) => ty.type_name(),
            Self::CppConst(ty) => ty.type_name(),
            Self::CppFn(ty) => ty.type_name(),

            Self::PSTR => TypeName("", "PSTR"),
            Self::PCSTR => TypeName("", "PCSTR"),
            Self::PWSTR => TypeName("", "PWSTR"),
            Self::PCWSTR => TypeName("", "PCWSTR"),
            Self::GUID => TypeName("", "GUID"),
            Self::HRESULT => TypeName("", "HRESULT"),
            Self::BOOL => TypeName("", "BOOL"),
            Self::IUnknown => TypeName("", "IUnknown"),
            Self::BSTR => TypeName("", "BSTR"),
            Self::String => TypeName("", "String"),
            Self::Object => TypeName("", "Object"),

            _ => TypeName("", ""),
        }
    }

    pub fn is_core(&self) -> bool {
        matches!(
            self,
            Self::PSTR
                | Self::PCSTR
                | Self::PWSTR
                | Self::PCWSTR
                | Self::GUID
                | Self::HRESULT
                | Self::BOOL
                | Self::IUnknown
                | Self::Object
                | Self::BSTR
                | Self::String
        )
    }

    pub fn write_result_map(&self) -> TokenStream {
        if self.is_copyable() {
            quote! { map(|| result__) }
        } else if self.is_convertible() {
            quote! { and_then(||windows_core::Type::from_abi(result__)) }
        } else {
            quote! { map(|| core::mem::transmute(result__)) }
        }
    }
}

impl Dependencies for Type {
    fn combine(&self, dependencies: &mut TypeMap) {
        let ty = self.decay();

        if ty.is_intrinsic() {
            return;
        }

        let mut nested = false;

        if let Self::CppStruct(ty) = ty {
            if ty.def.namespace().is_empty() {
                nested = true;
            }
        }

        let (ty, generics) = ty.split_generic();

        for ty in generics {
            ty.combine(dependencies);
        }

        if !nested && !dependencies.insert(ty.clone()) {
            return;
        }

        if let Some(multi) = match &ty {
            Self::CppStruct(ty) => Some(
                ty.def
                    .reader()
                    .with_full_name(ty.def.namespace(), ty.def.name()),
            ),
            Self::CppFn(ty) => Some(
                ty.method
                    .reader()
                    .with_full_name(ty.namespace, ty.method.name()),
            ),
            _ => None,
        } {
            multi.for_each(|multi| {
                if ty != multi {
                    multi.combine(dependencies)
                }
            });
        }

        match &ty {
            Self::Class(ty) => ty.combine(dependencies),
            Self::Delegate(ty) => ty.combine(dependencies),
            Self::Enum(..) => {}
            Self::Interface(ty) => ty.combine(dependencies),
            Self::Struct(ty) => ty.combine(dependencies),
            Self::CppConst(ty) => ty.combine(dependencies),
            Self::CppDelegate(ty) => ty.combine(dependencies),
            Self::CppFn(ty) => ty.combine(dependencies),
            Self::CppInterface(ty) => ty.combine(dependencies),
            Self::CppStruct(ty) => ty.combine(dependencies),
            Self::CppEnum(ty) => ty.combine(dependencies),

            Self::IUnknown => {
                Self::GUID.combine(dependencies);
                Self::HRESULT.combine(dependencies);
            }

            Self::Object => {
                Self::IUnknown.combine(dependencies);
            }

            _ => {}
        }
    }
}

pub fn interface_signature(def: TypeDef, generics: &[Type]) -> String {
    if generics.is_empty() {
        let guid = def.guid_attribute().unwrap();
        format!("{{{guid}}}")
    } else {
        let guid = def.guid_attribute().unwrap();
        let mut signature = format!("pinterface({{{guid}}}");

        for generic in generics {
            signature.push(';');
            signature.push_str(&generic.runtime_signature())
        }

        signature.push(')');
        signature
    }
}

fn write_ptr_mut(pointers: usize) -> TokenStream {
    "*mut ".repeat(pointers).into()
}

fn write_ptr_const(pointers: usize) -> TokenStream {
    "*const ".repeat(pointers).into()
}
