//! Line diff: minimal LCS-based hunks over buffer text (plan §15 pipeline).
//!
//! Deliberately small: review needs changed line ranges, not a full diff
//! engine. Zed `BufferDiff`/`DiffPatch` replaces this under the fork; the
//! [`Hunk`] shape (path + line ranges + status) survives the swap.

/// One changed line range: `before` lines replaced by `after` lines.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Hunk {
    /// 0-based start line in `before`.
    pub before_start: usize,
    /// Line count in `before` (0 = pure insertion).
    pub before_count: usize,
    /// 0-based start line in `after`.
    pub after_start: usize,
    /// Line count in `after` (0 = pure deletion).
    pub after_count: usize,
}

/// Review state of one hunk (§18 R3 precedence: Interrupted > Unattributed >
/// Stale; accepted/rejected are terminal user resolutions). `OutOfScope` is a
/// scope fact (ADR-0033 decision 3), not part of that precedence: it wins over
/// Stale, Pending and Unattributed, and is resolved like Stale.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HunkStatus {
    /// Attributed agent edit, awaiting review.
    #[default]
    Pending,
    /// Crash orphan — reconciled to `Unattributed`, never directly resolved.
    Interrupted,
    /// No correlating `tool_call_id` — per-hunk resolve only, never bulk accept.
    Unattributed,
    /// User edited after the agent — compare/restore manually, never auto-reject.
    Stale,
    /// Changed outside the running workers' briefs (ADR-0033 decision 3):
    /// never auto-resolved — a person resolves it, and merge-back is refused
    /// while one is open.
    OutOfScope,
    /// User accepted (keep-buffer — already in the buffer, just marked).
    Accepted,
    /// User rejected (inverse patch applied).
    Rejected,
}

/// Per-file diff: path + hunks in line order.
#[derive(Debug, Clone)]
pub struct FileDiff {
    pub path: String,
    pub hunks: Vec<Hunk>,
    pub statuses: Vec<HunkStatus>,
    /// Buffer text this diff was computed against. A reject whose buffer no
    /// longer matches refuses (`Outdated`) instead of patching shifted lines.
    pub snapshot: String,
}

impl FileDiff {
    /// Hunk count (== statuses len by construction).
    pub fn len(&self) -> usize {
        self.hunks.len()
    }

    pub fn is_empty(&self) -> bool {
        self.hunks.is_empty()
    }
}

/// Compute hunks between `before` and `after` text (line-granular LCS).
/// Empty vec when identical. Adjacent changes merge into one hunk.
pub fn line_diff(before: &str, after: &str) -> Vec<Hunk> {
    let a: Vec<&str> = before.lines().collect();
    let b: Vec<&str> = after.lines().collect();
    if a == b {
        return Vec::new();
    }
    // LCS table (fine for review-sized buffers; fork swaps in BufferDiff).
    let n = a.len();
    let m = b.len();
    let mut dp = vec![vec![0usize; m + 1]; n + 1];
    for i in (0..n).rev() {
        for j in (0..m).rev() {
            dp[i][j] = if a[i] == b[j] {
                dp[i + 1][j + 1] + 1
            } else {
                dp[i + 1][j].max(dp[i][j + 1])
            };
        }
    }
    // Walk the LCS to find changed ranges.
    let mut hunks = Vec::new();
    let (mut i, mut j) = (0, 0);
    let (mut cs, mut ds) = (None, None); // change-start in a / b
    let flush = |cs: &mut Option<usize>,
                 ds: &mut Option<usize>,
                 i: usize,
                 j: usize,
                 hunks: &mut Vec<Hunk>| {
        if let (Some(s), Some(t)) = (cs.take(), ds.take()) {
            hunks.push(Hunk {
                before_start: s,
                before_count: i - s,
                after_start: t,
                after_count: j - t,
            });
        }
    };
    while i < n || j < m {
        let same = i < n && j < m && a[i] == b[j];
        if same {
            flush(&mut cs, &mut ds, i, j, &mut hunks);
            i += 1;
            j += 1;
        } else if j < m && (i >= n || dp[i][j + 1] >= dp[i + 1][j]) {
            if cs.is_none() {
                cs = Some(i);
                ds = Some(j);
            }
            j += 1;
        } else {
            if cs.is_none() {
                cs = Some(i);
                ds = Some(j);
            }
            i += 1;
        }
    }
    flush(&mut cs, &mut ds, i, j, &mut hunks);
    hunks
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn identical_is_empty() {
        assert!(line_diff("a\nb\n", "a\nb\n").is_empty());
    }

    #[test]
    fn single_line_change() {
        let h = line_diff("a\nb\nc\n", "a\nB\nc\n");
        assert_eq!(h.len(), 1);
        assert_eq!(h[0].before_start, 1);
        assert_eq!((h[0].before_count, h[0].after_count), (1, 1));
    }

    #[test]
    fn insertion_and_deletion() {
        let h = line_diff("a\nc\n", "a\nb\nc\nd\n");
        assert_eq!(h.len(), 2);
        assert_eq!((h[0].before_count, h[0].after_count), (0, 1));
        assert_eq!((h[1].before_count, h[1].after_count), (0, 1));
    }
}
