use super::*;

pub trait Decode {
    fn decode(file: &'static File, code: usize) -> Self;
}

macro_rules! code {
    ($name:ident($size:literal) $(($table:ident, $code:literal))+) => {
        #[derive(Clone, Debug, Hash, PartialEq, Eq, Ord, PartialOrd)]
        pub enum $name {
            $($table($table),)*
        }
        impl Decode for $name {
            fn decode(file: &'static File, code: usize) -> Self {
                let (kind, row) = (code & ((1 << $size) - 1), (code >> $size) - 1);
                match kind {
                    $($code => Self::$table($table(Row::new(file, row))),)*
                    rest => panic!("{rest:?}"),
                }
            }
        }
        impl $name {
            #[allow(dead_code)]
            pub fn encode(&self) -> usize {
                match self {
                    $(Self::$table(row) => (row.index() + 1) << $size | $code,)*
                }
            }
        }
        $(
            impl From<$table> for $name {
                fn from(from: $table) -> Self {
                    Self::$table(from)
                }
            }
        )*
    };
}

code! { AttributeType(3)
    (MethodDef, 2)
    (MemberRef, 3)
}

impl AttributeType {
    pub fn parent(&self) -> MemberRefParent {
        match self {
            Self::MethodDef(row) => row.parent(),
            Self::MemberRef(row) => row.parent(),
        }
    }

    pub fn signature(&self) -> Blob {
        match self {
            Self::MethodDef(row) => row.blob(4),
            Self::MemberRef(row) => row.blob(2),
        }
    }
}

code! { HasAttribute(5)
    (MethodDef, 0)
    (Field, 1)
    (TypeRef, 2)
    (TypeDef, 3)
    (MethodParam, 4)
    (InterfaceImpl, 5)
    (MemberRef, 6)
    (TypeSpec, 13)
    (GenericParam, 19)
}

code! { HasConstant(2)
    (Field, 0)
}

code! { MemberForwarded(1)
    (MethodDef, 1)
}

code! { MemberRefParent(3)
    (TypeDef, 0)
    (TypeRef, 1)
}

impl MemberRefParent {
    pub fn type_name(&self) -> TypeName {
        match self {
            Self::TypeDef(row) => row.type_name(),
            Self::TypeRef(row) => row.type_name(),
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            Self::TypeDef(row) => row.name(),
            Self::TypeRef(row) => row.name(),
        }
    }
}

code! { TypeDefOrRef(2)
    (TypeDef, 0)
    (TypeRef, 1)
    (TypeSpec, 2)
}

code! { TypeOrMethodDef(1)
    (TypeDef, 0)
}

impl TypeDefOrRef {
    pub fn type_name(&self) -> TypeName {
        match self {
            Self::TypeDef(row) => row.type_name(),
            Self::TypeRef(row) => row.type_name(),
            rest => panic!("{rest:?}"),
        }
    }

    pub fn reader(&self) -> &'static Reader {
        match self {
            Self::TypeDef(row) => row.reader(),
            Self::TypeRef(row) => row.reader(),
            Self::TypeSpec(row) => row.reader(),
        }
    }
}
