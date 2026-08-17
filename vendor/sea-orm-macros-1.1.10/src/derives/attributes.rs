pub mod derive_attr {
    use bae::FromAttributes;

    /// Attributes for Models and ActiveModels
    #[derive(Default, FromAttributes)]
    #[allow(dead_code)]
    pub struct SeaOrm {
        pub column: Option<syn::Ident>,
        pub entity: Option<syn::Ident>,
        pub model: Option<syn::Ident>,
        pub active_model: Option<syn::Ident>,
        pub primary_key: Option<syn::Ident>,
        pub relation: Option<syn::Ident>,
        pub schema_name: Option<syn::Lit>,
        pub table_name: Option<syn::Lit>,
        pub comment: Option<syn::Lit>,
        pub table_iden: Option<()>,
        pub rename_all: Option<syn::Lit>,
    }
}

pub mod field_attr {
    use bae::FromAttributes;

    /// Operations for Models and ActiveModels
    #[derive(Default, FromAttributes)]
    pub struct SeaOrm {
        pub belongs_to: Option<syn::Lit>,
        pub has_one: Option<syn::Lit>,
        pub has_many: Option<syn::Lit>,
        pub on_update: Option<syn::Lit>,
        pub on_delete: Option<syn::Lit>,
        pub on_condition: Option<syn::Lit>,
        pub from: Option<syn::Lit>,
        pub to: Option<syn::Lit>,
        pub fk_name: Option<syn::Lit>,
        pub condition_type: Option<syn::Lit>,
    }
}

#[cfg(feature = "seaography")]
pub mod related_attr {
    use bae::FromAttributes;

    /// Operations for RelatedEntity enumeration
    #[derive(Default, FromAttributes)]
    pub struct SeaOrm {
        ///
        /// Allows to modify target entity
        ///
        /// Required on enumeration variants
        ///
        /// If used on enumeration attributes
        /// it allows to specify different
        /// Entity ident
        pub entity: Option<syn::Lit>,
        ///
        /// Allows to specify RelationDef
        ///
        /// Optional
        ///
        /// If not supplied the generated code
        /// will utilize `impl Related` trait
        pub def: Option<syn::Lit>,
    }
}
