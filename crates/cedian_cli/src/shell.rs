//! `cedian shell` (ADR-0021 / P4): one long-lived, foreground headless
//! process per workspace. Holds ONE OMP runtime across turns, so a turn can
//! be steered or aborted while it streams. Holds `shell.lock` in the state dir, so
//! mutating one-shot commands refuse while it runs. Dies with its terminal.
//!
//! Verbs: `prompt <msg>`, `edit <path> <start>-<end> <instruction>
//! [--model provider/id]` (inline edit, P6), `steer <msg>`, `abort`, `help`,
//! `quit`; any other
//! line runs the one-shot command of the same name in-process (`review`,
//! `accept <path> <hunk>`, `workflow status`, …).

use crate::shell_lock::ShellLock;
use cedian_omp::{OmpRuntime, RuntimeControl};
use cedian_workspace::HostTools;
use std::{
    io::BufRead as _,
    path::{Path, PathBuf},
    sync::{Arc, mpsc},
    thread::JoinHandle,
    time::Duration,
};

const HELP: &str = "prompt <msg> | edit <path> <start>-<end> <instruction> [--model p/id] | \
steer <msg> | abort | review | <any cedian verb> | help | quit";

type TurnResult = (OmpRuntime, Result<(), String>);

struct Running {
    turn: JoinHandle<TurnResult>,
    control: RuntimeControl,
}

/// Split a shell line into words; `"double quotes"` keep spaces.
fn words(line: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut cur = String::new();
    let mut quoted = false;
    for c in line.chars() {
        match c {
            '"' => quoted = !quoted,
            c if c.is_whitespace() && !quoted => {
                if !cur.is_empty() {
                    out.push(std::mem::take(&mut cur));
                }
            }
            c => cur.push(c),
        }
    }
    if !cur.is_empty() {
        out.push(cur);
    }
    out
}

/// The rest of the line after the verb, verbatim (prompt/steer text).
fn rest(line: &str) -> &str {
    let line = line.trim_start();
    line.split_once(char::is_whitespace)
        .map(|(_, r)| r.trim())
        .unwrap_or("")
}

pub fn run(
    session_dir: &Path,
    workdir: &Path,
    settings: &cedian_shell::Settings,
) -> Result<(), String> {
    let _lock = ShellLock::acquire(workdir)?;
    let host = HostTools::shared(workdir);
    let started = std::time::Instant::now();
    let mut rt = crate::spawn(session_dir, workdir, settings, &host)?;
    rt.open_session("shell").map_err(|e| e.to_string())?;
    cedian_shell::timing::record(cedian_shell::timing::Row::Spawn(started.elapsed()));
    println!(
        "cedian shell — {} (pid {}). {HELP}",
        workdir.display(),
        std::process::id()
    );

    let (lines_tx, lines) = mpsc::channel::<String>();
    std::thread::spawn(move || {
        for line in std::io::stdin().lock().lines() {
            let Ok(line) = line else { break };
            if lines_tx.send(line).is_err() {
                break;
            }
        }
    });

    let mut idle: Option<OmpRuntime> = Some(rt);
    let mut running: Option<Running> = None;
    loop {
        if running.as_ref().is_some_and(|r| r.turn.is_finished()) {
            idle = Some(finish(running.take())?);
        }
        let line = match lines.recv_timeout(Duration::from_millis(100)) {
            Ok(line) => line,
            Err(mpsc::RecvTimeoutError::Timeout) => continue,
            Err(mpsc::RecvTimeoutError::Disconnected) => break, // EOF
        };
        let args = words(&line);
        let Some(verb) = args.first().map(String::as_str) else {
            continue;
        };
        match (verb, &running) {
            ("quit" | "exit", _) => break,
            ("help", _) => println!("{HELP}"),
            ("steer", Some(r)) => report(r.control.steer(rest(&line)).map(|()| "steered")),
            ("abort", Some(r)) => report(r.control.abort().map(|()| "abort sent")),
            ("steer" | "abort", None) => println!("no turn running"),
            (_, Some(_)) => println!("turn running — only steer, abort, help, quit"),
            ("prompt", None) => {
                let message = rest(&line).to_string();
                if message.is_empty() {
                    println!("usage: prompt <message>");
                    continue;
                }
                let Some(rt) = idle.take() else {
                    return Err("runtime lost".to_string());
                };
                running = Some(start_turn(
                    rt,
                    &host,
                    workdir.to_path_buf(),
                    Turn::prompt(message),
                ));
            }
            ("edit", None) => match InlineEdit::parse(&args, workdir) {
                Ok(edit) => {
                    let Some(rt) = idle.take() else {
                        return Err("runtime lost".to_string());
                    };
                    host.set_selection(Some((edit.key.clone(), edit.start_byte, edit.end_byte)));
                    host.set_active_file(Some(edit.key.clone()));
                    running = Some(start_turn(
                        rt,
                        &host,
                        workdir.to_path_buf(),
                        Turn::Edit(edit),
                    ));
                }
                Err(e) => println!("error: {e}"),
            },
            (_, None) => {
                if let Err(e) = crate::dispatch(args, true) {
                    println!("error: {e}");
                }
            }
        }
    }

    // Leaving: abort a running turn, then shut the runtime down cleanly.
    if let Some(r) = &running {
        let _ = r.control.abort();
    }
    if running.is_some() {
        idle = Some(finish(running.take())?);
    }
    if let Some(rt) = idle {
        rt.shutdown().map_err(|e| e.to_string())?;
    }
    println!("cedian shell closed");
    Ok(())
}

/// What a shell turn runs.
enum Turn {
    Prompt(String),
    Edit(InlineEdit),
}

impl Turn {
    fn prompt(message: String) -> Self {
        Self::Prompt(message)
    }
}

/// `edit <path> <start>-<end> <instruction> [--model provider/id]`
/// (ADR-0026 decision 2, headless form): one OMP turn whose explicit
/// target is those lines (1-based, inclusive).
#[derive(Debug)]
struct InlineEdit {
    key: PathBuf,
    start: usize,
    end: usize,
    start_byte: usize,
    end_byte: usize,
    selected: String,
    instruction: String,
    model: Option<(String, String)>,
}

const EDIT_USAGE: &str = "usage: edit <path> <start>-<end> <instruction> [--model provider/id]";

impl InlineEdit {
    fn parse(args: &[String], workdir: &Path) -> Result<Self, String> {
        let mut args: Vec<String> = args[1..].to_vec();
        let mut model = None;
        if let Some(i) = args.iter().position(|a| a == "--model") {
            let spec = args.get(i + 1).ok_or(EDIT_USAGE)?.clone();
            let (provider, id) = spec.split_once('/').ok_or("--model wants provider/id")?;
            model = Some((provider.to_string(), id.to_string()));
            args.drain(i..i + 2);
        }
        let [path, range, instruction @ ..] = args.as_slice() else {
            return Err(EDIT_USAGE.to_string());
        };
        if instruction.is_empty() {
            return Err(EDIT_USAGE.to_string());
        }
        let (start, end) = range
            .split_once('-')
            .and_then(|(a, b)| Some((a.parse::<usize>().ok()?, b.parse::<usize>().ok()?)))
            .filter(|(a, b)| *a >= 1 && a <= b)
            .ok_or(EDIT_USAGE)?;
        let key = PathBuf::from(format!("/{}", path.trim_start_matches('/')));
        let local = cedian_shell::workspace_files::local_path(workdir, &key)
            .ok_or_else(|| format!("path outside the workspace: {path}"))?;
        let text = std::fs::read_to_string(&local).map_err(|e| format!("{path}: {e}"))?;
        let lines: Vec<&str> = text.split_inclusive('\n').collect();
        if end > lines.len() {
            return Err(format!("{path} has {} line(s)", lines.len()));
        }
        let start_byte: usize = lines[..start - 1].iter().map(|l| l.len()).sum();
        let selected: String = lines[start - 1..end].concat();
        Ok(Self {
            key,
            start,
            end,
            start_byte,
            end_byte: start_byte + selected.len(),
            selected,
            instruction: instruction.join(" "),
            model,
        })
    }

    fn rel(&self) -> &str {
        self.key.to_str().unwrap_or("").trim_start_matches('/')
    }

    fn message(&self) -> String {
        format!(
            "Inline edit from the cedian editor. Change ONLY lines {s}-{e} of {p}; leave every \
             other line and every other file untouched. Instruction: {i}\n\
             Current lines {s}-{e} of {p}:\n```\n{t}```\n\
             Make the change with your own edit tool, then reply \
             with only: done",
            s = self.start,
            e = self.end,
            p = self.rel(),
            i = self.instruction,
            t = self.selected,
        )
    }

    fn label(&self) -> String {
        format!(
            "edit {}:{}-{} {}",
            self.rel(),
            self.start,
            self.end,
            self.instruction
        )
    }
}

fn start_turn(mut rt: OmpRuntime, host: &Arc<HostTools>, workdir: PathBuf, turn: Turn) -> Running {
    let control = rt.control();
    let host = Arc::clone(host);
    let turn = std::thread::spawn(move || {
        let result = match &turn {
            Turn::Prompt(message) => crate::run_turn(
                &mut rt,
                &host,
                &workdir,
                message,
                true,
                crate::session::TurnKind::Prompt,
                message,
            ),
            Turn::Edit(edit) => run_edit(&mut rt, &host, &workdir, edit),
        };
        (rt, result)
    });
    Running { turn, control }
}

/// One inline-edit turn: optional per-edit model (restored afterwards),
/// then report anything the turn changed outside the target lines.
fn run_edit(
    rt: &mut OmpRuntime,
    host: &Arc<HostTools>,
    workdir: &Path,
    edit: &InlineEdit,
) -> Result<(), String> {
    let previous = match &edit.model {
        Some((provider, id)) => {
            let before = rt.get_state().map_err(|e| e.to_string())?.model;
            rt.set_model(provider, id).map_err(|e| e.to_string())?;
            before
        }
        None => None,
    };
    let turns_before = crate::session::load(workdir)?.map_or(0, |s| s.turns.len());
    let result = crate::run_turn(
        rt,
        host,
        workdir,
        &edit.message(),
        true,
        crate::session::TurnKind::Edit,
        &edit.label(),
    );
    host.set_selection(None);
    if let Some(model) = previous {
        let _ = rt.set_model(&model.provider, &model.id);
    }
    result?;
    let store = crate::session::load(workdir)?.unwrap_or_default();
    if let Some(turn) = store.turns.get(turns_before) {
        for warning in out_of_range(turn, edit) {
            println!("warning: {warning}");
        }
    }
    Ok(())
}

/// Changes an inline-edit turn made outside its target lines.
fn out_of_range(turn: &crate::session::TurnRecord, edit: &InlineEdit) -> Vec<String> {
    let mut out = Vec::new();
    let target = edit.key.to_string_lossy();
    for file in &turn.files {
        if file.file != target {
            out.push(format!(
                "edit also changed {}",
                file.file.trim_start_matches('/')
            ));
            continue;
        }
        for hunk in cedian_review::line_diff(&file.before, &file.after) {
            let (lo, hi) = (hunk.before_start + 1, hunk.before_start + hunk.before_count);
            let inside = lo >= edit.start && hi <= edit.end && hunk.before_count > 0
                || hunk.before_count == 0 && lo >= edit.start && lo <= edit.end + 1;
            if !inside {
                out.push(format!(
                    "edit changed {} lines {lo}-{} outside {}-{}",
                    edit.rel(),
                    hi.max(lo),
                    edit.start,
                    edit.end
                ));
            }
        }
    }
    out
}

/// Join a finished (or aborted) turn and hand the runtime back.
fn finish(running: Option<Running>) -> Result<OmpRuntime, String> {
    let Running { turn, control } = running.ok_or("no turn")?;
    drop(control); // shutdown needs the only client handle
    let (rt, result) = turn
        .join()
        .map_err(|_| "turn thread panicked".to_string())?;
    match result {
        Ok(()) => println!("(turn done)"),
        Err(e) => println!("(turn failed: {e})"),
    }
    Ok(rt)
}

fn report(result: Result<&str, cedian_omp::OmpError>) {
    match result {
        Ok(msg) => println!("({msg})"),
        Err(e) => println!("error: {e}"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn edit_in(dir: &Path, line: &str) -> Result<InlineEdit, String> {
        InlineEdit::parse(&words(line), dir)
    }

    #[test]
    fn inline_edit_parses_range_model_and_selection() {
        let dir = crate::test_dir::TestDir::new("inline");
        std::fs::write(dir.join("n.txt"), "alpha\nbeta\ngamma\n").unwrap();
        let e = edit_in(&dir, "edit n.txt 2-2 \"make it loud\" --model acme/fast-1").unwrap();
        assert_eq!((e.start, e.end), (2, 2));
        assert_eq!(e.selected, "beta\n");
        assert_eq!((e.start_byte, e.end_byte), (6, 11));
        assert_eq!(e.instruction, "make it loud");
        assert_eq!(e.model, Some(("acme".into(), "fast-1".into())));
        assert!(e.message().contains("Change ONLY lines 2-2 of n.txt"));
        assert!(
            edit_in(&dir, "edit n.txt 3-9 x")
                .unwrap_err()
                .contains("3 line(s)")
        );
        assert!(edit_in(&dir, "edit n.txt 2-1 x").is_err());
        assert!(edit_in(&dir, "edit n.txt 1-1").is_err());

        let turn = crate::session::TurnRecord {
            n: 1,
            kind: crate::session::TurnKind::Edit,
            label: e.label(),
            files: vec![
                crate::session::TurnFile {
                    file: "/n.txt".into(),
                    before: "alpha\nbeta\ngamma\n".into(),
                    after: "alpha\nBETA\nGAMMA\n".into(),
                    created: false,
                },
                crate::session::TurnFile {
                    file: "/other.txt".into(),
                    before: String::new(),
                    after: "x\n".into(),
                    created: true,
                },
            ],
        };
        let warnings = out_of_range(&turn, &e);
        assert_eq!(warnings.len(), 2, "{warnings:?}");
        assert!(
            warnings
                .iter()
                .any(|w| w.contains("also changed other.txt"))
        );
        assert!(
            warnings.iter().any(|w| w.contains("lines 2-3 outside 2-2")),
            "{warnings:?}"
        );
        std::fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn words_and_rest() {
        assert_eq!(
            words(r#"review dismiss f1 "not a bug""#),
            ["review", "dismiss", "f1", "not a bug"]
        );
        assert_eq!(
            words(r#"workflow run bug_fix "fix login""#),
            ["workflow", "run", "bug_fix", "fix login"]
        );
        assert_eq!(rest("prompt  change \"x\" to y "), "change \"x\" to y");
        assert_eq!(rest("abort"), "");
    }
}
