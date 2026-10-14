//! P7 (ADR-0034): every RPC command, server notification and UI request in
//! the vendored `wire.rs` has a row in `cedian/OMP_PARITY.md`. An OMP pin bump
//! regenerates `wire.rs`, so a new OMP feature fails this test until the
//! ledger says where it surfaces in cedian. Every row's status is one of
//! ADR-0057's, so the ledger cannot hold an undecided row.

use std::path::PathBuf;

fn repo_file(rel: &str) -> String {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(rel);
    std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()))
}

/// `"name"` literals in `text`, in order.
fn quoted(text: &str) -> Vec<String> {
    text.split('"')
        .skip(1)
        .step_by(2)
        .map(str::to_string)
        .collect()
}

/// Wire `type` of every command (`const NAME: &'static str = "…"`).
fn commands(wire: &str) -> Vec<String> {
    wire.lines()
        .filter(|l| l.contains("const NAME: &'static str ="))
        .flat_map(quoted)
        .collect()
}

/// Every server-to-client notification frame type: the alternation on the
/// line that decodes into `Self::RpcNotification`.
fn notifications(wire: &str) -> Vec<String> {
    let line = wire
        .lines()
        .find(|l| l.contains(".map(Self::RpcNotification)"))
        .expect("RpcNotification decoder line in wire.rs");
    let alternation = line.split("=>").next().unwrap_or_default();
    quoted(alternation)
}

/// Every UI request method (`("method", "…")` on extension UI requests).
fn ui_requests(wire: &str) -> Vec<String> {
    wire.lines()
        .filter(|l| l.contains("(\"type\", \"extension_ui_request\"), (\"method\","))
        .filter_map(|l| l.split("(\"method\", \"").nth(1))
        .filter_map(|rest| rest.split('"').next())
        .map(str::to_string)
        .collect()
}

#[test]
fn every_omp_feature_in_wire_rs_has_a_parity_row() {
    let wire = repo_file("vendor/omp-rpc/src/wire.rs");
    let ledger = repo_file("cedian/OMP_PARITY.md");

    let groups = [
        ("command", commands(&wire), 60),
        ("notification", notifications(&wire), 40),
        ("ui request", ui_requests(&wire), 10),
    ];
    let mut missing = Vec::new();
    for (kind, names, floor) in &groups {
        assert!(
            names.len() >= *floor,
            "parsed only {} {kind}s from wire.rs; the parser no longer matches the generated file",
            names.len()
        );
        for name in names {
            if !ledger.contains(&format!("`{name}`")) {
                missing.push(format!("{kind} `{name}`"));
            }
        }
    }
    assert!(
        missing.is_empty(),
        "cedian/OMP_PARITY.md has no row for {} OMP feature(s) (ADR-0034):\n  {}",
        missing.len(),
        missing.join("\n  ")
    );
}

/// The status cell of every table row, by its table's `Status` header, with
/// its first column; a row with another number of cells has an empty one.
fn statuses(ledger: &str) -> Vec<(String, String)> {
    let mut header: Option<(usize, usize)> = None;
    let mut out = Vec::new();
    for line in ledger.lines() {
        if !line.starts_with("| ") {
            if !line.starts_with("|---") {
                header = None;
            }
            continue;
        }
        let cells: Vec<&str> = line.trim_matches('|').split('|').map(str::trim).collect();
        match header {
            None => {
                header = cells
                    .iter()
                    .position(|c| *c == "Status")
                    .map(|i| (i, cells.len()))
            }
            Some((at, width)) => {
                let status = if cells.len() == width { cells[at] } else { "" };
                out.push((cells[0].to_string(), status.to_string()));
            }
        }
    }
    out
}

/// Why `part` (one `;`-separated part of a status cell) is outside ADR-0057's
/// vocabulary, if it is. A status is its exact word, then nothing or a
/// parenthesised note.
fn status_error(part: &str) -> Option<&'static str> {
    if let Some(slice) = part.strip_prefix("deferred: S") {
        let digits = slice.chars().take_while(char::is_ascii_digit).count();
        let rest = &slice[digits..];
        let named = digits > 0 && (rest.is_empty() || rest.starts_with(' '));
        return (!named).then_some("`deferred` names no slice");
    }
    let (word, note) = match part.split_once(" (") {
        Some((word, note)) if note.ends_with(')') => (word, note),
        Some(_) => return Some("not an ADR-0057 status"),
        None => (part, ""),
    };
    let needs = |what: &str, why: &'static str| (!note.contains(what)).then_some(why);
    match word {
        "native" | "pinned" | "internal" | "reviewer" => None,
        "gated" => needs("ADR-", "`gated` names no ADR"),
        "unused by decision" => needs("ADR-", "`unused by decision` names no ADR"),
        "upstream-blocked" => needs("http", "`upstream-blocked` links no upstream issue"),
        _ => Some("not an ADR-0057 status"),
    }
}

/// `status` split on the `;`s outside parentheses and brackets.
fn parts(status: &str) -> Vec<&str> {
    let mut depth = 0i32;
    let mut start = 0;
    let mut out = Vec::new();
    for (i, c) in status.char_indices() {
        match c {
            '(' | '[' => depth += 1,
            ')' | ']' => depth -= 1,
            ';' if depth == 0 => {
                out.push(status[start..i].trim());
                start = i + 1;
            }
            _ => {}
        }
    }
    out.push(status[start..].trim());
    out
}

#[test]
fn every_parity_row_has_an_adr_0057_status() {
    let ledger = repo_file("cedian/OMP_PARITY.md");
    let rows = statuses(&ledger);
    assert!(rows.len() >= 90, "parsed only {} ledger rows", rows.len());
    let bad: Vec<String> = rows
        .iter()
        .flat_map(|(name, status)| {
            parts(status)
                .into_iter()
                .filter_map(|part| status_error(part).map(|why| format!("{name}: {why}: {part}")))
                .collect::<Vec<_>>()
        })
        .collect();
    assert!(
        bad.is_empty(),
        "{} ledger status(es) outside ADR-0057's vocabulary:\n  {}",
        bad.len(),
        bad.join("\n  ")
    );
}

/// The rows still `deferred: S9`, which S9 cannot close with. Listed here
/// and printed, so a row cannot slip into or out of S9's partial exit
/// unnoticed: changing one means changing this list.
const DEFERRED_IN_S9: &[&str] = &[];

#[test]
fn the_rows_deferred_in_s9_are_the_listed_ones() {
    let ledger = repo_file("cedian/OMP_PARITY.md");
    let in_s9: Vec<String> = statuses(&ledger)
        .into_iter()
        .filter(|(_, status)| {
            parts(status)
                .iter()
                .any(|part| part.starts_with("deferred: S9 ") || *part == "deferred: S9")
        })
        .map(|(name, _)| name)
        .collect();
    println!("deferred: S9 (S9 does not close with these):");
    for name in &in_s9 {
        println!("  {name}");
    }
    assert_eq!(in_s9, DEFERRED_IN_S9);
}

#[test]
fn status_vocabulary_refuses_what_adr_0057_does() {
    for ok in [
        "native (S9 U5)",
        "gated (ADR-0035)",
        "upstream-blocked (https://github.com/can1357/oh-my-pi/issues/1)",
        "pinned",
        "unused by decision (ADR-0050)",
        "internal",
        "reviewer",
        "deferred: S10",
        "deferred: S9 U11",
    ] {
        assert_eq!(status_error(ok), None, "{ok}");
    }
    for bad in [
        "headless",
        "planned S9",
        "deferred: later",
        "deferred",
        "gated",
        "upstream-blocked",
        "nativeX",
        "natively planned S12",
        "internal, later",
        "internalfoo",
        "reviewer anything",
        "pinned later",
        "deferred: S10x",
    ] {
        assert!(status_error(bad).is_some(), "{bad}");
    }
    let table = "| Command | cedian surface | Status |\n|---|---|---|\n\
                 | `a` | the panel | native |\n| `b` | native |\n";
    assert_eq!(
        statuses(table),
        [
            ("`a`".to_string(), "native".to_string()),
            ("`b`".to_string(), String::new())
        ],
        "a row short of the header's cells has no status"
    );
    assert_eq!(
        parts("native (a; b); deferred: S10"),
        ["native (a; b)", "deferred: S10"]
    );
}
