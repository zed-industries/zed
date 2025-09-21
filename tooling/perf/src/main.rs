//! Perf profiler for Zed tests. Outputs timings of tests marked with the `#[perf]`
//! attribute to stdout in Markdown. See the documentation of `util_macros::perf`
//! for usage details on the actual attribute.
//!
//! # Setup
//! Make sure `hyperfine` is installed and in the shell path, then run
//! `cargo build --bin perf --workspace --release` to build the profiler.
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
//! # Notes
//! This should probably not be called manually unless you're working on the profiler
//! itself; use the `cargo perf-test` alias (after building this crate) instead.

#[allow(clippy::wildcard_imports, reason = "Our crate")]
use perf::*;

use std::{
    process::{Command, Stdio},
    time::{Duration, Instant},
};

/// How many iterations to attempt the first time a test is run.
const DEFAULT_ITER_COUNT: usize = 3;
/// Multiplier for the iteration count when a test doesn't pass the noise cutoff.
const ITER_COUNT_MUL: usize = 4;
/// How long a test must have run to be assumed to be reliable-ish.
const NOISE_CUTOFF: Duration = Duration::from_millis(250);

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

/// Why or when did this test fail?
#[derive(Clone, Debug)]
enum FailKind {
    /// Failed while triaging it to determine the iteration count.
    Triage,
    /// Failed while profiling it.
    Profile,
    /// Failed due to an incompatible version for the test.
    VersionMismatch,
    /// Skipped due to filters applied on the perf run.
    Skipped,
}

impl std::fmt::Display for FailKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FailKind::Triage => f.write_str("failed in triage"),
            FailKind::Profile => f.write_str("failed while profiling"),
            FailKind::VersionMismatch => f.write_str("test version mismatch"),
            FailKind::Skipped => f.write_str("skipped"),
        }
    }
}

/// Information about a given perf test.
#[derive(Clone, Debug)]
struct TestMdata {
    /// A version number for when the test was generated. If this is greater
    /// than the version this test handler expects, one of the following will
    /// happen in an unspecified manner:
    /// - The test is skipped silently.
    /// - The handler exits with an error message indicating the version mismatch
    ///   or inability to parse the metadata.
    ///
    /// INVARIANT: If `version` <= `MDATA_VER`, this tool *must* be able to
    /// correctly parse the output of this test.
    _version: u32,
    /// How many iterations to pass this test, if this is preset.
    iterations: Option<usize>,
    /// The importance of this particular test. See the docs on `Importance` for
    /// details.
    importance: Importance,
    /// The weight of this particular test within its importance category. Used
    /// when comparing across runs.
    weight: u8,
}

impl TestMdata {
    /// Runs a given metadata-returning function from a test handler, parsing its
    /// output into a `TestMdata`.
    fn parse(test_bin: &str, mdata_fn: &str) -> Result<Self, FailKind> {
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
        let mut weight = WEIGHT_DEFAULT;
        for line in stdout
            .lines()
            .filter_map(|l| l.strip_prefix(MDATA_LINE_PREF))
        {
            let mut items = line.split_whitespace();
            // For v0, we know the ident always comes first, then one field.
            match items.next().unwrap() {
                VERSION_LINE_NAME => {
                    let v = items.next().unwrap().parse::<u32>().unwrap();
                    if v > MDATA_VER {
                        return Err(FailKind::VersionMismatch);
                    }
                    version = Some(v);
                }
                ITER_COUNT_LINE_NAME => {
                    iterations = Some(items.next().unwrap().parse::<usize>().unwrap());
                }
                IMPORTANCE_LINE_NAME => {
                    importance = match items.next().unwrap() {
                        "critical" => Importance::Critical,
                        "important" => Importance::Important,
                        "average" => Importance::Average,
                        "iffy" => Importance::Iffy,
                        "fluff" => Importance::Fluff,
                        _ => unreachable!(),
                    };
                }
                WEIGHT_LINE_NAME => {
                    weight = items.next().unwrap().parse::<u8>().unwrap();
                }
                _ => unreachable!(),
            }
        }

        Ok(TestMdata {
            _version: version.unwrap(),
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
}

/// Aggregate output of all tests run by this handler.
#[derive(Clone, Debug, Default)]
struct Output {
    /// A list of test outputs. Format is `(test_name, iter_count, timings)`.
    /// The latter being set indicates the test succeeded.
    ///
    /// INVARIANT: If the test succeeded, the second field is `Some(mdata)` and
    /// `mdata.iterations` is `Some(_)`.
    tests: Vec<(String, Option<TestMdata>, Result<Timings, FailKind>)>,
}

impl Output {
    /// Reports a success and adds it to this run's `Output`.
    fn success(
        &mut self,
        name: impl AsRef<str>,
        mut mdata: TestMdata,
        iters: usize,
        timings: Timings,
    ) {
        mdata.iterations = Some(iters);
        self.tests
            .push((name.as_ref().to_string(), Some(mdata), Ok(timings)));
    }

    /// Reports a failure and adds it to this run's `Output`. If this test was tried
    /// with some number of iterations (i.e. this was not a version mismatch or skipped
    /// test), it should be reported also.
    ///
    /// Using the `fail!()` macro is usually more convenient.
    fn failure(
        &mut self,
        name: impl AsRef<str>,
        mut mdata: Option<TestMdata>,
        attempted_iters: Option<usize>,
        kind: FailKind,
    ) {
        if let Some(ref mut mdata) = mdata {
            mdata.iterations = attempted_iters;
        }
        self.tests
            .push((name.as_ref().to_string(), mdata, Err(kind)));
    }

    /// Sorts the runs in the output in the order that we want it printed.
    fn sort(&mut self) {
        self.tests.sort_unstable_by(|a, b| match (a, b) {
            // Tests where we got no metadata go at the end.
            ((_, Some(_), _), (_, None, _)) => std::cmp::Ordering::Greater,
            ((_, None, _), (_, Some(_), _)) => std::cmp::Ordering::Less,
            // Then sort by importance, then weight.
            ((_, Some(a_mdata), _), (_, Some(b_mdata), _)) => {
                let c = a_mdata.importance.cmp(&b_mdata.importance);
                if matches!(c, std::cmp::Ordering::Equal) {
                    a_mdata.weight.cmp(&b_mdata.weight)
                } else {
                    c
                }
            }
            // Lastly by name.
            ((a_name, ..), (b_name, ..)) => a_name.cmp(b_name),
        });
    }
}

impl std::fmt::Display for Output {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Don't print the header for an empty run.
        if self.tests.is_empty() {
            return Ok(());
        }

        // We want to print important tests at the top, then alphabetical.
        let mut sorted = self.clone();
        sorted.sort();
        // Markdown header for making a nice little table :>
        f.write_str(
            "| Command | Iter/sec | Mean [ms] | SD [ms] | Iterations | Importance (weight) |\n",
        )?;
        f.write_str("|:---|---:|---:|---:|---:|---:|\n")?;
        for (name, metadata, timings) in &sorted.tests {
            match metadata {
                Some(metadata) => match timings {
                    // Happy path.
                    Ok(timings) => {
                        // If the test succeeded, then metadata.iterations is Some(_).
                        f.write_str(&format!(
                            "| {} | {:.2} | {} | {:.2} | {} | {} ({}) |\n",
                            name,
                            timings.iters_per_sec(metadata.iterations.unwrap()),
                            {
                                // Very small mean runtimes will give inaccurate
                                // results. Should probably also penalise weight.
                                let mean = timings.mean.as_secs_f64() * 1000.;
                                if mean < NOISE_CUTOFF.as_secs_f64() * 1000. / 8. {
                                    format!("{mean:.2} (unreliable)")
                                } else {
                                    format!("{mean:.2}")
                                }
                            },
                            timings.stddev.as_secs_f64() * 1000.,
                            metadata.iterations.unwrap(),
                            metadata.importance,
                            metadata.weight,
                        ))?;
                    }
                    // We have (some) metadata, but the test errored.
                    Err(err) => f.write_str(&format!(
                        "| ({}) {} | N/A | N/A | N/A | {} | {} ({}) |\n",
                        err,
                        name,
                        metadata
                            .iterations
                            .map_or_else(|| "N/A".to_owned(), |i| format!("{i}")),
                        metadata.importance,
                        metadata.weight
                    ))?,
                },
                // No metadata, couldn't even parse the test output.
                None => f.write_str(&format!(
                    "| ({}) {} | N/A | N/A | N/A | N/A | N/A |\n",
                    timings.as_ref().unwrap_err(),
                    name
                ))?,
            }
        }
        f.write_str("\n")?;
        Ok(())
    }
}

/// The actual timings of a test, as measured by Hyperfine.
#[derive(Clone, Debug)]
struct Timings {
    /// Mean runtime for `self.iter_total` runs of this test.
    mean: Duration,
    /// Standard deviation for the above.
    stddev: Duration,
}

impl Timings {
    /// How many iterations does this test seem to do per second?
    #[expect(
        clippy::cast_precision_loss,
        reason = "We only care about a couple sig figs anyways"
    )]
    fn iters_per_sec(&self, total_iters: usize) -> f64 {
        (1000. / self.mean.as_millis() as f64) * total_iters as f64
    }
}

#[expect(clippy::too_many_lines, reason = "This will be split up soon!")]
fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    // We get passed the test we need to run as the 1st argument after our own name.
    let test_bin = &args[1];

    // Whether to skip printing some information to stderr.
    let mut quiet = false;
    // Minimum test importance we care about this run.
    let mut thresh = Importance::Iffy;

    for arg in args.iter().skip(2) {
        match arg.as_str() {
            "--critical" => thresh = Importance::Critical,
            "--important" => thresh = Importance::Important,
            "--average" => thresh = Importance::Average,
            "--iffy" => thresh = Importance::Iffy,
            "--fluff" => thresh = Importance::Fluff,
            "--quiet" => quiet = true,
            _ => (),
        }
    }
    if !quiet {
        eprintln!("Starting perf check...");
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
        .filter(|t_name| t_name.ends_with(SUF_NORMAL) || t_name.ends_with(SUF_MDATA))
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
        let (t_name, t_mdata) = if SUF_NORMAL < SUF_MDATA {
            (t_pair[0], t_pair[1])
        } else {
            (t_pair[1], t_pair[0])
        };
        // Pretty-printable stripped name for the test.
        let t_name_pretty = t_name.replace(SUF_NORMAL, "");

        // Get the metadata this test reports for us.
        let t_mdata = match TestMdata::parse(test_bin, t_mdata) {
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
                cmd.env(ITER_ENV_VAR, format!("{iter_count}"));
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
                if post - pre > NOISE_CUTOFF {
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
        perf_cmd.env(ITER_ENV_VAR, format!("{final_iter_count}"));
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
        if output.tests.is_empty() {
            eprintln!("Nothing to do.");
        } else {
            // If stdout and stderr are on the same terminal, move us after the
            // output from above (with an extra newline for good measure).
            eprintln!("\n");
        }
    }
    print!("{output}");
}
