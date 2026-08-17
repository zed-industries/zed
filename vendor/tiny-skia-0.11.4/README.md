# tiny-skia
![Build Status](https://github.com/RazrFalcon/tiny-skia/workflows/Rust/badge.svg)
[![Crates.io](https://img.shields.io/crates/v/tiny-skia.svg)](https://crates.io/crates/tiny-skia)
[![Documentation](https://docs.rs/tiny-skia/badge.svg)](https://docs.rs/tiny-skia)

`tiny-skia` is a tiny [Skia] subset ported to Rust.

The goal is to provide an absolute minimal, CPU only, 2D rendering library for the Rust ecosystem,
with a focus on a rendering quality, speed and binary size.

And while `tiny-skia` is definitely tiny, it support all the common 2D operations
like: filling and stroking a shape with a solid color, gradient or pattern;
stroke dashing; clipping; images blending; PNG load/save.
The main missing feature is text rendering
(see [#1](https://github.com/RazrFalcon/tiny-skia/issues/1)).

**Note:** this is not a Skia replacement and never will be. It's more of a research project.

MSRV: stable

## Motivation

The main motivation behind this library is to have a small, high-quality 2D rendering
library that can be used by [resvg]. And the choice is rather limited.
You basically have to choose between [cairo], Qt and Skia. And all of them are
relatively bloated, hard to compile and distribute. Not to mention that none of them
are written in Rust.

But if we ignore those problems and focus only on quality and speed alone,
Skia is by far the best one.
However, the main problem with Skia is that it's huge. Really huge.
It supports CPU and GPU rendering, multiple input and output formats (including SVG and PDF),
various filters, color spaces, color types and text rendering.
It consists of 370 KLOC without dependencies (around 7 MLOC with dependencies)
and requires around 4-8 GiB of disk space to be built from sources.
And the final binary is 3-8 MiB big, depending on enabled features.
Not to mention that it requires `clang` and no other compiler
and uses an obscure build system (`gn`) which was using Python2 until recently.

`tiny-skia` tries to be small, simple and easy to build.
Currently, it has around 14 KLOC, compiles in less than 5s on a modern CPU
and adds around 200KiB to your binary.

## Performance

Currently, `tiny-skia` is 20-100% slower than Skia on x86-64 and about 100-300% slower on ARM.
Which is still faster than [cairo] and [raqote] in many cases.
See benchmark results [here](https://razrfalcon.github.io/tiny-skia/x86_64.html).

The heart of Skia's CPU rendering is
[SkRasterPipeline](https://github.com/google/skia/blob/master/src/opts/SkRasterPipeline_opts.h).
And this is an extremely optimized piece of code.
But to be a bit pedantic, it's not really a C++ code. It relies on clang's
non-standard vector extensions, which means that it works only with clang.
You can actually build it with gcc/msvc, but it will simply ignore all the optimizations
and become 15-30 *times* slower! Which makes it kinda useless.

Also note, that neither Skia or `tiny-skia` are supporting dynamic CPU detection,
so by enabling newer instructions you're making the resulting binary non-portable.

Essentially, you will get a decent performance on x86 targets by default.
But if you are looking for an even better performance, you should compile your application
with `RUSTFLAGS="-Ctarget-cpu=haswell"` environment variable to enable AVX instructions.

We support ARM AArch64 NEON as well and there is no need to pass any additional flags.

You can find more information in [benches/README.md](./benches/README.md).

## Rendering quality

Unless there is a bug, `tiny-skia` must produce exactly the same results as Skia.

## Safety

While a quick search would shown tons of `unsafe`, the library is actually fully safe.
All pixels access is bound-checked. And all memory-related operations are safe.

We must use `unsafe` to call SIMD intrinsics, which is perfectly safe,
but Rust's std still marks them as `unsafe` because they may be missing on the target CPU.
We do check for that.

We also have to mark some types (to cast `[u32; 1]` to `[u8; 4]` and vise-versa) as
[bytemuck::Pod](https://docs.rs/bytemuck/1.4.1/bytemuck/trait.Pod.html),
which is an `unsafe` trait, but still is perfectly safe.

## Out of scope

Skia is a huge library and we support only a tiny part of.
And more importantly, we do not plan to support many feature at all.

- GPU rendering.
- Text rendering (maybe someday).
- PDF generation.
- Non-RGBA8888 images.
- Non-PNG image formats.
- Advanced Bézier path operations.
- Conic path segments.
- Path effects (except dashing).
- Any kind of resource caching.
- ICC profiles.

## Notable changes

Despite being a port, we still have a lot of changes even in the supported subset.

- No global alpha.<br/>
  Unlike Skia, only `Pattern` is allowed to have opacity.
  In all other cases you should adjust colors opacity manually.
- No bilinear + mipmap down-scaling support.
- `tiny-skia` uses just a simple alpha mask for clipping, while Skia has a very complicated,
but way faster algorithm.

## Notes about the port

`tiny-skia` should be viewed as a Rust 2D rendering library that uses Skia algorithms internally.
We have a completely different public API. The internals are also extremely simplified.
But all the core logic and math is borrowed from Skia. Hence the name.

As for the porting process itself, Skia uses goto, inheritance, virtual methods, linked lists,
const generics and templates specialization a lot, and all of this features are unavailable in Rust.
There are also a lot of pointers magic, implicit mutations and caches.
Therefore we have to compromise or even rewrite some parts from scratch.

## Alternatives

Right now, the only pure Rust alternative is [raqote].

- It doesn't support high-quality antialiasing (hairline stroking in particular).
- It's very slow (see [benchmarks](./benches/README.md)).
- There are some rendering issues (like gradient transparency).
- Raqote has a very rudimentary text rendering support, while tiny-skia has none.

## License

The same as used by [Skia]: [New BSD License](./LICENSE)

[Skia]: https://skia.org/
[cairo]: https://www.cairographics.org/
[raqote]: https://github.com/jrmuizel/raqote
[resvg]: https://github.com/RazrFalcon/resvg
