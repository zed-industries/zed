use crate::prelude::*;
#[cfg(feature = "std")]
use crate::runtime::vm::open_file_for_mmap;
use crate::runtime::vm::{CompiledModuleId, ModuleMemoryImages, VMWasmCallFunction};
use crate::sync::OnceLock;
use crate::{
    Engine,
    code::CodeObject,
    code_memory::CodeMemory,
    instantiate::CompiledModule,
    resources::ResourcesRequired,
    types::{ExportType, ExternType, ImportType},
};
use alloc::sync::Arc;
use core::fmt;
use core::ops::Range;
use core::ptr::NonNull;
#[cfg(feature = "std")]
use std::{fs::File, path::Path};
use wasmparser::{Parser, ValidPayload, Validator};
use wasmtime_environ::{
    CompiledModuleInfo, EntityIndex, HostPtr, ModuleTypes, ObjectKind, TypeTrace, VMOffsets,
    VMSharedTypeIndex,
};
mod registry;

pub use registry::*;

/// A compiled WebAssembly module, ready to be instantiated.
///
/// A `Module` is a compiled in-memory representation of an input WebAssembly
/// binary. A `Module` is then used to create an [`Instance`](crate::Instance)
/// through an instantiation process. You cannot call functions or fetch
/// globals, for example, on a `Module` because it's purely a code
/// representation. Instead you'll need to create an
/// [`Instance`](crate::Instance) to interact with the wasm module.
///
/// A `Module` can be created by compiling WebAssembly code through APIs such as
/// [`Module::new`]. This would be a JIT-style use case where code is compiled
/// just before it's used. Alternatively a `Module` can be compiled in one
/// process and [`Module::serialize`] can be used to save it to storage. A later
/// call to [`Module::deserialize`] will quickly load the module to execute and
/// does not need to compile any code, representing a more AOT-style use case.
///
/// Currently a `Module` does not implement any form of tiering or dynamic
/// optimization of compiled code. Creation of a `Module` via [`Module::new`] or
/// related APIs will perform the entire compilation step synchronously. When
/// finished no further compilation will happen at runtime or later during
/// execution of WebAssembly instances for example.
///
/// Compilation of WebAssembly by default goes through Cranelift and is
/// recommended to be done once-per-module. The same WebAssembly binary need not
/// be compiled multiple times and can instead used an embedder-cached result of
/// the first call.
///
/// `Module` is thread-safe and safe to share across threads.
///
/// ## Modules and `Clone`
///
/// Using `clone` on a `Module` is a cheap operation. It will not create an
/// entirely new module, but rather just a new reference to the existing module.
/// In other words it's a shallow copy, not a deep copy.
///
/// ## Examples
///
/// There are a number of ways you can create a `Module`, for example pulling
/// the bytes from a number of locations. One example is loading a module from
/// the filesystem:
///
/// ```no_run
/// # use wasmtime::*;
/// # fn main() -> anyhow::Result<()> {
/// let engine = Engine::default();
/// let module = Module::from_file(&engine, "path/to/foo.wasm")?;
/// # Ok(())
/// # }
/// ```
///
/// You can also load the wasm text format if more convenient too:
///
/// ```no_run
/// # use wasmtime::*;
/// # fn main() -> anyhow::Result<()> {
/// let engine = Engine::default();
/// // Now we're using the WebAssembly text extension: `.wat`!
/// let module = Module::from_file(&engine, "path/to/foo.wat")?;
/// # Ok(())
/// # }
/// ```
///
/// And if you've already got the bytes in-memory you can use the
/// [`Module::new`] constructor:
///
/// ```no_run
/// # use wasmtime::*;
/// # fn main() -> anyhow::Result<()> {
/// let engine = Engine::default();
/// # let wasm_bytes: Vec<u8> = Vec::new();
/// let module = Module::new(&engine, &wasm_bytes)?;
///
/// // It also works with the text format!
/// let module = Module::new(&engine, "(module (func))")?;
/// # Ok(())
/// # }
/// ```
///
/// Serializing and deserializing a module looks like:
///
/// ```no_run
/// # use wasmtime::*;
/// # fn main() -> anyhow::Result<()> {
/// let engine = Engine::default();
/// # let wasm_bytes: Vec<u8> = Vec::new();
/// let module = Module::new(&engine, &wasm_bytes)?;
/// let module_bytes = module.serialize()?;
///
/// // ... can save `module_bytes` to disk or other storage ...
///
/// // recreate the module from the serialized bytes. For the `unsafe` bits
/// // see the documentation of `deserialize`.
/// let module = unsafe { Module::deserialize(&engine, &module_bytes)? };
/// # Ok(())
/// # }
/// ```
///
/// [`Config`]: crate::Config
#[derive(Clone)]
pub struct Module {
    inner: Arc<ModuleInner>,
}

struct ModuleInner {
    engine: Engine,
    /// The compiled artifacts for this module that will be instantiated and
    /// executed.
    module: CompiledModule,

    /// Runtime information such as the underlying mmap, type information, etc.
    ///
    /// Note that this `Arc` is used to share information between compiled
    /// modules within a component. For bare core wasm modules created with
    /// `Module::new`, for example, this is a uniquely owned `Arc`.
    code: Arc<CodeObject>,

    /// A set of initialization images for memories, if any.
    ///
    /// Note that this is behind a `OnceCell` to lazily create this image. On
    /// Linux where `memfd_create` may be used to create the backing memory
    /// image this is a pretty expensive operation, so by deferring it this
    /// improves memory usage for modules that are created but may not ever be
    /// instantiated.
    memory_images: OnceLock<Option<ModuleMemoryImages>>,

    /// Flag indicating whether this module can be serialized or not.
    #[cfg(any(feature = "cranelift", feature = "winch"))]
    serializable: bool,

    /// Runtime offset information for `VMContext`.
    offsets: VMOffsets<HostPtr>,
}

impl fmt::Debug for Module {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Module")
            .field("name", &self.name())
            .finish_non_exhaustive()
    }
}

impl fmt::Debug for ModuleInner {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ModuleInner")
            .field("name", &self.module.module().name.as_ref())
            .finish_non_exhaustive()
    }
}

impl Module {
    /// Creates a new WebAssembly `Module` from the given in-memory `bytes`.
    ///
    /// The `bytes` provided must be in one of the following formats:
    ///
    /// * A [binary-encoded][binary] WebAssembly module. This is always supported.
    /// * A [text-encoded][text] instance of the WebAssembly text format.
    ///   This is only supported when the `wat` feature of this crate is enabled.
    ///   If this is supplied then the text format will be parsed before validation.
    ///   Note that the `wat` feature is enabled by default.
    ///
    /// The data for the wasm module must be loaded in-memory if it's present
    /// elsewhere, for example on disk. This requires that the entire binary is
    /// loaded into memory all at once, this API does not support streaming
    /// compilation of a module.
    ///
    /// The WebAssembly binary will be decoded and validated. It will also be
    /// compiled according to the configuration of the provided `engine`.
    ///
    /// # Errors
    ///
    /// This function may fail and return an error. Errors may include
    /// situations such as:
    ///
    /// * The binary provided could not be decoded because it's not a valid
    ///   WebAssembly binary
    /// * The WebAssembly binary may not validate (e.g. contains type errors)
    /// * Implementation-specific limits were exceeded with a valid binary (for
    ///   example too many locals)
    /// * The wasm binary may use features that are not enabled in the
    ///   configuration of `engine`
    /// * If the `wat` feature is enabled and the input is text, then it may be
    ///   rejected if it fails to parse.
    ///
    /// The error returned should contain full information about why module
    /// creation failed if one is returned.
    ///
    /// [binary]: https://webassembly.github.io/spec/core/binary/index.html
    /// [text]: https://webassembly.github.io/spec/core/text/index.html
    ///
    /// # Examples
    ///
    /// The `new` function can be invoked with a in-memory array of bytes:
    ///
    /// ```no_run
    /// # use wasmtime::*;
    /// # fn main() -> anyhow::Result<()> {
    /// # let engine = Engine::default();
    /// # let wasm_bytes: Vec<u8> = Vec::new();
    /// let module = Module::new(&engine, &wasm_bytes)?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// Or you can also pass in a string to be parsed as the wasm text
    /// format:
    ///
    /// ```
    /// # use wasmtime::*;
    /// # fn main() -> anyhow::Result<()> {
    /// # let engine = Engine::default();
    /// let module = Module::new(&engine, "(module (func))")?;
    /// # Ok(())
    /// # }
    /// ```
    #[cfg(any(feature = "cranelift", feature = "winch"))]
    pub fn new(engine: &Engine, bytes: impl AsRef<[u8]>) -> Result<Module> {
        crate::CodeBuilder::new(engine)
            .wasm_binary_or_text(bytes.as_ref(), None)?
            .compile_module()
    }

    /// Creates a new WebAssembly `Module` from the contents of the given
    /// `file` on disk.
    ///
    /// This is a convenience function that will read the `file` provided and
    /// pass the bytes to the [`Module::new`] function. For more information
    /// see [`Module::new`]
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use wasmtime::*;
    /// # fn main() -> anyhow::Result<()> {
    /// let engine = Engine::default();
    /// let module = Module::from_file(&engine, "./path/to/foo.wasm")?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// The `.wat` text format is also supported:
    ///
    /// ```no_run
    /// # use wasmtime::*;
    /// # fn main() -> anyhow::Result<()> {
    /// # let engine = Engine::default();
    /// let module = Module::from_file(&engine, "./path/to/foo.wat")?;
    /// # Ok(())
    /// # }
    /// ```
    #[cfg(all(feature = "std", any(feature = "cranelift", feature = "winch")))]
    pub fn from_file(engine: &Engine, file: impl AsRef<Path>) -> Result<Module> {
        crate::CodeBuilder::new(engine)
            .wasm_binary_or_text_file(file.as_ref())?
            .compile_module()
    }

    /// Creates a new WebAssembly `Module` from the given in-memory `binary`
    /// data.
    ///
    /// This is similar to [`Module::new`] except that it requires that the
    /// `binary` input is a WebAssembly binary, the text format is not supported
    /// by this function. It's generally recommended to use [`Module::new`], but
    /// if it's required to not support the text format this function can be
    /// used instead.
    ///
    /// # Examples
    ///
    /// ```
    /// # use wasmtime::*;
    /// # fn main() -> anyhow::Result<()> {
    /// # let engine = Engine::default();
    /// let wasm = b"\0asm\x01\0\0\0";
    /// let module = Module::from_binary(&engine, wasm)?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// Note that the text format is **not** accepted by this function:
    ///
    /// ```
    /// # use wasmtime::*;
    /// # fn main() -> anyhow::Result<()> {
    /// # let engine = Engine::default();
    /// assert!(Module::from_binary(&engine, b"(module)").is_err());
    /// # Ok(())
    /// # }
    /// ```
    #[cfg(any(feature = "cranelift", feature = "winch"))]
    pub fn from_binary(engine: &Engine, binary: &[u8]) -> Result<Module> {
        crate::CodeBuilder::new(engine)
            .wasm_binary(binary, None)?
            .compile_module()
    }

    /// Creates a new WebAssembly `Module` from the contents of the given `file`
    /// on disk, but with assumptions that the file is from a trusted source.
    /// The file should be a binary- or text-format WebAssembly module, or a
    /// precompiled artifact generated by the same version of Wasmtime.
    ///
    /// # Unsafety
    ///
    /// All of the reasons that [`deserialize`] is `unsafe` apply to this
    /// function as well. Arbitrary data loaded from a file may trick Wasmtime
    /// into arbitrary code execution since the contents of the file are not
    /// validated to be a valid precompiled module.
    ///
    /// [`deserialize`]: Module::deserialize
    ///
    /// Additionally though this function is also `unsafe` because the file
    /// referenced must remain unchanged and a valid precompiled module for the
    /// entire lifetime of the [`Module`] returned. Any changes to the file on
    /// disk may change future instantiations of the module to be incorrect.
    /// This is because the file is mapped into memory and lazily loaded pages
    /// reflect the current state of the file, not necessarily the original
    /// state of the file.
    #[cfg(all(feature = "std", any(feature = "cranelift", feature = "winch")))]
    pub unsafe fn from_trusted_file(engine: &Engine, file: impl AsRef<Path>) -> Result<Module> {
        let open_file = open_file_for_mmap(file.as_ref())?;
        let mmap = crate::runtime::vm::MmapVec::from_file(open_file)?;
        if &mmap[0..4] == b"\x7fELF" {
            let code = engine.load_code(mmap, ObjectKind::Module)?;
            return Module::from_parts(engine, code, None);
        }

        crate::CodeBuilder::new(engine)
            .wasm_binary_or_text(&mmap[..], Some(file.as_ref()))?
            .compile_module()
    }

    /// Deserializes an in-memory compiled module previously created with
    /// [`Module::serialize`] or [`Engine::precompile_module`].
    ///
    /// This function will deserialize the binary blobs emitted by
    /// [`Module::serialize`] and [`Engine::precompile_module`] back into an
    /// in-memory [`Module`] that's ready to be instantiated.
    ///
    /// Note that the [`Module::deserialize_file`] method is more optimized than
    /// this function, so if the serialized module is already present in a file
    /// it's recommended to use that method instead.
    ///
    /// # Unsafety
    ///
    /// This function is marked as `unsafe` because if fed invalid input or used
    /// improperly this could lead to memory safety vulnerabilities. This method
    /// should not, for example, be exposed to arbitrary user input.
    ///
    /// The structure of the binary blob read here is only lightly validated
    /// internally in `wasmtime`. This is intended to be an efficient
    /// "rehydration" for a [`Module`] which has very few runtime checks beyond
    /// deserialization. Arbitrary input could, for example, replace valid
    /// compiled code with any other valid compiled code, meaning that this can
    /// trivially be used to execute arbitrary code otherwise.
    ///
    /// For these reasons this function is `unsafe`. This function is only
    /// designed to receive the previous input from [`Module::serialize`] and
    /// [`Engine::precompile_module`]. If the exact output of those functions
    /// (unmodified) is passed to this function then calls to this function can
    /// be considered safe. It is the caller's responsibility to provide the
    /// guarantee that only previously-serialized bytes are being passed in
    /// here.
    ///
    /// Note that this function is designed to be safe receiving output from
    /// *any* compiled version of `wasmtime` itself. This means that it is safe
    /// to feed output from older versions of Wasmtime into this function, in
    /// addition to newer versions of wasmtime (from the future!). These inputs
    /// will deterministically and safely produce an `Err`. This function only
    /// successfully accepts inputs from the same version of `wasmtime`, but the
    /// safety guarantee only applies to externally-defined blobs of bytes, not
    /// those defined by any version of wasmtime. (this means that if you cache
    /// blobs across versions of wasmtime you can be safely guaranteed that
    /// future versions of wasmtime will reject old cache entries).
    pub unsafe fn deserialize(engine: &Engine, bytes: impl AsRef<[u8]>) -> Result<Module> {
        let code = engine.load_code_bytes(bytes.as_ref(), ObjectKind::Module)?;
        Module::from_parts(engine, code, None)
    }

    /// In-place deserialization of an in-memory compiled module previously
    /// created with [`Module::serialize`] or [`Engine::precompile_module`].
    ///
    /// See [`Self::deserialize`] for additional information; this method
    /// works identically except that it will not create a copy of the provided
    /// memory but will use it directly.
    ///
    /// # Unsafety
    ///
    /// All of the safety notes from [`Self::deserialize`] apply here as well
    /// with the additional constraint that the code memory provide by `memory`
    /// lives for as long as the module and is nevery externally modified for
    /// the lifetime of the deserialized module.
    pub unsafe fn deserialize_raw(engine: &Engine, memory: NonNull<[u8]>) -> Result<Module> {
        // SAFETY: the contract required by `load_code_raw` is the same as this
        // function.
        let code = unsafe { engine.load_code_raw(memory, ObjectKind::Module)? };
        Module::from_parts(engine, code, None)
    }

    /// Same as [`deserialize`], except that the contents of `path` are read to
    /// deserialize into a [`Module`].
    ///
    /// This method is provided because it can be faster than [`deserialize`]
    /// since the data doesn't need to be copied around, but rather the module
    /// can be used directly from an mmap'd view of the file provided.
    ///
    /// [`deserialize`]: Module::deserialize
    ///
    /// # Unsafety
    ///
    /// All of the reasons that [`deserialize`] is `unsafe` applies to this
    /// function as well. Arbitrary data loaded from a file may trick Wasmtime
    /// into arbitrary code execution since the contents of the file are not
    /// validated to be a valid precompiled module.
    ///
    /// Additionally though this function is also `unsafe` because the file
    /// referenced must remain unchanged and a valid precompiled module for the
    /// entire lifetime of the [`Module`] returned. Any changes to the file on
    /// disk may change future instantiations of the module to be incorrect.
    /// This is because the file is mapped into memory and lazily loaded pages
    /// reflect the current state of the file, not necessarily the original
    /// state of the file.
    #[cfg(feature = "std")]
    pub unsafe fn deserialize_file(engine: &Engine, path: impl AsRef<Path>) -> Result<Module> {
        let file = open_file_for_mmap(path.as_ref())?;
        // SAFETY: the contract of `deserialize_open_file` is the samea s this
        // function.
        unsafe {
            Self::deserialize_open_file(engine, file)
                .with_context(|| format!("failed deserialization for: {}", path.as_ref().display()))
        }
    }

    /// Same as [`deserialize_file`], except that it takes an open `File`
    /// instead of a path.
    ///
    /// This method is provided because it can be used instead of
    /// [`deserialize_file`] in situations where `wasmtime` is running with
    /// limited file system permissions. In that case a process
    /// with file system access can pass already opened files to `wasmtime`.
    ///
    /// [`deserialize_file`]: Module::deserialize_file
    ///
    /// Note that the corresponding will be mapped as private writeable
    /// (copy-on-write) and executable. For `windows` this means the file needs
    /// to be opened with at least `FILE_GENERIC_READ | FILE_GENERIC_EXECUTE`
    /// [`access_mode`].
    ///
    /// [`access_mode`]: https://doc.rust-lang.org/std/os/windows/fs/trait.OpenOptionsExt.html#tymethod.access_mode
    ///
    /// # Unsafety
    ///
    /// All of the reasons that [`deserialize_file`] is `unsafe` applies to this
    /// function as well.
    #[cfg(feature = "std")]
    pub unsafe fn deserialize_open_file(engine: &Engine, file: File) -> Result<Module> {
        let code = engine.load_code_file(file, ObjectKind::Module)?;
        Module::from_parts(engine, code, None)
    }

    /// Entrypoint for creating a `Module` for all above functions, both
    /// of the AOT and jit-compiled categories.
    ///
    /// In all cases the compilation artifact, `code_memory`, is provided here.
    /// The `info_and_types` argument is `None` when a module is being
    /// deserialized from a precompiled artifact or it's `Some` if it was just
    /// compiled and the values are already available.
    pub(crate) fn from_parts(
        engine: &Engine,
        code_memory: Arc<CodeMemory>,
        info_and_types: Option<(CompiledModuleInfo, ModuleTypes)>,
    ) -> Result<Self> {
        // Acquire this module's metadata and type information, deserializing
        // it from the provided artifact if it wasn't otherwise provided
        // already.
        let (mut info, mut types) = match info_and_types {
            Some((info, types)) => (info, types),
            None => postcard::from_bytes(code_memory.wasmtime_info())?,
        };

        // Register function type signatures into the engine for the lifetime
        // of the `Module` that will be returned. This notably also builds up
        // maps for trampolines to be used for this module when inserted into
        // stores.
        //
        // Note that the unsafety here should be ok since the `trampolines`
        // field should only point to valid trampoline function pointers
        // within the text section.
        let signatures =
            engine.register_and_canonicalize_types(&mut types, core::iter::once(&mut info.module));

        // Package up all our data into a `CodeObject` and delegate to the final
        // step of module compilation.
        let code = Arc::new(CodeObject::new(code_memory, signatures, types.into()));
        Module::from_parts_raw(engine, code, info, true)
    }

    pub(crate) fn from_parts_raw(
        engine: &Engine,
        code: Arc<CodeObject>,
        info: CompiledModuleInfo,
        serializable: bool,
    ) -> Result<Self> {
        let module =
            CompiledModule::from_artifacts(code.code_memory().clone(), info, engine.profiler())?;

        // Validate the module can be used with the current instance allocator.
        let offsets = VMOffsets::new(HostPtr, module.module());
        engine
            .allocator()
            .validate_module(module.module(), &offsets)?;

        let _ = serializable;

        Ok(Self {
            inner: Arc::new(ModuleInner {
                engine: engine.clone(),
                code,
                memory_images: OnceLock::new(),
                module,
                #[cfg(any(feature = "cranelift", feature = "winch"))]
                serializable,
                offsets,
            }),
        })
    }

    /// Validates `binary` input data as a WebAssembly binary given the
    /// configuration in `engine`.
    ///
    /// This function will perform a speedy validation of the `binary` input
    /// WebAssembly module (which is in [binary form][binary], the text format
    /// is not accepted by this function) and return either `Ok` or `Err`
    /// depending on the results of validation. The `engine` argument indicates
    /// configuration for WebAssembly features, for example, which are used to
    /// indicate what should be valid and what shouldn't be.
    ///
    /// Validation automatically happens as part of [`Module::new`].
    ///
    /// # Errors
    ///
    /// If validation fails for any reason (type check error, usage of a feature
    /// that wasn't enabled, etc) then an error with a description of the
    /// validation issue will be returned.
    ///
    /// [binary]: https://webassembly.github.io/spec/core/binary/index.html
    pub fn validate(engine: &Engine, binary: &[u8]) -> Result<()> {
        let mut validator = Validator::new_with_features(engine.features());

        let mut functions = Vec::new();
        for payload in Parser::new(0).parse_all(binary) {
            let payload = payload?;
            if let ValidPayload::Func(a, b) = validator.payload(&payload)? {
                functions.push((a, b));
            }
            if let wasmparser::Payload::Version { encoding, .. } = &payload {
                if let wasmparser::Encoding::Component = encoding {
                    bail!("component passed to module validation");
                }
            }
        }

        engine.run_maybe_parallel(functions, |(validator, body)| {
            // FIXME: it would be best here to use a rayon-specific parallel
            // iterator that maintains state-per-thread to share the function
            // validator allocations (`Default::default` here) across multiple
            // functions.
            validator.into_validator(Default::default()).validate(&body)
        })?;
        Ok(())
    }

    /// Serializes this module to a vector of bytes.
    ///
    /// This function is similar to the [`Engine::precompile_module`] method
    /// where it produces an artifact of Wasmtime which is suitable to later
    /// pass into [`Module::deserialize`]. If a module is never instantiated
    /// then it's recommended to use [`Engine::precompile_module`] instead of
    /// this method, but if a module is both instantiated and serialized then
    /// this method can be useful to get the serialized version without
    /// compiling twice.
    #[cfg(any(feature = "cranelift", feature = "winch"))]
    pub fn serialize(&self) -> Result<Vec<u8>> {
        // The current representation of compiled modules within a compiled
        // component means that it cannot be serialized. The mmap returned here
        // is the mmap for the entire component and while it contains all
        // necessary data to deserialize this particular module it's all
        // embedded within component-specific information.
        //
        // It's not the hardest thing in the world to support this but it's
        // expected that there's not much of a use case at this time. In theory
        // all that needs to be done is to edit the `.wasmtime.info` section
        // to contains this module's metadata instead of the metadata for the
        // whole component. The metadata itself is fairly trivially
        // recreateable here it's more that there's no easy one-off API for
        // editing the sections of an ELF object to use here.
        //
        // Overall for now this simply always returns an error in this
        // situation. If you're reading this and feel that the situation should
        // be different please feel free to open an issue.
        if !self.inner.serializable {
            bail!("cannot serialize a module exported from a component");
        }
        Ok(self.compiled_module().mmap().to_vec())
    }

    pub(crate) fn compiled_module(&self) -> &CompiledModule {
        &self.inner.module
    }

    pub(crate) fn code_object(&self) -> &Arc<CodeObject> {
        &self.inner.code
    }

    pub(crate) fn env_module(&self) -> &Arc<wasmtime_environ::Module> {
        self.compiled_module().module()
    }

    pub(crate) fn types(&self) -> &ModuleTypes {
        self.inner.code.module_types()
    }

    #[cfg(any(feature = "component-model", feature = "gc-drc"))]
    pub(crate) fn signatures(&self) -> &crate::type_registry::TypeCollection {
        self.inner.code.signatures()
    }

    /// Returns identifier/name that this [`Module`] has. This name
    /// is used in traps/backtrace details.
    ///
    /// Note that most LLVM/clang/Rust-produced modules do not have a name
    /// associated with them, but other wasm tooling can be used to inject or
    /// add a name.
    ///
    /// # Examples
    ///
    /// ```
    /// # use wasmtime::*;
    /// # fn main() -> anyhow::Result<()> {
    /// # let engine = Engine::default();
    /// let module = Module::new(&engine, "(module $foo)")?;
    /// assert_eq!(module.name(), Some("foo"));
    ///
    /// let module = Module::new(&engine, "(module)")?;
    /// assert_eq!(module.name(), None);
    ///
    /// # Ok(())
    /// # }
    /// ```
    pub fn name(&self) -> Option<&str> {
        self.compiled_module().module().name.as_deref()
    }

    /// Returns the list of imports that this [`Module`] has and must be
    /// satisfied.
    ///
    /// This function returns the list of imports that the wasm module has, but
    /// only the types of each import. The type of each import is used to
    /// typecheck the [`Instance::new`](crate::Instance::new) method's `imports`
    /// argument. The arguments to that function must match up 1-to-1 with the
    /// entries in the array returned here.
    ///
    /// The imports returned reflect the order of the imports in the wasm module
    /// itself, and note that no form of deduplication happens.
    ///
    /// # Examples
    ///
    /// Modules with no imports return an empty list here:
    ///
    /// ```
    /// # use wasmtime::*;
    /// # fn main() -> anyhow::Result<()> {
    /// # let engine = Engine::default();
    /// let module = Module::new(&engine, "(module)")?;
    /// assert_eq!(module.imports().len(), 0);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// and modules with imports will have a non-empty list:
    ///
    /// ```
    /// # use wasmtime::*;
    /// # fn main() -> anyhow::Result<()> {
    /// # let engine = Engine::default();
    /// let wat = r#"
    ///     (module
    ///         (import "host" "foo" (func))
    ///     )
    /// "#;
    /// let module = Module::new(&engine, wat)?;
    /// assert_eq!(module.imports().len(), 1);
    /// let import = module.imports().next().unwrap();
    /// assert_eq!(import.module(), "host");
    /// assert_eq!(import.name(), "foo");
    /// match import.ty() {
    ///     ExternType::Func(_) => { /* ... */ }
    ///     _ => panic!("unexpected import type!"),
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub fn imports<'module>(
        &'module self,
    ) -> impl ExactSizeIterator<Item = ImportType<'module>> + 'module {
        let module = self.compiled_module().module();
        let types = self.types();
        let engine = self.engine();
        module
            .imports()
            .map(move |(imp_mod, imp_field, ty)| {
                debug_assert!(ty.is_canonicalized_for_runtime_usage());
                ImportType::new(imp_mod, imp_field, ty, types, engine)
            })
            .collect::<Vec<_>>()
            .into_iter()
    }

    /// Returns the list of exports that this [`Module`] has and will be
    /// available after instantiation.
    ///
    /// This function will return the type of each item that will be returned
    /// from [`Instance::exports`](crate::Instance::exports). Each entry in this
    /// list corresponds 1-to-1 with that list, and the entries here will
    /// indicate the name of the export along with the type of the export.
    ///
    /// # Examples
    ///
    /// Modules might not have any exports:
    ///
    /// ```
    /// # use wasmtime::*;
    /// # fn main() -> anyhow::Result<()> {
    /// # let engine = Engine::default();
    /// let module = Module::new(&engine, "(module)")?;
    /// assert!(module.exports().next().is_none());
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// When the exports are not empty, you can inspect each export:
    ///
    /// ```
    /// # use wasmtime::*;
    /// # fn main() -> anyhow::Result<()> {
    /// # let engine = Engine::default();
    /// let wat = r#"
    ///     (module
    ///         (func (export "foo"))
    ///         (memory (export "memory") 1)
    ///     )
    /// "#;
    /// let module = Module::new(&engine, wat)?;
    /// assert_eq!(module.exports().len(), 2);
    ///
    /// let mut exports = module.exports();
    /// let foo = exports.next().unwrap();
    /// assert_eq!(foo.name(), "foo");
    /// match foo.ty() {
    ///     ExternType::Func(_) => { /* ... */ }
    ///     _ => panic!("unexpected export type!"),
    /// }
    ///
    /// let memory = exports.next().unwrap();
    /// assert_eq!(memory.name(), "memory");
    /// match memory.ty() {
    ///     ExternType::Memory(_) => { /* ... */ }
    ///     _ => panic!("unexpected export type!"),
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub fn exports<'module>(
        &'module self,
    ) -> impl ExactSizeIterator<Item = ExportType<'module>> + 'module {
        let module = self.compiled_module().module();
        let types = self.types();
        let engine = self.engine();
        module.exports.iter().map(move |(name, entity_index)| {
            ExportType::new(name, module.type_of(*entity_index), types, engine)
        })
    }

    /// Looks up an export in this [`Module`] by name.
    ///
    /// This function will return the type of an export with the given name.
    ///
    /// # Examples
    ///
    /// There may be no export with that name:
    ///
    /// ```
    /// # use wasmtime::*;
    /// # fn main() -> anyhow::Result<()> {
    /// # let engine = Engine::default();
    /// let module = Module::new(&engine, "(module)")?;
    /// assert!(module.get_export("foo").is_none());
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// When there is an export with that name, it is returned:
    ///
    /// ```
    /// # use wasmtime::*;
    /// # fn main() -> anyhow::Result<()> {
    /// # let engine = Engine::default();
    /// let wat = r#"
    ///     (module
    ///         (func (export "foo"))
    ///         (memory (export "memory") 1)
    ///     )
    /// "#;
    /// let module = Module::new(&engine, wat)?;
    /// let foo = module.get_export("foo");
    /// assert!(foo.is_some());
    ///
    /// let foo = foo.unwrap();
    /// match foo {
    ///     ExternType::Func(_) => { /* ... */ }
    ///     _ => panic!("unexpected export type!"),
    /// }
    ///
    /// # Ok(())
    /// # }
    /// ```
    pub fn get_export(&self, name: &str) -> Option<ExternType> {
        let module = self.compiled_module().module();
        let entity_index = module.exports.get(name)?;
        Some(ExternType::from_wasmtime(
            self.engine(),
            self.types(),
            &module.type_of(*entity_index),
        ))
    }

    /// Looks up an export in this [`Module`] by name to get its index.
    ///
    /// This function will return the index of an export with the given name. This can be useful
    /// to avoid the cost of looking up the export by name multiple times. Instead the
    /// [`ModuleExport`] can be stored and used to look up the export on the
    /// [`Instance`](crate::Instance) later.
    pub fn get_export_index(&self, name: &str) -> Option<ModuleExport> {
        let compiled_module = self.compiled_module();
        let module = compiled_module.module();
        let entity = *module.exports.get(name)?;
        Some(ModuleExport {
            module: self.id(),
            entity,
        })
    }

    /// Returns the [`Engine`] that this [`Module`] was compiled by.
    pub fn engine(&self) -> &Engine {
        &self.inner.engine
    }

    /// Returns a summary of the resources required to instantiate this
    /// [`Module`].
    ///
    /// Potential uses of the returned information:
    ///
    /// * Determining whether your pooling allocator configuration supports
    ///   instantiating this module.
    ///
    /// * Deciding how many of which `Module` you want to instantiate within a
    ///   fixed amount of resources, e.g. determining whether to create 5
    ///   instances of module X or 10 instances of module Y.
    ///
    /// # Example
    ///
    /// ```
    /// # fn main() -> wasmtime::Result<()> {
    /// use wasmtime::{Config, Engine, Module};
    ///
    /// let mut config = Config::new();
    /// config.wasm_multi_memory(true);
    /// let engine = Engine::new(&config)?;
    ///
    /// let module = Module::new(&engine, r#"
    ///     (module
    ///         ;; Import a memory. Doesn't count towards required resources.
    ///         (import "a" "b" (memory 10))
    ///         ;; Define two local memories. These count towards the required
    ///         ;; resources.
    ///         (memory 1)
    ///         (memory 6)
    ///     )
    /// "#)?;
    ///
    /// let resources = module.resources_required();
    ///
    /// // Instantiating the module will require allocating two memories, and
    /// // the maximum initial memory size is six Wasm pages.
    /// assert_eq!(resources.num_memories, 2);
    /// assert_eq!(resources.max_initial_memory_size, Some(6));
    ///
    /// // The module doesn't need any tables.
    /// assert_eq!(resources.num_tables, 0);
    /// assert_eq!(resources.max_initial_table_size, None);
    /// # Ok(()) }
    /// ```
    pub fn resources_required(&self) -> ResourcesRequired {
        let em = self.env_module();
        let num_memories = u32::try_from(em.num_defined_memories()).unwrap();
        let max_initial_memory_size = em
            .memories
            .values()
            .skip(em.num_imported_memories)
            .map(|memory| memory.limits.min)
            .max();
        let num_tables = u32::try_from(em.num_defined_tables()).unwrap();
        let max_initial_table_size = em
            .tables
            .values()
            .skip(em.num_imported_tables)
            .map(|table| table.limits.min)
            .max();
        ResourcesRequired {
            num_memories,
            max_initial_memory_size,
            num_tables,
            max_initial_table_size,
        }
    }

    /// Returns the range of bytes in memory where this module's compilation
    /// image resides.
    ///
    /// The compilation image for a module contains executable code, data, debug
    /// information, etc. This is roughly the same as the `Module::serialize`
    /// but not the exact same.
    ///
    /// The range of memory reported here is exposed to allow low-level
    /// manipulation of the memory in platform-specific manners such as using
    /// `mlock` to force the contents to be paged in immediately or keep them
    /// paged in after they're loaded.
    ///
    /// It is not safe to modify the memory in this range, nor is it safe to
    /// modify the protections of memory in this range.
    pub fn image_range(&self) -> Range<*const u8> {
        self.compiled_module().mmap().image_range()
    }

    /// Force initialization of copy-on-write images to happen here-and-now
    /// instead of when they're requested during first instantiation.
    ///
    /// When [copy-on-write memory
    /// initialization](crate::Config::memory_init_cow) is enabled then Wasmtime
    /// will lazily create the initialization image for a module. This method
    /// can be used to explicitly dictate when this initialization happens.
    ///
    /// Note that this largely only matters on Linux when memfd is used.
    /// Otherwise the copy-on-write image typically comes from disk and in that
    /// situation the creation of the image is trivial as the image is always
    /// sourced from disk. On Linux, though, when memfd is used a memfd is
    /// created and the initialization image is written to it.
    ///
    /// Also note that this method is not required to be called, it's available
    /// as a performance optimization if required but is otherwise handled
    /// automatically.
    pub fn initialize_copy_on_write_image(&self) -> Result<()> {
        self.memory_images()?;
        Ok(())
    }

    /// Get the map from `.text` section offsets to Wasm binary offsets for this
    /// module.
    ///
    /// Each entry is a (`.text` section offset, Wasm binary offset) pair.
    ///
    /// Entries are yielded in order of `.text` section offset.
    ///
    /// Some entries are missing a Wasm binary offset. This is for code that is
    /// not associated with any single location in the Wasm binary, or for when
    /// source information was optimized away.
    ///
    /// Not every module has an address map, since address map generation can be
    /// turned off on `Config`.
    ///
    /// There is not an entry for every `.text` section offset. Every offset
    /// after an entry's offset, but before the next entry's offset, is
    /// considered to map to the same Wasm binary offset as the original
    /// entry. For example, the address map will not contain the following
    /// sequence of entries:
    ///
    /// ```ignore
    /// [
    ///     // ...
    ///     (10, Some(42)),
    ///     (11, Some(42)),
    ///     (12, Some(42)),
    ///     (13, Some(43)),
    ///     // ...
    /// ]
    /// ```
    ///
    /// Instead, it will drop the entries for offsets `11` and `12` since they
    /// are the same as the entry for offset `10`:
    ///
    /// ```ignore
    /// [
    ///     // ...
    ///     (10, Some(42)),
    ///     (13, Some(43)),
    ///     // ...
    /// ]
    /// ```
    pub fn address_map<'a>(&'a self) -> Option<impl Iterator<Item = (usize, Option<u32>)> + 'a> {
        Some(
            wasmtime_environ::iterate_address_map(
                self.code_object().code_memory().address_map_data(),
            )?
            .map(|(offset, file_pos)| (offset as usize, file_pos.file_offset())),
        )
    }

    /// Get this module's code object's `.text` section, containing its compiled
    /// executable code.
    pub fn text(&self) -> &[u8] {
        self.code_object().code_memory().text()
    }

    /// Get information about functions in this module's `.text` section: their
    /// index, name, and offset+length.
    ///
    /// Results are yielded in a ModuleFunction struct.
    pub fn functions<'a>(&'a self) -> impl ExactSizeIterator<Item = ModuleFunction> + 'a {
        let module = self.compiled_module();
        module.finished_functions().map(|(idx, _)| {
            let loc = module.func_loc(idx);
            let idx = module.module().func_index(idx);
            ModuleFunction {
                index: idx,
                name: module.func_name(idx).map(|n| n.to_string()),
                offset: loc.start as usize,
                len: loc.length as usize,
            }
        })
    }

    pub(crate) fn id(&self) -> CompiledModuleId {
        self.inner.module.unique_id()
    }

    pub(crate) fn offsets(&self) -> &VMOffsets<HostPtr> {
        &self.inner.offsets
    }

    /// Return the address, in memory, of the trampoline that allows Wasm to
    /// call a array function of the given signature.
    pub(crate) fn wasm_to_array_trampoline(
        &self,
        signature: VMSharedTypeIndex,
    ) -> Option<NonNull<VMWasmCallFunction>> {
        log::trace!("Looking up trampoline for {signature:?}");
        let trampoline_shared_ty = self.inner.engine.signatures().trampoline_type(signature);
        let trampoline_module_ty = self
            .inner
            .code
            .signatures()
            .trampoline_type(trampoline_shared_ty)?;
        debug_assert!(
            self.inner
                .engine
                .signatures()
                .borrow(
                    self.inner
                        .code
                        .signatures()
                        .shared_type(trampoline_module_ty)
                        .unwrap()
                )
                .unwrap()
                .unwrap_func()
                .is_trampoline_type()
        );

        let ptr = self
            .compiled_module()
            .wasm_to_array_trampoline(trampoline_module_ty)
            .as_ptr()
            .cast::<VMWasmCallFunction>()
            .cast_mut();
        Some(NonNull::new(ptr).unwrap())
    }

    pub(crate) fn memory_images(&self) -> Result<Option<&ModuleMemoryImages>> {
        let images = self
            .inner
            .memory_images
            .get_or_try_init(|| memory_images(&self.inner.engine, &self.inner.module))?
            .as_ref();
        Ok(images)
    }

    /// Lookup the stack map at a program counter value.
    #[cfg(feature = "gc")]
    pub(crate) fn lookup_stack_map(&self, pc: usize) -> Option<wasmtime_environ::StackMap<'_>> {
        let text_offset = u32::try_from(pc - self.inner.module.text().as_ptr() as usize).unwrap();
        let info = self.inner.code.code_memory().stack_map_data();
        wasmtime_environ::StackMap::lookup(text_offset, info)
    }
}

/// Describes a function for a given module.
pub struct ModuleFunction {
    pub index: wasmtime_environ::FuncIndex,
    pub name: Option<String>,
    pub offset: usize,
    pub len: usize,
}

impl Drop for ModuleInner {
    fn drop(&mut self) {
        // When a `Module` is being dropped that means that it's no longer
        // present in any `Store` and it's additionally not longer held by any
        // embedder. Take this opportunity to purge any lingering instantiations
        // within a pooling instance allocator, if applicable.
        self.engine
            .allocator()
            .purge_module(self.module.unique_id());
    }
}

/// Describes the location of an export in a module.
#[derive(Copy, Clone)]
pub struct ModuleExport {
    /// The module that this export is defined in.
    pub(crate) module: CompiledModuleId,
    /// A raw index into the wasm module.
    pub(crate) entity: EntityIndex,
}

fn _assert_send_sync() {
    fn _assert<T: Send + Sync>() {}
    _assert::<Module>();
}

/// Helper method to construct a `ModuleMemoryImages` for an associated
/// `CompiledModule`.
fn memory_images(engine: &Engine, module: &CompiledModule) -> Result<Option<ModuleMemoryImages>> {
    // If initialization via copy-on-write is explicitly disabled in
    // configuration then this path is skipped entirely.
    if !engine.tunables().memory_init_cow {
        return Ok(None);
    }

    // ... otherwise logic is delegated to the `ModuleMemoryImages::new`
    // constructor.
    let mmap = if engine.config().force_memory_init_memfd {
        None
    } else {
        Some(module.mmap())
    };
    ModuleMemoryImages::new(module.module(), module.code_memory().wasm_data(), mmap)
}

#[cfg(test)]
mod tests {
    use crate::{Engine, Module};
    use wasmtime_environ::MemoryInitialization;

    #[test]
    fn cow_on_by_default() {
        let engine = Engine::default();
        let module = Module::new(
            &engine,
            r#"
                (module
                    (memory 1)
                    (data (i32.const 100) "abcd")
                )
            "#,
        )
        .unwrap();

        let init = &module.env_module().memory_initialization;
        assert!(matches!(init, MemoryInitialization::Static { .. }));
    }
}
