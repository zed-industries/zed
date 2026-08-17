use super::*;
use serde::Serialize;

#[derive(Default, Serialize)]
struct Index {
    version: u8,
    namespaces: Vec<String>,
    items: BTreeMap<usize, Vec<IndexItem>>,
}

#[derive(Default, Serialize)]
struct IndexItem {
    #[serde(rename = "n")]
    name: String,
    #[serde(rename = "f")]
    features: Vec<usize>,
}

impl Index {
    fn new() -> Self {
        Self {
            version: 2,
            ..Default::default()
        }
    }

    fn get_or_create_namespace_index(&mut self, namespace: &str) -> usize {
        match self.namespaces.iter().position(|ns| ns == namespace) {
            Some(idx) => idx,
            None => {
                self.namespaces.push(namespace.to_string());
                self.namespaces.len() - 1
            }
        }
    }

    fn add_item(&mut self, namespace: &str, name: &str, features: BTreeSet<String>) {
        let namespace_idx = self.get_or_create_namespace_index(namespace);

        let mut compact = BTreeSet::new();
        for feature in features.iter().rev() {
            if feature.is_empty() {
                continue;
            }

            if !compact
                .iter()
                .any(|c: &&str| namespace_starts_with(c, feature))
            {
                compact.insert(feature.as_str());
            }
        }

        let features: Vec<usize> = compact
            .iter()
            .map(|dep| self.get_or_create_namespace_index(dep))
            .collect();
        let items = self.items.entry(namespace_idx).or_default();

        if let Some(existing_item) = items.iter_mut().find(|item| item.name == name) {
            for &feature in &features {
                if !existing_item.features.contains(&feature) {
                    existing_item.features.push(feature);
                }
            }
            existing_item.features.sort();
            return;
        }

        let index_item = IndexItem {
            name: name.to_string(),
            features,
        };

        items.push(index_item);
    }
}

const EXCLUDED_NAMESPACES: &[&str] = &["Windows.Foundation", "Windows.Win32.Foundation"];

#[doc(hidden)]
pub fn write(types: &TypeMap, output: &str) {
    let mut feature_index = Index::new();
    let mut all_types: Vec<_> = types.values().flatten().collect();

    all_types.sort_by(|a, b| {
        let a_name = a.type_name();
        let b_name = b.type_name();
        (a_name.namespace(), a_name.name()).cmp(&(b_name.namespace(), b_name.name()))
    });

    for ty in all_types {
        let type_deps = ty.dependencies();
        let type_name = ty.type_name();

        let namespace = type_name.namespace();
        if namespace.is_empty() {
            continue;
        }

        let features: BTreeSet<String> = type_deps
            .keys()
            .filter(|tn| !EXCLUDED_NAMESPACES.contains(&tn.namespace()))
            .map(|tn| tn.namespace().to_string())
            .chain(std::iter::once(namespace.to_string()))
            .collect();

        feature_index.add_item(namespace, type_name.name(), features);

        let methods = match ty {
            Type::CppInterface(ty) => {
                let interface_name = ty.def.name();
                Some((ty.def.methods(), interface_name))
            }
            Type::Interface(ty) => {
                let interface_name = ty.def.name();
                Some((ty.def.methods(), interface_name))
            }
            _ => None,
        };

        if let Some((methods, interface_name)) = methods {
            for method in methods {
                let method_deps = method.signature(namespace, &[]).dependencies();
                let method_features: BTreeSet<String> = method_deps
                    .keys()
                    .filter(|tn| !EXCLUDED_NAMESPACES.contains(&tn.namespace()))
                    .map(|tn| tn.namespace().to_string())
                    .chain(std::iter::once(namespace.to_string()))
                    .collect();

                let scoped_name = format!("{}.{}", interface_name, method.name());
                feature_index.add_item(namespace, &scoped_name, method_features);
            }
        }
    }

    write_to_file(output, serde_json::to_string(&feature_index).unwrap());
}
