use crate::{
    ExtensionLibraryKind, ExtensionManifest, GrammarManifestEntry, build_debug_adapter_schema_path,
    parse_wasm_extension_version,
};
use ::fs::Fs;
use anyhow::{Context as _, Result, bail};
use futures::{AsyncReadExt, StreamExt};
use heck::ToSnakeCase;
use http_client::{self, AsyncBody, HttpClient};
use serde::Deserialize;
use std::{
    env, fs, mem,
    path::{Path, PathBuf},
    process::Stdio,
    str::FromStr,
    sync::Arc,
};
use wasm_encoder::{ComponentSectionId, Encode as _, RawSection, Section as _};
use wasmparser::Parser;

/// Currently, we compile with Rust's `wasm32-wasip2` target, which works with WASI `preview2` and the component model.
const RUST_TARGET: &str = "wasm32-wasip2";

/// Compiling Tree-sitter parsers from C to WASM requires Clang 17, and a WASM build of libc
/// and clang's runtime library. The `wasi-sdk` provides these binaries.
///
/// Once Clang 17 and its wasm target are available via system package managers, we won't need
/// to download this.
const WASI_SDK_URL: &str = "https://github.com/WebAssembly/wasi-sdk/releases/download/wasi-sdk-25/";
const WASI_SDK_ASSET_NAME: Option<&str> = if cfg!(all(target_os = "macos", target_arch = "x86_64"))
{
    Some("wasi-sdk-25.0-x86_64-macos.tar.gz")
} else if cfg!(all(target_os = "macos", target_arch = "aarch64")) {
    Some("wasi-sdk-25.0-arm64-macos.tar.gz")
} else if cfg!(all(target_os = "linux", target_arch = "x86_64")) {
    Some("wasi-sdk-25.0-x86_64-linux.tar.gz")
} else if cfg!(all(target_os = "linux", target_arch = "aarch64")) {
    Some("wasi-sdk-25.0-arm64-linux.tar.gz")
} else if cfg!(all(target_os = "freebsd", target_arch = "x86_64")) {
    Some("wasi-sdk-25.0-x86_64-linux.tar.gz")
} else if cfg!(all(target_os = "freebsd", target_arch = "aarch64")) {
    Some("wasi-sdk-25.0-arm64-linux.tar.gz")
} else if cfg!(all(target_os = "windows", target_arch = "x86_64")) {
    Some("wasi-sdk-25.0-x86_64-windows.tar.gz")
} else {
    None
};

pub struct ExtensionBuilder {
    cache_dir: PathBuf,
    pub http: Arc<dyn HttpClient>,
}

pub struct CompileExtensionOptions {
    pub release: bool,
}

impl CompileExtensionOptions {
    pub const fn dev() -> Self {
        Self { release: false }
    }
}

#[derive(Deserialize)]
struct CargoToml {
    package: CargoTomlPackage,
}

#[derive(Deserialize)]
struct CargoTomlPackage {
    name: String,
}

impl ExtensionBuilder {
    pub fn new(http_client: Arc<dyn HttpClient>, cache_dir: PathBuf) -> Self {
        Self {
            cache_dir,
            http: http_client,
        }
    }

    pub async fn compile_extension(
        &self,
        extension_dir: &Path,
        extension_manifest: &mut ExtensionManifest,
        options: CompileExtensionOptions,
        fs: Arc<dyn Fs>,
    ) -> Result<()> {
        populate_defaults(extension_manifest, extension_dir, fs).await?;

        if extension_dir.is_relative() {
            bail!(
                "extension dir {} is not an absolute path",
                extension_dir.display()
            );
        }

        fs::create_dir_all(&self.cache_dir).context("failed to create cache dir")?;

        if extension_manifest.lib.kind == Some(ExtensionLibraryKind::Rust) {
            log::info!("compiling Rust extension {}", extension_dir.display());
            self.compile_rust_extension(extension_dir, extension_manifest, options)
                .await
                .context("failed to compile Rust extension")?;
            log::info!("compiled Rust extension {}", extension_dir.display());
        }

        for (debug_adapter_name, meta) in &mut extension_manifest.debug_adapters {
            let debug_adapter_schema_path =
                extension_dir.join(build_debug_adapter_schema_path(debug_adapter_name, meta));

            let debug_adapter_schema = fs::read_to_string(&debug_adapter_schema_path)
                .with_context(|| {
                    format!("failed to read debug adapter schema for `{debug_adapter_name}` from `{debug_adapter_schema_path:?}`")
                })?;
            _ = serde_json::Value::from_str(&debug_adapter_schema).with_context(|| {
                format!("Debug adapter schema for `{debug_adapter_name}` (path: `{debug_adapter_schema_path:?}`) is not a valid JSON")
            })?;
        }
        for (grammar_name, grammar_metadata) in &extension_manifest.grammars {
            let snake_cased_grammar_name = grammar_name.to_snake_case();
            if grammar_name.as_ref() != snake_cased_grammar_name.as_str() {
                bail!(
                    "grammar name '{grammar_name}' must be written in snake_case: {snake_cased_grammar_name}"
                );
            }

            log::info!(
                "compiling grammar {grammar_name} for extension {}",
                extension_dir.display()
            );
            self.compile_grammar(extension_dir, grammar_name.as_ref(), grammar_metadata)
                .await
                .with_context(|| format!("failed to compile grammar '{grammar_name}'"))?;
            log::info!(
                "compiled grammar {grammar_name} for extension {}",
                extension_dir.display()
            );
        }

        log::info!("finished compiling extension {}", extension_dir.display());
        Ok(())
    }

    async fn compile_rust_extension(
        &self,
        extension_dir: &Path,
        manifest: &mut ExtensionManifest,
        options: CompileExtensionOptions,
    ) -> anyhow::Result<()> {
        self.install_rust_wasm_target_if_needed().await?;

        let cargo_toml_content = fs::read_to_string(extension_dir.join("Cargo.toml"))?;
        let cargo_toml: CargoToml = toml::from_str(&cargo_toml_content)?;

        log::info!(
            "compiling Rust crate for extension {}",
            extension_dir.display()
        );
        let output = util::command::new_smol_command("cargo")
            .args(["build", "--target", RUST_TARGET])
            .args(options.release.then_some("--release"))
            .arg("--target-dir")
            .arg(extension_dir.join("target"))
            // WASI builds do not work with sccache and just stuck, so disable it.
            .env("RUSTC_WRAPPER", "")
            .current_dir(extension_dir)
            .output()
            .await
            .context("failed to run `cargo`")?;
        if !output.status.success() {
            bail!(
                "failed to build extension {}",
                String::from_utf8_lossy(&output.stderr)
            );
        }

        log::info!(
            "compiled Rust crate for extension {}",
            extension_dir.display()
        );

        let mut wasm_path = PathBuf::from(extension_dir);
        wasm_path.extend([
            "target",
            RUST_TARGET,
            if options.release { "release" } else { "debug" },
            &cargo_toml
                .package
                .name
                // The wasm32-wasip2 target normalizes `-` in package names to `_` in the resulting `.wasm` file.
                .replace('-', "_"),
        ]);
        wasm_path.set_extension("wasm");

        log::info!(
            "encoding wasm component for extension {}",
            extension_dir.display()
        );

        let component_bytes = fs::read(&wasm_path)
            .with_context(|| format!("failed to read output module `{}`", wasm_path.display()))?;

        let component_bytes = self
            .strip_custom_sections(&component_bytes)
            .context("failed to strip debug sections from wasm component")?;

        let wasm_extension_api_version =
            parse_wasm_extension_version(&manifest.id, &component_bytes)
                .context("compiled wasm did not contain a valid zed extension api version")?;
        manifest.lib.version = Some(wasm_extension_api_version);

        let extension_file = extension_dir.join("extension.wasm");
        fs::write(extension_file.clone(), &component_bytes)
            .context("failed to write extension.wasm")?;

        log::info!(
            "extension {} written to {}",
            extension_dir.display(),
            extension_file.display()
        );

        Ok(())
    }

    async fn compile_grammar(
        &self,
        extension_dir: &Path,
        grammar_name: &str,
        grammar_metadata: &GrammarManifestEntry,
    ) -> Result<()> {
        let clang_path = self.install_wasi_sdk_if_needed().await?;

        let mut grammar_repo_dir = extension_dir.to_path_buf();
        grammar_repo_dir.extend(["grammars", grammar_name]);

        let mut grammar_wasm_path = grammar_repo_dir.clone();
        grammar_wasm_path.set_extension("wasm");

        log::info!("checking out {grammar_name} parser");
        self.checkout_repo(
            &grammar_repo_dir,
            &grammar_metadata.repository,
            &grammar_metadata.rev,
        )
        .await?;

        let base_grammar_path = grammar_metadata
            .path
            .as_ref()
            .map(|path| grammar_repo_dir.join(path))
            .unwrap_or(grammar_repo_dir);

        let src_path = base_grammar_path.join("src");
        let parser_path = src_path.join("parser.c");
        let scanner_path = src_path.join("scanner.c");

        // Skip recompiling if the WASM object is already newer than the source files
        if file_newer_than_deps(&grammar_wasm_path, &[&parser_path, &scanner_path]).unwrap_or(false)
        {
            log::info!(
                "skipping compilation of {grammar_name} parser because the existing compiled grammar is up to date"
            );
        } else {
            log::info!("compiling {grammar_name} parser");
            let clang_output = util::command::new_smol_command(&clang_path)
                .args(["-fPIC", "-shared", "-Os"])
                .arg(format!("-Wl,--export=tree_sitter_{grammar_name}"))
                .arg("-o")
                .arg(&grammar_wasm_path)
                .arg("-I")
                .arg(&src_path)
                .arg(&parser_path)
                .args(scanner_path.exists().then_some(scanner_path))
                .output()
                .await
                .context("failed to run clang")?;

            if !clang_output.status.success() {
                bail!(
                    "failed to compile {} parser with clang: {}",
                    grammar_name,
                    String::from_utf8_lossy(&clang_output.stderr),
                );
            }
        }

        Ok(())
    }

    async fn checkout_repo(&self, directory: &Path, url: &str, rev: &str) -> Result<()> {
        let git_dir = directory.join(".git");

        if directory.exists() {
            let remotes_output = util::command::new_smol_command("git")
                .arg("--git-dir")
                .arg(&git_dir)
                .args(["remote", "-v"])
                .output()
                .await?;
            let has_remote = remotes_output.status.success()
                && String::from_utf8_lossy(&remotes_output.stdout)
                    .lines()
                    .any(|line| {
                        let mut parts = line.split(|c: char| c.is_whitespace());
                        parts.next() == Some("origin") && parts.any(|part| part == url)
                    });
            if !has_remote {
                bail!(
                    "grammar directory '{}' already exists, but is not a git clone of '{}'",
                    directory.display(),
                    url
                );
            }
        } else {
            fs::create_dir_all(directory).with_context(|| {
                format!("failed to create grammar directory {}", directory.display(),)
            })?;
            let init_output = util::command::new_smol_command("git")
                .arg("init")
                .current_dir(directory)
                .output()
                .await?;
            if !init_output.status.success() {
                bail!(
                    "failed to run `git init` in directory '{}'",
                    directory.display()
                );
            }

            let remote_add_output = util::command::new_smol_command("git")
                .arg("--git-dir")
                .arg(&git_dir)
                .args(["remote", "add", "origin", url])
                .output()
                .await
                .context("failed to execute `git remote add`")?;
            if !remote_add_output.status.success() {
                bail!(
                    "failed to add remote {url} for git repository {}",
                    git_dir.display()
                );
            }
        }

        let fetch_output = util::command::new_smol_command("git")
            .arg("--git-dir")
            .arg(&git_dir)
            .args(["fetch", "--depth", "1", "origin", rev])
            .output()
            .await
            .context("failed to execute `git fetch`")?;

        let checkout_output = util::command::new_smol_command("git")
            .arg("--git-dir")
            .arg(&git_dir)
            .args(["checkout", rev])
            .current_dir(directory)
            .output()
            .await
            .context("failed to execute `git checkout`")?;
        if !checkout_output.status.success() {
            if !fetch_output.status.success() {
                bail!(
                    "failed to fetch revision {} in directory '{}'",
                    rev,
                    directory.display()
                );
            }
            bail!(
                "failed to checkout revision {} in directory '{}': {}",
                rev,
                directory.display(),
                String::from_utf8_lossy(&checkout_output.stderr)
            );
        }

        Ok(())
    }

    async fn install_rust_wasm_target_if_needed(&self) -> Result<()> {
        let rustc_output = util::command::new_smol_command("rustc")
            .arg("--print")
            .arg("sysroot")
            .output()
            .await
            .context("failed to run rustc")?;
        if !rustc_output.status.success() {
            bail!(
                "failed to retrieve rust sysroot: {}",
                String::from_utf8_lossy(&rustc_output.stderr)
            );
        }

        let sysroot = PathBuf::from(String::from_utf8(rustc_output.stdout)?.trim());
        if sysroot.join("lib/rustlib").join(RUST_TARGET).exists() {
            return Ok(());
        }

        let output = util::command::new_smol_command("rustup")
            .args(["target", "add", RUST_TARGET])
            .stderr(Stdio::piped())
            .stdout(Stdio::inherit())
            .output()
            .await
            .context("failed to run `rustup target add`")?;
        if !output.status.success() {
            bail!(
                "failed to install the `{RUST_TARGET}` target: {}",
                String::from_utf8_lossy(&rustc_output.stderr)
            );
        }

        Ok(())
    }

    async fn install_wasi_sdk_if_needed(&self) -> Result<PathBuf> {
        let url = if let Some(asset_name) = WASI_SDK_ASSET_NAME {
            format!("{WASI_SDK_URL}{asset_name}")
        } else {
            bail!("wasi-sdk is not available for platform {}", env::consts::OS);
        };

        let wasi_sdk_dir = self.cache_dir.join("wasi-sdk");
        let mut clang_path = wasi_sdk_dir.clone();
        clang_path.extend(["bin", &format!("clang{}", env::consts::EXE_SUFFIX)]);

        log::info!("downloading wasi-sdk to {}", wasi_sdk_dir.display());

        if fs::metadata(&clang_path).is_ok_and(|metadata| metadata.is_file()) {
            return Ok(clang_path);
        }

        let tar_out_dir = self.cache_dir.join("wasi-sdk-temp");

        fs::remove_dir_all(&wasi_sdk_dir).ok();
        fs::remove_dir_all(&tar_out_dir).ok();
        fs::create_dir_all(&tar_out_dir).context("failed to create extraction directory")?;

        let mut response = self.http.get(&url, AsyncBody::default(), true).await?;

        // Write the response to a temporary file
        let tar_gz_path = self.cache_dir.join("wasi-sdk.tar.gz");
        let mut tar_gz_file =
            fs::File::create(&tar_gz_path).context("failed to create temporary tar.gz file")?;
        let response_body = response.body_mut();
        let mut body_bytes = Vec::new();
        response_body.read_to_end(&mut body_bytes).await?;
        std::io::Write::write_all(&mut tar_gz_file, &body_bytes)?;
        drop(tar_gz_file);

        log::info!("un-tarring wasi-sdk to {}", tar_out_dir.display());

        // Shell out to tar to extract the archive
        let tar_output = util::command::new_smol_command("tar")
            .arg("-xzf")
            .arg(&tar_gz_path)
            .arg("-C")
            .arg(&tar_out_dir)
            .output()
            .await
            .context("failed to run tar")?;

        if !tar_output.status.success() {
            bail!(
                "failed to extract wasi-sdk archive: {}",
                String::from_utf8_lossy(&tar_output.stderr)
            );
        }

        log::info!("finished downloading wasi-sdk");

        // Clean up the temporary tar.gz file
        fs::remove_file(&tar_gz_path).ok();

        let inner_dir = fs::read_dir(&tar_out_dir)?
            .next()
            .context("no content")?
            .context("failed to read contents of extracted wasi archive directory")?
            .path();
        fs::rename(&inner_dir, &wasi_sdk_dir).context("failed to move extracted wasi dir")?;
        fs::remove_dir_all(&tar_out_dir).ok();

        Ok(clang_path)
    }

    // This was adapted from:
    // https://github.com/bytecodealliance/wasm-tools/blob/e8809bb17fcf69aa8c85cd5e6db7cff5cf36b1de/src/bin/wasm-tools/strip.rs
    fn strip_custom_sections(&self, input: &Vec<u8>) -> Result<Vec<u8>> {
        use wasmparser::Payload::*;

        let strip_custom_section = |name: &str| {
            // Default strip everything but:
            // * the `name` section
            // * any `component-type` sections
            // * the `dylink.0` section
            // * our custom version section
            name != "name"
                && !name.starts_with("component-type:")
                && name != "dylink.0"
                && name != "zed:api-version"
        };

        let mut output = Vec::new();
        let mut stack = Vec::new();

        for payload in Parser::new(0).parse_all(input) {
            let payload = payload?;

            // Track nesting depth, so that we don't mess with inner producer sections:
            match payload {
                Version { encoding, .. } => {
                    output.extend_from_slice(match encoding {
                        wasmparser::Encoding::Component => &wasm_encoder::Component::HEADER,
                        wasmparser::Encoding::Module => &wasm_encoder::Module::HEADER,
                    });
                }
                ModuleSection { .. } | ComponentSection { .. } => {
                    stack.push(mem::take(&mut output));
                    continue;
                }
                End { .. } => {
                    let mut parent = match stack.pop() {
                        Some(c) => c,
                        None => break,
                    };
                    if output.starts_with(&wasm_encoder::Component::HEADER) {
                        parent.push(ComponentSectionId::Component as u8);
                        output.encode(&mut parent);
                    } else {
                        parent.push(ComponentSectionId::CoreModule as u8);
                        output.encode(&mut parent);
                    }
                    output = parent;
                }
                _ => {}
            }

            if let CustomSection(c) = &payload
                && strip_custom_section(c.name())
            {
                continue;
            }
            if let Some((id, range)) = payload.as_section() {
                RawSection {
                    id,
                    data: &input[range],
                }
                .append_to(&mut output);
            }
        }

        Ok(output)
    }
}

async fn populate_defaults(
    manifest: &mut ExtensionManifest,
    extension_path: &Path,
    fs: Arc<dyn Fs>,
) -> Result<()> {
    // For legacy extensions on the v0 schema (aka, using `extension.json`), clear out any existing
    // contents of the computed fields, since we don't care what the existing values are.
    if manifest.schema_version.is_v0() {
        manifest.languages.clear();
        manifest.grammars.clear();
        manifest.themes.clear();
    }

    let cargo_toml_path = extension_path.join("Cargo.toml");
    if cargo_toml_path.exists() {
        manifest.lib.kind = Some(ExtensionLibraryKind::Rust);
    }

    let languages_dir = extension_path.join("languages");
    if fs.is_dir(&languages_dir).await {
        let mut language_dir_entries = fs
            .read_dir(&languages_dir)
            .await
            .context("failed to list languages dir")?;

        while let Some(language_dir) = language_dir_entries.next().await {
            let language_dir = language_dir?;
            let config_path = language_dir.join("config.toml");
            if fs.is_file(config_path.as_path()).await {
                let relative_language_dir =
                    language_dir.strip_prefix(extension_path)?.to_path_buf();
                if !manifest.languages.contains(&relative_language_dir) {
                    manifest.languages.push(relative_language_dir);
                }
            }
        }
    }

    let themes_dir = extension_path.join("themes");
    if fs.is_dir(&themes_dir).await {
        let mut theme_dir_entries = fs
            .read_dir(&themes_dir)
            .await
            .context("failed to list themes dir")?;

        while let Some(theme_path) = theme_dir_entries.next().await {
            let theme_path = theme_path?;
            if theme_path.extension() == Some("json".as_ref()) {
                let relative_theme_path = theme_path.strip_prefix(extension_path)?.to_path_buf();
                if !manifest.themes.contains(&relative_theme_path) {
                    manifest.themes.push(relative_theme_path);
                }
            }
        }
    }

    let icon_themes_dir = extension_path.join("icon_themes");
    if fs.is_dir(&icon_themes_dir).await {
        let mut icon_theme_dir_entries = fs
            .read_dir(&icon_themes_dir)
            .await
            .context("failed to list icon themes dir")?;

        while let Some(icon_theme_path) = icon_theme_dir_entries.next().await {
            let icon_theme_path = icon_theme_path?;
            if icon_theme_path.extension() == Some("json".as_ref()) {
                let relative_icon_theme_path =
                    icon_theme_path.strip_prefix(extension_path)?.to_path_buf();
                if !manifest.icon_themes.contains(&relative_icon_theme_path) {
                    manifest.icon_themes.push(relative_icon_theme_path);
                }
            }
        }
    };
    if manifest.snippets.is_none()
        && let snippets_json_path = extension_path.join("snippets.json")
        && fs.is_file(&snippets_json_path).await
    {
        manifest.snippets = Some("snippets.json".into());
    }

    // For legacy extensions on the v0 schema (aka, using `extension.json`), we want to populate the grammars in
    // the manifest using the contents of the `grammars` directory.
    if manifest.schema_version.is_v0() {
        let grammars_dir = extension_path.join("grammars");
        if fs.is_dir(&grammars_dir).await {
            let mut grammar_dir_entries = fs
                .read_dir(&grammars_dir)
                .await
                .context("failed to list grammars dir")?;

            while let Some(grammar_path) = grammar_dir_entries.next().await {
                let grammar_path = grammar_path?;
                if grammar_path.extension() == Some("toml".as_ref()) {
                    #[derive(Deserialize)]
                    struct GrammarConfigToml {
                        pub repository: String,
                        pub commit: String,
                        #[serde(default)]
                        pub path: Option<String>,
                    }

                    let grammar_config = fs.load(&grammar_path).await?;
                    let grammar_config: GrammarConfigToml = toml::from_str(&grammar_config)?;

                    let grammar_name = grammar_path
                        .file_stem()
                        .and_then(|stem| stem.to_str())
                        .context("no grammar name")?;
                    if !manifest.grammars.contains_key(grammar_name) {
                        manifest.grammars.insert(
                            grammar_name.into(),
                            GrammarManifestEntry {
                                repository: grammar_config.repository,
                                rev: grammar_config.commit,
                                path: grammar_config.path,
                            },
                        );
                    }
                }
            }
        }
    }

    Ok(())
}

/// Returns `true` if the target exists and its last modified time is greater than that
/// of each dependency which exists (i.e., dependency paths which do not exist are ignored).
///
/// # Errors
///
/// Returns `Err` if any of the underlying file I/O operations fail.
fn file_newer_than_deps(target: &Path, dependencies: &[&Path]) -> Result<bool, std::io::Error> {
    if !target.try_exists()? {
        return Ok(false);
    }
    let target_modified = target.metadata()?.modified()?;
    for dependency in dependencies {
        if !dependency.try_exists()? {
            continue;
        }
        let dep_modified = dependency.metadata()?.modified()?;
        if target_modified < dep_modified {
            return Ok(false);
        }
    }
    Ok(true)
}

#[cfg(test)]
mod tests {
    use std::{
        path::{Path, PathBuf},
        str::FromStr,
        thread::sleep,
        time::Duration,
    };

    use gpui::TestAppContext;
    use indoc::indoc;

    use crate::{
        ExtensionManifest,
        extension_builder::{file_newer_than_deps, populate_defaults},
    };

    #[test]
    fn test_file_newer_than_deps() {
        // Don't use TempTree because we need to guarantee the order
        let tmpdir = tempfile::tempdir().unwrap();
        let target = tmpdir.path().join("target.wasm");
        let dep1 = tmpdir.path().join("parser.c");
        let dep2 = tmpdir.path().join("scanner.c");

        assert!(
            !file_newer_than_deps(&target, &[&dep1, &dep2]).unwrap(),
            "target doesn't exist"
        );
        std::fs::write(&target, "foo").unwrap(); // Create target
        assert!(
            file_newer_than_deps(&target, &[&dep1, &dep2]).unwrap(),
            "dependencies don't exist; target is newer"
        );
        sleep(Duration::from_secs(1));
        std::fs::write(&dep1, "foo").unwrap(); // Create dep1 (newer than target)
        // Dependency is newer
        assert!(
            !file_newer_than_deps(&target, &[&dep1, &dep2]).unwrap(),
            "a dependency is newer (target {:?}, dep1 {:?})",
            target.metadata().unwrap().modified().unwrap(),
            dep1.metadata().unwrap().modified().unwrap(),
        );
        sleep(Duration::from_secs(1));
        std::fs::write(&dep2, "foo").unwrap(); // Create dep2
        sleep(Duration::from_secs(1));
        std::fs::write(&target, "foobar").unwrap(); // Update target
        assert!(
            file_newer_than_deps(&target, &[&dep1, &dep2]).unwrap(),
            "target is newer than dependencies (target {:?}, dep2 {:?})",
            target.metadata().unwrap().modified().unwrap(),
            dep2.metadata().unwrap().modified().unwrap(),
        );
    }

    #[gpui::test]
    async fn test_snippet_location_is_kept(cx: &mut TestAppContext) {
        let fs = fs::FakeFs::new(cx.executor());
        let extension_path = Path::new("/extension");

        fs.insert_tree(
            extension_path,
            serde_json::json!({
                "extension.toml": indoc! {r#"
                    id = "test-manifest"
                    name = "Test Manifest"
                    version = "0.0.1"
                    schema_version = 1

                    snippets = "./snippets/snippets.json"
                    "#
                },
                "snippets.json": "",
            }),
        )
        .await;

        let mut manifest = ExtensionManifest::load(fs.clone(), extension_path)
            .await
            .unwrap();

        populate_defaults(&mut manifest, extension_path, fs.clone())
            .await
            .unwrap();

        assert_eq!(
            manifest.snippets,
            Some(PathBuf::from_str("./snippets/snippets.json").unwrap())
        )
    }

    #[gpui::test]
    async fn test_automatic_snippet_location_is_relative(cx: &mut TestAppContext) {
        let fs = fs::FakeFs::new(cx.executor());
        let extension_path = Path::new("/extension");

        fs.insert_tree(
            extension_path,
            serde_json::json!({
                "extension.toml": indoc! {r#"
                    id = "test-manifest"
                    name = "Test Manifest"
                    version = "0.0.1"
                    schema_version = 1

                    "#
                },
                "snippets.json": "",
            }),
        )
        .await;

        let mut manifest = ExtensionManifest::load(fs.clone(), extension_path)
            .await
            .unwrap();

        populate_defaults(&mut manifest, extension_path, fs.clone())
            .await
            .unwrap();

        assert_eq!(
            manifest.snippets,
            Some(PathBuf::from_str("snippets.json").unwrap())
        )
    }
}
