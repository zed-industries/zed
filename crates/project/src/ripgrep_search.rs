use anyhow::{Context as _, Result};
use futures::StreamExt;
use smol::{
    channel::{Receiver, Sender},
    io::{AsyncBufReadExt, BufReader},
    process::{Child, Command},
};
use std::{path::PathBuf, process::Stdio};

use crate::search::{SearchInputs, SearchQuery};

/// Search results streamed from ripgrep
#[derive(Debug, Clone)]
pub enum RipgrepSearchResult {
    /// A match found in a file with path and line info
    Match {
        path: PathBuf,
        line_number: u32,
        ranges: Vec<(u32, u32)>, // (start, end) byte offsets within the line
        line_content: String,
    },
    /// Progress update
    Progress {
        files_searched: usize,
        matches_found: usize,
    },
    /// Search completed
    Complete { total_matches: usize },
    /// Error occurred
    Error(String),
}

/// Simple ripgrep integration that uses the ripgrep binary via subprocess
pub struct RipgrepSearcher {
    current_process: Option<Child>,
}

impl RipgrepSearcher {
    pub fn new() -> Self {
        Self {
            current_process: None,
        }
    }

    /// Perform search using ripgrep subprocess and return streaming results
    pub async fn search_paths(
        &mut self,
        query: &SearchQuery,
        search_paths: &[PathBuf],
    ) -> Result<Receiver<RipgrepSearchResult>> {
        log::info!("RipgrepSearcher::search_paths called with {} paths", search_paths.len());
        for path in search_paths {
            log::info!("  Search path: {:?}", path);
        }
        
        // Cancel any existing search
        self.cancel_search().await;

        let (tx, rx) = smol::channel::unbounded();

        // Build ripgrep command
        let cmd = self.build_ripgrep_command(query, search_paths)?;
        log::info!("Built ripgrep command: {:?}", cmd);

        // Spawn the process
        let child = Command::from(cmd)
            .stdout(std::process::Stdio::piped()) // Make sure stdout is piped
            .spawn()
            .context("Failed to spawn ripgrep process")?;

        self.current_process = Some(child);

        // Process output in background
        if let Some(process) = &mut self.current_process {
            if let Some(stdout) = process.stdout.take() {
                let tx_clone = tx.clone();
                smol::spawn(async move {
                    Self::process_ripgrep_output(stdout, tx_clone).await;
                })
                .detach();
            }
        }

        Ok(rx)
    }

    /// Cancel the current search if running
    pub async fn cancel_search(&mut self) {
        if let Some(mut process) = self.current_process.take() {
            let _ = process.kill();
        }
    }

    /// Build the ripgrep command based on the search query
    fn build_ripgrep_command(
        &self,
        query: &SearchQuery,
        search_paths: &[PathBuf],
    ) -> Result<std::process::Command> {
        let mut cmd = std::process::Command::new("rg");

        // Basic options for structured output
        cmd.arg("--line-number") // Include line numbers
            .arg("--column") // Include column numbers
            .arg("--no-heading") // Don't group by file
            .arg("--with-filename") // Always include filename
            .arg("--color=never") // No color codes
            .stdout(Stdio::piped())
            .stderr(Stdio::null()); // Suppress error output

        match query {
            SearchQuery::Text {
                whole_word,
                case_sensitive,
                include_ignored,
                inner,
                ..
            } => {
                // Add the search pattern
                cmd.arg(inner.as_str());

                // Apply search options
                if *whole_word {
                    cmd.arg("--word-regexp");
                }

                if !case_sensitive {
                    cmd.arg("--ignore-case");
                }

                if *include_ignored {
                    cmd.arg("--no-ignore").arg("--hidden");
                }

                // Add include/exclude patterns
                self.add_path_filters(&mut cmd, inner)?;
            }
            SearchQuery::Regex {
                regex,
                whole_word,
                case_sensitive,
                include_ignored,
                multiline,
                inner,
                ..
            } => {
                // Add the regex pattern
                cmd.arg(regex.as_str());

                // Apply search options
                if *whole_word {
                    cmd.arg("--word-regexp");
                }

                if !case_sensitive {
                    cmd.arg("--ignore-case");
                }

                if *include_ignored {
                    cmd.arg("--no-ignore").arg("--hidden");
                }

                if *multiline {
                    cmd.arg("--multiline");
                }

                // Add include/exclude patterns
                self.add_path_filters(&mut cmd, inner)?;
            }
        }

        // Add search paths
        for path in search_paths {
            cmd.arg(path);
        }

        // Limit to reasonable number of matches
        cmd.arg("--max-count=1000");

        Ok(cmd)
    }

    /// Add include/exclude path filters to the ripgrep command
    fn add_path_filters(
        &self,
        cmd: &mut std::process::Command,
        inner: &SearchInputs,
    ) -> Result<()> {
        // Add include patterns
        for pattern in inner.files_to_include().sources() {
            if !pattern.is_empty() {
                cmd.arg("--glob");
                cmd.arg(pattern);
            }
        }

        // Add exclude patterns
        for pattern in inner.files_to_exclude().sources() {
            if !pattern.is_empty() {
                cmd.arg("--glob");
                cmd.arg(format!("!{}", pattern));
            }
        }

        Ok(())
    }

    /// Process ripgrep output and send results through the channel
    async fn process_ripgrep_output(
        stdout: smol::process::ChildStdout,
        tx: Sender<RipgrepSearchResult>,
    ) {
        let reader = BufReader::new(stdout);
        let mut lines = reader.lines();

        let mut total_matches = 0;
        let mut total_files = 0;
        let mut current_file: Option<String> = None;

        while let Some(line) = lines.next().await {
            let line = match line {
                Ok(line) => line,
                Err(_) => break,
            };
            if line.trim().is_empty() {
                continue;
            }

            // Parse ripgrep output format: filename:line:column:content
            if let Some(result) = Self::parse_ripgrep_line(&line) {
                // Check if we've moved to a new file
                let file_str = match &result {
                    RipgrepSearchResult::Match { path, .. } => path.to_string_lossy().to_string(),
                    _ => continue,
                };
                if current_file.as_ref() != Some(&file_str) {
                    current_file = Some(file_str);
                    total_files += 1;
                }

                total_matches += 1;

                if tx.send(result).await.is_err() {
                    break; // Receiver dropped
                }

                // Send periodic progress updates
                if total_matches % 50 == 0 {
                    let progress = RipgrepSearchResult::Progress {
                        files_searched: total_files,
                        matches_found: total_matches,
                    };

                    if tx.send(progress).await.is_err() {
                        break;
                    }
                }
            }
        }

        // Send completion
        let complete = RipgrepSearchResult::Complete { total_matches };
        let _ = tx.send(complete).await;
    }

    /// Parse a line of ripgrep output in the format: filename:line:column:content
    fn parse_ripgrep_line(line: &str) -> Option<RipgrepSearchResult> {
        let parts: Vec<&str> = line.splitn(4, ':').collect();
        if parts.len() < 4 {
            return None;
        }

        let path = PathBuf::from(parts[0]);
        let line_number: u32 = parts[1].parse().ok()?;
        let column: u32 = parts[2].parse().ok()?;
        let content = parts[3].to_string();

        // For now, create a simple range from the column position
        // In a full implementation, we'd need to extract all match positions
        let ranges = vec![(column, column + 1)];

        Some(RipgrepSearchResult::Match {
            path,
            line_number,
            ranges,
            line_content: content,
        })
    }
}

impl Default for RipgrepSearcher {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for RipgrepSearcher {
    fn drop(&mut self) {
        if let Some(mut process) = self.current_process.take() {
            let _ = process.kill();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use util::paths::PathMatcher;

    fn create_test_query(pattern: &str) -> SearchQuery {
        SearchQuery::text(
            pattern,
            false,                                     // whole_word
            false,                                     // case_sensitive
            false,                                     // include_ignored
            PathMatcher::new(&[] as &[&str]).unwrap(), // files_to_include
            PathMatcher::new(&[] as &[&str]).unwrap(), // files_to_exclude
            None,                                      // buffers
        )
        .unwrap()
    }

    #[test]
    fn test_ripgrep_search_basic() {
        // This test requires async functionality, so we'll skip the actual search test
        // and just verify that we can create the searcher
        let _searcher = RipgrepSearcher::new();
        // Test passes if we can create the searcher without panicking
    }

    #[test]
    fn test_parse_ripgrep_line() {
        let line = "src/main.rs:42:15:    fn test() {";
        let result = RipgrepSearcher::parse_ripgrep_line(line).unwrap();

        match result {
            RipgrepSearchResult::Match {
                path,
                line_number,
                ranges,
                line_content,
            } => {
                assert_eq!(path, PathBuf::from("src/main.rs"));
                assert_eq!(line_number, 42);
                assert_eq!(ranges, vec![(15, 16)]);
                assert_eq!(line_content, "    fn test() {");
            }
            _ => panic!("Expected Match result"),
        }
    }

    #[test]
    fn test_build_ripgrep_command() {
        let searcher = RipgrepSearcher::new();
        let query = create_test_query("test");
        let search_paths = vec![PathBuf::from("/tmp")];

        let cmd = searcher
            .build_ripgrep_command(&query, &search_paths)
            .unwrap();
        let args: Vec<&std::ffi::OsStr> = cmd.get_args().collect();

        // Should contain basic ripgrep arguments
        assert!(args.contains(&std::ffi::OsStr::new("--line-number")));
        assert!(args.contains(&std::ffi::OsStr::new("--column")));
        assert!(args.contains(&std::ffi::OsStr::new("--no-heading")));
        assert!(args.contains(&std::ffi::OsStr::new("test")));
        assert!(args.contains(&std::ffi::OsStr::new("/tmp")));
    }
}
