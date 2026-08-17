// Copyright 2015 The Servo Project Developers. See the
// COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! 3.3.4 - 3.3.6. Resolve implicit levels and types.

#[cfg(not(feature = "smallvec"))]
use alloc::vec::Vec;
use core::cmp::max;
#[cfg(feature = "smallvec")]
use smallvec::SmallVec;

use super::char_data::BidiClass::{self, *};
use super::level::Level;
use super::prepare::{not_removed_by_x9, IsolatingRunSequence};
use super::{BidiDataSource, TextSource};

/// 3.3.4 Resolving Weak Types
///
/// <http://www.unicode.org/reports/tr9/#Resolving_Weak_Types>
#[cfg_attr(feature = "flame_it", flamer::flame)]
pub fn resolve_weak<'a, T: TextSource<'a> + ?Sized>(
    text: &'a T,
    sequence: &IsolatingRunSequence,
    processing_classes: &mut [BidiClass],
) {
    // Note: The spec treats these steps as individual passes that are applied one after the other
    // on the entire IsolatingRunSequence at once. We instead collapse it into a single iteration,
    // which is straightforward for rules that are based on the state of the current character, but not
    // for rules that care about surrounding characters. To deal with them, we retain additional state
    // about previous character classes that may have since been changed by later rules.

    // The previous class for the purposes of rule W4/W6, not tracking changes made after or during W4.
    let mut prev_class_before_w4 = sequence.sos;
    // The previous class for the purposes of rule W5.
    let mut prev_class_before_w5 = sequence.sos;
    // The previous class for the purposes of rule W1, not tracking changes from any other rules.
    let mut prev_class_before_w1 = sequence.sos;
    let mut last_strong_is_al = false;
    #[cfg(feature = "smallvec")]
    let mut et_run_indices = SmallVec::<[usize; 8]>::new(); // for W5
    #[cfg(not(feature = "smallvec"))]
    let mut et_run_indices = Vec::new(); // for W5
    #[cfg(feature = "smallvec")]
    let mut bn_run_indices = SmallVec::<[usize; 8]>::new(); // for W5 +  <https://www.unicode.org/reports/tr9/#Retaining_Explicit_Formatting_Characters>
    #[cfg(not(feature = "smallvec"))]
    let mut bn_run_indices = Vec::new(); // for W5 +  <https://www.unicode.org/reports/tr9/#Retaining_Explicit_Formatting_Characters>

    for (run_index, level_run) in sequence.runs.iter().enumerate() {
        for i in &mut level_run.clone() {
            if processing_classes[i] == BN {
                // <https://www.unicode.org/reports/tr9/#Retaining_Explicit_Formatting_Characters>
                // Keeps track of bn runs for W5 in case we see an ET.
                bn_run_indices.push(i);
                // BNs aren't real, skip over them.
                continue;
            }

            // Store the processing class of all rules before W2/W1.
            // Used to keep track of the last strong character for W2. W3 is able to insert new strong
            // characters, so we don't want to be misled by it.
            let mut w2_processing_class = processing_classes[i];

            // <http://www.unicode.org/reports/tr9/#W1>
            //

            if processing_classes[i] == NSM {
                processing_classes[i] = match prev_class_before_w1 {
                    RLI | LRI | FSI | PDI => ON,
                    _ => prev_class_before_w1,
                };
                // W1 occurs before W2, update this.
                w2_processing_class = processing_classes[i];
            }

            prev_class_before_w1 = processing_classes[i];

            // <http://www.unicode.org/reports/tr9/#W2>
            // <http://www.unicode.org/reports/tr9/#W3>
            //
            match processing_classes[i] {
                EN => {
                    if last_strong_is_al {
                        // W2. If previous strong char was AL, change EN to AN.
                        processing_classes[i] = AN;
                    }
                }
                // W3.
                AL => processing_classes[i] = R,
                _ => {}
            }

            // update last_strong_is_al.
            match w2_processing_class {
                L | R => {
                    last_strong_is_al = false;
                }
                AL => {
                    last_strong_is_al = true;
                }
                _ => {}
            }

            let class_before_w456 = processing_classes[i];

            // <http://www.unicode.org/reports/tr9/#W4>
            // <http://www.unicode.org/reports/tr9/#W5>
            // <http://www.unicode.org/reports/tr9/#W6> (separators only)
            // (see below for W6 terminator code)
            //
            match processing_classes[i] {
                // <http://www.unicode.org/reports/tr9/#W6>
                EN => {
                    // W5. If a run of ETs is adjacent to an EN, change the ETs to EN.
                    for j in &et_run_indices {
                        processing_classes[*j] = EN;
                    }
                    et_run_indices.clear();
                }

                // <http://www.unicode.org/reports/tr9/#W4>
                // <http://www.unicode.org/reports/tr9/#W6>
                ES | CS => {
                    // See https://github.com/servo/unicode-bidi/issues/86 for improving this.
                    // We want to make sure we check the correct next character by skipping past the rest
                    // of this one.
                    if let Some((_, char_len)) = text.char_at(i) {
                        let mut next_class = sequence
                            .iter_forwards_from(i + char_len, run_index)
                            .map(|j| processing_classes[j])
                            // <https://www.unicode.org/reports/tr9/#Retaining_Explicit_Formatting_Characters>
                            .find(not_removed_by_x9)
                            .unwrap_or(sequence.eos);
                        if next_class == EN && last_strong_is_al {
                            // Apply W2 to next_class. We know that last_strong_is_al
                            // has no chance of changing on this character so we can still assume its value
                            // will be the same by the time we get to it.
                            next_class = AN;
                        }
                        processing_classes[i] =
                            match (prev_class_before_w4, processing_classes[i], next_class) {
                                // W4
                                (EN, ES, EN) | (EN, CS, EN) => EN,
                                // W4
                                (AN, CS, AN) => AN,
                                // W6 (separators only)
                                (_, _, _) => ON,
                            };

                        // W6 + <https://www.unicode.org/reports/tr9/#Retaining_Explicit_Formatting_Characters>
                        // We have to do this before W5 gets its grubby hands on these characters and thinks
                        // they're part of an ET run.
                        // We check for ON to ensure that we had hit the W6 branch above, since this `ES | CS` match
                        // arm handles both W4 and W6.
                        if processing_classes[i] == ON {
                            for idx in sequence.iter_backwards_from(i, run_index) {
                                let class = &mut processing_classes[idx];
                                if *class != BN {
                                    break;
                                }
                                *class = ON;
                            }
                            for idx in sequence.iter_forwards_from(i + char_len, run_index) {
                                let class = &mut processing_classes[idx];
                                if *class != BN {
                                    break;
                                }
                                *class = ON;
                            }
                        }
                    } else {
                        // We're in the middle of a character, copy over work done for previous bytes
                        // since it's going to be the same answer.
                        processing_classes[i] = processing_classes[i - 1];
                    }
                }
                // <http://www.unicode.org/reports/tr9/#W5>
                ET => {
                    match prev_class_before_w5 {
                        EN => processing_classes[i] = EN,
                        _ => {
                            // <https://www.unicode.org/reports/tr9/#Retaining_Explicit_Formatting_Characters>
                            // If there was a BN run before this, that's now a part of this ET run.
                            et_run_indices.extend(bn_run_indices.clone());

                            // In case this is followed by an EN.
                            et_run_indices.push(i);
                        }
                    }
                }
                _ => {}
            }

            // Common loop iteration code
            //

            // <https://www.unicode.org/reports/tr9/#Retaining_Explicit_Formatting_Characters>
            // BN runs would have already continued the loop, clear them before we get to the next one.
            bn_run_indices.clear();

            // W6 above only deals with separators, so it doesn't change anything W5 cares about,
            // so we still can update this after running that part of W6.
            prev_class_before_w5 = processing_classes[i];

            // <http://www.unicode.org/reports/tr9/#W6> (terminators only)
            // (see above for W6 separator code)
            //
            if prev_class_before_w5 != ET {
                // W6. If we didn't find an adjacent EN, turn any ETs into ON instead.
                for j in &et_run_indices {
                    processing_classes[*j] = ON;
                }
                et_run_indices.clear();
            }

            // We stashed this before W4/5/6 could get their grubby hands on it, and it's not
            // used in the W6 terminator code below so we can update it now.
            prev_class_before_w4 = class_before_w456;
        }
    }
    // Rerun this check in case we ended with a sequence of BNs (i.e., we'd never
    // hit the end of the for loop above).
    // W6. If we didn't find an adjacent EN, turn any ETs into ON instead.
    for j in &et_run_indices {
        processing_classes[*j] = ON;
    }
    et_run_indices.clear();

    // W7. If the previous strong char was L, change EN to L.
    let mut last_strong_is_l = sequence.sos == L;
    for i in sequence.runs.iter().cloned().flatten() {
        match processing_classes[i] {
            EN if last_strong_is_l => {
                processing_classes[i] = L;
            }
            L => {
                last_strong_is_l = true;
            }
            R | AL => {
                last_strong_is_l = false;
            }
            // <https://www.unicode.org/reports/tr9/#Retaining_Explicit_Formatting_Characters>
            // Already scanning past BN here.
            _ => {}
        }
    }
}

#[cfg(feature = "smallvec")]
type BracketPairVec = SmallVec<[BracketPair; 8]>;
#[cfg(not(feature = "smallvec"))]
type BracketPairVec = Vec<BracketPair>;

/// 3.3.5 Resolving Neutral Types
///
/// <http://www.unicode.org/reports/tr9/#Resolving_Neutral_Types>
#[cfg_attr(feature = "flame_it", flamer::flame)]
pub fn resolve_neutral<'a, D: BidiDataSource, T: TextSource<'a> + ?Sized>(
    text: &'a T,
    data_source: &D,
    sequence: &IsolatingRunSequence,
    levels: &[Level],
    original_classes: &[BidiClass],
    processing_classes: &mut [BidiClass],
) {
    // e = embedding direction
    let e: BidiClass = levels[sequence.runs[0].start].bidi_class();
    let not_e = if e == BidiClass::L {
        BidiClass::R
    } else {
        BidiClass::L
    };
    // N0. Process bracket pairs.

    // > Identify the bracket pairs in the current isolating run sequence according to BD16.
    // We use processing_classes, not original_classes, due to BD14/BD15
    let mut bracket_pairs = BracketPairVec::new();
    identify_bracket_pairs(
        text,
        data_source,
        sequence,
        processing_classes,
        &mut bracket_pairs,
    );

    // > For each bracket-pair element in the list of pairs of text positions
    //
    // Note: Rust ranges are interpreted as [start..end), be careful using `pair` directly
    // for indexing as it will include the opening bracket pair but not the closing one.
    for pair in bracket_pairs {
        #[cfg(feature = "std")]
        debug_assert!(
            pair.start < processing_classes.len(),
            "identify_bracket_pairs returned a range that is out of bounds!"
        );
        #[cfg(feature = "std")]
        debug_assert!(
            pair.end < processing_classes.len(),
            "identify_bracket_pairs returned a range that is out of bounds!"
        );
        let mut found_e = false;
        let mut found_not_e = false;
        let mut class_to_set = None;

        let start_char_len =
            T::char_len(text.subrange(pair.start..pair.end).chars().next().unwrap());
        // > Inspect the bidirectional types of the characters enclosed within the bracket pair.
        //
        // `pair` is [start, end) so we will end up processing the opening character but not the closing one.
        //
        for enclosed_i in sequence.iter_forwards_from(pair.start + start_char_len, pair.start_run) {
            if enclosed_i >= pair.end {
                #[cfg(feature = "std")]
                debug_assert!(
                    enclosed_i == pair.end,
                    "If we skipped past this, the iterator is broken"
                );
                break;
            }
            let class = processing_classes[enclosed_i];
            if class == e {
                found_e = true;
            } else if class == not_e {
                found_not_e = true;
            } else if matches!(class, BidiClass::EN | BidiClass::AN) {
                // > Within this scope, bidirectional types EN and AN are treated as R.
                if e == BidiClass::L {
                    found_not_e = true;
                } else {
                    found_e = true;
                }
            }

            // If we have found a character with the class of the embedding direction
            // we can bail early.
            if found_e {
                break;
            }
        }
        // > If any strong type (either L or R) matching the embedding direction is found
        if found_e {
            // > .. set the type for both brackets in the pair to match the embedding direction
            class_to_set = Some(e);
        // > Otherwise, if there is a strong type it must be opposite the embedding direction
        } else if found_not_e {
            // > Therefore, test for an established context with a preceding strong type by
            // > checking backwards before the opening paired bracket
            // > until the first strong type (L, R, or sos) is found.
            // (see note above about processing_classes and character boundaries)
            let mut previous_strong = sequence
                .iter_backwards_from(pair.start, pair.start_run)
                .map(|i| processing_classes[i])
                .find(|class| {
                    matches!(
                        class,
                        BidiClass::L | BidiClass::R | BidiClass::EN | BidiClass::AN
                    )
                })
                .unwrap_or(sequence.sos);

            // > Within this scope, bidirectional types EN and AN are treated as R.
            if matches!(previous_strong, BidiClass::EN | BidiClass::AN) {
                previous_strong = BidiClass::R;
            }

            // > If the preceding strong type is also opposite the embedding direction,
            // > context is established,
            // > so set the type for both brackets in the pair to that direction.
            // AND
            // > Otherwise set the type for both brackets in the pair to the embedding direction.
            // > Either way it gets set to previous_strong
            //
            // Both branches amount to setting the type to the strong type.
            class_to_set = Some(previous_strong);
        }

        if let Some(class_to_set) = class_to_set {
            // Update all processing classes corresponding to the start and end elements, as requested.
            // We should include all bytes of the character, not the first one.
            let end_char_len =
                T::char_len(text.subrange(pair.end..text.len()).chars().next().unwrap());
            for class in &mut processing_classes[pair.start..pair.start + start_char_len] {
                *class = class_to_set;
            }
            for class in &mut processing_classes[pair.end..pair.end + end_char_len] {
                *class = class_to_set;
            }
            // <https://www.unicode.org/reports/tr9/#Retaining_Explicit_Formatting_Characters>
            for idx in sequence.iter_backwards_from(pair.start, pair.start_run) {
                let class = &mut processing_classes[idx];
                if *class != BN {
                    break;
                }
                *class = class_to_set;
            }
            // > Any number of characters that had original bidirectional character type NSM prior to the application of
            // > W1 that immediately follow a paired bracket which changed to L or R under N0 should change to match the type of their preceding bracket.

            // This rule deals with sequences of NSMs, so we can just update them all at once, we don't need to worry
            // about character boundaries. We do need to be careful to skip the full set of bytes for the parentheses characters.
            let nsm_start = pair.start + start_char_len;
            for idx in sequence.iter_forwards_from(nsm_start, pair.start_run) {
                let class = original_classes[idx];
                if class == BidiClass::NSM || processing_classes[idx] == BN {
                    processing_classes[idx] = class_to_set;
                } else {
                    break;
                }
            }
            let nsm_end = pair.end + end_char_len;
            for idx in sequence.iter_forwards_from(nsm_end, pair.end_run) {
                let class = original_classes[idx];
                if class == BidiClass::NSM || processing_classes[idx] == BN {
                    processing_classes[idx] = class_to_set;
                } else {
                    break;
                }
            }
        }
        // > Otherwise, there are no strong types within the bracket pair
        // > Therefore, do not set the type for that bracket pair
    }

    // N1 and N2.
    // Indices of every byte in this isolating run sequence
    let mut indices = sequence.runs.iter().flat_map(Clone::clone);
    let mut prev_class = sequence.sos;
    while let Some(mut i) = indices.next() {
        // Process sequences of NI characters.
        #[cfg(feature = "smallvec")]
        let mut ni_run = SmallVec::<[usize; 8]>::new();
        #[cfg(not(feature = "smallvec"))]
        let mut ni_run = Vec::new();
        // The BN is for <https://www.unicode.org/reports/tr9/#Retaining_Explicit_Formatting_Characters>
        if is_NI(processing_classes[i]) || processing_classes[i] == BN {
            // Consume a run of consecutive NI characters.
            ni_run.push(i);
            let mut next_class;
            loop {
                match indices.next() {
                    Some(j) => {
                        i = j;
                        next_class = processing_classes[j];
                        // The BN is for <https://www.unicode.org/reports/tr9/#Retaining_Explicit_Formatting_Characters>
                        if is_NI(next_class) || next_class == BN {
                            ni_run.push(i);
                        } else {
                            break;
                        }
                    }
                    None => {
                        next_class = sequence.eos;
                        break;
                    }
                };
            }
            // N1-N2.
            //
            // <http://www.unicode.org/reports/tr9/#N1>
            // <http://www.unicode.org/reports/tr9/#N2>
            let new_class = match (prev_class, next_class) {
                (L, L) => L,
                (R, R)
                | (R, AN)
                | (R, EN)
                | (AN, R)
                | (AN, AN)
                | (AN, EN)
                | (EN, R)
                | (EN, AN)
                | (EN, EN) => R,
                (_, _) => e,
            };
            for j in &ni_run {
                processing_classes[*j] = new_class;
            }
            ni_run.clear();
        }
        prev_class = processing_classes[i];
    }
}

struct BracketPair {
    /// The text-relative index of the opening bracket.
    start: usize,
    /// The text-relative index of the closing bracket.
    end: usize,
    /// The index of the run (in the run sequence) that the opening bracket is in.
    start_run: usize,
    /// The index of the run (in the run sequence) that the closing bracket is in.
    end_run: usize,
}
/// 3.1.3 Identifying Bracket Pairs
///
/// Returns all paired brackets in the source, as indices into the
/// text source.
///
/// <https://www.unicode.org/reports/tr9/#BD16>
fn identify_bracket_pairs<'a, T: TextSource<'a> + ?Sized, D: BidiDataSource>(
    text: &'a T,
    data_source: &D,
    run_sequence: &IsolatingRunSequence,
    original_classes: &[BidiClass],
    bracket_pairs: &mut BracketPairVec,
) {
    #[cfg(feature = "smallvec")]
    let mut stack = SmallVec::<[(char, usize, usize); 8]>::new();
    #[cfg(not(feature = "smallvec"))]
    let mut stack = Vec::new();

    for (run_index, level_run) in run_sequence.runs.iter().enumerate() {
        for (i, ch) in text.subrange(level_run.clone()).char_indices() {
            let actual_index = level_run.start + i;

            // All paren characters are ON.
            // From BidiBrackets.txt:
            // > The Unicode property value stability policy guarantees that characters
            // > which have bpt=o or bpt=c also have bc=ON and Bidi_M=Y
            if original_classes[actual_index] != BidiClass::ON {
                continue;
            }

            if let Some(matched) = data_source.bidi_matched_opening_bracket(ch) {
                if matched.is_open {
                    // > If an opening paired bracket is found ...

                    // > ... and there is no room in the stack,
                    // > stop processing BD16 for the remainder of the isolating run sequence.
                    if stack.len() >= 63 {
                        break;
                    }
                    // > ... push its Bidi_Paired_Bracket property value and its text position onto the stack
                    stack.push((matched.opening, actual_index, run_index))
                } else {
                    // > If a closing paired bracket is found, do the following

                    // > Declare a variable that holds a reference to the current stack element
                    // > and initialize it with the top element of the stack.
                    // AND
                    // > Else, if the current stack element is not at the bottom of the stack
                    for (stack_index, element) in stack.iter().enumerate().rev() {
                        // > Compare the closing paired bracket being inspected or its canonical
                        // > equivalent to the bracket in the current stack element.
                        if element.0 == matched.opening {
                            // > If the values match, meaning the two characters form a bracket pair, then

                            // > Append the text position in the current stack element together with the
                            // > text position of the closing paired bracket to the list.
                            let pair = BracketPair {
                                start: element.1,
                                end: actual_index,
                                start_run: element.2,
                                end_run: run_index,
                            };
                            bracket_pairs.push(pair);

                            // > Pop the stack through the current stack element inclusively.
                            stack.truncate(stack_index);
                            break;
                        }
                    }
                }
            }
        }
    }
    // > Sort the list of pairs of text positions in ascending order based on
    // > the text position of the opening paired bracket.
    bracket_pairs.sort_by_key(|r| r.start);
}

/// 3.3.6 Resolving Implicit Levels
///
/// Returns the maximum embedding level in the paragraph.
///
/// <http://www.unicode.org/reports/tr9/#Resolving_Implicit_Levels>
#[cfg_attr(feature = "flame_it", flamer::flame)]
pub fn resolve_levels(processing_classes: &[BidiClass], levels: &mut [Level]) -> Level {
    let mut max_level = Level::ltr();
    assert_eq!(processing_classes.len(), levels.len());
    for i in 0..levels.len() {
        match (levels[i].is_rtl(), processing_classes[i]) {
            (false, AN) | (false, EN) => levels[i].raise(2).expect("Level number error"),
            (false, R) | (true, L) | (true, EN) | (true, AN) => {
                levels[i].raise(1).expect("Level number error")
            }
            // <https://www.unicode.org/reports/tr9/#Retaining_Explicit_Formatting_Characters> handled here
            (_, _) => {}
        }
        max_level = max(max_level, levels[i]);
    }

    max_level
}

/// Neutral or Isolate formatting character (B, S, WS, ON, FSI, LRI, RLI, PDI)
///
/// <http://www.unicode.org/reports/tr9/#NI>
#[allow(non_snake_case)]
fn is_NI(class: BidiClass) -> bool {
    matches!(class, B | S | WS | ON | FSI | LRI | RLI | PDI)
}
