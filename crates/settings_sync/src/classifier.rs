use std::collections::HashMap;

use regex::Regex;
use serde_json::{Map, Value};
use settings_content::UserSettingsContent;

const MAX_REF_DEPTH: usize = 32;

pub struct DocumentClassifier {
    schema: Value,
    patterns: HashMap<String, Regex>,
}

pub struct ContainerView<'a> {
    properties: Vec<&'a Map<String, Value>>,
    pattern_properties: Vec<(&'a Regex, &'a Value)>,
    additional: Option<&'a Value>,
}

impl<'a> ContainerView<'a> {
    pub fn child_schema(&self, key: &str) -> Option<&'a Value> {
        self.properties
            .iter()
            .find_map(|properties| properties.get(key))
            .or_else(|| {
                self.pattern_properties
                    .iter()
                    .find(|(pattern, _)| pattern.is_match(key))
                    .map(|(_, schema)| *schema)
            })
            .or(self.additional)
    }

    fn is_empty(&self) -> bool {
        self.properties
            .iter()
            .all(|properties| properties.is_empty())
            && self.pattern_properties.is_empty()
            && self.additional.is_none()
    }
}

impl DocumentClassifier {
    pub fn from_schema(schema: Value) -> Self {
        let mut patterns = HashMap::new();
        collect_patterns(&schema, &mut patterns);
        Self { schema, patterns }
    }

    pub fn for_user_settings() -> Self {
        let mut generator = schemars::generate::SchemaSettings::draft2019_09().into_generator();
        Self::from_schema(
            generator
                .root_schema_for::<UserSettingsContent>()
                .to_value(),
        )
    }

    pub fn root_schema(&self) -> &Value {
        &self.schema
    }

    pub fn container_view<'a>(&'a self, schema: &'a Value) -> Option<ContainerView<'a>> {
        self.container_view_inner(schema, 0)
    }

    fn container_view_inner<'a>(
        &'a self,
        schema: &'a Value,
        depth: usize,
    ) -> Option<ContainerView<'a>> {
        if depth > MAX_REF_DEPTH {
            return None;
        }
        let schema = self.resolve(schema, depth)?;
        let object = schema.as_object()?;

        let mut view = ContainerView {
            properties: Vec::new(),
            pattern_properties: Vec::new(),
            additional: None,
        };

        if let Some(properties) = object.get("properties").and_then(Value::as_object) {
            view.properties.push(properties);
        }
        if let Some(pattern_properties) = object.get("patternProperties").and_then(Value::as_object)
        {
            view.pattern_properties.extend(
                pattern_properties
                    .iter()
                    .filter_map(|(pattern, schema)| Some((self.patterns.get(pattern)?, schema))),
            );
        }
        if let Some(additional) = object.get("additionalProperties")
            && additional.is_object()
        {
            view.additional = Some(additional);
        }

        if let Some(all_of) = object.get("allOf").and_then(Value::as_array) {
            for branch in all_of {
                if let Some(branch_view) = self.container_view_inner(branch, depth + 1) {
                    view.properties.extend(branch_view.properties);
                    view.pattern_properties
                        .extend(branch_view.pattern_properties);
                    view.additional = view.additional.or(branch_view.additional);
                }
            }
        }

        if view.is_empty()
            && let Some(branch) = single_non_null_branch(object)
        {
            return self.container_view_inner(branch, depth + 1);
        }

        if view.is_empty() { None } else { Some(view) }
    }

    fn resolve<'a>(&'a self, schema: &'a Value, depth: usize) -> Option<&'a Value> {
        if depth > MAX_REF_DEPTH {
            return None;
        }
        let Some(reference) = schema.get("$ref").and_then(Value::as_str) else {
            return Some(schema);
        };
        let definition_name = reference
            .strip_prefix("#/definitions/")
            .or_else(|| reference.strip_prefix("#/$defs/"))?;
        let definition = self
            .schema
            .get("definitions")
            .or_else(|| self.schema.get("$defs"))?
            .get(definition_name)?;
        self.resolve(definition, depth + 1)
    }
}

fn collect_patterns(schema: &Value, patterns: &mut HashMap<String, Regex>) {
    match schema {
        Value::Object(object) => {
            if let Some(pattern_properties) =
                object.get("patternProperties").and_then(Value::as_object)
            {
                for pattern in pattern_properties.keys() {
                    if patterns.contains_key(pattern) {
                        continue;
                    }
                    match Regex::new(pattern) {
                        Ok(regex) => {
                            patterns.insert(pattern.clone(), regex);
                        }
                        Err(error) => {
                            log::warn!("settings sync: invalid schema pattern {pattern:?}: {error}")
                        }
                    }
                }
            }
            for value in object.values() {
                collect_patterns(value, patterns);
            }
        }
        Value::Array(values) => {
            for value in values {
                collect_patterns(value, patterns);
            }
        }
        _ => {}
    }
}

fn single_non_null_branch(object: &Map<String, Value>) -> Option<&Value> {
    let branches = object
        .get("anyOf")
        .or_else(|| object.get("oneOf"))?
        .as_array()?;
    let mut non_null_branches = branches.iter().filter(|branch| !is_null_schema(branch));
    let first = non_null_branches.next()?;
    if non_null_branches.next().is_some() {
        return None;
    }
    Some(first)
}

fn is_null_schema(schema: &Value) -> bool {
    match schema.get("type") {
        Some(Value::String(type_name)) => type_name == "null",
        Some(Value::Array(type_names)) => type_names.iter().all(|name| name == "null"),
        _ => false,
    }
}
