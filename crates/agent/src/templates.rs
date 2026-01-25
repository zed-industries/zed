use anyhow::{Result, anyhow};
use gpui::SharedString;
use handlebars::{
    Handlebars, RenderError, Renderable, StringOutput,
    template::{BlockParam, HelperTemplate, Parameter, TemplateElement},
};
use rust_embed::RustEmbed;
use serde::Serialize;
use serde_json::{Map, Value as Json};
use std::{
    collections::{BTreeSet, HashMap},
    sync::{Arc, LazyLock, Mutex, Weak},
};

#[derive(RustEmbed)]
#[folder = "src/templates"]
#[include = "*.hbs"]
struct Assets;

pub struct Templates(Handlebars<'static>);

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
enum TemplatesCacheKey {
    Default,
    Config(TemplateSectionConfigKey),
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct TemplateSectionConfigKey {
    sections: Vec<(String, TemplateSectionOverrideKey)>,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
enum TemplateSectionOverrideKey {
    Replace(String),
    Remove,
}

static TEMPLATES_CACHE: LazyLock<Mutex<HashMap<TemplatesCacheKey, Weak<Templates>>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

/// Bump this when section dependency rules change.
pub const SECTION_GRAPH_VERSION: SectionGraphVersion = SectionGraphVersion {
    major: 0,
    minor: 1,
    patch: 0,
};

impl Templates {
    pub fn new() -> Arc<Self> {
        Self::new_with_section_config(None)
    }

    pub fn new_with_section_config(section_config: Option<TemplateSectionConfig>) -> Arc<Self> {
        let cache_key = match section_config.as_ref() {
            Some(config) if config.is_empty() => TemplatesCacheKey::Default,
            Some(config) => TemplatesCacheKey::Config(config.cache_key()),
            None => TemplatesCacheKey::Default,
        };
        if let Ok(cache) = TEMPLATES_CACHE.lock() {
            if let Some(cached) = cache.get(&cache_key).and_then(Weak::upgrade) {
                return cached;
            }
        }

        let section_config = section_config.unwrap_or_default();
        let mut handlebars = Handlebars::new();
        handlebars.set_strict_mode(true);
        handlebars.register_helper("contains", Box::new(contains));
        handlebars.register_helper(
            "section",
            Box::new(SectionHelper {
                section_config: Arc::new(section_config),
            }),
        );
        handlebars.register_embed_templates::<Assets>().unwrap();
        let templates = Arc::new(Self(handlebars));

        if let Ok(mut cache) = TEMPLATES_CACHE.lock() {
            cache.insert(cache_key, Arc::downgrade(&templates));
        }

        templates
    }

    /// Returns the section graph for all embedded templates so callers can
    /// understand which sections are nested beneath others.
    pub fn section_graph() -> Result<SectionGraph> {
        let mut graph = SectionGraph {
            version: SECTION_GRAPH_VERSION,
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

    /// Returns the default section bodies for embedded templates, keyed by section name.
    pub fn section_defaults() -> Result<HashMap<String, String>> {
        let mut defaults = HashMap::new();
        for template_name in Assets::iter() {
            let asset = Assets::get(template_name.as_ref())
                .ok_or_else(|| anyhow!("Missing template asset: {template_name}"))?;
            let contents = std::str::from_utf8(asset.data.as_ref())
                .map_err(|err| anyhow!("Invalid utf-8 in {template_name}: {err}"))?;
            let template_defaults = parse_section_defaults_from_text(contents)
                .map_err(|err| anyhow!("Failed to parse {template_name}: {err}"))?;
            for (section, body) in template_defaults {
                defaults.entry(section).or_insert(body);
            }
        }
        Ok(defaults)
    }
}

/// The complete section graph for each embedded template.
#[derive(Clone, Debug, Default)]
pub struct SectionGraph {
    pub version: SectionGraphVersion,
    pub templates: HashMap<String, TemplateSectionGraph>,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SectionGraphVersion {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
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

    fn is_empty(&self) -> bool {
        self.sections.is_empty()
    }

    fn cache_key(&self) -> TemplateSectionConfigKey {
        let mut sections = Vec::with_capacity(self.sections.len());
        for (section, override_entry) in &self.sections {
            let override_key = match override_entry {
                TemplateSectionOverride::Replace(replacement) => {
                    TemplateSectionOverrideKey::Replace(replacement.clone())
                }
                TemplateSectionOverride::Remove => TemplateSectionOverrideKey::Remove,
            };
            sections.push((section.clone(), override_key));
        }
        sections.sort_by(|left, right| left.0.cmp(&right.0));
        TemplateSectionConfigKey { sections }
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
    let template =
        handlebars::Template::compile(contents).map_err(|err| anyhow!(err.to_string()))?;
    let mut sections: HashMap<String, SectionNode> = HashMap::new();
    let mut argument_pool: HashMap<Vec<String>, Arc<Vec<String>>> = HashMap::new();
    let mut roots = BTreeSet::new();
    let mut stack: Vec<String> = Vec::new();
    let mut scope_stack = vec![BlockScope {
        arguments: intern_arguments(&default_arguments(), &mut argument_pool),
    }];

    visit_template(
        &template,
        &mut sections,
        &mut roots,
        &mut stack,
        &mut scope_stack,
        &mut argument_pool,
    )?;

    let roots = roots.into_iter().collect::<Vec<_>>();
    Ok(TemplateSectionGraph { sections, roots })
}

fn parse_section_defaults_from_text(contents: &str) -> Result<HashMap<String, String>> {
    let mut defaults = HashMap::new();
    let mut stack: Vec<(String, usize)> = Vec::new();
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
            stack.push((section_name, tag_end));
        } else if tag.starts_with("/section") {
            let (section_name, start) = stack
                .pop()
                .ok_or_else(|| anyhow!("section: unmatched closing tag"))?;
            let body = contents[start..tag_start].to_string();
            defaults.entry(section_name).or_insert(body);
        }
        index = tag_end;
    }

    if !stack.is_empty() {
        return Err(anyhow!("section: unclosed tag"));
    }

    Ok(defaults)
}

#[derive(Clone, Debug)]
struct BlockScope {
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

fn visit_template(
    template: &handlebars::Template,
    sections: &mut HashMap<String, SectionNode>,
    roots: &mut BTreeSet<String>,
    stack: &mut Vec<String>,
    scope_stack: &mut Vec<BlockScope>,
    argument_pool: &mut HashMap<Vec<String>, Arc<Vec<String>>>,
) -> Result<()> {
    for element in &template.elements {
        match element {
            TemplateElement::HelperBlock(helper) => {
                visit_helper_block(helper, sections, roots, stack, scope_stack, argument_pool)?;
            }
            TemplateElement::DecoratorBlock(decorator)
            | TemplateElement::PartialBlock(decorator) => {
                if let Some(template) = decorator.template.as_ref() {
                    visit_template(template, sections, roots, stack, scope_stack, argument_pool)?;
                }
            }
            _ => {}
        }
    }
    Ok(())
}

fn visit_helper_block(
    helper: &HelperTemplate,
    sections: &mut HashMap<String, SectionNode>,
    roots: &mut BTreeSet<String>,
    stack: &mut Vec<String>,
    scope_stack: &mut Vec<BlockScope>,
    argument_pool: &mut HashMap<Vec<String>, Arc<Vec<String>>>,
) -> Result<()> {
    let helper_name = helper.name.as_name().unwrap_or_default().to_string();
    let next_arguments = scoped_arguments(scope_stack, helper.block_param.as_ref(), argument_pool);

    if helper_name == "section" {
        let section_name = helper
            .params
            .get(0)
            .and_then(parameter_name)
            .or_else(|| helper.hash.get("name").and_then(parameter_name))
            .ok_or_else(|| anyhow!("section: missing name"))?;

        let entry = sections
            .entry(section_name.clone())
            .or_insert_with(|| SectionNode {
                name: section_name.clone(),
                ..SectionNode::default()
            });
        entry.arguments = scope_stack
            .last()
            .map(|scope| scope.arguments.clone())
            .unwrap_or_else(|| intern_arguments(&default_arguments(), argument_pool));

        if let Some(parent) = stack.last() {
            add_unique(&mut entry.parents, parent);
            let parent_entry = sections
                .entry(parent.clone())
                .or_insert_with(|| SectionNode {
                    name: parent.clone(),
                    arguments: scope_stack
                        .last()
                        .map(|scope| scope.arguments.clone())
                        .unwrap_or_else(|| intern_arguments(&default_arguments(), argument_pool)),
                    ..SectionNode::default()
                });
            add_unique(&mut parent_entry.children, &section_name);
        } else {
            roots.insert(section_name.clone());
        }

        stack.push(section_name);
        scope_stack.push(BlockScope {
            arguments: next_arguments,
        });
        if let Some(template) = helper.template.as_ref() {
            visit_template(template, sections, roots, stack, scope_stack, argument_pool)?;
        }
        if let Some(inverse) = helper.inverse.as_ref() {
            visit_template(inverse, sections, roots, stack, scope_stack, argument_pool)?;
        }
        scope_stack.pop();
        stack.pop();
        return Ok(());
    }

    scope_stack.push(BlockScope {
        arguments: next_arguments,
    });
    if let Some(template) = helper.template.as_ref() {
        visit_template(template, sections, roots, stack, scope_stack, argument_pool)?;
    }
    if let Some(inverse) = helper.inverse.as_ref() {
        visit_template(inverse, sections, roots, stack, scope_stack, argument_pool)?;
    }
    scope_stack.pop();
    Ok(())
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

fn parameter_name(param: &Parameter) -> Option<String> {
    match param {
        Parameter::Literal(Json::String(value)) => Some(value.clone()),
        _ => param.as_name().map(str::to_owned),
    }
}

fn add_unique(target: &mut Vec<String>, value: &str) {
    if !target.iter().any(|existing| existing == value) {
        target.push(value.to_string());
    }
}

fn scoped_arguments(
    scope_stack: &[BlockScope],
    block_param: Option<&BlockParam>,
    argument_pool: &mut HashMap<Vec<String>, Arc<Vec<String>>>,
) -> Arc<Vec<String>> {
    let Some(scope) = scope_stack.last() else {
        return intern_arguments(&default_arguments(), argument_pool);
    };
    let Some(block_param) = block_param else {
        return scope.arguments.clone();
    };
    let mut arguments = (*scope.arguments).clone();
    match block_param {
        BlockParam::Single(param) => {
            if let Some(name) = parameter_name(param) {
                arguments.push(name);
            }
        }
        BlockParam::Pair((first, second)) => {
            if let Some(name) = parameter_name(first) {
                arguments.push(name);
            }
            if let Some(name) = parameter_name(second) {
                arguments.push(name);
            }
        }
    }
    intern_arguments(&arguments, argument_pool)
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
    use std::sync::Arc;

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
    fn test_section_graph_has_version() {
        let graph = Templates::section_graph().unwrap();
        assert_eq!(graph.version, SECTION_GRAPH_VERSION);
    }

    #[test]
    fn test_section_defaults_smoke() {
        let defaults = Templates::section_defaults().unwrap();
        let default = defaults
            .get("system_prompt")
            .expect("system_prompt default missing");
        assert!(!default.trim().is_empty());
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
        assert!(parse_section_graph_from_text("{{#section root}}\ntext\n").is_err());
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

    #[test]
    fn test_templates_cache_reuses_by_config() {
        let default_first = Templates::new();
        let default_second = Templates::new();
        assert!(Arc::ptr_eq(&default_first, &default_second));

        let empty_config =
            Templates::new_with_section_config(Some(TemplateSectionConfig::default()));
        assert!(Arc::ptr_eq(&default_first, &empty_config));

        let mut section_config = TemplateSectionConfig::default();
        section_config.insert_replace("cache_test_section", "A");
        let templates_a_first = Templates::new_with_section_config(Some(section_config));

        let mut section_config = TemplateSectionConfig::default();
        section_config.insert_replace("cache_test_section", "A");
        let templates_a_second = Templates::new_with_section_config(Some(section_config));

        assert!(Arc::ptr_eq(&templates_a_first, &templates_a_second));

        let mut section_config = TemplateSectionConfig::default();
        section_config.insert_replace("cache_test_section", "B");
        let templates_b = Templates::new_with_section_config(Some(section_config));

        assert!(!Arc::ptr_eq(&templates_a_first, &templates_b));
    }
}
