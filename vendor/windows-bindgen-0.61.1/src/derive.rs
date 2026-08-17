use super::*;

pub struct Derive(HashMap<TypeName, Vec<String>>);

impl Derive {
    pub fn new(reader: &Reader, types: &TypeMap, derive: &[&str]) -> Self {
        let mut map = HashMap::new();

        for derive in derive {
            let Some((name, derive)) = derive.split_once('=') else {
                panic!("`--derive` must be `<type name>=Comma,Separated,List");
            };

            let tn = get_type_name(reader, name);

            if !types.contains_key(&tn) {
                panic!("type not included: `{name}`");
            }

            let derive = derive
                .split(',')
                .filter(|&derive| (!derive.is_empty()))
                .map(|derive| derive.to_string())
                .collect();
            map.insert(tn, derive);
        }

        Self(map)
    }

    pub fn get(&self, type_name: TypeName) -> impl Iterator<Item = String> + '_ {
        self.0.get(&type_name).into_iter().flatten().cloned()
    }
}

fn get_type_name(reader: &Reader, path: &str) -> TypeName {
    if let Some((namespace, name)) = path.rsplit_once('.') {
        if let Some((namespace, types)) = reader.get_key_value(namespace) {
            if let Some((name, _)) = types.get_key_value(name) {
                return TypeName(namespace, name);
            }
        }
    } else {
        for (namespace, types) in reader.iter() {
            if let Some((name, _)) = types.get_key_value(path) {
                return TypeName(namespace, name);
            }
        }
    }

    panic!("type not found: `{path}`");
}
