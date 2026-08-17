use super::*;

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub struct TypeName(pub &'static str, pub &'static str);

impl Ord for TypeName {
    fn cmp(&self, other: &Self) -> Ordering {
        // Type names are sorted before namespaces. The `Type` sort order depends on this.
        // This is more efficient in general as many types typically share a namespace.
        (self.1, self.0).cmp(&(other.1, other.0))
    }
}

impl PartialOrd for TypeName {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl TypeName {
    pub const Object: Self = Self("System", "Object");
    pub const IsConst: Self = Self("System.Runtime.CompilerServices", "IsConst");

    pub const IAsyncAction: Self = Self("Windows.Foundation", "IAsyncAction");
    pub const IAsyncActionWithProgress: Self =
        Self("Windows.Foundation", "IAsyncActionWithProgress");
    pub const IAsyncOperation: Self = Self("Windows.Foundation", "IAsyncOperation");
    pub const IAsyncOperationWithProgress: Self =
        Self("Windows.Foundation", "IAsyncOperationWithProgress");

    pub const IIterable: Self = Self("Windows.Foundation.Collections", "IIterable");
    pub const IIterator: Self = Self("Windows.Foundation.Collections", "IIterator");

    pub const NTSTATUS: Self = Self("Windows.Win32.Foundation", "NTSTATUS");
    pub const WIN32_ERROR: Self = Self("Windows.Win32.Foundation", "WIN32_ERROR");
    pub const RPC_STATUS: Self = Self("Windows.Win32.System.Rpc", "RPC_STATUS");
    pub const VARIANT: Self = Self("Windows.Win32.System.Variant", "VARIANT");
    pub const PROPVARIANT: Self = Self("Windows.Win32.System.Com.StructuredStorage", "PROPVARIANT");

    pub fn parse(full_name: &'static str) -> Self {
        let index = full_name
            .rfind('.')
            .expect("Expected full name separated with `.`");
        Self(&full_name[0..index], &full_name[index + 1..])
    }

    pub fn namespace(&self) -> &'static str {
        self.0
    }

    pub fn name(&self) -> &'static str {
        self.1
    }

    pub fn write(&self, config: &Config<'_>, generics: &[Type]) -> TokenStream {
        let name = to_ident(self.name());
        let namespace = config.write_namespace(*self);

        if generics.is_empty() {
            quote! { #namespace #name }
        } else {
            let generics = generics.iter().map(|ty| ty.write_name(config));
            quote! { #namespace #name < #(#generics),* > }
        }
    }
}

impl std::fmt::Display for TypeName {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(fmt, "{}.{}", self.0, self.1)
    }
}
