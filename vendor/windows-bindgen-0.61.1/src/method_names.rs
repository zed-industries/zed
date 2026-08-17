use super::*;

#[derive(Default)]
pub struct MethodNames(HashMap<String, u32>);

impl MethodNames {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn add(&mut self, method: MethodDef) -> TokenStream {
        let name = method_def_special_name(method);
        let overload = self.0.entry(name.to_string()).or_insert(0);
        *overload += 1;
        if *overload > 1 {
            format!("{name}{overload}").into()
        } else {
            to_ident(&name)
        }
    }
}

fn method_def_special_name(row: MethodDef) -> String {
    let name = row.name();
    if row.flags().contains(MethodAttributes::SpecialName) {
        if name.starts_with("get") {
            name[4..].to_string()
        } else if name.starts_with("put") {
            format!("Set{}", &name[4..])
        } else if name.starts_with("add") {
            name[4..].to_string()
        } else if name.starts_with("remove") {
            format!("Remove{}", &name[7..])
        } else {
            name.to_string()
        }
    } else {
        if let Some(attribute) = row.find_attribute("OverloadAttribute") {
            for (_, arg) in attribute.args() {
                if let Value::Str(overload) = arg {
                    // Detect generated overload and revert back to original name.
                    if let Some(suffix) = overload.strip_prefix(name) {
                        if suffix.parse::<u32>().is_ok() {
                            return name.to_string();
                        }
                    }

                    return overload.to_string();
                }
            }
        }
        name.to_string()
    }
}
