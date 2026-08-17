use super::*;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Param {
    pub def: MethodParam,
    pub ty: Type,
}

impl std::ops::Deref for Param {
    type Target = Type;

    fn deref(&self) -> &Self::Target {
        &self.ty
    }
}

impl Param {
    pub fn is_convertible(&self) -> bool {
        self.is_input() && self.ty.is_convertible()
    }

    pub fn is_input(&self) -> bool {
        !self.def.flags().contains(ParamAttributes::Out)
    }

    pub fn is_optional(&self) -> bool {
        self.def.flags().contains(ParamAttributes::Optional)
            || self.def.has_attribute("ReservedAttribute")
    }

    pub fn is_retval(&self) -> bool {
        if !self.ty.is_pointer() {
            return false;
        }

        if self.ty.is_void() {
            return false;
        }

        let flags = self.def.flags();

        if flags.contains(ParamAttributes::In)
            || !flags.contains(ParamAttributes::Out)
            || flags.contains(ParamAttributes::Optional)
        {
            return false;
        }

        for attribute in self.def.attributes() {
            if matches!(
                attribute.name(),
                "NativeArrayInfoAttribute" | "MemorySizeAttribute"
            ) {
                return false;
            }
        }

        // If it's bigger than 128 bits, best to pass as a reference.
        if self.ty.deref().size() > 16 {
            return false;
        }

        true
    }

    pub fn write_ident(&self) -> TokenStream {
        to_ident(&self.def.name().to_lowercase())
    }
}
