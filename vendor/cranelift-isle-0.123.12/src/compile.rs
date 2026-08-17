//! Compilation process, from AST to Sema to Sequences of Insts.

use std::path::Path;
use std::sync::Arc;

use crate::ast::Def;
use crate::error::Errors;
use crate::files::Files;
use crate::{ast, codegen, overlap, sema};

/// Compile the given AST definitions into Rust source code.
pub fn compile(
    files: Arc<Files>,
    defs: &[ast::Def],
    options: &codegen::CodegenOptions,
) -> Result<String, Errors> {
    let mut type_env = match sema::TypeEnv::from_ast(defs) {
        Ok(type_env) => type_env,
        Err(errs) => return Err(Errors::new(errs, files)),
    };
    let term_env = match sema::TermEnv::from_ast(&mut type_env, defs, true) {
        Ok(term_env) => term_env,
        Err(errs) => return Err(Errors::new(errs, files)),
    };
    let terms = match overlap::check(&term_env) {
        Ok(terms) => terms,
        Err(errs) => return Err(Errors::new(errs, files)),
    };

    Ok(codegen::codegen(
        files, &type_env, &term_env, &terms, options,
    ))
}

/// Compile the given files into Rust source code.
pub fn from_files<P: AsRef<Path>>(
    inputs: impl IntoIterator<Item = P>,
    options: &codegen::CodegenOptions,
) -> Result<String, Errors> {
    let files = match Files::from_paths(inputs, &options.prefixes) {
        Ok(files) => files,
        Err((path, err)) => {
            return Err(Errors::from_io(
                err,
                format!("cannot read file {}", path.display()),
            ));
        }
    };

    let files = Arc::new(files);

    let mut defs = Vec::new();
    for (file, src) in files.file_texts.iter().enumerate() {
        let lexer = match crate::lexer::Lexer::new(file, src) {
            Ok(lexer) => lexer,
            Err(err) => return Err(Errors::new(vec![err], files)),
        };

        match crate::parser::parse(lexer) {
            Ok(mut ds) => defs.append(&mut ds),
            Err(err) => return Err(Errors::new(vec![err], files)),
        }
    }

    compile(files, &defs, options)
}

/// Construct the ISLE type and term environments for further analysis
/// (i.e., verification), without going all the way through codegen.
pub fn create_envs(
    inputs: Vec<std::path::PathBuf>,
) -> Result<(sema::TypeEnv, sema::TermEnv, Vec<Def>), Errors> {
    let files = match Files::from_paths(inputs, &[]) {
        Ok(files) => files,
        Err((path, err)) => {
            return Err(Errors::from_io(
                err,
                format!("cannot read file {}", path.display()),
            ));
        }
    };
    let files = Arc::new(files);
    let mut defs = Vec::new();
    for (file, src) in files.file_texts.iter().enumerate() {
        let lexer = match crate::lexer::Lexer::new(file, src) {
            Ok(lexer) => lexer,
            Err(err) => return Err(Errors::new(vec![err], files)),
        };

        match crate::parser::parse(lexer) {
            Ok(mut ds) => defs.append(&mut ds),
            Err(err) => return Err(Errors::new(vec![err], files)),
        }
    }
    let mut type_env = match sema::TypeEnv::from_ast(&defs) {
        Ok(type_env) => type_env,
        Err(errs) => return Err(Errors::new(errs, files)),
    };
    // We want to allow annotations on terms with internal extractors,
    // so we avoid expanding them within the sema rules.
    let term_env = match sema::TermEnv::from_ast(&mut type_env, &defs, false) {
        Ok(term_env) => term_env,
        Err(errs) => return Err(Errors::new(errs, files)),
    };
    Ok((type_env, term_env, defs))
}
