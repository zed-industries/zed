mod wit {
    wit_bindgen::generate!({
        path: "../../wit",
        exports: { world: super::Component },
    });
}

struct Component;

impl wit::Guest for Component {
    fn get_language_server_command(worktree: &wit::Worktree) -> Result<wit::Command, String> {
        let tool_versions = worktree.read_text_file(".tool-versions")?;

        println!("tool versions: {tool_versions}");

        let rust_version = tool_versions.lines().find_map(|line| {
            let mut parts = line.split(" ");
            if parts.next() == Some("rust") {
                parts.next()
            } else {
                None
            }
        });

        println!("rust version: {rust_version:?}");

        let release = wit::latest_github_release(
            "rust-lang/rust-analyzer",
            wit::GithubReleaseOptions {
                require_assets: true,
                pre_release: false,
            },
        )?;

        println!("Download URL: {}", release.assets[0].download_url);

        Ok(wit::Command {
            command: "path/to/rust-analyzer".to_string(),
            args: vec!["--stdio".into()],
            env: vec![],
        })
    }
}
