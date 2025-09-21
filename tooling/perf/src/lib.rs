//! Some constants and datatypes used in the Zed perf profiler. Should only be
//! consumed by the crate providing the matching macros.

/// The suffix on the actual test function.
pub const SUF_NORMAL: &str = "__ZED_PERF_FN";
/// The suffix on an extra function which prints metadata about a test to stdout.
pub const SUF_MDATA: &str = "__ZED_PERF_MDATA";
/// The env var in which we pass the iteration count to our tests.
pub const ITER_ENV_VAR: &str = "ZED_PERF_ITER";
/// The prefix printed on all benchmark test metadata lines, to distinguish it from
/// possible output by the test harness itself.
pub const MDATA_LINE_PREF: &str = "ZED_MDATA_";
/// The version number for the data returned from the test metadata function.
/// Increment on non-backwards-compatible changes.
pub const MDATA_VER: u32 = 0;
/// The default weight, if none is specified.
pub const WEIGHT_DEFAULT: u8 = 50;

/// Identifier for the iteration count of a test metadata.
pub const ITER_COUNT_LINE_NAME: &str = "iter_count";
/// Identifier for the weight of a test metadata.
pub const WEIGHT_LINE_NAME: &str = "weight";
/// Identifier for importance in test metadata.
pub const IMPORTANCE_LINE_NAME: &str = "importance";
/// Identifier for the test metadata version.
pub const VERSION_LINE_NAME: &str = "version";

/// How relevant a benchmark is.
#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub enum Importance {
    /// Regressions shouldn't be accepted without good reason.
    Critical = 4,
    /// Regressions should be paid extra attention.
    Important = 3,
    /// No extra attention should be paid to regressions, but they might still
    /// be indicative of something happening.
    #[default]
    Average = 2,
    /// Unclear if regressions are likely to be meaningful, but still worth keeping
    /// an eye on. Lowest level that's checked by default by the profiler.
    Iffy = 1,
    /// Regressions are likely to be spurious or don't affect core functionality.
    /// Only relevant if a lot of them happen, or as supplemental evidence for a
    /// higher-importance benchmark regressing. Not checked by default.
    Fluff = 0,
}

impl std::fmt::Display for Importance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Importance::Critical => f.write_str("critical"),
            Importance::Important => f.write_str("important"),
            Importance::Average => f.write_str("average"),
            Importance::Iffy => f.write_str("iffy"),
            Importance::Fluff => f.write_str("fluff"),
        }
    }
}
