// Functionality that is shared between the cxx_build::bridge entry point and
// the cxxbridge CLI command.

mod block;
mod builtin;
mod cfg;
mod check;
pub(super) mod error;
mod file;
pub(super) mod fs;
mod guard;
mod ifndef;
pub(super) mod include;
mod names;
mod namespace;
mod nested;
pub(super) mod out;
mod pragma;
mod write;

use self::cfg::UnsupportedCfgEvaluator;
use self::error::{format_err, Result};
use self::file::File;
use self::include::Include;
use crate::syntax::cfg::CfgExpr;
use crate::syntax::report::Errors;
use crate::syntax::{self, attrs, Types};
use std::collections::BTreeSet as Set;
use std::path::Path;

pub(super) use self::error::Error;

/// Options for C++ code generation.
///
/// We expect options to be added over time, so this is a non-exhaustive struct.
/// To instantiate one you need to crate a default value and mutate those fields
/// that you want to modify.
///
/// ```
/// # use cxx_gen::Opt;
/// #
/// let impl_annotations = r#"__attribute__((visibility("default")))"#.to_owned();
///
/// let mut opt = Opt::default();
/// opt.cxx_impl_annotations = Some(impl_annotations);
/// ```
#[non_exhaustive]
pub struct Opt {
    /// Any additional headers to #include. The cxxbridge tool does not parse or
    /// even require the given paths to exist; they simply go into the generated
    /// C++ code as #include lines.
    pub include: Vec<Include>,
    /// Optional annotation for implementations of C++ function wrappers that
    /// may be exposed to Rust. You may for example need to provide
    /// `__declspec(dllexport)` or `__attribute__((visibility("default")))` if
    /// Rust code from one shared object or executable depends on these C++
    /// functions in another.
    pub cxx_impl_annotations: Option<String>,
    /// Impl for handling conditional compilation attributes.
    pub cfg_evaluator: Box<dyn CfgEvaluator>,

    pub(super) gen_header: bool,
    pub(super) gen_implementation: bool,
    pub(super) allow_dot_includes: bool,
    pub(super) doxygen: bool,
}

/// Logic to decide whether a conditional compilation attribute is enabled or
/// disabled.
pub trait CfgEvaluator {
    /// A name-only attribute such as `cfg(ident)` is passed with a `value` of
    /// None, while `cfg(key = "value")` is passed with the "value" in `value`.
    fn eval(&self, name: &str, value: Option<&str>) -> CfgResult;
}

/// Result of a [`CfgEvaluator`] evaluation.
pub enum CfgResult {
    /// Cfg option is enabled.
    True,
    /// Cfg option is disabled.
    False,
    /// Cfg option is neither enabled nor disabled.
    Undetermined {
        /// Message explaining why the cfg option is undetermined.
        msg: String,
    },
}

/// Results of code generation.
#[derive(Default)]
pub struct GeneratedCode {
    /// The bytes of a C++ header file.
    pub header: Vec<u8>,
    /// The bytes of a C++ implementation file (e.g. .cc, cpp etc.)
    pub implementation: Vec<u8>,
}

impl Default for Opt {
    fn default() -> Self {
        Opt {
            include: Vec::new(),
            cxx_impl_annotations: None,
            gen_header: true,
            gen_implementation: true,
            allow_dot_includes: true,
            cfg_evaluator: Box::new(UnsupportedCfgEvaluator),
            doxygen: false,
        }
    }
}

pub(super) fn generate_from_path(path: &Path, opt: &Opt) -> GeneratedCode {
    let source = match read_to_string(path) {
        Ok(source) => source,
        Err(err) => format_err(path, "", err),
    };
    match generate_from_string(&source, opt) {
        Ok(out) => out,
        Err(err) => format_err(path, &source, err),
    }
}

fn read_to_string(path: &Path) -> Result<String> {
    let bytes = if path == Path::new("-") {
        fs::read_stdin()
    } else {
        fs::read(path)
    }?;
    match String::from_utf8(bytes) {
        Ok(string) => Ok(string),
        Err(err) => Err(Error::Utf8(path.to_owned(), err.utf8_error())),
    }
}

fn generate_from_string(source: &str, opt: &Opt) -> Result<GeneratedCode> {
    let mut source = source;
    if source.starts_with("#!") && !source.starts_with("#![") {
        let shebang_end = source.find('\n').unwrap_or(source.len());
        source = &source[shebang_end..];
    }
    let syntax: File = syn::parse_str(source)?;
    generate(syntax, opt)
}

pub(super) fn generate(syntax: File, opt: &Opt) -> Result<GeneratedCode> {
    if syntax.modules.is_empty() {
        return Err(Error::NoBridgeMod);
    }

    let ref mut apis = Vec::new();
    let ref mut errors = Errors::new();
    let ref mut cfg_errors = Set::new();
    for bridge in syntax.modules {
        let mut cfg = CfgExpr::Unconditional;
        let _ = attrs::parse(
            errors,
            bridge.attrs,
            attrs::Parser {
                cfg: Some(&mut cfg),
                ignore_unrecognized: true,
                ..Default::default()
            },
        );
        if cfg::eval(errors, cfg_errors, opt.cfg_evaluator.as_ref(), &cfg) {
            let ref namespace = bridge.namespace;
            let trusted = bridge.unsafety.is_some();
            apis.extend(syntax::parse_items(
                errors,
                bridge.content,
                trusted,
                namespace,
            ));
        }
    }

    cfg::strip(errors, cfg_errors, opt.cfg_evaluator.as_ref(), apis);
    errors.propagate()?;

    let ref types = Types::collect(errors, apis);
    check::precheck(errors, apis, opt);
    errors.propagate()?;

    let generator = check::Generator::Build;
    check::typecheck(errors, apis, types, generator);
    errors.propagate()?;

    // Some callers may wish to generate both header and implementation from the
    // same token stream to avoid parsing twice. Others only need to generate
    // one or the other.
    let (mut header, mut implementation) = Default::default();
    if opt.gen_header {
        header = write::gen(apis, types, opt, true);
    }
    if opt.gen_implementation {
        implementation = write::gen(apis, types, opt, false);
    }
    Ok(GeneratedCode {
        header,
        implementation,
    })
}
