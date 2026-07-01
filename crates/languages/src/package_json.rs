use chrono::{DateTime, Local};
use collections::{BTreeSet, HashMap};
use serde_json_lenient::Value;
use std::{path::Path, sync::Arc};

#[derive(Clone, Debug)]
pub struct PackageJson {
    pub mtime: DateTime<Local>,
    pub data: PackageJsonData,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct PackageJsonData {
    pub scripts: BTreeSet<(Arc<Path>, String)>,
    pub package_manager: Option<&'static str>,
}

impl PackageJsonData {
    pub fn new(path: Arc<Path>, package_json: HashMap<String, Value>) -> Self {
        let mut scripts = BTreeSet::new();
        if let Some(Value::Object(package_json_scripts)) = package_json.get("scripts") {
            scripts.extend(
                package_json_scripts
                    .keys()
                    .cloned()
                    .map(|name| (path.clone(), name)),
            );
        }

        let package_manager = package_manager_from_package_json(&package_json);

        Self {
            scripts,
            package_manager,
        }
    }

    pub fn merge(&mut self, other: Self) {
        self.scripts.extend(other.scripts);
        self.package_manager = self.package_manager.or(other.package_manager);
    }
}

fn package_manager_from_package_json(
    package_json: &HashMap<String, Value>,
) -> Option<&'static str> {
    package_json
        .get("packageManager")
        .and_then(|value| value.as_str())
        .and_then(package_manager_name)
        .or_else(|| {
            package_json
                .get("devEngines")
                .and_then(|value| value.as_object())
                .and_then(|dev_engines| dev_engines.get("packageManager"))
                .and_then(package_manager_from_dev_engine)
        })
}

fn package_manager_from_dev_engine(value: &Value) -> Option<&'static str> {
    match value {
        Value::Object(package_manager) => package_manager
            .get("name")
            .and_then(|name| name.as_str())
            .and_then(package_manager_name),
        Value::Array(package_managers) => package_managers
            .iter()
            .find_map(package_manager_from_dev_engine),
        _ => None,
    }
}

fn package_manager_name(value: &str) -> Option<&'static str> {
    let value = value.split_once('@').map(|(name, _)| name).unwrap_or(value);
    match value {
        "pnpm" => Some("pnpm"),
        "yarn" => Some("yarn"),
        "npm" => Some("npm"),
        "bun" => Some("bun"),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::PackageJsonData;

    #[test]
    fn package_manager_detection() {
        fn package_manager(source: &str) -> Option<&'static str> {
            PackageJsonData::new(
                Path::new("/root/package.json").into(),
                serde_json_lenient::from_str(source).expect("provided source should be valid JSON"),
            )
            .package_manager
        }

        assert_eq!(
            package_manager(r#"{"packageManager": "pnpm@11.1.3"}"#),
            Some("pnpm")
        );

        assert_eq!(
            package_manager(
                r#"{"devEngines": {"packageManager": {"name": "pnpm", "version": "^11.1.3", "onFail": "download"}}}"#
            ),
            Some("pnpm"),
        );

        assert_eq!(
            package_manager(
                r#"{"devEngines": {"packageManager": [{"name": "foo"}, {"name": "yarn", "version": "^4.0.0"}]}}"#
            ),
            Some("yarn"),
        );

        assert_eq!(
            package_manager(
                r#"{"packageManager": "npm@10.0.0", "devEngines": {"packageManager": {"name": "pnpm"}}}"#
            ),
            Some("npm"),
        );

        assert_eq!(
            package_manager(r#"{"devEngines": {"packageManager": {"version": "^11.1.3"}}}"#),
            None,
        );
    }
}
