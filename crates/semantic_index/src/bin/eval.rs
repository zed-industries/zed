use anyhow::{Context as _, Result};
use clap::{Parser, Subcommand};
use gpui::AsyncAppContext;
use http_client::http;
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use serde::{Deserialize, Serialize};
use smol::io::{AsyncReadExt, BufReader};
use std::{
    collections::BTreeSet,
    fs,
    path::Path,
    process::{exit, Command, Stdio},
};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

const DATASETS_DIR: &'static str = "target/datasets";
const CODESEARCH_NET: &'static str = "code-search-net";
const GITHUB_URL: &str = "https://github.com";

const CODE_SEARCH_NET_LANGUAGES: &[&str] = &[
    "python",
    "javascript",
    "go",
    // "java", "ruby", "php",
];

#[derive(Subcommand)]
enum Commands {
    Install {},
    Run {
        #[arg(value_name = "LANGUAGE")]
        language: String,
    },
}
fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Install {} => {
            gpui::App::headless().run(|cx| {
                cx.spawn(|mut cx| async move {
                    if let Err(err) = install(&mut cx).await {
                        eprintln!("Error: {}", err);
                        exit(1);
                    }
                    exit(0);
                })
                .detach();
            });
        }
        Commands::Run { language } => {
            gpui::App::headless().run(|cx| {
                cx.spawn(|mut cx| async move {
                    // if let Err(err) = fetch_code_search_net_repos(&mut cx).await {
                    //     eprintln!("Error: {}", err);
                    //     std::process::exit(1);
                    // }
                    exit(0);
                })
                .detach();
            });
        }
    }

    Ok(())
}

async fn install(cx: &mut AsyncAppContext) -> Result<()> {
    fetch_code_search_net_resources(cx).await?;
    fetch_code_search_net_repos(cx).await?;
    Ok(())
}

async fn fetch_code_search_net_resources(cx: &mut AsyncAppContext) -> Result<()> {
    let destination_dir = Path::new(DATASETS_DIR).join(CODESEARCH_NET);
    fs::create_dir_all(&destination_dir).with_context(|| {
        format!(
            "Failed to create destination directory: {:?}",
            destination_dir
        )
    })?;

    let http_client = http_client::HttpClientWithProxy::new(None, None);

    let multi_progress = MultiProgress::new();
    let sty = ProgressStyle::default_bar()
        .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}")
        .unwrap()
        .progress_chars("##-");

    let urls = vec![
        ("annotation_store.csv", "https://raw.githubusercontent.com/github/CodeSearchNet/master/resources/annotationStore.csv"),
        ("queries.csv", "https://raw.githubusercontent.com/github/CodeSearchNet/master/resources/queries.csv"),
    ];
    for (filename, url) in urls {
        let file_path = destination_dir.join(filename);
        if file_path.exists() {
            println!("{} already exists, skipping download...", filename);
            continue;
        }

        cx.background_executor()
            .spawn({
                let file_path = file_path.clone();
                let http_client = http_client.clone();
                async move {
                    let mut response = http_client
                        .get(url, Default::default(), true)
                        .await?
                        .into_body();
                    let mut buf = Vec::new();
                    response.read_to_end(&mut buf).await?;
                    fs::write(file_path, &buf)
                        .with_context(|| format!("Failed to write {}", filename))?;
                    anyhow::Ok(())
                }
            })
            .await
            .with_context(|| format!("Failed to download from URL: {}", url))?;
    }

    for language in CODE_SEARCH_NET_LANGUAGES {
        let language_dir = destination_dir.join(language);
        let language_zip_path = destination_dir.join(format!("{language}.zip"));
        if language_dir.exists() && language_dir.is_dir() {
            println!("Directory for {} already exists, skipping...", language);
            continue;
        }

        let url = format!(
            "https://huggingface.co/datasets/code-search-net/code_search_net/resolve/main/data/{language}.zip"
        );

        let response = cx
            .background_executor()
            .spawn({
                let http_client = http_client.clone();
                let url = url.clone();
                async move { http_client.get(&url, Default::default(), true).await }
            })
            .await
            .with_context(|| format!("Failed to download dataset from URL: {}", url))?;

        let total_size = response
            .headers()
            .get(http::header::CONTENT_LENGTH)
            .and_then(|h| h.to_str().ok()?.parse().ok())
            .context("No valid Content-Length header found")?;

        let should_download = match fs::metadata(&language_zip_path) {
            Ok(metadata) => metadata.len() != total_size,
            Err(_) => true,
        };

        if should_download {
            let pb = multi_progress.add(ProgressBar::new(total_size));
            pb.set_style(sty.clone());
            pb.set_message(format!("Downloading {}", language));

            let mut body = response.into_body();
            let mut zip_content = Vec::new();
            let mut buffer = [0; 8192];

            while let Ok(n) = body.read(&mut buffer).await {
                if n == 0 {
                    break;
                }
                zip_content.extend_from_slice(&buffer[..n]);
                pb.inc(n as u64);
            }

            pb.finish_with_message(format!("{} downloaded", language));

            fs::write(&language_zip_path, &zip_content).with_context(|| {
                format!("Failed to write zip file: {}", language_zip_path.display())
            })?;
        }

        let file = fs::File::open(&language_zip_path)
            .with_context(|| format!("Failed to open zip file: {}", language_zip_path.display()))?;
        let mut archive = zip::ZipArchive::new(file).with_context(|| {
            format!(
                "Failed to read zip archive: {}",
                language_zip_path.display()
            )
        })?;

        let pb = multi_progress.add(ProgressBar::new(archive.len() as u64));
        pb.set_style(sty.clone());
        pb.set_message(format!("Extracting {}", language));

        for i in 0..archive.len() {
            let mut file = archive.by_index(i).unwrap();
            let path = match file.enclosed_name() {
                Some(path) => destination_dir.join(path),
                None => continue,
            };

            if file.is_dir() {
                fs::create_dir_all(path).unwrap();
            } else {
                if let Some(p) = path.parent() {
                    if !p.exists() {
                        fs::create_dir_all(p).unwrap();
                    }
                }

                let mut outfile = fs::File::create(&path).unwrap();
                std::io::copy(&mut file, &mut outfile).unwrap();
            }

            pb.inc(1);
        }

        pb.finish_with_message(format!("{} extracted", language));

        // fs::remove_file(&language_zip_path).with_context(|| {
        //     format!("Failed to remove zip file: {}", language_zip_path.display())
        // })?;
    }

    println!(
        "Datasets installed successfully in {}",
        destination_dir.display()
    );
    Ok(())
}

#[derive(Deserialize, Serialize, PartialOrd, Ord, PartialEq, Eq)]
struct RepoInfo {
    repo: String,
    sha: String,
}

async fn fetch_code_search_net_repos(cx: &mut AsyncAppContext) -> Result<()> {
    for language in CODE_SEARCH_NET_LANGUAGES {
        let dataset_dir = Path::new(DATASETS_DIR).join(CODESEARCH_NET);
        let language_dir = dataset_dir.join(&language);
        let mut repos = BTreeSet::new();

        let pickle_file = dataset_dir.join(format!("{}_dedupe_definitions_v2.pkl", language));
        if !pickle_file.exists() {
            return Err(anyhow::anyhow!("Pickle file for {} not found", language));
        }

        let pickle_path = pickle_file.to_str().unwrap();
        let output = Command::new("python3")
            .args(&[
                "-c",
                r#"
import pickle
import sys

with open(sys.argv[1], 'rb') as f:
    data = pickle.load(f)
    for item in data:
        print(f"{item['nwo']},{item['sha']}")
        "#,
                pickle_path,
            ])
            .output()?;

        if !output.status.success() {
            return Err(anyhow::anyhow!(
                "Failed to run Python script: {}",
                String::from_utf8_lossy(&output.stderr)
            ));
        }

        let output_str = String::from_utf8(output.stdout)?;

        for line in output_str.lines() {
            let parts: Vec<&str> = line.split(',').collect();
            if parts.len() == 2 {
                repos.insert(RepoInfo {
                    repo: parts[0].to_string(),
                    sha: parts[1].to_string(),
                });
            }
        }

        let repos_json_path = dataset_dir.join(format!("{language}_repos.json"));
        fs::write(&repos_json_path, serde_json::to_vec_pretty(&repos).unwrap()).unwrap();

        return Ok(());

        // let test_file = language_dir
        //     .join("final/jsonl/test")
        //     .join(format!("{}_test_0.jsonl.gz", language));

        // if !test_file.exists() {
        //     return Err(anyhow::anyhow!("Test file for {} not found", language));
        // }

        // let file = smol::fs::File::open(&test_file).await?;
        // let reader = BufReader::new(file);
        // let gz = BufReader::new(GzipDecoder::new(reader));
        // let mut lines = gz.lines();

        // while let Some(line) = lines.next().await {
        //     let line = line?;
        //     if let Ok(repo_info) = serde_json::from_str::<RepoInfo>(&line) {
        //         repos.insert((repo_info.repo, repo_info.sha));
        //     }
        // }

        let repos_dir = dataset_dir.join("repos");
        fs::create_dir_all(&repos_dir)?;

        let multi_progress = MultiProgress::new();
        let sty = ProgressStyle::default_bar()
            .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}")
            .unwrap()
            .progress_chars("##-");

        let pb = multi_progress.add(ProgressBar::new(repos.len() as u64));
        pb.set_style(sty);
        pb.set_message(format!("Cloning repositories for {}", language));

        for RepoInfo { repo, sha } in repos {
            let repo_dir = repos_dir.join(&repo);
            if !repo_dir.exists() {
                fs::create_dir_all(&repo_dir)?;
                let url = format!("https://github.com/{}.git", repo);

                let init_output = Command::new("git")
                    .current_dir(&repo_dir)
                    .args(&["init"])
                    .output()?;
                if !init_output.status.success() {
                    eprintln!(
                        "Failed to initialize git repository for {}: {}",
                        repo,
                        String::from_utf8_lossy(&init_output.stderr)
                    );
                    continue;
                }

                let remote_output = Command::new("git")
                    .current_dir(&repo_dir)
                    .args(&["remote", "add", "origin", &url])
                    .stdin(Stdio::null())
                    .output()?;
                if !remote_output.status.success() {
                    eprintln!(
                        "Failed to add remote for {}: {}",
                        repo,
                        String::from_utf8_lossy(&remote_output.stderr)
                    );
                    continue;
                }

                let fetch_output = Command::new("git")
                    .current_dir(&repo_dir)
                    .args(&["fetch", "--depth", "1", "origin", &sha])
                    .stdin(Stdio::null())
                    .output()?;
                if !fetch_output.status.success() {
                    eprintln!(
                        "Failed to fetch {} for {}: {}",
                        sha,
                        repo,
                        String::from_utf8_lossy(&fetch_output.stderr)
                    );
                    continue;
                }

                let checkout_output = Command::new("git")
                    .current_dir(&repo_dir)
                    .args(&["checkout", &sha])
                    .output()?;
                if !checkout_output.status.success() {
                    eprintln!(
                        "Failed to checkout {} for {}: {}",
                        sha,
                        repo,
                        String::from_utf8_lossy(&checkout_output.stderr)
                    );
                    continue;
                }
            }

            pb.inc(1);
        }

        pb.finish_with_message(format!("Finished cloning repositories for {}", language));

        break;
    }
    Ok(())
}
