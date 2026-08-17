use std::borrow::{Borrow, Cow};
use std::collections::{BTreeMap, VecDeque};
use std::fmt;
use std::rc::Rc;

use serde_json::value::Value as Json;

use crate::block::BlockContext;
use crate::context::Context;
use crate::error::RenderError;
use crate::helpers::HelperDef;
use crate::json::path::Path;
use crate::json::value::{JsonRender, PathAndJson, ScopedJson};
use crate::output::{Output, StringOutput};
use crate::partial;
use crate::registry::Registry;
use crate::support;
use crate::template::TemplateElement::*;
use crate::template::{
    BlockParam, DecoratorTemplate, HelperTemplate, Parameter, Template, TemplateElement,
    TemplateMapping,
};

const HELPER_MISSING: &str = "helperMissing";
const BLOCK_HELPER_MISSING: &str = "blockHelperMissing";

/// The context of a render call
///
/// This context stores information of a render and a writer where generated
/// content is written to.
///
#[derive(Clone, Debug)]
pub struct RenderContext<'reg, 'rc> {
    inner: Rc<RenderContextInner<'reg, 'rc>>,
    blocks: VecDeque<BlockContext<'reg>>,
    // copy-on-write context
    modified_context: Option<Rc<Context>>,
}

#[derive(Clone)]
pub struct RenderContextInner<'reg: 'rc, 'rc> {
    partials: BTreeMap<String, &'reg Template>,
    partial_block_stack: VecDeque<&'reg Template>,
    partial_block_depth: isize,
    local_helpers: BTreeMap<String, Rc<dyn HelperDef + Send + Sync + 'rc>>,
    /// current template name
    current_template: Option<&'reg String>,
    /// root template name
    root_template: Option<&'reg String>,
    disable_escape: bool,
    indent_string: Option<&'reg String>,
}

impl<'reg: 'rc, 'rc> RenderContext<'reg, 'rc> {
    /// Create a render context
    pub fn new(root_template: Option<&'reg String>) -> RenderContext<'reg, 'rc> {
        let inner = Rc::new(RenderContextInner {
            partials: BTreeMap::new(),
            partial_block_stack: VecDeque::new(),
            partial_block_depth: 0,
            local_helpers: BTreeMap::new(),
            current_template: None,
            root_template,
            disable_escape: false,
            indent_string: None,
        });

        let mut blocks = VecDeque::with_capacity(5);
        blocks.push_front(BlockContext::new());

        let modified_context = None;
        RenderContext {
            inner,
            blocks,
            modified_context,
        }
    }

    pub(crate) fn new_for_block(&self) -> RenderContext<'reg, 'rc> {
        let inner = self.inner.clone();

        let mut blocks = VecDeque::with_capacity(2);
        blocks.push_front(BlockContext::new());

        let modified_context = self.modified_context.clone();

        RenderContext {
            inner,
            blocks,
            modified_context,
        }
    }

    /// Push a block context into render context stack. This is typically
    /// called when you entering a block scope.
    pub fn push_block(&mut self, block: BlockContext<'reg>) {
        self.blocks.push_front(block);
    }

    /// Pop and drop current block context.
    /// This is typically called when leaving a block scope.
    pub fn pop_block(&mut self) {
        self.blocks.pop_front();
    }

    pub(crate) fn clear_blocks(&mut self) {
        self.blocks.clear();
    }

    /// Borrow a reference to current block context
    pub fn block(&self) -> Option<&BlockContext<'reg>> {
        self.blocks.front()
    }

    /// Borrow a mutable reference to current block context in order to
    /// modify some data.
    pub fn block_mut(&mut self) -> Option<&mut BlockContext<'reg>> {
        self.blocks.front_mut()
    }

    fn inner(&self) -> &RenderContextInner<'reg, 'rc> {
        self.inner.borrow()
    }

    fn inner_mut(&mut self) -> &mut RenderContextInner<'reg, 'rc> {
        Rc::make_mut(&mut self.inner)
    }

    /// Get the modified context data if any
    pub fn context(&self) -> Option<Rc<Context>> {
        self.modified_context.clone()
    }

    /// Set new context data into the render process.
    /// This is typically called in decorators where user can modify
    /// the data they were rendering.
    pub fn set_context(&mut self, ctx: Context) {
        self.modified_context = Some(Rc::new(ctx))
    }

    /// Evaluate a Json path in current scope.
    ///
    /// Typically you don't need to evaluate it by yourself.
    /// The Helper and Decorator API will provide your evaluated value of
    /// their parameters and hash data.
    pub fn evaluate(
        &self,
        context: &'rc Context,
        relative_path: &str,
    ) -> Result<ScopedJson<'reg, 'rc>, RenderError> {
        let path = Path::parse(relative_path)?;
        self.evaluate2(context, &path)
    }

    pub(crate) fn evaluate2(
        &self,
        context: &'rc Context,
        path: &Path,
    ) -> Result<ScopedJson<'reg, 'rc>, RenderError> {
        match path {
            Path::Local((level, name, _)) => Ok(self
                .get_local_var(*level, name)
                .map(|v| ScopedJson::Derived(v.clone()))
                .unwrap_or_else(|| ScopedJson::Missing)),
            Path::Relative((segs, _)) => context.navigate(segs, &self.blocks),
        }
    }

    /// Get registered partial in this render context
    pub fn get_partial(&self, name: &str) -> Option<&Template> {
        if name == partial::PARTIAL_BLOCK {
            return self
                .inner()
                .partial_block_stack
                .get(self.inner().partial_block_depth as usize)
                .copied();
        }
        self.inner().partials.get(name).copied()
    }

    /// Register a partial for this context
    pub fn set_partial(&mut self, name: String, partial: &'reg Template) {
        self.inner_mut().partials.insert(name, partial);
    }

    pub(crate) fn push_partial_block(&mut self, partial: &'reg Template) {
        self.inner_mut().partial_block_stack.push_front(partial);
    }

    pub(crate) fn pop_partial_block(&mut self) {
        self.inner_mut().partial_block_stack.pop_front();
    }

    pub(crate) fn inc_partial_block_depth(&mut self) {
        self.inner_mut().partial_block_depth += 1;
    }

    pub(crate) fn dec_partial_block_depth(&mut self) {
        let depth = &mut self.inner_mut().partial_block_depth;
        if *depth > 0 {
            *depth -= 1;
        }
    }

    pub(crate) fn set_indent_string(&mut self, indent: Option<&'reg String>) {
        self.inner_mut().indent_string = indent;
    }

    #[inline]
    pub(crate) fn get_indent_string(&self) -> Option<&'reg String> {
        self.inner.indent_string
    }

    /// Remove a registered partial
    pub fn remove_partial(&mut self, name: &str) {
        self.inner_mut().partials.remove(name);
    }

    fn get_local_var(&self, level: usize, name: &str) -> Option<&Json> {
        self.blocks
            .get(level)
            .and_then(|blk| blk.get_local_var(name))
    }

    /// Test if given template name is current template.
    pub fn is_current_template(&self, p: &str) -> bool {
        self.inner()
            .current_template
            .map(|s| s == p)
            .unwrap_or(false)
    }

    /// Register a helper in this render context.
    /// This is a feature provided by Decorator where you can create
    /// temporary helpers.
    pub fn register_local_helper(
        &mut self,
        name: &str,
        def: Box<dyn HelperDef + Send + Sync + 'rc>,
    ) {
        self.inner_mut()
            .local_helpers
            .insert(name.to_string(), def.into());
    }

    /// Remove a helper from render context
    pub fn unregister_local_helper(&mut self, name: &str) {
        self.inner_mut().local_helpers.remove(name);
    }

    /// Attempt to get a helper from current render context.
    pub fn get_local_helper(&self, name: &str) -> Option<Rc<dyn HelperDef + Send + Sync + 'rc>> {
        self.inner().local_helpers.get(name).cloned()
    }

    #[inline]
    fn has_local_helper(&self, name: &str) -> bool {
        self.inner.local_helpers.contains_key(name)
    }

    /// Returns the current template name.
    /// Note that the name can be vary from root template when you are rendering
    /// from partials.
    pub fn get_current_template_name(&self) -> Option<&'reg String> {
        self.inner().current_template
    }

    /// Set the current template name.
    pub fn set_current_template_name(&mut self, name: Option<&'reg String>) {
        self.inner_mut().current_template = name;
    }

    /// Get root template name if any.
    /// This is the template name that you call `render` from `Handlebars`.
    pub fn get_root_template_name(&self) -> Option<&'reg String> {
        self.inner().root_template
    }

    /// Get the escape toggle
    pub fn is_disable_escape(&self) -> bool {
        self.inner().disable_escape
    }

    /// Set the escape toggle.
    /// When toggle is on, escape_fn will be called when rendering.
    pub fn set_disable_escape(&mut self, disable: bool) {
        self.inner_mut().disable_escape = disable
    }
}

impl<'reg, 'rc> fmt::Debug for RenderContextInner<'reg, 'rc> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        f.debug_struct("RenderContextInner")
            .field("partials", &self.partials)
            .field("partial_block_stack", &self.partial_block_stack)
            .field("partial_block_depth", &self.partial_block_depth)
            .field("root_template", &self.root_template)
            .field("current_template", &self.current_template)
            .field("disable_escape", &self.disable_escape)
            .finish()
    }
}

/// Render-time Helper data when using in a helper definition
#[derive(Debug)]
pub struct Helper<'reg, 'rc> {
    name: Cow<'reg, str>,
    params: Vec<PathAndJson<'reg, 'rc>>,
    hash: BTreeMap<&'reg str, PathAndJson<'reg, 'rc>>,
    template: Option<&'reg Template>,
    inverse: Option<&'reg Template>,
    block_param: Option<&'reg BlockParam>,
    block: bool,
}

impl<'reg: 'rc, 'rc> Helper<'reg, 'rc> {
    fn try_from_template(
        ht: &'reg HelperTemplate,
        registry: &'reg Registry<'reg>,
        context: &'rc Context,
        render_context: &mut RenderContext<'reg, 'rc>,
    ) -> Result<Helper<'reg, 'rc>, RenderError> {
        let name = ht.name.expand_as_name(registry, context, render_context)?;
        let mut pv = Vec::with_capacity(ht.params.len());
        for p in &ht.params {
            let r = p.expand(registry, context, render_context)?;
            pv.push(r);
        }

        let mut hm = BTreeMap::new();
        for (k, p) in &ht.hash {
            let r = p.expand(registry, context, render_context)?;
            hm.insert(k.as_ref(), r);
        }

        Ok(Helper {
            name,
            params: pv,
            hash: hm,
            template: ht.template.as_ref(),
            inverse: ht.inverse.as_ref(),
            block_param: ht.block_param.as_ref(),
            block: ht.block,
        })
    }

    /// Returns helper name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns all helper params, resolved within the context
    pub fn params(&self) -> &Vec<PathAndJson<'reg, 'rc>> {
        &self.params
    }

    /// Returns nth helper param, resolved within the context.
    ///
    /// ## Example
    ///
    /// To get the first param in `{{my_helper abc}}` or `{{my_helper 2}}`,
    /// use `h.param(0)` in helper definition.
    /// Variable `abc` is auto resolved in current context.
    ///
    /// ```
    /// use handlebars::*;
    ///
    /// fn my_helper(h: &Helper, rc: &mut RenderContext) -> Result<(), RenderError> {
    ///     let v = h.param(0).map(|v| v.value())
    ///         .ok_or(RenderError::new("param not found"));
    ///     // ..
    ///     Ok(())
    /// }
    /// ```
    pub fn param(&self, idx: usize) -> Option<&PathAndJson<'reg, 'rc>> {
        self.params.get(idx)
    }

    /// Returns hash, resolved within the context
    pub fn hash(&self) -> &BTreeMap<&'reg str, PathAndJson<'reg, 'rc>> {
        &self.hash
    }

    /// Return hash value of a given key, resolved within the context
    ///
    /// ## Example
    ///
    /// To get the first param in `{{my_helper v=abc}}` or `{{my_helper v=2}}`,
    /// use `h.hash_get("v")` in helper definition.
    /// Variable `abc` is auto resolved in current context.
    ///
    /// ```
    /// use handlebars::*;
    ///
    /// fn my_helper(h: &Helper, rc: &mut RenderContext) -> Result<(), RenderError> {
    ///     let v = h.hash_get("v").map(|v| v.value())
    ///         .ok_or(RenderError::new("param not found"));
    ///     // ..
    ///     Ok(())
    /// }
    /// ```
    pub fn hash_get(&self, key: &str) -> Option<&PathAndJson<'reg, 'rc>> {
        self.hash.get(key)
    }

    /// Returns the default inner template if the helper is a block helper.
    ///
    /// Typically you will render the template via: `template.render(registry, render_context)`
    ///
    pub fn template(&self) -> Option<&'reg Template> {
        self.template
    }

    /// Returns the template of `else` branch if any
    pub fn inverse(&self) -> Option<&'reg Template> {
        self.inverse
    }

    /// Returns if the helper is a block one `{{#helper}}{{/helper}}` or not `{{helper 123}}`
    pub fn is_block(&self) -> bool {
        self.block
    }

    /// Returns if the helper has either a block param or block param pair
    pub fn has_block_param(&self) -> bool {
        self.block_param.is_some()
    }

    /// Returns block param if any
    pub fn block_param(&self) -> Option<&'reg str> {
        if let Some(&BlockParam::Single(Parameter::Name(ref s))) = self.block_param {
            Some(s)
        } else {
            None
        }
    }

    /// Return block param pair (for example |key, val|) if any
    pub fn block_param_pair(&self) -> Option<(&'reg str, &'reg str)> {
        if let Some(&BlockParam::Pair((Parameter::Name(ref s1), Parameter::Name(ref s2)))) =
            self.block_param
        {
            Some((s1, s2))
        } else {
            None
        }
    }
}

/// Render-time Decorator data when using in a decorator definition
#[derive(Debug)]
pub struct Decorator<'reg, 'rc> {
    name: Cow<'reg, str>,
    params: Vec<PathAndJson<'reg, 'rc>>,
    hash: BTreeMap<&'reg str, PathAndJson<'reg, 'rc>>,
    template: Option<&'reg Template>,
    indent: Option<&'reg String>,
}

impl<'reg: 'rc, 'rc> Decorator<'reg, 'rc> {
    fn try_from_template(
        dt: &'reg DecoratorTemplate,
        registry: &'reg Registry<'reg>,
        context: &'rc Context,
        render_context: &mut RenderContext<'reg, 'rc>,
    ) -> Result<Decorator<'reg, 'rc>, RenderError> {
        let name = dt.name.expand_as_name(registry, context, render_context)?;

        let mut pv = Vec::with_capacity(dt.params.len());
        for p in &dt.params {
            let r = p.expand(registry, context, render_context)?;
            pv.push(r);
        }

        let mut hm = BTreeMap::new();
        for (k, p) in &dt.hash {
            let r = p.expand(registry, context, render_context)?;
            hm.insert(k.as_ref(), r);
        }

        Ok(Decorator {
            name,
            params: pv,
            hash: hm,
            template: dt.template.as_ref(),
            indent: dt.indent.as_ref(),
        })
    }

    /// Returns helper name
    pub fn name(&self) -> &str {
        self.name.as_ref()
    }

    /// Returns all helper params, resolved within the context
    pub fn params(&self) -> &Vec<PathAndJson<'reg, 'rc>> {
        &self.params
    }

    /// Returns nth helper param, resolved within the context
    pub fn param(&self, idx: usize) -> Option<&PathAndJson<'reg, 'rc>> {
        self.params.get(idx)
    }

    /// Returns hash, resolved within the context
    pub fn hash(&self) -> &BTreeMap<&'reg str, PathAndJson<'reg, 'rc>> {
        &self.hash
    }

    /// Return hash value of a given key, resolved within the context
    pub fn hash_get(&self, key: &str) -> Option<&PathAndJson<'reg, 'rc>> {
        self.hash.get(key)
    }

    /// Returns the default inner template if any
    pub fn template(&self) -> Option<&'reg Template> {
        self.template
    }

    pub fn indent(&self) -> Option<&'reg String> {
        self.indent
    }
}

/// Render trait
pub trait Renderable {
    /// render into RenderContext's `writer`
    fn render<'reg: 'rc, 'rc>(
        &'reg self,
        registry: &'reg Registry<'reg>,
        context: &'rc Context,
        rc: &mut RenderContext<'reg, 'rc>,
        out: &mut dyn Output,
    ) -> Result<(), RenderError>;

    /// render into string
    fn renders<'reg: 'rc, 'rc>(
        &'reg self,
        registry: &'reg Registry<'reg>,
        ctx: &'rc Context,
        rc: &mut RenderContext<'reg, 'rc>,
    ) -> Result<String, RenderError> {
        let mut so = StringOutput::new();
        self.render(registry, ctx, rc, &mut so)?;
        so.into_string().map_err(RenderError::from)
    }
}

/// Evaluate decorator
pub trait Evaluable {
    fn eval<'reg: 'rc, 'rc>(
        &'reg self,
        registry: &'reg Registry<'reg>,
        context: &'rc Context,
        rc: &mut RenderContext<'reg, 'rc>,
    ) -> Result<(), RenderError>;
}

#[inline]
fn call_helper_for_value<'reg: 'rc, 'rc>(
    hd: &dyn HelperDef,
    ht: &Helper<'reg, 'rc>,
    r: &'reg Registry<'reg>,
    ctx: &'rc Context,
    rc: &mut RenderContext<'reg, 'rc>,
) -> Result<PathAndJson<'reg, 'rc>, RenderError> {
    match hd.call_inner(ht, r, ctx, rc) {
        Ok(result) => Ok(PathAndJson::new(None, result)),
        Err(e) => {
            if e.is_unimplemented() {
                // parse value from output
                let mut so = StringOutput::new();

                // here we don't want subexpression result escaped,
                // so we temporarily disable it
                let disable_escape = rc.is_disable_escape();
                rc.set_disable_escape(true);

                hd.call(ht, r, ctx, rc, &mut so)?;
                rc.set_disable_escape(disable_escape);

                let string = so.into_string().map_err(RenderError::from)?;
                Ok(PathAndJson::new(
                    None,
                    ScopedJson::Derived(Json::String(string)),
                ))
            } else {
                Err(e)
            }
        }
    }
}

impl Parameter {
    pub fn expand_as_name<'reg: 'rc, 'rc>(
        &'reg self,
        registry: &'reg Registry<'reg>,
        ctx: &'rc Context,
        rc: &mut RenderContext<'reg, 'rc>,
    ) -> Result<Cow<'reg, str>, RenderError> {
        match self {
            Parameter::Name(ref name) => Ok(Cow::Borrowed(name)),
            Parameter::Path(ref p) => Ok(Cow::Borrowed(p.raw())),
            Parameter::Subexpression(_) => self
                .expand(registry, ctx, rc)
                .map(|v| v.value().render())
                .map(Cow::Owned),
            Parameter::Literal(ref j) => Ok(Cow::Owned(j.render())),
        }
    }

    pub fn expand<'reg: 'rc, 'rc>(
        &'reg self,
        registry: &'reg Registry<'reg>,
        ctx: &'rc Context,
        rc: &mut RenderContext<'reg, 'rc>,
    ) -> Result<PathAndJson<'reg, 'rc>, RenderError> {
        match self {
            Parameter::Name(ref name) => {
                // FIXME: raise error when expanding with name?
                Ok(PathAndJson::new(Some(name.to_owned()), ScopedJson::Missing))
            }
            Parameter::Path(ref path) => {
                if let Some(rc_context) = rc.context() {
                    let result = rc.evaluate2(rc_context.borrow(), path)?;
                    Ok(PathAndJson::new(
                        Some(path.raw().to_owned()),
                        ScopedJson::Derived(result.as_json().clone()),
                    ))
                } else {
                    let result = rc.evaluate2(ctx, path)?;
                    Ok(PathAndJson::new(Some(path.raw().to_owned()), result))
                }
            }
            Parameter::Literal(ref j) => Ok(PathAndJson::new(None, ScopedJson::Constant(j))),
            Parameter::Subexpression(ref t) => match *t.as_element() {
                Expression(ref ht) => {
                    let name = ht.name.expand_as_name(registry, ctx, rc)?;

                    let h = Helper::try_from_template(ht, registry, ctx, rc)?;
                    if let Some(ref d) = rc.get_local_helper(&name) {
                        call_helper_for_value(d.as_ref(), &h, registry, ctx, rc)
                    } else {
                        let mut helper = registry.get_or_load_helper(&name)?;

                        if helper.is_none() {
                            helper = registry.get_or_load_helper(if ht.block {
                                BLOCK_HELPER_MISSING
                            } else {
                                HELPER_MISSING
                            })?;
                        }

                        helper
                            .ok_or_else(|| {
                                RenderError::new(format!("Helper not defined: {:?}", ht.name))
                            })
                            .and_then(|d| call_helper_for_value(d.as_ref(), &h, registry, ctx, rc))
                    }
                }
                _ => unreachable!(),
            },
        }
    }
}

impl Renderable for Template {
    fn render<'reg: 'rc, 'rc>(
        &'reg self,
        registry: &'reg Registry<'reg>,
        ctx: &'rc Context,
        rc: &mut RenderContext<'reg, 'rc>,
        out: &mut dyn Output,
    ) -> Result<(), RenderError> {
        rc.set_current_template_name(self.name.as_ref());
        let iter = self.elements.iter();

        for (idx, t) in iter.enumerate() {
            t.render(registry, ctx, rc, out).map_err(|mut e| {
                // add line/col number if the template has mapping data
                if e.line_no.is_none() {
                    if let Some(&TemplateMapping(line, col)) = self.mapping.get(idx) {
                        e.line_no = Some(line);
                        e.column_no = Some(col);
                    }
                }

                if e.template_name.is_none() {
                    e.template_name = self.name.clone();
                }

                e
            })?;
        }
        Ok(())
    }
}

impl Evaluable for Template {
    fn eval<'reg: 'rc, 'rc>(
        &'reg self,
        registry: &'reg Registry<'reg>,
        ctx: &'rc Context,
        rc: &mut RenderContext<'reg, 'rc>,
    ) -> Result<(), RenderError> {
        let iter = self.elements.iter();

        for (idx, t) in iter.enumerate() {
            t.eval(registry, ctx, rc).map_err(|mut e| {
                if e.line_no.is_none() {
                    if let Some(&TemplateMapping(line, col)) = self.mapping.get(idx) {
                        e.line_no = Some(line);
                        e.column_no = Some(col);
                    }
                }

                e.template_name = self.name.clone();
                e
            })?;
        }
        Ok(())
    }
}

fn helper_exists<'reg: 'rc, 'rc>(
    name: &str,
    reg: &Registry<'reg>,
    rc: &RenderContext<'reg, 'rc>,
) -> bool {
    rc.has_local_helper(name) || reg.has_helper(name)
}

#[inline]
fn render_helper<'reg: 'rc, 'rc>(
    ht: &'reg HelperTemplate,
    registry: &'reg Registry<'reg>,
    ctx: &'rc Context,
    rc: &mut RenderContext<'reg, 'rc>,
    out: &mut dyn Output,
) -> Result<(), RenderError> {
    let h = Helper::try_from_template(ht, registry, ctx, rc)?;
    debug!(
        "Rendering helper: {:?}, params: {:?}, hash: {:?}",
        h.name(),
        h.params(),
        h.hash()
    );
    if let Some(ref d) = rc.get_local_helper(h.name()) {
        d.call(&h, registry, ctx, rc, out)
    } else {
        let mut helper = registry.get_or_load_helper(h.name())?;

        if helper.is_none() {
            helper = registry.get_or_load_helper(if ht.block {
                BLOCK_HELPER_MISSING
            } else {
                HELPER_MISSING
            })?;
        }

        helper
            .ok_or_else(|| RenderError::new(format!("Helper not defined: {:?}", h.name())))
            .and_then(|d| d.call(&h, registry, ctx, rc, out))
    }
}

pub(crate) fn do_escape(r: &Registry<'_>, rc: &RenderContext<'_, '_>, content: String) -> String {
    if !rc.is_disable_escape() {
        r.get_escape_fn()(&content)
    } else {
        content
    }
}

#[inline]
fn indent_aware_write(
    v: &str,
    rc: &mut RenderContext<'_, '_>,
    out: &mut dyn Output,
) -> Result<(), RenderError> {
    if let Some(indent) = rc.get_indent_string() {
        out.write(support::str::with_indent(v, indent).as_ref())?;
    } else {
        out.write(v.as_ref())?;
    }
    Ok(())
}

impl Renderable for TemplateElement {
    fn render<'reg: 'rc, 'rc>(
        &'reg self,
        registry: &'reg Registry<'reg>,
        ctx: &'rc Context,
        rc: &mut RenderContext<'reg, 'rc>,
        out: &mut dyn Output,
    ) -> Result<(), RenderError> {
        match self {
            RawString(ref v) => indent_aware_write(v.as_ref(), rc, out),
            Expression(ref ht) | HtmlExpression(ref ht) => {
                let is_html_expression = matches!(self, HtmlExpression(_));
                if is_html_expression {
                    rc.set_disable_escape(true);
                }

                // test if the expression is to render some value
                let result = if ht.is_name_only() {
                    let helper_name = ht.name.expand_as_name(registry, ctx, rc)?;
                    if helper_exists(&helper_name, registry, rc) {
                        render_helper(ht, registry, ctx, rc, out)
                    } else {
                        debug!("Rendering value: {:?}", ht.name);
                        let context_json = ht.name.expand(registry, ctx, rc)?;
                        if context_json.is_value_missing() {
                            if registry.strict_mode() {
                                Err(RenderError::strict_error(context_json.relative_path()))
                            } else {
                                // helper missing
                                if let Some(hook) = registry.get_or_load_helper(HELPER_MISSING)? {
                                    let h = Helper::try_from_template(ht, registry, ctx, rc)?;
                                    hook.call(&h, registry, ctx, rc, out)
                                } else {
                                    Ok(())
                                }
                            }
                        } else {
                            let rendered = context_json.value().render();
                            let output = do_escape(registry, rc, rendered);
                            indent_aware_write(output.as_ref(), rc, out)
                        }
                    }
                } else {
                    // this is a helper expression
                    render_helper(ht, registry, ctx, rc, out)
                };

                if is_html_expression {
                    rc.set_disable_escape(false);
                }

                result
            }
            HelperBlock(ref ht) => render_helper(ht, registry, ctx, rc, out),
            DecoratorExpression(_) | DecoratorBlock(_) => self.eval(registry, ctx, rc),
            PartialExpression(ref dt) | PartialBlock(ref dt) => {
                let di = Decorator::try_from_template(dt, registry, ctx, rc)?;

                partial::expand_partial(&di, registry, ctx, rc, out)
            }
            _ => Ok(()),
        }
    }
}

impl Evaluable for TemplateElement {
    fn eval<'reg: 'rc, 'rc>(
        &'reg self,
        registry: &'reg Registry<'reg>,
        ctx: &'rc Context,
        rc: &mut RenderContext<'reg, 'rc>,
    ) -> Result<(), RenderError> {
        match *self {
            DecoratorExpression(ref dt) | DecoratorBlock(ref dt) => {
                let di = Decorator::try_from_template(dt, registry, ctx, rc)?;
                match registry.get_decorator(di.name()) {
                    Some(d) => d.call(&di, registry, ctx, rc),
                    None => Err(RenderError::new(format!(
                        "Decorator not defined: {:?}",
                        dt.name
                    ))),
                }
            }
            _ => Ok(()),
        }
    }
}

#[cfg(test)]
mod test {
    use std::collections::BTreeMap;

    use super::{Helper, RenderContext, Renderable};
    use crate::block::BlockContext;
    use crate::context::Context;
    use crate::error::RenderError;
    use crate::json::path::Path;
    use crate::json::value::JsonRender;
    use crate::output::{Output, StringOutput};
    use crate::registry::Registry;
    use crate::template::TemplateElement::*;
    use crate::template::{HelperTemplate, Template, TemplateElement};

    #[test]
    fn test_raw_string() {
        let r = Registry::new();
        let raw_string = RawString("<h1>hello world</h1>".to_string());

        let mut out = StringOutput::new();
        let ctx = Context::null();
        {
            let mut rc = RenderContext::new(None);
            raw_string.render(&r, &ctx, &mut rc, &mut out).ok().unwrap();
        }
        assert_eq!(
            out.into_string().unwrap(),
            "<h1>hello world</h1>".to_string()
        );
    }

    #[test]
    fn test_expression() {
        let r = Registry::new();
        let element = Expression(Box::new(HelperTemplate::with_path(Path::with_named_paths(
            &["hello"],
        ))));

        let mut out = StringOutput::new();
        let mut m: BTreeMap<String, String> = BTreeMap::new();
        let value = "<p></p>".to_string();
        m.insert("hello".to_string(), value);
        let ctx = Context::wraps(&m).unwrap();
        {
            let mut rc = RenderContext::new(None);
            element.render(&r, &ctx, &mut rc, &mut out).ok().unwrap();
        }

        assert_eq!(
            out.into_string().unwrap(),
            "&lt;p&gt;&lt;/p&gt;".to_string()
        );
    }

    #[test]
    fn test_html_expression() {
        let r = Registry::new();
        let element = HtmlExpression(Box::new(HelperTemplate::with_path(Path::with_named_paths(
            &["hello"],
        ))));

        let mut out = StringOutput::new();
        let mut m: BTreeMap<String, String> = BTreeMap::new();
        let value = "world";
        m.insert("hello".to_string(), value.to_string());
        let ctx = Context::wraps(&m).unwrap();
        {
            let mut rc = RenderContext::new(None);
            element.render(&r, &ctx, &mut rc, &mut out).ok().unwrap();
        }

        assert_eq!(out.into_string().unwrap(), value.to_string());
    }

    #[test]
    fn test_template() {
        let r = Registry::new();
        let mut out = StringOutput::new();
        let mut m: BTreeMap<String, String> = BTreeMap::new();
        let value = "world".to_string();
        m.insert("hello".to_string(), value);
        let ctx = Context::wraps(&m).unwrap();

        let elements: Vec<TemplateElement> = vec![
            RawString("<h1>".to_string()),
            Expression(Box::new(HelperTemplate::with_path(Path::with_named_paths(
                &["hello"],
            )))),
            RawString("</h1>".to_string()),
            Comment("".to_string()),
        ];

        let template = Template {
            elements,
            name: None,
            mapping: Vec::new(),
        };

        {
            let mut rc = RenderContext::new(None);
            template.render(&r, &ctx, &mut rc, &mut out).ok().unwrap();
        }

        assert_eq!(out.into_string().unwrap(), "<h1>world</h1>".to_string());
    }

    #[test]
    fn test_render_context_promotion_and_demotion() {
        use crate::json::value::to_json;
        let mut render_context = RenderContext::new(None);
        let mut block = BlockContext::new();

        block.set_local_var("index", to_json(0));
        render_context.push_block(block);

        render_context.push_block(BlockContext::new());
        assert_eq!(
            render_context.get_local_var(1, "index").unwrap(),
            &to_json(0)
        );

        render_context.pop_block();

        assert_eq!(
            render_context.get_local_var(0, "index").unwrap(),
            &to_json(0)
        );
    }

    #[test]
    fn test_render_subexpression_issue_115() {
        use crate::support::str::StringWriter;

        let mut r = Registry::new();
        r.register_helper(
            "format",
            Box::new(
                |h: &Helper<'_, '_>,
                 _: &Registry<'_>,
                 _: &Context,
                 _: &mut RenderContext<'_, '_>,
                 out: &mut dyn Output|
                 -> Result<(), RenderError> {
                    out.write(&h.param(0).unwrap().value().render())
                        .map(|_| ())
                        .map_err(RenderError::from)
                },
            ),
        );

        let mut sw = StringWriter::new();
        let mut m: BTreeMap<String, String> = BTreeMap::new();
        m.insert("a".to_string(), "123".to_string());

        {
            if let Err(e) = r.render_template_to_write("{{format (format a)}}", &m, &mut sw) {
                panic!("{}", e);
            }
        }

        assert_eq!(sw.into_string(), "123".to_string());
    }

    #[test]
    fn test_render_error_line_no() {
        let mut r = Registry::new();
        let m: BTreeMap<String, String> = BTreeMap::new();

        let name = "invalid_template";
        assert!(r
            .register_template_string(name, "<h1>\n{{#if true}}\n  {{#each}}{{/each}}\n{{/if}}")
            .is_ok());

        if let Err(e) = r.render(name, &m) {
            assert_eq!(e.line_no.unwrap(), 3);
            assert_eq!(e.column_no.unwrap(), 3);
            assert_eq!(e.template_name, Some(name.to_owned()));
        } else {
            panic!("Error expected");
        }
    }

    #[test]
    fn test_partial_failback_render() {
        let mut r = Registry::new();

        assert!(r
            .register_template_string("parent", "<html>{{> layout}}</html>")
            .is_ok());
        assert!(r
            .register_template_string(
                "child",
                "{{#*inline \"layout\"}}content{{/inline}}{{#> parent}}{{> seg}}{{/parent}}"
            )
            .is_ok());
        assert!(r.register_template_string("seg", "1234").is_ok());

        let r = r.render("child", &true).expect("should work");
        assert_eq!(r, "<html>content</html>");
    }

    #[test]
    fn test_key_with_slash() {
        let mut r = Registry::new();

        assert!(r
            .register_template_string("t", "{{#each this}}{{@key}}: {{this}}\n{{/each}}")
            .is_ok());

        let r = r.render("t", &json!({"/foo": "bar"})).unwrap();

        assert_eq!(r, "/foo: bar\n");
    }

    #[test]
    fn test_comment() {
        let r = Registry::new();

        assert_eq!(
            r.render_template("Hello {{this}} {{! test me }}", &0)
                .unwrap(),
            "Hello 0 "
        );
    }

    #[test]
    fn test_zero_args_heler() {
        let mut r = Registry::new();

        r.register_helper(
            "name",
            Box::new(
                |_: &Helper<'_, '_>,
                 _: &Registry<'_>,
                 _: &Context,
                 _: &mut RenderContext<'_, '_>,
                 out: &mut dyn Output|
                 -> Result<(), RenderError> {
                    out.write("N/A").map_err(Into::into)
                },
            ),
        );

        r.register_template_string("t0", "Output name: {{name}}")
            .unwrap();
        r.register_template_string("t1", "Output name: {{first_name}}")
            .unwrap();
        r.register_template_string("t2", "Output name: {{./name}}")
            .unwrap();

        // when "name" is available in context, use context first
        assert_eq!(
            r.render("t0", &json!({"name": "Alex"})).unwrap(),
            "Output name: N/A"
        );

        // when "name" is unavailable, call helper with same name
        assert_eq!(
            r.render("t2", &json!({"name": "Alex"})).unwrap(),
            "Output name: Alex"
        );

        // output nothing when neither context nor helper available
        assert_eq!(
            r.render("t1", &json!({"name": "Alex"})).unwrap(),
            "Output name: "
        );

        // generate error in strict mode for above case
        r.set_strict_mode(true);
        assert!(r.render("t1", &json!({"name": "Alex"})).is_err());

        // output nothing when helperMissing was defined
        r.set_strict_mode(false);
        r.register_helper(
            "helperMissing",
            Box::new(
                |h: &Helper<'_, '_>,
                 _: &Registry<'_>,
                 _: &Context,
                 _: &mut RenderContext<'_, '_>,
                 out: &mut dyn Output|
                 -> Result<(), RenderError> {
                    let name = h.name();
                    write!(out, "{} not resolved", name)?;
                    Ok(())
                },
            ),
        );
        assert_eq!(
            r.render("t1", &json!({"name": "Alex"})).unwrap(),
            "Output name: first_name not resolved"
        );
    }

    #[test]
    fn test_identifiers_starting_with_numbers() {
        let mut r = Registry::new();

        assert!(r
            .register_template_string("r1", "{{#if 0a}}true{{/if}}")
            .is_ok());
        let r1 = r.render("r1", &json!({"0a": true})).unwrap();
        assert_eq!(r1, "true");

        assert!(r.register_template_string("r2", "{{eq 1a 1}}").is_ok());
        let r2 = r.render("r2", &json!({"1a": 2, "a": 1})).unwrap();
        assert_eq!(r2, "false");

        assert!(r
        .register_template_string("r3", "0: {{0}} {{#if (eq 0 true)}}resolved from context{{/if}}\n1a: {{1a}} {{#if (eq 1a true)}}resolved from context{{/if}}\n2_2: {{2_2}} {{#if (eq 2_2 true)}}resolved from context{{/if}}") // YUP it is just eq that barfs! is if handled specially? maybe this test should go nearer to specific helpers that fail?
        .is_ok());
        let r3 = r
            .render("r3", &json!({"0": true, "1a": true, "2_2": true}))
            .unwrap();
        assert_eq!(
            r3,
            "0: true \n1a: true resolved from context\n2_2: true resolved from context"
        );

        // these should all be errors:
        assert!(r.register_template_string("r4", "{{eq 1}}").is_ok());
        assert!(r.register_template_string("r5", "{{eq a1}}").is_ok());
        assert!(r.register_template_string("r6", "{{eq 1a}}").is_ok());
        assert!(r.render("r4", &()).is_err());
        assert!(r.render("r5", &()).is_err());
        assert!(r.render("r6", &()).is_err());
    }
}
