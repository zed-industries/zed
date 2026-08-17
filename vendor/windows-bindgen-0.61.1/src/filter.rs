use super::*;

#[derive(Debug, Default)]
pub struct Filter(Vec<(String, bool)>);

impl Filter {
    #[track_caller]
    pub fn new(reader: &Reader, include: &[&str], exclude: &[&str]) -> Self {
        let mut rules = vec![];

        for filter in include {
            push_filter(reader, &mut rules, filter, true);
        }

        for filter in exclude {
            push_filter(reader, &mut rules, filter, false)
        }

        debug_assert!(!rules.is_empty());

        rules.sort_unstable_by(|left, right| {
            let left = (left.0.len(), !left.1);
            let right = (right.0.len(), !right.1);
            left.cmp(&right).reverse()
        });

        Self(rules)
    }

    pub fn includes_namespace(&self, namespace: &str) -> bool {
        for rule in &self.0 {
            if rule.1 {
                // include
                if namespace_starts_with(&rule.0, namespace) {
                    return true;
                }
                if namespace_starts_with(namespace, &rule.0) {
                    return true;
                }
            } else {
                // exclude
                if namespace_starts_with(namespace, &rule.0) {
                    return false;
                }
            }
        }

        false
    }

    pub fn includes_type_name(&self, name: TypeName) -> bool {
        for rule in &self.0 {
            if match_type_name(&rule.0, name.namespace(), name.name()) {
                return rule.1;
            }
        }

        false
    }

    pub fn excludes_type_name(&self, name: TypeName) -> bool {
        for rule in &self.0 {
            if match_type_name(&rule.0, name.namespace(), name.name()) {
                return !rule.1;
            }
        }

        false
    }
}

#[track_caller]
fn push_filter(reader: &Reader, rules: &mut Vec<(String, bool)>, filter: &str, include: bool) {
    if reader.contains_key(filter) {
        rules.push((filter.to_string(), include));
        return;
    }

    if let Some((namespace, name)) = filter.rsplit_once('.') {
        if reader.with_full_name(namespace, name).next().is_some() {
            rules.push((filter.to_string(), include));
            return;
        }

        if let Some(starts_with) = name.strip_suffix('*') {
            if let Some(types) = reader.get(namespace) {
                let prev_len = rules.len();

                for name in types.keys() {
                    if name.starts_with(starts_with) {
                        rules.push((format!("{namespace}.{name}"), include));
                    }
                }

                if prev_len != rules.len() {
                    return;
                }
            }
        }
    }

    let mut pushed = false;

    for (namespace, types) in reader.iter() {
        if types.get(filter).is_some() {
            rules.push((format!("{namespace}.{filter}"), include));
            pushed = true;
        }
    }

    if pushed {
        return;
    }

    if reader
        .keys()
        .any(|namespace| namespace_starts_with(namespace, filter))
    {
        rules.push((filter.to_string(), include));
        return;
    }

    panic!("type not found: `{filter}`");
}

fn match_type_name(rule: &str, namespace: &str, name: &str) -> bool {
    if rule.len() <= namespace.len() {
        return namespace.starts_with(rule);
    }

    if !rule.starts_with(namespace) {
        return false;
    }

    if rule.as_bytes()[namespace.len()] != b'.' {
        return false;
    }

    name == &rule[namespace.len() + 1..]
}
