//! Perf profiler for Zed tests. Outputs timings of tests marked with the `#[perf]`
//! attribute to stdout in Markdown. See the documentation of `util_macros::perf`
//! for usage details on the actual attribute.
//!
//! # Setup
//! Make sure `hyperfine` is installed and in the shell path, then run
//! `cargo build -p perf --release` to build the profiler.
//!
//! # Usage
//! Calling this tool rebuilds the targeted crate(s) with some cfg flags set for the
//! perf proc macro *and* enables optimisations (`release-fast` profile), so expect
//! it to take a little while.
//!
//! To test an individual crate, run:
//! ```sh
//! cargo perf-test -p $CRATE
//! ```
//!
//! To test everything (which will be **VERY SLOW**), run:
//! ```sh
//! cargo perf-test --workspace
//! ```
//!
//! Some command-line parameters are also recognised by this profiler. To filter
//! out all tests below a certain importance (e.g. `important`), run:
//! ```sh
//! cargo perf-test $WHATEVER -- --important
//! ```
//!
//! Similarly, to skip outputting progress to the command line, pass `-- --quiet`.
//! These flags can be combined.
//!
//! ## Comparing runs
//! Passing `--json=ident` will save per-crate run files in `.perf-runs`, e.g.
//! `cargo perf-test -p gpui -- --json=blah` will result in `.perf-runs/blah.gpui.json`
//! being created (unless no tests were run). These results can be automatically
//! compared. To do so, run `cargo perf-compare new-ident old-ident`.
//!
//! NB: All files matching `.perf-runs/ident.*.json` will be considered when
//! doing this comparison, so ensure there aren't leftover files in your `.perf-runs`
//! directory that might match that!
//!
//! # Notes
//! This should probably not be called manually unless you're working on the profiler
//! itself; use the `cargo perf-test` alias (after building this crate) instead.

#[allow(clippy::wildcard_imports, reason = "Our crate")]
use perf::*;

use std::{
    fs::OpenOptions,
    io::Write,
    path::{Path, PathBuf},
    process::{Command, Stdio},
    time::{Duration, Instant},
};

/// How many iterations to attempt the first time a test is run.
const DEFAULT_ITER_COUNT: usize = 3;
/// Multiplier for the iteration count when a test doesn't pass the noise cutoff.
const ITER_COUNT_MUL: usize = 4;

/// Report a failure into the output and skip an iteration.
macro_rules! fail {
    ($output:ident, $name:expr, $kind:expr) => {{
        $output.failure($name, None, None, $kind);
        continue;
    }};
    ($output:ident, $name:expr, $mdata:expr, $kind:expr) => {{
        $output.failure($name, Some($mdata), None, $kind);
        continue;
    }};
    ($output:ident, $name:expr, $mdata:expr, $count:expr, $kind:expr) => {{
        $output.failure($name, Some($mdata), Some($count), $kind);
        continue;
    }};
}

/// How does this perf run return its output?
enum OutputKind<'a> {
    /// Print markdown to the terminal.
    Markdown,
    /// Save JSON to a file.
    Json(&'a Path),
}

/// Runs a given metadata-returning function from a test handler, parsing its
/// output into a `TestMdata`.
fn parse_mdata(test_bin: &str, mdata_fn: &str) -> Result<TestMdata, FailKind> {
    let mut cmd = Command::new(test_bin);
    cmd.args([mdata_fn, "--exact", "--nocapture"]);
    let out = cmd
        .output()
        .expect("FATAL: Could not run test binary {test_bin}");
    assert!(out.status.success());
    let stdout = String::from_utf8_lossy(&out.stdout);
    let mut version = None;
    let mut iterations = None;
    let mut importance = Importance::default();
    let mut weight = consts::WEIGHT_DEFAULT;
    for line in stdout
        .lines()
        .filter_map(|l| l.strip_prefix(consts::MDATA_LINE_PREF))
    {
        let mut items = line.split_whitespace();
        // For v0, we know the ident always comes first, then one field.
        match items.next().unwrap() {
            consts::VERSION_LINE_NAME => {
                let v = items.next().unwrap().parse::<u32>().unwrap();
                if v > consts::MDATA_VER {
                    return Err(FailKind::VersionMismatch);
                }
                version = Some(v);
            }
            consts::ITER_COUNT_LINE_NAME => {
                iterations = Some(items.next().unwrap().parse::<usize>().unwrap());
            }
            consts::IMPORTANCE_LINE_NAME => {
                importance = match items.next().unwrap() {
                    "critical" => Importance::Critical,
                    "important" => Importance::Important,
                    "average" => Importance::Average,
                    "iffy" => Importance::Iffy,
                    "fluff" => Importance::Fluff,
                    _ => unreachable!(),
                };
            }
            consts::WEIGHT_LINE_NAME => {
                weight = items.next().unwrap().parse::<u8>().unwrap();
            }
            _ => unreachable!(),
        }
    }

    Ok(TestMdata {
        version: version.unwrap(),
        // Iterations may be determined by us and thus left unspecified.
        iterations,
        // In principle this should always be set, but just for the sake of
        // stability allow the potentially-breaking change of not reporting the
        // importance without erroring. Maybe we want to change this.
        importance,
        // Same with weight.
        weight,
    })
}

/// Compares the perf results of two profiles as per the arguments passed in.
fn compare_profiles(args: &[String]) {
    let ident_new = args.first().expect("FATAL: missing identifier for new run");
    let ident_old = args.get(1).expect("FATAL: missing identifier for old run");
    // TODO: move this to a constant also tbh
    let wspace_dir = std::env::var("CARGO_WORKSPACE_DIR").unwrap();
    let runs_dir = PathBuf::from(&wspace_dir).join(consts::RUNS_DIR);

    // Use the blank outputs initially, so we can merge into these with prefixes.
    let mut outputs_new = Output::blank();
    let mut outputs_old = Output::blank();

    for e in runs_dir.read_dir().unwrap() {
        let Ok(entry) = e else {
            continue;
        };
        let Ok(metadata) = entry.metadata() else {
            continue;
        };
        if metadata.is_file() {
            let Ok(name) = entry.file_name().into_string() else {
                continue;
            };

            // A little helper to avoid code duplication. Reads the `output` from
            // a json file, then merges it into what we have so far.
            let read_into = |output: &mut Output| {
                let mut elems = name.split('.').skip(1);
                let prefix = elems.next().unwrap();
                assert_eq!("json", elems.next().unwrap());
                assert!(elems.next().is_none());
                let handle = OpenOptions::new().read(true).open(entry.path()).unwrap();
                let o_other: Output = serde_json::from_reader(handle).unwrap();
                output.merge(o_other, prefix);
            };

            if name.starts_with(ident_old) {
                read_into(&mut outputs_old);
            } else if name.starts_with(ident_new) {
                read_into(&mut outputs_new);
            }
        }
    }

    let res = outputs_new.compare_perf(outputs_old);
    println!("{res}");
}

#[expect(clippy::too_many_lines, reason = "This will be split up soon!")]
fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    // We get passed the test we need to run as the 1st argument after our own name.
    let test_bin = args
        .get(1)
        .expect("FATAL: No test binary or command; this shouldn't be manually invoked!");

    // We're being asked to compare two results, not run the profiler.
    if test_bin == "compare" {
        compare_profiles(&args[2..]);
        return;
    }

    // Whether to skip printing some information to stderr.
    let mut quiet = false;
    // Minimum test importance we care about this run.
    let mut thresh = Importance::Iffy;
    // Where to print the output of this run.
    let mut out_kind = OutputKind::Markdown;

    for arg in args.iter().skip(2) {
        match arg.as_str() {
            "--critical" => thresh = Importance::Critical,
            "--important" => thresh = Importance::Important,
            "--average" => thresh = Importance::Average,
            "--iffy" => thresh = Importance::Iffy,
            "--fluff" => thresh = Importance::Fluff,
            "--quiet" => quiet = true,
            s if s.starts_with("--json") => {
                out_kind = OutputKind::Json(Path::new(
                    s.strip_prefix("--json=")
                        .expect("FATAL: Invalid json parameter; pass --json=filename"),
                ));
            }
            _ => (),
        }
    }
    if !quiet {
        eprintln!("Starting perf check");
    }

    let mut cmd = Command::new(test_bin);
    // --format=json is nightly-only :(
    cmd.args(["--list", "--format=terse"]);
    let out = cmd
        .output()
        .expect("FATAL: Could not run test binary {test_bin}");
    assert!(
        out.status.success(),
        "FATAL: Cannot do perf check - test binary {test_bin} returned an error"
    );
    if !quiet {
        eprintln!("Test binary ran successfully; starting profile...");
    }
    // Parse the test harness output to look for tests we care about.
    let stdout = String::from_utf8_lossy(&out.stdout);
    let mut test_list: Vec<_> = stdout
        .lines()
        .filter_map(|line| {
            // This should split only in two; e.g.,
            // "app::test::test_arena: test" => "app::test::test_arena:", "test"
            let line: Vec<_> = line.split_whitespace().collect();
            match line[..] {
                // Final byte of t_name is ":", which we need to ignore.
                [t_name, kind] => (kind == "test").then(|| &t_name[..t_name.len() - 1]),
                _ => None,
            }
        })
        // Exclude tests that aren't marked for perf triage based on suffix.
        .filter(|t_name| {
            t_name.ends_with(consts::SUF_NORMAL) || t_name.ends_with(consts::SUF_MDATA)
        })
        .collect();

    // Pulling itertools just for .dedup() would be quite a big dependency that's
    // not used elsewhere, so do this on a vec instead.
    test_list.sort_unstable();
    test_list.dedup();

    let len = test_list.len();

    // Tests should come in pairs with their mdata fn!
    assert!(
        len.is_multiple_of(2),
        "Malformed tests in test binary {test_bin}"
    );

    let mut output = Output::default();

    // Spawn and profile an instance of each perf-sensitive test, via hyperfine.
    // Each test is a pair of (test, metadata-returning-fn), so grab both. We also
    // know the list is sorted.
    for (idx, t_pair) in test_list.chunks_exact(2).enumerate() {
        if !quiet {
            eprint!("\rProfiling test {}/{}", idx + 1, len / 2);
        }
        // Be resilient against changes to these constants.
        let (t_name, t_mdata) = if consts::SUF_NORMAL < consts::SUF_MDATA {
            (t_pair[0], t_pair[1])
        } else {
            (t_pair[1], t_pair[0])
        };
        // Pretty-printable stripped name for the test.
        let t_name_pretty = t_name.replace(consts::SUF_NORMAL, "");

        // Get the metadata this test reports for us.
        let t_mdata = match parse_mdata(test_bin, t_mdata) {
            Ok(mdata) => mdata,
            Err(err) => fail!(output, t_name_pretty, err),
        };

        if t_mdata.importance < thresh {
            fail!(output, t_name_pretty, t_mdata, FailKind::Skipped);
        }

        // Time test execution to see how many iterations we need to do in order
        // to account for random noise. This is skipped for tests with fixed
        // iteration counts.
        let mut errored = false;
        let final_iter_count = t_mdata.iterations.unwrap_or_else(|| {
            let mut iter_count = DEFAULT_ITER_COUNT;
            loop {
                let mut cmd = Command::new(test_bin);
                cmd.args([t_name, "--exact"]);
                cmd.env(consts::ITER_ENV_VAR, format!("{iter_count}"));
                // Don't let the child muck up our stdin/out/err.
                cmd.stdin(Stdio::null());
                cmd.stdout(Stdio::null());
                cmd.stderr(Stdio::null());
                let pre = Instant::now();
                // Discard the output beyond ensuring success.
                let out = cmd.spawn().unwrap().wait();
                let post = Instant::now();
                if !out.unwrap().success() {
                    errored = true;
                    break iter_count;
                }
                if post - pre > consts::NOISE_CUTOFF {
                    break iter_count;
                } else if let Some(c) = iter_count.checked_mul(ITER_COUNT_MUL) {
                    iter_count = c;
                } else {
                    // This should almost never happen, but maybe..?
                    eprintln!(
                        "WARNING: Running nearly usize::MAX iterations of test {t_name_pretty}"
                    );
                    break iter_count;
                }
            }
        });

        // Don't profile failing tests.
        if errored {
            fail!(output, t_name_pretty, t_mdata, FailKind::Triage);
        }

        // Now profile!
        let mut perf_cmd = Command::new("hyperfine");
        // Warm up the cache and print markdown output to stdout.
        // TODO: json
        perf_cmd.args([
            "--style",
            "none",
            "--warmup",
            "1",
            "--export-markdown",
            "-",
            &format!("{test_bin} {t_name}"),
        ]);
        perf_cmd.env(consts::ITER_ENV_VAR, format!("{final_iter_count}"));
        let p_out = perf_cmd.output().unwrap();
        if p_out.status.success() {
            let cmd_output = String::from_utf8_lossy(&p_out.stdout);
            // Can't use .last() since we have a trailing newline. Sigh.
            let results_line = cmd_output.lines().nth(3).unwrap();
            // Grab the values out of the pretty-print.
            // TODO: Parse json instead.
            let mut res_iter = results_line.split_whitespace();
            // Durations are given in milliseconds, so account for that.
            let mean =
                Duration::from_secs_f64(res_iter.nth(4).unwrap().parse::<f64>().unwrap() / 1000.);
            let stddev =
                Duration::from_secs_f64(res_iter.nth(1).unwrap().parse::<f64>().unwrap() / 1000.);

            output.success(
                t_name_pretty,
                t_mdata,
                final_iter_count,
                Timings { mean, stddev },
            );
        } else {
            fail!(
                output,
                t_name_pretty,
                t_mdata,
                final_iter_count,
                FailKind::Profile
            );
        }
    }
    if !quiet {
        if output.is_empty() {
            eprintln!("Nothing to do.");
        } else {
            // If stdout and stderr are on the same terminal, move us after the
            // output from above.
            eprintln!();
        }
    }

    // No need making an empty json file on every empty test bin.
    if output.is_empty() {
        return;
    }

    match out_kind {
        OutputKind::Markdown => print!("{output}"),
        OutputKind::Json(user_path) => {
            let wspace_dir = std::env::var("CARGO_WORKSPACE_DIR").unwrap();
            let runs_dir = PathBuf::from(&wspace_dir).join(consts::RUNS_DIR);
            std::fs::create_dir_all(&runs_dir).unwrap();
            assert!(
                !user_path.to_string_lossy().is_empty(),
                "FATAL: Empty filename specified!"
            );
            // Get the test binary's crate's name; a path like
            // target/release-fast/deps/gpui-061ff76c9b7af5d7
            // would be reduced to just "gpui".
            let test_bin_stripped = Path::new(test_bin)
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .rsplit_once('-')
                .unwrap()
                .0;
            let mut file_path = runs_dir.join(user_path);
            file_path
                .as_mut_os_string()
                .push(format!(".{test_bin_stripped}.json"));
            let mut out_file = OpenOptions::new()
                .write(true)
                .create(true)
                .truncate(true)
                .open(&file_path)
                .unwrap();
            out_file
                .write_all(&serde_json::to_vec(&output).unwrap())
                .unwrap();
            if !quiet {
                eprintln!("JSON output written to {}", file_path.display());
            }
        }
    }
}
