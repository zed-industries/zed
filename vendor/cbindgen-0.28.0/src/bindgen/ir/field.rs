use syn::ext::IdentExt;

use crate::bindgen::ir::{AnnotationSet, Cfg};
use crate::bindgen::ir::{Documentation, Path, Type};

#[derive(Debug, Clone)]
pub struct Field {
    pub name: String,
    pub ty: Type,
    pub cfg: Option<Cfg>,
    pub annotations: AnnotationSet,
    pub documentation: Documentation,
}

impl Field {
    pub fn from_name_and_type(name: String, ty: Type) -> Field {
        Field {
            name,
            ty,
            cfg: None,
            annotations: AnnotationSet::new(),
            documentation: Documentation::none(),
        }
    }

    pub fn load(field: &syn::Field, self_path: &Path) -> Result<Option<Field>, String> {
        Ok(if let Some(mut ty) = Type::load(&field.ty)? {
            ty.replace_self_with(self_path);
            Some(Field {
                name: field
                    .ident
                    .as_ref()
                    .ok_or_else(|| "field is missing identifier".to_string())?
                    .unraw()
                    .to_string(),
                ty,
                cfg: Cfg::load(&field.attrs),
                annotations: AnnotationSet::load(&field.attrs)?,
                documentation: Documentation::load(&field.attrs),
            })
        } else {
            None
        })
    }
}
