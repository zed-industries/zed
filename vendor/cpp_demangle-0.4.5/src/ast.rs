//! Abstract syntax tree types for mangled symbols.

use super::{DemangleNodeType, DemangleOptions, DemangleWrite, ParseOptions};
use crate::error::{self, Result};
use crate::index_str::IndexStr;
use crate::subs::{Substitutable, SubstitutionTable};
use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec::Vec;
use core::cell::Cell;
#[cfg(feature = "logging")]
use core::cell::RefCell;
use core::fmt::{self, Write};
use core::hash::{Hash, Hasher};
use core::mem;
use core::ops;
use core::ptr;
use core::str;

macro_rules! r#try_recurse {
    ($expr:expr $(,)?) => {
        match $expr {
            Result::Err(error::Error::TooMuchRecursion) => {
                return Result::Err(error::Error::TooMuchRecursion);
            }
            val => val,
        }
    };
}

struct AutoLogParse;

#[cfg(feature = "logging")]
thread_local! {
    static LOG_DEPTH: RefCell<usize> = RefCell::new(0);
}

impl AutoLogParse {
    #[cfg(feature = "logging")]
    fn new(production: &'static str, input: IndexStr<'_>) -> AutoLogParse {
        LOG_DEPTH.with(|depth| {
            if *depth.borrow() == 0 {
                println!();
            }

            let indent: String = (0..*depth.borrow() * 4).map(|_| ' ').collect();
            log!(
                "{}({} \"{}\" {}",
                indent,
                production,
                String::from_utf8_lossy(input.as_ref()),
                input.len(),
            );
            *depth.borrow_mut() += 1;
        });
        AutoLogParse
    }

    #[cfg(not(feature = "logging"))]
    #[inline(always)]
    fn new(_: &'static str, _: IndexStr) -> AutoLogParse {
        AutoLogParse
    }
}

#[cfg(feature = "logging")]
impl Drop for AutoLogParse {
    fn drop(&mut self) {
        LOG_DEPTH.with(|depth| {
            *depth.borrow_mut() -= 1;
            let indent: String = (0..*depth.borrow() * 4).map(|_| ' ').collect();
            log!("{})", indent);
        });
    }
}

/// Performs the two operations that begin every parse:
///
/// 1. Keeps track of recursion levels and early returns with an error if there
///    is too much recursion.
///
/// 2. Automatically log start and end parsing in an s-expression format, when the
///    `logging` feature is enabled.
macro_rules! try_begin_parse {
    ( $production:expr , $ctx:expr , $input:expr ) => {
        let _log = AutoLogParse::new($production, $input);
        let _auto_check_recursion = AutoParseRecursion::new($ctx)?;
    };
}

struct AutoLogDemangle;

impl AutoLogDemangle {
    #[cfg(feature = "logging")]
    fn new<P, W>(
        production: &P,
        ctx: &DemangleContext<W>,
        scope: Option<ArgScopeStack>,
        is_inner: bool,
    ) -> AutoLogDemangle
    where
        P: ?Sized + fmt::Debug,
        W: DemangleWrite,
    {
        LOG_DEPTH.with(|depth| {
            if *depth.borrow() == 0 {
                println!();
            }

            let indent: String = (0..*depth.borrow() * 4).map(|_| ' ').collect();
            log!("{}(", indent);
            log!(
                "{}  {}{:?}",
                indent,
                if is_inner { "as_inner: " } else { "" },
                production
            );
            log!("{}  inner = {:?}", indent, ctx.inner);
            log!("{}  scope = {:?}", indent, scope);

            *depth.borrow_mut() += 1;
        });
        AutoLogDemangle
    }

    #[cfg(not(feature = "logging"))]
    #[inline(always)]
    fn new<P, W>(
        _: &P,
        _: &DemangleContext<W>,
        _: Option<ArgScopeStack>,
        _: bool,
    ) -> AutoLogDemangle
    where
        P: ?Sized + fmt::Debug,
        W: DemangleWrite,
    {
        AutoLogDemangle
    }
}

#[cfg(feature = "logging")]
impl Drop for AutoLogDemangle {
    fn drop(&mut self) {
        LOG_DEPTH.with(|depth| {
            *depth.borrow_mut() -= 1;
            let indent: String = (0..*depth.borrow() * 4).map(|_| ' ').collect();
            log!("{})", indent);
        });
    }
}

/// Automatically log start and end demangling in an s-expression format, when
/// the `logging` feature is enabled.
macro_rules! try_begin_demangle {
    ( $production:expr, $ctx:expr, $scope:expr ) => {{
        let _log = AutoLogDemangle::new($production, $ctx, $scope, false);
        &mut AutoParseDemangle::new($ctx)?
    }};
}

/// Automatically log start and end demangling in an s-expression format, when
/// the `logging` feature is enabled.
macro_rules! try_begin_demangle_as_inner {
    ( $production:expr, $ctx:expr, $scope:expr ) => {{
        let _log = AutoLogDemangle::new($production, $ctx, $scope, true);
        &mut AutoParseDemangle::new($ctx)?
    }};
}

#[derive(Debug, Default, Clone, Copy)]
struct ParseContextState {
    // The current recursion level. Should always be less than or equal to the
    // maximum.
    recursion_level: u32,
    // Whether or not we are currently parsing a conversion operator.
    in_conversion: bool,
}

/// Common context needed when parsing.
#[derive(Debug, Clone)]
pub struct ParseContext {
    // Maximum amount of recursive parsing calls we will allow. If this is too
    // large, we can blow the stack.
    max_recursion: u32,
    // Mutable state within the `ParseContext`.
    state: Cell<ParseContextState>,
}

impl ParseContext {
    /// Construct a new `ParseContext`.
    pub fn new(options: ParseOptions) -> ParseContext {
        ParseContext {
            max_recursion: options.recursion_limit.map(|v| v.get()).unwrap_or(96),
            state: Cell::new(ParseContextState::default()),
        }
    }

    /// Get the current recursion level for this context.
    pub fn recursion_level(&self) -> u32 {
        self.state.get().recursion_level
    }

    #[inline]
    fn enter_recursion(&self) -> error::Result<()> {
        let mut state = self.state.get();
        let new_recursion_level = state.recursion_level + 1;

        if new_recursion_level >= self.max_recursion {
            log!("Hit too much recursion at level {}", self.max_recursion);
            Err(error::Error::TooMuchRecursion)
        } else {
            state.recursion_level = new_recursion_level;
            self.state.set(state);
            Ok(())
        }
    }

    #[inline]
    fn exit_recursion(&self) {
        let mut state = self.state.get();
        debug_assert!(state.recursion_level >= 1);
        state.recursion_level -= 1;
        self.state.set(state);
    }

    #[inline]
    fn in_conversion(&self) -> bool {
        self.state.get().in_conversion
    }

    fn set_in_conversion(&self, in_conversion: bool) -> bool {
        let mut state = self.state.get();
        let previously_in_conversion = state.in_conversion;
        state.in_conversion = in_conversion;
        self.state.set(state);
        previously_in_conversion
    }
}

/// An RAII type to automatically check the recursion level against the
/// maximum. If the maximum has been crossed, return an error. Otherwise,
/// increment the level upon construction, and decrement it upon destruction.
struct AutoParseRecursion<'a>(&'a ParseContext);

impl<'a> AutoParseRecursion<'a> {
    #[inline]
    fn new(ctx: &'a ParseContext) -> error::Result<AutoParseRecursion<'a>> {
        ctx.enter_recursion()?;
        Ok(AutoParseRecursion(ctx))
    }
}

impl<'a> Drop for AutoParseRecursion<'a> {
    #[inline]
    fn drop(&mut self) {
        self.0.exit_recursion();
    }
}

/// A trait for anything that can be parsed from an `IndexStr` and return a
/// `Result` of the parsed `Self` value and the rest of the `IndexStr` input
/// that has not been consumed in parsing the `Self` value.
///
/// For AST types representing productions which have `<substitution>` as a
/// possible right hand side, do not implement this trait directly. Instead,
/// make a newtype over `usize`, parse either the `<substitution>` back
/// reference or "real" value, insert the "real" value into the substitution
/// table if needed, and *always* return the newtype index into the substitution
/// table.
#[doc(hidden)]
pub trait Parse: Sized {
    /// Parse the `Self` value from `input` and return it, updating the
    /// substitution table as needed.
    fn parse<'a, 'b>(
        ctx: &'a ParseContext,
        subs: &'a mut SubstitutionTable,
        input: IndexStr<'b>,
    ) -> Result<(Self, IndexStr<'b>)>;
}

/// Determine whether this AST node is an instantiated[*] template function, and
/// get its concrete template arguments.
///
/// [*] Note that we will never see an abstract, un-instantiated template
/// function, since they don't end up in object files and don't get mangled
/// names.
trait GetTemplateArgs {
    /// Returns `Some` if this is a template function, `None` otherwise.
    fn get_template_args<'a>(&'a self, subs: &'a SubstitutionTable) -> Option<&'a TemplateArgs>;
}

/// A leaf name is the part the name that describes some type or class without
/// any leading namespace qualifiers.
///
/// This is used when figuring out how to format constructors and destructors,
/// which are formatted as `gooble::dodo::Thing::~Thing()` but we don't have
/// direct access to `Thing` in the `CtorDtorName` AST.
#[derive(Debug)]
pub(crate) enum LeafName<'a> {
    SourceName(&'a SourceName),
    WellKnownComponent(&'a WellKnownComponent),
    Closure(&'a ClosureTypeName),
    UnnamedType(&'a UnnamedTypeName),
}

impl<'subs, W> DemangleAsLeaf<'subs, W> for LeafName<'subs>
where
    W: 'subs + DemangleWrite,
{
    fn demangle_as_leaf<'me, 'ctx>(
        &'me self,
        ctx: &'ctx mut DemangleContext<'subs, W>,
    ) -> fmt::Result {
        match *self {
            LeafName::SourceName(sn) => sn.demangle(ctx, None),
            LeafName::Closure(c) => c.demangle(ctx, None),
            LeafName::WellKnownComponent(wkc) => wkc.demangle_as_leaf(ctx),
            LeafName::UnnamedType(utn) => utn.demangle_as_leaf(ctx),
        }
    }
}

/// Determine whether this AST node is some kind (potentially namespaced) name
/// and if so get its leaf name.
pub(crate) trait GetLeafName<'a> {
    fn get_leaf_name(&'a self, subs: &'a SubstitutionTable) -> Option<LeafName<'a>>;
}

/// Determine whether this AST node is a constructor, destructor, or conversion
/// function.
pub(crate) trait IsCtorDtorConversion {
    fn is_ctor_dtor_conversion(&self, subs: &SubstitutionTable) -> bool;
}

/// When formatting a mangled symbol's parsed AST as a demangled symbol, we need
/// to resolve indirect references to template and function arguments with
/// direct `TemplateArg` and `Type` references respectively.
///
/// Note that which set of arguments are implicitly referenced change as we
/// enter and leave different functions' scope. One might usually use de Brujin
/// indices to keep arguments within scopes separated from each other, but the
/// Itanium C++ ABI does not allow us the luxury. AFAIK, when the ABI was first
/// drafted, C++ did not have lambdas, and the issue did not come up at all
/// since a function simply couldn't refer to the types of closed over
/// variables.
///
/// This trait is implemented by anything that can potentially resolve arguments
/// for us.
trait ArgScope<'me, 'ctx>: fmt::Debug {
    /// Get the current scope's leaf name.
    fn leaf_name(&'me self) -> Result<LeafName<'ctx>>;

    /// Get the current scope's `index`th template argument.
    fn get_template_arg(&'me self, index: usize)
        -> Result<(&'ctx TemplateArg, &'ctx TemplateArgs)>;

    #[allow(unused)]
    /// Get the current scope's `index`th function argument's type.
    fn get_function_arg(&'me self, index: usize) -> Result<&'ctx Type>;
}

/// An `ArgScopeStack` represents the current function and template demangling
/// scope we are within. As we enter new demangling scopes, we construct new
/// `ArgScopeStack`s whose `prev` references point back to the old ones. These
/// `ArgScopeStack`s are kept on the native stack, and as functions return, they
/// go out of scope and we use the previous `ArgScopeStack`s again.
#[derive(Copy, Clone, Debug)]
pub struct ArgScopeStack<'prev, 'subs>
where
    'subs: 'prev,
{
    item: &'subs dyn ArgScope<'subs, 'subs>,
    in_arg: Option<(usize, &'subs TemplateArgs)>,
    prev: Option<&'prev ArgScopeStack<'prev, 'subs>>,
}

/// When we first begin demangling, we haven't entered any function or template
/// demangling scope and we don't have any useful `ArgScopeStack`. Therefore, we
/// are never actually dealing with `ArgScopeStack` directly in practice, but
/// always an `Option<ArgScopeStack>` instead. Nevertheless, we want to define
/// useful methods on `Option<ArgScopeStack>`.
///
/// A custom "extension" trait with exactly one implementor: Rust's principled
/// monkey patching!
trait ArgScopeStackExt<'prev, 'subs>: Copy {
    /// Push a new `ArgScope` onto this `ArgScopeStack` and return the new
    /// `ArgScopeStack` with the pushed resolver on top.
    fn push(
        &'prev self,
        item: &'subs dyn ArgScope<'subs, 'subs>,
    ) -> Option<ArgScopeStack<'prev, 'subs>>;
}

impl<'prev, 'subs> ArgScopeStackExt<'prev, 'subs> for Option<ArgScopeStack<'prev, 'subs>> {
    fn push(
        &'prev self,
        item: &'subs dyn ArgScope<'subs, 'subs>,
    ) -> Option<ArgScopeStack<'prev, 'subs>> {
        log!("ArgScopeStack::push: {:?}", item);
        Some(ArgScopeStack {
            prev: self.as_ref(),
            in_arg: None,
            item: item,
        })
    }
}

/// A stack of `ArgScope`s is itself an `ArgScope`!
impl<'prev, 'subs> ArgScope<'prev, 'subs> for Option<ArgScopeStack<'prev, 'subs>> {
    fn leaf_name(&'prev self) -> Result<LeafName<'subs>> {
        let mut scope = self.as_ref();
        while let Some(s) = scope {
            if let Ok(c) = s.item.leaf_name() {
                return Ok(c);
            }
            scope = s.prev;
        }
        Err(error::Error::BadLeafNameReference)
    }

    fn get_template_arg(
        &'prev self,
        idx: usize,
    ) -> Result<(&'subs TemplateArg, &'subs TemplateArgs)> {
        let mut scope = self.as_ref();
        while let Some(s) = scope {
            if let Ok((arg, args)) = s.item.get_template_arg(idx) {
                if let Some((in_idx, in_args)) = s.in_arg {
                    if args as *const TemplateArgs == in_args as *const TemplateArgs
                        && in_idx <= idx
                    {
                        return Err(error::Error::ForwardTemplateArgReference);
                    }
                }
                return Ok((arg, args));
            }
            scope = s.prev;
        }

        Err(error::Error::BadTemplateArgReference)
    }

    fn get_function_arg(&'prev self, idx: usize) -> Result<&'subs Type> {
        let mut scope = self.as_ref();
        while let Some(s) = scope {
            if let Ok(arg) = s.item.get_function_arg(idx) {
                return Ok(arg);
            }
            scope = s.prev;
        }

        Err(error::Error::BadFunctionArgReference)
    }
}

#[derive(Debug, Copy, Clone)]
struct DemangleState {
    /// How deep in the demangling are we?
    pub recursion_level: u32,
}

/// An RAII type to automatically check the recursion level against the
/// maximum. If the maximum has been crossed, return an error. Otherwise,
/// increment the level upon construction, and decrement it upon destruction.
struct AutoParseDemangle<'a, 'b, W: 'a + DemangleWrite>(&'b mut DemangleContext<'a, W>);

impl<'a, 'b, W: 'a + DemangleWrite> AutoParseDemangle<'a, 'b, W> {
    #[inline]
    fn new(ctx: &'b mut DemangleContext<'a, W>) -> core::result::Result<Self, fmt::Error> {
        ctx.enter_recursion()?;
        Ok(AutoParseDemangle(ctx))
    }
}

impl<'a, 'b, W: 'a + DemangleWrite> ops::Deref for AutoParseDemangle<'a, 'b, W> {
    type Target = DemangleContext<'a, W>;

    fn deref(&self) -> &Self::Target {
        self.0
    }
}

impl<'a, 'b, W: 'a + DemangleWrite> ops::DerefMut for AutoParseDemangle<'a, 'b, W> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.0
    }
}

impl<'a, 'b, W: 'a + DemangleWrite> Drop for AutoParseDemangle<'a, 'b, W> {
    #[inline]
    fn drop(&mut self) {
        self.0.exit_recursion();
    }
}

/// Common state that is required when demangling a mangled symbol's parsed AST.
#[doc(hidden)]
#[derive(Debug)]
pub struct DemangleContext<'a, W>
where
    W: 'a + DemangleWrite,
{
    // The substitution table built up when parsing the mangled symbol into an
    // AST.
    subs: &'a SubstitutionTable,

    // The maximum recursion
    max_recursion: u32,

    // Sometimes an AST node needs to insert itself as an inner item within one
    // of its children when demangling that child. For example, the AST
    //
    //     (array 10 int)
    //
    // is demangled as `int[10]`, but if we were to demangle the AST
    //
    //     (lvalue-ref (array 10 int))
    //
    // then we would want this demangled form: `int (&) [10]`, which requires
    // the parent lvalue-ref to be passed into the child array's demangling
    // method. This kind of thing also pops up with function pointers.
    //
    // The `inner` stack enables such behavior by allowing us to pass AST
    // parents down to their children as inner items.
    inner: Vec<&'a dyn DemangleAsInner<'a, W>>,

    // The original input string.
    input: &'a [u8],

    // `Identifier`s will be placed here, so `UnnamedTypeName` can utilize and print
    // out the Constructor/Destructor used.
    source_name: Option<&'a str>,

    // What the demangled name is being written to.
    out: &'a mut W,

    // The total number of bytes written to `out`. This is maintained by the
    // `Write` implementation for `DemangleContext`.
    bytes_written: usize,

    // The last char written to `out`, if any.
    last_char_written: Option<char>,

    // We are currently demangling a lambda argument, so template substitution
    // should be suppressed to match libiberty.
    is_lambda_arg: bool,

    // We are currently demangling a template-prefix.
    is_template_prefix: bool,

    // We are currently demangling a template-prefix in a nested-name.
    is_template_prefix_in_nested_name: bool,

    //  `PackExpansion`'s should only print '...', only when there is no template
    //  argument pack.
    is_template_argument_pack: bool,

    // We are currently demangling an object with an explicit named parameter.
    is_explicit_obj_param: bool,

    // Whether to show function parameters.
    // This must be set to true before calling `demangle` on `Encoding`
    // unless that call is via the toplevel call to `MangledName::demangle`.
    show_params: bool,

    // Whether to show function return types.
    // This must be set to true before calling `demangle` on `Encoding`
    // unless that call is via the toplevel call to `MangledName::demangle`.
    show_return_type: bool,

    // Whether to show types of expression literals.
    show_expression_literal_types: bool,

    // recursion protection.
    state: Cell<DemangleState>,
}

impl<'a, W> fmt::Write for DemangleContext<'a, W>
where
    W: 'a + DemangleWrite,
{
    fn write_str(&mut self, s: &str) -> fmt::Result {
        if s.is_empty() {
            return Ok(());
        }

        log!("DemangleContext::write: '{}'", s);

        self.out.write_string(s).map(|_| {
            self.last_char_written = s.chars().last();
            self.bytes_written += s.len();
        })
    }
}

impl<'a, W> DemangleContext<'a, W>
where
    W: 'a + DemangleWrite,
{
    /// Construct a new `DemangleContext`.
    pub fn new(
        subs: &'a SubstitutionTable,
        input: &'a [u8],
        options: DemangleOptions,
        out: &'a mut W,
    ) -> DemangleContext<'a, W> {
        DemangleContext {
            subs: subs,
            max_recursion: options.recursion_limit.map(|v| v.get()).unwrap_or(128),
            inner: vec![],
            input: input,
            source_name: None,
            out: out,
            bytes_written: 0,
            last_char_written: None,
            is_lambda_arg: false,
            is_template_prefix: false,
            is_template_prefix_in_nested_name: false,
            is_template_argument_pack: false,
            is_explicit_obj_param: false,
            show_params: !options.no_params,
            show_return_type: !options.no_return_type,
            show_expression_literal_types: !options.hide_expression_literal_types,
            state: Cell::new(DemangleState { recursion_level: 0 }),
        }
    }

    /// Get the current recursion level for this context.
    pub fn recursion_level(&self) -> u32 {
        self.state.get().recursion_level
    }

    #[inline]
    fn enter_recursion(&self) -> fmt::Result {
        let mut state = self.state.get();
        let new_recursion_level = state.recursion_level + 1;

        if new_recursion_level >= self.max_recursion {
            log!("Hit too much recursion at level {}", self.max_recursion);
            Err(Default::default())
        } else {
            state.recursion_level = new_recursion_level;
            self.state.set(state);
            Ok(())
        }
    }

    #[inline]
    fn exit_recursion(&self) {
        let mut state = self.state.get();
        debug_assert!(state.recursion_level >= 1);
        state.recursion_level -= 1;
        self.state.set(state);
    }

    #[inline]
    fn ensure(&mut self, ch: char) -> fmt::Result {
        if self.last_char_written == Some(ch) {
            Ok(())
        } else {
            write!(self, "{}", ch)?;
            Ok(())
        }
    }

    #[inline]
    fn ensure_space(&mut self) -> fmt::Result {
        self.ensure(' ')
    }

    #[inline]
    fn push_inner(&mut self, item: &'a dyn DemangleAsInner<'a, W>) {
        log!("DemangleContext::push_inner: {:?}", item);
        self.inner.push(item);
    }

    #[inline]
    fn pop_inner(&mut self) -> Option<&'a dyn DemangleAsInner<'a, W>> {
        let popped = self.inner.pop();
        log!("DemangleContext::pop_inner: {:?}", popped);
        popped
    }

    #[inline]
    fn pop_inner_if(&mut self, inner: &'a dyn DemangleAsInner<'a, W>) -> bool {
        let last = match self.inner.last() {
            None => return false,
            Some(last) => *last,
        };

        if ptr::eq(last, inner) {
            self.inner.pop();
            true
        } else {
            false
        }
    }

    fn demangle_inner_prefixes<'prev>(
        &mut self,
        scope: Option<ArgScopeStack<'prev, 'a>>,
    ) -> fmt::Result {
        log!("DemangleContext::demangle_inner_prefixes");
        let mut new_inner = vec![];
        while let Some(inner) = self.pop_inner() {
            if inner
                .downcast_to_function_type()
                .map_or(false, |f| !f.cv_qualifiers.is_empty())
            {
                log!(
                    "DemangleContext::demangle_inner_prefixes: not a prefix, saving: {:?}",
                    inner
                );
                new_inner.push(inner);
            } else {
                log!(
                    "DemangleContext::demangle_inner_prefixes: demangling prefix: {:?}",
                    inner
                );
                inner.demangle_as_inner(self, scope)?;
            }
        }
        new_inner.reverse();
        self.inner = new_inner;
        Ok(())
    }

    fn demangle_inners<'prev>(&mut self, scope: Option<ArgScopeStack<'prev, 'a>>) -> fmt::Result {
        while let Some(inner) = self.pop_inner() {
            inner.demangle_as_inner(self, scope)?;
        }
        Ok(())
    }

    fn set_source_name(&mut self, start: usize, end: usize) {
        let ident = &self.input[start..end];
        self.source_name = str::from_utf8(ident).ok();
    }

    fn push_demangle_node(&mut self, t: DemangleNodeType) {
        self.out.push_demangle_node(t);
    }

    /// This should not be called on error paths.
    /// pop_inner_if already doesn't balance if there are errors.
    fn pop_demangle_node(&mut self) {
        self.out.pop_demangle_node();
    }
}

#[doc(hidden)]
#[derive(Debug)]
pub struct AutoDemangleContextInnerBarrier<'ctx, 'a, W>
where
    W: 'a + DemangleWrite,
    'a: 'ctx,
{
    ctx: &'ctx mut DemangleContext<'a, W>,
    saved_inner: Vec<&'a dyn DemangleAsInner<'a, W>>,
}

impl<'ctx, 'a, W> AutoDemangleContextInnerBarrier<'ctx, 'a, W>
where
    W: 'a + DemangleWrite,
    'a: 'ctx,
{
    /// Set aside the current inner stack on the demangle context.
    pub fn new(ctx: &'ctx mut DemangleContext<'a, W>) -> Self {
        let mut saved_inner = vec![];
        mem::swap(&mut saved_inner, &mut ctx.inner);
        AutoDemangleContextInnerBarrier {
            ctx: ctx,
            saved_inner: saved_inner,
        }
    }
}

impl<'ctx, 'a, W> ops::Deref for AutoDemangleContextInnerBarrier<'ctx, 'a, W>
where
    W: 'a + DemangleWrite,
    'a: 'ctx,
{
    type Target = DemangleContext<'a, W>;

    fn deref(&self) -> &Self::Target {
        self.ctx
    }
}

impl<'ctx, 'a, W> ops::DerefMut for AutoDemangleContextInnerBarrier<'ctx, 'a, W>
where
    W: 'a + DemangleWrite,
    'a: 'ctx,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.ctx
    }
}

impl<'ctx, 'a, W> Drop for AutoDemangleContextInnerBarrier<'ctx, 'a, W>
where
    W: 'a + DemangleWrite,
    'a: 'ctx,
{
    fn drop(&mut self) {
        // NB: We cannot assert that the context's inner is empty here,
        // because if demangling failed we'll unwind the stack without
        // using everything that put on the inner.
        if !self.ctx.inner.is_empty() {
            log!("Context inner was not emptied, did demangling fail?");
        }
        mem::swap(&mut self.saved_inner, &mut self.ctx.inner);
    }
}

/// The inner stack allows passing AST nodes down deeper into the tree so that
/// nodes that logically precede something (e.g. PointerRef) can show up after
/// that thing in the demangled output. What's on the stack may not always be
/// intended for the first node that looks at the stack to grab, though.
///
/// Consider a function with template arguments and parameters, f<T>(a).
/// The function parameters logically precede the template arguments in the AST,
/// but they must be reversed in the output. The parameters end up on the inner
/// stack before processing the template argument nodes. If we're not careful,
/// a node inside the template arguments might pick the function parameters
/// off of the inner stack!
///
/// To solve this, certain nodes act as "inner barriers". By using this macro,
/// they set the existing inner stack aside and replace it with an empty stack
/// while visiting their children. This allows these barrier nodes to have
/// completely self-contained children.
macro_rules! inner_barrier {
    ( $ctx:ident ) => {
        let mut _ctx = AutoDemangleContextInnerBarrier::new($ctx);
        let $ctx = &mut _ctx;
    };
}

/// Any AST node that can be printed in a demangled form.
#[doc(hidden)]
pub trait Demangle<'subs, W>: fmt::Debug
where
    W: 'subs + DemangleWrite,
{
    /// Write the demangled form of this AST node to the given context.
    fn demangle<'prev, 'ctx>(
        &'subs self,
        ctx: &'ctx mut DemangleContext<'subs, W>,
        scope: Option<ArgScopeStack<'prev, 'subs>>,
    ) -> fmt::Result;
}

/// Any AST node that can be printed as an inner type.
///
/// See the comments surrounding `DemangleContext::inner` for details.
#[doc(hidden)]
pub trait DemangleAsInner<'subs, W>: Demangle<'subs, W>
where
    W: 'subs + DemangleWrite,
{
    /// Write the inner demangling form of this AST node to the given context.
    fn demangle_as_inner<'prev, 'ctx>(
        &'subs self,
        ctx: &'ctx mut DemangleContext<'subs, W>,
        scope: Option<ArgScopeStack<'prev, 'subs>>,
    ) -> fmt::Result {
        self.demangle(ctx, scope)
    }

    /// Cast this `DemangleAsInner` to a `Type`.
    fn downcast_to_type(&self) -> Option<&Type> {
        None
    }

    /// Cast this `DemangleAsInner` to a `FunctionType`.
    fn downcast_to_function_type(&self) -> Option<&FunctionType> {
        None
    }

    /// Cast this `DemangleAsInner` to an `ArrayType`.
    fn downcast_to_array_type(&self) -> Option<&ArrayType> {
        None
    }

    /// Cast this `DemangleAsInner` to a `PointerToMember`.
    fn downcast_to_pointer_to_member(&self) -> Option<&PointerToMemberType> {
        None
    }

    fn is_qualified(&self) -> bool {
        false
    }
}

/// Demangle this thing in the leaf name position.
///
/// For most things this should be the same as its `Demangle`
/// implementation. For `WellKnownComponent`s we need to strip the embedded
/// `std::` namespace prefix.
pub(crate) trait DemangleAsLeaf<'subs, W>
where
    W: 'subs + DemangleWrite,
{
    fn demangle_as_leaf<'me, 'ctx>(
        &'me self,
        ctx: &'ctx mut DemangleContext<'subs, W>,
    ) -> fmt::Result;
}

macro_rules! reference_newtype {
    ( $newtype_name:ident , $oldtype:ty ) => {
        #[derive(Debug)]
        struct $newtype_name($oldtype);

        impl $newtype_name {
            #[allow(clippy::ptr_arg)]
            #[allow(unsafe_code)]
            fn new(types: &$oldtype) -> &$newtype_name {
                unsafe {
                    // This is safe because we only create an immutable
                    // reference. We are not breaking unique mutable aliasing
                    // requirements. An immutable reference does not allow
                    // dropping the referent, so no worries about double-free
                    // (additionally, see the assertion inside `Drop` below).
                    &*(types as *const $oldtype as *const $newtype_name)
                }
            }
        }

        impl Drop for $newtype_name {
            fn drop(&mut self) {
                unreachable!(
                    "Dropping implies we dereferenced and took ownership, which \
                              is not safe for this newtype"
                );
            }
        }

        impl ops::Deref for $newtype_name {
            type Target = $oldtype;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }
    };
}

// We can't implement `DemangleAsInner` for newtypes of `[TypeHandle]` like we
// want to because it is unsized and we need to make trait objects out of
// `DemangleAsInner` for pushing onto the context's inner stack. Therefore, we
// have this inelegant newtyping of `Vec<TypeHandle>`.

// A set of function arguments.
reference_newtype!(FunctionArgList, Vec<TypeHandle>);

// A set of function arguments prefixed by a return type (which we want to
// ignore).
reference_newtype!(FunctionArgListAndReturnType, Vec<TypeHandle>);

// A newtype around a slice of type handles that we format as function
// arguments.
reference_newtype!(FunctionArgSlice, [TypeHandle]);

// Demangle a slice of TypeHandle as a function argument list.
impl<'subs, W> Demangle<'subs, W> for FunctionArgSlice
where
    W: 'subs + DemangleWrite,
{
    fn demangle<'prev, 'ctx>(
        &'subs self,
        ctx: &'ctx mut DemangleContext<'subs, W>,
        scope: Option<ArgScopeStack<'prev, 'subs>>,
    ) -> fmt::Result {
        let ctx = try_begin_demangle!(self, ctx, scope);

        let mut saw_needs_paren = false;
        let (needs_space, needs_paren) = ctx
            .inner
            .iter()
            .rev()
            .map(|inner| {
                if inner.downcast_to_pointer_to_member().is_some() {
                    (true, true)
                } else {
                    match inner.downcast_to_type() {
                        Some(&Type::Qualified(..))
                        | Some(&Type::Complex(_))
                        | Some(&Type::Imaginary(_))
                        | Some(&Type::PointerToMember(_)) => (true, true),
                        Some(&Type::PointerTo(_))
                        | Some(&Type::LvalueRef(_))
                        | Some(&Type::RvalueRef(_)) => (false, true),
                        _ => (false, false),
                    }
                }
            })
            .take_while(|&(_, needs_paren)| {
                if saw_needs_paren {
                    false
                } else {
                    saw_needs_paren |= needs_paren;
                    true
                }
            })
            .fold(
                (false, false),
                |(space, paren), (next_space, next_paren)| {
                    (space || next_space, paren || next_paren)
                },
            );

        if needs_paren {
            let needs_space = needs_space
                || match ctx.last_char_written {
                    Some('(') | Some('*') => false,
                    _ => true,
                };

            if needs_space {
                ctx.ensure_space()?;
            }

            write!(ctx, "(")?;
        }

        ctx.demangle_inner_prefixes(scope)?;

        if needs_paren {
            write!(ctx, ")")?;
        }

        write!(ctx, "(")?;

        // To maintain compatibility with libiberty, print `()` instead of
        // `(void)` for functions that take no arguments.
        if self.len() == 1 && self[0].is_void() {
            write!(ctx, ")")?;
            return Ok(());
        }

        // Only the first param should have `this`.
        if ctx.is_explicit_obj_param {
            write!(ctx, "this ")?;
            ctx.is_explicit_obj_param = false;
        }

        let mut need_comma = false;
        for arg in self.iter() {
            if need_comma {
                write!(ctx, ", ")?;
            }
            arg.demangle(ctx, scope)?;
            need_comma = true;
        }

        write!(ctx, ")")?;

        ctx.demangle_inners(scope)
    }
}

impl<'subs, W> Demangle<'subs, W> for FunctionArgList
where
    W: 'subs + DemangleWrite,
{
    fn demangle<'prev, 'ctx>(
        &'subs self,
        ctx: &'ctx mut DemangleContext<'subs, W>,
        scope: Option<ArgScopeStack<'prev, 'subs>>,
    ) -> fmt::Result {
        FunctionArgSlice::new(&self.0[..]).demangle(ctx, scope)
    }
}

impl<'subs, W> DemangleAsInner<'subs, W> for FunctionArgList where W: 'subs + DemangleWrite {}

impl<'subs, W> Demangle<'subs, W> for FunctionArgListAndReturnType
where
    W: 'subs + DemangleWrite,
{
    fn demangle<'prev, 'ctx>(
        &'subs self,
        ctx: &'ctx mut DemangleContext<'subs, W>,
        scope: Option<ArgScopeStack<'prev, 'subs>>,
    ) -> fmt::Result {
        FunctionArgSlice::new(&self.0[1..]).demangle(ctx, scope)
    }
}

impl<'subs, W> DemangleAsInner<'subs, W> for FunctionArgListAndReturnType where
    W: 'subs + DemangleWrite
{
}

/// Define a handle to a AST type that lives inside the substitution table. A
/// handle is always either an index into the substitution table, or it is a
/// reference to a "well-known" component.
///
/// This declares:
///
/// - The enum of either a back reference into the substitution table or a
///   reference to a "well-known" component
/// - a `Demangle` impl that proxies to the appropriate `Substitutable` in the
///   `SubstitutionTable`
macro_rules! define_handle {
    (
        $(#[$attr:meta])*
        pub enum $typename:ident
    ) => {
        define_handle! {
            $(#[$attr])*
            pub enum $typename {}
        }
    };

    (
        $(#[$attr:meta])*
        pub enum $typename:ident {
            $(
                $( #[$extra_attr:meta] )*
                extra $extra_variant:ident ( $extra_variant_ty:ty ),
            )*
        }
    ) => {
        $(#[$attr])*
        #[derive(Clone, Debug, PartialEq, Eq)]
        pub enum $typename {
            /// A reference to a "well-known" component.
            WellKnown(WellKnownComponent),

            /// A back-reference into the substitution table to a component we
            /// have already parsed.
            BackReference(usize),

            $(
                $( #[$extra_attr] )*
                $extra_variant( $extra_variant_ty ),
            )*
        }

        impl $typename {
            /// If this is a `BackReference`, get its index.
            pub fn back_reference(&self) -> Option<usize> {
                match *self {
                    $typename::BackReference(n) => Some(n),
                    _ => None,
                }
            }
        }

        impl<'subs, W> Demangle<'subs, W> for $typename
        where
            W: 'subs + DemangleWrite
        {
            #[inline]
            fn demangle<'prev, 'ctx>(&'subs self,
                                     ctx: &'ctx mut DemangleContext<'subs, W>,
                                     scope: Option<ArgScopeStack<'prev, 'subs>>)
                                     -> fmt::Result {
                match *self {
                    $typename::WellKnown(ref comp) => comp.demangle(ctx, scope),
                    $typename::BackReference(idx) => ctx.subs[idx].demangle(ctx, scope),
                    $(
                        $typename::$extra_variant(ref extra) => extra.demangle(ctx, scope),
                    )*
                }
            }
        }

        impl<'a> GetLeafName<'a> for $typename {
            fn get_leaf_name(&'a self, subs: &'a SubstitutionTable) -> Option<LeafName<'a>> {
                match *self {
                    $typename::WellKnown(ref wk) => wk.get_leaf_name(subs),
                    $typename::BackReference(idx) => {
                        subs.get(idx).and_then(|s| s.get_leaf_name(subs))
                    }
                    $(
                        $typename::$extra_variant(ref e) => e.get_leaf_name(subs),
                    )*
                }
            }
        }
    };
}

/// A handle to a component that is usually substitutable, and lives in the
/// substitutions table, but in this particular case does not qualify for
/// substitutions.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NonSubstitution(usize);

impl<'subs, W> Demangle<'subs, W> for NonSubstitution
where
    W: 'subs + DemangleWrite,
{
    fn demangle<'prev, 'ctx>(
        &'subs self,
        ctx: &'ctx mut DemangleContext<'subs, W>,
        scope: Option<ArgScopeStack<'prev, 'subs>>,
    ) -> fmt::Result {
        ctx.subs.non_substitution(self.0).demangle(ctx, scope)
    }
}

impl<'a> GetLeafName<'a> for NonSubstitution {
    fn get_leaf_name(&'a self, subs: &'a SubstitutionTable) -> Option<LeafName<'a>> {
        subs.get_non_substitution(self.0)
            .and_then(|ns| ns.get_leaf_name(subs))
    }
}

/// Define a "vocabulary" nonterminal, something like `OperatorName` or
/// `CtorDtorName` that's basically a big list of constant strings.
///
/// This declares:
///
/// - the enum itself
/// - a `Parse` impl
/// - a `Demangle` impl
///
/// See the definition of `CTorDtorName` for an example of its use.
///
/// Optionally, a piece of user data can be attached to the definitions
/// and be returned by a generated accessor. See `SimpleOperatorName` for
/// an example.
macro_rules! define_vocabulary {
    ( $(#[$attr:meta])* pub enum $typename:ident {
        $($variant:ident ( $mangled:expr, $printable:expr )),*
    } ) => {

        $(#[$attr])*
        pub enum $typename {
            $(
                #[doc=$printable]
                $variant
            ),*
        }

        impl Parse for $typename {
            fn parse<'a, 'b>(ctx: &'a ParseContext,
                             _subs: &'a mut SubstitutionTable,
                             input: IndexStr<'b>)
                             -> Result<($typename, IndexStr<'b>)> {
                try_begin_parse!(stringify!($typename), ctx, input);

                let mut found_prefix = false;
                $(
                    if let Some((head, tail)) = input.try_split_at($mangled.len()) {
                        if head.as_ref() == $mangled {
                            return Ok(($typename::$variant, tail));
                        }
                    } else {
                        found_prefix |= 0 < input.len() &&
                            input.len() < $mangled.len() &&
                            input.as_ref() == &$mangled[..input.len()];
                    }
                )*

                if input.is_empty() || found_prefix {
                    Err(error::Error::UnexpectedEnd)
                } else {
                    Err(error::Error::UnexpectedText)
                }
            }
        }

        impl<'subs, W> Demangle<'subs, W> for $typename
        where
            W: 'subs + DemangleWrite,
        {
            fn demangle<'prev, 'ctx>(
                &'subs self,
                ctx: &'ctx mut DemangleContext<'subs, W>,
                scope: Option<ArgScopeStack<'prev, 'subs>>
            ) -> fmt::Result {
                let ctx = try_begin_demangle!(self, ctx, scope);

                write!(ctx, "{}", match *self {
                    $(
                        $typename::$variant => $printable
                    ),*
                })
            }
        }

        impl $typename {
            #[allow(dead_code)]
            #[inline]
            fn starts_with(byte: u8) -> bool {
                $(
                    if $mangled[0] == byte {
                        return true;
                    }
                )*

                false
            }
        }
    };
    ( $(#[$attr:meta])* pub enum $typename:ident {
        $($variant:ident ( $mangled:expr, $printable:expr, $userdata:expr)),*
    }

      impl $typename2:ident {
          fn $fn_name:ident(&self) -> $userdata_ty:ty;
    } ) => {
        define_vocabulary! {
            $(#[$attr])*
            pub enum $typename {
                $(
                    $variant ( $mangled, $printable )
                ),*
            }
        }

        impl $typename2 {
            fn $fn_name(&self) -> $userdata_ty {
                match *self {
                    $(
                        $typename2::$variant => $userdata,
                    )*
                }
            }
        }
    };
}

/// The root AST node, and starting production.
///
/// ```text
/// <mangled-name> ::= _Z <encoding> [<clone-suffix>]*
///                ::= ___Z <encoding> <block_invoke>
///                ::= <type>
///
/// <block_invoke> ::= _block_invoke
///                ::= _block_invoke<decimal-digit>+
///                ::= _block_invoke_<decimal-digit>+
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MangledName {
    /// The encoding of the mangled symbol name.
    Encoding(Encoding, Vec<CloneSuffix>),

    /// The encoding of the mangled symbol name.
    BlockInvoke(Encoding, Option<isize>),

    /// A top-level type. Technically not allowed by the standard, however in
    /// practice this can happen, and is tested for by libiberty.
    Type(TypeHandle),

    /// A global constructor or destructor. This is another de facto standard
    /// extension (I think originally from `g++`?) that is not actually part of
    /// the standard proper.
    GlobalCtorDtor(GlobalCtorDtor),
}

impl Parse for MangledName {
    fn parse<'a, 'b>(
        ctx: &'a ParseContext,
        subs: &'a mut SubstitutionTable,
        input: IndexStr<'b>,
    ) -> Result<(MangledName, IndexStr<'b>)> {
        try_begin_parse!("MangledName", ctx, input);

        if let Ok(tail) = consume(b"_Z", input).or_else(|_| consume(b"__Z", input)) {
            let (encoding, tail) = Encoding::parse(ctx, subs, tail)?;
            let (clone_suffixes, tail) = zero_or_more(ctx, subs, tail)?;
            return Ok((MangledName::Encoding(encoding, clone_suffixes), tail));
        }

        if let Ok(tail) = consume(b"___Z", input).or_else(|_| consume(b"____Z", input)) {
            let (encoding, tail) = Encoding::parse(ctx, subs, tail)?;
            let tail = consume(b"_block_invoke", tail)?;

            let tail_opt = match consume(b"_", tail).or_else(|_| consume(b".", tail)) {
                Ok(tail) => Some(parse_number(10, false, tail)?),
                Err(_) => parse_number(10, false, tail).ok(),
            };

            let (digits, tail) = match tail_opt {
                Some((digits, tail)) => (Some(digits), tail),
                None => (None, tail),
            };

            return Ok((MangledName::BlockInvoke(encoding, digits), tail));
        }

        if let Ok(tail) = consume(b"_GLOBAL_", input) {
            let (global_ctor_dtor, tail) = GlobalCtorDtor::parse(ctx, subs, tail)?;
            return Ok((MangledName::GlobalCtorDtor(global_ctor_dtor), tail));
        }

        // The libiberty tests also specify that a type can be top level,
        // and they are not prefixed with "_Z".
        let (ty, tail) = TypeHandle::parse(ctx, subs, input)?;
        Ok((MangledName::Type(ty), tail))
    }
}

impl<'subs, W> Demangle<'subs, W> for MangledName
where
    W: 'subs + DemangleWrite,
{
    fn demangle<'prev, 'ctx>(
        &'subs self,
        ctx: &'ctx mut DemangleContext<'subs, W>,
        scope: Option<ArgScopeStack<'prev, 'subs>>,
    ) -> fmt::Result {
        let ctx = try_begin_demangle!(self, ctx, scope);

        match *self {
            MangledName::Encoding(ref enc, ref cs) => {
                enc.demangle(ctx, scope)?;
                if !cs.is_empty() && ctx.show_params {
                    for clone_suffix in cs {
                        clone_suffix.demangle(ctx, scope)?;
                    }
                }
                Ok(())
            }
            MangledName::BlockInvoke(ref enc, _) => {
                write!(ctx, "invocation function for block in ")?;
                enc.demangle(ctx, scope)?;
                Ok(())
            }
            MangledName::Type(ref ty) => ty.demangle(ctx, scope),
            MangledName::GlobalCtorDtor(ref gcd) => gcd.demangle(ctx, scope),
        }
    }
}

/// The `<encoding>` production.
///
/// ```text
/// <encoding> ::= <function name> <bare-function-type>
///            ::= <data name>
///            ::= <special-name>
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Encoding {
    /// An encoded function.
    Function(Name, BareFunctionType),

    /// An encoded static variable.
    Data(Name),

    /// A special encoding.
    Special(SpecialName),
}

impl Parse for Encoding {
    fn parse<'a, 'b>(
        ctx: &'a ParseContext,
        subs: &'a mut SubstitutionTable,
        input: IndexStr<'b>,
    ) -> Result<(Encoding, IndexStr<'b>)> {
        try_begin_parse!("Encoding", ctx, input);

        if let Ok((name, tail)) = try_recurse!(Name::parse(ctx, subs, input)) {
            if let Ok((ty, tail)) = try_recurse!(BareFunctionType::parse(ctx, subs, tail)) {
                return Ok((Encoding::Function(name, ty), tail));
            } else {
                return Ok((Encoding::Data(name), tail));
            }
        }

        let (name, tail) = SpecialName::parse(ctx, subs, input)?;
        Ok((Encoding::Special(name), tail))
    }
}

impl<'subs, W> Demangle<'subs, W> for Encoding
where
    W: 'subs + DemangleWrite,
{
    fn demangle<'prev, 'ctx>(
        &'subs self,
        ctx: &'ctx mut DemangleContext<'subs, W>,
        scope: Option<ArgScopeStack<'prev, 'subs>>,
    ) -> fmt::Result {
        let ctx = try_begin_demangle!(self, ctx, scope);
        inner_barrier!(ctx);

        match *self {
            Encoding::Function(ref name, ref fun_ty) => {
                // Even if this function takes no args and doesn't have a return
                // value (see below), it will have the void parameter.
                debug_assert!(!fun_ty.0.is_empty());

                let scope = if let Some(leaf) = name.get_leaf_name(ctx.subs) {
                    match leaf {
                        LeafName::SourceName(leaf) => scope.push(leaf),
                        LeafName::WellKnownComponent(leaf) => scope.push(leaf),
                        LeafName::Closure(leaf) => scope.push(leaf),
                        LeafName::UnnamedType(leaf) => scope.push(leaf),
                    }
                } else {
                    scope
                };

                // Whether the first type in the BareFunctionType is a return
                // type or parameter depends on the context in which it
                // appears.
                //
                // * Templates and functions in a type or parameter position
                // have return types, unless they are constructors, destructors,
                // or conversion operator functions.
                //
                // * Non-template functions that are not in a type or parameter
                // position do not have a return type.
                //
                // We know we are not printing a type, so we only need to check
                // whether this is a template.
                //
                // For the details, see
                // https://itanium-cxx-abi.github.io/cxx-abi/abi.html#mangle.function-type
                let scope = if let Some(template_args) = name.get_template_args(ctx.subs) {
                    let scope = scope.push(template_args);
                    if ctx.show_return_type && !name.is_ctor_dtor_conversion(ctx.subs) {
                        fun_ty.0[0].demangle(ctx, scope)?;
                        write!(ctx, " ")?;
                    }

                    scope
                } else {
                    scope
                };

                if ctx.show_params {
                    ctx.push_inner(self);
                    name.demangle(ctx, scope)?;
                    if ctx.pop_inner_if(self) {
                        self.demangle_as_inner(ctx, scope)?;
                    }
                } else {
                    name.demangle(ctx, scope)?;
                }

                Ok(())
            }
            Encoding::Data(ref name) => name.demangle(ctx, scope),
            Encoding::Special(ref name) => name.demangle(ctx, scope),
        }
    }
}

impl<'subs, W> DemangleAsInner<'subs, W> for Encoding
where
    W: 'subs + DemangleWrite,
{
    fn demangle_as_inner<'prev, 'ctx>(
        &'subs self,
        ctx: &'ctx mut DemangleContext<'subs, W>,
        scope: Option<ArgScopeStack<'prev, 'subs>>,
    ) -> fmt::Result {
        if let Encoding::Function(ref name, ref fun_ty) = *self {
            let (scope, function_args) =
                if let Some(template_args) = name.get_template_args(ctx.subs) {
                    let scope = scope.push(template_args);
                    let function_args = FunctionArgListAndReturnType::new(&fun_ty.0);
                    (scope, function_args as &dyn DemangleAsInner<W>)
                } else {
                    let function_args = FunctionArgList::new(&fun_ty.0);
                    (scope, function_args as &dyn DemangleAsInner<W>)
                };
            function_args.demangle_as_inner(ctx, scope)
        } else {
            unreachable!("we only push Encoding::Function onto the inner stack");
        }
    }
}

/// <clone-suffix> ::= [ . <clone-type-identifier> ] [ . <nonnegative number> ]*

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CloneSuffix(CloneTypeIdentifier, Vec<isize>);

impl Parse for CloneSuffix {
    fn parse<'a, 'b>(
        ctx: &'a ParseContext,
        subs: &'a mut SubstitutionTable,
        input: IndexStr<'b>,
    ) -> Result<(CloneSuffix, IndexStr<'b>)> {
        try_begin_parse!("CloneSuffix", ctx, input);

        let tail = consume(b".", input)?;
        let (identifier, mut tail) = CloneTypeIdentifier::parse(ctx, subs, tail)?;

        let mut numbers = Vec::with_capacity(1);
        while let Ok((n, t)) = consume(b".", tail).and_then(|t| parse_number(10, false, t)) {
            numbers.push(n);
            tail = t;
        }

        let clone_suffix = CloneSuffix(identifier, numbers);
        Ok((clone_suffix, tail))
    }
}

impl<'subs, W> Demangle<'subs, W> for CloneSuffix
where
    W: 'subs + DemangleWrite,
{
    fn demangle<'prev, 'ctx>(
        &'subs self,
        ctx: &'ctx mut DemangleContext<'subs, W>,
        scope: Option<ArgScopeStack<'prev, 'subs>>,
    ) -> fmt::Result {
        let ctx = try_begin_demangle!(self, ctx, scope);
        write!(ctx, " [clone")?;
        self.0.demangle(ctx, scope)?;
        for nonnegative in &self.1 {
            write!(ctx, ".{}", nonnegative)?;
        }
        write!(ctx, "]")?;
        Ok(())
    }
}

/// A global constructor or destructor.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum GlobalCtorDtor {
    /// A global constructor.
    Ctor(Box<MangledName>),
    /// A global destructor.
    Dtor(Box<MangledName>),
}

impl Parse for GlobalCtorDtor {
    fn parse<'a, 'b>(
        ctx: &'a ParseContext,
        subs: &'a mut SubstitutionTable,
        input: IndexStr<'b>,
    ) -> Result<(GlobalCtorDtor, IndexStr<'b>)> {
        try_begin_parse!("GlobalCtorDtor", ctx, input);

        let tail = match input.next_or(error::Error::UnexpectedEnd)? {
            (b'_', t) | (b'.', t) | (b'$', t) => t,
            _ => return Err(error::Error::UnexpectedText),
        };

        match tail.next_or(error::Error::UnexpectedEnd)? {
            (b'I', tail) => {
                let tail = consume(b"_", tail)?;
                let (name, tail) = MangledName::parse(ctx, subs, tail)?;
                Ok((GlobalCtorDtor::Ctor(Box::new(name)), tail))
            }
            (b'D', tail) => {
                let tail = consume(b"_", tail)?;
                let (name, tail) = MangledName::parse(ctx, subs, tail)?;
                Ok((GlobalCtorDtor::Dtor(Box::new(name)), tail))
            }
            _ => Err(error::Error::UnexpectedText),
        }
    }
}

impl<'subs, W> Demangle<'subs, W> for GlobalCtorDtor
where
    W: 'subs + DemangleWrite,
{
    fn demangle<'prev, 'ctx>(
        &'subs self,
        ctx: &'ctx mut DemangleContext<'subs, W>,
        scope: Option<ArgScopeStack<'prev, 'subs>>,
    ) -> fmt::Result {
        let ctx = try_begin_demangle!(self, ctx, scope);
        inner_barrier!(ctx);

        let saved_show_params = ctx.show_params;
        ctx.show_params = true;
        let ret = match *self {
            GlobalCtorDtor::Ctor(ref name) => {
                write!(ctx, "global constructors keyed to ")?;
                name.demangle(ctx, scope)
            }
            GlobalCtorDtor::Dtor(ref name) => {
                write!(ctx, "global destructors keyed to ")?;
                name.demangle(ctx, scope)
            }
        };
        ctx.show_params = saved_show_params;
        ret
    }
}

/// The `<name>` production.
///
/// ```text
/// <name> ::= <nested-name>
///        ::= <unscoped-name>
///        ::= <unscoped-template-name> <template-args>
///        ::= <local-name>
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Name {
    /// A nested name
    Nested(NestedName),

    /// An unscoped name.
    Unscoped(UnscopedName),

    /// An unscoped template.
    UnscopedTemplate(UnscopedTemplateNameHandle, TemplateArgs),

    /// A local name.
    Local(LocalName),
}

impl Parse for Name {
    fn parse<'a, 'b>(
        ctx: &'a ParseContext,
        subs: &'a mut SubstitutionTable,
        input: IndexStr<'b>,
    ) -> Result<(Name, IndexStr<'b>)> {
        try_begin_parse!("Name", ctx, input);

        if let Ok((name, tail)) = try_recurse!(NestedName::parse(ctx, subs, input)) {
            return Ok((Name::Nested(name), tail));
        }

        if let Ok((name, tail)) = try_recurse!(UnscopedName::parse(ctx, subs, input)) {
            if tail.peek() == Some(b'I') {
                let name = UnscopedTemplateName(name);
                let idx = subs.insert(Substitutable::UnscopedTemplateName(name));
                let handle = UnscopedTemplateNameHandle::BackReference(idx);

                let (args, tail) = TemplateArgs::parse(ctx, subs, tail)?;
                return Ok((Name::UnscopedTemplate(handle, args), tail));
            } else {
                return Ok((Name::Unscoped(name), tail));
            }
        }

        if let Ok((name, tail)) = try_recurse!(UnscopedTemplateNameHandle::parse(ctx, subs, input))
        {
            let (args, tail) = TemplateArgs::parse(ctx, subs, tail)?;
            return Ok((Name::UnscopedTemplate(name, args), tail));
        }

        let (name, tail) = LocalName::parse(ctx, subs, input)?;
        Ok((Name::Local(name), tail))
    }
}

impl<'subs, W> Demangle<'subs, W> for Name
where
    W: 'subs + DemangleWrite,
{
    fn demangle<'prev, 'ctx>(
        &'subs self,
        ctx: &'ctx mut DemangleContext<'subs, W>,
        scope: Option<ArgScopeStack<'prev, 'subs>>,
    ) -> fmt::Result {
        let ctx = try_begin_demangle!(self, ctx, scope);

        match *self {
            Name::Nested(ref nested) => nested.demangle(ctx, scope),
            Name::Unscoped(ref unscoped) => unscoped.demangle(ctx, scope),
            Name::UnscopedTemplate(ref template, ref args) => {
                template.demangle(ctx, scope.push(args))?;
                args.demangle(ctx, scope)
            }
            Name::Local(ref local) => local.demangle(ctx, scope),
        }
    }
}

impl GetTemplateArgs for Name {
    fn get_template_args<'a>(&'a self, subs: &'a SubstitutionTable) -> Option<&'a TemplateArgs> {
        match *self {
            Name::UnscopedTemplate(_, ref args) => Some(args),
            Name::Nested(ref nested) => nested.get_template_args(subs),
            Name::Local(ref local) => local.get_template_args(subs),
            Name::Unscoped(_) => None,
        }
    }
}

impl<'a> GetLeafName<'a> for Name {
    fn get_leaf_name(&'a self, subs: &'a SubstitutionTable) -> Option<LeafName<'a>> {
        match *self {
            Name::UnscopedTemplate(ref templ, _) => templ.get_leaf_name(subs),
            Name::Nested(ref nested) => nested.get_leaf_name(subs),
            Name::Unscoped(ref unscoped) => unscoped.get_leaf_name(subs),
            Name::Local(ref local) => local.get_leaf_name(subs),
        }
    }
}

impl IsCtorDtorConversion for Name {
    fn is_ctor_dtor_conversion(&self, subs: &SubstitutionTable) -> bool {
        match *self {
            Name::Unscoped(ref unscoped) => unscoped.is_ctor_dtor_conversion(subs),
            Name::Nested(ref nested) => nested.is_ctor_dtor_conversion(subs),
            Name::Local(_) | Name::UnscopedTemplate(..) => false,
        }
    }
}

/// The `<unscoped-name>` production.
///
/// ```text
/// <unscoped-name> ::= <unqualified-name>
///                 ::= St <unqualified-name>   # ::std::
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum UnscopedName {
    /// An unqualified name.
    Unqualified(UnqualifiedName),

    /// A name within the `std::` namespace.
    Std(UnqualifiedName),
}

impl Parse for UnscopedName {
    fn parse<'a, 'b>(
        ctx: &'a ParseContext,
        subs: &'a mut SubstitutionTable,
        input: IndexStr<'b>,
    ) -> Result<(UnscopedName, IndexStr<'b>)> {
        try_begin_parse!("UnscopedName", ctx, input);

        if let Ok(tail) = consume(b"St", input) {
            let (name, tail) = UnqualifiedName::parse(ctx, subs, tail)?;
            return Ok((UnscopedName::Std(name), tail));
        }

        let (name, tail) = UnqualifiedName::parse(ctx, subs, input)?;
        Ok((UnscopedName::Unqualified(name), tail))
    }
}

impl<'subs, W> Demangle<'subs, W> for UnscopedName
where
    W: 'subs + DemangleWrite,
{
    fn demangle<'prev, 'ctx>(
        &'subs self,
        ctx: &'ctx mut DemangleContext<'subs, W>,
        scope: Option<ArgScopeStack<'prev, 'subs>>,
    ) -> fmt::Result {
        let ctx = try_begin_demangle!(self, ctx, scope);

        match *self {
            UnscopedName::Unqualified(ref unqualified) => unqualified.demangle(ctx, scope),
            UnscopedName::Std(ref std) => {
                write!(ctx, "std::")?;
                std.demangle(ctx, scope)
            }
        }
    }
}

impl<'a> GetLeafName<'a> for UnscopedName {
    fn get_leaf_name(&'a self, subs: &'a SubstitutionTable) -> Option<LeafName<'a>> {
        match *self {
            UnscopedName::Unqualified(ref name) | UnscopedName::Std(ref name) => {
                name.get_leaf_name(subs)
            }
        }
    }
}

impl IsCtorDtorConversion for UnscopedName {
    fn is_ctor_dtor_conversion(&self, subs: &SubstitutionTable) -> bool {
        match *self {
            UnscopedName::Unqualified(ref name) | UnscopedName::Std(ref name) => {
                name.is_ctor_dtor_conversion(subs)
            }
        }
    }
}

/// The `<unscoped-template-name>` production.
///
/// ```text
/// <unscoped-template-name> ::= <unscoped-name>
///                          ::= <substitution>
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UnscopedTemplateName(UnscopedName);

define_handle! {
    /// A handle to an `UnscopedTemplateName`.
    pub enum UnscopedTemplateNameHandle {
        /// A handle to some `<unscoped-name>` component that isn't by itself
        /// substitutable.
        extra NonSubstitution(NonSubstitution),
    }
}

impl Parse for UnscopedTemplateNameHandle {
    fn parse<'a, 'b>(
        ctx: &'a ParseContext,
        subs: &'a mut SubstitutionTable,
        input: IndexStr<'b>,
    ) -> Result<(UnscopedTemplateNameHandle, IndexStr<'b>)> {
        try_begin_parse!("UnscopedTemplateNameHandle", ctx, input);

        if let Ok((name, tail)) = try_recurse!(UnscopedName::parse(ctx, subs, input)) {
            let name = UnscopedTemplateName(name);
            let idx = subs.insert(Substitutable::UnscopedTemplateName(name));
            let handle = UnscopedTemplateNameHandle::BackReference(idx);
            return Ok((handle, tail));
        }

        let (sub, tail) = Substitution::parse(ctx, subs, input)?;

        match sub {
            Substitution::WellKnown(component) => {
                Ok((UnscopedTemplateNameHandle::WellKnown(component), tail))
            }
            Substitution::BackReference(idx) => {
                // TODO: should this check/assert that subs[idx] is an
                // UnscopedTemplateName?
                Ok((UnscopedTemplateNameHandle::BackReference(idx), tail))
            }
        }
    }
}

impl<'subs, W> Demangle<'subs, W> for UnscopedTemplateName
where
    W: 'subs + DemangleWrite,
{
    fn demangle<'prev, 'ctx>(
        &'subs self,
        ctx: &'ctx mut DemangleContext<'subs, W>,
        scope: Option<ArgScopeStack<'prev, 'subs>>,
    ) -> fmt::Result {
        let ctx = try_begin_demangle!(self, ctx, scope);

        self.0.demangle(ctx, scope)
    }
}

impl<'a> GetLeafName<'a> for UnscopedTemplateName {
    fn get_leaf_name(&'a self, subs: &'a SubstitutionTable) -> Option<LeafName<'a>> {
        self.0.get_leaf_name(subs)
    }
}

/// The `<nested-name>` production.
///
/// ```text
/// <nested-name> ::= N [<CV-qualifiers>] [<ref-qualifier>] <prefix> <unqualified-name> E
///               ::= N [<CV-qualifiers>] [<ref-qualifier>] <template-prefix> <template-args> E
///               ::= N H <prefix> <unqualified-name> E
///               ::= N H <template-prefix> <template-args> E
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum NestedName {
    /// A nested name.
    Unqualified(
        CvQualifiers,
        Option<RefQualifier>,
        Option<PrefixHandle>,
        UnqualifiedName,
    ),

    /// A nested template name. The `<template-args>` are part of the `PrefixHandle`.
    Template(CvQualifiers, Option<RefQualifier>, PrefixHandle),

    /// A nested name with an explicit object.
    UnqualifiedExplicitObject(
        Option<PrefixHandle>,
        UnqualifiedName,
        ExplicitObjectParameter,
    ),

    /// A nested template name with an explicit object.
    TemplateExplicitObject(PrefixHandle, ExplicitObjectParameter),
}

impl Parse for NestedName {
    fn parse<'a, 'b>(
        ctx: &'a ParseContext,
        subs: &'a mut SubstitutionTable,
        input: IndexStr<'b>,
    ) -> Result<(NestedName, IndexStr<'b>)> {
        try_begin_parse!("NestedName", ctx, input);

        let tail = consume(b"N", input)?;

        let (cv_qualifiers, ref_qualifier, explicit_obj_param, tail) = match tail.peek() {
            Some(b'H') => {
                let (explicit_obj_param, tail) = ExplicitObjectParameter::parse(ctx, subs, tail)?;
                (Default::default(), None, Some(explicit_obj_param), tail)
            }
            _ => {
                let (cv_qualifiers, tail) =
                    if let Ok((q, tail)) = try_recurse!(CvQualifiers::parse(ctx, subs, tail)) {
                        (q, tail)
                    } else {
                        (Default::default(), tail)
                    };

                let (ref_qualifier, tail) =
                    if let Ok((r, tail)) = try_recurse!(RefQualifier::parse(ctx, subs, tail)) {
                        (Some(r), tail)
                    } else {
                        (None, tail)
                    };

                (cv_qualifiers, ref_qualifier, None, tail)
            }
        };

        let (prefix, tail) = PrefixHandle::parse(ctx, subs, tail)?;
        let tail = consume(b"E", tail)?;

        let substitutable = match prefix {
            PrefixHandle::BackReference(idx) => subs.get(idx),
            PrefixHandle::NonSubstitution(NonSubstitution(idx)) => subs.get_non_substitution(idx),
            PrefixHandle::WellKnown(_) => None,
        };

        match (substitutable, explicit_obj_param) {
            (Some(&Substitutable::Prefix(Prefix::Unqualified(ref name))), None) => Ok((
                NestedName::Unqualified(cv_qualifiers, ref_qualifier, None, name.clone()),
                tail,
            )),
            (Some(&Substitutable::Prefix(Prefix::Nested(ref prefix, ref name))), None) => Ok((
                NestedName::Unqualified(
                    cv_qualifiers,
                    ref_qualifier,
                    Some(prefix.clone()),
                    name.clone(),
                ),
                tail,
            )),
            (Some(&Substitutable::Prefix(Prefix::Template(..))), None) => Ok((
                NestedName::Template(cv_qualifiers, ref_qualifier, prefix),
                tail,
            )),
            (Some(&Substitutable::Prefix(Prefix::Unqualified(ref name))), Some(param)) => Ok((
                NestedName::UnqualifiedExplicitObject(None, name.clone(), param),
                tail,
            )),
            (Some(&Substitutable::Prefix(Prefix::Nested(ref prefix, ref name))), Some(param)) => {
                Ok((
                    NestedName::UnqualifiedExplicitObject(
                        Some(prefix.clone()),
                        name.clone(),
                        param,
                    ),
                    tail,
                ))
            }
            (Some(&Substitutable::Prefix(Prefix::Template(..))), Some(param)) => {
                Ok((NestedName::TemplateExplicitObject(prefix, param), tail))
            }
            _ => Err(error::Error::UnexpectedText),
        }
    }
}

impl NestedName {
    /// Get the CV-qualifiers for this name.
    pub fn cv_qualifiers(&self) -> Option<&CvQualifiers> {
        match *self {
            NestedName::Unqualified(ref q, ..) | NestedName::Template(ref q, ..) => Some(q),
            _ => None,
        }
    }

    /// Get the ref-qualifier for this name, if one exists.
    pub fn ref_qualifier(&self) -> Option<&RefQualifier> {
        match *self {
            NestedName::Unqualified(_, Some(ref r), ..)
            | NestedName::Template(_, Some(ref r), ..) => Some(r),
            _ => None,
        }
    }

    // Not public because the prefix means different things for different
    // variants, and for `::Template` it actually contains part of what
    // conceptually belongs to `<nested-name>`.
    fn prefix(&self) -> Option<&PrefixHandle> {
        match *self {
            NestedName::Unqualified(_, _, ref p, _)
            | NestedName::UnqualifiedExplicitObject(ref p, ..) => p.as_ref(),
            NestedName::Template(_, _, ref p) | NestedName::TemplateExplicitObject(ref p, _) => {
                Some(p)
            }
        }
    }

    /// Check to see if the object has an explicit named parameter.
    pub fn has_explicit_obj_param(&self) -> bool {
        match *self {
            NestedName::UnqualifiedExplicitObject(_, _, ref _e)
            | NestedName::TemplateExplicitObject(_, ref _e) => true,
            _ => false,
        }
    }
}

impl<'subs, W> Demangle<'subs, W> for NestedName
where
    W: 'subs + DemangleWrite,
{
    fn demangle<'prev, 'ctx>(
        &'subs self,
        ctx: &'ctx mut DemangleContext<'subs, W>,
        scope: Option<ArgScopeStack<'prev, 'subs>>,
    ) -> fmt::Result {
        let ctx = try_begin_demangle!(self, ctx, scope);

        match *self {
            NestedName::Unqualified(_, _, ref p, ref name)
            | NestedName::UnqualifiedExplicitObject(ref p, ref name, _) => {
                ctx.push_demangle_node(DemangleNodeType::NestedName);
                if let Some(p) = p.as_ref() {
                    p.demangle(ctx, scope)?;
                    ctx.write_str("::")?;
                }
                name.demangle(ctx, scope)?;
                ctx.pop_demangle_node();
            }
            NestedName::Template(_, _, ref p) | NestedName::TemplateExplicitObject(ref p, _) => {
                ctx.is_template_prefix_in_nested_name = true;
                p.demangle(ctx, scope)?;
                ctx.is_template_prefix_in_nested_name = false;
            }
        }

        if self.has_explicit_obj_param() {
            ctx.is_explicit_obj_param = true;
        }

        if let Some(inner) = ctx.pop_inner() {
            inner.demangle_as_inner(ctx, scope)?;
        }

        if let Some(cv_qualifiers) = self.cv_qualifiers() {
            if cv_qualifiers != &CvQualifiers::default() && ctx.show_params {
                cv_qualifiers.demangle(ctx, scope)?;
            }
        }

        if let Some(ref refs) = self.ref_qualifier() {
            ctx.ensure_space()?;
            refs.demangle(ctx, scope)?;
        }

        Ok(())
    }
}

impl GetTemplateArgs for NestedName {
    fn get_template_args<'a>(&'a self, subs: &'a SubstitutionTable) -> Option<&'a TemplateArgs> {
        match *self {
            NestedName::Template(_, _, ref prefix)
            | NestedName::TemplateExplicitObject(ref prefix, _) => prefix.get_template_args(subs),
            _ => None,
        }
    }
}

impl<'a> GetLeafName<'a> for NestedName {
    fn get_leaf_name(&'a self, subs: &'a SubstitutionTable) -> Option<LeafName<'a>> {
        match *self {
            NestedName::Unqualified(_, _, ref prefix, ref name)
            | NestedName::UnqualifiedExplicitObject(ref prefix, ref name, _) => name
                .get_leaf_name(subs)
                .or_else(|| prefix.as_ref().and_then(|p| p.get_leaf_name(subs))),
            NestedName::Template(_, _, ref prefix)
            | NestedName::TemplateExplicitObject(ref prefix, _) => prefix.get_leaf_name(subs),
        }
    }
}

impl IsCtorDtorConversion for NestedName {
    fn is_ctor_dtor_conversion(&self, subs: &SubstitutionTable) -> bool {
        self.prefix()
            .map(|p| p.is_ctor_dtor_conversion(subs))
            .unwrap_or(false)
    }
}

/// The `<prefix>` production.
///
/// ```text
/// <prefix> ::= <unqualified-name>
///          ::= <prefix> <unqualified-name>
///          ::= <template-prefix> <template-args>
///          ::= <template-param>
///          ::= <decltype>
///          ::= <prefix> <data-member-prefix>
///          ::= <substitution>
///
/// <template-prefix> ::= <template unqualified-name>
///                   ::= <prefix> <template unqualified-name>
///                   ::= <template-param>
///                   ::= <substitution>
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Prefix {
    /// An unqualified name.
    Unqualified(UnqualifiedName),

    /// Some nested name.
    Nested(PrefixHandle, UnqualifiedName),

    /// A prefix and template arguments.
    Template(PrefixHandle, TemplateArgs),

    /// A template parameter.
    TemplateParam(TemplateParam),

    /// A decltype.
    Decltype(Decltype),

    /// A prefix and data member.
    DataMember(PrefixHandle, DataMemberPrefix),
}

impl GetTemplateArgs for Prefix {
    fn get_template_args<'a>(&'a self, _: &'a SubstitutionTable) -> Option<&'a TemplateArgs> {
        match *self {
            Prefix::Template(_, ref args) => Some(args),
            Prefix::Unqualified(_)
            | Prefix::Nested(_, _)
            | Prefix::TemplateParam(_)
            | Prefix::Decltype(_)
            | Prefix::DataMember(_, _) => None,
        }
    }
}

define_handle! {
    /// A reference to a parsed `<prefix>` production.
    pub enum PrefixHandle {
        /// A handle to some `<prefix>` component that isn't by itself
        /// substitutable; instead, it's only substitutable *with* its parent
        /// component.
        extra NonSubstitution(NonSubstitution),
    }
}

impl Parse for PrefixHandle {
    fn parse<'a, 'b>(
        ctx: &'a ParseContext,
        subs: &'a mut SubstitutionTable,
        input: IndexStr<'b>,
    ) -> Result<(PrefixHandle, IndexStr<'b>)> {
        try_begin_parse!("PrefixHandle", ctx, input);

        #[inline]
        fn save(
            subs: &mut SubstitutionTable,
            prefix: Prefix,
            tail_tail: IndexStr<'_>,
        ) -> PrefixHandle {
            if let Some(b'E') = tail_tail.peek() {
                // An `E` means that we just finished parsing a `<nested-name>`
                // and this final set of prefixes isn't substitutable itself,
                // only as part of the whole `<nested-name>`. Since they are
                // effectively equivalent, it doesn't make sense to add entries
                // for both.
                let idx = subs.insert_non_substitution(Substitutable::Prefix(prefix));
                PrefixHandle::NonSubstitution(NonSubstitution(idx))
            } else {
                let idx = subs.insert(Substitutable::Prefix(prefix));
                PrefixHandle::BackReference(idx)
            }
        }

        let mut tail = input;
        let mut current = None;

        loop {
            try_begin_parse!("PrefixHandle iteration", ctx, tail);

            match tail.peek() {
                Some(b'E') | None => {
                    if let Some(handle) = current {
                        return Ok((handle, tail));
                    } else {
                        return Err(error::Error::UnexpectedEnd);
                    }
                }
                Some(b'S') => {
                    // <prefix> ::= <substitution>
                    let (sub, tail_tail) = Substitution::parse(ctx, subs, tail)?;
                    current = Some(match sub {
                        Substitution::WellKnown(component) => PrefixHandle::WellKnown(component),
                        Substitution::BackReference(idx) => {
                            // TODO: do we need to check that the idx actually points to
                            // a Prefix?
                            PrefixHandle::BackReference(idx)
                        }
                    });
                    tail = tail_tail;
                }
                Some(b'T') => {
                    // <prefix> ::= <template-param>
                    let (param, tail_tail) = TemplateParam::parse(ctx, subs, tail)?;
                    current = Some(save(subs, Prefix::TemplateParam(param), tail_tail));
                    tail = tail_tail;
                }
                Some(b'D') => {
                    // Either
                    //
                    //     <prefix> ::= <decltype>
                    //
                    // or
                    //
                    //     <prefix> ::= <unqualified-name> ::= <ctor-dtor-name>
                    if let Ok((decltype, tail_tail)) =
                        try_recurse!(Decltype::parse(ctx, subs, tail))
                    {
                        current = Some(save(subs, Prefix::Decltype(decltype), tail_tail));
                        tail = tail_tail;
                    } else {
                        let (name, tail_tail) = UnqualifiedName::parse(ctx, subs, tail)?;
                        let prefix = match current {
                            None => Prefix::Unqualified(name),
                            Some(handle) => Prefix::Nested(handle, name),
                        };
                        current = Some(save(subs, prefix, tail_tail));
                        tail = tail_tail;
                    }
                }
                Some(b'I')
                    if current.is_some() && current.as_ref().unwrap().is_template_prefix() =>
                {
                    // <prefix> ::= <template-prefix> <template-args>
                    let (args, tail_tail) = TemplateArgs::parse(ctx, subs, tail)?;
                    let prefix = Prefix::Template(current.unwrap(), args);
                    current = Some(save(subs, prefix, tail_tail));
                    tail = tail_tail;
                }
                Some(c) if current.is_some() && UnqualifiedName::starts_with(c, &tail) => {
                    // Either
                    //
                    //     <prefix> ::= <unqualified-name>
                    //
                    // or
                    //
                    //     <prefix> ::= <data-member-prefix> ::= <prefix> <source-name> M
                    debug_assert!(UnqualifiedName::starts_with(c, &tail));

                    let (name, tail_tail) = UnqualifiedName::parse(ctx, subs, tail)?;
                    if tail_tail.peek() == Some(b'M') {
                        // XXXkhuey This seems to be a legacy thing that's dropped from the standard.
                        // Behave the way we used to.
                        let UnqualifiedName::Source(name, _) = name else {
                            return Err(error::Error::UnexpectedText);
                        };
                        let prefix = Prefix::DataMember(current.unwrap(), DataMemberPrefix(name));
                        current = Some(save(subs, prefix, tail_tail));
                        tail = consume(b"M", tail_tail).unwrap();
                    } else {
                        let prefix = match current {
                            None => Prefix::Unqualified(name),
                            Some(handle) => Prefix::Nested(handle, name),
                        };
                        current = Some(save(subs, prefix, tail_tail));
                        tail = tail_tail;
                    }
                }
                Some(c) if UnqualifiedName::starts_with(c, &tail) => {
                    // <prefix> ::= <unqualified-name>
                    let (name, tail_tail) = UnqualifiedName::parse(ctx, subs, tail)?;
                    let prefix = match current {
                        None => Prefix::Unqualified(name),
                        Some(handle) => Prefix::Nested(handle, name),
                    };
                    current = Some(save(subs, prefix, tail_tail));
                    tail = tail_tail;
                }
                Some(_) => {
                    if let Some(handle) = current {
                        return Ok((handle, tail));
                    } else if tail.is_empty() {
                        return Err(error::Error::UnexpectedEnd);
                    } else {
                        return Err(error::Error::UnexpectedText);
                    }
                }
            }
        }
    }
}

impl<'a> GetLeafName<'a> for Prefix {
    fn get_leaf_name(&'a self, subs: &'a SubstitutionTable) -> Option<LeafName<'a>> {
        match *self {
            Prefix::Nested(ref prefix, ref name) => name
                .get_leaf_name(subs)
                .or_else(|| prefix.get_leaf_name(subs)),
            Prefix::Unqualified(ref name) => name.get_leaf_name(subs),
            Prefix::Template(ref prefix, _) => prefix.get_leaf_name(subs),
            Prefix::DataMember(_, ref name) => name.get_leaf_name(subs),
            Prefix::TemplateParam(_) | Prefix::Decltype(_) => None,
        }
    }
}

impl GetTemplateArgs for PrefixHandle {
    // XXX: Not an impl GetTemplateArgs for PrefixHandle because the 'me
    // reference to self may not live long enough.
    fn get_template_args<'a>(&'a self, subs: &'a SubstitutionTable) -> Option<&'a TemplateArgs> {
        match *self {
            PrefixHandle::BackReference(idx) => {
                if let Some(&Substitutable::Prefix(ref p)) = subs.get(idx) {
                    p.get_template_args(subs)
                } else {
                    None
                }
            }
            PrefixHandle::NonSubstitution(NonSubstitution(idx)) => {
                if let Some(&Substitutable::Prefix(ref p)) = subs.get_non_substitution(idx) {
                    p.get_template_args(subs)
                } else {
                    None
                }
            }
            _ => None,
        }
    }
}

impl<'subs, W> Demangle<'subs, W> for Prefix
where
    W: 'subs + DemangleWrite,
{
    fn demangle<'prev, 'ctx>(
        &'subs self,
        ctx: &'ctx mut DemangleContext<'subs, W>,
        scope: Option<ArgScopeStack<'prev, 'subs>>,
    ) -> fmt::Result {
        let ctx = try_begin_demangle!(self, ctx, scope);
        if ctx.is_template_prefix {
            ctx.push_demangle_node(DemangleNodeType::TemplatePrefix);
            ctx.is_template_prefix = false;
        } else if ctx.is_template_prefix_in_nested_name {
            ctx.push_demangle_node(DemangleNodeType::NestedName);
            ctx.is_template_prefix_in_nested_name = false;
        } else {
            ctx.push_demangle_node(DemangleNodeType::Prefix);
        }

        let ret = match *self {
            Prefix::Unqualified(ref unqualified) => unqualified.demangle(ctx, scope),
            Prefix::Nested(ref prefix, ref unqualified) => {
                prefix.demangle(ctx, scope)?;
                write!(ctx, "::")?;
                unqualified.demangle(ctx, scope)
            }
            Prefix::Template(ref prefix, ref args) => {
                ctx.is_template_prefix = true;
                prefix.demangle(ctx, scope)?;
                ctx.is_template_prefix = false;
                args.demangle(ctx, scope)
            }
            Prefix::TemplateParam(ref param) => param.demangle(ctx, scope),
            Prefix::Decltype(ref dt) => dt.demangle(ctx, scope),
            Prefix::DataMember(ref prefix, ref member) => {
                prefix.demangle(ctx, scope)?;
                write!(ctx, "::")?;
                member.demangle(ctx, scope)
            }
        };
        ctx.pop_demangle_node();
        ret
    }
}

impl IsCtorDtorConversion for Prefix {
    fn is_ctor_dtor_conversion(&self, subs: &SubstitutionTable) -> bool {
        match *self {
            Prefix::Unqualified(ref unqualified) | Prefix::Nested(_, ref unqualified) => {
                unqualified.is_ctor_dtor_conversion(subs)
            }
            Prefix::Template(ref prefix, _) => prefix.is_ctor_dtor_conversion(subs),
            _ => false,
        }
    }
}

impl IsCtorDtorConversion for PrefixHandle {
    fn is_ctor_dtor_conversion(&self, subs: &SubstitutionTable) -> bool {
        match *self {
            PrefixHandle::BackReference(idx) => {
                if let Some(sub) = subs.get(idx) {
                    sub.is_ctor_dtor_conversion(subs)
                } else {
                    false
                }
            }
            PrefixHandle::NonSubstitution(NonSubstitution(idx)) => {
                if let Some(sub) = subs.get_non_substitution(idx) {
                    sub.is_ctor_dtor_conversion(subs)
                } else {
                    false
                }
            }
            PrefixHandle::WellKnown(_) => false,
        }
    }
}

impl PrefixHandle {
    // Is this <prefix> also a valid <template-prefix> production? Not to be
    // confused with the `GetTemplateArgs` trait.
    fn is_template_prefix(&self) -> bool {
        match *self {
            PrefixHandle::BackReference(_) | PrefixHandle::WellKnown(_) => true,
            PrefixHandle::NonSubstitution(_) => false,
        }
    }
}

/// The `<unqualified-name>` production.
///
/// ```text
/// <unqualified-name> ::= <operator-name> [<abi-tags>]
///                    ::= <ctor-dtor-name> [<abi-tags>]
///                    ::= <source-name> [<abi-tags>]
///                    ::= <local-source-name> [<abi-tags>]
///                    ::= <unnamed-type-name> [<abi-tags>]
///                    ::= <closure-type-name> [<abi-tags>]
///
/// # I think this is from an older version of the standard. It isn't in the
/// # current version, but all the other demanglers support it, so we will too.
/// <local-source-name> ::= L <source-name> [<discriminator>]
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum UnqualifiedName {
    /// An operator name.
    Operator(OperatorName, AbiTags),
    /// A constructor or destructor name.
    CtorDtor(CtorDtorName, AbiTags),
    /// A source name.
    Source(SourceName, AbiTags),
    /// A local source name.
    LocalSourceName(SourceName, Option<Discriminator>, AbiTags),
    /// A generated name for an unnamed type.
    UnnamedType(UnnamedTypeName, AbiTags),
    /// A closure type name
    ClosureType(ClosureTypeName, AbiTags),
}

impl Parse for UnqualifiedName {
    fn parse<'a, 'b>(
        ctx: &'a ParseContext,
        subs: &'a mut SubstitutionTable,
        input: IndexStr<'b>,
    ) -> Result<(UnqualifiedName, IndexStr<'b>)> {
        try_begin_parse!("UnqualifiedName", ctx, input);

        if let Ok((op, tail)) = try_recurse!(OperatorName::parse(ctx, subs, input)) {
            let (abi_tags, tail) = AbiTags::parse(ctx, subs, tail)?;
            return Ok((UnqualifiedName::Operator(op, abi_tags), tail));
        }

        if let Ok((ctor_dtor, tail)) = try_recurse!(CtorDtorName::parse(ctx, subs, input)) {
            let (abi_tags, tail) = AbiTags::parse(ctx, subs, tail)?;
            return Ok((UnqualifiedName::CtorDtor(ctor_dtor, abi_tags), tail));
        }

        if let Ok(tail) = consume(b"L", input) {
            let (name, tail) = SourceName::parse(ctx, subs, tail)?;
            let (discr, tail) =
                if let Ok((d, t)) = try_recurse!(Discriminator::parse(ctx, subs, tail)) {
                    (Some(d), t)
                } else {
                    (None, tail)
                };
            let (abi_tags, tail) = AbiTags::parse(ctx, subs, tail)?;
            return Ok((
                UnqualifiedName::LocalSourceName(name, discr, abi_tags),
                tail,
            ));
        }

        if let Ok((source, tail)) = try_recurse!(SourceName::parse(ctx, subs, input)) {
            let (abi_tags, tail) = AbiTags::parse(ctx, subs, tail)?;
            return Ok((UnqualifiedName::Source(source, abi_tags), tail));
        }

        if let Ok((closure, tail)) = try_recurse!(ClosureTypeName::parse(ctx, subs, input)) {
            let (abi_tags, tail) = AbiTags::parse(ctx, subs, tail)?;
            return Ok((UnqualifiedName::ClosureType(closure, abi_tags), tail));
        }

        let (unnamed, tail) = UnnamedTypeName::parse(ctx, subs, input)?;
        let (abi_tags, tail) = AbiTags::parse(ctx, subs, tail)?;
        Ok((UnqualifiedName::UnnamedType(unnamed, abi_tags), tail))
    }
}

impl<'subs, W> Demangle<'subs, W> for UnqualifiedName
where
    W: 'subs + DemangleWrite,
{
    fn demangle<'prev, 'ctx>(
        &'subs self,
        ctx: &'ctx mut DemangleContext<'subs, W>,
        scope: Option<ArgScopeStack<'prev, 'subs>>,
    ) -> fmt::Result {
        let ctx = try_begin_demangle!(self, ctx, scope);

        ctx.push_demangle_node(DemangleNodeType::UnqualifiedName);
        let ret = match *self {
            UnqualifiedName::Operator(ref op_name, ref abi_tags) => {
                write!(ctx, "operator")?;
                op_name.demangle(ctx, scope)?;
                abi_tags.demangle(ctx, scope)
            }
            UnqualifiedName::CtorDtor(ref ctor_dtor, ref abi_tags) => {
                ctor_dtor.demangle(ctx, scope)?;
                abi_tags.demangle(ctx, scope)
            }
            UnqualifiedName::Source(ref name, ref abi_tags)
            | UnqualifiedName::LocalSourceName(ref name, _, ref abi_tags) => {
                name.demangle(ctx, scope)?;
                abi_tags.demangle(ctx, scope)
            }
            UnqualifiedName::UnnamedType(ref unnamed, ref abi_tags) => {
                unnamed.demangle(ctx, scope)?;
                abi_tags.demangle(ctx, scope)
            }
            UnqualifiedName::ClosureType(ref closure, ref abi_tags) => {
                closure.demangle(ctx, scope)?;
                abi_tags.demangle(ctx, scope)
            }
        };
        ctx.pop_demangle_node();
        ret
    }
}

impl<'a> GetLeafName<'a> for UnqualifiedName {
    fn get_leaf_name(&'a self, subs: &'a SubstitutionTable) -> Option<LeafName<'a>> {
        match *self {
            UnqualifiedName::Operator(..) | UnqualifiedName::CtorDtor(..) => None,
            UnqualifiedName::UnnamedType(ref name, _) => Some(LeafName::UnnamedType(name)),
            UnqualifiedName::ClosureType(ref closure, _) => closure.get_leaf_name(subs),
            UnqualifiedName::Source(ref name, _)
            | UnqualifiedName::LocalSourceName(ref name, ..) => Some(LeafName::SourceName(name)),
        }
    }
}

impl IsCtorDtorConversion for UnqualifiedName {
    fn is_ctor_dtor_conversion(&self, _: &SubstitutionTable) -> bool {
        match *self {
            UnqualifiedName::CtorDtor(..)
            | UnqualifiedName::Operator(OperatorName::Conversion(_), _) => true,
            UnqualifiedName::Operator(..)
            | UnqualifiedName::Source(..)
            | UnqualifiedName::LocalSourceName(..)
            | UnqualifiedName::UnnamedType(..)
            | UnqualifiedName::ClosureType(..) => false,
        }
    }
}

impl UnqualifiedName {
    #[inline]
    fn starts_with(byte: u8, input: &IndexStr) -> bool {
        byte == b'L'
            || OperatorName::starts_with(byte)
            || CtorDtorName::starts_with(byte)
            || SourceName::starts_with(byte)
            || UnnamedTypeName::starts_with(byte)
            || ClosureTypeName::starts_with(byte, input)
    }
}

/// The `<source-name>` non-terminal.
///
/// ```text
/// <source-name> ::= <positive length number> <identifier>
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SourceName(Identifier);

impl Parse for SourceName {
    fn parse<'a, 'b>(
        ctx: &'a ParseContext,
        subs: &'a mut SubstitutionTable,
        input: IndexStr<'b>,
    ) -> Result<(SourceName, IndexStr<'b>)> {
        try_begin_parse!("SourceName", ctx, input);

        let (source_name_len, input) = parse_number(10, false, input)?;
        debug_assert!(source_name_len >= 0);
        if source_name_len == 0 {
            return Err(error::Error::UnexpectedText);
        }

        let (head, tail) = match input.try_split_at(source_name_len as _) {
            Some((head, tail)) => (head, tail),
            None => return Err(error::Error::UnexpectedEnd),
        };

        let (identifier, empty) = Identifier::parse(ctx, subs, head)?;
        if !empty.is_empty() {
            return Err(error::Error::UnexpectedText);
        }

        let source_name = SourceName(identifier);
        Ok((source_name, tail))
    }
}

impl<'subs> ArgScope<'subs, 'subs> for SourceName {
    fn leaf_name(&'subs self) -> Result<LeafName<'subs>> {
        Ok(LeafName::SourceName(self))
    }

    fn get_template_arg(
        &'subs self,
        _: usize,
    ) -> Result<(&'subs TemplateArg, &'subs TemplateArgs)> {
        Err(error::Error::BadTemplateArgReference)
    }

    fn get_function_arg(&'subs self, _: usize) -> Result<&'subs Type> {
        Err(error::Error::BadFunctionArgReference)
    }
}

impl SourceName {
    #[inline]
    fn starts_with(byte: u8) -> bool {
        byte == b'0' || (b'0' <= byte && byte <= b'9')
    }
}

impl<'subs, W> Demangle<'subs, W> for SourceName
where
    W: 'subs + DemangleWrite,
{
    #[inline]
    fn demangle<'prev, 'ctx>(
        &'subs self,
        ctx: &'ctx mut DemangleContext<'subs, W>,
        scope: Option<ArgScopeStack<'prev, 'subs>>,
    ) -> fmt::Result {
        let ctx = try_begin_demangle!(self, ctx, scope);

        self.0.demangle(ctx, scope)
    }
}

/// The `<abi-tags>` non-terminal.
///
/// ```text
/// <abi-tags> ::= <abi-tag> [<abi-tags>]
/// ```
///
/// To make things easier on ourselves, despite the fact that the `<abi-tags>`
/// production requires at least one tag, we'll allow a zero-length vector
/// here instead of having to use Option<AbiTags> in everything that accepts
/// an AbiTags.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct AbiTags(Vec<AbiTag>);

impl Parse for AbiTags {
    fn parse<'a, 'b>(
        ctx: &'a ParseContext,
        subs: &'a mut SubstitutionTable,
        input: IndexStr<'b>,
    ) -> Result<(AbiTags, IndexStr<'b>)> {
        try_begin_parse!("AbiTags", ctx, input);

        let (tags, tail) = zero_or_more::<AbiTag>(ctx, subs, input)?;
        Ok((AbiTags(tags), tail))
    }
}

impl<'subs, W> Demangle<'subs, W> for AbiTags
where
    W: 'subs + DemangleWrite,
{
    fn demangle<'prev, 'ctx>(
        &'subs self,
        ctx: &'ctx mut DemangleContext<'subs, W>,
        scope: Option<ArgScopeStack<'prev, 'subs>>,
    ) -> fmt::Result {
        let ctx = try_begin_demangle!(self, ctx, scope);

        for tag in &self.0 {
            tag.demangle(ctx, scope)?;
        }
        Ok(())
    }
}

/// The `<abi-tag>` non-terminal.
///
/// ```text
/// <abi-tag> ::= B <source-name>
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AbiTag(SourceName);

impl Parse for AbiTag {
    fn parse<'a, 'b>(
        ctx: &'a ParseContext,
        subs: &'a mut SubstitutionTable,
        input: IndexStr<'b>,
    ) -> Result<(AbiTag, IndexStr<'b>)> {
        try_begin_parse!("AbiTag", ctx, input);

        let tail = consume(b"B", input)?;
        let (source_name, tail) = SourceName::parse(ctx, subs, tail)?;
        Ok((AbiTag(source_name), tail))
    }
}

impl<'subs, W> Demangle<'subs, W> for AbiTag
where
    W: 'subs + DemangleWrite,
{
    fn demangle<'prev, 'ctx>(
        &'subs self,
        ctx: &'ctx mut DemangleContext<'subs, W>,
        scope: Option<ArgScopeStack<'prev, 'subs>>,
    ) -> fmt::Result {
        let ctx = try_begin_demangle!(self, ctx, scope);

        write!(ctx, "[abi:")?;
        self.0.demangle(ctx, scope)?;
        write!(ctx, "]")
    }
}

/// The `<identifier>` pseudo-terminal.
///
/// ```text
/// <identifier> ::= <unqualified source code identifier>
/// ```
///
/// > `<identifier>` is a pseudo-terminal representing the characters in the
/// > unqualified identifier for the entity in the source code. This ABI does not
/// > yet specify a mangling for identifiers containing characters outside of
/// > `_A-Za-z0-9.`.
///
/// Mangled symbols' identifiers also have `$` characters in the wild.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Identifier {
    start: usize,
    end: usize,
}

impl Parse for Identifier {
    fn parse<'a, 'b>(
        ctx: &'a ParseContext,
        _subs: &'a mut SubstitutionTable,
        input: IndexStr<'b>,
    ) -> Result<(Identifier, IndexStr<'b>)> {
        try_begin_parse!("Identifier", ctx, input);

        if input.is_empty() {
            return Err(error::Error::UnexpectedEnd);
        }

        let end = input
            .as_ref()
            .iter()
            .map(|&c| c as char)
            .take_while(|&c| c == '$' || c == '_' || c == '.' || c.is_digit(36))
            .count();

        if end == 0 {
            return Err(error::Error::UnexpectedText);
        }

        let tail = input.range_from(end..);

        let identifier = Identifier {
            start: input.index(),
            end: tail.index(),
        };

        Ok((identifier, tail))
    }
}

impl<'subs, W> Demangle<'subs, W> for Identifier
where
    W: 'subs + DemangleWrite,
{
    #[inline]
    fn demangle<'prev, 'ctx>(
        &'subs self,
        ctx: &'ctx mut DemangleContext<'subs, W>,
        scope: Option<ArgScopeStack<'prev, 'subs>>,
    ) -> fmt::Result {
        let ctx = try_begin_demangle!(self, ctx, scope);

        let ident = &ctx.input[self.start..self.end];

        // Handle GCC's anonymous namespace mangling.
        let anon_namespace_prefix = b"_GLOBAL_";
        if ident.starts_with(anon_namespace_prefix)
            && ident.len() >= anon_namespace_prefix.len() + 2
        {
            let first = ident[anon_namespace_prefix.len()];
            let second = ident[anon_namespace_prefix.len() + 1];

            match (first, second) {
                (b'.', b'N') | (b'_', b'N') | (b'$', b'N') => {
                    write!(ctx, "(anonymous namespace)")?;
                    return Ok(());
                }
                _ => {
                    // Fall through.
                }
            }
        }

        let source_name = String::from_utf8_lossy(ident);
        ctx.set_source_name(self.start, self.end);
        write!(ctx, "{}", source_name)?;
        Ok(())
    }
}

/// The `<clone-type-identifier>` pseudo-terminal.
///
/// ```text
/// <clone-type-identifier> ::= <unqualified source code identifier>
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CloneTypeIdentifier {
    start: usize,
    end: usize,
}

impl Parse for CloneTypeIdentifier {
    fn parse<'a, 'b>(
        ctx: &'a ParseContext,
        _subs: &'a mut SubstitutionTable,
        input: IndexStr<'b>,
    ) -> Result<(CloneTypeIdentifier, IndexStr<'b>)> {
        try_begin_parse!("CloneTypeIdentifier", ctx, input);

        if input.is_empty() {
            return Err(error::Error::UnexpectedEnd);
        }

        let end = input
            .as_ref()
            .iter()
            .map(|&c| c as char)
            .take_while(|&c| c == '$' || c == '_' || c.is_digit(36))
            .count();

        if end == 0 {
            return Err(error::Error::UnexpectedText);
        }

        let tail = input.range_from(end..);

        let identifier = CloneTypeIdentifier {
            start: input.index(),
            end: tail.index(),
        };

        Ok((identifier, tail))
    }
}

impl<'subs, W> Demangle<'subs, W> for CloneTypeIdentifier
where
    W: 'subs + DemangleWrite,
{
    #[inline]
    fn demangle<'prev, 'ctx>(
        &'subs self,
        ctx: &'ctx mut DemangleContext<'subs, W>,
        scope: Option<ArgScopeStack<'prev, 'subs>>,
    ) -> fmt::Result {
        let ctx = try_begin_demangle!(self, ctx, scope);

        let ident = &ctx.input[self.start..self.end];

        let source_name = String::from_utf8_lossy(ident);
        ctx.set_source_name(self.start, self.end);
        write!(ctx, " .{}", source_name)?;
        Ok(())
    }
}

/// The `<number>` production.
///
/// ```text
/// <number> ::= [n] <non-negative decimal integer>
/// ```
type Number = isize;

impl Parse for Number {
    fn parse<'a, 'b>(
        ctx: &'a ParseContext,
        _subs: &'a mut SubstitutionTable,
        input: IndexStr<'b>,
    ) -> Result<(isize, IndexStr<'b>)> {
        try_begin_parse!("Number", ctx, input);
        parse_number(10, true, input)
    }
}

/// A <seq-id> production encoding a base-36 positive number.
///
/// ```text
/// <seq-id> ::= <0-9A-Z>+
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SeqId(usize);

impl Parse for SeqId {
    fn parse<'a, 'b>(
        ctx: &'a ParseContext,
        _subs: &'a mut SubstitutionTable,
        input: IndexStr<'b>,
    ) -> Result<(SeqId, IndexStr<'b>)> {
        try_begin_parse!("SeqId", ctx, input);

        parse_number(36, false, input).map(|(num, tail)| (SeqId(num as _), tail))
    }
}

/// The `<operator-name>` production.
///
/// ```text
/// <operator-name> ::= <simple-operator-name>
///                 ::= cv <type>               # (cast)
///                 ::= li <source-name>        # operator ""
///                 ::= v <digit> <source-name> # vendor extended operator
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum OperatorName {
    /// A simple operator name.
    Simple(SimpleOperatorName),

    /// A type cast.
    Cast(TypeHandle),

    /// A type conversion.
    Conversion(TypeHandle),

    /// Operator literal, ie `operator ""`.
    Literal(SourceName),

    /// A non-standard, vendor extension operator.
    VendorExtension(u8, SourceName),
}

impl OperatorName {
    fn starts_with(byte: u8) -> bool {
        byte == b'c' || byte == b'l' || byte == b'v' || SimpleOperatorName::starts_with(byte)
    }

    fn arity(&self) -> u8 {
        match self {
            &OperatorName::Cast(_) | &OperatorName::Conversion(_) | &OperatorName::Literal(_) => 1,
            &OperatorName::Simple(ref s) => s.arity(),
            &OperatorName::VendorExtension(arity, _) => arity,
        }
    }

    fn parse_from_expr<'a, 'b>(
        ctx: &'a ParseContext,
        subs: &'a mut SubstitutionTable,
        input: IndexStr<'b>,
    ) -> Result<(Expression, IndexStr<'b>)> {
        let (operator, tail) = OperatorName::parse_internal(ctx, subs, input, true)?;

        let arity = operator.arity();
        if arity == 1 {
            let (first, tail) = Expression::parse(ctx, subs, tail)?;
            let expr = Expression::Unary(operator, Box::new(first));
            Ok((expr, tail))
        } else if arity == 2 {
            let (first, tail) = Expression::parse(ctx, subs, tail)?;
            let (second, tail) = Expression::parse(ctx, subs, tail)?;
            let expr = Expression::Binary(operator, Box::new(first), Box::new(second));
            Ok((expr, tail))
        } else if arity == 3 {
            let (first, tail) = Expression::parse(ctx, subs, tail)?;
            let (second, tail) = Expression::parse(ctx, subs, tail)?;
            let (third, tail) = Expression::parse(ctx, subs, tail)?;
            let expr =
                Expression::Ternary(operator, Box::new(first), Box::new(second), Box::new(third));
            Ok((expr, tail))
        } else {
            Err(error::Error::UnexpectedText)
        }
    }

    fn parse_internal<'a, 'b>(
        ctx: &'a ParseContext,
        subs: &'a mut SubstitutionTable,
        input: IndexStr<'b>,
        from_expr: bool,
    ) -> Result<(OperatorName, IndexStr<'b>)> {
        try_begin_parse!("OperatorName", ctx, input);

        if let Ok((simple, tail)) = try_recurse!(SimpleOperatorName::parse(ctx, subs, input)) {
            return Ok((OperatorName::Simple(simple), tail));
        }

        if let Ok(tail) = consume(b"cv", input) {
            // If we came through the expression path, we're a cast. If not,
            // we're a conversion.
            let previously_in_conversion = ctx.set_in_conversion(!from_expr);
            let parse_result = TypeHandle::parse(ctx, subs, tail);
            ctx.set_in_conversion(previously_in_conversion);
            let (ty, tail) = parse_result?;
            if from_expr {
                return Ok((OperatorName::Cast(ty), tail));
            } else {
                return Ok((OperatorName::Conversion(ty), tail));
            }
        }

        if let Ok(tail) = consume(b"li", input) {
            let (name, tail) = SourceName::parse(ctx, subs, tail)?;
            return Ok((OperatorName::Literal(name), tail));
        }

        let tail = consume(b"v", input)?;
        let (arity, tail) = match tail.peek() {
            Some(c) if b'0' <= c && c <= b'9' => (c - b'0', tail.range_from(1..)),
            None => return Err(error::Error::UnexpectedEnd),
            _ => return Err(error::Error::UnexpectedText),
        };
        let (name, tail) = SourceName::parse(ctx, subs, tail)?;
        Ok((OperatorName::VendorExtension(arity, name), tail))
    }
}

impl Parse for OperatorName {
    fn parse<'a, 'b>(
        ctx: &'a ParseContext,
        subs: &'a mut SubstitutionTable,
        input: IndexStr<'b>,
    ) -> Result<(OperatorName, IndexStr<'b>)> {
        OperatorName::parse_internal(ctx, subs, input, false)
    }
}

impl<'subs, W> Demangle<'subs, W> for OperatorName
where
    W: 'subs + DemangleWrite,
{
    fn demangle<'prev, 'ctx>(
        &'subs self,
        ctx: &'ctx mut DemangleContext<'subs, W>,
        scope: Option<ArgScopeStack<'prev, 'subs>>,
    ) -> fmt::Result {
        let ctx = try_begin_demangle!(self, ctx, scope);

        match *self {
            OperatorName::Simple(ref simple) => {
                match *simple {
                    SimpleOperatorName::New
                    | SimpleOperatorName::NewArray
                    | SimpleOperatorName::Delete
                    | SimpleOperatorName::DeleteArray => {
                        ctx.ensure_space()?;
                    }
                    _ => {}
                }
                simple.demangle(ctx, scope)
            }
            OperatorName::Cast(ref ty) | OperatorName::Conversion(ref ty) => {
                ctx.ensure_space()?;

                // Cast operators can refer to template arguments before they
                // actually appear in the AST, so we go traverse down the tree
                // and fetch them if they exist.
                let scope = ty
                    .get_template_args(ctx.subs)
                    .map_or(scope, |args| scope.push(args));

                ty.demangle(ctx, scope)?;
                Ok(())
            }
            OperatorName::Literal(ref name) => {
                name.demangle(ctx, scope)?;
                write!(ctx, "::operator \"\"")?;
                Ok(())
            }
            OperatorName::VendorExtension(arity, ref name) => {
                // TODO: no idea how this should be demangled...
                name.demangle(ctx, scope)?;
                write!(ctx, "::operator {}", arity)?;
                Ok(())
            }
        }
    }
}

define_vocabulary! {
    /// The `<simple-operator-name>` production.
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub enum SimpleOperatorName {
        New              (b"nw",  "new",      3),
        NewArray         (b"na",  "new[]",    3),
        Delete           (b"dl",  "delete",   1),
        DeleteArray      (b"da",  "delete[]", 1),
        UnaryPlus        (b"ps",  "+",        1),
        Neg              (b"ng",  "-",        1),
        AddressOf        (b"ad",  "&",        1),
        Deref            (b"de",  "*",        1),
        BitNot           (b"co",  "~",        1),
        Add              (b"pl",  "+",        2),
        Sub              (b"mi",  "-",        2),
        Mul              (b"ml",  "*",        2),
        Div              (b"dv",  "/",        2),
        Rem              (b"rm",  "%",        2),
        BitAnd           (b"an",  "&",        2),
        BitOr            (b"or",  "|",        2),
        BitXor           (b"eo",  "^",        2),
        Assign           (b"aS",  "=",        2),
        AddAssign        (b"pL",  "+=",       2),
        SubAssign        (b"mI",  "-=",       2),
        MulAssign        (b"mL",  "*=",       2),
        DivAssign        (b"dV",  "/=",       2),
        RemAssign        (b"rM",  "%=",       2),
        BitAndAssign     (b"aN",  "&=",       2),
        BitOrAssign      (b"oR",  "|=",       2),
        BitXorAssign     (b"eO",  "^=",       2),
        Shl              (b"ls",  "<<",       2),
        Shr              (b"rs",  ">>",       2),
        ShlAssign        (b"lS",  "<<=",      2),
        ShrAssign        (b"rS",  ">>=",      2),
        Eq               (b"eq",  "==",       2),
        Ne               (b"ne",  "!=",       2),
        Less             (b"lt",  "<",        2),
        Greater          (b"gt",  ">",        2),
        LessEq           (b"le",  "<=",       2),
        GreaterEq        (b"ge",  ">=",       2),
        Not              (b"nt",  "!",        1),
        LogicalAnd       (b"aa",  "&&",       2),
        LogicalOr        (b"oo",  "||",       2),
        PostInc          (b"pp",  "++",       1), // (postfix in <expression> context)
        PostDec          (b"mm",  "--",       1), // (postfix in <expression> context)
        Comma            (b"cm",  ",",        2),
        DerefMemberPtr   (b"pm",  "->*",      2),
        DerefMember      (b"pt",  "->",       2),
        Call             (b"cl",  "()",       2),
        Index            (b"ix",  "[]",       2),
        Question         (b"qu",  "?:",       3),
        Spaceship        (b"ss",  "<=>",      2)
    }

    impl SimpleOperatorName {
        // Automatically implemented by define_vocabulary!
        fn arity(&self) -> u8;
    }
}

/// The `<call-offset>` production.
///
/// ```text
/// <call-offset> ::= h <nv-offset> _
///               ::= v <v-offset> _
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CallOffset {
    /// A non-virtual offset.
    NonVirtual(NvOffset),
    /// A virtual offset.
    Virtual(VOffset),
}

impl Parse for CallOffset {
    fn parse<'a, 'b>(
        ctx: &'a ParseContext,
        subs: &'a mut SubstitutionTable,
        input: IndexStr<'b>,
    ) -> Result<(CallOffset, IndexStr<'b>)> {
        try_begin_parse!("CallOffset", ctx, input);

        if input.is_empty() {
            return Err(error::Error::UnexpectedEnd);
        }

        if let Ok(tail) = consume(b"h", input) {
            let (offset, tail) = NvOffset::parse(ctx, subs, tail)?;
            let tail = consume(b"_", tail)?;
            return Ok((CallOffset::NonVirtual(offset), tail));
        }

        if let Ok(tail) = consume(b"v", input) {
            let (offset, tail) = VOffset::parse(ctx, subs, tail)?;
            let tail = consume(b"_", tail)?;
            return Ok((CallOffset::Virtual(offset), tail));
        }

        Err(error::Error::UnexpectedText)
    }
}

impl<'subs, W> Demangle<'subs, W> for CallOffset
where
    W: 'subs + DemangleWrite,
{
    fn demangle<'prev, 'ctx>(
        &'subs self,
        ctx: &'ctx mut DemangleContext<'subs, W>,
        scope: Option<ArgScopeStack<'prev, 'subs>>,
    ) -> fmt::Result {
        let ctx = try_begin_demangle!(self, ctx, scope);

        match *self {
            CallOffset::NonVirtual(NvOffset(offset)) => {
                write!(ctx, "{{offset({})}}", offset)?;
            }
            CallOffset::Virtual(VOffset(vbase, vcall)) => {
                write!(ctx, "{{virtual offset({}, {})}}", vbase, vcall)?;
            }
        }
        Ok(())
    }
}

/// A non-virtual offset, as described by the <nv-offset> production.
///
/// ```text
/// <nv-offset> ::= <offset number>
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NvOffset(isize);

impl Parse for NvOffset {
    fn parse<'a, 'b>(
        ctx: &'a ParseContext,
        subs: &'a mut SubstitutionTable,
        input: IndexStr<'b>,
    ) -> Result<(NvOffset, IndexStr<'b>)> {
        try_begin_parse!("NvOffset", ctx, input);

        Number::parse(ctx, subs, input).map(|(num, tail)| (NvOffset(num), tail))
    }
}

/// A virtual offset, as described by the <v-offset> production.
///
/// ```text
/// <v-offset> ::= <offset number> _ <virtual offset number>
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct VOffset(isize, isize);

impl Parse for VOffset {
    fn parse<'a, 'b>(
        ctx: &'a ParseContext,
        subs: &'a mut SubstitutionTable,
        input: IndexStr<'b>,
    ) -> Result<(VOffset, IndexStr<'b>)> {
        try_begin_parse!("VOffset", ctx, input);

        let (offset, tail) = Number::parse(ctx, subs, input)?;
        let tail = consume(b"_", tail)?;
        let (virtual_offset, tail) = Number::parse(ctx, subs, tail)?;
        Ok((VOffset(offset, virtual_offset), tail))
    }
}

/// The `<ctor-dtor-name>` production.
///
/// ```text
/// <ctor-dtor-name> ::= C1  # complete object constructor
///                  ::= C2  # base object constructor
///                  ::= C3  # complete object allocating constructor
///                  ::= D0  # deleting destructor
///                  ::= D1  # complete object destructor
///                  ::= D2  # base object destructor
/// ```
///
/// GCC also emits a C4 constructor under some conditions when building
/// an optimized binary. GCC's source says:
///
/// ```
/// /* This is the old-style "[unified]" constructor.
///    In some cases, we may emit this function and call
///    it from the clones in order to share code and save space.  */
/// ```
///
/// Based on the GCC source we'll call this the "maybe in-charge constructor".
/// Similarly, there is a D4 destructor, the "maybe in-charge destructor".
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CtorDtorName {
    /// "C1", the "complete object constructor"
    CompleteConstructor(Option<TypeHandle>),
    /// "C2", the "base object constructor"
    BaseConstructor(Option<TypeHandle>),
    /// "C3", the "complete object allocating constructor"
    CompleteAllocatingConstructor(Option<TypeHandle>),
    /// "C4", the "maybe in-charge constructor"
    MaybeInChargeConstructor(Option<TypeHandle>),
    /// "D0", the "deleting destructor"
    DeletingDestructor,
    /// "D1", the "complete object destructor"
    CompleteDestructor,
    /// "D2", the "base object destructor"
    BaseDestructor,
    /// "D4", the "maybe in-charge destructor"
    MaybeInChargeDestructor,
}

impl CtorDtorName {
    fn inheriting_mut(&mut self) -> &mut Option<TypeHandle> {
        match self {
            CtorDtorName::CompleteConstructor(ref mut inheriting)
            | CtorDtorName::BaseConstructor(ref mut inheriting)
            | CtorDtorName::CompleteAllocatingConstructor(ref mut inheriting)
            | CtorDtorName::MaybeInChargeConstructor(ref mut inheriting) => inheriting,
            CtorDtorName::DeletingDestructor
            | CtorDtorName::CompleteDestructor
            | CtorDtorName::BaseDestructor
            | CtorDtorName::MaybeInChargeDestructor => unreachable!(),
        }
    }
}

impl Parse for CtorDtorName {
    fn parse<'a, 'b>(
        ctx: &'a ParseContext,
        subs: &'a mut SubstitutionTable,
        input: IndexStr<'b>,
    ) -> Result<(CtorDtorName, IndexStr<'b>)> {
        try_begin_parse!(stringify!(CtorDtorName), ctx, input);

        match input.peek() {
            Some(b'C') => {
                let mut tail = consume(b"C", input)?;
                let inheriting = match tail.peek() {
                    Some(b'I') => {
                        tail = consume(b"I", tail)?;
                        true
                    }
                    _ => false,
                };

                let mut ctor_type: CtorDtorName = match tail
                    .try_split_at(1)
                    .as_ref()
                    .map(|&(ref h, t)| (h.as_ref(), t))
                {
                    None => Err(error::Error::UnexpectedEnd),
                    Some((b"1", t)) => {
                        tail = t;
                        Ok(CtorDtorName::CompleteConstructor(None))
                    }
                    Some((b"2", t)) => {
                        tail = t;
                        Ok(CtorDtorName::BaseConstructor(None))
                    }
                    Some((b"3", t)) => {
                        tail = t;
                        Ok(CtorDtorName::CompleteAllocatingConstructor(None))
                    }
                    Some((b"4", t)) => {
                        tail = t;
                        Ok(CtorDtorName::MaybeInChargeConstructor(None))
                    }
                    _ => Err(error::Error::UnexpectedText),
                }?;

                if inheriting {
                    let (ty, tail) = TypeHandle::parse(ctx, subs, tail)?;
                    *ctor_type.inheriting_mut() = Some(ty);
                    Ok((ctor_type, tail))
                } else {
                    Ok((ctor_type, tail))
                }
            }
            Some(b'D') => {
                match input
                    .try_split_at(2)
                    .as_ref()
                    .map(|&(ref h, t)| (h.as_ref(), t))
                {
                    Some((b"D0", tail)) => Ok((CtorDtorName::DeletingDestructor, tail)),
                    Some((b"D1", tail)) => Ok((CtorDtorName::CompleteDestructor, tail)),
                    Some((b"D2", tail)) => Ok((CtorDtorName::BaseDestructor, tail)),
                    Some((b"D4", tail)) => Ok((CtorDtorName::MaybeInChargeDestructor, tail)),
                    _ => Err(error::Error::UnexpectedText),
                }
            }
            None => Err(error::Error::UnexpectedEnd),
            _ => Err(error::Error::UnexpectedText),
        }
    }
}

impl<'subs, W> Demangle<'subs, W> for CtorDtorName
where
    W: 'subs + DemangleWrite,
{
    fn demangle<'prev, 'ctx>(
        &'subs self,
        ctx: &'ctx mut DemangleContext<'subs, W>,
        scope: Option<ArgScopeStack<'prev, 'subs>>,
    ) -> fmt::Result {
        let ctx = try_begin_demangle!(self, ctx, scope);

        let leaf = scope.leaf_name().map_err(|e| {
            log!("Error getting leaf name: {}", e);
            fmt::Error
        })?;

        match *self {
            CtorDtorName::CompleteConstructor(ref inheriting)
            | CtorDtorName::BaseConstructor(ref inheriting)
            | CtorDtorName::CompleteAllocatingConstructor(ref inheriting)
            | CtorDtorName::MaybeInChargeConstructor(ref inheriting) => match inheriting {
                Some(ty) => ty
                    .get_leaf_name(ctx.subs)
                    .ok_or_else(|| {
                        log!("Error getting leaf name: {:?}", ty);
                        fmt::Error
                    })?
                    .demangle_as_leaf(ctx),
                None => leaf.demangle_as_leaf(ctx),
            },
            CtorDtorName::DeletingDestructor
            | CtorDtorName::CompleteDestructor
            | CtorDtorName::BaseDestructor
            | CtorDtorName::MaybeInChargeDestructor => {
                write!(ctx, "~")?;
                leaf.demangle_as_leaf(ctx)
            }
        }
    }
}

impl CtorDtorName {
    #[inline]
    fn starts_with(byte: u8) -> bool {
        byte == b'C' || byte == b'D'
    }
}

/// The `<type>` production.
///
/// ```text
/// <type> ::= <builtin-type>
///        ::= <function-type>
///        ::= <class-enum-type>
///        ::= <array-type>
///        ::= <vector-type>
///        ::= <pointer-to-member-type>
///        ::= <template-param>
///        ::= <template-template-param> <template-args>
///        ::= <decltype>
///        ::= <CV-qualifiers> <type>
///        ::= P <type>                                 # pointer-to
///        ::= R <type>                                 # reference-to
///        ::= O <type>                                 # rvalue reference-to (C++0x)
///        ::= C <type>                                 # complex pair (C 2000)
///        ::= G <type>                                 # imaginary (C 2000)
///        ::= U <source-name> [<template-args>] <type> # vendor extended type qualifier
///        ::= Dp <type>                                # pack expansion (C++0x)
///        ::= <substitution>
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
#[allow(clippy::large_enum_variant)]
pub enum Type {
    /// A function type.
    Function(FunctionType),

    /// A class, union, or enum type.
    ClassEnum(ClassEnumType),

    /// An array type.
    Array(ArrayType),

    /// A vector type.
    Vector(VectorType),

    /// A pointer-to-member type.
    PointerToMember(PointerToMemberType),

    /// A named template parameter type.
    TemplateParam(TemplateParam),

    /// A template template type.
    TemplateTemplate(TemplateTemplateParamHandle, TemplateArgs),

    /// A decltype.
    Decltype(Decltype),

    /// A const-, restrict-, and/or volatile-qualified type.
    Qualified(CvQualifiers, TypeHandle),

    /// A pointer to a type.
    PointerTo(TypeHandle),

    /// An lvalue reference to a type.
    LvalueRef(TypeHandle),

    /// An rvalue reference to a type.
    RvalueRef(TypeHandle),

    /// A complex pair of the given type.
    Complex(TypeHandle),

    /// An imaginary of the given type.
    Imaginary(TypeHandle),

    /// A vendor extended type qualifier.
    VendorExtension(SourceName, Option<TemplateArgs>, TypeHandle),

    /// A pack expansion.
    PackExpansion(TypeHandle),
}

define_handle! {
    /// A reference to a parsed `Type` production.
    pub enum TypeHandle {
        /// A builtin type. These don't end up in the substitutions table.
        extra Builtin(BuiltinType),

        /// A CV-qualified builtin type. These don't end up in the table either.
        extra QualifiedBuiltin(QualifiedBuiltin),
    }
}

impl TypeHandle {
    fn is_void(&self) -> bool {
        match *self {
            TypeHandle::Builtin(BuiltinType::Standard(StandardBuiltinType::Void)) => true,
            _ => false,
        }
    }
}

impl Parse for TypeHandle {
    fn parse<'a, 'b>(
        ctx: &'a ParseContext,
        subs: &'a mut SubstitutionTable,
        input: IndexStr<'b>,
    ) -> Result<(TypeHandle, IndexStr<'b>)> {
        try_begin_parse!("TypeHandle", ctx, input);

        /// Insert the given type into the substitution table, and return a
        /// handle referencing the index in the table where it ended up.
        fn insert_and_return_handle<'a, 'b>(
            ty: Type,
            subs: &'a mut SubstitutionTable,
            tail: IndexStr<'b>,
        ) -> Result<(TypeHandle, IndexStr<'b>)> {
            let ty = Substitutable::Type(ty);
            let idx = subs.insert(ty);
            let handle = TypeHandle::BackReference(idx);
            Ok((handle, tail))
        }

        if let Ok((builtin, tail)) = try_recurse!(BuiltinType::parse(ctx, subs, input)) {
            // Builtin types are one of two exceptions that do not end up in the
            // substitutions table.
            let handle = TypeHandle::Builtin(builtin);
            return Ok((handle, tail));
        }

        // ::= <qualified-type>
        // We don't have a separate type for the <qualified-type> production.
        // Process these all up front, so that any ambiguity that might exist
        // with later productions is handled correctly.

        // ::= <extended-qualifier>
        if let Ok(tail) = consume(b"U", input) {
            let (name, tail) = SourceName::parse(ctx, subs, tail)?;
            let (args, tail) =
                if let Ok((args, tail)) = try_recurse!(TemplateArgs::parse(ctx, subs, tail)) {
                    (Some(args), tail)
                } else {
                    (None, tail)
                };
            let (ty, tail) = TypeHandle::parse(ctx, subs, tail)?;
            let ty = Type::VendorExtension(name, args, ty);
            return insert_and_return_handle(ty, subs, tail);
        }

        // ::= <CV-qualifiers>
        if let Ok((qualifiers, tail)) = try_recurse!(CvQualifiers::parse(ctx, subs, input)) {
            // CvQualifiers can parse successfully without consuming any input,
            // but we don't want to recurse unless we know we did consume some
            // input, lest we go into an infinite loop and blow the stack.
            if tail.len() < input.len() {
                // If the following production is a <function-type>, we want to let
                // it pick up these <CV-qualifiers>.
                if !FunctionType::starts_with(&tail) {
                    let (ty, tail) = TypeHandle::parse(ctx, subs, tail)?;
                    let ty = Type::Qualified(qualifiers, ty);
                    return insert_and_return_handle(ty, subs, tail);
                }
            }
        }

        if let Ok((ty, tail)) = try_recurse!(ClassEnumType::parse(ctx, subs, input)) {
            let ty = Type::ClassEnum(ty);
            return insert_and_return_handle(ty, subs, tail);
        }

        if let Ok((sub, tail)) = try_recurse!(Substitution::parse(ctx, subs, input)) {
            // If we see an 'I', then this is actually a substitution for a
            // <template-template-param>, and the template args are what
            // follows. Throw away what we just parsed, and re-parse it in
            // `TemplateTemplateParamHandle::parse` for now, but it would be
            // nice not to duplicate work we've already done.
            if tail.peek() != Some(b'I') {
                match sub {
                    Substitution::WellKnown(component) => {
                        return Ok((TypeHandle::WellKnown(component), tail));
                    }
                    Substitution::BackReference(idx) => {
                        // TODO: should this check if the back reference actually points
                        // to a <type>?
                        return Ok((TypeHandle::BackReference(idx), tail));
                    }
                }
            }
        }

        if let Ok((funty, tail)) = try_recurse!(FunctionType::parse(ctx, subs, input)) {
            let ty = Type::Function(funty);
            return insert_and_return_handle(ty, subs, tail);
        }

        if let Ok((ty, tail)) = try_recurse!(ArrayType::parse(ctx, subs, input)) {
            let ty = Type::Array(ty);
            return insert_and_return_handle(ty, subs, tail);
        }

        if let Ok((ty, tail)) = try_recurse!(VectorType::parse(ctx, subs, input)) {
            let ty = Type::Vector(ty);
            return insert_and_return_handle(ty, subs, tail);
        }

        if let Ok((ty, tail)) = try_recurse!(PointerToMemberType::parse(ctx, subs, input)) {
            let ty = Type::PointerToMember(ty);
            return insert_and_return_handle(ty, subs, tail);
        }

        if let Ok((param, tail)) = try_recurse!(TemplateParam::parse(ctx, subs, input)) {
            // Same situation as with `Substitution::parse` at the top of this
            // function: this is actually a <template-template-param> and
            // <template-args>.
            if tail.peek() != Some(b'I') {
                let ty = Type::TemplateParam(param);
                return insert_and_return_handle(ty, subs, tail);
            } else if ctx.in_conversion() {
                // This may be <template-template-param> <template-args>.
                // But if we're here for a conversion operator, that's only
                // possible if the grammar looks like:
                //
                // <nested-name>
                // -> <source-name> cv <template-template-param> <template-args> <template-args>
                //
                // That is, there must be *another* <template-args> production after ours.
                // If there isn't one, then this really is a <template-param>.
                //
                // NB: Parsing a <template-args> production may modify the substitutions
                // table, so we need to avoid contaminating the official copy.
                let mut tmp_subs = subs.clone();
                if let Ok((_, new_tail)) =
                    try_recurse!(TemplateArgs::parse(ctx, &mut tmp_subs, tail))
                {
                    if new_tail.peek() != Some(b'I') {
                        // Don't consume the TemplateArgs.
                        let ty = Type::TemplateParam(param);
                        return insert_and_return_handle(ty, subs, tail);
                    }
                    // We really do have a <template-template-param>. Fall through.
                    // NB: We can't use the arguments we just parsed because a
                    // TemplateTemplateParam is substitutable, and if we use it
                    // any substitutions in the arguments will come *before* it,
                    // putting the substitution table out of order.
                }
            }
        }

        if let Ok((ttp, tail)) = try_recurse!(TemplateTemplateParamHandle::parse(ctx, subs, input))
        {
            let (args, tail) = TemplateArgs::parse(ctx, subs, tail)?;
            let ty = Type::TemplateTemplate(ttp, args);
            return insert_and_return_handle(ty, subs, tail);
        }

        if let Ok((param, tail)) = try_recurse!(Decltype::parse(ctx, subs, input)) {
            let ty = Type::Decltype(param);
            return insert_and_return_handle(ty, subs, tail);
        }

        if let Ok(tail) = consume(b"P", input) {
            let (ty, tail) = TypeHandle::parse(ctx, subs, tail)?;
            let ty = Type::PointerTo(ty);
            return insert_and_return_handle(ty, subs, tail);
        }

        if let Ok(tail) = consume(b"R", input) {
            let (ty, tail) = TypeHandle::parse(ctx, subs, tail)?;
            let ty = Type::LvalueRef(ty);
            return insert_and_return_handle(ty, subs, tail);
        }

        if let Ok(tail) = consume(b"O", input) {
            let (ty, tail) = TypeHandle::parse(ctx, subs, tail)?;
            let ty = Type::RvalueRef(ty);
            return insert_and_return_handle(ty, subs, tail);
        }

        if let Ok(tail) = consume(b"C", input) {
            let (ty, tail) = TypeHandle::parse(ctx, subs, tail)?;
            let ty = Type::Complex(ty);
            return insert_and_return_handle(ty, subs, tail);
        }

        if let Ok(tail) = consume(b"G", input) {
            let (ty, tail) = TypeHandle::parse(ctx, subs, tail)?;
            let ty = Type::Imaginary(ty);
            return insert_and_return_handle(ty, subs, tail);
        }

        let tail = consume(b"Dp", input)?;
        let (ty, tail) = TypeHandle::parse(ctx, subs, tail)?;
        let ty = Type::PackExpansion(ty);
        insert_and_return_handle(ty, subs, tail)
    }
}

impl GetTemplateArgs for TypeHandle {
    fn get_template_args<'a>(&'a self, subs: &'a SubstitutionTable) -> Option<&'a TemplateArgs> {
        subs.get_type(self)
            .and_then(|ty| ty.get_template_args(subs))
    }
}

impl<'subs, W> Demangle<'subs, W> for Type
where
    W: 'subs + DemangleWrite,
{
    fn demangle<'prev, 'ctx>(
        &'subs self,
        ctx: &'ctx mut DemangleContext<'subs, W>,
        scope: Option<ArgScopeStack<'prev, 'subs>>,
    ) -> fmt::Result {
        let ctx = try_begin_demangle!(self, ctx, scope);

        match *self {
            Type::Function(ref func_ty) => func_ty.demangle(ctx, scope),
            Type::ClassEnum(ref cls_enum_ty) => cls_enum_ty.demangle(ctx, scope),
            Type::Array(ref array_ty) => array_ty.demangle(ctx, scope),
            Type::Vector(ref vector_ty) => vector_ty.demangle(ctx, scope),
            Type::PointerToMember(ref ptm) => ptm.demangle(ctx, scope),
            Type::TemplateParam(ref param) => param.demangle(ctx, scope),
            Type::TemplateTemplate(ref tt_param, ref args) => {
                tt_param.demangle(ctx, scope)?;
                args.demangle(ctx, scope)
            }
            Type::Decltype(ref dt) => dt.demangle(ctx, scope),
            Type::Qualified(_, ref ty) => {
                ctx.push_inner(self);
                ty.demangle(ctx, scope)?;
                if ctx.pop_inner_if(self) {
                    self.demangle_as_inner(ctx, scope)?;
                }
                Ok(())
            }
            Type::PointerTo(ref ty) | Type::LvalueRef(ref ty) | Type::RvalueRef(ref ty) => {
                ctx.push_inner(self);
                ty.demangle(ctx, scope)?;
                if ctx.pop_inner_if(self) {
                    self.demangle_as_inner(ctx, scope)?;
                }
                Ok(())
            }
            Type::Complex(ref ty) => {
                ty.demangle(ctx, scope)?;
                write!(ctx, " complex")?;
                Ok(())
            }
            Type::Imaginary(ref ty) => {
                ty.demangle(ctx, scope)?;
                write!(ctx, " imaginary")?;
                Ok(())
            }
            Type::VendorExtension(ref name, ref template_args, ref ty) => {
                ty.demangle(ctx, scope)?;
                write!(ctx, " ")?;
                name.demangle(ctx, scope)?;
                if let Some(ref args) = *template_args {
                    args.demangle(ctx, scope)?;
                }
                Ok(())
            }
            Type::PackExpansion(ref ty) => {
                ty.demangle(ctx, scope)?;
                if !ctx.is_template_argument_pack {
                    write!(ctx, "...")?;
                }
                Ok(())
            }
        }
    }
}

impl<'subs, W> DemangleAsInner<'subs, W> for Type
where
    W: 'subs + DemangleWrite,
{
    fn demangle_as_inner<'prev, 'ctx>(
        &'subs self,
        ctx: &'ctx mut DemangleContext<'subs, W>,
        scope: Option<ArgScopeStack<'prev, 'subs>>,
    ) -> fmt::Result {
        let ctx = try_begin_demangle_as_inner!(self, ctx, scope);

        match *self {
            Type::Qualified(ref quals, _) => quals.demangle_as_inner(ctx, scope),
            Type::PointerTo(_) => write!(ctx, "*"),
            Type::RvalueRef(_) => {
                while let Some(v) = ctx.inner.last().and_then(|ty| ty.downcast_to_type()) {
                    match v {
                        // Two r-value references combine into a single r-value reference
                        // Consume any adjacent r-value references on the inner stack.
                        Type::RvalueRef(_) => {
                            ctx.inner.pop().unwrap();
                        }
                        // An r-value and an l-value reference combine into an l-value reference.
                        // Skip printing this, and allow the LvalueRef implementation to
                        // continue combining references.
                        Type::LvalueRef(_) => return Ok(()),
                        _ => break,
                    }
                }
                write!(ctx, "&&")
            }
            Type::LvalueRef(_) => {
                while let Some(v) = ctx.inner.last().and_then(|ty| ty.downcast_to_type()) {
                    match v {
                        // An l-value reference combines with an r-value reference to form a
                        // single l-value reference. Consume any adjacent r-value references
                        // on the inner stack.
                        Type::RvalueRef(_) => {
                            ctx.inner.pop().unwrap();
                        }
                        // Two l-value references combine to form a single l-value reference.
                        // Skip printing this, and allow the LvalueRef implementation for
                        // the next l-value reference to continue combining references.
                        Type::LvalueRef(_) => return Ok(()),
                        _ => break,
                    }
                }
                write!(ctx, "&")
            }
            ref otherwise => {
                unreachable!(
                    "We shouldn't ever put any other types on the inner stack: {:?}",
                    otherwise
                );
            }
        }
    }

    fn downcast_to_type(&self) -> Option<&Type> {
        Some(self)
    }

    fn downcast_to_function_type(&self) -> Option<&FunctionType> {
        if let Type::Function(ref f) = *self {
            Some(f)
        } else {
            None
        }
    }

    fn downcast_to_array_type(&self) -> Option<&ArrayType> {
        if let Type::Array(ref arr) = *self {
            Some(arr)
        } else {
            None
        }
    }

    fn downcast_to_pointer_to_member(&self) -> Option<&PointerToMemberType> {
        if let Type::PointerToMember(ref ptm) = *self {
            Some(ptm)
        } else {
            None
        }
    }

    fn is_qualified(&self) -> bool {
        match *self {
            Type::Qualified(..) => true,
            _ => false,
        }
    }
}

impl GetTemplateArgs for Type {
    fn get_template_args<'a>(&'a self, subs: &'a SubstitutionTable) -> Option<&'a TemplateArgs> {
        // TODO: This should probably recurse through all the nested type
        // handles too.

        match *self {
            Type::VendorExtension(_, Some(ref args), _) | Type::TemplateTemplate(_, ref args) => {
                Some(args)
            }
            Type::PointerTo(ref ty) | Type::LvalueRef(ref ty) | Type::RvalueRef(ref ty) => {
                ty.get_template_args(subs)
            }
            _ => None,
        }
    }
}

impl<'a> GetLeafName<'a> for Type {
    fn get_leaf_name(&'a self, subs: &'a SubstitutionTable) -> Option<LeafName<'a>> {
        match *self {
            Type::ClassEnum(ref cls_enum_ty) => cls_enum_ty.get_leaf_name(subs),
            _ => None,
        }
    }
}

/// The `<CV-qualifiers>` production.
///
/// ```text
/// <CV-qualifiers> ::= [r] [V] [K]   # restrict (C99), volatile, const
/// ```
#[derive(Clone, Debug, Default, Hash, PartialEq, Eq)]
pub struct CvQualifiers {
    /// Is this `restrict` qualified?
    pub restrict: bool,
    /// Is this `volatile` qualified?
    pub volatile: bool,
    /// Is this `const` qualified?
    pub const_: bool,
}

impl CvQualifiers {
    #[inline]
    fn is_empty(&self) -> bool {
        !self.restrict && !self.volatile && !self.const_
    }
}

impl Parse for CvQualifiers {
    fn parse<'a, 'b>(
        ctx: &'a ParseContext,
        _subs: &'a mut SubstitutionTable,
        input: IndexStr<'b>,
    ) -> Result<(CvQualifiers, IndexStr<'b>)> {
        try_begin_parse!("CvQualifiers", ctx, input);

        let (restrict, tail) = if let Ok(tail) = consume(b"r", input) {
            (true, tail)
        } else {
            (false, input)
        };

        let (volatile, tail) = if let Ok(tail) = consume(b"V", tail) {
            (true, tail)
        } else {
            (false, tail)
        };

        let (const_, tail) = if let Ok(tail) = consume(b"K", tail) {
            (true, tail)
        } else {
            (false, tail)
        };

        let qualifiers = CvQualifiers {
            restrict: restrict,
            volatile: volatile,
            const_: const_,
        };

        Ok((qualifiers, tail))
    }
}

impl<'subs, W> Demangle<'subs, W> for CvQualifiers
where
    W: 'subs + DemangleWrite,
{
    fn demangle<'prev, 'ctx>(
        &'subs self,
        ctx: &'ctx mut DemangleContext<'subs, W>,
        scope: Option<ArgScopeStack<'prev, 'subs>>,
    ) -> fmt::Result {
        let ctx = try_begin_demangle!(self, ctx, scope);

        if self.const_ {
            ctx.ensure_space()?;
            write!(ctx, "const")?;
        }

        if self.volatile {
            ctx.ensure_space()?;
            write!(ctx, "volatile")?;
        }

        if self.restrict {
            ctx.ensure_space()?;
            write!(ctx, "restrict")?;
        }

        Ok(())
    }
}

impl<'subs, W> DemangleAsInner<'subs, W> for CvQualifiers where W: 'subs + DemangleWrite {}

define_vocabulary! {
    /// A <ref-qualifier> production.
    ///
    /// ```text
    /// <ref-qualifier> ::= R   # & ref-qualifier
    ///                 ::= O   # && ref-qualifier
    /// ```
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub enum RefQualifier {
        LValueRef(b"R", "&"),
        RValueRef(b"O", "&&")
    }
}

define_vocabulary! {
    /// A named explicit object parameter.
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub enum ExplicitObjectParameter {
        ExplicitObjectParameter(b"H", "this")
    }
}

define_vocabulary! {
    /// A one of the standard variants of the <builtin-type> production.
    ///
    /// ```text
    /// <builtin-type> ::= v  # void
    ///                ::= w  # wchar_t
    ///                ::= b  # bool
    ///                ::= c  # char
    ///                ::= a  # signed char
    ///                ::= h  # unsigned char
    ///                ::= s  # short
    ///                ::= t  # unsigned short
    ///                ::= i  # int
    ///                ::= j  # unsigned int
    ///                ::= l  # long
    ///                ::= m  # unsigned long
    ///                ::= x  # long long, __int64
    ///                ::= y  # unsigned long long, __int64
    ///                ::= n  # __int128
    ///                ::= o  # unsigned __int128
    ///                ::= f  # float
    ///                ::= d  # double
    ///                ::= e  # long double, __float80
    ///                ::= g  # __float128
    ///                ::= z  # ellipsis
    ///                ::= Dd # IEEE 754r decimal floating point (64 bits)
    ///                ::= De # IEEE 754r decimal floating point (128 bits)
    ///                ::= Df # IEEE 754r decimal floating point (32 bits)
    ///                ::= Dh # IEEE 754r half-precision floating point (16 bits)
    ///                ::= DF16b # C++23 std::bfloat16_t
    ///                ::= Di # char32_t
    ///                ::= Ds # char16_t
    ///                ::= Du # char8_t
    ///                ::= Da # auto
    ///                ::= Dc # decltype(auto)
    ///                ::= Dn # std::nullptr_t (i.e., decltype(nullptr))
    ///                ::= [DS] DA  # N1169 fixed-point [_Sat] T _Accum
    ///                ::= [DS] DR  # N1169 fixed-point [_Sat] T _Fract
    ///
    ///  <fixed-point-size> ::= s # short
    ///                     ::= t # unsigned short
    ///                     ::= i # plain
    ///                     ::= j # unsigned
    ///                     ::= l # long
    ///                     ::= m # unsigned long
    /// ```
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub enum StandardBuiltinType {
        Void             (b"v",  "void"),
        Wchar            (b"w",  "wchar_t"),
        Bool             (b"b",  "bool"),
        Char             (b"c",  "char"),
        SignedChar       (b"a",  "signed char"),
        UnsignedChar     (b"h",  "unsigned char"),
        Short            (b"s",  "short"),
        UnsignedShort    (b"t",  "unsigned short"),
        Int              (b"i",  "int"),
        UnsignedInt      (b"j",  "unsigned int"),
        Long             (b"l",  "long"),
        UnsignedLong     (b"m",  "unsigned long"),
        LongLong         (b"x",  "long long"),
        UnsignedLongLong (b"y",  "unsigned long long"),
        Int128           (b"n",  "__int128"),
        Uint128          (b"o",  "unsigned __int128"),
        Float            (b"f",  "float"),
        Double           (b"d",  "double"),
        LongDouble       (b"e",  "long double"),
        Float128         (b"g",  "__float128"),
        Ellipsis         (b"z",  "..."),
        DecimalFloat64   (b"Dd", "decimal64"),
        DecimalFloat128  (b"De", "decimal128"),
        DecimalFloat32   (b"Df", "decimal32"),
        DecimalFloat16   (b"Dh", "half"),
        BFloat16         (b"DF16b", "std::bfloat16_t"),
        Char32           (b"Di", "char32_t"),
        Char16           (b"Ds", "char16_t"),
        Char8            (b"Du", "char8_t"),
        Auto             (b"Da", "auto"),
        Decltype         (b"Dc", "decltype(auto)"),
        Nullptr          (b"Dn", "std::nullptr_t"),
        AccumShort       (b"DAs", "short _Accum"),
        AccumUShort      (b"DAt", "unsigned short _Accum"),
        Accum            (b"DAi", "_Accum"),
        AccumUnsigned    (b"DAj", "unsigned _Accum"),
        AccumLong        (b"DAl", "long _Accum"),
        AccumULong       (b"DAm", "unsigned long _Accum"),
        FractShort       (b"DRs", "short _Fract"),
        FractUShort      (b"DRt", "unsigned short _Fract"),
        Fract            (b"DRi", "_Fract"),
        FractUnsigned    (b"DRj", "unsigned _Fract"),
        FractLong        (b"DRl", "long _Fract"),
        FractULong       (b"DRm", "unsigned long _Fract"),
        SatAccumShort    (b"DSDAs", "_Sat short _Accum"),
        SatAccumUShort   (b"DSDAt", "_Sat unsigned short _Accum"),
        SatAccum         (b"DSDAi", "_Sat _Accum"),
        SatAccumUnsigned (b"DSDAj", "_Sat unsigned _Accum"),
        SatAccumLong     (b"DSDAl", "_Sat long _Accum"),
        SatAccumULong    (b"DSDAm", "_Sat unsigned long _Accum"),
        SatFractShort    (b"DSDRs", "_Sat short _Fract"),
        SatFractUShort   (b"DSDRt", "_Sat unsigned short _Fract"),
        SatFract         (b"DSDRi", "_Sat _Fract"),
        SatFractUnsigned (b"DSDRj", "_Sat unsigned _Fract"),
        SatFractLong     (b"DSDRl", "_Sat long _Fract"),
        SatFractULong    (b"DSDRm", "_Sat unsigned long _Fract")
    }
}

/// <builtin-type> ::= DF <number> _ # ISO/IEC TS 18661 binary floating point type _FloatN (N bits), C++23 std::floatN_t
///                ::= DF <number> x # IEEE extended precision formats, C23 _FloatNx (N bits)
///                ::= DB <number> _        # C23 signed _BitInt(N)
///                ::= DB <instantiation-dependent expression> _ # C23 signed _BitInt(N)
///                ::= DU <number> _        # C23 unsigned _BitInt(N)
///                ::= DU <instantiation-dependent expression> _ # C23 unsigned _BitInt(N)
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ParametricBuiltinType {
    /// _FloatN
    FloatN(Number),
    /// _FloatNx
    FloatNx(Number),
    /// signed _BitInt(N)
    SignedBitInt(Number),
    /// unsigned _BitInt(N)
    UnsignedBitInt(Number),
    /// signed _BitInt(expr)
    SignedBitIntExpression(Box<Expression>),
    /// unsigned _BitInt(expr)
    UnsignedBitIntExpression(Box<Expression>),
}

impl Parse for ParametricBuiltinType {
    fn parse<'a, 'b>(
        ctx: &'a ParseContext,
        subs: &'a mut SubstitutionTable,
        input: IndexStr<'b>,
    ) -> Result<(ParametricBuiltinType, IndexStr<'b>)> {
        try_begin_parse!("ParametricBuiltinType", ctx, input);

        let input = consume(b"D", input)?;
        let (ch, input) = input.next_or(error::Error::UnexpectedEnd)?;
        let allow_expression = match ch {
            b'F' => false,
            b'B' | b'U' => true,
            _ => return Err(error::Error::UnexpectedText),
        };
        if input
            .next_or(error::Error::UnexpectedEnd)?
            .0
            .is_ascii_digit()
        {
            let (bit_size, input) = parse_number(10, false, input)?;
            if ch == b'F' {
                if let Ok(input) = consume(b"x", input) {
                    return Ok((ParametricBuiltinType::FloatNx(bit_size), input));
                }
            }
            let input = consume(b"_", input)?;
            let t = match ch {
                b'F' => ParametricBuiltinType::FloatN(bit_size),
                b'B' => ParametricBuiltinType::SignedBitInt(bit_size),
                b'U' => ParametricBuiltinType::UnsignedBitInt(bit_size),
                _ => panic!("oh noes"),
            };
            Ok((t, input))
        } else if allow_expression {
            let (expr, input) = Expression::parse(ctx, subs, input)?;
            let expr = Box::new(expr);
            let t = match ch {
                b'B' => ParametricBuiltinType::SignedBitIntExpression(expr),
                b'U' => ParametricBuiltinType::UnsignedBitIntExpression(expr),
                _ => panic!("oh noes"),
            };
            Ok((t, input))
        } else {
            Err(error::Error::UnexpectedText)
        }
    }
}

impl<'subs, W> Demangle<'subs, W> for ParametricBuiltinType
where
    W: 'subs + DemangleWrite,
{
    fn demangle<'prev, 'ctx>(
        &'subs self,
        ctx: &'ctx mut DemangleContext<'subs, W>,
        scope: Option<ArgScopeStack<'prev, 'subs>>,
    ) -> fmt::Result {
        let ctx = try_begin_demangle!(self, ctx, scope);

        match *self {
            Self::FloatN(n) => write!(ctx, "_Float{}", n),
            Self::FloatNx(n) => write!(ctx, "_Float{}x", n),
            Self::SignedBitInt(n) => write!(ctx, "signed _BitInt({})", n),
            Self::UnsignedBitInt(n) => write!(ctx, "unsigned _BitInt({})", n),
            Self::SignedBitIntExpression(ref expr) => {
                write!(ctx, "signed _BitInt(")?;
                expr.demangle(ctx, scope)?;
                write!(ctx, ")")
            }
            Self::UnsignedBitIntExpression(ref expr) => {
                write!(ctx, "unsigned _BitInt(")?;
                expr.demangle(ctx, scope)?;
                write!(ctx, ")")
            }
        }
    }
}

/// The `<builtin-type>` production.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BuiltinType {
    /// A simple standards compliant builtin type.
    Standard(StandardBuiltinType),

    /// A standards compliant builtin type with a parameter, e.g. _BitInt(32).
    Parametric(ParametricBuiltinType),

    /// A non-standard, vendor extension type.
    ///
    /// ```text
    /// <builtin-type> ::= u <source-name>   # vendor extended type
    /// ```
    Extension(SourceName),
}

impl Parse for BuiltinType {
    fn parse<'a, 'b>(
        ctx: &'a ParseContext,
        subs: &'a mut SubstitutionTable,
        input: IndexStr<'b>,
    ) -> Result<(BuiltinType, IndexStr<'b>)> {
        try_begin_parse!("BuiltinType", ctx, input);

        if let Ok((ty, tail)) = try_recurse!(StandardBuiltinType::parse(ctx, subs, input)) {
            return Ok((BuiltinType::Standard(ty), tail));
        }

        if let Ok(tail) = consume(b"u", input) {
            let (name, tail) = SourceName::parse(ctx, subs, tail)?;
            Ok((BuiltinType::Extension(name), tail))
        } else {
            match try_recurse!(ParametricBuiltinType::parse(ctx, subs, input)) {
                Ok((ty, tail)) => Ok((BuiltinType::Parametric(ty), tail)),
                Err(e) => Err(e),
            }
        }
    }
}

impl<'subs, W> Demangle<'subs, W> for BuiltinType
where
    W: 'subs + DemangleWrite,
{
    fn demangle<'prev, 'ctx>(
        &'subs self,
        ctx: &'ctx mut DemangleContext<'subs, W>,
        scope: Option<ArgScopeStack<'prev, 'subs>>,
    ) -> fmt::Result {
        let ctx = try_begin_demangle!(self, ctx, scope);

        match *self {
            BuiltinType::Standard(ref ty) => ty.demangle(ctx, scope),
            BuiltinType::Parametric(ref ty) => ty.demangle(ctx, scope),
            BuiltinType::Extension(ref name) => name.demangle(ctx, scope),
        }
    }
}

impl<'a> GetLeafName<'a> for BuiltinType {
    fn get_leaf_name(&'a self, _: &'a SubstitutionTable) -> Option<LeafName<'a>> {
        None
    }
}

/// A built-in type with CV-qualifiers.
///
/// Like unqualified built-in types, CV-qualified built-in types do not go into
/// the substitutions table.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct QualifiedBuiltin(CvQualifiers, BuiltinType);

impl<'subs, W> Demangle<'subs, W> for QualifiedBuiltin
where
    W: 'subs + DemangleWrite,
{
    fn demangle<'prev, 'ctx>(
        &'subs self,
        ctx: &'ctx mut DemangleContext<'subs, W>,
        scope: Option<ArgScopeStack<'prev, 'subs>>,
    ) -> fmt::Result {
        let ctx = try_begin_demangle!(self, ctx, scope);

        ctx.push_inner(&self.0);
        self.1.demangle(ctx, scope)?;
        if ctx.pop_inner_if(&self.0) {
            self.0.demangle_as_inner(ctx, scope)?;
        }
        Ok(())
    }
}

impl<'a> GetLeafName<'a> for QualifiedBuiltin {
    fn get_leaf_name(&'a self, _: &'a SubstitutionTable) -> Option<LeafName<'a>> {
        None
    }
}

/// The `<exception-spec>` production.
///
/// <exception-spec> ::= Do                # non-throwing exception-specification (e.g., noexcept, throw())
///                  ::= DO <expression> E # computed (instantiation-dependent) noexcept
///                  ::= Dw <type>+ E      # dynamic exception specification with instantiation-dependent types
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ExceptionSpec {
    /// noexcept
    NoExcept,
    /// noexcept(expression)
    Computed(Expression),
    // Dynamic exception specification is deprecated, lets see if we can get away with
    // not implementing it.
}

impl Parse for ExceptionSpec {
    fn parse<'a, 'b>(
        ctx: &'a ParseContext,
        subs: &'a mut SubstitutionTable,
        input: IndexStr<'b>,
    ) -> Result<(ExceptionSpec, IndexStr<'b>)> {
        try_begin_parse!("ExceptionSpec", ctx, input);

        if let Ok(tail) = consume(b"Do", input) {
            return Ok((ExceptionSpec::NoExcept, tail));
        }

        let tail = consume(b"DO", input)?;
        let (expr, tail) = Expression::parse(ctx, subs, tail)?;
        let tail = consume(b"E", tail)?;
        Ok((ExceptionSpec::Computed(expr), tail))
    }
}

impl<'subs, W> Demangle<'subs, W> for ExceptionSpec
where
    W: 'subs + DemangleWrite,
{
    fn demangle<'prev, 'ctx>(
        &'subs self,
        ctx: &'ctx mut DemangleContext<'subs, W>,
        scope: Option<ArgScopeStack<'prev, 'subs>>,
    ) -> fmt::Result {
        let ctx = try_begin_demangle!(self, ctx, scope);

        match *self {
            ExceptionSpec::NoExcept => write!(ctx, "noexcept"),
            ExceptionSpec::Computed(ref expr) => {
                write!(ctx, "noexcept(")?;
                expr.demangle(ctx, scope)?;
                write!(ctx, ")")
            }
        }
    }
}

/// The `<function-type>` production.
///
/// ```text
/// <function-type> ::= [<CV-qualifiers>] [exception-spec] [Dx] F [Y] <bare-function-type> [<ref-qualifier>] E
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FunctionType {
    cv_qualifiers: CvQualifiers,
    exception_spec: Option<ExceptionSpec>,
    transaction_safe: bool,
    extern_c: bool,
    bare: BareFunctionType,
    ref_qualifier: Option<RefQualifier>,
}

impl Parse for FunctionType {
    fn parse<'a, 'b>(
        ctx: &'a ParseContext,
        subs: &'a mut SubstitutionTable,
        input: IndexStr<'b>,
    ) -> Result<(FunctionType, IndexStr<'b>)> {
        try_begin_parse!("FunctionType", ctx, input);

        let (cv_qualifiers, tail) = if let Ok((cv_qualifiers, tail)) =
            try_recurse!(CvQualifiers::parse(ctx, subs, input))
        {
            (cv_qualifiers, tail)
        } else {
            (Default::default(), input)
        };

        let (exception_spec, tail) = if let Ok((exception_spec, tail)) =
            try_recurse!(ExceptionSpec::parse(ctx, subs, tail))
        {
            (Some(exception_spec), tail)
        } else {
            (None, tail)
        };

        let (transaction_safe, tail) = if let Ok(tail) = consume(b"Dx", tail) {
            (true, tail)
        } else {
            (false, tail)
        };

        let tail = consume(b"F", tail)?;

        let (extern_c, tail) = if let Ok(tail) = consume(b"Y", tail) {
            (true, tail)
        } else {
            (false, tail)
        };

        let (bare, tail) = BareFunctionType::parse(ctx, subs, tail)?;

        let (ref_qualifier, tail) =
            if let Ok((ref_qualifier, tail)) = try_recurse!(RefQualifier::parse(ctx, subs, tail)) {
                (Some(ref_qualifier), tail)
            } else {
                (None, tail)
            };

        let tail = consume(b"E", tail)?;

        let func_ty = FunctionType {
            cv_qualifiers: cv_qualifiers,
            exception_spec: exception_spec,
            transaction_safe: transaction_safe,
            extern_c: extern_c,
            bare: bare,
            ref_qualifier: ref_qualifier,
        };
        Ok((func_ty, tail))
    }
}

impl<'subs, W> Demangle<'subs, W> for FunctionType
where
    W: 'subs + DemangleWrite,
{
    fn demangle<'prev, 'ctx>(
        &'subs self,
        ctx: &'ctx mut DemangleContext<'subs, W>,
        scope: Option<ArgScopeStack<'prev, 'subs>>,
    ) -> fmt::Result {
        let ctx = try_begin_demangle!(self, ctx, scope);

        ctx.push_inner(self);
        self.bare.demangle(ctx, scope)?;
        if ctx.pop_inner_if(self) {
            self.demangle_as_inner(ctx, scope)?;
        }
        if let Some(ref es) = self.exception_spec {
            // Print out a space before printing "noexcept"
            ctx.ensure_space()?;
            es.demangle(ctx, scope)?;
        }
        Ok(())
    }
}

impl<'subs, W> DemangleAsInner<'subs, W> for FunctionType
where
    W: 'subs + DemangleWrite,
{
    fn demangle_as_inner<'prev, 'ctx>(
        &'subs self,
        ctx: &'ctx mut DemangleContext<'subs, W>,
        scope: Option<ArgScopeStack<'prev, 'subs>>,
    ) -> fmt::Result {
        let ctx = try_begin_demangle_as_inner!(self, ctx, scope);

        if !self.cv_qualifiers.is_empty() {
            self.cv_qualifiers.demangle(ctx, scope)?;
        }

        if let Some(ref rq) = self.ref_qualifier {
            // Print out a space before printing "&" or "&&"
            ctx.ensure_space()?;
            rq.demangle(ctx, scope)?;
        }

        Ok(())
    }

    fn downcast_to_function_type(&self) -> Option<&FunctionType> {
        Some(self)
    }
}

impl FunctionType {
    #[inline]
    fn starts_with(input: &IndexStr) -> bool {
        input.peek() == Some(b'F')
            || (input.peek() == Some(b'D')
                && (matches!(
                    input.peek_second(),
                    Some(b'o') | Some(b'O') | Some(b'x') | Some(b'w')
                )))
    }
}

/// The `<bare-function-type>` production.
///
/// ```text
/// <bare-function-type> ::= <signature type>+
///      # types are possible return type, then parameter types
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BareFunctionType(Vec<TypeHandle>);

impl BareFunctionType {
    fn ret(&self) -> &TypeHandle {
        &self.0[0]
    }

    fn args(&self) -> &FunctionArgListAndReturnType {
        FunctionArgListAndReturnType::new(&self.0)
    }
}

impl Parse for BareFunctionType {
    fn parse<'a, 'b>(
        ctx: &'a ParseContext,
        subs: &'a mut SubstitutionTable,
        input: IndexStr<'b>,
    ) -> Result<(BareFunctionType, IndexStr<'b>)> {
        try_begin_parse!("BareFunctionType", ctx, input);

        let (types, tail) = one_or_more::<TypeHandle>(ctx, subs, input)?;
        Ok((BareFunctionType(types), tail))
    }
}

impl<'subs, W> Demangle<'subs, W> for BareFunctionType
where
    W: 'subs + DemangleWrite,
{
    fn demangle<'prev, 'ctx>(
        &'subs self,
        ctx: &'ctx mut DemangleContext<'subs, W>,
        scope: Option<ArgScopeStack<'prev, 'subs>>,
    ) -> fmt::Result {
        let ctx = try_begin_demangle!(self, ctx, scope);

        ctx.push_inner(self);

        self.ret().demangle(ctx, scope)?;

        if ctx.pop_inner_if(self) {
            ctx.ensure_space()?;
            self.demangle_as_inner(ctx, scope)?;
        }

        Ok(())
    }
}

impl<'subs, W> DemangleAsInner<'subs, W> for BareFunctionType
where
    W: 'subs + DemangleWrite,
{
    fn demangle_as_inner<'prev, 'ctx>(
        &'subs self,
        ctx: &'ctx mut DemangleContext<'subs, W>,
        scope: Option<ArgScopeStack<'prev, 'subs>>,
    ) -> fmt::Result {
        let ctx = try_begin_demangle_as_inner!(self, ctx, scope);
        self.args().demangle_as_inner(ctx, scope)?;
        Ok(())
    }
}

/// The `<decltype>` production.
///
/// ```text
/// <decltype> ::= Dt <expression> E
///            ::= DT <expression> E
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Decltype {
    /// A `decltype` of an id-expression or class member access (C++0x).
    IdExpression(Expression),

    /// A `decltype` of an expression (C++0x).
    Expression(Expression),
}

impl Parse for Decltype {
    fn parse<'a, 'b>(
        ctx: &'a ParseContext,
        subs: &'a mut SubstitutionTable,
        input: IndexStr<'b>,
    ) -> Result<(Decltype, IndexStr<'b>)> {
        try_begin_parse!("Decltype", ctx, input);

        let tail = consume(b"D", input)?;

        if let Ok(tail) = consume(b"t", tail) {
            let (expr, tail) = Expression::parse(ctx, subs, tail)?;
            let tail = consume(b"E", tail)?;
            return Ok((Decltype::IdExpression(expr), tail));
        }

        let tail = consume(b"T", tail)?;
        let (expr, tail) = Expression::parse(ctx, subs, tail)?;
        let tail = consume(b"E", tail)?;
        Ok((Decltype::Expression(expr), tail))
    }
}

impl<'subs, W> Demangle<'subs, W> for Decltype
where
    W: 'subs + DemangleWrite,
{
    fn demangle<'prev, 'ctx>(
        &'subs self,
        ctx: &'ctx mut DemangleContext<'subs, W>,
        scope: Option<ArgScopeStack<'prev, 'subs>>,
    ) -> fmt::Result {
        let ctx = try_begin_demangle!(self, ctx, scope);

        ctx.push_demangle_node(DemangleNodeType::TemplateParam);
        let ret = match *self {
            Decltype::Expression(ref expr) | Decltype::IdExpression(ref expr) => {
                write!(ctx, "decltype (")?;
                expr.demangle(ctx, scope)?;
                write!(ctx, ")")?;
                Ok(())
            }
        };
        ctx.pop_demangle_node();
        ret
    }
}

/// The `<class-enum-type>` production.
///
/// ```text
/// <class-enum-type> ::= <name>
///                   ::= Ts <name>
///                   ::= Tu <name>
///                   ::= Te <name>
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ClassEnumType {
    /// A non-dependent type name, dependent type name, or dependent
    /// typename-specifier.
    Named(Name),

    /// A dependent elaborated type specifier using 'struct' or 'class'.
    ElaboratedStruct(Name),

    /// A dependent elaborated type specifier using 'union'.
    ElaboratedUnion(Name),

    /// A dependent elaborated type specifier using 'enum'.
    ElaboratedEnum(Name),
}

impl Parse for ClassEnumType {
    fn parse<'a, 'b>(
        ctx: &'a ParseContext,
        subs: &'a mut SubstitutionTable,
        input: IndexStr<'b>,
    ) -> Result<(ClassEnumType, IndexStr<'b>)> {
        try_begin_parse!("ClassEnumType", ctx, input);

        if let Ok((name, tail)) = try_recurse!(Name::parse(ctx, subs, input)) {
            return Ok((ClassEnumType::Named(name), tail));
        }

        let tail = consume(b"T", input)?;

        if let Ok(tail) = consume(b"s", tail) {
            let (name, tail) = Name::parse(ctx, subs, tail)?;
            return Ok((ClassEnumType::ElaboratedStruct(name), tail));
        }

        if let Ok(tail) = consume(b"u", tail) {
            let (name, tail) = Name::parse(ctx, subs, tail)?;
            return Ok((ClassEnumType::ElaboratedUnion(name), tail));
        }

        let tail = consume(b"e", tail)?;
        let (name, tail) = Name::parse(ctx, subs, tail)?;
        Ok((ClassEnumType::ElaboratedEnum(name), tail))
    }
}

impl<'subs, W> Demangle<'subs, W> for ClassEnumType
where
    W: 'subs + DemangleWrite,
{
    fn demangle<'prev, 'ctx>(
        &'subs self,
        ctx: &'ctx mut DemangleContext<'subs, W>,
        scope: Option<ArgScopeStack<'prev, 'subs>>,
    ) -> fmt::Result {
        let ctx = try_begin_demangle!(self, ctx, scope);

        match *self {
            ClassEnumType::Named(ref name) => name.demangle(ctx, scope),
            ClassEnumType::ElaboratedStruct(ref name) => {
                write!(ctx, "class ")?;
                name.demangle(ctx, scope)
            }
            ClassEnumType::ElaboratedUnion(ref name) => {
                write!(ctx, "union ")?;
                name.demangle(ctx, scope)
            }
            ClassEnumType::ElaboratedEnum(ref name) => {
                write!(ctx, "enum ")?;
                name.demangle(ctx, scope)
            }
        }
    }
}

impl<'a> GetLeafName<'a> for ClassEnumType {
    fn get_leaf_name(&'a self, subs: &'a SubstitutionTable) -> Option<LeafName<'a>> {
        match *self {
            ClassEnumType::Named(ref name)
            | ClassEnumType::ElaboratedStruct(ref name)
            | ClassEnumType::ElaboratedUnion(ref name)
            | ClassEnumType::ElaboratedEnum(ref name) => name.get_leaf_name(subs),
        }
    }
}

/// The `<unnamed-type-name>` production.
///
/// ```text
/// <unnamed-type-name> ::= Ut [ <nonnegative number> ] _
///                     ::= <closure-type-name>
///     ```
///
/// We handle `<closure-type-name>` separately.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UnnamedTypeName(Option<usize>);

impl Parse for UnnamedTypeName {
    fn parse<'a, 'b>(
        ctx: &'a ParseContext,
        _subs: &'a mut SubstitutionTable,
        input: IndexStr<'b>,
    ) -> Result<(UnnamedTypeName, IndexStr<'b>)> {
        try_begin_parse!("UnnamedTypeName", ctx, input);

        let input = consume(b"Ut", input)?;
        let (number, input) = match parse_number(10, false, input) {
            Ok((number, input)) => (Some(number as _), input),
            Err(_) => (None, input),
        };
        let input = consume(b"_", input)?;
        Ok((UnnamedTypeName(number), input))
    }
}

impl UnnamedTypeName {
    #[inline]
    fn starts_with(byte: u8) -> bool {
        byte == b'U'
    }
}

impl<'subs, W> Demangle<'subs, W> for UnnamedTypeName
where
    W: 'subs + DemangleWrite,
{
    fn demangle<'prev, 'ctx>(
        &'subs self,
        ctx: &'ctx mut DemangleContext<'subs, W>,
        scope: Option<ArgScopeStack<'prev, 'subs>>,
    ) -> fmt::Result {
        let ctx = try_begin_demangle!(self, ctx, scope);

        write!(ctx, "{{unnamed type#{}}}", self.0.map_or(1, |n| n + 1))?;
        Ok(())
    }
}

impl<'subs, W> DemangleAsLeaf<'subs, W> for UnnamedTypeName
where
    W: 'subs + DemangleWrite,
{
    fn demangle_as_leaf<'me, 'ctx>(
        &'me self,
        ctx: &'ctx mut DemangleContext<'subs, W>,
    ) -> fmt::Result {
        let ctx = try_begin_demangle!(self, ctx, None);
        if let Some(source_name) = ctx.source_name {
            write!(ctx, "{}", source_name)?;
        } else {
            write!(ctx, "{{unnamed type#{}}}", self.0.map_or(1, |n| n + 1))?;
        }
        Ok(())
    }
}

impl<'subs> ArgScope<'subs, 'subs> for UnnamedTypeName {
    fn leaf_name(&'subs self) -> Result<LeafName<'subs>> {
        Ok(LeafName::UnnamedType(self))
    }

    fn get_template_arg(
        &'subs self,
        _: usize,
    ) -> Result<(&'subs TemplateArg, &'subs TemplateArgs)> {
        Err(error::Error::BadTemplateArgReference)
    }

    fn get_function_arg(&'subs self, _: usize) -> Result<&'subs Type> {
        Err(error::Error::BadFunctionArgReference)
    }
}

/// The `<array-type>` production.
///
/// ```text
/// <array-type> ::= A <positive dimension number> _ <element type>
///              ::= A [<dimension expression>] _ <element type>
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ArrayType {
    /// An array with a number-literal dimension.
    DimensionNumber(usize, TypeHandle),

    /// An array with an expression for its dimension.
    DimensionExpression(Expression, TypeHandle),

    /// An array with no dimension.
    NoDimension(TypeHandle),
}

impl Parse for ArrayType {
    fn parse<'a, 'b>(
        ctx: &'a ParseContext,
        subs: &'a mut SubstitutionTable,
        input: IndexStr<'b>,
    ) -> Result<(ArrayType, IndexStr<'b>)> {
        try_begin_parse!("ArrayType", ctx, input);

        let tail = consume(b"A", input)?;

        if let Ok((num, tail)) = parse_number(10, false, tail) {
            debug_assert!(num >= 0);
            let tail = consume(b"_", tail)?;
            let (ty, tail) = TypeHandle::parse(ctx, subs, tail)?;
            return Ok((ArrayType::DimensionNumber(num as _, ty), tail));
        }

        if let Ok((expr, tail)) = try_recurse!(Expression::parse(ctx, subs, tail)) {
            let tail = consume(b"_", tail)?;
            let (ty, tail) = TypeHandle::parse(ctx, subs, tail)?;
            return Ok((ArrayType::DimensionExpression(expr, ty), tail));
        }

        let tail = consume(b"_", tail)?;
        let (ty, tail) = TypeHandle::parse(ctx, subs, tail)?;
        Ok((ArrayType::NoDimension(ty), tail))
    }
}

impl<'subs, W> Demangle<'subs, W> for ArrayType
where
    W: 'subs + DemangleWrite,
{
    fn demangle<'prev, 'ctx>(
        &'subs self,
        ctx: &'ctx mut DemangleContext<'subs, W>,
        scope: Option<ArgScopeStack<'prev, 'subs>>,
    ) -> fmt::Result {
        let ctx = try_begin_demangle!(self, ctx, scope);

        ctx.push_inner(self);

        match *self {
            ArrayType::DimensionNumber(_, ref ty)
            | ArrayType::DimensionExpression(_, ref ty)
            | ArrayType::NoDimension(ref ty) => {
                ty.demangle(ctx, scope)?;
            }
        }

        if ctx.pop_inner_if(self) {
            self.demangle_as_inner(ctx, scope)?;
        }

        Ok(())
    }
}

impl<'subs, W> DemangleAsInner<'subs, W> for ArrayType
where
    W: 'subs + DemangleWrite,
{
    fn demangle_as_inner<'prev, 'ctx>(
        &'subs self,
        ctx: &'ctx mut DemangleContext<'subs, W>,
        scope: Option<ArgScopeStack<'prev, 'subs>>,
    ) -> fmt::Result {
        let ctx = try_begin_demangle_as_inner!(self, ctx, scope);

        // Whether we should add a final space before the dimensions.
        let mut needs_space = true;

        while let Some(inner) = ctx.pop_inner() {
            // We need to add parentheses around array inner types, unless they
            // are also (potentially qualified) arrays themselves, in which case
            // we format them as multi-dimensional arrays.
            let inner_is_array = match inner.downcast_to_type() {
                Some(&Type::Qualified(_, ref ty)) => ctx.subs.get_type(ty).map_or(false, |ty| {
                    DemangleAsInner::<W>::downcast_to_array_type(ty).is_some()
                }),
                _ => {
                    if inner.downcast_to_array_type().is_some() {
                        needs_space = false;
                        true
                    } else {
                        false
                    }
                }
            };

            if inner_is_array {
                inner.demangle_as_inner(ctx, scope)?;
            } else {
                ctx.ensure_space()?;

                // CvQualifiers should have the parentheses printed after, not before
                if inner.is_qualified() {
                    inner.demangle_as_inner(ctx, scope)?;
                    ctx.ensure_space()?;
                    write!(ctx, "(")?;
                } else {
                    write!(ctx, "(")?;
                    inner.demangle_as_inner(ctx, scope)?;
                }

                ctx.demangle_inners(scope)?;
                write!(ctx, ")")?;
            }
        }

        if needs_space {
            ctx.ensure_space()?;
        }

        match *self {
            ArrayType::DimensionNumber(n, _) => {
                write!(ctx, "[{}]", n)?;
            }
            ArrayType::DimensionExpression(ref expr, _) => {
                write!(ctx, "[")?;
                expr.demangle(ctx, scope)?;
                write!(ctx, "]")?;
            }
            ArrayType::NoDimension(_) => {
                write!(ctx, "[]")?;
            }
        }

        Ok(())
    }

    fn downcast_to_array_type(&self) -> Option<&ArrayType> {
        Some(self)
    }
}

/// The `<vector-type>` production.
///
/// ```text
/// <vector-type> ::= Dv <number> _ <type>
///               ::= Dv <expression> _ <type>
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum VectorType {
    /// An vector with a number-literal dimension.
    DimensionNumber(usize, TypeHandle),

    /// An vector with an expression for its dimension.
    DimensionExpression(Expression, TypeHandle),
}

impl Parse for VectorType {
    fn parse<'a, 'b>(
        ctx: &'a ParseContext,
        subs: &'a mut SubstitutionTable,
        input: IndexStr<'b>,
    ) -> Result<(VectorType, IndexStr<'b>)> {
        try_begin_parse!("VectorType", ctx, input);

        let tail = consume(b"Dv", input)?;

        if let Ok((num, tail)) = parse_number(10, false, tail) {
            debug_assert!(num >= 0);
            let tail = consume(b"_", tail)?;
            let (ty, tail) = TypeHandle::parse(ctx, subs, tail)?;
            return Ok((VectorType::DimensionNumber(num as _, ty), tail));
        }

        let (expr, tail) = Expression::parse(ctx, subs, tail)?;
        let tail = consume(b"_", tail)?;
        let (ty, tail) = TypeHandle::parse(ctx, subs, tail)?;
        Ok((VectorType::DimensionExpression(expr, ty), tail))
    }
}

impl<'subs, W> Demangle<'subs, W> for VectorType
where
    W: 'subs + DemangleWrite,
{
    fn demangle<'prev, 'ctx>(
        &'subs self,
        ctx: &'ctx mut DemangleContext<'subs, W>,
        scope: Option<ArgScopeStack<'prev, 'subs>>,
    ) -> fmt::Result {
        let ctx = try_begin_demangle!(self, ctx, scope);

        ctx.push_inner(self);

        match *self {
            VectorType::DimensionNumber(_, ref ty) | VectorType::DimensionExpression(_, ref ty) => {
                ty.demangle(ctx, scope)?;
            }
        }

        if ctx.pop_inner_if(self) {
            self.demangle_as_inner(ctx, scope)?;
        }

        Ok(())
    }
}

impl<'subs, W> DemangleAsInner<'subs, W> for VectorType
where
    W: 'subs + DemangleWrite,
{
    fn demangle_as_inner<'prev, 'ctx>(
        &'subs self,
        ctx: &'ctx mut DemangleContext<'subs, W>,
        scope: Option<ArgScopeStack<'prev, 'subs>>,
    ) -> fmt::Result {
        let ctx = try_begin_demangle_as_inner!(self, ctx, scope);

        match *self {
            VectorType::DimensionNumber(n, _) => {
                write!(ctx, " __vector({})", n)?;
            }
            VectorType::DimensionExpression(ref expr, _) => {
                write!(ctx, " __vector(")?;
                expr.demangle(ctx, scope)?;
                write!(ctx, ")")?;
            }
        }

        Ok(())
    }
}

/// The `<pointer-to-member-type>` production.
///
/// ```text
/// <pointer-to-member-type> ::= M <class type> <member type>
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PointerToMemberType(TypeHandle, TypeHandle);

impl Parse for PointerToMemberType {
    fn parse<'a, 'b>(
        ctx: &'a ParseContext,
        subs: &'a mut SubstitutionTable,
        input: IndexStr<'b>,
    ) -> Result<(PointerToMemberType, IndexStr<'b>)> {
        try_begin_parse!("PointerToMemberType", ctx, input);

        let tail = consume(b"M", input)?;
        let (ty1, tail) = TypeHandle::parse(ctx, subs, tail)?;
        let (ty2, tail) = TypeHandle::parse(ctx, subs, tail)?;
        Ok((PointerToMemberType(ty1, ty2), tail))
    }
}

impl<'subs, W> Demangle<'subs, W> for PointerToMemberType
where
    W: 'subs + DemangleWrite,
{
    fn demangle<'prev, 'ctx>(
        &'subs self,
        ctx: &'ctx mut DemangleContext<'subs, W>,
        scope: Option<ArgScopeStack<'prev, 'subs>>,
    ) -> fmt::Result {
        let ctx = try_begin_demangle!(self, ctx, scope);

        ctx.push_inner(self);
        self.1.demangle(ctx, scope)?;
        if ctx.pop_inner_if(self) {
            self.demangle_as_inner(ctx, scope)?;
        }
        Ok(())
    }
}

impl<'subs, W> DemangleAsInner<'subs, W> for PointerToMemberType
where
    W: 'subs + DemangleWrite,
{
    fn demangle_as_inner<'prev, 'ctx>(
        &'subs self,
        ctx: &'ctx mut DemangleContext<'subs, W>,
        scope: Option<ArgScopeStack<'prev, 'subs>>,
    ) -> fmt::Result {
        let ctx = try_begin_demangle_as_inner!(self, ctx, scope);

        if ctx.last_char_written != Some('(') {
            ctx.ensure_space()?;
        }

        self.0.demangle(ctx, scope)?;
        write!(ctx, "::*")?;
        Ok(())
    }

    fn downcast_to_pointer_to_member(&self) -> Option<&PointerToMemberType> {
        Some(self)
    }
}

/// The `<template-param>` production.
///
/// ```text
/// <template-param> ::= T_ # first template parameter
///                  ::= T <parameter-2 non-negative number> _
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TemplateParam(usize);

impl Parse for TemplateParam {
    fn parse<'a, 'b>(
        ctx: &'a ParseContext,
        _subs: &'a mut SubstitutionTable,
        input: IndexStr<'b>,
    ) -> Result<(TemplateParam, IndexStr<'b>)> {
        try_begin_parse!("TemplateParam", ctx, input);

        let input = consume(b"T", input)?;
        let (number, input) = match parse_number(10, false, input) {
            Ok((number, input)) => ((number + 1) as _, input),
            Err(_) => (0, input),
        };
        let input = consume(b"_", input)?;
        Ok((TemplateParam(number), input))
    }
}

impl<'subs, W> Demangle<'subs, W> for TemplateParam
where
    W: 'subs + DemangleWrite,
{
    fn demangle<'prev, 'ctx>(
        &'subs self,
        ctx: &'ctx mut DemangleContext<'subs, W>,
        scope: Option<ArgScopeStack<'prev, 'subs>>,
    ) -> fmt::Result {
        let ctx = try_begin_demangle!(self, ctx, scope);

        ctx.push_demangle_node(DemangleNodeType::TemplateParam);
        let ret = if ctx.is_lambda_arg {
            // To match libiberty, template references are converted to `auto`.
            write!(ctx, "auto:{}", self.0 + 1)
        } else {
            let arg = self.resolve(scope)?;
            arg.demangle(ctx, scope)
        };
        ctx.pop_demangle_node();
        ret
    }
}

impl TemplateParam {
    fn resolve<'subs, 'prev>(
        &'subs self,
        scope: Option<ArgScopeStack<'prev, 'subs>>,
    ) -> ::core::result::Result<&'subs TemplateArg, fmt::Error> {
        scope
            .get_template_arg(self.0)
            .map_err(|e| {
                log!("Error obtaining template argument: {}", e);
                fmt::Error
            })
            .map(|v| v.0)
    }
}

impl<'a> Hash for &'a TemplateParam {
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        let self_ref: &TemplateParam = *self;
        let self_ptr = self_ref as *const TemplateParam;
        self_ptr.hash(state);
    }
}

/// The `<template-template-param>` production.
///
/// ```text
/// <template-template-param> ::= <template-param>
///                           ::= <substitution>
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TemplateTemplateParam(TemplateParam);

define_handle! {
    /// A reference to a parsed `TemplateTemplateParam`.
    pub enum TemplateTemplateParamHandle
}

impl Parse for TemplateTemplateParamHandle {
    fn parse<'a, 'b>(
        ctx: &'a ParseContext,
        subs: &'a mut SubstitutionTable,
        input: IndexStr<'b>,
    ) -> Result<(TemplateTemplateParamHandle, IndexStr<'b>)> {
        try_begin_parse!("TemplateTemplateParamHandle", ctx, input);

        if let Ok((sub, tail)) = try_recurse!(Substitution::parse(ctx, subs, input)) {
            match sub {
                Substitution::WellKnown(component) => {
                    return Ok((TemplateTemplateParamHandle::WellKnown(component), tail));
                }
                Substitution::BackReference(idx) => {
                    // TODO: should this check if the thing at idx is a
                    // template-template-param? There could otherwise be ambiguity
                    // with <type>'s <substitution> form...
                    return Ok((TemplateTemplateParamHandle::BackReference(idx), tail));
                }
            }
        }

        let (param, tail) = TemplateParam::parse(ctx, subs, input)?;
        let ttp = TemplateTemplateParam(param);
        let ttp = Substitutable::TemplateTemplateParam(ttp);
        let idx = subs.insert(ttp);
        let handle = TemplateTemplateParamHandle::BackReference(idx);
        Ok((handle, tail))
    }
}

impl<'subs, W> Demangle<'subs, W> for TemplateTemplateParam
where
    W: 'subs + DemangleWrite,
{
    #[inline]
    fn demangle<'prev, 'ctx>(
        &'subs self,
        ctx: &'ctx mut DemangleContext<'subs, W>,
        scope: Option<ArgScopeStack<'prev, 'subs>>,
    ) -> fmt::Result {
        let ctx = try_begin_demangle!(self, ctx, scope);

        self.0.demangle(ctx, scope)
    }
}

/// The <function-param> production.
///
/// ```text
/// <function-param> ::= fp <top-level CV-qualifiers> _
///                          # L == 0, first parameter
///                  ::= fp <top-level CV-qualifiers> <parameter-2 non-negative number> _
///                          # L == 0, second and later parameters
///                  ::= fL <L-1 non-negative number> p <top-level CV-qualifiers> _
///                          # L > 0, first parameter
///                  ::= fL <L-1 non-negative number> p <top-level CV-qualifiers> <parameter-2 non-negative number> _
///                          # L > 0, second and later parameters
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FunctionParam(usize, CvQualifiers, Option<usize>);

impl Parse for FunctionParam {
    fn parse<'a, 'b>(
        ctx: &'a ParseContext,
        subs: &'a mut SubstitutionTable,
        input: IndexStr<'b>,
    ) -> Result<(FunctionParam, IndexStr<'b>)> {
        try_begin_parse!("FunctionParam", ctx, input);

        let tail = consume(b"f", input)?;
        if tail.is_empty() {
            return Err(error::Error::UnexpectedEnd);
        }

        let (scope, tail) = if let Ok(tail) = consume(b"L", tail) {
            parse_number(10, false, tail)?
        } else {
            (0, tail)
        };

        let tail = consume(b"p", tail)?;

        let (qualifiers, tail) = CvQualifiers::parse(ctx, subs, tail)?;

        let (param, tail) = if tail.peek() == Some(b'T') {
            (None, consume(b"T", tail)?)
        } else if let Ok((num, tail)) = parse_number(10, false, tail) {
            (Some(num as usize + 1), consume(b"_", tail)?)
        } else {
            (Some(0), consume(b"_", tail)?)
        };

        Ok((FunctionParam(scope as _, qualifiers, param), tail))
    }
}

impl<'subs, W> Demangle<'subs, W> for FunctionParam
where
    W: 'subs + DemangleWrite,
{
    fn demangle<'prev, 'ctx>(
        &'subs self,
        ctx: &'ctx mut DemangleContext<'subs, W>,
        scope: Option<ArgScopeStack<'prev, 'subs>>,
    ) -> fmt::Result {
        let ctx = try_begin_demangle!(self, ctx, scope);

        match self.2 {
            None => write!(ctx, "this"),
            Some(i) => write!(ctx, "{{parm#{}}}", i + 1),
        }
    }
}

/// The `<template-args>` production.
///
/// ```text
/// <template-args> ::= I <template-arg>+ E
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TemplateArgs(Vec<TemplateArg>);

impl Parse for TemplateArgs {
    fn parse<'a, 'b>(
        ctx: &'a ParseContext,
        subs: &'a mut SubstitutionTable,
        input: IndexStr<'b>,
    ) -> Result<(TemplateArgs, IndexStr<'b>)> {
        try_begin_parse!("TemplateArgs", ctx, input);

        let tail = consume(b"I", input)?;

        let (args, tail) = one_or_more::<TemplateArg>(ctx, subs, tail)?;
        let tail = consume(b"E", tail)?;
        Ok((TemplateArgs(args), tail))
    }
}

impl<'subs, W> Demangle<'subs, W> for TemplateArgs
where
    W: 'subs + DemangleWrite,
{
    fn demangle<'prev, 'ctx>(
        &'subs self,
        ctx: &'ctx mut DemangleContext<'subs, W>,
        mut scope: Option<ArgScopeStack<'prev, 'subs>>,
    ) -> fmt::Result {
        let ctx = try_begin_demangle!(self, ctx, scope);
        inner_barrier!(ctx);

        if ctx.last_char_written == Some('<') {
            write!(ctx, " ")?;
        }
        write!(ctx, "<")?;
        ctx.push_demangle_node(DemangleNodeType::TemplateArgs);
        let mut need_comma = false;
        for arg_index in 0..self.0.len() {
            if need_comma {
                write!(ctx, ", ")?;
            }
            if let Some(ref mut scope) = scope {
                scope.in_arg = Some((arg_index, self));
            }
            self.0[arg_index].demangle(ctx, scope)?;
            need_comma = true;
        }

        // Ensure "> >" because old C++ sucks and libiberty (and its tests)
        // supports old C++.
        if ctx.last_char_written == Some('>') {
            write!(ctx, " ")?;
        }
        ctx.pop_demangle_node();
        write!(ctx, ">")?;
        Ok(())
    }
}

impl<'subs> ArgScope<'subs, 'subs> for TemplateArgs {
    fn leaf_name(&'subs self) -> Result<LeafName<'subs>> {
        Err(error::Error::BadLeafNameReference)
    }

    fn get_template_arg(
        &'subs self,
        idx: usize,
    ) -> Result<(&'subs TemplateArg, &'subs TemplateArgs)> {
        self.0
            .get(idx)
            .ok_or(error::Error::BadTemplateArgReference)
            .map(|v| (v, self))
    }

    fn get_function_arg(&'subs self, _: usize) -> Result<&'subs Type> {
        Err(error::Error::BadFunctionArgReference)
    }
}

/// A <template-arg> production.
///
/// ```text
/// <template-arg> ::= <type>                # type or template
///                ::= X <expression> E      # expression
///                ::= <expr-primary>        # simple expressions
///                ::= J <template-arg>* E   # argument pack
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TemplateArg {
    /// A type or template.
    Type(TypeHandle),

    /// An expression.
    Expression(Expression),

    /// A simple expression.
    SimpleExpression(ExprPrimary),

    /// An argument pack.
    ArgPack(Vec<TemplateArg>),
}

impl Parse for TemplateArg {
    fn parse<'a, 'b>(
        ctx: &'a ParseContext,
        subs: &'a mut SubstitutionTable,
        input: IndexStr<'b>,
    ) -> Result<(TemplateArg, IndexStr<'b>)> {
        try_begin_parse!("TemplateArg", ctx, input);

        if let Ok(tail) = consume(b"X", input) {
            let (expr, tail) = Expression::parse(ctx, subs, tail)?;
            let tail = consume(b"E", tail)?;
            return Ok((TemplateArg::Expression(expr), tail));
        }

        if let Ok((expr, tail)) = try_recurse!(ExprPrimary::parse(ctx, subs, input)) {
            return Ok((TemplateArg::SimpleExpression(expr), tail));
        }

        if let Ok((ty, tail)) = try_recurse!(TypeHandle::parse(ctx, subs, input)) {
            return Ok((TemplateArg::Type(ty), tail));
        }

        let tail = if input.peek() == Some(b'J') {
            consume(b"J", input)?
        } else {
            consume(b"I", input)?
        };

        let (args, tail) = if tail.peek() == Some(b'E') {
            (vec![], tail)
        } else {
            zero_or_more::<TemplateArg>(ctx, subs, tail)?
        };
        let tail = consume(b"E", tail)?;
        Ok((TemplateArg::ArgPack(args), tail))
    }
}

impl<'subs, W> Demangle<'subs, W> for TemplateArg
where
    W: 'subs + DemangleWrite,
{
    fn demangle<'prev, 'ctx>(
        &'subs self,
        ctx: &'ctx mut DemangleContext<'subs, W>,
        scope: Option<ArgScopeStack<'prev, 'subs>>,
    ) -> fmt::Result {
        let ctx = try_begin_demangle!(self, ctx, scope);

        match *self {
            TemplateArg::Type(ref ty) => ty.demangle(ctx, scope),
            TemplateArg::Expression(ref expr) => expr.demangle(ctx, scope),
            TemplateArg::SimpleExpression(ref expr) => expr.demangle(ctx, scope),
            TemplateArg::ArgPack(ref args) => {
                ctx.is_template_argument_pack = true;
                let mut need_comma = false;
                for arg in &args[..] {
                    if need_comma {
                        write!(ctx, ", ")?;
                    }
                    arg.demangle(ctx, scope)?;
                    need_comma = true;
                }
                Ok(())
            }
        }
    }
}

/// In libiberty, Member and DerefMember expressions have special handling.
/// They parse an `UnqualifiedName` (not an `UnscopedName` as the cxxabi docs
/// say) and optionally a `TemplateArgs` if it is present. We can't just parse
/// a `Name` or an `UnscopedTemplateName` here because that allows other inputs
/// that libiberty does not.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MemberName(Name);

impl Parse for MemberName {
    fn parse<'a, 'b>(
        ctx: &'a ParseContext,
        subs: &'a mut SubstitutionTable,
        input: IndexStr<'b>,
    ) -> Result<(MemberName, IndexStr<'b>)> {
        try_begin_parse!("MemberName", ctx, input);

        let (name, tail) = UnqualifiedName::parse(ctx, subs, input)?;
        let name = UnscopedName::Unqualified(name);
        if let Ok((template, tail)) = try_recurse!(TemplateArgs::parse(ctx, subs, tail)) {
            let name = UnscopedTemplateName(name);
            // In libiberty, these are unsubstitutable.
            let idx = subs.insert_non_substitution(Substitutable::UnscopedTemplateName(name));
            let handle = UnscopedTemplateNameHandle::NonSubstitution(NonSubstitution(idx));
            Ok((MemberName(Name::UnscopedTemplate(handle, template)), tail))
        } else {
            Ok((MemberName(Name::Unscoped(name)), tail))
        }
    }
}

impl<'subs, W> Demangle<'subs, W> for MemberName
where
    W: 'subs + DemangleWrite,
{
    fn demangle<'prev, 'ctx>(
        &'subs self,
        ctx: &'ctx mut DemangleContext<'subs, W>,
        scope: Option<ArgScopeStack<'prev, 'subs>>,
    ) -> fmt::Result {
        let ctx = try_begin_demangle!(self, ctx, scope);

        let needs_parens = self.0.get_template_args(ctx.subs).is_some();
        if needs_parens {
            write!(ctx, "(")?;
        }

        self.0.demangle(ctx, scope)?;

        if needs_parens {
            write!(ctx, ")")?;
        }

        Ok(())
    }
}

/// The `<expression>` production.
///
/// ```text
///  <expression> ::= <unary operator-name> <expression>
///               ::= <binary operator-name> <expression> <expression>
///               ::= <ternary operator-name> <expression> <expression> <expression>
///               ::= pp_ <expression>                             # prefix ++
///               ::= mm_ <expression>                             # prefix --
///               ::= cl <expression>+ E                           # expression (expr-list), call
///               ::= cv <type> <expression>                       # type (expression), conversion with one argument
///               ::= cv <type> _ <expression>* E                  # type (expr-list), conversion with other than one argument
///               ::= tl <type> <expression>* E                    # type {expr-list}, conversion with braced-init-list argument
///               ::= il <expression> E                            # {expr-list}, braced-init-list in any other context
///               ::= [gs] nw <expression>* _ <type> E             # new (expr-list) type
///               ::= [gs] nw <expression>* _ <type> <initializer> # new (expr-list) type (init)
///               ::= [gs] na <expression>* _ <type> E             # new[] (expr-list) type
///               ::= [gs] na <expression>* _ <type> <initializer> # new[] (expr-list) type (init)
///               ::= [gs] dl <expression>                         # delete expression
///               ::= [gs] da <expression>                         # delete[] expression
///               ::= dc <type> <expression>                       # dynamic_cast<type> (expression)
///               ::= sc <type> <expression>                       # static_cast<type> (expression)
///               ::= cc <type> <expression>                       # const_cast<type> (expression)
///               ::= rc <type> <expression>                       # reinterpret_cast<type> (expression)
///               ::= ti <type>                                    # typeid (type)
///               ::= te <expression>                              # typeid (expression)
///               ::= st <type>                                    # sizeof (type)
///               ::= sz <expression>                              # sizeof (expression)
///               ::= at <type>                                    # alignof (type)
///               ::= az <expression>                              # alignof (expression)
///               ::= nx <expression>                              # noexcept (expression)
///               ::= so <subobject-expr>
///               ::= <template-param>
///               ::= <function-param>
///               ::= dt <expression> <unresolved-name>            # expr.name
///               ::= pt <expression> <unresolved-name>            # expr->name
///               ::= ds <expression> <expression>                 # expr.*expr
///               ::= sZ <template-param>                          # sizeof...(T), size of a template parameter pack
///               ::= sZ <function-param>                          # sizeof...(parameter), size of a function parameter pack
///               ::= sP <template-arg>* E                         # sizeof...(T), size of a captured template parameter pack from an alias template
///               ::= sp <expression>                              # expression..., pack expansion
///               ::= tw <expression>                              # throw expression
///               ::= tr                                           # throw with no operand (rethrow)
///               ::= <unresolved-name>                            # f(p), N::f(p), ::f(p),
///                                                                # freestanding dependent name (e.g., T::x),
///                                                                # objectless nonstatic member reference
///               ::= <expr-primary>
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Expression {
    /// A unary operator expression.
    Unary(OperatorName, Box<Expression>),

    /// A binary operator expression.
    Binary(OperatorName, Box<Expression>, Box<Expression>),

    /// A ternary operator expression.
    Ternary(
        OperatorName,
        Box<Expression>,
        Box<Expression>,
        Box<Expression>,
    ),

    /// A prefix `++`.
    PrefixInc(Box<Expression>),

    /// A prefix `--`.
    PrefixDec(Box<Expression>),

    /// A call with functor and arguments.
    Call(Box<Expression>, Vec<Expression>),

    /// A type conversion with one argument.
    ConversionOne(TypeHandle, Box<Expression>),

    /// A type conversion with many arguments.
    ConversionMany(TypeHandle, Vec<Expression>),

    /// A type conversion with many arguments.
    ConversionBraced(TypeHandle, Vec<Expression>),

    /// A braced init list expression.
    BracedInitList(Box<Expression>),

    /// The `new` operator.
    New(Vec<Expression>, TypeHandle, Option<Initializer>),

    /// The global `::new` operator.
    GlobalNew(Vec<Expression>, TypeHandle, Option<Initializer>),

    /// The `new[]` operator.
    NewArray(Vec<Expression>, TypeHandle, Option<Initializer>),

    /// The global `::new[]` operator.
    GlobalNewArray(Vec<Expression>, TypeHandle, Option<Initializer>),

    /// The `delete` operator.
    Delete(Box<Expression>),

    /// The global `::delete` operator.
    GlobalDelete(Box<Expression>),

    /// The `delete[]` operator.
    DeleteArray(Box<Expression>),

    /// The global `::delete[]` operator.
    GlobalDeleteArray(Box<Expression>),

    /// `dynamic_cast<type> (expression)`
    DynamicCast(TypeHandle, Box<Expression>),

    /// `static_cast<type> (expression)`
    StaticCast(TypeHandle, Box<Expression>),

    /// `const_cast<type> (expression)`
    ConstCast(TypeHandle, Box<Expression>),

    /// `reinterpret_cast<type> (expression)`
    ReinterpretCast(TypeHandle, Box<Expression>),

    /// `typeid (type)`
    TypeidType(TypeHandle),

    /// `typeid (expression)`
    TypeidExpr(Box<Expression>),

    /// `sizeof (type)`
    SizeofType(TypeHandle),

    /// `sizeof (expression)`
    SizeofExpr(Box<Expression>),

    /// `alignof (type)`
    AlignofType(TypeHandle),

    /// `alignof (expression)`
    AlignofExpr(Box<Expression>),

    /// `noexcept (expression)`
    Noexcept(Box<Expression>),

    /// Subobject expression,
    Subobject(SubobjectExpr),

    /// A named template parameter.
    TemplateParam(TemplateParam),

    /// A function parameter.
    FunctionParam(FunctionParam),

    /// `expr.name`
    Member(Box<Expression>, MemberName),

    /// `expr->name`
    DerefMember(Box<Expression>, MemberName),

    /// `expr.*expr`
    PointerToMember(Box<Expression>, Box<Expression>),

    /// `sizeof...(T)`, size of a template parameter pack.
    SizeofTemplatePack(TemplateParam),

    /// `sizeof...(parameter)`, size of a function parameter pack.
    SizeofFunctionPack(FunctionParam),

    /// `sizeof...(T)`, size of a captured template parameter pack from an alias
    /// template.
    SizeofCapturedTemplatePack(Vec<TemplateArg>),

    /// `expression...`, pack expansion.
    PackExpansion(Box<Expression>),

    /// `throw expression`
    Throw(Box<Expression>),

    /// `throw` with no operand
    Rethrow,

    /// `f(p)`, `N::f(p)`, `::f(p)`, freestanding dependent name (e.g., `T::x`),
    /// objectless nonstatic member reference.
    UnresolvedName(UnresolvedName),

    /// An `<expr-primary>` production.
    Primary(ExprPrimary),
}

impl Parse for Expression {
    fn parse<'a, 'b>(
        ctx: &'a ParseContext,
        subs: &'a mut SubstitutionTable,
        input: IndexStr<'b>,
    ) -> Result<(Expression, IndexStr<'b>)> {
        try_begin_parse!("Expression", ctx, input);

        if let Ok(tail) = consume(b"pp_", input) {
            let (expr, tail) = Expression::parse(ctx, subs, tail)?;
            let expr = Expression::PrefixInc(Box::new(expr));
            return Ok((expr, tail));
        }

        if let Ok(tail) = consume(b"mm_", input) {
            let (expr, tail) = Expression::parse(ctx, subs, tail)?;
            let expr = Expression::PrefixDec(Box::new(expr));
            return Ok((expr, tail));
        }

        if let Some((head, tail)) = input.try_split_at(2) {
            match head.as_ref() {
                b"cl" => {
                    let (func, tail) = Expression::parse(ctx, subs, tail)?;
                    let (args, tail) = zero_or_more::<Expression>(ctx, subs, tail)?;
                    let tail = consume(b"E", tail)?;
                    let expr = Expression::Call(Box::new(func), args);
                    return Ok((expr, tail));
                }
                b"cv" => {
                    let (ty, tail) = TypeHandle::parse(ctx, subs, tail)?;
                    if let Ok(tail) = consume(b"_", tail) {
                        let (exprs, tail) = zero_or_more::<Expression>(ctx, subs, tail)?;
                        let tail = consume(b"E", tail)?;
                        let expr = Expression::ConversionMany(ty, exprs);
                        return Ok((expr, tail));
                    } else {
                        let (expr, tail) = Expression::parse(ctx, subs, tail)?;
                        let expr = Expression::ConversionOne(ty, Box::new(expr));
                        return Ok((expr, tail));
                    }
                }
                b"tl" => {
                    let (ty, tail) = TypeHandle::parse(ctx, subs, tail)?;
                    let (exprs, tail) = zero_or_more::<Expression>(ctx, subs, tail)?;
                    let expr = Expression::ConversionBraced(ty, exprs);
                    let tail = consume(b"E", tail)?;
                    return Ok((expr, tail));
                }
                b"il" => {
                    let (expr, tail) = Expression::parse(ctx, subs, tail)?;
                    let tail = consume(b"E", tail)?;
                    let expr = Expression::BracedInitList(Box::new(expr));
                    return Ok((expr, tail));
                }
                b"dc" => {
                    let (ty, tail) = TypeHandle::parse(ctx, subs, tail)?;
                    let (expr, tail) = Expression::parse(ctx, subs, tail)?;
                    let expr = Expression::DynamicCast(ty, Box::new(expr));
                    return Ok((expr, tail));
                }
                b"sc" => {
                    let (ty, tail) = TypeHandle::parse(ctx, subs, tail)?;
                    let (expr, tail) = Expression::parse(ctx, subs, tail)?;
                    let expr = Expression::StaticCast(ty, Box::new(expr));
                    return Ok((expr, tail));
                }
                b"cc" => {
                    let (ty, tail) = TypeHandle::parse(ctx, subs, tail)?;
                    let (expr, tail) = Expression::parse(ctx, subs, tail)?;
                    let expr = Expression::ConstCast(ty, Box::new(expr));
                    return Ok((expr, tail));
                }
                b"rc" => {
                    let (ty, tail) = TypeHandle::parse(ctx, subs, tail)?;
                    let (expr, tail) = Expression::parse(ctx, subs, tail)?;
                    let expr = Expression::ReinterpretCast(ty, Box::new(expr));
                    return Ok((expr, tail));
                }
                b"ti" => {
                    let (ty, tail) = TypeHandle::parse(ctx, subs, tail)?;
                    let expr = Expression::TypeidType(ty);
                    return Ok((expr, tail));
                }
                b"te" => {
                    let (expr, tail) = Expression::parse(ctx, subs, tail)?;
                    let expr = Expression::TypeidExpr(Box::new(expr));
                    return Ok((expr, tail));
                }
                b"st" => {
                    let (ty, tail) = TypeHandle::parse(ctx, subs, tail)?;
                    let expr = Expression::SizeofType(ty);
                    return Ok((expr, tail));
                }
                b"sz" => {
                    let (expr, tail) = Expression::parse(ctx, subs, tail)?;
                    let expr = Expression::SizeofExpr(Box::new(expr));
                    return Ok((expr, tail));
                }
                b"at" => {
                    let (ty, tail) = TypeHandle::parse(ctx, subs, tail)?;
                    let expr = Expression::AlignofType(ty);
                    return Ok((expr, tail));
                }
                b"az" => {
                    let (expr, tail) = Expression::parse(ctx, subs, tail)?;
                    let expr = Expression::AlignofExpr(Box::new(expr));
                    return Ok((expr, tail));
                }
                b"nx" => {
                    let (expr, tail) = Expression::parse(ctx, subs, tail)?;
                    let expr = Expression::Noexcept(Box::new(expr));
                    return Ok((expr, tail));
                }
                b"so" => {
                    let (expr, tail) = SubobjectExpr::parse(ctx, subs, tail)?;
                    let expr = Expression::Subobject(expr);
                    return Ok((expr, tail));
                }
                b"dt" => {
                    let (expr, tail) = Expression::parse(ctx, subs, tail)?;
                    let (name, tail) = MemberName::parse(ctx, subs, tail)?;
                    let expr = Expression::Member(Box::new(expr), name);
                    return Ok((expr, tail));
                }
                b"pt" => {
                    let (expr, tail) = Expression::parse(ctx, subs, tail)?;
                    let (name, tail) = MemberName::parse(ctx, subs, tail)?;
                    let expr = Expression::DerefMember(Box::new(expr), name);
                    return Ok((expr, tail));
                }
                b"ds" => {
                    let (first, tail) = Expression::parse(ctx, subs, tail)?;
                    let (second, tail) = Expression::parse(ctx, subs, tail)?;
                    let expr = Expression::PointerToMember(Box::new(first), Box::new(second));
                    return Ok((expr, tail));
                }
                b"sZ" => {
                    if let Ok((param, tail)) = TemplateParam::parse(ctx, subs, tail) {
                        let expr = Expression::SizeofTemplatePack(param);
                        return Ok((expr, tail));
                    }

                    let (param, tail) = FunctionParam::parse(ctx, subs, tail)?;
                    let expr = Expression::SizeofFunctionPack(param);
                    return Ok((expr, tail));
                }
                b"sP" => {
                    let (args, tail) = zero_or_more::<TemplateArg>(ctx, subs, tail)?;
                    let expr = Expression::SizeofCapturedTemplatePack(args);
                    let tail = consume(b"E", tail)?;
                    return Ok((expr, tail));
                }
                b"sp" => {
                    let (expr, tail) = Expression::parse(ctx, subs, tail)?;
                    let expr = Expression::PackExpansion(Box::new(expr));
                    return Ok((expr, tail));
                }
                b"tw" => {
                    let (expr, tail) = Expression::parse(ctx, subs, tail)?;
                    let expr = Expression::Throw(Box::new(expr));
                    return Ok((expr, tail));
                }
                b"tr" => {
                    let expr = Expression::Rethrow;
                    return Ok((expr, tail));
                }
                b"gs" => {
                    if let Ok((expr, tail)) = try_recurse!(can_be_global(true, ctx, subs, tail)) {
                        return Ok((expr, tail));
                    }
                }
                _ => {}
            }
        }

        if let Ok((expr, tail)) = try_recurse!(can_be_global(false, ctx, subs, input)) {
            return Ok((expr, tail));
        }

        if let Ok((param, tail)) = try_recurse!(TemplateParam::parse(ctx, subs, input)) {
            let expr = Expression::TemplateParam(param);
            return Ok((expr, tail));
        }

        if let Ok((param, tail)) = try_recurse!(FunctionParam::parse(ctx, subs, input)) {
            let expr = Expression::FunctionParam(param);
            return Ok((expr, tail));
        }

        if let Ok((name, tail)) = try_recurse!(UnresolvedName::parse(ctx, subs, input)) {
            let expr = Expression::UnresolvedName(name);
            return Ok((expr, tail));
        }

        if let Ok((prim, tail)) = try_recurse!(ExprPrimary::parse(ctx, subs, input)) {
            let expr = Expression::Primary(prim);
            return Ok((expr, tail));
        }

        // "A production for <expression> that directly specifies an operation
        // code (e.g., for the -> operator) takes precedence over one that is
        // expressed in terms of (unary/binary/ternary) <operator-name>." So try
        // and parse unary/binary/ternary expressions last.
        let (expr, tail) = OperatorName::parse_from_expr(ctx, subs, input)?;
        return Ok((expr, tail));

        // Parse the various expressions that can optionally have a leading "gs"
        // to indicate that they are in the global namespace. The input is after
        // we have already detected consumed the optional "gs" and if we did
        // find it, then `is_global` should be true.
        fn can_be_global<'a, 'b>(
            is_global: bool,
            ctx: &'a ParseContext,
            subs: &'a mut SubstitutionTable,
            input: IndexStr<'b>,
        ) -> Result<(Expression, IndexStr<'b>)> {
            match input.try_split_at(2) {
                None => Err(error::Error::UnexpectedEnd),
                Some((head, tail)) => match head.as_ref() {
                    b"nw" => {
                        let (exprs, tail) = zero_or_more::<Expression>(ctx, subs, tail)?;
                        let tail = consume(b"_", tail)?;
                        let (ty, tail) = TypeHandle::parse(ctx, subs, tail)?;
                        if let Ok(tail) = consume(b"E", tail) {
                            let expr = if is_global {
                                Expression::GlobalNew(exprs, ty, None)
                            } else {
                                Expression::New(exprs, ty, None)
                            };
                            Ok((expr, tail))
                        } else {
                            let (init, tail) = Initializer::parse(ctx, subs, tail)?;
                            let expr = if is_global {
                                Expression::GlobalNew(exprs, ty, Some(init))
                            } else {
                                Expression::New(exprs, ty, Some(init))
                            };
                            Ok((expr, tail))
                        }
                    }
                    b"na" => {
                        let (exprs, tail) = zero_or_more::<Expression>(ctx, subs, tail)?;
                        let tail = consume(b"_", tail)?;
                        let (ty, tail) = TypeHandle::parse(ctx, subs, tail)?;
                        if let Ok(tail) = consume(b"E", tail) {
                            let expr = if is_global {
                                Expression::GlobalNewArray(exprs, ty, None)
                            } else {
                                Expression::NewArray(exprs, ty, None)
                            };
                            Ok((expr, tail))
                        } else {
                            let (init, tail) = Initializer::parse(ctx, subs, tail)?;
                            let expr = if is_global {
                                Expression::GlobalNewArray(exprs, ty, Some(init))
                            } else {
                                Expression::NewArray(exprs, ty, Some(init))
                            };
                            Ok((expr, tail))
                        }
                    }
                    b"dl" => {
                        let (expr, tail) = Expression::parse(ctx, subs, tail)?;
                        let expr = if is_global {
                            Expression::GlobalDelete(Box::new(expr))
                        } else {
                            Expression::Delete(Box::new(expr))
                        };
                        Ok((expr, tail))
                    }
                    b"da" => {
                        let (expr, tail) = Expression::parse(ctx, subs, tail)?;
                        let expr = if is_global {
                            Expression::GlobalDeleteArray(Box::new(expr))
                        } else {
                            Expression::DeleteArray(Box::new(expr))
                        };
                        Ok((expr, tail))
                    }
                    _ => Err(error::Error::UnexpectedText),
                },
            }
        }
    }
}

impl<'subs, W> Demangle<'subs, W> for Expression
where
    W: 'subs + DemangleWrite,
{
    fn demangle<'prev, 'ctx>(
        &'subs self,
        ctx: &'ctx mut DemangleContext<'subs, W>,
        scope: Option<ArgScopeStack<'prev, 'subs>>,
    ) -> fmt::Result {
        let ctx = try_begin_demangle!(self, ctx, scope);

        match *self {
            Expression::Unary(OperatorName::Simple(ref op), ref expr)
                if *op == SimpleOperatorName::PostInc || *op == SimpleOperatorName::PostDec =>
            {
                expr.demangle_as_subexpr(ctx, scope)?;
                op.demangle(ctx, scope)
            }
            Expression::Unary(ref op, ref expr) => {
                op.demangle(ctx, scope)?;
                expr.demangle_as_subexpr(ctx, scope)
            }
            // These need an extra set of parens so that it doesn't close any
            // template argument accidentally.
            Expression::Binary(
                OperatorName::Simple(SimpleOperatorName::Greater),
                ref lhs,
                ref rhs,
            ) => {
                write!(ctx, "((")?;
                lhs.demangle(ctx, scope)?;
                write!(ctx, ")>(")?;
                rhs.demangle(ctx, scope)?;
                write!(ctx, "))")
            }
            Expression::Binary(ref op, ref lhs, ref rhs) => {
                lhs.demangle_as_subexpr(ctx, scope)?;
                op.demangle(ctx, scope)?;
                rhs.demangle_as_subexpr(ctx, scope)
            }
            Expression::Ternary(
                OperatorName::Simple(SimpleOperatorName::Question),
                ref condition,
                ref consequent,
                ref alternative,
            ) => {
                condition.demangle_as_subexpr(ctx, scope)?;
                write!(ctx, "?")?;
                consequent.demangle_as_subexpr(ctx, scope)?;
                write!(ctx, " : ")?;
                alternative.demangle_as_subexpr(ctx, scope)
            }
            Expression::Ternary(ref op, ref e1, ref e2, ref e3) => {
                // Nonsensical ternary operator? Just print it like a function call,
                // I suppose...
                //
                // TODO: should we detect and reject this during parsing?
                op.demangle(ctx, scope)?;
                write!(ctx, "(")?;
                e1.demangle(ctx, scope)?;
                write!(ctx, ", ")?;
                e2.demangle(ctx, scope)?;
                write!(ctx, ", ")?;
                e3.demangle(ctx, scope)?;
                write!(ctx, ")")?;
                Ok(())
            }
            Expression::PrefixInc(ref expr) => {
                write!(ctx, "++")?;
                expr.demangle(ctx, scope)
            }
            Expression::PrefixDec(ref expr) => {
                write!(ctx, "--")?;
                expr.demangle(ctx, scope)
            }
            Expression::Call(ref functor_expr, ref args) => {
                functor_expr.demangle_as_subexpr(ctx, scope)?;
                write!(ctx, "(")?;
                let mut need_comma = false;
                for arg in args {
                    if need_comma {
                        write!(ctx, ", ")?;
                    }
                    arg.demangle(ctx, scope)?;
                    need_comma = true;
                }
                write!(ctx, ")")?;
                Ok(())
            }
            Expression::ConversionOne(ref ty, ref expr) => {
                write!(ctx, "(")?;
                ty.demangle(ctx, scope)?;
                write!(ctx, ")(")?;
                expr.demangle(ctx, scope)?;
                write!(ctx, ")")?;
                Ok(())
            }
            Expression::ConversionMany(ref ty, ref exprs) => {
                ty.demangle(ctx, scope)?;
                write!(ctx, "(")?;
                let mut need_comma = false;
                for expr in exprs {
                    if need_comma {
                        write!(ctx, ", ")?;
                    }
                    expr.demangle(ctx, scope)?;
                    need_comma = true;
                }
                write!(ctx, ")")?;
                Ok(())
            }
            Expression::ConversionBraced(ref ty, ref exprs) => {
                ty.demangle(ctx, scope)?;
                write!(ctx, "{{")?;
                let mut need_comma = false;
                for expr in exprs {
                    if need_comma {
                        write!(ctx, ", ")?;
                    }
                    expr.demangle(ctx, scope)?;
                    need_comma = true;
                }
                write!(ctx, "}}")?;
                Ok(())
            }
            Expression::BracedInitList(ref expr) => {
                write!(ctx, "{{")?;
                expr.demangle(ctx, scope)?;
                write!(ctx, "}}")?;
                Ok(())
            }
            // TODO: factor out all this duplication in the `new` variants.
            Expression::New(ref exprs, ref ty, ref init) => {
                write!(ctx, "new (")?;
                let mut need_comma = false;
                for expr in exprs {
                    if need_comma {
                        write!(ctx, ", ")?;
                    }
                    expr.demangle(ctx, scope)?;
                    need_comma = true;
                }
                write!(ctx, ") ")?;
                ty.demangle(ctx, scope)?;
                if let Some(ref init) = *init {
                    init.demangle(ctx, scope)?;
                }
                Ok(())
            }
            Expression::GlobalNew(ref exprs, ref ty, ref init) => {
                write!(ctx, "::new (")?;
                let mut need_comma = false;
                for expr in exprs {
                    if need_comma {
                        write!(ctx, ", ")?;
                    }
                    expr.demangle(ctx, scope)?;
                    need_comma = true;
                }
                write!(ctx, ") ")?;
                ty.demangle(ctx, scope)?;
                if let Some(ref init) = *init {
                    init.demangle(ctx, scope)?;
                }
                Ok(())
            }
            Expression::NewArray(ref exprs, ref ty, ref init) => {
                write!(ctx, "new[] (")?;
                let mut need_comma = false;
                for expr in exprs {
                    if need_comma {
                        write!(ctx, ", ")?;
                    }
                    expr.demangle(ctx, scope)?;
                    need_comma = true;
                }
                write!(ctx, ") ")?;
                ty.demangle(ctx, scope)?;
                if let Some(ref init) = *init {
                    init.demangle(ctx, scope)?;
                }
                Ok(())
            }
            Expression::GlobalNewArray(ref exprs, ref ty, ref init) => {
                write!(ctx, "::new[] (")?;
                let mut need_comma = false;
                for expr in exprs {
                    if need_comma {
                        write!(ctx, ", ")?;
                    }
                    expr.demangle(ctx, scope)?;
                    need_comma = true;
                }
                write!(ctx, ") ")?;
                ty.demangle(ctx, scope)?;
                if let Some(ref init) = *init {
                    init.demangle(ctx, scope)?;
                }
                Ok(())
            }
            Expression::Delete(ref expr) => {
                write!(ctx, "delete ")?;
                expr.demangle(ctx, scope)
            }
            Expression::GlobalDelete(ref expr) => {
                write!(ctx, "::delete ")?;
                expr.demangle(ctx, scope)
            }
            Expression::DeleteArray(ref expr) => {
                write!(ctx, "delete[] ")?;
                expr.demangle(ctx, scope)
            }
            Expression::GlobalDeleteArray(ref expr) => {
                write!(ctx, "::delete[] ")?;
                expr.demangle(ctx, scope)
            }
            // TODO: factor out duplicated code from cast variants.
            Expression::DynamicCast(ref ty, ref expr) => {
                write!(ctx, "dynamic_cast<")?;
                ty.demangle(ctx, scope)?;
                write!(ctx, ">(")?;
                expr.demangle(ctx, scope)?;
                write!(ctx, ")")?;
                Ok(())
            }
            Expression::StaticCast(ref ty, ref expr) => {
                write!(ctx, "static_cast<")?;
                ty.demangle(ctx, scope)?;
                write!(ctx, ">(")?;
                expr.demangle(ctx, scope)?;
                write!(ctx, ")")?;
                Ok(())
            }
            Expression::ConstCast(ref ty, ref expr) => {
                write!(ctx, "const_cast<")?;
                ty.demangle(ctx, scope)?;
                write!(ctx, ">(")?;
                expr.demangle(ctx, scope)?;
                write!(ctx, ")")?;
                Ok(())
            }
            Expression::ReinterpretCast(ref ty, ref expr) => {
                write!(ctx, "reinterpret_cast<")?;
                ty.demangle(ctx, scope)?;
                write!(ctx, ">(")?;
                expr.demangle(ctx, scope)?;
                write!(ctx, ")")?;
                Ok(())
            }
            Expression::TypeidType(ref ty) => {
                write!(ctx, "typeid (")?;
                ty.demangle(ctx, scope)?;
                write!(ctx, ")")?;
                Ok(())
            }
            Expression::TypeidExpr(ref expr) => {
                write!(ctx, "typeid (")?;
                expr.demangle(ctx, scope)?;
                write!(ctx, ")")?;
                Ok(())
            }
            Expression::SizeofType(ref ty) => {
                write!(ctx, "sizeof (")?;
                ty.demangle(ctx, scope)?;
                write!(ctx, ")")?;
                Ok(())
            }
            Expression::SizeofExpr(ref expr) => {
                write!(ctx, "sizeof (")?;
                expr.demangle(ctx, scope)?;
                write!(ctx, ")")?;
                Ok(())
            }
            Expression::AlignofType(ref ty) => {
                write!(ctx, "alignof (")?;
                ty.demangle(ctx, scope)?;
                write!(ctx, ")")?;
                Ok(())
            }
            Expression::AlignofExpr(ref expr) => {
                write!(ctx, "alignof (")?;
                expr.demangle(ctx, scope)?;
                write!(ctx, ")")?;
                Ok(())
            }
            Expression::Noexcept(ref expr) => {
                write!(ctx, "noexcept (")?;
                expr.demangle(ctx, scope)?;
                write!(ctx, ")")?;
                Ok(())
            }
            Expression::Subobject(ref expr) => expr.demangle(ctx, scope),
            Expression::TemplateParam(ref param) => param.demangle(ctx, scope),
            Expression::FunctionParam(ref param) => param.demangle(ctx, scope),
            Expression::Member(ref expr, ref name) => {
                expr.demangle_as_subexpr(ctx, scope)?;
                write!(ctx, ".")?;
                name.demangle(ctx, scope)
            }
            Expression::DerefMember(ref expr, ref name) => {
                expr.demangle(ctx, scope)?;
                write!(ctx, "->")?;
                name.demangle(ctx, scope)
            }
            Expression::PointerToMember(ref e1, ref e2) => {
                e1.demangle(ctx, scope)?;
                write!(ctx, ".*")?;
                e2.demangle(ctx, scope)
            }
            Expression::SizeofTemplatePack(ref param) => {
                write!(ctx, "sizeof...(")?;
                param.demangle(ctx, scope)?;
                write!(ctx, ")")?;
                Ok(())
            }
            Expression::SizeofFunctionPack(ref param) => {
                write!(ctx, "sizeof...(")?;
                param.demangle(ctx, scope)?;
                write!(ctx, ")")?;
                Ok(())
            }
            Expression::SizeofCapturedTemplatePack(ref args) => {
                write!(ctx, "sizeof...(")?;
                let mut need_comma = false;
                for arg in args {
                    if need_comma {
                        write!(ctx, ", ")?;
                    }
                    arg.demangle(ctx, scope)?;
                    need_comma = true;
                }
                write!(ctx, ")")?;
                Ok(())
            }
            Expression::PackExpansion(ref pack) => {
                pack.demangle_as_subexpr(ctx, scope)?;
                write!(ctx, "...")?;
                Ok(())
            }
            Expression::Throw(ref expr) => {
                write!(ctx, "throw ")?;
                expr.demangle(ctx, scope)
            }
            Expression::Rethrow => {
                write!(ctx, "throw")?;
                Ok(())
            }
            Expression::UnresolvedName(ref name) => name.demangle(ctx, scope),
            Expression::Primary(ref expr) => expr.demangle(ctx, scope),
        }
    }
}

impl Expression {
    fn demangle_as_subexpr<'subs, 'prev, 'ctx, W>(
        &'subs self,
        ctx: &'ctx mut DemangleContext<'subs, W>,
        scope: Option<ArgScopeStack<'prev, 'subs>>,
    ) -> fmt::Result
    where
        W: 'subs + DemangleWrite,
    {
        let needs_parens = match *self {
            Expression::FunctionParam(_) | Expression::Primary(ExprPrimary::External(_)) => false,
            _ => true,
        };

        if needs_parens {
            write!(ctx, "(")?;
        }

        self.demangle(ctx, scope)?;

        if needs_parens {
            write!(ctx, ")")?;
        }

        Ok(())
    }
}

/// The `<unresolved-name>` production.
///
/// ```text
/// <unresolved-name> ::= [gs] <base-unresolved-name>
///                          #
///                   ::= sr <unresolved-type> <base-unresolved-name>
///                          #
///                   ::= srN <unresolved-type> <unresolved-qualifier-level>+ E <base-unresolved-name>
///                          #
///                   ::= [gs] sr <unresolved-qualifier-level>+ E <base-unresolved-name>
///                          # A::x, N::y, A<T>::z; "gs" means leading "::"
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum UnresolvedName {
    /// `x`
    Name(BaseUnresolvedName),

    /// `::x`
    Global(BaseUnresolvedName),

    /// `T::x`  or `decltype(p)::x` or `T::N::x` or `decltype(p)::N::x`
    Nested1(
        UnresolvedTypeHandle,
        Vec<UnresolvedQualifierLevel>,
        BaseUnresolvedName,
    ),

    /// `A::x` or `N::y` or `A<T>::z`
    Nested2(Vec<UnresolvedQualifierLevel>, BaseUnresolvedName),

    /// `::A::x` or `::N::y` or `::A<T>::z`
    GlobalNested2(Vec<UnresolvedQualifierLevel>, BaseUnresolvedName),
}

impl Parse for UnresolvedName {
    fn parse<'a, 'b>(
        ctx: &'a ParseContext,
        subs: &'a mut SubstitutionTable,
        input: IndexStr<'b>,
    ) -> Result<(UnresolvedName, IndexStr<'b>)> {
        try_begin_parse!("UnresolvedName", ctx, input);

        if let Ok(tail) = consume(b"gs", input) {
            if let Ok((name, tail)) = try_recurse!(BaseUnresolvedName::parse(ctx, subs, tail)) {
                return Ok((UnresolvedName::Global(name), tail));
            }

            let tail = consume(b"sr", tail)?;
            let (levels, tail) = one_or_more::<UnresolvedQualifierLevel>(ctx, subs, tail)?;
            let tail = consume(b"E", tail)?;
            let (name, tail) = BaseUnresolvedName::parse(ctx, subs, tail)?;
            return Ok((UnresolvedName::GlobalNested2(levels, name), tail));
        }

        if let Ok((name, tail)) = try_recurse!(BaseUnresolvedName::parse(ctx, subs, input)) {
            return Ok((UnresolvedName::Name(name), tail));
        }

        let tail = consume(b"sr", input)?;

        if tail.peek() == Some(b'N') {
            let tail = consume(b"N", tail).unwrap();
            let (ty, tail) = UnresolvedTypeHandle::parse(ctx, subs, tail)?;
            let (levels, tail) = one_or_more::<UnresolvedQualifierLevel>(ctx, subs, tail)?;
            let tail = consume(b"E", tail)?;
            let (name, tail) = BaseUnresolvedName::parse(ctx, subs, tail)?;
            return Ok((UnresolvedName::Nested1(ty, levels, name), tail));
        }

        if let Ok((ty, tail)) = try_recurse!(UnresolvedTypeHandle::parse(ctx, subs, tail)) {
            let (name, tail) = BaseUnresolvedName::parse(ctx, subs, tail)?;
            return Ok((UnresolvedName::Nested1(ty, vec![], name), tail));
        }

        let (levels, tail) = one_or_more::<UnresolvedQualifierLevel>(ctx, subs, tail)?;
        let tail = consume(b"E", tail)?;
        let (name, tail) = BaseUnresolvedName::parse(ctx, subs, tail)?;
        Ok((UnresolvedName::Nested2(levels, name), tail))
    }
}

impl<'subs, W> Demangle<'subs, W> for UnresolvedName
where
    W: 'subs + DemangleWrite,
{
    fn demangle<'prev, 'ctx>(
        &'subs self,
        ctx: &'ctx mut DemangleContext<'subs, W>,
        scope: Option<ArgScopeStack<'prev, 'subs>>,
    ) -> fmt::Result {
        let ctx = try_begin_demangle!(self, ctx, scope);

        match *self {
            UnresolvedName::Name(ref name) => name.demangle(ctx, scope),
            UnresolvedName::Global(ref name) => {
                write!(ctx, "::")?;
                name.demangle(ctx, scope)
            }
            UnresolvedName::Nested1(ref ty, ref levels, ref name) => {
                ty.demangle(ctx, scope)?;
                write!(ctx, "::")?;
                for lvl in &levels[..] {
                    lvl.demangle(ctx, scope)?;
                    write!(ctx, "::")?;
                }
                name.demangle(ctx, scope)
            }
            UnresolvedName::Nested2(ref levels, ref name) => {
                for lvl in &levels[..] {
                    lvl.demangle(ctx, scope)?;
                    write!(ctx, "::")?;
                }
                name.demangle(ctx, scope)
            }
            // `::A::x` or `::N::y` or `::A<T>::z`
            UnresolvedName::GlobalNested2(ref levels, ref name) => {
                write!(ctx, "::")?;
                for lvl in &levels[..] {
                    lvl.demangle(ctx, scope)?;
                    write!(ctx, "::")?;
                }
                name.demangle(ctx, scope)
            }
        }
    }
}

/// The `<unresolved-type>` production.
///
/// ```text
/// <unresolved-type> ::= <template-param> [ <template-args> ]  # T:: or T<X,Y>::
///                   ::= <decltype>                            # decltype(p)::
///                   ::= <substitution>
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum UnresolvedType {
    /// An unresolved template type.
    Template(TemplateParam, Option<TemplateArgs>),

    /// An unresolved `decltype`.
    Decltype(Decltype),
}

define_handle! {
    /// A reference to a parsed `<unresolved-type>` production.
    pub enum UnresolvedTypeHandle
}

impl Parse for UnresolvedTypeHandle {
    fn parse<'a, 'b>(
        ctx: &'a ParseContext,
        subs: &'a mut SubstitutionTable,
        input: IndexStr<'b>,
    ) -> Result<(UnresolvedTypeHandle, IndexStr<'b>)> {
        try_begin_parse!("UnresolvedTypeHandle", ctx, input);

        if let Ok((param, tail)) = try_recurse!(TemplateParam::parse(ctx, subs, input)) {
            let (args, tail) =
                if let Ok((args, tail)) = try_recurse!(TemplateArgs::parse(ctx, subs, tail)) {
                    (Some(args), tail)
                } else {
                    (None, tail)
                };
            let ty = UnresolvedType::Template(param, args);
            let ty = Substitutable::UnresolvedType(ty);
            let idx = subs.insert(ty);
            let handle = UnresolvedTypeHandle::BackReference(idx);
            return Ok((handle, tail));
        }

        if let Ok((decltype, tail)) = try_recurse!(Decltype::parse(ctx, subs, input)) {
            let ty = UnresolvedType::Decltype(decltype);
            let ty = Substitutable::UnresolvedType(ty);
            let idx = subs.insert(ty);
            let handle = UnresolvedTypeHandle::BackReference(idx);
            return Ok((handle, tail));
        }

        let (sub, tail) = Substitution::parse(ctx, subs, input)?;
        match sub {
            Substitution::WellKnown(component) => {
                Ok((UnresolvedTypeHandle::WellKnown(component), tail))
            }
            Substitution::BackReference(idx) => {
                // TODO: should this check that the back reference actually
                // points to an `<unresolved-type>`?
                Ok((UnresolvedTypeHandle::BackReference(idx), tail))
            }
        }
    }
}

impl<'subs, W> Demangle<'subs, W> for UnresolvedType
where
    W: 'subs + DemangleWrite,
{
    fn demangle<'prev, 'ctx>(
        &'subs self,
        ctx: &'ctx mut DemangleContext<'subs, W>,
        scope: Option<ArgScopeStack<'prev, 'subs>>,
    ) -> fmt::Result {
        let ctx = try_begin_demangle!(self, ctx, scope);

        match *self {
            UnresolvedType::Decltype(ref dt) => dt.demangle(ctx, scope),
            UnresolvedType::Template(ref param, ref args) => {
                if let Some(ref args) = *args {
                    let scope = scope.push(args);
                    param.demangle(ctx, scope)?;
                    args.demangle(ctx, scope)?;
                } else {
                    param.demangle(ctx, scope)?;
                }
                Ok(())
            }
        }
    }
}

/// The `<unresolved-qualifier-level>` production.
///
/// ```text
/// <unresolved-qualifier-level> ::= <simple-id>
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UnresolvedQualifierLevel(SimpleId);

impl Parse for UnresolvedQualifierLevel {
    fn parse<'a, 'b>(
        ctx: &'a ParseContext,
        subs: &'a mut SubstitutionTable,
        input: IndexStr<'b>,
    ) -> Result<(UnresolvedQualifierLevel, IndexStr<'b>)> {
        try_begin_parse!("UnresolvedQualifierLevel", ctx, input);

        let (id, tail) = SimpleId::parse(ctx, subs, input)?;
        Ok((UnresolvedQualifierLevel(id), tail))
    }
}

impl<'subs, W> Demangle<'subs, W> for UnresolvedQualifierLevel
where
    W: 'subs + DemangleWrite,
{
    #[inline]
    fn demangle<'prev, 'ctx>(
        &'subs self,
        ctx: &'ctx mut DemangleContext<'subs, W>,
        scope: Option<ArgScopeStack<'prev, 'subs>>,
    ) -> fmt::Result {
        let ctx = try_begin_demangle!(self, ctx, scope);

        self.0.demangle(ctx, scope)
    }
}

/// The `<simple-id>` production.
///
/// ```text
/// <simple-id> ::= <source-name> [ <template-args> ]
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SimpleId(SourceName, Option<TemplateArgs>);

impl Parse for SimpleId {
    fn parse<'a, 'b>(
        ctx: &'a ParseContext,
        subs: &'a mut SubstitutionTable,
        input: IndexStr<'b>,
    ) -> Result<(SimpleId, IndexStr<'b>)> {
        try_begin_parse!("SimpleId", ctx, input);

        let (name, tail) = SourceName::parse(ctx, subs, input)?;
        let (args, tail) =
            if let Ok((args, tail)) = try_recurse!(TemplateArgs::parse(ctx, subs, tail)) {
                (Some(args), tail)
            } else {
                (None, tail)
            };
        Ok((SimpleId(name, args), tail))
    }
}

impl<'subs, W> Demangle<'subs, W> for SimpleId
where
    W: 'subs + DemangleWrite,
{
    fn demangle<'prev, 'ctx>(
        &'subs self,
        ctx: &'ctx mut DemangleContext<'subs, W>,
        scope: Option<ArgScopeStack<'prev, 'subs>>,
    ) -> fmt::Result {
        let ctx = try_begin_demangle!(self, ctx, scope);

        self.0.demangle(ctx, scope)?;
        if let Some(ref args) = self.1 {
            args.demangle(ctx, scope)?;
        }
        Ok(())
    }
}

/// The `<base-unresolved-name>` production.
///
/// ```text
/// <base-unresolved-name> ::= <simple-id>                        # unresolved name
///                        ::= on <operator-name>                 # unresolved operator-function-id
///                        ::= on <operator-name> <template-args> # unresolved operator template-id
///                        ::= dn <destructor-name>               # destructor or pseudo-destructor;
///                                                               # e.g. ~X or ~X<N-1>
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BaseUnresolvedName {
    /// An unresolved name.
    Name(SimpleId),

    /// An unresolved function or template function name.
    Operator(OperatorName, Option<TemplateArgs>),

    /// An unresolved destructor name.
    Destructor(DestructorName),
}

impl Parse for BaseUnresolvedName {
    fn parse<'a, 'b>(
        ctx: &'a ParseContext,
        subs: &'a mut SubstitutionTable,
        input: IndexStr<'b>,
    ) -> Result<(BaseUnresolvedName, IndexStr<'b>)> {
        try_begin_parse!("BaseUnresolvedName", ctx, input);

        if let Ok((name, tail)) = try_recurse!(SimpleId::parse(ctx, subs, input)) {
            return Ok((BaseUnresolvedName::Name(name), tail));
        }

        if let Ok(tail) = consume(b"on", input) {
            let (opname, tail) = OperatorName::parse(ctx, subs, tail)?;
            let (args, tail) =
                if let Ok((args, tail)) = try_recurse!(TemplateArgs::parse(ctx, subs, tail)) {
                    (Some(args), tail)
                } else {
                    (None, tail)
                };
            return Ok((BaseUnresolvedName::Operator(opname, args), tail));
        }

        let tail = consume(b"dn", input)?;
        let (name, tail) = DestructorName::parse(ctx, subs, tail)?;
        Ok((BaseUnresolvedName::Destructor(name), tail))
    }
}

impl<'subs, W> Demangle<'subs, W> for BaseUnresolvedName
where
    W: 'subs + DemangleWrite,
{
    fn demangle<'prev, 'ctx>(
        &'subs self,
        ctx: &'ctx mut DemangleContext<'subs, W>,
        scope: Option<ArgScopeStack<'prev, 'subs>>,
    ) -> fmt::Result {
        let ctx = try_begin_demangle!(self, ctx, scope);

        match *self {
            BaseUnresolvedName::Name(ref name) => name.demangle(ctx, scope),
            BaseUnresolvedName::Destructor(ref dtor) => dtor.demangle(ctx, scope),
            BaseUnresolvedName::Operator(ref op, ref args) => {
                op.demangle(ctx, scope)?;
                if let Some(ref args) = *args {
                    args.demangle(ctx, scope)?;
                }
                Ok(())
            }
        }
    }
}

/// The `<destructor-name>` production.
///
/// ```text
/// <destructor-name> ::= <unresolved-type> # e.g., ~T or ~decltype(f())
///                   ::= <simple-id>       # e.g., ~A<2*N>
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DestructorName {
    /// A destructor for an unresolved type.
    Unresolved(UnresolvedTypeHandle),

    /// A destructor for a resolved type name.
    Name(SimpleId),
}

impl Parse for DestructorName {
    fn parse<'a, 'b>(
        ctx: &'a ParseContext,
        subs: &'a mut SubstitutionTable,
        input: IndexStr<'b>,
    ) -> Result<(DestructorName, IndexStr<'b>)> {
        try_begin_parse!("DestructorName", ctx, input);

        if let Ok((ty, tail)) = try_recurse!(UnresolvedTypeHandle::parse(ctx, subs, input)) {
            return Ok((DestructorName::Unresolved(ty), tail));
        }

        let (name, tail) = SimpleId::parse(ctx, subs, input)?;
        Ok((DestructorName::Name(name), tail))
    }
}

impl<'subs, W> Demangle<'subs, W> for DestructorName
where
    W: 'subs + DemangleWrite,
{
    fn demangle<'prev, 'ctx>(
        &'subs self,
        ctx: &'ctx mut DemangleContext<'subs, W>,
        scope: Option<ArgScopeStack<'prev, 'subs>>,
    ) -> fmt::Result {
        let ctx = try_begin_demangle!(self, ctx, scope);

        write!(ctx, "~")?;
        match *self {
            DestructorName::Unresolved(ref ty) => ty.demangle(ctx, scope),
            DestructorName::Name(ref name) => name.demangle(ctx, scope),
        }
    }
}

/// The `<expr-primary>` production.
///
/// ```text
/// <expr-primary> ::= L <type> <value number> E                        # integer literal
///                ::= L <type> <value float> E                         # floating literal
///                ::= L <string type> E                                # string literal
///                ::= L <nullptr type> E                               # nullptr literal (i.e., "LDnE")
///                ::= L <pointer type> 0 E                             # null pointer template argument
///                ::= L <type> <real-part float> _ <imag-part float> E # complex floating point literal (C 2000)
///                ::= L <mangled-name> E                               # external name
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ExprPrimary {
    /// A type literal.
    Literal(TypeHandle, usize, usize),

    /// An external name.
    External(MangledName),
}

impl Parse for ExprPrimary {
    fn parse<'a, 'b>(
        ctx: &'a ParseContext,
        subs: &'a mut SubstitutionTable,
        input: IndexStr<'b>,
    ) -> Result<(ExprPrimary, IndexStr<'b>)> {
        try_begin_parse!("ExprPrimary", ctx, input);

        let tail = consume(b"L", input)?;

        if let Ok((ty, tail)) = try_recurse!(TypeHandle::parse(ctx, subs, tail)) {
            let start = tail.index();
            let num_bytes_in_literal = tail.as_ref().iter().take_while(|&&c| c != b'E').count();
            let tail = tail.range_from(num_bytes_in_literal..);
            let end = tail.index();
            let tail = consume(b"E", tail)?;
            let expr = ExprPrimary::Literal(ty, start, end);
            return Ok((expr, tail));
        }

        let (name, tail) = MangledName::parse(ctx, subs, tail)?;
        let tail = consume(b"E", tail)?;
        let expr = ExprPrimary::External(name);
        Ok((expr, tail))
    }
}

impl<'subs, W> Demangle<'subs, W> for ExprPrimary
where
    W: 'subs + DemangleWrite,
{
    fn demangle<'prev, 'ctx>(
        &'subs self,
        ctx: &'ctx mut DemangleContext<'subs, W>,
        scope: Option<ArgScopeStack<'prev, 'subs>>,
    ) -> fmt::Result {
        let ctx = try_begin_demangle!(self, ctx, scope);

        fn write_literal<W>(ctx: &mut DemangleContext<W>, start: usize, end: usize) -> fmt::Result
        where
            W: DemangleWrite,
        {
            debug_assert!(start <= end);
            let start = if start < end && ctx.input[start] == b'n' {
                write!(ctx, "-")?;
                start + 1
            } else {
                start
            };
            let s = str::from_utf8(&ctx.input[start..end]).map_err(|e| {
                log!("Error writing literal: {}", e);
                fmt::Error
            })?;
            ctx.write_str(s)
        }

        match *self {
            ExprPrimary::External(ref name) => {
                let saved_show_params = ctx.show_params;
                ctx.show_params = true;
                let ret = name.demangle(ctx, scope);
                ctx.show_params = saved_show_params;
                ret
            }
            ExprPrimary::Literal(
                TypeHandle::Builtin(BuiltinType::Standard(StandardBuiltinType::Bool)),
                start,
                end,
            ) => match &ctx.input[start..end] {
                b"0" => write!(ctx, "false"),
                b"1" => write!(ctx, "true"),
                _ => {
                    write!(ctx, "(bool)")?;
                    write_literal(ctx, start, end)
                }
            },
            ExprPrimary::Literal(
                TypeHandle::Builtin(BuiltinType::Standard(StandardBuiltinType::Nullptr)),
                _,
                _,
            ) => write!(ctx, "nullptr"),
            ExprPrimary::Literal(
                ref ty @ TypeHandle::Builtin(BuiltinType::Standard(StandardBuiltinType::Double)),
                start,
                end,
            )
            | ExprPrimary::Literal(
                ref ty @ TypeHandle::Builtin(BuiltinType::Standard(StandardBuiltinType::Float)),
                start,
                end,
            ) => {
                if ctx.show_expression_literal_types {
                    write!(ctx, "(")?;
                    ty.demangle(ctx, scope)?;
                    write!(ctx, ")")?;
                }
                let start = if start < end && ctx.input[start] == b'n' {
                    write!(ctx, "-[")?;
                    start + 1
                } else {
                    write!(ctx, "[")?;
                    start
                };
                let s = str::from_utf8(&ctx.input[start..end]).map_err(|e| {
                    log!("Error writing literal: {}", e);
                    fmt::Error
                })?;
                ctx.write_str(s)?;
                write!(ctx, "]")
            }
            ExprPrimary::Literal(
                TypeHandle::Builtin(BuiltinType::Standard(StandardBuiltinType::Int)),
                start,
                end,
            ) => write_literal(ctx, start, end),
            ExprPrimary::Literal(ref ty, start, end) => {
                if ctx.show_expression_literal_types {
                    write!(ctx, "(")?;
                    ty.demangle(ctx, scope)?;
                    write!(ctx, ")")?;
                }
                write_literal(ctx, start, end)
            }
        }
    }
}

/// The `<initializer>` production.
///
/// ```text
/// <initializer> ::= pi <expression>* E # parenthesized initialization
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Initializer(Vec<Expression>);

impl Parse for Initializer {
    fn parse<'a, 'b>(
        ctx: &'a ParseContext,
        subs: &'a mut SubstitutionTable,
        input: IndexStr<'b>,
    ) -> Result<(Initializer, IndexStr<'b>)> {
        try_begin_parse!("Initializer", ctx, input);

        let tail = consume(b"pi", input)?;
        let (exprs, tail) = zero_or_more::<Expression>(ctx, subs, tail)?;
        let tail = consume(b"E", tail)?;
        Ok((Initializer(exprs), tail))
    }
}

impl<'subs, W> Demangle<'subs, W> for Initializer
where
    W: 'subs + DemangleWrite,
{
    fn demangle<'prev, 'ctx>(
        &'subs self,
        ctx: &'ctx mut DemangleContext<'subs, W>,
        scope: Option<ArgScopeStack<'prev, 'subs>>,
    ) -> fmt::Result {
        let ctx = try_begin_demangle!(self, ctx, scope);

        write!(ctx, "(")?;
        let mut need_comma = false;
        for expr in &self.0 {
            if need_comma {
                write!(ctx, ", ")?;
            }
            expr.demangle(ctx, scope)?;
            need_comma = true;
        }
        write!(ctx, ")")?;
        Ok(())
    }
}

/// The `<local-name>` production.
///
/// ```text
/// <local-name> := Z <function encoding> E <entity name> [<discriminator>]
///              := Z <function encoding> E s [<discriminator>]
///              := Z <function encoding> Ed [ <parameter number> ] _ <entity name>
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LocalName {
    /// The mangling of the enclosing function, the mangling of the entity
    /// relative to the function, and an optional discriminator.
    Relative(Box<Encoding>, Option<Box<Name>>, Option<Discriminator>),

    /// A default argument in a class definition.
    Default(Box<Encoding>, Option<usize>, Box<Name>),
}

impl Parse for LocalName {
    fn parse<'a, 'b>(
        ctx: &'a ParseContext,
        subs: &'a mut SubstitutionTable,
        input: IndexStr<'b>,
    ) -> Result<(LocalName, IndexStr<'b>)> {
        try_begin_parse!("LocalName", ctx, input);

        let tail = consume(b"Z", input)?;
        let (encoding, tail) = Encoding::parse(ctx, subs, tail)?;
        let tail = consume(b"E", tail)?;

        if let Ok(tail) = consume(b"s", tail) {
            let (disc, tail) =
                if let Ok((disc, tail)) = try_recurse!(Discriminator::parse(ctx, subs, tail)) {
                    (Some(disc), tail)
                } else {
                    (None, tail)
                };
            return Ok((LocalName::Relative(Box::new(encoding), None, disc), tail));
        }

        if let Ok(tail) = consume(b"d", tail) {
            let (param, tail) =
                if let Ok((num, tail)) = try_recurse!(Number::parse(ctx, subs, tail)) {
                    (Some(num as _), tail)
                } else {
                    (None, tail)
                };
            let tail = consume(b"_", tail)?;
            let (name, tail) = Name::parse(ctx, subs, tail)?;
            return Ok((
                LocalName::Default(Box::new(encoding), param, Box::new(name)),
                tail,
            ));
        }

        let (name, tail) = Name::parse(ctx, subs, tail)?;
        let (disc, tail) =
            if let Ok((disc, tail)) = try_recurse!(Discriminator::parse(ctx, subs, tail)) {
                (Some(disc), tail)
            } else {
                (None, tail)
            };

        Ok((
            LocalName::Relative(Box::new(encoding), Some(Box::new(name)), disc),
            tail,
        ))
    }
}

impl<'subs, W> Demangle<'subs, W> for LocalName
where
    W: 'subs + DemangleWrite,
{
    fn demangle<'prev, 'ctx>(
        &'subs self,
        ctx: &'ctx mut DemangleContext<'subs, W>,
        scope: Option<ArgScopeStack<'prev, 'subs>>,
    ) -> fmt::Result {
        let ctx = try_begin_demangle!(self, ctx, scope);

        let saved_show_params = ctx.show_params;
        ctx.show_params = true;
        let ret = match *self {
            LocalName::Relative(ref encoding, Some(ref name), _) => {
                encoding.demangle(ctx, scope)?;
                write!(ctx, "::")?;
                name.demangle(ctx, scope)
            }
            LocalName::Relative(ref encoding, None, _) => {
                // No name means that this is the symbol for a string literal.
                encoding.demangle(ctx, scope)?;
                write!(ctx, "::string literal")?;
                Ok(())
            }
            LocalName::Default(ref encoding, _, _) => encoding.demangle(ctx, scope),
        };
        ctx.show_params = saved_show_params;
        ret
    }
}

impl GetTemplateArgs for LocalName {
    fn get_template_args<'a>(&'a self, subs: &'a SubstitutionTable) -> Option<&'a TemplateArgs> {
        match *self {
            LocalName::Relative(_, None, _) => None,
            LocalName::Relative(_, Some(ref name), _) | LocalName::Default(_, _, ref name) => {
                name.get_template_args(subs)
            }
        }
    }
}

impl<'a> GetLeafName<'a> for LocalName {
    fn get_leaf_name(&'a self, subs: &'a SubstitutionTable) -> Option<LeafName<'a>> {
        match *self {
            LocalName::Relative(_, None, _) => None,
            LocalName::Relative(_, Some(ref name), _) | LocalName::Default(_, _, ref name) => {
                name.get_leaf_name(subs)
            }
        }
    }
}

/// The `<discriminator>` production.
///
/// ```text
/// <discriminator> := _ <non-negative number>      # when number < 10
///                 := __ <non-negative number> _   # when number >= 10
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Discriminator(usize);

impl Parse for Discriminator {
    fn parse<'a, 'b>(
        ctx: &'a ParseContext,
        _subs: &'a mut SubstitutionTable,
        input: IndexStr<'b>,
    ) -> Result<(Discriminator, IndexStr<'b>)> {
        try_begin_parse!("Discriminator", ctx, input);

        let tail = consume(b"_", input)?;

        if let Ok(tail) = consume(b"_", tail) {
            let (num, tail) = parse_number(10, false, tail)?;
            debug_assert!(num >= 0);
            if num < 10 {
                return Err(error::Error::UnexpectedText);
            }
            let tail = consume(b"_", tail)?;
            return Ok((Discriminator(num as _), tail));
        }

        match tail.try_split_at(1) {
            None => Err(error::Error::UnexpectedEnd),
            Some((head, tail)) => match head.as_ref()[0] {
                b'0' => Ok((Discriminator(0), tail)),
                b'1' => Ok((Discriminator(1), tail)),
                b'2' => Ok((Discriminator(2), tail)),
                b'3' => Ok((Discriminator(3), tail)),
                b'4' => Ok((Discriminator(4), tail)),
                b'5' => Ok((Discriminator(5), tail)),
                b'6' => Ok((Discriminator(6), tail)),
                b'7' => Ok((Discriminator(7), tail)),
                b'8' => Ok((Discriminator(8), tail)),
                b'9' => Ok((Discriminator(9), tail)),
                _ => Err(error::Error::UnexpectedText),
            },
        }
    }
}

/// The `<closure-type-name>` production.
///
/// ```text
/// <closure-type-name> ::= Ul <lambda-sig> E [ <nonnegative number> ] _
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ClosureTypeName(LambdaSig, Option<usize>);

impl Parse for ClosureTypeName {
    fn parse<'a, 'b>(
        ctx: &'a ParseContext,
        subs: &'a mut SubstitutionTable,
        input: IndexStr<'b>,
    ) -> Result<(ClosureTypeName, IndexStr<'b>)> {
        try_begin_parse!("ClosureTypeName", ctx, input);

        let tail = consume(b"Ul", input)?;
        let (sig, tail) = LambdaSig::parse(ctx, subs, tail)?;
        let tail = consume(b"E", tail)?;
        let (num, tail) = if let Ok((num, tail)) = parse_number(10, false, tail) {
            (Some(num as _), tail)
        } else {
            (None, tail)
        };
        let tail = consume(b"_", tail)?;
        Ok((ClosureTypeName(sig, num), tail))
    }
}

impl<'subs, W> Demangle<'subs, W> for ClosureTypeName
where
    W: 'subs + DemangleWrite,
{
    fn demangle<'prev, 'ctx>(
        &'subs self,
        ctx: &'ctx mut DemangleContext<'subs, W>,
        scope: Option<ArgScopeStack<'prev, 'subs>>,
    ) -> fmt::Result {
        let ctx = try_begin_demangle!(self, ctx, scope);

        write!(ctx, "{{lambda(")?;
        self.0.demangle(ctx, scope)?;
        write!(ctx, ")#{}}}", self.1.map_or(1, |n| n + 2))?;
        Ok(())
    }
}

impl<'subs> ArgScope<'subs, 'subs> for ClosureTypeName {
    fn leaf_name(&'subs self) -> Result<LeafName<'subs>> {
        Ok(LeafName::Closure(self))
    }

    fn get_template_arg(
        &'subs self,
        _: usize,
    ) -> Result<(&'subs TemplateArg, &'subs TemplateArgs)> {
        Err(error::Error::BadTemplateArgReference)
    }

    fn get_function_arg(&'subs self, _: usize) -> Result<&'subs Type> {
        Err(error::Error::BadFunctionArgReference)
    }
}

impl<'a> GetLeafName<'a> for ClosureTypeName {
    #[inline]
    fn get_leaf_name(&'a self, _: &'a SubstitutionTable) -> Option<LeafName<'a>> {
        Some(LeafName::Closure(self))
    }
}

impl ClosureTypeName {
    #[inline]
    fn starts_with(byte: u8, input: &IndexStr) -> bool {
        byte == b'U' && input.peek_second().map(|b| b == b'l').unwrap_or(false)
    }
}

/// The `<lambda-sig>` production.
///
/// ```text
/// <lambda-sig> ::= <parameter type>+  # Parameter types or "v" if the lambda has no parameters
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LambdaSig(Vec<TypeHandle>);

impl LambdaSig {
    fn demangle_args<'subs, 'prev, 'ctx, W>(
        &'subs self,
        ctx: &'ctx mut DemangleContext<'subs, W>,
        scope: Option<ArgScopeStack<'prev, 'subs>>,
    ) -> fmt::Result
    where
        W: 'subs + DemangleWrite,
    {
        let mut need_comma = false;
        for ty in &self.0 {
            if need_comma {
                write!(ctx, ", ")?;
            }
            ty.demangle(ctx, scope)?;
            need_comma = true;
        }
        Ok(())
    }
}

impl Parse for LambdaSig {
    fn parse<'a, 'b>(
        ctx: &'a ParseContext,
        subs: &'a mut SubstitutionTable,
        input: IndexStr<'b>,
    ) -> Result<(LambdaSig, IndexStr<'b>)> {
        try_begin_parse!("LambdaSig", ctx, input);

        let (types, tail) = if let Ok(tail) = consume(b"v", input) {
            (vec![], tail)
        } else {
            one_or_more::<TypeHandle>(ctx, subs, input)?
        };
        Ok((LambdaSig(types), tail))
    }
}

impl<'subs, W> Demangle<'subs, W> for LambdaSig
where
    W: 'subs + DemangleWrite,
{
    fn demangle<'prev, 'ctx>(
        &'subs self,
        ctx: &'ctx mut DemangleContext<'subs, W>,
        scope: Option<ArgScopeStack<'prev, 'subs>>,
    ) -> fmt::Result {
        let ctx = try_begin_demangle!(self, ctx, scope);

        ctx.is_lambda_arg = true;
        let r = self.demangle_args(ctx, scope);
        ctx.is_lambda_arg = false;
        r
    }
}

/// The `<data-member-prefix>` production.
///
/// ```text
/// <data-member-prefix> := <member source-name> M
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DataMemberPrefix(SourceName);

impl Parse for DataMemberPrefix {
    fn parse<'a, 'b>(
        ctx: &'a ParseContext,
        subs: &'a mut SubstitutionTable,
        input: IndexStr<'b>,
    ) -> Result<(DataMemberPrefix, IndexStr<'b>)> {
        try_begin_parse!("DataMemberPrefix", ctx, input);

        let (name, tail) = SourceName::parse(ctx, subs, input)?;
        let tail = consume(b"M", tail)?;
        Ok((DataMemberPrefix(name), tail))
    }
}

impl<'a> GetLeafName<'a> for DataMemberPrefix {
    #[inline]
    fn get_leaf_name(&'a self, _: &'a SubstitutionTable) -> Option<LeafName<'a>> {
        Some(LeafName::SourceName(&self.0))
    }
}

impl<'subs, W> Demangle<'subs, W> for DataMemberPrefix
where
    W: 'subs + DemangleWrite,
{
    #[inline]
    fn demangle<'prev, 'ctx>(
        &'subs self,
        ctx: &'ctx mut DemangleContext<'subs, W>,
        scope: Option<ArgScopeStack<'prev, 'subs>>,
    ) -> fmt::Result {
        let ctx = try_begin_demangle!(self, ctx, scope);

        ctx.push_demangle_node(DemangleNodeType::DataMemberPrefix);
        let ret = self.0.demangle(ctx, scope);
        ctx.pop_demangle_node();
        ret
    }
}

/// The `<substitution>` form: a back-reference to some component we've already
/// parsed.
///
/// ```text
/// <substitution> ::= S <seq-id> _
///                ::= S_
///                ::= St # ::std::
///                ::= Sa # ::std::allocator
///                ::= Sb # ::std::basic_string
///                ::= Ss # ::std::basic_string < char,
///                                               ::std::char_traits<char>,
///                                               ::std::allocator<char> >
///                ::= Si # ::std::basic_istream<char,  std::char_traits<char> >
///                ::= So # ::std::basic_ostream<char,  std::char_traits<char> >
///                ::= Sd # ::std::basic_iostream<char, std::char_traits<char> >
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Substitution {
    /// A reference to an entity that already occurred, ie the `S_` and `S
    /// <seq-id> _` forms.
    BackReference(usize),

    /// A well-known substitution component. These are the components that do
    /// not appear in the substitution table, but have abbreviations specified
    /// directly in the grammar.
    WellKnown(WellKnownComponent),
}

impl Parse for Substitution {
    fn parse<'a, 'b>(
        ctx: &'a ParseContext,
        subs: &'a mut SubstitutionTable,
        input: IndexStr<'b>,
    ) -> Result<(Substitution, IndexStr<'b>)> {
        try_begin_parse!("Substitution", ctx, input);

        if let Ok((well_known, tail)) = try_recurse!(WellKnownComponent::parse(ctx, subs, input)) {
            return Ok((Substitution::WellKnown(well_known), tail));
        }

        let tail = consume(b"S", input)?;
        let (idx, tail) = if let Ok((idx, tail)) = try_recurse!(SeqId::parse(ctx, subs, tail)) {
            (idx.0 + 1, tail)
        } else {
            (0, tail)
        };

        if !subs.contains(idx) {
            return Err(error::Error::BadBackReference);
        }

        let tail = consume(b"_", tail)?;
        log!("Found a reference to @ {}", idx);
        Ok((Substitution::BackReference(idx), tail))
    }
}

define_vocabulary! {
/// The `<substitution>` variants that are encoded directly in the grammar,
/// rather than as back references to other components in the substitution
/// table.
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub enum WellKnownComponent {
        Std          (b"St", "std"),
        StdAllocator (b"Sa", "std::allocator"),
        StdString1   (b"Sb", "std::basic_string"),
        StdString2   (b"Ss", "std::string"),
        StdIstream   (b"Si", "std::basic_istream<char, std::char_traits<char> >"),
        StdOstream   (b"So", "std::ostream"),
        StdIostream  (b"Sd", "std::basic_iostream<char, std::char_traits<char> >")
    }
}

impl<'a> GetLeafName<'a> for WellKnownComponent {
    fn get_leaf_name(&'a self, _: &'a SubstitutionTable) -> Option<LeafName<'a>> {
        match *self {
            WellKnownComponent::Std => None,
            _ => Some(LeafName::WellKnownComponent(self)),
        }
    }
}

impl<'a> ArgScope<'a, 'a> for WellKnownComponent {
    fn leaf_name(&'a self) -> Result<LeafName<'a>> {
        Ok(LeafName::WellKnownComponent(self))
    }

    fn get_template_arg(&'a self, _: usize) -> Result<(&'a TemplateArg, &'a TemplateArgs)> {
        Err(error::Error::BadTemplateArgReference)
    }

    fn get_function_arg(&'a self, _: usize) -> Result<&'a Type> {
        Err(error::Error::BadFunctionArgReference)
    }
}

impl<'subs, W> DemangleAsLeaf<'subs, W> for WellKnownComponent
where
    W: 'subs + DemangleWrite,
{
    fn demangle_as_leaf<'me, 'ctx>(
        &'me self,
        ctx: &'ctx mut DemangleContext<'subs, W>,
    ) -> fmt::Result {
        match *self {
            WellKnownComponent::Std => {
                panic!("should never treat `WellKnownComponent::Std` as a leaf name")
            }
            WellKnownComponent::StdAllocator => write!(ctx, "allocator"),
            WellKnownComponent::StdString1 => write!(ctx, "basic_string"),
            WellKnownComponent::StdString2 => write!(ctx, "string"),
            WellKnownComponent::StdIstream => write!(ctx, "basic_istream"),
            WellKnownComponent::StdOstream => write!(ctx, "ostream"),
            WellKnownComponent::StdIostream => write!(ctx, "basic_iostream"),
        }
    }
}

/// The `<special-name>` production.
///
/// The `<special-name>` production is spread in pieces through out the ABI
/// spec, and then there are a bunch of `g++` extensions that have become de
/// facto.
///
/// ### 5.1.4.1 Virtual Tables and RTTI
///
/// ```text
/// <special-name> ::= TV <type>    # virtual table
///                ::= TT <type>    # VTT structure (construction vtable index)
///                ::= TI <type>    # typeinfo structure
///                ::= TS <type>    # typeinfo name (null-terminated byte string)
/// ```
///
/// ### 5.1.4.2 Virtual Override Thunks
///
/// ```text
/// <special-name> ::= T <call-offset> <base encoding>
///     # base is the nominal target function of thunk
///
/// <special-name> ::= Tc <call-offset> <call-offset> <base encoding>
///     # base is the nominal target function of thunk
///     # first call-offset is 'this' adjustment
///     # second call-offset is result adjustment
/// ```
///
/// ### 5.1.4.4 Guard Variables
///
/// ```text
/// <special-name> ::= GV <object name> # Guard variable for one-time initialization
///     # No <type>
/// ```
///
/// ### 5.1.4.5 Lifetime-Extended Temporaries
///
/// ```text
/// <special-name> ::= GR <object name> _             # First temporary
/// <special-name> ::= GR <object name> <seq-id> _    # Subsequent temporaries
/// ```
///
/// ### De Facto Standard Extensions
///
/// ```text
/// <special-name> ::= TC <type> <number> _ <type>    # construction vtable
///                ::= TF <type>                      # typinfo function
///                ::= TH <name>                      # TLS initialization function
///                ::= TW <name>                      # TLS wrapper function
///                ::= Gr <resource name>             # Java Resource
///                ::= GTt <encoding>                 # Transaction-Safe function
///                ::= GTn <encoding>                 # Non-Transaction-Safe function
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SpecialName {
    /// A virtual table.
    VirtualTable(TypeHandle),

    /// A VTT structure (construction vtable index).
    Vtt(TypeHandle),

    /// A typeinfo structure.
    Typeinfo(TypeHandle),

    /// A typeinfo name (null-terminated byte string).
    TypeinfoName(TypeHandle),

    /// A virtual override thunk.
    VirtualOverrideThunk(CallOffset, Box<Encoding>),

    /// A virtual override thunk with a covariant return type.
    VirtualOverrideThunkCovariant(CallOffset, CallOffset, Box<Encoding>),

    /// An initialization guard for some static storage.
    Guard(Name),

    /// A temporary used in the initialization of a static storage and promoted
    /// to a static lifetime.
    GuardTemporary(Name, usize),

    /// A construction vtable structure.
    ConstructionVtable(TypeHandle, usize, TypeHandle),

    /// A typeinfo function.
    TypeinfoFunction(TypeHandle),

    /// A TLS initialization function.
    TlsInit(Name),

    /// A TLS wrapper function.
    TlsWrapper(Name),

    /// A Java Resource.
    JavaResource(Vec<ResourceName>),

    /// A function declared transaction-safe
    TransactionClone(Box<Encoding>),

    /// A function declared non-transaction-safe
    NonTransactionClone(Box<Encoding>),
}

impl Parse for SpecialName {
    fn parse<'a, 'b>(
        ctx: &'a ParseContext,
        subs: &'a mut SubstitutionTable,
        input: IndexStr<'b>,
    ) -> Result<(SpecialName, IndexStr<'b>)> {
        try_begin_parse!("SpecialName", ctx, input);

        let (head, tail) = match input.try_split_at(2) {
            None => return Err(error::Error::UnexpectedEnd),
            Some((head, tail)) => (head, tail),
        };

        match head.as_ref() {
            b"TV" => {
                let (ty, tail) = TypeHandle::parse(ctx, subs, tail)?;
                Ok((SpecialName::VirtualTable(ty), tail))
            }
            b"TT" => {
                let (ty, tail) = TypeHandle::parse(ctx, subs, tail)?;
                Ok((SpecialName::Vtt(ty), tail))
            }
            b"TI" => {
                let (ty, tail) = TypeHandle::parse(ctx, subs, tail)?;
                Ok((SpecialName::Typeinfo(ty), tail))
            }
            b"TS" => {
                let (ty, tail) = TypeHandle::parse(ctx, subs, tail)?;
                Ok((SpecialName::TypeinfoName(ty), tail))
            }
            b"Tc" => {
                let (first, tail) = CallOffset::parse(ctx, subs, tail)?;
                let (second, tail) = CallOffset::parse(ctx, subs, tail)?;
                let (base, tail) = Encoding::parse(ctx, subs, tail)?;
                Ok((
                    SpecialName::VirtualOverrideThunkCovariant(first, second, Box::new(base)),
                    tail,
                ))
            }
            b"Th" | b"Tv" => {
                // The "h"/"v" is part of the `<call-offset>`, so back up to the
                // `input`.
                let tail = consume(b"T", input).unwrap();
                let (offset, tail) = CallOffset::parse(ctx, subs, tail)?;
                let (base, tail) = Encoding::parse(ctx, subs, tail)?;
                Ok((
                    SpecialName::VirtualOverrideThunk(offset, Box::new(base)),
                    tail,
                ))
            }
            b"TC" => {
                let (ty1, tail) = TypeHandle::parse(ctx, subs, tail)?;
                let (n, tail) = parse_number(10, false, tail)?;
                let tail = consume(b"_", tail)?;
                let (ty2, tail) = TypeHandle::parse(ctx, subs, tail)?;
                Ok((SpecialName::ConstructionVtable(ty1, n as usize, ty2), tail))
            }
            b"TF" => {
                let (ty, tail) = TypeHandle::parse(ctx, subs, tail)?;
                Ok((SpecialName::TypeinfoFunction(ty), tail))
            }
            b"TH" => {
                let (name, tail) = Name::parse(ctx, subs, tail)?;
                Ok((SpecialName::TlsInit(name), tail))
            }
            b"TW" => {
                let (name, tail) = Name::parse(ctx, subs, tail)?;
                Ok((SpecialName::TlsWrapper(name), tail))
            }
            b"GV" => {
                let (name, tail) = Name::parse(ctx, subs, tail)?;
                Ok((SpecialName::Guard(name), tail))
            }
            b"GR" => {
                let (name, tail) = Name::parse(ctx, subs, tail)?;
                let (idx, tail) = if let Ok(tail) = consume(b"_", tail) {
                    (0, tail)
                } else {
                    let (idx, tail) = SeqId::parse(ctx, subs, tail)?;
                    let tail = consume(b"_", tail)?;
                    (idx.0 + 1, tail)
                };
                Ok((SpecialName::GuardTemporary(name, idx), tail))
            }
            b"Gr" => {
                let (resource_name_len, tail) = parse_number(10, false, tail)?;
                if resource_name_len == 0 {
                    return Err(error::Error::UnexpectedText);
                }

                let (head, tail) = match tail.try_split_at(resource_name_len as _) {
                    Some((head, tail)) => (head, tail),
                    None => return Err(error::Error::UnexpectedEnd),
                };

                let head = consume(b"_", head)?;

                let (resource_names, empty) = zero_or_more::<ResourceName>(ctx, subs, head)?;
                if !empty.is_empty() {
                    return Err(error::Error::UnexpectedText);
                }

                Ok((SpecialName::JavaResource(resource_names), tail))
            }
            b"GT" => {
                match tail.next_or(error::Error::UnexpectedEnd)? {
                    (b'n', tail) => {
                        let (base, tail) = Encoding::parse(ctx, subs, tail)?;
                        Ok((SpecialName::NonTransactionClone(Box::new(base)), tail))
                    }
                    // Different letters could stand for different types of
                    // transactional cloning, but for now, treat them all the same
                    (b't', tail) | (_, tail) => {
                        let (base, tail) = Encoding::parse(ctx, subs, tail)?;
                        Ok((SpecialName::TransactionClone(Box::new(base)), tail))
                    }
                }
            }
            _ => Err(error::Error::UnexpectedText),
        }
    }
}

impl<'subs, W> Demangle<'subs, W> for SpecialName
where
    W: 'subs + DemangleWrite,
{
    fn demangle<'prev, 'ctx>(
        &'subs self,
        ctx: &'ctx mut DemangleContext<'subs, W>,
        scope: Option<ArgScopeStack<'prev, 'subs>>,
    ) -> fmt::Result {
        let ctx = try_begin_demangle!(self, ctx, scope);

        match *self {
            SpecialName::VirtualTable(ref ty) => {
                write!(ctx, "{{vtable(")?;
                ctx.push_demangle_node(DemangleNodeType::VirtualTable);
                ty.demangle(ctx, scope)?;
                ctx.pop_demangle_node();
                write!(ctx, ")}}")?;
                Ok(())
            }
            SpecialName::Vtt(ref ty) => {
                write!(ctx, "{{vtt(")?;
                ty.demangle(ctx, scope)?;
                write!(ctx, ")}}")?;
                Ok(())
            }
            SpecialName::Typeinfo(ref ty) => {
                write!(ctx, "typeinfo for ")?;
                ty.demangle(ctx, scope)
            }
            SpecialName::TypeinfoName(ref ty) => {
                write!(ctx, "typeinfo name for ")?;
                ty.demangle(ctx, scope)
            }
            SpecialName::VirtualOverrideThunk(ref offset, ref encoding) => {
                write!(ctx, "{{virtual override thunk(")?;
                offset.demangle(ctx, scope)?;
                write!(ctx, ", ")?;
                encoding.demangle(ctx, scope)?;
                write!(ctx, ")}}")?;
                Ok(())
            }
            SpecialName::VirtualOverrideThunkCovariant(
                ref this_offset,
                ref result_offset,
                ref encoding,
            ) => {
                write!(ctx, "{{virtual override thunk(")?;
                this_offset.demangle(ctx, scope)?;
                write!(ctx, ", ")?;
                result_offset.demangle(ctx, scope)?;
                write!(ctx, ", ")?;
                encoding.demangle(ctx, scope)?;
                write!(ctx, ")}}")?;
                Ok(())
            }
            SpecialName::Guard(ref name) => {
                write!(ctx, "guard variable for ")?;
                name.demangle(ctx, scope)
            }
            SpecialName::GuardTemporary(ref name, n) => {
                write!(ctx, "reference temporary #{} for ", n)?;
                name.demangle(ctx, scope)
            }
            SpecialName::ConstructionVtable(ref ty1, _, ref ty2) => {
                write!(ctx, "construction vtable for ")?;
                ty1.demangle(ctx, scope)?;
                write!(ctx, "-in-")?;
                ty2.demangle(ctx, scope)
            }
            SpecialName::TypeinfoFunction(ref ty) => {
                write!(ctx, "typeinfo fn for ")?;
                ty.demangle(ctx, scope)
            }
            SpecialName::TlsInit(ref name) => {
                write!(ctx, "TLS init function for ")?;
                name.demangle(ctx, scope)
            }
            SpecialName::TlsWrapper(ref name) => {
                write!(ctx, "TLS wrapper function for ")?;
                name.demangle(ctx, scope)
            }
            SpecialName::TransactionClone(ref encoding) => {
                write!(ctx, "transaction clone for ")?;
                encoding.demangle(ctx, scope)
            }
            SpecialName::NonTransactionClone(ref encoding) => {
                write!(ctx, "non-transaction clone for ")?;
                encoding.demangle(ctx, scope)
            }
            SpecialName::JavaResource(ref names) => {
                write!(ctx, "java resource ")?;
                for name in names {
                    name.demangle(ctx, scope)?;
                }
                Ok(())
            }
        }
    }
}

/// The `<resource name>` pseudo-terminal.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ResourceName {
    start: usize,
    end: usize,
}

impl Parse for ResourceName {
    fn parse<'a, 'b>(
        ctx: &'a ParseContext,
        _subs: &'a mut SubstitutionTable,
        input: IndexStr<'b>,
    ) -> Result<(ResourceName, IndexStr<'b>)> {
        try_begin_parse!("ResourceName", ctx, input);

        if input.is_empty() {
            return Err(error::Error::UnexpectedEnd);
        }

        let mut end = input
            .as_ref()
            .iter()
            .map(|&c| c as char)
            .take_while(|&c| c != '$' || c.is_digit(36))
            .count();

        if end == 0 {
            return Err(error::Error::UnexpectedText);
        }

        if input.range_from(end..).peek() == Some(b'$') {
            match input.range_from(end..).peek_second() {
                Some(b'S') | Some(b'_') | Some(b'$') => end += 2,
                _ => return Err(error::Error::UnexpectedText),
            }
        }

        let tail = input.range_from(end..);

        let resource_name = ResourceName {
            start: input.index(),
            end: tail.index(),
        };

        Ok((resource_name, tail))
    }
}

impl<'subs, W> Demangle<'subs, W> for ResourceName
where
    W: 'subs + DemangleWrite,
{
    #[inline]
    fn demangle<'prev, 'ctx>(
        &'subs self,
        ctx: &'ctx mut DemangleContext<'subs, W>,
        scope: Option<ArgScopeStack<'prev, 'subs>>,
    ) -> fmt::Result {
        let ctx = try_begin_demangle!(self, ctx, scope);

        let mut i = self.start;
        while i < self.end {
            let ch = ctx.input[i];
            if ch == b'$' {
                // Skip past the '$'
                i += 1;
                match ctx.input[i] {
                    b'S' => write!(ctx, "{}", '/')?,
                    b'_' => write!(ctx, "{}", '.')?,
                    b'$' => write!(ctx, "{}", '$')?,
                    _ => {
                        // Fall through
                    }
                }
            } else {
                write!(ctx, "{}", ch as char)?;
            }
            i += 1;
        }

        Ok(())
    }
}

/// The subobject expression production.
///
/// <expression> ::= so <referent type> <expr> [<offset number>] <union-selector>* [p] E
/// <union-selector> ::= _ [<number>]
///
/// Not yet in the spec: https://github.com/itanium-cxx-abi/cxx-abi/issues/47
/// But it has been shipping in clang for some time.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SubobjectExpr {
    ty: TypeHandle,
    expr: Box<Expression>,
    offset: isize,
}

impl Parse for SubobjectExpr {
    fn parse<'a, 'b>(
        ctx: &'a ParseContext,
        subs: &'a mut SubstitutionTable,
        input: IndexStr<'b>,
    ) -> Result<(SubobjectExpr, IndexStr<'b>)> {
        try_begin_parse!("SubobjectExpr", ctx, input);

        let (ty, tail) = TypeHandle::parse(ctx, subs, input)?;
        let (expr, tail) = Expression::parse(ctx, subs, tail)?;
        let (offset, tail) = parse_number(10, true, tail).unwrap_or((0, tail));

        // XXXkhuey handle union-selector and [p]
        let tail = consume(b"E", tail)?;
        Ok((
            SubobjectExpr {
                ty: ty,
                expr: Box::new(expr),
                offset: offset,
            },
            tail,
        ))
    }
}

impl<'subs, W> Demangle<'subs, W> for SubobjectExpr
where
    W: 'subs + DemangleWrite,
{
    #[inline]
    fn demangle<'prev, 'ctx>(
        &'subs self,
        ctx: &'ctx mut DemangleContext<'subs, W>,
        scope: Option<ArgScopeStack<'prev, 'subs>>,
    ) -> fmt::Result {
        let ctx = try_begin_demangle!(self, ctx, scope);

        self.expr.demangle(ctx, scope)?;
        write!(ctx, ".<")?;
        self.ty.demangle(ctx, scope)?;
        write!(ctx, " at offset {}>", self.offset)
    }
}

/// Expect and consume the given byte str, and return the advanced `IndexStr` if
/// we saw the expectation. Otherwise return an error of kind
/// `error::Error::UnexpectedText` if the input doesn't match, or
/// `error::Error::UnexpectedEnd` if it isn't long enough.
#[inline]
fn consume<'a>(expected: &[u8], input: IndexStr<'a>) -> Result<IndexStr<'a>> {
    match input.try_split_at(expected.len()) {
        Some((head, tail)) if head == expected => Ok(tail),
        Some(_) => Err(error::Error::UnexpectedText),
        None => Err(error::Error::UnexpectedEnd),
    }
}

fn one_or_more<'a, 'b, P>(
    ctx: &'a ParseContext,
    subs: &'a mut SubstitutionTable,
    input: IndexStr<'b>,
) -> Result<(Vec<P>, IndexStr<'b>)>
where
    P: Parse,
{
    let (first, mut tail) = P::parse(ctx, subs, input)?;
    let mut results = vec![first];
    loop {
        if let Ok((parsed, tail_tail)) = try_recurse!(P::parse(ctx, subs, tail)) {
            results.push(parsed);
            tail = tail_tail;
        } else {
            return Ok((results, tail));
        }
    }
}

fn zero_or_more<'a, 'b, P>(
    ctx: &'a ParseContext,
    subs: &'a mut SubstitutionTable,
    input: IndexStr<'b>,
) -> Result<(Vec<P>, IndexStr<'b>)>
where
    P: Parse,
{
    let mut tail = input;
    let mut results = vec![];
    loop {
        if let Ok((parsed, tail_tail)) = try_recurse!(P::parse(ctx, subs, tail)) {
            results.push(parsed);
            tail = tail_tail;
        } else {
            return Ok((results, tail));
        }
    }
}

/// Parse a number with the given `base`. Do not allow negative numbers
/// (prefixed with an 'n' instead of a '-') if `allow_signed` is false.
#[allow(unsafe_code)]
fn parse_number(base: u32, allow_signed: bool, mut input: IndexStr) -> Result<(isize, IndexStr)> {
    if input.is_empty() {
        return Err(error::Error::UnexpectedEnd);
    }

    let num_is_negative = if allow_signed && input.as_ref()[0] == b'n' {
        input = input.range_from(1..);

        if input.is_empty() {
            return Err(error::Error::UnexpectedEnd);
        }

        true
    } else {
        false
    };

    let num_numeric = input
        .as_ref()
        .iter()
        .map(|&c| c as char)
        .take_while(|c| c.is_digit(base) && (c.is_numeric() || c.is_uppercase()))
        .count();
    if num_numeric == 0 {
        return Err(error::Error::UnexpectedText);
    }

    let (head, tail) = input.split_at(num_numeric);
    let head = head.as_ref();

    if num_numeric > 1 && head[0] == b'0' {
        // "<number>s appearing in mangled names never have leading zeroes,
        // except for the value zero, represented as '0'."
        return Err(error::Error::UnexpectedText);
    }

    let head = unsafe {
        // Safe because we know we only have valid numeric chars in this
        // slice, which are valid UTF-8.
        str::from_utf8_unchecked(head)
    };

    let mut number = isize::from_str_radix(head, base).map_err(|_| error::Error::Overflow)?;
    if num_is_negative {
        number = -number;
    }

    Ok((number, tail))
}

#[cfg(test)]
mod tests {
    use super::{
        AbiTag, AbiTags, ArrayType, BareFunctionType, BaseUnresolvedName, BuiltinType, CallOffset,
        ClassEnumType, ClosureTypeName, CtorDtorName, CvQualifiers, DataMemberPrefix, Decltype,
        DestructorName, Discriminator, Encoding, ExceptionSpec, ExprPrimary, Expression,
        FunctionParam, FunctionType, GlobalCtorDtor, Identifier, Initializer, LambdaSig, LocalName,
        MangledName, MemberName, Name, NestedName, NonSubstitution, Number, NvOffset, OperatorName,
        ParametricBuiltinType, Parse, ParseContext, PointerToMemberType, Prefix, PrefixHandle,
        RefQualifier, ResourceName, SeqId, SimpleId, SimpleOperatorName, SourceName, SpecialName,
        StandardBuiltinType, SubobjectExpr, Substitution, TemplateArg, TemplateArgs, TemplateParam,
        TemplateTemplateParam, TemplateTemplateParamHandle, Type, TypeHandle, UnnamedTypeName,
        UnqualifiedName, UnresolvedName, UnresolvedQualifierLevel, UnresolvedType,
        UnresolvedTypeHandle, UnscopedName, UnscopedTemplateName, UnscopedTemplateNameHandle,
        VOffset, VectorType, WellKnownComponent,
    };

    use crate::error::Error;
    use crate::index_str::IndexStr;
    use crate::subs::{Substitutable, SubstitutionTable};
    use alloc::boxed::Box;
    use alloc::string::String;
    use core::fmt::Debug;
    use core::iter::FromIterator;

    fn assert_parse_ok<P, S1, S2, I1, I2>(
        production: &'static str,
        subs: S1,
        input: I1,
        expected: P,
        expected_tail: I2,
        expected_new_subs: S2,
    ) where
        P: Debug + Parse + PartialEq,
        S1: AsRef<[Substitutable]>,
        S2: AsRef<[Substitutable]>,
        I1: AsRef<[u8]>,
        I2: AsRef<[u8]>,
    {
        let ctx = ParseContext::new(Default::default());
        let input = input.as_ref();
        let expected_tail = expected_tail.as_ref();

        let expected_subs = SubstitutionTable::from_iter(
            subs.as_ref()
                .iter()
                .cloned()
                .chain(expected_new_subs.as_ref().iter().cloned()),
        );
        let mut subs = SubstitutionTable::from_iter(subs.as_ref().iter().cloned());

        match P::parse(&ctx, &mut subs, IndexStr::from(input)) {
            Err(error) => panic!(
                "Parsing {:?} as {} failed: {}",
                String::from_utf8_lossy(input),
                production,
                error
            ),
            Ok((value, tail)) => {
                if value != expected {
                    panic!(
                        "Parsing {:?} as {} produced\n\n{:#?}\n\nbut we expected\n\n{:#?}",
                        String::from_utf8_lossy(input),
                        production,
                        value,
                        expected
                    );
                }
                if tail != expected_tail {
                    panic!(
                        "Parsing {:?} as {} left a tail of {:?}, expected {:?}",
                        String::from_utf8_lossy(input),
                        production,
                        tail,
                        String::from_utf8_lossy(expected_tail)
                    );
                }
                if subs[..] != expected_subs[..] {
                    panic!(
                        "Parsing {:?} as {} produced a substitutions table of\n\n\
                         {:#?}\n\n\
                         but we expected\n\n\
                         {:#?}",
                        String::from_utf8_lossy(input),
                        production,
                        subs,
                        expected_subs
                    );
                }
            }
        }

        log!("=== assert_parse_ok PASSED ====================================");
    }

    fn simple_assert_parse_ok<P, I1, I2>(
        production: &'static str,
        input: I1,
        expected: P,
        expected_tail: I2,
    ) where
        P: Debug + Parse + PartialEq,
        I1: AsRef<[u8]>,
        I2: AsRef<[u8]>,
    {
        assert_parse_ok::<P, _, _, _, _>(production, [], input, expected, expected_tail, []);
    }

    fn assert_parse_err<P, S, I>(production: &'static str, subs: S, input: I, expected_error: Error)
    where
        P: Debug + Parse + PartialEq,
        S: AsRef<[Substitutable]>,
        I: AsRef<[u8]>,
    {
        let input = input.as_ref();
        let ctx = ParseContext::new(Default::default());
        let mut subs = SubstitutionTable::from_iter(subs.as_ref().iter().cloned());

        match P::parse(&ctx, &mut subs, IndexStr::from(input)) {
            Err(ref error) if *error == expected_error => {}
            Err(ref error) => {
                panic!(
                    "Parsing {:?} as {} produced an error of kind {:?}, but we expected kind {:?}",
                    String::from_utf8_lossy(input),
                    production,
                    error,
                    expected_error
                );
            }
            Ok((value, tail)) => {
                panic!(
                    "Parsing {:?} as {} produced value\
                     \n\n\
                     {:#?}\
                     \n\n\
                     and tail {:?}, but we expected error kind {:?}",
                    String::from_utf8_lossy(input),
                    production,
                    value,
                    tail,
                    expected_error
                );
            }
        }

        log!("=== assert_parse_err PASSED ===================================");
    }

    fn simple_assert_parse_err<P, I>(production: &'static str, input: I, expected_error: Error)
    where
        P: Debug + Parse + PartialEq,
        I: AsRef<[u8]>,
    {
        assert_parse_err::<P, _, _>(production, [], input, expected_error);
    }

    #[test]
    fn recursion_limit() {
        // Build the mangled symbol for the type `*****char` where the "*****"
        // is 10,000 pointer indirections. This is a valid type symbol, but
        // something that would cause us to blow the stack.
        let mut mangled = String::new();
        for _ in 0..10_000 {
            mangled.push('P');
        }
        mangled += "c";

        simple_assert_parse_err::<TypeHandle, _>("TypeHandle", mangled, Error::TooMuchRecursion);
    }

    macro_rules! assert_parse {
        ( $production:ident {
            $( with subs $subs:expr => {
                Ok => {
                    $( $input:expr => {
                        $expected:expr ,
                        $expected_tail:expr ,
                        $expected_new_subs:expr
                    } )*
                }
                Err => {
                    $( $error_input:expr => $error:expr , )*
                }
            } )*
        } ) => {
            $( $(
                assert_parse_ok::<$production, _, _, _, _>(stringify!($production),
                                                           $subs,
                                                           $input,
                                                           $expected,
                                                           $expected_tail,
                                                           $expected_new_subs);
            )* )*

            $( $(
                assert_parse_err::<$production, _, _>(stringify!($production),
                                                      $subs,
                                                      $error_input,
                                                      $error);
            )* )*
        };

        ( $production:ident {
            Ok => {
                $( $input:expr => {
                    $expected:expr ,
                    $expected_tail:expr
                } )*
            }
            Err => {
                $( $error_input:expr => $error:expr , )*
            }
        } ) => {
            $(
                simple_assert_parse_ok::<$production, _, _>(stringify!($production),
                                                            $input,
                                                            $expected,
                                                            $expected_tail);
            )*


            $(
                simple_assert_parse_err::<$production, _>(stringify!($production),
                                                          $error_input,
                                                          $error);
            )*
        };
    }

    #[test]
    fn parse_mangled_name() {
        assert_parse!(MangledName {
            Ok => {
                b"_Z3foo..." => {
                    MangledName::Encoding(
                        Encoding::Data(
                            Name::Unscoped(
                                UnscopedName::Unqualified(
                                    UnqualifiedName::Source(
                                        SourceName(Identifier {
                                            start: 3,
                                            end: 6,
                                        }),
                                        AbiTags::default())))), vec![]),
                    b"..."
                }
                b"_GLOBAL__I__Z3foo..." => {
                    MangledName::GlobalCtorDtor(
                        GlobalCtorDtor::Ctor(
                            Box::new(
                                MangledName::Encoding(
                                    Encoding::Data(
                                        Name::Unscoped(
                                            UnscopedName::Unqualified(
                                                UnqualifiedName::Source(
                                                    SourceName(
                                                        Identifier {
                                                            start: 14,
                                                            end: 17,
                                                        }),
                                                    AbiTags::default())))), vec![])))),
                    b"..."
                }
            }
            Err => {
                b"_Y" => Error::UnexpectedText,
                b"_Z" => Error::UnexpectedEnd,
                b"_" => Error::UnexpectedEnd,
                b"" => Error::UnexpectedEnd,
                b"_GLOBAL_" => Error::UnexpectedEnd,
            }
        });
    }

    #[test]
    fn parse_encoding() {
        assert_parse!(Encoding {
            with subs [] => {
                Ok => {
                    b"3fooi..." => {
                        Encoding::Function(
                            Name::Unscoped(
                                UnscopedName::Unqualified(
                                    UnqualifiedName::Source(
                                        SourceName(Identifier {
                                            start: 1,
                                            end: 4,
                                        }),
                                        AbiTags::default()))),
                            BareFunctionType(vec![
                                TypeHandle::Builtin(BuiltinType::Standard(StandardBuiltinType::Int))
                            ])),
                        b"...",
                        []
                    }
                    b"3foo..." => {
                        Encoding::Data(
                            Name::Unscoped(
                                UnscopedName::Unqualified(
                                    UnqualifiedName::Source(
                                        SourceName(Identifier {
                                            start: 1,
                                            end: 4,
                                        }),
                                        AbiTags::default())))),
                        b"...",
                        []
                    }
                    b"GV3abc..." => {
                        Encoding::Special(
                            SpecialName::Guard(
                                Name::Unscoped(
                                    UnscopedName::Unqualified(
                                        UnqualifiedName::Source(
                                            SourceName(Identifier {
                                                start: 3,
                                                end: 6,
                                            }),
                                            AbiTags::default()))))),
                        b"...",
                        []
                    }
                }
                Err => {
                    b"zzz" => Error::UnexpectedText,
                    b"" => Error::UnexpectedEnd,
                }
            }
        });
    }

    #[test]
    fn parse_global_ctor_dtor() {
        assert_parse!(GlobalCtorDtor {
            Ok => {
                b"_I__Z3foo..." => {
                    GlobalCtorDtor::Ctor(
                        Box::new(
                            MangledName::Encoding(
                                Encoding::Data(
                                    Name::Unscoped(
                                        UnscopedName::Unqualified(
                                            UnqualifiedName::Source(
                                                SourceName(
                                                    Identifier {
                                                        start: 6,
                                                        end: 9,
                                                    }),
                                                AbiTags::default())))), vec![]))),
                    b"..."
                }
                b".I__Z3foo..." => {
                    GlobalCtorDtor::Ctor(
                        Box::new(
                            MangledName::Encoding(
                                Encoding::Data(
                                    Name::Unscoped(
                                        UnscopedName::Unqualified(
                                            UnqualifiedName::Source(
                                                SourceName(
                                                    Identifier {
                                                        start: 6,
                                                        end: 9,
                                                    }),
                                                AbiTags::default())))), vec![]))),
                    b"..."
                }
                b"$I__Z3foo..." => {
                    GlobalCtorDtor::Ctor(
                        Box::new(
                            MangledName::Encoding(
                                Encoding::Data(
                                    Name::Unscoped(
                                        UnscopedName::Unqualified(
                                            UnqualifiedName::Source(
                                                SourceName(
                                                    Identifier {
                                                        start: 6,
                                                        end: 9,
                                                    }),
                                                AbiTags::default())))), vec![]))),
                    b"..."
                }
                b"_D__Z3foo..." => {
                    GlobalCtorDtor::Dtor(
                        Box::new(
                            MangledName::Encoding(
                                Encoding::Data(
                                    Name::Unscoped(
                                        UnscopedName::Unqualified(
                                            UnqualifiedName::Source(
                                                SourceName(
                                                    Identifier {
                                                        start: 6,
                                                        end: 9,
                                                    }),
                                                AbiTags::default())))), vec![]))),
                    b"..."
                }
                b".D__Z3foo..." => {
                    GlobalCtorDtor::Dtor(
                        Box::new(
                            MangledName::Encoding(
                                Encoding::Data(
                                    Name::Unscoped(
                                        UnscopedName::Unqualified(
                                            UnqualifiedName::Source(
                                                SourceName(
                                                    Identifier {
                                                        start: 6,
                                                        end: 9,
                                                    }),
                                                AbiTags::default())))), vec![]))),
                    b"..."
                }
                b"$D__Z3foo..." => {
                    GlobalCtorDtor::Dtor(
                        Box::new(
                            MangledName::Encoding(
                                Encoding::Data(
                                    Name::Unscoped(
                                        UnscopedName::Unqualified(
                                            UnqualifiedName::Source(
                                                SourceName(
                                                    Identifier {
                                                        start: 6,
                                                        end: 9,
                                                    }),
                                                AbiTags::default())))), vec![]))),
                    b"..."
                }
            }
            Err => {
                b"_I" => Error::UnexpectedEnd,
                b"_" => Error::UnexpectedEnd,
                b"" => Error::UnexpectedEnd,
                b"blag" => Error::UnexpectedText,
                b"_J" => Error::UnexpectedText,
                b"_IJ" => Error::UnexpectedText,
            }
        });
    }

    #[test]
    fn parse_name() {
        assert_parse!(Name {
            with subs [
                Substitutable::Prefix(
                    Prefix::Unqualified(
                        UnqualifiedName::Operator(OperatorName::Simple(SimpleOperatorName::New),
                                                  AbiTags::default()))),
                Substitutable::Prefix(
                    Prefix::Nested(PrefixHandle::BackReference(0),
                                   UnqualifiedName::Operator(OperatorName::Simple(SimpleOperatorName::New),
                                                             AbiTags::default()))),
            ] => {
                Ok => {
                    b"NS0_3abcE..." => {
                        Name::Nested(NestedName::Unqualified(CvQualifiers::default(),
                                                             None,
                                                             Some(PrefixHandle::BackReference(1)),
                                                             UnqualifiedName::Source(SourceName(Identifier {
                                                                 start: 5,
                                                                 end: 8,
                                                             }), AbiTags::default()))),
                        b"...",
                        []
                    }
                    b"3abc..." => {
                        Name::Unscoped(
                            UnscopedName::Unqualified(
                                UnqualifiedName::Source(
                                    SourceName(Identifier {
                                        start: 1,
                                        end: 4,
                                    }),
                                    AbiTags::default()))),
                        b"...",
                        []
                    }
                    b"dlIcE..." => {
                        Name::UnscopedTemplate(
                            UnscopedTemplateNameHandle::BackReference(2),
                            TemplateArgs(vec![
                                TemplateArg::Type(
                                    TypeHandle::Builtin(
                                        BuiltinType::Standard(StandardBuiltinType::Char)))
                            ])),
                        b"...",
                        [
                            Substitutable::UnscopedTemplateName(
                                UnscopedTemplateName(
                                    UnscopedName::Unqualified(
                                        UnqualifiedName::Operator(
                                            OperatorName::Simple(
                                                SimpleOperatorName::Delete),
                                            AbiTags::default())))),
                        ]
                    }
                    b"Z3abcEs..." => {
                        Name::Local(
                            LocalName::Relative(
                                Box::new(Encoding::Data(
                                    Name::Unscoped(
                                        UnscopedName::Unqualified(
                                            UnqualifiedName::Source(
                                                SourceName(Identifier {
                                                    start: 2,
                                                    end: 5,
                                                }),
                                                AbiTags::default()))))),
                                None,
                                None)),
                        b"...",
                        []
                    }
                }
                Err => {
                    b"zzz" => Error::UnexpectedText,
                    b"" => Error::UnexpectedEnd,
                }
            }
        });
    }

    #[test]
    fn parse_unscoped_template_name_handle() {
        assert_parse!(UnscopedTemplateNameHandle {
            with subs [
                Substitutable::UnscopedTemplateName(
                    UnscopedTemplateName(
                        UnscopedName::Unqualified(
                            UnqualifiedName::Operator(
                                OperatorName::Simple(
                                    SimpleOperatorName::New),
                                AbiTags::default())))),
            ] => {
                Ok => {
                    b"S_..." => {
                        UnscopedTemplateNameHandle::BackReference(0),
                        b"...",
                        []
                    }
                    b"dl..." => {
                        UnscopedTemplateNameHandle::BackReference(1),
                        b"...",
                        [
                            Substitutable::UnscopedTemplateName(
                                UnscopedTemplateName(
                                    UnscopedName::Unqualified(
                                        UnqualifiedName::Operator(
                                            OperatorName::Simple(
                                                SimpleOperatorName::Delete),
                                            AbiTags::default()))))
                        ]
                    }
                }
                Err => {
                    b"zzzz" => Error::UnexpectedText,
                    b"" => Error::UnexpectedEnd,
                }
            }
        });
    }

    #[test]
    fn parse_nested_name() {
        // <nested-name> ::= N [<CV-qualifiers>] [<ref-qualifier>] <prefix> <unqualified-name> E
        //               ::= N [<CV-qualifiers>] [<ref-qualifier>] <template-prefix> <template-args> E
        assert_parse!(NestedName {
            with subs [
                Substitutable::Prefix(
                    Prefix::Unqualified(
                        UnqualifiedName::Operator(
                            OperatorName::Simple(
                                SimpleOperatorName::New),
                            AbiTags::default()))),
            ] => {
                Ok => {
                    b"NKOS_3abcE..." => {
                        NestedName::Unqualified(
                            CvQualifiers {
                                restrict: false,
                                volatile: false,
                                const_: true,
                            },
                            Some(RefQualifier::RValueRef),
                            Some(PrefixHandle::BackReference(0)),
                            UnqualifiedName::Source(
                                SourceName(Identifier {
                                    start: 6,
                                    end: 9,
                                }),
                                AbiTags::default())),
                        b"...",
                        []
                    }
                    b"NOS_3abcE..." => {
                        NestedName::Unqualified(
                            CvQualifiers {
                                restrict: false,
                                volatile: false,
                                const_: false,
                            },
                            Some(RefQualifier::RValueRef),
                            Some(PrefixHandle::BackReference(0)),
                            UnqualifiedName::Source(
                                SourceName(Identifier {
                                    start: 5,
                                    end: 8,
                                }), AbiTags::default())),
                        b"...",
                        []
                    }
                    b"NS_3abcE..." => {
                        NestedName::Unqualified(
                            CvQualifiers {
                                restrict: false,
                                volatile: false,
                                const_: false,
                            },
                            None,
                            Some(PrefixHandle::BackReference(0)),
                            UnqualifiedName::Source(
                                SourceName(Identifier {
                                    start: 4,
                                    end: 7,
                                }), AbiTags::default())),
                        b"...",
                        []
                    }
                    b"NK1fE..." => {
                        NestedName::Unqualified(
                            CvQualifiers {
                                restrict: false,
                                volatile: false,
                                const_: true,
                            },
                            None,
                            None,
                            UnqualifiedName::Source(
                                SourceName(Identifier {
                                    start: 3,
                                    end: 4,
                                }),
                                AbiTags::default())),
                        b"...",
                        []
                    }
                    b"NKOS_3abcIJEEE..." => {
                        NestedName::Template(
                            CvQualifiers {
                                restrict: false,
                                volatile: false,
                                const_: true,
                            },
                            Some(RefQualifier::RValueRef),
                            PrefixHandle::NonSubstitution(NonSubstitution(0))),
                        b"...",
                        [
                            Substitutable::Prefix(
                                Prefix::Nested(
                                    PrefixHandle::BackReference(0),
                                    UnqualifiedName::Source(
                                        SourceName(Identifier {
                                            start: 6,
                                            end: 9,
                                        }),
                                        AbiTags::default()))),
                        ]
                    }
                    b"NOS_3abcIJEEE..." => {
                        NestedName::Template(
                            CvQualifiers {
                                restrict: false,
                                volatile: false,
                                const_: false,
                            },
                            Some(RefQualifier::RValueRef),
                            PrefixHandle::NonSubstitution(NonSubstitution(0))),
                        b"...",
                        [
                            Substitutable::Prefix(
                                Prefix::Nested(
                                    PrefixHandle::BackReference(0),
                                    UnqualifiedName::Source(
                                        SourceName(Identifier {
                                            start: 5,
                                            end: 8,
                                        }),
                                        AbiTags::default()))),
                        ]
                    }
                    b"NS_3abcIJEEE..." => {
                        NestedName::Template(
                            CvQualifiers {
                                restrict: false,
                                volatile: false,
                                const_: false,
                            },
                            None,
                            PrefixHandle::NonSubstitution(NonSubstitution(0))),
                        b"...",
                        [
                            Substitutable::Prefix(
                                Prefix::Nested(
                                    PrefixHandle::BackReference(0),
                                    UnqualifiedName::Source(
                                        SourceName(Identifier {
                                            start: 4,
                                            end: 7,
                                        }),
                                        AbiTags::default()))),
                        ]
                    }
                }
                Err => {
                    // Ends with a prefix that is not a name or template.
                    b"NS_DttrEE..." => Error::UnexpectedText,

                    b"zzz" => Error::UnexpectedText,
                    b"Nzzz" => Error::UnexpectedText,
                    b"NKzzz" => Error::UnexpectedText,
                    b"NKOzzz" => Error::UnexpectedText,
                    b"NKO3abczzz" => Error::UnexpectedText,
                    b"NKO3abc3abczzz" => Error::UnexpectedText,
                    b"" => Error::UnexpectedEnd,
                    b"N" => Error::UnexpectedEnd,
                    b"NK" => Error::UnexpectedEnd,
                    b"NKO" => Error::UnexpectedEnd,
                    b"NKO3abc" => Error::UnexpectedEnd,
                    b"NKO3abc3abc" => Error::UnexpectedEnd,
                }
            }
        });
    }

    #[test]
    fn parse_prefix_handle() {
        // <prefix> ::= <unqualified-name>
        //          ::= <prefix> <unqualified-name>
        //          ::= <template-prefix> <template-args>
        //          ::= <template-param>
        //          ::= <decltype>
        //          ::= <prefix> <data-member-prefix>
        //          ::= <substitution>
        assert_parse!(PrefixHandle {
            with subs [
                Substitutable::Prefix(
                    Prefix::Unqualified(
                        UnqualifiedName::Operator(
                            OperatorName::Simple(
                                SimpleOperatorName::New),
                            AbiTags::default()))),
            ] => {
                Ok => {
                    b"3foo..." => {
                        PrefixHandle::BackReference(1),
                        b"...",
                        [
                            Substitutable::Prefix(
                                Prefix::Unqualified(
                                    UnqualifiedName::Source(
                                        SourceName(Identifier {
                                            start: 1,
                                            end: 4,
                                        }),
                                        AbiTags::default())))
                        ]
                    }
                    b"3abc3def..." => {
                        PrefixHandle::BackReference(2),
                        b"...",
                        [
                            Substitutable::Prefix(
                                Prefix::Unqualified(
                                    UnqualifiedName::Source(
                                        SourceName(Identifier {
                                            start: 1,
                                            end: 4,
                                        }),
                                    AbiTags::default()))),
                            Substitutable::Prefix(
                                Prefix::Nested(
                                    PrefixHandle::BackReference(1),
                                    UnqualifiedName::Source(
                                        SourceName(Identifier {
                                            start: 5,
                                            end: 8,
                                        }),
                                    AbiTags::default()))),
                        ]
                    }
                    b"3fooIJEE..." => {
                        PrefixHandle::BackReference(2),
                        b"...",
                        [
                            Substitutable::Prefix(
                                Prefix::Unqualified(
                                    UnqualifiedName::Source(
                                        SourceName(Identifier {
                                            start: 1,
                                            end: 4,
                                        }),
                                        AbiTags::default()))),
                            Substitutable::Prefix(
                                Prefix::Template(PrefixHandle::BackReference(1),
                                                 TemplateArgs(vec![
                                                     TemplateArg::ArgPack(vec![]),
                                                 ])))
                        ]
                    }
                    b"T_..." => {
                        PrefixHandle::BackReference(1),
                        b"...",
                        [
                            Substitutable::Prefix(Prefix::TemplateParam(TemplateParam(0))),
                        ]
                    }
                    b"DTtrE..." => {
                        PrefixHandle::BackReference(1),
                        b"...",
                        [
                            Substitutable::Prefix(
                                Prefix::Decltype(
                                    Decltype::Expression(Expression::Rethrow))),
                        ]
                    }
                    b"3abc3defM..." => {
                        PrefixHandle::BackReference(2),
                        b"...",
                        [
                            Substitutable::Prefix(
                                Prefix::Unqualified(
                                    UnqualifiedName::Source(
                                        SourceName(Identifier {
                                            start: 1,
                                            end: 4,
                                        }),
                                        AbiTags::default()))),
                            Substitutable::Prefix(
                                Prefix::DataMember(
                                    PrefixHandle::BackReference(1),
                                    DataMemberPrefix(
                                        SourceName(Identifier {
                                            start: 5,
                                            end: 8,
                                        })))),
                        ]
                    }
                    b"S_..." => {
                        PrefixHandle::BackReference(0),
                        b"...",
                        []
                    }
                    // The trailing E and <nested-name> case...
                    b"3abc3defE..." => {
                        PrefixHandle::NonSubstitution(NonSubstitution(0)),
                        b"E...",
                        [
                            Substitutable::Prefix(
                                Prefix::Unqualified(
                                    UnqualifiedName::Source(
                                        SourceName(Identifier {
                                            start: 1,
                                            end: 4,
                                        }),
                                    AbiTags::default()))),
                        ]
                    }
                }
                Err => {
                    b"zzz" => Error::UnexpectedText,
                    b"" => Error::UnexpectedEnd,
                }
            }
        });
    }

    #[test]
    fn parse_type_handle() {
        assert_parse!(TypeHandle {
            with subs [
                Substitutable::Type(
                    Type::PointerTo(
                        TypeHandle::Builtin(BuiltinType::Standard(StandardBuiltinType::Char)))),
            ] => {
                Ok => {
                    b"S_..." => {
                        TypeHandle::BackReference(0),
                        b"...",
                        []
                    }
                    b"c..." => {
                        TypeHandle::Builtin(BuiltinType::Standard(StandardBuiltinType::Char)),
                        b"...",
                        []
                    }
                    b"FS_E..." => {
                        TypeHandle::BackReference(1),
                        b"...",
                        [
                            Substitutable::Type(
                                Type::Function(FunctionType {
                                    cv_qualifiers: CvQualifiers {
                                        restrict: false,
                                        volatile: false,
                                        const_: false,
                                    },
                                    exception_spec: None,
                                    transaction_safe: false,
                                    extern_c: false,
                                    bare: BareFunctionType(vec![TypeHandle::BackReference(0)]),
                                    ref_qualifier: None,
                                })),
                        ]
                    }
                    b"A_S_..." => {
                        TypeHandle::BackReference(1),
                        b"...",
                        [
                            Substitutable::Type(
                                Type::Array(ArrayType::NoDimension(TypeHandle::BackReference(0)))),
                        ]
                    }
                    b"MS_S_..." => {
                        TypeHandle::BackReference(1),
                        b"...",
                        [
                            Substitutable::Type(
                                Type::PointerToMember(
                                    PointerToMemberType(TypeHandle::BackReference(0),
                                                        TypeHandle::BackReference(0)))),
                        ]
                    }
                    b"T_..." => {
                        TypeHandle::BackReference(1),
                        b"...",
                        [
                            Substitutable::Type(Type::TemplateParam(TemplateParam(0))),
                        ]
                    }
                    b"T_IS_E..." => {
                        TypeHandle::BackReference(2),
                        b"...",
                        [
                            Substitutable::TemplateTemplateParam(
                                TemplateTemplateParam(TemplateParam(0))),
                            Substitutable::Type(
                                Type::TemplateTemplate(
                                    TemplateTemplateParamHandle::BackReference(1),
                                    TemplateArgs(vec![
                                        TemplateArg::Type(TypeHandle::BackReference(0))
                                    ]))),
                        ]
                    }
                    b"DTtrE..." => {
                        TypeHandle::BackReference(1),
                        b"...",
                        [
                            Substitutable::Type(
                                Type::Decltype(Decltype::Expression(Expression::Rethrow))),
                        ]
                    }
                    b"KS_..." => {
                        TypeHandle::BackReference(1),
                        b"...",
                        [
                            Substitutable::Type(Type::Qualified(CvQualifiers {
                                restrict: false,
                                volatile: false,
                                const_: true,
                            }, TypeHandle::BackReference(0)))
                        ]
                    }
                    b"PS_..." => {
                        TypeHandle::BackReference(1),
                        b"...",
                        [
                            Substitutable::Type(Type::PointerTo(TypeHandle::BackReference(0)))
                        ]
                    }
                    b"RS_..." => {
                        TypeHandle::BackReference(1),
                        b"...",
                        [
                            Substitutable::Type(Type::LvalueRef(TypeHandle::BackReference(0)))
                        ]
                    }
                    b"OS_..." => {
                        TypeHandle::BackReference(1),
                        b"...",
                        [
                            Substitutable::Type(Type::RvalueRef(TypeHandle::BackReference(0)))
                        ]
                    }
                    b"CS_..." => {
                        TypeHandle::BackReference(1),
                        b"...",
                        [
                            Substitutable::Type(Type::Complex(TypeHandle::BackReference(0)))
                        ]
                    }
                    b"GS_..." => {
                        TypeHandle::BackReference(1),
                        b"...",
                        [
                            Substitutable::Type(Type::Imaginary(TypeHandle::BackReference(0)))
                        ]
                    }
                    b"U3abcS_..." => {
                        TypeHandle::BackReference(1),
                        b"...",
                        [
                            Substitutable::Type(
                                Type::VendorExtension(
                                    SourceName(Identifier {
                                        start: 2,
                                        end: 5,
                                    }),
                                    None,
                                    TypeHandle::BackReference(0)))
                        ]
                    }
                    b"U3abcIS_ES_..." => {
                        TypeHandle::BackReference(1),
                        b"...",
                        [
                            Substitutable::Type(
                                Type::VendorExtension(
                                    SourceName(Identifier {
                                        start: 2,
                                        end: 5,
                                    }),
                                    Some(TemplateArgs(vec![
                                        TemplateArg::Type(TypeHandle::BackReference(0))
                                    ])),
                                    TypeHandle::BackReference(0)))
                        ]
                    }
                    b"DpS_..." => {
                        TypeHandle::BackReference(1),
                        b"...",
                        [
                            Substitutable::Type(
                                Type::PackExpansion(TypeHandle::BackReference(0))),
                        ]
                    }
                    b"3abc..." => {
                        TypeHandle::BackReference(1),
                        b"...",
                        [
                            Substitutable::Type(
                                Type::ClassEnum(
                                    ClassEnumType::Named(
                                        Name::Unscoped(
                                            UnscopedName::Unqualified(
                                                UnqualifiedName::Source(
                                                    SourceName(Identifier {
                                                        start: 1,
                                                        end: 4,
                                                    }),
                                                    AbiTags::default()))))))
                        ]
                    }
                }
                Err => {
                    b"P" => Error::UnexpectedEnd,
                    b"R" => Error::UnexpectedEnd,
                    b"O" => Error::UnexpectedEnd,
                    b"C" => Error::UnexpectedEnd,
                    b"G" => Error::UnexpectedEnd,
                    b"Dp" => Error::UnexpectedEnd,
                    b"D" => Error::UnexpectedEnd,
                    b"P" => Error::UnexpectedEnd,
                    b"UlvE_" => Error::UnexpectedText,
                    b"" => Error::UnexpectedEnd,
                }
            }
        });
    }

    #[test]
    fn parse_exception_spec() {
        assert_parse!(ExceptionSpec {
            Ok => {
                b"Do..." => {
                    ExceptionSpec::NoExcept,
                    b"..."
                }
                b"DOtrE..." => {
                    ExceptionSpec::Computed(Expression::Rethrow),
                    b"..."
                }
            }
            Err => {
                b"DOtre" => Error::UnexpectedText,
                b"DOE" => Error::UnexpectedText,
                b"D" => Error::UnexpectedEnd,
                b"" => Error::UnexpectedEnd,
            }
        });
    }

    #[test]
    fn parse_function_type() {
        assert_parse!(FunctionType {
            with subs [
                Substitutable::Type(
                    Type::PointerTo(
                        TypeHandle::Builtin(BuiltinType::Standard(StandardBuiltinType::Char)))),
            ] => {
                Ok => {
                    b"KDxFYS_RE..." => {
                        FunctionType {
                            cv_qualifiers: CvQualifiers {
                                restrict: false,
                                volatile: false,
                                const_: true,
                            },
                            exception_spec: None,
                            transaction_safe: true,
                            extern_c: true,
                            bare: BareFunctionType(vec![TypeHandle::BackReference(0)]),
                            ref_qualifier: Some(RefQualifier::LValueRef),
                        },
                        b"...",
                        []
                    }
                    b"DxFYS_RE..." => {
                        FunctionType {
                            cv_qualifiers: CvQualifiers {
                                restrict: false,
                                volatile: false,
                                const_: false,
                            },
                            exception_spec: None,
                            transaction_safe: true,
                            extern_c: true,
                            bare: BareFunctionType(vec![TypeHandle::BackReference(0)]),
                            ref_qualifier: Some(RefQualifier::LValueRef),
                        },
                        b"...",
                        []
                    }
                    b"FYS_RE..." => {
                        FunctionType {
                            cv_qualifiers: CvQualifiers {
                                restrict: false,
                                volatile: false,
                                const_: false,
                            },
                            exception_spec: None,
                            transaction_safe: false,
                            extern_c: true,
                            bare: BareFunctionType(vec![TypeHandle::BackReference(0)]),
                            ref_qualifier: Some(RefQualifier::LValueRef),
                        },
                        b"...",
                        []
                    }
                    b"FS_RE..." => {
                        FunctionType {
                            cv_qualifiers: CvQualifiers {
                                restrict: false,
                                volatile: false,
                                const_: false,
                            },
                            exception_spec: None,
                            transaction_safe: false,
                            extern_c: false,
                            bare: BareFunctionType(vec![TypeHandle::BackReference(0)]),
                            ref_qualifier: Some(RefQualifier::LValueRef),
                        },
                        b"...",
                        []
                    }
                    b"FS_E..." => {
                        FunctionType {
                            cv_qualifiers: CvQualifiers {
                                restrict: false,
                                volatile: false,
                                const_: false,
                            },
                            exception_spec: None,
                            transaction_safe: false,
                            extern_c: false,
                            bare: BareFunctionType(vec![TypeHandle::BackReference(0)]),
                            ref_qualifier: None,
                        },
                        b"...",
                        []
                    }
                }
                Err => {
                    b"DFYS_E" => Error::UnexpectedText,
                    b"KKFS_E" => Error::UnexpectedText,
                    b"FYS_..." => Error::UnexpectedText,
                    b"FYS_" => Error::UnexpectedEnd,
                    b"F" => Error::UnexpectedEnd,
                    b"" => Error::UnexpectedEnd,
                }
            }
        });
    }

    #[test]
    fn parse_bare_function_type() {
        assert_parse!(BareFunctionType {
            with subs [
                Substitutable::Type(
                    Type::PointerTo(
                        TypeHandle::Builtin(BuiltinType::Standard(StandardBuiltinType::Char)))),
            ] => {
                Ok => {
                    b"S_S_..." => {
                        BareFunctionType(vec![
                            TypeHandle::BackReference(0),
                            TypeHandle::BackReference(0),
                        ]),
                        b"...",
                        []
                    }
                }
                Err => {
                    b"" => Error::UnexpectedEnd,
                }
            }
        });
    }

    #[test]
    fn parse_decltype() {
        assert_parse!(Decltype {
            Ok => {
                b"DTtrE..." => {
                    Decltype::Expression(Expression::Rethrow),
                    b"..."
                }
                b"DttrE..." => {
                    Decltype::IdExpression(Expression::Rethrow),
                    b"..."
                }
            }
            Err => {
                b"Dtrtz" => Error::UnexpectedText,
                b"DTrtz" => Error::UnexpectedText,
                b"Dz" => Error::UnexpectedText,
                b"Dtrt" => Error::UnexpectedText,
                b"DTrt" => Error::UnexpectedText,
                b"Dt" => Error::UnexpectedEnd,
                b"DT" => Error::UnexpectedEnd,
                b"D" => Error::UnexpectedEnd,
                b"" => Error::UnexpectedEnd,
            }
        });
    }

    #[test]
    fn parse_class_enum_type() {
        assert_parse!(ClassEnumType {
            Ok => {
                b"3abc..." => {
                    ClassEnumType::Named(
                        Name::Unscoped(
                            UnscopedName::Unqualified(
                                UnqualifiedName::Source(
                                    SourceName(Identifier {
                                        start: 1,
                                        end: 4,
                                    }),
                                    AbiTags::default())))),
                    b"..."
                }
                b"Ts3abc..." => {
                    ClassEnumType::ElaboratedStruct(
                        Name::Unscoped(
                            UnscopedName::Unqualified(
                                UnqualifiedName::Source(
                                    SourceName(Identifier {
                                        start: 3,
                                        end: 6,
                                    }),
                                    AbiTags::default())))),
                    b"..."
                }
                b"Tu3abc..." => {
                    ClassEnumType::ElaboratedUnion(
                        Name::Unscoped(
                            UnscopedName::Unqualified(
                                UnqualifiedName::Source(
                                    SourceName(Identifier {
                                        start: 3,
                                        end: 6,
                                    }),
                                    AbiTags::default())))),
                    b"..."
                }
                b"Te3abc..." => {
                    ClassEnumType::ElaboratedEnum(
                        Name::Unscoped(
                            UnscopedName::Unqualified(
                                UnqualifiedName::Source(
                                    SourceName(Identifier {
                                        start: 3,
                                        end: 6,
                                    }),
                                AbiTags::default())))),
                    b"..."
                }
            }
            Err => {
                b"zzz" => Error::UnexpectedText,
                b"Tzzz" => Error::UnexpectedText,
                b"T" => Error::UnexpectedEnd,
                b"" => Error::UnexpectedEnd,
            }
        });
    }

    #[test]
    fn parse_array_type() {
        assert_parse!(ArrayType {
            with subs [
                Substitutable::Type(Type::Decltype(Decltype::Expression(Expression::Rethrow)))
            ] => {
                Ok => {
                    b"A10_S_..." => {
                        ArrayType::DimensionNumber(10, TypeHandle::BackReference(0)),
                        b"...",
                        []
                    }
                    b"A10_Sb..." => {
                        ArrayType::DimensionNumber(10,
                                                   TypeHandle::WellKnown(
                                                       WellKnownComponent::StdString1)),
                        b"...",
                        []
                    }
                    b"Atr_S_..." => {
                        ArrayType::DimensionExpression(Expression::Rethrow,
                                                       TypeHandle::BackReference(0)),
                        b"...",
                        []
                    }
                    b"A_S_..." => {
                        ArrayType::NoDimension(TypeHandle::BackReference(0)),
                        b"...",
                        []
                    }
                }
                Err => {
                    b"A10_" => Error::UnexpectedEnd,
                    b"A10" => Error::UnexpectedEnd,
                    b"A" => Error::UnexpectedEnd,
                    b"" => Error::UnexpectedEnd,
                    b"A10_..." => Error::UnexpectedText,
                    b"A10..." => Error::UnexpectedText,
                    b"A..." => Error::UnexpectedText,
                    b"..." => Error::UnexpectedText,
                }
            }
        });
    }

    #[test]
    fn parse_vector_type() {
        assert_parse!(VectorType {
            with subs [
                Substitutable::Type(Type::Decltype(Decltype::Expression(Expression::Rethrow)))
            ] => {
                Ok => {
                    b"Dv10_S_..." => {
                        VectorType::DimensionNumber(10, TypeHandle::BackReference(0)),
                        b"...",
                        []
                    }
                    b"Dv10_Sb..." => {
                        VectorType::DimensionNumber(10,
                                                    TypeHandle::WellKnown(
                                                        WellKnownComponent::StdString1)),
                        b"...",
                        []
                    }
                    b"Dvtr_S_..." => {
                        VectorType::DimensionExpression(Expression::Rethrow,
                                                        TypeHandle::BackReference(0)),
                        b"...",
                        []
                    }
                }
                Err => {
                    b"Dq" => Error::UnexpectedText,
                    b"Dv" => Error::UnexpectedEnd,
                    b"Dv42_" => Error::UnexpectedEnd,
                    b"Dv42_..." => Error::UnexpectedText,
                    b"Dvtr_" => Error::UnexpectedEnd,
                    b"Dvtr_..." => Error::UnexpectedText,
                    b"" => Error::UnexpectedEnd,
                    b"..." => Error::UnexpectedText,
                }
            }
        });
    }

    #[test]
    fn parse_pointer_to_member_type() {
        assert_parse!(PointerToMemberType {
            with subs [
                Substitutable::Type(Type::Decltype(Decltype::Expression(Expression::Rethrow)))
            ] => {
                Ok => {
                    b"MS_S_..." => {
                        PointerToMemberType(TypeHandle::BackReference(0),
                                            TypeHandle::BackReference(0)),
                        b"...",
                        []
                    }
                }
                Err => {
                    b"MS_S" => Error::UnexpectedEnd,
                    b"MS_" => Error::UnexpectedEnd,
                    b"MS" => Error::UnexpectedEnd,
                    b"M" => Error::UnexpectedEnd,
                    b"" => Error::UnexpectedEnd,
                    b"MS_..." => Error::UnexpectedText,
                    b"M..." => Error::UnexpectedText,
                    b"..." => Error::UnexpectedText,
                }
            }
        });
    }

    #[test]
    fn parse_template_template_param_handle() {
        assert_parse!(TemplateTemplateParamHandle {
            with subs [
                Substitutable::TemplateTemplateParam(TemplateTemplateParam(TemplateParam(0)))
            ] => {
                Ok => {
                    b"S_..." => {
                        TemplateTemplateParamHandle::BackReference(0),
                        b"...",
                        []
                    }
                    b"T1_..." => {
                        TemplateTemplateParamHandle::BackReference(1),
                        b"...",
                        [
                            Substitutable::TemplateTemplateParam(TemplateTemplateParam(TemplateParam(2)))
                        ]
                    }
                }
                Err => {
                    b"S" => Error::UnexpectedText,
                    b"T" => Error::UnexpectedEnd,
                    b"" => Error::UnexpectedEnd,
                    b"S..." => Error::UnexpectedText,
                    b"T..." => Error::UnexpectedText,
                    b"..." => Error::UnexpectedText,
                }
            }
        });
    }

    #[test]
    fn parse_template_args() {
        assert_parse!(TemplateArgs {
            with subs [
                Substitutable::Type(Type::Decltype(Decltype::Expression(Expression::Rethrow)))
            ] => {
                Ok => {
                    b"IS_E..." => {
                        TemplateArgs(vec![TemplateArg::Type(TypeHandle::BackReference(0))]),
                        b"...",
                        []
                    }
                    b"IS_S_S_S_E..." => {
                        TemplateArgs(vec![
                            TemplateArg::Type(TypeHandle::BackReference(0)),
                            TemplateArg::Type(TypeHandle::BackReference(0)),
                            TemplateArg::Type(TypeHandle::BackReference(0)),
                            TemplateArg::Type(TypeHandle::BackReference(0)),
                        ]),
                        b"...",
                        []
                    }
                }
                Err => {
                    b"zzz" => Error::UnexpectedText,
                    b"IE" => Error::UnexpectedText,
                    b"IS_" => Error::UnexpectedEnd,
                    b"I" => Error::UnexpectedEnd,
                    b"" => Error::UnexpectedEnd,
                }
            }
        });
    }

    #[test]
    fn parse_template_arg() {
        assert_parse!(TemplateArg {
            with subs [
                Substitutable::Type(Type::Decltype(Decltype::Expression(Expression::Rethrow)))
            ] => {
                Ok => {
                    b"S_..." => {
                        TemplateArg::Type(TypeHandle::BackReference(0)),
                        b"...",
                        []
                    }
                    b"XtrE..." => {
                        TemplateArg::Expression(Expression::Rethrow),
                        b"...",
                        []
                    }
                    b"XsrS_1QE..." => {
                        TemplateArg::Expression(
                            Expression::UnresolvedName(
                                UnresolvedName::Nested1(
                                    UnresolvedTypeHandle::BackReference(0),
                                    vec![],
                                    BaseUnresolvedName::Name(
                                        SimpleId(
                                            SourceName(Identifier {
                                                start: 6,
                                                end: 7
                                            }),
                                            None
                                        )
                                    )
                                )
                            )
                        ),
                        b"...",
                        []
                    }
                    b"XsrS_1QIlEE..." => {
                        TemplateArg::Expression(
                            Expression::UnresolvedName(
                                UnresolvedName::Nested1(
                                    UnresolvedTypeHandle::BackReference(0),
                                    vec![],
                                    BaseUnresolvedName::Name(
                                        SimpleId(
                                            SourceName(Identifier {
                                                start: 6,
                                                end: 7
                                            }),
                                            Some(
                                                TemplateArgs(
                                                    vec![
                                                        TemplateArg::Type(
                                                            TypeHandle::Builtin(
                                                                BuiltinType::Standard(
                                                                    StandardBuiltinType::Long
                                                                )
                                                            )
                                                        )
                                                    ]
                                                )
                                            )
                                        )
                                    )
                                )
                            )
                        ),
                        b"...",
                        []
                    }
                    b"LS_E..." => {
                        TemplateArg::SimpleExpression(
                            ExprPrimary::Literal(TypeHandle::BackReference(0), 3, 3)),
                        b"...",
                        []
                    }
                    b"JE..." => {
                        TemplateArg::ArgPack(vec![]),
                        b"...",
                        []
                    }
                    b"JS_XtrELS_EJEE..." => {
                        TemplateArg::ArgPack(vec![
                            TemplateArg::Type(TypeHandle::BackReference(0)),
                            TemplateArg::Expression(Expression::Rethrow),
                            TemplateArg::SimpleExpression(
                                ExprPrimary::Literal(TypeHandle::BackReference(0), 10, 10)),
                            TemplateArg::ArgPack(vec![]),
                        ]),
                        b"...",
                        []
                    }
                }
                Err => {
                    b"..." => Error::UnexpectedText,
                    b"X..." => Error::UnexpectedText,
                    b"J..." => Error::UnexpectedText,
                    b"JS_..." => Error::UnexpectedText,
                    b"JS_" => Error::UnexpectedEnd,
                    b"X" => Error::UnexpectedEnd,
                    b"J" => Error::UnexpectedEnd,
                    b"" => Error::UnexpectedEnd,
                }
            }
        });
    }

    #[test]
    fn parse_expression() {
        assert_parse!(Expression {
            with subs [
                Substitutable::Type(
                    Type::PointerTo(TypeHandle::Builtin(
                        BuiltinType::Standard(StandardBuiltinType::Int)))),
            ] => {
                Ok => {
                    b"psLS_1E..." => {
                        Expression::Unary(OperatorName::Simple(SimpleOperatorName::UnaryPlus),
                                          Box::new(Expression::Primary(
                                              ExprPrimary::Literal(
                                                  TypeHandle::BackReference(0),
                                                  5,
                                                  6)))),
                        b"...",
                        []
                    }
                    b"rsLS_1ELS_1E..." => {
                        Expression::Binary(OperatorName::Simple(SimpleOperatorName::Shr),
                                           Box::new(Expression::Primary(
                                               ExprPrimary::Literal(
                                                   TypeHandle::BackReference(0),
                                                   5,
                                                   6))),
                                           Box::new(Expression::Primary(
                                               ExprPrimary::Literal(
                                                   TypeHandle::BackReference(0),
                                                   10,
                                                   11)))),
                        b"...",
                        []
                    }
                    b"quLS_1ELS_2ELS_3E..." => {
                        Expression::Ternary(OperatorName::Simple(SimpleOperatorName::Question),
                                            Box::new(Expression::Primary(
                                                ExprPrimary::Literal(
                                                    TypeHandle::BackReference(0),
                                                    5,
                                                    6))),
                                            Box::new(Expression::Primary(
                                                ExprPrimary::Literal(
                                                    TypeHandle::BackReference(0),
                                                    10,
                                                    11))),
                                            Box::new(Expression::Primary(
                                                ExprPrimary::Literal(
                                                    TypeHandle::BackReference(0),
                                                    15,
                                                    16)))),
                        b"...",
                        []
                    }
                    b"pp_LS_1E..." => {
                        Expression::PrefixInc(
                            Box::new(Expression::Primary(
                                ExprPrimary::Literal(
                                    TypeHandle::BackReference(0),
                                    6,
                                    7)))),
                        b"...",
                        []
                    }
                    b"mm_LS_1E..." => {
                        Expression::PrefixDec(
                            Box::new(Expression::Primary(
                                ExprPrimary::Literal(
                                    TypeHandle::BackReference(0),
                                    6,
                                    7)))),
                        b"...",
                        []
                    }
                    b"clLS_1EE..." => {
                        Expression::Call(
                            Box::new(Expression::Primary(
                                ExprPrimary::Literal(
                                    TypeHandle::BackReference(0),
                                    5,
                                    6))),
                            vec![]),
                        b"...",
                        []
                    }
                    //               ::= cv <type> <expression>                       # type (expression), conversion with one argument
                    b"cvS_LS_1E..." => {
                        Expression::ConversionOne(
                            TypeHandle::BackReference(0),
                            Box::new(Expression::Primary(
                                ExprPrimary::Literal(
                                    TypeHandle::BackReference(0),
                                    7,
                                    8)))),
                        b"...",
                        []
                    }
                    b"cvS__LS_1ELS_1EE..." => {
                        Expression::ConversionMany(
                            TypeHandle::BackReference(0),
                            vec![
                                Expression::Primary(
                                    ExprPrimary::Literal(
                                        TypeHandle::BackReference(0),
                                        8,
                                        9)),
                                Expression::Primary(
                                    ExprPrimary::Literal(
                                        TypeHandle::BackReference(0),
                                        13,
                                        14)),
                            ]),
                        b"...",
                        []
                    }
                    b"tlS_LS_1ELS_1EE..." => {
                        Expression::ConversionBraced(
                            TypeHandle::BackReference(0),
                            vec![
                                Expression::Primary(
                                    ExprPrimary::Literal(
                                        TypeHandle::BackReference(0),
                                        7,
                                        8)),
                                Expression::Primary(
                                    ExprPrimary::Literal(
                                        TypeHandle::BackReference(0),
                                        12,
                                        13)),
                            ]),
                        b"...",
                        []
                    }
                    b"ilLS_1EE..." => {
                        Expression::BracedInitList(
                            Box::new(Expression::Primary(
                                ExprPrimary::Literal(
                                    TypeHandle::BackReference(0),
                                    5,
                                    6)))),
                        b"...",
                        []
                    }
                    b"gsnwLS_1E_S_E..." => {
                        Expression::GlobalNew(
                            vec![
                                Expression::Primary(
                                    ExprPrimary::Literal(
                                        TypeHandle::BackReference(0),
                                        7,
                                        8))
                            ],
                            TypeHandle::BackReference(0),
                            None),
                        b"...",
                        []
                    }
                    b"nwLS_1E_S_E..." => {
                        Expression::New(
                            vec![
                                Expression::Primary(
                                    ExprPrimary::Literal(
                                        TypeHandle::BackReference(0),
                                        5,
                                        6))
                            ],
                            TypeHandle::BackReference(0),
                            None),
                        b"...",
                        []
                    }
                    b"gsnwLS_1E_S_piE..." => {
                        Expression::GlobalNew(
                            vec![
                                Expression::Primary(
                                    ExprPrimary::Literal(
                                        TypeHandle::BackReference(0),
                                        7,
                                        8))
                            ],
                            TypeHandle::BackReference(0),
                            Some(Initializer(vec![]))),
                        b"...",
                        []
                    }
                    b"nwLS_1E_S_piE..." => {
                        Expression::New(
                            vec![
                                Expression::Primary(
                                    ExprPrimary::Literal(
                                        TypeHandle::BackReference(0),
                                        5,
                                        6))
                            ],
                            TypeHandle::BackReference(0),
                            Some(Initializer(vec![]))),
                        b"...",
                        []
                    }
                    b"gsnaLS_1E_S_E..." => {
                        Expression::GlobalNewArray(
                            vec![
                                Expression::Primary(
                                    ExprPrimary::Literal(
                                        TypeHandle::BackReference(0),
                                        7,
                                        8))
                            ],
                            TypeHandle::BackReference(0),
                            None),
                        b"...",
                        []
                    }
                    b"naLS_1E_S_E..." => {
                        Expression::NewArray(
                            vec![
                                Expression::Primary(
                                    ExprPrimary::Literal(
                                        TypeHandle::BackReference(0),
                                        5,
                                        6))
                            ],
                            TypeHandle::BackReference(0),
                            None),
                        b"...",
                        []
                    }
                    b"gsnaLS_1E_S_piE..." => {
                        Expression::GlobalNewArray(
                            vec![
                                Expression::Primary(
                                    ExprPrimary::Literal(
                                        TypeHandle::BackReference(0),
                                        7,
                                        8))
                            ],
                            TypeHandle::BackReference(0),
                            Some(Initializer(vec![]))),
                        b"...",
                        []
                    }
                    b"naLS_1E_S_piE..." => {
                        Expression::NewArray(
                            vec![
                                Expression::Primary(
                                    ExprPrimary::Literal(
                                        TypeHandle::BackReference(0),
                                        5,
                                        6))
                            ],
                            TypeHandle::BackReference(0),
                            Some(Initializer(vec![]))),
                        b"...",
                        []
                    }
                    b"gsdlLS_1E..." => {
                        Expression::GlobalDelete(
                            Box::new(Expression::Primary(
                                ExprPrimary::Literal(
                                    TypeHandle::BackReference(0),
                                    7,
                                    8)))),
                        b"...",
                        []
                    }
                    b"dlLS_1E..." => {
                        Expression::Delete(
                            Box::new(Expression::Primary(
                                ExprPrimary::Literal(
                                    TypeHandle::BackReference(0),
                                    5,
                                    6)))),
                        b"...",
                        []
                    }
                    //               ::= [gs] da <expression>                         # delete[] expression
                    b"gsdaLS_1E..." => {
                        Expression::GlobalDeleteArray(
                            Box::new(Expression::Primary(
                                ExprPrimary::Literal(
                                    TypeHandle::BackReference(0),
                                    7,
                                    8)))),
                        b"...",
                        []
                    }
                    b"daLS_1E..." => {
                        Expression::DeleteArray(
                            Box::new(Expression::Primary(
                                ExprPrimary::Literal(
                                    TypeHandle::BackReference(0),
                                    5,
                                    6)))),
                        b"...",
                        []
                    }
                    b"dcS_LS_1E..." => {
                        Expression::DynamicCast(
                            TypeHandle::BackReference(0),
                            Box::new(Expression::Primary(
                                ExprPrimary::Literal(
                                    TypeHandle::BackReference(0),
                                    7,
                                    8)))),
                        b"...",
                        []
                    }
                    b"scS_LS_1E..." => {
                        Expression::StaticCast(
                            TypeHandle::BackReference(0),
                            Box::new(Expression::Primary(
                                ExprPrimary::Literal(
                                    TypeHandle::BackReference(0),
                                    7,
                                    8)))),
                        b"...",
                        []
                    }
                    b"ccS_LS_1E..." => {
                        Expression::ConstCast(
                            TypeHandle::BackReference(0),
                            Box::new(Expression::Primary(
                                ExprPrimary::Literal(
                                    TypeHandle::BackReference(0),
                                    7,
                                    8)))),
                        b"...",
                        []
                    }
                    b"rcS_LS_1E..." => {
                        Expression::ReinterpretCast(
                            TypeHandle::BackReference(0),
                            Box::new(Expression::Primary(
                                ExprPrimary::Literal(
                                    TypeHandle::BackReference(0),
                                    7,
                                    8)))),
                        b"...",
                        []
                    }
                    b"tiS_..." => {
                        Expression::TypeidType(TypeHandle::BackReference(0)),
                        b"...",
                        []
                    }
                    b"teLS_1E..." => {
                        Expression::TypeidExpr(
                            Box::new(Expression::Primary(
                                ExprPrimary::Literal(
                                    TypeHandle::BackReference(0),
                                    5,
                                    6)))),
                        b"...",
                        []
                    }
                    b"stS_..." => {
                        Expression::SizeofType(TypeHandle::BackReference(0)),
                        b"...",
                        []
                    }
                    b"szLS_1E..." => {
                        Expression::SizeofExpr(
                            Box::new(Expression::Primary(
                                ExprPrimary::Literal(
                                    TypeHandle::BackReference(0),
                                    5,
                                    6)))),
                        b"...",
                        []
                    }
                    b"atS_..." => {
                        Expression::AlignofType(TypeHandle::BackReference(0)),
                        b"...",
                        []
                    }
                    b"azLS_1E..." => {
                        Expression::AlignofExpr(
                            Box::new(Expression::Primary(
                                ExprPrimary::Literal(
                                    TypeHandle::BackReference(0),
                                    5,
                                    6)))),
                        b"...",
                        []
                    }
                    b"nxLS_1E..." => {
                        Expression::Noexcept(
                            Box::new(Expression::Primary(
                                ExprPrimary::Literal(
                                    TypeHandle::BackReference(0),
                                    5,
                                    6)))),
                        b"...",
                        []
                    }
                    b"T_..." => {
                        Expression::TemplateParam(TemplateParam(0)),
                        b"...",
                        []
                    }
                    b"fp_..." => {
                        Expression::FunctionParam(FunctionParam(0, CvQualifiers::default(), Some(0))),
                        b"...",
                        []
                    }
                    b"dtT_3abc..." => {
                        Expression::Member(
                            Box::new(Expression::TemplateParam(TemplateParam(0))),
                            MemberName(
                                Name::Unscoped(
                                    UnscopedName::Unqualified(
                                        UnqualifiedName::Source(
                                            SourceName(
                                                Identifier {
                                                    start: 5,
                                                    end: 8,
                                                }),
                                            AbiTags::default()))))),
                        b"...",
                        []
                    }
                    b"ptT_3abc..." => {
                        Expression::DerefMember(
                            Box::new(Expression::TemplateParam(TemplateParam(0))),
                            MemberName(
                                Name::Unscoped(
                                    UnscopedName::Unqualified(
                                        UnqualifiedName::Source(
                                            SourceName(
                                                Identifier {
                                                    start: 5,
                                                    end: 8,
                                                }),
                                            AbiTags::default()))))),
                        b"...",
                        []
                    }
                    b"dtfp_clI3abcE..." => {
                        Expression::Member(
                            Box::new(Expression::FunctionParam(FunctionParam(0, CvQualifiers::default(), Some(0)))),
                            MemberName(
                                Name::UnscopedTemplate(
                                    UnscopedTemplateNameHandle::NonSubstitution(NonSubstitution(0)),
                                    TemplateArgs(vec![
                                        TemplateArg::Type(
                                            TypeHandle::BackReference(1))])))),
                        b"...",
                        [
                            Substitutable::Type(
                                Type::ClassEnum(
                                    ClassEnumType::Named(
                                        Name::Unscoped(
                                            UnscopedName::Unqualified(
                                                UnqualifiedName::Source(
                                                    SourceName(
                                                        Identifier {
                                                            start: 9,
                                                            end: 12
                                                        }),
                                                    AbiTags::default()))))))
                        ]
                    }
                    //               ::= ds <expression> <expression>                 # expr.*expr
                    b"dsT_T_..." => {
                        Expression::PointerToMember(
                            Box::new(Expression::TemplateParam(TemplateParam(0))),
                            Box::new(Expression::TemplateParam(TemplateParam(0)))),
                        b"...",
                        []
                    }
                    b"sZT_..." => {
                        Expression::SizeofTemplatePack(TemplateParam(0)),
                        b"...",
                        []
                    }
                    b"sZfp_..." => {
                        Expression::SizeofFunctionPack(
                            FunctionParam(0, CvQualifiers::default(), Some(0))),
                        b"...",
                        []
                    }
                    b"sPE..." => {
                        Expression::SizeofCapturedTemplatePack(vec![]),
                        b"...",
                        []
                    }
                    b"spT_..." => {
                        Expression::PackExpansion(
                            Box::new(Expression::TemplateParam(TemplateParam(0)))),
                        b"...",
                        []
                    }
                    b"twT_..." => {
                        Expression::Throw(Box::new(Expression::TemplateParam(TemplateParam(0)))),
                        b"...",
                        []
                    }
                    b"tr..." => {
                        Expression::Rethrow,
                        b"...",
                        []
                    }
                    b"3abc..." => {
                        Expression::UnresolvedName(
                            UnresolvedName::Name(
                                BaseUnresolvedName::Name(
                                    SimpleId(
                                        SourceName(Identifier {
                                            start: 1,
                                            end: 4,
                                        }),
                                        None)))),
                        b"...",
                        []
                    }
                    b"L_Z3abcE..." => {
                        Expression::Primary(
                            ExprPrimary::External(
                                MangledName::Encoding(
                                    Encoding::Data(
                                        Name::Unscoped(
                                            UnscopedName::Unqualified(
                                                UnqualifiedName::Source(
                                                    SourceName(Identifier {
                                                        start: 4,
                                                        end: 7,
                                                    }),
                                                AbiTags::default())))), vec![]))),
                        b"...",
                        []
                    }
                    // An expression where arity matters
                    b"cldtdefpT4TypeadsrT_5EnterE..." => {
                        Expression::Call(
                            Box::new(Expression::Member(
                                Box::new(Expression::Unary(OperatorName::Simple(SimpleOperatorName::Deref),
                                                           Box::new(Expression::FunctionParam(
                                                               FunctionParam(0,
                                                                             CvQualifiers::default(),
                                                                             None)
                                                           ))
                                )),
                                MemberName(
                                    Name::Unscoped(
                                        UnscopedName::Unqualified(
                                            UnqualifiedName::Source(
                                                SourceName(Identifier {
                                                    start: 10,
                                                    end: 14,
                                                }),
                                                AbiTags::default()))
                                     )
                                )
                            )),
                            vec![Expression::Unary(OperatorName::Simple(SimpleOperatorName::AddressOf),
                                                   Box::new(Expression::UnresolvedName(
                                                       UnresolvedName::Nested1(
                                                           UnresolvedTypeHandle::BackReference(1),
                                                           vec![],
                                                           BaseUnresolvedName::Name(
                                                               SimpleId(
                                                                   SourceName(Identifier {
                                                                           start: 21,
                                                                           end: 26
                                                                   }
                                                                   ),
                                                                   None
                                                               )
                                                           )
                                                       ))))]
                        ),
                        b"...",
                        [
                            Substitutable::UnresolvedType(UnresolvedType::Template(TemplateParam(0), None))
                        ]
                    }
                }
                Err => {
                    b"dtStfp_clI3abcE..." => Error::UnexpectedText,
                }
            }
        });
    }

    #[test]
    fn parse_unresolved_name() {
        assert_parse!(UnresolvedName {
            with subs [
                Substitutable::UnresolvedType(
                    UnresolvedType::Decltype(Decltype::Expression(Expression::Rethrow))),
            ] => {
                Ok => {
                    b"gs3abc..." => {
                        UnresolvedName::Global(BaseUnresolvedName::Name(SimpleId(SourceName(Identifier {
                            start: 3,
                            end: 6,
                        }), None))),
                        b"...",
                        []
                    }
                    b"3abc..." => {
                        UnresolvedName::Name(BaseUnresolvedName::Name(SimpleId(SourceName(Identifier {
                            start: 1,
                            end: 4,
                        }), None))),
                        b"...",
                        []
                    }
                    b"srS_3abc..." => {
                        UnresolvedName::Nested1(UnresolvedTypeHandle::BackReference(0),
                                                vec![],
                                                BaseUnresolvedName::Name(SimpleId(SourceName(Identifier {
                                                    start: 5,
                                                    end: 8,
                                                }), None))),
                        b"...",
                        []
                    }
                    b"srNS_3abc3abcE3abc..." => {
                        UnresolvedName::Nested1(
                            UnresolvedTypeHandle::BackReference(0),
                            vec![
                                UnresolvedQualifierLevel(SimpleId(SourceName(Identifier {
                                    start: 6,
                                    end: 9,
                                }), None)),
                                UnresolvedQualifierLevel(SimpleId(SourceName(Identifier {
                                    start: 10,
                                    end: 13,
                                }), None)),
                            ],
                            BaseUnresolvedName::Name(SimpleId(SourceName(Identifier {
                                start: 15,
                                end: 18,
                            }), None))),
                        b"...",
                        []
                    }
                    b"gssr3abcE3abc..." => {
                        UnresolvedName::GlobalNested2(
                            vec![
                                UnresolvedQualifierLevel(SimpleId(SourceName(Identifier {
                                    start: 5,
                                    end: 8,
                                }), None)),
                            ],
                            BaseUnresolvedName::Name(SimpleId(SourceName(Identifier {
                                start: 10,
                                end: 13,
                            }), None))),
                        b"...",
                        []
                    }
                    b"sr3abcE3abc..." => {
                        UnresolvedName::Nested2(
                            vec![
                                UnresolvedQualifierLevel(SimpleId(SourceName(Identifier {
                                    start: 3,
                                    end: 6,
                                }), None)),
                            ],
                            BaseUnresolvedName::Name(SimpleId(SourceName(Identifier {
                                start: 8,
                                end: 11,
                            }), None))),
                        b"...",
                        []
                    }
                }
                Err => {
                    b"zzzzzz" => Error::UnexpectedText,
                    b"gszzz" => Error::UnexpectedText,
                    b"gssrzzz" => Error::UnexpectedText,
                    b"srNzzz" => Error::UnexpectedText,
                    b"srzzz" => Error::UnexpectedText,
                    b"srN3abczzzz" => Error::UnexpectedText,
                    b"srN3abcE" => Error::UnexpectedText,
                    b"srN3abc" => Error::UnexpectedText,
                    b"srN" => Error::UnexpectedEnd,
                    b"sr" => Error::UnexpectedEnd,
                    b"gssr" => Error::UnexpectedEnd,
                    b"gs" => Error::UnexpectedEnd,
                    b"" => Error::UnexpectedEnd,
                }
            }
        });
    }

    #[test]
    fn parse_unresolved_type_handle() {
        assert_parse!(UnresolvedTypeHandle {
            with subs [
                Substitutable::UnresolvedType(
                    UnresolvedType::Decltype(Decltype::Expression(Expression::Rethrow))),
            ] => {
                Ok => {
                    b"S_..." => {
                        UnresolvedTypeHandle::BackReference(0),
                        b"...",
                        []
                    }
                    b"T_..." => {
                        UnresolvedTypeHandle::BackReference(1),
                        b"...",
                        [
                            Substitutable::UnresolvedType(
                                UnresolvedType::Template(TemplateParam(0), None)),
                        ]
                    }
                    b"T_IS_E..." => {
                        UnresolvedTypeHandle::BackReference(1),
                        b"...",
                        [
                            Substitutable::UnresolvedType(
                                UnresolvedType::Template(TemplateParam(0), Some(TemplateArgs(vec![
                                    TemplateArg::Type(TypeHandle::BackReference(0))
                                ])))),
                        ]
                    }
                    b"DTtrE..." => {
                        UnresolvedTypeHandle::BackReference(1),
                        b"...",
                        [
                            Substitutable::UnresolvedType(
                                UnresolvedType::Decltype(Decltype::Expression(Expression::Rethrow)))
                        ]

                    }
                }
                Err => {
                    b"zzzzzzz" => Error::UnexpectedText,
                    b"" => Error::UnexpectedEnd,
                }
            }
        });
    }

    #[test]
    fn parse_unresolved_qualifier_level() {
        assert_parse!(UnresolvedQualifierLevel {
            with subs [
                Substitutable::Type(Type::Decltype(Decltype::Expression(Expression::Rethrow)))
            ] => {
                Ok => {
                    b"3abc..." => {
                        UnresolvedQualifierLevel(SimpleId(SourceName(Identifier {
                            start: 1,
                            end: 4,
                        }), None)),
                        b"...",
                        []
                    }
                    b"3abcIS_E..." => {
                        UnresolvedQualifierLevel(SimpleId(SourceName(Identifier {
                            start: 1,
                            end: 4,
                        }), Some(TemplateArgs(vec![
                            TemplateArg::Type(TypeHandle::BackReference(0))
                        ])))),
                        b"...",
                        []
                    }
                }
                Err => {
                    b"zzz" => Error::UnexpectedText,
                    b"" => Error::UnexpectedEnd,
                }
            }
        });
    }

    #[test]
    fn parse_simple_id() {
        assert_parse!(SimpleId {
            with subs [
                Substitutable::Type(Type::Decltype(Decltype::Expression(Expression::Rethrow)))
            ] => {
                Ok => {
                    b"3abc..." => {
                        SimpleId(SourceName(Identifier {
                            start: 1,
                            end: 4,
                        }), None),
                        b"...",
                        []
                    }
                    b"3abcIS_E..." => {
                        SimpleId(SourceName(Identifier {
                            start: 1,
                            end: 4,
                        }), Some(TemplateArgs(vec![
                            TemplateArg::Type(TypeHandle::BackReference(0))
                        ]))),
                        b"...",
                        []
                    }
                }
                Err => {
                    b"zzz" => Error::UnexpectedText,
                    b"" => Error::UnexpectedEnd,
                }
            }
        });
    }

    #[test]
    fn parse_base_unresolved_name() {
        assert_parse!(BaseUnresolvedName {
            with subs [
                Substitutable::Type(Type::Decltype(Decltype::Expression(Expression::Rethrow)))
            ] => {
                Ok => {
                    b"3abc..." => {
                        BaseUnresolvedName::Name(SimpleId(SourceName(Identifier {
                            start: 1,
                            end: 4,
                        }), None)),
                        b"...",
                        []
                    }
                    b"onnw..." => {
                        BaseUnresolvedName::Operator(OperatorName::Simple(SimpleOperatorName::New), None),
                        b"...",
                        []
                    }
                    b"onnwIS_E..." => {
                        BaseUnresolvedName::Operator(OperatorName::Simple(SimpleOperatorName::New),
                                                     Some(TemplateArgs(vec![
                                                         TemplateArg::Type(TypeHandle::BackReference(0))
                                                     ]))),
                        b"...",
                        []
                    }
                    b"dn3abc..." => {
                        BaseUnresolvedName::Destructor(DestructorName::Name(SimpleId(SourceName(Identifier {
                            start: 3,
                            end: 6,
                        }), None))),
                        b"...",
                        []
                    }
                }
                Err => {
                    b"ozzz" => Error::UnexpectedText,
                    b"dzzz" => Error::UnexpectedText,
                    b"dn" => Error::UnexpectedEnd,
                    b"on" => Error::UnexpectedEnd,
                    b"" => Error::UnexpectedEnd,
                }
            }
        });
    }

    #[test]
    fn parse_destructor_name() {
        assert_parse!(DestructorName {
            with subs [
                Substitutable::UnresolvedType(
                    UnresolvedType::Decltype(Decltype::Expression(Expression::Rethrow))),
            ] => {
                Ok => {
                    b"S_..." => {
                        DestructorName::Unresolved(UnresolvedTypeHandle::BackReference(0)),
                        b"...",
                        []
                    }
                    b"3abc..." => {
                        DestructorName::Name(SimpleId(SourceName(Identifier {
                            start: 1,
                            end: 4,
                        }), None)),
                        b"...",
                        []
                    }
                }
                Err => {
                    b"zzz" => Error::UnexpectedText,
                    b"" => Error::UnexpectedEnd,
                }
            }
        });
    }

    #[test]
    fn parse_expr_primary() {
        assert_parse!(ExprPrimary {
            with subs [
                Substitutable::Type(Type::Decltype(Decltype::Expression(Expression::Rethrow)))
            ] => {
                Ok => {
                    b"LS_12345E..." => {
                        ExprPrimary::Literal(TypeHandle::BackReference(0), 3, 8),
                        b"...",
                        []
                    }
                    b"LS_E..." => {
                        ExprPrimary::Literal(TypeHandle::BackReference(0), 3, 3),
                        b"...",
                        []
                    }
                    b"L_Z3abcE..." => {
                        ExprPrimary::External(
                            MangledName::Encoding(
                                Encoding::Data(
                                    Name::Unscoped(
                                        UnscopedName::Unqualified(
                                            UnqualifiedName::Source(
                                                SourceName(Identifier {
                                                    start: 4,
                                                    end: 7,
                                                }),
                                                AbiTags::default())))), vec![])),
                        b"...",
                        []
                    }
                }
                Err => {
                    b"zzz" => Error::UnexpectedText,
                    b"LS_zzz" => Error::UnexpectedEnd,
                    b"LS_12345" => Error::UnexpectedEnd,
                    b"LS_" => Error::UnexpectedEnd,
                    b"L" => Error::UnexpectedEnd,
                    b"" => Error::UnexpectedEnd,
                }
            }
        });
    }

    #[test]
    fn parse_initializer() {
        assert_parse!(Initializer {
            Ok => {
                b"piE..." => {
                    Initializer(vec![]),
                    b"..."
                }
                b"pitrtrtrE..." => {
                    Initializer(vec![
                        Expression::Rethrow,
                        Expression::Rethrow,
                        Expression::Rethrow,
                    ]),
                    b"..."
                }
            }
            Err => {
                b"pirtrtrt..." => Error::UnexpectedText,
                b"pi..." => Error::UnexpectedText,
                b"..." => Error::UnexpectedText,
                b"pirt" => Error::UnexpectedText,
                b"pi" => Error::UnexpectedEnd,
                b"p" => Error::UnexpectedEnd,
                b"" => Error::UnexpectedEnd,
            }
        });
    }

    #[test]
    fn parse_local_name() {
        assert_parse!(LocalName {
            Ok => {
                b"Z3abcE3def_0..." => {
                    LocalName::Relative(
                        Box::new(Encoding::Data(
                            Name::Unscoped(
                                UnscopedName::Unqualified(
                                    UnqualifiedName::Source(
                                        SourceName(Identifier {
                                            start: 2,
                                            end: 5,
                                        }),
                                        AbiTags::default()))))),
                        Some(Box::new(Name::Unscoped(
                            UnscopedName::Unqualified(
                                UnqualifiedName::Source(
                                    SourceName(Identifier {
                                        start: 7,
                                        end: 10,
                                    }),
                                    AbiTags::default()))))),
                        Some(Discriminator(0))),
                    b"..."
                }
                b"Z3abcE3def..." => {
                    LocalName::Relative(
                        Box::new(Encoding::Data(
                            Name::Unscoped(
                                UnscopedName::Unqualified(
                                    UnqualifiedName::Source(
                                        SourceName(Identifier {
                                            start: 2,
                                            end: 5,
                                        }),
                                        AbiTags::default()))))),
                        Some(Box::new(Name::Unscoped(
                            UnscopedName::Unqualified(
                                UnqualifiedName::Source(
                                    SourceName(Identifier {
                                        start: 7,
                                        end: 10,
                                    }),
                                    AbiTags::default()))))),
                        None),
                    b"..."
                }
                b"Z3abcEs_0..." => {
                    LocalName::Relative(
                        Box::new(Encoding::Data(
                            Name::Unscoped(
                                UnscopedName::Unqualified(
                                    UnqualifiedName::Source(
                                        SourceName(Identifier {
                                            start: 2,
                                            end: 5,
                                        }),
                                    AbiTags::default()))))),
                        None,
                        Some(Discriminator(0))),
                    b"..."
                }
                b"Z3abcEs..." => {
                    LocalName::Relative(
                        Box::new(Encoding::Data(
                            Name::Unscoped(
                                UnscopedName::Unqualified(
                                    UnqualifiedName::Source(
                                        SourceName(Identifier {
                                            start: 2,
                                            end: 5,
                                        }),
                                    AbiTags::default()))))),
                        None,
                        None),
                    b"..."
                }
                b"Z3abcEd1_3abc..." => {
                    LocalName::Default(
                        Box::new(Encoding::Data(
                            Name::Unscoped(
                                UnscopedName::Unqualified(
                                    UnqualifiedName::Source(
                                        SourceName(Identifier {
                                            start: 2,
                                            end: 5,
                                        }),
                                    AbiTags::default()))))),
                        Some(1),
                        Box::new(Name::Unscoped(
                            UnscopedName::Unqualified(
                                UnqualifiedName::Source(
                                    SourceName(Identifier {
                                        start: 10,
                                        end: 13,
                                    }),
                                    AbiTags::default()))))),
                    b"..."
                }
                b"Z3abcEd_3abc..." => {
                    LocalName::Default(
                        Box::new(Encoding::Data(
                            Name::Unscoped(
                                UnscopedName::Unqualified(
                                    UnqualifiedName::Source(
                                        SourceName(Identifier {
                                            start: 2,
                                            end: 5,
                                        }),
                                        AbiTags::default()))))),
                        None,
                        Box::new(Name::Unscoped(
                            UnscopedName::Unqualified(
                                UnqualifiedName::Source(
                                    SourceName(Identifier {
                                        start: 9,
                                        end: 12,
                                    }), AbiTags::default()))))),
                    b"..."
                }
            }
            Err => {
                b"A" => Error::UnexpectedText,
                b"Z1a" => Error::UnexpectedEnd,
                b"Z1aE" => Error::UnexpectedEnd,
                b"Z" => Error::UnexpectedEnd,
                b"" => Error::UnexpectedEnd,
            }
        });
    }

    #[test]
    fn parse_closure_type_name() {
        assert_parse!(ClosureTypeName {
            Ok => {
                b"UlvE_..." => {
                    ClosureTypeName(LambdaSig(vec![]), None),
                    b"..."
                }
                b"UlvE36_..." => {
                    ClosureTypeName(LambdaSig(vec![]), Some(36)),
                    b"..."
                }
            }
            Err => {
                b"UlvE36zzz" => Error::UnexpectedText,
                b"UlvEzzz" => Error::UnexpectedText,
                b"Ulvzzz" => Error::UnexpectedText,
                b"zzz" => Error::UnexpectedText,
                b"UlvE10" => Error::UnexpectedEnd,
                b"UlvE" => Error::UnexpectedEnd,
                b"Ulv" => Error::UnexpectedEnd,
                b"Ul" => Error::UnexpectedEnd,
                b"U" => Error::UnexpectedEnd,
                b"" => Error::UnexpectedEnd,
            }
        });
    }

    #[test]
    fn parse_lambda_sig() {
        assert_parse!(LambdaSig {
            with subs [
                Substitutable::Type(
                    Type::PointerTo(
                        TypeHandle::Builtin(BuiltinType::Standard(StandardBuiltinType::Bool))))
            ] => {
                Ok => {
                    b"v..." => {
                        LambdaSig(vec![]),
                        b"...",
                        []
                    }
                    b"S_S_S_..." => {
                        LambdaSig(vec![
                            TypeHandle::BackReference(0),
                            TypeHandle::BackReference(0),
                            TypeHandle::BackReference(0),
                        ]),
                        b"...",
                        []
                    }
                }
                Err => {
                    b"..." => Error::UnexpectedText,
                    b"S" => Error::UnexpectedEnd,
                    b"" => Error::UnexpectedEnd,
                }
            }
        });
    }

    #[test]
    fn parse_substitution() {
        assert_parse!(Substitution {
            with subs [
                Substitutable::Type(Type::Decltype(Decltype::Expression(Expression::Rethrow))),
                Substitutable::Type(Type::Decltype(Decltype::Expression(Expression::Rethrow))),
                Substitutable::Type(Type::Decltype(Decltype::Expression(Expression::Rethrow)))
            ] => {
                Ok => {
                    b"S_..." => {
                        Substitution::BackReference(0),
                        b"...",
                        []
                    }
                    b"S1_..." => {
                        Substitution::BackReference(2),
                        b"...",
                        []
                    }
                    b"St..." => {
                        Substitution::WellKnown(WellKnownComponent::Std),
                        b"...",
                        []
                    }
                    b"Sa..." => {
                        Substitution::WellKnown(WellKnownComponent::StdAllocator),
                        b"...",
                        []
                    }
                    b"Sb..." => {
                        Substitution::WellKnown(WellKnownComponent::StdString1),
                        b"...",
                        []
                    }
                    b"Ss..." => {
                        Substitution::WellKnown(WellKnownComponent::StdString2),
                        b"...",
                        []
                    }
                    b"Si..." => {
                        Substitution::WellKnown(WellKnownComponent::StdIstream),
                        b"...",
                        []
                    }
                    b"So..." => {
                        Substitution::WellKnown(WellKnownComponent::StdOstream),
                        b"...",
                        []
                    }
                    b"Sd..." => {
                        Substitution::WellKnown(WellKnownComponent::StdIostream),
                        b"...",
                        []
                    }
                }
                Err => {
                    b"S999_" => Error::BadBackReference,
                    b"Sz" => Error::UnexpectedText,
                    b"zzz" => Error::UnexpectedText,
                    b"S1" => Error::UnexpectedEnd,
                    b"S" => Error::UnexpectedEnd,
                    b"" => Error::UnexpectedEnd,
                }
            }
        });
    }

    #[test]
    fn parse_special_name() {
        assert_parse!(SpecialName {
            Ok => {
                b"TVi..." => {
                    SpecialName::VirtualTable(TypeHandle::Builtin(BuiltinType::Standard(StandardBuiltinType::Int))),
                    b"..."
                }
                b"TTi..." => {
                    SpecialName::Vtt(TypeHandle::Builtin(BuiltinType::Standard(StandardBuiltinType::Int))),
                    b"..."
                }
                b"TIi..." => {
                    SpecialName::Typeinfo(TypeHandle::Builtin(BuiltinType::Standard(StandardBuiltinType::Int))),
                    b"..."
                }
                b"TSi..." => {
                    SpecialName::TypeinfoName(TypeHandle::Builtin(BuiltinType::Standard(StandardBuiltinType::Int))),
                    b"..."
                }
                b"Tv42_36_3abc..." => {
                    SpecialName::VirtualOverrideThunk(
                        CallOffset::Virtual(VOffset(42, 36)),
                        Box::new(Encoding::Data(
                            Name::Unscoped(
                                UnscopedName::Unqualified(
                                    UnqualifiedName::Source(
                                        SourceName(Identifier {
                                            start: 9,
                                            end: 12,
                                        }), AbiTags::default())))))),
                    b"..."
                }
                b"Tcv42_36_v42_36_3abc..." => {
                    SpecialName::VirtualOverrideThunkCovariant(
                        CallOffset::Virtual(VOffset(42, 36)),
                        CallOffset::Virtual(VOffset(42, 36)),
                        Box::new(Encoding::Data(
                            Name::Unscoped(
                                UnscopedName::Unqualified(
                                    UnqualifiedName::Source(
                                        SourceName(Identifier {
                                            start: 17,
                                            end: 20,
                                        }), AbiTags::default())))))),
                    b"..."
                }
                b"GV3abc..." => {
                    SpecialName::Guard(
                        Name::Unscoped(
                            UnscopedName::Unqualified(
                                UnqualifiedName::Source(
                                    SourceName(Identifier {
                                        start: 3,
                                        end: 6,
                                    }), AbiTags::default())))),
                    b"..."
                }
                b"GR3abc_..." => {
                    SpecialName::GuardTemporary(
                        Name::Unscoped(
                            UnscopedName::Unqualified(
                                UnqualifiedName::Source(
                                    SourceName(Identifier {
                                        start: 3,
                                        end: 6,
                                    }),
                                    AbiTags::default()))),
                        0),
                    b"..."
                }
                b"GR3abc0_..." => {
                    SpecialName::GuardTemporary(
                        Name::Unscoped(
                            UnscopedName::Unqualified(
                                UnqualifiedName::Source(
                                    SourceName(Identifier {
                                        start: 3,
                                        end: 6,
                                    }),
                                    AbiTags::default()))),
                        1),
                    b"..."
                }
                b"Gr4_abc..." => {
                    SpecialName::JavaResource(vec![ResourceName {
                        start: 4,
                        end: 7,
                    }]),
                    b"..."
                }
                b"TCc7_i..." => {
                    SpecialName::ConstructionVtable(
                        TypeHandle::Builtin(BuiltinType::Standard(StandardBuiltinType::Char)),
                        7,
                        TypeHandle::Builtin(BuiltinType::Standard(StandardBuiltinType::Int))
                    ),
                    b"..."
                }
                b"TFi..." => {
                    SpecialName::TypeinfoFunction(
                        TypeHandle::Builtin(BuiltinType::Standard(StandardBuiltinType::Int))),
                    b"..."
                }
                b"TH4name..." => {
                    SpecialName::TlsInit(
                        Name::Unscoped(
                            UnscopedName::Unqualified(
                                UnqualifiedName::Source(
                                    SourceName(Identifier { start: 3, end: 7 }),
                                AbiTags::default())))),
                    b"..."
                }
                b"TW4name..." => {
                    SpecialName::TlsWrapper(
                        Name::Unscoped(
                            UnscopedName::Unqualified(
                                UnqualifiedName::Source(
                                    SourceName(Identifier { start: 3, end: 7 }),
                                AbiTags::default())))),
                    b"..."
                }
            }
            Err => {
                b"TZ" => Error::UnexpectedText,
                b"GZ" => Error::UnexpectedText,
                b"GR3abcz" => Error::UnexpectedText,
                b"GR3abc0z" => Error::UnexpectedText,
                b"T" => Error::UnexpectedEnd,
                b"G" => Error::UnexpectedEnd,
                b"" => Error::UnexpectedEnd,
                b"GR3abc" => Error::UnexpectedEnd,
                b"GR3abc0" => Error::UnexpectedEnd,
                // This number is not allowed to be negative.
                b"TCcn7_i..." => Error::UnexpectedText,
                b"Gr3abc0" => Error::UnexpectedText,
            }
        });
    }

    #[test]
    fn parse_function_param() {
        assert_parse!(FunctionParam {
            Ok => {
                b"fpK_..." => {
                    FunctionParam(0,
                                  CvQualifiers {
                                      restrict: false,
                                      volatile: false,
                                      const_: true,
                                  },
                                  Some(0)),
                    b"..."
                }
                b"fL1pK_..." => {
                    FunctionParam(1,
                                  CvQualifiers {
                                      restrict: false,
                                      volatile: false,
                                      const_: true,
                                  },
                                  Some(0)),
                    b"..."
                }
                b"fpK3_..." => {
                    FunctionParam(0,
                                  CvQualifiers {
                                      restrict: false,
                                      volatile: false,
                                      const_: true,
                                  },
                                  Some(4)),
                    b"..."
                }
                b"fL1pK4_..." => {
                    FunctionParam(1,
                                  CvQualifiers {
                                      restrict: false,
                                      volatile: false,
                                      const_: true,
                                  },
                                  Some(5)),
                    b"..."
                }
            }
            Err => {
                b"fz" => Error::UnexpectedText,
                b"fLp_" => Error::UnexpectedText,
                b"fpL_" => Error::UnexpectedText,
                b"fL1pK4z" => Error::UnexpectedText,
                b"fL1pK4" => Error::UnexpectedEnd,
                b"fL1p" => Error::UnexpectedEnd,
                b"fL1" => Error::UnexpectedEnd,
                b"fL" => Error::UnexpectedEnd,
                b"f" => Error::UnexpectedEnd,
                b"" => Error::UnexpectedEnd,
            }
        });
    }

    #[test]
    fn parse_discriminator() {
        assert_parse!(Discriminator {
            Ok => {
                b"_0..." => {
                    Discriminator(0),
                    b"..."
                }
                b"_9..." => {
                    Discriminator(9),
                    b"..."
                }
                b"__99_..." => {
                    Discriminator(99),
                    b"..."
                }
            }
            Err => {
                b"_n1" => Error::UnexpectedText,
                b"__99..." => Error::UnexpectedText,
                b"__99" => Error::UnexpectedEnd,
                b"..." => Error::UnexpectedText,
            }
        });
    }

    #[test]
    fn parse_data_member_prefix() {
        assert_parse!(DataMemberPrefix {
            Ok => {
                b"3fooM..." => {
                    DataMemberPrefix(SourceName(Identifier {
                        start: 1,
                        end: 4,
                    })),
                    b"..."
                }
            }
            Err => {
                b"zzz" => Error::UnexpectedText,
                b"1" => Error::UnexpectedEnd,
                b"" => Error::UnexpectedEnd,
            }
        });
    }

    #[test]
    fn parse_ref_qualifier() {
        assert_parse!(RefQualifier {
            Ok => {
                b"R..." => {
                    RefQualifier::LValueRef,
                    b"..."
                }
                b"O..." => {
                    RefQualifier::RValueRef,
                    b"..."
                }
            }
            Err => {
                b"..." => Error::UnexpectedText,
                b"" => Error::UnexpectedEnd,
            }
        });
    }

    #[test]
    fn parse_cv_qualifiers() {
        assert_parse!(CvQualifiers {
            Ok => {
                b"" => {
                    CvQualifiers { restrict: false, volatile: false, const_: false },
                    b""
                }
                b"..." => {
                    CvQualifiers { restrict: false, volatile: false, const_: false },
                    b"..."
                }
                b"r..." => {
                    CvQualifiers { restrict: true, volatile: false, const_: false },
                    b"..."
                }
                b"rV..." => {
                    CvQualifiers { restrict: true, volatile: true, const_: false },
                    b"..."
                }
                b"rVK..." => {
                    CvQualifiers { restrict: true, volatile: true, const_: true },
                    b"..."
                }
                b"V" => {
                    CvQualifiers { restrict: false, volatile: true, const_: false },
                    b""
                }
                b"VK" => {
                    CvQualifiers { restrict: false, volatile: true, const_: true },
                    b""
                }
                b"K..." => {
                    CvQualifiers { restrict: false, volatile: false, const_: true },
                    b"..."
                }
            }
            Err => {
                // None.
            }
        });
    }

    #[test]
    fn parse_builtin_type() {
        assert_parse!(BuiltinType {
            Ok => {
                b"c..." => {
                    BuiltinType::Standard(StandardBuiltinType::Char),
                    b"..."
                }
                b"c" => {
                    BuiltinType::Standard(StandardBuiltinType::Char),
                    b""
                }
                b"u3abc..." => {
                    BuiltinType::Extension(SourceName(Identifier {
                        start: 2,
                        end: 5,
                    })),
                    b"..."
                }
                b"DF16b..." => {
                    BuiltinType::Standard(StandardBuiltinType::BFloat16),
                    b"..."
                }
            }
            Err => {
                b"." => Error::UnexpectedText,
                b"" => Error::UnexpectedEnd,
            }
        });
    }

    #[test]
    fn parse_parametric_builtin_type() {
        assert_parse!(BuiltinType {
            Ok => {
                b"DB8_..." => {
                    BuiltinType::Parametric(ParametricBuiltinType::SignedBitInt(8)),
                    b"..."
                }
                b"DUsZT_" => {
                    BuiltinType::Parametric(ParametricBuiltinType::UnsignedBitIntExpression(Box::new(Expression::SizeofTemplatePack(TemplateParam(0))))),
                    b""
                }
                b"DF128_..." => {
                    BuiltinType::Parametric(ParametricBuiltinType::FloatN(128)),
                    b"..."
                }
                b"DF256x..." => {
                    BuiltinType::Parametric(ParametricBuiltinType::FloatNx(256)),
                    b"..."
                }
            }
            Err => {
                b"DB100000000000000000000000_" => Error::Overflow,
                b"DFsZT_" => Error::UnexpectedText,
                b"DB" => Error::UnexpectedEnd,
                b"DB32" => Error::UnexpectedEnd,
            }
        });
    }

    #[test]
    fn parse_template_param() {
        assert_parse!(TemplateParam {
            Ok => {
                b"T_..." => {
                    TemplateParam(0),
                    b"..."
                }
                b"T3_..." => {
                    TemplateParam(4),
                    b"..."
                }
            }
            Err => {
                b"wtf" => Error::UnexpectedText,
                b"Twtf" => Error::UnexpectedText,
                b"T3wtf" => Error::UnexpectedText,
                b"T" => Error::UnexpectedEnd,
                b"T3" => Error::UnexpectedEnd,
                b"" => Error::UnexpectedEnd,
            }
        });
    }

    #[test]
    fn parse_unscoped_name() {
        assert_parse!(UnscopedName {
            Ok => {
                b"St5hello..." => {
                    UnscopedName::Std(UnqualifiedName::Source(SourceName(Identifier {
                        start: 3,
                        end: 8,
                    }),
                    AbiTags::default())),
                    b"..."
                }
                b"5hello..." => {
                    UnscopedName::Unqualified(UnqualifiedName::Source(SourceName(Identifier {
                        start: 1,
                        end: 6,
                    }), AbiTags::default())),
                    b"..."
                }
            }
            Err => {
                b"St..." => Error::UnexpectedText,
                b"..." => Error::UnexpectedText,
                b"" => Error::UnexpectedEnd,
            }
        });
    }

    #[test]
    fn parse_unqualified_name() {
        // <unqualified-name> ::= <operator-name> [<abi-tags>]
        //                    ::= <ctor-dtor-name> [<abi-tags>]
        //                    ::= <source-name> [<abi-tags>]
        //                    ::= <local-source-name> [<abi-tags>]
        //                    ::= <unnamed-type-name> [<abi-tags>]
        //                    ::= <closure-type-name> [<abi-tags>]
        assert_parse!(UnqualifiedName {
            Ok => {
                b"qu.." => {
                    UnqualifiedName::Operator(OperatorName::Simple(SimpleOperatorName::Question), AbiTags::default()),
                    b".."
                }
                b"C1.." => {
                    UnqualifiedName::CtorDtor(CtorDtorName::CompleteConstructor(None), AbiTags::default()),
                    b".."
                }
                b"10abcdefghij..." => {
                    UnqualifiedName::Source(SourceName(Identifier {
                        start: 2,
                        end: 12,
                    }), AbiTags::default()),
                    b"..."
                }
                b"UllE_..." => {
                    UnqualifiedName::ClosureType(
                        ClosureTypeName(
                            LambdaSig(vec![
                                TypeHandle::Builtin(
                                    BuiltinType::Standard(
                                        StandardBuiltinType::Long))
                            ]),
                            None),
                        AbiTags::default()),
                    b"..."
                }
                b"Ut5_..." => {
                    UnqualifiedName::UnnamedType(UnnamedTypeName(Some(5)), AbiTags::default()),
                    b"..."
                }
                b"L3foo_0..." => {
                    UnqualifiedName::LocalSourceName(
                        SourceName(Identifier {
                            start: 2,
                            end: 5
                        }),
                        Some(Discriminator(0)),
                        AbiTags::default(),
                    ),
                    "..."
                }
                b"L3foo..." => {
                    UnqualifiedName::LocalSourceName(
                        SourceName(Identifier {
                            start: 2,
                            end: 5
                        }),
                        None,
                        AbiTags::default(),
                    ),
                    "..."
                }
                b"quB1Q.." => {
                    UnqualifiedName::Operator(
                        OperatorName::Simple(SimpleOperatorName::Question),
                        AbiTags(vec![AbiTag(
                            SourceName(
                                Identifier {
                                    start: 4,
                                    end: 5,
                                },
                            ),
                        )])
                    ),
                    b".."
                }
                b"C1B1Q.." => {
                    UnqualifiedName::CtorDtor(
                        CtorDtorName::CompleteConstructor(None),
                        AbiTags(vec![AbiTag(
                            SourceName(
                                Identifier {
                                    start: 4,
                                    end: 5,
                                },
                            ),
                        )])
                    ),
                    b".."
                }
                b"10abcdefghijB1QB2lp..." => {
                    UnqualifiedName::Source(
                        SourceName(Identifier {
                            start: 2,
                            end: 12,
                        }),
                        AbiTags(vec![
                            AbiTag(
                                SourceName(
                                    Identifier {
                                        start: 14,
                                        end: 15,
                                    },
                                ),
                            ),
                            AbiTag(
                                SourceName(
                                    Identifier {
                                        start: 17,
                                        end: 19,
                                    },
                                ),
                            ),
                        ])
                    ),
                    b"..."
                }
                b"UllE_B1Q..." => {
                    UnqualifiedName::ClosureType(
                        ClosureTypeName(
                            LambdaSig(vec![
                                TypeHandle::Builtin(
                                    BuiltinType::Standard(
                                        StandardBuiltinType::Long))
                            ]),
                            None),
                        AbiTags(vec![AbiTag(
                            SourceName(
                                Identifier {
                                    start: 7,
                                    end: 8,
                                },
                            ),
                        )])
                    ),
                    b"..."
                }
                b"Ut5_B1QB2lp..." => {
                    UnqualifiedName::UnnamedType(
                        UnnamedTypeName(Some(5)),
                        AbiTags(vec![
                            AbiTag(
                                SourceName(
                                    Identifier {
                                        start: 6,
                                        end: 7,
                                    },
                                ),
                            ),
                            AbiTag(
                                SourceName(
                                    Identifier {
                                        start: 9,
                                        end: 11,
                                    },
                                ),
                            ),
                        ])
                    ),
                    b"..."
                }
                b"L3foo_0B1Q..." => {
                    UnqualifiedName::LocalSourceName(
                        SourceName(Identifier {
                            start: 2,
                            end: 5
                        }),
                        Some(Discriminator(0)),
                        AbiTags(vec![AbiTag(
                            SourceName(
                                Identifier {
                                    start: 9,
                                    end: 10,
                                },
                            ),
                        )])
                    ),
                    "..."
                }
                b"L3fooB1QB2lp..." => {
                    UnqualifiedName::LocalSourceName(
                        SourceName(Identifier {
                            start: 2,
                            end: 5
                        }),
                        None,
                        AbiTags(vec![
                            AbiTag(
                                SourceName(
                                    Identifier {
                                        start: 7,
                                        end: 8,
                                    },
                                ),
                            ),
                            AbiTag(
                                SourceName(
                                    Identifier {
                                        start: 10,
                                        end: 12,
                                    },
                                ),
                            )
                        ])
                    ),
                    "..."
                }
            }
            Err => {
                b"zzz" => Error::UnexpectedText,
                b"Uq" => Error::UnexpectedText,
                b"C" => Error::UnexpectedEnd,
                b"" => Error::UnexpectedEnd,
            }
        });
    }

    #[test]
    fn parse_unnamed_type_name() {
        assert_parse!(UnnamedTypeName {
            Ok => {
                b"Ut_abc" => {
                    UnnamedTypeName(None),
                    b"abc"
                }
                b"Ut42_abc" => {
                    UnnamedTypeName(Some(42)),
                    b"abc"
                }
                b"Ut42_" => {
                    UnnamedTypeName(Some(42)),
                    b""
                }
            }
            Err => {
                b"ut_" => Error::UnexpectedText,
                b"u" => Error::UnexpectedEnd,
                b"Ut" => Error::UnexpectedEnd,
                b"Ut._" => Error::UnexpectedText,
                b"Ut42" => Error::UnexpectedEnd,
            }
        });
    }

    #[test]
    fn parse_identifier() {
        assert_parse!(Identifier {
            Ok => {
                b"1abc" => {
                    Identifier { start: 0, end: 4 },
                    b""
                }
                b"_Az1\0\0\0" => {
                    Identifier { start: 0, end: 4 },
                    b"\0\0\0"
                }
                b"$_0\0\0\0" => {
                    Identifier { start: 0, end: 3 },
                    b"\0\0\0"
                }
            }
            Err => {
                b"\0\0\0" => Error::UnexpectedText,
                b"" => Error::UnexpectedEnd,
            }
        });
    }

    #[test]
    fn parse_source_name() {
        assert_parse!(SourceName {
            Ok => {
                b"1abc" => {
                    SourceName(Identifier { start: 1, end: 2 }),
                    b"bc"
                }
                b"10abcdefghijklm" => {
                    SourceName(Identifier { start: 2, end: 12 }),
                    b"klm"
                }
            }
            Err => {
                b"0abc" => Error::UnexpectedText,
                b"n1abc" => Error::UnexpectedText,
                b"10abcdef" => Error::UnexpectedEnd,
                b"" => Error::UnexpectedEnd,
            }
        });
    }

    #[test]
    fn parse_number() {
        assert_parse!(Number {
            Ok => {
                b"n2n3" => {
                    -2,
                    b"n3"
                }
                b"12345abcdef" => {
                    12345,
                    b"abcdef"
                }
                b"0abcdef" => {
                    0,
                    b"abcdef"
                }
                b"42" => {
                    42,
                    b""
                }
            }
            Err => {
                b"001" => Error::UnexpectedText,
                b"wutang" => Error::UnexpectedText,
                b"n" => Error::UnexpectedEnd,
                b"" => Error::UnexpectedEnd,
            }
        });
    }

    #[test]
    fn parse_call_offset() {
        assert_parse!(CallOffset {
            Ok => {
                b"hn42_..." => {
                    CallOffset::NonVirtual(NvOffset(-42)),
                    b"..."
                }
                b"vn42_36_..." => {
                    CallOffset::Virtual(VOffset(-42, 36)),
                    b"..."
                }
            }
            Err => {
                b"h1..." => Error::UnexpectedText,
                b"v1_1..." => Error::UnexpectedText,
                b"hh" => Error::UnexpectedText,
                b"vv" => Error::UnexpectedText,
                b"z" => Error::UnexpectedText,
                b"" => Error::UnexpectedEnd,
            }
        });
    }

    #[test]
    fn parse_v_offset() {
        assert_parse!(VOffset {
            Ok => {
                b"n2_n3abcdef" => {
                    VOffset(-2, -3),
                    b"abcdef"
                }
                b"12345_12345abcdef" => {
                    VOffset(12345, 12345),
                    b"abcdef"
                }
                b"0_0abcdef" => {
                    VOffset(0, 0),
                    b"abcdef"
                }
                b"42_n3" => {
                    VOffset(42, -3),
                    b""
                }
            }
            Err => {
                b"001" => Error::UnexpectedText,
                b"1_001" => Error::UnexpectedText,
                b"wutang" => Error::UnexpectedText,
                b"n_" => Error::UnexpectedText,
                b"1_n" => Error::UnexpectedEnd,
                b"1_" => Error::UnexpectedEnd,
                b"n" => Error::UnexpectedEnd,
                b"" => Error::UnexpectedEnd,
            }
        });
    }

    #[test]
    fn parse_nv_offset() {
        assert_parse!(NvOffset {
            Ok => {
                b"n2n3" => {
                    NvOffset(-2),
                    b"n3"
                }
                b"12345abcdef" => {
                    NvOffset(12345),
                    b"abcdef"
                }
                b"0abcdef" => {
                    NvOffset(0),
                    b"abcdef"
                }
                b"42" => {
                    NvOffset(42),
                    b""
                }
            }
            Err => {
                b"001" => Error::UnexpectedText,
                b"wutang" => Error::UnexpectedText,
                b"" => Error::UnexpectedEnd,
            }
        });
    }

    #[test]
    fn parse_seq_id() {
        assert_parse!(SeqId {
            Ok => {
                b"1_" => {
                    SeqId(1),
                    b"_"
                }
                b"42" => {
                    SeqId(146),
                    b""
                }
                b"ABCabc" => {
                    SeqId(13368),
                    b"abc"
                }
            }
            Err => {
                b"abc" => Error::UnexpectedText,
                b"001" => Error::UnexpectedText,
                b"wutang" => Error::UnexpectedText,
                b"" => Error::UnexpectedEnd,
            }
        });
    }

    #[test]
    fn parse_ctor_dtor_name() {
        assert_parse!(CtorDtorName {
            Ok => {
                b"D0" => {
                    CtorDtorName::DeletingDestructor,
                    b""
                }
                b"C101" => {
                    CtorDtorName::CompleteConstructor(None),
                    b"01"
                }
            }
            Err => {
                b"gayagaya" => Error::UnexpectedText,
                b"C" => Error::UnexpectedEnd,
                b"" => Error::UnexpectedEnd,
            }
        });
    }

    #[test]
    fn parse_operator_name() {
        assert_parse!(OperatorName {
            Ok => {
                b"qu..." => {
                    OperatorName::Simple(SimpleOperatorName::Question),
                    b"..."
                }
                b"cvi..." => {
                    OperatorName::Conversion(
                        TypeHandle::Builtin(
                            BuiltinType::Standard(
                                StandardBuiltinType::Int))),
                    b"..."
                }
                b"li3Foo..." => {
                    OperatorName::Literal(SourceName(Identifier {
                        start: 3,
                        end: 6,
                    })),
                    b"..."
                }
                b"v33Foo..." => {
                    OperatorName::VendorExtension(3, SourceName(Identifier {
                        start: 3,
                        end: 6
                    })),
                    b"..."
                }
            }
            Err => {
                b"cv" => Error::UnexpectedEnd,
                b"li3ab" => Error::UnexpectedEnd,
                b"li" => Error::UnexpectedEnd,
                b"v33ab" => Error::UnexpectedEnd,
                b"v3" => Error::UnexpectedEnd,
                b"v" => Error::UnexpectedEnd,
                b"" => Error::UnexpectedEnd,
                b"q" => Error::UnexpectedText,
                b"c" => Error::UnexpectedText,
                b"l" => Error::UnexpectedText,
                b"zzz" => Error::UnexpectedText,
            }
        });
    }

    #[test]
    fn parse_simple_operator_name() {
        assert_parse!(SimpleOperatorName {
            Ok => {
                b"qu" => {
                    SimpleOperatorName::Question,
                    b""
                }
                b"quokka" => {
                    SimpleOperatorName::Question,
                    b"okka"
                }
            }
            Err => {
                b"bu-buuuu" => Error::UnexpectedText,
                b"q" => Error::UnexpectedEnd,
                b"" => Error::UnexpectedEnd,
            }
        });
    }

    #[test]
    fn parse_subobject_expr() {
        assert_parse!(SubobjectExpr {
            with subs [] => {
                Ok => {
                    "PKcL_Z3FooEE..." => {
                        SubobjectExpr {
                            ty: TypeHandle::BackReference(1),
                            expr: Box::new(Expression::Primary(
                                ExprPrimary::External(
                                    MangledName::Encoding(
                                        Encoding::Data(
                                            Name::Unscoped(
                                                UnscopedName::Unqualified(
                                                    UnqualifiedName::Source(
                                                        SourceName(
                                                            Identifier {
                                                                start: 7,
                                                                end: 10,
                                                            }
                                                        ),
                                                        AbiTags::default()
                                                    )
                                                )
                                            )
                                        ),
                                        vec![]
                                    )
                                )
                            )),
                            offset: 0,
                        },
                        b"...",
                        [
                            Substitutable::Type(
                                Type::Qualified(
                                    CvQualifiers {
                                        restrict: false,
                                        volatile: false,
                                        const_: true,
                                    },
                                    TypeHandle::Builtin(
                                        BuiltinType::Standard(
                                            StandardBuiltinType::Char,
                                        ),
                                    ),
                                )
                            ),
                            Substitutable::Type(
                                Type::PointerTo(
                                    TypeHandle::BackReference(
                                        0,
                                    ),
                                ),
                            )
                        ]
                    }
                }
                Err => {
                    "" => Error::UnexpectedEnd,
                    "" => Error::UnexpectedEnd,
                }
            }
        });
    }
}
