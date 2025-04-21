use anyhow::anyhow;
use collections::HashMap;
use serde::Deserialize;
use util::ResultExt as _;

use crate::{
    AttachConfig, DebugArgs, DebugArgsRequest, EnvVariableReplacer, TCPHost, TaskTemplate,
    TaskTemplates, TaskType, VariableName,
};

#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
enum Request {
    Launch,
    Attach,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
struct VsCodeDebugTaskDefinition {
    r#type: String,
    name: String,
    request: Request,

    #[serde(default)]
    program: Option<String>,
    #[serde(default)]
    args: Vec<String>,
    #[serde(default)]
    env: HashMap<String, Option<String>>,
    // TODO envFile?
    #[serde(default)]
    cwd: Option<String>,
    #[serde(default)]
    port: Option<u16>,
    #[serde(default)]
    stop_on_entry: Option<bool>,

    #[serde(flatten)]
    other_attributes: HashMap<String, serde_json_lenient::Value>,
}

impl VsCodeDebugTaskDefinition {
    fn try_to_zed(self, replacer: &EnvVariableReplacer) -> anyhow::Result<TaskTemplate> {
        let label = replacer.replace(&self.name);
        let command = self
            .program
            .ok_or_else(|| anyhow!("debug task configuration does not define a program"))?;
        let command = replacer.replace(&command);
        let args = self
            .args
            .into_iter()
            .map(|arg| replacer.replace(&arg))
            .collect();
        let cwd = self.cwd.map(|cwd| replacer.replace(&cwd));
        // TODO should we replace variables in other things?
        let template = TaskTemplate {
            label,
            command,
            args,
            cwd,
            // TODO support unsetting envs like vscode
            env: self
                .env
                .into_iter()
                .filter_map(|(k, v)| Some((k, v?)))
                .collect(),
            task_type: TaskType::Debug(DebugArgs {
                request: match self.request {
                    Request::Launch => DebugArgsRequest::Launch,
                    Request::Attach => DebugArgsRequest::Attach(AttachConfig { process_id: None }),
                },
                adapter: task_type_to_adapter_name(self.r#type),
                // TODO host?
                tcp_connection: self.port.map(|port| TCPHost {
                    port: Some(port),
                    host: None,
                    timeout: None,
                }),
                stop_on_entry: self.stop_on_entry,
                // TODO
                initialize_args: None,
                locator: None,
            }),
            ..Default::default()
        };
        Ok(template)
    }
}

/// blah
#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct VsCodeDebugTaskFile {
    version: String,
    configurations: Vec<VsCodeDebugTaskDefinition>,
}

impl TryFrom<VsCodeDebugTaskFile> for TaskTemplates {
    type Error = anyhow::Error;

    fn try_from(file: VsCodeDebugTaskFile) -> Result<Self, Self::Error> {
        let replacer = EnvVariableReplacer::new(HashMap::from_iter([
            (
                "workspaceFolder".to_owned(),
                VariableName::WorktreeRoot.to_string(),
            ),
            ("file".to_owned(), VariableName::File.to_string()),
            // TODO other interesting variables?
        ]));
        let templates = file
            .configurations
            .into_iter()
            .filter_map(|config| config.try_to_zed(&replacer).log_err())
            .collect::<Vec<_>>();
        Ok(TaskTemplates(templates))
    }
}

// FIXME figure out how to make JsDebugAdapter::ADAPTER_NAME et al available here
fn task_type_to_adapter_name(task_type: String) -> String {
    match task_type.as_str() {
        "node" => "JavaScript".to_owned(),
        "go" => "Delve".to_owned(),
        "php" => "PHP".to_owned(),
        // TODO figure out appropriate names for the other built-in debug adapters
        _ => task_type,
    }
}
