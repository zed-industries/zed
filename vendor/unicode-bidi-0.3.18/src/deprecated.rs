// Copyright 2015 The Servo Project Developers. See the
// COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! This module holds deprecated assets only.

use super::*;

/// Find the level runs within a line and return them in visual order.
///
/// NOTE: This implementation is incomplete. The algorithm needs information about the text,
/// including original `BidiClass` property of each character, to be able to perform correctly.
/// Please see [`BidiInfo::visual_runs()`](../struct.BidiInfo.html#method.visual_runs) for the
/// improved implementation.
///
/// `line` is a range of bytes indices within `levels`.
///
/// <http://www.unicode.org/reports/tr9/#Reordering_Resolved_Levels>
#[deprecated(
    since = "0.3.0",
    note = "please use `BidiInfo::visual_runs()` instead."
)]
pub fn visual_runs(line: Range<usize>, levels: &[Level]) -> Vec<LevelRun> {
    assert!(line.start <= levels.len());
    assert!(line.end <= levels.len());

    let mut runs = Vec::new();

    // Find consecutive level runs.
    let mut start = line.start;
    let mut run_level = levels[start];
    let mut min_level = run_level;
    let mut max_level = run_level;

    for (i, &new_level) in levels.iter().enumerate().take(line.end).skip(start + 1) {
        if new_level != run_level {
            // End of the previous run, start of a new one.
            runs.push(start..i);
            start = i;
            run_level = new_level;

            min_level = cmp::min(run_level, min_level);
            max_level = cmp::max(run_level, max_level);
        }
    }
    runs.push(start..line.end);

    let run_count = runs.len();

    // Re-order the odd runs.
    // <http://www.unicode.org/reports/tr9/#L2>

    // Stop at the lowest *odd* level.
    min_level = min_level.new_lowest_ge_rtl().expect("Level error");

    while max_level >= min_level {
        // Look for the start of a sequence of consecutive runs of max_level or higher.
        let mut seq_start = 0;
        while seq_start < run_count {
            if levels[runs[seq_start].start] < max_level {
                seq_start += 1;
                continue;
            }

            // Found the start of a sequence. Now find the end.
            let mut seq_end = seq_start + 1;

            while seq_end < run_count && levels[runs[seq_end].start] >= max_level {
                seq_end += 1;
            }

            // Reverse the runs within this sequence.
            runs[seq_start..seq_end].reverse();

            seq_start = seq_end;
        }

        max_level
            .lower(1)
            .expect("Lowering embedding level below zero");
    }

    runs
}
