# Changelog
All notable changes to this project will be documented in this file.

Before 1.0, this project does not adhere to [Semantic Versioning](http://semver.org/spec/v2.0.0.html).

Goblin is now 0.9, which means we will try our best to ease breaking changes. Tracking issue is here: https://github.com/m4b/goblin/issues/97

## [0.9.3]  - 2025-1-5
### Fixed
pe: fix import parser for non-well-formed import table, thanks @kkent030315: https://github.com/m4b/goblin/pull/429
pe: fix empty import table parsing, thanks @kkent030315: https://github.com/m4b/goblin/pull/430
### Added
pe: Add tests for TLS parser and characteristics constants, thanks @kkent030315: https://github.com/m4b/goblin/pull/426
elf: added new constants for 32-bit PowerPC, thanks @ivlzme https://github.com/m4b/goblin/pull/439

## [0.9.2]  - 2024-10-26
### Fixed
pe: fix PE with zero `raw_data_size` of section, thanks @ideeockus: https://github.com/m4b/goblin/pull/396
### Added
pe: allow parsing pe::Header without dos stubs, `Header::parse_without_dos`, thanks @ideeockus: https://github.com/m4b/goblin/pull/396

## [0.9.1]  - 2024-10-24
### (hot) Fix
pe: fix parsing of tls in certain cases (issue: https://github.com/m4b/goblin/issues/424), thanks @kkent030315: https://github.com/m4b/goblin/pull/425

## [0.9.0]  - 2024-10-20
### Added, Breaking
pe: add TE (terse executable) support, big thanks @Javagedes: https://github.com/m4b/goblin/pull/397
pe: add support for codeview PDB 2.0, thanks @joschock: https://github.com/m4b/goblin/pull/409
pe: parse TLS in data directories, thanks @kkent030315: https://github.com/m4b/goblin/pull/404

## [0.8.2]  - 2024-04-29
Everything in 0.8.1 except TE support in https://github.com/m4b/goblin/pull/397 was reverted,
due to it being technically a breaking change.
0.8.1 was yanked from crates.

## [0.8.1]  - 2024-04-27 (YANKED)
### Docs
pe: document pe header, thanks @JohnScience: https://github.com/m4b/goblin/pull/399
pe, elf: fix doc warnings, thanks @5225225: https://github.com/m4b/goblin/pull/395
pe: document dos header, thanks @JohnScience: https://github.com/m4b/goblin/pull/393
### Added
pe: add TE (terse executable) support, big thanks @Javagedes: https://github.com/m4b/goblin/pull/397
elf: allow parsing section headers from raw bytes, thanks @lissyx: https://github.com/m4b/goblin/pull/391
mach: add support for lossy parsing, thanks @h33p: https://github.com/m4b/goblin/pull/386
elf: add convenience functions, thanks @tiann : https://github.com/m4b/goblin/pull/387
### Fixed
pe: read reserved dos headers, thanks @kkent030315: https://github.com/m4b/goblin/pull/405


## [0.8.0]  - 2023-12-31 - Happy New Years!
### Breaking
msrv: bumped to 1.63.0 since scroll bumped as well
pe: new field added to parse options: https://github.com/m4b/goblin/pull/377
pe: attribute certs now non-exhaustive: https://github.com/m4b/goblin/pull/378
goblin: hint and object enum is now non-exhaustive
pe: write support introduced some breaking changes, e.g., data directories array adds a tuple of usize and data directory,
    DosHeader has all the fields filled out, Header struct has a dos_stub field added,
	symbols and strings fields is made optional in Coff struct, see: https://github.com/m4b/goblin/pull/361
### Fixed
elf: fix documentation, thanks @crzysdrs: https://github.com/m4b/goblin/pull/374
pe: attribute certificates non-exhaustive, thanks @RaitoBezarius: https://github.com/m4b/goblin/pull/378
pe: fix authenticode parsing, thanks @baloo: https://github.com/m4b/goblin/pull/383
### Added
strtab: len method added to return number of bytes of the strtab
pe: absolutely epic pe write support PR, thanks @RaitoBezarius and @Baloo: https://github.com/m4b/goblin/pull/361
pe: add coff object file support, thanks @vadimcn, https://github.com/m4b/goblin/pull/379
pe: allow toggling parsing of attribute certs, thanks @suttonbradley: https://github.com/m4b/goblin/pull/377
mach: add new mach-o constants, thanks @keith: https://github.com/m4b/goblin/pull/372

## [0.7.1] - 2023-6-11
### MSRV bump from log

## [0.7.0] - 2023-6-11
### Breaking
mach: Implement `LC_NOTE`, (breakage=load commands are marked non-exhaustive), thanks @messense: https://github.com/m4b/goblin/pull/342
### Fixed
elf: fix is_lib detection, thanks @m-hilgendorf: https://github.com/m4b/goblin/pull/366
pe: fix out of bounds access while parsing AttributeCertificate, thanks @anfedotoff: https://github.com/m4b/goblin/pull/368
### Added
pe: support basic certificates enumeration, thanks @RaitoBezarius: https://github.com/m4b/goblin/pull/354
pe: fix certificate tables parsing, thanks @baloo: https://github.com/m4b/goblin/pull/359
pe: add pe authenticode support, thanks @baloo: https://github.com/m4b/goblin/pull/362
mach: implement `LC_FILESET_ENTRY`, thanks @mmaekr: https://github.com/m4b/goblin/pull/369
build: add afl fuzzing support, thanks @anfedotoff: https://github.com/m4b/goblin/pull/351

## [0.6.1] - 2023-2-26
### Fixed
elf.section_header: additional workaround for 0-length sections, thanks @Jhynjhiruu: https://github.com/m4b/goblin/pull/347
pe.utils: file alignment check, thanks @anfedotoff: https://github.com/m4b/goblin/pull/340
### Added
elf: Add basic GNU PROPERTY note support, thanks @x64k: https://github.com/m4b/goblin/pull/352
mach: Implement `LC_BUILD_VERSION`, thanks @messense: https://github.com/m4b/goblin/pull/341

## [0.6.0] - 2022-10-23
### Breaking
macho: add support for archives in multi-arch binaries, big thanks to @nick96: https://github.com/m4b/goblin/pull/322
### Changed
elf: only consider loadable segments for VM translation (this may semantically break someone, if they depended on older behavior), thanks @lumag: https://github.com/m4b/goblin/pull/329
### Fixed
archive: fix potential panic in bsd filenames, thanks @nathaniel-daniel:  https://github.com/m4b/goblin/pull/335
archive: fix subtract with overflow, thanks @anfedotoff: https://github.com/m4b/goblin/pull/333
pe: fix oob access, thanks @anfedetoff: https://github.com/m4b/goblin/pull/330
archive: fix oob access, thanks @anfedetoff: https://github.com/m4b/goblin/pull/329
### Added
pe: add machine_to_str utility function, thanks @cgzones: https://github.com/m4b/goblin/pull/338
fuzz: add debug info for line numbers, thanks @SweetVishnya: https://github.com/m4b/goblin/pull/336

## [0.5.4] - 2022-8-14
### Fixed
pe: fix regression in PE binary parsing, thanks @SquareMan: https://github.com/m4b/goblin/pull/321

## [0.5.3] - 2022-7-16
### Fixed
elf: fix elf strtab parsing, thanks @tux3: https://github.com/m4b/goblin/pull/316
### Added
elf: implement plain for note headers, thanks @mkroening: https://github.com/m4b/goblin/pull/317

## [0.5.2] - 2022-6-5
### Fixed
elf: fix arithmetic overflows in `file_range()` and `vm_range()`, thanks @alessandron: https://github.com/m4b/goblin/pull/306
pe: fix string table containing empty strings, thanks @track-5: https://github.com/m4b/goblin/pull/310
pe: remove check on debug directory size, thanks @lzybkr: https://github.com/m4b/goblin/pull/313
### Added
elf: expose more of programheader impl regardless of alloc feature flag, thanks @dancrossnyc: https://github.com/m4b/goblin/pull/308
mach.parse: Handle DyldExportsTrie, thanks @apalm: https://github.com/m4b/goblin/pull/303

## [0.5.1] - 2022-2-13
### BREAKING
goblin: guard all capacity allocations with bounds checks, this is breaking because we introduced a new error enum, which is now marked as non_exhaustive, thanks @Swatinem: https://github.com/m4b/goblin/pull/298
pe: support exports without an offset, thanks @dureuill: https://github.com/m4b/goblin/pull/293
### Fixed
mach: fix overflow panics, thanks @Swatinem: https://github.com/m4b/goblin/pull/302
pe: add signature header check, thanks @skdltmxn: https://github.com/m4b/goblin/pull/286
elf: improve parsing `SHT_SYMTAB` complexity from O(N^2) to O(N), thanks @Lichsto: https://github.com/m4b/goblin/pull/297
### Added
elf: clarify documentation on strtab behavior better, and add nice doc example, thanks @n01e0: https://github.com/m4b/goblin/pull/301
elf: add rpaths and runpath to elf, thanks @messense: https://github.com/m4b/goblin/pull/294
elf: complete elf OSABI constants, thanks @messense: https://github.com/m4b/goblin/pull/295
elf: fill out more elf constants, thanks @n01e0: https://github.com/m4b/goblin/pull/296

## [0.5.0] - 2022-2-13
YANKED, see 0.5.1

## [0.4.3] - 2021-9-18
### Added
- elf: add initial versioned symbols support, thanks @johannst: https://github.com/m4b/goblin/pull/280
- elf: add some missing constants, `PF_MASKOS` and `PF_MASKPROC`, thanks @npmccallum: https://github.com/m4b/goblin/pull/281

## [0.4.2] - 2021-7-4
### Added
- strtab: preparses the string table to prevent certain class of DoS attacks, thanks @Lichtsto: https://github.com/m4b/goblin/pull/275

## [0.4.1] - 2021-5-30
### Fixed
- elf: fix error when alloc, but not endian, thanks @dancrossnyc: https://github.com/m4b/goblin/pull/273

## [0.4.0] - 2021-4-11
### BREAKING
- elf: fix returning invalid ranges for SH_NOBIT sections,
  method changed to return optional range instead, thanks @Tiwalun: https://github.com/m4b/goblin/pull/253
### Fixed
  pe: pass parse opts correctly in pe parser in lookup table, fixes some issues loading and parsing pe libraries: https://github.com/m4b/goblin/pull/268
  elf: remove unnecessary unsafe blocks, thanks @nico-abram: https://github.com/m4b/goblin/pull/261
  elf: replace pub type with pub use, thanks @sollyucko: https://github.com/m4b/goblin/pull/259
### Added
  elf: add a lazy parse example, thanks @jesseui: https://github.com/m4b/goblin/pull/258
  elf: add a new fuzzing harness + fix overflows in hash functions and note data iterator construction, thanks @Mrmaxmeier: https://github.com/m4b/goblin/pull/260

## [0.3.4] - 2021-1-31
### Added
- elf: introduce "lazy" parsing of elf structure with new lazy_parse function, which allows user to fill in parts of the ELF struct they need later on; new example provided, as well as some tests, thanks @jessehui: https://github.com/m4b/goblin/pull/254
- elf: also add new `Elf::parse_header` convenience function, which allows to parse elf header from bytes without e.g., explicitly depending on scroll, etc.

## [0.3.3] - 2021-1-31
### Fixed
- mach: fix debug print panic, thanks @messense: https://github.com/m4b/goblin/pull/251
### Added
- pe: allow pe virtual memory resolve to be optional, allowing memory/process dump parsing, thanks @ko1n (as well as patience for very long time to merge PR!): https://github.com/m4b/goblin/pull/188

## [0.3.2] - 2021-1-29
### Fixed
- elf: overflow panic when note name is 0, thanks @glandium: https://github.com/m4b/goblin/pull/256

## [0.3.1] - 2021-1-18
### Added
- mach: add rpaths, thanks @keith: https://github.com/m4b/goblin/pull/248
### Fixed
- elf: fix regression parsing binaries like busybox (https://github.com/m4b/bingrep/issues/28), thanks @jan-auer: https://github.com/m4b/goblin/pull/249

## [0.3.0] - 2020-11-26
### BREAKING
- mach: add missing load commands, and fixup minversion enum and api, thanks @woodruffw !: https://github.com/m4b/goblin/pull/240
### Fixed
- elf: prevent overflow in bad section sizes, thanks @jackcmay: https://github.com/m4b/goblin/pull/243
- `Object::parse` no longer needs `std`! thanks @Evian-Zhang: https://github.com/m4b/goblin/pull/235
-  test: remove hardcoded CommandLineTools path in macos test, thanks @quake: https://github.com/m4b/goblin/pull/238
- build: Resolve clippy lints, thanks @connorkuehl: https://github.com/m4b/goblin/pull/225
### Added
- elf: add the x86-64 unwind processor specific section header type https://github.com/m4b/goblin/pull/224
- elf: Add ability to get archive members by index https://github.com/m4b/goblin/pull/225

## [0.2.3] - 2020-5-10
### Fixed
- pe: remove unwrap on coffheader strtab parsing, thanks @ExPixel: https://github.com/m4b/goblin/pull/222
### Added
- pe: add more machine constants, thanks @ExPixel: https://github.com/m4b/goblin/pull/223

## [0.2.2] - 2020-5-08
### Fixed
- elf: protect against out of memory when parsing, thanks @jackcmay: https://github.com/m4b/goblin/pull/219
- pe: fix panic when parsing unwind info, thanks @jan-auer: https://github.com/m4b/goblin/pull/218

## [0.2.1] - 2020-3-14
### Added
- elf: add more robust debug printing to various elf data structures, thanks @connorkuehl, e.g.: https://github.com/m4b/goblin/pull/211
- elf: derive PartialEq for DynamicInfo, thanks @connorkuehl: https://github.com/m4b/goblin/pull/209

## [0.2.0] - 2020-1-20
### Changed
- BREAKING: Changes in `elf::gnu_hash::GnuHash`:
  + `new(*const u32, usize, &[sym::Sym]) -> Self`
    to `from_raw_table(&[u8], &[Sym]) -> Result<Self, &str>`
  + `find(&self, &str, u32, &Strtab) -> Option<&Sym>`
    to `find(&self, &str, &Strtab) -> Option<&Sym>`.
- BREAKING: mach: fix generic relocation constants, @philipc: https://github.com/m4b/goblin/pull/204/files
### Added
- elf: add more elf note values, thanks @xcoldhandsx: https://github.com/m4b/goblin/pull/201
- Finally rustfmt'd entire repo :D

## [0.1.3] - 2019-12-28
### Removed
- alloc feature, stabilized in 1.36 @philipc https://github.com/m4b/goblin/pull/196
### Added
elf: support empty PT_DYNAMIC references, @jan-auer https://github.com/m4b/goblin/pull/193
elf: move various elf::Sym impls out of alloc gate, @lzutao https://github.com/m4b/goblin/pull/198
### Fixed
elf: parsing 0 section header had regression introduced in 779d0ce, fixed by @philipc https://github.com/m4b/goblin/pull/200

## [0.1.2] - 2019-12-02
### Fixed
mach: don't return data for zerofill sections, @philipc https://github.com/m4b/goblin/pull/195

## [0.1.1] - 2019-11-10
### Fixed
elf: Don't fail entire elf parse when interpreter is malformed string, @jsgf https://github.com/m4b/goblin/pull/192

## [0.1.0] - 2019-11-3
### Added
- update to scroll 0.10 api

### Changed
- BREAKING: rename export to lib in Reexport::DLLOrdinal from @lzybkr
- pe: only parse ExceptionData for machine X86_64, thanks @wyxloading

### Fixed
pe: Fix resolution of redirect unwind info, thanks @jan-auer https://github.com/m4b/goblin/pull/183
pe: fix reexport dll and ordinal, thanks @lzybkr: d62889f469846af0cceb789b415f1e14f5f9e402

## [0.0.24] - 2019-7-13
### Added
- archive: new public enum type to determine which kind of archive was parsed
### Fixed
- archive: thanks @raindev
    * fix parsing of windows style archives: https://github.com/m4b/goblin/pull/174
    * stricter parsing of archives with multiple indexes: https://github.com/m4b/goblin/pull/175

## [0.0.23] - 2019-6-30
### Added
- pe: add write support for COFF object files!!! This is huge; we now support at a basic level writing out all major binary object formats, thanks @philipc: https://github.com/m4b/goblin/pull/159
- elf: add more e_ident constants
- mach: add segment protection constants
- elf: add risc-v relocation constants
- elf: add constants for arm64_32 (ILP32 ABI on 64-bit arm)
- pe: coff relocations and other auxiliary symbol records

### Fixed
- mach: fix 0 length data sections in mach-o segments, seen in some object files, thanks @raindev: https://github.com/m4b/goblin/pull/172
- build: alloc build was fixed: https://github.com/m4b/goblin/pull/170
- pe: fix `set_name_offset` compilation for 32-bit: https://github.com/m4b/goblin/pull/163

## [0.0.22] - 2019-4-13
### Added
- Beautify debugging by using `debug_struct` in `Debug` implementation of many structs.
- PE: fix rva mask, thanks @wickawacka: https://github.com/m4b/goblin/pull/152
- PE: add PE exception tables, thanks @jan-auer: https://github.com/m4b/goblin/pull/136

### Changed
- Bump lowest Rust version to 1.31.1 and transition project to Rust 2018 edition.
- BREAKING: Rename module `goblin::elf::dyn` to `goblin::elf::dynamic` due to `dyn`
  become a keyword in Rust 2018 edition.
- BREAKING: Rename `mach::exports::SymbolKind::to_str(kind: SymbolKind)` -> `to_str(&self)`.
- BREAKING: Rename `strtab::Strtab::to_vec(self)` -> `to_vec(&self).`

### Removed
- BREAKING: `goblin::error::Error::description` would be removed. Use `to_string()` method instead.

### Fixed
- elf: handle some invalid sizes, thanks @philipc: https://github.com/m4b/goblin/pull/121

## [0.0.21] - 2019-2-21
### Added
- elf: add symbol visibility. thanks @pchickey: https://github.com/m4b/goblin/pull/119

## [0.0.20] - 2019-2-10
### Added
- elf: parse section header relocs even when not an object file. thanks @Techno-Coder: https://github.com/m4b/goblin/pull/118
- pe: make utils public, add better examples for data directory usage. thanks @Pzixel: https://github.com/m4b/goblin/pull/116

## [0.0.19] - 2018-10-23
### Added
- elf: fix regression when parsing dynamic symbols from some binaries, thanks @philipc: https://github.com/m4b/goblin/issues/111

## [0.0.18] - 2018-10-14
### Changed
 - BREAKING: updated required compiler to 1.20 (due to scroll 1.20 requirement)
 - BREAKING: elf: removed bias field, as it was misleading/useless/incorrect
 - BREAKING: elf: add lazy relocation iterators: Thanks @ibabushkin https://github.com/m4b/goblin/pull/102
 - BREAKING: mach: remove repr(packed) from dylib and fvmlib (this should not affect anyone): https://github.com/m4b/goblin/issues/105
### Added
 - elf: use gnu/sysv hash table to compute sizeof dynsyms more accurately: again _huge_ thanks to @philipc https://github.com/m4b/goblin/pull/109
 - elf: handle multiple load biases: _huge_ thanks @philipc: https://github.com/m4b/goblin/pull/107
 - mach: add arm64e constants: Thanks @mitsuhiko https://github.com/m4b/goblin/pull/103
 - PE: calculate read bytes using alignment: Thanks @tathanhdinh https://github.com/m4b/goblin/pull/101
 - PE: get proper names for PE sections: Thanks @roblabla https://github.com/m4b/goblin/pull/100

## [0.0.17] - 2018-7-16
### Changed
 - BREAKING: updated required compiler to 1.19 (technically only required for tests, but assume this is required for building as well)
 - fixed nightly alloc api issues: https://github.com/m4b/goblin/issues/94

## [0.0.16] - 2018-7-14
### Changed
 - BREAKING: pe.export: name is now optional to reflect realities of PE parsing, and add more robustness to parser. many thanks to @tathanhdinh! https://github.com/m4b/goblin/pull/88
 - elf.note: treat alignment similar to other tools, e.g., readelf. Thanks @xcoldhandsx: https://github.com/m4b/goblin/pull/91
### Added
 - elf: more inline annotations on various methods, thanks@amanieu: https://github.com/m4b/goblin/pull/87

## [0.0.15] - 2018-4-22
### Changed
 - BREAKING: elf.reloc: u64/i64 used for r_offset/r_addend, and addend is now proper optional, thanks @amanieu! https://github.com/m4b/goblin/pull/86/
 - update to scroll 0.9
 - pe32+: parse better, thanks @kjempelodott, https://github.com/m4b/goblin/pull/82
### Added
 - mach: add constants for `n_types` when `N_STAB` field is being used, thanks @jrmuizel! https://github.com/m4b/goblin/pull/85
 - elf: implement support for compressed headers, thanks @rocallahan! https://github.com/m4b/goblin/pull/83
 - new nightly "alloc" feature: allows compiling the goblin parser on nightly with extern crate + no_std, thanks @philipc! https://github.com/m4b/goblin/pull/77
 - mach.segments: do not panic on bad internal data bounds: https://github.com/m4b/goblin/issues/74
 - mach: correctly add weak dylibs to import libs: https://github.com/m4b/goblin/issues/73

## [0.0.14] - 2018-1-15
### Changed
- BREAKING: elf: `iter_notes` renamed to `iter_note_headers`
- BREAKING: mach: remove `is_little_endian()`, `ctx()`, and `container()` methods from header, as they were completely invalid for big-endian architectures since the header was parsed according to the endianness of the binary correctly into memory, and hence would always report `MH_MAGIC` or `MH_MAGIC64` as the magic value.
- elf: courtesy of @jan-auer, note iterator now properly iterates over multiple PH_NOTEs
### Added
- mach: added hotly requested feature - goblin now has new functionality to parse big-endian, powerpc 32-bit mach-o binaries correctly
- mach: new function to correctly extract the parsing context for a mach-o binary, `parse_magic_and_ctx`
- elf: note iterator has new `iter_note_sections` method

## [0.0.13] - 2017-12-10
### Changed
- BREAKING: remove deprecated goblin::parse method
- BREAKING: ELF `to_range` removed on program and section headers; use `vm_range` and `file_range` for respective ranges
- Technically BREAKING: @philipc added Symtab and symbol iterator to ELF, but is basically the same, unless you were explicitly relying on the backing vector
- use scroll 0.8.0 and us scroll_derive via scroll
- fix notes including \0 terminator (causes breakage because tools like grep treat resulting output as a binary output...)
### Added
- pe: add PE characteristics constants courtesy @philipc
- mach: SizeWith for RelocationInfo
- mach: IOWrite and Pwrite impls for Nlist

## [0.0.12] - 2017-10-29
### Changed
- fix proper std feature flag to log; this was an oversight in last version
- proper cputype and cpusubtype constants to mach, along with mappings, courtesy of @mitsuhiko
- new osx and ios version constants
- all mach load commands now implement IOread and IOwrite from scroll
- add new elf::note module and associated structs + constants, and `iter_notes` method to Elf object
- remove all unused muts; this will make nightly and future stables no longer warn

### Added
- fix macho nstab treatment, thanks @philipc !
- mach header cpusubtype bug fixed, thanks @mitsuhiko !

## [0.0.11] - 2017-08-24
### Added
- goblin::Object::parse; add deprecation to goblin::parse
- MAJOR archive now parses bsd style archives AND is zero-copy by @willglynn
- MAJOR macho import parser bug fixed by @willglynn
- added writer impls for Section and Segment
- add get_unsafe to strtab for Option<&str> returns
- relocations method on mach
- more elf relocations
- mach relocations
- convenience functions for many elf structures that elf writer will appreciate
- mach relocation iteration
- update to scroll 0.7
- add cread/ioread impls for various structs

### Changed
- BREAKING: sections() and section iterator now return (Section, &[u8])
- Segment, Section, RelocationIterator are now in segment module
- removed lifetime from section, removed data and raw data, and embedded ctx
- all scroll::Error have been removed from public API ref #33
- better mach symbol iteration
- better mach section iteration
- remove wow_so_meta_doge due to linker issues
- Strtab.get now returns a Option<Result>, when index is bad
- elf.soname is &str
- elf.libraries is now Vec<&str>

## [0.0.10] - 2017-05-09
### Added
- New goblin::Object for enum containing the parsed binary container, or convenience goblin::parse(&[u8) for parsing bytes into respective container format
### Changed
- All binaries formats now have lifetimes
- Elf has a lifetime
- Strtab.new now requires a &'a[u8]
- Strtab.get now returns a scroll::Result<&'a str> (use strtab[index] if you want old behavior and don't care about panics); returning scroll::Error is a bug, fixed in next release

## [0.0.9] - 2017-04-05
### Changed
- Archive has a lifetime
- Mach has a lifetime
