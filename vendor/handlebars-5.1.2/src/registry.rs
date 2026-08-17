use std::borrow::Cow;
use std::collections::HashMap;
use std::fmt::{self, Debug, Formatter};
use std::io::{Error as IoError, Write};
use std::path::Path;
use std::sync::Arc;

use serde::Serialize;

use crate::context::Context;
use crate::decorators::{self, DecoratorDef};
#[cfg(feature = "script_helper")]
use crate::error::ScriptError;
use crate::error::{RenderError, RenderErrorReason, TemplateError};
use crate::helpers::{self, HelperDef};
use crate::output::{Output, StringOutput, WriteOutput};
use crate::render::{RenderContext, Renderable};
use crate::sources::{FileSource, Source};
use crate::support::str::{self, StringWriter};
use crate::template::{Template, TemplateOptions};

#[cfg(feature = "dir_source")]
use walkdir::WalkDir;

#[cfg(feature = "script_helper")]
use rhai::Engine;

#[cfg(feature = "script_helper")]
use crate::helpers::scripting::ScriptHelper;

#[cfg(feature = "rust-embed")]
use crate::sources::LazySource;
#[cfg(feature = "rust-embed")]
use rust_embed::RustEmbed;

/// This type represents an *escape fn*, that is a function whose purpose it is
/// to escape potentially problematic characters in a string.
///
/// An *escape fn* is represented as a `Box` to avoid unnecessary type
/// parameters (and because traits cannot be aliased using `type`).
pub type EscapeFn = Arc<dyn Fn(&str) -> String + Send + Sync>;

/// The default *escape fn* replaces the characters `&"<>`
/// with the equivalent html / xml entities.
pub fn html_escape(data: &str) -> String {
    str::escape_html(data)
}

/// `EscapeFn` that does not change anything. Useful when using in a non-html
/// environment.
pub fn no_escape(data: &str) -> String {
    data.to_owned()
}

/// The single entry point of your Handlebars templates
///
/// It maintains compiled templates and registered helpers.
#[derive(Clone)]
pub struct Registry<'reg> {
    templates: HashMap<String, Template>,

    helpers: HashMap<String, Arc<dyn HelperDef + Send + Sync + 'reg>>,
    decorators: HashMap<String, Arc<dyn DecoratorDef + Send + Sync + 'reg>>,

    escape_fn: EscapeFn,
    strict_mode: bool,
    dev_mode: bool,
    prevent_indent: bool,
    #[cfg(feature = "script_helper")]
    pub(crate) engine: Arc<Engine>,

    template_sources:
        HashMap<String, Arc<dyn Source<Item = String, Error = IoError> + Send + Sync + 'reg>>,
    #[cfg(feature = "script_helper")]
    script_sources:
        HashMap<String, Arc<dyn Source<Item = String, Error = IoError> + Send + Sync + 'reg>>,
}

impl<'reg> Debug for Registry<'reg> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        f.debug_struct("Handlebars")
            .field("templates", &self.templates)
            .field("helpers", &self.helpers.keys())
            .field("decorators", &self.decorators.keys())
            .field("strict_mode", &self.strict_mode)
            .field("dev_mode", &self.dev_mode)
            .finish()
    }
}

impl<'reg> Default for Registry<'reg> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(feature = "script_helper")]
fn rhai_engine() -> Engine {
    Engine::new()
}

/// Options for importing template files from a directory.
#[cfg(feature = "dir_source")]
pub struct DirectorySourceOptions {
    /// The name extension for template files
    pub tpl_extension: String,
    /// Whether to include hidden files (file name that starts with `.`)
    pub hidden: bool,
    /// Whether to include temporary files (file name that starts with `#`)
    pub temporary: bool,
}

#[cfg(feature = "dir_source")]
impl DirectorySourceOptions {
    fn ignore_file(&self, name: &str) -> bool {
        self.ignored_as_hidden_file(name) || self.ignored_as_temporary_file(name)
    }

    #[inline]
    fn ignored_as_hidden_file(&self, name: &str) -> bool {
        !self.hidden && name.starts_with('.')
    }

    #[inline]
    fn ignored_as_temporary_file(&self, name: &str) -> bool {
        !self.temporary && name.starts_with('#')
    }
}

#[cfg(feature = "dir_source")]
impl Default for DirectorySourceOptions {
    fn default() -> Self {
        DirectorySourceOptions {
            tpl_extension: ".hbs".to_owned(),
            hidden: false,
            temporary: false,
        }
    }
}

impl<'reg> Registry<'reg> {
    pub fn new() -> Registry<'reg> {
        let r = Registry {
            templates: HashMap::new(),
            template_sources: HashMap::new(),
            helpers: HashMap::new(),
            decorators: HashMap::new(),
            escape_fn: Arc::new(html_escape),
            strict_mode: false,
            dev_mode: false,
            prevent_indent: false,
            #[cfg(feature = "script_helper")]
            engine: Arc::new(rhai_engine()),
            #[cfg(feature = "script_helper")]
            script_sources: HashMap::new(),
        };

        r.setup_builtins()
    }

    fn setup_builtins(mut self) -> Registry<'reg> {
        self.register_helper("if", Box::new(helpers::IF_HELPER));
        self.register_helper("unless", Box::new(helpers::UNLESS_HELPER));
        self.register_helper("each", Box::new(helpers::EACH_HELPER));
        self.register_helper("with", Box::new(helpers::WITH_HELPER));
        self.register_helper("lookup", Box::new(helpers::LOOKUP_HELPER));
        self.register_helper("raw", Box::new(helpers::RAW_HELPER));
        self.register_helper("log", Box::new(helpers::LOG_HELPER));

        self.register_helper("eq", Box::new(helpers::helper_extras::eq));
        self.register_helper("ne", Box::new(helpers::helper_extras::ne));
        self.register_helper("gt", Box::new(helpers::helper_extras::gt));
        self.register_helper("gte", Box::new(helpers::helper_extras::gte));
        self.register_helper("lt", Box::new(helpers::helper_extras::lt));
        self.register_helper("lte", Box::new(helpers::helper_extras::lte));
        self.register_helper("and", Box::new(helpers::helper_extras::and));
        self.register_helper("or", Box::new(helpers::helper_extras::or));
        self.register_helper("not", Box::new(helpers::helper_extras::not));
        self.register_helper("len", Box::new(helpers::helper_extras::len));

        #[cfg(feature = "string_helpers")]
        self.register_string_helpers();

        self.register_decorator("inline", Box::new(decorators::INLINE_DECORATOR));
        self
    }

    /// Enable or disable handlebars strict mode
    ///
    /// By default, handlebars renders empty string for value that
    /// undefined or never exists. Since rust is a static type
    /// language, we offer strict mode in handlebars-rust.  In strict
    /// mode, if you were to render a value that doesn't exist, a
    /// `RenderError` will be raised.
    pub fn set_strict_mode(&mut self, enabled: bool) {
        self.strict_mode = enabled;
    }

    /// Return strict mode state, default is false.
    ///
    /// By default, handlebars renders empty string for value that
    /// undefined or never exists. Since rust is a static type
    /// language, we offer strict mode in handlebars-rust.  In strict
    /// mode, if you were access a value that doesn't exist, a
    /// `RenderError` will be raised.
    pub fn strict_mode(&self) -> bool {
        self.strict_mode
    }

    /// Return dev mode state, default is false
    ///
    /// With dev mode turned on, handlebars enables a set of development
    /// friendly features, that may affect its performance.
    pub fn dev_mode(&self) -> bool {
        self.dev_mode
    }

    /// Enable or disable dev mode
    ///
    /// With dev mode turned on, handlebars enables a set of development
    /// friendly features, that may affect its performance.
    ///
    /// **Note that you have to enable dev mode before adding templates to
    /// the registry**. Otherwise it won't take effect at all.
    pub fn set_dev_mode(&mut self, enabled: bool) {
        self.dev_mode = enabled;

        // clear template source when disabling dev mode
        if !enabled {
            self.template_sources.clear();
        }
    }

    /// Enable or disable indent for partial include tag `{{>}}`
    ///
    /// By default handlebars keeps indent whitespaces for partial
    /// include tag, to change this behaviour, set this toggle to `true`.
    pub fn set_prevent_indent(&mut self, enable: bool) {
        self.prevent_indent = enable;
    }

    /// Return state for `prevent_indent` option, default to `false`.
    pub fn prevent_indent(&self) -> bool {
        self.prevent_indent
    }

    /// Register a `Template`
    ///
    /// This is infallible since the template has already been parsed and
    /// insert cannot fail. If there is an existing template with this name it
    /// will be overwritten.
    ///
    /// Dev mode doesn't apply for pre-compiled template because it's lifecycle
    /// is not managed by the registry.
    pub fn register_template(&mut self, name: &str, tpl: Template) {
        self.templates.insert(name.to_string(), tpl);
    }

    /// Register a template string
    ///
    /// Returns `TemplateError` if there is syntax error on parsing the template.
    pub fn register_template_string<S>(
        &mut self,
        name: &str,
        tpl_str: S,
    ) -> Result<(), TemplateError>
    where
        S: AsRef<str>,
    {
        let template = Template::compile2(
            tpl_str.as_ref(),
            TemplateOptions {
                name: Some(name.to_owned()),
                prevent_indent: self.prevent_indent,
            },
        )?;
        self.register_template(name, template);
        Ok(())
    }

    /// Register a partial string
    ///
    /// A named partial will be added to the registry. It will overwrite template with
    /// same name. Currently a registered partial is just identical to a template.
    pub fn register_partial<S>(&mut self, name: &str, partial_str: S) -> Result<(), TemplateError>
    where
        S: AsRef<str>,
    {
        self.register_template_string(name, partial_str)
    }

    /// Register a template from a path on file system
    ///
    /// If dev mode is enabled, the registry will keep reading the template file
    /// from file system everytime it's visited.
    pub fn register_template_file<P>(
        &mut self,
        name: &str,
        tpl_path: P,
    ) -> Result<(), TemplateError>
    where
        P: AsRef<Path>,
    {
        let source = FileSource::new(tpl_path.as_ref().into());
        let template_string = source
            .load()
            .map_err(|err| TemplateError::from((err, name.to_owned())))?;

        self.register_template_string(name, template_string)?;
        if self.dev_mode {
            self.template_sources
                .insert(name.to_owned(), Arc::new(source));
        }

        Ok(())
    }

    /// Register templates from a directory
    ///
    /// * `tpl_extension`: the template file extension
    /// * `dir_path`: the path of directory
    ///
    /// Hidden files and tempfile (starts with `#`) will be ignored by default.
    /// Set `DirectorySourceOptions` to something other than `DirectorySourceOptions::default()` to adjust this.
    /// All registered templates will use their relative path to determine their template name.
    /// For example, when `dir_path` is `templates/` and `DirectorySourceOptions.tpl_extension` is `.hbs`, the file
    /// `templates/some/path/file.hbs` will be registered as `some/path/file`.
    ///
    /// This method is not available by default.
    /// You will need to enable the `dir_source` feature to use it.
    ///
    /// When dev_mode is enabled, like with `register_template_file`, templates are reloaded
    /// from the file system every time they're visited.
    #[cfg(feature = "dir_source")]
    #[cfg_attr(docsrs, doc(cfg(feature = "dir_source")))]
    pub fn register_templates_directory<P>(
        &mut self,
        dir_path: P,
        options: DirectorySourceOptions,
    ) -> Result<(), TemplateError>
    where
        P: AsRef<Path>,
    {
        let dir_path = dir_path.as_ref();

        let walker = WalkDir::new(dir_path);
        let dir_iter = walker
            .min_depth(1)
            .into_iter()
            .filter_map(|e| e.ok().map(|e| e.into_path()))
            // Checks if extension matches
            .filter(|tpl_path| {
                tpl_path
                    .to_string_lossy()
                    .ends_with(options.tpl_extension.as_str())
            })
            // Rejects any hidden or temporary files.
            .filter(|tpl_path| {
                tpl_path
                    .file_stem()
                    .map(|stem| !options.ignore_file(&stem.to_string_lossy()))
                    .unwrap_or(false)
            })
            .filter_map(|tpl_path| {
                tpl_path
                    .strip_prefix(dir_path)
                    .ok()
                    .map(|tpl_canonical_name| {
                        let tpl_name = tpl_canonical_name
                            .components()
                            .map(|component| component.as_os_str().to_string_lossy())
                            .collect::<Vec<_>>()
                            .join("/");

                        tpl_name
                            .strip_suffix(options.tpl_extension.as_str())
                            .map(|s| s.to_owned())
                            .unwrap_or(tpl_name)
                    })
                    .map(|tpl_canonical_name| (tpl_canonical_name, tpl_path))
            });

        for (tpl_canonical_name, tpl_path) in dir_iter {
            self.register_template_file(&tpl_canonical_name, &tpl_path)?;
        }

        Ok(())
    }

    /// Register templates using a
    /// [RustEmbed](https://github.com/pyros2097/rust-embed) type
    /// Calls register_embed_templates_with_extension with empty extension.
    ///
    /// File names from embed struct are used as template name.
    ///
    /// ```skip
    /// #[derive(RustEmbed)]
    /// #[folder = "templates"]
    /// #[include = "*.hbs"]
    /// struct Assets;
    ///
    /// let mut hbs = Handlebars::new();
    /// hbs.register_embed_templates::<Assets>();
    /// ```
    ///
    #[cfg(feature = "rust-embed")]
    #[cfg_attr(docsrs, doc(cfg(feature = "rust-embed")))]
    pub fn register_embed_templates<E>(&mut self) -> Result<(), TemplateError>
    where
        E: RustEmbed,
    {
        self.register_embed_templates_with_extension::<E>("")
    }

    /// Register templates using a
    /// [RustEmbed](https://github.com/pyros2097/rust-embed) type
    /// * `tpl_extension`: the template file extension
    ///
    /// File names from embed struct are used as template name, but extension is stripped.
    ///
    /// When dev_mode enabled templates is reloaded
    /// from embed struct everytime it's visied.
    ///
    /// ```skip
    /// #[derive(RustEmbed)]
    /// #[folder = "templates"]
    /// struct Assets;
    ///
    /// let mut hbs = Handlebars::new();
    /// hbs.register_embed_templates_with_extension::<Assets>(".hbs");
    /// ```
    ///
    #[cfg(feature = "rust-embed")]
    #[cfg_attr(docsrs, doc(cfg(feature = "rust-embed")))]
    pub fn register_embed_templates_with_extension<E>(
        &mut self,
        tpl_extension: &str,
    ) -> Result<(), TemplateError>
    where
        E: RustEmbed,
    {
        for file_name in E::iter().filter(|x| x.ends_with(tpl_extension)) {
            let tpl_name = file_name
                .strip_suffix(tpl_extension)
                .unwrap_or(&file_name)
                .to_owned();
            let source = LazySource::new(move || {
                E::get(&file_name)
                    .map(|file| file.data.to_vec())
                    .and_then(|data| String::from_utf8(data).ok())
            });
            let tpl_content = source
                .load()
                .map_err(|e| (e, "Template load error".to_owned()))?;
            self.register_template_string(&tpl_name, &tpl_content)?;

            if self.dev_mode {
                self.template_sources.insert(tpl_name, Arc::new(source));
            }
        }
        Ok(())
    }

    /// Remove a template from the registry
    pub fn unregister_template(&mut self, name: &str) {
        self.templates.remove(name);
        self.template_sources.remove(name);
    }

    /// Register a helper
    pub fn register_helper(&mut self, name: &str, def: Box<dyn HelperDef + Send + Sync + 'reg>) {
        self.helpers.insert(name.to_string(), def.into());
    }

    /// Register a [rhai](https://docs.rs/rhai/) script as handlebars helper
    ///
    /// Currently only simple helpers are supported. You can do computation or
    /// string formatting with rhai script.
    ///
    /// Helper parameters and hash are available in rhai script as array `params`
    /// and map `hash`. Example script:
    ///
    /// ```handlebars
    /// {{percent 0.34 label="%"}}
    /// ```
    ///
    /// ```rhai
    /// // percent.rhai
    /// let value = params[0];
    /// let label = hash["label"];
    ///
    /// (value * 100).to_string() + label
    /// ```
    ///
    #[cfg(feature = "script_helper")]
    #[cfg_attr(docsrs, doc(cfg(feature = "script_helper")))]
    pub fn register_script_helper(&mut self, name: &str, script: &str) -> Result<(), ScriptError> {
        let compiled = self.engine.compile(script)?;
        let script_helper = ScriptHelper { script: compiled };
        self.helpers
            .insert(name.to_string(), Arc::new(script_helper));
        Ok(())
    }

    /// Register a [rhai](https://docs.rs/rhai/) script from file
    ///
    /// When dev mode is enable, script file is reloaded from original file
    /// everytime it is called.
    #[cfg(feature = "script_helper")]
    #[cfg_attr(docsrs, doc(cfg(feature = "script_helper")))]
    pub fn register_script_helper_file<P>(
        &mut self,
        name: &str,
        script_path: P,
    ) -> Result<(), ScriptError>
    where
        P: AsRef<Path>,
    {
        let source = FileSource::new(script_path.as_ref().into());
        let script = source.load()?;

        self.script_sources
            .insert(name.to_owned(), Arc::new(source));
        self.register_script_helper(name, &script)
    }

    /// Borrow a read-only reference to current rhai engine
    #[cfg(feature = "script_helper")]
    #[cfg_attr(docsrs, doc(cfg(feature = "script_helper")))]
    pub fn engine(&self) -> &Engine {
        self.engine.as_ref()
    }

    /// Set a custom rhai engine for the registry.
    ///
    /// *Note that* you need to set custom engine before adding scripts.
    #[cfg(feature = "script_helper")]
    #[cfg_attr(docsrs, doc(cfg(feature = "script_helper")))]
    pub fn set_engine(&mut self, engine: Engine) {
        self.engine = Arc::new(engine);
    }

    /// Register a decorator
    pub fn register_decorator(
        &mut self,
        name: &str,
        def: Box<dyn DecoratorDef + Send + Sync + 'reg>,
    ) {
        self.decorators.insert(name.to_string(), def.into());
    }

    /// Register a new *escape fn* to be used from now on by this registry.
    pub fn register_escape_fn<F: 'static + Fn(&str) -> String + Send + Sync>(
        &mut self,
        escape_fn: F,
    ) {
        self.escape_fn = Arc::new(escape_fn);
    }

    /// Restore the default *escape fn*.
    pub fn unregister_escape_fn(&mut self) {
        self.escape_fn = Arc::new(html_escape);
    }

    /// Get a reference to the current *escape fn*.
    pub fn get_escape_fn(&self) -> &dyn Fn(&str) -> String {
        self.escape_fn.as_ref()
    }

    /// Return `true` if a template is registered for the given name
    pub fn has_template(&self, name: &str) -> bool {
        self.get_template(name).is_some()
    }

    /// Return a registered template,
    pub fn get_template(&self, name: &str) -> Option<&Template> {
        self.templates.get(name)
    }

    #[inline]
    pub(crate) fn get_or_load_template_optional(
        &'reg self,
        name: &str,
    ) -> Option<Result<Cow<'reg, Template>, RenderError>> {
        if let (true, Some(source)) = (self.dev_mode, self.template_sources.get(name)) {
            let r = source
                .load()
                .map_err(|e| TemplateError::from((e, name.to_owned())))
                .and_then(|tpl_str| {
                    Template::compile2(
                        tpl_str.as_ref(),
                        TemplateOptions {
                            name: Some(name.to_owned()),
                            prevent_indent: self.prevent_indent,
                        },
                    )
                })
                .map(Cow::Owned)
                .map_err(RenderError::from);
            Some(r)
        } else {
            self.templates.get(name).map(|t| Ok(Cow::Borrowed(t)))
        }
    }

    #[inline]
    pub(crate) fn get_or_load_template(
        &'reg self,
        name: &str,
    ) -> Result<Cow<'reg, Template>, RenderError> {
        if let Some(result) = self.get_or_load_template_optional(name) {
            result
        } else {
            Err(RenderErrorReason::TemplateNotFound(name.to_owned()).into())
        }
    }

    /// Return a registered helper
    #[inline]
    pub(crate) fn get_or_load_helper(
        &'reg self,
        name: &str,
    ) -> Result<Option<Arc<dyn HelperDef + Send + Sync + 'reg>>, RenderError> {
        #[cfg(feature = "script_helper")]
        if let (true, Some(source)) = (self.dev_mode, self.script_sources.get(name)) {
            return source
                .load()
                .map_err(ScriptError::from)
                .and_then(|s| {
                    let helper = Box::new(ScriptHelper {
                        script: self.engine.compile(s)?,
                    }) as Box<dyn HelperDef + Send + Sync>;
                    Ok(Some(helper.into()))
                })
                .map_err(|e| RenderError::from(RenderErrorReason::from(e)));
        }

        Ok(self.helpers.get(name).cloned())
    }

    #[inline]
    pub(crate) fn has_helper(&self, name: &str) -> bool {
        self.helpers.contains_key(name)
    }

    /// Return a registered decorator
    #[inline]
    pub(crate) fn get_decorator(
        &self,
        name: &str,
    ) -> Option<&(dyn DecoratorDef + Send + Sync + 'reg)> {
        self.decorators.get(name).map(|v| v.as_ref())
    }

    /// Return all templates registered
    ///
    /// **Note that** in dev mode, the template returned from this method may
    /// not reflect its latest state. This method doesn't try to reload templates
    /// from its source.
    pub fn get_templates(&self) -> &HashMap<String, Template> {
        &self.templates
    }

    /// Unregister all templates
    pub fn clear_templates(&mut self) {
        self.templates.clear();
        self.template_sources.clear();
    }

    #[inline]
    fn render_to_output<O>(
        &self,
        name: &str,
        ctx: &Context,
        output: &mut O,
    ) -> Result<(), RenderError>
    where
        O: Output,
    {
        self.get_or_load_template(name).and_then(|t| {
            let mut render_context = RenderContext::new(t.name.as_ref());
            t.render(self, ctx, &mut render_context, output)
        })
    }

    /// Render a registered template with some data into a string
    ///
    /// * `name` is the template name you registered previously
    /// * `data` is the data that implements `serde::Serialize`
    ///
    /// Returns rendered string or a struct with error information
    pub fn render<T>(&self, name: &str, data: &T) -> Result<String, RenderError>
    where
        T: Serialize,
    {
        let mut output = StringOutput::new();
        let ctx = Context::wraps(data)?;
        self.render_to_output(name, &ctx, &mut output)?;
        output.into_string().map_err(RenderError::from)
    }

    /// Render a registered template with reused context
    pub fn render_with_context(&self, name: &str, ctx: &Context) -> Result<String, RenderError> {
        let mut output = StringOutput::new();
        self.render_to_output(name, ctx, &mut output)?;
        output.into_string().map_err(RenderError::from)
    }

    /// Render a registered template and write data to the `std::io::Write`
    pub fn render_to_write<T, W>(&self, name: &str, data: &T, writer: W) -> Result<(), RenderError>
    where
        T: Serialize,
        W: Write,
    {
        let mut output = WriteOutput::new(writer);
        let ctx = Context::wraps(data)?;
        self.render_to_output(name, &ctx, &mut output)
    }

    /// Render a registered template using reusable `Context`, and write data to
    /// the `std::io::Write`
    pub fn render_with_context_to_write<W>(
        &self,
        name: &str,
        ctx: &Context,
        writer: W,
    ) -> Result<(), RenderError>
    where
        W: Write,
    {
        let mut output = WriteOutput::new(writer);
        self.render_to_output(name, ctx, &mut output)
    }

    /// Render a template string using current registry without registering it
    pub fn render_template<T>(&self, template_string: &str, data: &T) -> Result<String, RenderError>
    where
        T: Serialize,
    {
        let mut writer = StringWriter::new();
        self.render_template_to_write(template_string, data, &mut writer)?;
        Ok(writer.into_string())
    }

    /// Render a template string using reusable context data
    pub fn render_template_with_context(
        &self,
        template_string: &str,
        ctx: &Context,
    ) -> Result<String, RenderError> {
        let tpl = Template::compile2(
            template_string,
            TemplateOptions {
                prevent_indent: self.prevent_indent,
                ..Default::default()
            },
        )
        .map_err(RenderError::from)?;

        let mut out = StringOutput::new();
        {
            let mut render_context = RenderContext::new(None);
            tpl.render(self, ctx, &mut render_context, &mut out)?;
        }

        out.into_string().map_err(RenderError::from)
    }

    /// Render a template string using resuable context, and write data into
    /// `std::io::Write`
    pub fn render_template_with_context_to_write<W>(
        &self,
        template_string: &str,
        ctx: &Context,
        writer: W,
    ) -> Result<(), RenderError>
    where
        W: Write,
    {
        let tpl = Template::compile2(
            template_string,
            TemplateOptions {
                prevent_indent: self.prevent_indent,
                ..Default::default()
            },
        )
        .map_err(RenderError::from)?;
        let mut render_context = RenderContext::new(None);
        let mut out = WriteOutput::new(writer);
        tpl.render(self, ctx, &mut render_context, &mut out)
    }

    /// Render a template string using current registry without registering it
    pub fn render_template_to_write<T, W>(
        &self,
        template_string: &str,
        data: &T,
        writer: W,
    ) -> Result<(), RenderError>
    where
        T: Serialize,
        W: Write,
    {
        let ctx = Context::wraps(data)?;
        self.render_template_with_context_to_write(template_string, &ctx, writer)
    }

    #[cfg(feature = "string_helpers")]
    #[inline]
    fn register_string_helpers(&mut self) {
        use helpers::string_helpers::{
            kebab_case, lower_camel_case, shouty_kebab_case, shouty_snake_case, snake_case,
            title_case, train_case, upper_camel_case,
        };

        self.register_helper("lowerCamelCase", Box::new(lower_camel_case));
        self.register_helper("upperCamelCase", Box::new(upper_camel_case));
        self.register_helper("snakeCase", Box::new(snake_case));
        self.register_helper("kebabCase", Box::new(kebab_case));
        self.register_helper("shoutySnakeCase", Box::new(shouty_snake_case));
        self.register_helper("shoutyKebabCase", Box::new(shouty_kebab_case));
        self.register_helper("titleCase", Box::new(title_case));
        self.register_helper("trainCase", Box::new(train_case));
    }
}

#[cfg(test)]
mod test {
    use crate::context::Context;
    use crate::error::{RenderError, RenderErrorReason};
    use crate::helpers::HelperDef;
    use crate::output::Output;
    use crate::registry::Registry;
    use crate::render::{Helper, RenderContext, Renderable};
    use crate::support::str::StringWriter;
    use crate::template::Template;
    use std::fs::File;
    use std::io::Write;
    use tempfile::tempdir;

    #[derive(Clone, Copy)]
    struct DummyHelper;

    impl HelperDef for DummyHelper {
        fn call<'reg: 'rc, 'rc>(
            &self,
            h: &Helper<'rc>,
            r: &'reg Registry<'reg>,
            ctx: &'rc Context,
            rc: &mut RenderContext<'reg, 'rc>,
            out: &mut dyn Output,
        ) -> Result<(), RenderError> {
            h.template().unwrap().render(r, ctx, rc, out)
        }
    }

    static DUMMY_HELPER: DummyHelper = DummyHelper;

    #[test]
    fn test_registry_operations() {
        let mut r = Registry::new();

        assert!(r.register_template_string("index", "<h1></h1>").is_ok());

        let tpl = Template::compile("<h2></h2>").unwrap();
        r.register_template("index2", tpl);

        assert_eq!(r.templates.len(), 2);

        r.unregister_template("index");
        assert_eq!(r.templates.len(), 1);

        r.clear_templates();
        assert_eq!(r.templates.len(), 0);

        r.register_helper("dummy", Box::new(DUMMY_HELPER));

        // built-in helpers plus 1
        let num_helpers = 7;
        let num_boolean_helpers = 10; // stuff like gt and lte
        let num_custom_helpers = 1; // dummy from above
        #[cfg(feature = "string_helpers")]
        let string_helpers = 8;
        #[cfg(not(feature = "string_helpers"))]
        let string_helpers = 0;
        assert_eq!(
            r.helpers.len(),
            num_helpers + num_boolean_helpers + num_custom_helpers + string_helpers
        );
    }

    #[test]
    #[cfg(feature = "dir_source")]
    fn test_register_templates_directory() {
        use std::fs::DirBuilder;

        use crate::registry::DirectorySourceOptions;

        let mut r = Registry::new();
        {
            let dir = tempdir().unwrap();

            assert_eq!(r.templates.len(), 0);

            let file1_path = dir.path().join("t1.hbs");
            let mut file1: File = File::create(&file1_path).unwrap();
            writeln!(file1, "<h1>Hello {{world}}!</h1>").unwrap();

            let file2_path = dir.path().join("t2.hbs");
            let mut file2: File = File::create(&file2_path).unwrap();
            writeln!(file2, "<h1>Hola {{world}}!</h1>").unwrap();

            let file3_path = dir.path().join("t3.hbs");
            let mut file3: File = File::create(&file3_path).unwrap();
            writeln!(file3, "<h1>Hallo {{world}}!</h1>").unwrap();

            let file4_path = dir.path().join(".t4.hbs");
            let mut file4: File = File::create(&file4_path).unwrap();
            writeln!(file4, "<h1>Hallo {{world}}!</h1>").unwrap();

            r.register_templates_directory(dir.path(), DirectorySourceOptions::default())
                .unwrap();

            assert_eq!(r.templates.len(), 3);
            assert_eq!(r.templates.contains_key("t1"), true);
            assert_eq!(r.templates.contains_key("t2"), true);
            assert_eq!(r.templates.contains_key("t3"), true);
            assert_eq!(r.templates.contains_key("t4"), false);

            drop(file1);
            drop(file2);
            drop(file3);

            dir.close().unwrap();
        }

        {
            let dir = tempdir().unwrap();

            let file1_path = dir.path().join("t4.hbs");
            let mut file1: File = File::create(&file1_path).unwrap();
            writeln!(file1, "<h1>Hello {{world}}!</h1>").unwrap();

            let file2_path = dir.path().join("t5.erb");
            let mut file2: File = File::create(&file2_path).unwrap();
            writeln!(file2, "<h1>Hello {{% world %}}!</h1>").unwrap();

            let file3_path = dir.path().join("t6.html");
            let mut file3: File = File::create(&file3_path).unwrap();
            writeln!(file3, "<h1>Hello world!</h1>").unwrap();

            r.register_templates_directory(dir.path(), DirectorySourceOptions::default())
                .unwrap();

            assert_eq!(r.templates.len(), 4);
            assert_eq!(r.templates.contains_key("t4"), true);

            drop(file1);
            drop(file2);
            drop(file3);

            dir.close().unwrap();
        }

        {
            let dir = tempdir().unwrap();

            let _ = DirBuilder::new().create(dir.path().join("french")).unwrap();
            let _ = DirBuilder::new()
                .create(dir.path().join("portugese"))
                .unwrap();
            let _ = DirBuilder::new()
                .create(dir.path().join("italian"))
                .unwrap();

            let file1_path = dir.path().join("french/t7.hbs");
            let mut file1: File = File::create(&file1_path).unwrap();
            writeln!(file1, "<h1>Bonjour {{world}}!</h1>").unwrap();

            let file2_path = dir.path().join("portugese/t8.hbs");
            let mut file2: File = File::create(&file2_path).unwrap();
            writeln!(file2, "<h1>Ola {{world}}!</h1>").unwrap();

            let file3_path = dir.path().join("italian/t9.hbs");
            let mut file3: File = File::create(&file3_path).unwrap();
            writeln!(file3, "<h1>Ciao {{world}}!</h1>").unwrap();

            r.register_templates_directory(dir.path(), DirectorySourceOptions::default())
                .unwrap();

            assert_eq!(r.templates.len(), 7);
            assert_eq!(r.templates.contains_key("french/t7"), true);
            assert_eq!(r.templates.contains_key("portugese/t8"), true);
            assert_eq!(r.templates.contains_key("italian/t9"), true);

            drop(file1);
            drop(file2);
            drop(file3);

            dir.close().unwrap();
        }

        {
            let dir = tempdir().unwrap();

            let file1_path = dir.path().join("t10.hbs");
            let mut file1: File = File::create(&file1_path).unwrap();
            writeln!(file1, "<h1>Bonjour {{world}}!</h1>").unwrap();

            let mut dir_path = dir
                .path()
                .to_string_lossy()
                .replace(std::path::MAIN_SEPARATOR, "/");
            if !dir_path.ends_with("/") {
                dir_path.push('/');
            }
            r.register_templates_directory(dir_path, DirectorySourceOptions::default())
                .unwrap();

            assert_eq!(r.templates.len(), 8);
            assert_eq!(r.templates.contains_key("t10"), true);

            drop(file1);
            dir.close().unwrap();
        }

        {
            let dir = tempdir().unwrap();
            let mut r = Registry::new();

            let file1_path = dir.path().join("t11.hbs.html");
            let mut file1: File = File::create(&file1_path).unwrap();
            writeln!(file1, "<h1>Bonjour {{world}}!</h1>").unwrap();

            let mut dir_path = dir
                .path()
                .to_string_lossy()
                .replace(std::path::MAIN_SEPARATOR, "/");
            if !dir_path.ends_with("/") {
                dir_path.push('/');
            }
            r.register_templates_directory(
                dir_path,
                DirectorySourceOptions {
                    tpl_extension: ".hbs.html".to_owned(),
                    ..Default::default()
                },
            )
            .unwrap();

            assert_eq!(r.templates.len(), 1);
            assert_eq!(r.templates.contains_key("t11"), true);

            drop(file1);
            dir.close().unwrap();
        }

        {
            let dir = tempdir().unwrap();
            let mut r = Registry::new();

            assert_eq!(r.templates.len(), 0);

            let file1_path = dir.path().join(".t12.hbs");
            let mut file1: File = File::create(&file1_path).unwrap();
            writeln!(file1, "<h1>Hello {{world}}!</h1>").unwrap();

            r.register_templates_directory(
                dir.path(),
                DirectorySourceOptions {
                    hidden: true,
                    ..Default::default()
                },
            )
            .unwrap();

            assert_eq!(r.templates.len(), 1);
            assert_eq!(r.templates.contains_key(".t12"), true);

            drop(file1);

            dir.close().unwrap();
        }
    }

    #[test]
    fn test_render_to_write() {
        let mut r = Registry::new();

        assert!(r.register_template_string("index", "<h1></h1>").is_ok());

        let mut sw = StringWriter::new();
        {
            r.render_to_write("index", &(), &mut sw).ok().unwrap();
        }

        assert_eq!("<h1></h1>".to_string(), sw.into_string());
    }

    #[test]
    fn test_escape_fn() {
        let mut r = Registry::new();

        let input = String::from("\"<>&");

        r.register_template_string("test", String::from("{{this}}"))
            .unwrap();

        assert_eq!("&quot;&lt;&gt;&amp;", r.render("test", &input).unwrap());

        r.register_escape_fn(|s| s.into());

        assert_eq!("\"<>&", r.render("test", &input).unwrap());

        r.unregister_escape_fn();

        assert_eq!("&quot;&lt;&gt;&amp;", r.render("test", &input).unwrap());
    }

    #[test]
    fn test_escape() {
        let r = Registry::new();
        let data = json!({"hello": "world"});

        assert_eq!(
            "{{hello}}",
            r.render_template(r"\{{hello}}", &data).unwrap()
        );

        assert_eq!(
            " {{hello}}",
            r.render_template(r" \{{hello}}", &data).unwrap()
        );

        assert_eq!(r"\world", r.render_template(r"\\{{hello}}", &data).unwrap());
    }

    #[test]
    fn test_strict_mode() {
        let mut r = Registry::new();
        assert!(!r.strict_mode());

        r.set_strict_mode(true);
        assert!(r.strict_mode());

        let data = json!({
            "the_only_key": "the_only_value"
        });

        assert!(r
            .render_template("accessing the_only_key {{the_only_key}}", &data)
            .is_ok());
        assert!(r
            .render_template("accessing non-exists key {{the_key_never_exists}}", &data)
            .is_err());

        let render_error = r
            .render_template("accessing non-exists key {{the_key_never_exists}}", &data)
            .unwrap_err();
        assert_eq!(render_error.column_no.unwrap(), 26);
        assert_eq!(
            match render_error.reason() {
                RenderErrorReason::MissingVariable(path) => path.as_ref().unwrap(),
                _ => unreachable!(),
            },
            "the_key_never_exists"
        );

        let data2 = json!([1, 2, 3]);
        assert!(r
            .render_template("accessing valid array index {{this.[2]}}", &data2)
            .is_ok());
        assert!(r
            .render_template("accessing invalid array index {{this.[3]}}", &data2)
            .is_err());
        let render_error2 = r
            .render_template("accessing invalid array index {{this.[3]}}", &data2)
            .unwrap_err();
        assert_eq!(render_error2.column_no.unwrap(), 31);
        assert_eq!(
            match render_error2.reason() {
                RenderErrorReason::MissingVariable(path) => path.as_ref().unwrap(),
                _ => unreachable!(),
            },
            "this.[3]"
        )
    }

    use crate::json::value::ScopedJson;
    struct GenMissingHelper;
    impl HelperDef for GenMissingHelper {
        fn call_inner<'reg: 'rc, 'rc>(
            &self,
            _: &Helper<'rc>,
            _: &'reg Registry<'reg>,
            _: &'rc Context,
            _: &mut RenderContext<'reg, 'rc>,
        ) -> Result<ScopedJson<'rc>, RenderError> {
            Ok(ScopedJson::Missing)
        }
    }

    #[test]
    fn test_strict_mode_in_helper() {
        let mut r = Registry::new();
        r.set_strict_mode(true);

        r.register_helper(
            "check_missing",
            Box::new(
                |h: &Helper<'_>,
                 _: &Registry<'_>,
                 _: &Context,
                 _: &mut RenderContext<'_, '_>,
                 _: &mut dyn Output|
                 -> Result<(), RenderError> {
                    let value = h.param(0).unwrap();
                    assert!(value.is_value_missing());
                    Ok(())
                },
            ),
        );

        r.register_helper("generate_missing_value", Box::new(GenMissingHelper));

        let data = json!({
            "the_key_we_have": "the_value_we_have"
        });
        assert!(r
            .render_template("accessing non-exists key {{the_key_we_dont_have}}", &data)
            .is_err());
        assert!(r
            .render_template(
                "accessing non-exists key from helper {{check_missing the_key_we_dont_have}}",
                &data
            )
            .is_ok());
        assert!(r
            .render_template(
                "accessing helper that generates missing value {{generate_missing_value}}",
                &data
            )
            .is_err());
    }

    #[test]
    fn test_html_expression() {
        let reg = Registry::new();
        assert_eq!(
            reg.render_template("{{{ a }}}", &json!({"a": "<b>bold</b>"}))
                .unwrap(),
            "<b>bold</b>"
        );
        assert_eq!(
            reg.render_template("{{ &a }}", &json!({"a": "<b>bold</b>"}))
                .unwrap(),
            "<b>bold</b>"
        );
    }

    #[test]
    fn test_render_context() {
        let mut reg = Registry::new();

        let data = json!([0, 1, 2, 3]);

        assert_eq!(
            "0123",
            reg.render_template_with_context(
                "{{#each this}}{{this}}{{/each}}",
                &Context::wraps(&data).unwrap()
            )
            .unwrap()
        );

        reg.register_template_string("t0", "{{#each this}}{{this}}{{/each}}")
            .unwrap();
        assert_eq!(
            "0123",
            reg.render_with_context("t0", &Context::from(data)).unwrap()
        );
    }

    #[test]
    fn test_keys_starts_with_null() {
        env_logger::init();
        let reg = Registry::new();
        let data = json!({
            "optional": true,
            "is_null": true,
            "nullable": true,
            "null": true,
            "falsevalue": true,
        });
        assert_eq!(
            "optional: true --> true",
            reg.render_template(
                "optional: {{optional}} --> {{#if optional }}true{{else}}false{{/if}}",
                &data
            )
            .unwrap()
        );
        assert_eq!(
            "is_null: true --> true",
            reg.render_template(
                "is_null: {{is_null}} --> {{#if is_null }}true{{else}}false{{/if}}",
                &data
            )
            .unwrap()
        );
        assert_eq!(
            "nullable: true --> true",
            reg.render_template(
                "nullable: {{nullable}} --> {{#if nullable }}true{{else}}false{{/if}}",
                &data
            )
            .unwrap()
        );
        assert_eq!(
            "falsevalue: true --> true",
            reg.render_template(
                "falsevalue: {{falsevalue}} --> {{#if falsevalue }}true{{else}}false{{/if}}",
                &data
            )
            .unwrap()
        );
        assert_eq!(
            "null: true --> false",
            reg.render_template(
                "null: {{null}} --> {{#if null }}true{{else}}false{{/if}}",
                &data
            )
            .unwrap()
        );
        assert_eq!(
            "null: true --> true",
            reg.render_template(
                "null: {{null}} --> {{#if this.[null]}}true{{else}}false{{/if}}",
                &data
            )
            .unwrap()
        );
    }

    #[test]
    fn test_dev_mode_template_reload() {
        let mut reg = Registry::new();
        reg.set_dev_mode(true);
        assert!(reg.dev_mode());

        let dir = tempdir().unwrap();
        let file1_path = dir.path().join("t1.hbs");
        {
            let mut file1: File = File::create(&file1_path).unwrap();
            write!(file1, "<h1>Hello {{{{name}}}}!</h1>").unwrap();
        }

        reg.register_template_file("t1", &file1_path).unwrap();

        assert_eq!(
            reg.render("t1", &json!({"name": "Alex"})).unwrap(),
            "<h1>Hello Alex!</h1>"
        );

        {
            let mut file1: File = File::create(&file1_path).unwrap();
            write!(file1, "<h1>Privet {{{{name}}}}!</h1>").unwrap();
        }

        assert_eq!(
            reg.render("t1", &json!({"name": "Alex"})).unwrap(),
            "<h1>Privet Alex!</h1>"
        );

        dir.close().unwrap();
    }

    #[test]
    #[cfg(feature = "script_helper")]
    fn test_script_helper() {
        let mut reg = Registry::new();

        reg.register_script_helper("acc", "params.reduce(|sum, x| x + sum, 0)")
            .unwrap();

        assert_eq!(
            reg.render_template("{{acc 1 2 3 4}}", &json!({})).unwrap(),
            "10"
        );
    }

    #[test]
    #[cfg(feature = "script_helper")]
    fn test_script_helper_dev_mode() {
        let mut reg = Registry::new();
        reg.set_dev_mode(true);

        let dir = tempdir().unwrap();
        let file1_path = dir.path().join("acc.rhai");
        {
            let mut file1: File = File::create(&file1_path).unwrap();
            write!(file1, "params.reduce(|sum, x| x + sum, 0)").unwrap();
        }

        reg.register_script_helper_file("acc", &file1_path).unwrap();

        assert_eq!(
            reg.render_template("{{acc 1 2 3 4}}", &json!({})).unwrap(),
            "10"
        );

        {
            let mut file1: File = File::create(&file1_path).unwrap();
            write!(file1, "params.reduce(|sum, x| x * sum, 1)").unwrap();
        }

        assert_eq!(
            reg.render_template("{{acc 1 2 3 4}}", &json!({})).unwrap(),
            "24"
        );

        dir.close().unwrap();
    }

    #[test]
    #[cfg(feature = "script_helper")]
    fn test_engine_access() {
        use rhai::Engine;

        let mut registry = Registry::new();
        let mut eng = Engine::new();
        eng.set_max_string_size(1000);
        registry.set_engine(eng);

        assert_eq!(1000, registry.engine().max_string_size());
    }
}
