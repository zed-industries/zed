use collections::HashMap;
use dap_adapters::JsDebugAdapter;
use schemars::{Schema, json_schema};
use serde::{Deserialize, Serialize};
use task::{EnvVariableReplacer, VariableName};
use tempfile::TempDir;

fn main() {
    #[derive(Serialize, Deserialize)]
    struct PackageJsonConfigurationAttributes {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        launch: Option<Schema>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        attach: Option<Schema>,
    }

    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct PackageJsonDebugger {
        r#type: String,
        configuration_attributes: PackageJsonConfigurationAttributes,
    }

    #[derive(Serialize, Deserialize)]
    struct PackageJsonContributes {
        debuggers: Vec<PackageJsonDebugger>,
    }

    #[derive(Serialize, Deserialize)]
    struct PackageJson {
        contributes: PackageJsonContributes,
    }

    let dir = TempDir::new().unwrap();
    let path = std::fs::canonicalize(dir.path()).unwrap();
    let (package_json, package_nls_json) =
        dap_adapters::JsDebugAdapter::fetch_schema(&path).unwrap();
    let package_nls_json =
        serde_json::from_str::<HashMap<String, serde_json::Value>>(&package_nls_json)
            .unwrap()
            .into_iter()
            .filter_map(|(k, v)| {
                let v = v.as_str()?;
                Some((k, v.to_owned()))
            })
            .collect::<HashMap<_, _>>();

    let package_json: serde_json::Value = serde_json::from_str(&package_json).unwrap();

    struct Replacer {
        package_nls_json: HashMap<String, String>,
        env: EnvVariableReplacer,
    }

    impl Replacer {
        fn replace(&self, input: serde_json::Value) -> serde_json::Value {
            match input {
                serde_json::Value::String(s) => {
                    if s.starts_with("%") && s.ends_with("%") {
                        self.package_nls_json
                            .get(s.trim_matches('%'))
                            .map(|s| s.as_str().into())
                            .unwrap_or("(missing)".into())
                    } else {
                        self.env.replace(&s).into()
                    }
                }
                serde_json::Value::Array(arr) => {
                    serde_json::Value::Array(arr.into_iter().map(|v| self.replace(v)).collect())
                }
                serde_json::Value::Object(obj) => serde_json::Value::Object(
                    obj.into_iter().map(|(k, v)| (k, self.replace(v))).collect(),
                ),
                _ => input,
            }
        }
    }

    let env = EnvVariableReplacer::new(HashMap::from_iter([(
        "workspaceFolder".to_owned(),
        VariableName::WorktreeRoot.to_string(),
    )]));
    let replacer = Replacer {
        env,
        package_nls_json,
    };
    let package_json = replacer.replace(package_json);

    let package_json: PackageJson = serde_json::from_value(package_json).unwrap();

    let alternatives = package_json
        .contributes
        .debuggers
        .into_iter()
        .flat_map(|debugger| {
            let r#type = debugger.r#type;
            let configuration_attributes = debugger.configuration_attributes;
            configuration_attributes
                .launch
                .map(|schema| ("launch", schema))
                .into_iter()
                .chain(
                    configuration_attributes
                        .attach
                        .map(|schema| ("attach", schema)),
                )
                .map(|(request, schema)| {
                    json_schema!({
                        "if": {
                            "properties": {
                                "type": {
                                    "enum": [r#type]
                                },
                                "request": {
                                    "enum": [request]
                                }
                            },
                            "required": ["type", "request"]
                        },
                        "then": schema
                    })
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let schema = json_schema!({
        "allOf": alternatives
    });

    let mut schema = serde_json::to_string_pretty(&schema.to_value()).unwrap();
    schema.push('\n');
    std::fs::write(
        format!(
            "crates/dap_adapters/schemas/{}.json",
            JsDebugAdapter::ADAPTER_NAME
        ),
        schema,
    )
    .unwrap();
}
