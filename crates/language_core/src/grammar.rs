use crate::{
    HighlightId, HighlightMap, LanguageConfig, LanguageConfigOverride, LanguageName,
    LanguageQueries, language_config::BracketPairConfig,
};
use anyhow::{Context as _, Result};
use collections::HashMap;
use gpui::SharedString;
use lsp::LanguageServerName;
use parking_lot::Mutex;
use std::sync::atomic::{AtomicUsize, Ordering::SeqCst};
use tree_sitter::Query;

pub static NEXT_GRAMMAR_ID: AtomicUsize = AtomicUsize::new(0);

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub struct GrammarId(pub usize);

impl GrammarId {
    pub fn new() -> Self {
        Self(NEXT_GRAMMAR_ID.fetch_add(1, SeqCst))
    }
}

impl Default for GrammarId {
    fn default() -> Self {
        Self::new()
    }
}

pub struct Grammar {
    id: GrammarId,
    pub ts_language: tree_sitter::Language,
    pub error_query: Option<Query>,
    pub highlights_config: Option<HighlightsConfig>,
    pub brackets_config: Option<BracketsConfig>,
    pub redactions_config: Option<RedactionConfig>,
    pub runnable_config: Option<RunnableConfig>,
    pub indents_config: Option<IndentConfig>,
    pub outline_config: Option<OutlineConfig>,
    pub text_object_config: Option<TextObjectConfig>,
    pub injection_config: Option<InjectionConfig>,
    pub override_config: Option<OverrideConfig>,
    pub debug_variables_config: Option<DebugVariablesConfig>,
    pub imports_config: Option<ImportsConfig>,
    pub highlight_map: Mutex<HighlightMap>,
}

pub struct HighlightsConfig {
    pub query: Query,
    pub identifier_capture_indices: Vec<u32>,
}

pub struct IndentConfig {
    pub query: Query,
    pub indent_capture_ix: u32,
    pub start_capture_ix: Option<u32>,
    pub end_capture_ix: Option<u32>,
    pub outdent_capture_ix: Option<u32>,
    pub suffixed_start_captures: HashMap<u32, SharedString>,
}

pub struct OutlineConfig {
    pub query: Query,
    pub item_capture_ix: u32,
    pub name_capture_ix: u32,
    pub context_capture_ix: Option<u32>,
    pub extra_context_capture_ix: Option<u32>,
    pub open_capture_ix: Option<u32>,
    pub close_capture_ix: Option<u32>,
    pub annotation_capture_ix: Option<u32>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DebuggerTextObject {
    Variable,
    Scope,
}

impl DebuggerTextObject {
    pub fn from_capture_name(name: &str) -> Option<DebuggerTextObject> {
        match name {
            "debug-variable" => Some(DebuggerTextObject::Variable),
            "debug-scope" => Some(DebuggerTextObject::Scope),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TextObject {
    InsideFunction,
    AroundFunction,
    InsideClass,
    AroundClass,
    InsideComment,
    AroundComment,
}

impl TextObject {
    pub fn from_capture_name(name: &str) -> Option<TextObject> {
        match name {
            "function.inside" => Some(TextObject::InsideFunction),
            "function.around" => Some(TextObject::AroundFunction),
            "class.inside" => Some(TextObject::InsideClass),
            "class.around" => Some(TextObject::AroundClass),
            "comment.inside" => Some(TextObject::InsideComment),
            "comment.around" => Some(TextObject::AroundComment),
            _ => None,
        }
    }

    pub fn around(&self) -> Option<Self> {
        match self {
            TextObject::InsideFunction => Some(TextObject::AroundFunction),
            TextObject::InsideClass => Some(TextObject::AroundClass),
            TextObject::InsideComment => Some(TextObject::AroundComment),
            _ => None,
        }
    }
}

pub struct TextObjectConfig {
    pub query: Query,
    pub text_objects_by_capture_ix: Vec<(u32, TextObject)>,
}

pub struct InjectionConfig {
    pub query: Query,
    pub content_capture_ix: u32,
    pub language_capture_ix: Option<u32>,
    pub patterns: Vec<InjectionPatternConfig>,
}

pub struct RedactionConfig {
    pub query: Query,
    pub redaction_capture_ix: u32,
}

#[derive(Clone, Debug, PartialEq)]
pub enum RunnableCapture {
    Named(SharedString),
    Run,
}

pub struct RunnableConfig {
    pub query: Query,
    /// A mapping from capture index to capture kind
    pub extra_captures: Vec<RunnableCapture>,
}

pub struct OverrideConfig {
    pub query: Query,
    pub values: HashMap<u32, OverrideEntry>,
}

#[derive(Debug)]
pub struct OverrideEntry {
    pub name: String,
    pub range_is_inclusive: bool,
    pub value: LanguageConfigOverride,
}

#[derive(Default, Clone)]
pub struct InjectionPatternConfig {
    pub language: Option<Box<str>>,
    pub combined: bool,
}

#[derive(Debug)]
pub struct BracketsConfig {
    pub query: Query,
    pub open_capture_ix: u32,
    pub close_capture_ix: u32,
    pub patterns: Vec<BracketsPatternConfig>,
}

#[derive(Clone, Debug, Default)]
pub struct BracketsPatternConfig {
    pub newline_only: bool,
    pub rainbow_exclude: bool,
}

pub struct DebugVariablesConfig {
    pub query: Query,
    pub objects_by_capture_ix: Vec<(u32, DebuggerTextObject)>,
}

pub struct ImportsConfig {
    pub query: Query,
    pub import_ix: u32,
    pub name_ix: Option<u32>,
    pub namespace_ix: Option<u32>,
    pub source_ix: Option<u32>,
    pub list_ix: Option<u32>,
    pub wildcard_ix: Option<u32>,
    pub alias_ix: Option<u32>,
}

enum Capture<'a> {
    Required(&'static str, &'a mut u32),
    Optional(&'static str, &'a mut Option<u32>),
}

fn populate_capture_indices(
    query: &Query,
    language_name: &LanguageName,
    query_type: &str,
    expected_prefixes: &[&str],
    captures: &mut [Capture<'_>],
) -> bool {
    let mut found_required_indices = Vec::new();
    'outer: for (ix, name) in query.capture_names().iter().enumerate() {
        for (required_ix, capture) in captures.iter_mut().enumerate() {
            match capture {
                Capture::Required(capture_name, index) if capture_name == name => {
                    **index = ix as u32;
                    found_required_indices.push(required_ix);
                    continue 'outer;
                }
                Capture::Optional(capture_name, index) if capture_name == name => {
                    **index = Some(ix as u32);
                    continue 'outer;
                }
                _ => {}
            }
        }
        if !name.starts_with("_")
            && !expected_prefixes
                .iter()
                .any(|&prefix| name.starts_with(prefix))
        {
            log::warn!(
                "unrecognized capture name '{}' in {} {} TreeSitter query \
                (suppress this warning by prefixing with '_')",
                name,
                language_name,
                query_type
            );
        }
    }
    let mut missing_required_captures = Vec::new();
    for (capture_ix, capture) in captures.iter().enumerate() {
        if let Capture::Required(capture_name, _) = capture
            && !found_required_indices.contains(&capture_ix)
        {
            missing_required_captures.push(*capture_name);
        }
    }
    let success = missing_required_captures.is_empty();
    if !success {
        log::error!(
            "missing required capture(s) in {} {} TreeSitter query: {}",
            language_name,
            query_type,
            missing_required_captures.join(", ")
        );
    }
    success
}

impl Grammar {
    pub fn new(ts_language: tree_sitter::Language) -> Self {
        Self {
            id: GrammarId::new(),
            highlights_config: None,
            brackets_config: None,
            outline_config: None,
            text_object_config: None,
            indents_config: None,
            injection_config: None,
            override_config: None,
            redactions_config: None,
            runnable_config: None,
            error_query: Query::new(&ts_language, "(ERROR) @error").ok(),
            debug_variables_config: None,
            imports_config: None,
            ts_language,
            highlight_map: Default::default(),
        }
    }

    pub fn id(&self) -> GrammarId {
        self.id
    }

    pub fn highlight_map(&self) -> HighlightMap {
        self.highlight_map.lock().clone()
    }

    pub fn highlight_id_for_name(&self, name: &str) -> Option<HighlightId> {
        let capture_id = self
            .highlights_config
            .as_ref()?
            .query
            .capture_index_for_name(name)?;
        Some(self.highlight_map.lock().get(capture_id))
    }

    pub fn debug_variables_config(&self) -> Option<&DebugVariablesConfig> {
        self.debug_variables_config.as_ref()
    }

    pub fn imports_config(&self) -> Option<&ImportsConfig> {
        self.imports_config.as_ref()
    }

    /// Load all queries from `LanguageQueries` into this grammar, mutating the
    /// associated `LanguageConfig` (the override query clears
    /// `brackets.disabled_scopes_by_bracket_ix`).
    pub fn with_queries(
        mut self,
        queries: LanguageQueries,
        config: &mut LanguageConfig,
    ) -> Result<Self> {
        let name = &config.name;
        if let Some(query) = queries.highlights {
            self = self
                .with_highlights_query(query.as_ref())
                .context("Error loading highlights query")?;
        }
        if let Some(query) = queries.brackets {
            self = self
                .with_brackets_query(query.as_ref(), name)
                .context("Error loading brackets query")?;
        }
        if let Some(query) = queries.indents {
            self = self
                .with_indents_query(query.as_ref(), name)
                .context("Error loading indents query")?;
        }
        if let Some(query) = queries.outline {
            self = self
                .with_outline_query(query.as_ref(), name)
                .context("Error loading outline query")?;
        }
        if let Some(query) = queries.injections {
            self = self
                .with_injection_query(query.as_ref(), name)
                .context("Error loading injection query")?;
        }
        if let Some(query) = queries.overrides {
            self = self
                .with_override_query(
                    query.as_ref(),
                    name,
                    &config.overrides,
                    &mut config.brackets,
                    &config.scope_opt_in_language_servers,
                )
                .context("Error loading override query")?;
        }
        if let Some(query) = queries.redactions {
            self = self
                .with_redaction_query(query.as_ref(), name)
                .context("Error loading redaction query")?;
        }
        if let Some(query) = queries.runnables {
            self = self
                .with_runnable_query(query.as_ref())
                .context("Error loading runnables query")?;
        }
        if let Some(query) = queries.text_objects {
            self = self
                .with_text_object_query(query.as_ref(), name)
                .context("Error loading textobject query")?;
        }
        if let Some(query) = queries.debugger {
            self = self
                .with_debug_variables_query(query.as_ref(), name)
                .context("Error loading debug variables query")?;
        }
        if let Some(query) = queries.imports {
            self = self
                .with_imports_query(query.as_ref(), name)
                .context("Error loading imports query")?;
        }
        Ok(self)
    }

    pub fn with_highlights_query(mut self, source: &str) -> Result<Self> {
        let query = Query::new(&self.ts_language, source)?;

        let mut identifier_capture_indices = Vec::new();
        for name in [
            "variable",
            "constant",
            "constructor",
            "function",
            "function.method",
            "function.method.call",
            "function.special",
            "property",
            "type",
            "type.interface",
        ] {
            identifier_capture_indices.extend(query.capture_index_for_name(name));
        }

        self.highlights_config = Some(HighlightsConfig {
            query,
            identifier_capture_indices,
        });

        Ok(self)
    }

    pub fn with_runnable_query(mut self, source: &str) -> Result<Self> {
        let query = Query::new(&self.ts_language, source)?;
        let extra_captures: Vec<_> = query
            .capture_names()
            .iter()
            .map(|&name| match name {
                "run" => RunnableCapture::Run,
                name => RunnableCapture::Named(name.to_string().into()),
            })
            .collect();

        self.runnable_config = Some(RunnableConfig {
            extra_captures,
            query,
        });

        Ok(self)
    }

    pub fn with_outline_query(
        mut self,
        source: &str,
        language_name: &LanguageName,
    ) -> Result<Self> {
        let query = Query::new(&self.ts_language, source)?;
        let mut item_capture_ix = 0;
        let mut name_capture_ix = 0;
        let mut context_capture_ix = None;
        let mut extra_context_capture_ix = None;
        let mut open_capture_ix = None;
        let mut close_capture_ix = None;
        let mut annotation_capture_ix = None;
        if populate_capture_indices(
            &query,
            language_name,
            "outline",
            &[],
            &mut [
                Capture::Required("item", &mut item_capture_ix),
                Capture::Required("name", &mut name_capture_ix),
                Capture::Optional("context", &mut context_capture_ix),
                Capture::Optional("context.extra", &mut extra_context_capture_ix),
                Capture::Optional("open", &mut open_capture_ix),
                Capture::Optional("close", &mut close_capture_ix),
                Capture::Optional("annotation", &mut annotation_capture_ix),
            ],
        ) {
            self.outline_config = Some(OutlineConfig {
                query,
                item_capture_ix,
                name_capture_ix,
                context_capture_ix,
                extra_context_capture_ix,
                open_capture_ix,
                close_capture_ix,
                annotation_capture_ix,
            });
        }
        Ok(self)
    }

    pub fn with_text_object_query(
        mut self,
        source: &str,
        language_name: &LanguageName,
    ) -> Result<Self> {
        let query = Query::new(&self.ts_language, source)?;

        let mut text_objects_by_capture_ix = Vec::new();
        for (ix, name) in query.capture_names().iter().enumerate() {
            if let Some(text_object) = TextObject::from_capture_name(name) {
                text_objects_by_capture_ix.push((ix as u32, text_object));
            } else {
                log::warn!(
                    "unrecognized capture name '{}' in {} textobjects TreeSitter query",
                    name,
                    language_name,
                );
            }
        }

        self.text_object_config = Some(TextObjectConfig {
            query,
            text_objects_by_capture_ix,
        });
        Ok(self)
    }

    pub fn with_debug_variables_query(
        mut self,
        source: &str,
        language_name: &LanguageName,
    ) -> Result<Self> {
        let query = Query::new(&self.ts_language, source)?;

        let mut objects_by_capture_ix = Vec::new();
        for (ix, name) in query.capture_names().iter().enumerate() {
            if let Some(text_object) = DebuggerTextObject::from_capture_name(name) {
                objects_by_capture_ix.push((ix as u32, text_object));
            } else {
                log::warn!(
                    "unrecognized capture name '{}' in {} debugger TreeSitter query",
                    name,
                    language_name,
                );
            }
        }

        self.debug_variables_config = Some(DebugVariablesConfig {
            query,
            objects_by_capture_ix,
        });
        Ok(self)
    }

    pub fn with_imports_query(
        mut self,
        source: &str,
        language_name: &LanguageName,
    ) -> Result<Self> {
        let query = Query::new(&self.ts_language, source)?;

        let mut import_ix = 0;
        let mut name_ix = None;
        let mut namespace_ix = None;
        let mut source_ix = None;
        let mut list_ix = None;
        let mut wildcard_ix = None;
        let mut alias_ix = None;
        if populate_capture_indices(
            &query,
            language_name,
            "imports",
            &[],
            &mut [
                Capture::Required("import", &mut import_ix),
                Capture::Optional("name", &mut name_ix),
                Capture::Optional("namespace", &mut namespace_ix),
                Capture::Optional("source", &mut source_ix),
                Capture::Optional("list", &mut list_ix),
                Capture::Optional("wildcard", &mut wildcard_ix),
                Capture::Optional("alias", &mut alias_ix),
            ],
        ) {
            self.imports_config = Some(ImportsConfig {
                query,
                import_ix,
                name_ix,
                namespace_ix,
                source_ix,
                list_ix,
                wildcard_ix,
                alias_ix,
            });
        }
        Ok(self)
    }

    pub fn with_brackets_query(
        mut self,
        source: &str,
        language_name: &LanguageName,
    ) -> Result<Self> {
        let query = Query::new(&self.ts_language, source)?;
        let mut open_capture_ix = 0;
        let mut close_capture_ix = 0;
        if populate_capture_indices(
            &query,
            language_name,
            "brackets",
            &[],
            &mut [
                Capture::Required("open", &mut open_capture_ix),
                Capture::Required("close", &mut close_capture_ix),
            ],
        ) {
            let patterns = (0..query.pattern_count())
                .map(|ix| {
                    let mut config = BracketsPatternConfig::default();
                    for setting in query.property_settings(ix) {
                        let setting_key = setting.key.as_ref();
                        if setting_key == "newline.only" {
                            config.newline_only = true
                        }
                        if setting_key == "rainbow.exclude" {
                            config.rainbow_exclude = true
                        }
                    }
                    config
                })
                .collect();
            self.brackets_config = Some(BracketsConfig {
                query,
                open_capture_ix,
                close_capture_ix,
                patterns,
            });
        }
        Ok(self)
    }

    pub fn with_indents_query(
        mut self,
        source: &str,
        language_name: &LanguageName,
    ) -> Result<Self> {
        let query = Query::new(&self.ts_language, source)?;
        let mut indent_capture_ix = 0;
        let mut start_capture_ix = None;
        let mut end_capture_ix = None;
        let mut outdent_capture_ix = None;
        if populate_capture_indices(
            &query,
            language_name,
            "indents",
            &["start."],
            &mut [
                Capture::Required("indent", &mut indent_capture_ix),
                Capture::Optional("start", &mut start_capture_ix),
                Capture::Optional("end", &mut end_capture_ix),
                Capture::Optional("outdent", &mut outdent_capture_ix),
            ],
        ) {
            let mut suffixed_start_captures = HashMap::default();
            for (ix, name) in query.capture_names().iter().enumerate() {
                if let Some(suffix) = name.strip_prefix("start.") {
                    suffixed_start_captures.insert(ix as u32, suffix.to_owned().into());
                }
            }

            self.indents_config = Some(IndentConfig {
                query,
                indent_capture_ix,
                start_capture_ix,
                end_capture_ix,
                outdent_capture_ix,
                suffixed_start_captures,
            });
        }
        Ok(self)
    }

    pub fn with_injection_query(
        mut self,
        source: &str,
        language_name: &LanguageName,
    ) -> Result<Self> {
        let query = Query::new(&self.ts_language, source)?;
        let mut language_capture_ix = None;
        let mut injection_language_capture_ix = None;
        let mut content_capture_ix = None;
        let mut injection_content_capture_ix = None;
        if populate_capture_indices(
            &query,
            language_name,
            "injections",
            &[],
            &mut [
                Capture::Optional("language", &mut language_capture_ix),
                Capture::Optional("injection.language", &mut injection_language_capture_ix),
                Capture::Optional("content", &mut content_capture_ix),
                Capture::Optional("injection.content", &mut injection_content_capture_ix),
            ],
        ) {
            language_capture_ix = match (language_capture_ix, injection_language_capture_ix) {
                (None, Some(ix)) => Some(ix),
                (Some(_), Some(_)) => {
                    anyhow::bail!("both language and injection.language captures are present");
                }
                _ => language_capture_ix,
            };
            content_capture_ix = match (content_capture_ix, injection_content_capture_ix) {
                (None, Some(ix)) => Some(ix),
                (Some(_), Some(_)) => {
                    anyhow::bail!("both content and injection.content captures are present")
                }
                _ => content_capture_ix,
            };
            let patterns = (0..query.pattern_count())
                .map(|ix| {
                    let mut config = InjectionPatternConfig::default();
                    for setting in query.property_settings(ix) {
                        match setting.key.as_ref() {
                            "language" | "injection.language" => {
                                config.language.clone_from(&setting.value);
                            }
                            "combined" | "injection.combined" => {
                                config.combined = true;
                            }
                            _ => {}
                        }
                    }
                    config
                })
                .collect();
            if let Some(content_capture_ix) = content_capture_ix {
                self.injection_config = Some(InjectionConfig {
                    query,
                    language_capture_ix,
                    content_capture_ix,
                    patterns,
                });
            } else {
                log::error!(
                    "missing required capture in injections {} TreeSitter query: \
                    content or injection.content",
                    language_name,
                );
            }
        }
        Ok(self)
    }

    pub fn with_override_query(
        mut self,
        source: &str,
        language_name: &LanguageName,
        overrides: &HashMap<String, LanguageConfigOverride>,
        brackets: &mut BracketPairConfig,
        scope_opt_in_language_servers: &[LanguageServerName],
    ) -> Result<Self> {
        let query = Query::new(&self.ts_language, source)?;

        let mut override_configs_by_id = HashMap::default();
        for (ix, mut name) in query.capture_names().iter().copied().enumerate() {
            let mut range_is_inclusive = false;
            if name.starts_with('_') {
                continue;
            }
            if let Some(prefix) = name.strip_suffix(".inclusive") {
                name = prefix;
                range_is_inclusive = true;
            }

            let value = overrides.get(name).cloned().unwrap_or_default();
            for server_name in &value.opt_into_language_servers {
                if !scope_opt_in_language_servers.contains(server_name) {
                    util::debug_panic!(
                        "Server {server_name:?} has been opted-in by scope {name:?} but has not been marked as an opt-in server"
                    );
                }
            }

            override_configs_by_id.insert(
                ix as u32,
                OverrideEntry {
                    name: name.to_string(),
                    range_is_inclusive,
                    value,
                },
            );
        }

        let referenced_override_names = overrides
            .keys()
            .chain(brackets.disabled_scopes_by_bracket_ix.iter().flatten());

        for referenced_name in referenced_override_names {
            if !override_configs_by_id
                .values()
                .any(|entry| entry.name == *referenced_name)
            {
                anyhow::bail!(
                    "language {:?} has overrides in config not in query: {referenced_name:?}",
                    language_name
                );
            }
        }

        for entry in override_configs_by_id.values_mut() {
            entry.value.disabled_bracket_ixs = brackets
                .disabled_scopes_by_bracket_ix
                .iter()
                .enumerate()
                .filter_map(|(ix, disabled_scope_names)| {
                    if disabled_scope_names.contains(&entry.name) {
                        Some(ix as u16)
                    } else {
                        None
                    }
                })
                .collect();
        }

        brackets.disabled_scopes_by_bracket_ix.clear();

        self.override_config = Some(OverrideConfig {
            query,
            values: override_configs_by_id,
        });
        Ok(self)
    }

    pub fn with_redaction_query(
        mut self,
        source: &str,
        language_name: &LanguageName,
    ) -> Result<Self> {
        let query = Query::new(&self.ts_language, source)?;
        let mut redaction_capture_ix = 0;
        if populate_capture_indices(
            &query,
            language_name,
            "redactions",
            &[],
            &mut [Capture::Required("redact", &mut redaction_capture_ix)],
        ) {
            self.redactions_config = Some(RedactionConfig {
                query,
                redaction_capture_ix,
            });
        }
        Ok(self)
    }
}
