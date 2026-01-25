use anyhow::{Result, anyhow, bail};
use gpui::SharedString;
use handlebars::{Handlebars, RenderError, Renderable, StringOutput};
use rust_embed::RustEmbed;
use serde::Serialize;
use serde_json::{Map, Value as Json};
use std::{
    collections::{BTreeSet, HashMap},
    sync::Arc,
};

#[derive(RustEmbed)]
#[folder = "src/templates"]
#[include = "*.hbs"]
struct Assets;

pub struct Templates(Handlebars<'static>);

impl Templates {
    pub fn new() -> Arc<Self> {
        Self::new_with_section_config(None)
    }

    pub fn new_with_section_config(section_config: Option<TemplateSectionConfig>) -> Arc<Self> {
        let mut handlebars = Handlebars::new();
        handlebars.set_strict_mode(true);
        handlebars.register_helper("contains", Box::new(contains));
        handlebars.register_helper(
            "section",
            Box::new(SectionHelper {
                section_config: Arc::new(section_config.unwrap_or_default()),
            }),
        );
        handlebars.register_embed_templates::<Assets>().unwrap();
        Arc::new(Self(handlebars))
    }

    /// Returns the section graph for all embedded templates so callers can
    /// understand which sections are nested beneath others.
    pub fn section_graph() -> Result<SectionGraph> {
        let mut graph = SectionGraph {
            templates: HashMap::new(),
        };
        for template_name in Assets::iter() {
            let asset = Assets::get(template_name.as_ref())
                .ok_or_else(|| anyhow!("Missing template asset: {template_name}"))?;
            let contents = std::str::from_utf8(asset.data.as_ref())
                .map_err(|err| anyhow!("Invalid utf-8 in {template_name}: {err}"))?;
            let template_graph = parse_section_graph_from_text(contents)
                .map_err(|err| anyhow!("Failed to parse {template_name}: {err}"))?;
            graph
                .templates
                .insert(template_name.into_owned(), template_graph);
        }
        Ok(graph)
    }
}

/// The complete section graph for each embedded template.
#[derive(Clone, Debug, Default)]
pub struct SectionGraph {
    pub templates: HashMap<String, TemplateSectionGraph>,
}

/// The section hierarchy for a single template.
#[derive(Clone, Debug, Default)]
pub struct TemplateSectionGraph {
    /// All sections in the template keyed by name.
    pub sections: HashMap<String, SectionNode>,
    /// Sections that are not nested under any other section.
    pub roots: Vec<String>,
}

/// A node in the section hierarchy with parent/child relationships.
#[derive(Clone, Debug, Default)]
pub struct SectionNode {
    pub name: String,
    pub children: Vec<String>,
    pub parents: Vec<String>,
    /// Available argument names for replacements rendered within this section.
    pub arguments: Arc<Vec<String>>,
}

#[derive(Clone, Debug, Default)]
pub struct TemplateSectionConfig {
    sections: HashMap<String, TemplateSectionOverride>,
}

impl TemplateSectionConfig {
    pub fn insert_replace(
        &mut self,
        section_name: impl Into<String>,
        replacement: impl Into<String>,
    ) {
        self.sections.insert(
            section_name.into(),
            TemplateSectionOverride::Replace(replacement.into()),
        );
    }

    pub fn insert_remove(&mut self, section_name: impl Into<String>) {
        self.sections
            .insert(section_name.into(), TemplateSectionOverride::Remove);
    }

    fn section_override(&self, section_name: &str) -> Option<&TemplateSectionOverride> {
        self.sections.get(section_name)
    }
}

#[derive(Clone, Debug)]
pub enum TemplateSectionOverride {
    Replace(String),
    Remove,
}

struct SectionHelper {
    section_config: Arc<TemplateSectionConfig>,
}

impl handlebars::HelperDef for SectionHelper {
    fn call<'reg: 'rc, 'rc>(
        &self,
        helper: &handlebars::Helper<'reg, 'rc>,
        registry: &'reg handlebars::Handlebars,
        context: &'rc handlebars::Context,
        render_context: &mut handlebars::RenderContext<'reg, 'rc>,
        out: &mut dyn handlebars::Output,
    ) -> handlebars::HelperResult {
        let section_name = helper
            .param(0)
            .and_then(section_name_from_param)
            .or_else(|| {
                helper
                    .hash_get("name")
                    .and_then(|value| value.value().as_str())
                    .map(str::to_owned)
            })
            .ok_or_else(|| handlebars::RenderError::new("section: missing name parameter"))?;

        if let Some(section_override) = self.section_config.section_override(&section_name) {
            match section_override {
                TemplateSectionOverride::Replace(replacement) => {
                    let rendered = render_replacement_with_context(
                        replacement,
                        registry,
                        context,
                        render_context,
                    )?;
                    out.write(&trim_section_trailing_newline(rendered))?;
                }
                TemplateSectionOverride::Remove => {
                    if let Some(inverse) = helper.inverse() {
                        let rendered =
                            render_template_to_string(inverse, registry, context, render_context)?;
                        out.write(&trim_section_trailing_newline(rendered))?;
                    }
                }
            }
            return Ok(());
        }

        let template = helper
            .template()
            .ok_or_else(|| handlebars::RenderError::new("section: missing block"))?;
        let rendered = render_template_to_string(template, registry, context, render_context)?;
        out.write(&trim_section_trailing_newline(rendered))?;
        Ok(())
    }
}

fn render_template_to_string<'reg, 'rc>(
    template: &'reg handlebars::Template,
    registry: &'reg handlebars::Handlebars<'reg>,
    context: &'rc handlebars::Context,
    render_context: &mut handlebars::RenderContext<'reg, 'rc>,
) -> Result<String, RenderError> {
    let mut output = StringOutput::new();
    template.render(registry, context, render_context, &mut output)?;
    output
        .into_string()
        .map_err(|err| RenderError::new(format!("section: invalid utf-8 output: {err}")))
}

fn render_replacement_with_context<'reg, 'rc>(
    replacement: &str,
    registry: &'reg handlebars::Handlebars<'reg>,
    context: &'rc handlebars::Context,
    render_context: &mut handlebars::RenderContext<'reg, 'rc>,
) -> Result<String, RenderError> {
    let replacement_context = replacement_context(context, render_context)?;
    let context = replacement_context.as_ref().unwrap_or(context);
    registry
        .render_template_with_context(replacement, context)
        .map_err(|err| RenderError::new(format!("section: replacement render failed: {err}")))
}

fn replacement_context<'reg, 'rc>(
    context: &'rc handlebars::Context,
    render_context: &handlebars::RenderContext<'reg, 'rc>,
) -> Result<Option<handlebars::Context>, RenderError> {
    let scoped = render_context.evaluate(context, "this")?;
    if scoped.is_missing() {
        return Ok(None);
    }
    let this_value = scoped.as_json().clone();
    let root_value = context.data().clone();
    let mut map = match &this_value {
        Json::Object(map) => map.clone(),
        _ => Map::new(),
    };
    map.insert("this".to_string(), this_value);
    map.insert("root".to_string(), root_value);
    let value = Json::Object(map);
    let context = handlebars::Context::wraps(value)
        .map_err(|err| RenderError::new(format!("section: invalid replacement context: {err}")))?;
    Ok(Some(context))
}

fn section_name_from_param(param: &handlebars::PathAndJson) -> Option<String> {
    if let Some(section_name) = param.value().as_str() {
        return Some(section_name.to_owned());
    }

    if param.is_value_missing() {
        return param.relative_path().cloned();
    }

    None
}

fn trim_section_trailing_newline(mut rendered: String) -> String {
    if rendered.ends_with("\r\n\r\n") {
        rendered.truncate(rendered.len() - 2);
    } else if rendered.ends_with("\n\n") {
        rendered.pop();
    }
    rendered
}

fn parse_section_graph_from_text(contents: &str) -> Result<TemplateSectionGraph> {
    let mut sections: HashMap<String, SectionNode> = HashMap::new();
    let mut argument_pool: HashMap<Vec<String>, Arc<Vec<String>>> = HashMap::new();
    let mut roots = BTreeSet::new();
    let mut stack: Vec<String> = Vec::new();
    let mut scope_stack = vec![BlockScope {
        name: None,
        arguments: intern_arguments(&default_arguments(), &mut argument_pool),
    }];
    let mut index = 0;
    while let Some(open_index) = contents[index..].find("{{") {
        let tag_start = index + open_index;
        let tag_end = contents[tag_start..]
            .find("}}")
            .map(|offset| tag_start + offset + 2)
            .ok_or_else(|| anyhow!("Unterminated template tag"))?;
        let tag = &contents[tag_start + 2..tag_end - 2];
        let tag = tag.trim();
        if tag.starts_with("!--") {
            index = tag_end;
            continue;
        }
        if let Some(section_name) = tag.strip_prefix("#section") {
            let section_name =
                parse_section_name(section_name).ok_or_else(|| anyhow!("section: missing name"))?;
            let entry = sections
                .entry(section_name.clone())
                .or_insert_with(|| SectionNode {
                    name: section_name.clone(),
                    ..SectionNode::default()
                });
            entry.arguments = scope_stack
                .last()
                .map(|scope| scope.arguments.clone())
                .unwrap_or_else(|| intern_arguments(&default_arguments(), &mut argument_pool));
            if let Some(parent) = stack.last() {
                add_unique(&mut entry.parents, parent);
                let parent_entry = sections
                    .entry(parent.clone())
                    .or_insert_with(|| SectionNode {
                        name: parent.clone(),
                        arguments: scope_stack
                            .last()
                            .map(|scope| scope.arguments.clone())
                            .unwrap_or_else(|| {
                                intern_arguments(&default_arguments(), &mut argument_pool)
                            }),
                        ..SectionNode::default()
                    });
                add_unique(&mut parent_entry.children, &section_name);
            } else {
                roots.insert(section_name.clone());
            }
            stack.push(section_name);
        } else if let Some(scope_name) = parse_block_start(tag) {
            let arguments = if scope_name == "each" || scope_name == "with" {
                let new_params = parse_block_params(tag);
                if new_params.is_empty() {
                    scope_stack
                        .last()
                        .map(|scope| scope.arguments.clone())
                        .unwrap_or_else(|| {
                            intern_arguments(&default_arguments(), &mut argument_pool)
                        })
                } else {
                    let mut arguments = scope_stack
                        .last()
                        .map(|scope| (*scope.arguments).clone())
                        .unwrap_or_else(default_arguments);
                    arguments.extend(new_params);
                    intern_arguments(&arguments, &mut argument_pool)
                }
            } else {
                scope_stack
                    .last()
                    .map(|scope| scope.arguments.clone())
                    .unwrap_or_else(|| intern_arguments(&default_arguments(), &mut argument_pool))
            };
            scope_stack.push(BlockScope {
                name: Some(scope_name),
                arguments,
            });
        } else if tag.starts_with("/section") {
            if stack.pop().is_none() {
                bail!("section: closing tag without opener");
            }
        } else if let Some(close_name) = parse_block_end(tag) {
            if scope_stack.len() > 1
                && scope_stack.last().and_then(|scope| scope.name.as_deref())
                    == Some(close_name.as_str())
            {
                scope_stack.pop();
            }
        }
        index = tag_end;
    }
    if !stack.is_empty() {
        bail!("section: unclosed tag");
    }
    let roots = roots.into_iter().collect::<Vec<_>>();
    Ok(TemplateSectionGraph { sections, roots })
}

#[derive(Clone, Debug)]
struct BlockScope {
    name: Option<String>,
    arguments: Arc<Vec<String>>,
}

fn default_arguments() -> Vec<String> {
    vec!["root".to_string(), "this".to_string()]
}

fn intern_arguments(
    arguments: &[String],
    pool: &mut HashMap<Vec<String>, Arc<Vec<String>>>,
) -> Arc<Vec<String>> {
    if let Some(shared) = pool.get(arguments) {
        return shared.clone();
    }
    let shared = Arc::new(arguments.to_vec());
    pool.insert(arguments.to_vec(), shared.clone());
    shared
}

fn parse_block_start(tag: &str) -> Option<String> {
    let tag = tag.trim_start();
    let tag = tag.strip_prefix('#')?;
    parse_block_name(tag)
}

fn parse_block_end(tag: &str) -> Option<String> {
    let tag = tag.trim_start();
    let tag = tag.strip_prefix('/')?;
    parse_block_name(tag)
}

fn parse_block_name(tag: &str) -> Option<String> {
    let end = tag
        .find(|char: char| char.is_whitespace() || char == '}')
        .unwrap_or(tag.len());
    if end == 0 {
        None
    } else {
        Some(tag[..end].to_string())
    }
}

fn parse_block_params(tag: &str) -> Vec<String> {
    let Some(as_index) = tag.find(" as |") else {
        return Vec::new();
    };
    let after_as = &tag[as_index + " as |".len()..];
    let Some(end) = after_as.find('|') else {
        return Vec::new();
    };
    after_as[..end]
        .split_whitespace()
        .map(str::to_string)
        .collect()
}

fn parse_section_name(tag_body: &str) -> Option<String> {
    let tag_body = tag_body.trim();
    if tag_body.is_empty() {
        return None;
    }

    if let Some(quoted) = tag_body.strip_prefix('"') {
        let end = quoted.find('"')?;
        return Some(quoted[..end].to_string());
    }

    let name_end = tag_body
        .find(|char: char| char.is_whitespace() || char == '}')
        .unwrap_or(tag_body.len());
    if name_end == 0 {
        None
    } else {
        Some(tag_body[..name_end].to_string())
    }
}

fn add_unique(target: &mut Vec<String>, value: &str) {
    if !target.iter().any(|existing| existing == value) {
        target.push(value.to_string());
    }
}

pub trait Template: Sized {
    const TEMPLATE_NAME: &'static str;

    fn render(&self, templates: &Templates) -> Result<String>
    where
        Self: Serialize + Sized,
    {
        Ok(templates.0.render(Self::TEMPLATE_NAME, self)?)
    }
}

#[derive(Serialize)]
pub struct SystemPromptTemplate<'a> {
    #[serde(flatten)]
    pub project: &'a prompt_store::ProjectContext,
    pub available_tools: Vec<SharedString>,
    pub model_name: Option<String>,
}

impl Template for SystemPromptTemplate<'_> {
    const TEMPLATE_NAME: &'static str = "system_prompt.hbs";
}

/// Handlebars helper for checking if an item is in a list
fn contains(
    h: &handlebars::Helper,
    _: &handlebars::Handlebars,
    _: &handlebars::Context,
    _: &mut handlebars::RenderContext,
    out: &mut dyn handlebars::Output,
) -> handlebars::HelperResult {
    let list = h
        .param(0)
        .and_then(|v| v.value().as_array())
        .ok_or_else(|| {
            handlebars::RenderError::new("contains: missing or invalid list parameter")
        })?;
    let query = h.param(1).map(|v| v.value()).ok_or_else(|| {
        handlebars::RenderError::new("contains: missing or invalid query parameter")
    })?;

    if list.contains(query) {
        out.write("true")?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_system_prompt_template() {
        let project = prompt_store::ProjectContext::default();
        let template = SystemPromptTemplate {
            project: &project,
            available_tools: vec!["echo".into()],
            model_name: Some("test-model".to_string()),
        };
        let templates = Templates::new();
        let rendered = template.render(&templates).unwrap();
        assert!(rendered.contains("## Fixing Diagnostics"));
        assert!(rendered.contains("test-model"));
    }

    #[test]
    fn test_section_override_removes_example_block() {
        let mut section_config = TemplateSectionConfig::default();
        section_config.insert_remove("edit_file_prompt_xml_example");
        let templates = Templates::new_with_section_config(Some(section_config));
        let rendered = templates
            .0
            .render(
                "edit_file_prompt_xml.hbs",
                &json!({
                    "path": "/tmp/example.rs",
                    "edit_description": "test",
                }),
            )
            .unwrap();
        assert!(!rendered.contains("struct User"));
    }

    #[test]
    fn test_section_unquoted_name_trims_trailing_newline() {
        let templates = Templates::new_with_section_config(None);
        let rendered = templates
            .0
            .render_template("{{#section root}}\nhello\n{{/section}}\nworld", &json!({}))
            .unwrap();
        assert_eq!(rendered, "hello\nworld");
    }

    #[test]
    fn test_section_graph_parses_roots_and_children() {
        let graph = parse_section_graph_from_text(
            "{{#section root}}\n{{#section child}}\ntext\n{{/section}}\n{{/section}}\n",
        )
        .unwrap();
        let root = graph.sections.get("root").unwrap();
        assert!(graph.roots.contains(&"root".to_string()));
        assert_eq!(root.children, vec!["child".to_string()]);
        assert!(root.arguments.contains(&"root".to_string()));
        assert!(root.arguments.contains(&"this".to_string()));
        let child = graph.sections.get("child").unwrap();
        assert_eq!(child.parents, vec!["root".to_string()]);
    }

    #[test]
    fn test_section_graph_ignores_comments() {
        let graph = parse_section_graph_from_text(
            "{{!-- {{#section ignored}} --}}\n{{#section root}}\ntext\n{{/section}}\n",
        )
        .unwrap();
        assert!(graph.sections.contains_key("root"));
        assert!(!graph.sections.contains_key("ignored"));
    }

    #[test]
    fn test_section_graph_errors_on_unclosed_sections() {
        let err = parse_section_graph_from_text("{{#section root}}\ntext\n").unwrap_err();
        assert!(err.to_string().contains("section: unclosed tag"));
    }

    #[test]
    fn test_section_replace_uses_context() {
        let mut section_config = TemplateSectionConfig::default();
        section_config.insert_replace("root", "Hello {{name}}");
        let templates = Templates::new_with_section_config(Some(section_config));
        let rendered = templates
            .0
            .render_template(
                "{{#section root}}ignored{{/section}}",
                &json!({"name": "Sandro"}),
            )
            .unwrap();
        assert_eq!(rendered, "Hello Sandro");
    }

    #[test]
    fn test_section_replace_uses_each_context() {
        let mut section_config = TemplateSectionConfig::default();
        section_config.insert_replace("root", "{{name}}\n");
        let templates = Templates::new_with_section_config(Some(section_config));
        let rendered = templates
            .0
            .render_template(
                "{{#each assertions}}{{#section root}}ignored{{/section}}{{/each}}",
                &json!({"assertions": [{"name": "A"}, {"name": "B"}]}),
            )
            .unwrap();
        assert_eq!(rendered, "A\nB\n");
    }

    #[test]
    fn test_section_graph_shares_arguments() {
        let graph = parse_section_graph_from_text(
            "{{#section root}}\n{{#section first}}x{{/section}}{{#section second}}y{{/section}}{{/section}}",
        )
        .unwrap();
        let first = graph.sections.get("first").unwrap();
        let second = graph.sections.get("second").unwrap();
        assert!(Arc::ptr_eq(&first.arguments, &second.arguments));
    }

    #[test]
    fn test_section_replace_exposes_root_context() {
        let mut section_config = TemplateSectionConfig::default();
        section_config.insert_replace("root", "{{root.title}}:{{name}}\n");
        let templates = Templates::new_with_section_config(Some(section_config));
        let rendered = templates
            .0
            .render_template(
                "{{#each assertions}}{{#section root}}ignored{{/section}}{{/each}}",
                &json!({"title": "Checklist", "assertions": [{"name": "A"}, {"name": "B"}]}),
            )
            .unwrap();
        assert_eq!(rendered, "Checklist:A\nChecklist:B\n");
    }
}
