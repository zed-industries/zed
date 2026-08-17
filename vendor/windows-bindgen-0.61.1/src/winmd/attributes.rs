macro_rules! flags {
    ($name:ident, $size:ty) => {
        #[derive(Default, Copy, Clone, PartialEq, Eq, Debug, Ord, PartialOrd)]
        pub struct $name(pub $size);
        impl $name {
            #[allow(dead_code)]
            pub fn contains(&self, contains: Self) -> bool {
                *self & contains == contains
            }
        }
        impl std::ops::BitOr for $name {
            type Output = Self;
            fn bitor(self, other: Self) -> Self {
                Self(self.0 | other.0)
            }
        }
        impl std::ops::BitAnd for $name {
            type Output = Self;
            fn bitand(self, other: Self) -> Self {
                Self(self.0 & other.0)
            }
        }
        impl std::ops::BitOrAssign for $name {
            fn bitor_assign(&mut self, other: Self) {
                self.0.bitor_assign(other.0)
            }
        }
        impl std::ops::BitAndAssign for $name {
            fn bitand_assign(&mut self, other: Self) {
                self.0.bitand_assign(other.0)
            }
        }
        impl std::ops::Not for $name {
            type Output = Self;
            fn not(self) -> Self {
                Self(self.0.not())
            }
        }
    };
}

flags!(FieldAttributes, u16);
impl FieldAttributes {
    pub const Literal: Self = Self(0x40);
}

flags!(MethodAttributes, u16);
impl MethodAttributes {
    pub const SpecialName: Self = Self(0x800);
}

flags!(MethodImplAttributes, u16);
impl MethodImplAttributes {}

// These are not really ECMA-335 attributes but instead the flags found in the method signature.
flags!(MethodCallAttributes, u8);
impl MethodCallAttributes {
    pub const HASTHIS: Self = Self(0x20);
    pub const VARARG: Self = Self(0x05);
}

flags!(ParamAttributes, u16);
impl ParamAttributes {
    pub const In: Self = Self(0x1);
    pub const Out: Self = Self(0x2);
    pub const Optional: Self = Self(0x10);
}

flags!(PInvokeAttributes, usize);
impl PInvokeAttributes {
    pub const SupportsLastError: Self = Self(0x40);
    pub const CallConvPlatformapi: Self = Self(0x100);
    pub const CallConvCdecl: Self = Self(0x200);
}

flags!(TypeAttributes, u32);
impl TypeAttributes {
    pub const ExplicitLayout: Self = Self(0x10);
    pub const WindowsRuntime: Self = Self(0x4000);

    pub fn is_nested(&self) -> bool {
        (self.0 & 0x00000006) != 0
    }
}
