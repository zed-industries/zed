//! Jack-in: start an nREPL server for the project (with cider-nrepl
//! middleware injected) and connect to it once it's listening.
//!
//! Mirrors Calva's / CIDER's jack-in. The project type is detected from
//! the build files in the worktree root:
//!
//! | file              | command                                              |
//! |-------------------|------------------------------------------------------|
//! | `deps.edn`        | `clojure -Sdeps … -M -m nrepl.cmdline --port N …`    |
//! | `project.clj`     | `lein update-in :plugins conj … -- repl :headless …` |
//! | `shadow-cljs.edn` | `npx shadow-cljs -d … server`                         |
//! | `bb.edn`          | `bb nrepl-server localhost:N`                         |
//!
//! Except for shadow-cljs (whose nREPL port comes from its own config) we
//! pick a free port up front and pass it to the server, so a stale
//! `.nrepl-port` left behind by an earlier session can never be picked up.

use std::{
    net::{Ipv4Addr, SocketAddr, TcpListener},
    path::{Path, PathBuf},
    sync::Arc,
    time::{Duration, Instant, SystemTime},
};

use anyhow::{Context as _, Result, anyhow, bail};
use futures::{AsyncBufReadExt as _, io::BufReader};
use gpui::{BackgroundExecutor, Task};
use parking_lot::Mutex;
use std::collections::VecDeque;
use util::command::{Child, Stdio, new_command};

pub const NREPL_VERSION: &str = "1.3.1";
pub const CIDER_NREPL_VERSION: &str = "0.52.1";

/// How long we wait for the server to come up. The first jack-in of a
/// project may have to download every dependency, so this is generous.
const STARTUP_TIMEOUT: Duration = Duration::from_secs(300);
const POLL_INTERVAL: Duration = Duration::from_millis(500);
/// Lines of server output kept for error messages and the REPL console.
const OUTPUT_TAIL_LINES: usize = 200;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ProjectKind {
    ClojureCli,
    Leiningen,
    ShadowCljs,
    Babashka,
}

impl ProjectKind {
    pub fn label(self) -> &'static str {
        match self {
            ProjectKind::ClojureCli => "Clojure CLI (deps.edn)",
            ProjectKind::Leiningen => "Leiningen (project.clj)",
            ProjectKind::ShadowCljs => "shadow-cljs",
            ProjectKind::Babashka => "Babashka",
        }
    }

    fn marker_file(self) -> &'static str {
        match self {
            ProjectKind::ClojureCli => "deps.edn",
            ProjectKind::Leiningen => "project.clj",
            ProjectKind::ShadowCljs => "shadow-cljs.edn",
            ProjectKind::Babashka => "bb.edn",
        }
    }

    const ALL: [ProjectKind; 4] = [
        ProjectKind::ClojureCli,
        ProjectKind::Leiningen,
        ProjectKind::ShadowCljs,
        ProjectKind::Babashka,
    ];
}

/// Returns the first project kind whose marker file exists in `root`.
pub fn detect_project(root: &Path) -> Option<ProjectKind> {
    ProjectKind::ALL
        .into_iter()
        .find(|kind| root.join(kind.marker_file()).is_file())
}

/// How the jack-in process tells us where its nREPL server listens.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PortSource {
    /// We chose the port and passed it on the command line.
    Fixed(u16),
    /// The server writes its port to this file once it's listening.
    PortFile(PathBuf),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct JackInCommand {
    pub kind: ProjectKind,
    pub program: String,
    pub args: Vec<String>,
    pub cwd: PathBuf,
    pub port: PortSource,
}

impl JackInCommand {
    pub fn new(kind: ProjectKind, root: &Path, port: u16) -> Self {
        let cider_middleware = "[cider.nrepl/cider-middleware]".to_string();
        let (program, args, port_source) = match kind {
            ProjectKind::ClojureCli => (
                "clojure",
                vec![
                    "-Sdeps".to_string(),
                    format!(
                        "{{:deps {{nrepl/nrepl {{:mvn/version \"{NREPL_VERSION}\"}} \
                         cider/cider-nrepl {{:mvn/version \"{CIDER_NREPL_VERSION}\"}}}}}}"
                    ),
                    "-M".to_string(),
                    "-m".to_string(),
                    "nrepl.cmdline".to_string(),
                    "--port".to_string(),
                    port.to_string(),
                    "--middleware".to_string(),
                    cider_middleware,
                ],
                PortSource::Fixed(port),
            ),
            ProjectKind::Leiningen => (
                "lein",
                vec![
                    "update-in".to_string(),
                    ":plugins".to_string(),
                    "conj".to_string(),
                    format!("[cider/cider-nrepl \"{CIDER_NREPL_VERSION}\"]"),
                    "--".to_string(),
                    "repl".to_string(),
                    ":headless".to_string(),
                    ":port".to_string(),
                    port.to_string(),
                ],
                PortSource::Fixed(port),
            ),
            ProjectKind::ShadowCljs => (
                "npx",
                vec![
                    "shadow-cljs".to_string(),
                    "-d".to_string(),
                    format!("nrepl/nrepl:{NREPL_VERSION}"),
                    "-d".to_string(),
                    format!("cider/cider-nrepl:{CIDER_NREPL_VERSION}"),
                    "server".to_string(),
                ],
                PortSource::PortFile(root.join(".shadow-cljs").join("nrepl.port")),
            ),
            ProjectKind::Babashka => (
                "bb",
                vec!["nrepl-server".to_string(), format!("localhost:{port}")],
                PortSource::Fixed(port),
            ),
        };
        Self {
            kind,
            program: program.to_string(),
            args,
            cwd: root.to_path_buf(),
            port: port_source,
        }
    }

    /// The command line, for display in the sessions panel.
    pub fn display(&self) -> String {
        let mut line = self.program.clone();
        for arg in &self.args {
            line.push(' ');
            if arg.contains(char::is_whitespace) || arg.contains('"') {
                line.push('\'');
                line.push_str(arg);
                line.push('\'');
            } else {
                line.push_str(arg);
            }
        }
        line
    }
}

/// Asks the OS for a free TCP port on localhost.
pub fn free_port() -> Result<u16> {
    let listener = TcpListener::bind((Ipv4Addr::LOCALHOST, 0)).context("finding a free port")?;
    Ok(listener.local_addr()?.port())
}

/// Shared tail of the server's stdout/stderr.
#[derive(Clone, Default)]
pub struct ProcessOutput(Arc<Mutex<VecDeque<String>>>);

impl ProcessOutput {
    fn push(&self, line: String) {
        let mut lines = self.0.lock();
        if lines.len() == OUTPUT_TAIL_LINES {
            lines.pop_front();
        }
        lines.push_back(line);
    }

    pub fn lines(&self) -> Vec<String> {
        self.0.lock().iter().cloned().collect()
    }

    fn tail(&self, count: usize) -> String {
        let lines = self.0.lock();
        let skip = lines.len().saturating_sub(count);
        lines
            .iter()
            .skip(skip)
            .cloned()
            .collect::<Vec<_>>()
            .join("\n")
    }
}

/// A running jack-in server. Dropping it kills the process.
pub struct JackInProcess {
    pub command: JackInCommand,
    pub output: ProcessOutput,
    child: Child,
    _output_tasks: Vec<Task<()>>,
}

impl JackInProcess {
    pub fn spawn(command: JackInCommand, executor: &BackgroundExecutor) -> Result<Self> {
        let mut child = new_command(&command.program)
            .args(&command.args)
            .current_dir(&command.cwd)
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .kill_on_drop(true)
            .spawn()
            .with_context(|| {
                format!(
                    "starting `{}` — is it installed and on your PATH?",
                    command.program
                )
            })?;

        let output = ProcessOutput::default();
        let mut output_tasks = Vec::new();
        if let Some(stdout) = child.stdout.take() {
            output_tasks.push(forward_lines(
                BufReader::new(stdout),
                output.clone(),
                executor,
            ));
        }
        if let Some(stderr) = child.stderr.take() {
            output_tasks.push(forward_lines(
                BufReader::new(stderr),
                output.clone(),
                executor,
            ));
        }

        Ok(Self {
            command,
            output,
            child,
            _output_tasks: output_tasks,
        })
    }

    /// Waits until the server accepts connections and returns its address.
    pub async fn wait_until_ready(
        &mut self,
        host: Ipv4Addr,
        executor: &BackgroundExecutor,
    ) -> Result<SocketAddr> {
        let started_at = Instant::now();
        let started_at_system = SystemTime::now();
        loop {
            if let Some(status) = self.child.try_status()? {
                bail!(
                    "`{}` exited ({status}) before the nREPL server started:\n{}",
                    self.command.program,
                    self.output.tail(15)
                );
            }

            let port = match &self.command.port {
                PortSource::Fixed(port) => Some(*port),
                PortSource::PortFile(path) => read_fresh_port_file(path, started_at_system),
            };
            if let Some(port) = port {
                let addr = SocketAddr::new(host.into(), port);
                if smol::net::TcpStream::connect(addr).await.is_ok() {
                    return Ok(addr);
                }
            }

            if started_at.elapsed() > STARTUP_TIMEOUT {
                return Err(anyhow!(
                    "timed out after {}s waiting for the nREPL server:\n{}",
                    STARTUP_TIMEOUT.as_secs(),
                    self.output.tail(15)
                ));
            }
            executor.timer(POLL_INTERVAL).await;
        }
    }
}

fn forward_lines<R>(
    reader: BufReader<R>,
    output: ProcessOutput,
    executor: &BackgroundExecutor,
) -> Task<()>
where
    R: futures::AsyncRead + Unpin + Send + 'static,
{
    executor.spawn(async move {
        let mut lines = reader.lines();
        use futures::StreamExt as _;
        while let Some(Ok(line)) = lines.next().await {
            output.push(line);
        }
    })
}

/// Reads a port file, ignoring it if it predates `not_before` (left over
/// from an earlier server).
fn read_fresh_port_file(path: &Path, not_before: SystemTime) -> Option<u16> {
    let modified = std::fs::metadata(path).ok()?.modified().ok()?;
    if modified < not_before {
        return None;
    }
    std::fs::read_to_string(path).ok()?.trim().parse().ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clojure_cli_command_injects_cider() {
        let command = JackInCommand::new(ProjectKind::ClojureCli, Path::new("/p"), 7888);
        assert_eq!(command.program, "clojure");
        assert!(
            command
                .args
                .iter()
                .any(|arg| arg.contains("cider/cider-nrepl"))
        );
        assert!(command.args.windows(2).any(|w| w == ["--port", "7888"]));
        assert_eq!(command.port, PortSource::Fixed(7888));
    }

    #[test]
    fn leiningen_command_uses_headless_repl() {
        let command = JackInCommand::new(ProjectKind::Leiningen, Path::new("/p"), 7888);
        assert_eq!(command.program, "lein");
        assert!(command.args.ends_with(&[
            "repl".to_string(),
            ":headless".to_string(),
            ":port".to_string(),
            "7888".to_string()
        ]));
    }

    #[test]
    fn shadow_cljs_reads_port_file() {
        let command = JackInCommand::new(ProjectKind::ShadowCljs, Path::new("/p"), 1);
        assert_eq!(
            command.port,
            PortSource::PortFile(PathBuf::from("/p/.shadow-cljs/nrepl.port"))
        );
    }

    #[test]
    fn detects_project_kind() {
        let dir = std::env::temp_dir().join(format!("nrepl-jack-in-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        assert_eq!(detect_project(&dir), None);
        std::fs::write(dir.join("bb.edn"), "{}").unwrap();
        assert_eq!(detect_project(&dir), Some(ProjectKind::Babashka));
        std::fs::write(dir.join("deps.edn"), "{}").unwrap();
        assert_eq!(detect_project(&dir), Some(ProjectKind::ClojureCli));
        std::fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn output_keeps_a_bounded_tail() {
        let output = ProcessOutput::default();
        for index in 0..(OUTPUT_TAIL_LINES + 5) {
            output.push(index.to_string());
        }
        assert_eq!(output.lines().len(), OUTPUT_TAIL_LINES);
        assert_eq!(output.tail(1), (OUTPUT_TAIL_LINES + 4).to_string());
    }
}
