# rust-embed-resource [![AppVeyorCI build status](https://ci.appveyor.com/api/projects/status/nqd8kaa2pgwyiqkk/branch/master?svg=true)](https://ci.appveyor.com/project/nabijaczleweli/rust-embed-resource/branch/master) [![Licence](https://img.shields.io/badge/license-MIT-blue.svg?style=flat)](LICENSE) [![Crates.io version](https://img.shields.io/crates/v/embed-resource)](https://crates.io/crates/embed-resource)
A [`Cargo` build script](https://doc.rust-lang.org/cargo/reference/build-scripts.html) library to handle compilation and inclusion of Windows resources
in the most resilient fashion imaginable

## [Documentation](https://docs.rs/embed-resource)

## Quickstart

In your build script, assuming the resource file is called `checksums.rc`:

```rust
extern crate embed_resource;

fn main() {
    // Compile and link checksums.rc
    embed_resource::compile("checksums.rc", embed_resource::NONE).manifest_optional().unwrap();

    // Or, to select a resource file for each binary separately
    embed_resource::compile_for("assets/poke-a-mango.rc", &["poke-a-mango", "poke-a-mango-installer"], &["VERSION=\"0.5.0\""]).manifest_required().unwrap();
    embed_resource::compile_for("assets/uninstaller.rc", &["unins001"], embed_resource::NONE).manifest_required().unwrap();
}
```

Use `.manifest_optional().unwrap()` if the manifest is cosmetic (like an icon).<br />
Use `.manifest_required().unwrap()` if the manifest is required (security, entry point, &c.).

Parameters that look like `&["string"]` or `embed_resource::NONE` in the example above
can be anything that satisfies `IntoIterator<AsRef<OsStr>>`:
`&[&str]`, of course, but also `Option<PathBuf>`, `Vec<OsString>`, &c.

## Example: Embedding a Windows Manifest
Courtesy of [@jpoles1](https://github.com/jpoles1).

The following steps are used to embed a manifest in your compiled rust .exe file. In this example the manifest will cause admin permissions to be requested for the final executable:

1. Add the following to your cargo.toml:
```toml
[build-dependencies]
embed-resource = "3.0"
```

2. In your project root directory, add a file named `build.rs` with the following:
```rust
extern crate embed_resource;
fn main() {
    embed_resource::compile("app-name-manifest.rc", embed_resource::NONE).manifest_optional().unwrap();
}
```

3. In your project root directory, add a file named `app-name-manifest.rc` with the following:
```c
#define RT_MANIFEST 24
1 RT_MANIFEST "app-name.exe.manifest"
```

4. In your project root directory, add a file named `app-name.exe.manifest` with the following:
```xml
<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<assembly xmlns="urn:schemas-microsoft-com:asm.v1" manifestVersion="1.0">
    <trustInfo xmlns="urn:schemas-microsoft-com:asm.v3">
        <security>
            <requestedPrivileges>
                <requestedExecutionLevel level="requireAdministrator" uiAccess="false"/>
            </requestedPrivileges>
        </security>
    </trustInfo>
</assembly>
```

5. Build your project!

## Errata

If no `cargo:rerun-if-changed` annotations are generated, Cargo scans the entire build root by default.
Because the first step in building a manifest is an unspecified C preprocessor step with-out the ability to generate the equivalent of `cc -MD`, we do *not* output said annotation.

If scanning is prohibitively expensive, or you have something else that generates the annotations, you may want to spec the full non-system dependency list for your manifest manually, so:
```rust
println!("cargo:rerun-if-changed=app-name-manifest.rc");
embed_resource::compile("app-name-manifest.rc", embed_resource::NONE).manifest_optional().unwrap();
```
for the above example (cf. [#41](https://github.com/nabijaczleweli/rust-embed-resource/issues/41)).

## Old releases with backports
`v1.6-stable` continues after 1.6.6 broke library-only crates, then 1.7.0 introduced a new interface. 1.6.6 was yanked, 1.6.7 fixed this.<br />
`v2.5-stable` continues after it turned out that builds have been universally broken on Win32 below `\?\\` paths. 3.0.1 and 2.5.1 (and 1.6.14) fixed this.

These both receive backports of all fixes that affect them. You should still probably update.<br />
The default branch is stable, and currently has 3.x.

## Migration
### 2.x

Add `embed_resource::NONE` as the last argument to `embed_resource::compile()` and  `embed_resource::compile_for()`.

### 3.x

Add `.manifest_optional().unwrap()` or `.manifest_required().unwrap()` to all `embed_resource::compile()` and `embed_resource::compile_for*()` calls.
`CompilationResult` is `#[must_use]` so should be highlighted automatically.

Embed-resource <3.x always behaves like `.manifest_optional().unwrap()`.

## Credit

In chronological order:

[@liigo](https://github.com/liigo) -- persistency in pestering me and investigating problems where I have failed

[@mzji](https://github.com/mzji) -- MSVC lab rat

[@TheCatPlusPlus](https://github.com/TheCatPlusPlus) -- knowledge and providing first iteration of manifest-embedding code

[@azyobuzin](https://github.com/azyobuzin) -- providing code for finding places where RC.EXE could hide

[@retep998](https://github.com/retep998) -- fixing MSVC support

[@SonnyX](https://github.com/SonnyX) -- Windows cross-compilation support and testing

[@MSxDOS](https://github.com/MSxDOS) -- finding and supplying RC.EXE its esoteric header include paths

[@roblabla](https://github.com/roblabla) -- cross-compilation to Windows MSVC via LLVM-RC

## Special thanks

To all who support further development on Patreon, in particular:

  * ThePhD
  * Embark Studios
  * Lars Strojny
  * EvModder
