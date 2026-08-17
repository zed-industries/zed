use super::*;

#[derive(Default, Clone, Debug, PartialEq, Eq)]
pub struct TypeMap(HashMap<TypeName, HashSet<Type>>);

pub trait Dependencies {
    fn combine(&self, dependencies: &mut TypeMap);

    fn dependencies(&self) -> TypeMap {
        let mut dependencies = TypeMap::new();
        self.combine(&mut dependencies);
        dependencies
    }
}

impl std::ops::Deref for TypeMap {
    type Target = HashMap<TypeName, HashSet<Type>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl TypeMap {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    #[track_caller]
    pub fn filter(reader: &Reader, filter: &Filter, references: &References) -> Self {
        let mut dependencies = Self::new();

        for namespace in reader.keys() {
            if filter.includes_namespace(namespace) {
                for (name, types) in &reader[namespace] {
                    if filter.includes_type_name(TypeName(namespace, name)) {
                        let mut item_dependencies = Self::new();

                        for ty in types {
                            ty.combine(&mut item_dependencies);
                        }

                        if item_dependencies.excluded(filter, references) {
                            continue;
                        }

                        for ty in types {
                            dependencies.insert(ty.clone());
                        }

                        dependencies.combine_references(&item_dependencies, references);
                    }
                }
            }
        }

        dependencies
    }

    pub fn insert(&mut self, ty: Type) -> bool {
        self.0.entry(ty.type_name()).or_default().insert(ty)
    }

    fn combine_references(&mut self, other: &Self, references: &References) {
        for (tn, types) in &other.0 {
            if references.contains(*tn).is_none() {
                let set = self.0.entry(*tn).or_default();
                types.iter().for_each(|ty| {
                    set.insert(ty.clone());
                });
            }
        }
    }

    pub fn combine(&mut self, other: &Self) {
        for (name, types) in &other.0 {
            let set = self.0.entry(*name).or_default();
            types.iter().for_each(|ty| {
                set.insert(ty.clone());
            });
        }
    }

    pub fn included(&self, config: &Config<'_>) -> bool {
        self.0.iter().all(|(tn, _)| {
            // An empty namespace covers core types like `HRESULT`. This way we don't exclude methods
            // that depend on core types that aren't explicitly included in the filter.
            if tn.namespace().is_empty() {
                return true;
            }

            if config.types.contains_key(tn) {
                return true;
            }

            if config.references.contains(*tn).is_some() {
                return true;
            }

            false
        })
    }

    fn excluded(&self, filter: &Filter, references: &References) -> bool {
        self.0
            .iter()
            .any(|(tn, _)| filter.excludes_type_name(*tn) && references.contains(*tn).is_none())
    }
}
