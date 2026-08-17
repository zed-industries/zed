use super::*;

pub fn write_arches<R: HasAttributes>(row: R) -> TokenStream {
    let mut tokens = quote! {};

    if let Some(attribute) = row.find_attribute("SupportedArchitectureAttribute") {
        if let Some((_, Value::I32(value))) = attribute.args().first() {
            let mut arches = BTreeSet::new();

            if value & 1 == 1 {
                arches.insert("x86");
            }

            if value & 2 == 2 {
                arches.insert("x86_64");
                arches.insert("arm64ec");
            }

            if value & 4 == 4 {
                arches.insert("aarch64");
            }

            match arches.len() {
                0 => {}
                1 => tokens.combine(quote! { #[cfg(#(target_arch = #arches),*)] }),
                _ => tokens.combine(quote! { #[cfg(any(#(target_arch = #arches),*))] }),
            }
        }
    }

    tokens
}

#[derive(Default)]
pub struct Cfg {
    features: BTreeSet<&'static str>,
    deprecated: bool,
}

impl Cfg {
    pub fn new<R: HasAttributes>(row: R, dependencies: &TypeMap, config: &Config<'_>) -> Self {
        let features: BTreeSet<&'static str> = dependencies
            .keys()
            .filter_map(|tn| {
                if config.types.contains_key(tn) {
                    Some(tn.namespace())
                } else {
                    None
                }
            })
            .collect();

        Self {
            features,
            deprecated: row.has_attribute("DeprecatedAttribute"),
        }
    }

    pub fn difference<R: HasAttributes>(
        &self,
        row: R,
        dependencies: &TypeMap,
        config: &Config<'_>,
    ) -> Self {
        let mut difference = Self::new(row, dependencies, config);

        for feature in &self.features {
            difference.features.remove(feature);
        }

        difference.deprecated = !self.deprecated && difference.deprecated;
        difference
    }

    pub fn write(&self, config: &Config<'_>, not: bool) -> TokenStream {
        if !config.package {
            return quote! {};
        }

        let mut compact = BTreeSet::<&'static str>::new();

        for feature in self.features.iter().rev() {
            let mut keep = true;

            for compact in &compact {
                if namespace_starts_with(compact, feature) {
                    keep = false;
                    break;
                }
            }

            if keep {
                compact.insert(feature);
            }
        }

        let mut features = BTreeSet::new();

        if self.deprecated {
            features.insert("deprecated".to_string());
        }

        for dependency in compact {
            if dependency.is_empty()
                || namespace_starts_with(config.namespace, dependency)
                || dependency == "Windows.Foundation"
                || dependency == "Windows.Win32.Foundation"
            {
                continue;
            }

            let mut feature = String::new();

            for name in dependency.split('.').skip(1) {
                feature.push_str(name);
                feature.push('_');
            }

            feature.truncate(feature.len() - 1);
            features.insert(feature);
        }

        let mut tokens = quote! {};

        match features.len() {
            0 => {}
            1 => {
                if not {
                    tokens.combine(quote! { #[cfg(not(#(feature = #features)*))] });
                } else {
                    tokens.combine(quote! { #[cfg(#(feature = #features)*)] });
                }
            }
            _ => {
                if not {
                    tokens.combine(quote! { #[cfg(not(all( #(feature = #features),* )))] });
                } else {
                    tokens.combine(quote! { #[cfg(all( #(feature = #features),* ))] });
                }
            }
        }

        tokens
    }
}
