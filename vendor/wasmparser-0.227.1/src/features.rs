macro_rules! define_wasm_features {
    (
        $(#[$outer:meta])*
        pub struct WasmFeatures: $repr:ty {
            $(
                $(#[$inner:ident $($args:tt)*])*
                pub $field:ident: $const:ident($flag:expr) = $default:expr;
            )*
        }
    ) => {
        #[cfg(feature = "features")]
        bitflags::bitflags! {
            $(#[$outer])*
            pub struct WasmFeatures: $repr {
                $(
                    $(#[$inner $($args)*])*
                    #[doc = "\nDefaults to `"]
                    #[doc = stringify!($default)]
                    #[doc = "`.\n"]
                    const $const = $flag;
                )*
            }
        }

        /// Enabled WebAssembly proposals and features.
        ///
        /// This is the disabled zero-size version of this structure because the
        /// `features` feature was disabled at compile time of this crate.
        #[cfg(not(feature = "features"))]
        #[derive(Clone, Debug, Default, Hash, Copy)]
        pub struct WasmFeatures {
            _priv: (),
        }

        #[cfg(feature = "features")]
        impl Default for WasmFeatures {
            #[inline]
            fn default() -> Self {
                let mut features = WasmFeatures::empty();
                $(
                    features.set(WasmFeatures::$const, $default);
                )*
                features
            }
        }

        impl WasmFeatures {
            /// Construct a bit-packed `WasmFeatures` from the inflated struct version.
            #[inline]
            #[cfg(feature = "features")]
            pub fn from_inflated(inflated: WasmFeaturesInflated) -> Self {
                let mut features = WasmFeatures::empty();
                $(
                    features.set(WasmFeatures::$const, inflated.$field);
                )*
                features
            }

            /// Inflate these bit-packed features into a struct with a field per
            /// feature.
            ///
            /// Although the inflated struct takes up much more memory than the
            /// bit-packed version, its fields can be exhaustively matched
            /// upon. This makes it useful for temporarily checking against,
            /// while keeping the bit-packed version as the method of storing
            /// the features for longer periods of time.
            #[inline]
            #[cfg(feature = "features")]
            pub fn inflate(&self) -> WasmFeaturesInflated {
                WasmFeaturesInflated {
                    $(
                        $field: self.$field(),
                    )*
                }
            }

            $(
                /// Returns whether this feature is enabled in this feature set.
                #[inline]
                pub fn $field(&self) -> bool {
                    #[cfg(feature = "features")]
                    { self.contains(WasmFeatures::$const) }
                    #[cfg(not(feature = "features"))]
                    { $default }
                }
            )*
        }

        /// Inflated version of [`WasmFeatures`][crate::WasmFeatures] that
        /// allows for exhaustive matching on fields.
        #[cfg(feature = "features")]
        pub struct WasmFeaturesInflated {
            $(
                $(#[$inner $($args)*])*
                #[doc = "\nDefaults to `"]
                #[doc = stringify!($default)]
                #[doc = "`.\n"]
                pub $field: bool,
            )*
        }

        macro_rules! foreach_wasm_feature {
            ($f:ident) => {
                $($f!($field = $default);)*
            }
        }
        pub(crate) use foreach_wasm_feature;
    };
}

define_wasm_features! {
    /// Flags for features that are enabled for validation.
    ///
    /// This type controls the set of WebAssembly proposals and features that
    /// are active during validation and parsing of WebAssembly binaries. This
    /// is used in conjunction with
    /// [`Validator::new_with_features`](crate::Validator::new_with_features)
    /// for example.
    ///
    /// The [`Default`] implementation for this structure returns the set of
    /// supported WebAssembly proposals this crate implements. All features are
    /// required to be in Phase 4 or above in the WebAssembly standardization
    /// process.
    ///
    /// Enabled/disabled features can affect both parsing and validation at this
    /// time. When decoding a WebAssembly binary it's generally recommended if
    /// possible to enable all features, as is the default with
    /// [`BinaryReader::new`](crate::BinaryReader::new). If strict conformance
    /// with historical versions of the specification are required then
    /// [`BinaryReader::new_features`](crate::BinaryReader::new_features) or
    /// [`BinaryReader::set_features`](crate::BinaryReader::set_features) can be
    /// used.
    ///
    /// This crate additionally has a compile-time Cargo feature called
    /// `features` which can be used to enable/disable most of this type at
    /// compile time. This crate feature is turned on by default and enables
    /// this bitflags-representation of this structure. When the `features`
    /// feature is disabled then this is a zero-sized structure and no longer
    /// has any associated constants. When `features` are disabled all values
    /// for proposals are fixed at compile time to their defaults.
    #[derive(Hash, Debug, Copy, Clone, Eq, PartialEq)]
    pub struct WasmFeatures: u32 {
        /// The WebAssembly `mutable-global` proposal.
        pub mutable_global: MUTABLE_GLOBAL(1) = true;
        /// The WebAssembly `saturating-float-to-int` proposal.
        pub saturating_float_to_int: SATURATING_FLOAT_TO_INT(1 << 1) = true;
        /// The WebAssembly `sign-extension-ops` proposal.
        pub sign_extension: SIGN_EXTENSION(1 << 2) = true;
        /// The WebAssembly reference types proposal.
        pub reference_types: REFERENCE_TYPES(1 << 3) = true;
        /// The WebAssembly multi-value proposal.
        pub multi_value: MULTI_VALUE(1 << 4) = true;
        /// The WebAssembly bulk memory operations proposal.
        pub bulk_memory: BULK_MEMORY(1 << 5) = true;
        /// The WebAssembly SIMD proposal.
        pub simd: SIMD(1 << 6) = true;
        /// The WebAssembly Relaxed SIMD proposal.
        pub relaxed_simd: RELAXED_SIMD(1 << 7) = true;
        /// The WebAssembly threads proposal.
        pub threads: THREADS(1 << 8) = true;
        /// The WebAssembly shared-everything-threads proposal; includes new
        /// component model built-ins.
        pub shared_everything_threads: SHARED_EVERYTHING_THREADS(1 << 9) = false;
        /// The WebAssembly tail-call proposal.
        pub tail_call: TAIL_CALL(1 << 10) = true;
        /// Whether or not floating-point instructions are enabled.
        ///
        /// This is enabled by default can be used to disallow floating-point
        /// operators and types.
        ///
        /// This does not correspond to a WebAssembly proposal but is instead
        /// intended for embeddings which have stricter-than-usual requirements
        /// about execution. Floats in WebAssembly can have different NaN patterns
        /// across hosts which can lead to host-dependent execution which some
        /// runtimes may not desire.
        pub floats: FLOATS(1 << 11) = true;
        /// The WebAssembly multi memory proposal.
        pub multi_memory: MULTI_MEMORY(1 << 12) = true;
        /// The WebAssembly exception handling proposal.
        pub exceptions: EXCEPTIONS(1 << 13) = true;
        /// The WebAssembly memory64 proposal.
        pub memory64: MEMORY64(1 << 14) = true;
        /// The WebAssembly extended_const proposal.
        pub extended_const: EXTENDED_CONST(1 << 15) = true;
        /// The WebAssembly component model proposal.
        pub component_model: COMPONENT_MODEL(1 << 16) = true;
        /// The WebAssembly typed function references proposal.
        pub function_references: FUNCTION_REFERENCES(1 << 17) = true;
        /// The WebAssembly memory control proposal.
        pub memory_control: MEMORY_CONTROL(1 << 18) = false;
        /// The WebAssembly gc proposal.
        pub gc: GC(1 << 19) = true;
        /// The WebAssembly [custom-page-sizes
        /// proposal](https://github.com/WebAssembly/custom-page-sizes).
        pub custom_page_sizes: CUSTOM_PAGE_SIZES(1 << 20) = false;
        /// Support for the `value` type in the component model proposal.
        pub component_model_values: COMPONENT_MODEL_VALUES(1 << 21) = false;
        /// Support for the nested namespaces and projects in component model names.
        pub component_model_nested_names: COMPONENT_MODEL_NESTED_NAMES(1 << 22) = false;
        /// The WebAssembly legacy exception handling proposal (phase 1)
        ///
        /// # Note
        ///
        /// Support this feature as long as all leading browsers also support it
        /// <https://github.com/WebAssembly/exception-handling/blob/main/proposals/exception-handling/legacy/Exceptions.md>
        pub legacy_exceptions: LEGACY_EXCEPTIONS(1 << 25) = false;
        /// Whether or not gc types are enabled.
        ///
        /// This feature does not correspond to any WebAssembly proposal nor
        /// concept in the specification itself. This is intended to assist
        /// embedders in disabling support for GC types at validation time. For
        /// example if an engine wants to support all of WebAssembly except
        /// a runtime garbage collector it could disable this feature.
        ///
        /// This features is enabled by default and is used to gate types such
        /// as `externref` or `anyref`. Note that the requisite WebAssembly
        /// proposal must also be enabled for types like `externref`, meaning
        /// that it requires both `REFERENCE_TYPES` and `GC_TYPE` to be enabled.
        ///
        /// Note that the `funcref` and `exnref` types are not gated by this
        /// feature. Those are expected to not require a full garbage collector
        /// so are not gated by this.
        pub gc_types: GC_TYPES(1 << 26) = true;
        /// The WebAssembly [stack-switching proposal](https://github.com/WebAssembly/stack-switching).
        pub stack_switching: STACK_SWITCHING(1 << 27) = false;
        /// The WebAssembly [wide-arithmetic proposal](https://github.com/WebAssembly/wide-arithmetic).
        pub wide_arithmetic: WIDE_ARITHMETIC(1 << 28) = false;
        /// Support for component model async lift/lower ABI, as well as streams, futures, and errors.
        pub component_model_async: COMPONENT_MODEL_ASYNC(1 << 29) = false;
    }
}

impl WasmFeatures {
    /// The feature set associated with the MVP release of WebAssembly (its
    /// first release).
    //
    // Note that the features listed here are the wasmparser-specific built-in
    // features such as "floats" and "gc-types". These don't actually correspond
    // to any wasm proposals themselves and instead just gate constructs in
    // wasm. They're listed here so they otherwise don't have to be listed
    // below, but for example wasm with `externref` will be rejected due to lack
    // of `externref` first.
    #[cfg(feature = "features")]
    pub const MVP: WasmFeatures = WasmFeatures::FLOATS.union(WasmFeatures::GC_TYPES);

    /// The feature set associated with the 1.0 version of the
    /// WebAssembly specification circa 2017.
    ///
    /// <https://webassembly.github.io/spec/versions/core/WebAssembly-1.0.pdf>
    #[cfg(feature = "features")]
    pub const WASM1: WasmFeatures = WasmFeatures::MVP.union(WasmFeatures::MUTABLE_GLOBAL);

    /// The feature set associated with the 2.0 version of the
    /// WebAssembly specification circa 2022.
    ///
    /// <https://webassembly.github.io/spec/versions/core/WebAssembly-2.0.pdf>
    #[cfg(feature = "features")]
    pub const WASM2: WasmFeatures = WasmFeatures::WASM1
        .union(WasmFeatures::BULK_MEMORY)
        .union(WasmFeatures::REFERENCE_TYPES)
        .union(WasmFeatures::SIGN_EXTENSION)
        .union(WasmFeatures::SATURATING_FLOAT_TO_INT)
        .union(WasmFeatures::MULTI_VALUE)
        .union(WasmFeatures::SIMD);

    /// The feature set associated with the 3.0 version of the
    /// WebAssembly specification.
    ///
    /// Note that as of the time of this writing the 3.0 version of the
    /// specification is not yet published. The precise set of features set
    /// here may change as that continues to evolve.
    ///
    /// (draft)
    /// <https://webassembly.github.io/spec/versions/core/WebAssembly-3.0-draft.pdf>
    #[cfg(feature = "features")]
    pub const WASM3: WasmFeatures = WasmFeatures::WASM2
        .union(WasmFeatures::GC)
        .union(WasmFeatures::TAIL_CALL)
        .union(WasmFeatures::EXTENDED_CONST)
        .union(WasmFeatures::FUNCTION_REFERENCES)
        .union(WasmFeatures::MULTI_MEMORY)
        .union(WasmFeatures::RELAXED_SIMD)
        .union(WasmFeatures::THREADS)
        .union(WasmFeatures::EXCEPTIONS)
        .union(WasmFeatures::MEMORY64);
}

#[cfg(feature = "features")]
impl From<WasmFeaturesInflated> for WasmFeatures {
    #[inline]
    fn from(inflated: WasmFeaturesInflated) -> Self {
        Self::from_inflated(inflated)
    }
}

#[cfg(feature = "features")]
impl From<WasmFeatures> for WasmFeaturesInflated {
    #[inline]
    fn from(features: WasmFeatures) -> Self {
        features.inflate()
    }
}

impl WasmFeatures {
    /// Returns whether any types considered valid with this set of
    /// `WasmFeatures` requires any type canonicalization/interning internally.
    #[cfg(feature = "validate")]
    pub(crate) fn needs_type_canonicalization(&self) -> bool {
        #[cfg(feature = "features")]
        {
            // Types from the function-references proposal and beyond (namely
            // GC) need type canonicalization for inter-type references and for
            // rec-groups to work. Prior proposals have no such inter-type
            // references structurally and as such can hit various fast paths in
            // the validator to do a bit less work when processing the type
            // section.
            //
            // None of these proposals below support inter-type references. If
            // `self` contains any proposal outside of this set then it requires
            // type canonicalization.
            const FAST_VALIDATION_FEATURES: WasmFeatures = WasmFeatures::WASM2
                .union(WasmFeatures::CUSTOM_PAGE_SIZES)
                .union(WasmFeatures::EXTENDED_CONST)
                .union(WasmFeatures::MEMORY64)
                .union(WasmFeatures::MULTI_MEMORY)
                .union(WasmFeatures::RELAXED_SIMD)
                .union(WasmFeatures::TAIL_CALL)
                .union(WasmFeatures::THREADS)
                .union(WasmFeatures::WIDE_ARITHMETIC);
            !FAST_VALIDATION_FEATURES.contains(*self)
        }
        #[cfg(not(feature = "features"))]
        {
            // GC/function-references are enabled by default, so
            // canonicalization is required if feature flags aren't enabled at
            // runtime.
            true
        }
    }
}
