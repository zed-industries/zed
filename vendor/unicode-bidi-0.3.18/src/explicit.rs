// Copyright 2015 The Servo Project Developers. See the
// COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! 3.3.2 Explicit Levels and Directions
//!
//! <http://www.unicode.org/reports/tr9/#Explicit_Levels_and_Directions>

#[cfg(feature = "smallvec")]
use smallvec::{smallvec, SmallVec};

use super::char_data::{
    is_rtl,
    BidiClass::{self, *},
};
use super::level::Level;
use super::prepare::removed_by_x9;
use super::LevelRunVec;
use super::TextSource;

/// Compute explicit embedding levels for one paragraph of text (X1-X8), and identify
/// level runs (BD7) for use when determining Isolating Run Sequences (X10).
///
/// `processing_classes[i]` must contain the `BidiClass` of the char at byte index `i`,
/// for each char in `text`.
///
/// `runs` returns the list of level runs (BD7) of the text.
#[cfg_attr(feature = "flame_it", flamer::flame)]
pub fn compute<'a, T: TextSource<'a> + ?Sized>(
    text: &'a T,
    para_level: Level,
    original_classes: &[BidiClass],
    levels: &mut [Level],
    processing_classes: &mut [BidiClass],
    runs: &mut LevelRunVec,
) {
    assert_eq!(text.len(), original_classes.len());

    // <http://www.unicode.org/reports/tr9/#X1>
    #[cfg(feature = "smallvec")]
    let mut stack: SmallVec<[Status; 8]> = smallvec![Status {
        level: para_level,
        status: OverrideStatus::Neutral,
    }];
    #[cfg(not(feature = "smallvec"))]
    let mut stack = vec![Status {
        level: para_level,
        status: OverrideStatus::Neutral,
    }];

    let mut overflow_isolate_count = 0u32;
    let mut overflow_embedding_count = 0u32;
    let mut valid_isolate_count = 0u32;

    let mut current_run_level = Level::ltr();
    let mut current_run_start = 0;

    for (i, len) in text.indices_lengths() {
        let last = stack.last().unwrap();

        match original_classes[i] {
            // Rules X2-X5c
            RLE | LRE | RLO | LRO | RLI | LRI | FSI => {
                // <https://www.unicode.org/reports/tr9/#Retaining_Explicit_Formatting_Characters>
                levels[i] = last.level;

                // X5a-X5c: Isolate initiators get the level of the last entry on the stack.
                let is_isolate = matches!(original_classes[i], RLI | LRI | FSI);
                if is_isolate {
                    // Redundant due to "Retaining explicit formatting characters" step.
                    // levels[i] = last.level;
                    match last.status {
                        OverrideStatus::RTL => processing_classes[i] = R,
                        OverrideStatus::LTR => processing_classes[i] = L,
                        _ => {}
                    }
                }

                let new_level = if is_rtl(original_classes[i]) {
                    last.level.new_explicit_next_rtl()
                } else {
                    last.level.new_explicit_next_ltr()
                };

                if new_level.is_ok() && overflow_isolate_count == 0 && overflow_embedding_count == 0
                {
                    let new_level = new_level.unwrap();

                    stack.push(Status {
                        level: new_level,
                        status: match original_classes[i] {
                            RLO => OverrideStatus::RTL,
                            LRO => OverrideStatus::LTR,
                            RLI | LRI | FSI => OverrideStatus::Isolate,
                            _ => OverrideStatus::Neutral,
                        },
                    });

                    if is_isolate {
                        valid_isolate_count += 1;
                    } else {
                        // The spec doesn't explicitly mention this step, but it is necessary.
                        // See the reference implementations for comparison.
                        levels[i] = new_level;
                    }
                } else if is_isolate {
                    overflow_isolate_count += 1;
                } else if overflow_isolate_count == 0 {
                    overflow_embedding_count += 1;
                }

                if !is_isolate {
                    // X9 +
                    // <https://www.unicode.org/reports/tr9/#Retaining_Explicit_Formatting_Characters>
                    // (PDF handled below)
                    processing_classes[i] = BN;
                }
            }

            // <http://www.unicode.org/reports/tr9/#X6a>
            PDI => {
                if overflow_isolate_count > 0 {
                    overflow_isolate_count -= 1;
                } else if valid_isolate_count > 0 {
                    overflow_embedding_count = 0;

                    while !matches!(
                        stack.pop(),
                        None | Some(Status {
                            status: OverrideStatus::Isolate,
                            ..
                        })
                    ) {}

                    valid_isolate_count -= 1;
                }

                let last = stack.last().unwrap();
                levels[i] = last.level;

                match last.status {
                    OverrideStatus::RTL => processing_classes[i] = R,
                    OverrideStatus::LTR => processing_classes[i] = L,
                    _ => {}
                }
            }

            // <http://www.unicode.org/reports/tr9/#X7>
            PDF => {
                if overflow_isolate_count > 0 {
                    // do nothing
                } else if overflow_embedding_count > 0 {
                    overflow_embedding_count -= 1;
                } else if last.status != OverrideStatus::Isolate && stack.len() >= 2 {
                    stack.pop();
                }

                // <https://www.unicode.org/reports/tr9/#Retaining_Explicit_Formatting_Characters>
                levels[i] = stack.last().unwrap().level;
                // X9 part of retaining explicit formatting characters.
                processing_classes[i] = BN;
            }

            // Nothing.
            // BN case moved down to X6, see <https://www.unicode.org/reports/tr9/#Retaining_Explicit_Formatting_Characters>
            B => {}

            // <http://www.unicode.org/reports/tr9/#X6>
            _ => {
                levels[i] = last.level;

                // This condition is not in the spec, but I am pretty sure that is a spec bug.
                // https://www.unicode.org/L2/L2023/23014-amd-to-uax9.pdf
                if original_classes[i] != BN {
                    match last.status {
                        OverrideStatus::RTL => processing_classes[i] = R,
                        OverrideStatus::LTR => processing_classes[i] = L,
                        _ => {}
                    }
                }
            }
        }

        // Handle multi-byte characters.
        for j in 1..len {
            levels[i + j] = levels[i];
            processing_classes[i + j] = processing_classes[i];
        }

        // Identify level runs to be passed to prepare::isolating_run_sequences().
        if i == 0 {
            // Initialize for the first (or only) run.
            current_run_level = levels[i];
        } else {
            // Check if we need to start a new level run.
            // <https://www.unicode.org/reports/tr9/#BD7>
            if !removed_by_x9(original_classes[i]) && levels[i] != current_run_level {
                // End the last run and start a new one.
                runs.push(current_run_start..i);
                current_run_level = levels[i];
                current_run_start = i;
            }
        }
    }

    // Append the trailing level run, if non-empty.
    if levels.len() > current_run_start {
        runs.push(current_run_start..levels.len());
    }
}

/// Entries in the directional status stack:
struct Status {
    level: Level,
    status: OverrideStatus,
}

#[derive(PartialEq)]
enum OverrideStatus {
    Neutral,
    RTL,
    LTR,
    Isolate,
}
