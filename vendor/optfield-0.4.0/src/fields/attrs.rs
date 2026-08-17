use syn::{Attribute, Field};

use crate::args::{Args, Attrs};
use crate::attrs::generator::AttrGenerator;

struct FieldAttrGen<'a> {
    field: &'a Field,
    args: &'a Args,
}

impl<'a> FieldAttrGen<'a> {
    fn new(field: &'a Field, args: &'a Args) -> Self {
        Self { field, args }
    }
}

impl AttrGenerator for FieldAttrGen<'_> {
    fn no_docs(&self) -> bool {
        !self.args.field_doc
    }

    fn error_action_text(&self) -> String {
        let field_name = match &self.field.ident {
            None => "".to_string(),
            Some(ident) => format!("{} ", ident),
        };

        format!("generating {}field attrs", field_name)
    }

    fn original_attrs(&self) -> &[Attribute] {
        &self.field.attrs
    }

    fn attrs_arg(&self) -> &Option<Attrs> {
        &self.args.field_attrs
    }
}

pub fn generate(field: &Field, args: &Args) -> Vec<Attribute> {
    FieldAttrGen::new(field, args).generate()
}

#[cfg(test)]
mod tests {
    use super::*;

    use quote::quote;

    use crate::test_util::*;

    #[test]
    fn remove_docs() {
        let field = parse_field(quote! {
            /// some
            /// doc
            /// lines
            #[attr]
            field: i32
        });

        let cases = vec![
            quote! {
                Opt

            },
            quote! {
                Opt,
                field_attrs
            },
            quote! {
                Opt,
                field_attrs = (new_attr)
            },
            quote! {
                Opt,
                field_attrs = add(new_attr)
            },
        ];

        let item_docs = doc_attrs(&field.attrs);

        for case in cases {
            let args = parse_args(case);

            let generated = generate(&field, &args);

            assert!(!attrs_contain_any(&generated, &item_docs));
        }
    }

    #[test]
    fn keep_docs() {
        let field = parse_field(quote! {
            /// field
            /// with
            /// docs
            #[attr]
            field: String
        });

        let cases = vec![
            quote! {
                Opt,
                field_doc
            },
            quote! {
                Opt,
                field_doc,
                field_attrs
            },
            quote! {
                Opt,
                field_doc,
                field_attrs = (new_attr)
            },
            quote! {
                Opt,
                field_doc,
                field_attrs = add(new_attr)
            },
        ];

        let item_docs = doc_attrs(&field.attrs);

        for case in cases {
            let args = parse_args(case);

            let generated = generate(&field, &args);

            assert!(attrs_contain_all(&generated, &item_docs));
        }
    }

    #[test]
    fn remove_attrs() {
        let (field, args) = parse_field_and_args(
            quote! {
                #[some]
                #[attrs]
                field: String
            },
            quote! {
                Opt
            },
        );

        let generated = generate(&field, &args);

        assert!(!attrs_contain_any(&generated, &field.attrs));
    }

    #[test]
    fn keep_attrs() {
        let (field, args) = parse_field_and_args(
            quote! {
                #[some]
                #[attrs]
                field: String
            },
            quote! {
                Opt,
                field_attrs
            },
        );

        let generated = generate(&field, &args);

        assert!(attrs_contain_all(&generated, &field.attrs));
    }

    #[test]
    fn replace_attrs() {
        let (field, args) = parse_field_and_args(
            quote! {
                #[some]
                #[attrs]
                field: String
            },
            quote! {
                Opt,
                field_attrs = (
                    other,
                    new,
                    attribute
                )
            },
        );

        let new_attrs = parse_attrs(quote! {
            #[other]
            #[new]
            #[attribute]
        });

        let generated = generate(&field, &args);

        assert!(!attrs_contain_any(&generated, &field.attrs));
        assert!(attrs_contain_all(&generated, &new_attrs));
    }

    #[test]
    fn add_attrs() {
        let (field, args) = parse_field_and_args(
            quote! {
                #[old]
                #[attrs]
                field: u8
            },
            quote! {
                Opt,
                field_attrs = add(
                    new,
                    field,
                    attributes
                )
            },
        );

        let new_attrs = parse_attrs(quote! {
            #[new]
            #[field]
            #[attributes]
        });

        let generated = generate(&field, &args);

        assert!(attrs_contain_all(&generated, &field.attrs));
        assert!(attrs_contain_all(&generated, &new_attrs));
    }
}
