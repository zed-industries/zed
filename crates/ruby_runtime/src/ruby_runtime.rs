use anyhow::{anyhow, bail, Context, Result};
use regex::Regex;
use smol::fs::unix::PermissionsExt;
use smol::lock::Mutex;
use std::fs;
use std::process::Output;
use std::{
    path::{Path, PathBuf},
    sync::Arc,
};
use util::ResultExt;

#[derive(Clone)]
pub struct RubyRuntime(Arc<Mutex<RubyRuntimeState>>);

struct RubyRuntimeState {
    instance: Option<Box<dyn RubyRuntimeTrait>>,
}

impl RubyRuntime {
    pub fn new() -> Self {
        RubyRuntime(Arc::new(Mutex::new(RubyRuntimeState { instance: None })))
    }

    pub fn unavailable() -> Self {
        RubyRuntime(Arc::new(Mutex::new(RubyRuntimeState { instance: None })))
    }

    async fn instance(&self) -> Result<Box<dyn RubyRuntimeTrait>> {
        let mut state = self.0.lock().await;

        if let Some(instance) = state.instance.as_ref() {
            return Ok(instance.boxed_clone());
        }

        if let Some(instance) = SystemRubyRuntime::detect().await {
            state.instance = Some(instance.boxed_clone());
            return Ok(instance);
        }

        let instance = Box::new(UnavailableRubyRuntime);

        state.instance = Some(instance.boxed_clone());
        return Ok(instance);
    }

    pub async fn binary_path(&self) -> Result<PathBuf> {
        self.instance().await?.binary_path()
    }

    pub async fn run_gem_subcommand(
        &self,
        directory: &Path,
        subcommand: &str,
        args: &[&str],
    ) -> Result<Output> {
        self.instance()
            .await?
            .run_gem_subcommand(directory, subcommand, args)
            .await
    }

    pub async fn gem_installed_version(
        &self,
        directory: &Path,
        name: &str,
    ) -> Result<Option<String>> {
        self.instance()
            .await?
            .gem_installed_version(directory, name)
            .await
    }

    pub async fn gem_latest_version(&self, directory: &Path, name: &str) -> Result<String> {
        self.gem_all_versions(directory, name)
            .await
            .and_then(|versions| {
                versions
                    .into_iter()
                    .next()
                    .ok_or_else(|| anyhow!("No versions found for gem {}", name))
            })
    }

    pub async fn gem_install_gem(
        &self,
        directory: &Path,
        name: &str,
        version: &str,
        binaries: Vec<String>,
    ) -> Result<()> {
        let arguments = [
            "--no-user-install",      // Do not install gems in user's home directory
            "--no-format-executable", // Do not make installed executable names match Ruby
            "--no-document",          // Do not generate documentation
            &format!("{name}:{version}"),
        ];

        self.run_gem_subcommand(directory, "install", &arguments)
            .await?;

        if binaries.is_empty() {
            return Ok(());
        }

        for binary in binaries {
            let bin_path = directory.join(&binary);

            let bin_wrapper = format!(
                r#"#!/usr/bin/env bash
export GEM_PATH="{gem_path}:$GEM_PATH"
exec "{exec_path}" "$@"
"#,
                gem_path = directory.display(),
                exec_path = directory.join("bin").join(&binary).display()
            );

            fs::write(&bin_path, bin_wrapper).with_context(|| {
                format!(
                    "Failed to write binary wrapper for '{}' to {:?}",
                    &binary, bin_path
                )
            })?;

            let mut perms = fs::metadata(&bin_path)?.permissions();
            perms.set_mode(0o755);
            fs::set_permissions(&bin_path, perms).with_context(|| {
                format!(
                    "Failed to set executable permissions for '{}' at {:?}",
                    binary, bin_path
                )
            })?;
        }

        Ok(())
    }

    async fn gem_all_versions(&self, directory: &Path, name: &str) -> Result<Vec<String>> {
        let output = self
            .instance()
            .await?
            .run_gem_subcommand(directory, "list", &[name, "--remote", "--all"])
            .await?;

        let output = String::from_utf8_lossy(&output.stdout);

        output
            .lines()
            .find(|line| line.starts_with(name))
            .and_then(|line| {
                line.rfind('(')
                    .and_then(|start| line.rfind(')').map(|end| &line[start + 1..end]))
            })
            .map(|versions| versions.split(", ").map(String::from).collect())
            .ok_or_else(|| anyhow!("Failed to parse gem list output."))
    }
}

#[async_trait::async_trait]
trait RubyRuntimeTrait: Send + Sync {
    fn boxed_clone(&self) -> Box<dyn RubyRuntimeTrait>;
    fn binary_path(&self) -> Result<PathBuf>;

    async fn run_gem_subcommand(
        &self,
        directory: &Path,
        subcommand: &str,
        args: &[&str],
    ) -> Result<Output>;

    async fn gem_installed_version(&self, directory: &Path, name: &str) -> Result<Option<String>>;
}

#[derive(Clone)]
pub struct SystemRubyRuntime {
    ruby: PathBuf,
    gem: PathBuf,
    // TODO: Put .gemrc with GEM_PATH
    // scratch_dir: PathBuf,
}

impl SystemRubyRuntime {
    const MIN_VERSION: semver::Version = semver::Version::new(3, 4, 1);

    async fn new(ruby: PathBuf, gem: PathBuf) -> Result<Box<dyn RubyRuntimeTrait>> {
        let output = util::command::new_smol_command(&ruby)
            .arg("--version")
            .output()
            .await
            .with_context(|| format!("running ruby from {:?}", ruby))?;

        if !output.status.success() {
            anyhow::bail!(
                "failed to run ruby --version. stdout: {}, stderr: {}",
                String::from_utf8_lossy(&output.stdout),
                String::from_utf8_lossy(&output.stderr),
            );
        }

        let version_str = String::from_utf8_lossy(&output.stdout);
        let version_str = version_str
            .trim()
            .split_whitespace()
            .nth(1)
            .ok_or_else(|| anyhow!("Unable to parse Ruby version"))?;
        let version = semver::Version::parse(version_str)?;
        if version < Self::MIN_VERSION {
            anyhow::bail!(
                "ruby at {} is too old. want: {}, got: {}",
                ruby.to_string_lossy(),
                Self::MIN_VERSION,
                version
            )
        }

        // let scratch_dir = paths::support_dir().join("ruby");
        // fs::create_dir(&scratch_dir).await.ok();
        // fs::create_dir(scratch_dir.join("cache")).await.ok();

        Ok(Box::new(Self {
            ruby,
            gem,
            // scratch_dir,
        }))
    }

    async fn detect() -> Option<Box<dyn RubyRuntimeTrait>> {
        let ruby = which::which("ruby").ok()?;
        let gem = which::which("gem").ok()?;
        Self::new(ruby, gem).await.log_err()
    }
}

#[async_trait::async_trait]
impl RubyRuntimeTrait for SystemRubyRuntime {
    fn boxed_clone(&self) -> Box<dyn RubyRuntimeTrait> {
        Box::new(self.clone())
    }

    fn binary_path(&self) -> Result<PathBuf> {
        Ok(self.ruby.clone())
    }

    async fn run_gem_subcommand(
        &self,
        directory: &Path,
        subcommand: &str,
        args: &[&str],
    ) -> anyhow::Result<Output> {
        let mut command = util::command::new_smol_command(self.gem.clone());

        let gem_home = directory;

        command
            .env_clear()
            .env("PATH", std::env::var_os("PATH").unwrap_or_default())
            .env("GEM_HOME", gem_home)
            .arg(subcommand)
            .args(args);

        let output = command.output().await?;
        if !output.status.success() {
            return Err(anyhow!(
                "failed to execute gem {subcommand} subcommand:\nstdout: {:?}\nstderr: {:?}",
                String::from_utf8_lossy(&output.stdout),
                String::from_utf8_lossy(&output.stderr)
            ));
        }

        Ok(output)
    }

    async fn gem_installed_version(&self, directory: &Path, name: &str) -> Result<Option<String>> {
        // Example output from `gem list`:
        /*
            *** LOCAL GEMS ***

            abbrev (0.1.2)
            prism (default: 1.2.0)
            test-unit (3.6.7)
        */
        let re = Regex::new(r"^(\S+) \((\S+)\)$").unwrap();

        let output = self.run_gem_subcommand(directory, "list", &[]).await?;

        let output =
            String::from_utf8(output.stdout).context("Failed to parse gem list output as UTF-8")?;

        for line in output.lines() {
            let captures = match re.captures(line) {
                Some(c) => c,
                None => continue,
            };

            let gem_package = captures.get(1).map(|m| m.as_str());
            let version = captures.get(2).map(|m| m.as_str());

            if gem_package == Some(name) {
                return Ok(version.map(|v| v.to_owned()));
            }
        }

        Ok(None)
    }
}

pub struct UnavailableRubyRuntime;

#[async_trait::async_trait]
impl RubyRuntimeTrait for UnavailableRubyRuntime {
    fn boxed_clone(&self) -> Box<dyn RubyRuntimeTrait> {
        Box::new(UnavailableRubyRuntime)
    }

    fn binary_path(&self) -> Result<PathBuf> {
        bail!("binary_path: no ruby runtime available")
    }

    async fn run_gem_subcommand(&self, _: &Path, _: &str, _: &[&str]) -> anyhow::Result<Output> {
        bail!("run_gem_subcommand: no ruby runtime available")
    }

    async fn gem_installed_version(&self, _: &Path, _: &str) -> Result<Option<String>> {
        bail!("gem_installed_version: no ruby runtime available")
    }
}
