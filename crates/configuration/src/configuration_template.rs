use anyhow::{Context as _, bail};
use collections::{HashMap, HashSet};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::path::PathBuf;
use task::{
    TaskContext, VariableName,
    ZED_VARIABLE_NAME_PREFIX,
};
use util::schemars::{AllowTrailingCommas, DefaultDenyUnknownFields};
use util::{ResultExt, truncate_and_remove_front};

use crate::{ConfigurationContext, ConfigurationId, ExecutionConfig, ResolvedConfiguration};

/// Type of configuration - determines how it should be executed
#[derive(Clone, Copy, Default, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ConfigurationType {
    /// A normal run configuration
    #[default]
    Run,
    /// A debug configuration (will integrate with DAP)
    Debug,
}

/// A configuration template that can be resolved with a context to create an executable configuration.
/// Similar to JetBrains run configurations - defines how to execute a program with template variables.
#[derive(Clone, Default, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct ConfigurationTemplate {
    /// Human-readable name of the configuration
    pub label: String,
    
    /// Type of configuration (run or debug)
    #[serde(default)]
    pub config_type: ConfigurationType,
    
    /// The executable command to run
    pub command: String,
    
    /// Arguments to pass to the command
    #[serde(default)]
    pub args: Vec<String>,
    
    /// Working directory for execution (defaults to project root)
    #[serde(default)]
    pub cwd: Option<String>,
    
    /// Environment variables to set
    #[serde(default)]
    pub env: HashMap<String, String>,
    
    /// For NPM/Node.js configurations - npm script to run
    #[serde(default)]
    pub npm_script: Option<String>,
    
    /// For test configurations - test filter/pattern
    #[serde(default)]
    pub test_filter: Option<String>,
    
    /// For test configurations - test harness to use
    #[serde(default)]
    pub test_harness: Option<String>,
    
    /// Tags for categorizing configurations
    #[serde(default)]
    pub tags: Vec<String>,
}

/// A collection of configuration templates
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct ConfigurationTemplates(pub Vec<ConfigurationTemplate>);

impl ConfigurationTemplates {
    /// Generates JSON schema for the configuration file format
    pub fn generate_json_schema() -> serde_json::Value {
        let schema = schemars::generate::SchemaSettings::draft2019_09()
            .with_transform(DefaultDenyUnknownFields)
            .with_transform(AllowTrailingCommas)
            .into_generator()
            .root_schema_for::<Self>();

        serde_json::to_value(schema).unwrap()
    }
}

const MAX_DISPLAY_VARIABLE_LENGTH: usize = 15;

impl ConfigurationTemplate {
    /// Resolves the template with the given context to create a ResolvedConfiguration
    pub fn resolve(&self, id_base: &str, cx: &ConfigurationContext) -> Option<ResolvedConfiguration> {
        if self.label.trim().is_empty() || self.command.trim().is_empty() {
            return None;
        }

        let mut variable_names = HashMap::default();
        let mut substituted_variables = HashSet::default();
        
        // Convert to TaskContext for variable substitution
        let task_cx: TaskContext = cx.clone().into();
        
        let task_variables = task_cx
            .task_variables
            .iter()
            .map(|(key, value)| {
                let key_string = key.to_string();
                if !variable_names.contains_key(&key_string) {
                    variable_names.insert(key_string.clone(), key.clone());
                }
                (key_string, value.as_str())
            })
            .collect::<HashMap<_, _>>();

        let truncated_variables = truncate_variables(&task_variables);

        // Resolve cwd
        let cwd = match self.cwd.as_deref() {
            Some(cwd) => {
                let substituted_cwd = substitute_all_template_variables_in_str(
                    cwd,
                    &task_variables,
                    &variable_names,
                    &mut substituted_variables,
                )?;
                Some(PathBuf::from(substituted_cwd))
            }
            None => None,
        }
        .or(cx.cwd.clone());

        // Resolve label
        let full_label = substitute_all_template_variables_in_str(
            &self.label,
            &task_variables,
            &variable_names,
            &mut substituted_variables,
        )?;

        const TRUNCATION_THRESHOLD: usize = 64;

        let human_readable_label = if full_label.len() > TRUNCATION_THRESHOLD {
            substitute_all_template_variables_in_str(
                &self.label,
                &truncated_variables,
                &variable_names,
                &mut substituted_variables,
            )?
        } else {
            #[allow(
                clippy::redundant_clone,
                reason = "We want to clone the full_label to avoid borrowing it in the fold closure"
            )]
            full_label.clone()
        }
        .lines()
        .fold(String::new(), |mut string, line| {
            if string.is_empty() {
                string.push_str(line);
            } else {
                string.push_str("\\n");
                string.push_str(line);
            }
            string
        });

        // Resolve command
        let command = substitute_all_template_variables_in_str(
            &self.command,
            &task_variables,
            &variable_names,
            &mut substituted_variables,
        )?;

        // Resolve args
        let args = substitute_all_template_variables_in_vec(
            &self.args,
            &task_variables,
            &variable_names,
            &mut substituted_variables,
        )?;

        // Generate ID
        let template_hash = to_hex_hash(self)
            .context("hashing configuration template")
            .log_err()?;
        let variables_hash = to_hex_hash(&task_variables)
            .context("hashing configuration variables")
            .log_err()?;
        let id = ConfigurationId(format!("{id_base}_{template_hash}_{variables_hash}"));

        // Resolve environment variables
        let env = {
            let mut env = cx.project_env.clone();
            env.extend(self.env.clone());
            let mut env = substitute_all_template_variables_in_map(
                &env,
                &task_variables,
                &variable_names,
                &mut substituted_variables,
            )?;
            env.extend(task_variables.into_iter().map(|(k, v)| (k, v.to_owned())));
            env
        };

        Some(ResolvedConfiguration {
            id: id.clone(),
            substituted_variables,
            original_template: self.clone(),
            resolved_label: full_label.clone(),
            resolved: ExecutionConfig {
                id,
                full_label,
                label: human_readable_label,
                command: Some(command),
                args,
                cwd,
                env,
                config_type: self.config_type,
            },
        })
    }
}

fn truncate_variables(task_variables: &HashMap<String, &str>) -> HashMap<String, String> {
    task_variables
        .iter()
        .map(|(key, value)| {
            (
                key.clone(),
                truncate_and_remove_front(value, MAX_DISPLAY_VARIABLE_LENGTH),
            )
        })
        .collect()
}

fn to_hex_hash(object: impl Serialize) -> anyhow::Result<String> {
    let json = serde_json_lenient::to_string(&object).context("serializing the object")?;
    let mut hasher = Sha256::new();
    hasher.update(json.as_bytes());
    Ok(hex::encode(hasher.finalize()))
}

fn substitute_all_template_variables_in_str<A: AsRef<str>>(
    template_str: &str,
    task_variables: &HashMap<String, A>,
    variable_names: &HashMap<String, VariableName>,
    substituted_variables: &mut HashSet<VariableName>,
) -> Option<String> {
    let substituted_string = shellexpand::env_with_context(template_str, |var| {
        let colon_position = var.find(':').unwrap_or(var.len());
        let (variable_name, default) = var.split_at(colon_position);
        if let Some(name) = task_variables.get(variable_name) {
            if let Some(substituted_variable) = variable_names.get(variable_name) {
                substituted_variables.insert(substituted_variable.clone());
            }
            return Ok(Some(name.as_ref().to_owned()));
        } else if variable_name.starts_with(ZED_VARIABLE_NAME_PREFIX) {
            if !default.is_empty() {
                return Ok(Some(default[1..].to_owned()));
            } else {
                bail!("Unknown variable name: {variable_name}");
            }
        }
        if !default.is_empty() {
            return Ok(Some(format!("${{{var}}}")));
        }
        Ok(None)
    })
    .ok()?;
    Some(substituted_string.into_owned())
}

fn substitute_all_template_variables_in_vec(
    template_strs: &[String],
    task_variables: &HashMap<String, &str>,
    variable_names: &HashMap<String, VariableName>,
    substituted_variables: &mut HashSet<VariableName>,
) -> Option<Vec<String>> {
    let mut expanded = Vec::with_capacity(template_strs.len());
    for variable in template_strs {
        let new_value = substitute_all_template_variables_in_str(
            variable,
            task_variables,
            variable_names,
            substituted_variables,
        )?;
        expanded.push(new_value);
    }
    Some(expanded)
}

fn substitute_all_template_variables_in_map(
    keys_and_values: &HashMap<String, String>,
    task_variables: &HashMap<String, &str>,
    variable_names: &HashMap<String, VariableName>,
    substituted_variables: &mut HashSet<VariableName>,
) -> Option<HashMap<String, String>> {
    let mut new_map: HashMap<String, String> = Default::default();
    for (key, value) in keys_and_values {
        let new_value = substitute_all_template_variables_in_str(
            value,
            task_variables,
            variable_names,
            substituted_variables,
        )?;
        let new_key = substitute_all_template_variables_in_str(
            key,
            task_variables,
            variable_names,
            substituted_variables,
        )?;
        new_map.insert(new_key, new_value);
    }
    Some(new_map)
}

#[cfg(test)]
mod tests {
    use super::*;
    use task::TaskVariables;

    const TEST_ID_BASE: &str = "test_base";

    #[test]
    fn test_resolving_templates_with_blank_command_and_label() {
        let config_with_all_properties = ConfigurationTemplate {
            label: "test_label".to_string(),
            command: "test_command".to_string(),
            args: vec!["test_arg".to_string()],
            env: HashMap::from_iter([("test_env_key".to_string(), "test_env_var".to_string())]),
            ..ConfigurationTemplate::default()
        };

        for config_with_blank_property in &[
            ConfigurationTemplate {
                label: "".to_string(),
                ..config_with_all_properties.clone()
            },
            ConfigurationTemplate {
                command: "".to_string(),
                ..config_with_all_properties.clone()
            },
            ConfigurationTemplate {
                label: "".to_string(),
                command: "".to_string(),
                ..config_with_all_properties
            },
        ] {
            assert_eq!(
                config_with_blank_property.resolve(TEST_ID_BASE, &ConfigurationContext::default()),
                None,
                "should not resolve configuration with blank label and/or command: {config_with_blank_property:?}"
            );
        }
    }

    #[test]
    fn test_template_variable_resolution() {
        let config = ConfigurationTemplate {
            label: "Test ${ZED_FILE}".to_string(),
            command: "echo".to_string(),
            args: vec!["${ZED_FILE}".to_string()],
            ..ConfigurationTemplate::default()
        };

        let mut task_vars = TaskVariables::default();
        task_vars.insert(VariableName::File, "/path/to/file.rs".to_string());

        let cx = ConfigurationContext {
            cwd: None,
            task_variables: task_vars,
            project_env: HashMap::default(),
        };

        let resolved = config.resolve(TEST_ID_BASE, &cx).unwrap();
        assert_eq!(resolved.resolved_label, "Test /path/to/file.rs");
        assert_eq!(resolved.resolved.args, vec!["/path/to/file.rs"]);
    }
}
