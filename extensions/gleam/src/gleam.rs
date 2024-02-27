use zed_extension_api::{self as zed, Result};

struct GleamExtension;

impl zed::Extension for GleamExtension {
    fn get_language_server_command(
        &self,
        config: zed::LanguageServerConfig,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        let release = zed::latest_github_release(
            "gleam-lang/gleam",
            zed::GithubReleaseOptions {
                require_assets: true,
                pre_release: false,
            },
        )?;
        let (platform, arch) = zed::current_platform();
        let asset_name = format!(
            "gleam-{version}-{arch}-{os}.tar.gz",
            version = release.version,
            arch = match arch {
                zed::Architecture::Aarch64 => "aarch64",
                zed::Architecture::X86 => "x86",
                zed::Architecture::X8664 => "x86_64",
            },
            os = match platform {
                zed::Os::Mac => "apple-darwin",
                zed::Os::Linux => "unknown-linux-musl",
                zed::Os::Windows => "pc-windows-msvc",
            },
        );

        println!("{}", &asset_name);

        let asset = release
            .assets
            .iter()
            .find(|asset| asset.name == asset_name)
            .ok_or_else(|| format!("no asset found matching {:?}", asset_name))?;

        Ok(zed::Command {
            command: "ok".into(),
            args: vec![],
            env: Default::default(),
        })
    }
}

zed::register_extension!(GleamExtension);
