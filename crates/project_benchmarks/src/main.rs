use std::{sync::Arc, time::Duration};

use anyhow::anyhow;
use askpass::EncryptedPassword;
use clap::Parser;
use client::{Client, UserStore};
use futures::channel::oneshot;
use gpui::{AppContext as _, Application};
use http_client::FakeHttpClient;
use language::LanguageRegistry;
use node_runtime::NodeRuntime;
use project::{
    Project, RealFs,
    search::{SearchQuery, SearchResult},
};
use release_channel::ReleaseChannel;
use remote::{ConnectionIdentifier, RemoteClientDelegate, SshConnectionOptions};
use semver::Version;

#[derive(Parser)]
struct Args {
    /// List of worktrees to run the search against.
    worktrees: Vec<String>,
    #[clap(short)]
    query: Option<String>,
    /// Askpass socket for SSH authentication
    #[clap(long)]
    askpass: Option<String>,
    /// Treat query as a regex.
    #[clap(short, long)]
    regex: bool,
    /// Matches have to be standalone words.
    #[clap(long)]
    whole_word: bool,
    /// Make matching case-sensitive.
    #[clap(long, default_value_t = false)]
    case_sensitive: bool,
    /// Include gitignored files in the search.
    #[clap(long)]
    include_ignored: bool,
    #[clap(long)]
    ssh: Option<String>,
}

struct BenchmarkRemoteClient;
impl RemoteClientDelegate for BenchmarkRemoteClient {
    fn ask_password(
        &self,
        prompt: String,
        tx: oneshot::Sender<EncryptedPassword>,
        _cx: &mut gpui::AsyncApp,
    ) {
        eprintln!("SSH asking for password: {}", prompt);
        match rpassword::prompt_password(&prompt) {
            Ok(password) => match EncryptedPassword::try_from(password.as_ref()) {
                Ok(encrypted) => {
                    if tx.send(encrypted).is_err() {
                        eprintln!("Failed to send password");
                    }
                }
                Err(e) => eprintln!("Failed to encrypt password: {e}"),
            },
            Err(e) => eprintln!("Failed to read password: {e}"),
        }
    }

    fn get_download_url(
        &self,
        _platform: remote::RemotePlatform,
        _release_channel: ReleaseChannel,
        _version: Option<Version>,
        _cx: &mut gpui::AsyncApp,
    ) -> gpui::Task<gpui::Result<Option<String>>> {
        unimplemented!()
    }

    fn download_server_binary_locally(
        &self,
        _platform: remote::RemotePlatform,
        _release_channel: ReleaseChannel,
        _version: Option<Version>,
        _cx: &mut gpui::AsyncApp,
    ) -> gpui::Task<gpui::Result<std::path::PathBuf>> {
        unimplemented!()
    }

    fn set_status(&self, status: Option<&str>, _: &mut gpui::AsyncApp) {
        if let Some(status) = status {
            println!("SSH status: {status}");
        }
    }
}
fn main() -> Result<(), anyhow::Error> {
    let args = Args::parse();

    if let Some(socket) = &args.askpass {
        askpass::main(socket);
        return Ok(());
    }

    let query_str = args
        .query
        .ok_or_else(|| anyhow!("-q/--query is required"))?;
    let query = if args.regex {
        SearchQuery::regex(
            query_str,
            args.whole_word,
            args.case_sensitive,
            args.include_ignored,
            false,
            Default::default(),
            Default::default(),
            false,
            None,
        )
    } else {
        SearchQuery::text(
            query_str,
            args.whole_word,
            args.case_sensitive,
            args.include_ignored,
            Default::default(),
            Default::default(),
            false,
            None,
        )
    }?;
    Application::headless().run(|cx| {
        release_channel::init_test(semver::Version::new(0, 0, 0), ReleaseChannel::Dev, cx);
        settings::init(cx);
        let client = Client::production(cx);
        let http_client = FakeHttpClient::with_200_response();
        let (_, rx) = watch::channel(None);
        let node = NodeRuntime::new(http_client, None, rx);
        let user_store = cx.new(|cx| UserStore::new(client.clone(), cx));
        let registry = Arc::new(LanguageRegistry::new(cx.background_executor().clone()));
        let fs = Arc::new(RealFs::new(None, cx.background_executor().clone()));



            cx.spawn(async move |cx| {
                let project = if let Some(ssh_target) = args.ssh {
                    println!("Setting up SSH connection for {}", &ssh_target);
                    let ssh_connection_options = SshConnectionOptions::parse_command_line(&ssh_target)?;

                    let connection_options = remote::RemoteConnectionOptions::from(ssh_connection_options);
                    let delegate = Arc::new(BenchmarkRemoteClient);
                    let remote_connection = remote::connect(connection_options.clone(), delegate.clone(), cx).await.unwrap();

                    let (_tx, rx) = oneshot::channel();
                    let remote_client =  cx.update(|cx| remote::RemoteClient::new(ConnectionIdentifier::setup(), remote_connection, rx, delegate.clone(), cx )).await?.ok_or_else(|| anyhow!("ssh initialization returned None"))?;

                    cx.update(|cx| Project::remote(remote_client,  client, node, user_store, registry, fs, false, cx))
                } else {
                    println!("Setting up local project");
                    cx.update(|cx| Project::local(
                    client,
                    node,
                    user_store,
                    registry,
                    fs,
                    Some(Default::default()),
                    project::LocalProjectFlags {
                        init_worktree_trust: false,
                        ..Default::default()
                    },
                    cx,
                ))
                };
                println!("Loading worktrees");
                let worktrees = project.update(cx, |this, cx| {
                    args.worktrees
                        .into_iter()
                        .map(|worktree| this.find_or_create_worktree(worktree, true, cx))
                        .collect::<Vec<_>>()
                });

                let worktrees = futures::future::join_all(worktrees)
                    .await
                    .into_iter()
                    .collect::<Result<Vec<_>, anyhow::Error>>()?;

                for (worktree, _) in &worktrees {
                    let scan_complete = worktree
                        .update(cx, |this, _| {
                            if let Some(local) = this.as_local() {
                                Some(local.scan_complete())
                            } else {
                                None
                            }
                        });
                    if let Some(scan_complete) = scan_complete {
                        scan_complete.await;
                    } else {
                        cx.background_executor().timer(Duration::from_secs(10)).await;
                    }

                }
                println!("Worktrees loaded");

                println!("Starting a project search");
                let timer = std::time::Instant::now();
                let mut first_match = None;
                let matches = project.update(cx, |this, cx| this.search(query, cx));
                let mut matched_files = 0;
                let mut matched_chunks = 0;
                while let Ok(match_result) = matches.rx.recv().await {
                    if first_match.is_none() {
                        let time = timer.elapsed();
                        first_match = Some(time);
                        println!("First match found after {time:?}");
                    }
                    if let SearchResult::Buffer { ranges, .. } = match_result {
                        matched_files += 1;
                        matched_chunks += ranges.len();
                    } else {
                        break;
                    }
                }
                let elapsed = timer.elapsed();
                println!(
                    "Finished project search after {elapsed:?}. Matched {matched_files} files and {matched_chunks} excerpts"
                );
                drop(project);
                cx.update(|cx| cx.quit());

                anyhow::Ok(())
            })
            .detach_and_log_err(cx);

    });
    Ok(())
}
