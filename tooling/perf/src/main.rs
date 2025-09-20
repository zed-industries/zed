#![warn(clippy::all, clippy::pedantic, clippy::undocumented_unsafe_blocks)]
#![cfg_attr(release, deny(warnings))]

//! Perf profiler for Zed tests. Outputs timings of tests marked with the `#[perf]`
//! attribute to stdout in Markdown. See the documentation of `util_macros::perf`
//! for usage details on the actual attribute.
//!
//! # Setup
//! Make sure `hyperfine` is installed and in the shell path, then run
//! `cargo build --bin perf --workspace --release` to build the profiler.
//!
//! # Usage
//! Calling this tool rebuilds everything with some cfg flags set for the perf
//! proc macro *and* enables optimisations (`release-fast` profile), so expect it
//! to take a little while.
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
//! # Notes
//! This should probably not be called manually unless you're working on the profiler
//! itself; use the `cargo perf-test` alias (after building this crate) instead.

use std::{
    process::{Command, Stdio},
    time::{Duration, Instant},
};

/// How many iterations to attempt the first time a test is run.
const DEFAULT_ITER_COUNT: usize = 12;
/// Multiplier for the iteration count when a test doesn't pass the noise cutoff.
const ITER_COUNT_MUL: usize = 4;
/// How long a test must have run to be assumed to be reliable-ish.
const NOISE_CUTOFF: Duration = Duration::from_millis(250);

// If any of the below constants are changed, make sure to also update the perf
// proc macro to match!

/// The suffix on tests marked with `#[perf]`.
const SUF_NORMAL: &str = "__ZED_PERF";
/// The suffix on tests marked with `#[perf(iterations = n)]`.
const SUF_FIXED: &str = "__ZED_PERF_FIXEDITER";
/// The env var in which we pass the iteration count to our tests.
const ITER_ENV_VAR: &str = "ZED_PERF_ITER";

#[allow(clippy::too_many_lines)]
fn main() {
    // We get passed the test we need to run as the 1st argument after our own name.
    let test_bin = std::env::args().nth(1).unwrap();
    let mut cmd = Command::new(&test_bin);
    // --format=json is nightly-only :(
    cmd.args(["--list", "--format=terse"]);
    let out = cmd
        .output()
        .expect("FATAL: Could not run test binary {test_bin}");
    assert!(
        out.status.success(),
        "FATAL: Cannot do perf check - test binary {test_bin} returned an error"
    );
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
        .filter(|t_name| t_name.ends_with(SUF_NORMAL) || t_name.ends_with(SUF_FIXED))
        .collect();

    // Pulling itertools just for .dedup() would be quite a big dependency that's
    // not used elsewhere, so do this on the vec instead.
    test_list.sort_unstable();
    test_list.dedup();

    if !test_list.is_empty() {
        // Print the markdown header which matches hyperfine's result.
        // TODO: Support exporting JSON also.
        println!(
            "| Command | Mean [ms] | Min [ms] | Max [ms] | Iterations | Iter/sec |\n|:---|---:|---:|---:|---:|---:|"
        );
    }

    // Spawn and profile an instance of each perf-sensitive test, via hyperfine.
    for t_name in test_list {
        // Pretty-print the stripped name for the test.
        let t_name_normal = t_name.replace(SUF_FIXED, "").replace(SUF_NORMAL, "");
        // Time test execution to see how many iterations we need to do in order
        // to account for random noise. This is skipped for tests with fixed
        // iteration counts.
        let final_iter_count = if t_name.ends_with(SUF_FIXED) {
            None
        } else {
            let mut iter_count = DEFAULT_ITER_COUNT;
            loop {
                let mut cmd = Command::new(&test_bin);
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
                    println!(
                        "| {t_name_normal} (ERRORED IN TRIAGE) | N/A | N/A | N/A | {iter_count} | N/A |"
                    );
                    return;
                }
                if post - pre > NOISE_CUTOFF {
                    break Some(iter_count);
                } else if let Some(c) = iter_count.checked_mul(ITER_COUNT_MUL) {
                    iter_count = c;
                } else {
                    // This should almost never happen, but maybe..?
                    eprintln!(
                        "WARNING: Running nearly usize::MAX iterations of test {t_name_normal}"
                    );
                    break Some(iter_count);
                }
            }
        };

        // Now profile!
        let mut perf_cmd = Command::new("hyperfine");
        // Warm up the cache and print markdown output to stdout.
        perf_cmd.args([
            "--style",
            "none",
            "--warmup",
            "1",
            "--export-markdown",
            "-",
            &format!("{test_bin} {t_name}"),
        ]);
        if let Some(final_iter_count) = final_iter_count {
            perf_cmd.env(ITER_ENV_VAR, format!("{final_iter_count}"));
        }
        let p_out = perf_cmd.output().unwrap();
        let fin_iter = match final_iter_count {
            Some(i) => &format!("{i}"),
            None => "(preset)",
        };
        if p_out.status.success() {
            let output = String::from_utf8_lossy(&p_out.stdout);
            // Strip the name of the test binary from the table (and the space after it!)
            // + our extraneous test bits + the "Relative" column (which is always at the end and "1.00").
            let output = output
                .replace(&format!("{test_bin} "), "")
                .replace(SUF_FIXED, "")
                .replace(SUF_NORMAL, "")
                .replace(" 1.00 |", "");
            // Can't use .last() since we have a trailing newline. Sigh.
            let fin = output.lines().nth(3).unwrap();

            // Calculate how many iterations this does per second, for easy comparison.
            let ms = fin
                .split_whitespace()
                .nth(3)
                .unwrap()
                .parse::<f64>()
                .unwrap();
            let mul_fac = 1000.0 / ms;
            let iter_sec = match final_iter_count {
                #[allow(clippy::cast_precision_loss)]
                Some(c) => &format!("{:.1}", mul_fac * c as f64),
                None => "(unknown)",
            };
            println!("{fin} {fin_iter} | {iter_sec} |");
        } else {
            println!("{t_name_normal} (ERRORED) | N/A | N/A | N/A | {fin_iter} | N/A |");
        }
    }
}
