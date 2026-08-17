# Change Log

For changelogs after v0.14, see [the wgpu changelog](../CHANGELOG.md).

## v0.14 (2023-10-25)

#### GENERAL

- Add support for const-expressions. ([#2309](https://github.com/gfx-rs/naga/pull/2309)) **@teoxoy**, **@jimblandy**
- Add support for the `rgb10a2uint` storage format. ([#2525](https://github.com/gfx-rs/naga/pull/2525)) **@teoxoy**
- Implement module compaction for snapshot testing and the CLI. ([#2472](https://github.com/gfx-rs/naga/pull/2472)) **@jimblandy**
- Fix validation and GLSL parsing of `ldexp`. ([#2449](https://github.com/gfx-rs/naga/pull/2449)) **@fornwall**
- Add support for dual source blending. ([#2427](https://github.com/gfx-rs/naga/pull/2427)) **@freqmod**
- Bump `indexmap` to v2. ([#2426](https://github.com/gfx-rs/naga/pull/2426)) **@daxpedda**
- Bump MSRV to 1.65. ([#2420](https://github.com/gfx-rs/naga/pull/2420)) **@jimblandy**

#### API

- Split `UnaryOperator::Not` into `UnaryOperator::LogicalNot` & `UnaryOperator::BitwiseNot`. ([#2554](https://github.com/gfx-rs/naga/pull/2554)) **@teoxoy**
- Remove `IsFinite` & `IsNormal` relational functions. ([#2532](https://github.com/gfx-rs/naga/pull/2532)) **@teoxoy**
- Derive `PartialEq` on `Expression`. ([#2417](https://github.com/gfx-rs/naga/pull/2417)) **@robtfm**
- Use `FastIndexMap` for `SpecialTypes::predeclared_types`. ([#2495](https://github.com/gfx-rs/naga/pull/2495)) **@jimblandy**

#### CLI

- Change `--generate-debug-symbols` from an `option` to a `switch`. ([#2472](https://github.com/gfx-rs/naga/pull/2472)) **@jimblandy**
- Add support for `.{vert,frag,comp}.glsl` files. ([#2462](https://github.com/gfx-rs/naga/pull/2462)) **@eliemichel**

#### VALIDATOR

- Require `Capabilities::FLOAT64` for 64-bit floating-point literals. ([#2567](https://github.com/gfx-rs/naga/pull/2567)) **@jimblandy**
- Add `Capabilities::CUBE_ARRAY_TEXTURES`. ([#2530](https://github.com/gfx-rs/naga/pull/2530)) **@teoxoy**
- Disallow passing pointers to variables in the workgroup address space to functions. ([#2507](https://github.com/gfx-rs/naga/pull/2507)) **@teoxoy**
- Avoid OOM with large sparse resource bindings. ([#2561](https://github.com/gfx-rs/naga/pull/2561)) **@teoxoy**
- Require that `Function` and `Private` variables be `CONSTRUCTIBLE`. ([#2545](https://github.com/gfx-rs/naga/pull/2545)) **@jimblandy**
- Disallow floating-point NaNs and infinities. ([#2508](https://github.com/gfx-rs/naga/pull/2508)) **@teoxoy**
- Temporarily disable uniformity analysis for the fragment stage. ([#2515](https://github.com/gfx-rs/naga/pull/2515)) **@teoxoy**
- Validate that `textureSampleBias` is only used in the fragment stage. ([#2515](https://github.com/gfx-rs/naga/pull/2515)) **@teoxoy**
- Validate variable initializer for address spaces. ([#2513](https://github.com/gfx-rs/naga/pull/2513)) **@teoxoy**
- Prevent using multiple push constant variables in one entry point. ([#2484](https://github.com/gfx-rs/naga/pull/2484)) **@andriyDev**
- Validate `binding_array` variable address space. ([#2422](https://github.com/gfx-rs/naga/pull/2422)) **@teoxoy**
- Validate storage buffer access. ([#2415](https://github.com/gfx-rs/naga/pull/2415)) **@teoxoy**

#### WGSL-IN

- Fix expected min arg count of `textureLoad`. ([#2584](https://github.com/gfx-rs/naga/pull/2584)) **@teoxoy**
- Turn `Error::Other` into `Error::Internal`, to help devs. ([#2574](https://github.com/gfx-rs/naga/pull/2574)) **@jimblandy**
- Fix OOB typifier indexing. ([#2570](https://github.com/gfx-rs/naga/pull/2570)) **@teoxoy**
- Add support for the `bgra8unorm` storage format. ([#2542](https://github.com/gfx-rs/naga/pull/2542) & [#2550](https://github.com/gfx-rs/naga/pull/2550)) **@nical**
- Remove the `outerProduct` built-in function. ([#2535](https://github.com/gfx-rs/naga/pull/2535)) **@teoxoy**
- Add support for `i32` overload of the `sign` built-in function. ([#2463](https://github.com/gfx-rs/naga/pull/2463)) **@fornwall**
- Properly implement `modf` and `frexp`. ([#2454](https://github.com/gfx-rs/naga/pull/2454)) **@fornwall**
- Add support for scalar overloads of `all` & `any` built-in functions. ([#2445](https://github.com/gfx-rs/naga/pull/2445)) **@fornwall**
- Don't splat the left hand operand of a binary operation if it's not a scalar. ([#2444](https://github.com/gfx-rs/naga/pull/2444)) **@fornwall**
- Avoid splatting all binary operator expressions. ([#2440](https://github.com/gfx-rs/naga/pull/2440)) **@fornwall**
- Error on repeated or missing `@workgroup_size()`. ([#2435](https://github.com/gfx-rs/naga/pull/2435)) **@fornwall**
- Error on repeated attributes. ([#2428](https://github.com/gfx-rs/naga/pull/2428)) **@fornwall**
- Fix error message for invalid `texture{Load,Store}()` on arrayed textures. ([#2432](https://github.com/gfx-rs/naga/pull/2432)) **@fornwall**

#### SPV-IN

- Disable `Modf` & `Frexp` and translate `ModfStruct` & `FrexpStruct` to their IR equivalents. ([#2527](https://github.com/gfx-rs/naga/pull/2527)) **@teoxoy**
- Don't advertise support for `Capability::ImageMSArray` & `Capability::InterpolationFunction`. ([#2529](https://github.com/gfx-rs/naga/pull/2529)) **@teoxoy**
- Fix `OpImageQueries` to allow Uints. ([#2404](https://github.com/gfx-rs/naga/pull/2404)) **@evahop**

#### GLSL-IN

- Disable `modf` & `frexp`. ([#2527](https://github.com/gfx-rs/naga/pull/2527)) **@teoxoy**

#### SPV-OUT

- Require `ClipDistance` & `CullDistance` capabilities if necessary. ([#2528](https://github.com/gfx-rs/naga/pull/2528)) **@teoxoy**
- Change `naga::back::spv::DebugInfo::file_name` to a `&Path`. ([#2501](https://github.com/gfx-rs/naga/pull/2501)) **@jimblandy**
- Always give structs with runtime arrays a `Block` decoration. ([#2455](https://github.com/gfx-rs/naga/pull/2455)) **@TheoDulka**
- Decorate the result of the `OpLoad` with `NonUniform` (not the access chain) when loading images/samplers (resources in the Handle address space). ([#2422](https://github.com/gfx-rs/naga/pull/2422)) **@teoxoy**
- Cache `OpConstantNull`. ([#2414](https://github.com/gfx-rs/naga/pull/2414)) **@evahop**

#### MSL-OUT

- Add and fix minimum Metal version checks for optional functionality. ([#2486](https://github.com/gfx-rs/naga/pull/2486)) **@teoxoy**
- Make varyings' struct members unique. ([#2521](https://github.com/gfx-rs/naga/pull/2521)) **@evahop**
- Add experimental vertex pulling transform flag. ([#5254](https://github.com/gfx-rs/wgpu/pull/5254)) **@bradwerth**
- Fixup some generated MSL for vertex buffer unpack functions. ([#5829](https://github.com/gfx-rs/wgpu/pull/5829)) **@bradwerth**
- Make vertex pulling transform on by default. ([#5773](https://github.com/gfx-rs/wgpu/pull/5773)) **@bradwerth**

#### GLSL-OUT

- Cull functions that should not be available for a given stage. ([#2531](https://github.com/gfx-rs/naga/pull/2531)) **@teoxoy**
- Rename identifiers containing double underscores. ([#2510](https://github.com/gfx-rs/naga/pull/2510)) **@evahop**
- Polyfill `frexp`. ([#2504](https://github.com/gfx-rs/naga/pull/2504)) **@evahop**
- Add built-in functions to keywords. ([#2410](https://github.com/gfx-rs/naga/pull/2410)) **@fornwall**

#### WGSL-OUT

- Generate correct code for bit complement on integers. ([#2548](https://github.com/gfx-rs/naga/pull/2548)) **@jimblandy**
- Don't include type parameter in splat expressions. ([#2469](https://github.com/gfx-rs/naga/pull/2469)) **@jimblandy**

## v0.13 (2023-07-21)

#### GENERAL

- Move from `make` to `cargo xtask` workflows. ([#2297](https://github.com/gfx-rs/naga/pull/2297)) **@ErichDonGubler**
- Omit non referenced expressions from output. ([#2378](https://github.com/gfx-rs/naga/pull/2378)) **@teoxoy**
- Bump `bitflags` to v2. ([#2358](https://github.com/gfx-rs/naga/pull/2358)) **@daxpedda**
- Implement `workgroupUniformLoad`. ([#2201](https://github.com/gfx-rs/naga/pull/2201)) **@DJMcNab**

#### API

- Expose early depth test field. ([#2393](https://github.com/gfx-rs/naga/pull/2393)) **@Joeoc2001**
- Split image bounds check policy. ([#2265](https://github.com/gfx-rs/naga/pull/2265)) **@teoxoy**
- Change type of constant sized arrays to `NonZeroU32`. ([#2337](https://github.com/gfx-rs/naga/pull/2337)) **@teoxoy**
- Introduce `GlobalCtx`. ([#2335](https://github.com/gfx-rs/naga/pull/2335)) **@teoxoy**
- Introduce `Expression::Literal`. ([#2333](https://github.com/gfx-rs/naga/pull/2333)) **@teoxoy**
- Introduce `Expression::ZeroValue`. ([#2332](https://github.com/gfx-rs/naga/pull/2332)) **@teoxoy**
- Add support for const-expressions (only at the API level, functionality is still WIP). ([#2266](https://github.com/gfx-rs/naga/pull/2266)) **@teoxoy**, **@jimblandy**

#### DOCS

- Document which expressions are in scope for a `break_if` expression. ([#2326](https://github.com/gfx-rs/naga/pull/2326)) **@jimblandy**

#### VALIDATOR

- Don't `use std::opsIndex`, used only when `"validate"` is on. ([#2383](https://github.com/gfx-rs/naga/pull/2383)) **@jimblandy**
- Remove unneeded `ConstantError::Unresolved{Component,Size}`. ([#2330](https://github.com/gfx-rs/naga/pull/2330)) **@ErichDonGubler**
- Remove `TypeError::UnresolvedBase`. ([#2308](https://github.com/gfx-rs/naga/pull/2308)) **@ErichDonGubler**

#### WGSL-IN

- Error on param redefinition. ([#2342](https://github.com/gfx-rs/naga/pull/2342)) **@SparkyPotato**

#### SPV-IN

- Improve documentation for SPIR-V control flow parsing. ([#2324](https://github.com/gfx-rs/naga/pull/2324)) **@jimblandy**
- Obey the `is_depth` field of `OpTypeImage`. ([#2341](https://github.com/gfx-rs/naga/pull/2341)) **@expenses**
- Convert conditional backedges to `break if`. ([#2290](https://github.com/gfx-rs/naga/pull/2290)) **@eddyb**

#### GLSL-IN

- Support commas in structure definitions. ([#2400](https://github.com/gfx-rs/naga/pull/2400)) **@fornwall**

#### SPV-OUT

- Add debug info. ([#2379](https://github.com/gfx-rs/naga/pull/2379)) **@wicast**
- Use `IndexSet` instead of `HashSet` for iterated sets (capabilities/extensions). ([#2389](https://github.com/gfx-rs/naga/pull/2389)) **@eddyb**
- Support array bindings of buffers. ([#2282](https://github.com/gfx-rs/naga/pull/2282)) **@kvark**

#### MSL-OUT

- Rename `allow_point_size` to `allow_and_force_point_size`. ([#2280](https://github.com/gfx-rs/naga/pull/2280)) **@teoxoy**
- Initialize arrays inline. ([#2331](https://github.com/gfx-rs/naga/pull/2331)) **@teoxoy**

#### HLSL-OUT

- Implement Pack/Unpack for HLSL. ([#2353](https://github.com/gfx-rs/naga/pull/2353)) **@Elabajaba**
- Complete HLSL reserved symbols. ([#2367](https://github.com/gfx-rs/naga/pull/2367)) **@teoxoy**
- Handle case insensitive FXC keywords. ([#2347](https://github.com/gfx-rs/naga/pull/2347)) **@PJB3005**
- Fix return type for firstbitlow/high. ([#2315](https://github.com/gfx-rs/naga/pull/2315)) **@evahop**

#### GLSL-OUT

- `textureSize` level must be a signed integer. ([#2397](https://github.com/gfx-rs/naga/pull/2397)) **@nical**
- Fix functions with array return type. ([#2382](https://github.com/gfx-rs/naga/pull/2382)) **@Gordon-F**

#### WGSL-OUT

- Output `@interpolate(flat)` attribute for integer locations. ([#2318](https://github.com/gfx-rs/naga/pull/2318)) **@expenses**

## v0.12.3 (2023-07-09)

#### WGSL-OUT

- (Backport) Output `@interpolate(flat)` attribute for integer locations. ([#2318](https://github.com/gfx-rs/naga/pull/2318)) **@expenses**

## v0.12.2 (2023-05-30)

#### SPV-OUT

- (Backport) Support array bindings of buffers. ([#2282](https://github.com/gfx-rs/naga/pull/2282)) **@kvark**

## v0.12.1 (2023-05-18)

#### SPV-IN

- (Backport) Convert conditional backedges to `break if`. ([#2290](https://github.com/gfx-rs/naga/pull/2290)) **@eddyb**

## v0.12 (2023-04-19)

#### GENERAL

- Allow `array_index` to be unsigned. ([#2298](https://github.com/gfx-rs/naga/pull/2298)) **@daxpedda**
- Add ray query support. ([#2256](https://github.com/gfx-rs/naga/pull/2256)) **@kvark**
- Add partial derivative builtins. ([#2277](https://github.com/gfx-rs/naga/pull/2277)) **@evahop**
- Skip `gl_PerVertex` unused builtins in the SPIR-V frontend. ([#2272](https://github.com/gfx-rs/naga/pull/2272)) **@teoxoy**
- Differentiate between `i32` and `u32` in switch statement cases. ([#2269](https://github.com/gfx-rs/naga/pull/2269)) **@evahop**
- Fix zero initialization of workgroup memory. ([#2259](https://github.com/gfx-rs/naga/pull/2259)) **@teoxoy**
- Add `countTrailingZeros`. ([#2243](https://github.com/gfx-rs/naga/pull/2243)) **@gents83**
- Fix texture built-ins where u32 was expected. ([#2245](https://github.com/gfx-rs/naga/pull/2245)) **@evahop**
- Add `countLeadingZeros`. ([#2226](https://github.com/gfx-rs/naga/pull/2226)) **@evahop**
- [glsl/hlsl-out] Write sizes of arrays behind pointers in function arguments. ([#2250](https://github.com/gfx-rs/naga/pull/2250)) **@pluiedev**

#### VALIDATOR

- Validate vertex stage returns the position built-in. ([#2264](https://github.com/gfx-rs/naga/pull/2264)) **@teoxoy**
- Enforce discard is only used in the fragment stage. ([#2262](https://github.com/gfx-rs/naga/pull/2262)) **@Uriopass**
- Add `Capabilities::MULTISAMPLED_SHADING`. ([#2255](https://github.com/gfx-rs/naga/pull/2255)) **@teoxoy**
- Add `Capabilities::EARLY_DEPTH_TEST`. ([#2255](https://github.com/gfx-rs/naga/pull/2255)) **@teoxoy**
- Add `Capabilities::MULTIVIEW`. ([#2255](https://github.com/gfx-rs/naga/pull/2255)) **@teoxoy**
- Improve forward declaration validation. ([#2232](https://github.com/gfx-rs/naga/pull/2232)) **@JCapucho**

#### WGSL-IN

- Use `alias` instead of `type` for type aliases. ([#2299](https://github.com/gfx-rs/naga/pull/2299)) **@FL33TW00D**
- Add predeclared vector and matrix type aliases. ([#2251](https://github.com/gfx-rs/naga/pull/2251)) **@evahop**
- Improve invalid assignment diagnostic. ([#2233](https://github.com/gfx-rs/naga/pull/2233)) **@SparkyPotato**
- Expect semicolons wherever required. ([#2233](https://github.com/gfx-rs/naga/pull/2233)) **@SparkyPotato**
- Fix panic on invalid zero array size. ([#2233](https://github.com/gfx-rs/naga/pull/2233)) **@SparkyPotato**
- Check for leading `{` while parsing a block. ([#2233](https://github.com/gfx-rs/naga/pull/2233)) **@SparkyPotato**

#### SPV-IN

- Don't apply interpolation to fragment shaders outputs. ([#2239](https://github.com/gfx-rs/naga/pull/2239)) **@JCapucho**

#### GLSL-IN

- Add switch implicit type conversion. ([#2273](https://github.com/gfx-rs/naga/pull/2273)) **@evahop**
- Document some fields of `naga::front::glsl::context::Context`. ([#2244](https://github.com/gfx-rs/naga/pull/2244)) **@jimblandy**
- Perform output parameters implicit casts. ([#2063](https://github.com/gfx-rs/naga/pull/2063)) **@JCapucho**
- Add `not` vector relational builtin. ([#2227](https://github.com/gfx-rs/naga/pull/2227)) **@JCapucho**
- Add double overloads for relational vector builtins. ([#2227](https://github.com/gfx-rs/naga/pull/2227)) **@JCapucho**
- Add bool overloads for relational vector builtins. ([#2227](https://github.com/gfx-rs/naga/pull/2227)) **@JCapucho**

#### SPV-OUT

- Fix invalid spirv being generated from integer dot products. ([#2291](https://github.com/gfx-rs/naga/pull/2291)) **@PyryM**
- Fix adding illegal decorators on fragment outputs. ([#2286](https://github.com/gfx-rs/naga/pull/2286)) **@Wumpf**
- Fix `countLeadingZeros` impl. ([#2258](https://github.com/gfx-rs/naga/pull/2258)) **@teoxoy**
- Cache constant composites. ([#2257](https://github.com/gfx-rs/naga/pull/2257)) **@evahop**
- Support SPIR-V version 1.4. ([#2230](https://github.com/gfx-rs/naga/pull/2230)) **@kvark**

#### MSL-OUT

- Replace `per_stage_map` with `per_entry_point_map` ([#2237](https://github.com/gfx-rs/naga/pull/2237)) **@armansito**
- Update `firstLeadingBit` for signed integers ([#2235](https://github.com/gfx-rs/naga/pull/2235)) **@evahop**

#### HLSL-OUT

- Use `Interlocked<op>` intrinsic for atomic integers (#2294) ([#2294](https://github.com/gfx-rs/naga/pull/2294)) **@ErichDonGubler**
- Document storage access generation. ([#2295](https://github.com/gfx-rs/naga/pull/2295)) **@jimblandy**
- Emit constructor functions for arrays. ([#2281](https://github.com/gfx-rs/naga/pull/2281)) **@ErichDonGubler**
- Clear `named_expressions` inserted by duplicated blocks. ([#2116](https://github.com/gfx-rs/naga/pull/2116)) **@teoxoy**

#### GLSL-OUT

- Skip `invariant` for `gl_FragCoord` on WebGL2. ([#2254](https://github.com/gfx-rs/naga/pull/2254)) **@grovesNL**
- Inject default `gl_PointSize = 1.0` in vertex shaders if `FORCE_POINT_SIZE` option was set. ([#2223](https://github.com/gfx-rs/naga/pull/2223)) **@REASY**

## v0.11.1 (2023-05-18)

#### SPV-IN

- (Backport) Convert conditional backedges to `break if`. ([#2290](https://github.com/gfx-rs/naga/pull/2290)) **@eddyb**

## v0.11 (2023-01-25)

- Move to the Rust 2021 edition ([#2085](https://github.com/gfx-rs/naga/pull/2085)) **@ErichDonGubler**
- Bump MSRV to 1.63 ([#2129](https://github.com/gfx-rs/naga/pull/2129)) **@teoxoy**

#### API

- Add handle validation pass to `Validator` ([#2090](https://github.com/gfx-rs/naga/pull/2090)) **@ErichDonGubler**
- Add `Range::new_from_bounds` ([#2148](https://github.com/gfx-rs/naga/pull/2148)) **@robtfm**

#### DOCS

- Fix docs for `Emit` statements ([#2208](https://github.com/gfx-rs/naga/pull/2208)) **@jimblandy**
- Fix invalid `<...>` URLs with code spans ([#2176](https://github.com/gfx-rs/naga/pull/2176)) **@ErichDonGubler**
- Explain how case clauses with multiple selectors are supported ([#2126](https://github.com/gfx-rs/naga/pull/2126)) **@teoxoy**
- Document `EarlyDepthTest` and `ConservativeDepth` syntax ([#2132](https://github.com/gfx-rs/naga/pull/2132)) **@coreh**

#### VALIDATOR

- Allow `u32` coordinates for `textureStore`/`textureLoad` ([#2172](https://github.com/gfx-rs/naga/pull/2172)) **@PENGUINLIONG**
- Fix array being flagged as constructible when its base isn't ([#2111](https://github.com/gfx-rs/naga/pull/2111)) **@teoxoy**
- Add `type_flags` to `ModuleInfo` ([#2111](https://github.com/gfx-rs/naga/pull/2111)) **@teoxoy**
- Remove overly restrictive array stride check ([#2215](https://github.com/gfx-rs/naga/pull/2215)) **@fintelia**
- Let the uniformity analysis trust the handle validation pass ([#2200](https://github.com/gfx-rs/naga/pull/2200)) **@jimblandy**
- Fix warnings when building tests without validation ([#2177](https://github.com/gfx-rs/naga/pull/2177)) **@jimblandy**
- Add `ValidationFlags::BINDINGS` ([#2156](https://github.com/gfx-rs/naga/pull/2156)) **@kvark**
- Fix `textureGather` on `texture_2d<u32/i32>` ([#2138](https://github.com/gfx-rs/naga/pull/2138)) **@JMS55**

#### ALL (FRONTENDS/BACKENDS)

- Support 16-bit unorm/snorm formats ([#2210](https://github.com/gfx-rs/naga/pull/2210)) **@fintelia**
- Support `gl_PointCoord` ([#2180](https://github.com/gfx-rs/naga/pull/2180)) **@Neo-Zhixing**

#### ALL BACKENDS

- Add support for zero-initializing workgroup memory ([#2111](https://github.com/gfx-rs/naga/pull/2111)) **@teoxoy**

#### WGSL-IN

- Implement module-level scoping ([#2075](https://github.com/gfx-rs/naga/pull/2075)) **@SparkyPotato**
- Remove `isFinite` and `isNormal` ([#2218](https://github.com/gfx-rs/naga/pull/2218)) **@evahop**
- Update inverse hyperbolic built-ins ([#2218](https://github.com/gfx-rs/naga/pull/2218)) **@evahop**
- Add `refract` built-in ([#2218](https://github.com/gfx-rs/naga/pull/2218)) **@evahop**
- Update reserved keywords ([#2130](https://github.com/gfx-rs/naga/pull/2130)) **@teoxoy**
- Remove non-32bit integers ([#2146](https://github.com/gfx-rs/naga/pull/2146)) **@teoxoy**
- Remove `workgroup_size` builtin ([#2147](https://github.com/gfx-rs/naga/pull/2147)) **@teoxoy**
- Remove fallthrough statement ([#2126](https://github.com/gfx-rs/naga/pull/2126)) **@teoxoy**

#### SPV-IN

- Support binding arrays ([#2199](https://github.com/gfx-rs/naga/pull/2199)) **@Patryk27**

#### GLSL-IN

- Fix position propagation in lowering ([#2079](https://github.com/gfx-rs/naga/pull/2079)) **@JCapucho**
- Update initializer list type when parsing ([#2066](https://github.com/gfx-rs/naga/pull/2066)) **@JCapucho**
- Parenthesize unary negations to avoid `--` ([#2087](https://github.com/gfx-rs/naga/pull/2087)) **@ErichDonGubler**

#### SPV-OUT

- Add support for `atomicCompareExchangeWeak` ([#2165](https://github.com/gfx-rs/naga/pull/2165)) **@aweinstock314**
- Omit extra switch case blocks where possible ([#2126](https://github.com/gfx-rs/naga/pull/2126)) **@teoxoy**
- Fix switch cases after default not being output ([#2126](https://github.com/gfx-rs/naga/pull/2126)) **@teoxoy**

#### MSL-OUT

- Don't panic on missing bindings ([#2175](https://github.com/gfx-rs/naga/pull/2175)) **@kvark**
- Omit extra switch case blocks where possible ([#2126](https://github.com/gfx-rs/naga/pull/2126)) **@teoxoy**
- Fix `textureGather` compatibility on macOS 10.13 ([#2104](https://github.com/gfx-rs/naga/pull/2104)) **@xiaopengli89**
- Fix incorrect atomic bounds check on metal back-end ([#2099](https://github.com/gfx-rs/naga/pull/2099)) **@raphlinus**
- Parenthesize unary negations to avoid `--` ([#2087](https://github.com/gfx-rs/naga/pull/2087)) **@ErichDonGubler**

#### HLSL-OUT

- Simplify `write_default_init` ([#2111](https://github.com/gfx-rs/naga/pull/2111)) **@teoxoy**
- Omit extra switch case blocks where possible ([#2126](https://github.com/gfx-rs/naga/pull/2126)) **@teoxoy**
- Properly implement bitcast ([#2097](https://github.com/gfx-rs/naga/pull/2097)) **@cwfitzgerald**
- Fix storage access chain through a matrix ([#2097](https://github.com/gfx-rs/naga/pull/2097)) **@cwfitzgerald**
- Workaround FXC Bug in Matrix Indexing ([#2096](https://github.com/gfx-rs/naga/pull/2096)) **@cwfitzgerald**
- Parenthesize unary negations to avoid `--` ([#2087](https://github.com/gfx-rs/naga/pull/2087)) **@ErichDonGubler**

#### GLSL-OUT

- Introduce a flag to include unused items ([#2205](https://github.com/gfx-rs/naga/pull/2205)) **@robtfm**
- Use `fma` polyfill for versions below gles 320 ([#2197](https://github.com/gfx-rs/naga/pull/2197)) **@teoxoy**
- Emit reflection info for non-struct uniforms ([#2189](https://github.com/gfx-rs/naga/pull/2189)) **@Rainb0wCodes**
- Introduce a new block for switch cases ([#2126](https://github.com/gfx-rs/naga/pull/2126)) **@teoxoy**

#### WGSL-OUT

- Write correct scalar kind when `width != 4` ([#1514](https://github.com/gfx-rs/naga/pull/1514)) **@fintelia**

## v0.10.1 (2023-06-21)

SPV-OUT
- Backport #2389 (Use `IndexSet` instead of `HashSet` for iterated sets (capabilities/extensions)) by @eddyb, @jimblandy in https://github.com/gfx-rs/naga/pull/2391

SPV-IN
- Backport #2290 (Convert conditional backedges to `break if`) by @eddyb in https://github.com/gfx-rs/naga/pull/2387

## v0.10 (2022-10-05)

- Make termcolor dependency optional by @AldaronLau in https://github.com/gfx-rs/naga/pull/2014
- Fix clippy lints for 1.63 by @JCapucho in https://github.com/gfx-rs/naga/pull/2026
- Saturate by @evahop in https://github.com/gfx-rs/naga/pull/2025
- Use `Option::as_deref` as appropriate. by @jimblandy in https://github.com/gfx-rs/naga/pull/2040
- Explicitly enable std for indexmap by @maxammann in https://github.com/gfx-rs/naga/pull/2062
- Fix compiler warning by @Gordon-F in https://github.com/gfx-rs/naga/pull/2074

API
- Implement `Clone` for `Module` by @daxpedda in https://github.com/gfx-rs/naga/pull/2013
- Remove the glsl-validate feature by @JCapucho in https://github.com/gfx-rs/naga/pull/2045

DOCS
- Document arithmetic binary operation type rules. by @jimblandy in https://github.com/gfx-rs/naga/pull/2051

VALIDATOR
- Add `emit_to_{stderr,string}` helpers to validation error by @nolanderc in https://github.com/gfx-rs/naga/pull/2012
- Check regular functions don't have bindings by @JCapucho in https://github.com/gfx-rs/naga/pull/2050

WGSL-IN
- Update reserved WGSL keywords by @norepimorphism in https://github.com/gfx-rs/naga/pull/2009
- Implement lexical scopes by @JCapucho in https://github.com/gfx-rs/naga/pull/2024
- Rename `Scope` to `Rule`, since we now have lexical scope. by @jimblandy in https://github.com/gfx-rs/naga/pull/2042
- Splat on compound assignments by @JCapucho in https://github.com/gfx-rs/naga/pull/2049
- Fix bad span in assignment lhs error by @JCapucho in https://github.com/gfx-rs/naga/pull/2054
- Fix inclusion of trivia in spans by @SparkyPotato in https://github.com/gfx-rs/naga/pull/2055
- Improve assignment diagnostics by @SparkyPotato in https://github.com/gfx-rs/naga/pull/2056
- Break up long string, reformat rest of file. by @jimblandy in https://github.com/gfx-rs/naga/pull/2057
- Fix line endings on wgsl reserved words list. by @jimblandy in https://github.com/gfx-rs/naga/pull/2059

GLSL-IN
- Add support for .length() by @SpaceCat-Chan in https://github.com/gfx-rs/naga/pull/2017
- Fix missing stores for local declarations by @adeline-sparks in https://github.com/gfx-rs/naga/pull/2029
- Migrate to `SymbolTable` by @JCapucho in https://github.com/gfx-rs/naga/pull/2044
- Update initializer list type when parsing by @JCapucho in https://github.com/gfx-rs/naga/pull/2066

SPV-OUT
- Don't decorate varyings with interpolation modes at pipeline start/end by @nical in https://github.com/gfx-rs/naga/pull/2038
- Decorate integer builtins as Flat in the spirv writer by @nical in https://github.com/gfx-rs/naga/pull/2035
- Properly combine the fixes for #2035 and #2038. by @jimblandy in https://github.com/gfx-rs/naga/pull/2041
- Don't emit no-op `OpBitCast` instructions. by @jimblandy in https://github.com/gfx-rs/naga/pull/2043

HLSL-OUT
- Use the namer to sanitise entrypoint input/output struct names by @expenses in https://github.com/gfx-rs/naga/pull/2001
- Handle Unpack2x16float in hlsl by @expenses in https://github.com/gfx-rs/naga/pull/2002
- Add support for push constants by @JCapucho in https://github.com/gfx-rs/naga/pull/2005

DOT-OUT
- Improvements by @JCapucho in https://github.com/gfx-rs/naga/pull/1987

## v0.9 (2022-06-30)

- Fix minimal-versions of dependencies ([#1840](https://github.com/gfx-rs/naga/pull/1840)) **@teoxoy**
- Update MSRV to 1.56 ([#1838](https://github.com/gfx-rs/naga/pull/1838)) **@teoxoy**

API

- Rename `TypeFlags` `INTERFACE`/`HOST_SHARED` to `IO_SHARED`/`HOST_SHAREABLE` ([#1872](https://github.com/gfx-rs/naga/pull/1872)) **@jimblandy**
- Expose more error information ([#1827](https://github.com/gfx-rs/naga/pull/1827), [#1937](https://github.com/gfx-rs/naga/pull/1937)) **@jakobhellermann** **@nical** **@jimblandy**
- Do not unconditionally make error output colorful ([#1707](https://github.com/gfx-rs/naga/pull/1707)) **@rhysd**
- Rename `StorageClass` to `AddressSpace` ([#1699](https://github.com/gfx-rs/naga/pull/1699)) **@kvark**
- Add a way to emit errors to a path ([#1640](https://github.com/gfx-rs/naga/pull/1640)) **@laptou**

CLI

- Add `bincode` representation ([#1729](https://github.com/gfx-rs/naga/pull/1729)) **@kvark**
- Include file path in WGSL parse error ([#1708](https://github.com/gfx-rs/naga/pull/1708)) **@rhysd**
- Add `--version` flag ([#1706](https://github.com/gfx-rs/naga/pull/1706)) **@rhysd**
- Support reading input from stdin via `--stdin-file-path` ([#1701](https://github.com/gfx-rs/naga/pull/1701)) **@rhysd**
- Use `panic = "abort"` ([#1597](https://github.com/gfx-rs/naga/pull/1597)) **@jrmuizel**

DOCS

- Standardize some docs ([#1660](https://github.com/gfx-rs/naga/pull/1660)) **@NoelTautges**
- Document `TypeInner::BindingArray` ([#1859](https://github.com/gfx-rs/naga/pull/1859)) **@jimblandy**
- Clarify accepted types for `Expression::AccessIndex` ([#1862](https://github.com/gfx-rs/naga/pull/1862)) **@NoelTautges**
- Document `proc::layouter` ([#1693](https://github.com/gfx-rs/naga/pull/1693)) **@jimblandy**
- Document Naga's promises around validation and panics ([#1828](https://github.com/gfx-rs/naga/pull/1828)) **@jimblandy**
- `FunctionInfo` doc fixes ([#1726](https://github.com/gfx-rs/naga/pull/1726)) **@jimblandy**

VALIDATOR

- Forbid returning pointers and atomics from functions ([#911](https://github.com/gfx-rs/naga/pull/911)) **@jimblandy**
- Let validation check for more unsupported builtins ([#1962](https://github.com/gfx-rs/naga/pull/1962)) **@jimblandy**
- Fix `Capabilities::SAMPLER_NON_UNIFORM_INDEXING` bitflag ([#1915](https://github.com/gfx-rs/naga/pull/1915)) **@cwfitzgerald**
- Properly check that user-defined IO uses IO-shareable types ([#912](https://github.com/gfx-rs/naga/pull/912)) **@jimblandy**
- Validate `ValuePointer` exactly like a `Pointer` to a `Scalar` ([#1875](https://github.com/gfx-rs/naga/pull/1875)) **@jimblandy**
- Reject empty structs ([#1826](https://github.com/gfx-rs/naga/pull/1826)) **@jimblandy**
- Validate uniform address space layout constraints ([#1812](https://github.com/gfx-rs/naga/pull/1812)) **@teoxoy**
- Improve `AddressSpace` related error messages ([#1710](https://github.com/gfx-rs/naga/pull/1710)) **@kvark**

WGSL-IN

Main breaking changes

- Commas to separate struct members (comma after last member is optional)
  - `struct S { a: f32; b: i32; }` -> `struct S { a: f32, b: i32 }`
- Attribute syntax
  - `[[binding(0), group(0)]]` -> `@binding(0) @group(0)`
- Entry point stage attributes
  - `@stage(vertex)` -> `@vertex`
  - `@stage(fragment)` -> `@fragment`
  - `@stage(compute)` -> `@compute`
- Function renames
  - `smoothStep` -> `smoothstep`
  - `findLsb` -> `firstTrailingBit`
  - `findMsb` -> `firstLeadingBit`

Specification Changes (relevant changes have also been applied to the WGSL backend)

- Add support for `break if` ([#1993](https://github.com/gfx-rs/naga/pull/1993)) **@JCapucho**
- Update number literal format ([#1863](https://github.com/gfx-rs/naga/pull/1863)) **@teoxoy**
- Allow non-ascii characters in identifiers ([#1849](https://github.com/gfx-rs/naga/pull/1849)) **@teoxoy**
- Update reserved keywords ([#1847](https://github.com/gfx-rs/naga/pull/1847), [#1870](https://github.com/gfx-rs/naga/pull/1870), [#1905](https://github.com/gfx-rs/naga/pull/1905)) **@teoxoy** **@Gordon-F**
- Update entry point stage attributes ([#1833](https://github.com/gfx-rs/naga/pull/1833)) **@Gordon-F**
- Make colon in case optional ([#1801](https://github.com/gfx-rs/naga/pull/1801)) **@Gordon-F**
- Rename `smoothStep` to `smoothstep` ([#1800](https://github.com/gfx-rs/naga/pull/1800)) **@Gordon-F**
- Make semicolon after struct declaration optional ([#1791](https://github.com/gfx-rs/naga/pull/1791)) **@stshine**
- Use commas to separate struct members instead of semicolons ([#1773](https://github.com/gfx-rs/naga/pull/1773)) **@Gordon-F**
- Rename `findLsb`/`findMsb` to `firstTrailingBit`/`firstLeadingBit` ([#1735](https://github.com/gfx-rs/naga/pull/1735)) **@kvark**
- Make parenthesis optional for `if` and `switch` statements ([#1725](https://github.com/gfx-rs/naga/pull/1725)) **@Gordon-F**
- Declare attributes with `@attrib` instead of `[[attrib]]` ([#1676](https://github.com/gfx-rs/naga/pull/1676)) **@kvark**
- Allow non-structure buffer types ([#1682](https://github.com/gfx-rs/naga/pull/1682)) **@kvark**
- Remove `stride` attribute ([#1681](https://github.com/gfx-rs/naga/pull/1681)) **@kvark**

Improvements

- Implement complete validation for size and align attributes ([#1979](https://github.com/gfx-rs/naga/pull/1979)) **@teoxoy**
- Implement `firstTrailingBit`/`firstLeadingBit` u32 overloads ([#1865](https://github.com/gfx-rs/naga/pull/1865)) **@teoxoy**
- Add error for non-floating-point matrix ([#1917](https://github.com/gfx-rs/naga/pull/1917)) **@grovesNL**
- Implement partial vector & matrix identity constructors ([#1916](https://github.com/gfx-rs/naga/pull/1916)) **@teoxoy**
- Implement phony assignment ([#1866](https://github.com/gfx-rs/naga/pull/1866), [#1869](https://github.com/gfx-rs/naga/pull/1869)) **@teoxoy**
- Fix being able to match `~=` as LogicalOperation ([#1849](https://github.com/gfx-rs/naga/pull/1849)) **@teoxoy**
- Implement Binding Arrays ([#1845](https://github.com/gfx-rs/naga/pull/1845)) **@cwfitzgerald**
- Implement unary vector operators ([#1820](https://github.com/gfx-rs/naga/pull/1820)) **@teoxoy**
- Implement zero value constructors and constructors that infer their type from their parameters ([#1790](https://github.com/gfx-rs/naga/pull/1790)) **@teoxoy**
- Implement invariant attribute ([#1789](https://github.com/gfx-rs/naga/pull/1789), [#1822](https://github.com/gfx-rs/naga/pull/1822)) **@teoxoy** **@jimblandy**
- Implement increment and decrement statements ([#1788](https://github.com/gfx-rs/naga/pull/1788), [#1912](https://github.com/gfx-rs/naga/pull/1912)) **@teoxoy**
- Implement `while` loop ([#1787](https://github.com/gfx-rs/naga/pull/1787)) **@teoxoy**
- Fix array size on globals ([#1717](https://github.com/gfx-rs/naga/pull/1717)) **@jimblandy**
- Implement integer vector overloads for `dot` function ([#1689](https://github.com/gfx-rs/naga/pull/1689)) **@francesco-cattoglio**
- Implement block comments ([#1675](https://github.com/gfx-rs/naga/pull/1675)) **@kocsis1david**
- Implement assignment binary operators ([#1662](https://github.com/gfx-rs/naga/pull/1662)) **@kvark**
- Implement `radians`/`degrees` builtin functions ([#1627](https://github.com/gfx-rs/naga/pull/1627)) **@encounter**
- Implement `findLsb`/`findMsb` builtin functions ([#1473](https://github.com/gfx-rs/naga/pull/1473)) **@fintelia**
- Implement `textureGather`/`textureGatherCompare` builtin functions ([#1596](https://github.com/gfx-rs/naga/pull/1596)) **@kvark**

SPV-IN

- Implement `OpBitReverse` and `OpBitCount` ([#1954](https://github.com/gfx-rs/naga/pull/1954)) **@JCapucho**
- Add `MultiView` to `SUPPORTED_CAPABILITIES` ([#1934](https://github.com/gfx-rs/naga/pull/1934)) **@expenses**
- Translate `OpSMod` and `OpFMod` correctly ([#1867](https://github.com/gfx-rs/naga/pull/1867), [#1995](https://github.com/gfx-rs/naga/pull/1995)) **@teoxoy** **@JCapucho**
- Error on unsupported `MatrixStride` ([#1805](https://github.com/gfx-rs/naga/pull/1805)) **@teoxoy**
- Align array stride for undecorated arrays ([#1724](https://github.com/gfx-rs/naga/pull/1724)) **@JCapucho**

GLSL-IN

- Don't allow empty last case in switch ([#1981](https://github.com/gfx-rs/naga/pull/1981)) **@JCapucho**
- Fix last case fallthrough and empty switch ([#1981](https://github.com/gfx-rs/naga/pull/1981)) **@JCapucho**
- Splat inputs for smoothstep if needed ([#1976](https://github.com/gfx-rs/naga/pull/1976)) **@JCapucho**
- Fix parameter not changing to depth ([#1967](https://github.com/gfx-rs/naga/pull/1967)) **@JCapucho**
- Fix matrix multiplication check ([#1953](https://github.com/gfx-rs/naga/pull/1953)) **@JCapucho**
- Fix panic (stop emitter in conditional) ([#1952](https://github.com/gfx-rs/naga/pull/1952)) **@JCapucho**
- Translate `mod` fn correctly ([#1867](https://github.com/gfx-rs/naga/pull/1867)) **@teoxoy**
- Make the ternary operator behave as an if ([#1877](https://github.com/gfx-rs/naga/pull/1877)) **@JCapucho**
- Add support for `clamp` function ([#1502](https://github.com/gfx-rs/naga/pull/1502)) **@sjinno**
- Better errors for bad constant expression ([#1501](https://github.com/gfx-rs/naga/pull/1501)) **@sjinno**
- Error on a `matCx2` used with the `std140` layout ([#1806](https://github.com/gfx-rs/naga/pull/1806)) **@teoxoy**
- Allow nested accesses in lhs positions ([#1794](https://github.com/gfx-rs/naga/pull/1794)) **@JCapucho**
- Use forced conversions for vector/matrix constructors ([#1796](https://github.com/gfx-rs/naga/pull/1796)) **@JCapucho**
- Add support for `barrier` function ([#1793](https://github.com/gfx-rs/naga/pull/1793)) **@fintelia**
- Fix panic (resume expression emit after `imageStore`) ([#1795](https://github.com/gfx-rs/naga/pull/1795)) **@JCapucho**
- Allow multiple array specifiers ([#1780](https://github.com/gfx-rs/naga/pull/1780)) **@JCapucho**
- Fix memory qualifiers being inverted ([#1779](https://github.com/gfx-rs/naga/pull/1779)) **@JCapucho**
- Support arrays as input/output types ([#1759](https://github.com/gfx-rs/naga/pull/1759)) **@JCapucho**
- Fix freestanding constructor parsing ([#1758](https://github.com/gfx-rs/naga/pull/1758)) **@JCapucho**
- Fix matrix - scalar operations ([#1757](https://github.com/gfx-rs/naga/pull/1757)) **@JCapucho**
- Fix matrix - matrix division ([#1757](https://github.com/gfx-rs/naga/pull/1757)) **@JCapucho**
- Fix matrix comparisons ([#1757](https://github.com/gfx-rs/naga/pull/1757)) **@JCapucho**
- Add support for `texelFetchOffset` ([#1746](https://github.com/gfx-rs/naga/pull/1746)) **@JCapucho**
- Inject `sampler2DMSArray` builtins on use ([#1737](https://github.com/gfx-rs/naga/pull/1737)) **@JCapucho**
- Inject `samplerCubeArray` builtins on use ([#1736](https://github.com/gfx-rs/naga/pull/1736)) **@JCapucho**
- Add support for image builtin functions ([#1723](https://github.com/gfx-rs/naga/pull/1723)) **@JCapucho**
- Add support for image declarations ([#1723](https://github.com/gfx-rs/naga/pull/1723)) **@JCapucho**
- Texture builtins fixes ([#1719](https://github.com/gfx-rs/naga/pull/1719)) **@JCapucho**
- Type qualifiers rework ([#1713](https://github.com/gfx-rs/naga/pull/1713)) **@JCapucho**
- `texelFetch` accept multisampled textures ([#1715](https://github.com/gfx-rs/naga/pull/1715)) **@JCapucho**
- Fix panic when culling nested block ([#1714](https://github.com/gfx-rs/naga/pull/1714)) **@JCapucho**
- Fix composite constructors ([#1631](https://github.com/gfx-rs/naga/pull/1631)) **@JCapucho**
- Fix using swizzle as out arguments ([#1632](https://github.com/gfx-rs/naga/pull/1632)) **@JCapucho**

SPV-OUT

- Implement `reverseBits` and `countOneBits` ([#1897](https://github.com/gfx-rs/naga/pull/1897)) **@hasali19**
- Use `OpCopyObject` for matrix identity casts ([#1916](https://github.com/gfx-rs/naga/pull/1916)) **@teoxoy**
- Use `OpCopyObject` for bool - bool conversion due to `OpBitcast` not being feasible for booleans ([#1916](https://github.com/gfx-rs/naga/pull/1916)) **@teoxoy**
- Zero init variables in function and private address spaces ([#1871](https://github.com/gfx-rs/naga/pull/1871)) **@teoxoy**
- Use `SRem` instead of `SMod` ([#1867](https://github.com/gfx-rs/naga/pull/1867)) **@teoxoy**
- Add support for integer vector - scalar multiplication ([#1820](https://github.com/gfx-rs/naga/pull/1820)) **@teoxoy**
- Add support for matrix addition and subtraction ([#1820](https://github.com/gfx-rs/naga/pull/1820)) **@teoxoy**
- Emit required decorations on wrapper struct types ([#1815](https://github.com/gfx-rs/naga/pull/1815)) **@jimblandy**
- Decorate array and struct type layouts unconditionally ([#1815](https://github.com/gfx-rs/naga/pull/1815)) **@jimblandy**
- Fix wrong `MatrixStride` for `matCx2` and `mat2xR` ([#1781](https://github.com/gfx-rs/naga/pull/1781)) **@teoxoy**
- Use `OpImageQuerySize` for MS images ([#1742](https://github.com/gfx-rs/naga/pull/1742)) **@JCapucho**

MSL-OUT

- Insert padding initialization for global constants ([#1988](https://github.com/gfx-rs/naga/pull/1988)) **@teoxoy**
- Don't rely on cached expressions ([#1975](https://github.com/gfx-rs/naga/pull/1975)) **@JCapucho**
- Fix pointers to private or workgroup address spaces possibly being read only ([#1901](https://github.com/gfx-rs/naga/pull/1901)) **@teoxoy**
- Zero init variables in function address space ([#1871](https://github.com/gfx-rs/naga/pull/1871)) **@teoxoy**
- Make binding arrays play nice with bounds checks ([#1855](https://github.com/gfx-rs/naga/pull/1855)) **@cwfitzgerald**
- Permit `invariant` qualifier on vertex shader outputs ([#1821](https://github.com/gfx-rs/naga/pull/1821)) **@jimblandy**
- Fix packed `vec3` stores ([#1816](https://github.com/gfx-rs/naga/pull/1816)) **@teoxoy**
- Actually test push constants to be used ([#1767](https://github.com/gfx-rs/naga/pull/1767)) **@kvark**
- Properly rename entry point arguments for struct members ([#1766](https://github.com/gfx-rs/naga/pull/1766)) **@jimblandy**
- Qualify read-only storage with const ([#1763](https://github.com/gfx-rs/naga/pull/1763)) **@kvark**
- Fix not unary operator for integer scalars ([#1760](https://github.com/gfx-rs/naga/pull/1760)) **@vincentisambart**
- Add bounds checks for `ImageLoad` and `ImageStore` ([#1730](https://github.com/gfx-rs/naga/pull/1730)) **@jimblandy**
- Fix resource bindings for non-structures ([#1718](https://github.com/gfx-rs/naga/pull/1718)) **@kvark**
- Always check whether _buffer_sizes arg is needed ([#1717](https://github.com/gfx-rs/naga/pull/1717)) **@jimblandy**
- WGSL storage address space should always correspond to MSL device address space ([#1711](https://github.com/gfx-rs/naga/pull/1711)) **@wtholliday**
- Mitigation for MSL atomic bounds check ([#1703](https://github.com/gfx-rs/naga/pull/1703)) **@glalonde**

HLSL-OUT

- More `matCx2` fixes (#1989) ([#1989](https://github.com/gfx-rs/naga/pull/1989)) **@teoxoy**
- Fix fallthrough in switch statements ([#1920](https://github.com/gfx-rs/naga/pull/1920)) **@teoxoy**
- Fix missing break statements ([#1919](https://github.com/gfx-rs/naga/pull/1919)) **@teoxoy**
- Fix `countOneBits` and `reverseBits` for signed integers ([#1928](https://github.com/gfx-rs/naga/pull/1928)) **@hasali19**
- Fix array constructor return type ([#1914](https://github.com/gfx-rs/naga/pull/1914)) **@teoxoy**
- Fix hlsl output for writes to scalar/vector storage buffer ([#1903](https://github.com/gfx-rs/naga/pull/1903)) **@hasali19**
- Use `fmod` instead of `%` ([#1867](https://github.com/gfx-rs/naga/pull/1867)) **@teoxoy**
- Use wrapped constructors when loading from storage address space ([#1893](https://github.com/gfx-rs/naga/pull/1893)) **@teoxoy**
- Zero init struct constructor ([#1890](https://github.com/gfx-rs/naga/pull/1890)) **@teoxoy**
- Flesh out matrix handling documentation ([#1850](https://github.com/gfx-rs/naga/pull/1850)) **@jimblandy**
- Emit `row_major` qualifier on matrix uniform globals ([#1846](https://github.com/gfx-rs/naga/pull/1846)) **@jimblandy**
- Fix bool splat ([#1820](https://github.com/gfx-rs/naga/pull/1820)) **@teoxoy**
- Add more padding when necessary ([#1814](https://github.com/gfx-rs/naga/pull/1814)) **@teoxoy**
- Support multidimensional arrays ([#1814](https://github.com/gfx-rs/naga/pull/1814)) **@teoxoy**
- Don't output interpolation modifier if it's the default ([#1809](https://github.com/gfx-rs/naga/pull/1809)) **@NoelTautges**
- Fix `matCx2` translation for uniform buffers ([#1802](https://github.com/gfx-rs/naga/pull/1802)) **@teoxoy**
- Fix modifiers not being written in the vertex output and fragment input structs ([#1789](https://github.com/gfx-rs/naga/pull/1789)) **@teoxoy**
- Fix matrix not being declared as transposed ([#1784](https://github.com/gfx-rs/naga/pull/1784)) **@teoxoy**
- Insert padding between struct members ([#1786](https://github.com/gfx-rs/naga/pull/1786)) **@teoxoy**
- Fix not unary operator for integer scalars ([#1760](https://github.com/gfx-rs/naga/pull/1760)) **@vincentisambart**

GLSL-OUT

- Fix vector bitcasts (#1966) ([#1966](https://github.com/gfx-rs/naga/pull/1966)) **@expenses**
- Perform casts in int only math functions ([#1978](https://github.com/gfx-rs/naga/pull/1978)) **@JCapucho**
- Don't rely on cached expressions ([#1975](https://github.com/gfx-rs/naga/pull/1975)) **@JCapucho**
- Fix type error for `countOneBits` implementation ([#1897](https://github.com/gfx-rs/naga/pull/1897)) **@hasali19**
- Fix storage format for `Rgba8Unorm` ([#1955](https://github.com/gfx-rs/naga/pull/1955)) **@JCapucho**
- Implement bounds checks for `ImageLoad` ([#1889](https://github.com/gfx-rs/naga/pull/1889)) **@JCapucho**
- Fix feature search in expressions ([#1887](https://github.com/gfx-rs/naga/pull/1887)) **@JCapucho**
- Emit globals of any type ([#1823](https://github.com/gfx-rs/naga/pull/1823)) **@jimblandy**
- Add support for boolean vector `~`, `|` and `&` ops ([#1820](https://github.com/gfx-rs/naga/pull/1820)) **@teoxoy**
- Fix array function arguments ([#1814](https://github.com/gfx-rs/naga/pull/1814)) **@teoxoy**
- Write constant sized array type for uniform ([#1768](https://github.com/gfx-rs/naga/pull/1768)) **@hatoo**
- Texture function fixes ([#1742](https://github.com/gfx-rs/naga/pull/1742)) **@JCapucho**
- Push constants use anonymous uniforms ([#1683](https://github.com/gfx-rs/naga/pull/1683)) **@JCapucho**
- Add support for push constant emulation ([#1672](https://github.com/gfx-rs/naga/pull/1672)) **@JCapucho**
- Skip unsized types if unused ([#1649](https://github.com/gfx-rs/naga/pull/1649)) **@kvark**
- Write struct and array initializers ([#1644](https://github.com/gfx-rs/naga/pull/1644)) **@JCapucho**


## v0.8.5 (2022-01-25)

MSL-OUT

- Make VS-output positions invariant on even more systems ([#1697](https://github.com/gfx-rs/naga/pull/1697)) **@cwfitzgerald**
- Improve support for point primitives ([#1696](https://github.com/gfx-rs/naga/pull/1696)) **@kvark**


## v0.8.4 (2022-01-24)

MSL-OUT

- Make VS-output positions invariant if possible ([#1687](https://github.com/gfx-rs/naga/pull/1687)) **@kvark**

GLSL-OUT

- Fix `floatBitsToUint` spelling ([#1688](https://github.com/gfx-rs/naga/pull/1688)) **@cwfitzgerald**
- Call proper memory barrier functions ([#1680](https://github.com/gfx-rs/naga/pull/1680)) **@francesco-cattoglio**


## v0.8.3 (2022-01-20)

- Don't pin `indexmap` version ([#1666](https://github.com/gfx-rs/naga/pull/1666)) **@a1phyr**

MSL-OUT

- Fix support for point primitives ([#1674](https://github.com/gfx-rs/naga/pull/1674)) **@kvark**

GLSL-OUT

- Fix sampler association ([#1671](https://github.com/gfx-rs/naga/pull/1671)) **@JCapucho**


## v0.8.2 (2022-01-11)

VALIDATOR

- Check structure resource types ([#1639](https://github.com/gfx-rs/naga/pull/1639)) **@kvark**

WGSL-IN

- Improve type mismatch errors ([#1658](https://github.com/gfx-rs/naga/pull/1658)) **@Gordon-F**

SPV-IN

- Implement more sign agnostic operations ([#1651](https://github.com/gfx-rs/naga/pull/1651), [#1650](https://github.com/gfx-rs/naga/pull/1650)) **@JCapucho**

SPV-OUT

- Fix modulo operator (use `OpFRem` instead of `OpFMod`) ([#1653](https://github.com/gfx-rs/naga/pull/1653)) **@JCapucho**

MSL-OUT

- Fix `texture1d` accesses ([#1647](https://github.com/gfx-rs/naga/pull/1647)) **@jimblandy**
- Fix data packing functions ([#1637](https://github.com/gfx-rs/naga/pull/1637)) **@phoekz**


## v0.8.1 (2021-12-29)

API

- Make `WithSpan` cloneable ([#1620](https://github.com/gfx-rs/naga/pull/1620)) **@jakobhellermann**

MSL-OUT

- Fix packed vec access ([#1634](https://github.com/gfx-rs/naga/pull/1634)) **@kvark**
- Fix packed float support ([#1630](https://github.com/gfx-rs/naga/pull/1630)) **@kvark**

HLSL-OUT

- Support arrays of matrices ([#1629](https://github.com/gfx-rs/naga/pull/1629)) **@kvark**
- Use `mad` instead of `fma` function ([#1580](https://github.com/gfx-rs/naga/pull/1580)) **@parasyte**

GLSL-OUT

- Fix conflicting names for globals ([#1616](https://github.com/gfx-rs/naga/pull/1616)) **@Gordon-F**
- Fix `fma` function ([#1580](https://github.com/gfx-rs/naga/pull/1580)) **@parasyte**


## v0.8 (2021-12-18)
  - development release for wgpu-0.12
  - lots of fixes in all parts
  - validator:
    - now gated by `validate` feature
    - nicely detailed error messages with spans
  - API:
    - image gather operations
  - WGSL-in:
    - remove `[[block]]` attribute
    - `elseif` is removed in favor of `else if`
  - MSL-out:
    - full out-of-bounds checking

## v0.7.3 (2021-12-14)
  - API:
    - `view_index` builtin
  - GLSL-out:
    - reflect textures without samplers
  - SPV-out:
    - fix incorrect pack/unpack

## v0.7.2 (2021-12-01)
  - validator:
    - check stores for proper pointer class
  - HLSL-out:
    - fix stores into `mat3`
    - respect array strides
  - SPV-out:
    - fix multi-word constants
  - WGSL-in:
    - permit names starting with underscores
  - SPV-in:
    - cull unused builtins
    - support empty debug labels
  - GLSL-in:
    - don't panic on invalid integer operations

## v0.7.1 (2021-10-12)
  - implement casts from and to booleans in the backends

## v0.7 (2021-10-07)
  - development release for wgpu-0.11
  - API:
    - bit extraction and packing functions
    - hyperbolic trigonometry functions
    - validation is gated by a cargo feature
    - `view_index` builtin
    - separate bounds checking policies for locals/buffers/textures
  - IR:
    - types and constants are guaranteed to be unique
  - WGSL-in:
    - new hex literal parser
    - updated list of reserved words
    - rewritten logic for resolving references and pointers
    - `switch` can use unsigned selectors
  - GLSL-in:
    - better support for texture sampling
    - better logic for auto-splatting scalars
  - GLSL-out:
    - fixed storage buffer layout
    - fix module operator
  - HLSL-out:
    - fixed texture queries
  - SPV-in:
    - control flow handling is rewritten from scratch
  - SPV-out:
    - fully covered out-of-bounds checking
    - option to emit point size
    - option to clamp output depth

## v0.6.3 (2021-09-08)
  - Reduced heap allocations when generating WGSL, HLSL, and GLSL
  - WGSL-in:
    - support module-scope `let` type inference
  - SPV-in:
    - fix depth sampling with projection
  - HLSL-out:
    - fix local struct construction
  - GLSL-out:
    - fix `select()` order
  - SPV-out:
    - allow working around Adreno issue with `OpName`

## v0.6.2 (2021-09-01)
  - SPV-out fixes:
    - requested capabilities for 1D and cube images, storage formats
    - handling `break` and `continue` in a `switch` statement
    - avoid generating duplicate `OpTypeImage` types
  - HLSL-out fixes:
    - fix output struct member names
  - MSL-out fixes:
    - fix packing of fields in interface structs
  - GLSL-out fixes:
    - fix non-fallthrough `switch` cases
  - GLSL-in fixes:
    - avoid infinite loop on invalid statements

## v0.6.1 (2021-08-24)
  - HLSL-out fixes:
    - array arguments
    - pointers to array arguments
    - switch statement
    - rewritten interface matching
  - SPV-in fixes:
    - array storage texture stores
    - tracking sampling across function parameters
    - updated petgraph dependencies
  - MSL-out:
    - gradient sampling
  - GLSL-out:
    - modulo operator on floats

## v0.6 (2021-08-18)
  - development release for wgpu-0.10
  - API:
    - atomic types and functions
    - storage access is moved from global variables to the storage class and storage texture type
    - new built-ins: `primitive_index` and `num_workgroups`
    - support for multi-sampled depth images
  - WGSL:
    - `select()` order of true/false is swapped
  - HLSL backend is vastly improved and now usable
  - GLSL frontend is heavily reworked

## v0.5 (2021-06-18)
  - development release for wgpu-0.9
  - API:
    - barriers
    - dynamic indexing of matrices and arrays is only allowed on variables
    - validator now accepts a list of IR capabilities to allow
    - improved documentation
  - Infrastructure:
    - much richer test suite, focused around consuming or emitting WGSL
    - lazy testing on large shader corpuses
    - the binary is moved to a sub-crate "naga-cli"
  - Frontends:
    - GLSL frontend:
      - rewritten from scratch and effectively revived, no longer depends on `pomelo`
      - only supports 440/450/460 versions for now
      - has optional support for codespan messages
    - SPIRV frontend has improved CFG resolution (still with issues unresolved)
    - WGSL got better error messages, workgroup memory support
  - Backends:
    - general: better expression naming and emitting
    - new HLSL backend (in progress)
    - MSL:
      - support `ArraySize` expression
      - better texture sampling instructions
    - GLSL:
      - multisampling on GLES
    - WGSL is vastly improved and now usable

## v0.4.2 (2021-05-28)
  - SPIR-V frontend:
    - fix image stores
    - fix matrix stride check
  - SPIR-V backend:
    - fix auto-deriving the capabilities
  - GLSL backend:
    - support sample interpolation
    - write out swizzled vector accesses

## v0.4.1 (2021-05-14)
  - numerous additions and improvements to SPIR-V frontend:
    - int8, in16, int64
    - null constant initializers for structs and matrices
    - `OpArrayLength`, `OpCopyMemory`, `OpInBoundsAccessChain`, `OpLogicalXxxEqual`
    - outer product
    - fix struct size alignment
    - initialize built-ins with default values
    - fix read-only decorations on struct members
  - fix struct size alignment in WGSL
  - fix `fwidth` in WGSL
  - fix scalars arrays in GLSL backend

## v0.4 (2021-04-29)
  - development release for wgpu-0.8
  - API:
    - expressions are explicitly emitted with `Statement::Emit`
    - entry points have inputs in arguments and outputs in the result type
    - `input`/`output` storage classes are gone, but `push_constant` is added
    - `Interpolation` is moved into `Binding::Location` variant
    - real pointer semantics with required `Expression::Load`
    - `TypeInner::ValuePointer` is added
    - image query expressions are added
    - new `Statement::ImageStore`
    - all function calls are `Statement::Call`
    - `GlobalUse` is moved out into processing
    - `Header` is removed
    - entry points are an array instead of a map
    - new `Swizzle` and `Splat` expressions
    - interpolation qualifiers are extended and required
    - struct member layout is based on the byte offsets
  - Infrastructure:
    - control flow uniformity analysis
    - texture-sampler combination gathering
    - `CallGraph` processor is moved out into `glsl` backend
    - `Interface` is removed, instead the analysis produces `ModuleInfo` with all the derived info
    - validation of statement tree, expressions, and constants
    - code linting is more strict for matches
  - new GraphViz `dot` backend for pretty visualization of the IR
  - Metal support for inlined samplers
  - `convert` example is transformed into the default binary target named `naga`
  - lots of frontend and backend fixes

## v0.3.2 (2021-02-15)
  - fix logical expression types
  - fix _FragDepth_ semantics
  - spv-in:
    - derive block status of structures
  - spv-out:
    - add lots of missing math functions
    - implement discard

## v0.3.1 (2021-01-31)
  - wgsl:
    - support constant array sizes
  - spv-out:
    - fix block decorations on nested structures
    - fix fixed-size arrays
    - fix matrix decorations inside structures
    - implement read-only decorations

## v0.3 (2021-01-30)
  - development release for wgpu-0.7
  - API:
    - math functions
    - type casts
    - updated storage classes
    - updated image sub-types
    - image sampling/loading options
    - storage images
    - interpolation qualifiers
    - early and conservative depth
  - Processors:
    - name manager
    - automatic layout
    - termination analysis
    - validation of types, constants, variables, and entry points

## v0.2 (2020-08-17)
  - development release for wgpu-0.6

## v0.1 (2020-02-26)
  - initial release
