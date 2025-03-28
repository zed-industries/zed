use std::{ffi::OsStr, path::Path, sync::Arc};

use anyhow::{anyhow, Context};
use fs::Fs;
use futures::future;
use gpui::{App, AppContext, Task};
use util::maybe;
use worktree::Worktree;

mod cursor_mdc;

#[derive(Debug)]
pub struct LoadRulesResult {
    pub files: Vec<RulesFile>,
    pub first_error: Option<String>,
}

#[derive(Debug, PartialEq)]
pub struct RulesFile {
    pub rel_path: Arc<Path>,
    pub abs_path: Arc<Path>,
    pub content: RulesContent,
}

#[derive(Debug, PartialEq)]
pub struct RulesContent {
    pub when_included: WhenIncluded,
    pub description: Option<String>,
    pub text: String,
}

// todo! better names
#[derive(Debug, PartialEq)]
pub enum WhenIncluded {
    Always,
    AutoAttached { globs: Vec<String> },
    AgentRequested,
    Manual,
}

pub fn load_rules(fs: Arc<dyn Fs>, worktree: &Worktree, cx: &App) -> Task<LoadRulesResult> {
    // Note that Cline supports `.clinerules` being a directory, but that is not currently
    // supported. This doesn't seem to occur often in GitHub repositories.
    const PLAIN_RULES_FILE_NAMES: [&'static str; 6] = [
        ".rules",
        ".cursorrules",
        ".windsurfrules",
        ".clinerules",
        ".github/copilot-instructions.md",
        "CLAUDE.md",
    ];
    let selected_plain_rules_file = PLAIN_RULES_FILE_NAMES
        .into_iter()
        .filter_map(|name| {
            worktree
                .entry_for_path(name)
                .filter(|entry| entry.is_file())
                .map(|entry| (entry.path.clone(), worktree.absolutize(&entry.path)))
        })
        .next();

    let mut rules_futures = Vec::new();

    rules_futures.extend(selected_plain_rules_file.map(|(rel_path, abs_path)| {
        let fs = fs.clone();
        future::Either::Left(maybe!(async move {
            let abs_path = abs_path?;
            let text = fs
                .load(&abs_path)
                .await
                .with_context(|| format!("Failed to load assistant rules file {:?}", abs_path))?;
            anyhow::Ok(RulesFile {
                rel_path,
                abs_path: abs_path.into(),
                content: RulesContent {
                    when_included: WhenIncluded::Always,
                    description: None,
                    text: text.trim().to_string(),
                },
            })
        }))
    }));

    // todo! Does this already recurse?
    let mdc_rules_path = Path::new(".cursor/rules");
    let mdc_extension = OsStr::new("mdc");
    rules_futures.extend(
        worktree
            .child_entries(mdc_rules_path)
            .filter(|entry| entry.is_file() && entry.path.extension() == Some(mdc_extension))
            .map(|entry| {
                let fs = fs.clone();
                let rel_path = entry.path.clone();
                let abs_path = worktree.absolutize(&rel_path);
                future::Either::Right(maybe!(async move {
                    let abs_path = abs_path?;
                    let text = fs.load(&abs_path).await.with_context(|| {
                        format!("Failed to load assistant rules file {:?}", abs_path)
                    })?;
                    match cursor_mdc::parse(&text) {
                        Ok(content) => anyhow::Ok(RulesFile {
                            rel_path: rel_path,
                            abs_path: abs_path.into(),
                            content,
                        }),
                        Err(cursor_mdc::ParseError::MissingFrontmatter) => Err(anyhow!("todo!")),
                    }
                }))
            }),
    );

    cx.background_spawn(async move {
        let results = future::join_all(rules_futures).await;
        let mut first_error = None;
        let files = results
            .into_iter()
            .filter_map(|result| match result {
                Ok(file) => Some(file),
                Err(err) => {
                    if first_error.is_none() {
                        first_error = Some(err.to_string());
                    }
                    None
                }
            })
            .collect::<Vec<_>>();
        LoadRulesResult { files, first_error }
    })
}
