mod headless;

use anyhow::{Result, anyhow};
use clap::{Parser, Subcommand};
use gpui::Task;
use gpui::{App, AppContext, Application};
use language::Bias;
use language::Buffer;
use language::Point;
use project::Project;
use reqwest_client::ReqwestClient;
use std::collections::VecDeque;
use std::path::{Path, PathBuf};
use std::process::exit;
use std::str::FromStr;
use std::sync::Arc;
use zed_llm_client::PredictEditsBody;
use zeta::gather_context;

use crate::headless::ZetaCliAppState;

#[derive(Parser, Debug)]
#[command(name = "zeta")]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    GetContext {
        #[arg(long)]
        worktree: PathBuf,
        #[arg(long)]
        cursor: CursorPosition,
        #[arg(long)]
        use_language_server: bool,
    },
}

#[derive(Debug, Clone)]
struct CursorPosition {
    path: PathBuf,
    point: Point,
}

impl FromStr for CursorPosition {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let parts: Vec<&str> = s.split(':').collect();
        if parts.len() != 3 {
            return Err(anyhow!(
                "Invalid cursor format. Expected 'file.rs:line:column', got '{}'",
                s
            ));
        }

        let path = PathBuf::from(parts[0]);
        let line: u32 = parts[1]
            .parse()
            .map_err(|_| anyhow!("Invalid line number: '{}'", parts[1]))?;
        let column: u32 = parts[2]
            .parse()
            .map_err(|_| anyhow!("Invalid column number: '{}'", parts[2]))?;

        // Convert from 1-based to 0-based indexing
        let point = Point::new(line.saturating_sub(1), column.saturating_sub(1));

        Ok(CursorPosition { path, point })
    }
}

fn get_context(
    worktree: &Path,
    cursor: &CursorPosition,
    use_language_server: bool,
    app_state: Arc<ZetaCliAppState>,
    cx: &mut App,
) -> Task<Result<PredictEditsBody>> {
    let project = if use_language_server {
        // todo! actually implement use of language server
        Some(Project::local(
            app_state.client.clone(),
            app_state.node_runtime.clone(),
            app_state.user_store.clone(),
            app_state.languages.clone(),
            app_state.fs.clone(),
            None,
            cx,
        ))
    } else {
        None
    };

    let worktree = match worktree.canonicalize() {
        Ok(worktree) => worktree,
        Err(e) => return Task::ready(Err(anyhow!(e))),
    };
    let abs_path = worktree.join(&cursor.path);
    let worktree_name = match worktree.file_name() {
        Some(name) => name,
        None => return Task::ready(Err(anyhow!("--worktree path must end with a folder name"))),
    };
    let full_path_str = PathBuf::from(worktree_name)
        .join(&cursor.path)
        .to_string_lossy()
        .to_string();

    let content = match std::fs::read_to_string(&abs_path) {
        Ok(content) => content,
        Err(e) => {
            return Task::ready(Err(anyhow!(
                "Failed to read file {:?}: {:?}",
                abs_path.display(),
                e
            )));
        }
    };

    let buffer = cx.new(|cx| Buffer::local(content, cx));

    let snapshot = buffer.read(cx).snapshot();
    let clipped_cursor = snapshot.clip_point(cursor.point, Bias::Left);
    if clipped_cursor != cursor.point {
        let max_row = snapshot.max_point().row;
        if cursor.point.row < max_row {
            return Task::ready(Err(anyhow!(
                "Cursor position {:?} is out of bounds (line length is {})",
                cursor.point,
                snapshot.line_len(cursor.point.row)
            )));
        } else {
            return Task::ready(Err(anyhow!(
                "Cursor position {:?} is out of bounds (max row is {})",
                cursor.point,
                max_row
            )));
        }
    }
    let cursor_anchor = snapshot.anchor_before(cursor.point);

    let events = VecDeque::new();
    let can_collect_data = false;
    let gather_task = gather_context(
        project.as_ref(),
        full_path_str,
        &snapshot,
        cursor_anchor,
        events,
        can_collect_data,
        cx,
    );

    cx.background_spawn(async move {
        let output = gather_task.await?;
        Ok(output.body)
    })
}

fn main() -> Result<()> {
    let args = Args::parse();

    let http_client = Arc::new(ReqwestClient::new());
    let app = Application::headless().with_http_client(http_client);

    match args.command {
        Commands::GetContext {
            worktree,
            cursor,
            use_language_server,
        } => {
            app.run(move |cx| {
                let app_state = Arc::new(headless::init(cx));
                let task = get_context(&worktree, &cursor, use_language_server, app_state, cx);
                cx.spawn(async move |cx| match task.await {
                    Ok(output) => {
                        println!("{}", serde_json::to_string_pretty(&output).unwrap());
                        let _ = cx.update(|cx| cx.quit());
                    }
                    Err(e) => {
                        eprintln!("Failed to get context:\n{:?}", e);
                        exit(1);
                    }
                })
                .detach();
            });
        }
    }

    Ok(())
}
