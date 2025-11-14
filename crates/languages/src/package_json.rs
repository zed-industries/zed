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
    pub jest_package_path: Option<Arc<Path>>,
    pub mocha_package_path: Option<Arc<Path>>,
    pub vitest_package_path: Option<Arc<Path>>,
    pub jasmine_package_path: Option<Arc<Path>>,
    pub bun_package_path: Option<Arc<Path>>,
    pub node_package_path: Option<Arc<Path>>,
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

        let mut jest_package_path = None;
        let mut mocha_package_path = None;
        let mut vitest_package_path = None;
        let mut jasmine_package_path = None;
        let mut bun_package_path = None;
        let mut node_package_path = None;
        if let Some(Value::Object(dependencies)) = package_json.get("devDependencies") {
            if dependencies.contains_key("jest") {
                jest_package_path.get_or_insert_with(|| path.clone());
            }
            if dependencies.contains_key("mocha") {
                mocha_package_path.get_or_insert_with(|| path.clone());
            }
            if dependencies.contains_key("vitest") {
                vitest_package_path.get_or_insert_with(|| path.clone());
            }
            if dependencies.contains_key("jasmine") {
                jasmine_package_path.get_or_insert_with(|| path.clone());
            }
            if dependencies.contains_key("@types/bun") {
                bun_package_path.get_or_insert_with(|| path.clone());
            }
            if dependencies.contains_key("@types/node") {
                node_package_path.get_or_insert_with(|| path.clone());
            }
        }
        if let Some(Value::Object(dev_dependencies)) = package_json.get("dependencies") {
            if dev_dependencies.contains_key("jest") {
                jest_package_path.get_or_insert_with(|| path.clone());
            }
            if dev_dependencies.contains_key("mocha") {
                mocha_package_path.get_or_insert_with(|| path.clone());
            }
            if dev_dependencies.contains_key("vitest") {
                vitest_package_path.get_or_insert_with(|| path.clone());
            }
            if dev_dependencies.contains_key("jasmine") {
                jasmine_package_path.get_or_insert_with(|| path.clone());
            }
            if dev_dependencies.contains_key("@types/bun") {
                bun_package_path.get_or_insert_with(|| path.clone());
            }
            if dev_dependencies.contains_key("@types/node") {
                node_package_path.get_or_insert_with(|| path.clone());
            }
        }

        let package_manager = package_json
            .get("packageManager")
            .and_then(|value| value.as_str())
            .and_then(|value| {
                if value.starts_with("pnpm") {
                    Some("pnpm")
                } else if value.starts_with("yarn") {
                    Some("yarn")
                } else if value.starts_with("npm") {
                    Some("npm")
                } else if value.starts_with("bun") {
                    Some("bun")
                } else {
                    None
                }
            });

        Self {
            jest_package_path,
            mocha_package_path,
            vitest_package_path,
            jasmine_package_path,
            bun_package_path,
            node_package_path,
            scripts,
            package_manager,
        }
    }

    pub fn merge(&mut self, other: Self) {
        self.jest_package_path = self.jest_package_path.take().or(other.jest_package_path);
        self.mocha_package_path = self.mocha_package_path.take().or(other.mocha_package_path);
        self.vitest_package_path = self
            .vitest_package_path
            .take()
            .or(other.vitest_package_path);
        self.jasmine_package_path = self
            .jasmine_package_path
            .take()
            .or(other.jasmine_package_path);
        self.bun_package_path = self.bun_package_path.take().or(other.bun_package_path);
        self.node_package_path = self.node_package_path.take().or(other.node_package_path);
        self.scripts.extend(other.scripts);
        self.package_manager = self.package_manager.or(other.package_manager);
    }
}
