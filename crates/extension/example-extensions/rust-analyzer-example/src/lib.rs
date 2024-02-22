mod bindings;
use bindings::{latest_github_release, Command, GithubReleaseOptions, Guest, Worktree};

struct Component;

impl Guest for Component {
    fn get_language_server_command(worktree: &Worktree) -> Result<Command, String> {
        println!(
            "worktree file content: {}",
            worktree.read_text_file("something")?
        );

        let release = latest_github_release(
            "rust-lang/rust-analyzer",
            GithubReleaseOptions {
                require_assets: true,
                pre_release: false,
            },
        )?;

        println!("Download URL: {}", release.assets[0].download_url);

        Ok(Command {
            command: "path/to/rust-analyzer".to_string(),
            args: vec!["--stdio".into()],
            env: vec![],
        })
    }
}
