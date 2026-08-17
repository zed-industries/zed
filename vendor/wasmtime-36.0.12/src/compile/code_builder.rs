use crate::Engine;
use crate::prelude::*;
use std::borrow::Cow;
use std::path::Path;

/// Builder-style structure used to create a [`Module`](crate::module::Module) or
/// pre-compile a module to a serialized list of bytes.
///
/// This structure can be used for more advanced configuration when compiling a
/// WebAssembly module. Most configuration can use simpler constructors such as:
///
/// * [`Module::new`](crate::Module::new)
/// * [`Module::from_file`](crate::Module::from_file)
/// * [`Module::from_binary`](crate::Module::from_binary)
///
/// Note that a [`CodeBuilder`] always involves compiling WebAssembly bytes
/// to machine code. To deserialize a list of bytes use
/// [`Module::deserialize`](crate::Module::deserialize) instead.
///
/// A [`CodeBuilder`] requires a source of WebAssembly bytes to be configured
/// before calling [`compile_module_serialized`] or [`compile_module`]. This can
/// be provided with either the [`wasm_binary`] or [`wasm_binary_file`] method.
/// Note that only a single source of bytes can be provided.
///
/// # WebAssembly Text Format
///
/// This builder supports the WebAssembly Text Format (`*.wat` files) through
/// the [`CodeBuilder::wasm_binary_or_text`] and
/// [`CodeBuilder::wasm_binary_or_text_file`] methods. These methods
/// automatically convert WebAssembly text files to binary. Note though that
/// this behavior is disabled if the `wat` crate feature is not enabled.
///
/// [`compile_module_serialized`]: CodeBuilder::compile_module_serialized
/// [`compile_module`]: CodeBuilder::compile_module
/// [`wasm_binary`]: CodeBuilder::wasm_binary
/// [`wasm_binary_file`]: CodeBuilder::wasm_binary_file
pub struct CodeBuilder<'a> {
    pub(super) engine: &'a Engine,
    wasm: Option<Cow<'a, [u8]>>,
    wasm_path: Option<Cow<'a, Path>>,
    dwarf_package: Option<Cow<'a, [u8]>>,
    dwarf_package_path: Option<Cow<'a, Path>>,
}

/// Return value of [`CodeBuilder::hint`]
pub enum CodeHint {
    /// Hint that the code being compiled is a module.
    Module,
    /// Hint that the code being compiled is a component.
    Component,
}

impl<'a> CodeBuilder<'a> {
    /// Creates a new builder which will insert modules into the specified
    /// [`Engine`].
    pub fn new(engine: &'a Engine) -> CodeBuilder<'a> {
        CodeBuilder {
            engine,
            wasm: None,
            wasm_path: None,
            dwarf_package: None,
            dwarf_package_path: None,
        }
    }

    /// Configures the WebAssembly binary that is being compiled.
    ///
    /// The `wasm_bytes` parameter must be a binary WebAssembly file.
    /// This will be stored within the [`CodeBuilder`] for processing later when
    /// compilation is finalized.
    ///
    /// The optional `wasm_path` parameter is the path to the `wasm_bytes` on
    /// disk, if any. This may be used for diagnostics and other
    /// debugging-related purposes, but this method will not read the path
    /// specified.
    ///
    /// # Errors
    ///
    /// This method will return an error if WebAssembly bytes have already been
    /// configured.
    pub fn wasm_binary(
        &mut self,
        wasm_bytes: impl Into<Cow<'a, [u8]>>,
        wasm_path: Option<&'a Path>,
    ) -> Result<&mut Self> {
        if self.wasm.is_some() {
            bail!("cannot configure wasm bytes twice");
        }
        self.wasm = Some(wasm_bytes.into());
        self.wasm_path = wasm_path.map(|p| p.into());

        if self.wasm_path.is_some() {
            self.dwarf_package_from_wasm_path()?;
        }

        Ok(self)
    }

    /// Equivalent of [`CodeBuilder::wasm_binary`] that also accepts the
    /// WebAssembly text format.
    ///
    /// This method will configure the WebAssembly binary to be compiled. The
    /// input `wasm_bytes` may either be the wasm text format or the binary
    /// format. If the `wat` crate feature is enabled, which is enabled by
    /// default, then the text format will automatically be converted to the
    /// binary format.
    ///
    /// # Errors
    ///
    /// This method will return an error if WebAssembly bytes have already been
    /// configured. This method will also return an error if `wasm_bytes` is the
    /// wasm text format and the text syntax is not valid.
    pub fn wasm_binary_or_text(
        &mut self,
        wasm_bytes: &'a [u8],
        wasm_path: Option<&'a Path>,
    ) -> Result<&mut Self> {
        #[cfg(feature = "wat")]
        let wasm_bytes = wat::parse_bytes(wasm_bytes).map_err(|mut e| {
            if let Some(path) = wasm_path {
                e.set_path(path);
            }
            e
        })?;
        self.wasm_binary(wasm_bytes, wasm_path)
    }

    /// Reads the `file` specified for the WebAssembly bytes that are going to
    /// be compiled.
    ///
    /// This method will read `file` from the filesystem and interpret it
    /// as a WebAssembly binary.
    ///
    /// A DWARF package file will be probed using the root of `file` and with a
    /// `.dwp` extension. If found, it will be loaded and DWARF fusion
    /// performed.
    ///
    /// # Errors
    ///
    /// This method will return an error if WebAssembly bytes have already been
    /// configured.
    ///
    /// If `file` can't be read or an error happens reading it then that will
    /// also be returned.
    ///
    /// If DWARF fusion is performed and the DWARF packaged file cannot be read
    /// then an error will be returned.
    pub fn wasm_binary_file(&mut self, file: &'a Path) -> Result<&mut Self> {
        let wasm = std::fs::read(file)
            .with_context(|| format!("failed to read input file: {}", file.display()))?;
        self.wasm_binary(wasm, Some(file))
    }

    /// Equivalent of [`CodeBuilder::wasm_binary_file`] that also accepts the
    /// WebAssembly text format.
    ///
    /// This method is will read the file at `path` and interpret the contents
    /// to determine if it's the wasm text format or binary format. The file
    /// extension of `file` is not consulted. The text format is automatically
    /// converted to the binary format if the crate feature `wat` is active.
    ///
    /// # Errors
    ///
    /// In addition to the errors returned by [`CodeBuilder::wasm_binary_file`]
    /// this may also fail if the text format is read and the syntax is invalid.
    pub fn wasm_binary_or_text_file(&mut self, file: &'a Path) -> Result<&mut Self> {
        #[cfg(feature = "wat")]
        {
            let wasm = wat::parse_file(file)?;
            self.wasm_binary(wasm, Some(file))
        }
        #[cfg(not(feature = "wat"))]
        {
            self.wasm_binary_file(file)
        }
    }

    pub(super) fn get_wasm(&self) -> Result<&[u8]> {
        self.wasm
            .as_deref()
            .ok_or_else(|| anyhow!("no wasm bytes have been configured"))
    }

    /// Explicitly specify DWARF `.dwp` path.
    ///
    /// # Errors
    ///
    /// This method will return an error if the `.dwp` file has already been set
    /// through [`CodeBuilder::dwarf_package`] or auto-detection in
    /// [`CodeBuilder::wasm_binary_file`].
    ///
    /// This method will also return an error if `file` cannot be read.
    pub fn dwarf_package_file(&mut self, file: &Path) -> Result<&mut Self> {
        if self.dwarf_package.is_some() {
            bail!("cannot call `dwarf_package` or `dwarf_package_file` twice");
        }

        let dwarf_package = std::fs::read(file)
            .with_context(|| format!("failed to read dwarf input file: {}", file.display()))?;
        self.dwarf_package_path = Some(Cow::Owned(file.to_owned()));
        self.dwarf_package = Some(dwarf_package.into());

        Ok(self)
    }

    fn dwarf_package_from_wasm_path(&mut self) -> Result<&mut Self> {
        let dwarf_package_path_buf = self.wasm_path.as_ref().unwrap().with_extension("dwp");
        if dwarf_package_path_buf.exists() {
            return self.dwarf_package_file(dwarf_package_path_buf.as_path());
        }

        Ok(self)
    }

    /// Gets the DWARF package.
    pub(super) fn get_dwarf_package(&self) -> Option<&[u8]> {
        self.dwarf_package.as_deref()
    }

    /// Set the DWARF package binary.
    ///
    /// Initializes `dwarf_package` from `dwp_bytes` in preparation for
    /// DWARF fusion. Allows the DWARF package to be supplied as a byte array
    /// when the file probing performed in `wasm_file` is not appropriate.
    ///
    /// # Errors
    ///
    /// Returns an error if the `*.dwp` file is already set via auto-probing in
    /// [`CodeBuilder::wasm_binary_file`] or explicitly via
    /// [`CodeBuilder::dwarf_package_file`].
    pub fn dwarf_package(&mut self, dwp_bytes: &'a [u8]) -> Result<&mut Self> {
        if self.dwarf_package.is_some() {
            bail!("cannot call `dwarf_package` or `dwarf_package_file` twice");
        }
        self.dwarf_package = Some(dwp_bytes.into());
        Ok(self)
    }

    /// Returns a hint, if possible, of what the provided bytes are.
    ///
    /// This method can be use to detect what the previously supplied bytes to
    /// methods such as [`CodeBuilder::wasm_binary_or_text`] are. This will
    /// return whether a module or a component was found in the provided bytes.
    ///
    /// This method will return `None` if wasm bytes have not been configured
    /// or if the provided bytes don't look like either a component or a
    /// module.
    pub fn hint(&self) -> Option<CodeHint> {
        let wasm = self.wasm.as_ref()?;
        if wasmparser::Parser::is_component(wasm) {
            Some(CodeHint::Component)
        } else if wasmparser::Parser::is_core_wasm(wasm) {
            Some(CodeHint::Module)
        } else {
            None
        }
    }

    /// Finishes this compilation and produces a serialized list of bytes.
    ///
    /// This method requires that either [`CodeBuilder::wasm_binary`] or
    /// related methods were invoked prior to indicate what is being compiled.
    ///
    /// This method will block the current thread until compilation has
    /// finished, and when done the serialized artifact will be returned.
    ///
    /// Note that this method will never cache compilations, even if the
    /// `cache` feature is enabled.
    ///
    /// # Errors
    ///
    /// This can fail if the input wasm module was not valid or if another
    /// compilation-related error is encountered.
    pub fn compile_module_serialized(&self) -> Result<Vec<u8>> {
        let wasm = self.get_wasm()?;
        let dwarf_package = self.get_dwarf_package();
        let (v, _) = super::build_artifacts(self.engine, &wasm, dwarf_package.as_deref(), &())?;
        Ok(v)
    }

    /// Same as [`CodeBuilder::compile_module_serialized`] except that it
    /// compiles a serialized [`Component`](crate::component::Component)
    /// instead of a module.
    #[cfg(feature = "component-model")]
    pub fn compile_component_serialized(&self) -> Result<Vec<u8>> {
        let bytes = self.get_wasm()?;
        let (v, _) = super::build_component_artifacts(self.engine, &bytes, None, &())?;
        Ok(v)
    }
}

/// This is a helper struct used when caching to hash the state of an `Engine`
/// used for module compilation.
///
/// The hash computed for this structure is used to key the global wasmtime
/// cache and dictates whether artifacts are reused. Consequently the contents
/// of this hash dictate when artifacts are or aren't re-used.
pub struct HashedEngineCompileEnv<'a>(pub &'a Engine);

impl std::hash::Hash for HashedEngineCompileEnv<'_> {
    fn hash<H: std::hash::Hasher>(&self, hasher: &mut H) {
        // Hash the compiler's state based on its target and configuration.
        let compiler = self.0.compiler();
        compiler.triple().hash(hasher);
        compiler.flags().hash(hasher);
        compiler.isa_flags().hash(hasher);

        // Hash configuration state read for compilation
        let config = self.0.config();
        self.0.tunables().hash(hasher);
        self.0.features().hash(hasher);
        config.wmemcheck.hash(hasher);

        // Catch accidental bugs of reusing across crate versions.
        config.module_version.hash(hasher);
    }
}
