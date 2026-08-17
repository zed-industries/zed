use crate::cdsl::settings::{SettingGroup, SettingGroupBuilder};

pub(crate) fn define() -> SettingGroup {
    let mut settings = SettingGroupBuilder::new("shared");

    settings.add_bool(
        "regalloc_checker",
        "Enable the symbolic checker for register allocation.",
        r#"
            This performs a verification that the register allocator preserves
            equivalent dataflow with respect to the original (pre-regalloc)
            program. This analysis is somewhat expensive. However, if it succeeds,
            it provides independent evidence (by a carefully-reviewed, from-first-principles
            analysis) that no regalloc bugs were triggered for the particular compilations
            performed. This is a valuable assurance to have as regalloc bugs can be
            very dangerous and difficult to debug.
        "#,
        false,
    );

    settings.add_bool(
        "regalloc_verbose_logs",
        "Enable verbose debug logs for regalloc2.",
        r#"
            This adds extra logging for regalloc2 output, that is quite valuable to understand
            decisions taken by the register allocator as well as debugging it. It is disabled by
            default, as it can cause many log calls which can slow down compilation by a large
            amount.
        "#,
        false,
    );

    settings.add_enum(
        "regalloc_algorithm",
        "Algorithm to use in register allocator.",
        r#"
            Supported options:

            - `backtracking`: A backtracking allocator with range splitting; more expensive
                              but generates better code.

            Note that the `single_pass` option is currently disabled because it does not
            have adequate support for the kinds of allocations required by exception
            handling (https://github.com/bytecodealliance/regalloc2/issues/217).
        "#,
        vec!["backtracking"],
    );

    settings.add_enum(
        "opt_level",
        "Optimization level for generated code.",
        r#"
            Supported levels:

            - `none`: Minimise compile time by disabling most optimizations.
            - `speed`: Generate the fastest possible code
            - `speed_and_size`: like "speed", but also perform transformations aimed at reducing code size.
        "#,
        vec!["none", "speed", "speed_and_size"],
    );

    settings.add_bool(
        "enable_alias_analysis",
        "Do redundant-load optimizations with alias analysis.",
        r#"
            This enables the use of a simple alias analysis to optimize away redundant loads.
            Only effective when `opt_level` is `speed` or `speed_and_size`.
        "#,
        true,
    );

    settings.add_bool(
        "enable_verifier",
        "Run the Cranelift IR verifier at strategic times during compilation.",
        r#"
            This makes compilation slower but catches many bugs. The verifier is always enabled by
            default, which is useful during development.
        "#,
        true,
    );

    settings.add_bool(
        "enable_pcc",
        "Enable proof-carrying code translation validation.",
        r#"
            This adds a proof-carrying-code mode. Proof-carrying code (PCC) is a strategy to verify
            that the compiler preserves certain properties or invariants in the compiled code.
            For example, a frontend that translates WebAssembly to CLIF can embed PCC facts in
            the CLIF, and Cranelift will verify that the final machine code satisfies the stated
            facts at each intermediate computed value. Loads and stores can be marked as "checked"
            and their memory effects can be verified as safe.
        "#,
        false,
    );

    // Note that Cranelift doesn't currently need an is_pie flag, because PIE is
    // just PIC where symbols can't be pre-empted, which can be expressed with the
    // `colocated` flag on external functions and global values.
    settings.add_bool(
        "is_pic",
        "Enable Position-Independent Code generation.",
        "",
        false,
    );

    settings.add_bool(
        "use_colocated_libcalls",
        "Use colocated libcalls.",
        r#"
            Generate code that assumes that libcalls can be declared "colocated",
            meaning they will be defined along with the current function, such that
            they can use more efficient addressing.
        "#,
        false,
    );

    settings.add_bool(
        "enable_float",
        "Enable the use of floating-point instructions.",
        r#"
            Disabling use of floating-point instructions is not yet implemented.
        "#,
        true,
    );

    settings.add_bool(
        "enable_nan_canonicalization",
        "Enable NaN canonicalization.",
        r#"
            This replaces NaNs with a single canonical value, for users requiring
            entirely deterministic WebAssembly computation. This is not required
            by the WebAssembly spec, so it is not enabled by default.
        "#,
        false,
    );

    settings.add_bool(
        "enable_pinned_reg",
        "Enable the use of the pinned register.",
        r#"
            This register is excluded from register allocation, and is completely under the control of
            the end-user. It is possible to read it via the get_pinned_reg instruction, and to set it
            with the set_pinned_reg instruction.
        "#,
        false,
    );

    settings.add_bool(
        "enable_atomics",
        "Enable the use of atomic instructions",
        "",
        true,
    );

    settings.add_bool(
        "enable_safepoints",
        "Enable safepoint instruction insertions.",
        r#"
            This will allow the emit_stack_maps() function to insert the safepoint
            instruction on top of calls and interrupt traps in order to display the
            live reference values at that point in the program.
        "#,
        false,
    );

    settings.add_enum(
        "tls_model",
        "Defines the model used to perform TLS accesses.",
        "",
        vec!["none", "elf_gd", "macho", "coff"],
    );

    settings.add_enum(
        "stack_switch_model",
        "Defines the model used to performing stack switching.",
        r#"
           This determines the compilation of `stack_switch` instructions. If
           set to `basic`, we simply save all registers, update stack pointer
           and frame pointer (if needed), and jump to the target IP.
           If set to `update_windows_tib`, we *additionally* update information
           about the active stack in Windows' Thread Information Block.
        "#,
        vec!["none", "basic", "update_windows_tib"],
    );

    settings.add_enum(
        "libcall_call_conv",
        "Defines the calling convention to use for LibCalls call expansion.",
        r#"
            This may be different from the ISA default calling convention.

            The default value is to use the same calling convention as the ISA
            default calling convention.

            This list should be kept in sync with the list of calling
            conventions available in isa/call_conv.rs.
        "#,
        vec![
            "isa_default",
            "fast",
            "cold",
            "system_v",
            "windows_fastcall",
            "apple_aarch64",
            "probestack",
        ],
    );

    settings.add_bool(
        "enable_llvm_abi_extensions",
        "Enable various ABI extensions defined by LLVM's behavior.",
        r#"
            In some cases, LLVM's implementation of an ABI (calling convention)
            goes beyond a standard and supports additional argument types or
            behavior. This option instructs Cranelift codegen to follow LLVM's
            behavior where applicable.

            Currently, this applies only to Windows Fastcall on x86-64, and
            allows an `i128` argument to be spread across two 64-bit integer
            registers. The Fastcall implementation otherwise does not support
            `i128` arguments, and will panic if they are present and this
            option is not set.
        "#,
        false,
    );

    settings.add_bool(
        "enable_multi_ret_implicit_sret",
        "Enable support for sret arg introduction when there are too many ret vals.",
        r#"
            When there are more returns than available return registers, the
            return value has to be returned through the introduction of a
            return area pointer. Normally this return area pointer has to be
            introduced as `ArgumentPurpose::StructReturn` parameter, but for
            backward compatibility reasons Cranelift also supports implicitly
            introducing this parameter and writing the return values through it.

            **This option currently does not conform to platform ABIs and the
            used ABI should not be assumed to remain the same between Cranelift
            versions.**

            This option is **deprecated** and will be removed in the future.

            Because of the above issues, and complexities of native ABI support
            for the concept in general, Cranelift's support for multiple return
            values may also be removed in the future (#9510). For the most
            robust solution, it is recommended to build a convention on top of
            Cranelift's primitives for passing multiple return values, for
            example by allocating a stackslot in the caller, passing it as an
            explicit StructReturn argument, storing return values in the callee,
            and loading results in the caller.
        "#,
        false,
    );

    settings.add_bool(
        "unwind_info",
        "Generate unwind information.",
        r#"
            This increases metadata size and compile time, but allows for the
            debugger to trace frames, is needed for GC tracing that relies on
            libunwind (such as in Wasmtime), and is unconditionally needed on
            certain platforms (such as Windows) that must always be able to unwind.
          "#,
        true,
    );

    settings.add_bool(
        "preserve_frame_pointers",
        "Preserve frame pointers",
        r#"
            Preserving frame pointers -- even inside leaf functions -- makes it
            easy to capture the stack of a running program, without requiring any
            side tables or metadata (like `.eh_frame` sections). Many sampling
            profilers and similar tools walk frame pointers to capture stacks.
            Enabling this option will play nice with those tools.
        "#,
        false,
    );

    settings.add_bool(
        "machine_code_cfg_info",
        "Generate CFG metadata for machine code.",
        r#"
            This increases metadata size and compile time, but allows for the
            embedder to more easily post-process or analyze the generated
            machine code. It provides code offsets for the start of each
            basic block in the generated machine code, and a list of CFG
            edges (with blocks identified by start offsets) between them.
            This is useful for, e.g., machine-code analyses that verify certain
            properties of the generated code.
        "#,
        false,
    );

    // Stack probing options.

    settings.add_bool(
        "enable_probestack",
        "Enable the use of stack probes for supported calling conventions.",
        "",
        false,
    );

    settings.add_num(
        "probestack_size_log2",
        "The log2 of the size of the stack guard region.",
        r#"
            Stack frames larger than this size will have stack overflow checked
            by calling the probestack function.

            The default is 12, which translates to a size of 4096.
        "#,
        12,
    );

    settings.add_enum(
        "probestack_strategy",
        "Controls what kinds of stack probes are emitted.",
        r#"
            Supported strategies:

            - `outline`: Always emits stack probes as calls to a probe stack function.
            - `inline`: Always emits inline stack probes.
        "#,
        vec!["outline", "inline"],
    );

    // Jump table options.

    settings.add_bool(
        "enable_jump_tables",
        "Enable the use of jump tables in generated machine code.",
        "",
        true,
    );

    // Spectre options.

    settings.add_bool(
        "enable_heap_access_spectre_mitigation",
        "Enable Spectre mitigation on heap bounds checks.",
        r#"
            This is a no-op for any heap that needs no bounds checks; e.g.,
            if the limit is static and the guard region is large enough that
            the index cannot reach past it.

            This option is enabled by default because it is highly
            recommended for secure sandboxing. The embedder should consider
            the security implications carefully before disabling this option.
        "#,
        true,
    );

    settings.add_bool(
        "enable_table_access_spectre_mitigation",
        "Enable Spectre mitigation on table bounds checks.",
        r#"
            This option uses a conditional move to ensure that when a table
            access index is bounds-checked and a conditional branch is used
            for the out-of-bounds case, a misspeculation of that conditional
            branch (falsely predicted in-bounds) will select an in-bounds
            index to load on the speculative path.

            This option is enabled by default because it is highly
            recommended for secure sandboxing. The embedder should consider
            the security implications carefully before disabling this option.
        "#,
        true,
    );

    settings.add_bool(
        "enable_incremental_compilation_cache_checks",
        "Enable additional checks for debugging the incremental compilation cache.",
        r#"
            Enables additional checks that are useful during development of the incremental
            compilation cache. This should be mostly useful for Cranelift hackers, as well as for
            helping to debug false incremental cache positives for embedders.

            This option is disabled by default and requires enabling the "incremental-cache" Cargo
            feature in cranelift-codegen.
        "#,
        false,
    );

    settings.add_num(
        "bb_padding_log2_minus_one",
        "The log2 of the size to insert dummy padding between basic blocks",
        r#"
            This is a debugging option for stressing various cases during code
            generation without requiring large functions. This will insert
            0-byte padding between basic blocks of the specified size.

            The amount of padding inserted two raised to the power of this value
            minus one. If this value is 0 then no padding is inserted.

            The default for this option is 0 to insert no padding as it's only
            intended for testing and development.
        "#,
        0,
    );

    settings.add_num(
        "log2_min_function_alignment",
        "The log2 of the minimum alignment of functions",
        "The bigger of this value and the default alignment will be used as actual alignment.",
        0,
    );

    // When adding new settings please check if they can also be added
    // in cranelift/fuzzgen/src/lib.rs for fuzzing.
    settings.build()
}
